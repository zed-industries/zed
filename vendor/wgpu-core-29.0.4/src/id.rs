use crate::{Epoch, Index};
use core::{
    cmp::Ordering,
    fmt::{self, Debug},
    hash::Hash,
    marker::PhantomData,
    num::NonZeroU64,
};
use wgt::WasmNotSendSync;

const _: () = {
    if size_of::<Index>() != 4 {
        panic!()
    }
};
const _: () = {
    if size_of::<Epoch>() != 4 {
        panic!()
    }
};
const _: () = {
    if size_of::<RawId>() != 8 {
        panic!()
    }
};

/// The raw underlying representation of an identifier.
#[repr(transparent)]
#[cfg_attr(
    any(feature = "serde", feature = "trace"),
    derive(serde::Serialize),
    serde(into = "SerialId")
)]
#[cfg_attr(
    any(feature = "serde", feature = "replay"),
    derive(serde::Deserialize),
    serde(try_from = "SerialId")
)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawId(NonZeroU64);

impl RawId {
    /// Zip together an identifier and return its raw underlying representation.
    ///
    /// # Panics
    ///
    /// If both ID components are zero.
    pub fn zip(index: Index, epoch: Epoch) -> RawId {
        let v = (index as u64) | ((epoch as u64) << 32);
        Self(NonZeroU64::new(v).expect("IDs may not be zero"))
    }

    /// Unzip a raw identifier into its components.
    pub fn unzip(self) -> (Index, Epoch) {
        (self.0.get() as Index, (self.0.get() >> 32) as Epoch)
    }
}

/// An identifier for a wgpu object.
///
/// An `Id<T>` value identifies a value stored in a [`Global`]'s [`Hub`].
///
/// ## Note on `Id` typing
///
/// You might assume that an `Id<T>` can only be used to retrieve a resource of
/// type `T`, but that is not quite the case. The id types in `wgpu-core`'s
/// public API ([`TextureId`], for example) can refer to resources belonging to
/// any backend, but the corresponding resource types ([`Texture<A>`], for
/// example) are always parameterized by a specific backend `A`.
///
/// So the `T` in `Id<T>` is usually a resource type like `Texture<Noop>`,
/// where [`Noop`] is the `wgpu_hal` dummy back end. These empty types are
/// never actually used, beyond just making sure you access each `Storage` with
/// the right kind of identifier. The members of [`Hub<A>`] pair up each
/// `X<Noop>` type with the resource type `X<A>`, for some specific backend
/// `A`.
///
/// [`Global`]: crate::global::Global
/// [`Hub`]: crate::hub::Hub
/// [`Hub<A>`]: crate::hub::Hub
/// [`Texture<A>`]: crate::resource::Texture
/// [`Registry`]: crate::registry::Registry
/// [`Noop`]: hal::api::Noop
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Id<T: Marker>(RawId, PhantomData<T>);

// This type represents Id in a more readable (and editable) way.
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum SerialId {
    // The only variant forces RON to not ignore "Id"
    Id(Index, Epoch),
}

#[cfg(feature = "serde")]
impl From<RawId> for SerialId {
    fn from(id: RawId) -> Self {
        let (index, epoch) = id.unzip();
        Self::Id(index, epoch)
    }
}

#[cfg(feature = "serde")]
pub struct ZeroIdError;

#[cfg(feature = "serde")]
impl fmt::Display for ZeroIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IDs may not be zero")
    }
}

#[cfg(feature = "serde")]
impl TryFrom<SerialId> for RawId {
    type Error = ZeroIdError;
    fn try_from(id: SerialId) -> Result<Self, ZeroIdError> {
        let SerialId::Id(index, epoch) = id;
        if index == 0 && epoch == 0 {
            Err(ZeroIdError)
        } else {
            Ok(RawId::zip(index, epoch))
        }
    }
}

/// Identify an object by the pointer returned by `Arc::as_ptr`.
///
/// This is used for tracing. See [IDs and tracing](crate::hub#ids-and-tracing).
#[allow(dead_code)]
#[cfg(feature = "serde")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum PointerId<T: Marker> {
    // The only variant forces RON to not ignore "Id"
    PointerId(core::num::NonZeroUsize, #[serde(skip)] PhantomData<T>),
}

#[cfg(feature = "serde")]
impl<T: Marker> Copy for PointerId<T> {}

#[cfg(feature = "serde")]
impl<T: Marker> Clone for PointerId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(feature = "serde")]
impl<T: Marker> PartialEq for PointerId<T> {
    fn eq(&self, other: &Self) -> bool {
        let PointerId::PointerId(this, _) = self;
        let PointerId::PointerId(other, _) = other;
        this == other
    }
}

#[cfg(feature = "serde")]
impl<T: Marker> Eq for PointerId<T> {}

