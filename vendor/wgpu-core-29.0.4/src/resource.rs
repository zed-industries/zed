use alloc::{borrow::Cow, borrow::ToOwned as _, boxed::Box, string::String, sync::Arc, vec::Vec};
use core::{
    borrow::Borrow,
    fmt,
    mem::{self, size_of, ManuallyDrop},
    num::NonZeroU64,
    ops::Range,
    ptr::NonNull,
};
use smallvec::SmallVec;
use thiserror::Error;
use wgt::{
    error::{ErrorType, WebGpuError},
    TextureSelector,
};

#[cfg(feature = "trace")]
use crate::device::trace;
use crate::{
    binding_model::{BindGroup, BindingError},
    device::{
        queue, resource::DeferredDestroy, BufferMapPendingClosure, Device, DeviceError,
        DeviceMismatch, HostMap, MissingDownlevelFlags, MissingFeatures,
    },
    hal_label,
    init_tracker::{BufferInitTracker, TextureInitTracker},
    lock::{rank, Mutex, RwLock},
    ray_tracing::{BlasCompactReadyPendingClosure, BlasPrepareCompactError},
    resource_log,
    snatch::{SnatchGuard, Snatchable},
    timestamp_normalization::TimestampNormalizationBindGroup,
    track::{SharedTrackerIndexAllocator, TrackerIndex},
    weak_vec::WeakVec,
    Label, LabelHelpers, SubmissionIndex,
};

/// Information about the wgpu-core resource.
///
/// Each type representing a `wgpu-core` resource, like [`Device`],
/// [`Buffer`], etc., contains a `ResourceInfo` which contains
/// its latest submission index and label.
///
/// A resource may need to be retained for any of several reasons:
/// and any lifetime logic will be handled by `Arc<Resource>` refcount
///
/// - The user may hold a reference to it (via a `wgpu::Buffer`, say).
///
/// - Other resources may depend on it (a texture view's backing
///   texture, for example).
///
/// - It may be used by commands sent to the GPU that have not yet
///   finished execution.
///
/// [`Device`]: crate::device::resource::Device
/// [`Buffer`]: crate::resource::Buffer
#[derive(Debug)]
pub(crate) struct TrackingData {
    tracker_index: TrackerIndex,
    tracker_indices: Arc<SharedTrackerIndexAllocator>,
}

impl Drop for TrackingData {
    fn drop(&mut self) {
        self.tracker_indices.free(self.tracker_index);
    }
}

impl TrackingData {
    pub(crate) fn new(tracker_indices: Arc<SharedTrackerIndexAllocator>) -> Self {
        Self {
            tracker_index: tracker_indices.alloc(),
            tracker_indices,
        }
    }

    pub(crate) fn tracker_index(&self) -> TrackerIndex {
        self.tracker_index
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourceErrorIdent {
    r#type: Cow<'static, str>,
    label: String,
}

impl fmt::Display for ResourceErrorIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} with '{}' label", self.r#type, self.label)
    }
}

pub trait ParentDevice: Labeled {
    fn device(&self) -> &Arc<Device>;

    fn is_equal(self: &Arc<Self>, other: &Arc<Self>) -> bool {
        Arc::ptr_eq(self, other)
    }

    fn same_device_as<O: ParentDevice>(&self, other: &O) -> Result<(), DeviceError> {
        if Arc::ptr_eq(self.device(), other.device()) {
            Ok(())
        } else {
            Err(DeviceError::DeviceMismatch(Box::new(DeviceMismatch {
                res: self.error_ident(),
                res_device: self.device().error_ident(),
                target: Some(other.error_ident()),
                target_device: other.device().error_ident(),
            })))
        }
    }

    fn same_device(&self, device: &Device) -> Result<(), DeviceError> {
        if core::ptr::eq(&**self.device(), device) {
            Ok(())
        } else {
            Err(DeviceError::DeviceMismatch(Box::new(DeviceMismatch {
                res: self.error_ident(),
                res_device: self.device().error_ident(),
                target: None,
                target_device: device.error_ident(),
            })))
        }
    }
}

#[macro_export]
macro_rules! impl_parent_device {
    ($ty:ident) => {
        impl $crate::resource::ParentDevice for $ty {
            fn device(&self) -> &Arc<Device> {
                &self.device
            }
        }
    };
}

/// Allow access to the hal resource as guarded by the `SnatchGuard`.
pub trait RawResourceAccess: ParentDevice {
    type DynResource: hal::DynResource + ?Sized;

    /// Get access to the raw resource if it is not destroyed.
    ///
    /// Returns `None` if the resource has been destroyed. This method
    /// does not allocate in either case.
    fn raw<'a>(&'a self, guard: &'a SnatchGuard) -> Option<&'a Self::DynResource>;

    /// Get access to the raw resource if it is not destroyed.
    ///
    /// Returns a full error if the resource has been destroyed. This
    /// method allocates a label in the error case.
    fn try_raw<'a>(
        &'a self,
        guard: &'a SnatchGuard,
    ) -> Result<&'a Self::DynResource, DestroyedResourceError> {
        self.raw(guard)
            .ok_or_else(|| DestroyedResourceError(self.error_ident()))
    }
}

pub trait ResourceType {
    const TYPE: &'static str;
}

#[macro_export]
macro_rules! impl_resource_type {
    ($ty:ident) => {
        impl $crate::resource::ResourceType for $ty {
            const TYPE: &'static str = stringify!($ty);
        }
    };
}

pub trait Labeled: ResourceType {
    /// Returns a string identifying this resource for logging and errors.
    ///
    /// It may be a user-provided string or it may be a placeholder from wgpu.
    ///
    /// It is non-empty unless the user-provided string was empty.
    fn label(&self) -> &str;

    fn error_ident(&self) -> ResourceErrorIdent {
        ResourceErrorIdent {
            r#type: Cow::Borrowed(Self::TYPE),
            label: self.label().to_owned(),
        }
    }
}

#[macro_export]
macro_rules! impl_labeled {
    ($ty:ident) => {
        impl $crate::resource::Labeled for $ty {
            fn label(&self) -> &str {
                &self.label
            }
        }
    };
}

pub(crate) trait Trackable {
    fn tracker_index(&self) -> TrackerIndex;
}

#[macro_export]
macro_rules! impl_trackable {
    ($ty:ident) => {
        impl $crate::resource::Trackable for $ty {
            fn tracker_index(&self) -> $crate::track::TrackerIndex {
                self.tracking_data.tracker_index()
            }
        }
    };
}

#[derive(Debug)]
pub(crate) enum BufferMapState {
    /// Mapped at creation.
    Init { staging_buffer: StagingBuffer },
    /// Waiting for GPU to be done before mapping
    Waiting(BufferPendingMapping),
    /// Mapped
    Active {
        mapping: hal::BufferMapping,
        range: hal::MemoryRange,
        host: HostMap,
    },
    /// Not mapped
    Idle,
}

#[cfg(send_sync)]
unsafe impl Send for BufferMapState {}
#[cfg(send_sync)]
unsafe impl Sync for BufferMapState {}

#[cfg(send_sync)]
pub type BufferMapCallback = Box<dyn FnOnce(BufferAccessResult) + Send + 'static>;
#[cfg(not(send_sync))]
pub type BufferMapCallback = Box<dyn FnOnce(BufferAccessResult) + 'static>;

pub struct BufferMapOperation {
    pub host: HostMap,
    pub callback: Option<BufferMapCallback>,
}

impl fmt::Debug for BufferMapOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufferMapOperation")
            .field("host", &self.host)
            .field("callback", &self.callback.as_ref().map(|_| "?"))
            .finish()
    }
}

#[derive(Clone, Debug, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum BufferAccessError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Buffer map failed")]
    Failed,
    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),
    #[error("Buffer is already mapped")]
    AlreadyMapped,
    #[error("Buffer map is pending")]
    MapAlreadyPending,
    #[error(transparent)]
    MissingBufferUsage(#[from] MissingBufferUsageError),
    #[error("Buffer is not mapped")]
    NotMapped,
    #[error(
        "Buffer map range must start aligned to `MAP_ALIGNMENT` and end to `COPY_BUFFER_ALIGNMENT`"
    )]
    UnalignedRange,
    #[error("Buffer offset invalid: offset {offset} must be multiple of 8")]
    UnalignedOffset { offset: wgt::BufferAddress },
    #[error("Buffer range size invalid: range_size {range_size} must be multiple of 4")]
    UnalignedRangeSize { range_size: wgt::BufferAddress },
    #[error("Buffer access out of bounds: index {index} would underrun the buffer (limit: {min})")]
    OutOfBoundsStartOffsetUnderrun {
        index: wgt::BufferAddress,
        min: wgt::BufferAddress,
    },
    #[error(
        "Buffer access out of bounds: start offset {index} would overrun the buffer (limit: {max})"
    )]
    OutOfBoundsStartOffsetOverrun {
        index: wgt::BufferAddress,
        max: wgt::BufferAddress,
    },
    #[error(
        "Buffer access out of bounds: start offset {index} + size {size} would overrun the buffer (limit: {max})"
    )]
    OutOfBoundsEndOffsetOverrun {
        index: wgt::BufferAddress,
        size: wgt::BufferAddress,
        max: wgt::BufferAddress,
    },
    #[error("Buffer map aborted")]
    MapAborted,
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
    #[error("Map start offset ({offset}) is out-of-bounds for buffer of size {buffer_size}")]
    MapStartOffsetOverrun {
        offset: wgt::BufferAddress,
        buffer_size: wgt::BufferAddress,
    },
    #[error(
        "Map end offset (start at {} + size of {}) is out-of-bounds for buffer of size {}",
        offset,
        size,
        buffer_size
    )]
    MapEndOffsetOverrun {
        offset: wgt::BufferAddress,
        size: wgt::BufferAddress,
        buffer_size: wgt::BufferAddress,
    },
}

