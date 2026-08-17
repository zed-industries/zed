bitstream-io
============

A Rust library for reading or writing binary values to or from streams
which may not be aligned at a whole byte.

This library is intended to be flexible enough to wrap
around any stream which implements the `Read` or `Write` traits.
It also supports a wide array of integer data types as
containers for those binary values.

## Minimum Compiler Version

Beginning with version 2.4, the minimum compiler version has been
updated to Rust 1.79 in order to support compile-time assertion
in `const` blocks, which can be used to check for a class
of errors at compile-time rather than runtime.
