use alloc::{boxed::Box, vec::Vec};
use core::ops::Range;

use crate::{
    AccelerationStructureBarrier, Api, Attachment, BufferBarrier, BufferBinding, BufferCopy,
    BufferTextureCopy, BuildAccelerationStructureDescriptor, ColorAttachment, CommandEncoder,
    ComputePassDescriptor, DepthStencilAttachment, DeviceError, Label, MemoryRange,
    PassTimestampWrites, Rect, RenderPassDescriptor, TextureBarrier, TextureCopy,
};

use super::{
    DynAccelerationStructure, DynBindGroup, DynBuffer, DynCommandBuffer, DynComputePipeline,
    DynPipelineLayout, DynQuerySet, DynRenderPipeline, DynResource, DynResourceExt as _,
    DynTexture, DynTextureView,
};

pub trait DynCommandEncoder: DynResource + core::fmt::Debug {
    unsafe fn begin_encoding(&mut self, label: Label) -> Result<(), DeviceError>;

    unsafe fn discard_encoding(&mut self);

    unsafe fn end_encoding(&mut self) -> Result<Box<dyn DynCommandBuffer>, DeviceError>;

    unsafe fn reset_all(&mut self, command_buffers: Vec<Box<dyn DynCommandBuffer>>);

    unsafe fn transition_buffers(&mut self, barriers: &[BufferBarrier<'_, dyn DynBuffer>]);
    unsafe fn transition_textures(&mut self, barriers: &[TextureBarrier<'_, dyn DynTexture>]);

    unsafe fn clear_buffer(&mut self, buffer: &dyn DynBuffer, range: MemoryRange);

    unsafe fn copy_buffer_to_buffer(
        &mut self,
        src: &dyn DynBuffer,
        dst: &dyn DynBuffer,
        regions: &[BufferCopy],
    );

    unsafe fn copy_texture_to_texture(
        &mut self,
        src: &dyn DynTexture,
        src_usage: wgt::TextureUses,
        dst: &dyn DynTexture,
        regions: &[TextureCopy],
    );

    unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &dyn DynBuffer,
        dst: &dyn DynTexture,
        regions: &[BufferTextureCopy],
    );

    unsafe fn copy_texture_to_buffer(
        &mut self,
        src: &dyn DynTexture,
        src_usage: wgt::TextureUses,
        dst: &dyn DynBuffer,
        regions: &[BufferTextureCopy],
    );

    unsafe fn set_bind_group(
        &mut self,
        layout: &dyn DynPipelineLayout,
        index: u32,
        group: &dyn DynBindGroup,
        dynamic_offsets: &[wgt::DynamicOffset],
    );

    unsafe fn set_immediates(
        &mut self,
        layout: &dyn DynPipelineLayout,
        offset_bytes: u32,
        data: &[u32],
    );

    unsafe fn insert_debug_marker(&mut self, label: &str);
    unsafe fn begin_debug_marker(&mut self, group_label: &str);
    unsafe fn end_debug_marker(&mut self);

    unsafe fn begin_query(&mut self, set: &dyn DynQuerySet, index: u32);
    unsafe fn end_query(&mut self, set: &dyn DynQuerySet, index: u32);
    unsafe fn write_timestamp(&mut self, set: &dyn DynQuerySet, index: u32);
    unsafe fn reset_queries(&mut self, set: &dyn DynQuerySet, range: Range<u32>);
    unsafe fn copy_query_results(
        &mut self,
        set: &dyn DynQuerySet,
        range: Range<u32>,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        stride: wgt::BufferSize,
    );

    unsafe fn begin_render_pass(
        &mut self,
        desc: &RenderPassDescriptor<dyn DynQuerySet, dyn DynTextureView>,
    ) -> Result<(), DeviceError>;
    unsafe fn end_render_pass(&mut self);

    unsafe fn set_render_pipeline(&mut self, pipeline: &dyn DynRenderPipeline);

    unsafe fn set_index_buffer<'a>(
        &mut self,
        binding: BufferBinding<'a, dyn DynBuffer>,
        format: wgt::IndexFormat,
    );

    unsafe fn set_vertex_buffer<'a>(
        &mut self,
        index: u32,
        binding: BufferBinding<'a, dyn DynBuffer>,
    );
    unsafe fn set_viewport(&mut self, rect: &Rect<f32>, depth_range: Range<f32>);
    unsafe fn set_scissor_rect(&mut self, rect: &Rect<u32>);
    unsafe fn set_stencil_reference(&mut self, value: u32);
    unsafe fn set_blend_constants(&mut self, color: &[f32; 4]);

    unsafe fn draw(
        &mut self,
        first_vertex: u32,
        vertex_count: u32,
        first_instance: u32,
        instance_count: u32,
    );
    unsafe fn draw_indexed(
        &mut self,
        first_index: u32,
        index_count: u32,
        base_vertex: i32,
        first_instance: u32,
        instance_count: u32,
    );
    unsafe fn draw_mesh_tasks(
        &mut self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );
    unsafe fn draw_indirect(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    );
    unsafe fn draw_indexed_indirect(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    );
    unsafe fn draw_mesh_tasks_indirect(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    );
    unsafe fn draw_indirect_count(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        count_buffer: &dyn DynBuffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    );
    unsafe fn draw_indexed_indirect_count(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        count_buffer: &dyn DynBuffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    );
    unsafe fn draw_mesh_tasks_indirect_count(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        count_buffer: &dyn DynBuffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    );

    unsafe fn begin_compute_pass(&mut self, desc: &ComputePassDescriptor<dyn DynQuerySet>);
    unsafe fn end_compute_pass(&mut self);

    unsafe fn set_compute_pipeline(&mut self, pipeline: &dyn DynComputePipeline);

    unsafe fn dispatch(&mut self, count: [u32; 3]);
    unsafe fn dispatch_indirect(&mut self, buffer: &dyn DynBuffer, offset: wgt::BufferAddress);

    unsafe fn build_acceleration_structures<'a>(
        &mut self,
        descriptors: &'a [BuildAccelerationStructureDescriptor<
            'a,
            dyn DynBuffer,
            dyn DynAccelerationStructure,
        >],
    );
    unsafe fn place_acceleration_structure_barrier(
        &mut self,
        barrier: AccelerationStructureBarrier,
    );
    unsafe fn copy_acceleration_structure_to_acceleration_structure(
        &mut self,
        src: &dyn DynAccelerationStructure,
        dst: &dyn DynAccelerationStructure,
        copy: wgt::AccelerationStructureCopy,
    );
    unsafe fn read_acceleration_structure_compact_size(
        &mut self,
        acceleration_structure: &dyn DynAccelerationStructure,
        buf: &dyn DynBuffer,
    );
    unsafe fn set_acceleration_structure_dependencies(
        &self,
        command_buffers: &[Box<dyn DynCommandBuffer>],
        dependencies: &[&dyn DynAccelerationStructure],
    );
}