impl WebGpuError for BufferAccessError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::DestroyedResource(e) => e.webgpu_error_type(),

            Self::Failed
            | Self::AlreadyMapped
            | Self::MapAlreadyPending
            | Self::MissingBufferUsage(_)
            | Self::NotMapped
            | Self::UnalignedRange
            | Self::UnalignedOffset { .. }
            | Self::UnalignedRangeSize { .. }
            | Self::OutOfBoundsStartOffsetUnderrun { .. }
            | Self::OutOfBoundsStartOffsetOverrun { .. }
            | Self::OutOfBoundsEndOffsetOverrun { .. }
            | Self::MapAborted
            | Self::MapStartOffsetOverrun { .. }
            | Self::MapEndOffsetOverrun { .. } => ErrorType::Validation,
        }
    }
}

#[derive(Clone, Debug, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("Usage flags {actual:?} of {res} do not contain required usage flags {expected:?}")]
pub struct MissingBufferUsageError {
    pub(crate) res: ResourceErrorIdent,
    pub(crate) actual: wgt::BufferUsages,
    pub(crate) expected: wgt::BufferUsages,
}

impl WebGpuError for MissingBufferUsageError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[error("Usage flags {actual:?} of {res} do not contain required usage flags {expected:?}")]
pub struct MissingTextureUsageError {
    pub(crate) res: ResourceErrorIdent,
    pub(crate) actual: wgt::TextureUsages,
    pub(crate) expected: wgt::TextureUsages,
}

impl WebGpuError for MissingTextureUsageError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("{0} has been destroyed")]
pub struct DestroyedResourceError(pub ResourceErrorIdent);

impl WebGpuError for DestroyedResourceError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("{0} is invalid")]
pub struct InvalidResourceError(pub ResourceErrorIdent);

impl WebGpuError for InvalidResourceError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

pub enum Fallible<T: ParentDevice> {
    Valid(Arc<T>),
    Invalid(Arc<String>),
}

impl<T: ParentDevice> Fallible<T> {
    pub fn get(self) -> Result<Arc<T>, InvalidResourceError> {
        match self {
            Fallible::Valid(v) => Ok(v),
            Fallible::Invalid(label) => Err(InvalidResourceError(ResourceErrorIdent {
                r#type: Cow::Borrowed(T::TYPE),
                label: (*label).clone(),
            })),
        }
    }
}

impl<T: ParentDevice> Clone for Fallible<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Valid(v) => Self::Valid(v.clone()),
            Self::Invalid(l) => Self::Invalid(l.clone()),
        }
    }
}

impl<T: ParentDevice> ResourceType for Fallible<T> {
    const TYPE: &'static str = T::TYPE;
}

impl<T: ParentDevice + crate::storage::StorageItem> crate::storage::StorageItem for Fallible<T> {
    type Marker = T::Marker;
}

pub type BufferAccessResult = Result<(), BufferAccessError>;

#[derive(Debug)]
pub(crate) struct BufferPendingMapping {
    pub(crate) range: Range<wgt::BufferAddress>,
    pub(crate) op: BufferMapOperation,
    // hold the parent alive while the mapping is active
    pub(crate) _parent_buffer: Arc<Buffer>,
}

pub type BufferDescriptor<'a> = wgt::BufferDescriptor<Label<'a>>;

#[derive(Debug)]
pub struct Buffer {
    pub(crate) raw: Snatchable<Box<dyn hal::DynBuffer>>,
    pub(crate) device: Arc<Device>,
    pub(crate) usage: wgt::BufferUsages,
    pub(crate) size: wgt::BufferAddress,
    pub(crate) initialization_status: RwLock<BufferInitTracker>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
    pub(crate) map_state: Mutex<BufferMapState>,
    pub(crate) bind_groups: Mutex<WeakVec<BindGroup>>,
    pub(crate) timestamp_normalization_bind_group: Snatchable<TimestampNormalizationBindGroup>,
    pub(crate) indirect_validation_bind_groups: Snatchable<crate::indirect_validation::BindGroups>,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if let Some(raw) = self.timestamp_normalization_bind_group.take() {
            raw.dispose(self.device.raw());
        }

        if let Some(raw) = self.indirect_validation_bind_groups.take() {
            raw.dispose(self.device.raw());
        }

        if let Some(raw) = self.raw.take() {
            resource_log!("Destroy raw {}", self.error_ident());
            unsafe {
                self.device.raw().destroy_buffer(raw);
            }
        }
    }
}

impl RawResourceAccess for Buffer {
    type DynResource = dyn hal::DynBuffer;

    fn raw<'a>(&'a self, guard: &'a SnatchGuard) -> Option<&'a Self::DynResource> {
        self.raw.get(guard).map(|b| b.as_ref())
    }
}

impl Buffer {
    pub(crate) fn check_destroyed(
        &self,
        guard: &SnatchGuard,
    ) -> Result<(), DestroyedResourceError> {
        self.raw
            .get(guard)
            .map(|_| ())
            .ok_or_else(|| DestroyedResourceError(self.error_ident()))
    }

    /// Checks that the given buffer usage contains the required buffer usage,
    /// returns an error otherwise.
    pub(crate) fn check_usage(
        &self,
        expected: wgt::BufferUsages,
    ) -> Result<(), MissingBufferUsageError> {
        if self.usage.contains(expected) {
            Ok(())
        } else {
            Err(MissingBufferUsageError {
                res: self.error_ident(),
                actual: self.usage,
                expected,
            })
        }
    }

    /// Resolve the size of a binding for buffer with `offset` and `size`.
    ///
    /// If `size` is `None`, then the remainder of the buffer starting from
    /// `offset` is used.
    ///
    /// If the binding would overflow the buffer, then an error is returned.
    ///
    /// Zero-size bindings are permitted here for historical reasons. Although
    /// zero-size bindings are permitted by WebGPU, they are not permitted by
    /// some backends. See [`Buffer::binding`] and
    /// [#3170](https://github.com/gfx-rs/wgpu/issues/3170).
    pub fn resolve_binding_size(
        &self,
        offset: wgt::BufferAddress,
        binding_size: Option<wgt::BufferSize>,
    ) -> Result<u64, BindingError> {
        let buffer_size = self.size;

        match binding_size {
            Some(binding_size) => match offset.checked_add(binding_size.get()) {
                Some(end) if end <= buffer_size => Ok(binding_size.get()),
                _ => Err(BindingError::BindingRangeTooLarge {
                    buffer: self.error_ident(),
                    offset,
                    binding_size: binding_size.get(),
                    buffer_size,
                }),
            },
            None => {
                buffer_size
                    .checked_sub(offset)
                    .ok_or_else(|| BindingError::BindingOffsetTooLarge {
                        buffer: self.error_ident(),
                        offset,
                        buffer_size,
                    })
            }
        }
    }

