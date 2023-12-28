//! `Merge` implementation for `syn` types.
//!
//! TODO: Automatically generate this?
//!
//! Note: There is no need to compare tokens, they're always equal.
//! <https://docs.rs/syn/2.0.43/src/syn/token.rs.html#302-306>
use proc_macro2::TokenStream;
use syn::{Attribute, Expr, Field, ForeignItem, ImplItem, Stmt, Variant};

use super::Merge;

impl Merge for TokenStream {
    fn top_level_eq(&self, _other: &Self) -> bool {
        // TODO: Implement this by comparing token trees
        unimplemented!()
    }

    fn add_attr(&mut self, _attr: Attribute) {
        // TODO: Maybe implement this by not considering the higher-level items equal?
        unimplemented!()
    }
}

macro_rules! impl_merge_enum {
    (
        $ty:ident {
            $($variant:ident),* $(,)?
            $(_ $comma:tt)?
        }
    ) => {
        impl Merge for syn::$ty {
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
        impl Merge for syn::$ty {
            fn top_level_eq(&self, other: &Self) -> bool {
                // Ensure we've named all fields
                let syn::$ty {
                    attrs: _,
                    $($field : _,)*
                    $($($recursed : _,)*)?
                } = self;

                true $(&& self.$field == other.$field)*
            }

            fn add_attr(&mut self, attr: Attribute) {
                self.attrs.push(attr);
            }
        }
    };
}

impl_merge_struct! {
    ItemConst (expr) {
        vis,
        const_token,
        ident,
        generics,
        colon_token,
        ty,
        eq_token,
        semi_token,
    }
}

impl_merge_struct! {
    ItemEnum (variants) {
        vis,
        enum_token,
        ident,
        generics,
        brace_token,
    }
}

impl_merge_struct! {
    ItemExternCrate {
        vis,
        extern_token,
        crate_token,
        ident,
        rename,
        semi_token,
    }
}

impl_merge_struct! {
    ItemFn (block) {
        vis,
        sig,
    }
}

impl_merge_struct! {
    ItemForeignMod (items) {
        unsafety,
        abi,
        brace_token,
    }
}

impl_merge_struct! {
    ItemImpl (items) {
        defaultness,
        unsafety,
        impl_token,
        generics,
        trait_,
        self_ty,
        brace_token,
    }
}

impl_merge_struct! {
    ItemMacro {
        ident,
        mac,
        semi_token,
    }
}

impl_merge_struct! {
    ItemMod (content) {
        vis,
        unsafety,
        mod_token,
        ident,
        semi,
    }
}

impl_merge_struct! {
    ItemStatic (expr) {
        vis,
        static_token,
        mutability,
        ident,
        colon_token,
        ty,
        eq_token,
        semi_token,
    }
}

impl_merge_struct! {
    ItemStruct (fields) {
        vis,
        struct_token,
        ident,
        generics,
        semi_token,
    }
}

impl_merge_struct! {
    ItemTrait (items) {
        vis,
        unsafety,
        auto_token,
        restriction,
        trait_token,
        ident,
        generics,
        colon_token,
        supertraits,
        brace_token,
    }
}

impl_merge_struct! {
    ItemTraitAlias {
        vis,
        trait_token,
        ident,
        generics,
        eq_token,
        bounds,
        semi_token,
    }
}

impl_merge_struct! {
    ItemType {
        vis,
        type_token,
        ident,
        generics,
        eq_token,
        ty,
        semi_token,
    }
}

impl_merge_struct! {
    ItemUnion (fields) {
        vis,
        union_token,
        ident,
        generics,
    }
}

impl_merge_struct! {
    ItemUse {
        vis,
        use_token,
        leading_colon,
        tree,
        semi_token,
    }
}

impl_merge_enum! {
    Item {
        Const,
        Enum,
        ExternCrate,
        Fn,
        ForeignMod,
        Impl,
        Macro,
        Mod,
        Static,
        Struct,
        Trait,
        TraitAlias,
        Type,
        Union,
        Use,
        Verbatim,
        _,
    }
}

// TODO: Implement properly for the below types

impl Merge for Variant {
    fn add_attr(&mut self, attr: Attribute) {
        self.attrs.push(attr)
    }
}

impl Merge for ForeignItem {
    fn add_attr(&mut self, attr: Attribute) {
        match self {
            ForeignItem::Fn(item) => item.attrs.push(attr),
            ForeignItem::Static(item) => item.attrs.push(attr),
            ForeignItem::Type(item) => item.attrs.push(attr),
            ForeignItem::Macro(item) => item.attrs.push(attr),
            ForeignItem::Verbatim(_) => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

impl Merge for ImplItem {
    fn add_attr(&mut self, attr: Attribute) {
        match self {
            ImplItem::Const(item) => item.attrs.push(attr),
            ImplItem::Fn(item) => item.attrs.push(attr),
            ImplItem::Type(item) => item.attrs.push(attr),
            ImplItem::Macro(item) => item.attrs.push(attr),
            ImplItem::Verbatim(_) => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

impl Merge for Field {
    fn add_attr(&mut self, attr: Attribute) {
        self.attrs.push(attr)
    }
}

impl Merge for Stmt {
    fn add_attr(&mut self, attr: Attribute) {
        match self {
            Stmt::Local(local) => local.attrs.push(attr),
            Stmt::Item(item) => item.add_attr(attr),
            Stmt::Expr(expr, _) => expr.add_attr(attr),
            Stmt::Macro(macro_) => macro_.attrs.push(attr),
        }
    }
}

impl Merge for Expr {
    fn add_attr(&mut self, attr: Attribute) {
        match self {
            Expr::Array(expr) => expr.attrs.push(attr),
            Expr::Assign(expr) => expr.attrs.push(attr),
            Expr::Async(expr) => expr.attrs.push(attr),
            Expr::Await(expr) => expr.attrs.push(attr),
            Expr::Binary(expr) => expr.attrs.push(attr),
            Expr::Block(expr) => expr.attrs.push(attr),
            Expr::Break(expr) => expr.attrs.push(attr),
            Expr::Call(expr) => expr.attrs.push(attr),
            Expr::Cast(expr) => expr.attrs.push(attr),
            Expr::Closure(expr) => expr.attrs.push(attr),
            Expr::Const(expr) => expr.attrs.push(attr),
            Expr::Continue(expr) => expr.attrs.push(attr),
            Expr::Field(expr) => expr.attrs.push(attr),
            Expr::ForLoop(expr) => expr.attrs.push(attr),
            Expr::Group(expr) => expr.attrs.push(attr),
            Expr::If(expr) => expr.attrs.push(attr),
            Expr::Index(expr) => expr.attrs.push(attr),
            Expr::Infer(expr) => expr.attrs.push(attr),
            Expr::Let(expr) => expr.attrs.push(attr),
            Expr::Lit(expr) => expr.attrs.push(attr),
            Expr::Loop(expr) => expr.attrs.push(attr),
            Expr::Macro(expr) => expr.attrs.push(attr),
            Expr::Match(expr) => expr.attrs.push(attr),
            Expr::MethodCall(expr) => expr.attrs.push(attr),
            Expr::Paren(expr) => expr.attrs.push(attr),
            Expr::Path(expr) => expr.attrs.push(attr),
            Expr::Range(expr) => expr.attrs.push(attr),
            Expr::Reference(expr) => expr.attrs.push(attr),
            Expr::Repeat(expr) => expr.attrs.push(attr),
            Expr::Return(expr) => expr.attrs.push(attr),
            Expr::Struct(expr) => expr.attrs.push(attr),
            Expr::Try(expr) => expr.attrs.push(attr),
            Expr::TryBlock(expr) => expr.attrs.push(attr),
            Expr::Tuple(expr) => expr.attrs.push(attr),
            Expr::Unary(expr) => expr.attrs.push(attr),
            Expr::Unsafe(expr) => expr.attrs.push(attr),
            Expr::While(expr) => expr.attrs.push(attr),
            Expr::Yield(expr) => expr.attrs.push(attr),
            Expr::Verbatim(_) => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}
