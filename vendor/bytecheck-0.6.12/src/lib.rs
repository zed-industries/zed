//! # bytecheck
//!
//! bytecheck is a type validation framework for Rust.
//!
//! For some types, creating an invalid value immediately results in undefined
//! behavior. This can cause some issues when trying to validate potentially
//! invalid bytes, as just casting the bytes to your type can technically cause
//! errors. This makes it difficult to write validation routines, because until
//! you're certain that the bytes represent valid values you cannot cast them.
//!
//! bytecheck provides a framework for performing these byte-level validations
//! and implements checks for basic types along with a derive macro to implement
//! validation for custom structs and enums.
//!
//! ## Design
//!
//! [`CheckBytes`] is at the heart of bytecheck, and does the heavy lifting of
//! verifying that some bytes represent a valid type. Implementing it can be
//! done manually or automatically with the [derive macro](macro@CheckBytes).
//!
//! ## Examples
//!
//! ```
//! use bytecheck::CheckBytes;
//!
//! #[derive(CheckBytes, Debug)]
//! #[repr(C)]
//! struct Test {
//!     a: u32,
//!     b: char,
//!     c: bool,
//! }
//! #[repr(C, align(16))]
//! struct Aligned<const N: usize>([u8; N]);
//!
//! macro_rules! bytes {
//!     ($($byte:literal,)*) => {
//!         (&Aligned([$($byte,)*]).0 as &[u8]).as_ptr()
//!     };
//!     ($($byte:literal),*) => {
//!         bytes!($($byte,)*)
//!     };
//! }
//!
//! // This type is laid out as (u32, char, bool)
//! // In this example, the architecture is assumed to be little-endian
//! # #[cfg(target_endian = "little")]
//! unsafe {
//!     // These are valid bytes for (0, 'x', true)
//!     Test::check_bytes(
//!         bytes![
//!             0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8,
//!             1u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
//!         ].cast(),
//!         &mut ()
//!     ).unwrap();
//!
//!     // Changing the bytes for the u32 is OK, any bytes are a valid u32
//!     Test::check_bytes(
//!         bytes![
//!             42u8, 16u8, 20u8, 3u8, 0x78u8, 0u8, 0u8, 0u8,
//!             1u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
//!         ].cast(),
//!         &mut ()
//!     ).unwrap();
//!
//!     // Characters outside the valid ranges are invalid
//!     Test::check_bytes(
//!         bytes![
//!             0u8, 0u8, 0u8, 0u8, 0x00u8, 0xd8u8, 0u8, 0u8,
//!             1u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
//!         ].cast(),
//!         &mut ()
//!     ).unwrap_err();
//!     Test::check_bytes(
//!         bytes![
//!             0u8, 0u8, 0u8, 0u8, 0x00u8, 0x00u8, 0x11u8, 0u8,
//!             1u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
//!         ].cast(),
//!         &mut ()
//!     ).unwrap_err();
//!
//!     // 0 is a valid boolean value (false) but 2 is not
//!     Test::check_bytes(
//!         bytes![
//!             0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8,
//!             0u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
//!         ].cast(),
//!         &mut ()
//!     ).unwrap();
//!     Test::check_bytes(
//!         bytes![
//!             0u8, 0u8, 0u8, 0u8, 0x78u8, 0u8, 0u8, 0u8,
//!             2u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8
//!         ].cast(),
//!         &mut ()
//!     ).unwrap_err();
//! }
//! ```
//!
//! ## Features
//!
//! - `verbose`: Some validation algorithms are optimized for speed and do not report full error
//!   details by default. This feature provides full error information.
//! - `std`: Enables standard library support (enabled by default). If the `std` feature is not
//!   enabled, the `alloc` crate is required.
//!
//! ## Crate support
//!
//! Some common crates need to be supported by bytecheck before an official integration has been
//! made. Support is provided by bytecheck for these crates, but in the future crates should depend
//! on bytecheck and provide their own implementations. The crates that already have support
//! provided by bytecheck should work toward integrating the implementations into themselves.
//!
//! Crates supported by bytecheck:
//!
//! - [`uuid`](https://docs.rs/uuid)

