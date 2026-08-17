use super::{
    utils::{BufferBarrierScratch, BufferBarriers, UniqueIndexExt as _, UniqueIndexScratch},
    CreateIndirectValidationPipelineError,
};
use crate::{
    command::RenderPassErrorInner,
    device::{queue::TempResource, Device, DeviceError},
    hal_label,
    lock::{rank, Mutex},
    pipeline::{CreateComputePipelineError, CreateShaderModuleError},
    resource::{RawResourceAccess as _, StagingBuffer, Trackable},
    snatch::SnatchGuard,
    track::TrackerIndex,
    FastHashMap,
};
use alloc::{boxed::Box, string::ToString, sync::Arc, vec, vec::Vec};
use core::{mem::size_of, num::NonZeroU64};
use wgt::Limits;

/// Note: This needs to be under:
///
/// default max_compute_workgroups_per_dimension * size_of::<wgt::DrawIndirectArgs>() * `workgroup_size` used by the shader
///
/// = (2^16 - 1) * 2^4 * 2^6
///
/// It is currently set to:
///
/// = (2^16 - 1) * 2^4
///
/// This is enough space for:
///
/// - 65535 [`wgt::DrawIndirectArgs`] / [`MetadataEntry`]
/// - 52428 [`wgt::DrawIndexedIndirectArgs`]
const BUFFER_SIZE: wgt::BufferSize = wgt::BufferSize::new(1_048_560).unwrap();

/// Holds all device-level resources that are needed to validate indirect draws.
///
/// This machinery requires the following limits:
///
/// - max_bind_groups: 3,
/// - max_dynamic_storage_buffers_per_pipeline_layout: 1,
/// - max_storage_buffers_per_shader_stage: 3,
/// - max_immediate_size: 8,
///
/// These are all indirectly satisfied by `DownlevelFlags::INDIRECT_EXECUTION`, which is also
/// required for this module's functionality to work.
#[derive(Debug)]
pub(crate) struct Draw {
    module: Box<dyn hal::DynShaderModule>,
    metadata_bind_group_layout: Box<dyn hal::DynBindGroupLayout>,
    src_bind_group_layout: Box<dyn hal::DynBindGroupLayout>,
    dst_bind_group_layout: Box<dyn hal::DynBindGroupLayout>,
    pipeline_layout: Box<dyn hal::DynPipelineLayout>,
    pipeline: Box<dyn hal::DynComputePipeline>,

    free_indirect_entries: Mutex<Vec<BufferPoolEntry>>,
    free_metadata_entries: Mutex<Vec<BufferPoolEntry>>,
}

