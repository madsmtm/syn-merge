//! # Merge `syn` structures by adding `cfg`s

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

// use similar::algorithms::diff;
use std::fmt;

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

pub struct Cfgs {}

pub fn merge_files(files: &[(syn::File, Cfgs)]) -> Result<syn::File, Error> {
    todo!()
}