    /// Create a new [`hal::BufferBinding`] for the buffer with `offset` and
    /// `binding_size`.
    ///
    /// If `binding_size` is `None`, then the remainder of the buffer starting
    /// from `offset` is used.
    ///
    /// If the binding would overflow the buffer, then an error is returned.
    ///
    /// A zero-size binding at the end of the buffer is permitted here for historical reasons. Although
    /// zero-size bindings are permitted by WebGPU, they are not permitted by
    /// some backends. The zero-size binding need to be quashed or remapped to a
    /// non-zero size, either universally in wgpu-core, or in specific backends
    /// that do not support them. See
    /// [#3170](https://github.com/gfx-rs/wgpu/issues/3170).
    ///
    /// Although it seems like it would be simpler and safer to use the resolved
    /// size in the returned [`hal::BufferBinding`], doing this (and removing
    /// redundant logic in backends to resolve the implicit size) was observed
    /// to cause problems in certain CTS tests, so an implicit size
    /// specification is preserved in the output.
    pub fn binding<'a>(
        &'a self,
        offset: wgt::BufferAddress,
        binding_size: Option<wgt::BufferSize>,
        snatch_guard: &'a SnatchGuard,
    ) -> Result<(hal::BufferBinding<'a, dyn hal::DynBuffer>, u64), BindingError> {
        let buf_raw = self.try_raw(snatch_guard)?;
        let resolved_size = self.resolve_binding_size(offset, binding_size)?;
        // SAFETY: The offset and size passed to hal::BufferBinding::new_unchecked must
        // define a binding contained within the buffer.
        Ok((
            hal::BufferBinding::new_unchecked(buf_raw, offset, binding_size),
            resolved_size,
        ))
    }

    /// Returns the mapping callback in case of error so that the callback can be fired outside
    /// of the locks that are held in this function.
    pub fn map_async(
        self: &Arc<Self>,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferAddress>,
        op: BufferMapOperation,
    ) -> Result<SubmissionIndex, (BufferMapOperation, BufferAccessError)> {
        let range_size = if let Some(size) = size {
            size
        } else {
            self.size.saturating_sub(offset)
        };

        if !offset.is_multiple_of(wgt::MAP_ALIGNMENT) {
            return Err((op, BufferAccessError::UnalignedOffset { offset }));
        }
        if !range_size.is_multiple_of(wgt::COPY_BUFFER_ALIGNMENT) {
            return Err((op, BufferAccessError::UnalignedRangeSize { range_size }));
        }

        if offset > self.size {
            return Err((
                op,
                BufferAccessError::MapStartOffsetOverrun {
                    offset,
                    buffer_size: self.size,
                },
            ));
        }
        // NOTE: Should never underflow because of our earlier check.
        if range_size > self.size - offset {
            return Err((
                op,
                BufferAccessError::MapEndOffsetOverrun {
                    offset,
                    size: range_size,
                    buffer_size: self.size,
                },
            ));
        }
        let end_offset = offset + range_size;

        if !offset.is_multiple_of(wgt::MAP_ALIGNMENT)
            || !end_offset.is_multiple_of(wgt::COPY_BUFFER_ALIGNMENT)
        {
            return Err((op, BufferAccessError::UnalignedRange));
        }

        let (pub_usage, internal_use) = match op.host {
            HostMap::Read => (wgt::BufferUsages::MAP_READ, wgt::BufferUses::MAP_READ),
            HostMap::Write => (wgt::BufferUsages::MAP_WRITE, wgt::BufferUses::MAP_WRITE),
        };

        if let Err(e) = self.check_usage(pub_usage) {
            return Err((op, e.into()));
        }

        let device = &self.device;
        if let Err(e) = device.check_is_valid() {
            return Err((op, e.into()));
        }

        {
            let snatch_guard = device.snatchable_lock.read();
            if let Err(e) = self.check_destroyed(&snatch_guard) {
                return Err((op, e.into()));
            }
        }

        {
            let map_state = &mut *self.map_state.lock();
            *map_state = match *map_state {
                BufferMapState::Init { .. } | BufferMapState::Active { .. } => {
                    return Err((op, BufferAccessError::AlreadyMapped));
                }
                BufferMapState::Waiting(_) => {
                    return Err((op, BufferAccessError::MapAlreadyPending));
                }
                BufferMapState::Idle => BufferMapState::Waiting(BufferPendingMapping {
                    range: offset..end_offset,
                    op,
                    _parent_buffer: self.clone(),
                }),
            };
        }

        // TODO: we are ignoring the transition here, I think we need to add a barrier
        // at the end of the submission
        device
            .trackers
            .lock()
            .buffers
            .set_single(self, internal_use);

        let submit_index = if let Some(queue) = device.get_queue() {
            queue.lock_life().map(self).unwrap_or(0) // '0' means no wait is necessary
        } else {
            // We can safely unwrap below since we just set the `map_state` to `BufferMapState::Waiting`.
            let (mut operation, status) = self.map(&device.snatchable_lock.read()).unwrap();
            if let Some(callback) = operation.callback.take() {
                callback(status);
            }
            0
        };

        Ok(submit_index)
    }

    pub fn get_mapped_range(
        self: &Arc<Self>,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferAddress>,
    ) -> Result<(NonNull<u8>, u64), BufferAccessError> {
        {
            let snatch_guard = self.device.snatchable_lock.read();
            self.check_destroyed(&snatch_guard)?;
        }

        let range_size = if let Some(size) = size {
            size
        } else {
            self.size.saturating_sub(offset)
        };

        if !offset.is_multiple_of(wgt::MAP_ALIGNMENT) {
            return Err(BufferAccessError::UnalignedOffset { offset });
        }
        if !range_size.is_multiple_of(wgt::COPY_BUFFER_ALIGNMENT) {
            return Err(BufferAccessError::UnalignedRangeSize { range_size });
        }
        let map_state = &*self.map_state.lock();
        match *map_state {
            BufferMapState::Init { ref staging_buffer } => {
                if offset > self.size {
                    return Err(BufferAccessError::MapStartOffsetOverrun {
                        offset,
                        buffer_size: self.size,
                    });
                }
                // NOTE: Should never underflow because of our earlier check.
                if range_size > self.size - offset {
                    return Err(BufferAccessError::MapEndOffsetOverrun {
                        offset,
                        size: range_size,
                        buffer_size: self.size,
                    });
                }
                let ptr = unsafe { staging_buffer.ptr() };
                let ptr = unsafe { NonNull::new_unchecked(ptr.as_ptr().offset(offset as isize)) };
                Ok((ptr, range_size))
            }
            BufferMapState::Active {
                ref mapping,
                ref range,
                ..
            } => {
                if offset > range.end {
                    return Err(BufferAccessError::OutOfBoundsStartOffsetOverrun {
                        index: offset,
                        max: range.end,
                    });
                }
                if offset < range.start {
                    return Err(BufferAccessError::OutOfBoundsStartOffsetUnderrun {
                        index: offset,
                        min: range.start,
                    });
                }
                if range_size > range.end - offset {
                    return Err(BufferAccessError::OutOfBoundsEndOffsetOverrun {
                        index: offset,
                        size: range_size,
                        max: range.end,
                    });
                }
                // ptr points to the beginning of the range we mapped in map_async
                // rather than the beginning of the buffer.
                let relative_offset = (offset - range.start) as isize;
                unsafe {
                    Ok((
                        NonNull::new_unchecked(mapping.ptr.as_ptr().offset(relative_offset)),
                        range_size,
                    ))
                }
            }
            BufferMapState::Idle | BufferMapState::Waiting(_) => Err(BufferAccessError::NotMapped),
        }
    }
    /// This function returns [`None`] only if [`Self::map_state`] is not [`BufferMapState::Waiting`].
    #[must_use]
    pub(crate) fn map(&self, snatch_guard: &SnatchGuard) -> Option<BufferMapPendingClosure> {
        // This _cannot_ be inlined into the match. If it is, the lock will be held
        // open through the whole match, resulting in a deadlock when we try to re-lock
        // the buffer back to active.
        let mapping = mem::replace(&mut *self.map_state.lock(), BufferMapState::Idle);
        let pending_mapping = match mapping {
            BufferMapState::Waiting(pending_mapping) => pending_mapping,
            // Mapping cancelled
            BufferMapState::Idle => return None,
            // Mapping queued at least twice by map -> unmap -> map
            // and was already successfully mapped below
            BufferMapState::Active { .. } => {
                *self.map_state.lock() = mapping;
                return None;
            }
            _ => panic!("No pending mapping."),
        };
        let status = if pending_mapping.range.start != pending_mapping.range.end {
            let host = pending_mapping.op.host;
            let size = pending_mapping.range.end - pending_mapping.range.start;
            match crate::device::map_buffer(
                self,
                pending_mapping.range.start,
                size,
                host,
                snatch_guard,
            ) {
                Ok(mapping) => {
                    *self.map_state.lock() = BufferMapState::Active {
                        mapping,
                        range: pending_mapping.range.clone(),
                        host,
                    };
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            *self.map_state.lock() = BufferMapState::Active {
                mapping: hal::BufferMapping {
                    ptr: NonNull::dangling(),
                    is_coherent: true,
                },
                range: pending_mapping.range,
                host: pending_mapping.op.host,
            };
            Ok(())
        };
        Some((pending_mapping.op, status))
    }

    // Note: This must not be called while holding a lock.
    pub fn unmap(self: &Arc<Self>) -> Result<(), BufferAccessError> {
        if let Some((mut operation, status)) = self.unmap_inner()? {
            if let Some(callback) = operation.callback.take() {
                callback(status);
            }
        }

        Ok(())
    }

    fn unmap_inner(self: &Arc<Self>) -> Result<Option<BufferMapPendingClosure>, BufferAccessError> {
        let device = &self.device;
        let snatch_guard = device.snatchable_lock.read();
        let raw_buf = self.try_raw(&snatch_guard)?;
        match mem::replace(&mut *self.map_state.lock(), BufferMapState::Idle) {
            BufferMapState::Init { staging_buffer } => {
                #[cfg(feature = "trace")]
                if let Some(ref mut trace) = *device.trace.lock() {
                    use crate::device::trace::{DataKind, IntoTrace};

                    let data = trace.make_binary(DataKind::Bin, staging_buffer.get_data());
                    trace.add(trace::Action::WriteBuffer {
                        id: self.to_trace(),
                        data,
                        // NOTE: `self.size` here corresponds to `data`'s actual length.
                        offset: 0,
                        size: self.size,
                        queued: true,
                    });
                }

                let staging_buffer = staging_buffer.flush();

                if let Some(queue) = device.get_queue() {
                    let region = wgt::BufferSize::new(self.size).map(|size| hal::BufferCopy {
                        src_offset: 0,
                        dst_offset: 0,
                        size,
                    });
                    let transition_src = hal::BufferBarrier {
                        buffer: staging_buffer.raw(),
                        usage: hal::StateTransition {
                            from: wgt::BufferUses::MAP_WRITE,
                            to: wgt::BufferUses::COPY_SRC,
                        },
                    };
                    let transition_dst = hal::BufferBarrier::<dyn hal::DynBuffer> {
                        buffer: raw_buf,
                        usage: hal::StateTransition {
                            from: wgt::BufferUses::empty(),
                            to: wgt::BufferUses::COPY_DST,
                        },
                    };
                    let mut pending_writes = queue.pending_writes.lock();
                    let encoder = pending_writes.activate();
                    unsafe {
                        encoder.transition_buffers(&[transition_src, transition_dst]);
                        if self.size > 0 {
                            encoder.copy_buffer_to_buffer(
                                staging_buffer.raw(),
                                raw_buf,
                                region.as_slice(),
                            );
                        }
                    }
                    pending_writes.consume(staging_buffer);
                    pending_writes.insert_buffer(self);
                }
            }
            BufferMapState::Idle => {
                return Err(BufferAccessError::NotMapped);
            }
            BufferMapState::Waiting(pending) => {
                return Ok(Some((pending.op, Err(BufferAccessError::MapAborted))));
            }
            BufferMapState::Active {
                mapping,
                range,
                host,
            } => {
                if host == HostMap::Write {
                    #[cfg(feature = "trace")]
                    if let Some(ref mut trace) = *device.trace.lock() {
                        use crate::device::trace::{DataKind, IntoTrace};

                        let size = range.end - range.start;
                        let data = trace.make_binary(DataKind::Bin, unsafe {
                            core::slice::from_raw_parts(mapping.ptr.as_ptr(), size as usize)
                        });
                        trace.add(trace::Action::WriteBuffer {
                            id: self.to_trace(),
                            data,
                            offset: range.start,
                            size,
                            queued: false,
                        });
                    }
                    if !mapping.is_coherent {
                        unsafe { device.raw().flush_mapped_ranges(raw_buf, &[range]) };
                    }
                }
                unsafe { device.raw().unmap_buffer(raw_buf) };
            }
        }
        Ok(None)
    }

    pub fn destroy(self: &Arc<Self>) {
        let device = &self.device;

        let temp = {
            let mut snatch_guard = device.snatchable_lock.write();

            let raw = match self.raw.snatch(&mut snatch_guard) {
                Some(raw) => raw,
                None => {
                    // Per spec, it is valid to call `destroy` multiple times.
                    return;
                }
            };

            let timestamp_normalization_bind_group = self
                .timestamp_normalization_bind_group
                .snatch(&mut snatch_guard);

            let indirect_validation_bind_groups = self
                .indirect_validation_bind_groups
                .snatch(&mut snatch_guard);

            drop(snatch_guard);

            let bind_groups = {
                let mut guard = self.bind_groups.lock();
                mem::take(&mut *guard)
            };

            queue::TempResource::DestroyedBuffer(DestroyedBuffer {
                raw: ManuallyDrop::new(raw),
                device: Arc::clone(&self.device),
                label: self.label().to_owned(),
                bind_groups,
                timestamp_normalization_bind_group,
                indirect_validation_bind_groups,
            })
        };

        if let Some(queue) = device.get_queue() {
            let mut pending_writes = queue.pending_writes.lock();
            if pending_writes.contains_buffer(self) {
                pending_writes.consume_temp(temp);
            } else {
                let mut life_lock = queue.lock_life();
                let last_submit_index = life_lock.get_buffer_latest_submission_index(self);
                if let Some(last_submit_index) = last_submit_index {
                    life_lock.schedule_resource_destruction(temp, last_submit_index);
                }
            }
        }
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateBufferError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Failed to map buffer while creating: {0}")]
    AccessError(#[from] BufferAccessError),
    #[error("Buffers that are mapped at creation have to be aligned to `COPY_BUFFER_ALIGNMENT`")]
    UnalignedSize,
    #[error("Invalid usage flags {0:?}")]
    InvalidUsage(wgt::BufferUsages),
    #[error("`MAP` usage can only be combined with the opposite `COPY`, requested {0:?}")]
    UsageMismatch(wgt::BufferUsages),
    #[error("Buffer size {requested} is greater than the maximum buffer size ({maximum})")]
    MaxBufferSize { requested: u64, maximum: u64 },
    #[error(transparent)]
    MissingDownlevelFlags(#[from] MissingDownlevelFlags),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error("Failed to create bind group for indirect buffer validation: {0}")]
    IndirectValidationBindGroup(DeviceError),
}

crate::impl_resource_type!(Buffer);
crate::impl_labeled!(Buffer);
crate::impl_parent_device!(Buffer);
crate::impl_storage_item!(Buffer);
crate::impl_trackable!(Buffer);

impl WebGpuError for CreateBufferError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::AccessError(e) => e.webgpu_error_type(),
            Self::MissingDownlevelFlags(e) => e.webgpu_error_type(),
            Self::IndirectValidationBindGroup(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),

            Self::UnalignedSize
            | Self::InvalidUsage(_)
            | Self::UsageMismatch(_)
            | Self::MaxBufferSize { .. } => ErrorType::Validation,
        }
    }
}

/// A buffer that has been marked as destroyed and is staged for actual deletion soon.
#[derive(Debug)]
pub struct DestroyedBuffer {
    raw: ManuallyDrop<Box<dyn hal::DynBuffer>>,
    device: Arc<Device>,
    label: String,
    bind_groups: WeakVec<BindGroup>,
    timestamp_normalization_bind_group: Option<TimestampNormalizationBindGroup>,
    indirect_validation_bind_groups: Option<crate::indirect_validation::BindGroups>,
}

impl DestroyedBuffer {
    pub fn label(&self) -> &dyn fmt::Debug {
        &self.label
    }
}

impl Drop for DestroyedBuffer {
    fn drop(&mut self) {
        let mut deferred = self.device.deferred_destroy.lock();
        deferred.push(DeferredDestroy::BindGroups(mem::take(
            &mut self.bind_groups,
        )));
        drop(deferred);

        if let Some(raw) = self.timestamp_normalization_bind_group.take() {
            raw.dispose(self.device.raw());
        }

        if let Some(raw) = self.indirect_validation_bind_groups.take() {
            raw.dispose(self.device.raw());
        }

        resource_log!("Destroy raw Buffer (destroyed) {:?}", self.label());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            hal::DynDevice::destroy_buffer(self.device.raw(), raw);
        }
    }
}

