{{#title The bridge module — Rust ♡ C++}}
# The bridge module reference

The ***[Core concepts](concepts.md)*** in chapter 2 covered the high level model
that CXX uses to represent a language boundary. This chapter builds on that one
to document an exhaustive reference on the syntax and functionality of
\#\[cxx::bridge\].

- ***[extern "Rust"](extern-rust.md)*** &mdash; exposing opaque Rust types, Rust
  functions, Rust methods to C++; functions with lifetimes.

- ***[extern "C++"](extern-c++.md)*** &mdash; binding opaque C++ types, C++
  functions, C++ member functions; sharing an opaque type definition across
  multiple bridge modules or different crates; using bindgen-generated data
  structures across a CXX bridge; Rust orphan-rule-compatible way to request
  that particular glue code be emitted in a specific bridge module.

- ***[Shared types](shared.md)*** &mdash; shared structs; shared enums; using
  Rust as source of truth vs C++ as source of truth.

- ***[Attributes](attributes.md)*** &mdash; working with namespaces; giving
  functions a different name in their non-native language.

- ***[Async functions](async.md)*** &mdash; integrating async C++ with async
  Rust.

- ***[Error handling](binding/result.md)*** &mdash; representing fallibility on
  the language boundary; accessing a Rust error message from C++; customizing
  the set of caught exceptions and their conversion to a Rust error message.
