# Contributor's Guide

Thank you for helping with SNAFU!

This document has some guidelines and tips that can help you make a contribution.
Feel free to make a pull request to this file, too, if you learn anything during your contribution that can help others.

## Code of Conduct

This project is governed by the [Code of Conduct](https://github.com/shepmaster/snafu/blob/master/CODE_OF_CONDUCT.md).
Please understand those guidelines, and report violations to @shepmaster.

## Getting Started

If you're looking for a way to contribute - first of all, thanks!
Here are some ideas:

* Issues that we need help on are tagged [help wanted](https://github.com/shepmaster/snafu/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
* Issues good for beginners are tagged [good first issue](https://github.com/shepmaster/snafu/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
* Anything that's not clear to you in the documentation, particularly in the [user's guide](https://docs.rs/snafu/latest/snafu/guide/index.html)

## Communication tips

* Open an issue for discussion before writing any non-trivial changes.  Often the author or other contributors can help shape an even better idea.  Or maybe someone is already working on it!
* Even if you're making a breaking change, don't worry about updating the version number or changelog.  They're done together before a release.  Feel free to suggest some wording you like in the pull request, though.
* We value correctness and clarity in the code, API, and docs, and it's worth putting in the time for thorough review in issues and pull requests.
* Don't try to fix the world in a single issue or pull request.  Even small issues can sprout many good ideas, and feel free to split those into new issues.

## Testing tips

* We maintain compatibility with older versions of Rust, and this is enforced through compatibility testing that runs automatically when you create or update a pull request.  You can run these earlier, locally, by running `cargo test` in one of the directories under `compatibility-tests/`.  The `rust-toolchain` files there will cause the right version of Rust to be used, assuming you use rustup.  The `Cargo.toml` files there will make sure that compatible dependency versions are used, too.
* If you're adding a new compile-time error, add a sample under `compatibility-tests/compile-fail/tests/ui/` to be sure it fails in the way you expect.
* If you're adding a feature, please add a test for it.  This helps show your intent, and makes sure others don't accidentally break the feature.
   * Because the majority of SNAFU code lives in snafu-derive and deals with procedural macros, integration tests are often simpler than unit tests.  They live under `tests/`.  Add to the file that sounds most relevant, or create a new one if necessary.
   * Unit tests are still great when you're working on something that doesn't need to parse Rust source.  They follow standard unit testing practice in Rust - a `#[test]` function in a `tests` module at the bottom of the relevant source module.

## General tips

* Breaking changes (changes in SNAFU's interface) are OK if they're adding value.
  * Before 1.0, this may happen relatively frequently, and will result in new minor versions.
  * After 1.0, this should be relatively rare, but new major versions are OK with good reasons.
* If you're making a code change, please run the code through rustfmt (`cargo fmt`) and check it with clippy (`cargo clippy`).
* The user's guide is a valuable resource!  It's worth the time to keep up to date when adding or changing the library.
