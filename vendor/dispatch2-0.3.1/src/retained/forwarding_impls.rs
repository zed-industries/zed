//! Trivial forwarding impls on `Retained`-like types.
//!
//! Kept here to keep `retained.rs` free from this boilerplate, and to allow
//! re-use in `objc2-core-foundation` and `dispatch2` (this file is symlinked
//! there as well).

#![forbid(unsafe_code)]

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::future::Future;
use core::hash;
use core::pin::Pin;
use core::task::{Context, Poll};
#[cfg(feature = "std")] // TODO: Use core::error::Error once 1.81 is in MSRV
use std::error::Error;
#[cfg(feature = "std")]
use std::io;

use super::Retained;

impl<T: ?Sized + PartialEq<U>, U: ?Sized> PartialEq<Retained<U>> for Retained<T> {
    #[inline]
    fn eq(&self, other: &Retained<U>) -> bool {
        (**self).eq(&**other)
    }

    #[inline]
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Retained<U>) -> bool {
        (**self).ne(&**other)
    }
}

impl<T: Eq + ?Sized> Eq for Retained<T> {}

impl<T: ?Sized + PartialOrd<U>, U: ?Sized> PartialOrd<Retained<U>> for Retained<T> {
    #[inline]
    fn partial_cmp(&self, other: &Retained<U>) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
    #[inline]
    fn lt(&self, other: &Retained<U>) -> bool {
        (**self).lt(&**other)
    }
    #[inline]
    fn le(&self, other: &Retained<U>) -> bool {
        (**self).le(&**other)
    }
    #[inline]
    fn ge(&self, other: &Retained<U>) -> bool {
        (**self).ge(&**other)
    }
    #[inline]
    fn gt(&self, other: &Retained<U>) -> bool {
        (**self).gt(&**other)
    }
}

impl<T: Ord + ?Sized> Ord for Retained<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: hash::Hash + ?Sized> hash::Hash for Retained<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: ?Sized> hash::Hasher for Retained<T>
where
    for<'a> &'a T: hash::Hasher,
{
    fn finish(&self) -> u64 {
        (&**self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (&**self).write(bytes);
    }
    fn write_u8(&mut self, i: u8) {
        (&**self).write_u8(i);
    }
    fn write_u16(&mut self, i: u16) {
        (&**self).write_u16(i);
    }
    fn write_u32(&mut self, i: u32) {
        (&**self).write_u32(i);
    }
    fn write_u64(&mut self, i: u64) {
        (&**self).write_u64(i);
    }
    fn write_u128(&mut self, i: u128) {
        (&**self).write_u128(i);
    }
    fn write_usize(&mut self, i: usize) {
        (&**self).write_usize(i);
    }
    fn write_i8(&mut self, i: i8) {
        (&**self).write_i8(i);
    }
    fn write_i16(&mut self, i: i16) {
        (&**self).write_i16(i);
    }
    fn write_i32(&mut self, i: i32) {
        (&**self).write_i32(i);
    }
    fn write_i64(&mut self, i: i64) {
        (&**self).write_i64(i);
    }
    fn write_i128(&mut self, i: i128) {
        (&**self).write_i128(i);
    }
    fn write_isize(&mut self, i: isize) {
        (&**self).write_isize(i);
    }
}

impl<'a, T: ?Sized> hash::Hasher for &'a Retained<T>
where
    &'a T: hash::Hasher,
{
    fn finish(&self) -> u64 {
        (&***self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (&***self).write(bytes);
    }
    fn write_u8(&mut self, i: u8) {
        (&***self).write_u8(i);
    }
    fn write_u16(&mut self, i: u16) {
        (&***self).write_u16(i);
    }
    fn write_u32(&mut self, i: u32) {
        (&***self).write_u32(i);
    }
    fn write_u64(&mut self, i: u64) {
        (&***self).write_u64(i);
    }
    fn write_u128(&mut self, i: u128) {
        (&***self).write_u128(i);
    }
    fn write_usize(&mut self, i: usize) {
        (&***self).write_usize(i);
    }
    fn write_i8(&mut self, i: i8) {
        (&***self).write_i8(i);
    }
    fn write_i16(&mut self, i: i16) {
        (&***self).write_i16(i);
    }
    fn write_i32(&mut self, i: i32) {
        (&***self).write_i32(i);
    }
    fn write_i64(&mut self, i: i64) {
        (&***self).write_i64(i);
    }
    fn write_i128(&mut self, i: i128) {
        (&***self).write_i128(i);
    }
    fn write_isize(&mut self, i: isize) {
        (&***self).write_isize(i);
    }
}

macro_rules! forward_fmt_impl {
    ($trait:path) => {
        impl<T: $trait + ?Sized> $trait for Retained<T> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                (**self).fmt(f)
            }
        }
    };
}

