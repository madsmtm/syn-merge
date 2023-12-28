macro_rules! impl_merge_eq {
    ($(<($generics:ident),*>)? $ty:ty) => {
        impl $(<($generics: PartialEq),*>)? Merge for $ty {
            fn top_level_eq(&self, other: &Self) -> bool {
                self == other
            }

            fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
            where
                Self: 'a,
                I::IntoIter: Clone,
            {
                let (item, _cfgs) = iter.into_iter().next().unwrap();
                item.clone()
            }

            fn add_attr(&mut self, attr: Attribute) {
                unreachable!()
            }
        }
    };
}

macro_rules! impl_merge_enum {
    (
        $ty:ty {
            $($variant:ident ,)*
            $(_ $comma:tt)?
        }
    ) => {
        impl Merge for $ty {
            fn top_level_eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (Self::$variant(this), Self::$variant(other)) => this.top_level_eq(other),
                    )*
                    _ => false,
                }
            }

            fn add_attr(&mut self, attr: Attribute) {
                match self {
                    $(
                        Self::$variant(item) => item.add_attr(attr),
                    )*
                    $(_ => unimplemented!() $comma)?
                }
            }
        }
    };
}

macro_rules! impl_merge_struct {
    (
        $ty:ident $(($($recursed:ident),*))? {
            $($field:ident),* $(,)?
        }
    ) => {
        impl Merge for $ty {
            fn top_level_eq(&self, other: &Self) -> bool {
                // Ensure we've named all fields
                let Self {
                    attrs: _,
                    $($field : _,)*
                    $($($recursed : _,)*)?
                } = self;

                // TODO: This should maybe use `top_level_eq` recursively?
                true $(&& self.$field == other.$field)*
            }

            fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
            where
                Self: 'a,
                I::IntoIter: Clone,
            {
                let iter = iter.into_iter();
                Self {
                    attrs: Merge::merge(iter.clone().map(|(Self { attrs, .. }, cfgs)| (attrs, cfgs))),
                    $($field: merge_by_extracting_first(iter.clone().map(|(Self { $field, .. }, cfgs)| ($field, cfgs))),)*
                    $($($recursed: Merge::merge(iter.clone().map(|(Self { $recursed, .. }, cfgs)| ($recursed, cfgs))),)*)?
                }
            }

            fn add_attr(&mut self, attr: Attribute) {
                self.attrs.push(attr);
            }
        }
    };
}
