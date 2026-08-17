// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

// Make sure that macro hygiene will ensure that when we reference "zerocopy",
// that will work properly even if they've renamed the crate and have not
// imported its traits.

// See comment in `include.rs` for why we disable the prelude.
#![no_implicit_prelude]
#![allow(warnings)]

include!("include.rs");

extern crate zerocopy as _zerocopy;

#[derive(_zerocopy::KnownLayout, _zerocopy::FromBytes, _zerocopy::Unaligned)]
#[repr(C)]
struct TypeParams<'a, T, I: imp::Iterator> {
    a: T,
    c: I::Item,
    d: u8,
    e: imp::PhantomData<&'a [::core::primitive::u8]>,
    f: imp::PhantomData<&'static ::core::primitive::str>,
    g: imp::PhantomData<imp::String>,
}

util_assert_impl_all!(
    TypeParams<'static, (), imp::IntoIter<()>>:
        _zerocopy::KnownLayout,
        _zerocopy::FromZeros,
        _zerocopy::FromBytes,
        _zerocopy::Unaligned
);
