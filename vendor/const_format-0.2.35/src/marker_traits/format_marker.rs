//! Marker trait for types that implement the const formatting methods.
//!
//!

use crate::wrapper_types::PWrapper;

use core::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that implement the const formatting methods.
///
/// Debug formatting can be derived using the [`ConstDebug`] derive macro.
///
/// # Implementors
///
/// Types that implement this trait are also expected to implement at least one of
/// these inherent methods:
///
/// ```ignore
/// // use const_format::{Error, Format};
///
/// const fn const_debug_fmt(&self, &mut Formatter<'_>) -> Result<(), Error>
///
/// const fn const_display_fmt(&self, &mut Formatter<'_>) -> Result<(), Error>
/// ```
///
/// # Coercions
///
/// The [`Kind`](#associatedtype.Kind) and [`This`](#associatedtype.This) associated types
/// are used in the [`IsAFormatMarker`] marker type
/// to automatically wrap types in [`PWrapper`] if they're from the standard library,
/// otherwise leaving them unwrapped.
///
/// # Examples
///
/// ### Display formatting
///
/// This example demonstrates how you can implement display formatting,
/// without using the [`impl_fmt`] macro.
///
/// ```rust
///
/// use const_format::{
///     marker_traits::{FormatMarker, IsNotStdKind},
///     Error, Formatter, StrWriter,
///     formatc, writec,
/// };
///
/// use std::cmp::Ordering;
///
///
/// struct Compared(u32, Ordering, u32);
///
/// // This is what the `impl_fmt` macro implements implicitly for all non-std types
/// impl FormatMarker for Compared {
///     type Kind = IsNotStdKind;
///     type This = Self;
/// }
///
/// impl Compared {
///     pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         let op = match self.1 {
///             Ordering::Less => "<",
///             Ordering::Equal => "==",
///             Ordering::Greater => ">",
///         };
///         writec!(f, "{} {} {}", self.0, op, self.2)
///     }
/// }
///
/// const S_0: &str = formatc!("{}", Compared(0, Ordering::Less, 1));
/// const S_1: &str = formatc!("{}", Compared(1, Ordering::Equal, 1));
/// const S_2: &str = formatc!("{}", Compared(2, Ordering::Greater, 1));
///
/// assert_eq!(S_0, "0 < 1");
/// assert_eq!(S_1, "1 == 1");
/// assert_eq!(S_2, "2 > 1");
///
/// ```
///
/// ### Debug formatting
///
/// For examples of using the [`ConstDebug`] derive macro,
/// [look here](crate::ConstDebug#examples).
///
/// These are examples of implementing debug formatting using the `impl_fmt` macro for:
///
/// - [Tupled structs and tuple variants.](../fmt/struct.DebugTuple.html#example)
///
/// - [Braced structs and braced variants.](../fmt/struct.DebugStruct.html#example)
///
///
/// [`IsAFormatMarker`]: ./struct.IsAFormatMarker.html
/// [`ConstDebug`]: crate::ConstDebug
/// [`impl_fmt`]: ../macro.impl_fmt.html
///
pub trait FormatMarker {
    /// What kind of type this is, this can be one of:
    ///
    /// - [`IsArrayKind`]: For slices, and arrays.
    ///
    /// - [`IsStdKind`]: Any other standard library type.
    ///
    /// - [`IsNotStdKind`]: Any type that is not from the standard library.
    ///
    /// [`IsArrayKind`]: ./struct.IsArrayKind.html
    /// [`IsStdKind`]: ./struct.IsStdKind.html
    /// [`IsNotStdKind`]: ./struct.IsNotStdKind.html
    type Kind;

    /// The type after dereferencing,
    /// implemented as `type This = Self;` for all non-reference types
    type This: ?Sized;
}

/// Marker type for arrays and slices,
/// used as the [`Kind`] associated type  in [`FormatMarker`].
///
/// [`Kind`]: ./trait.FormatMarker.html#associatedtype.Kind
///
pub struct IsArrayKind<T>(PhantomData<T>);

/// Marker type for the remaining standard library types,,
/// used as the [`Kind`] associated type  in [`FormatMarker`].
///
/// [`Kind`]: ./trait.FormatMarker.html#associatedtype.Kind
///
pub struct IsStdKind;

/// Marker type for non-standard library types,
/// used as the [`Kind`] associated type  in [`FormatMarker`].
///
/// [`Kind`]: ./trait.FormatMarker.html#associatedtype.Kind
///
pub struct IsNotStdKind;

