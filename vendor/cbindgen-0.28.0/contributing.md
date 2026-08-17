# Contributing

Thanks for wanting to contribute!

If you want help or mentorship, please file a GitHub issue and I'll be sure to provide guidance to the best of my ability.

Otherwise be sure to check out `internals.md` for an overview on the internals.

## Filing a pull request

Check out [Servo's GitHub workflow](https://github.com/servo/servo/wiki/Github-workflow) for an overview on creating a pull request.

Don't worry about requesting code review, as there is nothing formally setup for this repository. I try and review each pull request as soon as I can.

There is continuous integration setup for `cbindgen` using [GitHub Actions](https://github.com/mozilla/cbindgen/actions). It automatically runs `cargo test` which runs `cbindgen` against a series of Rust files from `tests/rust/` and checks that the output compiles using `gcc` or `g++`.

In addition to a C/C++ compiler `cargo test` requires Python and Cython
(`python -m pip install Cython`) for checking Cython bindings generated from tests (`.pyx` files).
Note that the tests will be failed with Cython 3.x or later.

Please run `cargo test` before filing a pull request to be sure that all tests pass. This will also update the test expectations.

Rustfmt is also enforced by GitHub Actions. To format your code install `rustfmt-preview` using `rustup component add rustfmt-preview` and then `cargo fmt`. GitHub Actions runs with Rust nightly, so use `rustup run nightly -- cargo fmt` to guarantee consistent results.

Writing new tests with your pull requests is also appreciated.
