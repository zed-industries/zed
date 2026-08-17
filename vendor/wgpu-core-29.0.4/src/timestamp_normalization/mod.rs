//! Utility for normalizing GPU timestamp queries to have a consistent
//! 1GHz period. This uses a compute shader to do the normalization,
//! so the timestamps exist in their correct format on the GPU, as
//! is required by the WebGPU specification.
//!
//! ## Algorithm
//!
//! The fundamental operation is multiplying a u64 timestamp by an f32
//! value. We have neither f64s nor u64s in shaders, so we need to do
//! something more complicated.
//!
//! We first decompose the f32 into a u32 fraction where the denominator
//! is a power of two. We do the computation with f64 for ease of computation,
//! as those can store u32s losslessly.
//!
//! Because the denominator is a power of two, this means the shader can evaluate
//! this divide by using a shift. Additionally, we always choose the largest denominator
//! we can, so that the fraction is as precise as possible.
//!
//! To evaluate this function, we have two helper operations (both in common.wgsl).
//!
//! 1. `u64_mul_u32` multiplies a u64 by a u32 and returns a u96.
//! 2. `shift_right_u96` shifts a u96 right by a given amount, returning a u96.
//!
//! See their implementations for more details.
//!
//! We then multiply the timestamp by the numerator, and shift it right by the
//! denominator. This gives us the normalized timestamp.

use core::num::NonZeroU64;

use alloc::{boxed::Box, string::String, string::ToString, sync::Arc};

use hashbrown::HashMap;

use crate::{
    device::{Device, DeviceError},
    hal_label,
    pipeline::{CreateComputePipelineError, CreateShaderModuleError},
    resource::Buffer,
    snatch::SnatchGuard,
    track::BufferTracker,
};

pub const TIMESTAMP_NORMALIZATION_BUFFER_USES: wgt::BufferUses =
    wgt::BufferUses::STORAGE_READ_WRITE;