#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    missing_docs,
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

// Support for various common crates. These are primarily to get users off the ground and build some
// momentum.

// These are NOT PLANNED to remain in bytecheck for the final release. Much like serde, these
// implementations should be moved into their respective crates over time. Before adding support for
// another crate, please consider getting bytecheck support in the crate instead.

#[cfg(feature = "uuid")]
pub mod uuid;

#[cfg(not(feature = "simdutf8"))]
use core::str::{from_utf8, Utf8Error};
#[cfg(has_atomics)]
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicU16, AtomicU32, AtomicU8,
};
#[cfg(has_atomics_64)]
use core::sync::atomic::{AtomicI64, AtomicU64};
use core::{
    convert::{Infallible, TryFrom},
    fmt,
    marker::{PhantomData, PhantomPinned},
    mem::ManuallyDrop,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
        NonZeroU32, NonZeroU64, NonZeroU8,
    },
    ops, ptr, slice,
};
use ptr_meta::PtrExt;
#[cfg(all(feature = "simdutf8", not(feature = "verbose")))]
use simdutf8::basic::{from_utf8, Utf8Error};
#[cfg(all(feature = "simdutf8", feature = "verbose"))]
use simdutf8::compat::{from_utf8, Utf8Error};

pub use bytecheck_derive::CheckBytes;

/// An error that can be debugged and displayed.
///
/// With the `std` feature, this also supports `std::error::Error`.
#[cfg(not(feature = "std"))]
pub trait Error: fmt::Debug + fmt::Display + 'static {}

#[cfg(not(feature = "std"))]
impl<T: fmt::Debug + fmt::Display + 'static> Error for T {}

/// An error that can be debugged and displayed.
///
/// With the `std` feature, this also supports `std::error::Error`.
#[cfg(feature = "std")]
pub trait Error: std::error::Error + 'static {
    /// Gets this error as an `std::error::Error`.
    fn as_error(&self) -> &(dyn std::error::Error + 'static);
}

#[cfg(feature = "std")]
impl<T: std::error::Error + 'static> Error for T {
    fn as_error(&self) -> &(dyn std::error::Error + 'static) {
        self
    }
}

/// The type used for boxing errors.
#[cfg(not(feature = "std"))]
pub type ErrorBox<E> = alloc::boxed::Box<E>;

/// The type used for boxing errors.
#[cfg(feature = "std")]
pub type ErrorBox<E> = std::boxed::Box<E>;

/// A type that can check whether a pointer points to a valid value.
///
/// `CheckBytes` can be derived with [`CheckBytes`](macro@CheckBytes) or
/// implemented manually for custom behavior.
pub trait CheckBytes<C: ?Sized> {
    /// The error that may result from checking the type.
    type Error: Error + 'static;

    /// Checks whether the given pointer points to a valid value within the
    /// given context.
    ///
    /// # Safety
    ///
    /// The passed pointer must be aligned and point to enough bytes to
    /// represent the type.
    unsafe fn check_bytes<'a>(value: *const Self, context: &mut C)
        -> Result<&'a Self, Self::Error>;
}

macro_rules! impl_primitive {
    ($type:ty) => {
        impl<C: ?Sized> CheckBytes<C> for $type {
            type Error = Infallible;

            #[inline]
            unsafe fn check_bytes<'a>(
                value: *const Self,
                _: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                Ok(&*value)
            }
        }
    };
}

