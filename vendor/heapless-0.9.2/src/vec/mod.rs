//! A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).

use core::{
    borrow,
    cmp::Ordering,
    fmt, hash,
    iter::FusedIterator,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    ops::{self, Range, RangeBounds},
    ptr::{self, NonNull},
    slice,
};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

use crate::{
    len_type::{check_capacity_fits, LenType},
    CapacityError,
};

mod drain;

mod storage {
    use core::mem::MaybeUninit;

    use crate::{
        binary_heap::{BinaryHeapInner, BinaryHeapView},
        deque::{DequeInner, DequeView},
        len_type::LenType,
    };

    use super::{VecInner, VecView};

    /// Trait defining how data for a container is stored.
    ///
    /// There's two implementations available:
    ///
    /// - [`OwnedVecStorage`]: stores the data in an array `[T; N]` whose size is known at compile
    ///   time.
    /// - [`ViewVecStorage`]: stores the data in an unsized `[T]`.
    ///
    /// This allows [`Vec`] to be generic over either sized or unsized storage. The [`vec`](super)
    /// module contains a [`VecInner`] struct that's generic on [`VecStorage`],
    /// and two type aliases for convenience:
    ///
    /// - [`Vec<T, N>`](crate::vec::Vec) = `VecInner<T, OwnedStorage<T, N>>`
    /// - [`VecView<T>`](crate::vec::VecView) = `VecInner<T, ViewStorage<T>>`
    ///
    /// `Vec` can be unsized into `VecView`, either by unsizing coercions such as `&mut Vec -> &mut
    /// VecView` or `Box<Vec> -> Box<VecView>`, or explicitly with
    /// [`.as_view()`](crate::vec::Vec::as_view) or
    /// [`.as_mut_view()`](crate::vec::Vec::as_mut_view).
    ///
    /// This trait is sealed, so you cannot implement it for your own types. You can only use
    /// the implementations provided by this crate.
    ///
    /// [`VecInner`]: super::VecInner
    /// [`Vec`]: super::Vec
    /// [`VecView`]: super::VecView
    #[allow(private_bounds)]
    pub trait VecStorage<T>: VecSealedStorage<T> {}

    pub trait VecSealedStorage<T> {
        // part of the sealed trait so that no trait is publicly implemented by `OwnedVecStorage`
        // besides `Storage`
        fn borrow(&self) -> &[MaybeUninit<T>];
        fn borrow_mut(&mut self) -> &mut [MaybeUninit<T>];

        fn as_vec_view<LenT: LenType>(this: &VecInner<T, LenT, Self>) -> &VecView<T, LenT>
        where
            Self: VecStorage<T>;
        fn as_vec_view_mut<LenT: LenType>(
            this: &mut VecInner<T, LenT, Self>,
        ) -> &mut VecView<T, LenT>
        where
            Self: VecStorage<T>;

        fn as_binary_heap_view<K>(this: &BinaryHeapInner<T, K, Self>) -> &BinaryHeapView<T, K>
        where
            Self: VecStorage<T>;
        fn as_binary_heap_view_mut<K>(
            this: &mut BinaryHeapInner<T, K, Self>,
        ) -> &mut BinaryHeapView<T, K>
        where
            Self: VecStorage<T>;

        fn as_deque_view(this: &DequeInner<T, Self>) -> &DequeView<T>
        where
            Self: VecStorage<T>;
        fn as_deque_view_mut(this: &mut DequeInner<T, Self>) -> &mut DequeView<T>
        where
            Self: VecStorage<T>;
    }

    #[cfg(feature = "zeroize")]
    use zeroize::Zeroize;

    // One sealed layer of indirection to hide the internal details (The MaybeUninit).
    #[cfg_attr(feature = "zeroize", derive(Zeroize))]
    pub struct VecStorageInner<T: ?Sized> {
        pub(crate) buffer: T,
    }

    /// Implementation of [`VecStorage`] that stores the data in an array `[T; N]` whose size is
    /// known at compile time.
    pub type OwnedVecStorage<T, const N: usize> = VecStorageInner<[MaybeUninit<T>; N]>;
    /// Implementation of [`VecStorage`] that stores the data in an unsized `[T]`.
    pub type ViewVecStorage<T> = VecStorageInner<[MaybeUninit<T>]>;

    impl<T, const N: usize> VecSealedStorage<T> for OwnedVecStorage<T, N> {
        fn borrow(&self) -> &[MaybeUninit<T>] {
            &self.buffer
        }
        fn borrow_mut(&mut self) -> &mut [MaybeUninit<T>] {
            &mut self.buffer
        }

        fn as_vec_view<LenT: LenType>(this: &VecInner<T, LenT, Self>) -> &VecView<T, LenT>
        where
            Self: VecStorage<T>,
        {
            this
        }
        fn as_vec_view_mut<LenT: LenType>(
            this: &mut VecInner<T, LenT, Self>,
        ) -> &mut VecView<T, LenT>
        where
            Self: VecStorage<T>,
        {
            this
        }

        fn as_binary_heap_view<K>(this: &BinaryHeapInner<T, K, Self>) -> &BinaryHeapView<T, K>
        where
            Self: VecStorage<T>,
        {
            this
        }
        fn as_binary_heap_view_mut<K>(
            this: &mut BinaryHeapInner<T, K, Self>,
        ) -> &mut BinaryHeapView<T, K>
        where
            Self: VecStorage<T>,
        {
            this
        }
        fn as_deque_view(this: &DequeInner<T, Self>) -> &DequeView<T>
        where
            Self: VecStorage<T>,
        {
            this
        }
        fn as_deque_view_mut(this: &mut DequeInner<T, Self>) -> &mut DequeView<T>
        where
            Self: VecStorage<T>,
        {
            this
        }
    }
    impl<T, const N: usize> VecStorage<T> for OwnedVecStorage<T, N> {}

    impl<T> VecSealedStorage<T> for ViewVecStorage<T> {
        fn borrow(&self) -> &[MaybeUninit<T>] {
            &self.buffer
        }
        fn borrow_mut(&mut self) -> &mut [MaybeUninit<T>] {
            &mut self.buffer
        }

        fn as_vec_view<LenT: LenType>(this: &VecInner<T, LenT, Self>) -> &VecView<T, LenT>
        where
            Self: VecStorage<T>,
        {
            this
        }
        fn as_vec_view_mut<LenT: LenType>(
            this: &mut VecInner<T, LenT, Self>,
        ) -> &mut VecView<T, LenT>
        where
            Self: VecStorage<T>,
        {
            this
        }

        fn as_binary_heap_view<K>(this: &BinaryHeapInner<T, K, Self>) -> &BinaryHeapView<T, K>
        where
            Self: VecStorage<T>,
        {
            this
        }
        fn as_binary_heap_view_mut<K>(
            this: &mut BinaryHeapInner<T, K, Self>,
        ) -> &mut BinaryHeapView<T, K>
        where
            Self: VecStorage<T>,
        {
            this
        }
        fn as_deque_view(this: &DequeInner<T, Self>) -> &DequeView<T>
        where
            Self: VecStorage<T>,
        {
            this
        }
        fn as_deque_view_mut(this: &mut DequeInner<T, Self>) -> &mut DequeView<T>
        where
            Self: VecStorage<T>,
        {
            this
        }
    }
    impl<T> VecStorage<T> for ViewVecStorage<T> {}
}
pub use storage::{OwnedVecStorage, VecStorage, ViewVecStorage};

pub(crate) use storage::VecStorageInner;

pub use drain::Drain;

/// Base struct for [`Vec`] and [`VecView`], generic over the [`VecStorage`].
///
/// In most cases you should use [`Vec`] or [`VecView`] directly. Only use this
/// struct if you want to write code that's generic over both.
#[cfg_attr(feature = "zeroize", derive(Zeroize), zeroize(bound = "S: Zeroize"))]
pub struct VecInner<T, LenT: LenType, S: VecStorage<T> + ?Sized> {
    phantom: PhantomData<T>,
    len: LenT,
    buffer: S,
}