macro_rules! std_kind_impls {
    ($($ty:ty),* $(,)* ) => (
        $(
            impl FormatMarker for $ty {
                type Kind = IsStdKind;
                type This = Self;
            }

            impl<T> IsAFormatMarker<IsStdKind, $ty, T> {
                /// Copies the value from `reference`, and wraps it in a `PWrapper`
                #[inline(always)]
                pub const fn coerce(self, reference: &$ty) -> PWrapper<$ty> {
                    PWrapper(*reference)
                }
            }
        )*
    )
}

///////////////////////////////////////////////////////////////////////////////

/// Hack used to automatically wrap standard library types inside [`PWrapper`],
/// while leaving user defined types unwrapped.
///
/// # Type parameters
///
/// `K` is `<R as FormatMarker>::Kind`
/// The kind of type that `T` is,
/// [a slice](./struct.IsArrayKind.html),
/// [other std types](./struct.IsStdKind.html),
/// [non-std types](./struct.IsNotStdKind.html).
///
/// `T` is `<R as FormatMarker>::This`:
/// The `R` type after removing all layers of references.
///
/// `R`: a type that implements [`FormatMarker`].
///
/// # Coerce Method
///
/// The `coerce` method is what does the conversion from a `&T` depending on
/// the `K` type parameter:
///
/// - [`IsArrayKind`]: the reference is coerced to a slice, and wrapped in a [`PWrapper`].
///
/// - [`IsStdKind`]: the referenced value is copied, and wrapped in a [`PWrapper`].
///
/// - [`IsNotStdKind`]: the reference is simply returned as a `&T`.
///
#[allow(clippy::type_complexity)]
pub struct IsAFormatMarker<K, T: ?Sized, R: ?Sized>(
    PhantomData<(
        PhantomData<fn() -> PhantomData<K>>,
        PhantomData<fn() -> PhantomData<T>>,
        PhantomData<fn() -> PhantomData<R>>,
    )>,
);

impl<K, T: ?Sized, R: ?Sized> Copy for IsAFormatMarker<K, T, R> {}

impl<K, T: ?Sized, R: ?Sized> Clone for IsAFormatMarker<K, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> IsAFormatMarker<R::Kind, R::This, R>
where
    R: ?Sized + FormatMarker,
{
    /// Constructs an `IsAFormatMarker`
    pub const NEW: Self = Self(PhantomData);
}

impl<K, T: ?Sized, R: ?Sized> IsAFormatMarker<K, T, R> {
    /// Infers the type parameters by taking a reference to `R` .
    ///
    /// The `K` and `T` type parameters are determined by `R` in
    /// the [`NEW`] associated constant.
    ///
    /// [`NEW`]: #associatedconstant.NEW
    #[inline(always)]
    pub const fn infer_type(self, _: &R) -> Self {
        self
    }

    /// Removes layers of references by coercing the argument.
    #[inline(always)]
    pub const fn unreference(self, r: &T) -> &T {
        r
    }
}

/////////////////////////////////////////////////////////////////////////////

impl<U, T: ?Sized, R: ?Sized> IsAFormatMarker<IsArrayKind<U>, T, R> {
    /// Coerces an array to a slice, then wraps the slice in a `PWrapper`
    #[inline(always)]
    pub const fn coerce(self, slice: &[U]) -> PWrapper<&[U]> {
        PWrapper(slice)
    }
}

impl<T: ?Sized, R: ?Sized> IsAFormatMarker<IsNotStdKind, T, R> {
    /// An identity function, just takes `reference` and returns it.
    #[inline(always)]
    pub const fn coerce(self, reference: &T) -> &T {
        reference
    }
}

/////////////////////////////////////////////////////////////////////////////

std_kind_impls! {
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    i128, u128,
    isize, usize,
    bool, char,
}

impl FormatMarker for str {
    type Kind = IsStdKind;
    type This = Self;
}

impl<R: ?Sized> IsAFormatMarker<IsStdKind, str, R> {
    /// Wraps `reference` in a `PWrapper`.
    #[inline(always)]
    pub const fn coerce(self, reference: &str) -> PWrapper<&str> {
        PWrapper(reference)
    }
}

impl<T, const N: usize> FormatMarker for [T; N] {
    type Kind = IsArrayKind<T>;
    type This = Self;
}

impl<T> FormatMarker for [T] {
    type Kind = IsArrayKind<T>;
    type This = [T];
}

impl<T> FormatMarker for &T
where
    T: ?Sized + FormatMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}

impl<T> FormatMarker for &mut T
where
    T: ?Sized + FormatMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}
