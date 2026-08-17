//! Marker trait for types that can be written to.
//!
//!

use crate::fmt::{Formatter, StrWriter, StrWriterMut};

use core::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that can be written into.
///
/// # Implementors
///
/// Types that implement this trait are also expected to implement these inherent methods:
///
/// ```ignore
/// // use const_format::{FormattingFlags, Formatter};
///
/// const fn borrow_mutably(&mut self) -> &mut Self
/// const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_>
/// ```
///
/// # Coercions
///
/// The [`Kind`](#associatedtype.Kind) and [`This`](#associatedtype.This) associated types
/// are used in the [`IsAWriteMarker`] marker type
/// to convert a `&mut StrWriter<_>` to a `StrWriterMut<'_>`,
/// and leave other mutable references unconverted.
///
/// # Example
///
/// Implementing this trait for a String-like inline allocated type.
///
/// ```rust
///
/// use const_format::marker_traits::{IsNotAStrWriter, WriteMarker};
/// use const_format::{Formatter, FormattingFlags};
/// use const_format::writec;
///
/// mod arraystring {
///     use const_format::marker_traits::{IsNotAStrWriter, WriteMarker};
///     use const_format::{Formatter, FormattingFlags};
///
///     const ARRAY_CAP: usize = 64;
///     pub struct ArrayString  {
///         len: usize,
///         arr: [u8; ARRAY_CAP],
///     }
///    
///     impl WriteMarker for ArrayString  {
///         type Kind = IsNotAStrWriter;
///         type This = Self;
///     }
///    
///     impl ArrayString {
///         pub const fn new() -> Self {
///             Self { len: 0, arr: [0; ARRAY_CAP] }
///         }
///         
///         // Gets the part of the array that has been written to.
///         pub const fn as_bytes(&self) -> &[u8] {
///             const_format::utils::slice_up_to_len_alt(&self.arr, self.len)
///         }
///    
///         pub const fn borrow_mutably(&mut self) -> &mut Self {
///             self
///         }
///    
///         pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
///             Formatter::from_custom(&mut self.arr, &mut self.len, flags)
///         }
///     }
/// }
/// use arraystring::ArrayString;
///
///
/// let mut buffer = ArrayString::new();
///
/// writec!(buffer, "{:?}", [3u8, 5])?;
/// assert_eq!(buffer.as_bytes(), b"[3, 5]");
///
/// writec!(buffer, "{}{:b}", "Hello, world!", 100u16)?;
/// assert_eq!(buffer.as_bytes(), b"[3, 5]Hello, world!1100100");
///
/// # Ok::<(), const_format::Error>(())
/// ```
pub trait WriteMarker {
    /// Whether this is a StrWriter or not, this can be either of
    /// [`IsAStrWriter`] or [`IsNotAStrWriter`]
    ///
    /// [`IsAStrWriter`]: crate::marker_traits::IsAStrWriter
    /// [`IsNotAStrWriter`]: crate::marker_traits::IsNotAStrWriter
    type Kind;

    /// The type after dereferencing,
    /// implemented as `type This = Self;` for all non-reference types
    type This: ?Sized;
}

/// Marker type for `StrWriter`'s [`Kind`] in [`WriteMarker`]s
///
/// [`Kind`]: ./trait.WriteMarker.html#associatedtype.Kind
/// [`WriteMarker`]: ./trait.WriteMarker.html
///
pub struct IsAStrWriter;

/// Marker type for the [`Kind`] of all non-`StrWriter` types that implement [`WriteMarker`].
///
/// [`Kind`]: ./trait.WriteMarker.html#associatedtype.Kind
/// [`WriteMarker`]: ./trait.WriteMarker.html
///
pub struct IsNotAStrWriter;

///////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized> WriteMarker for StrWriter<T> {
    type Kind = IsAStrWriter;
    type This = Self;
}

impl WriteMarker for StrWriterMut<'_> {
    type Kind = IsNotAStrWriter;
    type This = Self;
}

impl WriteMarker for Formatter<'_> {
    type Kind = IsNotAStrWriter;
    type This = Self;
}

impl<T> WriteMarker for &T
where
    T: ?Sized + WriteMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}

impl<T> WriteMarker for &mut T
where
    T: ?Sized + WriteMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}

///////////////////////////////////////////////////////////////////////////////

/// Hack used to automatically convert a
/// mutable reference to a [`StrWriter`] to a [`StrWriterMut`],
/// and do nothing with other types.
///
/// The conversion is done with the `coerce` methods.
///
///
/// # Type parameters
///
/// `K` is `<R as WriteMarker>::Kind`
/// The kind of type that `T` is, either a [`IsAStrWriter`] or [`IsNotAStrWriter`]
///
/// `T` is `<R as WriteMarker>::This`:
/// The `R` type after removing all layers of references.
///
/// `R`: A type that implements `WriteMarker`.
///
/// # Coerce Method
///
/// The `coerce` method is what does the conversion from a `&mut T`
/// depending on the `K` type parameter:
///
/// - [`IsAStrWriter`]: the reference is converted into a `StrWriterMut<'_>`.
///
/// - [`IsNotAStrWriter`]: the reference is simply returned unchanged.
///  
///
/// [`StrWriter`]: ../fmt/struct.StrWriter.html
///
/// [`StrWriterMut`]: ../fmt/struct.StrWriterMut.html
///
/// [`IsAStrWriter`]: ./struct.IsAStrWriter.html
///
/// [`IsNotAStrWriter`]: ./struct.IsNotAStrWriter.html
///
#[allow(clippy::type_complexity)]
pub struct IsAWriteMarker<K, T: ?Sized, R: ?Sized>(
    PhantomData<(
        PhantomData<fn() -> PhantomData<K>>,
        PhantomData<fn() -> PhantomData<T>>,
        PhantomData<fn() -> PhantomData<R>>,
    )>,
);

impl<K, T: ?Sized, R: ?Sized> Copy for IsAWriteMarker<K, T, R> {}

impl<K, T: ?Sized, R: ?Sized> Clone for IsAWriteMarker<K, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> IsAWriteMarker<R::Kind, R::This, R>
where
    R: ?Sized + WriteMarker,
{
    /// Constructs a `IsAWriteMarker`
    pub const NEW: Self = Self(PhantomData);
}

/////////////////////////////////////////////////////////////////////////////

impl<K, T: ?Sized, R: ?Sized> IsAWriteMarker<K, T, R> {
    /// Infers the type parmaeters of this `IsAWriteMarker` with the passed reference.
    #[inline(always)]
    pub const fn infer_type(self, _: &R) -> Self {
        self
    }
}

/////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized, R: ?Sized> IsAWriteMarker<IsAStrWriter, StrWriter<T>, R> {
    /// Converts the `&mut StrWriter` to a `StrWriterMut<'_>`.
    #[inline(always)]
    pub const fn coerce(self, mutref: &mut StrWriter) -> StrWriterMut<'_> {
        mutref.as_mut()
    }
}

impl<T: ?Sized, R: ?Sized> IsAWriteMarker<IsNotAStrWriter, T, R> {
    /// An idntity function, just takes`mutref` and returns it.
    #[inline(always)]
    pub const fn coerce(self, mutref: &mut T) -> &mut T {
        mutref
    }
}

/////////////////////////////////////////////////////////////////////////////