/// A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
///
/// # Examples
///
/// ```
/// use heapless::Vec;
///
/// // A vector with a fixed capacity of 8 elements allocated on the stack
/// let mut vec = Vec::<_, 8>::new();
/// vec.push(1).unwrap();
/// vec.push(2).unwrap();
///
/// assert_eq!(vec.len(), 2);
/// assert_eq!(vec[0], 1);
///
/// assert_eq!(vec.pop(), Some(2));
/// assert_eq!(vec.len(), 1);
///
/// vec[0] = 7;
/// assert_eq!(vec[0], 7);
///
/// vec.extend([1, 2, 3].iter().cloned());
///
/// for x in &vec {
///     println!("{}", x);
/// }
/// assert_eq!(*vec, [7, 1, 2, 3]);
/// ```
///
/// In some cases, the const-generic might be cumbersome. `Vec` can coerce into a [`VecView`] to
/// remove the need for the const-generic:
///
/// ```rust
/// use heapless::{Vec, VecView};
///
/// let vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
/// let view: &VecView<_, _> = &vec;
/// ```
///
/// For uncommmon capacity values, or in generic scenarios, you may have to provide the `LenT`
/// generic yourself.
///
/// This should be the smallest unsigned integer type that your capacity fits in, or `usize` if you
/// don't want to consider this.
pub type Vec<T, const N: usize, LenT = usize> = VecInner<T, LenT, OwnedVecStorage<T, N>>;

/// A [`Vec`] with dynamic capacity
///
/// [`Vec`] coerces to `VecView`. `VecView` is `!Sized`, meaning it can only ever be used by
/// reference.
///
/// Unlike [`Vec`], `VecView` does not have an `N` const-generic parameter.
/// This has the ergonomic advantage of making it possible to use functions without needing to know
/// at compile-time the size of the buffers used, for example for use in `dyn` traits.
///
/// `VecView<T>` is to `Vec<T, N>` what `[T]` is to `[T; N]`.
///
/// ```rust
/// use heapless::{Vec, VecView};
///
/// let mut vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
/// let view: &VecView<_, _> = &vec;
/// assert_eq!(view, &[1, 2, 3, 4]);
///
/// let mut_view: &mut VecView<_, _> = &mut vec;
/// mut_view.push(5);
/// assert_eq!(vec, [1, 2, 3, 4, 5]);
/// ```
pub type VecView<T, LenT = usize> = VecInner<T, LenT, ViewVecStorage<T>>;

impl<T, LenT: LenType, const N: usize> Vec<T, N, LenT> {
    const ELEM: MaybeUninit<T> = MaybeUninit::uninit();
    const INIT: [MaybeUninit<T>; N] = [Self::ELEM; N]; // important for optimization of `new`

    /// Constructs a new, empty vector with a fixed capacity of `N`
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// // allocate the vector on the stack
    /// let mut x: Vec<u8, 16> = Vec::new();
    ///
    /// // allocate the vector in a static variable
    /// static mut X: Vec<u8, 16> = Vec::new();
    /// ```
    pub const fn new() -> Self {
        const { check_capacity_fits::<LenT, N>() }

        Self {
            phantom: PhantomData,
            len: LenT::ZERO,
            buffer: VecStorageInner { buffer: Self::INIT },
        }
    }

    /// Constructs a new vector with a fixed capacity of `N` and fills it
    /// with the provided slice.
    ///
    /// This is equivalent to the following code:
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut v: Vec<u8, 16> = Vec::new();
    /// v.extend_from_slice(&[1, 2, 3]).unwrap();
    /// ```
    pub fn from_slice(other: &[T]) -> Result<Self, CapacityError>
    where
        T: Clone,
    {
        let mut v = Self::new();
        v.extend_from_slice(other)?;
        Ok(v)
    }

    /// Constructs a new vector with a fixed capacity of `N`, initializing
    /// it with the provided array.
    ///
    /// The length of the provided array, `M` may be equal to _or_ less than
    /// the capacity of the vector, `N`.
    ///
    /// If the length of the provided array is greater than the capacity of the
    /// vector a compile-time error will be produced.
    pub fn from_array<const M: usize>(src: [T; M]) -> Self {
        const {
            assert!(N >= M);
        }

        // We've got to copy `src`, but we're functionally moving it. Don't run
        // any Drop code for T.
        let src = ManuallyDrop::new(src);

        if N == M {
            Self {
                phantom: PhantomData,
                len: LenT::from_usize(N),
                // NOTE(unsafe) ManuallyDrop<[T; M]> and [MaybeUninit<T>; N]
                // have the same layout when N == M.
                buffer: unsafe { mem::transmute_copy(&src) },
            }
        } else {
            let mut v = Self::new();

            for (src_elem, dst_elem) in src.iter().zip(v.buffer.buffer.iter_mut()) {
                // NOTE(unsafe) src element is not going to drop as src itself
                // is wrapped in a ManuallyDrop.
                dst_elem.write(unsafe { ptr::read(src_elem) });
            }

            unsafe { v.set_len(M) };
            v
        }
    }

    /// Returns the contents of the vector as an array of length `M` if the length
    /// of the vector is exactly `M`, otherwise returns `Err(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    /// let buffer: Vec<u8, 42> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
    /// let array: [u8; 5] = buffer.into_array().unwrap();
    /// assert_eq!(array, [1, 2, 3, 5, 8]);
    /// ```
    pub fn into_array<const M: usize>(self) -> Result<[T; M], Self> {
        if self.len() == M {
            // This is how the unstable `MaybeUninit::array_assume_init` method does it
            let array = unsafe { (core::ptr::from_ref(&self.buffer).cast::<[T; M]>()).read() };

            // We don't want `self`'s destructor to be called because that would drop all the
            // items in the array
            core::mem::forget(self);

            Ok(array)
        } else {
            Err(self)
        }
    }

    /// Clones a vec into a new vec
    pub(crate) fn clone(&self) -> Self
    where
        T: Clone,
    {
        let mut new = Self::new();
        // avoid `extend_from_slice` as that introduces a runtime check/panicking branch
        for elem in self {
            unsafe {
                new.push_unchecked(elem.clone());
            }
        }
        new
    }

