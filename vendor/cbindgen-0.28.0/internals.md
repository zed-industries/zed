## Overview

`cbindgen` works in four phases:

1. *Parsing* - Crate information is gathered from `cargo`, and `rust` source files are read using `syn`
1. *Loading* - `syn` AST nodes are converted into an IR of `Item`s that loosely correspond to the C types that will be output
1. *Transformation* - Several passes are run that transform the IR. Some examples:
   - Generic `type` aliases are used to specialize the type they refer to
   - Annotations are transferred from `type` aliases to the item they refer to
   - `Option<&T>` is converted to `*const T`
   - `Option<&mut T>` is converted to `*mut T`
   - Generic paths in struct fields, union variants, and static globals are collected and used to generate monomorphs of the structs or unions they refer to
   - The items are sorted by dependencies and type and unused items are filtered out
1. *Writing* - The IR is pretty printed to a file or `stdout`

## Process Flow

The main interface for `cbindgen` is `bindgen::Builder` which accepts configuration options and either a crate directory to parse or a list of source files.

If a list of source files is given, then `bindgen::Builder` will parse them using `parser::parse_src` which will use `syn` to parse a specific file. No `extern crate` items will be followed for dependencies, but `mod` items will be attempted to be followed.

If a crate directory is given, then `bindgen::Builder` will use `cargo::Cargo` to load a dependency graph from `Cargo.toml`, `Cargo.lock`, and `cargo metadata`. Then `parser::parse_lib` will parse each crate, following `extern crate` items when `ParseConfig::parse_deps` is enabled and the crate is not in the whitelist or blacklist of crates. In addition `bindgen::Parser` may use `cargo expand` on a crate to expand macro definitions.

Once the `syn` nodes are collected by either method, they are given to `bindgen::Parse` which will perform *Loading* by creating a `ir::Item` for each `syn` node as appropriate.

`bindgen::Builder` will then convert the resulting `bindgen::Parse`'s into a `bindgen::Library` which is the driver of all of the *Transformation* passes.

// TODO - Talk more about passes

Then finally the `bindgen::Library` will create a `bindgen::Bindings` which contains the `ir::Item`'s that are ready to be written. The `bindgen::Bindings` can then be written to `stdout` or a specific file.
