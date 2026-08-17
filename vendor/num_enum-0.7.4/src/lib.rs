// Wrap this in two cfg_attrs so that it continues to parse pre-1.54.0.
// See https://github.com/rust-lang/rust/issues/82768
#![cfg_attr(feature = "external_doc", cfg_attr(all(), doc = include_str!("../README.md")))]
#![cfg_attr(
    not(feature = "external_doc"),
    doc = "See <https://docs.rs/num_enum> for more info about this crate."
)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use ::num_enum_derive::{
    Default, FromPrimitive, IntoPrimitive, TryFromPrimitive, UnsafeFromPrimitive,
};

use ::core::fmt;

pub trait FromPrimitive: Sized {
    type Primitive: Copy + Eq;

    fn from_primitive(number: Self::Primitive) -> Self;
}

pub trait TryFromPrimitive: Sized {
    type Primitive: Copy + Eq + fmt::Debug;
    type Error;

    const NAME: &'static str;

    fn try_from_primitive(number: Self::Primitive) -> Result<Self, Self::Error>;
}

pub trait UnsafeFromPrimitive: Sized {
    type Primitive: Copy + Eq;

    /// Transmutes into an enum from its primitive.
    ///
    /// # Safety
    ///
    /// - `number` must represent a valid discriminant of `Self`.
    #[deprecated(
        since = "0.6.0",
        note = "Prefer to use `unchecked_transmute_from`, `from_unchecked` will be removed in a future release."
    )]
    unsafe fn from_unchecked(number: Self::Primitive) -> Self {
        Self::unchecked_transmute_from(number)
    }

    /// Transmutes into an enum from its primitive.
    ///
    /// # Safety
    ///
    /// - `number` must represent a valid discriminant of `Self`.
    unsafe fn unchecked_transmute_from(number: Self::Primitive) -> Self;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TryFromPrimitiveError<Enum: TryFromPrimitive> {
    pub number: Enum::Primitive,
}

impl<Enum: TryFromPrimitive> TryFromPrimitiveError<Enum> {
    pub fn new(number: Enum::Primitive) -> Self {
        Self { number }
    }
}

impl<Enum: TryFromPrimitive> fmt::Debug for TryFromPrimitiveError<Enum> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("TryFromPrimitiveError")
            .field("number", &self.number)
            .finish()
    }
}
impl<Enum: TryFromPrimitive> fmt::Display for TryFromPrimitiveError<Enum> {
    fn fmt(&self, stream: &'_ mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            stream,
            "No discriminant in enum `{name}` matches the value `{input:?}`",
            name = Enum::NAME,
            input = self.number,
        )
    }
}

#[rustversion::since(1.81)]
impl<Enum: TryFromPrimitive> ::core::error::Error for TryFromPrimitiveError<Enum> {}

#[cfg(feature = "std")]
#[rustversion::before(1.81)]
impl<Enum: TryFromPrimitive> ::std::error::Error for TryFromPrimitiveError<Enum> {}

// This trait exists to try to give a more clear error message when someone attempts to derive both FromPrimitive and TryFromPrimitive.
// This isn't allowed because both end up creating a `TryFrom<primitive>` implementation.
// TryFromPrimitive explicitly implements TryFrom<primitive> with Error=TryFromPrimitiveError, which conflicts with:
// FromPrimitive explicitly implements From<primitive> which has a blanket implementation of TryFrom<primitive> with Error=Infallible.
//
// This is a private implementation detail of the num_enum crate which should not be depended on externally.
// It is subject to change in any release regardless of semver.
#[doc(hidden)]
pub trait CannotDeriveBothFromPrimitiveAndTryFromPrimitive {}
