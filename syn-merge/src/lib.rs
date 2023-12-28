//! # Merge `syn` structures by adding `cfg`s
// #![feature(non_exhaustive_omitted_patterns_lint)]
// #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use multidiff::DiffableSequence;
use proc_macro2::TokenStream;
use quote::format_ident;
use std::fmt;
use syn::{parse_quote, punctuated::Punctuated, Attribute, File};

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

pub trait Merge: Clone + Sized {
    fn top_level_eq(&self, _other: &Self) -> bool {
        todo!()
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        // TODO: Somehow merge the items here
        for (item, _cfgs) in iter {
            return item.clone();
        }
        unreachable!()
    }

    fn add_attr(&mut self, attr: Attribute);
}

impl<T: Merge> Merge for Box<T> {
    fn top_level_eq(&self, other: &Self) -> bool {
        (**self).top_level_eq(other)
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        Box::new(T::merge(iter.into_iter().map(|(t, cfgs)| (&**t, cfgs))))
    }

    fn add_attr(&mut self, attr: Attribute) {
        (**self).add_attr(attr)
    }
}

// TODO: Implement this properly
impl Merge for Attribute {
    fn top_level_eq(&self, other: &Self) -> bool {
        self == other
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        parse_quote! {
            #[cfg(todo)]
        }
    }

    fn add_attr(&mut self, attr: Attribute) {
        todo!()
    }
}

// TODO: Implement this properly
impl<T: Merge> Merge for Vec<T> {
    fn top_level_eq(&self, other: &Self) -> bool {
        true
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        let tmp: Vec<_> = iter
            .into_iter()
            .map(|(values, cfgs)| WithCfgs {
                values: &**values,
                cfgs,
            })
            .collect();
        merge_recursively(&tmp)
    }

    fn add_attr(&mut self, attr: Attribute) {
        for item in self {
            item.add_attr(attr.clone());
        }
    }
}

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

// TODO: Implement this properly
impl<T: Merge, P: PartialEq + Clone> Merge for Punctuated<T, P> {
    fn top_level_eq(&self, _other: &Self) -> bool {
        true
    }

    fn add_attr(&mut self, attr: Attribute) {
        for item in self {
            item.add_attr(attr.clone());
        }
    }
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

pub(crate) fn merge_recursively<T: Merge>(input: &[WithCfgs<'_, T>]) -> Vec<T> {
    multidiff::multidiff_indexes(input)
        .into_iter()
        .map(|indexes| {
            let iter = indexes.iter().zip(input).filter_map(
                |(idx, with_cfgs)| idx.map(|idx| (&with_cfgs.values[idx], with_cfgs.cfgs))
            );

            let cfgs: Vec<_> = iter.clone().map(|(_, cfgs)| cfgs).collect();

            let mut t = T::merge(iter);

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
