//! These types usually do not implement `PartialEq`, so we have to do
//! comparisons manually ourselves.
//!
//! Note: We deliberately ignore spans here!
use proc_macro2::{Group, Ident, Literal, Punct, TokenStream, TokenTree};
use syn::Attribute;

use crate::merge_by_extracting_first;

use super::{Cfgs, Merge};

impl Merge for TokenStream {
    fn top_level_eq(&self, other: &Self) -> bool {
        let mut x_iter = self.clone().into_iter();
        let mut y_iter = other.clone().into_iter();
        loop {
            let Some(x) = x_iter.next() else {
                return y_iter.next().is_none();
            };
            let Some(y) = y_iter.next() else {
                return y_iter.next().is_none();
            };
            if !x.top_level_eq(&y) {
                return false;
            }
        }
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        merge_by_extracting_first(iter)
    }

    fn add_attr(&mut self, _attr: Attribute) {
        // TODO: Maybe implement this by not considering the higher-level items equal?
        unimplemented!()
    }
}

impl_merge_enum! {
    TokenTree {
        Group,
        Ident,
        Punct,
        Literal,
    }
}

impl Merge for Group {
    fn top_level_eq(&self, other: &Self) -> bool {
        self.delimiter() == other.delimiter() && self.stream().top_level_eq(&other.stream())
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        merge_by_extracting_first(iter)
    }
}

impl Merge for Ident {
    fn top_level_eq(&self, other: &Self) -> bool {
        self == other
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        merge_by_extracting_first(iter)
    }
}

impl Merge for Punct {
    fn top_level_eq(&self, other: &Self) -> bool {
        self.as_char() == other.as_char() && self.spacing() == other.spacing()
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        merge_by_extracting_first(iter)
    }
}

impl Merge for Literal {
    fn top_level_eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }

    fn merge<'a, I: IntoIterator<Item = (&'a Self, &'a Cfgs)>>(iter: I) -> Self
    where
        Self: 'a,
        I::IntoIter: Clone,
    {
        merge_by_extracting_first(iter)
    }
}
