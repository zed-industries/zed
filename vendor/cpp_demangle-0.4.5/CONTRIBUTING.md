# Contributing to `cpp_demangle`

Hi! We'd love to have your contributions! If you want help or mentorship, reach
out to us in a GitHub issue, or ping `fitzgen`
in [#rust on irc.mozilla.org](irc://irc.mozilla.org#rust) and introduce
yourself.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [Code of Conduct](#code-of-conduct)
- [Filing an Issue](#filing-an-issue)
- [Building](#building)
- [Testing](#testing)
  - [Testing `libiberty` Compatibility](#testing-libiberty-compatibility)
- [Debugging](#debugging)
- [Fuzzing with AFL](#fuzzing-with-afl)
- [Automatic code formatting](#automatic-code-formatting)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Code of Conduct

We abide by the [Rust Code of Conduct][coc] and ask that you do as well.

[coc]: https://www.rust-lang.org/en-US/conduct.html

## Filing an Issue

When filing an issue, please provide:

* The mangled C++ symbol name, and
* The way that `cpp_demangle` demangled it (or failed to)

`cpp_demangle` should *never* panic or crash. If you find some input that causes
a panic or crash, **please file an issue!**

## Building

```
$ cargo build
```

## Testing

To run all the tests:

```
$ cargo test
```

### Testing `libiberty` Compatibility

We currently have partial compatibility with the canonical GNU C++ demangler in
`libiberty`. Work towards full compatibility is ongoing. To run all of
`libiberty`'s tests (many of which are failing due to formatting differences and
malformatting on `cpp_demangle`'s part) you can enable the `run_libiberty_tests`
feature when testing:

```
$ cargo test --features run_libiberty_tests
```

As more `libiberty` tests start passing, we start including them in our test
suite by default. Help getting more of `libiberty`'s tests passing is very
appreciated! See `LIBIBERTY_TEST_THRESHOLD` in `build.rs` for details.

## Debugging

The `logging` feature adds debug logging that is very helpful when trying to
determine how a mangled symbol is parsed, and what its substitutions table looks
like:

```
$ cargo test --feature logging <some-test-you-are-debugging>
```

## Fuzzing

### Fuzzing with `cargo-fuzz` and `libFuzzer`

This is a bit easier to set up than
AFL. See
[the `cargo-fuzz` book for details](https://rust-fuzz.github.io/book/cargo-fuzz/tutorial.html).

1. `$ cargo install cargo-fuzz`
2. `$ cargo fuzz parse_and_stringify`

Alternatively, run `cargo fuzz list` to get a list of fuzz targets to run
instead of the `parse_and_stringify` target.

### Fuzzing with AFL

What follows is a TLDR, for detailed instructions see
the [`afl.rs` book](https://rust-fuzz.github.io/book/afl/setup.html).

1. Install the afl.rs command line tool:

    `cargo install afl`

1. Build the cpp_mangle AFL fuzz target:

    `cargo afl build --features afl`

1. Run AFL:

     `cargo afl fuzz -i in -o out target/debug/afl_runner`

## Automatic code formatting

We use [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) to enforce a
consistent code style across the whole `cpp_demangle` code base.

You can install the latest version of `rustfmt` with this command:

```
$ cargo install -f rustfmt
```

Ensure that `~/.cargo/bin` is on your path.

Once that is taken care of, you can (re)format all code by running this command:

```
$ cargo fmt
```

The code style is described in the `rustfmt.toml` file in top level of the repo.
