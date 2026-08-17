use alloc::{borrow::ToOwned as _, boxed::Box, vec::Vec};

use crate::{
    AccelerationStructureBuildSizes, AccelerationStructureDescriptor, Api, BindGroupDescriptor,
    BindGroupLayoutDescriptor, BufferDescriptor, BufferMapping, CommandEncoderDescriptor,
    ComputePipelineDescriptor, Device, DeviceError, FenceValue,
    GetAccelerationStructureBuildSizesDescriptor, Label, MemoryRange, PipelineCacheDescriptor,
    PipelineCacheError, PipelineError, PipelineLayoutDescriptor, RenderPipelineDescriptor,
    SamplerDescriptor, ShaderError, ShaderInput, ShaderModuleDescriptor, TextureDescriptor,
    TextureViewDescriptor, TlasInstance,
};

use super::{
    DynAccelerationStructure, DynBindGroup, DynBindGroupLayout, DynBuffer, DynCommandEncoder,
    DynComputePipeline, DynFence, DynPipelineCache, DynPipelineLayout, DynQuerySet, DynQueue,
    DynRenderPipeline, DynResource, DynResourceExt as _, DynSampler, DynShaderModule, DynTexture,
    DynTextureView,
};

pub trait DynDevice: DynResource {
    unsafe fn create_buffer(
        &self,
        desc: &BufferDescriptor,
    ) -> Result<Box<dyn DynBuffer>, DeviceError>;

    unsafe fn destroy_buffer(&self, buffer: Box<dyn DynBuffer>);
    unsafe fn add_raw_buffer(&self, buffer: &dyn DynBuffer);

    unsafe fn map_buffer(
        &self,
        buffer: &dyn DynBuffer,
        range: MemoryRange,
    ) -> Result<BufferMapping, DeviceError>;

    unsafe fn unmap_buffer(&self, buffer: &dyn DynBuffer);

    unsafe fn flush_mapped_ranges(&self, buffer: &dyn DynBuffer, ranges: &[MemoryRange]);
    unsafe fn invalidate_mapped_ranges(&self, buffer: &dyn DynBuffer, ranges: &[MemoryRange]);

    unsafe fn create_texture(
        &self,
        desc: &TextureDescriptor,
    ) -> Result<Box<dyn DynTexture>, DeviceError>;
    unsafe fn destroy_texture(&self, texture: Box<dyn DynTexture>);
    unsafe fn add_raw_texture(&self, texture: &dyn DynTexture);

    unsafe fn create_texture_view(
        &self,
        texture: &dyn DynTexture,
        desc: &TextureViewDescriptor,
    ) -> Result<Box<dyn DynTextureView>, DeviceError>;
    unsafe fn destroy_texture_view(&self, view: Box<dyn DynTextureView>);
    unsafe fn create_sampler(
        &self,
        desc: &SamplerDescriptor,
    ) -> Result<Box<dyn DynSampler>, DeviceError>;
    unsafe fn destroy_sampler(&self, sampler: Box<dyn DynSampler>);

    unsafe fn create_command_encoder(
        &self,
        desc: &CommandEncoderDescriptor<dyn DynQueue>,
    ) -> Result<Box<dyn DynCommandEncoder>, DeviceError>;

    unsafe fn create_bind_group_layout(
        &self,
        desc: &BindGroupLayoutDescriptor,
    ) -> Result<Box<dyn DynBindGroupLayout>, DeviceError>;
    unsafe fn destroy_bind_group_layout(&self, bg_layout: Box<dyn DynBindGroupLayout>);

