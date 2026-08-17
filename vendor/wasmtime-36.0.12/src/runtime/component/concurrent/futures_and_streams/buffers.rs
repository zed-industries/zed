#[cfg(feature = "component-model-async-bytes")]
use bytes::{Bytes, BytesMut};
#[cfg(feature = "component-model-async-bytes")]
use std::io::Cursor;
use std::mem::{self, MaybeUninit};
use std::slice;
use std::vec::Vec;

// Inner module here to restrict possible readers of the fields of
// `UntypedWriteBuffer`.
pub use untyped::*;
mod untyped {
    use super::WriteBuffer;
    use std::any::TypeId;
    use std::marker;
    use std::mem;

    /// Helper structure to type-erase the `T` in `WriteBuffer<T>`.
    ///
    /// This is constructed with a `&mut dyn WriteBuffer<T>` and then can only
    /// be viewed as `&mut dyn WriteBuffer<T>` as well. The `T`, however, is
    /// carried through methods rather than the struct itself.
    ///
    /// Note that this structure has a lifetime `'a` which forces an active
    /// borrow on the original buffer passed in.
    pub struct UntypedWriteBuffer<'a> {
        element_type_id: TypeId,
        buf: *mut dyn WriteBuffer<()>,
        _marker: marker::PhantomData<&'a mut dyn WriteBuffer<()>>,
    }

    /// Helper structure to transmute between `WriteBuffer<T>` and
    /// `WriteBuffer<()>`.
    union ReinterpretWriteBuffer<T> {
        typed: *mut dyn WriteBuffer<T>,
        untyped: *mut dyn WriteBuffer<()>,
    }

    impl<'a> UntypedWriteBuffer<'a> {
        /// Creates a new `UntypedWriteBuffer` from the `buf` provided.
        ///
        /// The returned value can be used with the `get_mut` method to get the
        /// original write buffer back.
        pub fn new<T: 'static>(buf: &'a mut dyn WriteBuffer<T>) -> UntypedWriteBuffer<'a> {
            UntypedWriteBuffer {
                element_type_id: TypeId::of::<T>(),
                // SAFETY: this is `unsafe` due to reading union fields. That
                // is safe here because `typed` and `untyped` have the same size
                // and we're otherwise reinterpreting a raw pointer with a type
                // parameter to one without one.
                buf: unsafe {
                    let r = ReinterpretWriteBuffer { typed: buf };
                    assert_eq!(mem::size_of_val(&r.typed), mem::size_of_val(&r.untyped));
                    r.untyped
                },
                _marker: marker::PhantomData,
            }
        }

        /// Acquires the underyling `WriteBuffer<T>` this was created with.
        ///
        /// # Panics
        ///
        /// Panics if `T` does not match the type that this was created with.
        pub fn get_mut<T: 'static>(&mut self) -> &mut dyn WriteBuffer<T> {
            assert_eq!(self.element_type_id, TypeId::of::<T>());
            // SAFETY: the `T` has been checked with `TypeId` and this
            // structure also is proof of valid existence of the original
            // `&mut WriteBuffer<T>`, so taking the raw pointer back to a safe
            // reference is valid.
            unsafe { &mut *ReinterpretWriteBuffer { untyped: self.buf }.typed }
        }
    }
}

/// Trait representing a buffer which may be written to a `StreamWriter`.
///
/// See also [`crate::component::Instance::stream`].
///
/// # Unsafety
///
/// This trait is unsafe due to the contract of the `take` function. This trait
/// is only safe to implement if the `take` function is implemented correctly,
/// namely that all the items passed to the closure are fully initialized for
/// `T`.
pub unsafe trait WriteBuffer<T>: Send + Sync + 'static {
    /// Slice of items remaining to be read.
    fn remaining(&self) -> &[T];

    /// Skip and drop the specified number of items.
    fn skip(&mut self, count: usize);

    /// Take ownership of the specified number of items.
    ///
    /// This function will take `count` items from `self` and pass them as a
    /// contiguous slice to the closure `fun` provided. The `fun` closure may
    /// assume that the items are all fully initialized and available to read.
    /// It is expected that `fun` will read all the items provided. Any items
    /// that aren't read by `fun` will be leaked.
    ///
    /// # Panics
    ///
    /// Panics if `count` is larger than `self.remaining()`. If `fun` panics
    /// then items may be leaked.
    fn take(&mut self, count: usize, fun: &mut dyn FnMut(&[MaybeUninit<T>]));
}

