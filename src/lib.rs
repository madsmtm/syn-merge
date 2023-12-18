//! # Merge `syn` structures by adding `cfg`s
// #![feature(non_exhaustive_omitted_patterns_lint)]
// #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use quote::format_ident;
use similar::algorithms::{lcs, DiffHook};
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

struct Differ<'a> {
    combined: Vec<Item>,
    old: &'a [Item],
    cfg_old: &'a Cfgs,
    new: &'a [Item],
    cfg_new: &'a Cfgs,
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

impl DiffHook for Differ<'_> {
    type Error = Error;

    fn equal(&mut self, old_index: usize, new_index: usize, len: usize) -> Result<(), Self::Error> {
        eprintln!("equal {}/{}/{}", old_index, new_index, len);
        self.combined
            .extend(self.old[old_index..(old_index + len)].iter().cloned());
        Ok(())
    }

    fn delete(
        &mut self,
        old_index: usize,
        old_len: usize,
        new_index: usize,
    ) -> Result<(), Self::Error> {
        eprintln!("delete {}/{}/{}", old_index, old_len, new_index);
        for mut item in self.old[old_index..(old_index + old_len)].iter().cloned() {
            item.add_attr(self.cfg_old.attribute())?;
            self.combined.push(item);
        }
        Ok(())
    }

    fn insert(
        &mut self,
        old_index: usize,
        new_index: usize,
        new_len: usize,
    ) -> Result<(), Self::Error> {
        eprintln!("insert {}/{}/{}", old_index, new_index, new_len);
        for mut item in self.new[new_index..(new_index + new_len)].iter().cloned() {
            item.add_attr(self.cfg_new.attribute())?;
            self.combined.push(item);
        }
        Ok(())
    }
}

pub fn merge_files(files: &[(File, Cfgs)]) -> Result<File, Error> {
    // Rough outline:
    // 1. Prepare the input for diffing (including preparing for nested diffing).
    // 2. Diff recursively, and write cfgs to output directly.
    // 3. Do diffing for each file.
    // 4. Somehow merge redundant cfgs?
    assert_eq!(files.len(), 2, "todo");
    let old = &files[0].0.items;
    let new = &files[1].0.items;

    let mut d = Differ {
        combined: vec![],
        old,
        cfg_old: &files[0].1,
        new,
        cfg_new: &files[1].1,
    };
    lcs::diff(&mut d, old, 0..old.len(), new, 0..new.len())?;
    Ok(File {
        shebang: files[0].0.shebang.clone(),
        attrs: files[0].0.attrs.clone(),
        items: d.combined,
    })
}
