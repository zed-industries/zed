# `wasmparser`: A WebAssembly Binary Parser

**A [Bytecode Alliance](https://bytecodealliance.org/) project**

[![crates.io link](https://img.shields.io/crates/v/wasmparser.svg)](https://crates.io/crates/wasmparser)
[![docs.rs docs](https://img.shields.io/static/v1?label=docs&message=wasmparser&color=blue&style=flat-square)](https://docs.rs/wasmparser/)

A simple, event-driven library for parsing WebAssembly binary files (or
streams).

The library reports events as they happen and only stores parsing information
for a brief period of time, making it fast and memory-efficient. The
event-driven model, however, has some drawbacks. If you need random access to
the entire WebAssembly data-structure, this is not the right library for
you. You could however, build such a data-structure using this library.

To get started, create a
[`Parser`](https://docs.rs/wasmparser/latest/wasmparser/struct.Parser.html)
using
[`Parser::new`](https://docs.rs/wasmparser/latest/wasmparser/struct.Parser.html#method.new)
and then follow the examples documented for
[`Parser::parse`](https://docs.rs/wasmparser/latest/wasmparser/struct.Parser.html#method.parse)
or
[`Parser::parse_all`](https://docs.rs/wasmparser/latest/wasmparser/struct.Parser.html#method.parse_all).

## Documentation

Documentation and examples can be found at https://docs.rs/wasmparser/
