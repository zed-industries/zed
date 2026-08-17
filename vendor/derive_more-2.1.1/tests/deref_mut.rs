#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(nightly, feature(never_type))]
#![allow(dead_code)] // some code is tested for type checking only

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec, vec::Vec};

use derive_more::DerefMut;

#[derive(DerefMut)]
#[deref_mut(forward)]
struct MyBoxedInt(Box<i32>);

// `Deref` implementation is required for `DerefMut`.
impl ::core::ops::Deref for MyBoxedInt {
    type Target = <Box<i32> as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        <Box<i32> as ::core::ops::Deref>::deref(&self.0)
    }
}

#[derive(DerefMut)]
struct NumRef<'a> {
    #[deref_mut(forward)]
    num: &'a mut i32,
}

// `Deref` implementation is required for `DerefMut`.
impl<'a> ::core::ops::Deref for NumRef<'a> {
    type Target = <&'a mut i32 as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        <&'a mut i32 as ::core::ops::Deref>::deref(&self.num)
    }
}

#[derive(DerefMut)]
#[deref_mut(forward)]
struct NumRef2<'a> {
    num: &'a mut i32,
    #[deref_mut(ignore)]
    useless: bool,
}

// `Deref` implementation is required for `DerefMut`.
impl<'a> ::core::ops::Deref for NumRef2<'a> {
    type Target = <&'a mut i32 as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        <&'a mut i32 as ::core::ops::Deref>::deref(&self.num)
    }
}

#[derive(DerefMut)]
struct MyInt(i32);

// `Deref` implementation is required for `DerefMut`.
impl ::core::ops::Deref for MyInt {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(DerefMut)]
struct Point1D {
    x: i32,
}

// `Deref` implementation is required for `DerefMut`.
impl ::core::ops::Deref for Point1D {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

#[derive(DerefMut)]
struct CoolVec {
    cool: bool,
    #[deref_mut]
    vec: Vec<i32>,
}

// `Deref` implementation is required for `DerefMut`.
impl ::core::ops::Deref for CoolVec {
    type Target = Vec<i32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

#[derive(DerefMut)]
struct GenericVec<T>(Vec<T>);

// `Deref` implementation is required for `DerefMut`.
impl<T> ::core::ops::Deref for GenericVec<T> {
    type Target = Vec<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn deref_mut_generic() {
    let mut gv = GenericVec::<i32>(vec![42]);
    assert!(gv.get_mut(0).is_some());
}

#[derive(DerefMut)]
struct GenericBox<T>(#[deref_mut(forward)] Box<T>);

// `Deref` implementation is required for `DerefMut`.
impl<T> ::core::ops::Deref for GenericBox<T>
where
    Box<T>: ::core::ops::Deref,
{
    type Target = <Box<T> as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        <Box<T> as ::core::ops::Deref>::deref(&self.0)
    }
}

#[test]
fn deref_mut_generic_forward() {
    let mut boxed = GenericBox(Box::new(1i32));
    *boxed = 3;
    assert_eq!(*boxed, 3i32);
}

#[derive(DerefMut)]
enum MyEnum<'a> {
    Variant1(&'a [u8]),
    Variant2(&'a [u8]),
    Variant3(&'a [u8]),
}

impl<'a> ::core::ops::Deref for MyEnum<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            MyEnum::Variant1(inner)
            | MyEnum::Variant2(inner)
            | MyEnum::Variant3(inner) => inner,
        }
    }
}

#[derive(DerefMut)]
enum Enum {
    V1(i32),
    V2 { num: i32 },
}

impl ::core::ops::Deref for Enum {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Enum::V1(num) | Enum::V2 { num } => num,
        }
    }
}

#[derive(DerefMut)]
#[deref_mut(forward)]
enum MyBoxedIntEnum {
    V1(Box<i32>),
    V2 { num: Box<i32> },
}

impl ::core::ops::Deref for MyBoxedIntEnum {
    type Target = <Box<i32> as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            MyBoxedIntEnum::V1(num) | MyBoxedIntEnum::V2 { num } => {
                <Box<i32> as ::core::ops::Deref>::deref(num)
            }
        }
    }
}

#[derive(DerefMut)]
enum CoolVecEnum {
    V1(Vec<i32>),
    V2 {
        cool: bool,
        #[deref_mut]
        vec: Vec<i32>,
    },
}

impl ::core::ops::Deref for CoolVecEnum {
    type Target = Vec<i32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            CoolVecEnum::V1(vec) | CoolVecEnum::V2 { cool: _, vec } => vec,
        }
    }
}

#[derive(DerefMut)]
#[deref_mut(forward)]
enum NumRefEnum<'a> {
    Variant1 {
        num: &'a i32,
    },
    Variant2(&'a i32),
    Variant3 {
        n: &'a i32,
        #[deref_mut(ignore)]
        useless: bool,
    },
}

impl<'a> ::core::ops::Deref for NumRefEnum<'a> {
    type Target = <&'a i32 as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            NumRefEnum::Variant1 { num }
            | NumRefEnum::Variant2(num)
            | NumRefEnum::Variant3 { n: num, useless: _ } => {
                <&'a i32 as ::core::ops::Deref>::deref(num)
            }
        }
    }
}

#[derive(DerefMut)]
enum NumRefEnum2<'a> {
    Variant1 {
        #[deref_mut]
        num: &'a i32,
        useless: bool,
    },
    Variant2 {
        num: &'a i32,
        #[deref_mut(ignore)]
        useless: bool,
    },
    Variant3(&'a i32),
}