    /// Casts the `LenT` type to a new type, preserving everything else about the vector.
    ///
    /// This can be useful if you need to pass a `Vec<T, N, u8>` into a `Vec<T, N, usize>` for
    /// example.
    ///
    /// This will check at compile time if the `N` value will fit into `NewLenT`, and error if not.
    pub fn cast_len_type<NewLenT: LenType>(self) -> Vec<T, N, NewLenT> {
        const { check_capacity_fits::<NewLenT, N>() }
        let this = ManuallyDrop::new(self);

        // SAFETY: Pointer argument is derived from a reference, meeting the safety documented
        // invariants. This also prevents double drops by wrapping `self` in `ManuallyDrop`.
        Vec {
            len: NewLenT::from_usize(this.len()),
            buffer: unsafe { ptr::read(&this.buffer) },
            phantom: PhantomData,
        }
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> VecInner<T, LenT, S> {
    /// Removes the specified range from the vector in bulk, returning all
    /// removed elements as an iterator. If the iterator is dropped before
    /// being fully consumed, it drops the remaining removed elements.
    ///
    /// The returned iterator keeps a mutable borrow on the vector to optimize
    /// its implementation.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the vector.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to
    /// [`mem::forget`], for example), the vector may have lost and leaked
    /// elements arbitrarily, including elements outside the range.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut v = Vec::<_, 8>::from_array([1, 2, 3]);
    /// let u: Vec<_, 8> = v.drain(1..).collect();
    /// assert_eq!(v, &[1]);
    /// assert_eq!(u, &[2, 3]);
    ///
    /// // A full range clears the vector, like `clear()` does.
    /// v.drain(..);
    /// assert_eq!(v, &[]);
    /// ```
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, LenT>
    where
        R: RangeBounds<usize>,
    {
        // Memory Safety
        //
        // When the `Drain` is first created, it shortens the length of
        // the source vector to make sure no uninitialized or moved-from elements
        // are accessible at all if the `Drain`'s destructor never gets to run.
        //
        // `Drain` will `ptr::read` out the values to remove.
        // When finished, remaining tail of the vec is copied back to cover
        // the hole, and the vector length is restored to the new length.
        //
        let len = self.len();
        let Range { start, end } = crate::slice::range(range, ..len);

        unsafe {
            // Set `self.vec` length's to `start`, to be safe in case `Drain` is leaked.
            self.set_len(start);
            let vec = NonNull::from(self.as_mut_view());
            let range_slice = slice::from_raw_parts(vec.as_ref().as_ptr().add(start), end - start);
            Drain {
                tail_start: LenT::from_usize(end),
                tail_len: LenT::from_usize(len - end),
                iter: range_slice.iter(),
                vec,
            }
        }
    }

    /// Get a reference to the `Vec`, erasing the `N` const-generic.
    ///
    ///
    /// ```rust
    /// # use heapless::{Vec, VecView};
    /// let vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
    /// let view: &VecView<u8, _> = vec.as_view();
    /// ```
    ///
    /// It is often preferable to do the same through type coerction, since `Vec<T, N>` implements
    /// `Unsize<VecView<T>>`:
    ///
    /// ```rust
    /// # use heapless::{Vec, VecView};
    /// let vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
    /// let view: &VecView<u8, _> = &vec;
    /// ```
    #[inline]
    pub fn as_view(&self) -> &VecView<T, LenT> {
        S::as_vec_view(self)
    }

    /// Get a mutable reference to the `Vec`, erasing the `N` const-generic.
    ///
    /// ```rust
    /// # use heapless::{Vec, VecView};
    /// let mut vec: Vec<u8, 10, u8> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
    /// let view: &mut VecView<u8, _> = vec.as_mut_view();
    /// ```
    ///
    /// It is often preferable to do the same through type coerction, since `Vec<T, N>` implements
    /// `Unsize<VecView<T>>`:
    ///
    /// ```rust
    /// # use heapless::{Vec, VecView};
    /// let mut vec: Vec<u8, 10, u8> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
    /// let view: &mut VecView<u8, _> = &mut vec;
    /// ```
    #[inline]
    pub fn as_mut_view(&mut self) -> &mut VecView<T, LenT> {
        S::as_vec_view_mut(self)
    }

    /// Returns a raw pointer to the vector’s buffer.
    pub fn as_ptr(&self) -> *const T {
        self.buffer.borrow().as_ptr().cast::<T>()
    }

    /// Returns a raw pointer to the vector’s buffer, which may be mutated through.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.buffer.borrow_mut().as_mut_ptr().cast::<T>()
    }

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to `&s[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    /// let buffer: Vec<u8, 5> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
    /// assert_eq!(buffer.as_slice(), &[1, 2, 3, 5, 8]);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        // NOTE(unsafe) avoid bound checks in the slicing operation
        // &buffer[..self.len]
        unsafe {
            slice::from_raw_parts(
                self.buffer.borrow().as_ptr().cast::<T>(),
                self.len.into_usize(),
            )
        }
    }

    /// Extracts a mutable slice containing the entire vector.
    ///
    /// Equivalent to `&mut s[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    /// let mut buffer: Vec<u8, 5> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
    /// let buffer_slice = buffer.as_mut_slice();
    /// buffer_slice[0] = 9;
    /// assert_eq!(buffer.as_slice(), &[9, 2, 3, 5, 8]);
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // NOTE(unsafe) avoid bound checks in the slicing operation
        // &mut buffer[..self.len]
        unsafe {
            slice::from_raw_parts_mut(
                self.buffer.borrow_mut().as_mut_ptr().cast::<T>(),
                self.len.into_usize(),
            )
        }
    }

    /// Returns the maximum number of elements the vector can hold.
    pub fn capacity(&self) -> usize {
        self.buffer.borrow().len()
    }

    /// Clears the vector, removing all values.
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Extends the vec from an iterator.
    ///
    /// # Panic
    ///
    /// Panics if the vec cannot hold all elements of the iterator.
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for elem in iter {
            self.push(elem).ok().unwrap();
        }
    }

    /// Clones and appends all elements in a slice to the `Vec`.
    ///
    /// Iterates over the slice `other`, clones each element, and then appends
    /// it to this `Vec`. The `other` vector is traversed in-order.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut vec = Vec::<u8, 8>::new();
    /// vec.push(1).unwrap();
    /// vec.extend_from_slice(&[2, 3, 4]).unwrap();
    /// assert_eq!(*vec, [1, 2, 3, 4]);
    /// ```
    pub fn extend_from_slice(&mut self, other: &[T]) -> Result<(), CapacityError>
    where
        T: Clone,
    {
        pub fn extend_from_slice_inner<T, LenT: LenType>(
            len: &mut LenT,
            buf: &mut [MaybeUninit<T>],
            other: &[T],
        ) -> Result<(), CapacityError>
        where
            T: Clone,
        {
            if len.into_usize() + other.len() > buf.len() {
                // won't fit in the `Vec`; don't modify anything and return an error
                Err(CapacityError)
            } else {
                for elem in other {
                    unsafe {
                        *buf.get_unchecked_mut(len.into_usize()) = MaybeUninit::new(elem.clone());
                    }
                    *len += LenT::one();
                }
                Ok(())
            }
        }

        extend_from_slice_inner(&mut self.len, self.buffer.borrow_mut(), other)
    }

