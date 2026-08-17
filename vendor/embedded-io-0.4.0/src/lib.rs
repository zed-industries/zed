#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "async", feature(async_fn_in_trait, impl_trait_projections))]
#![cfg_attr(feature = "async", allow(incomplete_features))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

// mod fmt MUST go first, so that others see its macros.
mod fmt;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod asynch;
pub mod blocking;

pub mod adapters;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
/// Possible kinds of errors.
pub enum ErrorKind {
    /// Unspecified error kind.
    Other,
}

/// Error trait.
///
/// This trait allows generic code to do limited inspecting of errors,
/// to react differently to different kinds.
pub trait Error: core::fmt::Debug {
    /// Get the kind of this error.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

impl Error for ErrorKind {
    fn kind(&self) -> ErrorKind {
        *self
    }
}

/// Enumeration of possible methods to seek within an I/O object.
///
/// Semantics are the same as [`std::io::SeekFrom`], check its documentation for details.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeekFrom {
    /// Sets the offset to the provided number of bytes.
    Start(u64),
    /// Sets the offset to the size of this object plus the specified number of bytes.
    End(i64),
    /// Sets the offset to the current position plus the specified number of bytes.
    Current(i64),
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<SeekFrom> for std::io::SeekFrom {
    fn from(pos: SeekFrom) -> Self {
        match pos {
            SeekFrom::Start(n) => std::io::SeekFrom::Start(n),
            SeekFrom::End(n) => std::io::SeekFrom::End(n),
            SeekFrom::Current(n) => std::io::SeekFrom::Current(n),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<std::io::SeekFrom> for SeekFrom {
    fn from(pos: std::io::SeekFrom) -> SeekFrom {
        match pos {
            std::io::SeekFrom::Start(n) => SeekFrom::Start(n),
            std::io::SeekFrom::End(n) => SeekFrom::End(n),
            std::io::SeekFrom::Current(n) => SeekFrom::Current(n),
        }
    }
}

/// Base trait for all IO traits.
///
/// All IO operations of all traits return the error defined in this trait.
///
/// Having a shared trait instead of having every trait define its own
/// `io::Error` enforces all impls on the same type use the same error.
/// This is very convenient when writing generic code, it means you have to
/// handle a single error type `T::Error`, instead of `<T as Read>::Error`, `<T as Write>::Error`
/// which might be different types.
pub trait Io {
    /// Error type of all the IO operations on this type.
    type Error: Error;
}

impl<T: ?Sized + crate::Io> crate::Io for &mut T {
    type Error = T::Error;
}

impl crate::Io for &[u8] {
    type Error = core::convert::Infallible;
}

impl crate::Io for &mut [u8] {
    type Error = core::convert::Infallible;
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl<T: ?Sized + crate::Io> crate::Io for alloc::boxed::Box<T> {
    type Error = T::Error;
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
impl crate::Io for alloc::vec::Vec<u8> {
    type Error = core::convert::Infallible;
}
