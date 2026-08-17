# Contributing to glam

Thanks for contributing to `glam`! These guidelines will try to make the
process painless and efficient.

The short guide to contributing is [start a discussion] on GitHub.  Pull
requests are welcome for bug fixes, documentation improvements and
optimizations. For anything else it would be best to discuss it first.

## Questions

If you have a question about the usage of this library please [ask a question]
with GitHub Discussions. That's the easiest way to get support right now.

## Bugs

If you find a bug please [open an issue] on GitHub or submit a pull request. A
unit test for any bug that slipped through existing coverage would also be
greatly appreciated.

## New functions and methods

If `glam` is missing functionality on existing types, [suggest a new feature]
with GitHub Discussions describing what feature you would like added and
ideally what your use case is for it just so I have a better understanding of
the feature. I'd like to keep `glam` reasonably light functionality wise
initially but commonly used functionality that is missing is very welcome. If
you do submit a pull request please ensure any new functionality also has a
test.

## Optimizations

If you feel some functionality could be optimized please [open an issue] on
GitHub or submit a pull request. Any optimization pull request should include a
benchmark if there isn't one already, so I can confirm the performance
improvement.

## Documentation

If you feel any documentation could be added or improved please
[open a GitHub issue] or submit a pull request.

## Code contributions

Most of `glam`'s source code is generated. See the [codegen README] on how
to modify the code templates and generate new source code.

You can run some of `glam`'s test suite locally by running the
`build_and_test_features.sh` script. It's worth running that before creating a
PR.

Also run `cargo fmt` and `cargo clippy` on any new code.

[start a discussion]: https://github.com/bitshifter/glam-rs/discussions/new
[open an issue]: https://GitHub.com/bitshifter/glam-rs/issues/new
[ask a question]: https://github.com/bitshifter/glam-rs/discussions/new?category=q-a
[suggest a new feature]: https://github.com/bitshifter/glam-rs/discussions/new?category=ideas
[ARCHITECTURE]: ARCHITECTURE.md
[codegen README]: codegen/README.md
