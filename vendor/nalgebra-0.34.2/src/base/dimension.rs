#![allow(missing_docs)]

//! Traits and tags for identifying the dimension of all algebraic entities.

use std::any::{Any, TypeId};
use std::cmp;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use typenum::{self, Diff, Max, Maximum, Min, Minimum, Prod, Quot, Sum, Unsigned};

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;
#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Dim of dynamically-sized algebraic entities.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv-serialize",
    archive_attr(derive(bytecheck::CheckBytes))
)]
pub struct Dyn(pub usize);

#[deprecated(note = "use Dyn instead.")]
pub type Dynamic = Dyn;

impl Dyn {
    /// A dynamic size equal to `value`.
    #[inline]
    #[deprecated(note = "use Dyn(value) instead.")]
    pub const fn new(value: usize) -> Self {
        Self(value)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl Serialize for Dyn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'de> Deserialize<'de> for Dyn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        usize::deserialize(deserializer).map(Dyn)
    }
}

/// Trait implemented by `Dyn`.
pub trait IsDynamic {}
/// Trait implemented by `Dyn` and type-level integers different from `U1`.
pub trait IsNotStaticOne {}

impl IsDynamic for Dyn {}
impl IsNotStaticOne for Dyn {}

/// Trait implemented by any type that can be used as a dimension. This includes type-level
/// integers and `Dyn` (for dimensions not known at compile-time).
///
/// # Safety
///
/// Hoists integers to the type level, including binary operations.
pub unsafe trait Dim: Any + Debug + Copy + PartialEq + Send + Sync {
    #[inline(always)]
    fn is<D: Dim>() -> bool {
        TypeId::of::<Self>() == TypeId::of::<D>()
    }

    /// Gets the compile-time value of `Self`. Returns `None` if it is not known, i.e., if `Self =
    /// Dyn`.
    fn try_to_usize() -> Option<usize>;

    /// Gets the run-time value of `self`. For type-level integers, this is the same as
    /// `Self::try_to_usize().unwrap()`.
    fn value(&self) -> usize;

    /// Builds an instance of `Self` from a run-time value. Panics if `Self` is a type-level
    /// integer and `dim != Self::try_to_usize().unwrap()`.
    fn from_usize(dim: usize) -> Self;
}

unsafe impl Dim for Dyn {
    #[inline]
    fn try_to_usize() -> Option<usize> {
        None
    }

    #[inline]
    fn from_usize(dim: usize) -> Self {
        Self(dim)
    }

    #[inline]
    fn value(&self) -> usize {
        self.0
    }
}

impl Add<usize> for Dyn {
    type Output = Dyn;

