//! A "history buffer", similar to a write-only ring buffer of fixed length.
//!
//! This buffer keeps a fixed number of elements.  On write, the oldest element
//! is overwritten. Thus, the buffer is useful to keep a history of values with
//! some desired depth, and for example calculate a rolling average.
//!
//! # Examples
//! ```
//! use heapless::HistoryBuf;
//!
//! // Initialize a new buffer with 8 elements.
//! let mut buf = HistoryBuf::<_, 8>::new();
//!
//! // Starts with no data
//! assert_eq!(buf.recent(), None);
//!
//! buf.write(3);
//! buf.write(5);
//! buf.extend(&[4, 4]);
//!
//! // The most recent written element is a four.
//! assert_eq!(buf.recent(), Some(&4));
//!
//! // To access all elements in an unspecified order, use `as_slice()`.
//! for el in buf.as_slice() {
//!     println!("{:?}", el);
//! }
//!
//! // Now we can prepare an average of all values, which comes out to 4.
//! let avg = buf.as_slice().iter().sum::<usize>() / buf.len();
//! assert_eq!(avg, 4);
//! ```

use core::{fmt, marker::PhantomData, mem::MaybeUninit, ops::Deref, ptr, slice};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

mod storage {
    use super::{HistoryBufInner, HistoryBufView};
    use core::mem::MaybeUninit;
    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    /// Trait defining how data for a container is stored.
    ///
    /// There's two implementations available:
    ///
    /// - [`OwnedHistoryBufStorage`]: stores the data in an array `[T; N]` whose size is known at
    ///   compile time.
    /// - [`ViewHistoryBufStorage`]: stores the data in an unsized `[T]`.
    ///
    /// This allows [`HistoryBuf`] to be generic over either sized or unsized storage. The
    /// [`histbuf`] module contains a [`HistoryBufInner`] struct that's generic on
    /// [`HistoryBufStorage`], and two type aliases for convenience:
    ///
    /// - [`HistoryBuf<T, N>`](super::HistoryBuf) = `HistoryBufInner<T, OwnedHistoryBufStorage<T,
    ///   N>>`
    /// - [`HistoryBufView<T>`](super::HistoryBufView) = `HistoryBufInner<T,
    ///   ViewHistoryBufStorage<T>>`
    ///
    /// `HistoryBuf` can be unsized into `HistoryBufView`, either by unsizing coercions such as
    /// `&mut HistoryBuf -> &mut HistoryBufView` or `Box<HistoryBuf> -> Box<HistoryBufView>`, or
    /// explicitly with [`.as_view()`](super::HistoryBuf::as_view) or
    /// [`.as_mut_view()`](super::HistoryBuf::as_mut_view).
    ///
    /// This trait is sealed, so you cannot implement it for your own types. You can only use
    /// the implementations provided by this crate.
    ///
    /// [`HistoryBufInner`]: super::HistoryBufInner
    /// [`HistoryBuf`]: super::HistoryBuf
    /// [`HistoryBufView`]: super::HistoryBufView
    /// [`histbuf`]: super
    #[allow(private_bounds)]
    pub trait HistoryBufStorage<T>: HistoryBufSealedStorage<T> {}

    pub trait HistoryBufSealedStorage<T> {
        // part of the sealed trait so that no trait is publicly implemented by
        // `OwnedHistoryBufStorage` besides `Storage`
        fn borrow(&self) -> &[MaybeUninit<T>];
        fn borrow_mut(&mut self) -> &mut [MaybeUninit<T>];
        fn as_hist_buf_view(this: &HistoryBufInner<T, Self>) -> &HistoryBufView<T>
        where
            Self: HistoryBufStorage<T>;
        fn as_hist_buf_mut_view(this: &mut HistoryBufInner<T, Self>) -> &mut HistoryBufView<T>
        where
            Self: HistoryBufStorage<T>;
    }

    // One sealed layer of indirection to hide the internal details (The MaybeUninit).
    #[cfg_attr(feature = "zeroize", derive(Zeroize))]
    pub struct HistoryBufStorageInner<T: ?Sized> {
        pub(crate) buffer: T,
    }