impl<C: CommandEncoder + DynResource> DynCommandEncoder for C {
    unsafe fn begin_encoding(&mut self, label: Label) -> Result<(), DeviceError> {
        unsafe { C::begin_encoding(self, label) }
    }

    unsafe fn discard_encoding(&mut self) {
        unsafe { C::discard_encoding(self) }
    }

    unsafe fn end_encoding(&mut self) -> Result<Box<dyn DynCommandBuffer>, DeviceError> {
        unsafe { C::end_encoding(self) }.map(|cb| {
            let boxed_command_buffer: Box<<C::A as Api>::CommandBuffer> = Box::new(cb);
            let boxed_command_buffer: Box<dyn DynCommandBuffer> = boxed_command_buffer;
            boxed_command_buffer
        })
    }

    unsafe fn reset_all(&mut self, command_buffers: Vec<Box<dyn DynCommandBuffer>>) {
        unsafe { C::reset_all(self, command_buffers.into_iter().map(|cb| cb.unbox())) }
    }

    unsafe fn transition_buffers(&mut self, barriers: &[BufferBarrier<'_, dyn DynBuffer>]) {
        let barriers = barriers.iter().map(|barrier| BufferBarrier {
            buffer: barrier.buffer.expect_downcast_ref(),
            usage: barrier.usage.clone(),
        });
        unsafe { self.transition_buffers(barriers) };
    }

    unsafe fn transition_textures(&mut self, barriers: &[TextureBarrier<'_, dyn DynTexture>]) {
        let barriers = barriers.iter().map(|barrier| TextureBarrier {
            texture: barrier.texture.expect_downcast_ref(),
            usage: barrier.usage.clone(),
            range: barrier.range,
        });
        unsafe { self.transition_textures(barriers) };
    }

    unsafe fn clear_buffer(&mut self, buffer: &dyn DynBuffer, range: MemoryRange) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { C::clear_buffer(self, buffer, range) };
    }

    unsafe fn copy_buffer_to_buffer(
        &mut self,
        src: &dyn DynBuffer,
        dst: &dyn DynBuffer,
        regions: &[BufferCopy],
    ) {
        let src = src.expect_downcast_ref();
        let dst = dst.expect_downcast_ref();
        unsafe {
            C::copy_buffer_to_buffer(self, src, dst, regions.iter().copied());
        }
    }

    unsafe fn copy_texture_to_texture(
        &mut self,
        src: &dyn DynTexture,
        src_usage: wgt::TextureUses,
        dst: &dyn DynTexture,
        regions: &[TextureCopy],
    ) {
        let src = src.expect_downcast_ref();
        let dst = dst.expect_downcast_ref();
        unsafe {
            C::copy_texture_to_texture(self, src, src_usage, dst, regions.iter().cloned());
        }
    }

    unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &dyn DynBuffer,
        dst: &dyn DynTexture,
        regions: &[BufferTextureCopy],
    ) {
        let src = src.expect_downcast_ref();
        let dst = dst.expect_downcast_ref();
        unsafe {
            C::copy_buffer_to_texture(self, src, dst, regions.iter().cloned());
        }
    }

