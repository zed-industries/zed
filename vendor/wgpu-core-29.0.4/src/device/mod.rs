use alloc::{boxed::Box, string::String, vec::Vec};
use core::{fmt, num::NonZeroU32};

use crate::{
    binding_model,
    ray_tracing::BlasCompactReadyPendingClosure,
    resource::{
        Buffer, BufferAccessError, BufferAccessResult, BufferMapOperation, Labeled,
        RawResourceAccess, ResourceErrorIdent,
    },
    snatch::SnatchGuard,
    Label, DOWNLEVEL_ERROR_MESSAGE,
};

use arrayvec::ArrayVec;
use smallvec::SmallVec;
use thiserror::Error;
use wgt::{
    error::{ErrorType, WebGpuError},
    BufferAddress, DeviceLostReason, TextureFormat,
};

pub(crate) mod bgl;
pub mod global;
mod life;
pub mod queue;
pub mod ray_tracing;
pub mod resource;
#[cfg(any(feature = "trace", feature = "replay"))]
pub mod trace;
pub use {life::WaitIdleError, resource::Device};

pub const SHADER_STAGE_COUNT: usize = hal::MAX_CONCURRENT_SHADER_STAGES;
// Should be large enough for the largest possible texture row. This
// value is enough for a 16k texture with float4 format.
pub(crate) const ZERO_BUFFER_SIZE: BufferAddress = 512 << 10;

pub(crate) const ENTRYPOINT_FAILURE_ERROR: &str = "The given EntryPoint is Invalid";

pub type DeviceDescriptor<'a> = wgt::DeviceDescriptor<Label<'a>>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HostMap {
    Read,
    Write,
}

#[derive(Clone, Debug, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub(crate) struct AttachmentData<T> {
    pub colors: ArrayVec<Option<T>, { hal::MAX_COLOR_ATTACHMENTS }>,
    pub resolves: ArrayVec<T, { hal::MAX_COLOR_ATTACHMENTS }>,
    pub depth_stencil: Option<T>,
}
impl<T: PartialEq> Eq for AttachmentData<T> {}

#[derive(Clone, Debug, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub(crate) struct RenderPassContext {
    pub attachments: AttachmentData<TextureFormat>,
    pub sample_count: u32,
    pub multiview_mask: Option<NonZeroU32>,
}
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum RenderPassCompatibilityError {
    #[error(
        "Incompatible color attachments at indices {indices:?}: the RenderPass uses textures with formats {expected:?} but the {res} uses attachments with formats {actual:?}",
    )]
    IncompatibleColorAttachment {
        indices: Vec<usize>,
        expected: Vec<Option<TextureFormat>>,
        actual: Vec<Option<TextureFormat>>,
        res: ResourceErrorIdent,
    },
    #[error(
        "Incompatible depth-stencil attachment format: the RenderPass uses a texture with format {expected:?} but the {res} uses an attachment with format {actual:?}",
    )]
    IncompatibleDepthStencilAttachment {
        expected: Option<TextureFormat>,
        actual: Option<TextureFormat>,
        res: ResourceErrorIdent,
    },
    #[error(
        "Incompatible sample count: the RenderPass uses textures with sample count {expected:?} but the {res} uses attachments with format {actual:?}",
    )]
    IncompatibleSampleCount {
        expected: u32,
        actual: u32,
        res: ResourceErrorIdent,
    },
    #[error("Incompatible multiview setting: the RenderPass uses setting {expected:?} but the {res} uses setting {actual:?}")]
    IncompatibleMultiview {
        expected: Option<NonZeroU32>,
        actual: Option<NonZeroU32>,
        res: ResourceErrorIdent,
    },
}

impl WebGpuError for RenderPassCompatibilityError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