    /// Implementation of [`HistoryBufStorage`] that stores the data in an array `[T; N]` whose size
    /// is known at compile time.
    pub type OwnedHistoryBufStorage<T, const N: usize> =
        HistoryBufStorageInner<[MaybeUninit<T>; N]>;
    /// Implementation of [`HistoryBufStorage`] that stores the data in an unsized `[T]`.
    pub type ViewHistoryBufStorage<T> = HistoryBufStorageInner<[MaybeUninit<T>]>;

    impl<T, const N: usize> HistoryBufSealedStorage<T> for OwnedHistoryBufStorage<T, N> {
        fn borrow(&self) -> &[MaybeUninit<T>] {
            &self.buffer
        }
        fn borrow_mut(&mut self) -> &mut [MaybeUninit<T>] {
            &mut self.buffer
        }
        fn as_hist_buf_view(this: &HistoryBufInner<T, Self>) -> &HistoryBufView<T>
        where
            Self: HistoryBufStorage<T>,
        {
            this
        }
        fn as_hist_buf_mut_view(this: &mut HistoryBufInner<T, Self>) -> &mut HistoryBufView<T>
        where
            Self: HistoryBufStorage<T>,
        {
            this
        }
    }
    impl<T, const N: usize> HistoryBufStorage<T> for OwnedHistoryBufStorage<T, N> {}

    impl<T> HistoryBufSealedStorage<T> for ViewHistoryBufStorage<T> {
        fn borrow(&self) -> &[MaybeUninit<T>] {
            &self.buffer
        }
        fn borrow_mut(&mut self) -> &mut [MaybeUninit<T>] {
            &mut self.buffer
        }
        fn as_hist_buf_view(this: &HistoryBufInner<T, Self>) -> &HistoryBufView<T>
        where
            Self: HistoryBufStorage<T>,
        {
            this
        }
        fn as_hist_buf_mut_view(this: &mut HistoryBufInner<T, Self>) -> &mut HistoryBufView<T>
        where
            Self: HistoryBufStorage<T>,
        {
            this
        }
    }
    impl<T> HistoryBufStorage<T> for ViewHistoryBufStorage<T> {}
}

pub use storage::{HistoryBufStorage, OwnedHistoryBufStorage, ViewHistoryBufStorage};

use storage::HistoryBufStorageInner;

use self::storage::HistoryBufSealedStorage;

/// Base struct for [`HistoryBuf`] and [`HistoryBufView`], generic over the [`HistoryBufStorage`].
///
/// In most cases you should use [`HistoryBuf`] or [`HistoryBufView`] directly. Only use this
/// struct if you want to write code that's generic over both.
#[cfg_attr(feature = "zeroize", derive(Zeroize))]
pub struct HistoryBufInner<T, S: HistoryBufStorage<T> + ?Sized> {
    // This phantomdata is required because otherwise rustc thinks that `T` is not used
    phantom: PhantomData<T>,
    write_at: usize,
    filled: bool,
    data: S,
}

/// A "history buffer", similar to a write-only ring buffer of fixed length.
///
/// This buffer keeps a fixed number of elements.  On write, the oldest element
/// is overwritten. Thus, the buffer is useful to keep a history of values with
/// some desired depth, and for example calculate a rolling average.
///
/// # Examples
/// ```
/// use heapless::HistoryBuf;
///
/// // Initialize a new buffer with 8 elements.
/// let mut buf = HistoryBuf::<_, 8>::new();
///
/// // Starts with no data
/// assert_eq!(buf.recent(), None);
///
/// buf.write(3);
/// buf.write(5);
/// buf.extend(&[4, 4]);
///
/// // The most recent written element is a four.
/// assert_eq!(buf.recent(), Some(&4));
///
/// // To access all elements in an unspecified order, use `as_slice()`.
/// for el in buf.as_slice() {
///     println!("{:?}", el);
/// }
///
/// // Now we can prepare an average of all values, which comes out to 4.
/// let avg = buf.as_slice().iter().sum::<usize>() / buf.len();
/// assert_eq!(avg, 4);
/// ```
pub type HistoryBuf<T, const N: usize> = HistoryBufInner<T, OwnedHistoryBufStorage<T, N>>;

