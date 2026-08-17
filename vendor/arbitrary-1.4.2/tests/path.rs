#![cfg(feature = "derive")]
// Various structs/fields that we are deriving `Arbitrary` for aren't actually
// used except to show off the derive.
#![allow(dead_code)]

// Regression test for ensuring the derives work without Arbitrary being imported

#[derive(arbitrary::Arbitrary, Clone, Debug)]
pub struct Struct {
    x: u8,
    y: u8,
}

#[derive(arbitrary::Arbitrary, Clone, Debug)]
pub struct Tuple(u8);

#[derive(arbitrary::Arbitrary, Clone, Debug)]
pub struct Unit(u8);

#[derive(arbitrary::Arbitrary, Clone, Debug)]
pub enum Enum {
    X(u8),
    Y(u8),
}

#[derive(arbitrary::Arbitrary, Clone, Debug)]
struct EndingInVec(u8, bool, u32, Vec<u16>);

#[derive(arbitrary::Arbitrary, Debug)]
struct Generic<T> {
    inner: T,
}
