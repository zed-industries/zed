use core::ops::Range;

use wgt::{BufferAddress, DynamicOffset, IndexFormat};

use crate::{BindGroup, Buffer, BufferSlice, RenderBundleEncoder, RenderPass, RenderPipeline};

/// Methods shared by [`RenderPass`] and [`RenderBundleEncoder`].
pub trait RenderEncoder<'a> {
    /// Sets the active bind group for a given bind group index. The bind group layout
    /// in the active pipeline when any `draw()` function is called must match the layout of this bind group.
    ///
    /// If the bind group have dynamic offsets, provide them in order of their declaration.
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&'a BindGroup>,
        offsets: &[DynamicOffset],
    );

    /// Sets the active render pipeline.
    ///
    /// Subsequent draw calls will exhibit the behavior defined by `pipeline`.
    fn set_pipeline(&mut self, pipeline: &'a RenderPipeline);

    /// Sets the active index buffer.
    ///
    /// Subsequent calls to [`draw_indexed`](RenderEncoder::draw_indexed) on this [`RenderEncoder`] will
    /// use `buffer` as the source index buffer.
    fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'a>, index_format: IndexFormat);

    /// Assign a vertex buffer to a slot.
    ///
    /// Subsequent calls to [`draw`] and [`draw_indexed`] on this
    /// [`RenderEncoder`] will use `buffer` as one of the source vertex buffers.
    ///
    /// The `slot` refers to the index of the matching descriptor in
    /// [`VertexState::buffers`](crate::VertexState::buffers).
    ///
    /// [`draw`]: RenderEncoder::draw
    /// [`draw_indexed`]: RenderEncoder::draw_indexed
    fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>);

    /// Draws primitives from the active vertex buffer(s).
    ///
    /// The active vertex buffers can be set with [`RenderEncoder::set_vertex_buffer`].
    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);

    /// Draws indexed primitives using the active index buffer and the active vertex buffers.
    ///
    /// The active index buffer can be set with [`RenderEncoder::set_index_buffer`], while the active
    /// vertex buffers can be set with [`RenderEncoder::set_vertex_buffer`].
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);

    /// Draws primitives from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
    ///
    /// The active vertex buffers can be set with [`RenderEncoder::set_vertex_buffer`].
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
    fn draw_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: BufferAddress);

    /// Draws indexed primitives using the active index buffer and the active vertex buffers,
    /// based on the contents of the `indirect_buffer`.
    ///
    /// The active index buffer can be set with [`RenderEncoder::set_index_buffer`], while the active
    /// vertex buffers can be set with [`RenderEncoder::set_vertex_buffer`].
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &'a Buffer,
        indirect_offset: BufferAddress,
    );

    /// [`wgt::Features::IMMEDIATES`] must be enabled on the device in order to call this function.
    ///
    /// Set immediate data for subsequent draw calls.
    ///
    /// Write the bytes in `data` at offset `offset` within immediate data
    /// storage. Both `offset` and the length of `data` must be
    /// multiples of [`crate::IMMEDIATE_DATA_ALIGNMENT`], which is always 4.
    ///
    /// For example, if `offset` is `4` and `data` is eight bytes long, this
    /// call will write `data` to bytes `4..12` of immediate data storage.
    fn set_immediates(&mut self, offset: u32, data: &[u8]);
}

impl<'a> RenderEncoder<'a> for RenderPass<'a> {
    #[inline(always)]
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&'a BindGroup>,
        offsets: &[DynamicOffset],
    ) {
        Self::set_bind_group(self, index, bind_group, offsets);
    }

    #[inline(always)]
    fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        Self::set_pipeline(self, pipeline);
    }

    #[inline(always)]
    fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'a>, index_format: IndexFormat) {
        Self::set_index_buffer(self, buffer_slice, index_format);
    }

    #[inline(always)]
    fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
        Self::set_vertex_buffer(self, slot, buffer_slice);
    }

    #[inline(always)]
    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        Self::draw(self, vertices, instances);
    }

    #[inline(always)]
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        Self::draw_indexed(self, indices, base_vertex, instances);
    }

    #[inline(always)]
    fn draw_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: BufferAddress) {
        Self::draw_indirect(self, indirect_buffer, indirect_offset);
    }

    #[inline(always)]
    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &'a Buffer,
        indirect_offset: BufferAddress,
    ) {
        Self::draw_indexed_indirect(self, indirect_buffer, indirect_offset);
    }

    #[inline(always)]
    fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        Self::set_immediates(self, offset, data);
    }
}

impl<'a> RenderEncoder<'a> for RenderBundleEncoder<'a> {
    #[inline(always)]
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&'a BindGroup>,
        offsets: &[DynamicOffset],
    ) {
        Self::set_bind_group(self, index, bind_group, offsets);
    }

    #[inline(always)]
    fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        Self::set_pipeline(self, pipeline);
    }

    #[inline(always)]
    fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'a>, index_format: IndexFormat) {
        Self::set_index_buffer(self, buffer_slice, index_format);
    }

    #[inline(always)]
    fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
        Self::set_vertex_buffer(self, slot, buffer_slice);
    }

    #[inline(always)]
    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        Self::draw(self, vertices, instances);
    }

    #[inline(always)]
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        Self::draw_indexed(self, indices, base_vertex, instances);
    }

    #[inline(always)]
    fn draw_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: BufferAddress) {
        Self::draw_indirect(self, indirect_buffer, indirect_offset);
    }

    #[inline(always)]
    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &'a Buffer,
        indirect_offset: BufferAddress,
    ) {
        Self::draw_indexed_indirect(self, indirect_buffer, indirect_offset);
    }

    #[inline(always)]
    fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        Self::set_immediates(self, offset, data);
    }
}
