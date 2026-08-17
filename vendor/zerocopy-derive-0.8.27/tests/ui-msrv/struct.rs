// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

#[macro_use]
extern crate zerocopy;

#[path = "../include.rs"]
mod util;

use zerocopy::{IntoBytes, KnownLayout};

use self::util::util::AU16;

fn main() {}

//
// KnownLayout errors
//

struct NotKnownLayout;

struct NotKnownLayoutDst([u8]);

// | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
// |          N |        N |              N |        N |      KL00 |
#[derive(KnownLayout)]
struct KL00(u8, NotKnownLayoutDst);

// | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
// |          N |        N |              Y |        N |      KL02 |
#[derive(KnownLayout)]
struct KL02(u8, [u8]);

// | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
// |          Y |        N |              N |        N |      KL08 |
#[derive(KnownLayout)]
#[repr(C)]
struct KL08(u8, NotKnownLayoutDst);

// | `repr(C)`? | generic? | `KnownLayout`? | `Sized`? | Type Name |
// |          Y |        N |              N |        Y |      KL09 |
#[derive(KnownLayout)]
#[repr(C)]
struct KL09(NotKnownLayout, NotKnownLayout);

//
// Immutable errors
//

#[derive(Immutable)]
struct Immutable1 {
    a: core::cell::UnsafeCell<()>,
}

#[derive(Immutable)]
struct Immutable2 {
    a: [core::cell::UnsafeCell<u8>; 0],
}

//
// TryFromBytes errors
//

#[derive(TryFromBytes)]
#[repr(packed)]
struct TryFromBytesPacked {
    foo: AU16,
}

#[derive(TryFromBytes)]
#[repr(packed(1))]
struct TryFromBytesPackedN {
    foo: AU16,
}

#[derive(TryFromBytes)]
#[repr(C, packed)]
struct TryFromBytesCPacked {
    foo: AU16,
}

#[derive(TryFromBytes)]
#[repr(C, packed(1))]
struct TryFromBytesCPackedN {
    foo: AU16,
}

//
// IntoBytes errors
//

// Since `IntoBytes1` has at least one generic parameter, an `IntoBytes` impl is
// emitted in which each field type is given an `Unaligned` bound. Since `foo`'s
// type doesn't implement `Unaligned`, this should fail.
#[derive(IntoBytes)]
#[repr(C)]
struct IntoBytes1<T> {
    foo: AU16,
    bar: T,
}

#[derive(IntoBytes)]
#[repr(C)]
struct IntoBytes2 {
    foo: u8,
    bar: AU16,
}

#[derive(IntoBytes)]
#[repr(C, packed(2))]
struct IntoBytes3 {
    foo: u8,
    // We'd prefer to use AU64 here, but you can't use aligned types in
    // packed structs.
    bar: u64,
}

type SliceU8 = [u8];

// Padding between `u8` and `SliceU8`. `SliceU8` doesn't syntactically look like
// a slice, so this case is handled by our `Sized` support.
//
// NOTE(#1708): This exists to ensure that our error messages are good when a
// field is unsized.
#[derive(IntoBytes)]
#[repr(C)]
struct IntoBytes4 {
    a: u8,
    b: SliceU8,
}

// Padding between `u8` and `[u16]`. `[u16]` is syntactically identifiable as a
// slice, so this case is handled by our `repr(C)` slice DST support.
#[derive(IntoBytes)]
#[repr(C)]
struct IntoBytes5 {
    a: u8,
    b: [u16],
}

// Trailing padding after `[u8]`. `[u8]` is syntactically identifiable as a
// slice, so this case is handled by our `repr(C)` slice DST support.
#[derive(IntoBytes)]
#[repr(C)]
struct IntoBytes6 {
    a: u16,
    b: [u8],
}

// Padding between `u8` and `u16` and also trailing padding after `[u8]`. `[u8]`
// is syntactically identifiable as a slice, so this case is handled by our
// `repr(C)` slice DST support.
#[derive(IntoBytes)]
#[repr(C)]
struct IntoBytes7 {
    a: u8,
    b: u16,
    c: [u8],
}

#[derive(IntoBytes)]
#[repr(C, C)] // zerocopy-derive conservatively treats these as conflicting reprs
struct IntoBytes8 {
    a: u8,
}

#[derive(IntoBytes)]
struct IntoBytes9<T> {
    t: T,
}

#[derive(IntoBytes)]
#[repr(packed(2))]
struct IntoBytes10<T> {
    t: T,
}

// `repr(C, packed(2))` is not equivalent to `repr(C, packed)`.
#[derive(IntoBytes)]
#[repr(C, packed(2))]
struct IntoBytes11<T> {
    t0: T,
    // Add a second field to avoid triggering the "repr(C) struct with one
    // field" special case.
    t1: T,
}

fn is_into_bytes_11<T: IntoBytes>() {
    if false {
        is_into_bytes_11::<IntoBytes11<AU16>>();
    }
}

// `repr(C, align(2))` is not sufficient to guarantee the layout of this type.
#[derive(IntoBytes)]
#[repr(C, align(2))]
struct IntoBytes12<T> {
    t: T,
}

//
// Unaligned errors
//

#[derive(Unaligned)]
#[repr(C, align(2))]
struct Unaligned1;

#[derive(Unaligned)]
#[repr(transparent, align(2))]
struct Unaligned2 {
    foo: u8,
}

#[derive(Unaligned)]
#[repr(packed, align(2))]
struct Unaligned3;

#[derive(Unaligned)]
#[repr(align(1), align(2))]
struct Unaligned4;

#[derive(Unaligned)]
#[repr(align(2), align(4))]
struct Unaligned5;

#[derive(Unaligned)]
struct Unaligned6;

#[derive(Unaligned)]
#[repr(packed(2))]
struct Unaligned7;

// Test the error message emitted when conflicting reprs appear on different
// lines. On the nightly compiler, this emits a "joint span" that spans both
// problematic repr token trees and everything in between.
#[derive(Copy, Clone)]
#[repr(packed(2), C)]
#[derive(Unaligned)]
#[repr(C, packed(2))]
struct WeirdReprSpan;

#[derive(SplitAt)]
#[repr(C)]
struct SplitAtNotKnownLayout([u8]);

#[derive(SplitAt, KnownLayout)]
#[repr(C)]
struct SplitAtSized(u8);
