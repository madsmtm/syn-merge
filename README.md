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
