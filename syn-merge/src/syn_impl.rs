//! `Merge` implementation for `syn` types.
//!
//! Note: There is no need to compare tokens, they're always equal.
//! <https://docs.rs/syn/2.0.43/src/syn/token.rs.html#302-306>

impl_merge_eq!(syn::AngleBracketedGenericArguments);
impl_merge_eq!(syn::BinOp);
impl_merge_eq!(syn::BoundLifetimes);
impl_merge_eq!(syn::FieldMutability);
impl_merge_eq!(syn::Generics);
impl_merge_eq!(syn::Index);
impl_merge_eq!(syn::Label);
impl_merge_eq!(syn::Lifetime);
impl_merge_eq!(syn::Lit);
impl_merge_eq!(syn::LitBool);
impl_merge_eq!(syn::LitByte);
impl_merge_eq!(syn::LitByteStr);
impl_merge_eq!(syn::LitChar);
impl_merge_eq!(syn::LitFloat);
impl_merge_eq!(syn::LitInt);
impl_merge_eq!(syn::LitStr);
impl_merge_eq!(syn::MacroDelimiter);
impl_merge_eq!(syn::Member);
impl_merge_eq!(syn::Path);
impl_merge_eq!(syn::QSelf);
impl_merge_eq!(syn::RangeLimits);
impl_merge_eq!(syn::ReturnType);
impl_merge_eq!(syn::StaticMutability);
impl_merge_eq!(syn::Type);
impl_merge_eq!(syn::TypeParamBound);
impl_merge_eq!(syn::UnOp);
impl_merge_eq!(syn::UseTree);
impl_merge_eq!(syn::Visibility);

// For now!
impl_merge_eq!(syn::Macro);

impl crate::Merge for syn::Fields {
    fn top_level_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Named(this), Self::Named(other)) => this.top_level_eq(other),
            (Self::Unnamed(this), Self::Unnamed(other)) => this.top_level_eq(other),
            (Self::Unit, Self::Unit) => true,
            _ => false,
        }
    }

    fn add_attr(&mut self, attr: crate::Attribute) {
        match self {
            Self::Named(item) => item.add_attr(attr),
            Self::Unnamed(item) => item.add_attr(attr),
            Self::Unit => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

impl crate::Merge for syn::Stmt {
    fn top_level_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Local(this), Self::Local(other)) => this.top_level_eq(other),
            (Self::Item(this), Self::Item(other)) => this.top_level_eq(other),
            (Self::Expr(this, _), Self::Expr(other, _)) => this.top_level_eq(other),
            (Self::Macro(this), Self::Macro(other)) => this.top_level_eq(other),
            _ => false,
        }
    }

    fn add_attr(&mut self, attr: crate::Attribute) {
        match self {
            Self::Local(item) => item.add_attr(attr),
            Self::Item(item) => item.add_attr(attr),
            Self::Expr(item, _) => item.add_attr(attr),
            Self::Macro(item) => item.add_attr(attr),
            _ => unimplemented!(),
        }
    }
}