impl RenderPassContext {
    // Assumes the renderpass only contains one subpass
    pub(crate) fn check_compatible<T: Labeled>(
        &self,
        other: &Self,
        res: &T,
    ) -> Result<(), RenderPassCompatibilityError> {
        if self.attachments.colors != other.attachments.colors {
            let indices = self
                .attachments
                .colors
                .iter()
                .zip(&other.attachments.colors)
                .enumerate()
                .filter_map(|(idx, (left, right))| (left != right).then_some(idx))
                .collect();
            return Err(RenderPassCompatibilityError::IncompatibleColorAttachment {
                indices,
                expected: self.attachments.colors.iter().cloned().collect(),
                actual: other.attachments.colors.iter().cloned().collect(),
                res: res.error_ident(),
            });
        }
        if self.attachments.depth_stencil != other.attachments.depth_stencil {
            return Err(
                RenderPassCompatibilityError::IncompatibleDepthStencilAttachment {
                    expected: self.attachments.depth_stencil,
                    actual: other.attachments.depth_stencil,
                    res: res.error_ident(),
                },
            );
        }
        if self.sample_count != other.sample_count {
            return Err(RenderPassCompatibilityError::IncompatibleSampleCount {
                expected: self.sample_count,
                actual: other.sample_count,
                res: res.error_ident(),
            });
        }
        if self.multiview_mask != other.multiview_mask {
            return Err(RenderPassCompatibilityError::IncompatibleMultiview {
                expected: self.multiview_mask,
                actual: other.multiview_mask,
                res: res.error_ident(),
            });
        }
        Ok(())
    }
}

pub type BufferMapPendingClosure = (BufferMapOperation, BufferAccessResult);

#[derive(Default)]
pub struct UserClosures {
    pub mappings: Vec<BufferMapPendingClosure>,
    pub blas_compact_ready: Vec<BlasCompactReadyPendingClosure>,
    pub submissions: SmallVec<[queue::SubmittedWorkDoneClosure; 1]>,
    pub device_lost_invocations: SmallVec<[DeviceLostInvocation; 1]>,
}

impl UserClosures {
    fn extend(&mut self, other: Self) {
        self.mappings.extend(other.mappings);
        self.blas_compact_ready.extend(other.blas_compact_ready);
        self.submissions.extend(other.submissions);
        self.device_lost_invocations
            .extend(other.device_lost_invocations);
    }

    fn fire(self) {
        // Note: this logic is specifically moved out of `handle_mapping()` in order to
        // have nothing locked by the time we execute users callback code.

        // Mappings _must_ be fired before submissions, as the spec requires all mapping callbacks that are registered before
        // a on_submitted_work_done callback to be fired before the on_submitted_work_done callback.
        for (mut operation, status) in self.mappings {
            if let Some(callback) = operation.callback.take() {
                callback(status);
            }
        }
        for (mut operation, status) in self.blas_compact_ready {
            if let Some(callback) = operation.take() {
                callback(status);
            }
        }
        for closure in self.submissions {
            closure();
        }
        for invocation in self.device_lost_invocations {
            (invocation.closure)(invocation.reason, invocation.message);
        }
    }
}

#[cfg(send_sync)]
pub type DeviceLostClosure = Box<dyn FnOnce(DeviceLostReason, String) + Send + 'static>;
#[cfg(not(send_sync))]
pub type DeviceLostClosure = Box<dyn FnOnce(DeviceLostReason, String) + 'static>;

pub struct DeviceLostInvocation {
    closure: DeviceLostClosure,
    reason: DeviceLostReason,
    message: String,
}

