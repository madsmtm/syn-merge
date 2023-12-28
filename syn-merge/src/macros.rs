macro_rules! impl_merge_eq {
    ($(<($generics:ident),*>)? $ty:ty) => {
        impl $(<($generics: PartialEq),*>)? crate::Merge for $ty {
            fn top_level_eq(&self, other: &Self) -> bool {
                self == other
            }

            fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a crate::Cfgs)>>(iter: I) -> Self
            where
                Self: 'a,
                I::IntoIter: Clone,
            {
                let (item, _cfgs) = iter.into_iter().next().unwrap();
                item.clone()
            }

            fn add_attr(&mut self, _attr: crate::Attribute) {
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
        impl crate::Merge for $ty {
            fn top_level_eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (Self::$variant(this), Self::$variant(other)) => this.top_level_eq(other),
                    )*
                    _ => false,
                }
            }

            fn add_attr(&mut self, _attr: crate::Attribute) {
                match self {
                    $(
                        Self::$variant(item) => item.add_attr(_attr),
                    )*
                    $(_ => unimplemented!() $comma)?
                }
            }
        }
    };
}

macro_rules! impl_merge_struct {
    (
        $(#[$attrs:ident])?
        $ty:ty {
            $($field:ident),* $(,)?
        }
    ) => {
        impl crate::Merge for $ty {
            fn top_level_eq(&self, other: &Self) -> bool {
                true $(&& self.$field.top_level_eq(&other.$field))*
            }

            fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a crate::Cfgs)>>(iter: I) -> Self
            where
                Self: 'a,
                I::IntoIter: Clone,
            {
                let iter = iter.into_iter();
                Self {
                    $($field: crate::Merge::merge(iter.clone().map(|(Self { $field, .. }, cfgs)| ($field, cfgs))),)*
                }
            }

            fn add_attr(&mut self, _attr: crate::Attribute) {
                $(self.$attrs.push(_attr);)?
            }
        }
    };
}