    #[inline]
    fn add(self, rhs: usize) -> Self {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for Dyn {
    type Output = Dyn;

    #[inline]
    fn sub(self, rhs: usize) -> Self {
        Self(self.0 - rhs)
    }
}

/*
 *
 * Operations.
 *
 */

macro_rules! dim_ops(
    ($($DimOp:    ident, $DimNameOp: ident,
       $Op:       ident, $op: ident, $op_path: path,
       $DimResOp: ident, $DimNameResOp: ident,
       $ResOp: ident);* $(;)*) => {$(
        pub type $DimResOp<D1, D2> = <D1 as $DimOp<D2>>::Output;

        pub trait $DimOp<D: Dim>: Dim {
            type Output: Dim;

            fn $op(self, other: D) -> Self::Output;
        }

        impl<const A: usize, const B: usize> $DimOp<Const<B>> for Const<A>
        where
            Const<A>: ToTypenum,
            Const<B>: ToTypenum,
            <Const<A> as ToTypenum>::Typenum: $Op<<Const<B> as ToTypenum>::Typenum>,
            $ResOp<<Const<A> as ToTypenum>::Typenum, <Const<B> as ToTypenum>::Typenum>: ToConst,
        {
            type Output =
                <$ResOp<<Const<A> as ToTypenum>::Typenum, <Const<B> as ToTypenum>::Typenum> as ToConst>::Const;

            fn $op(self, _: Const<B>) -> Self::Output {
                Self::Output::name()
            }
        }

        impl<D: Dim> $DimOp<D> for Dyn {
            type Output = Dyn;

            #[inline]
            fn $op(self, other: D) -> Dyn {
                Dyn($op_path(self.value(), other.value()))
            }
        }

        // TODO: use Const<T> instead of D: DimName?
        impl<D: DimName> $DimOp<Dyn> for D {
            type Output = Dyn;

            #[inline]
            fn $op(self, other: Dyn) -> Dyn {
                Dyn($op_path(self.value(), other.value()))
            }
        }

        pub type $DimNameResOp<D1, D2> = <D1 as $DimNameOp<D2>>::Output;

        pub trait $DimNameOp<D: DimName>: DimName {
            type Output: DimName;

            fn $op(self, other: D) -> Self::Output;
        }

        impl<const A: usize, const B: usize> $DimNameOp<Const<B>> for Const<A>
        where
            Const<A>: ToTypenum,
            Const<B>: ToTypenum,
            <Const<A> as ToTypenum>::Typenum: $Op<<Const<B> as ToTypenum>::Typenum>,
            $ResOp<<Const<A> as ToTypenum>::Typenum, <Const<B> as ToTypenum>::Typenum>: ToConst,
        {
            type Output =
                <$ResOp<<Const<A> as ToTypenum>::Typenum, <Const<B> as ToTypenum>::Typenum> as ToConst>::Const;

            fn $op(self, _: Const<B>) -> Self::Output {
                Self::Output::name()
            }
        }
   )*}
);

dim_ops!(
    DimAdd, DimNameAdd, Add, add, Add::add, DimSum,     DimNameSum,     Sum;
    DimMul, DimNameMul, Mul, mul, Mul::mul, DimProd,    DimNameProd,    Prod;
    DimSub, DimNameSub, Sub, sub, Sub::sub, DimDiff,    DimNameDiff,    Diff;
    DimDiv, DimNameDiv, Div, div, Div::div, DimQuot,    DimNameQuot,    Quot;
    DimMin, DimNameMin, Min, min, cmp::min, DimMinimum, DimNameMinimum, Minimum;
    DimMax, DimNameMax, Max, max, cmp::max, DimMaximum, DimNameMaximum, Maximum;
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(as = "Self")
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Const<const R: usize>;

/// Trait implemented exclusively by type-level integers.
pub trait DimName: Dim {
    const DIM: usize;

    /// The name of this dimension, i.e., the singleton `Self`.
    fn name() -> Self;

    // TODO: this is not a very idiomatic name.
    /// The value of this dimension.
    fn dim() -> usize;
}

#[cfg(feature = "serde-serialize-no-std")]
impl<const D: usize> Serialize for Const<D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ().serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'de, const D: usize> Deserialize<'de> for Const<D> {
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'de>,
    {
        <()>::deserialize(deserializer).map(|_| Const::<D>)
    }
}

pub trait ToConst {
    type Const: DimName;
}

pub trait ToTypenum {
    type Typenum: Unsigned;
}

unsafe impl<const T: usize> Dim for Const<T> {
    #[inline]
    fn try_to_usize() -> Option<usize> {
        Some(T)
    }

    #[inline]
    fn value(&self) -> usize {
        T
    }

    #[inline]
    fn from_usize(dim: usize) -> Self {
        assert_eq!(dim, T);
        Self
    }
}

impl<const T: usize> DimName for Const<T> {
    const DIM: usize = T;

    #[inline]
    fn name() -> Self {
        Self
    }

    #[inline]
    fn dim() -> usize {
        T
    }
}

pub type U1 = Const<1>;

impl ToTypenum for Const<1> {
    type Typenum = typenum::U1;
}

impl ToConst for typenum::U1 {
    type Const = Const<1>;
}

macro_rules! from_to_typenum (
    ($($D: ident, $VAL: expr_2021);* $(;)*) => {$(
        pub type $D = Const<$VAL>;

        impl ToTypenum for Const<$VAL> {
            type Typenum = typenum::$D;
        }

        impl ToConst for typenum::$D {
            type Const = Const<$VAL>;
        }

        impl IsNotStaticOne for $D { }

        /// The constant dimension
        #[doc = stringify!($VAL)]
        /// .
        pub const $D: $D = Const::<$VAL>;
    )*}
);

from_to_typenum!(
    U0, 0; /*U1,1;*/ U2, 2; U3, 3; U4, 4; U5, 5; U6, 6; U7, 7; U8, 8; U9, 9; U10, 10; U11, 11; U12, 12; U13, 13; U14, 14; U15, 15; U16, 16; U17, 17; U18, 18;
    U19, 19; U20, 20; U21, 21; U22, 22; U23, 23; U24, 24; U25, 25; U26, 26; U27, 27; U28, 28; U29, 29; U30, 30; U31, 31; U32, 32; U33, 33; U34, 34; U35, 35; U36, 36; U37, 37;
    U38, 38; U39, 39; U40, 40; U41, 41; U42, 42; U43, 43; U44, 44; U45, 45; U46, 46; U47, 47; U48, 48; U49, 49; U50, 50; U51, 51; U52, 52; U53, 53; U54, 54; U55, 55; U56, 56;
    U57, 57; U58, 58; U59, 59; U60, 60; U61, 61; U62, 62; U63, 63; U64, 64; U65, 65; U66, 66; U67, 67; U68, 68; U69, 69; U70, 70; U71, 71; U72, 72; U73, 73; U74, 74; U75, 75;
    U76, 76; U77, 77; U78, 78; U79, 79; U80, 80; U81, 81; U82, 82; U83, 83; U84, 84; U85, 85; U86, 86; U87, 87; U88, 88; U89, 89; U90, 90; U91, 91; U92, 92; U93, 93; U94, 94;
    U95, 95; U96, 96; U97, 97; U98, 98; U99, 99; U100, 100; U101, 101; U102, 102; U103, 103; U104, 104; U105, 105; U106, 106; U107, 107; U108, 108; U109, 109; U110, 110;
    U111, 111; U112, 112; U113, 113; U114, 114; U115, 115; U116, 116; U117, 117; U118, 118; U119, 119; U120, 120; U121, 121; U122, 122; U123, 123; U124, 124; U125, 125; U126, 126;
    U127, 127
);

/// The constant dimension 1.
// Note: We add U1 separately since it's not covered by the from_to_typenum! macro.
pub const U1: U1 = Const::<1>;
