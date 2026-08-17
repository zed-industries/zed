//! `AsSlice` and `AsMutSlice` traits
//!
//! These traits are somewhat similar to the `AsRef` and `AsMut` except that they are **NOT**
//! polymorphic (no input type parameter) and their methods always return slices (`[T]`).
//!
//! The main use case of these traits is writing generic code that accepts (fixed size) buffers. For
//! example, a bound `T: StableDeref + AsMutSlice<Element = u8> + 'static` will accepts types like
//! `&'static mut [u8]` and `&'static mut [u8; 128]` -- all
//! of them are appropriate for DMA transfers.
//!
//! # Minimal Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.51 and up. It *might* compile on older
//! versions but that may change in any new patch release.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate stable_deref_trait;

/// Something that can be seen as an immutable slice
pub trait AsSlice {
    /// The element type of the slice view
    type Element;

    /// Returns the immutable slice view of `Self`
    fn as_slice(&self) -> &[Self::Element];
}

/// Something that can be seen as an mutable slice
pub trait AsMutSlice: AsSlice {
    /// Returns the mutable slice view of `Self`
    fn as_mut_slice(&mut self) -> &mut [Self::Element];
}

impl<'a, S> AsSlice for &'a S
where
    S: ?Sized + AsSlice,
{
    type Element = S::Element;

    fn as_slice(&self) -> &[S::Element] {
        (**self).as_slice()
    }
}

impl<'a, S> AsSlice for &'a mut S
where
    S: ?Sized + AsSlice,
{
    type Element = S::Element;

    fn as_slice(&self) -> &[S::Element] {
        (**self).as_slice()
    }
}

impl<'a, S> AsMutSlice for &'a mut S
where
    S: ?Sized + AsMutSlice,
{
    fn as_mut_slice(&mut self) -> &mut [S::Element] {
        (**self).as_mut_slice()
    }
}

impl<T> AsSlice for [T] {
    type Element = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T> AsMutSlice for [T] {
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

impl<T, const N: usize> AsSlice for [T; N] {
    type Element = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T, const N: usize> AsMutSlice for [T; N] {
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}