/// A "view" into a [`HistoryBuf`]
///
/// Unlike [`HistoryBuf`], it doesn't have the `const N: usize` in its type signature.
///
/// # Examples
/// ```
/// use heapless::history_buf::{HistoryBuf, HistoryBufView};
///
/// // Initialize a new buffer with 8 elements.
/// let mut owned_buf = HistoryBuf::<_, 8>::new();
/// let buf: &mut HistoryBufView<_> = &mut owned_buf;
///
/// // Starts with no data
/// assert_eq!(buf.recent(), None);
///
/// buf.write(3);
/// buf.write(5);
/// buf.extend(&[4, 4]);
///
/// // The most recent written element is a four.
/// assert_eq!(buf.recent(), Some(&4));
///
/// // To access all elements in an unspecified order, use `as_slice()`.
/// for el in buf.as_slice() {
///     println!("{:?}", el);
/// }
///
/// // Now we can prepare an average of all values, which comes out to 4.
/// let avg = buf.as_slice().iter().sum::<usize>() / buf.len();
/// assert_eq!(avg, 4);
/// ```
pub type HistoryBufView<T> = HistoryBufInner<T, ViewHistoryBufStorage<T>>;

impl<T, const N: usize> HistoryBuf<T, N> {
    const INIT: MaybeUninit<T> = MaybeUninit::uninit();

    /// Constructs a new history buffer.
    ///
    /// The construction of a `HistoryBuf` works in `const` contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// // Allocate a 16-element buffer on the stack
    /// let x: HistoryBuf<u8, 16> = HistoryBuf::new();
    /// assert_eq!(x.len(), 0);
    /// ```
    #[inline]
    pub const fn new() -> Self {
        const {
            assert!(N > 0);
        }

        Self {
            phantom: PhantomData,
            data: HistoryBufStorageInner {
                buffer: [Self::INIT; N],
            },
            write_at: 0,
            filled: false,
        }
    }
}

impl<T, const N: usize> HistoryBuf<T, N>
where
    T: Copy + Clone,
{
    /// Constructs a new history buffer, where every element is the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// // Allocate a 16-element buffer on the stack
    /// let mut x: HistoryBuf<u8, 16> = HistoryBuf::new_with(4);
    /// // All elements are four
    /// assert_eq!(x.as_slice(), [4; 16]);
    /// ```
    #[inline]
    pub fn new_with(t: T) -> Self {
        Self {
            phantom: PhantomData,
            data: HistoryBufStorageInner {
                buffer: [MaybeUninit::new(t); N],
            },
            write_at: 0,
            filled: true,
        }
    }
}

impl<T: Copy, S: HistoryBufStorage<T> + ?Sized> HistoryBufInner<T, S> {
    /// Get a reference to the `HistoryBuf`, erasing the `N` const-generic.
    #[inline]
    pub fn as_view(&self) -> &HistoryBufView<T> {
        S::as_hist_buf_view(self)
    }

    /// Get a mutable reference to the `HistoryBuf`, erasing the `N` const-generic.
    #[inline]
    pub fn as_mut_view(&mut self) -> &mut HistoryBufView<T> {
        S::as_hist_buf_mut_view(self)
    }

    /// Clears the buffer, replacing every element with the given value.
    pub fn clear_with(&mut self, t: T) {
        // SAFETY: we reset the values just after
        unsafe { self.drop_contents() };
        self.write_at = 0;
        self.filled = true;

        for d in self.data.borrow_mut() {
            *d = MaybeUninit::new(t);
        }
    }
}

impl<T, S: HistoryBufStorage<T> + ?Sized> HistoryBufInner<T, S> {
    /// Clears the buffer
    pub fn clear(&mut self) {
        // SAFETY: we reset the values just after
        unsafe { self.drop_contents() };
        self.write_at = 0;
        self.filled = false;
    }
}

