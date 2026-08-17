use std::ops;

/// A owned window around an underlying buffer.
///
/// Normally slices work great for considering sub-portions of a buffer, but
/// unfortunately a slice is a *borrowed* type in Rust which has an associated
/// lifetime. When working with future and async I/O these lifetimes are not
/// always appropriate, and are sometimes difficult to store in tasks. This
/// type strives to fill this gap by providing an "owned slice" around an
/// underlying buffer of bytes.
///
/// A `Window<T>` wraps an underlying buffer, `T`, and has configurable
/// start/end indexes to alter the behavior of the `AsRef<[u8]>` implementation
/// that this type carries.
///
/// This type can be particularly useful when working with the `write_all`
/// combinator in this crate. Data can be sliced via `Window`, consumed by
/// `write_all`, and then earned back once the write operation finishes through
/// the `into_inner` method on this type.
#[derive(Debug)]
pub struct Window<T> {
    inner: T,
    range: ops::Range<usize>,
}

impl<T: AsRef<[u8]>> Window<T> {
    /// Creates a new window around the buffer `t` defaulting to the entire
    /// slice.
    ///
    /// Further methods can be called on the returned `Window<T>` to alter the
    /// window into the data provided.
    pub fn new(t: T) -> Window<T> {
        Window {
            range: 0..t.as_ref().len(),
            inner: t,
        }
    }

    /// Gets a shared reference to the underlying buffer inside of this
    /// `Window`.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying buffer inside of this
    /// `Window`.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consumes this `Window`, returning the underlying buffer.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Returns the starting index of this window into the underlying buffer
    /// `T`.
    pub fn start(&self) -> usize {
        self.range.start
    }

    /// Returns the end index of this window into the underlying buffer
    /// `T`.
    pub fn end(&self) -> usize {
        self.range.end
    }

    /// Changes the starting index of this window to the index specified.
    ///
    /// Returns the windows back to chain multiple calls to this method.
    ///
    /// # Panics
    ///
    /// This method will panic if `start` is out of bounds for the underlying
    /// slice or if it comes after the `end` configured in this window.
    pub fn set_start(&mut self, start: usize) -> &mut Window<T> {
        assert!(start <= self.inner.as_ref().len());
        assert!(start <= self.range.end);
        self.range.start = start;
        self
    }

    /// Changes the end index of this window to the index specified.
    ///
    /// Returns the windows back to chain multiple calls to this method.
    ///
    /// # Panics
    ///
    /// This method will panic if `end` is out of bounds for the underlying
    /// slice or if it comes before the `start` configured in this window.
    pub fn set_end(&mut self, end: usize) -> &mut Window<T> {
        assert!(end <= self.inner.as_ref().len());
        assert!(self.range.start <= end);
        self.range.end = end;
        self
    }

    // TODO: how about a generic set() method along the lines of:
    //
    //       buffer.set(..3)
    //             .set(0..2)
    //             .set(4..)
    //
    // etc.
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Window<T> {
    fn as_ref(&self) -> &[u8] {
        &self.inner.as_ref()[self.range.start..self.range.end]
    }
}

impl<T: AsMut<[u8]>> AsMut<[u8]> for Window<T> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner.as_mut()[self.range.start..self.range.end]
    }
}
