use core::{marker::PhantomData, num::NonZeroU32, ops::Range};

use crate::dispatch::RenderBundleEncoderInterface;
use crate::*;

/// Encodes a series of GPU operations into a reusable "render bundle".
///
/// It only supports a handful of render commands, but it makes them reusable.
/// It can be created with [`Device::create_render_bundle_encoder`].
/// It can be executed onto a [`CommandEncoder`] using [`RenderPass::execute_bundles`].
///
/// Executing a [`RenderBundle`] is often more efficient than issuing the underlying commands
/// manually.
///
/// Corresponds to [WebGPU `GPURenderBundleEncoder`](
/// https://gpuweb.github.io/gpuweb/#gpurenderbundleencoder).
#[derive(Debug)]
pub struct RenderBundleEncoder<'a> {
    pub(crate) inner: dispatch::DispatchRenderBundleEncoder,
    /// This type should be !Send !Sync, because it represents an allocation on this thread's
    /// command buffer.
    pub(crate) _p: PhantomData<(*const u8, &'a ())>,
}
static_assertions::assert_not_impl_any!(RenderBundleEncoder<'_>: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(RenderBundleEncoder<'_> => .inner);

/// Describes a [`RenderBundleEncoder`].
///
/// For use with [`Device::create_render_bundle_encoder`].
///
/// Corresponds to [WebGPU `GPURenderBundleEncoderDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderbundleencoderdescriptor).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderBundleEncoderDescriptor<'a> {
    /// Debug label of the render bundle encoder. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// The formats of the color attachments that this render bundle is capable to rendering to. This
    /// must match the formats of the color attachments in the render pass this render bundle is executed in.
    pub color_formats: &'a [Option<TextureFormat>],
    /// Information about the depth attachment that this render bundle is capable to rendering to. This
    /// must match the format of the depth attachments in the render pass this render bundle is executed in.
    pub depth_stencil: Option<RenderBundleDepthStencil>,
    /// Sample count this render bundle is capable of rendering to. This must match the pipelines and
    /// the render passes it is used in.
    pub sample_count: u32,
    /// If this render bundle will rendering to multiple array layers in the attachments at the same time.
    pub multiview: Option<NonZeroU32>,
}
static_assertions::assert_impl_all!(RenderBundleEncoderDescriptor<'_>: Send, Sync);

impl<'a> RenderBundleEncoder<'a> {
    /// Finishes recording and returns a [`RenderBundle`] that can be executed in other render passes.
    pub fn finish(self, desc: &RenderBundleDescriptor<'_>) -> RenderBundle {
        let bundle = match self.inner {
            #[cfg(wgpu_core)]
            dispatch::DispatchRenderBundleEncoder::Core(b) => b.finish(desc),
            #[cfg(webgpu)]
            dispatch::DispatchRenderBundleEncoder::WebGPU(b) => b.finish(desc),
            #[cfg(custom)]
            dispatch::DispatchRenderBundleEncoder::Custom(_) => unimplemented!(),
        };

        RenderBundle { inner: bundle }
    }

    /// Sets the active bind group for a given bind group index. The bind group layout
    /// in the active pipeline when any `draw()` function is called must match the layout of this bind group.
    ///
    /// If the bind group have dynamic offsets, provide them in the binding order.
    pub fn set_bind_group<'b, BG>(&mut self, index: u32, bind_group: BG, offsets: &[DynamicOffset])
    where
        Option<&'b BindGroup>: From<BG>,
    {
        let bg: Option<&'b BindGroup> = bind_group.into();
        let bg = bg.map(|x| &x.inner);
        self.inner.set_bind_group(index, bg, offsets);
    }

    /// Sets the active render pipeline.
    ///
    /// Subsequent draw calls will exhibit the behavior defined by `pipeline`.
    pub fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        self.inner.set_pipeline(&pipeline.inner);
    }

    /// Sets the active index buffer.
    ///
    /// Subsequent calls to [`draw_indexed`](RenderBundleEncoder::draw_indexed) on this [`RenderBundleEncoder`] will
    /// use `buffer` as the source index buffer.
    pub fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'a>, index_format: IndexFormat) {
        self.inner.set_index_buffer(
            &buffer_slice.buffer.inner,
            index_format,
            buffer_slice.offset,
            Some(buffer_slice.size),
        );
    }

    /// Assign a vertex buffer to a slot.
    ///
    /// Subsequent calls to [`draw`] and [`draw_indexed`] on this
    /// [`RenderBundleEncoder`] will use `buffer` as one of the source vertex buffers.
    ///
    /// The `slot` refers to the index of the matching descriptor in
    /// [`VertexState::buffers`].
    ///
    /// [`draw`]: RenderBundleEncoder::draw
    /// [`draw_indexed`]: RenderBundleEncoder::draw_indexed
    pub fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'a>) {
        self.inner.set_vertex_buffer(
            slot,
            &buffer_slice.buffer.inner,
            buffer_slice.offset,
            Some(buffer_slice.size),
        );
    }

    /// Draws primitives from the active vertex buffer(s).
    ///
    /// The active vertex buffers can be set with [`RenderBundleEncoder::set_vertex_buffer`].
    /// Does not use an Index Buffer. If you need this see [`RenderBundleEncoder::draw_indexed`]
    ///
    /// Panics if vertices Range is outside of the range of the vertices range of any set vertex buffer.
    ///
    /// vertices: The range of vertices to draw.
    /// instances: Range of Instances to draw. Use 0..1 if instance buffers are not used.
    /// E.g.of how its used internally
    /// ```rust ignore
    /// for instance_id in instance_range {
    ///     for vertex_id in vertex_range {
    ///         let vertex = vertex[vertex_id];
    ///         vertex_shader(vertex, vertex_id, instance_id);
    ///     }
    /// }
    /// ```
    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.inner.draw(vertices, instances);
    }

    /// Draws indexed primitives using the active index buffer and the active vertex buffer(s).
    ///
    /// The active index buffer can be set with [`RenderBundleEncoder::set_index_buffer`].
    /// The active vertex buffer(s) can be set with [`RenderBundleEncoder::set_vertex_buffer`].
    ///
    /// Panics if indices Range is outside of the range of the indices range of any set index buffer.
    ///
    /// indices: The range of indices to draw.
    /// base_vertex: value added to each index value before indexing into the vertex buffers.
    /// instances: Range of Instances to draw. Use 0..1 if instance buffers are not used.
    /// E.g.of how its used internally
    /// ```rust ignore
    /// for instance_id in instance_range {
    ///     for index_index in index_range {
    ///         let vertex_id = index_buffer[index_index];
    ///         let adjusted_vertex_id = vertex_id + base_vertex;
    ///         let vertex = vertex[adjusted_vertex_id];
    ///         vertex_shader(vertex, adjusted_vertex_id, instance_id);
    ///     }
    /// }
    /// ```
    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.inner.draw_indexed(indices, base_vertex, instances);
    }

    /// Draws primitives from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
    ///
    /// The active vertex buffers can be set with [`RenderBundleEncoder::set_vertex_buffer`].
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
    pub fn draw_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: BufferAddress) {
        self.inner
            .draw_indirect(&indirect_buffer.inner, indirect_offset);
    }

    /// Draws indexed primitives using the active index buffer and the active vertex buffers,
    /// based on the contents of the `indirect_buffer`.
    ///
    /// The active index buffer can be set with [`RenderBundleEncoder::set_index_buffer`], while the active
    /// vertex buffers can be set with [`RenderBundleEncoder::set_vertex_buffer`].
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
    pub fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &'a Buffer,
        indirect_offset: BufferAddress,
    ) {
        self.inner
            .draw_indexed_indirect(&indirect_buffer.inner, indirect_offset);
    }

    #[cfg(custom)]
    /// Returns custom implementation of RenderBundleEncoder (if custom backend and is internally T)
    pub fn as_custom<T: custom::RenderBundleEncoderInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// [`Features::IMMEDIATES`] must be enabled on the device in order to call these functions.
impl RenderBundleEncoder<'_> {
    /// Set immediate data for subsequent draw calls within the render bundle.
    ///
    /// Write the bytes in `data` at offset `offset` within immediate data
    /// storage. Both `offset` and the length of `data` must be
    /// multiples of [`crate::IMMEDIATE_DATA_ALIGNMENT`], which is always 4.
    ///
    /// For example, if `offset` is `4` and `data` is eight bytes long, this
    /// call will write `data` to bytes `4..12` of immediate data storage.
    pub fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        self.inner.set_immediates(offset, data);
    }
}
