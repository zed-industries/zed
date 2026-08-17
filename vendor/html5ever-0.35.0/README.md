# html5ever

[![Build Status](https://github.com/servo/html5ever/actions/workflows/main.yml/badge.svg)](https://github.com/servo/html5ever/actions)
[![crates.io](https://img.shields.io/crates/v/html5ever.svg)](https://crates.io/crates/html5ever)

[API Documentation][API documentation]

html5ever is an HTML parser developed as part of the [Servo][] project.

It can parse and serialize HTML according to the [WHATWG](https://whatwg.org/) specs (aka "HTML5"). However, there are some differences in the actual behavior currently, most of which are documented [in the bug tracker][]. html5ever passes all tokenizer tests from [html5lib-tests][], with most tree builder tests outside of the unimplemented features. The goal is to pass all html5lib tests, while also providing all hooks needed by a production web browser, e.g. `document.write`.

Note that the HTML syntax is very similar to XML. For correct parsing of XHTML, use an XML parser (that said, many XHTML documents in the wild are serialized in an HTML-compatible form).

html5ever is written in [Rust][], therefore it avoids the notorious security problems that come along with using C. Being built with Rust also makes the library come with the high-grade performance you would expect from an HTML parser written in C. html5ever is basically a C HTML parser, but without needing a garbage collector or other heavy runtime processes.


## Getting started in Rust

Add html5ever as a dependency:

```bash
cargo add html5ever
```

You should also take a look at [`examples/html2html.rs`], [`examples/print-rcdom.rs`], and the [API documentation][].


## Getting started in other languages

Bindings for Python and other languages are much desired.


## Working on html5ever

To fetch the test suite, you need to run

```
git submodule update --init
```

Run `cargo doc` in the repository root to build local documentation under `target/doc/`.


## Details

html5ever uses callbacks to manipulate the DOM, therefore it does not provide any DOM tree representation. 

html5ever exclusively uses UTF-8 to represent strings. In the future it will support other document encodings (and UCS-2 `document.write`) by converting input.

The code is cross-referenced with the WHATWG syntax spec, and eventually we will have a way to present code and spec side-by-side.

html5ever builds against the official stable releases of Rust, though some optimizations are only supported on nightly releases.

[API documentation]: https://docs.rs/html5ever
[Servo]: https://github.com/servo/servo
[Rust]: https://www.rust-lang.org/
[in the bug tracker]: https://github.com/servo/html5ever/issues?q=is%3Aopen+is%3Aissue+label%3Aweb-compat
[html5lib-tests]: https://github.com/html5lib/html5lib-tests
[`examples/html2html.rs`]: https://github.com/servo/html5ever/blob/main/rcdom/examples/html2html.rs
[`examples/print-rcdom.rs`]: https://github.com/servo/html5ever/blob/main/rcdom/examples/print-rcdom.rs
