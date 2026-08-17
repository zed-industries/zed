# Contribution guide

All PRs are welcome.

When issueing a pull-request, make sure that the repo is formatted with `cargo fmt`
and that `cargo test --all-features` return no error.

The source code is organize as follow:

## `build/`

The code generation program (aka. the build script).
This program scans the XML definition to produce the Rust source code of the protocol implementation.

Sometimes, you have to hardcode exceptions in the code generation (e.g. generated different code for a specific request or event).
If you do this (it is better if you don't), add an entry to the `EXCEPTION_LIST.txt` file.

If you need to write unit test for the generated code, you can add them under `src/test.rs`.
There are also a few generated unit tests in the protocol modules.

It is often tedious to understand changes of the Rust generated code only by looking at the code generation source code.
There is utility to observe the changes to the generated code
 - run `./gen.sh previous` before starting your PR
 - run `./gen.sh current` when you think your work is ready.
 - generate a diff:
   - In VsCode you can open a diff editor by running `gen/diff.sh [module name]`.
   - Otherwise run `diff -ur gen/previous gen/current` to generate a textual diff

## `examples/`

Every example is welcome, especially if it covers requests, events or extensions not covered by the current set of examples.
If adding an example, add the corresponding entry in `Cargo.toml`.
The Github workflows ensure that every example compiles.

## `gen/`

Working directory to observe the generated code. The content is not under version control. A sub-directory can be populated by running `./gen.sh [dirname]`.

## `src/`

Base library source code.

## `xml/`

The XML definitions from the XCB project.
These defintions were slightly modified compared to the upstream definitions to ease a part of the code generation.
The unmodified upstream XML definitions are kept in the `xml/upstream` folder.