    unsafe fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDescriptor<dyn DynBindGroupLayout>,
    ) -> Result<Box<dyn DynPipelineLayout>, DeviceError>;
    unsafe fn destroy_pipeline_layout(&self, pipeline_layout: Box<dyn DynPipelineLayout>);

    unsafe fn create_bind_group(
        &self,
        desc: &BindGroupDescriptor<
            dyn DynBindGroupLayout,
            dyn DynBuffer,
            dyn DynSampler,
            dyn DynTextureView,
            dyn DynAccelerationStructure,
        >,
    ) -> Result<Box<dyn DynBindGroup>, DeviceError>;
    unsafe fn destroy_bind_group(&self, group: Box<dyn DynBindGroup>);

    unsafe fn create_shader_module(
        &self,
        desc: &ShaderModuleDescriptor,
        shader: ShaderInput,
    ) -> Result<Box<dyn DynShaderModule>, ShaderError>;
    unsafe fn destroy_shader_module(&self, module: Box<dyn DynShaderModule>);

    unsafe fn create_render_pipeline(
        &self,
        desc: &RenderPipelineDescriptor<
            dyn DynPipelineLayout,
            dyn DynShaderModule,
            dyn DynPipelineCache,
        >,
    ) -> Result<Box<dyn DynRenderPipeline>, PipelineError>;
    unsafe fn destroy_render_pipeline(&self, pipeline: Box<dyn DynRenderPipeline>);

    unsafe fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDescriptor<
            dyn DynPipelineLayout,
            dyn DynShaderModule,
            dyn DynPipelineCache,
        >,
    ) -> Result<Box<dyn DynComputePipeline>, PipelineError>;
    unsafe fn destroy_compute_pipeline(&self, pipeline: Box<dyn DynComputePipeline>);

    unsafe fn create_pipeline_cache(
        &self,
        desc: &PipelineCacheDescriptor<'_>,
    ) -> Result<Box<dyn DynPipelineCache>, PipelineCacheError>;
    fn pipeline_cache_validation_key(&self) -> Option<[u8; 16]> {
        None
    }
    unsafe fn destroy_pipeline_cache(&self, cache: Box<dyn DynPipelineCache>);

    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<Label>,
    ) -> Result<Box<dyn DynQuerySet>, DeviceError>;
    unsafe fn destroy_query_set(&self, set: Box<dyn DynQuerySet>);

    unsafe fn create_fence(&self) -> Result<Box<dyn DynFence>, DeviceError>;
    unsafe fn destroy_fence(&self, fence: Box<dyn DynFence>);
    unsafe fn get_fence_value(&self, fence: &dyn DynFence) -> Result<FenceValue, DeviceError>;

    unsafe fn wait(
        &self,
        fence: &dyn DynFence,
        value: FenceValue,
        timeout: Option<core::time::Duration>,
    ) -> Result<bool, DeviceError>;

    unsafe fn start_graphics_debugger_capture(&self) -> bool;
    unsafe fn stop_graphics_debugger_capture(&self);

    unsafe fn pipeline_cache_get_data(&self, cache: &dyn DynPipelineCache) -> Option<Vec<u8>>;

    unsafe fn create_acceleration_structure(
        &self,
        desc: &AccelerationStructureDescriptor,
    ) -> Result<Box<dyn DynAccelerationStructure>, DeviceError>;
    unsafe fn get_acceleration_structure_build_sizes(
        &self,
        desc: &GetAccelerationStructureBuildSizesDescriptor<dyn DynBuffer>,
    ) -> AccelerationStructureBuildSizes;
    unsafe fn get_acceleration_structure_device_address(
        &self,
        acceleration_structure: &dyn DynAccelerationStructure,
    ) -> wgt::BufferAddress;
    unsafe fn destroy_acceleration_structure(
        &self,
        acceleration_structure: Box<dyn DynAccelerationStructure>,
    );
    fn tlas_instance_to_bytes(&self, instance: TlasInstance) -> Vec<u8>;

    fn get_internal_counters(&self) -> wgt::HalCounters;
    fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport>;

    fn check_if_oom(&self) -> Result<(), DeviceError>;
}

impl<D: Device + DynResource> DynDevice for D {
    unsafe fn create_buffer(
        &self,
        desc: &BufferDescriptor,
    ) -> Result<Box<dyn DynBuffer>, DeviceError> {
        unsafe { D::create_buffer(self, desc) }.map(|b| -> Box<dyn DynBuffer> { Box::new(b) })
    }