pub(crate) fn map_buffer(
    buffer: &Buffer,
    offset: BufferAddress,
    size: BufferAddress,
    kind: HostMap,
    snatch_guard: &SnatchGuard,
) -> Result<hal::BufferMapping, BufferAccessError> {
    let raw_device = buffer.device.raw();
    let raw_buffer = buffer.try_raw(snatch_guard)?;
    let mapping = unsafe {
        raw_device
            .map_buffer(raw_buffer, offset..offset + size)
            .map_err(|e| buffer.device.handle_hal_error(e))?
    };

    if !mapping.is_coherent && kind == HostMap::Read {
        #[allow(clippy::single_range_in_vec_init)]
        unsafe {
            raw_device.invalidate_mapped_ranges(raw_buffer, &[offset..offset + size]);
        }
    }

    assert_eq!(offset % wgt::COPY_BUFFER_ALIGNMENT, 0);
    assert_eq!(size % wgt::COPY_BUFFER_ALIGNMENT, 0);
    // Zero out uninitialized parts of the mapping. (Spec dictates all resources
    // behave as if they were initialized with zero)
    //
    // If this is a read mapping, ideally we would use a `clear_buffer` command
    // before reading the data from GPU (i.e. `invalidate_range`). However, this
    // would require us to kick off and wait for a command buffer or piggy back
    // on an existing one (the later is likely the only worthwhile option). As
    // reading uninitialized memory isn't a particular important path to
    // support, we instead just initialize the memory here and make sure it is
    // GPU visible, so this happens at max only once for every buffer region.
    //
    // If this is a write mapping zeroing out the memory here is the only
    // reasonable way as all data is pushed to GPU anyways.

    let mapped = unsafe { core::slice::from_raw_parts_mut(mapping.ptr.as_ptr(), size as usize) };

    // We can't call flush_mapped_ranges in this case, so we can't drain the uninitialized ranges either
    if !mapping.is_coherent
        && kind == HostMap::Read
        && !buffer.usage.contains(wgt::BufferUsages::MAP_WRITE)
    {
        for uninitialized in buffer
            .initialization_status
            .write()
            .uninitialized(offset..(size + offset))
        {
            // The mapping's pointer is already offset, however we track the
            // uninitialized range relative to the buffer's start.
            let fill_range =
                (uninitialized.start - offset) as usize..(uninitialized.end - offset) as usize;
            mapped[fill_range].fill(0);
        }
    } else {
        for uninitialized in buffer
            .initialization_status
            .write()
            .drain(offset..(size + offset))
        {
            // The mapping's pointer is already offset, however we track the
            // uninitialized range relative to the buffer's start.
            let fill_range =
                (uninitialized.start - offset) as usize..(uninitialized.end - offset) as usize;
            mapped[fill_range].fill(0);

            // NOTE: This is only possible when MAPPABLE_PRIMARY_BUFFERS is enabled.
            if !mapping.is_coherent
                && kind == HostMap::Read
                && buffer.usage.contains(wgt::BufferUsages::MAP_WRITE)
            {
                unsafe { raw_device.flush_mapped_ranges(raw_buffer, &[uninitialized]) };
            }
        }
    }

    Ok(mapping)
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceMismatch {
    pub(super) res: ResourceErrorIdent,
    pub(super) res_device: ResourceErrorIdent,
    pub(super) target: Option<ResourceErrorIdent>,
    pub(super) target_device: ResourceErrorIdent,
}

impl fmt::Display for DeviceMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} of {} doesn't match {}",
            self.res_device, self.res, self.target_device
        )?;
        if let Some(target) = self.target.as_ref() {
            write!(f, " of {target}")?;
        }
        Ok(())
    }
}

impl core::error::Error for DeviceMismatch {}

impl WebGpuError for DeviceMismatch {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum DeviceError {
    #[error("Parent device is lost")]
    Lost,
    #[error("Not enough memory left.")]
    OutOfMemory,
    #[error(transparent)]
    DeviceMismatch(#[from] Box<DeviceMismatch>),
}

impl WebGpuError for DeviceError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::DeviceMismatch(e) => e.webgpu_error_type(),
            Self::Lost => ErrorType::DeviceLost,
            Self::OutOfMemory => ErrorType::OutOfMemory,
        }
    }
}

impl DeviceError {
    /// Only use this function in contexts where there is no `Device`.
    ///
    /// Use [`Device::handle_hal_error`] otherwise.
    pub fn from_hal(error: hal::DeviceError) -> Self {
        match error {
            hal::DeviceError::Lost => Self::Lost,
            hal::DeviceError::OutOfMemory => Self::OutOfMemory,
            hal::DeviceError::Unexpected => Self::Lost,
        }
    }
}

#[derive(Clone, Debug, Error)]
#[error("Features {0:?} are required but not enabled on the device")]
pub struct MissingFeatures(pub wgt::Features);

impl WebGpuError for MissingFeatures {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[error(
    "Downlevel flags {0:?} are required but not supported on the device.\n{DOWNLEVEL_ERROR_MESSAGE}",
)]
pub struct MissingDownlevelFlags(pub wgt::DownlevelFlags);

impl WebGpuError for MissingDownlevelFlags {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

pub use wgpu_naga_bridge::create_validator;
pub use wgpu_naga_bridge::features_to_naga_capabilities;
