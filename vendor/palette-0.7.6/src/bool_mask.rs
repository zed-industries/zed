//! Traits for abstracting over Boolean types.
//!
//! These traits are mainly useful for allowing SIMD values, where bit masks are
//! typically used instead of `bool`.

use core::ops::{BitAnd, BitOr, BitXor, Not};

#[cfg(feature = "wide")]
mod wide;

/// Associates a Boolean type to the implementing type.
///
/// This is primarily used in traits and functions that can accept SIMD values
/// and return a Boolean result. SIMD values use masks to select different values for
/// each lane and `HasBoolMask::Mask` can be used to know which type that mask
/// has.
pub trait HasBoolMask {
    /// The mask type to use for selecting `Self` values.
    type Mask: BoolMask;
}

impl<T> HasBoolMask for &'_ T
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> HasBoolMask for &'_ mut T
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> HasBoolMask for [T]
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T, const N: usize> HasBoolMask for [T; N]
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

macro_rules! impl_has_bool_mask {
    ($($ty:ident),+) => {
        $(
            impl HasBoolMask for $ty {
                type Mask = bool;
            }
        )+
    };
}

impl_has_bool_mask!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

/// Basic methods for boolean masks.
pub trait BoolMask {
    /// Create a new mask where each lane is set to `value`.
    #[must_use]
    fn from_bool(value: bool) -> Self;

    /// Checks if all lanes in the mask are `true`.
    #[must_use]
    fn is_true(&self) -> bool;

    /// Checks if all lanes in the mask are `false`.
    #[must_use]
    fn is_false(&self) -> bool;
}

impl BoolMask for bool {
    #[inline]
    fn from_bool(value: bool) -> Self {
        value
    }

    #[inline]
    fn is_true(&self) -> bool {
        *self
    }

    #[inline]
    fn is_false(&self) -> bool {
        !*self
    }
}

/// Makes a mask bale to select between two values.
pub trait Select<T>
where
    T: HasBoolMask<Mask = Self>,
{
    /// Select lanes from `a` when corresponding lanes in `self` are `true`, and
    /// select from `b` when `false`.
    #[must_use]
    fn select(self, a: T, b: T) -> T;
}

impl<T> Select<T> for bool
where
    T: HasBoolMask<Mask = Self>,
{
    #[inline(always)]
    fn select(self, a: T, b: T) -> T {
        if self {
            a
        } else {
            b
        }
    }
}

/// Like [`Select`], but can avoid evaluating the input.
pub trait LazySelect<T>: Select<T>
where
    T: HasBoolMask<Mask = Self>,
{
    /// Select lanes from the output of `a` when corresponding lanes in `self`
    /// are `true`, and select from the output of `b` when `false`. May avoid
    /// evaluating either option if it's not selected.
    #[must_use]
    fn lazy_select<A, B>(self, a: A, b: B) -> T
    where
        A: FnOnce() -> T,
        B: FnOnce() -> T;
}

impl<T> LazySelect<T> for bool
where
    T: HasBoolMask<Mask = Self>,
{
    #[inline(always)]
    fn lazy_select<A, B>(self, a: A, b: B) -> T
    where
        A: FnOnce() -> T,
        B: FnOnce() -> T,
    {
        if self {
            a()
        } else {
            b()
        }
    }
}

/// A helper trait that collects bit traits under one name.
pub trait BitOps:
    Sized
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + for<'a> BitAnd<&'a Self, Output = Self>
    + for<'a> BitOr<&'a Self, Output = Self>
    + for<'a> BitXor<&'a Self, Output = Self>
{
}

impl<T> BitOps for T where
    T: Sized
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + Not<Output = Self>
        + for<'a> BitAnd<&'a Self, Output = Self>
        + for<'a> BitOr<&'a Self, Output = Self>
        + for<'a> BitXor<&'a Self, Output = Self>
{
}