    unsafe fn copy_texture_to_buffer(
        &mut self,
        src: &dyn DynTexture,
        src_usage: wgt::TextureUses,
        dst: &dyn DynBuffer,
        regions: &[BufferTextureCopy],
    ) {
        let src = src.expect_downcast_ref();
        let dst = dst.expect_downcast_ref();
        unsafe {
            C::copy_texture_to_buffer(self, src, src_usage, dst, regions.iter().cloned());
        }
    }

    unsafe fn set_bind_group(
        &mut self,
        layout: &dyn DynPipelineLayout,
        index: u32,
        group: &dyn DynBindGroup,
        dynamic_offsets: &[wgt::DynamicOffset],
    ) {
        let layout = layout.expect_downcast_ref();
        let group = group.expect_downcast_ref();
        unsafe { C::set_bind_group(self, layout, index, group, dynamic_offsets) };
    }

    unsafe fn set_immediates(
        &mut self,
        layout: &dyn DynPipelineLayout,
        offset_bytes: u32,
        data: &[u32],
    ) {
        let layout = layout.expect_downcast_ref();
        unsafe { C::set_immediates(self, layout, offset_bytes, data) };
    }

    unsafe fn insert_debug_marker(&mut self, label: &str) {
        unsafe {
            C::insert_debug_marker(self, label);
        }
    }

    unsafe fn begin_debug_marker(&mut self, group_label: &str) {
        unsafe {
            C::begin_debug_marker(self, group_label);
        }
    }

    unsafe fn end_debug_marker(&mut self) {
        unsafe {
            C::end_debug_marker(self);
        }
    }

    unsafe fn begin_query(&mut self, set: &dyn DynQuerySet, index: u32) {
        let set = set.expect_downcast_ref();
        unsafe { C::begin_query(self, set, index) };
    }

