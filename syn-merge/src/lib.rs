//! # Merge `syn` structures by adding `cfg`s
// #![feature(non_exhaustive_omitted_patterns_lint)]
// #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use multidiff::DiffableSequence;
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

pub trait Merge: Clone + PartialEq {
    fn add_attr(&mut self, attr: Attribute);
}

impl Merge for Item {
    fn add_attr(&mut self, attr: Attribute) {
        match self {
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
        }
    }
}

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

#[derive(Debug)]
struct WithCfgs<'a, T> {
    values: &'a [T],
    cfgs: &'a Cfgs,
}

impl<'a, T: PartialEq> DiffableSequence for WithCfgs<'a, T> {
    type Item = &'a T;

    fn eq(a: &Self::Item, b: &Self::Item) -> bool {
        a == b
    }

    fn get_iter(&self) -> impl Iterator<Item = Self::Item> {
        self.values.iter()
    }
}

fn merge<'a, T: Merge + 'a>(iter: impl Iterator<Item = (&'a T, &'a Cfgs)>) -> T {
    // TODO: Somehow merge the items here
    for (item, _cfgs) in iter {
        return item.clone();
    }
    unreachable!()
}

fn merge_recursively<T: Merge>(input: &[WithCfgs<'_, T>]) -> Vec<T> {
    multidiff::multidiff_indexes(input)
        .into_iter()
        .map(|indexes| {
            let iter = indexes.iter().zip(input).filter_map(
                |(idx, with_cfgs)| idx.map(|idx| (&with_cfgs.values[idx], with_cfgs.cfgs))
            );

            let cfgs: Vec<_> = iter.clone().map(|(_, cfgs)| cfgs).collect();

            let mut t = merge(iter);

            // If it appears in all, just output the item
            if cfgs.len() == input.len() {
                t
            } else {
                for cfgs in cfgs {
                    t.add_attr(cfgs.attribute());
                }
                t
            }
        })
        .collect()
}

/// The order in which the files are passed influences the output.
pub fn merge_files(input: &[(File, Cfgs)]) -> Result<File, Error> {
    let items: Vec<_> = input
        .iter()
        .map(|(file, cfgs)| WithCfgs {
            values: &file.items,
            cfgs,
        })
        .collect();

    let combined = merge_recursively(&items);

    Ok(File {
        shebang: input[0].0.shebang.clone(),
        // TODO: Merge attributes
        attrs: input[0].0.attrs.clone(),
        items: combined,
    })
}
