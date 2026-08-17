# failspot

[![crates.io](https://img.shields.io/crates/v/failspot?style=for-the-badge&logo=rust)](https://crates.io/crates/failspot)
[![docs.rs](https://img.shields.io/docsrs/failspot?style=for-the-badge&logo=docs.rs&label=docs.rs)](https://docs.rs/failspot)
![license](https://img.shields.io/crates/l/failspot?style=for-the-badge)

A testing library that makes it easy(ish) to add intentional errors to a program

When testing error-handling codepaths, it is often useful to programmatically tell parts of the code to fail. This
crate provides the `failspot!()` macro, which can be used to mark a spot in the codepath where an intentional failure
can be toggled on and off from testing code.
