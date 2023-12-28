//! # Diff a number of inputs
//!
//! Fundamentally, it works by taking an input like:
//!
//! | Key | Data |
//! |-----|------|
//! | 0   | aab  |
//! | 1   | baa  |
//! | 2   | bc   |
//! | 3   | abc  |
//!
//! And producing an output like (exact output may vary depending on algorithm details):
//!
//! | Value | Appears in | Idx key 0 | Idx key 1 | Idx key 2 | Idx key 3 |
//! |-------|------------|-----------|-----------|-----------|-----------|
//! | a     | 0, 3       | Some(0)   | None      | None      | Some(0)   |
//! | a     | 0          | Some(1)   | None      | None      | None      |
//! | b     | 0, 1, 2, 3 | Some(2)   | Some(0)   | Some(0)   | Some(1)   |
//! | a     | 1          | None      | Some(1)   | None      | None      |
//! | a     | 1          | None      | Some(2)   | None      | None      |
//! | c     | 2, 3       | None      | None      | Some(1)   | Some(2)   |
//!
//! Note how this can encode the same information as a diff between just two elements:
//!
//! | Key | Data  |
//! |-----|-------|
//! | old | aabc  |
//! | new | baac  |
//!
//! | Char | Appears in | Idx old | Idx new |
//! |------|------------|---------|---------|
//! | b    | new        | None    | Some(0) |
//! | a    | old, new   | Some(0) | Some(1) |
//! | a    | old, new   | Some(1) | Some(2) |
//! | b    | old        | Some(2) | None    |
//! | c    | old, new   | Some(3) | Some(3) |
//!
//! ```diff
//! +b
//!  a
//!  a
//! -b
//!  c
//! ```
#![allow(unused)]

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use std::{fmt::Debug, mem};

use similar::{
    algorithms::{lcs, Capture},
    DiffOp,
};

#[derive(Debug)]
pub struct Chunk<'a, T> {
    pub value: &'a T,
    // Values index into top-level structure.
    pub appears_in: Vec<usize>,
}

impl<T> Clone for Chunk<'_, T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            appears_in: self.appears_in.clone(),
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
struct Wrapper<T>(T);

impl<T: PartialEq> PartialEq<Chunk<'_, T>> for Wrapper<T> {
    fn eq(&self, other: &Chunk<'_, T>) -> bool {
        &self.0 == other.value
    }
}

pub fn multidiff<'a, T: PartialEq + Debug>(to_diff: &[&'a [T]]) -> Vec<Chunk<'a, T>> {
    let mut iter = to_diff.into_iter().enumerate();
    let Some((_, first)) = iter.next() else {
        return vec![];
    };

    let mut current: Vec<_> = first
        .iter()
        .map(|item| Chunk {
            value: item,
            appears_in: vec![0],
        })
        .collect();

    for (i, &new) in iter {
        // SAFETY: `Wrapper<T>` is a `repr(transparent)` newtype over `T`.
        let new = unsafe { mem::transmute::<&'a [T], &'a [Wrapper<T>]>(new) };
        let mut new_current = vec![];

        if cfg!(feature = "use-similar") {
            let ops =
                {
                    let mut d = Capture::new();
                    lcs::diff(&mut d, &current, 0..current.len(), new, 0..new.len()).unwrap();
                    d.into_ops()
                };

            for op in ops {
                match op {
                    // Present in both
                    DiffOp::Equal {
                        old_index: current_index,
                        new_index,
                        len,
                    } => {
                        // eprintln!(
                        //     "equal: {new_index}/{}, current: {current_index}/{}",
                        //     new_index + len,
                        //     current_index + len
                        // );
                        new_current.extend(
                            current[current_index..(current_index + len)]
                                .iter()
                                .cloned()
                                .map(|mut item| {
                                    item.appears_in.push(i);
                                    item
                                }),
                        );
                    }
                    // Present in `current`, not in `new`
                    DiffOp::Delete {
                        old_index: current_index,
                        old_len: current_len,
                        new_index,
                    } => {
                        let end = current_index + current_len;
                        // eprintln!("in current: {current_index}..{end}, new: {new_index}");
                        new_current.extend(current[current_index..end].iter().cloned());
                    }
                    // Present in `new`, not in `current`
                    DiffOp::Insert {
                        old_index: current_index,
                        new_index,
                        new_len,
                    } => {
                        let end = new_index + new_len;
                        // eprintln!("in new: {new_index}..{end}, current: {current_index}");
                        new_current.extend(new[new_index..end].iter().map(|item| Chunk {
                            value: &item.0,
                            appears_in: vec![i],
                        }));
                    }
                    DiffOp::Replace { .. } => unimplemented!(),
                }
            }
        } else {
            enum Hack<New, Old> {
                New(New),
                Old(Old),
            }

            impl<New, Old> PartialEq for Hack<New, Old>
            where
                New: PartialEq<Old>,
            {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (Self::Old(new), Self::New(old)) => old == new,
                        (Self::New(new), Self::Old(old)) => new == old,
                        _ => unimplemented!(),
                    }
                }
            }

            let current_tmp = current.iter().map(Hack::Old).collect::<Vec<_>>();
            let new_tmp = new.iter().map(Hack::New).collect::<Vec<_>>();
            let ops = diff::slice(&current_tmp, &new_tmp);

            let mut current_iter = current.iter();
            let mut new_iter = new.iter();
            for op in ops {
                match op {
                    // Present in both
                    diff::Result::Both(_, _) => {
                        let chunk = current_iter.next().unwrap();
                        let _ = new_iter.next().unwrap();
                        new_current.push(Chunk {
                            value: chunk.value,
                            appears_in: chunk.appears_in.iter().copied().chain(Some(i)).collect(),
                        });
                    }
                    // Present in `current`, not in `new`
                    diff::Result::Left(_) => {
                        let chunk = current_iter.next().unwrap();
                        new_current.push(chunk.clone());
                    }
                    // Present in `new`, not in `current`
                    diff::Result::Right(_) => {
                        let value = new_iter.next().unwrap();
                        new_current.push(Chunk {
                            value: &value.0,
                            appears_in: vec![i],
                        });
                    }
                }
            }
        }

        // dbg!(&new_current);

        current = new_current;
    }

    debug_assert!(current.len() <= to_diff.iter().map(|slice| slice.len()).sum());

    current
}

// pub fn multidiff_slice<'a, T: PartialEq + 'a>(
//     to_diff: &'a [&'a [T]],
// ) -> impl Iterator<Item = Chunk<'a, T>> {
//     multidiff(to_diff.iter().map(AsRef::as_ref))
// }
