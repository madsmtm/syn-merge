//! # Diff an arbitary number of inputs
//!
//! Fundamentally, it works by constructing an input matrix like:
//!
//! | Key | Data |
//! |-----|------|
//! | 0   | aab  |
//! | 1   | baa  |
//! | 2   | bc   |
//! | 3   | abc  |
//!
//! And producing an output matrix like (may vary depending on algorithm details):
//!
//! | Char | Appears In |
//! |------|------------|
//! | a    | 0, 3       |
//! | a    | 0          |
//! | b    | 0, 1, 2, 3 |
//! | a    | 1          |
//! | a    | 1          |
//! | c    | 2, 3       |
//!
//! Note how this can encode the same information as a diff between just two elements:
//! | Key | Data  |
//! |-----|-------|
//! | old | aabc  |
//! | new | baac  |
//!
//! | Char | Appears In |
//! |------|------------|
//! | b    | new        |
//! | a    | old, new   |
//! | a    | old, new   |
//! | b    | old        |
//! | c    | old, new   |
//!
//! ```diff
//! +b
//!  a
//!  a
//! -b
//!  c
//! ```

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use similar::algorithms::{lcs, DiffHook};

#[derive(Debug)]
pub struct Chunk<'a, T> {
    pub value: &'a T,
    pub appears_in: Vec<usize>,
}

// TODO: Unsure if Rust 2024 lifetime capture rules affect this crate?
// pub trait Captures<U> {}
// impl<T: ?Sized, U> Captures<U> for T {}
// + Captures<&'a ()>

pub fn multidiff<'a, T: PartialEq + 'a>(
    to_diff: impl IntoIterator<Item = &'a [T]>,
) -> impl Iterator<Item = Chunk<'a, T>> {
    let mut iter = to_diff.into_iter();
    let Some(first) = iter.next() else {
        return vec![].into_iter();
    };

    let old = first;
    let new = iter.next().unwrap();
    let mut d = Differ { old, new };
    lcs::diff(&mut d, old, 0..old.len(), new, 0..new.len()).unwrap();

    vec![todo!()].into_iter()
}

// pub fn multidiff_slice<'a, T: PartialEq + 'a>(
//     to_diff: &'a [&'a [T]],
// ) -> impl Iterator<Item = Chunk<'a, T>> {
//     multidiff(to_diff.iter().map(AsRef::as_ref))
// }

struct Differ<'a, T> {
    old: &'a [T],
    new: &'a [T],
}

impl<T> DiffHook for Differ<'_, T> {
    type Error = ();

    fn equal(&mut self, old_index: usize, new_index: usize, len: usize) -> Result<(), Self::Error> {
        eprintln!("equal {}/{}/{}", old_index, new_index, len);
        // self.combined.extend(self.old[old_index..(old_index + len)].iter().cloned());
        Ok(())
    }

    fn delete(
        &mut self,
        old_index: usize,
        old_len: usize,
        new_index: usize,
    ) -> Result<(), Self::Error> {
        eprintln!("delete {}/{}/{}", old_index, old_len, new_index);
        for mut item in self.old[old_index..(old_index + old_len)].iter() {
            // self.combined.push(item);
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
        for mut item in self.new[new_index..(new_index + new_len)].iter() {
            // self.combined.push(item);
        }
        Ok(())
    }
}