    unsafe fn end_query(&mut self, set: &dyn DynQuerySet, index: u32) {
        let set = set.expect_downcast_ref();
        unsafe { C::end_query(self, set, index) };
    }

    unsafe fn write_timestamp(&mut self, set: &dyn DynQuerySet, index: u32) {
        let set = set.expect_downcast_ref();
        unsafe { C::write_timestamp(self, set, index) };
    }

    unsafe fn reset_queries(&mut self, set: &dyn DynQuerySet, range: Range<u32>) {
        let set = set.expect_downcast_ref();
        unsafe { C::reset_queries(self, set, range) };
    }

    unsafe fn copy_query_results(
        &mut self,
        set: &dyn DynQuerySet,
        range: Range<u32>,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        stride: wgt::BufferSize,
    ) {
        let set = set.expect_downcast_ref();
        let buffer = buffer.expect_downcast_ref();
        unsafe { C::copy_query_results(self, set, range, buffer, offset, stride) };
    }

    unsafe fn begin_render_pass(
        &mut self,
        desc: &RenderPassDescriptor<dyn DynQuerySet, dyn DynTextureView>,
    ) -> Result<(), DeviceError> {
        let color_attachments = desc
            .color_attachments
            .iter()
            .map(|attachment| {
                attachment
                    .as_ref()
                    .map(|attachment| attachment.expect_downcast())
            })
            .collect::<Vec<_>>();

        let desc: RenderPassDescriptor<<C::A as Api>::QuerySet, <C::A as Api>::TextureView> =
            RenderPassDescriptor {
                label: desc.label,
                extent: desc.extent,
                sample_count: desc.sample_count,
                color_attachments: &color_attachments,
                depth_stencil_attachment: desc
                    .depth_stencil_attachment
                    .as_ref()
                    .map(|ds| ds.expect_downcast()),
                multiview_mask: desc.multiview_mask,
                timestamp_writes: desc
                    .timestamp_writes
                    .as_ref()
                    .map(|writes| writes.expect_downcast()),
                occlusion_query_set: desc
                    .occlusion_query_set
                    .map(|set| set.expect_downcast_ref()),
            };
        unsafe { C::begin_render_pass(self, &desc) }
    }

    unsafe fn end_render_pass(&mut self) {
        unsafe {
            C::end_render_pass(self);
        }
    }

    unsafe fn set_viewport(&mut self, rect: &Rect<f32>, depth_range: Range<f32>) {
        unsafe {
            C::set_viewport(self, rect, depth_range);
        }
    }

    unsafe fn set_scissor_rect(&mut self, rect: &Rect<u32>) {
        unsafe {
            C::set_scissor_rect(self, rect);
        }
    }

    unsafe fn set_stencil_reference(&mut self, value: u32) {
        unsafe {
            C::set_stencil_reference(self, value);
        }
    }

    unsafe fn set_blend_constants(&mut self, color: &[f32; 4]) {
        unsafe { C::set_blend_constants(self, color) };
    }

    unsafe fn draw(
        &mut self,
        first_vertex: u32,
        vertex_count: u32,
        first_instance: u32,
        instance_count: u32,
    ) {
        unsafe {
            C::draw(
                self,
                first_vertex,
                vertex_count,
                first_instance,
                instance_count,
            )
        };
    }

    unsafe fn draw_indexed(
        &mut self,
        first_index: u32,
        index_count: u32,
        base_vertex: i32,
        first_instance: u32,
        instance_count: u32,
    ) {
        unsafe {
            C::draw_indexed(
                self,
                first_index,
                index_count,
                base_vertex,
                first_instance,
                instance_count,
            )
        };
    }

    unsafe fn draw_mesh_tasks(
        &mut self,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe { C::draw_mesh_tasks(self, group_count_x, group_count_y, group_count_z) };
    }

    unsafe fn draw_indirect(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { C::draw_indirect(self, buffer, offset, draw_count) };
    }

    unsafe fn draw_indexed_indirect(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { C::draw_indexed_indirect(self, buffer, offset, draw_count) };
    }