struct InternalState {
    temporary_bind_group_layout: Box<dyn hal::DynBindGroupLayout>,
    pipeline_layout: Box<dyn hal::DynPipelineLayout>,
    pipeline: Box<dyn hal::DynComputePipeline>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum TimestampNormalizerInitError {
    #[error("Failed to initialize bind group layout")]
    BindGroupLayout(#[source] DeviceError),
    #[cfg(feature = "wgsl")]
    #[error("Failed to parse shader")]
    ParseWgsl(#[source] naga::error::ShaderError<naga::front::wgsl::ParseError>),
    #[error("Failed to validate shader module")]
    ValidateWgsl(#[source] naga::error::ShaderError<naga::WithSpan<naga::valid::ValidationError>>),
    #[error("Failed to create shader module")]
    CreateShaderModule(#[from] CreateShaderModuleError),
    #[error("Failed to create pipeline layout")]
    PipelineLayout(#[source] DeviceError),
    #[error("Failed to create compute pipeline")]
    ComputePipeline(#[from] CreateComputePipelineError),
}

/// Normalizes GPU timestamps to have a consistent 1GHz period.
/// See module documentation for more information.
pub struct TimestampNormalizer {
    state: Option<InternalState>,
}

impl TimestampNormalizer {
    /// Creates a new timestamp normalizer.
    ///
    /// If the device cannot support automatic timestamp normalization,
    /// this will return a normalizer that does nothing.
    ///
    /// # Errors
    ///
    /// If any resources are invalid, this will return an error.
    pub fn new(
        device: &Device,
        timestamp_period: f32,
    ) -> Result<Self, TimestampNormalizerInitError> {
        unsafe {
            if !device
                .instance_flags
                .contains(wgt::InstanceFlags::AUTOMATIC_TIMESTAMP_NORMALIZATION)
            {
                return Ok(Self { state: None });
            }

            if !device
                .downlevel
                .flags
                .contains(wgt::DownlevelFlags::COMPUTE_SHADERS)
            {
                log::error!("Automatic timestamp normalization was requested, but compute shaders are not supported.");
                return Ok(Self { state: None });
            }

            if timestamp_period == 1.0 {
                // If the period is 1, we don't need to do anything to them.
                return Ok(Self { state: None });
            }

            let temporary_bind_group_layout = device
                .raw()
                .create_bind_group_layout(&hal::BindGroupLayoutDescriptor {
                    label: hal_label(
                        Some("(wgpu internal) Timestamp Normalization Bind Group Layout"),
                        device.instance_flags,
                    ),
                    flags: hal::BindGroupLayoutFlags::empty(),
                    entries: &[wgt::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgt::ShaderStages::COMPUTE,
                        ty: wgt::BindingType::Buffer {
                            ty: wgt::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: Some(NonZeroU64::new(8).unwrap()),
                        },
                        count: None,
                    }],
                })
                .map_err(|e| {
                    TimestampNormalizerInitError::BindGroupLayout(device.handle_hal_error(e))
                })?;

            let common_src = include_str!("common.wgsl");
            let src = include_str!("timestamp_normalization.wgsl");

            let preprocessed_src = alloc::format!("{common_src}\n{src}");

            #[cfg(feature = "wgsl")]
            let module = naga::front::wgsl::parse_str(&preprocessed_src).map_err(|inner| {
                TimestampNormalizerInitError::ParseWgsl(naga::error::ShaderError {
                    source: preprocessed_src.clone(),
                    label: None,
                    inner: Box::new(inner),
                })
            })?;
            #[cfg(not(feature = "wgsl"))]
            #[allow(clippy::diverging_sub_expression)]
            let module =
                panic!("Timestamp normalization requires the wgsl feature flag to be enabled!");

            let info = crate::device::create_validator(
                wgt::Features::IMMEDIATES,
                wgt::DownlevelFlags::empty(),
                naga::valid::ValidationFlags::all(),
            )
            .validate(&module)
            .map_err(|inner| {
                TimestampNormalizerInitError::ValidateWgsl(naga::error::ShaderError {
                    source: preprocessed_src.clone(),
                    label: None,
                    inner: Box::new(inner),
                })
            })?;
            let hal_shader = hal::ShaderInput::Naga(hal::NagaShader {
                module: alloc::borrow::Cow::Owned(module),
                info,
                debug_source: None,
            });
            let hal_desc = hal::ShaderModuleDescriptor {
                label: hal_label(
                    Some("(wgpu internal) Timestamp normalizer shader module"),
                    device.instance_flags,
                ),
                runtime_checks: wgt::ShaderRuntimeChecks::unchecked(),
            };
            let module = device
                .raw()
                .create_shader_module(&hal_desc, hal_shader)
                .map_err(|error| match error {
                    hal::ShaderError::Device(error) => {
                        CreateShaderModuleError::Device(device.handle_hal_error(error))
                    }
                    hal::ShaderError::Compilation(ref msg) => {
                        log::error!("Shader error: {msg}");
                        CreateShaderModuleError::Generation
                    }
                })?;

            let pipeline_layout = device
                .raw()
                .create_pipeline_layout(&hal::PipelineLayoutDescriptor {
                    label: hal_label(
                        Some("(wgpu internal) Timestamp normalizer pipeline layout"),
                        device.instance_flags,
                    ),
                    bind_group_layouts: &[Some(temporary_bind_group_layout.as_ref())],
                    immediate_size: 8,
                    flags: hal::PipelineLayoutFlags::empty(),
                })
                .map_err(|e| {
                    TimestampNormalizerInitError::PipelineLayout(device.handle_hal_error(e))
                })?;

            let (multiplier, shift) = compute_timestamp_period(timestamp_period);

            let mut constants = HashMap::with_capacity(2);
            constants.insert(String::from("TIMESTAMP_PERIOD_MULTIPLY"), multiplier as f64);
            constants.insert(String::from("TIMESTAMP_PERIOD_SHIFT"), shift as f64);

            let pipeline_desc = hal::ComputePipelineDescriptor {
                label: hal_label(
                    Some("(wgpu internal) Timestamp normalizer pipeline"),
                    device.instance_flags,
                ),
                layout: pipeline_layout.as_ref(),
                stage: hal::ProgrammableStage {
                    module: module.as_ref(),
                    entry_point: "main",
                    constants: &constants,
                    zero_initialize_workgroup_memory: false,
                },
                cache: None,
            };
            let pipeline = device
                .raw()
                .create_compute_pipeline(&pipeline_desc)
                .map_err(|err| match err {
                    hal::PipelineError::Device(error) => {
                        CreateComputePipelineError::Device(device.handle_hal_error(error))
                    }
                    hal::PipelineError::Linkage(_stages, msg) => {
                        CreateComputePipelineError::Internal(msg)
                    }
                    hal::PipelineError::EntryPoint(_stage) => CreateComputePipelineError::Internal(
                        crate::device::ENTRYPOINT_FAILURE_ERROR.to_string(),
                    ),
                    hal::PipelineError::PipelineConstants(_, error) => {
                        CreateComputePipelineError::PipelineConstants(error)
                    }
                })?;

            Ok(Self {
                state: Some(InternalState {
                    temporary_bind_group_layout,
                    pipeline_layout,
                    pipeline,
                }),
            })
        }
    }

    /// Create a bind group for normalizing timestamps in `buffer`.
    ///
    /// This function is unsafe because it does not know that `buffer_size` is
    /// the true size of the buffer.
    pub unsafe fn create_normalization_bind_group(
        &self,
        device: &Device,
        buffer: &dyn hal::DynBuffer,
        buffer_label: Option<&str>,
        buffer_size: wgt::BufferSize,
        buffer_usages: wgt::BufferUsages,
    ) -> Result<TimestampNormalizationBindGroup, DeviceError> {
        unsafe {
            let Some(ref state) = &self.state else {
                return Ok(TimestampNormalizationBindGroup { raw: None });
            };

            if !buffer_usages.contains(wgt::BufferUsages::QUERY_RESOLVE) {
                return Ok(TimestampNormalizationBindGroup { raw: None });
            }

            // If this buffer is large enough that we wouldn't be able to bind the entire thing
            // at once to normalize the timestamps, we can't use it. We force the buffer to fail
            // to allocate. The lowest max binding size is 128MB, and query sets must be small
            // (no more than 4096), so this should never be hit in practice by sane programs.
            if buffer_size.get() > device.adapter.limits().max_storage_buffer_binding_size {
                return Err(DeviceError::OutOfMemory);
            }

            let bg_label_alloc;
            let label = match buffer_label {
                Some(label) => {
                    bg_label_alloc = alloc::format!("Timestamp normalization bind group ({label})");
                    &*bg_label_alloc
                }
                None => "Timestamp normalization bind group",
            };

            let bg = device
                .raw()
                .create_bind_group(&hal::BindGroupDescriptor {
                    label: hal_label(Some(label), device.instance_flags),
                    layout: &*state.temporary_bind_group_layout,
                    buffers: &[hal::BufferBinding::new_unchecked(buffer, 0, buffer_size)],
                    samplers: &[],
                    textures: &[],
                    acceleration_structures: &[],
                    external_textures: &[],
                    entries: &[hal::BindGroupEntry {
                        binding: 0,
                        resource_index: 0,
                        count: 1,
                    }],
                })
                .map_err(|e| device.handle_hal_error(e))?;

            Ok(TimestampNormalizationBindGroup { raw: Some(bg) })
        }
    }

    pub fn normalize(
        &self,
        snatch_guard: &SnatchGuard<'_>,
        encoder: &mut dyn hal::DynCommandEncoder,
        tracker: &mut BufferTracker,
        bind_group: &TimestampNormalizationBindGroup,
        buffer: &Arc<Buffer>,
        buffer_offset_bytes: u64,
        total_timestamps: u32,
    ) {
        let Some(ref state) = &self.state else {
            return;
        };

        let Some(bind_group) = bind_group.raw.as_deref() else {
            return;
        };

        let buffer_offset_timestamps: u32 = (buffer_offset_bytes / 8).try_into().unwrap(); // Unreachable as MAX_QUERIES is way less than u32::MAX

        let pending_barrier = tracker.set_single(buffer, wgt::BufferUses::STORAGE_READ_WRITE);

        let barrier = pending_barrier.map(|pending| pending.into_hal(buffer, snatch_guard));

        let needed_workgroups = total_timestamps.div_ceil(64);

        unsafe {
            encoder.transition_buffers(barrier.as_slice());
            encoder.begin_compute_pass(&hal::ComputePassDescriptor {
                label: hal_label(
                    Some("(wgpu internal) Timestamp normalization pass"),
                    buffer.device.instance_flags,
                ),
                timestamp_writes: None,
            });
            encoder.set_compute_pipeline(&*state.pipeline);
            encoder.set_bind_group(&*state.pipeline_layout, 0, bind_group, &[]);
            encoder.set_immediates(
                &*state.pipeline_layout,
                0,
                &[buffer_offset_timestamps, total_timestamps],
            );
            encoder.dispatch([needed_workgroups, 1, 1]);
            encoder.end_compute_pass();
        }
    }

    pub fn dispose(self, device: &dyn hal::DynDevice) {
        unsafe {
            let Some(state) = self.state else {
                return;
            };

            device.destroy_compute_pipeline(state.pipeline);
            device.destroy_pipeline_layout(state.pipeline_layout);
            device.destroy_bind_group_layout(state.temporary_bind_group_layout);
        }
    }

    pub fn enabled(&self) -> bool {
        self.state.is_some()
    }
}

#[derive(Debug)]
pub struct TimestampNormalizationBindGroup {
    raw: Option<Box<dyn hal::DynBindGroup>>,
}

impl TimestampNormalizationBindGroup {
    pub fn dispose(self, device: &dyn hal::DynDevice) {
        unsafe {
            if let Some(raw) = self.raw {
                device.destroy_bind_group(raw);
            }
        }
    }
}

fn compute_timestamp_period(input: f32) -> (u32, u32) {
    let pow2 = input.log2().ceil() as i32;
    let clamped_pow2 = pow2.clamp(-32, 32).unsigned_abs();
    let shift = 32 - clamped_pow2;

    let denominator = (1u64 << shift) as f64;

    // float -> int conversions are defined to saturate.
    let multiplier = (input as f64 * denominator).round() as u32;

    (multiplier, shift)
}

#[cfg(test)]
mod tests {
    use core::f64;

    fn assert_timestamp_case(input: f32) {
        let (multiplier, shift) = super::compute_timestamp_period(input);

        let output = multiplier as f64 / (1u64 << shift) as f64;

        assert!((input as f64 - output).abs() < 0.0000001);
    }

    #[test]
    fn compute_timestamp_period() {
        assert_timestamp_case(0.01);
        assert_timestamp_case(0.5);
        assert_timestamp_case(1.0);
        assert_timestamp_case(2.0);
        assert_timestamp_case(2.7);
        assert_timestamp_case(1000.7);
    }
}