#[cfg(send_sync)]
unsafe impl Send for StagingBuffer {}
#[cfg(send_sync)]
unsafe impl Sync for StagingBuffer {}

/// A temporary buffer, consumed by the command that uses it.
///
/// A [`StagingBuffer`] is designed for one-shot uploads of data to the GPU. It
/// is always created mapped, and the command that uses it destroys the buffer
/// when it is done.
///
/// [`StagingBuffer`]s can be created with [`queue_create_staging_buffer`] and
/// used with [`queue_write_staging_buffer`]. They are also used internally by
/// operations like [`queue_write_texture`] that need to upload data to the GPU,
/// but that don't belong to any particular wgpu command buffer.
///
/// Used `StagingBuffer`s are accumulated in [`Device::pending_writes`], to be
/// freed once their associated operation's queue submission has finished
/// execution.
///
/// [`queue_create_staging_buffer`]: crate::global::Global::queue_create_staging_buffer
/// [`queue_write_staging_buffer`]: crate::global::Global::queue_write_staging_buffer
/// [`queue_write_texture`]: crate::global::Global::queue_write_texture
/// [`Device::pending_writes`]: crate::device::Device
#[derive(Debug)]
pub struct StagingBuffer {
    raw: Box<dyn hal::DynBuffer>,
    device: Arc<Device>,
    pub(crate) size: wgt::BufferSize,
    is_coherent: bool,
    ptr: NonNull<u8>,
}

impl StagingBuffer {
    pub(crate) fn new(device: &Arc<Device>, size: wgt::BufferSize) -> Result<Self, DeviceError> {
        profiling::scope!("StagingBuffer::new");
        let stage_desc = hal::BufferDescriptor {
            label: hal_label(Some("(wgpu internal) Staging"), device.instance_flags),
            size: size.get(),
            usage: wgt::BufferUses::MAP_WRITE | wgt::BufferUses::COPY_SRC,
            memory_flags: hal::MemoryFlags::TRANSIENT,
        };

        let raw = unsafe { device.raw().create_buffer(&stage_desc) }
            .map_err(|e| device.handle_hal_error(e))?;
        let mapping = unsafe { device.raw().map_buffer(raw.as_ref(), 0..size.get()) }
            .map_err(|e| device.handle_hal_error(e))?;

        let staging_buffer = StagingBuffer {
            raw,
            device: device.clone(),
            size,
            is_coherent: mapping.is_coherent,
            ptr: mapping.ptr,
        };

        Ok(staging_buffer)
    }

    /// SAFETY: You must not call any functions of `self`
    /// until you stopped using the returned pointer.
    pub(crate) unsafe fn ptr(&self) -> NonNull<u8> {
        self.ptr
    }

    #[cfg(feature = "trace")]
    pub(crate) fn get_data(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.size.get() as usize) }
    }

    pub(crate) fn write_zeros(&mut self) {
        unsafe { core::ptr::write_bytes(self.ptr.as_ptr(), 0, self.size.get() as usize) };
    }

    pub(crate) fn write(&mut self, data: &[u8]) {
        assert!(data.len() >= self.size.get() as usize);
        // SAFETY: With the assert above, all of `copy_nonoverlapping`'s
        // requirements are satisfied.
        unsafe {
            core::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.ptr.as_ptr(),
                self.size.get() as usize,
            );
        }
    }

    /// SAFETY: The offsets and size must be in-bounds.
    pub(crate) unsafe fn write_with_offset(
        &mut self,
        data: &[u8],
        src_offset: isize,
        dst_offset: isize,
        size: usize,
    ) {
        unsafe {
            debug_assert!(
                (src_offset + size as isize) as usize <= data.len(),
                "src_offset + size must be in-bounds: src_offset = {}, size = {}, data.len() = {}",
                src_offset,
                size,
                data.len()
            );
            core::ptr::copy_nonoverlapping(
                data.as_ptr().offset(src_offset),
                self.ptr.as_ptr().offset(dst_offset),
                size,
            );
        }
    }

    pub(crate) fn flush(self) -> FlushedStagingBuffer {
        let device = self.device.raw();
        if !self.is_coherent {
            #[allow(clippy::single_range_in_vec_init)]
            unsafe {
                device.flush_mapped_ranges(self.raw.as_ref(), &[0..self.size.get()])
            };
        }
        unsafe { device.unmap_buffer(self.raw.as_ref()) };

        let StagingBuffer {
            raw, device, size, ..
        } = self;

        FlushedStagingBuffer {
            raw: ManuallyDrop::new(raw),
            device,
            size,
        }
    }
}

crate::impl_resource_type!(StagingBuffer);
crate::impl_storage_item!(StagingBuffer);

#[derive(Debug)]
pub struct FlushedStagingBuffer {
    raw: ManuallyDrop<Box<dyn hal::DynBuffer>>,
    device: Arc<Device>,
    pub(crate) size: wgt::BufferSize,
}

impl FlushedStagingBuffer {
    pub(crate) fn raw(&self) -> &dyn hal::DynBuffer {
        self.raw.as_ref()
    }
}

impl Drop for FlushedStagingBuffer {
    fn drop(&mut self) {
        resource_log!("Destroy raw StagingBuffer");
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe { self.device.raw().destroy_buffer(raw) };
    }
}

pub type TextureDescriptor<'a> = wgt::TextureDescriptor<Label<'a>, Vec<wgt::TextureFormat>>;

#[derive(Debug)]
pub(crate) enum TextureInner {
    Native {
        raw: Box<dyn hal::DynTexture>,
    },
    Surface {
        raw: Box<dyn hal::DynSurfaceTexture>,
    },
}

impl TextureInner {
    pub(crate) fn raw(&self) -> &dyn hal::DynTexture {
        match self {
            Self::Native { raw } => raw.as_ref(),
            Self::Surface { raw, .. } => raw.as_ref().borrow(),
        }
    }
}

