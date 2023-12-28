//! # Merge `syn` structures by adding `cfg`s
// #![feature(non_exhaustive_omitted_patterns_lint)]
// #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use multidiff::DiffableSequence;
use quote::format_ident;
use std::any::Any;
use std::fmt;
use syn::{parse_quote, Attribute, File};

mod syn_impl;
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

pub trait Merge: Clone {
    fn top_level_eq(&self, _other: &Self) -> bool {
        todo!()
    }

    fn add_attr(&mut self, attr: Attribute);
}

#[derive(Debug)]
struct WithCfgs<'a, T> {
    values: &'a [T],
    cfgs: &'a Cfgs,
}

impl<'a, T: Merge> DiffableSequence for WithCfgs<'a, T> {
    type Item = &'a T;

    fn eq(a: &Self::Item, b: &Self::Item) -> bool {
        a.top_level_eq(b)
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
