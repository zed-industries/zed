// Copyright 2024 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

// See comment in `include.rs` for why we disable the prelude.
#![no_implicit_prelude]
#![allow(warnings)]
#![deny(deprecated)]

include!("include.rs");

// Make sure no deprecation warnings are generated from our derives (see #553).

#[macro_export]
macro_rules! test {
    ($name:ident => $ty:item => $($trait:ident),*) => {
        #[allow(non_snake_case)]
        mod $name {
            $(
                mod $trait {
                    use super::super::*;

                    #[deprecated = "do not use"]
                    #[derive(imp::$trait)]
                    $ty

                    #[allow(deprecated)]
                    fn _allow_deprecated() {
                        util_assert_impl_all!($name: imp::$trait);
                    }
                }
            )*
        }
    };
}

// NOTE: `FromBytes` is tested separately in `enum_from_bytes.rs` since it
// requires 256-variant enums which are extremely verbose; such enums are
// already in that file.
test!(Enum => #[repr(u8)] enum Enum { A, } => TryFromBytes, FromZeros, KnownLayout, Immutable, IntoBytes, Unaligned);

test!(Struct => #[repr(C)] struct Struct; => TryFromBytes, FromZeros, FromBytes, KnownLayout, Immutable, IntoBytes, Unaligned);

test!(Union => #[repr(C)] union Union{ a: (), } => TryFromBytes, FromZeros, FromBytes, KnownLayout, Immutable, IntoBytes, Unaligned);