#[derive(Debug)]
pub enum TextureClearMode {
    BufferCopy,
    // View for clear via RenderPass for every subsurface (mip/layer/slice)
    RenderPass {
        clear_views: SmallVec<[ManuallyDrop<Box<dyn hal::DynTextureView>>; 1]>,
        is_color: bool,
    },
    Surface {
        clear_view: ManuallyDrop<Box<dyn hal::DynTextureView>>,
    },
    // Texture can't be cleared, attempting to do so will cause panic.
    // (either because it is impossible for the type of texture or it is being destroyed)
    None,
}

#[derive(Debug)]
pub struct Texture {
    pub(crate) inner: Snatchable<TextureInner>,
    pub(crate) device: Arc<Device>,
    pub(crate) desc: wgt::TextureDescriptor<(), Vec<wgt::TextureFormat>>,
    pub(crate) _hal_usage: wgt::TextureUses,
    pub(crate) format_features: wgt::TextureFormatFeatures,
    pub(crate) initialization_status: RwLock<TextureInitTracker>,
    pub(crate) full_range: TextureSelector,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
    pub(crate) clear_mode: RwLock<TextureClearMode>,
    pub(crate) views: Mutex<WeakVec<TextureView>>,
    pub(crate) bind_groups: Mutex<WeakVec<BindGroup>>,
}

impl Texture {
    pub(crate) fn new(
        device: &Arc<Device>,
        inner: TextureInner,
        hal_usage: wgt::TextureUses,
        desc: &TextureDescriptor,
        format_features: wgt::TextureFormatFeatures,
        clear_mode: TextureClearMode,
        init: bool,
    ) -> Self {
        Texture {
            inner: Snatchable::new(inner),
            device: device.clone(),
            desc: desc.map_label(|_| ()),
            _hal_usage: hal_usage,
            format_features,
            initialization_status: RwLock::new(
                rank::TEXTURE_INITIALIZATION_STATUS,
                if init {
                    TextureInitTracker::new(desc.mip_level_count, desc.array_layer_count())
                } else {
                    TextureInitTracker::new(desc.mip_level_count, 0)
                },
            ),
            full_range: TextureSelector {
                mips: 0..desc.mip_level_count,
                layers: 0..desc.array_layer_count(),
            },
            label: desc.label.to_string(),
            tracking_data: TrackingData::new(device.tracker_indices.textures.clone()),
            clear_mode: RwLock::new(rank::TEXTURE_CLEAR_MODE, clear_mode),
            views: Mutex::new(rank::TEXTURE_VIEWS, WeakVec::new()),
            bind_groups: Mutex::new(rank::TEXTURE_BIND_GROUPS, WeakVec::new()),
        }
    }

    /// Checks that the given texture usage contains the required texture usage,
    /// returns an error otherwise.
    pub(crate) fn check_usage(
        &self,
        expected: wgt::TextureUsages,
    ) -> Result<(), MissingTextureUsageError> {
        if self.desc.usage.contains(expected) {
            Ok(())
        } else {
            Err(MissingTextureUsageError {
                res: self.error_ident(),
                actual: self.desc.usage,
                expected,
            })
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        match *self.clear_mode.write() {
            TextureClearMode::Surface {
                ref mut clear_view, ..
            } => {
                // SAFETY: We are in the Drop impl and we don't use clear_view anymore after this point.
                let raw = unsafe { ManuallyDrop::take(clear_view) };
                unsafe {
                    self.device.raw().destroy_texture_view(raw);
                }
            }
            TextureClearMode::RenderPass {
                ref mut clear_views,
                ..
            } => {
                clear_views.iter_mut().for_each(|clear_view| {
                    // SAFETY: We are in the Drop impl and we don't use clear_view anymore after this point.
                    let raw = unsafe { ManuallyDrop::take(clear_view) };
                    unsafe {
                        self.device.raw().destroy_texture_view(raw);
                    }
                });
            }
            _ => {}
        };

        if let Some(TextureInner::Native { raw }) = self.inner.take() {
            resource_log!("Destroy raw {}", self.error_ident());
            unsafe {
                self.device.raw().destroy_texture(raw);
            }
        }
    }
}

impl RawResourceAccess for Texture {
    type DynResource = dyn hal::DynTexture;

    fn raw<'a>(&'a self, guard: &'a SnatchGuard) -> Option<&'a Self::DynResource> {
        self.inner.get(guard).map(|t| t.raw())
    }
}

impl Texture {
    pub(crate) fn try_inner<'a>(
        &'a self,
        guard: &'a SnatchGuard,
    ) -> Result<&'a TextureInner, DestroyedResourceError> {
        self.inner
            .get(guard)
            .ok_or_else(|| DestroyedResourceError(self.error_ident()))
    }

    pub(crate) fn check_destroyed(
        &self,
        guard: &SnatchGuard,
    ) -> Result<(), DestroyedResourceError> {
        self.inner
            .get(guard)
            .map(|_| ())
            .ok_or_else(|| DestroyedResourceError(self.error_ident()))
    }

    pub(crate) fn get_clear_view<'a>(
        clear_mode: &'a TextureClearMode,
        desc: &'a wgt::TextureDescriptor<(), Vec<wgt::TextureFormat>>,
        mip_level: u32,
        depth_or_layer: u32,
    ) -> &'a dyn hal::DynTextureView {
        match *clear_mode {
            TextureClearMode::BufferCopy => {
                panic!("Given texture is cleared with buffer copies, not render passes")
            }
            TextureClearMode::None => {
                panic!("Given texture can't be cleared")
            }
            TextureClearMode::Surface { ref clear_view, .. } => clear_view.as_ref(),
            TextureClearMode::RenderPass {
                ref clear_views, ..
            } => {
                let index = if desc.dimension == wgt::TextureDimension::D3 {
                    (0..mip_level).fold(0, |acc, mip| {
                        acc + (desc.size.depth_or_array_layers >> mip).max(1)
                    })
                } else {
                    mip_level * desc.size.depth_or_array_layers
                } + depth_or_layer;
                clear_views[index as usize].as_ref()
            }
        }
    }

    pub fn destroy(self: &Arc<Self>) {
        let device = &self.device;

        let temp = {
            let raw = match self.inner.snatch(&mut device.snatchable_lock.write()) {
                Some(TextureInner::Native { raw }) => raw,
                Some(TextureInner::Surface { .. }) => {
                    return;
                }
                None => {
                    // Per spec, it is valid to call `destroy` multiple times.
                    return;
                }
            };

            let views = {
                let mut guard = self.views.lock();
                mem::take(&mut *guard)
            };

            let bind_groups = {
                let mut guard = self.bind_groups.lock();
                mem::take(&mut *guard)
            };

            queue::TempResource::DestroyedTexture(DestroyedTexture {
                raw: ManuallyDrop::new(raw),
                views,
                clear_mode: mem::replace(&mut *self.clear_mode.write(), TextureClearMode::None),
                bind_groups,
                device: Arc::clone(&self.device),
                label: self.label().to_owned(),
            })
        };

        if let Some(queue) = device.get_queue() {
            let mut pending_writes = queue.pending_writes.lock();
            if pending_writes.contains_texture(self) {
                pending_writes.consume_temp(temp);
            } else {
                let mut life_lock = queue.lock_life();
                let last_submit_index = life_lock.get_texture_latest_submission_index(self);
                if let Some(last_submit_index) = last_submit_index {
                    life_lock.schedule_resource_destruction(temp, last_submit_index);
                }
            }
        }
    }
}

/// A texture that has been marked as destroyed and is staged for actual deletion soon.
#[derive(Debug)]
pub struct DestroyedTexture {
    raw: ManuallyDrop<Box<dyn hal::DynTexture>>,
    views: WeakVec<TextureView>,
    clear_mode: TextureClearMode,
    bind_groups: WeakVec<BindGroup>,
    device: Arc<Device>,
    label: String,
}

impl DestroyedTexture {
    pub fn label(&self) -> &dyn fmt::Debug {
        &self.label
    }
}

