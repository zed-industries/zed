//! Serializers that can be used standalone and provide basic capabilities.

#[cfg(feature = "alloc")]
mod alloc;
mod core;
#[cfg(feature = "std")]
mod std;

#[cfg(feature = "alloc")]
use crate::AlignedVec;
use crate::{
    ser::{ScratchSpace, Serializer, SharedSerializeRegistry},
    AlignedBytes, Archive, ArchiveUnsized, Fallible, Infallible,
};
use ::core::{alloc::Layout, fmt, ptr::NonNull};

#[doc(inline)]
#[cfg(feature = "alloc")]
pub use self::alloc::*;
#[doc(inline)]
pub use self::core::*;
#[doc(inline)]
#[cfg(feature = "std")]
pub use self::std::*;

/// The default serializer error.
#[derive(Debug)]
pub enum CompositeSerializerError<S, C, H> {
    /// An error occurred while serializing
    SerializerError(S),
    /// An error occurred while using scratch space
    ScratchSpaceError(C),
    /// An error occurred while serializing shared memory
    SharedError(H),
}

impl<S, C, H> fmt::Display for CompositeSerializerError<S, C, H>
where
    S: fmt::Display,
    C: fmt::Display,
    H: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SerializerError(e) => write!(f, "serialization error: {}", e),
            Self::ScratchSpaceError(e) => write!(f, "scratch space error: {}", e),
            Self::SharedError(e) => write!(f, "shared memory error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use ::std::error::Error;

    impl<S, C, H> Error for CompositeSerializerError<S, C, H>
    where
        S: Error + 'static,
        C: Error + 'static,
        H: Error + 'static,
    {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                Self::SerializerError(e) => Some(e as &dyn Error),
                Self::ScratchSpaceError(e) => Some(e as &dyn Error),
                Self::SharedError(e) => Some(e as &dyn Error),
            }
        }
    }
};

/// A serializer built from composeable pieces.
#[derive(Debug)]
pub struct CompositeSerializer<S = Infallible, C = Infallible, H = Infallible> {
    serializer: S,
    scratch: C,
    shared: H,
}

impl<S, C, H> CompositeSerializer<S, C, H> {
    /// Creates a new composite serializer from serializer, scratch, and shared components.
    #[inline]
    pub fn new(serializer: S, scratch: C, shared: H) -> Self {
        Self {
            serializer,
            scratch,
            shared,
        }
    }

    /// Consumes the composite serializer and returns the components.
    #[inline]
    pub fn into_components(self) -> (S, C, H) {
        (self.serializer, self.scratch, self.shared)
    }

    /// Consumes the composite serializer and returns the serializer.
    ///
    /// The scratch space and shared component are discarded.
    #[inline]
    pub fn into_serializer(self) -> S {
        self.serializer
    }
}

impl<S: Default, C: Default, H: Default> Default for CompositeSerializer<S, C, H> {
    #[inline]
    fn default() -> Self {
        Self {
            serializer: S::default(),
            scratch: C::default(),
            shared: H::default(),
        }
    }
}

impl<S: Fallible, C: Fallible, H: Fallible> Fallible for CompositeSerializer<S, C, H> {
    type Error = CompositeSerializerError<S::Error, C::Error, H::Error>;
}

impl<S: Serializer, C: Fallible, H: Fallible> Serializer for CompositeSerializer<S, C, H> {
    #[inline]
    fn pos(&self) -> usize {
        self.serializer.pos()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.serializer
            .write(bytes)
            .map_err(CompositeSerializerError::SerializerError)
    }

    #[inline]
    fn pad(&mut self, padding: usize) -> Result<(), Self::Error> {
        self.serializer
            .pad(padding)
            .map_err(CompositeSerializerError::SerializerError)
    }

    #[inline]
    fn align(&mut self, align: usize) -> Result<usize, Self::Error> {
        self.serializer
            .align(align)
            .map_err(CompositeSerializerError::SerializerError)
    }

    #[inline]
    fn align_for<T>(&mut self) -> Result<usize, Self::Error> {
        self.serializer
            .align_for::<T>()
            .map_err(CompositeSerializerError::SerializerError)
    }

    #[inline]
    unsafe fn resolve_aligned<T: Archive + ?Sized>(
        &mut self,
        value: &T,
        resolver: T::Resolver,
    ) -> Result<usize, Self::Error> {
        self.serializer
            .resolve_aligned::<T>(value, resolver)
            .map_err(CompositeSerializerError::SerializerError)
    }

    #[inline]
    unsafe fn resolve_unsized_aligned<T: ArchiveUnsized + ?Sized>(
        &mut self,
        value: &T,
        to: usize,
        metadata_resolver: T::MetadataResolver,
    ) -> Result<usize, Self::Error> {
        self.serializer
            .resolve_unsized_aligned(value, to, metadata_resolver)
            .map_err(CompositeSerializerError::SerializerError)
    }
}

impl<S: Fallible, C: ScratchSpace, H: Fallible> ScratchSpace for CompositeSerializer<S, C, H> {
    #[inline]
    unsafe fn push_scratch(&mut self, layout: Layout) -> Result<NonNull<[u8]>, Self::Error> {
        self.scratch
            .push_scratch(layout)
            .map_err(CompositeSerializerError::ScratchSpaceError)
    }

    #[inline]
    unsafe fn pop_scratch(&mut self, ptr: NonNull<u8>, layout: Layout) -> Result<(), Self::Error> {
        self.scratch
            .pop_scratch(ptr, layout)
            .map_err(CompositeSerializerError::ScratchSpaceError)
    }
}

impl<S: Fallible, C: Fallible, H: SharedSerializeRegistry> SharedSerializeRegistry
    for CompositeSerializer<S, C, H>
{
    #[inline]
    fn get_shared_ptr(&self, value: *const u8) -> Option<usize> {
        self.shared.get_shared_ptr(value)
    }

    #[inline]
    fn add_shared_ptr(&mut self, value: *const u8, pos: usize) -> Result<(), Self::Error> {
        self.shared
            .add_shared_ptr(value, pos)
            .map_err(CompositeSerializerError::SharedError)
    }
}

/// A serializer suitable for environments where allocations cannot be made.
///
/// `CoreSerializer` takes two arguments: the amount of serialization memory to allocate and the
/// amount of scratch space to allocate. If you run out of either while serializing, the serializer
/// will return an error.
pub type CoreSerializer<const S: usize, const C: usize> = CompositeSerializer<
    BufferSerializer<AlignedBytes<S>>,
    BufferScratch<AlignedBytes<C>>,
    Infallible,
>;

/// A general-purpose serializer suitable for environments where allocations can be made.
///
/// `AllocSerializer` takes one argument: the amount of scratch space to allocate before spilling
/// allocations over into heap memory. A large amount of scratch space may result in some of it not
/// being used, but too little scratch space will result in many allocations and decreased
/// performance. You should consider your use case carefully when determining how much scratch space
/// to pre-allocate.
#[cfg(feature = "alloc")]
pub type AllocSerializer<const N: usize> = CompositeSerializer<
    AlignedSerializer<AlignedVec>,
    FallbackScratch<HeapScratch<N>, AllocScratch>,
    SharedSerializeMap,
>;
