//! # Merge `syn` structures by adding `cfg`s
// #![feature(non_exhaustive_omitted_patterns_lint)]
// #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use quote::format_ident;
use std::any::Any;
use std::fmt;
use syn::{parse_quote, Attribute, Expr, Field, File, ForeignItem, ImplItem, Item, Stmt, Variant};

#[cfg(test)]
mod tests;

/// Errors on:
/// - Differing shebangs.
/// - No input files.
/// - Unknown items (since we can't know for sure how to `cfg`-guard them).
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Error {
    inner: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Error {}

/// Should support:
/// - User-specified configs (e.g. flag `MYCRATE_XYZ` -> `feature = "xyz"`).
/// - Automatically generated `rustc --print cfg`.
#[derive(Debug)]
pub struct Cfgs {
    s: String,
}

impl Cfgs {
    pub fn new(s: &str) -> Self {
        Self { s: s.into() }
    }

    pub fn attribute(&self) -> Attribute {
        let ident = format_ident!("{}", self.s);
        parse_quote! {
            #[cfg(#ident)]
        }
    }
}

// Places where we need to recurse:
// ItemConst::expr
// ItemEnum::variants
// ItemFn::block
// ItemForeignMod::items
// ItemImpl::items
// - ImplItemConst::expr
// - ImplItemFn::block
// ItemMod::content
// ItemStatic::expr
// ItemStruct::fields
// ItemTrait::items
// - TraitItemConst::default
// - TraitItemFn::default
// ItemUnion::fields
//
// ... expressions

// Can't handle macro invocations? Maybe we can, if we assume it's valid items/statements (depending on context)?
// ItemMacro::mac
// ForeignItemMacro::mac
// ImplItemMacro::mac

// https://stackoverflow.com/a/25359060
trait MyPartialEq {
    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;

    // Perform the test.
    fn equals_a(&self, _: &dyn MyPartialEq) -> bool;
}

// Implement A for all 'static types implementing PartialEq.
impl<T: 'static + PartialEq> MyPartialEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals_a(&self, other: &dyn MyPartialEq) -> bool {
        other
            .as_any()
            .downcast_ref::<T>()
            .map_or(false, |a| self == a)
    }
}

trait AddAttr {
    fn add_attr(&mut self, attr: Attribute) -> Result<(), Error>;
}

impl AddAttr for Variant {
    fn add_attr(&mut self, attr: Attribute) -> Result<(), Error> {
        Ok(self.attrs.push(attr))
    }
}

impl AddAttr for Item {
    fn add_attr(&mut self, attr: Attribute) -> Result<(), Error> {
        Ok(match self {
            Item::Const(item) => item.attrs.push(attr),
            Item::Enum(item) => item.attrs.push(attr),
            Item::ExternCrate(item) => item.attrs.push(attr),
            Item::Fn(item) => item.attrs.push(attr),
            Item::ForeignMod(item) => item.attrs.push(attr),
            Item::Impl(item) => item.attrs.push(attr),
            Item::Macro(item) => item.attrs.push(attr),
            Item::Mod(item) => item.attrs.push(attr),
            Item::Static(item) => item.attrs.push(attr),
            Item::Struct(item) => item.attrs.push(attr),
            Item::Trait(item) => item.attrs.push(attr),
            Item::TraitAlias(item) => item.attrs.push(attr),
            Item::Type(item) => item.attrs.push(attr),
            Item::Union(item) => item.attrs.push(attr),
            Item::Use(item) => item.attrs.push(attr),
            Item::Verbatim(_) => unimplemented!(),
            _ => unimplemented!(),
        })
    }
}

impl AddAttr for ForeignItem {
    fn add_attr(&mut self, attr: Attribute) -> Result<(), Error> {
        Ok(match self {
            ForeignItem::Fn(item) => item.attrs.push(attr),
            ForeignItem::Static(item) => item.attrs.push(attr),
            ForeignItem::Type(item) => item.attrs.push(attr),
            ForeignItem::Macro(item) => item.attrs.push(attr),
            ForeignItem::Verbatim(_) => unimplemented!(),
            _ => unimplemented!(),
        })
    }
}

impl AddAttr for ImplItem {
    fn add_attr(&mut self, attr: Attribute) -> Result<(), Error> {
        Ok(match self {
            ImplItem::Const(item) => item.attrs.push(attr),
            ImplItem::Fn(item) => item.attrs.push(attr),
            ImplItem::Type(item) => item.attrs.push(attr),
            ImplItem::Macro(item) => item.attrs.push(attr),
            ImplItem::Verbatim(_) => unimplemented!(),
            _ => unimplemented!(),
        })
    }
}

impl AddAttr for Field {
    fn add_attr(&mut self, attr: Attribute) -> Result<(), Error> {
        Ok(self.attrs.push(attr))
    }
}

impl AddAttr for Stmt {
    fn add_attr(&mut self, attr: Attribute) -> Result<(), Error> {
        match self {
            Stmt::Local(local) => Ok(local.attrs.push(attr)),
            Stmt::Item(item) => item.add_attr(attr),
            Stmt::Expr(expr, _) => expr.add_attr(attr),
            Stmt::Macro(macro_) => Ok(macro_.attrs.push(attr)),
        }
    }
}

impl AddAttr for Expr {
    fn add_attr(&mut self, attr: Attribute) -> Result<(), Error> {
        Ok(match self {
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
        })
    }
}

/// The order in which the files are passed influences the output.
pub fn merge_files(files: &[(File, Cfgs)]) -> Result<File, Error> {
    let tmp_files: Vec<_> = files.iter().map(|(file, _)| &*file.items).collect();
    let combined: Vec<_> = multidiff::multidiff(&tmp_files)
        .into_iter()
        .map(|(item, appears_in)| {
            // If it appears in all, just output the item
            if appears_in.len() == files.len() {
                item.clone()
            } else {
                let mut item = item.clone();
                for &idx in appears_in.get() {
                    item.add_attr(files[idx].1.attribute()).unwrap();
                }
                item
            }
        })
        .collect();

    Ok(File {
        shebang: files[0].0.shebang.clone(),
        // TODO: Merge attributes
        attrs: files[0].0.attrs.clone(),
        items: combined,
    })
}
