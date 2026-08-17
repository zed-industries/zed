#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(nightly, feature(never_type))]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use ::alloc::{boxed::Box, vec::Vec};

use derive_more::Deref;

#[derive(Deref)]
#[deref(forward)]
struct MyBoxedInt(Box<i32>);

#[derive(Deref)]
#[deref(forward)]
struct NumRef<'a> {
    num: &'a i32,
}

#[derive(Deref)]
struct NumRef2<'a> {
    #[deref(forward)]
    num: &'a i32,
    useless: bool,
}

#[derive(Deref)]
#[deref(forward)]
struct NumRef3<'a> {
    num: &'a i32,
    #[deref(ignore)]
    useless: bool,
}

#[derive(Deref)]
struct MyInt(i32);

#[derive(Deref)]
struct Point1D {
    x: i32,
}

#[derive(Deref)]
struct Point1D2 {
    x: i32,
    #[deref(ignore)]
    useless: bool,
}

#[derive(Deref)]
struct CoolVec {
    cool: bool,
    #[deref]
    vec: Vec<i32>,
}

#[derive(Deref)]
struct GenericVec<T>(Vec<T>);

#[test]
fn deref_generic() {
    let gv = GenericVec(Vec::<i32>::new());
    assert!(gv.is_empty())
}

#[derive(Deref)]
struct GenericBox<T>(#[deref(forward)] Box<T>);

#[test]
fn deref_generic_forward() {
    let boxed = GenericBox(Box::new(1i32));
    assert_eq!(*boxed, 1i32);
}

#[cfg(nightly)]
mod never {
    use super::*;

    #[derive(Deref)]
    struct Tuple(!);

    #[derive(Deref)]
    struct Struct {
        field: !,
    }
}

#[derive(Deref)]
enum MyEnum<'a> {
    Variant1(&'a [u8]),
    Variant2(&'a [u8]),
    Variant3(&'a [u8]),
}

#[derive(Deref)]
enum Enum {
    V1(i32),
    V2 { num: i32 },
}

#[derive(Deref)]
#[deref(forward)]
enum MyBoxedIntEnum {
    V1(Box<i32>),
    V2 { num: Box<i32> },
}

#[derive(Deref)]
enum CoolVecEnum {
    V1(Vec<i32>),
    V2 {
        cool: bool,
        #[deref]
        vec: Vec<i32>,
    },
}

#[derive(Deref)]
#[deref(forward)]
enum NumRefEnum<'a> {
    Variant1 {
        num: &'a i32,
    },
    Variant2(&'a i32),
    Variant3 {
        n: &'a i32,
        #[deref(ignore)]
        useless: bool,
    },
}

#[derive(Deref)]
enum NumRefEnum2<'a> {
    Variant1 {
        #[deref]
        num: &'a i32,
        useless: bool,
    },
    Variant2 {
        num: &'a i32,
        #[deref(ignore)]
        useless: bool,
    },
    Variant3(&'a i32),
}

#[derive(Deref)]
enum NumRefEnum3 {
    Variant1 {
        #[deref(forward)]
        num: Box<i32>,
        useless: bool,
    },
    Variant2 {
        num: i32,
        #[deref(ignore)]
        useless: bool,
    },
    Variant3(i32),
}

#[derive(Deref)]
enum NumRefEnum4 {
    Variant1(i32),
    Variant2 {
        #[deref(forward)]
        num: Box<i32>,
        useless: bool,
    },
    Variant3 {
        num: i32,
        #[deref(ignore)]
        useless: bool,
    },
}

#[derive(Deref)]
enum GenericBoxEnum1<T> {
    Variant1 {
        #[deref(forward)]
        b: Box<T>,
    },
    Variant2(bool, #[deref(forward)] Box<T>),
}

#[test]
fn deref_generic_forward_enum_inner() {
    let boxed = GenericBoxEnum1::Variant2(true, Box::new(1i32));
    assert_eq!(*boxed, 1i32);
}

#[derive(Deref)]
enum GenericBoxEnum2<T> {
    Variant1 {
        a: T,
        #[deref(forward)]
        b: Box<i32>,
    },
    Variant2(bool, #[deref(forward)] Box<i32>),
}

#[derive(Deref)]
#[deref(forward)]
enum GenericBoxEnum3<T> {
    Variant1 { b: Box<T> },
    Variant2(Box<T>),
    Variant3(#[deref(ignore)] bool, Box<T>),
}

#[test]
fn deref_generic_forward_enum_outer() {
    let boxed = GenericBoxEnum3::Variant2(Box::new(1i32));
    assert_eq!(*boxed, 1i32);
}

mod deprecated {
    use super::*;

    #[derive(Deref)]
    #[deprecated(note = "struct")]
    struct Tuple(#[deprecated(note = "field")] i32);

    #[derive(Deref)]
    #[deprecated(note = "struct")]
    struct Struct {
        #[deprecated(note = "field")]
        field: i32,
    }
}