impl<T, S: HistoryBufStorage<T> + ?Sized> HistoryBufInner<T, S> {
    unsafe fn drop_contents(&mut self) {
        unsafe {
            ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                self.data.borrow_mut().as_mut_ptr().cast::<T>(),
                self.len(),
            ));
        }
    }

    /// Returns the current fill level of the buffer.
    #[inline]
    pub fn len(&self) -> usize {
        if self.filled {
            self.capacity()
        } else {
            self.write_at
        }
    }

    /// Returns true if the buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// let x: HistoryBuf<u8, 16> = HistoryBuf::new();
    /// assert!(x.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the capacity of the buffer, which is the length of the
    /// underlying backing array.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.borrow().len()
    }

    /// Returns whether the buffer is full
    #[inline]
    pub fn is_full(&self) -> bool {
        self.filled
    }

    /// Writes an element to the buffer, overwriting the oldest value.
    pub fn write(&mut self, t: T) {
        if self.filled {
            // Drop the old before we overwrite it.
            unsafe { ptr::drop_in_place(self.data.borrow_mut()[self.write_at].as_mut_ptr()) }
        }
        self.data.borrow_mut()[self.write_at] = MaybeUninit::new(t);

        self.write_at += 1;
        if self.write_at == self.capacity() {
            self.write_at = 0;
            self.filled = true;
        }
    }

    /// Clones and writes all elements in a slice to the buffer.
    ///
    /// If the slice is longer than the buffer, only the last `self.len()`
    /// elements will actually be stored.
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        for item in other {
            self.write(item.clone());
        }
    }

    /// Returns a reference to the most recently written value.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// let mut x: HistoryBuf<u8, 16> = HistoryBuf::new();
    /// x.write(4);
    /// x.write(10);
    /// assert_eq!(x.recent(), Some(&10));
    /// ```
    pub fn recent(&self) -> Option<&T> {
        self.recent_index()
            .map(|i| unsafe { &*self.data.borrow()[i].as_ptr() })
    }

    /// Returns index of the most recently written value in the underlying slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// let mut x: HistoryBuf<u8, 16> = HistoryBuf::new();
    /// x.write(4);
    /// x.write(10);
    /// assert_eq!(x.recent_index(), Some(1));
    /// ```
    pub fn recent_index(&self) -> Option<usize> {
        if self.write_at == 0 {
            if self.filled {
                Some(self.capacity() - 1)
            } else {
                None
            }
        } else {
            Some(self.write_at - 1)
        }
    }

    /// Returns a reference to the oldest value in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// let mut x: HistoryBuf<u8, 16> = HistoryBuf::new();
    /// x.write(4);
    /// x.write(10);
    /// assert_eq!(x.oldest(), Some(&4));
    /// ```
    pub fn oldest(&self) -> Option<&T> {
        self.oldest_index()
            .map(|i| unsafe { &*self.data.borrow()[i].as_ptr() })
    }

    /// Returns index of the oldest value in the underlying slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// let mut x: HistoryBuf<u8, 16> = HistoryBuf::new();
    /// x.write(4);
    /// x.write(10);
    /// assert_eq!(x.oldest_index(), Some(0));
    /// ```
    pub fn oldest_index(&self) -> Option<usize> {
        if self.filled {
            Some(self.write_at)
        } else if self.write_at == 0 {
            None
        } else {
            Some(0)
        }
    }

    /// Returns the array slice backing the buffer, without keeping track
    /// of the write position. Therefore, the element order is unspecified.
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data.borrow().as_ptr().cast(), self.len()) }
    }

    /// Returns a pair of slices which contain, in order, the contents of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// let mut buffer: HistoryBuf<u8, 6> = HistoryBuf::new();
    /// buffer.extend([0, 0, 0]);
    /// buffer.extend([1, 2, 3, 4, 5, 6]);
    /// assert_eq!(buffer.as_slices(), (&[1, 2, 3][..], &[4, 5, 6][..]));
    /// ```
    pub fn as_slices(&self) -> (&[T], &[T]) {
        let buffer = self.as_slice();

        if self.filled {
            (&buffer[self.write_at..], &buffer[..self.write_at])
        } else {
            (buffer, &[])
        }
    }

    /// Returns double ended iterator for iterating over the buffer from
    /// the oldest to the newest and back.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::HistoryBuf;
    ///
    /// let mut buffer: HistoryBuf<u8, 6> = HistoryBuf::new();
    /// buffer.extend([0, 0, 0, 1, 2, 3, 4, 5, 6]);
    /// let expected = [1, 2, 3, 4, 5, 6];
    /// for (x, y) in buffer.oldest_ordered().zip(expected.iter()) {
    ///     assert_eq!(x, y)
    /// }
    /// ```
    pub fn oldest_ordered(&self) -> OldestOrdered<'_, T> {
        let (old, new) = self.as_slices();
        OldestOrdered {
            inner: old.iter().chain(new),
        }
    }
}

