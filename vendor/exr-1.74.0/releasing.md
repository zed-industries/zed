# Release Checklist
Yanking shouldn't be the default.

## Safety Checklist
1. No `unsafe`
1. Everything builds, not only `cargo test`
1. Builds with `--release` flag
1. All tests pass, all benchmarks run
1. All tests pass, including `#[ignore]` tests and fuzzing!
1. Images produced by running the examples can be opened in other software
1. Only safe `as` casts
1. Always have a max limit when allocating based on file contents
    - limit max capacity `Vec::with_capacity( x.min(1024) )`
    - careful with `vec![ 0; x ]`
1. Only unreachable `unwrap()`, `expect("")` and `assert`s
1. No `println!` outside of tests and examples
1. `assert_eq` and `debug_assert_eq` should have a message explaining the context, except in internal algorithms like compressors
1. ensure `#![warn(missing_docs)]` in `lib.rs`
1. Example in README.md should be up-to-date
1. GUIDE.md should be up-to-date
1. Update Dependencies while you're at it?

## Tasks
1. Bump version in
    - `cargo.toml`
    - `README.md`
    - `examples/README.md`
    
1. Run `cargo publish`
    