    /// Removes the last element from a vector and returns it, or `None` if it's empty
    pub fn pop(&mut self) -> Option<T> {
        if self.len == LenT::ZERO {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    /// Appends an `item` to the back of the collection
    ///
    /// Returns back the `item` if the vector is full.
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.len() < self.capacity() {
            unsafe { self.push_unchecked(item) }
            Ok(())
        } else {
            Err(item)
        }
    }

    /// Removes the last element from a vector and returns it
    ///
    /// # Safety
    ///
    /// This assumes the vec to have at least one element.
    pub unsafe fn pop_unchecked(&mut self) -> T {
        debug_assert!(!self.is_empty());

        self.len -= LenT::one();
        self.buffer
            .borrow_mut()
            .get_unchecked_mut(self.len.into_usize())
            .as_ptr()
            .read()
    }

    /// Appends an `item` to the back of the collection
    ///
    /// # Safety
    ///
    /// This assumes the vec is not full.
    pub unsafe fn push_unchecked(&mut self, item: T) {
        // NOTE(ptr::write) the memory slot that we are about to write to is uninitialized. We
        // use `ptr::write` to avoid running `T`'s destructor on the uninitialized memory
        debug_assert!(!self.is_full());

        *self
            .buffer
            .borrow_mut()
            .get_unchecked_mut(self.len.into_usize()) = MaybeUninit::new(item);

        self.len += LenT::one();
    }

    /// Shortens the vector, keeping the first `len` elements and dropping the rest.
    pub fn truncate(&mut self, len: usize) {
        // This is safe because:
        //
        // * the slice passed to `drop_in_place` is valid; the `len > self.len` case avoids creating
        //   an invalid slice, and
        // * the `len` of the vector is shrunk before calling `drop_in_place`, such that no value
        //   will be dropped twice in case `drop_in_place` were to panic once (if it panics twice,
        //   the program aborts).
        unsafe {
            // Note: It's intentional that this is `>` and not `>=`.
            //       Changing it to `>=` has negative performance
            //       implications in some cases. See rust-lang/rust#78884 for more.
            if len > self.len() {
                return;
            }
            let remaining_len = self.len() - len;
            let s = ptr::slice_from_raw_parts_mut(self.as_mut_ptr().add(len), remaining_len);
            self.len = LenT::from_usize(len);
            ptr::drop_in_place(s);
        }
    }

    /// Resizes the Vec in-place so that len is equal to `new_len`.
    ///
    /// If `new_len` is greater than len, the Vec is extended by the
    /// difference, with each additional slot filled with value. If
    /// `new_len` is less than len, the Vec is simply truncated.
    ///
    /// See also [`resize_default`](Self::resize_default).
    pub fn resize(&mut self, new_len: usize, value: T) -> Result<(), CapacityError>
    where
        T: Clone,
    {
        if new_len > self.capacity() {
            return Err(CapacityError);
        }

        if new_len > self.len() {
            while self.len() < new_len {
                self.push(value.clone()).ok();
            }
        } else {
            self.truncate(new_len);
        }

        Ok(())
    }

    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `Default::default()`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    ///
    /// See also [`resize`](Self::resize).
    pub fn resize_default(&mut self, new_len: usize) -> Result<(), CapacityError>
    where
        T: Clone + Default,
    {
        self.resize(new_len, T::default())
    }

    /// Forces the length of the vector to `new_len`.
    ///
    /// This is a low-level operation that maintains none of the normal
    /// invariants of the type. Normally changing the length of a vector
    /// is done using one of the safe operations instead, such as
    /// [`truncate`], [`resize`], [`extend`], or [`clear`].
    ///
    /// [`truncate`]: Self::truncate
    /// [`resize`]: Self::resize
    /// [`extend`]: core::iter::Extend
    /// [`clear`]: Self::clear
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to [`capacity()`].
    /// - The elements at `old_len..new_len` must be initialized.
    ///
    /// [`capacity()`]: Self::capacity
    ///
    /// # Examples
    ///
    /// This method can be useful for situations in which the vector
    /// is serving as a buffer for other code, particularly over FFI:
    ///
    /// ```no_run
    /// # #![allow(dead_code)]
    /// use heapless::Vec;
    ///
    /// # // This is just a minimal skeleton for the doc example;
    /// # // don't use this as a starting point for a real library.
    /// # pub struct StreamWrapper { strm: *mut core::ffi::c_void }
    /// # const Z_OK: i32 = 0;
    /// # extern "C" {
    /// #     fn deflateGetDictionary(
    /// #         strm: *mut core::ffi::c_void,
    /// #         dictionary: *mut u8,
    /// #         dictLength: *mut usize,
    /// #     ) -> i32;
    /// # }
    /// # impl StreamWrapper {
    /// pub fn get_dictionary(&self) -> Option<Vec<u8, 32768>> {
    ///     // Per the FFI method's docs, "32768 bytes is always enough".
    ///     let mut dict = Vec::new();
    ///     let mut dict_length = 0;
    ///     // SAFETY: When `deflateGetDictionary` returns `Z_OK`, it holds that:
    ///     // 1. `dict_length` elements were initialized.
    ///     // 2. `dict_length` <= the capacity (32_768)
    ///     // which makes `set_len` safe to call.
    ///     unsafe {
    ///         // Make the FFI call...
    ///         let r = deflateGetDictionary(self.strm, dict.as_mut_ptr(), &mut dict_length);
    ///         if r == Z_OK {
    ///             // ...and update the length to what was initialized.
    ///             dict.set_len(dict_length);
    ///             Some(dict)
    ///         } else {
    ///             None
    ///         }
    ///     }
    /// }
    /// # }
    /// ```
    ///
    /// While the following example is sound, there is a memory leak since
    /// the inner vectors were not freed prior to the `set_len` call:
    ///
    /// ```
    /// use core::iter::FromIterator;
    /// use heapless::Vec;
    ///
    /// let mut vec = Vec::<Vec<u8, 3>, 3>::from_iter(
    ///     [
    ///         Vec::from_iter([1, 0, 0].iter().cloned()),
    ///         Vec::from_iter([0, 1, 0].iter().cloned()),
    ///         Vec::from_iter([0, 0, 1].iter().cloned()),
    ///     ]
    ///     .iter()
    ///     .cloned(),
    /// );
    /// // SAFETY:
    /// // 1. `old_len..0` is empty so no elements need to be initialized.
    /// // 2. `0 <= capacity` always holds whatever `capacity` is.
    /// unsafe {
    ///     vec.set_len(0);
    /// }
    /// ```
    ///
    /// Normally, here, one would use [`clear`] instead to correctly drop
    /// the contents and thus not leak memory.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.len = LenT::from_usize(new_len);
    }

    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is *O*(1).
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut v: Vec<_, 8> = Vec::new();
    /// v.push("foo").unwrap();
    /// v.push("bar").unwrap();
    /// v.push("baz").unwrap();
    /// v.push("qux").unwrap();
    ///
    /// assert_eq!(v.swap_remove(1), "bar");
    /// assert_eq!(&*v, ["foo", "qux", "baz"]);
    ///
    /// assert_eq!(v.swap_remove(0), "foo");
    /// assert_eq!(&*v, ["baz", "qux"]);
    /// ```
    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(index < self.len());
        unsafe { self.swap_remove_unchecked(index) }
    }

    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is *O*(1).
    ///
    /// # Safety
    ///
    ///  Assumes `index` within bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut v: Vec<_, 8> = Vec::new();
    /// v.push("foo").unwrap();
    /// v.push("bar").unwrap();
    /// v.push("baz").unwrap();
    /// v.push("qux").unwrap();
    ///
    /// assert_eq!(unsafe { v.swap_remove_unchecked(1) }, "bar");
    /// assert_eq!(&*v, ["foo", "qux", "baz"]);
    ///
    /// assert_eq!(unsafe { v.swap_remove_unchecked(0) }, "foo");
    /// assert_eq!(&*v, ["baz", "qux"]);
    /// ```
    pub unsafe fn swap_remove_unchecked(&mut self, index: usize) -> T {
        let length = self.len();
        debug_assert!(index < length);
        let value = ptr::read(self.as_ptr().add(index));
        let base_ptr = self.as_mut_ptr();
        ptr::copy(base_ptr.add(length - 1), base_ptr.add(index), 1);
        self.len -= LenT::one();
        value
    }

    /// Returns true if the vec is full
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Returns true if the vec is empty
    pub fn is_empty(&self) -> bool {
        self.len == LenT::ZERO
    }

    /// Returns `true` if `needle` is a prefix of the Vec.
    ///
    /// Always returns `true` if `needle` is an empty slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let v: Vec<_, 8> = Vec::from_slice(b"abc").unwrap();
    /// assert_eq!(v.starts_with(b""), true);
    /// assert_eq!(v.starts_with(b"ab"), true);
    /// assert_eq!(v.starts_with(b"bc"), false);
    /// ```
    pub fn starts_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        let n = needle.len();
        self.len() >= n && needle == &self[..n]
    }

    /// Returns `true` if `needle` is a suffix of the Vec.
    ///
    /// Always returns `true` if `needle` is an empty slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let v: Vec<_, 8> = Vec::from_slice(b"abc").unwrap();
    /// assert_eq!(v.ends_with(b""), true);
    /// assert_eq!(v.ends_with(b"ab"), false);
    /// assert_eq!(v.ends_with(b"bc"), true);
    /// ```
    pub fn ends_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        let (v, n) = (self.len(), needle.len());
        v >= n && needle == &self[v - n..]
    }

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// Returns back the `element` if the vector is full.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut vec: Vec<_, 8> = Vec::from_slice(&[1, 2, 3]).unwrap();
    /// vec.insert(1, 4);
    /// assert_eq!(vec, [1, 4, 2, 3]);
    /// vec.insert(4, 5);
    /// assert_eq!(vec, [1, 4, 2, 3, 5]);
    /// ```
    pub fn insert(&mut self, index: usize, element: T) -> Result<(), T> {
        let len = self.len();
        if index > len {
            panic!("insertion index (is {index}) should be <= len (is {len})");
        }

        // check there's space for the new element
        if self.is_full() {
            return Err(element);
        }

        unsafe {
            // infallible
            // The spot to put the new value
            {
                let p = self.as_mut_ptr().add(index);
                // Shift everything over to make space. (Duplicating the
                // `index`th element into two consecutive places.)
                ptr::copy(p, p.offset(1), len - index);
                // Write it in, overwriting the first copy of the `index`th
                // element.
                ptr::write(p, element);
            }
            self.set_len(len + 1);
        }

        Ok(())
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(n). If you don't need the order of
    /// elements to be preserved, use [`swap_remove`] instead. If you'd like to
    /// remove elements from the beginning of the `Vec`, consider using
    /// [`Deque::pop_front`] instead.
    ///
    /// [`swap_remove`]: Vec::swap_remove
    /// [`Deque::pop_front`]: crate::Deque::pop_front
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut v: Vec<_, 8> = Vec::from_slice(&[1, 2, 3]).unwrap();
    /// assert_eq!(v.remove(1), 2);
    /// assert_eq!(v, [1, 3]);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        let len = self.len();
        if index >= len {
            panic!("removal index (is {index}) should be < len (is {len})");
        }
        unsafe {
            // infallible
            let ret;
            {
                // the place we are taking from.
                let ptr = self.as_mut_ptr().add(index);
                // copy it out, unsafely having a copy of the value on
                // the stack and in the vector at the same time.
                ret = ptr::read(ptr);

                // Shift everything down to fill in that spot.
                ptr::copy(ptr.offset(1), ptr, len - index - 1);
            }
            self.set_len(len - 1);
            ret
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut vec: Vec<_, 8> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
    /// vec.retain(|&x| x % 2 == 0);
    /// assert_eq!(vec, [2, 4]);
    /// ```
    ///
    /// Because the elements are visited exactly once in the original order,
    /// external state may be used to decide which elements to keep.
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut vec: Vec<_, 8> = Vec::from_slice(&[1, 2, 3, 4, 5]).unwrap();
    /// let keep = [false, true, true, false, true];
    /// let mut iter = keep.iter();
    /// vec.retain(|_| *iter.next().unwrap());
    /// assert_eq!(vec, [2, 3, 5]);
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.retain_mut(|elem| f(elem));
    }

    /// Retains only the elements specified by the predicate, passing a mutable reference to it.
    ///
    /// In other words, remove all elements `e` such that `f(&mut e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// let mut vec: Vec<_, 8> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
    /// vec.retain_mut(|x| {
    ///     if *x <= 3 {
    ///         *x += 1;
    ///         true
    ///     } else {
    ///         false
    ///     }
    /// });
    /// assert_eq!(vec, [2, 3, 4]);
    /// ```
    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let original_len = self.len;
        // Avoid double drop if the drop guard is not executed,
        // since we may make some holes during the process.
        unsafe { self.set_len(0) };

        // Vec: [Kept, Kept, Hole, Hole, Hole, Hole, Unchecked, Unchecked]
        //      |<-              processed len   ->| ^- next to check
        //                  |<-  deleted cnt     ->|
        //      |<-              original_len                          ->|
        // Kept: Elements which predicate returns true on.
        // Hole: Moved or dropped element slot.
        // Unchecked: Unchecked valid elements.
        //
        // This drop guard will be invoked when predicate or `drop` of element panicked.
        // It shifts unchecked elements to cover holes and `set_len` to the correct length.
        // In cases when predicate and `drop` never panick, it will be optimized out.
        struct BackshiftOnDrop<'a, T, LenT: LenType, S: VecStorage<T> + ?Sized> {
            v: &'a mut VecInner<T, LenT, S>,
            processed_len: LenT,
            deleted_cnt: LenT,
            original_len: LenT,
        }

        impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Drop for BackshiftOnDrop<'_, T, LenT, S> {
            fn drop(&mut self) {
                if self.deleted_cnt > LenT::ZERO {
                    // SAFETY: Trailing unchecked items must be valid since we never touch them.
                    unsafe {
                        ptr::copy(
                            self.v.as_ptr().add(self.processed_len.into_usize()),
                            self.v
                                .as_mut_ptr()
                                .add((self.processed_len - self.deleted_cnt).into_usize()),
                            (self.original_len - self.processed_len).into_usize(),
                        );
                    }
                }
                // SAFETY: After filling holes, all items are in contiguous memory.
                unsafe {
                    self.v
                        .set_len((self.original_len - self.deleted_cnt).into_usize());
                }
            }
        }

        let mut g = BackshiftOnDrop {
            v: self,
            processed_len: LenT::ZERO,
            deleted_cnt: LenT::ZERO,
            original_len,
        };

        fn process_loop<F, T, LenT: LenType, S: VecStorage<T> + ?Sized, const DELETED: bool>(
            original_len: LenT,
            f: &mut F,
            g: &mut BackshiftOnDrop<'_, T, LenT, S>,
        ) where
            F: FnMut(&mut T) -> bool,
        {
            while g.processed_len != original_len {
                let p = g.v.as_mut_ptr();
                // SAFETY: Unchecked element must be valid.
                let cur = unsafe { &mut *p.add(g.processed_len.into_usize()) };
                if !f(cur) {
                    // Advance early to avoid double drop if `drop_in_place` panicked.
                    g.processed_len += LenT::one();
                    g.deleted_cnt += LenT::one();
                    // SAFETY: We never touch this element again after dropped.
                    unsafe { ptr::drop_in_place(cur) };
                    // We already advanced the counter.
                    if DELETED {
                        continue;
                    } else {
                        break;
                    }
                }
                if DELETED {
                    // SAFETY: `deleted_cnt` > 0, so the hole slot must not overlap with current
                    // element. We use copy for move, and never touch this
                    // element again.
                    unsafe {
                        let hole_slot = p.add((g.processed_len - g.deleted_cnt).into_usize());
                        ptr::copy_nonoverlapping(cur, hole_slot, 1);
                    }
                }
                g.processed_len += LenT::one();
            }
        }

        // Stage 1: Nothing was deleted.
        process_loop::<F, T, LenT, S, false>(original_len, &mut f, &mut g);

        // Stage 2: Some elements were deleted.
        process_loop::<F, T, LenT, S, true>(original_len, &mut f, &mut g);

        // All item are processed. This can be optimized to `set_len` by LLVM.
        drop(g);
    }

    /// Returns the remaining spare capacity of the vector as a slice of `MaybeUninit<T>`.
    ///
    /// The returned slice can be used to fill the vector with data before marking the data as
    /// initialized using the `set_len` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Vec;
    ///
    /// // Allocate vector big enough for 10 elements.
    /// let mut v: Vec<_, 10> = Vec::new();
    ///
    /// // Fill in the first 3 elements.
    /// let uninit = v.spare_capacity_mut();
    /// uninit[0].write(0);
    /// uninit[1].write(1);
    /// uninit[2].write(2);
    ///
    /// // Mark the first 3 elements of the vector as being initialized.
    /// unsafe {
    ///     v.set_len(3);
    /// }
    ///
    /// assert_eq!(&v, &[0, 1, 2]);
    /// ```
    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        &mut self.buffer.borrow_mut()[self.len.into_usize()..]
    }
}