impl<T, S: HistoryBufStorage<T> + ?Sized> Extend<T> for HistoryBufInner<T, S> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter.into_iter() {
            self.write(item);
        }
    }
}

impl<'a, T, S: HistoryBufStorage<T> + ?Sized> Extend<&'a T> for HistoryBufInner<T, S>
where
    T: 'a + Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T, const N: usize> Clone for HistoryBuf<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut ret = Self::new();
        for (new, old) in ret.data.borrow_mut().iter_mut().zip(self.as_slice()) {
            new.write(old.clone());
        }
        ret.filled = self.filled;
        ret.write_at = self.write_at;
        ret
    }
}

impl<T, S: HistoryBufStorage<T> + ?Sized> Drop for HistoryBufInner<T, S> {
    fn drop(&mut self) {
        unsafe { self.drop_contents() }
    }
}

impl<T, S: HistoryBufStorage<T> + ?Sized> Deref for HistoryBufInner<T, S> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, S: HistoryBufStorage<T> + ?Sized> AsRef<[T]> for HistoryBufInner<T, S> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T, S: HistoryBufStorage<T> + ?Sized> fmt::Debug for HistoryBufInner<T, S>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[T] as fmt::Debug>::fmt(self, f)
    }
}

impl<T, const N: usize> Default for HistoryBuf<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, S: HistoryBufStorage<T> + ?Sized> PartialEq for HistoryBufInner<T, S>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.oldest_ordered().eq(other.oldest_ordered())
    }
}

/// Double ended iterator on the underlying buffer ordered from the oldest data
/// to the newest.
pub struct OldestOrdered<'a, T> {
    inner: core::iter::Chain<core::slice::Iter<'a, T>, core::slice::Iter<'a, T>>,
}

impl<T> Clone for OldestOrdered<'_, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T> Iterator for OldestOrdered<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> DoubleEndedIterator for OldestOrdered<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