impl Drop for DestroyedTexture {
    fn drop(&mut self) {
        let device = &self.device;

        let mut deferred = device.deferred_destroy.lock();
        deferred.push(DeferredDestroy::TextureViews(mem::take(&mut self.views)));
        deferred.push(DeferredDestroy::BindGroups(mem::take(
            &mut self.bind_groups,
        )));
        drop(deferred);

        match mem::replace(&mut self.clear_mode, TextureClearMode::None) {
            TextureClearMode::RenderPass { clear_views, .. } => {
                for clear_view in clear_views {
                    let raw = ManuallyDrop::into_inner(clear_view);
                    unsafe { self.device.raw().destroy_texture_view(raw) };
                }
            }
            TextureClearMode::Surface { clear_view } => {
                let raw = ManuallyDrop::into_inner(clear_view);
                unsafe { self.device.raw().destroy_texture_view(raw) };
            }
            _ => (),
        }

        resource_log!("Destroy raw Texture (destroyed) {:?}", self.label());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            self.device.raw().destroy_texture(raw);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TextureErrorDimension {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum TextureDimensionError {
    #[error("Dimension {0:?} is zero")]
    Zero(TextureErrorDimension),
    #[error("Dimension {dim:?} value {given} exceeds the limit of {limit}")]
    LimitExceeded {
        dim: TextureErrorDimension,
        given: u32,
        limit: u32,
    },
    #[error("Sample count {0} is invalid")]
    InvalidSampleCount(u32),
    #[error("Width {width} is not a multiple of {format:?}'s block width ({block_width})")]
    NotMultipleOfBlockWidth {
        width: u32,
        block_width: u32,
        format: wgt::TextureFormat,
    },
    #[error("Height {height} is not a multiple of {format:?}'s block height ({block_height})")]
    NotMultipleOfBlockHeight {
        height: u32,
        block_height: u32,
        format: wgt::TextureFormat,
    },
    #[error(
        "Width {width} is not a multiple of {format:?}'s width multiple requirement ({multiple})"
    )]
    WidthNotMultipleOf {
        width: u32,
        multiple: u32,
        format: wgt::TextureFormat,
    },
    #[error("Height {height} is not a multiple of {format:?}'s height multiple requirement ({multiple})")]
    HeightNotMultipleOf {
        height: u32,
        multiple: u32,
        format: wgt::TextureFormat,
    },
    #[error("Multisampled texture depth or array layers must be 1, got {0}")]
    MultisampledDepthOrArrayLayer(u32),
}

impl WebGpuError for TextureDimensionError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateTextureError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    CreateTextureView(#[from] CreateTextureViewError),
    #[error("Invalid usage flags {0:?}")]
    InvalidUsage(wgt::TextureUsages),
    #[error("Texture usage {0:?} is not compatible with texture usage {1:?}")]
    IncompatibleUsage(wgt::TextureUsages, wgt::TextureUsages),
    #[error(transparent)]
    InvalidDimension(#[from] TextureDimensionError),
    #[error("Depth texture ({1:?}) can't be created as {0:?}")]
    InvalidDepthDimension(wgt::TextureDimension, wgt::TextureFormat),
    #[error("Compressed texture ({1:?}) can't be created as {0:?}")]
    InvalidCompressedDimension(wgt::TextureDimension, wgt::TextureFormat),
    #[error(
        "Texture descriptor mip level count {requested} is invalid, maximum allowed is {maximum}"
    )]
    InvalidMipLevelCount { requested: u32, maximum: u32 },
    #[error(
        "Texture usages {0:?} are not allowed on a texture of type {1:?}{downlevel_suffix}",
        downlevel_suffix = if *.2 { " due to downlevel restrictions" } else { "" }
    )]
    InvalidFormatUsages(wgt::TextureUsages, wgt::TextureFormat, bool),
    #[error("The view format {0:?} is not compatible with texture format {1:?}, only changing srgb-ness is allowed.")]
    InvalidViewFormat(wgt::TextureFormat, wgt::TextureFormat),
    #[error("Texture usages {0:?} are not allowed on a texture of dimensions {1:?}")]
    InvalidDimensionUsages(wgt::TextureUsages, wgt::TextureDimension),
    #[error("Texture usage STORAGE_BINDING is not allowed for multisampled textures")]
    InvalidMultisampledStorageBinding,
    #[error("Format {0:?} does not support multisampling")]
    InvalidMultisampledFormat(wgt::TextureFormat),
    #[error("Sample count {0} is not supported by format {1:?} on this device. The WebGPU spec guarantees {2:?} samples are supported by this format. With the TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES feature your device supports {3:?}.")]
    InvalidSampleCount(u32, wgt::TextureFormat, Vec<u32>, Vec<u32>),
    #[error("Multisampled textures must have RENDER_ATTACHMENT usage")]
    MultisampledNotRenderAttachment,
    #[error("Texture format {0:?} can't be used due to missing features")]
    MissingFeatures(wgt::TextureFormat, #[source] MissingFeatures),
    #[error(transparent)]
    MissingDownlevelFlags(#[from] MissingDownlevelFlags),
}

crate::impl_resource_type!(Texture);
crate::impl_labeled!(Texture);
crate::impl_parent_device!(Texture);
crate::impl_storage_item!(Texture);
crate::impl_trackable!(Texture);

impl Borrow<TextureSelector> for Texture {
    fn borrow(&self) -> &TextureSelector {
        &self.full_range
    }
}

impl WebGpuError for CreateTextureError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::CreateTextureView(e) => e.webgpu_error_type(),
            Self::InvalidDimension(e) => e.webgpu_error_type(),
            Self::MissingFeatures(_, e) => e.webgpu_error_type(),
            Self::MissingDownlevelFlags(e) => e.webgpu_error_type(),

            Self::InvalidUsage(_)
            | Self::IncompatibleUsage(_, _)
            | Self::InvalidDepthDimension(_, _)
            | Self::InvalidCompressedDimension(_, _)
            | Self::InvalidMipLevelCount { .. }
            | Self::InvalidFormatUsages(_, _, _)
            | Self::InvalidViewFormat(_, _)
            | Self::InvalidDimensionUsages(_, _)
            | Self::InvalidMultisampledStorageBinding
            | Self::InvalidMultisampledFormat(_)
            | Self::InvalidSampleCount(..)
            | Self::MultisampledNotRenderAttachment => ErrorType::Validation,
        }
    }
}

/// Describes a [`TextureView`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct TextureViewDescriptor<'a> {
    /// Debug label of the texture view.
    ///
    /// This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// Format of the texture view, or `None` for the same format as the texture
    /// itself.
    ///
    /// At this time, it must be the same the underlying format of the texture.
    pub format: Option<wgt::TextureFormat>,
    /// The dimension of the texture view.
    ///
    /// - For 1D textures, this must be `D1`.
    /// - For 2D textures it must be one of `D2`, `D2Array`, `Cube`, or `CubeArray`.
    /// - For 3D textures it must be `D3`.
    pub dimension: Option<wgt::TextureViewDimension>,
    /// The allowed usage(s) for the texture view. Must be a subset of the usage flags of the texture.
    /// If not provided, defaults to the full set of usage flags of the texture.
    pub usage: Option<wgt::TextureUsages>,
    /// Range within the texture that is accessible via this view.
    pub range: wgt::ImageSubresourceRange,
}

#[derive(Debug)]
pub(crate) struct HalTextureViewDescriptor {
    pub texture_format: wgt::TextureFormat,
    pub format: wgt::TextureFormat,
    pub usage: wgt::TextureUsages,
    pub dimension: wgt::TextureViewDimension,
    pub range: wgt::ImageSubresourceRange,
}

impl HalTextureViewDescriptor {
    pub fn aspects(&self) -> hal::FormatAspects {
        hal::FormatAspects::new(self.texture_format, self.range.aspect)
    }
}

#[derive(Debug, Copy, Clone, Error)]
pub enum TextureViewNotRenderableReason {
    #[error("The texture this view references doesn't include the RENDER_ATTACHMENT usage. Provided usages: {0:?}")]
    Usage(wgt::TextureUsages),
    #[error("The dimension of this texture view is not 2D. View dimension: {0:?}")]
    Dimension(wgt::TextureViewDimension),
    #[error("This texture view has more than one mipmap level. View mipmap levels: {0:?}")]
    MipLevelCount(u32),
    #[error("This texture view has more than one array layer. View array layers: {0:?}")]
    ArrayLayerCount(u32),
    #[error(
        "The aspects of this texture view are a subset of the aspects in the original texture. Aspects: {0:?}"
    )]
    Aspects(hal::FormatAspects),
}

#[derive(Debug)]
pub struct TextureView {
    pub(crate) raw: Snatchable<Box<dyn hal::DynTextureView>>,
    // if it's a surface texture - it's none
    pub(crate) parent: Arc<Texture>,
    pub(crate) device: Arc<Device>,
    pub(crate) desc: HalTextureViewDescriptor,
    pub(crate) format_features: wgt::TextureFormatFeatures,
    /// This is `Err` only if the texture view is not renderable
    pub(crate) render_extent: Result<wgt::Extent3d, TextureViewNotRenderableReason>,
    pub(crate) samples: u32,
    pub(crate) selector: TextureSelector,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
}

impl Drop for TextureView {
    fn drop(&mut self) {
        if let Some(raw) = self.raw.take() {
            resource_log!("Destroy raw {}", self.error_ident());
            unsafe {
                self.device.raw().destroy_texture_view(raw);
            }
        }
    }
}

impl RawResourceAccess for TextureView {
    type DynResource = dyn hal::DynTextureView;

    fn raw<'a>(&'a self, guard: &'a SnatchGuard) -> Option<&'a Self::DynResource> {
        self.raw.get(guard).map(|it| it.as_ref())
    }

    fn try_raw<'a>(
        &'a self,
        guard: &'a SnatchGuard,
    ) -> Result<&'a Self::DynResource, DestroyedResourceError> {
        self.parent.check_destroyed(guard)?;

        self.raw(guard)
            .ok_or_else(|| DestroyedResourceError(self.error_ident()))
    }
}

impl TextureView {
    /// Checks that the given texture usage contains the required texture usage,
    /// returns an error otherwise.
    pub(crate) fn check_usage(
        &self,
        expected: wgt::TextureUsages,
    ) -> Result<(), MissingTextureUsageError> {
        if self.desc.usage.contains(expected) {
            Ok(())
        } else {
            Err(MissingTextureUsageError {
                res: self.error_ident(),
                actual: self.desc.usage,
                expected,
            })
        }
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateTextureViewError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),
    #[error("Invalid texture view dimension `{view:?}` with texture of dimension `{texture:?}`")]
    InvalidTextureViewDimension {
        view: wgt::TextureViewDimension,
        texture: wgt::TextureDimension,
    },
    #[error("Texture view format `{0:?}` cannot be used as a render attachment. Make sure the format supports RENDER_ATTACHMENT usage and required device features are enabled.")]
    TextureViewFormatNotRenderable(wgt::TextureFormat),
    #[error("Texture view format `{0:?}` cannot be used as a storage binding. Make sure the format supports STORAGE usage and required device features are enabled.")]
    TextureViewFormatNotStorage(wgt::TextureFormat),
    #[error("Texture view usages (`{view:?}`) must be a subset of the texture's original usages (`{texture:?}`)")]
    InvalidTextureViewUsage {
        view: wgt::TextureUsages,
        texture: wgt::TextureUsages,
    },
    #[error("Texture view dimension `{0:?}` cannot be used with a multisampled texture")]
    InvalidMultisampledTextureViewDimension(wgt::TextureViewDimension),
    #[error(
        "TextureView has an arrayLayerCount of {depth}. Views of type `Cube` must have arrayLayerCount of 6."
    )]
    InvalidCubemapTextureDepth { depth: u32 },
    #[error("TextureView has an arrayLayerCount of {depth}. Views of type `CubeArray` must have an arrayLayerCount that is a multiple of 6.")]
    InvalidCubemapArrayTextureDepth { depth: u32 },
    #[error("Source texture width and height must be equal for a texture view of dimension `Cube`/`CubeArray`")]
    InvalidCubeTextureViewSize,
    #[error("Mip level count is 0")]
    ZeroMipLevelCount,
    #[error("Array layer count is 0")]
    ZeroArrayLayerCount,
    #[error(
        "`TextureView` starts at mip level {base_mip_level} and spans {mip_level_count} mip \
        levels, but the texture view only has {total} total mip level(s)"
    )]
    TooManyMipLevels {
        base_mip_level: u32,
        mip_level_count: u32,
        total: u32,
    },
    #[error(
        "`TextureView` starts at array layer {base_array_layer} and spans {array_layer_count}) \
        array layers, but the texture view only has {total} total layer(s)"
    )]
    TooManyArrayLayers {
        base_array_layer: u32,
        array_layer_count: u32,
        total: u32,
    },
    #[error("Requested array layer count {requested} is not valid for the target view dimension {dim:?}")]
    InvalidArrayLayerCount {
        requested: u32,
        dim: wgt::TextureViewDimension,
    },
    #[error(
        "Aspect {requested_aspect:?} is not a valid aspect of the source texture format {texture_format:?}"
    )]
    InvalidAspect {
        texture_format: wgt::TextureFormat,
        requested_aspect: wgt::TextureAspect,
    },
    #[error(
        "Trying to create a view of format {view:?} of a texture with format {texture:?}, \
         but this view format is not present in the texture's viewFormat array"
    )]
    FormatReinterpretation {
        texture: wgt::TextureFormat,
        view: wgt::TextureFormat,
    },
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
}