// Trait implementations

impl<T, LenT: LenType, const N: usize> Default for Vec<T, N, LenT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> fmt::Debug for VecInner<T, LenT, S>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[T] as fmt::Debug>::fmt(self, f)
    }
}

impl<LenT: LenType, S: VecStorage<u8> + ?Sized> fmt::Write for VecInner<u8, LenT, S> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.extend_from_slice(s.as_bytes()) {
            Ok(()) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl<T, LenT: LenType, const N: usize, const M: usize> From<[T; M]> for Vec<T, N, LenT> {
    fn from(array: [T; M]) -> Self {
        Self::from_array(array)
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Drop for VecInner<T, LenT, S> {
    fn drop(&mut self) {
        let mut_slice = self.as_mut_slice();
        // We drop each element used in the vector by turning into a `&mut [T]`.
        // SAFETY: the buffer contains initialized data for the range 0..self.len
        unsafe { ptr::drop_in_place(mut_slice) }
    }
}

#[cfg(feature = "alloc")]
/// Converts the given `alloc::vec::Vec<T>` into a `Vec<T, N>`.
impl<T, LenT: LenType, const N: usize> TryFrom<alloc::vec::Vec<T>> for Vec<T, N, LenT> {
    type Error = CapacityError;

    /// Converts the given `alloc::vec::Vec<T>` into a `Vec<T, N>`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the length of the `alloc::vec::Vec<T>` is greater than `N`.
    fn try_from(alloc_vec: alloc::vec::Vec<T>) -> Result<Self, Self::Error> {
        let mut vec = Self::new();

        for e in alloc_vec {
            // Push each element individually to allow handling capacity errors.
            vec.push(e).map_err(|_| CapacityError {})?;
        }

        Ok(vec)
    }
}

#[cfg(feature = "alloc")]
/// Converts the given `Vec<T, N>` into an `alloc::vec::Vec<T>`.
impl<T, LenT: LenType, const N: usize> TryFrom<Vec<T, N, LenT>> for alloc::vec::Vec<T> {
    type Error = alloc::collections::TryReserveError;

    /// Converts the given `Vec<T, N>` into an `alloc::vec::Vec<T>`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the `alloc::vec::Vec` fails to allocate memory.
    fn try_from(vec: Vec<T, N, LenT>) -> Result<Self, Self::Error> {
        let mut alloc_vec = Self::new();

        // Allocate enough space for the elements, return an error if the
        // allocation fails.
        alloc_vec.try_reserve_exact(vec.len())?;

        // Transfer the elements, since we reserved enough space above, this
        // should not fail due to OOM.
        alloc_vec.extend(vec);

        Ok(alloc_vec)
    }
}

impl<'a, T: Clone, LenT: LenType, const N: usize> TryFrom<&'a [T]> for Vec<T, N, LenT> {
    type Error = CapacityError;

    fn try_from(slice: &'a [T]) -> Result<Self, Self::Error> {
        Self::from_slice(slice)
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Extend<T> for VecInner<T, LenT, S> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.extend(iter);
    }
}

impl<'a, T, LenT: LenType, S: VecStorage<T> + ?Sized> Extend<&'a T> for VecInner<T, LenT, S>
where
    T: 'a + Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> hash::Hash for VecInner<T, LenT, S>
where
    T: core::hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        <[T] as hash::Hash>::hash(self, state);
    }
}

impl<'a, T, LenT: LenType, S: VecStorage<T> + ?Sized> IntoIterator for &'a VecInner<T, LenT, S> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, LenT: LenType, S: VecStorage<T> + ?Sized> IntoIterator
    for &'a mut VecInner<T, LenT, S>
{
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, LenT: LenType, const N: usize> FromIterator<T> for Vec<T, N, LenT> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut vec = Self::new();
        for i in iter {
            vec.push(i).ok().expect("Vec::from_iter overflow");
        }
        vec
    }
}

