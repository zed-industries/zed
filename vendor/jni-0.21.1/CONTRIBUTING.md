# Jni-rs Contribution Guide

Jni-rs is open to any contributions, whether
it is a feedback on existing features, a request for a new one, a bug report
or a pull request. This document describes how to work with this project:

* how to [build](#how-to-build) it
* how to [test](#tests) it
* the [code style guidelines](#the-code-style)
* how to [submit an issue](#submitting-issues)
* how to [submit a PR](#submitting-pull-requests).

## How to Build

### System Dependencies

You need to install the following dependencies:

* [JDK 1.8+](http://jdk.java.net/10/).
* [Rust (latest stable)](https://www.rust-lang.org/).

### Building

To build `jni-rs`, simply run

```$sh
cargo build
```

inside project root directory. You can also build the library in release mode by
adding `--release` flag.

## Tests

### Categories of Tests

* Unit tests are typically placed at the bottom of their module file.
  E.g. [unit tests of signature module](src/wrapper/signature.rs). Tests are wrapped
  in private module with `test` attribute:

  ```rust
  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn first_test() { /* ... */ }

    // More tests...
  }
  ```

* Integration tests are in [tests directory](tests). They use the same pattern as
  unit tests, but split into several files instead of private modules.
  Integration tests use `jni-rs` as every other Rust application — by importing
  library using `extern crate` keyword.

  ```rust
    extern crate jni;
    use jni::*;
  ```

  Integration tests typically require running a JVM, so you should add
  `#![cfg(feature = "invocation")]` at the top of the file. You can use helper
  methods from [util module](tests/util/mod.rs) to run JVM.

  Keep in mind, that only one JVM can be run per process. Therefore, tests that
  need to launch it with different parameters have to be placed in different
  source files. `Cargo` runs tests from different modules in parallel.

* Benchmarks are in [benches](benches) directory. As integration tests, they
  require launching a JVM from native.

* Doc tests are rarely used, but they allow to efficiently test some functionality
  by providing an example of its usage. Consult
  [Rust documentation](https://doc.rust-lang.org/beta/rustdoc/documentation-tests.html)
  for more info about writing these tests.

### Running Tests

#### Setup Environment

As some tests need to launch a JVM, add a directory with JVM library that you want
to use to `LD_LIBRARY_PATH` on Linux/Mac or `PATH` environment variable on Windows.
On Linux/Mac, you can use the following script for this purpose:

```$sh
source test_profile
```

#### Run Tests

To run all tests, execute the following command:

```$sh
cargo test --features=invocation
```

This command will run all tests, including unit, integration and documentation
tests.

#### Run Benchmarks

To run all benchmarks, execute the following command (nightly Rust required):

```$sh
cargo +nightly bench --features=invocation
```

They might help you to see the performance differences between
two [API flavours][checked-unchecked]: checked and unchecked, and
pick the right one for your application.

[checked-unchecked]: https://docs.rs/jni/0.21.1/jni/struct.JNIEnv.html#checked-and-unchecked-methods

## The Code Style

Rust code follows the [Rust style guide](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md).
[`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) enforces the code style.

After installation, you can run it with

```$sh
cargo fmt --all -- --check
```

Every public entry of the API must be thoroughly documented and covered with tests if it is possible.
You can use [JNI specification](https://docs.oracle.com/javase/10/docs/specs/jni/index.html) as
a reference for how to write detailed documentation.

Do not forget to update project guides and tutorials along with documentation.

To open local documentation of the project, you can use the following command:

```$sh
cargo doc --open
```

## Submitting Issues

Use Github Issues to submit an issue, whether it is a question, some feedback, a bug or a feature request: <https://github.com/jni-rs/jni-rs/issues/new>

## Submitting Pull Requests

Before starting to work on a PR, please submit an issue describing the intended changes.
Chances are — we are already working on something similar. If not — we can then offer some
help with the requirements, design, implementation or documentation.

It’s fine to open a PR as soon as you need any feedback — ask any questions in the description.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
  at your option.

### Contributor License Agreement

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion is the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