impl Draw {
    pub(super) fn new(
        device: &dyn hal::DynDevice,
        required_features: &wgt::Features,
        instance_flags: wgt::InstanceFlags,
        backend: wgt::Backend,
    ) -> Result<Self, CreateIndirectValidationPipelineError> {
        let module = create_validation_module(device, instance_flags)?;

        let metadata_bind_group_layout = create_bind_group_layout(
            device,
            true,
            false,
            BUFFER_SIZE,
            hal_label(
                Some("(wgpu internal) Indirect draw validation metadata bind group layout"),
                instance_flags,
            ),
        )?;
        let src_bind_group_layout = create_bind_group_layout(
            device,
            true,
            true,
            wgt::BufferSize::new(4 * 4).unwrap(),
            hal_label(
                Some("(wgpu internal) Indirect draw validation source bind group layout"),
                instance_flags,
            ),
        )?;
        let dst_bind_group_layout = create_bind_group_layout(
            device,
            false,
            false,
            BUFFER_SIZE,
            hal_label(
                Some("(wgpu internal) Indirect draw validation destination bind group layout"),
                instance_flags,
            ),
        )?;

        let pipeline_layout_desc = hal::PipelineLayoutDescriptor {
            label: hal_label(
                Some("(wgpu internal) Indirect draw validation pipeline layout"),
                instance_flags,
            ),
            flags: hal::PipelineLayoutFlags::empty(),
            bind_group_layouts: &[
                Some(metadata_bind_group_layout.as_ref()),
                Some(src_bind_group_layout.as_ref()),
                Some(dst_bind_group_layout.as_ref()),
            ],
            immediate_size: 8,
        };
        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&pipeline_layout_desc)
                .map_err(DeviceError::from_hal)?
        };

        let supports_indirect_first_instance =
            required_features.contains(wgt::Features::INDIRECT_FIRST_INSTANCE);
        let write_d3d12_special_constants = backend == wgt::Backend::Dx12;
        let pipeline = create_validation_pipeline(
            device,
            module.as_ref(),
            pipeline_layout.as_ref(),
            supports_indirect_first_instance,
            write_d3d12_special_constants,
            instance_flags,
        )?;

        Ok(Self {
            module,
            metadata_bind_group_layout,
            src_bind_group_layout,
            dst_bind_group_layout,
            pipeline_layout,
            pipeline,

            free_indirect_entries: Mutex::new(rank::BUFFER_POOL, Vec::new()),
            free_metadata_entries: Mutex::new(rank::BUFFER_POOL, Vec::new()),
        })
    }

    /// `Ok(None)` will only be returned if `buffer_size` is `0`.
    pub(super) fn create_src_bind_group(
        &self,
        device: &dyn hal::DynDevice,
        limits: &Limits,
        buffer_size: u64,
        buffer: &dyn hal::DynBuffer,
        instance_flags: wgt::InstanceFlags,
    ) -> Result<Option<Box<dyn hal::DynBindGroup>>, DeviceError> {
        let binding_size = calculate_src_buffer_binding_size(buffer_size, limits);
        let Some(binding_size) = NonZeroU64::new(binding_size) else {
            return Ok(None);
        };
        let hal_desc = hal::BindGroupDescriptor {
            label: hal_label(
                Some("(wgpu internal) Indirect draw validation source bind group"),
                instance_flags,
            ),
            layout: self.src_bind_group_layout.as_ref(),
            entries: &[hal::BindGroupEntry {
                binding: 0,
                resource_index: 0,
                count: 1,
            }],
            // SAFETY: We calculated the binding size to fit within the buffer.
            buffers: &[hal::BufferBinding::new_unchecked(buffer, 0, binding_size)],
            samplers: &[],
            textures: &[],
            acceleration_structures: &[],
            external_textures: &[],
        };
        unsafe {
            device
                .create_bind_group(&hal_desc)
                .map(Some)
                .map_err(DeviceError::from_hal)
        }
    }

    fn acquire_dst_entry(
        &self,
        device: &dyn hal::DynDevice,
        instance_flags: wgt::InstanceFlags,
    ) -> Result<BufferPoolEntry, hal::DeviceError> {
        let mut free_buffers = self.free_indirect_entries.lock();
        match free_buffers.pop() {
            Some(buffer) => Ok(buffer),
            None => {
                let usage = wgt::BufferUses::INDIRECT | wgt::BufferUses::STORAGE_READ_WRITE;
                create_buffer_and_bind_group(
                    device,
                    usage,
                    self.dst_bind_group_layout.as_ref(),
                    hal_label(Some("(wgpu internal) Indirect draw validation destination buffer"), instance_flags),
                    hal_label(Some("(wgpu internal) Indirect draw validation destination bind group layout"), instance_flags),
                )
            }
        }
    }

    fn release_dst_entries(&self, entries: impl Iterator<Item = BufferPoolEntry>) {
        self.free_indirect_entries.lock().extend(entries);
    }

    fn acquire_metadata_entry(
        &self,
        device: &dyn hal::DynDevice,
        instance_flags: wgt::InstanceFlags,
    ) -> Result<BufferPoolEntry, hal::DeviceError> {
        let mut free_buffers = self.free_metadata_entries.lock();
        match free_buffers.pop() {
            Some(buffer) => Ok(buffer),
            None => {
                let usage = wgt::BufferUses::COPY_DST | wgt::BufferUses::STORAGE_READ_ONLY;
                create_buffer_and_bind_group(
                    device,
                    usage,
                    self.metadata_bind_group_layout.as_ref(),
                    hal_label(
                        Some("(wgpu internal) Indirect draw validation metadata buffer"),
                        instance_flags,
                    ),
                    hal_label(
                        Some("(wgpu internal) Indirect draw validation metadata bind group layout"),
                        instance_flags,
                    ),
                )
            }
        }
    }

    fn release_metadata_entries(&self, entries: impl Iterator<Item = BufferPoolEntry>) {
        self.free_metadata_entries.lock().extend(entries);
    }

    /// Injects a compute pass that will validate all indirect draws in the current render pass.
    pub(crate) fn inject_validation_pass(
        &self,
        device: &Arc<Device>,
        snatch_guard: &SnatchGuard,
        resources: &mut DrawResources,
        temp_resources: &mut Vec<TempResource>,
        encoder: &mut dyn hal::DynCommandEncoder,
        batcher: DrawBatcher,
    ) -> Result<(), RenderPassErrorInner> {
        let mut batches = batcher.batches;

        if batches.is_empty() {
            return Ok(());
        }

        let max_staging_buffer_size = 1 << 26; // ~67MiB

        let mut staging_buffers = Vec::new();

        let mut current_size = 0;
        for batch in batches.values_mut() {
            let data = batch.metadata();
            let offset = if current_size + data.len() > max_staging_buffer_size {
                let staging_buffer =
                    StagingBuffer::new(device, NonZeroU64::new(current_size as u64).unwrap())?;
                staging_buffers.push(staging_buffer);
                current_size = data.len();
                0
            } else {
                let offset = current_size;
                current_size += data.len();
                offset as u64
            };
            batch.staging_buffer_index = staging_buffers.len();
            batch.staging_buffer_offset = offset;
        }
        if current_size != 0 {
            let staging_buffer =
                StagingBuffer::new(device, NonZeroU64::new(current_size as u64).unwrap())?;
            staging_buffers.push(staging_buffer);
        }

        for batch in batches.values() {
            let data = batch.metadata();
            let staging_buffer = &mut staging_buffers[batch.staging_buffer_index];
            unsafe {
                staging_buffer.write_with_offset(
                    data,
                    0,
                    batch.staging_buffer_offset as isize,
                    data.len(),
                )
            };
        }

        let staging_buffers: Vec<_> = staging_buffers
            .into_iter()
            .map(|buffer| buffer.flush())
            .collect();

        let mut current_metadata_entry = None;
        for batch in batches.values_mut() {
            let data = batch.metadata();
            let (metadata_resource_index, metadata_buffer_offset) =
                resources.get_metadata_subrange(data.len() as u64, &mut current_metadata_entry)?;
            batch.metadata_resource_index = metadata_resource_index;
            batch.metadata_buffer_offset = metadata_buffer_offset;
        }

        let buffer_barrier_scratch = &mut BufferBarrierScratch::new();
        let unique_index_scratch = &mut UniqueIndexScratch::new();

        BufferBarriers::new(buffer_barrier_scratch)
            .extend(
                batches
                    .values()
                    .map(|batch| batch.staging_buffer_index)
                    .unique(unique_index_scratch)
                    .map(|index| hal::BufferBarrier {
                        buffer: staging_buffers[index].raw(),
                        usage: hal::StateTransition {
                            from: wgt::BufferUses::MAP_WRITE,
                            to: wgt::BufferUses::COPY_SRC,
                        },
                    }),
            )
            .extend(
                batches
                    .values()
                    .map(|batch| batch.metadata_resource_index)
                    .unique(unique_index_scratch)
                    .map(|index| hal::BufferBarrier {
                        buffer: resources.get_metadata_buffer(index),
                        usage: hal::StateTransition {
                            from: wgt::BufferUses::STORAGE_READ_ONLY,
                            to: wgt::BufferUses::COPY_DST,
                        },
                    }),
            )
            .encode(encoder);

        for batch in batches.values() {
            let data = batch.metadata();
            let data_size = NonZeroU64::new(data.len() as u64).unwrap();

            let staging_buffer = &staging_buffers[batch.staging_buffer_index];

            let metadata_buffer = resources.get_metadata_buffer(batch.metadata_resource_index);

            unsafe {
                encoder.copy_buffer_to_buffer(
                    staging_buffer.raw(),
                    metadata_buffer,
                    &[hal::BufferCopy {
                        src_offset: batch.staging_buffer_offset,
                        dst_offset: batch.metadata_buffer_offset,
                        size: data_size,
                    }],
                );
            }
        }

        for staging_buffer in staging_buffers {
            temp_resources.push(TempResource::StagingBuffer(staging_buffer));
        }

        BufferBarriers::new(buffer_barrier_scratch)
            .extend(
                batches
                    .values()
                    .map(|batch| batch.metadata_resource_index)
                    .unique(unique_index_scratch)
                    .map(|index| hal::BufferBarrier {
                        buffer: resources.get_metadata_buffer(index),
                        usage: hal::StateTransition {
                            from: wgt::BufferUses::COPY_DST,
                            to: wgt::BufferUses::STORAGE_READ_ONLY,
                        },
                    }),
            )
            .extend(
                batches
                    .values()
                    .map(|batch| batch.dst_resource_index)
                    .unique(unique_index_scratch)
                    .map(|index| hal::BufferBarrier {
                        buffer: resources.get_dst_buffer(index),
                        usage: hal::StateTransition {
                            from: wgt::BufferUses::INDIRECT,
                            to: wgt::BufferUses::STORAGE_READ_WRITE,
                        },
                    }),
            )
            .encode(encoder);

        let desc = hal::ComputePassDescriptor {
            label: hal_label(
                Some("(wgpu internal) Indirect draw validation pass"),
                device.instance_flags,
            ),
            timestamp_writes: None,
        };
        unsafe {
            encoder.begin_compute_pass(&desc);
        }
        unsafe {
            encoder.set_compute_pipeline(self.pipeline.as_ref());
        }

        for batch in batches.values() {
            let pipeline_layout = self.pipeline_layout.as_ref();

            let metadata_start =
                (batch.metadata_buffer_offset / size_of::<MetadataEntry>() as u64) as u32;
            let metadata_count = batch.entries.len() as u32;
            unsafe {
                encoder.set_immediates(pipeline_layout, 0, &[metadata_start, metadata_count]);
            }

            let metadata_bind_group =
                resources.get_metadata_bind_group(batch.metadata_resource_index);
            unsafe {
                encoder.set_bind_group(pipeline_layout, 0, metadata_bind_group, &[]);
            }

            // Make sure the indirect buffer is still valid.
            batch.src_buffer.try_raw(snatch_guard)?;

            let src_bind_group = batch
                .src_buffer
                .indirect_validation_bind_groups
                .get(snatch_guard)
                .unwrap()
                .draw
                .as_ref();
            unsafe {
                encoder.set_bind_group(
                    pipeline_layout,
                    1,
                    src_bind_group,
                    &[batch.src_dynamic_offset as u32],
                );
            }

            let dst_bind_group = resources.get_dst_bind_group(batch.dst_resource_index);
            unsafe {
                encoder.set_bind_group(pipeline_layout, 2, dst_bind_group, &[]);
            }

            unsafe {
                encoder.dispatch([(batch.entries.len() as u32).div_ceil(64), 1, 1]);
            }
        }

        unsafe {
            encoder.end_compute_pass();
        }

        BufferBarriers::new(buffer_barrier_scratch)
            .extend(
                batches
                    .values()
                    .map(|batch| batch.dst_resource_index)
                    .unique(unique_index_scratch)
                    .map(|index| hal::BufferBarrier {
                        buffer: resources.get_dst_buffer(index),
                        usage: hal::StateTransition {
                            from: wgt::BufferUses::STORAGE_READ_WRITE,
                            to: wgt::BufferUses::INDIRECT,
                        },
                    }),
            )
            .encode(encoder);

        Ok(())
    }

    pub(super) fn dispose(self, device: &dyn hal::DynDevice) {
        let Draw {
            module,
            metadata_bind_group_layout,
            src_bind_group_layout,
            dst_bind_group_layout,
            pipeline_layout,
            pipeline,

            free_indirect_entries,
            free_metadata_entries,
        } = self;

        for entry in free_indirect_entries.into_inner().drain(..) {
            unsafe {
                device.destroy_bind_group(entry.bind_group);
                device.destroy_buffer(entry.buffer);
            }
        }

        for entry in free_metadata_entries.into_inner().drain(..) {
            unsafe {
                device.destroy_bind_group(entry.bind_group);
                device.destroy_buffer(entry.buffer);
            }
        }

        unsafe {
            device.destroy_compute_pipeline(pipeline);
            device.destroy_pipeline_layout(pipeline_layout);
            device.destroy_bind_group_layout(metadata_bind_group_layout);
            device.destroy_bind_group_layout(src_bind_group_layout);
            device.destroy_bind_group_layout(dst_bind_group_layout);
            device.destroy_shader_module(module);
        }
    }
}

