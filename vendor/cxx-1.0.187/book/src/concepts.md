{{#title Core concepts — Rust ♡ C++}}
# Core concepts

This page is a brief overview of the major concepts of CXX, enough so that you
recognize the shape of things as you read the tutorial and following chapters.

In CXX, the language of the FFI boundary involves 3 kinds of items:

- **Shared structs** &mdash; data structures whose fields are made visible to
  both languages. The definition written within cxx::bridge in Rust is usually
  the single source of truth, though there are ways to do sharing based on a
  bindgen-generated definition with C++ as source of truth.

- **Opaque types** &mdash; their fields are secret from the other language.
  These cannot be passed across the FFI by value but only behind an indirection,
  such as a reference `&`, a Rust `Box`, or a C++ `unique_ptr`. Can be a type
  alias for an arbitrarily complicated generic language-specific type depending
  on your use case.

- **Functions** &mdash; implemented in either language, callable from the other
  language.

```rust,noplayground,focuscomment
# #[cxx::bridge]
# mod ffi {
    // Any shared structs, whose fields will be visible to both languages.
#     struct BlobMetadata {
#         size: usize,
#         tags: Vec<String>,
#     }
#
#     extern "Rust" {
        // Zero or more opaque types which both languages can pass around
        // but only Rust can see the fields.
#         type MultiBuf;
#
        // Functions implemented in Rust.
#         fn next_chunk(buf: &mut MultiBuf) -> &[u8];
#     }
#
#     unsafe extern "C++" {
        // One or more headers with the matching C++ declarations for the
        // enclosing extern "C++" block. Our code generators don't read it
        // but it gets #include'd and used in static assertions to ensure
        // our picture of the FFI boundary is accurate.
#         include!("demo/include/blobstore.h");
#
        // Zero or more opaque types which both languages can pass around
        // but only C++ can see the fields.
#         type BlobstoreClient;
#
        // Functions implemented in C++.
#         fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
#         fn put(&self, parts: &mut MultiBuf) -> u64;
#         fn tag(&self, blobid: u64, tag: &str);
#         fn metadata(&self, blobid: u64) -> BlobMetadata;
#     }
# }
```

Within the `extern "Rust"` part of the CXX bridge we list the types and
functions for which Rust is the source of truth. These all implicitly refer to
the `super` module, the parent module of the CXX bridge. You can think of the
two items listed in the example above as being like `use super::MultiBuf` and
`use super::next_chunk` except re-exported to C++. The parent module will either
contain the definitions directly for simple things, or contain the relevant
`use` statements to bring them into scope from elsewhere.

Within the `extern "C++"` part, we list types and functions for which C++ is the
source of truth, as well as the header(s) that declare those APIs. In the future
it's possible that this section could be generated bindgen-style from the
headers but for now we need the signatures written out; static assertions verify
that they are accurate.

<br><br>

Be aware that the design of this library is intentionally restrictive and
opinionated! It isn't a goal to be flexible enough to handle an arbitrary
signature in either language. Instead this project is about carving out a highly
expressive set of functionality about which we can make powerful safety
guarantees today and extend over time. You may find that it takes some practice
to use CXX bridge effectively as it won't work in all the ways that you may be
used to.

<br>
