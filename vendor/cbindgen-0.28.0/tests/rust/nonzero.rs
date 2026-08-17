use std::num::*;

#[repr(C)]
pub struct NonZeroAliases {
    pub a: NonZeroU8,
    pub b: NonZeroU16,
    pub c: NonZeroU32,
    pub d: NonZeroU64,
    pub e: NonZeroI8,
    pub f: NonZeroI16,
    pub g: NonZeroI32,
    pub h: NonZeroI64,
    pub i: Option<NonZeroI64>,
    pub j: *const Option<Option<NonZeroI64>>,
}

#[no_mangle]
pub extern "C" fn root_nonzero_aliases(
    test: NonZeroAliases,
    a: NonZeroU8,
    b: NonZeroU16,
    c: NonZeroU32,
    d: NonZeroU64,
    e: NonZeroI8,
    f: NonZeroI16,
    g: NonZeroI32,
    h: NonZeroI64,
    i: Option<NonZeroI64>,
    j: *const Option<Option<NonZeroI64>>,
) {}

#[repr(C)]
pub struct NonZeroGenerics {
    pub a: NonZero<u8>,
    pub b: NonZero<u16>,
    pub c: NonZero<u32>,
    pub d: NonZero<u64>,
    pub e: NonZero<i8>,
    pub f: NonZero<i16>,
    pub g: NonZero<i32>,
    pub h: NonZero<i64>,
    pub i: Option<NonZero<i64>>,
    pub j: *const Option<Option<NonZero<i64>>>,
}

#[no_mangle]
pub extern "C" fn root_nonzero_generics(
    test: NonZeroGenerics,
    a: NonZero<u8>,
    b: NonZero<u16>,
    c: NonZero<u32>,
    d: NonZero<u64>,
    e: NonZero<i8>,
    f: NonZero<i16>,
    g: NonZero<i32>,
    h: NonZero<i64>,
    i: Option<NonZero<i64>>,
    j: *const Option<Option<NonZero<i64>>>,
) {}
