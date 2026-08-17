use crate::{
    ser::{ScratchSpace, Serializer},
    Fallible,
};
use core::{
    alloc::Layout,
    fmt,
    ops::DerefMut,
    ptr::{copy_nonoverlapping, NonNull},
};

/// The error type returned by an [`BufferSerializer`].
#[derive(Debug)]
pub enum BufferSerializerError {
    /// Writing has overflowed the internal buffer.
    Overflow {
        /// The position of the serializer
        pos: usize,
        /// The number of bytes needed
        bytes_needed: usize,
        /// The total length of the archive
        archive_len: usize,
    },
}

impl fmt::Display for BufferSerializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow {
                pos,
                bytes_needed,
                archive_len,
            } => write!(
                f,
                "writing has overflowed the serializer buffer: pos {}, needed {}, total length {}",
                pos, bytes_needed, archive_len
            ),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl Error for BufferSerializerError {}
};

/// Wraps a byte buffer and equips it with [`Serializer`].
///
/// Common uses include archiving in `#![no_std]` environments and archiving small objects without
/// allocating.
///
/// # Examples
/// ```
/// use rkyv::{
///     archived_value,
///     ser::{Serializer, serializers::BufferSerializer},
///     AlignedBytes,
///     AlignedVec,
///     Archive,
///     Archived,
///     Serialize,
/// };
///
/// #[derive(Archive, Serialize)]
/// enum Event {
///     Spawn,
///     Speak(String),
///     Die,
/// }
///
/// let mut serializer = BufferSerializer::new(AlignedBytes([0u8; 256]));
/// let pos = serializer.serialize_value(&Event::Speak("Help me!".to_string()))
///     .expect("failed to archive event");
/// let buf = serializer.into_inner();
/// let archived = unsafe { archived_value::<Event>(buf.as_ref(), pos) };
/// if let Archived::<Event>::Speak(message) = archived {
///     assert_eq!(message.as_str(), "Help me!");
/// } else {
///     panic!("archived event was of the wrong type");
/// }
/// ```
#[derive(Debug)]
pub struct BufferSerializer<T> {
    inner: T,
    pos: usize,
}

impl<T> BufferSerializer<T> {
    /// Creates a new archive buffer from a byte buffer.
    #[inline]
    pub fn new(inner: T) -> Self {
        Self::with_pos(inner, 0)
    }

    /// Creates a new archive buffer from a byte buffer. The buffer will start writing at the given
    /// position, but the buffer must contain all bytes (otherwise the alignments of types may not
    /// be correct).
    #[inline]
    pub fn with_pos(inner: T, pos: usize) -> Self {
        Self { inner, pos }
    }

    /// Consumes the serializer and returns the underlying type.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: Default> Default for BufferSerializer<T> {
    #[inline]
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Fallible for BufferSerializer<T> {
    type Error = BufferSerializerError;
}

impl<T: AsMut<[u8]>> Serializer for BufferSerializer<T> {
    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        let end_pos = self.pos + bytes.len();
        let archive_len = self.inner.as_mut().len();
        if end_pos > archive_len {
            Err(BufferSerializerError::Overflow {
                pos: self.pos,
                bytes_needed: bytes.len(),
                archive_len,
            })
        } else {
            unsafe {
                copy_nonoverlapping(
                    bytes.as_ptr(),
                    self.inner.as_mut().as_mut_ptr().add(self.pos),
                    bytes.len(),
                );
            }
            self.pos = end_pos;
            Ok(())
        }
    }
}

/// Errors that can occur when using a fixed-size allocator.
///
/// Pairing a fixed-size allocator with a fallback allocator can help prevent running out of scratch
/// space unexpectedly.
#[derive(Debug)]
pub enum FixedSizeScratchError {
    /// The allocator ran out of scratch space.
    OutOfScratch(Layout),
    /// Scratch space was not popped in reverse order.
    NotPoppedInReverseOrder {
        /// The current position of the start of free memory
        pos: usize,
        /// The next position according to the erroneous pop
        next_pos: usize,
        /// The size of the memory according to the erroneous pop
        next_size: usize,
    },
    /// The given allocation did not belong to the scratch allocator.
    UnownedAllocation,
}