#[cfg(test)]
mod tests {
    use core::{
        fmt::Debug,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use static_assertions::assert_not_impl_any;

    use super::{HistoryBuf, HistoryBufView};

    // Ensure a `HistoryBuf` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(HistoryBuf<*const (), 4>: Send);

    #[test]
    fn new() {
        let x: HistoryBuf<u8, 4> = HistoryBuf::new_with(1);
        assert_eq!(x.len(), 4);
        assert_eq!(x.as_slice(), [1; 4]);
        assert_eq!(*x, [1; 4]);
        assert!(x.is_full());

        let x: HistoryBuf<u8, 4> = HistoryBuf::new();
        assert_eq!(x.as_slice(), []);
        assert!(!x.is_full());
    }

    #[test]
    fn write() {
        let mut x: HistoryBuf<u8, 4> = HistoryBuf::new();
        x.write(1);
        x.write(4);
        assert_eq!(x.as_slice(), [1, 4]);

        x.write(5);
        x.write(6);
        x.write(10);
        assert_eq!(x.as_slice(), [10, 4, 5, 6]);

        x.extend([11, 12].iter());
        assert_eq!(x.as_slice(), [10, 11, 12, 6]);
    }

    #[test]
    fn clear() {
        let mut x: HistoryBuf<u8, 4> = HistoryBuf::new_with(1);
        x.clear();
        assert_eq!(x.as_slice(), []);

        let mut x: HistoryBuf<u8, 4> = HistoryBuf::new();
        x.clear_with(1);
        assert_eq!(x.as_slice(), [1; 4]);
    }

    #[test]
    fn clone() {
        let mut x: HistoryBuf<u8, 3> = HistoryBuf::new();
        for i in 0..10 {
            assert_eq!(x.as_slice(), x.clone().as_slice());
            x.write(i);
        }

        // Records number of clones locally and globally.
        static GLOBAL: AtomicUsize = AtomicUsize::new(0);
        #[derive(Default, PartialEq, Debug)]
        struct InstrumentedClone(usize);

        impl Clone for InstrumentedClone {
            fn clone(&self) -> Self {
                GLOBAL.fetch_add(1, Ordering::Relaxed);
                Self(self.0 + 1)
            }
        }

        let mut y: HistoryBuf<InstrumentedClone, 2> = HistoryBuf::new();
        let _ = y.clone();
        assert_eq!(GLOBAL.load(Ordering::Relaxed), 0);
        y.write(InstrumentedClone(0));
        assert_eq!(GLOBAL.load(Ordering::Relaxed), 0);
        assert_eq!(y.clone().as_slice(), [InstrumentedClone(1)]);
        assert_eq!(GLOBAL.load(Ordering::Relaxed), 1);
        y.write(InstrumentedClone(0));
        assert_eq!(GLOBAL.load(Ordering::Relaxed), 1);
        assert_eq!(
            y.clone().as_slice(),
            [InstrumentedClone(1), InstrumentedClone(1)]
        );
        assert_eq!(GLOBAL.load(Ordering::Relaxed), 3);
        assert_eq!(
            y.clone().clone().clone().as_slice(),
            [InstrumentedClone(3), InstrumentedClone(3)]
        );
        assert_eq!(GLOBAL.load(Ordering::Relaxed), 9);
    }

    #[test]
    fn recent() {
        let mut x: HistoryBuf<u8, 4> = HistoryBuf::new();
        assert_eq!(x.recent_index(), None);
        assert_eq!(x.recent(), None);

        x.write(1);
        x.write(4);
        assert_eq!(x.recent_index(), Some(1));
        assert_eq!(x.recent(), Some(&4));

        x.write(5);
        x.write(6);
        x.write(10);
        assert_eq!(x.recent_index(), Some(0));
        assert_eq!(x.recent(), Some(&10));
    }

    #[test]
    fn oldest() {
        let mut x: HistoryBuf<u8, 4> = HistoryBuf::new();
        assert_eq!(x.oldest_index(), None);
        assert_eq!(x.oldest(), None);

        x.write(1);
        x.write(4);
        assert_eq!(x.oldest_index(), Some(0));
        assert_eq!(x.oldest(), Some(&1));

        x.write(5);
        x.write(6);
        x.write(10);
        assert_eq!(x.oldest_index(), Some(1));
        assert_eq!(x.oldest(), Some(&4));
    }

    #[test]
    fn as_slice() {
        let mut x: HistoryBuf<u8, 4> = HistoryBuf::new();

        assert_eq!(x.as_slice(), []);

        x.extend([1, 2, 3, 4, 5].iter());

        assert_eq!(x.as_slice(), [5, 2, 3, 4]);
    }

    /// Test whether `.as_slices()` behaves as expected.
    #[test]
    fn as_slices() {
        let mut buffer: HistoryBuf<u8, 4> = HistoryBuf::new();
        let mut extend_then_assert = |extend: &[u8], assert: (&[u8], &[u8])| {
            buffer.extend(extend);
            assert_eq!(buffer.as_slices(), assert);
        };

        extend_then_assert(b"a", (b"a", b""));
        extend_then_assert(b"bcd", (b"abcd", b""));
        extend_then_assert(b"efg", (b"d", b"efg"));
        extend_then_assert(b"h", (b"efgh", b""));
        extend_then_assert(b"123456", (b"34", b"56"));
    }

    /// Test whether `.as_slices()` and `.oldest_ordered()` produce elements in the same order.
    #[test]
    fn as_slices_equals_ordered() {
        let mut buffer: HistoryBuf<u8, 6> = HistoryBuf::new();

        for n in 0..20 {
            buffer.write(n);
            let (head, tail) = buffer.as_slices();
            assert_eq_iter(
                [head, tail].iter().copied().flatten(),
                buffer.oldest_ordered(),
            );
        }
    }

    #[test]
    fn ordered() {
        // test on an empty buffer
        let buffer: HistoryBuf<u8, 6> = HistoryBuf::new();
        let mut iter = buffer.oldest_ordered();
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);

        // test on a un-filled buffer
        let mut buffer: HistoryBuf<u8, 6> = HistoryBuf::new();
        buffer.extend([1, 2, 3]);
        assert_eq!(buffer.len(), 3);
        assert_eq_iter(buffer.oldest_ordered(), &[1, 2, 3]);
        assert_eq_iter(buffer.oldest_ordered().rev(), &[3, 2, 1]);
        let mut iter = buffer.oldest_ordered();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), Some(&2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);