impl_primitive!(());
impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(i128);
impl_primitive!(u8);
impl_primitive!(u16);
impl_primitive!(u32);
impl_primitive!(u64);
impl_primitive!(u128);
impl_primitive!(f32);
impl_primitive!(f64);
#[cfg(has_atomics)]
impl_primitive!(AtomicI8);
#[cfg(has_atomics)]
impl_primitive!(AtomicI16);
#[cfg(has_atomics)]
impl_primitive!(AtomicI32);
#[cfg(has_atomics_64)]
impl_primitive!(AtomicI64);
#[cfg(has_atomics)]
impl_primitive!(AtomicU8);
#[cfg(has_atomics)]
impl_primitive!(AtomicU16);
#[cfg(has_atomics)]
impl_primitive!(AtomicU32);
#[cfg(has_atomics_64)]
impl_primitive!(AtomicU64);

impl<T: ?Sized, C: ?Sized> CheckBytes<C> for PhantomData<T> {
    type Error = Infallible;

    #[inline]
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        Ok(&*value)
    }
}

impl<C: ?Sized> CheckBytes<C> for PhantomPinned {
    type Error = Infallible;

    #[inline]
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        Ok(&*value)
    }
}

impl<C: ?Sized, T: CheckBytes<C> + ?Sized> CheckBytes<C> for ManuallyDrop<T> {
    type Error = T::Error;

    #[inline]
    unsafe fn check_bytes<'a>(value: *const Self, c: &mut C) -> Result<&'a Self, Self::Error> {
        let _ = T::check_bytes(value as *const T, c)?;
        Ok(&*value)
    }
}

/// An error resulting from an invalid boolean.
///
/// Booleans are one byte and may only have the value 0 or 1.
#[derive(Debug)]
pub struct BoolCheckError {
    /// The byte value of the invalid boolean
    pub invalid_value: u8,
}

impl fmt::Display for BoolCheckError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "check failed for bool: expected 0 or 1, found {}",
            self.invalid_value
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BoolCheckError {}

impl<C: ?Sized> CheckBytes<C> for bool {
    type Error = BoolCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        let byte = *value.cast::<u8>();
        match byte {
            0 | 1 => Ok(&*value),
            _ => Err(BoolCheckError {
                invalid_value: byte,
            }),
        }
    }
}

#[cfg(has_atomics)]
impl<C: ?Sized> CheckBytes<C> for AtomicBool {
    type Error = BoolCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        let byte = *value.cast::<u8>();
        match byte {
            0 | 1 => Ok(&*value),
            _ => Err(BoolCheckError {
                invalid_value: byte,
            }),
        }
    }
}

/// An error resulting from an invalid character.
///
/// Characters are four bytes and may only have values from `0x0` to `0xD7FF`
/// and `0xE000` to `0x10FFFF` inclusive.
#[derive(Debug)]
pub struct CharCheckError {
    /// The `u32` value of the invalid character
    pub invalid_value: u32,
}

impl From<Infallible> for CharCheckError {
    #[inline]
    fn from(_: Infallible) -> Self {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

impl fmt::Display for CharCheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "check failed for char: invalid value {}",
            self.invalid_value
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CharCheckError {}

impl<C: ?Sized> CheckBytes<C> for char {
    type Error = CharCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        let c = *u32::check_bytes(value.cast(), context)?;
        char::try_from(c).map_err(|_| CharCheckError { invalid_value: c })?;
        Ok(&*value)
    }
}

macro_rules! peel_tuple {
    ([$($error_rest:ident,)*], [$type:ident $index:tt, $($type_rest:ident $index_rest:tt,)*]) => { impl_tuple! { [$($error_rest,)*], [$($type_rest $index_rest,)*] } };
}

macro_rules! impl_tuple {
    ([], []) => {};
    ([$error:ident, $($error_rest:ident,)*], [$($type:ident $index:tt,)+]) => {
        /// An error resulting from an invalid tuple.
        #[derive(Debug)]
        pub enum $error<$($type),+> {
            $(
                /// The given tuple member was invalid.
                $type($type),
            )+
        }

        impl<$($type: fmt::Display),*> fmt::Display for $error<$($type),+> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                const SIZE: usize = [$($index,)+].len();
                match self {
                    $($error::$type(e) => write!(f, "check failed for {}-tuple index {}: {}", SIZE, SIZE - $index - 1, e),)+
                }
            }
        }