impl fmt::Display for FixedSizeScratchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfScratch(layout) => write!(
                f,
                "out of scratch: requested scratch space with size {} and align {}",
                layout.size(),
                layout.align()
            ),
            Self::NotPoppedInReverseOrder {
                pos,
                next_pos,
                next_size,
            } => write!(
                f,
                "scratch space was not popped in reverse order: pos {}, next pos {}, next size {}",
                pos, next_pos, next_size
            ),
            Self::UnownedAllocation => write!(f, "unowned allocation"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FixedSizeScratchError {}

/// Scratch space that allocates within a buffer.
#[derive(Debug)]
pub struct BufferScratch<T> {
    buffer: T,
    pos: usize,
    // TODO: Compute this pointer eagerly in a future version of rkyv.
    ptr: Option<NonNull<[u8]>>,
}

unsafe impl<T> Send for BufferScratch<T> where T: Send {}
unsafe impl<T> Sync for BufferScratch<T> where T: Sync {}

impl<T> BufferScratch<T> {
    /// Creates a new buffer scratch allocator.
    pub fn new(buffer: T) -> Self {
        Self {
            buffer,
            pos: 0,
            ptr: None,
        }
    }

    /// Resets the scratch space to its initial state.
    pub fn clear(&mut self) {
        self.pos = 0;
    }

    /// Consumes the buffer scratch allocator, returning the underlying buffer.
    pub fn into_inner(self) -> T {
        self.buffer
    }
}

impl<T: Default> Default for BufferScratch<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Fallible for BufferScratch<T> {
    type Error = FixedSizeScratchError;
}

impl<T: DerefMut<Target = U>, U: AsMut<[u8]>> ScratchSpace for BufferScratch<T> {
    #[inline]
    unsafe fn push_scratch(&mut self, layout: Layout) -> Result<NonNull<[u8]>, Self::Error> {
        if self.ptr.is_none() {
            self.ptr = Some(NonNull::from(self.buffer.as_mut()));
        }
        let bytes = self.ptr.unwrap().as_ptr();

        let start = bytes.cast::<u8>().add(self.pos);
        let pad = match (start as usize) & (layout.align() - 1) {
            0 => 0,
            x => layout.align() - x,
        };
        if pad + layout.size() <= ptr_meta::metadata(bytes) - self.pos {
            self.pos += pad;
            let result_slice = ptr_meta::from_raw_parts_mut(
                bytes.cast::<u8>().add(self.pos).cast(),
                layout.size(),
            );
            let result = NonNull::new_unchecked(result_slice);
            self.pos += layout.size();
            Ok(result)
        } else {
            Err(FixedSizeScratchError::OutOfScratch(layout))
        }
    }

    #[inline]
    unsafe fn pop_scratch(&mut self, ptr: NonNull<u8>, layout: Layout) -> Result<(), Self::Error> {
        let bytes = self.ptr.unwrap().as_ptr();

        let ptr = ptr.as_ptr();
        if ptr >= bytes.cast::<u8>() && ptr < bytes.cast::<u8>().add(ptr_meta::metadata(bytes)) {
            let next_pos = ptr.offset_from(bytes.cast::<u8>()) as usize;
            if next_pos + layout.size() <= self.pos {
                self.pos = next_pos;
                Ok(())
            } else {
                Err(FixedSizeScratchError::NotPoppedInReverseOrder {
                    pos: self.pos,
                    next_pos,
                    next_size: layout.size(),
                })
            }
        } else {
            Err(FixedSizeScratchError::UnownedAllocation)
        }
    }
}

/// Allocates scratch space with a main and backup scratch.
#[derive(Debug)]
pub struct FallbackScratch<M, F> {
    main: M,
    fallback: F,
}

impl<M, F> FallbackScratch<M, F> {
    /// Creates fallback scratch from a main and backup scratch.
    pub fn new(main: M, fallback: F) -> Self {
        Self { main, fallback }
    }
}

impl<M: Default, F: Default> Default for FallbackScratch<M, F> {
    fn default() -> Self {
        Self {
            main: M::default(),
            fallback: F::default(),
        }
    }
}

impl<M, F: Fallible> Fallible for FallbackScratch<M, F> {
    type Error = F::Error;
}

impl<M: ScratchSpace, F: ScratchSpace> ScratchSpace for FallbackScratch<M, F> {
    #[inline]
    unsafe fn push_scratch(&mut self, layout: Layout) -> Result<NonNull<[u8]>, Self::Error> {
        self.main
            .push_scratch(layout)
            .or_else(|_| self.fallback.push_scratch(layout))
    }

    #[inline]
    unsafe fn pop_scratch(&mut self, ptr: NonNull<u8>, layout: Layout) -> Result<(), Self::Error> {
        self.main
            .pop_scratch(ptr, layout)
            .or_else(|_| self.fallback.pop_scratch(ptr, layout))
    }
}

/// A passthrough scratch space allocator that tracks scratch space usage.
#[derive(Debug)]
pub struct ScratchTracker<T> {
    inner: T,
    bytes_allocated: usize,
    allocations: usize,
    max_bytes_allocated: usize,
    max_allocations: usize,
    max_alignment: usize,
}

impl<T> ScratchTracker<T> {
    /// Creates a new scratch tracker from the given inner scratch space.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            bytes_allocated: 0,
            allocations: 0,
            max_bytes_allocated: 0,
            max_allocations: 0,
            max_alignment: 1,
        }
    }

    /// Returns the maximum number of bytes that were concurrently allocated during serialization.
    pub fn max_bytes_allocated(&self) -> usize {
        self.max_bytes_allocated
    }

    /// Returns the maximum number of concurrent allocations during serialization.
    pub fn max_allocations(&self) -> usize {
        self.max_allocations
    }

    /// Returns the maximum alignment of scratch space requested during serialization.
    pub fn max_alignment(&self) -> usize {
        self.max_alignment
    }

    /// Returns the minimum buffer size required to serialize the same data.
    ///
    /// This calculation takes into account packing efficiency for slab allocated scratch space. It
    /// is not exact, and has an error bound of `max_allocations * (max_alignment - 1)` bytes. This
    /// should be suitably small for most use cases.
    pub fn min_buffer_size(&self) -> usize {
        self.max_bytes_allocated + self.min_buffer_size_max_error()
    }

    /// Returns the maximum error term for the minimum buffer size calculation.
    pub fn min_buffer_size_max_error(&self) -> usize {
        self.max_allocations * (self.max_alignment - 1)
    }
}

impl<T: Fallible> Fallible for ScratchTracker<T> {
    type Error = T::Error;
}

impl<T: ScratchSpace> ScratchSpace for ScratchTracker<T> {
    #[inline]
    unsafe fn push_scratch(&mut self, layout: Layout) -> Result<NonNull<[u8]>, Self::Error> {
        let result = self.inner.push_scratch(layout)?;

        self.bytes_allocated += layout.size();
        self.allocations += 1;
        self.max_bytes_allocated = usize::max(self.bytes_allocated, self.max_bytes_allocated);
        self.max_allocations = usize::max(self.allocations, self.max_allocations);
        self.max_alignment = usize::max(self.max_alignment, layout.align());

        Ok(result)
    }

    #[inline]
    unsafe fn pop_scratch(&mut self, ptr: NonNull<u8>, layout: Layout) -> Result<(), Self::Error> {
        self.inner.pop_scratch(ptr, layout)?;

        self.bytes_allocated -= layout.size();
        self.allocations -= 1;

        Ok(())
    }
}

impl<T> From<T> for ScratchTracker<T> {
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}