impl WebGpuError for CreateTextureViewError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),

            Self::InvalidTextureViewDimension { .. }
            | Self::InvalidResource(_)
            | Self::InvalidMultisampledTextureViewDimension(_)
            | Self::InvalidCubemapTextureDepth { .. }
            | Self::InvalidCubemapArrayTextureDepth { .. }
            | Self::InvalidCubeTextureViewSize
            | Self::ZeroMipLevelCount
            | Self::ZeroArrayLayerCount
            | Self::TooManyMipLevels { .. }
            | Self::TooManyArrayLayers { .. }
            | Self::InvalidArrayLayerCount { .. }
            | Self::InvalidAspect { .. }
            | Self::FormatReinterpretation { .. }
            | Self::DestroyedResource(_)
            | Self::TextureViewFormatNotRenderable(_)
            | Self::TextureViewFormatNotStorage(_)
            | Self::InvalidTextureViewUsage { .. }
            | Self::MissingFeatures(_) => ErrorType::Validation,
        }
    }
}

crate::impl_resource_type!(TextureView);
crate::impl_labeled!(TextureView);
crate::impl_parent_device!(TextureView);
crate::impl_storage_item!(TextureView);

pub type ExternalTextureDescriptor<'a> = wgt::ExternalTextureDescriptor<Label<'a>>;

#[derive(Debug)]
pub struct ExternalTexture {
    pub(crate) device: Arc<Device>,
    /// Between 1 and 3 (inclusive) planes of texture data.
    pub(crate) planes: arrayvec::ArrayVec<Arc<TextureView>, 3>,
    /// Buffer containing a [`crate::device::resource::ExternalTextureParams`]
    /// describing the external texture.
    pub(crate) params: Arc<Buffer>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
}

impl Drop for ExternalTexture {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
    }
}

impl ExternalTexture {
    pub fn destroy(self: &Arc<Self>) {
        self.params.destroy();
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateExternalTextureError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
    #[error(transparent)]
    CreateBuffer(#[from] CreateBufferError),
    #[error(transparent)]
    QueueWrite(#[from] queue::QueueWriteError),
    #[error("External texture format {format:?} expects {expected} planes, but given {provided}")]
    IncorrectPlaneCount {
        format: wgt::ExternalTextureFormat,
        expected: usize,
        provided: usize,
    },
    #[error("External texture planes cannot be multisampled, but given view with samples = {0}")]
    InvalidPlaneMultisample(u32),
    #[error("External texture planes expect a filterable float sample type, but given view with format {format:?} (sample type {sample_type:?})")]
    InvalidPlaneSampleType {
        format: wgt::TextureFormat,
        sample_type: wgt::TextureSampleType,
    },
    #[error("External texture planes expect 2D dimension, but given view with dimension = {0:?}")]
    InvalidPlaneDimension(wgt::TextureViewDimension),
    #[error(transparent)]
    MissingTextureUsage(#[from] MissingTextureUsageError),
    #[error("External texture format {format:?} plane {plane} expects format with {expected} components but given view with format {provided:?} ({} components)",
        provided.components())]
    InvalidPlaneFormat {
        format: wgt::ExternalTextureFormat,
        plane: usize,
        expected: u8,
        provided: wgt::TextureFormat,
    },
}

impl WebGpuError for CreateExternalTextureError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            CreateExternalTextureError::Device(e) => e.webgpu_error_type(),
            CreateExternalTextureError::MissingFeatures(e) => e.webgpu_error_type(),
            CreateExternalTextureError::InvalidResource(e) => e.webgpu_error_type(),
            CreateExternalTextureError::CreateBuffer(e) => e.webgpu_error_type(),
            CreateExternalTextureError::QueueWrite(e) => e.webgpu_error_type(),
            CreateExternalTextureError::MissingTextureUsage(e) => e.webgpu_error_type(),
            CreateExternalTextureError::IncorrectPlaneCount { .. }
            | CreateExternalTextureError::InvalidPlaneMultisample(_)
            | CreateExternalTextureError::InvalidPlaneSampleType { .. }
            | CreateExternalTextureError::InvalidPlaneDimension(_)
            | CreateExternalTextureError::InvalidPlaneFormat { .. } => ErrorType::Validation,
        }
    }
}

crate::impl_resource_type!(ExternalTexture);
crate::impl_labeled!(ExternalTexture);
crate::impl_parent_device!(ExternalTexture);
crate::impl_storage_item!(ExternalTexture);
crate::impl_trackable!(ExternalTexture);

/// Describes a [`Sampler`]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SamplerDescriptor<'a> {
    /// Debug label of the sampler.
    ///
    /// This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// How to deal with out of bounds accesses in the u (i.e. x) direction
    pub address_modes: [wgt::AddressMode; 3],
    /// How to filter the texture when it needs to be magnified (made larger)
    pub mag_filter: wgt::FilterMode,
    /// How to filter the texture when it needs to be minified (made smaller)
    pub min_filter: wgt::FilterMode,
    /// How to filter between mip map levels
    pub mipmap_filter: wgt::MipmapFilterMode,
    /// Minimum level of detail (i.e. mip level) to use
    pub lod_min_clamp: f32,
    /// Maximum level of detail (i.e. mip level) to use
    pub lod_max_clamp: f32,
    /// If this is enabled, this is a comparison sampler using the given comparison function.
    pub compare: Option<wgt::CompareFunction>,
    /// Must be at least 1. If this is not 1, all filter modes must be linear.
    pub anisotropy_clamp: u16,
    /// Border color to use when address_mode is
    /// [`AddressMode::ClampToBorder`](wgt::AddressMode::ClampToBorder)
    pub border_color: Option<wgt::SamplerBorderColor>,
}

#[derive(Debug)]
pub struct Sampler {
    pub(crate) raw: ManuallyDrop<Box<dyn hal::DynSampler>>,
    pub(crate) device: Arc<Device>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
    /// `true` if this is a comparison sampler
    pub(crate) comparison: bool,
    /// `true` if this is a filtering sampler
    pub(crate) filtering: bool,
}

impl Drop for Sampler {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            self.device.raw().destroy_sampler(raw);
        }
    }
}

impl Sampler {
    pub(crate) fn raw(&self) -> &dyn hal::DynSampler {
        self.raw.as_ref()
    }
}

#[derive(Copy, Clone)]
pub enum SamplerFilterErrorType {
    MagFilter,
    MinFilter,
    MipmapFilter,
}

impl fmt::Debug for SamplerFilterErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SamplerFilterErrorType::MagFilter => write!(f, "magFilter"),
            SamplerFilterErrorType::MinFilter => write!(f, "minFilter"),
            SamplerFilterErrorType::MipmapFilter => write!(f, "mipmapFilter"),
        }
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateSamplerError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Invalid lodMinClamp: {0}. Must be greater or equal to 0.0")]
    InvalidLodMinClamp(f32),
    #[error("Invalid lodMaxClamp: {lod_max_clamp}. Must be greater or equal to lodMinClamp (which is {lod_min_clamp}).")]
    InvalidLodMaxClamp {
        lod_min_clamp: f32,
        lod_max_clamp: f32,
    },
    #[error("Invalid anisotropic clamp: {0}. Must be at least 1.")]
    InvalidAnisotropy(u16),
    #[error("Invalid filter mode for {filter_type:?}: {filter_mode:?}. When anistropic clamp is not 1 (it is {anisotropic_clamp}), all filter modes must be linear.")]
    InvalidFilterModeWithAnisotropy {
        filter_type: SamplerFilterErrorType,
        filter_mode: wgt::FilterMode,
        anisotropic_clamp: u16,
    },
    #[error("Invalid filter mode for {filter_type:?}: {filter_mode:?}. When anistropic clamp is not 1 (it is {anisotropic_clamp}), all filter modes must be linear.")]
    InvalidMipmapFilterModeWithAnisotropy {
        filter_type: SamplerFilterErrorType,
        filter_mode: wgt::MipmapFilterMode,
        anisotropic_clamp: u16,
    },
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
}

crate::impl_resource_type!(Sampler);
crate::impl_labeled!(Sampler);
crate::impl_parent_device!(Sampler);
crate::impl_storage_item!(Sampler);
crate::impl_trackable!(Sampler);

impl WebGpuError for CreateSamplerError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),

            Self::InvalidLodMinClamp(_)
            | Self::InvalidLodMaxClamp { .. }
            | Self::InvalidAnisotropy(_)
            | Self::InvalidFilterModeWithAnisotropy { .. }
            | Self::InvalidMipmapFilterModeWithAnisotropy { .. } => ErrorType::Validation,
        }
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateQuerySetError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("QuerySets cannot be made with zero queries")]
    ZeroCount,
    #[error("{count} is too many queries for a single QuerySet. QuerySets cannot be made more than {maximum} queries.")]
    TooManyQueries { count: u32, maximum: u32 },
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
}