/// Trait representing a buffer which may be used to read from a `StreamReader`.
///
/// See also [`crate::component::Instance::stream`].
pub trait ReadBuffer<T>: Send + Sync + 'static {
    /// Move the specified items into this buffer.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I);

    /// Number of items which may be read before this buffer is full.
    fn remaining_capacity(&self) -> usize;

    /// Move (i.e. take ownership of) the specified items into this buffer.
    ///
    /// This method will drain `count` items from the `input` provided and move
    /// ownership into this buffer.
    ///
    /// # Panics
    ///
    /// This method will panic if `count` is larger than
    /// `self.remaining_capacity()` or if it's larger than `input.remaining()`.
    fn move_from(&mut self, input: &mut dyn WriteBuffer<T>, count: usize);
}

pub(super) struct Extender<'a, B>(pub(super) &'a mut B);

impl<T, B: ReadBuffer<T>> Extend<T> for Extender<'_, B> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

// SAFETY: the `take` implementation below guarantees that the `fun` closure is
// provided with fully initialized items.
unsafe impl<T: Send + Sync + 'static> WriteBuffer<T> for Option<T> {
    fn remaining(&self) -> &[T] {
        if let Some(me) = self {
            slice::from_ref(me)
        } else {
            &[]
        }
    }

    fn skip(&mut self, count: usize) {
        match count {
            0 => {}
            1 => {
                assert!(self.is_some());
                *self = None;
            }
            _ => panic!("cannot skip more than {} item(s)", self.remaining().len()),
        }
    }

    fn take(&mut self, count: usize, fun: &mut dyn FnMut(&[MaybeUninit<T>])) {
        match count {
            0 => fun(&mut []),
            1 => {
                let mut item = MaybeUninit::new(self.take().unwrap());
                fun(slice::from_mut(&mut item));
            }
            _ => panic!("cannot forget more than {} item(s)", self.remaining().len()),
        }
    }
}

impl<T: Send + Sync + 'static> ReadBuffer<T> for Option<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        if self.is_none() {
            *self = iter.next();
        }
        assert!(iter.next().is_none());
    }

    fn remaining_capacity(&self) -> usize {
        if self.is_some() { 0 } else { 1 }
    }

    fn move_from(&mut self, input: &mut dyn WriteBuffer<T>, count: usize) {
        match count {
            0 => {}
            1 => {
                assert!(self.is_none());
                input.take(1, &mut |slice| {
                    // SAFETY: Per the `WriteBuffer` trait contract this block
                    // has ownership of the items in `slice` and they're all
                    // valid to take.
                    unsafe {
                        *self = Some(slice[0].assume_init_read());
                    }
                });
            }
            _ => panic!(
                "cannot take more than {} item(s)",
                self.remaining_capacity()
            ),
        }
    }
}

/// A `WriteBuffer` implementation, backed by a `Vec`.
pub struct VecBuffer<T> {
    buffer: Vec<MaybeUninit<T>>,
    offset: usize,
}

impl<T> VecBuffer<T> {
    /// Create a new instance with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            offset: 0,
        }
    }

    /// Reset the state of this buffer, removing all items and preserving its
    /// capacity.
    pub fn reset(&mut self) {
        self.skip_(self.remaining_().len());
        self.buffer.clear();
        self.offset = 0;
    }

    fn remaining_(&self) -> &[T] {
        // SAFETY: This relies on the invariant (upheld in the other methods of
        // this type) that all the elements from `self.offset` onward are
        // initialized and valid for `self.buffer`.
        unsafe { mem::transmute::<&[MaybeUninit<T>], &[T]>(&self.buffer[self.offset..]) }
    }

    fn skip_(&mut self, count: usize) {
        assert!(count <= self.remaining_().len());
        for item in &mut self.buffer[self.offset..][..count] {
            // Note that the offset is incremented first here to ensure that if
            // any destructors panic we don't attempt to re-drop the item.
            self.offset += 1;
            // SAFETY: See comment in `Self::remaining`
            unsafe {
                item.assume_init_drop();
            }
        }
    }
}

// SAFETY: the `take` implementation below guarantees that the `fun` closure is
// provided with fully initialized items due to `self.offset`-and-onwards being
// always initialized.
unsafe impl<T: Send + Sync + 'static> WriteBuffer<T> for VecBuffer<T> {
    fn remaining(&self) -> &[T] {
        self.remaining_()
    }

    fn skip(&mut self, count: usize) {
        self.skip_(count)
    }

    fn take(&mut self, count: usize, fun: &mut dyn FnMut(&[MaybeUninit<T>])) {
        assert!(count <= self.remaining().len());
        // Note that the offset here is incremented before `fun` is called to
        // ensure that if `fun` panics that the items are still considered
        // transferred.
        self.offset += count;
        fun(&mut self.buffer[self.offset - count..]);
    }
}