fn create_validation_module(
    device: &dyn hal::DynDevice,
    instance_flags: wgt::InstanceFlags,
) -> Result<Box<dyn hal::DynShaderModule>, CreateIndirectValidationPipelineError> {
    let src = include_str!("./validate_draw.wgsl");

    #[cfg(feature = "wgsl")]
    let module = naga::front::wgsl::parse_str(src).map_err(|inner| {
        CreateShaderModuleError::Parsing(naga::error::ShaderError {
            source: src.to_string(),
            label: None,
            inner: Box::new(inner),
        })
    })?;
    #[cfg(not(feature = "wgsl"))]
    #[allow(clippy::diverging_sub_expression)]
    let module = panic!("Indirect validation requires the wgsl feature flag to be enabled!");

    let info = crate::device::create_validator(
        wgt::Features::IMMEDIATES,
        wgt::DownlevelFlags::empty(),
        naga::valid::ValidationFlags::all(),
    )
    .validate(&module)
    .map_err(|inner| {
        CreateShaderModuleError::Validation(naga::error::ShaderError {
            source: src.to_string(),
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
            Some("(wgpu internal) Indirect draw validation shader module"),
            instance_flags,
        ),
        runtime_checks: wgt::ShaderRuntimeChecks::unchecked(),
    };
    let module = unsafe { device.create_shader_module(&hal_desc, hal_shader) }.map_err(
        |error| match error {
            hal::ShaderError::Device(error) => {
                CreateShaderModuleError::Device(DeviceError::from_hal(error))
            }
            hal::ShaderError::Compilation(ref msg) => {
                log::error!("Shader error: {msg}");
                CreateShaderModuleError::Generation
            }
        },
    )?;

    Ok(module)
}

fn create_validation_pipeline(
    device: &dyn hal::DynDevice,
    module: &dyn hal::DynShaderModule,
    pipeline_layout: &dyn hal::DynPipelineLayout,
    supports_indirect_first_instance: bool,
    write_d3d12_special_constants: bool,
    instance_flags: wgt::InstanceFlags,
) -> Result<Box<dyn hal::DynComputePipeline>, CreateIndirectValidationPipelineError> {
    let pipeline_desc = hal::ComputePipelineDescriptor {
        label: hal_label(
            Some("(wgpu internal) Indirect draw validation pipeline"),
            instance_flags,
        ),
        layout: pipeline_layout,
        stage: hal::ProgrammableStage {
            module,
            entry_point: "main",
            constants: &hashbrown::HashMap::from([
                (
                    "supports_indirect_first_instance".to_string(),
                    f64::from(supports_indirect_first_instance),
                ),
                (
                    "write_d3d12_special_constants".to_string(),
                    f64::from(write_d3d12_special_constants),
                ),
            ]),
            zero_initialize_workgroup_memory: false,
        },
        cache: None,
    };
    let pipeline =
        unsafe { device.create_compute_pipeline(&pipeline_desc) }.map_err(|err| match err {
            hal::PipelineError::Device(error) => {
                CreateComputePipelineError::Device(DeviceError::from_hal(error))
            }
            hal::PipelineError::Linkage(_stages, msg) => CreateComputePipelineError::Internal(msg),
            hal::PipelineError::EntryPoint(_stage) => CreateComputePipelineError::Internal(
                crate::device::ENTRYPOINT_FAILURE_ERROR.to_string(),
            ),
            hal::PipelineError::PipelineConstants(_, error) => {
                CreateComputePipelineError::PipelineConstants(error)
            }
        })?;

    Ok(pipeline)
}

fn create_bind_group_layout(
    device: &dyn hal::DynDevice,
    read_only: bool,
    has_dynamic_offset: bool,
    min_binding_size: wgt::BufferSize,
    label: Option<&'static str>,
) -> Result<Box<dyn hal::DynBindGroupLayout>, CreateIndirectValidationPipelineError> {
    let bind_group_layout_desc = hal::BindGroupLayoutDescriptor {
        label,
        flags: hal::BindGroupLayoutFlags::empty(),
        entries: &[wgt::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgt::ShaderStages::COMPUTE,
            ty: wgt::BindingType::Buffer {
                ty: wgt::BufferBindingType::Storage { read_only },
                has_dynamic_offset,
                min_binding_size: Some(min_binding_size),
            },
            count: None,
        }],
    };
    let bind_group_layout = unsafe {
        device
            .create_bind_group_layout(&bind_group_layout_desc)
            .map_err(DeviceError::from_hal)?
    };

    Ok(bind_group_layout)
}

/// Returns the largest binding size that when combined with dynamic offsets can address the whole buffer.
fn calculate_src_buffer_binding_size(buffer_size: u64, limits: &Limits) -> u64 {
    let max_storage_buffer_binding_size = limits.max_storage_buffer_binding_size;
    let min_storage_buffer_offset_alignment = limits.min_storage_buffer_offset_alignment as u64;

    if buffer_size <= max_storage_buffer_binding_size {
        buffer_size
    } else {
        let buffer_rem = buffer_size % min_storage_buffer_offset_alignment;
        let binding_rem = max_storage_buffer_binding_size % min_storage_buffer_offset_alignment;

        // Can the buffer remainder fit in the binding remainder?
        // If so, align max binding size and add buffer remainder
        if buffer_rem <= binding_rem {
            max_storage_buffer_binding_size - binding_rem + buffer_rem
        }
        // If not, align max binding size, shorten it by a chunk and add buffer remainder
        else {
            max_storage_buffer_binding_size - binding_rem - min_storage_buffer_offset_alignment
                + buffer_rem
        }
    }
}

/// Splits the given `offset` into a dynamic offset & offset.
fn calculate_src_offsets(buffer_size: u64, limits: &Limits, offset: u64) -> (u64, u64) {
    let binding_size = calculate_src_buffer_binding_size(buffer_size, limits);

    let min_storage_buffer_offset_alignment = limits.min_storage_buffer_offset_alignment as u64;

    let chunk_adjustment = match min_storage_buffer_offset_alignment {
        // No need to adjust since the src_offset is 4 byte aligned.
        4 => 0,
        // With 16/20 bytes of data we can straddle up to 2 8 byte boundaries:
        //  - 16 bytes of data: (4|8|4)
        //  - 20 bytes of data: (4|8|8, 8|8|4)
        8 => 2,
        // With 16/20 bytes of data we can straddle up to 1 16+ byte boundary:
        //  - 16 bytes of data: (4|12, 8|8, 12|4)
        //  - 20 bytes of data: (4|16, 8|12, 12|8, 16|4)
        16.. => 1,
        _ => unreachable!(),
    };

    let chunks = binding_size / min_storage_buffer_offset_alignment;
    let dynamic_offset_stride =
        chunks.saturating_sub(chunk_adjustment) * min_storage_buffer_offset_alignment;

    if dynamic_offset_stride == 0 {
        return (0, offset);
    }

    let max_dynamic_offset = buffer_size - binding_size;
    let max_dynamic_offset_index = max_dynamic_offset / dynamic_offset_stride;

    let src_dynamic_offset_index = offset / dynamic_offset_stride;

    let src_dynamic_offset =
        src_dynamic_offset_index.min(max_dynamic_offset_index) * dynamic_offset_stride;
    let src_offset = offset - src_dynamic_offset;

    (src_dynamic_offset, src_offset)
}

#[derive(Debug)]
struct BufferPoolEntry {
    buffer: Box<dyn hal::DynBuffer>,
    bind_group: Box<dyn hal::DynBindGroup>,
}

fn create_buffer_and_bind_group(
    device: &dyn hal::DynDevice,
    usage: wgt::BufferUses,
    bind_group_layout: &dyn hal::DynBindGroupLayout,
    buffer_label: Option<&'static str>,
    bind_group_label: Option<&'static str>,
) -> Result<BufferPoolEntry, hal::DeviceError> {
    let buffer_desc = hal::BufferDescriptor {
        label: buffer_label,
        size: BUFFER_SIZE.get(),
        usage,
        memory_flags: hal::MemoryFlags::empty(),
    };
    let buffer = unsafe { device.create_buffer(&buffer_desc) }?;
    let bind_group_desc = hal::BindGroupDescriptor {
        label: bind_group_label,
        layout: bind_group_layout,
        entries: &[hal::BindGroupEntry {
            binding: 0,
            resource_index: 0,
            count: 1,
        }],
        // SAFETY: We just created the buffer with this size.
        buffers: &[hal::BufferBinding::new_unchecked(
            buffer.as_ref(),
            0,
            BUFFER_SIZE,
        )],
        samplers: &[],
        textures: &[],
        acceleration_structures: &[],
        external_textures: &[],
    };
    let bind_group = unsafe { device.create_bind_group(&bind_group_desc) }?;
    Ok(BufferPoolEntry { buffer, bind_group })
}

#[derive(Clone)]
struct CurrentEntry {
    index: usize,
    offset: u64,
}

/// Holds all command buffer-level resources that are needed to validate indirect draws.
pub(crate) struct DrawResources {
    device: Arc<Device>,
    dst_entries: Vec<BufferPoolEntry>,
    metadata_entries: Vec<BufferPoolEntry>,
}

impl Drop for DrawResources {
    fn drop(&mut self) {
        if let Some(ref indirect_validation) = self.device.indirect_validation {
            let indirect_draw_validation = &indirect_validation.draw;
            indirect_draw_validation.release_dst_entries(self.dst_entries.drain(..));
            indirect_draw_validation.release_metadata_entries(self.metadata_entries.drain(..));
        }
    }
}

impl DrawResources {
    pub(crate) fn new(device: Arc<Device>) -> Self {
        DrawResources {
            device,
            dst_entries: Vec::new(),
            metadata_entries: Vec::new(),
        }
    }

    pub(crate) fn get_dst_buffer(&self, index: usize) -> &dyn hal::DynBuffer {
        self.dst_entries.get(index).unwrap().buffer.as_ref()
    }

    fn get_dst_bind_group(&self, index: usize) -> &dyn hal::DynBindGroup {
        self.dst_entries.get(index).unwrap().bind_group.as_ref()
    }

    fn get_metadata_buffer(&self, index: usize) -> &dyn hal::DynBuffer {
        self.metadata_entries.get(index).unwrap().buffer.as_ref()
    }

    fn get_metadata_bind_group(&self, index: usize) -> &dyn hal::DynBindGroup {
        self.metadata_entries
            .get(index)
            .unwrap()
            .bind_group
            .as_ref()
    }

    fn get_dst_subrange(
        &mut self,
        size: u64,
        current_entry: &mut Option<CurrentEntry>,
    ) -> Result<(usize, u64), DeviceError> {
        let indirect_draw_validation = &self.device.indirect_validation.as_ref().unwrap().draw;
        let ensure_entry = |index: usize| {
            if self.dst_entries.len() <= index {
                let entry = indirect_draw_validation
                    .acquire_dst_entry(self.device.raw(), self.device.instance_flags)?;
                self.dst_entries.push(entry);
            }
            Ok(())
        };
        let entry_data = Self::get_subrange_impl(ensure_entry, current_entry, size)?;
        Ok((entry_data.index, entry_data.offset))
    }

    fn get_metadata_subrange(
        &mut self,
        size: u64,
        current_entry: &mut Option<CurrentEntry>,
    ) -> Result<(usize, u64), DeviceError> {
        let indirect_draw_validation = &self.device.indirect_validation.as_ref().unwrap().draw;
        let ensure_entry = |index: usize| {
            if self.metadata_entries.len() <= index {
                let entry = indirect_draw_validation
                    .acquire_metadata_entry(self.device.raw(), self.device.instance_flags)?;
                self.metadata_entries.push(entry);
            }
            Ok(())
        };
        let entry_data = Self::get_subrange_impl(ensure_entry, current_entry, size)?;
        Ok((entry_data.index, entry_data.offset))
    }

    fn get_subrange_impl(
        ensure_entry: impl FnOnce(usize) -> Result<(), hal::DeviceError>,
        current_entry: &mut Option<CurrentEntry>,
        size: u64,
    ) -> Result<CurrentEntry, DeviceError> {
        let index = if let Some(current_entry) = current_entry.as_mut() {
            if current_entry.offset + size <= BUFFER_SIZE.get() {
                let entry_data = current_entry.clone();
                current_entry.offset += size;
                return Ok(entry_data);
            } else {
                current_entry.index + 1
            }
        } else {
            0
        };

        ensure_entry(index).map_err(DeviceError::from_hal)?;

        let entry_data = CurrentEntry { index, offset: 0 };

        *current_entry = Some(CurrentEntry {
            index,
            offset: size,
        });

        Ok(entry_data)
    }
}

/// This must match the `MetadataEntry` struct used by the shader.
#[repr(C)]
struct MetadataEntry {
    src_offset: u32,
    dst_offset: u32,
    vertex_or_index_limit: u32,
    instance_limit: u32,
}

impl MetadataEntry {
    fn new(
        indexed: bool,
        src_offset: u64,
        dst_offset: u64,
        vertex_or_index_limit: u64,
        instance_limit: u64,
    ) -> Self {
        const U32_MAX_AS_U64: u64 = u32::MAX as u64;

        // NOTE: buffer sizes should never exceed `u32::MAX`.
        assert!(src_offset <= U32_MAX_AS_U64);
        assert!(dst_offset <= U32_MAX_AS_U64);

        let src_offset = src_offset as u32;
        let src_offset = src_offset / 4; // translate byte offset to offset in u32's

        // `src_offset` needs at most 30 bits,
        // pack `indexed` in bit 31 of `src_offset`
        let src_offset = src_offset | ((indexed as u32) << 31);

        // max value for limits since first_X and X_count indirect draw arguments are u32
        let max_limit = U32_MAX_AS_U64 + U32_MAX_AS_U64; // 1 11111111 11111111 11111111 11111110

        let vertex_or_index_limit = vertex_or_index_limit.min(max_limit);
        let vertex_or_index_limit_bit_32 = (vertex_or_index_limit >> 32) as u32; // extract bit 32
        let vertex_or_index_limit = vertex_or_index_limit as u32; // truncate the limit to a u32

        let instance_limit = instance_limit.min(max_limit);
        let instance_limit_bit_32 = (instance_limit >> 32) as u32; // extract bit 32
        let instance_limit = instance_limit as u32; // truncate the limit to a u32

        let dst_offset = dst_offset as u32;
        let dst_offset = dst_offset / 4; // translate byte offset to offset in u32's

        // `dst_offset` needs at most 30 bits,
        // pack `vertex_or_index_limit_bit_32` in bit 30 of `dst_offset` and
        // pack `instance_limit_bit_32` in bit 31 of `dst_offset`
        let dst_offset =
            dst_offset | (vertex_or_index_limit_bit_32 << 30) | (instance_limit_bit_32 << 31);

        Self {
            src_offset,
            dst_offset,
            vertex_or_index_limit,
            instance_limit,
        }
    }
}

struct DrawIndirectValidationBatch {
    src_buffer: Arc<crate::resource::Buffer>,
    src_dynamic_offset: u64,
    dst_resource_index: usize,
    entries: Vec<MetadataEntry>,

    staging_buffer_index: usize,
    staging_buffer_offset: u64,
    metadata_resource_index: usize,
    metadata_buffer_offset: u64,
}

impl DrawIndirectValidationBatch {
    /// Data to be written to the metadata buffer.
    fn metadata(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.entries.as_ptr().cast::<u8>(),
                self.entries.len() * size_of::<MetadataEntry>(),
            )
        }
    }
}

