# `wit-bindgen-rt`

This is an internal implementation detail of the [`wit-bindgen`] crate. The
source for this crate lives at https://github.com/bytecodealliance/wit-bindgen
and this crate is located in `crates/guest-rust/rt` folder. The purpose of this
crate is to contain "runtime" code related to the macro-expansion of the
`wit_bindgen::generate!` macro such that the `wit-bindgen` crate dependency can
be removed in some situations.

This crate contains a precompiled object file and archive at
`src/cabi_realloc.o` and `src/libwit_bindgen_cabi_realloc.a`. This is compiled
from the source `src/cabi_realloc.c` and is checked in as precompiled to avoid
needing a C compiler at compile-time which isn't always available. This object
file is only used on wasm targets.

The object file is compiled by
[this script]https://github.com/bytecodealliance/wit-bindgen/blob/main/ci/rebuild-libcabi-realloc.sh)
and is verified in repository continuous integration that the checked-in
versions match what CI produces.

[`wit-bindgen`]: https://crates.io/crates/wit-bindgen