impl<T> From<Vec<T>> for VecBuffer<T> {
    fn from(buffer: Vec<T>) -> Self {
        Self {
            // SAFETY: Transmuting from `Vec<T>` to `Vec<MaybeUninit<T>>` should
            // be sound for any `T`.
            buffer: unsafe { mem::transmute::<Vec<T>, Vec<MaybeUninit<T>>>(buffer) },
            offset: 0,
        }
    }
}

impl<T> Drop for VecBuffer<T> {
    fn drop(&mut self) {
        self.reset();
    }
}

impl<T: Send + Sync + 'static> ReadBuffer<T> for Vec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        Extend::extend(self, iter)
    }

    fn remaining_capacity(&self) -> usize {
        self.capacity().checked_sub(self.len()).unwrap()
    }

    fn move_from(&mut self, input: &mut dyn WriteBuffer<T>, count: usize) {
        assert!(count <= self.remaining_capacity());
        input.take(count, &mut |slice| {
            for item in slice {
                // SAFETY: Per the `WriteBuffer` implementation contract this
                // function has exclusive ownership of all items in `slice` so
                // this is safe to take and transfer them here.
                self.push(unsafe { item.assume_init_read() });
            }
        });
    }
}

// SAFETY: the `take` implementation below guarantees that the `fun` closure is
// provided with fully initialized items.
#[cfg(feature = "component-model-async-bytes")]
unsafe impl WriteBuffer<u8> for Cursor<Bytes> {
    fn remaining(&self) -> &[u8] {
        &self.get_ref()[usize::try_from(self.position()).unwrap()..]
    }

    fn skip(&mut self, count: usize) {
        assert!(
            count <= self.remaining().len(),
            "tried to skip {count} with {} remaining",
            self.remaining().len()
        );
        self.set_position(
            self.position()
                .checked_add(u64::try_from(count).unwrap())
                .unwrap(),
        );
    }

    fn take(&mut self, count: usize, fun: &mut dyn FnMut(&[MaybeUninit<u8>])) {
        assert!(count <= self.remaining().len());
        fun(unsafe_byte_slice(self.remaining()));
        self.skip(count);
    }
}

// SAFETY: the `take` implementation below guarantees that the `fun` closure is
// provided with fully initialized items.
#[cfg(feature = "component-model-async-bytes")]
unsafe impl WriteBuffer<u8> for Cursor<BytesMut> {
    fn remaining(&self) -> &[u8] {
        &self.get_ref()[usize::try_from(self.position()).unwrap()..]
    }

    fn skip(&mut self, count: usize) {
        assert!(count <= self.remaining().len());
        self.set_position(
            self.position()
                .checked_add(u64::try_from(count).unwrap())
                .unwrap(),
        );
    }

    fn take(&mut self, count: usize, fun: &mut dyn FnMut(&[MaybeUninit<u8>])) {
        assert!(count <= self.remaining().len());
        fun(unsafe_byte_slice(self.remaining()));
        self.skip(count);
    }
}

#[cfg(feature = "component-model-async-bytes")]
impl ReadBuffer<u8> for BytesMut {
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        Extend::extend(self, iter)
    }

    fn remaining_capacity(&self) -> usize {
        self.capacity().checked_sub(self.len()).unwrap()
    }

    fn move_from(&mut self, input: &mut dyn WriteBuffer<u8>, count: usize) {
        assert!(count <= self.remaining_capacity());
        input.take(count, &mut |slice| {
            // SAFETY: per the contract of `WriteBuffer` all the elements of
            // the input `slice` are fully initialized so this is safe
            // to reinterpret the slice.
            let slice = unsafe { mem::transmute::<&[MaybeUninit<u8>], &[u8]>(slice) };
            self.extend_from_slice(slice);
        });
    }
}

#[cfg(feature = "component-model-async-bytes")]
fn unsafe_byte_slice(slice: &[u8]) -> &[MaybeUninit<u8>] {
    // SAFETY: it's always safe to interpret a slice of items as a
    // possibly-initialized slice of items.
    unsafe { mem::transmute::<&[u8], &[MaybeUninit<u8>]>(slice) }
}
