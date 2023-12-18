//! # Merge `syn` structures by adding `cfg`s

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use quote::format_ident;
use similar::algorithms::{lcs, DiffHook};
use std::fmt;
use syn::{parse_quote, Attribute, File, Item};

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

trait AddAttr {
    fn add_attr(&mut self, attr: Attribute);
}

impl AddAttr for Item {
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

impl DiffHook for Differ<'_> {
    type Error = Error;

    fn equal(&mut self, old_index: usize, new_index: usize, len: usize) -> Result<(), Self::Error> {
        eprintln!("equal {}/{}/{}", old_index, new_index, len);
        self.combined.push(self.old[old_index].clone());
        Ok(())
    }

    fn delete(
        &mut self,
        old_index: usize,
        new_index: usize,
        old_len: usize,
    ) -> Result<(), Self::Error> {
        eprintln!("delete {}/{}/{}", old_index, new_index, old_len);
        let mut item = self.old[old_index].clone();
        item.add_attr(self.cfg_old.attribute());
        self.combined.push(item);
        Ok(())
    }

    fn insert(
        &mut self,
        old_index: usize,
        new_index: usize,
        new_len: usize,
    ) -> Result<(), Self::Error> {
        eprintln!("insert {}/{}/{}", old_index, new_index, new_len);
        let mut item = self.new[new_index].clone();
        item.add_attr(self.cfg_new.attribute());
        self.combined.push(item);
        Ok(())
    }
}

pub fn merge_files(files: &[(File, Cfgs)]) -> Result<File, Error> {
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
