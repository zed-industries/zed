# codespan-reporting

[![Continuous integration][actions-badge]][actions-url]
[![Crates.io][crate-badge]][crate-url]
[![Docs.rs][docs-badge]][docs-url]
[![Matrix][matrix-badge]][matrix-lobby]

[actions-badge]: https://img.shields.io/github/actions/workflow/status/brendanzab/codespan/ci.yml?branch=master
[actions-url]: https://github.com/brendanzab/codespan/actions
[crate-url]: https://crates.io/crates/codespan-reporting
[crate-badge]: https://img.shields.io/crates/v/codespan-reporting.svg
[docs-url]: https://docs.rs/codespan-reporting
[docs-badge]: https://docs.rs/codespan-reporting/badge.svg
[matrix-badge]: https://img.shields.io/badge/matrix-%23codespan%3Amatrix.org-blue.svg
[matrix-lobby]: https://app.element.io/#/room/#codespan:matrix.org

Beautiful diagnostic reporting for text-based programming languages.

![Example preview](./codespan-reporting/assets/readme_preview.svg?sanitize=true)

Languages like Rust and Elm already support beautiful error reporting output,
but it can take a significant amount work to implement this for new programming
languages! The `codespan-reporting` crate aims to make beautiful error
diagnostics easy and relatively painless for everyone!

We're still working on improving the crate to help it support broader use cases,
and improving the quality of the diagnostic rendering, so stay tuned for
updates and please give us feedback if you have it. Contributions are also very
welcome!

## Example

```rust
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

// `files::SimpleFile` and `files::SimpleFiles` help you get up and running with
// `codespan-reporting` quickly! More complicated use cases can be supported
// by creating custom implementations of the `files::Files` trait.

let mut files = SimpleFiles::new();

let file_id = files.add(
    "FizzBuzz.fun",
    unindent::unindent(
        r#"
            module FizzBuzz where

            fizz₁ : Nat → String
            fizz₁ num = case (mod num 5) (mod num 3) of
                0 0 => "FizzBuzz"
                0 _ => "Fizz"
                _ 0 => "Buzz"
                _ _ => num

            fizz₂ : Nat → String
            fizz₂ num =
                case (mod num 5) (mod num 3) of
                    0 0 => "FizzBuzz"
                    0 _ => "Fizz"
                    _ 0 => "Buzz"
                    _ _ => num
        "#,
    ),
);

// We normally recommend creating a custom diagnostic data type for your
// application, and then converting that to `codespan-reporting`'s diagnostic
// type, but for the sake of this example we construct it directly.

let diagnostic = Diagnostic::error()
    .with_message("`case` clauses have incompatible types")
    .with_code("E0308")
    .with_labels(vec![
        Label::primary(file_id, 328..331).with_message("expected `String`, found `Nat`"),
        Label::secondary(file_id, 211..331).with_message("`case` clauses have incompatible types"),
        Label::secondary(file_id, 258..268).with_message("this is found to be of type `String`"),
        Label::secondary(file_id, 284..290).with_message("this is found to be of type `String`"),
        Label::secondary(file_id, 306..312).with_message("this is found to be of type `String`"),
        Label::secondary(file_id, 186..192).with_message("expected type `String` found here"),
    ])
    .with_notes(vec![unindent::unindent(
        "
            expected type `String`
                found type `Nat`
        ",
    )]);

// We now set up the writer and configuration, and then finally render the
// diagnostic to standard error.

let writer = StandardStream::stderr(ColorChoice::Always);
let config = codespan_reporting::term::Config::default();

term::emit(&mut writer.lock(), &config, &files, &diagnostic)?;
```

## Running the CLI example

To get an idea of what the colored CLI output looks like,
clone the [repository](https://github.com/brendanzab/codespan)
and run the following shell command:

```sh
cargo run --example term
```

More examples of using `codespan-reporting` can be found in the
[examples directory](./codespan-reporting/examples).

## Projects using codespan-reporting

`codespan-reporting` is currently used in the following projects:

- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)
- [cargo-about](https://github.com/EmbarkStudios/cargo-about)
- [CXX](https://github.com/dtolnay/cxx)
- [full_moon](https://github.com/Kampfkarren/full-moon)
- [Gleam](https://github.com/gleam-lang/gleam)
- [Gluon](https://github.com/gluon-lang/gluon)
- [MDBook LinkCheck](https://github.com/Michael-F-Bryan/mdbook-linkcheck)
- [mos](https://github.com/datatrash/mos)
- [Pikelet](https://github.com/pikelet-lang/pikelet)
- [Naga](https://github.com/gfx-rs/wgpu/tree/trunk/naga)
- [Spade](https://gitlab.com/spade-lang/spade)
 
 ... [any many more](https://crates.io/crates/codespan-reporting/reverse_dependencies)

## Alternatives to codespan-reporting

There are a number of alternatives to `codespan-reporting`, including:

- [annotate-snippets][annotate-snippets]
- [codemap][codemap]
- [language-reporting][language-reporting] (a fork of codespan)

These are all ultimately inspired by rustc's excellent [error reporting infrastructure][librustc_errors].

[annotate-snippets]: https://crates.io/crates/annotate-snippets
[codemap]: https://crates.io/crates/codemap
[language-reporting]: https://crates.io/crates/language-reporting
[librustc_errors]: https://github.com/rust-lang/rust/tree/master/compiler/rustc_errors/src

## Contributing

A guide to contributing to codespan-reporting [can be found here](/CONTRIBUTING.md).

## Code of Conduct

Please note that this project is released with a [Code of Conduct](./CODE_OF_CONDUCT.md).
By participating in this project you agree to abide by its terms.