#[cfg(feature = "serde")]
impl<T: Marker> Hash for PointerId<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let PointerId::PointerId(this, _) = self;
        this.hash(state);
    }
}

#[cfg(feature = "serde")]
impl<T: crate::storage::StorageItem> From<&alloc::sync::Arc<T>> for PointerId<T::Marker> {
    fn from(arc: &alloc::sync::Arc<T>) -> Self {
        // Since the memory representation of `Arc<T>` is just a pointer to
        // `ArcInner<T>`, it would be nice to use that pointer as the trace ID,
        // since many `into_trace` implementations would then be no-ops at
        // runtime. However, `Arc::as_ptr` returns a pointer to the contained
        // data, not to the `ArcInner`. The `ArcInner` stores the reference
        // counts before the data, so the machine code for this conversion has
        // to add an offset to the pointer.
        PointerId::PointerId(
            core::num::NonZeroUsize::new(alloc::sync::Arc::as_ptr(arc) as usize).unwrap(),
            PhantomData,
        )
    }
}

impl<T> Id<T>
where
    T: Marker,
{
    /// # Safety
    ///
    /// The raw id must be valid for the type.
    pub unsafe fn from_raw(raw: RawId) -> Self {
        Self(raw, PhantomData)
    }

    /// Coerce the identifiers into its raw underlying representation.
    pub fn into_raw(self) -> RawId {
        self.0
    }

    #[inline]
    pub fn zip(index: Index, epoch: Epoch) -> Self {
        Id(RawId::zip(index, epoch), PhantomData)
    }

    #[inline]
    pub fn unzip(self) -> (Index, Epoch) {
        self.0.unzip()
    }
}

impl<T> Copy for Id<T> where T: Marker {}

impl<T> Clone for Id<T>
where
    T: Marker,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Debug for Id<T>
where
    T: Marker,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let (index, epoch) = self.unzip();
        write!(formatter, "Id({index},{epoch})")?;
        Ok(())
    }
}

impl<T> Hash for Id<T>
where
    T: Marker,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for Id<T>
where
    T: Marker,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> where T: Marker {}

impl<T> PartialOrd for Id<T>
where
    T: Marker,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Id<T>
where
    T: Marker,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

/// Marker trait used to determine which types uniquely identify a resource.
///
/// For example, `Device<A>` will have the same type of identifier as
/// `Device<B>` because `Device<T>` for any `T` defines the same maker type.
pub trait Marker: 'static + WasmNotSendSync {}

// This allows `()` to be used as a marker type for tests.
//
// We don't want these in production code, since they essentially remove type
// safety, like how identifiers across different types can be compared.
#[cfg(test)]
impl Marker for () {}

/// Define identifiers for each resource.
macro_rules! ids {
    ($(
        $(#[$($meta:meta)*])*
        pub type $name:ident $marker:ident;
    )*) => {
        /// Marker types for each resource.
        pub mod markers {
            $(
                #[derive(Debug)]
                pub enum $marker {}
                impl super::Marker for $marker {}
            )*
        }

        $(
            $(#[$($meta)*])*
            pub type $name = Id<self::markers::$marker>;
        )*
    }
}

ids! {
    pub type AdapterId Adapter;
    pub type SurfaceId Surface;
    pub type DeviceId Device;
    pub type QueueId Queue;
    pub type BufferId Buffer;
    pub type StagingBufferId StagingBuffer;
    pub type TextureViewId TextureView;
    pub type TextureId Texture;
    pub type ExternalTextureId ExternalTexture;
    pub type SamplerId Sampler;
    pub type BindGroupLayoutId BindGroupLayout;
    pub type PipelineLayoutId PipelineLayout;
    pub type BindGroupId BindGroup;
    pub type ShaderModuleId ShaderModule;
    pub type RenderPipelineId RenderPipeline;
    pub type ComputePipelineId ComputePipeline;
    pub type PipelineCacheId PipelineCache;
    pub type CommandEncoderId CommandEncoder;
    pub type CommandBufferId CommandBuffer;
    pub type RenderPassEncoderId RenderPassEncoder;
    pub type ComputePassEncoderId ComputePassEncoder;
    pub type RenderBundleEncoderId RenderBundleEncoder;
    pub type RenderBundleId RenderBundle;
    pub type QuerySetId QuerySet;
    pub type BlasId Blas;
    pub type TlasId Tlas;
}

#[test]
fn test_id() {
    let indexes = [0, Index::MAX / 2 - 1, Index::MAX / 2 + 1, Index::MAX];
    let epochs = [1, Epoch::MAX / 2 - 1, Epoch::MAX / 2 + 1, Epoch::MAX];
    for &i in &indexes {
        for &e in &epochs {
            let id = Id::<()>::zip(i, e);
            let (index, epoch) = id.unzip();
            assert_eq!(index, i);
            assert_eq!(epoch, e);
        }
    }
}