/// An iterator that moves out of an [`Vec`][`Vec`].
///
/// This struct is created by calling the `into_iter` method on [`Vec`][`Vec`].
pub struct IntoIter<T, const N: usize, LenT: LenType> {
    vec: Vec<T, N, LenT>,
    next: LenT,
}

impl<T, LenT: LenType, const N: usize> Iterator for IntoIter<T, N, LenT> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.vec.len {
            let item = unsafe {
                self.vec
                    .buffer
                    .buffer
                    .get_unchecked_mut(self.next.into_usize())
                    .as_ptr()
                    .read()
            };
            self.next += LenT::one();
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T, LenT: LenType, const N: usize> DoubleEndedIterator for IntoIter<T, N, LenT> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.next < self.vec.len {
            // SAFETY: len must be non-zero as next is less than len.
            let item = unsafe { self.vec.pop_unchecked() };
            Some(item)
        } else {
            None
        }
    }
}

impl<T, LenT: LenType, const N: usize> FusedIterator for IntoIter<T, N, LenT> {}

impl<T, LenT: LenType, const N: usize> ExactSizeIterator for IntoIter<T, N, LenT> {
    fn len(&self) -> usize {
        (self.vec.len - self.next).into_usize()
    }
}

impl<T, LenT: LenType, const N: usize> Clone for IntoIter<T, N, LenT>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut vec = Vec::new();

        if self.next < self.vec.len {
            let s = unsafe {
                slice::from_raw_parts(
                    self.vec
                        .buffer
                        .buffer
                        .as_ptr()
                        .cast::<T>()
                        .add(self.next.into_usize()),
                    (self.vec.len - self.next).into_usize(),
                )
            };
            vec.extend_from_slice(s).ok();
        }

        Self {
            vec,
            next: LenT::ZERO,
        }
    }
}

impl<T, LenT: LenType, const N: usize> core::fmt::Debug for IntoIter<T, N, LenT>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = if self.next < self.vec.len {
            unsafe {
                slice::from_raw_parts(
                    self.vec
                        .buffer
                        .buffer
                        .as_ptr()
                        .cast::<T>()
                        .add(self.next.into_usize()),
                    (self.vec.len - self.next).into_usize(),
                )
            }
        } else {
            &[]
        };

        write!(f, "{s:?}")
    }
}

impl<T, LenT: LenType, const N: usize> Drop for IntoIter<T, N, LenT> {
    fn drop(&mut self) {
        unsafe {
            // Drop all the elements that have not been moved out of vec
            ptr::drop_in_place(&mut self.vec.as_mut_slice()[self.next.into_usize()..]);
            // Prevent dropping of other elements
            self.vec.len = LenT::ZERO;
        }
    }
}

impl<T, LenT: LenType, const N: usize> IntoIterator for Vec<T, N, LenT> {
    type Item = T;
    type IntoIter = IntoIter<T, N, LenT>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: self,
            next: LenT::ZERO,
        }
    }
}

impl<A, B, LenTA, LenTB, SA, SB> PartialEq<VecInner<B, LenTB, SB>> for VecInner<A, LenTA, SA>
where
    A: PartialEq<B>,
    LenTA: LenType,
    LenTB: LenType,
    SA: VecStorage<A> + ?Sized,
    SB: VecStorage<B> + ?Sized,
{
    fn eq(&self, other: &VecInner<B, LenTB, SB>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<A, B, LenTB, const M: usize, SB> PartialEq<VecInner<B, LenTB, SB>> for [A; M]
where
    A: PartialEq<B>,
    LenTB: LenType,
    SB: VecStorage<B>,
{
    fn eq(&self, other: &VecInner<B, LenTB, SB>) -> bool {
        self.eq(other.as_slice())
    }
}

impl<A, B, LenTB, SB, const M: usize> PartialEq<VecInner<B, LenTB, SB>> for &[A; M]
where
    A: PartialEq<B>,
    LenTB: LenType,
    SB: VecStorage<B>,
{
    fn eq(&self, other: &VecInner<B, LenTB, SB>) -> bool {
        (*self).eq(other)
    }
}

impl<A, B, LenTB, SB> PartialEq<VecInner<B, LenTB, SB>> for [A]
where
    A: PartialEq<B>,
    LenTB: LenType,
    SB: VecStorage<B>,
{
    fn eq(&self, other: &VecInner<B, LenTB, SB>) -> bool {
        self.eq(other.as_slice())
    }
}

impl<A, B, LenTB, SB> PartialEq<VecInner<B, LenTB, SB>> for &[A]
where
    A: PartialEq<B>,
    LenTB: LenType,
    SB: VecStorage<B>,
{
    fn eq(&self, other: &VecInner<B, LenTB, SB>) -> bool {
        (*self).eq(other)
    }
}

impl<A, B, LenTB: LenType, SB: VecStorage<B>> PartialEq<VecInner<B, LenTB, SB>> for &mut [A]
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &VecInner<B, LenTB, SB>) -> bool {
        (**self).eq(other)
    }
}