        #[cfg(feature = "std")]
        impl<$($type: fmt::Display + fmt::Debug),*> std::error::Error for $error<$($type),+> {}

        impl<$($type: CheckBytes<C>,)+ C: ?Sized> CheckBytes<C> for ($($type,)+) {
            type Error = $error<$($type::Error),+>;

            #[inline]
            #[allow(clippy::unneeded_wildcard_pattern)]
            unsafe fn check_bytes<'a>(value: *const Self, context: &mut C) -> Result<&'a Self, Self::Error> {
                let field_bytes = ($(ptr::addr_of!((*value).$index),)+);
                impl_tuple!(@check_fields field_bytes, $error, context, $($type $index,)*);
                Ok(&*value)
            }
        }

        peel_tuple! {
            [$($error_rest,)*],
            [$($type $index,)+]
        }
    };
    (@check_fields $field_bytes:ident, $error:ident, $context:ident,) => {};
    (@check_fields $field_bytes:ident, $error:ident, $context:ident, $type:ident $index:tt, $($type_rest:ident $index_rest:tt,)*) => {
        impl_tuple!(@check_fields $field_bytes, $error, $context, $($type_rest $index_rest,)*);
        $type::check_bytes($field_bytes.$index, $context).map_err($error::$type)?;
    };
}

impl_tuple! {
    [Tuple12CheckError, Tuple11CheckError, Tuple10CheckError, Tuple9CheckError, Tuple8CheckError, Tuple7CheckError, Tuple6CheckError, Tuple5CheckError, Tuple4CheckError, Tuple3CheckError, Tuple2CheckError, Tuple1CheckError, ],
    [T11 11, T10 10, T9 9, T8 8, T7 7, T6 6, T5 5, T4 4, T3 3, T2 2, T1 1, T0 0, ]
}

/// An error resulting from an invalid array.
#[derive(Debug)]
pub struct ArrayCheckError<T> {
    /// The index of the invalid element
    pub index: usize,
    /// The error that occured while validating the invalid element
    pub error: T,
}

impl<T: fmt::Display> fmt::Display for ArrayCheckError<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "check failed for array index {}: {}",
            self.index, self.error
        )
    }
}

#[cfg(feature = "std")]
impl<T: fmt::Debug + fmt::Display> std::error::Error for ArrayCheckError<T> {}

impl<T: CheckBytes<C>, C: ?Sized, const N: usize> CheckBytes<C> for [T; N] {
    type Error = ArrayCheckError<T::Error>;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        for index in 0..N {
            let el = value.cast::<T>().add(index);
            T::check_bytes(el, context).map_err(|error| ArrayCheckError { index, error })?;
        }
        Ok(&*value)
    }
}

/// An error resulting from an invalid slice.
#[derive(Debug)]
pub enum SliceCheckError<T> {
    /// An element of the slice failed to validate
    CheckBytes {
        /// The index of the invalid element
        index: usize,
        /// The error that occured while validating the invalid element
        error: T,
    },
}

impl<T: fmt::Display> fmt::Display for SliceCheckError<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SliceCheckError::CheckBytes { index, error } => {
                write!(f, "check failed for slice index {}: {}", index, error)
            }
        }
    }
}

#[cfg(feature = "std")]
impl<T: fmt::Debug + fmt::Display> std::error::Error for SliceCheckError<T> {}

impl<T: CheckBytes<C>, C: ?Sized> CheckBytes<C> for [T] {
    type Error = SliceCheckError<T::Error>;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        let (data, len) = PtrExt::to_raw_parts(value);
        for index in 0..len {
            let el = data.cast::<T>().add(index);
            T::check_bytes(el, context)
                .map_err(|error| SliceCheckError::CheckBytes { index, error })?;
        }
        Ok(&*value)
    }
}