    unsafe fn draw_mesh_tasks_indirect(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { C::draw_mesh_tasks_indirect(self, buffer, offset, draw_count) };
    }

    unsafe fn draw_indirect_count(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        count_buffer: &dyn DynBuffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        let buffer = buffer.expect_downcast_ref();
        let count_buffer = count_buffer.expect_downcast_ref();
        unsafe {
            C::draw_indirect_count(self, buffer, offset, count_buffer, count_offset, max_count)
        };
    }

    unsafe fn draw_indexed_indirect_count(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        count_buffer: &dyn DynBuffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        let buffer = buffer.expect_downcast_ref();
        let count_buffer = count_buffer.expect_downcast_ref();
        unsafe {
            C::draw_indexed_indirect_count(
                self,
                buffer,
                offset,
                count_buffer,
                count_offset,
                max_count,
            )
        };
    }

    unsafe fn draw_mesh_tasks_indirect_count(
        &mut self,
        buffer: &dyn DynBuffer,
        offset: wgt::BufferAddress,
        count_buffer: &dyn DynBuffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        let buffer = buffer.expect_downcast_ref();
        let count_buffer = count_buffer.expect_downcast_ref();
        unsafe {
            C::draw_mesh_tasks_indirect_count(
                self,
                buffer,
                offset,
                count_buffer,
                count_offset,
                max_count,
            )
        };
    }

    unsafe fn begin_compute_pass(&mut self, desc: &ComputePassDescriptor<dyn DynQuerySet>) {
        let desc = ComputePassDescriptor {
            label: desc.label,
            timestamp_writes: desc
                .timestamp_writes
                .as_ref()
                .map(|writes| writes.expect_downcast()),
        };
        unsafe { C::begin_compute_pass(self, &desc) };
    }

    unsafe fn end_compute_pass(&mut self) {
        unsafe { C::end_compute_pass(self) };
    }

    unsafe fn set_compute_pipeline(&mut self, pipeline: &dyn DynComputePipeline) {
        let pipeline = pipeline.expect_downcast_ref();
        unsafe { C::set_compute_pipeline(self, pipeline) };
    }

    unsafe fn dispatch(&mut self, count: [u32; 3]) {
        unsafe { C::dispatch(self, count) };
    }

    unsafe fn dispatch_indirect(&mut self, buffer: &dyn DynBuffer, offset: wgt::BufferAddress) {
        let buffer = buffer.expect_downcast_ref();
        unsafe { C::dispatch_indirect(self, buffer, offset) };
    }

    unsafe fn set_render_pipeline(&mut self, pipeline: &dyn DynRenderPipeline) {
        let pipeline = pipeline.expect_downcast_ref();
        unsafe { C::set_render_pipeline(self, pipeline) };
    }