impl<A, B, LenTA: LenType, SA, const N: usize> PartialEq<[B; N]> for VecInner<A, LenTA, SA>
where
    A: PartialEq<B>,
    SA: VecStorage<A> + ?Sized,
{
    #[inline]
    fn eq(&self, other: &[B; N]) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<A, B, LenTA, SA, const N: usize> PartialEq<&[B; N]> for VecInner<A, LenTA, SA>
where
    A: PartialEq<B>,
    LenTA: LenType,
    SA: VecStorage<A> + ?Sized,
{
    #[inline]
    fn eq(&self, other: &&[B; N]) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<A, B, LenTA, SA> PartialEq<[B]> for VecInner<A, LenTA, SA>
where
    A: PartialEq<B>,
    LenTA: LenType,
    SA: VecStorage<A> + ?Sized,
{
    #[inline]
    fn eq(&self, other: &[B]) -> bool {
        self.as_slice().eq(other)
    }
}

impl<A, B, LenTA, SA> PartialEq<&[B]> for VecInner<A, LenTA, SA>
where
    A: PartialEq<B>,
    LenTA: LenType,
    SA: VecStorage<A> + ?Sized,
{
    #[inline]
    fn eq(&self, other: &&[B]) -> bool {
        self.as_slice().eq(*other)
    }
}

impl<A, B, LenTA, SA> PartialEq<&mut [B]> for VecInner<A, LenTA, SA>
where
    A: PartialEq<B>,
    LenTA: LenType,
    SA: VecStorage<A> + ?Sized,
{
    #[inline]
    fn eq(&self, other: &&mut [B]) -> bool {
        self.as_slice().eq(*other)
    }
}

// Implements Eq if underlying data is Eq
impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Eq for VecInner<T, LenT, S> where T: Eq {}

impl<T, LenTA: LenType, LenTB: LenType, SA: VecStorage<T> + ?Sized, SB: VecStorage<T> + ?Sized>
    PartialOrd<VecInner<T, LenTA, SA>> for VecInner<T, LenTB, SB>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &VecInner<T, LenTA, SA>) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Ord for VecInner<T, LenT, S>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> ops::Deref for VecInner<T, LenT, S> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> ops::DerefMut for VecInner<T, LenT, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> borrow::Borrow<[T]> for VecInner<T, LenT, S> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> borrow::BorrowMut<[T]> for VecInner<T, LenT, S> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> AsRef<Self> for VecInner<T, LenT, S> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> AsMut<Self> for VecInner<T, LenT, S> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> AsRef<[T]> for VecInner<T, LenT, S> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> AsMut<[T]> for VecInner<T, LenT, S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T, const N: usize, LenT: LenType> Clone for Vec<T, N, LenT>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;

    use static_assertions::assert_not_impl_any;

    use super::{Vec, VecView};

    // Ensure a `Vec` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(Vec<*const (), 4>: Send);

    #[test]
    fn static_new() {
        static mut _V: Vec<i32, 4> = Vec::new();
    }

    #[test]
    fn stack_new() {
        let mut _v: Vec<i32, 4> = Vec::new();
    }

    #[test]
    fn is_full_empty() {
        let mut v: Vec<i32, 4> = Vec::new();

        assert!(v.is_empty());
        assert!(!v.is_full());

        v.push(1).unwrap();
        assert!(!v.is_empty());
        assert!(!v.is_full());

        v.push(1).unwrap();
        assert!(!v.is_empty());
        assert!(!v.is_full());

        v.push(1).unwrap();
        assert!(!v.is_empty());
        assert!(!v.is_full());

        v.push(1).unwrap();
        assert!(!v.is_empty());
        assert!(v.is_full());
    }

    #[test]
    fn drop() {
        droppable!();

        {
            let mut v: Vec<Droppable, 2> = Vec::new();
            v.push(Droppable::new()).ok().unwrap();
            v.push(Droppable::new()).ok().unwrap();
            v.pop().unwrap();
        }

        assert_eq!(Droppable::count(), 0);

        {
            let mut v: Vec<Droppable, 2> = Vec::new();
            v.push(Droppable::new()).ok().unwrap();
            v.push(Droppable::new()).ok().unwrap();
        }

        assert_eq!(Droppable::count(), 0);
    }

    #[test]
    fn drop_vecview() {
        droppable!();

        {
            let v: Vec<Droppable, 2> = Vec::new();
            let v: Box<Vec<Droppable, 2>> = Box::new(v);
            let mut v: Box<VecView<Droppable>> = v;
            v.push(Droppable::new()).ok().unwrap();
            v.push(Droppable::new()).ok().unwrap();
            assert_eq!(Droppable::count(), 2);
            v.pop().unwrap();
            assert_eq!(Droppable::count(), 1);
        }

        assert_eq!(Droppable::count(), 0);

        {
            let v: Vec<Droppable, 2> = Vec::new();
            let v: Box<Vec<Droppable, 2>> = Box::new(v);
            let mut v: Box<VecView<Droppable>> = v;
            v.push(Droppable::new()).ok().unwrap();
            v.push(Droppable::new()).ok().unwrap();
            assert_eq!(Droppable::count(), 2);
        }

        assert_eq!(Droppable::count(), 0);
    }

    #[test]
    fn eq() {
        let mut xs: Vec<i32, 4> = Vec::new();
        let mut ys: Vec<i32, 8> = Vec::new();

        assert_eq!(xs, ys);

        xs.push(1).unwrap();
        ys.push(1).unwrap();

        assert_eq!(xs, ys);
    }

    #[test]
    fn cmp() {
        let mut xs: Vec<i32, 4> = Vec::new();
        let mut ys: Vec<i32, 4> = Vec::new();

        assert_eq!(xs, ys);

        xs.push(1).unwrap();
        ys.push(2).unwrap();

        assert!(xs < ys);
    }

    #[test]
    fn cmp_heterogenous_size() {
        let mut xs: Vec<i32, 4> = Vec::new();
        let mut ys: Vec<i32, 8> = Vec::new();

        assert_eq!(xs, ys);

        xs.push(1).unwrap();
        ys.push(2).unwrap();

        assert!(xs < ys);
    }

    #[test]
    fn cmp_with_arrays_and_slices() {
        let mut xs: Vec<i32, 12> = Vec::new();
        xs.push(1).unwrap();

        let array = [1];

        assert_eq!(xs, array);
        assert_eq!(array, xs);

        assert_eq!(xs, array.as_slice());
        assert_eq!(array.as_slice(), xs);

        assert_eq!(xs, &array);
        assert_eq!(&array, xs);

        let longer_array = [1; 20];

        assert_ne!(xs, longer_array);
        assert_ne!(longer_array, xs);
    }

    #[test]
    fn full() {
        let mut v: Vec<i32, 4> = Vec::new();

        v.push(0).unwrap();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();

        assert!(v.push(4).is_err());
    }

    #[test]
    fn iter() {
        let mut v: Vec<i32, 4> = Vec::new();

        v.push(0).unwrap();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();

        let mut items = v.iter();

        assert_eq!(items.next(), Some(&0));
        assert_eq!(items.next(), Some(&1));
        assert_eq!(items.next(), Some(&2));
        assert_eq!(items.next(), Some(&3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut v: Vec<i32, 4> = Vec::new();

        v.push(0).unwrap();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();

        let mut items = v.iter_mut();

        assert_eq!(items.next(), Some(&mut 0));
        assert_eq!(items.next(), Some(&mut 1));
        assert_eq!(items.next(), Some(&mut 2));
        assert_eq!(items.next(), Some(&mut 3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn collect_from_iter() {
        let slice = &[1, 2, 3];
        let vec: Vec<i32, 4> = slice.iter().cloned().collect();
        assert_eq!(&vec, slice);
    }

    #[test]
    #[should_panic]
    fn collect_from_iter_overfull() {
        let slice = &[1, 2, 3];
        let _vec = slice.iter().cloned().collect::<Vec<_, 2>>();
    }

    #[test]
    fn iter_move() {
        let mut v: Vec<i32, 4> = Vec::new();
        v.push(0).unwrap();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();

        let mut items = v.into_iter();

        assert_eq!(items.next(), Some(0));
        assert_eq!(items.next(), Some(1));
        assert_eq!(items.next(), Some(2));
        assert_eq!(items.next(), Some(3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_move_drop() {
        droppable!();

        {
            let mut vec: Vec<Droppable, 2> = Vec::new();
            vec.push(Droppable::new()).ok().unwrap();
            vec.push(Droppable::new()).ok().unwrap();
            let mut items = vec.into_iter();
            // Move all
            let _ = items.next();
            let _ = items.next();
        }

        assert_eq!(Droppable::count(), 0);

        {
            let mut vec: Vec<Droppable, 2> = Vec::new();
            vec.push(Droppable::new()).ok().unwrap();
            vec.push(Droppable::new()).ok().unwrap();
            let _items = vec.into_iter();
            // Move none
        }

        assert_eq!(Droppable::count(), 0);

        {
            let mut vec: Vec<Droppable, 2> = Vec::new();
            vec.push(Droppable::new()).ok().unwrap();
            vec.push(Droppable::new()).ok().unwrap();
            let mut items = vec.into_iter();
            let _ = items.next(); // Move partly
        }

        assert_eq!(Droppable::count(), 0);
    }

    #[test]
    fn iter_move_next_back() {
        let mut vec: Vec<&str, 3> = Vec::new();
        vec.push("a").unwrap();
        vec.push("b").unwrap();
        vec.push("c").unwrap();
        let mut items = vec.into_iter();
        let _ = items.next(); // Remove the first item.
        assert_eq!(items.next_back(), Some("c"));
        assert_eq!(items.next_back(), Some("b"));
        assert_eq!(items.next_back(), None);
    }

    #[test]
    fn iter_move_len() {
        let mut vec: Vec<&str, 2> = Vec::new();
        vec.push("a").unwrap();
        vec.push("b").unwrap();
        let mut items = vec.into_iter();
        assert_eq!(items.len(), 2);
        let _ = items.next(); // Remove the first item.
        assert_eq!(items.len(), 1);
        let _ = items.next_back(); // Remove the last item.
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn push_and_pop() {
        let mut v: Vec<i32, 4> = Vec::new();
        assert_eq!(v.len(), 0);

        assert_eq!(v.pop(), None);
        assert_eq!(v.len(), 0);

        v.push(0).unwrap();
        assert_eq!(v.len(), 1);

        assert_eq!(v.pop(), Some(0));
        assert_eq!(v.len(), 0);

        assert_eq!(v.pop(), None);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn resize_size_limit() {
        let mut v: Vec<u8, 4> = Vec::new();

        v.resize(0, 0).unwrap();
        v.resize(4, 0).unwrap();
        v.resize(5, 0).expect_err("full");
    }

    #[test]
    fn resize_length_cases() {
        let mut v: Vec<u8, 4> = Vec::new();

        assert_eq!(v.len(), 0);

        // Grow by 1
        v.resize(1, 0).unwrap();
        assert_eq!(v.len(), 1);

        // Grow by 2
        v.resize(3, 0).unwrap();
        assert_eq!(v.len(), 3);

        // Resize to current size
        v.resize(3, 0).unwrap();
        assert_eq!(v.len(), 3);

        // Shrink by 1
        v.resize(2, 0).unwrap();
        assert_eq!(v.len(), 2);

        // Shrink by 2
        v.resize(0, 0).unwrap();
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn resize_contents() {
        let mut v: Vec<u8, 4> = Vec::new();

        // New entries take supplied value when growing
        v.resize(1, 17).unwrap();
        assert_eq!(v[0], 17);

        // Old values aren't changed when growing
        v.resize(2, 18).unwrap();
        assert_eq!(v[0], 17);
        assert_eq!(v[1], 18);

        // Old values aren't changed when length unchanged
        v.resize(2, 0).unwrap();
        assert_eq!(v[0], 17);
        assert_eq!(v[1], 18);

        // Old values aren't changed when shrinking
        v.resize(1, 0).unwrap();
        assert_eq!(v[0], 17);
    }

    #[test]
    fn resize_default() {
        let mut v: Vec<u8, 4> = Vec::new();

        // resize_default is implemented using resize, so just check the
        // correct value is being written.
        v.resize_default(1).unwrap();
        assert_eq!(v[0], 0);
    }

    #[test]
    fn write() {
        let mut v: Vec<u8, 4> = Vec::new();
        write!(v, "{:x}", 1234).unwrap();
        assert_eq!(&v[..], b"4d2");
    }

    #[test]
    fn extend_from_slice() {
        let mut v: Vec<u8, 4> = Vec::new();
        assert_eq!(v.len(), 0);
        v.extend_from_slice(&[1, 2]).unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v.as_slice(), &[1, 2]);
        v.extend_from_slice(&[3]).unwrap();
        assert_eq!(v.len(), 3);
        assert_eq!(v.as_slice(), &[1, 2, 3]);
        assert!(v.extend_from_slice(&[4, 5]).is_err());
        assert_eq!(v.len(), 3);
        assert_eq!(v.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn from_slice() {
        // Successful construction
        let v: Vec<u8, 4> = Vec::from_slice(&[1, 2, 3]).unwrap();
        assert_eq!(v.len(), 3);
        assert_eq!(v.as_slice(), &[1, 2, 3]);

        // Slice too large
        assert!(Vec::<u8, 2>::from_slice(&[1, 2, 3]).is_err());
    }

    #[test]
    fn from_array() {
        // Successful construction, N == M
        let v: Vec<u8, 3> = Vec::from_array([1, 2, 3]);
        assert_eq!(v, Vec::<u8, 3>::from([1, 2, 3]));
        assert_eq!(v.len(), 3);
        assert_eq!(v.as_slice(), &[1, 2, 3]);

        // Successful construction, N > M
        let v: Vec<u8, 4> = Vec::from_array([1, 2, 3]);
        assert_eq!(v, Vec::<u8, 4>::from([1, 2, 3]));
        assert_eq!(v.len(), 3);
        assert_eq!(v.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn from_array_no_drop() {
        struct Drops(Option<u8>);

        impl Drop for Drops {
            fn drop(&mut self) {
                self.0 = None;
            }
        }

        let v: Vec<Drops, 3> = Vec::from([Drops(Some(1)), Drops(Some(2)), Drops(Some(3))]);

        assert_eq!(v[0].0, Some(1));
        assert_eq!(v[1].0, Some(2));
        assert_eq!(v[2].0, Some(3));
    }

    #[test]
    fn starts_with() {
        let v: Vec<_, 8> = Vec::from_slice(b"ab").unwrap();
        assert!(v.starts_with(&[]));
        assert!(v.starts_with(b""));
        assert!(v.starts_with(b"a"));
        assert!(v.starts_with(b"ab"));
        assert!(!v.starts_with(b"abc"));
        assert!(!v.starts_with(b"ba"));
        assert!(!v.starts_with(b"b"));
    }

    #[test]
    fn ends_with() {
        let v: Vec<_, 8> = Vec::from_slice(b"ab").unwrap();
        assert!(v.ends_with(&[]));
        assert!(v.ends_with(b""));
        assert!(v.ends_with(b"b"));
        assert!(v.ends_with(b"ab"));
        assert!(!v.ends_with(b"abc"));
        assert!(!v.ends_with(b"ba"));
        assert!(!v.ends_with(b"a"));
    }

    #[test]
    fn spare_capacity_mut() {
        let mut v: Vec<_, 4> = Vec::new();
        let uninit = v.spare_capacity_mut();
        assert_eq!(uninit.len(), 4);
        uninit[0].write(1);
        uninit[1].write(2);
        uninit[2].write(3);
        unsafe { v.set_len(3) };
        assert_eq!(v.as_slice(), &[1, 2, 3]);

        let uninit = v.spare_capacity_mut();
        assert_eq!(uninit.len(), 1);
        uninit[0].write(4);
        unsafe { v.set_len(4) };
        assert_eq!(v.as_slice(), &[1, 2, 3, 4]);

        assert!(v.spare_capacity_mut().is_empty());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn heapless_to_alloc() {
        let mut hv: Vec<u8, 4> = Vec::new();
        hv.push(0).unwrap();
        hv.push(1).unwrap();

        let av: alloc::vec::Vec<u8> = hv.clone().try_into().unwrap();
        assert_eq!(av.as_slice(), hv.as_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn alloc_to_heapless() {
        let mut av: alloc::vec::Vec<u8> = alloc::vec::Vec::new();
        av.push(0);
        av.push(1);

        let hv: Vec<u8, 2> = av.clone().try_into().unwrap();
        assert_eq!(hv.as_slice(), av.as_slice());

        let _: crate::CapacityError =
            <alloc::vec::Vec<u8> as TryInto<Vec<u8, 1>>>::try_into(av.clone()).unwrap_err();
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_vec_zeroize() {
        use zeroize::Zeroize;

        let mut v: Vec<u8, 8> = Vec::new();
        for i in 0..8 {
            v.push(i).unwrap();
        }

        for i in 0..8 {
            assert_eq!(v[i], i as u8);
        }

        v.truncate(4);
        assert_eq!(v.len(), 4);

        for i in 0..4 {
            assert_eq!(v[i], i as u8);
        }

        v.zeroize();

        assert_eq!(v.len(), 0);

        unsafe {
            v.set_len(8);
        }

        for i in 0..8 {
            assert_eq!(v[i], 0);
        }
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_vecview_zeroize() {
        use zeroize::Zeroize;

        let mut v: Vec<u8, 8> = Vec::new();
        for i in 0..8 {
            v.push(i).unwrap();
        }

        let view = v.as_mut_view();

        for i in 0..8 {
            assert_eq!(view[i], i as u8);
        }

        view.zeroize();

        assert_eq!(view.len(), 0);

        unsafe {
            view.set_len(8);
        }

        for i in 0..8 {
            assert_eq!(view[i], 0);
        }
    }

    fn _test_variance<'a: 'b, 'b>(x: Vec<&'a (), 42>) -> Vec<&'b (), 42> {
        x
    }
    fn _test_variance_view<'a: 'b, 'b, 'c>(x: &'c VecView<&'a ()>) -> &'c VecView<&'b ()> {
        x
    }
}
