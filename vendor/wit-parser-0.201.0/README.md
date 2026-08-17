# `wit-parser`

A Rust crate for parsing and interpreting the [`*.wit`][wit] text format. This
text format is used to describe the imports and exports of a [component].

[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[component]: https://github.com/webassembly/component-model

This crate is a low-level tooling crate which is intended to be integrated
further into toolchains elsewhere and isn't necessarily interacted with on a
day-to-day basis. Internally it supports parsing a `*.wit` document into a
structured AST. Additionally it implements mechanisms of the canonical ABI to
assist in binding the canonical ABI into various languages.