impl<'a> ::core::ops::Deref for NumRefEnum2<'a> {
    type Target = &'a i32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            NumRefEnum2::Variant1 { num, useless: _ }
            | NumRefEnum2::Variant2 { num, useless: _ }
            | NumRefEnum2::Variant3(num) => num,
        }
    }
}

#[derive(DerefMut)]
enum NumRefEnum3 {
    Variant1 {
        #[deref_mut(forward)]
        num: Box<i32>,
        useless: bool,
    },
    Variant2 {
        num: i32,
        #[deref_mut(ignore)]
        useless: bool,
    },
    Variant3(i32),
}

impl ::core::ops::Deref for NumRefEnum3 {
    type Target = <Box<i32> as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            NumRefEnum3::Variant1 { num, useless: _ } => {
                <Box<i32> as ::core::ops::Deref>::deref(num)
            }
            NumRefEnum3::Variant2 { num, useless: _ } | NumRefEnum3::Variant3(num) => {
                num
            }
        }
    }
}

#[derive(DerefMut)]
enum NumRefEnum4 {
    Variant1(i32),
    Variant2 {
        #[deref_mut(forward)]
        num: Box<i32>,
        useless: bool,
    },
    Variant3 {
        num: i32,
        #[deref_mut(ignore)]
        useless: bool,
    },
}

impl ::core::ops::Deref for NumRefEnum4 {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            NumRefEnum4::Variant1(num) | NumRefEnum4::Variant3 { num, useless: _ } => {
                num
            }
            NumRefEnum4::Variant2 { num, useless: _ } => {
                <Box<i32> as ::core::ops::Deref>::deref(num)
            }
        }
    }
}

#[derive(DerefMut)]
enum GenericVecEnum<T> {
    Variant1 {
        #[deref_mut]
        v: Vec<T>,
        useless: bool,
    },
    Variant2(#[deref_mut(ignore)] bool, Vec<T>),
}

impl<T> ::core::ops::Deref for GenericVecEnum<T> {
    type Target = Vec<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            GenericVecEnum::Variant1 { v, useless: _ }
            | GenericVecEnum::Variant2(_, v) => v,
        }
    }
}

#[test]
fn deref_mut_generic_enum() {
    let mut gv = GenericVecEnum::Variant2(true, vec![42i32]);
    assert!(gv.get_mut(0).is_some());
}

#[derive(DerefMut)]
enum GenericBoxEnum1<T> {
    Variant1 {
        #[deref_mut(forward)]
        b: Box<T>,
        useless: bool,
    },
    Variant2(bool, #[deref_mut(forward)] Box<T>),
}

impl<T> ::core::ops::Deref for GenericBoxEnum1<T> {
    type Target = <Box<T> as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            GenericBoxEnum1::Variant1 { b, useless: _ }
            | GenericBoxEnum1::Variant2(_, b) => {
                <Box<T> as ::core::ops::Deref>::deref(b)
            }
        }
    }
}

#[test]
fn deref_mut_generic_forward_enum_inner() {
    let mut boxed = GenericBoxEnum1::Variant2(true, Box::new(1i32));
    *boxed = 3;
    assert_eq!(*boxed, 3i32);
}

#[derive(DerefMut)]
enum GenericBoxEnum2<T> {
    Variant1 {
        a: T,
        #[deref_mut(forward)]
        b: Box<i32>,
    },
    Variant2(bool, #[deref_mut(forward)] Box<i32>),
}

impl<T> ::core::ops::Deref for GenericBoxEnum2<T> {
    type Target = <Box<i32> as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            GenericBoxEnum2::Variant1 { a: _, b } | GenericBoxEnum2::Variant2(_, b) => {
                <Box<i32> as ::core::ops::Deref>::deref(b)
            }
        }
    }
}

#[derive(DerefMut)]
#[deref_mut(forward)]
enum GenericBoxEnum3<T> {
    Variant1 { b: Box<T> },
    Variant2(Box<T>),
    Variant3(#[deref_mut(ignore)] bool, Box<T>),
}

impl<T> ::core::ops::Deref for GenericBoxEnum3<T> {
    type Target = <Box<T> as ::core::ops::Deref>::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            GenericBoxEnum3::Variant1 { b }
            | GenericBoxEnum3::Variant2(b)
            | GenericBoxEnum3::Variant3(_, b) => {
                <Box<T> as ::core::ops::Deref>::deref(b)
            }
        }
    }
}

#[test]
fn deref_mut_generic_forward_enum_outer() {
    let mut boxed = GenericBoxEnum3::Variant2(Box::new(1i32));
    *boxed = 3;
    assert_eq!(*boxed, 3i32);
}

#[cfg(nightly)]
mod never {
    use super::*;

    #[derive(DerefMut)]
    struct Tuple(!);

    // `Deref` implementation is required for `DerefMut`.
    impl ::core::ops::Deref for Tuple {
        type Target = !;
        #[inline]
        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    #[derive(DerefMut)]
    struct Struct {
        field: !,
    }

    // `Deref` implementation is required for `DerefMut`.
    impl ::core::ops::Deref for Struct {
        type Target = !;
        #[inline]
        fn deref(&self) -> &Self::Target {
            self.field
        }
    }
}

mod deprecated {
    use super::*;

    #[derive(DerefMut)]
    #[deprecated(note = "struct")]
    struct Tuple(#[deprecated(note = "field")] i32);

    // `Deref` implementation is required for `DerefMut`.
    #[allow(deprecated)]
    impl ::core::ops::Deref for Tuple {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(DerefMut)]
    #[deprecated(note = "struct")]
    struct Struct {
        #[deprecated(note = "field")]
        field: i32,
    }

    // `Deref` implementation is required for `DerefMut`.
    #[allow(deprecated)]
    impl ::core::ops::Deref for Struct {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.field
        }
    }
}
