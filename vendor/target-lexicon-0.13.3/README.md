This is a library for managing targets for compilers and related tools.

Currently, the main feature is support for decoding [LLVM "triples"], which
are strings that identify a particular target configuration. They're named
"triples" because historically they contained three fields, though over time
they've added additional fields. This library provides a `Triple` struct
containing enums for each of fields of a triple. `Triple` implements
`FromStr` and `fmt::Display` so it can be converted to and from the
conventional string representation of a triple.

`Triple` also has functions for querying a triple's endianness,
pointer bit width, and binary format.

And, `Triple` and the enum types have `host()` constructors, for targeting
the host.

It somewhat supports reading triples currently used by `rustc` and rustup,
though beware that the mapping between `rustc` and LLVM triples is not
one-to-one.

It does not support reading JSON target files itself. To use it with a JSON
target file, construct a `Triple` using the value of the "llvm-target" field.

[LLVM "triples"]: https://clang.llvm.org/docs/CrossCompilation.html#target-triple
