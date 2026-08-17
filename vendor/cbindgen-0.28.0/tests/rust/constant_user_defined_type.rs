#[repr(C)]
pub struct S {
    field: u8,
}

/// cbindgen:enum-class=false
#[repr(C)]
pub enum E {
    V,
}
use E::*;

pub type A = u8;

pub const C1: S = S { field: 0 };
pub const C2: E = V;
pub const C3: A = 0;