/// An error resulting from an invalid str.
#[derive(Debug)]
pub enum StrCheckError {
    /// The UTF-8 string failed to validate
    Utf8Error(Utf8Error),
}

impl From<Utf8Error> for StrCheckError {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}

impl fmt::Display for StrCheckError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrCheckError::Utf8Error(e) => write!(f, "utf8 error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for StrCheckError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StrCheckError::Utf8Error(e) => Some(e),
        }
    }
}

impl<C: ?Sized> CheckBytes<C> for str {
    type Error = StrCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        let (data, len) = PtrExt::to_raw_parts(value);
        from_utf8(slice::from_raw_parts(data.cast(), len))?;
        Ok(&*value)
    }
}

/// An error resulting from an invalid `CStr`.
#[cfg(feature = "std")]
#[derive(Debug)]
pub enum CStrCheckError {
    /// The UTF-8 C string failed to validate
    Utf8Error(Utf8Error),
    /// The string did not end with a null terminator
    MissingNullTerminator,
}

#[cfg(feature = "std")]
impl From<Utf8Error> for CStrCheckError {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for CStrCheckError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CStrCheckError::Utf8Error(e) => write!(f, "utf8 error: {}", e),
            CStrCheckError::MissingNullTerminator => write!(f, "missing null terminator"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CStrCheckError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CStrCheckError::Utf8Error(e) => Some(e),
            CStrCheckError::MissingNullTerminator => None,
        }
    }
}

#[cfg(feature = "std")]
impl<C: ?Sized> CheckBytes<C> for ::std::ffi::CStr {
    type Error = CStrCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        let (data, len) = PtrExt::to_raw_parts(value);
        if len == 0 {
            Err(CStrCheckError::MissingNullTerminator)
        } else {
            from_utf8(slice::from_raw_parts(data.cast(), len - 1))?;
            if *data.cast::<u8>().add(len - 1) != 0 {
                Err(CStrCheckError::MissingNullTerminator)
            } else {
                Ok(&*value)
            }
        }
    }
}

/// An error resulting from an invalid struct.
#[derive(Debug)]
pub struct StructCheckError {
    /// The name of the struct field that was invalid
    pub field_name: &'static str,
    /// The error that occurred while validating the field
    pub inner: ErrorBox<dyn Error>,
}

impl fmt::Display for StructCheckError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "check failed for struct member {}: {}",
            self.field_name, self.inner
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for StructCheckError {}

/// An error resulting from an invalid tuple struct.
#[derive(Debug)]
pub struct TupleStructCheckError {
    /// The index of the struct field that was invalid
    pub field_index: usize,
    /// The error that occurred while validating the field
    pub inner: ErrorBox<dyn Error>,
}

impl fmt::Display for TupleStructCheckError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "check failed for tuple struct member {}: {}",
            self.field_index, self.inner
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TupleStructCheckError {}

/// An error resulting from an invalid enum.
#[derive(Debug)]
pub enum EnumCheckError<T> {
    /// A struct variant was invalid
    InvalidStruct {
        /// The name of the variant that was invalid
        variant_name: &'static str,
        /// The error that occurred while validating the variant
        inner: StructCheckError,
    },
    /// A tuple variant was invalid
    InvalidTuple {
        /// The name of the variant that was invalid
        variant_name: &'static str,
        /// The error that occurred while validating the variant
        inner: TupleStructCheckError,
    },
    /// The enum tag was invalid
    InvalidTag(
        /// The invalid value of the tag
        T,
    ),
}

impl<T: fmt::Display> fmt::Display for EnumCheckError<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnumCheckError::InvalidStruct {
                variant_name,
                inner,
            } => write!(
                f,
                "check failed for enum struct variant {}: {}",
                variant_name, inner
            ),
            EnumCheckError::InvalidTuple {
                variant_name,
                inner,
            } => write!(
                f,
                "check failed for enum tuple variant {}: {}",
                variant_name, inner
            ),
            EnumCheckError::InvalidTag(tag) => write!(f, "invalid tag for enum: {}", tag),
        }
    }
}