    unsafe fn set_index_buffer<'a>(
        &mut self,
        binding: BufferBinding<'a, dyn DynBuffer>,
        format: wgt::IndexFormat,
    ) {
        let binding = binding.expect_downcast();
        unsafe { self.set_index_buffer(binding, format) };
    }

    unsafe fn set_vertex_buffer<'a>(
        &mut self,
        index: u32,
        binding: BufferBinding<'a, dyn DynBuffer>,
    ) {
        let binding = binding.expect_downcast();
        unsafe { self.set_vertex_buffer(index, binding) };
    }

    unsafe fn build_acceleration_structures<'a>(
        &mut self,
        descriptors: &'a [BuildAccelerationStructureDescriptor<
            'a,
            dyn DynBuffer,
            dyn DynAccelerationStructure,
        >],
    ) {
        // Need to collect entries here so we can reference them in the descriptor.
        // TODO: API should be redesigned to avoid this and other descriptor copies that happen due to the dyn api.
        let descriptor_entries = descriptors
            .iter()
            .map(|d| d.entries.expect_downcast())
            .collect::<Vec<_>>();
        let descriptors = descriptors
            .iter()
            .zip(descriptor_entries.iter())
            .map(|(d, entries)| BuildAccelerationStructureDescriptor::<
                <C::A as Api>::Buffer,
                <C::A as Api>::AccelerationStructure,
            > {
                entries,
                mode: d.mode,
                flags: d.flags,
                source_acceleration_structure: d
                    .source_acceleration_structure
                    .map(|a| a.expect_downcast_ref()),
                destination_acceleration_structure: d
                    .destination_acceleration_structure
                    .expect_downcast_ref(),
                scratch_buffer: d.scratch_buffer.expect_downcast_ref(),
                scratch_buffer_offset: d.scratch_buffer_offset,
            });
        unsafe { C::build_acceleration_structures(self, descriptors.len() as _, descriptors) };
    }

    unsafe fn place_acceleration_structure_barrier(
        &mut self,
        barrier: AccelerationStructureBarrier,
    ) {
        unsafe { C::place_acceleration_structure_barrier(self, barrier) };
    }

    unsafe fn copy_acceleration_structure_to_acceleration_structure(
        &mut self,
        src: &dyn DynAccelerationStructure,
        dst: &dyn DynAccelerationStructure,
        copy: wgt::AccelerationStructureCopy,
    ) {
        let src = src.expect_downcast_ref();
        let dst = dst.expect_downcast_ref();
        unsafe { C::copy_acceleration_structure_to_acceleration_structure(self, src, dst, copy) };
    }
    unsafe fn read_acceleration_structure_compact_size(
        &mut self,
        acceleration_structure: &dyn DynAccelerationStructure,
        buf: &dyn DynBuffer,
    ) {
        let acceleration_structure = acceleration_structure.expect_downcast_ref();
        let buf = buf.expect_downcast_ref();
        unsafe { C::read_acceleration_structure_compact_size(self, acceleration_structure, buf) }
    }

    unsafe fn set_acceleration_structure_dependencies(
        &self,
        command_buffers: &[Box<dyn DynCommandBuffer>],
        dependencies: &[&dyn DynAccelerationStructure],
    ) {
        let command_buffers: Vec<&<C::A as Api>::CommandBuffer> = command_buffers
            .iter()
            .map(|command_buffer| command_buffer.expect_downcast_ref())
            .collect();
        let dependencies: Vec<&<C::A as Api>::AccelerationStructure> = dependencies
            .iter()
            .map(|dependency| dependency.expect_downcast_ref())
            .collect();
        unsafe { C::set_acceleration_structure_dependencies(&command_buffers, &dependencies) }
    }
}

impl<'a> PassTimestampWrites<'a, dyn DynQuerySet> {
    pub fn expect_downcast<B: DynQuerySet>(&self) -> PassTimestampWrites<'a, B> {
        PassTimestampWrites {
            query_set: self.query_set.expect_downcast_ref(),
            beginning_of_pass_write_index: self.beginning_of_pass_write_index,
            end_of_pass_write_index: self.end_of_pass_write_index,
        }
    }
}

impl<'a> Attachment<'a, dyn DynTextureView> {
    pub fn expect_downcast<B: DynTextureView>(&self) -> Attachment<'a, B> {
        Attachment {
            view: self.view.expect_downcast_ref(),
            usage: self.usage,
        }
    }
}

impl<'a> ColorAttachment<'a, dyn DynTextureView> {
    pub fn expect_downcast<B: DynTextureView>(&self) -> ColorAttachment<'a, B> {
        ColorAttachment {
            target: self.target.expect_downcast(),
            depth_slice: self.depth_slice,
            resolve_target: self.resolve_target.as_ref().map(|rt| rt.expect_downcast()),
            ops: self.ops,
            clear_value: self.clear_value,
        }
    }
}

impl<'a> DepthStencilAttachment<'a, dyn DynTextureView> {
    pub fn expect_downcast<B: DynTextureView>(&self) -> DepthStencilAttachment<'a, B> {
        DepthStencilAttachment {
            target: self.target.expect_downcast(),
            depth_ops: self.depth_ops,
            stencil_ops: self.stencil_ops,
            clear_value: self.clear_value,
        }
    }
}