/// Accumulates all needed data needed to validate indirect draws.
pub(crate) struct DrawBatcher {
    batches: FastHashMap<(TrackerIndex, u64, usize), DrawIndirectValidationBatch>,
    current_dst_entry: Option<CurrentEntry>,
}

impl DrawBatcher {
    pub(crate) fn new() -> Self {
        Self {
            batches: FastHashMap::default(),
            current_dst_entry: None,
        }
    }

    /// Add an indirect draw to be validated.
    ///
    /// Returns the index of the indirect buffer in `indirect_draw_validation_resources`
    /// and the offset to be used for the draw.
    pub(crate) fn add<'a>(
        &mut self,
        indirect_draw_validation_resources: &'a mut DrawResources,
        device: &Device,
        src_buffer: &Arc<crate::resource::Buffer>,
        offset: u64,
        family: crate::command::DrawCommandFamily,
        vertex_or_index_limit: u64,
        instance_limit: u64,
    ) -> Result<(usize, u64), DeviceError> {
        let stride = crate::command::get_dst_stride_of_indirect_args(device.backend(), family);

        let (dst_resource_index, dst_offset) = indirect_draw_validation_resources
            .get_dst_subrange(stride, &mut self.current_dst_entry)?;

        let buffer_size = src_buffer.size;
        let limits = device.adapter.limits();
        let (src_dynamic_offset, src_offset) = calculate_src_offsets(buffer_size, &limits, offset);

        let src_buffer_tracker_index = src_buffer.tracker_index();

        let entry = MetadataEntry::new(
            family == crate::command::DrawCommandFamily::DrawIndexed,
            src_offset,
            dst_offset,
            vertex_or_index_limit,
            instance_limit,
        );

        match self.batches.entry((
            src_buffer_tracker_index,
            src_dynamic_offset,
            dst_resource_index,
        )) {
            hashbrown::hash_map::Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().entries.push(entry)
            }
            hashbrown::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(DrawIndirectValidationBatch {
                    src_buffer: src_buffer.clone(),
                    src_dynamic_offset,
                    dst_resource_index,
                    entries: vec![entry],

                    // these will be initialized once we accumulated all entries for the batch
                    staging_buffer_index: 0,
                    staging_buffer_offset: 0,
                    metadata_resource_index: 0,
                    metadata_buffer_offset: 0,
                });
            }
        }

        Ok((dst_resource_index, dst_offset))
    }
}