        // test on an exactly filled buffer
        let mut buffer: HistoryBuf<u8, 6> = HistoryBuf::new();
        buffer.extend([1, 2, 3, 4, 5, 6]);
        assert_eq!(buffer.len(), 6);
        assert_eq_iter(buffer.oldest_ordered(), &[1, 2, 3, 4, 5, 6]);
        assert_eq_iter(buffer.oldest_ordered().rev(), &[6, 5, 4, 3, 2, 1]);

        // test on a filled buffer
        let mut buffer: HistoryBuf<u8, 6> = HistoryBuf::new();
        buffer.extend([0, 0, 0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(buffer.len(), 6);
        assert_eq_iter(buffer.oldest_ordered(), &[1, 2, 3, 4, 5, 6]);
        assert_eq_iter(buffer.oldest_ordered().rev(), &[6, 5, 4, 3, 2, 1]);

        // comprehensive test all cases
        for n in 0..50 {
            const N: usize = 7;
            let mut buffer: HistoryBuf<u8, N> = HistoryBuf::new();
            buffer.extend(0..n);
            assert_eq_iter(
                buffer.oldest_ordered().copied(),
                n.saturating_sub(N as u8)..n,
            );
            assert_eq_iter(
                buffer.oldest_ordered().rev().copied(),
                (n.saturating_sub(N as u8)..n).rev(),
            );
        }
    }

    /// Compares two iterators item by item, making sure they stop at the same time.
    fn assert_eq_iter<I: Eq + Debug>(
        a: impl IntoIterator<Item = I>,
        b: impl IntoIterator<Item = I>,
    ) {
        let mut a = a.into_iter();
        let mut b = b.into_iter();

        let mut i = 0;
        loop {
            let a_item = a.next();
            let b_item = b.next();

            assert_eq!(a_item, b_item, "{i}");

            i += 1;

            if b_item.is_none() {
                break;
            }
        }
    }

    #[test]
    fn partial_eq() {
        let mut x: HistoryBuf<u8, 3> = HistoryBuf::new();
        let mut y: HistoryBuf<u8, 3> = HistoryBuf::new();
        assert_eq!(x, y);
        x.write(1);
        assert_ne!(x, y);
        y.write(1);
        assert_eq!(x, y);
        for _ in 0..4 {
            x.write(2);
            assert_ne!(x, y);
            for i in 0..5 {
                x.write(i);
                y.write(i);
            }
            assert_eq!(
                x,
                y,
                "{:?} {:?}",
                x.iter().collect::<Vec<_>>(),
                y.iter().collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn clear_drops_values() {
        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct DropCheck {}

        impl Drop for DropCheck {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        let mut x: HistoryBuf<DropCheck, 3> = HistoryBuf::new();
        x.write(DropCheck {});
        x.write(DropCheck {});
        x.write(DropCheck {});

        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 0);
        x.clear();
        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 3);
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_history_buf_zeroize() {
        use zeroize::Zeroize;

        let mut buffer = HistoryBuf::<u8, 8>::new();
        for i in 0..8 {
            buffer.write(i);
        }

        assert_eq!(buffer.len(), 8);
        assert_eq!(buffer.recent(), Some(&7));

        // Clear to mark formerly used memory as unused, to make sure that it also gets zeroed
        buffer.clear();

        buffer.write(20);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.recent(), Some(&20));

        buffer.zeroize();

        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());

        // Check that all underlying memory actually got zeroized
        unsafe {
            for a in buffer.data.buffer {
                assert_eq!(a.assume_init(), 0);
            }
        }
    }

    fn _test_variance<'a: 'b, 'b>(x: HistoryBuf<&'a (), 42>) -> HistoryBuf<&'b (), 42> {
        x
    }
    fn _test_variance_view<'a: 'b, 'b, 'c>(
        x: &'c HistoryBufView<&'a ()>,
    ) -> &'c HistoryBufView<&'b ()> {
        x
    }
}