#[cfg(feature = "std")]
impl<T: fmt::Debug + fmt::Display> std::error::Error for EnumCheckError<T> {}

// Range types
impl<T: CheckBytes<C>, C: ?Sized> CheckBytes<C> for ops::Range<T> {
    type Error = StructCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        T::check_bytes(ptr::addr_of!((*value).start), context).map_err(|error| {
            StructCheckError {
                field_name: "start",
                inner: ErrorBox::new(error),
            }
        })?;
        T::check_bytes(ptr::addr_of!((*value).end), context).map_err(|error| StructCheckError {
            field_name: "end",
            inner: ErrorBox::new(error),
        })?;
        Ok(&*value)
    }
}

impl<T: CheckBytes<C>, C: ?Sized> CheckBytes<C> for ops::RangeFrom<T> {
    type Error = StructCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        let bytes = value.cast::<u8>();
        T::check_bytes(ptr::addr_of!((*value).start), context).map_err(|error| {
            StructCheckError {
                field_name: "start",
                inner: ErrorBox::new(error),
            }
        })?;
        Ok(&*bytes.cast())
    }
}

impl<C: ?Sized> CheckBytes<C> for ops::RangeFull {
    type Error = Infallible;

    #[inline]
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        Ok(&*value)
    }
}

impl<T: CheckBytes<C>, C: ?Sized> CheckBytes<C> for ops::RangeTo<T> {
    type Error = StructCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        T::check_bytes(ptr::addr_of!((*value).end), context).map_err(|error| StructCheckError {
            field_name: "end",
            inner: ErrorBox::new(error),
        })?;
        Ok(&*value)
    }
}

impl<T: CheckBytes<C>, C: ?Sized> CheckBytes<C> for ops::RangeToInclusive<T> {
    type Error = StructCheckError;

    #[inline]
    unsafe fn check_bytes<'a>(
        value: *const Self,
        context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
        T::check_bytes(ptr::addr_of!((*value).end), context).map_err(|error| StructCheckError {
            field_name: "end",
            inner: ErrorBox::new(error),
        })?;
        Ok(&*value)
    }
}

/// An error resulting from an invalid `NonZero` integer.
#[derive(Debug)]
pub enum NonZeroCheckError {
    /// The integer was zero
    IsZero,
}

impl From<Infallible> for NonZeroCheckError {
    #[inline]
    fn from(_: Infallible) -> Self {
        unsafe { core::hint::unreachable_unchecked() }
    }
}

impl fmt::Display for NonZeroCheckError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NonZeroCheckError::IsZero => write!(f, "nonzero integer is zero"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NonZeroCheckError {}

macro_rules! impl_nonzero {
    ($nonzero:ident, $underlying:ident) => {
        impl<C: ?Sized> CheckBytes<C> for $nonzero {
            type Error = NonZeroCheckError;

            #[inline]
            unsafe fn check_bytes<'a>(
                value: *const Self,
                context: &mut C,
            ) -> Result<&'a Self, Self::Error> {
                if *$underlying::check_bytes(value.cast(), context)? == 0 {
                    Err(NonZeroCheckError::IsZero)
                } else {
                    Ok(&*value)
                }
            }
        }
    };
}

impl_nonzero!(NonZeroI8, i8);
impl_nonzero!(NonZeroI16, i16);
impl_nonzero!(NonZeroI32, i32);
impl_nonzero!(NonZeroI64, i64);
impl_nonzero!(NonZeroI128, i128);
impl_nonzero!(NonZeroU8, u8);
impl_nonzero!(NonZeroU16, u16);
impl_nonzero!(NonZeroU32, u32);
impl_nonzero!(NonZeroU64, u64);
impl_nonzero!(NonZeroU128, u128);