forward_fmt_impl!(fmt::Binary);
forward_fmt_impl!(fmt::Debug);
forward_fmt_impl!(fmt::Display);
forward_fmt_impl!(fmt::LowerExp);
forward_fmt_impl!(fmt::LowerHex);
forward_fmt_impl!(fmt::Octal);
// forward_fmt_impl!(fmt::Pointer);
forward_fmt_impl!(fmt::UpperExp);
forward_fmt_impl!(fmt::UpperHex);

impl<'a, T: ?Sized> fmt::Write for &'a Retained<T>
where
    &'a T: fmt::Write,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        (&***self).write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        (&***self).write_char(c)
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        (&***self).write_fmt(args)
    }
}

impl<T: ?Sized> Borrow<T> for Retained<T> {
    fn borrow(&self) -> &T {
        // Auto-derefs
        self
    }
}

// Forward to inner type's `AsRef`.
//
// This is different from what `Box` does, but is desirable in our case, as it
// allows going directly to superclasses.
//
// See also discussion in:
// <https://internals.rust-lang.org/t/semantics-of-asref/17016>
impl<T: ?Sized + AsRef<U>, U: ?Sized> AsRef<U> for Retained<T> {
    fn as_ref(&self) -> &U {
        (**self).as_ref()
    }
}

#[cfg(feature = "std")]
impl<T: ?Sized + Error> Error for Retained<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        (**self).source()
    }
}

#[cfg(feature = "std")]
impl<T: ?Sized> io::Read for Retained<T>
where
    for<'a> &'a T: io::Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&**self).read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        (&**self).read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut std::vec::Vec<u8>) -> io::Result<usize> {
        (&**self).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut std::string::String) -> io::Result<usize> {
        (&**self).read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (&**self).read_exact(buf)
    }
}

#[cfg(feature = "std")]
impl<'a, T: ?Sized> io::Read for &'a Retained<T>
where
    &'a T: io::Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&***self).read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        (&***self).read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut std::vec::Vec<u8>) -> io::Result<usize> {
        (&***self).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut std::string::String) -> io::Result<usize> {
        (&***self).read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (&***self).read_exact(buf)
    }
}

#[cfg(feature = "std")]
impl<T: ?Sized> io::Write for Retained<T>
where
    for<'a> &'a T: io::Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&**self).write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        (&**self).write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        (&**self).flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&**self).write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        (&**self).write_fmt(fmt)
    }
}

#[cfg(feature = "std")]
impl<'a, T: ?Sized> io::Write for &'a Retained<T>
where
    &'a T: io::Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&***self).write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        (&***self).write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        (&***self).flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&***self).write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        (&***self).write_fmt(fmt)
    }
}

#[cfg(feature = "std")]
impl<T: ?Sized> io::Seek for Retained<T>
where
    for<'a> &'a T: io::Seek,
{
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        (&**self).seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        (&**self).stream_position()
    }
}

#[cfg(feature = "std")]
impl<'a, T: ?Sized> io::Seek for &'a Retained<T>
where
    &'a T: io::Seek,
{
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        (&***self).seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        (&***self).stream_position()
    }
}

impl<'a, T: ?Sized> Future for &'a Retained<T>
where
    &'a T: Future,
{
    type Output = <&'a T as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        <&T>::poll(Pin::new(&mut &***self), cx)
    }
}

impl<A, T: ?Sized> Extend<A> for Retained<T>
where
    for<'a> &'a T: Extend<A>,
{
    #[inline]
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        (&**self).extend(iter)
    }
}

impl<'a, A, T: ?Sized> Extend<A> for &'a Retained<T>
where
    &'a T: Extend<A>,
{
    #[inline]
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        (&***self).extend(iter)
    }
}

// TODO: impl Fn traits, CoerceUnsized, Stream and so on when stabilized
