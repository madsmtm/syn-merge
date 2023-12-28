# `syn-merge`

[![Latest version](https://badgen.net/crates/v/syn-merge)](https://crates.io/crates/syn-merge)
[![Documentation](https://docs.rs/syn-merge/badge.svg)](https://docs.rs/syn-merge/)
[![CI](https://github.com/madsmtm/syn-merge/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/syn-merge/actions/workflows/ci.yml)

Merge `syn` structures by adding `cfg`s.

## Thoughts about diffing

Should operate directly on Rust source code (not on some custom AST).

Able to operate on multiple files (in contrast to most diffing out there).

Longest common subsequence algorithm?

How do we handle ties? Some kind of weighting?

Basically:

```rust
let contents = vec![...];
let files = contents.iter().map(syn::parse_file).flatten()?;
let merged = syn_merge::merge(files)?;
file.write(prettyplease::unparse(merged))?;
```

## Literature and prior work

The [`similar`](https://crates.io/crates/similar) crate implements various
diffing algorithms.

[`difftastic/src/diff`](https://github.com/Wilfred/difftastic/tree/26c58a25e86e944b60b75137b9c7a06bfc4f80ca/src/diff)
also has a few algorithms.

The [`wu-diff`](https://crates.io/crates/wu-diff) crate, which implements
[Wu, Sun; Manber, Udi; Myers, Gene (1989). "An O(NP) Sequence Comparison Algorithm"](https://publications.mpi-cbg.de/Wu_1990_6334.pdf).

[Hunt, James W; Szymanski, Thomas G. (1977). "A fast algorithm for computing longest common subsequences"](http://www.cs.ust.hk/mjg_lib/bibs/DPSu/DPSu.Files/HuSz77.pdf).

[Department of Mathematics and Computer Science. University of Southern Denmark (January 12, 2017). "The Hunt-Szymanski Algorithm for LCS"](https://imada.sdu.dk/~rolf/Edu/DM823/E16/HuntSzymanski.pdf).

---

However, there is very little research on diffing between multiple files.

[Khanna, Sanjeev & Kunal, Keshav & Pierce, Benjamin. (2007). A Formal Investigation of Diff3](https://www.cis.upenn.edu/~bcpierce/papers/diff3-short.pdf).

"Generalized Longest Common Subsequence / LCS"