impl WebGpuError for CreateQuerySetError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),

            Self::TooManyQueries { .. } | Self::ZeroCount => ErrorType::Validation,
        }
    }
}

pub type QuerySetDescriptor<'a> = wgt::QuerySetDescriptor<Label<'a>>;

#[derive(Debug)]
pub struct QuerySet {
    pub(crate) raw: ManuallyDrop<Box<dyn hal::DynQuerySet>>,
    pub(crate) device: Arc<Device>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
    pub(crate) desc: wgt::QuerySetDescriptor<()>,
}

impl Drop for QuerySet {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            self.device.raw().destroy_query_set(raw);
        }
    }
}

crate::impl_resource_type!(QuerySet);
crate::impl_labeled!(QuerySet);
crate::impl_parent_device!(QuerySet);
crate::impl_storage_item!(QuerySet);
crate::impl_trackable!(QuerySet);

impl QuerySet {
    pub(crate) fn raw(&self) -> &dyn hal::DynQuerySet {
        self.raw.as_ref()
    }
}

pub type BlasDescriptor<'a> = wgt::CreateBlasDescriptor<Label<'a>>;
pub type TlasDescriptor<'a> = wgt::CreateTlasDescriptor<Label<'a>>;

pub type BlasPrepareCompactResult = Result<(), BlasPrepareCompactError>;

#[cfg(send_sync)]
pub type BlasCompactCallback = Box<dyn FnOnce(BlasPrepareCompactResult) + Send + 'static>;
#[cfg(not(send_sync))]
pub type BlasCompactCallback = Box<dyn FnOnce(BlasPrepareCompactResult) + 'static>;

pub(crate) struct BlasPendingCompact {
    pub(crate) op: Option<BlasCompactCallback>,
    // hold the parent alive while the mapping is active
    pub(crate) _parent_blas: Arc<Blas>,
}

impl fmt::Debug for BlasPendingCompact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlasPendingCompact")
            .field("op", &())
            .field("_parent_blas", &self._parent_blas)
            .finish()
    }
}

#[derive(Debug)]
pub(crate) enum BlasCompactState {
    /// Created from a compact operation.
    Compacted,
    /// Waiting for GPU to be done before mapping to get compacted size
    Waiting(BlasPendingCompact),
    /// Ready to be compacted
    Ready { size: wgt::BufferAddress },
    /// Ready to prepare to compact.
    Idle,
}

#[cfg(send_sync)]
unsafe impl Send for BlasCompactState {}
#[cfg(send_sync)]
unsafe impl Sync for BlasCompactState {}

#[derive(Debug)]
pub struct Blas {
    pub(crate) raw: Snatchable<Box<dyn hal::DynAccelerationStructure>>,
    pub(crate) device: Arc<Device>,
    pub(crate) size_info: hal::AccelerationStructureBuildSizes,
    pub(crate) sizes: wgt::BlasGeometrySizeDescriptors,
    pub(crate) flags: wgt::AccelerationStructureFlags,
    pub(crate) update_mode: wgt::AccelerationStructureUpdateMode,
    pub(crate) built_index: RwLock<Option<NonZeroU64>>,
    pub(crate) handle: u64,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
    pub(crate) compaction_buffer: Option<ManuallyDrop<Box<dyn hal::DynBuffer>>>,
    pub(crate) compacted_state: Mutex<BlasCompactState>,
}

impl Drop for Blas {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        // SAFETY: We are in the Drop impl, and we don't use self.raw or self.compaction_buffer anymore after this point.
        if let Some(raw) = self.raw.take() {
            unsafe {
                self.device.raw().destroy_acceleration_structure(raw);
            }
        }
        if let Some(mut raw) = self.compaction_buffer.take() {
            unsafe {
                self.device
                    .raw()
                    .destroy_buffer(ManuallyDrop::take(&mut raw))
            }
        }
    }
}

impl RawResourceAccess for Blas {
    type DynResource = dyn hal::DynAccelerationStructure;

    fn raw<'a>(&'a self, guard: &'a SnatchGuard) -> Option<&'a Self::DynResource> {
        self.raw.get(guard).map(|it| it.as_ref())
    }
}

impl Blas {
    pub(crate) fn prepare_compact_async(
        self: &Arc<Self>,
        op: Option<BlasCompactCallback>,
    ) -> Result<SubmissionIndex, (Option<BlasCompactCallback>, BlasPrepareCompactError)> {
        let device = &self.device;
        if let Err(e) = device.check_is_valid() {
            return Err((op, e.into()));
        }

        if self.built_index.read().is_none() {
            return Err((op, BlasPrepareCompactError::NotBuilt));
        }

        if !self
            .flags
            .contains(wgt::AccelerationStructureFlags::ALLOW_COMPACTION)
        {
            return Err((op, BlasPrepareCompactError::CompactionUnsupported));
        }

        let mut state = self.compacted_state.lock();
        *state = match *state {
            BlasCompactState::Compacted => {
                return Err((op, BlasPrepareCompactError::DoubleCompaction))
            }
            BlasCompactState::Waiting(_) => {
                return Err((op, BlasPrepareCompactError::CompactionPreparingAlready))
            }
            BlasCompactState::Ready { .. } => {
                return Err((op, BlasPrepareCompactError::CompactionPreparingAlready))
            }
            BlasCompactState::Idle => BlasCompactState::Waiting(BlasPendingCompact {
                op,
                _parent_blas: self.clone(),
            }),
        };

        let submit_index = if let Some(queue) = device.get_queue() {
            queue.lock_life().prepare_compact(self).unwrap_or(0) // '0' means no wait is necessary
        } else {
            // We can safely unwrap below since we just set the `compacted_state` to `BlasCompactState::Waiting`.
            let (mut callback, status) = self.read_back_compact_size().unwrap();
            if let Some(callback) = callback.take() {
                callback(status);
            }
            0
        };

        Ok(submit_index)
    }

    /// This function returns [`None`] only if [`Self::compacted_state`] is not [`BlasCompactState::Waiting`].
    #[must_use]
    pub(crate) fn read_back_compact_size(&self) -> Option<BlasCompactReadyPendingClosure> {
        let mut state = self.compacted_state.lock();
        let pending_compact = match mem::replace(&mut *state, BlasCompactState::Idle) {
            BlasCompactState::Waiting(pending_mapping) => pending_mapping,
            // Compaction cancelled e.g. by rebuild
            BlasCompactState::Idle => return None,
            BlasCompactState::Ready { .. } => {
                unreachable!("This should be validated out by `prepare_for_compaction`")
            }
            _ => panic!("No pending mapping."),
        };
        let status = {
            let compaction_buffer = self.compaction_buffer.as_ref().unwrap().as_ref();
            unsafe {
                let map_res = self.device.raw().map_buffer(
                    compaction_buffer,
                    0..size_of::<wgpu_types::BufferAddress>() as wgt::BufferAddress,
                );
                match map_res {
                    Ok(mapping) => {
                        if !mapping.is_coherent {
                            // Clippy complains about this because it might not be intended, but
                            // this is intentional.
                            #[expect(clippy::single_range_in_vec_init)]
                            self.device.raw().invalidate_mapped_ranges(
                                compaction_buffer,
                                &[0..size_of::<wgpu_types::BufferAddress>() as wgt::BufferAddress],
                            );
                        }
                        let size = core::ptr::read_unaligned(
                            mapping.ptr.as_ptr().cast::<wgt::BufferAddress>(),
                        );
                        self.device.raw().unmap_buffer(compaction_buffer);
                        if self.size_info.acceleration_structure_size != 0 {
                            debug_assert_ne!(size, 0);
                        }
                        *state = BlasCompactState::Ready { size };
                        Ok(())
                    }
                    Err(err) => Err(BlasPrepareCompactError::from(DeviceError::from_hal(err))),
                }
            }
        };
        Some((pending_compact.op, status))
    }
}

crate::impl_resource_type!(Blas);
crate::impl_labeled!(Blas);
crate::impl_parent_device!(Blas);
crate::impl_storage_item!(Blas);
crate::impl_trackable!(Blas);

#[derive(Debug)]
pub struct Tlas {
    pub(crate) raw: Snatchable<Box<dyn hal::DynAccelerationStructure>>,
    pub(crate) device: Arc<Device>,
    pub(crate) size_info: hal::AccelerationStructureBuildSizes,
    pub(crate) max_instance_count: u32,
    pub(crate) flags: wgt::AccelerationStructureFlags,
    pub(crate) update_mode: wgt::AccelerationStructureUpdateMode,
    pub(crate) built_index: RwLock<Option<NonZeroU64>>,
    pub(crate) dependencies: RwLock<Vec<Arc<Blas>>>,
    pub(crate) instance_buffer: ManuallyDrop<Box<dyn hal::DynBuffer>>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
}

impl Drop for Tlas {
    fn drop(&mut self) {
        unsafe {
            resource_log!("Destroy raw {}", self.error_ident());
            if let Some(structure) = self.raw.take() {
                self.device.raw().destroy_acceleration_structure(structure);
            }
            let buffer = ManuallyDrop::take(&mut self.instance_buffer);
            self.device.raw().destroy_buffer(buffer);
        }
    }
}

impl RawResourceAccess for Tlas {
    type DynResource = dyn hal::DynAccelerationStructure;

    fn raw<'a>(&'a self, guard: &'a SnatchGuard) -> Option<&'a Self::DynResource> {
        self.raw.get(guard).map(|raw| raw.as_ref())
    }
}

crate::impl_resource_type!(Tlas);
crate::impl_labeled!(Tlas);
crate::impl_parent_device!(Tlas);
crate::impl_storage_item!(Tlas);
crate::impl_trackable!(Tlas);
