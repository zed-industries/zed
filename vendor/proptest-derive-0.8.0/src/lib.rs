// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This is the API documentation for the `proptest-derive` crate. As this
//! crate does not have an API _per se_, there isn't much to see here.
//!
//! You are probably looking for the [`proptest-derive` section of the Proptest
//! Book](https://proptest-rs.github.io/proptest/proptest-derive/index.html).

// # Known issues
//
// ## Fields with `[T; N]` where `N > 32`
//
// We can't derive for fields having arrays with sizes over 32.
// While proptest only supports in UniformArrayStrategy arrays of sizes up to
// 32, we can overcome that restriction by generating custom types on the
// fly here. What we can't overcome is that `T: Arbititrary |- T: Debug` due
// to the requirement by proptest. Since `T: Debug` must hold, we must also
// ensure that arrays with sizes over 33 are also Debug. We can't do this.
// Doing so would create orphan instances, which Rust does not allow to preserve
// coherence. Therefore, until const generics lands in stable or when
// we can remove the `T: Debug` bound on Arbitrary, we can not support arrays
// sized over 32.
//
// # Recursive types
//
// We can't handle self-recursive or mutually recursive types at all right now.

extern crate proc_macro as pm;
extern crate proc_macro2;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod ast;
mod attr;
mod derive;
mod error;
mod interp;
mod use_tracking;
mod util;
mod void;

/// See module level documentation for more information.
#[proc_macro_derive(Arbitrary, attributes(proptest))]
pub fn derive_proptest_arbitrary(input: pm::TokenStream) -> pm::TokenStream {
    // Bootstrap!
    // This function just delegates to impl_proptest_arbitrary.
    derive::impl_proptest_arbitrary(syn::parse(input).unwrap()).into()
}

#[cfg(test)]
mod tests;