    unsafe fn destroy_buffer(&self, buffer: Box<dyn DynBuffer>) {
        unsafe { D::destroy_buffer(self, buffer.unbox()) };
    }
    unsafe fn add_raw_buffer(&self, buffer: &dyn DynBuffer) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { D::add_raw_buffer(self, buffer) };
    }

    unsafe fn map_buffer(
        &self,
        buffer: &dyn DynBuffer,
        range: MemoryRange,
    ) -> Result<BufferMapping, DeviceError> {
        let buffer = buffer.expect_downcast_ref();
        unsafe { D::map_buffer(self, buffer, range) }
    }

    unsafe fn unmap_buffer(&self, buffer: &dyn DynBuffer) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { D::unmap_buffer(self, buffer) }
    }

    unsafe fn flush_mapped_ranges(&self, buffer: &dyn DynBuffer, ranges: &[MemoryRange]) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { D::flush_mapped_ranges(self, buffer, ranges.iter().cloned()) }
    }

    unsafe fn invalidate_mapped_ranges(&self, buffer: &dyn DynBuffer, ranges: &[MemoryRange]) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { D::invalidate_mapped_ranges(self, buffer, ranges.iter().cloned()) }
    }

    unsafe fn create_texture(
        &self,
        desc: &TextureDescriptor,
    ) -> Result<Box<dyn DynTexture>, DeviceError> {
        unsafe { D::create_texture(self, desc) }.map(|b| {
            let boxed_texture: Box<<D::A as Api>::Texture> = Box::new(b);
            let boxed_texture: Box<dyn DynTexture> = boxed_texture;
            boxed_texture
        })
    }

    unsafe fn destroy_texture(&self, texture: Box<dyn DynTexture>) {
        unsafe { D::destroy_texture(self, texture.unbox()) };
    }

    unsafe fn add_raw_texture(&self, texture: &dyn DynTexture) {
        let texture = texture.expect_downcast_ref();
        unsafe { D::add_raw_texture(self, texture) };
    }

    unsafe fn create_texture_view(
        &self,
        texture: &dyn DynTexture,
        desc: &TextureViewDescriptor,
    ) -> Result<Box<dyn DynTextureView>, DeviceError> {
        let texture = texture.expect_downcast_ref();
        unsafe { D::create_texture_view(self, texture, desc) }.map(|b| {
            let boxed_texture_view: Box<<D::A as Api>::TextureView> = Box::new(b);
            let boxed_texture_view: Box<dyn DynTextureView> = boxed_texture_view;
            boxed_texture_view
        })
    }

    unsafe fn destroy_texture_view(&self, view: Box<dyn DynTextureView>) {
        unsafe { D::destroy_texture_view(self, view.unbox()) };
    }

    unsafe fn create_sampler(
        &self,
        desc: &SamplerDescriptor,
    ) -> Result<Box<dyn DynSampler>, DeviceError> {
        unsafe { D::create_sampler(self, desc) }.map(|b| {
            let boxed_sampler: Box<<D::A as Api>::Sampler> = Box::new(b);
            let boxed_sampler: Box<dyn DynSampler> = boxed_sampler;
            boxed_sampler
        })
    }

    unsafe fn destroy_sampler(&self, sampler: Box<dyn DynSampler>) {
        unsafe { D::destroy_sampler(self, sampler.unbox()) };
    }

    unsafe fn create_command_encoder(
        &self,
        desc: &CommandEncoderDescriptor<'_, dyn DynQueue>,
    ) -> Result<Box<dyn DynCommandEncoder>, DeviceError> {
        let desc = CommandEncoderDescriptor {
            label: desc.label,
            queue: desc.queue.expect_downcast_ref(),
        };
        unsafe { D::create_command_encoder(self, &desc) }
            .map(|b| -> Box<dyn DynCommandEncoder> { Box::new(b) })
    }

    unsafe fn create_bind_group_layout(
        &self,
        desc: &BindGroupLayoutDescriptor,
    ) -> Result<Box<dyn DynBindGroupLayout>, DeviceError> {
        unsafe { D::create_bind_group_layout(self, desc) }
            .map(|b| -> Box<dyn DynBindGroupLayout> { Box::new(b) })
    }

    unsafe fn destroy_bind_group_layout(&self, bg_layout: Box<dyn DynBindGroupLayout>) {
        unsafe { D::destroy_bind_group_layout(self, bg_layout.unbox()) };
    }

    unsafe fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDescriptor<dyn DynBindGroupLayout>,
    ) -> Result<Box<dyn DynPipelineLayout>, DeviceError> {
        let bind_group_layouts: Vec<_> = desc
            .bind_group_layouts
            .iter()
            .map(|bgl| bgl.map(|bgl| bgl.expect_downcast_ref()))
            .collect();
        let desc = PipelineLayoutDescriptor {
            label: desc.label,
            bind_group_layouts: &bind_group_layouts,
            immediate_size: desc.immediate_size,
            flags: desc.flags,
        };

        unsafe { D::create_pipeline_layout(self, &desc) }
            .map(|b| -> Box<dyn DynPipelineLayout> { Box::new(b) })
    }

    unsafe fn destroy_pipeline_layout(&self, pipeline_layout: Box<dyn DynPipelineLayout>) {
        unsafe { D::destroy_pipeline_layout(self, pipeline_layout.unbox()) };
    }

    unsafe fn create_bind_group(
        &self,
        desc: &BindGroupDescriptor<
            dyn DynBindGroupLayout,
            dyn DynBuffer,
            dyn DynSampler,
            dyn DynTextureView,
            dyn DynAccelerationStructure,
        >,
    ) -> Result<Box<dyn DynBindGroup>, DeviceError> {
        let buffers: Vec<_> = desc
            .buffers
            .iter()
            .map(|b| b.clone().expect_downcast())
            .collect();
        let samplers: Vec<_> = desc
            .samplers
            .iter()
            .map(|s| s.expect_downcast_ref())
            .collect();
        let textures: Vec<_> = desc
            .textures
            .iter()
            .map(|t| t.clone().expect_downcast())
            .collect();
        let acceleration_structures: Vec<_> = desc
            .acceleration_structures
            .iter()
            .map(|a| a.expect_downcast_ref())
            .collect();
        let external_textures: Vec<_> = desc
            .external_textures
            .iter()
            .map(|et| et.clone().expect_downcast())
            .collect();

        let desc = BindGroupDescriptor {
            label: desc.label.to_owned(),
            layout: desc.layout.expect_downcast_ref(),
            buffers: &buffers,
            samplers: &samplers,
            textures: &textures,
            entries: desc.entries,
            acceleration_structures: &acceleration_structures,
            external_textures: &external_textures,
        };

        unsafe { D::create_bind_group(self, &desc) }
            .map(|b| -> Box<dyn DynBindGroup> { Box::new(b) })
    }

    unsafe fn destroy_bind_group(&self, group: Box<dyn DynBindGroup>) {
        unsafe { D::destroy_bind_group(self, group.unbox()) };
    }

    unsafe fn create_shader_module(
        &self,
        desc: &ShaderModuleDescriptor,
        shader: ShaderInput,
    ) -> Result<Box<dyn DynShaderModule>, ShaderError> {
        unsafe { D::create_shader_module(self, desc, shader) }
            .map(|b| -> Box<dyn DynShaderModule> { Box::new(b) })
    }

    unsafe fn destroy_shader_module(&self, module: Box<dyn DynShaderModule>) {
        unsafe { D::destroy_shader_module(self, module.unbox()) };
    }

    unsafe fn create_render_pipeline(
        &self,
        desc: &RenderPipelineDescriptor<
            dyn DynPipelineLayout,
            dyn DynShaderModule,
            dyn DynPipelineCache,
        >,
    ) -> Result<Box<dyn DynRenderPipeline>, PipelineError> {
        let desc = RenderPipelineDescriptor {
            label: desc.label,
            layout: desc.layout.expect_downcast_ref(),
            vertex_processor: match &desc.vertex_processor {
                crate::VertexProcessor::Standard {
                    vertex_buffers,
                    vertex_stage,
                } => crate::VertexProcessor::Standard {
                    vertex_buffers,
                    vertex_stage: vertex_stage.clone().expect_downcast(),
                },
                crate::VertexProcessor::Mesh {
                    task_stage: task,
                    mesh_stage: mesh,
                } => crate::VertexProcessor::Mesh {
                    task_stage: task.as_ref().map(|a| a.clone().expect_downcast()),
                    mesh_stage: mesh.clone().expect_downcast(),
                },
            },
            primitive: desc.primitive,
            depth_stencil: desc.depth_stencil.clone(),
            multisample: desc.multisample,
            fragment_stage: desc.fragment_stage.clone().map(|f| f.expect_downcast()),
            color_targets: desc.color_targets,
            multiview_mask: desc.multiview_mask,
            cache: desc.cache.map(|c| c.expect_downcast_ref()),
        };

        unsafe { D::create_render_pipeline(self, &desc) }
            .map(|b| -> Box<dyn DynRenderPipeline> { Box::new(b) })
    }

    unsafe fn destroy_render_pipeline(&self, pipeline: Box<dyn DynRenderPipeline>) {
        unsafe { D::destroy_render_pipeline(self, pipeline.unbox()) };
    }

    unsafe fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDescriptor<
            dyn DynPipelineLayout,
            dyn DynShaderModule,
            dyn DynPipelineCache,
        >,
    ) -> Result<Box<dyn DynComputePipeline>, PipelineError> {
        let desc = ComputePipelineDescriptor {
            label: desc.label,
            layout: desc.layout.expect_downcast_ref(),
            stage: desc.stage.clone().expect_downcast(),
            cache: desc.cache.as_ref().map(|c| c.expect_downcast_ref()),
        };

        unsafe { D::create_compute_pipeline(self, &desc) }
            .map(|b| -> Box<dyn DynComputePipeline> { Box::new(b) })
    }

    unsafe fn destroy_compute_pipeline(&self, pipeline: Box<dyn DynComputePipeline>) {
        unsafe { D::destroy_compute_pipeline(self, pipeline.unbox()) };
    }

    unsafe fn create_pipeline_cache(
        &self,
        desc: &PipelineCacheDescriptor<'_>,
    ) -> Result<Box<dyn DynPipelineCache>, PipelineCacheError> {
        unsafe { D::create_pipeline_cache(self, desc) }
            .map(|b| -> Box<dyn DynPipelineCache> { Box::new(b) })
    }

    fn pipeline_cache_validation_key(&self) -> Option<[u8; 16]> {
        D::pipeline_cache_validation_key(self)
    }

    unsafe fn destroy_pipeline_cache(&self, pipeline_cache: Box<dyn DynPipelineCache>) {
        unsafe { D::destroy_pipeline_cache(self, pipeline_cache.unbox()) };
    }

    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<Label>,
    ) -> Result<Box<dyn DynQuerySet>, DeviceError> {
        unsafe { D::create_query_set(self, desc) }.map(|b| -> Box<dyn DynQuerySet> { Box::new(b) })
    }

    unsafe fn destroy_query_set(&self, query_set: Box<dyn DynQuerySet>) {
        unsafe { D::destroy_query_set(self, query_set.unbox()) };
    }

    unsafe fn create_fence(&self) -> Result<Box<dyn DynFence>, DeviceError> {
        unsafe { D::create_fence(self) }.map(|b| -> Box<dyn DynFence> { Box::new(b) })
    }

    unsafe fn destroy_fence(&self, fence: Box<dyn DynFence>) {
        unsafe { D::destroy_fence(self, fence.unbox()) };
    }

    unsafe fn get_fence_value(&self, fence: &dyn DynFence) -> Result<FenceValue, DeviceError> {
        let fence = fence.expect_downcast_ref();
        unsafe { D::get_fence_value(self, fence) }
    }

    unsafe fn wait(
        &self,
        fence: &dyn DynFence,
        value: FenceValue,
        timeout: Option<core::time::Duration>,
    ) -> Result<bool, DeviceError> {
        let fence = fence.expect_downcast_ref();
        unsafe { D::wait(self, fence, value, timeout) }
    }

    unsafe fn start_graphics_debugger_capture(&self) -> bool {
        unsafe { D::start_graphics_debugger_capture(self) }
    }

    unsafe fn stop_graphics_debugger_capture(&self) {
        unsafe { D::stop_graphics_debugger_capture(self) }
    }

    unsafe fn pipeline_cache_get_data(&self, cache: &dyn DynPipelineCache) -> Option<Vec<u8>> {
        let cache = cache.expect_downcast_ref();
        unsafe { D::pipeline_cache_get_data(self, cache) }
    }

    unsafe fn create_acceleration_structure(
        &self,
        desc: &AccelerationStructureDescriptor,
    ) -> Result<Box<dyn DynAccelerationStructure>, DeviceError> {
        unsafe { D::create_acceleration_structure(self, desc) }
            .map(|b| -> Box<dyn DynAccelerationStructure> { Box::new(b) })
    }

    unsafe fn get_acceleration_structure_build_sizes(
        &self,
        desc: &GetAccelerationStructureBuildSizesDescriptor<dyn DynBuffer>,
    ) -> AccelerationStructureBuildSizes {
        let entries = desc.entries.expect_downcast();
        let desc = GetAccelerationStructureBuildSizesDescriptor {
            entries: &entries,
            flags: desc.flags,
        };
        unsafe { D::get_acceleration_structure_build_sizes(self, &desc) }
    }

    unsafe fn get_acceleration_structure_device_address(
        &self,
        acceleration_structure: &dyn DynAccelerationStructure,
    ) -> wgt::BufferAddress {
        let acceleration_structure = acceleration_structure.expect_downcast_ref();
        unsafe { D::get_acceleration_structure_device_address(self, acceleration_structure) }
    }

    unsafe fn destroy_acceleration_structure(
        &self,
        acceleration_structure: Box<dyn DynAccelerationStructure>,
    ) {
        unsafe { D::destroy_acceleration_structure(self, acceleration_structure.unbox()) }
    }

    fn tlas_instance_to_bytes(&self, instance: TlasInstance) -> Vec<u8> {
        D::tlas_instance_to_bytes(self, instance)
    }

    fn get_internal_counters(&self) -> wgt::HalCounters {
        D::get_internal_counters(self)
    }

    fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
        D::generate_allocator_report(self)
    }

    fn check_if_oom(&self) -> Result<(), DeviceError> {
        D::check_if_oom(self)
    }
}
