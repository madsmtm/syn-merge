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

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

// Index into top-level structure.
#[derive(Debug, Clone)]
pub struct AppearsIn(Vec<usize>);

impl AppearsIn {
    pub fn get(&self) -> &[usize] {
        &self.0
    }

    pub fn new(idx: usize) -> Self {
        Self(vec![idx])
    }

    pub fn add(self, idx: usize) -> Self {
        let mut vec = self.0;
        vec.push(idx);
        Self(vec)
    }

    pub fn contains(&self, idx: usize) -> bool {
        self.0.contains(&idx)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub struct Chunk<'a, T> {
    pub value: &'a T,
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

pub fn multidiff<'a, T: PartialEq>(to_diff: &[&'a [T]]) -> Vec<(&'a T, AppearsIn)> {
    let mut iter = to_diff.into_iter().enumerate();
    let Some((idx, first)) = iter.next() else {
        return vec![];
    };

    let mut current: Vec<_> = first
        .iter()
        .map(|value| (value, AppearsIn::new(idx)))
        .collect();

    for (idx, &new) in iter {
        enum DiffHelper<'a, T> {
            Current(&'a (&'a T, AppearsIn)),
            New(&'a T),
        }

        impl<T: PartialEq> PartialEq for DiffHelper<'_, T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::Current(current), Self::New(new)) => current.0 == *new,
                    (Self::New(new), Self::Current(current)) => current.0 == *new,
                    _ => unimplemented!(),
                }
            }
        }

        enum Op {
            /// Present in both.
            Both,
            /// Present in `current`, not in `new`
            InCurrent,
            /// Present in `new`, not in `current`
            InNew,
        }

        let current_tmp = current.iter().map(DiffHelper::Current).collect::<Vec<_>>();
        let new_tmp = new.iter().map(DiffHelper::New).collect::<Vec<_>>();
        let ops: Vec<_> =
            diff::slice(&current_tmp, &new_tmp)
                .into_iter()
                .map(|op| match op {
                    diff::Result::Both(_, _) => Op::Both,
                    diff::Result::Left(_) => Op::InCurrent,
                    diff::Result::Right(_) => Op::InNew,
                })
                .collect();

        let mut current_iter = current.into_iter();
        let mut new_iter = new.into_iter();
        let next_current = ops
            .iter()
            .map(|op| match op {
                Op::Both => {
                    let (current_value, appears_in) = current_iter.next().unwrap();
                    let new_value = new_iter.next().unwrap();

                    debug_assert!(current_value == new_value);

                    (current_value, appears_in.add(idx))
                }
                Op::InCurrent => {
                    let (current_value, appears_in) = current_iter.next().unwrap();

                    (current_value, appears_in)
                }
                Op::InNew => {
                    let new_value = new_iter.next().unwrap();

                    (new_value, AppearsIn::new(idx))
                }
            })
            .collect();

        current = next_current;
    }

    debug_assert!(current.len() <= to_diff.iter().map(|slice| slice.len()).sum());
    debug_assert!(to_diff.iter().all(|slice| slice.len() <= current.len()));

    current
}

pub fn multidiff_indexes<'a, T: PartialEq>(to_diff: &[&'a [T]]) -> Vec<Vec<Option<usize>>> {
    let mut current_indexes: Vec<usize> = to_diff.iter().map(|_| 0).collect();
    multidiff(to_diff)
        .into_iter()
        .map(|(_, appears_in)| {
            current_indexes
                .iter_mut()
                .enumerate()
                .map(|(i, idx)| {
                    if appears_in.contains(i) {
                        let res = Some(*idx);
                        *idx += 1;
                        res
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

// pub fn multidiff_slice<'a, T: PartialEq + 'a>(
//     to_diff: &'a [&'a [T]],
// ) -> impl Iterator<Item = Chunk<'a, T>> {
//     multidiff(to_diff.iter().map(AsRef::as_ref))
// }
