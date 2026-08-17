use core::{num::NonZeroU32, ops::Range};

use crate::{
    api::{impl_deferred_command_buffer_actions, SharedDeferredCommandBufferActions},
    *,
};
pub use wgt::{LoadOp, Operations, StoreOp};

/// In-progress recording of a render pass: a list of render commands in a [`CommandEncoder`].
///
/// It can be created with [`CommandEncoder::begin_render_pass()`], whose [`RenderPassDescriptor`]
/// specifies the attachments (textures) that will be rendered to.
///
/// Most of the methods on `RenderPass` serve one of two purposes, identifiable by their names:
///
/// * `draw_*()`: Drawing (that is, encoding a render command, which, when executed by the GPU, will
///   rasterize something and execute shaders).
/// * `set_*()`: Setting part of the [render state](https://gpuweb.github.io/gpuweb/#renderstate)
///   for future drawing commands.
///
/// A render pass may contain any number of drawing commands, and before/between each command the
/// render state may be updated however you wish; each drawing command will be executed using the
/// render state that has been set when the `draw_*()` function is called.
///
/// Corresponds to [WebGPU `GPURenderPassEncoder`](
/// https://gpuweb.github.io/gpuweb/#render-pass-encoder).
#[derive(Debug)]
pub struct RenderPass<'encoder> {
    pub(crate) inner: dispatch::DispatchRenderPass,
    pub(crate) actions: SharedDeferredCommandBufferActions,

    /// This lifetime is used to protect the [`CommandEncoder`] from being used
    /// while the pass is alive. This needs to be PhantomDrop to prevent the lifetime
    /// from being shortened.
    pub(crate) _encoder_guard: PhantomDrop<&'encoder ()>,
}

#[cfg(send_sync)]
static_assertions::assert_impl_all!(RenderPass<'_>: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(RenderPass<'_> => .inner);

impl RenderPass<'_> {
    /// Drops the lifetime relationship to the parent command encoder, making usage of
    /// the encoder while this pass is recorded a run-time error instead.
    ///
    /// Attention: As long as the render pass has not been ended, any mutating operation on the parent
    /// command encoder will cause a run-time error and invalidate it!
    /// By default, the lifetime constraint prevents this, but it can be useful
    /// to handle this at run time, such as when storing the pass and encoder in the same
    /// data structure.
    ///
    /// This operation has no effect on pass recording.
    /// It's a safe operation, since [`CommandEncoder`] is in a locked state as long as the pass is active
    /// regardless of the lifetime constraint or its absence.
    pub fn forget_lifetime(self) -> RenderPass<'static> {
        RenderPass {
            inner: self.inner,
            actions: self.actions,
            _encoder_guard: crate::api::PhantomDrop::default(),
        }
    }

    /// Sets the active bind group for a given bind group index. The bind group layout
    /// in the active pipeline when any `draw_*()` method is called must match the layout of
    /// this bind group.
    ///
    /// If the bind group have dynamic offsets, provide them in binding order.
    /// These offsets have to be aligned to [`Limits::min_uniform_buffer_offset_alignment`]
    /// or [`Limits::min_storage_buffer_offset_alignment`] appropriately.
    ///
    /// Subsequent draw calls’ shader executions will be able to access data in these bind groups.
    pub fn set_bind_group<'a, BG>(&mut self, index: u32, bind_group: BG, offsets: &[DynamicOffset])
    where
        Option<&'a BindGroup>: From<BG>,
    {
        let bg: Option<&'a BindGroup> = bind_group.into();
        let bg = bg.map(|bg| &bg.inner);

        self.inner.set_bind_group(index, bg, offsets);
    }

    /// Sets the active render pipeline.
    ///
    /// Subsequent draw calls will exhibit the behavior defined by `pipeline`.
    pub fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        self.inner.set_pipeline(&pipeline.inner);
    }

    /// Sets the blend color as used by some of the blending modes.
    ///
    /// Subsequent blending tests will test against this value.
    /// If this method has not been called, the blend constant defaults to [`Color::TRANSPARENT`]
    /// (all components zero).
    pub fn set_blend_constant(&mut self, color: Color) {
        self.inner.set_blend_constant(color);
    }

    /// Sets the active index buffer.
    ///
    /// Subsequent calls to [`draw_indexed`](RenderPass::draw_indexed) on this [`RenderPass`] will
    /// use `buffer` as the source index buffer.
    pub fn set_index_buffer(&mut self, buffer_slice: BufferSlice<'_>, index_format: IndexFormat) {
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
    /// [`RenderPass`] will use `buffer` as one of the source vertex buffers.
    /// The format of the data in the buffer is specified by the [`VertexBufferLayout`] in the
    /// pipeline's [`VertexState`].
    ///
    /// The `slot` refers to the index of the matching descriptor in
    /// [`VertexState::buffers`].
    ///
    /// [`draw`]: RenderPass::draw
    /// [`draw_indexed`]: RenderPass::draw_indexed
    pub fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice<'_>) {
        self.inner.set_vertex_buffer(
            slot,
            &buffer_slice.buffer.inner,
            buffer_slice.offset,
            Some(buffer_slice.size),
        );
    }

    /// Sets the scissor rectangle used during the rasterization stage.
    /// After transformation into [viewport coordinates](https://www.w3.org/TR/webgpu/#viewport-coordinates).
    ///
    /// Subsequent draw calls will discard any fragments which fall outside the scissor rectangle.
    /// If this method has not been called, the scissor rectangle defaults to the entire bounds of
    /// the render targets.
    ///
    /// The function of the scissor rectangle resembles [`set_viewport()`](Self::set_viewport),
    /// but it does not affect the coordinate system, only which fragments are discarded.
    pub fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.inner.set_scissor_rect(x, y, width, height);
    }

    /// Sets the viewport used during the rasterization stage to linearly map
    /// from [normalized device coordinates](https://www.w3.org/TR/webgpu/#ndc) to [viewport coordinates](https://www.w3.org/TR/webgpu/#viewport-coordinates).
    ///
    /// Subsequent draw calls will only draw within this region.
    /// If this method has not been called, the viewport defaults to the entire bounds of the render
    /// targets.
    pub fn set_viewport(&mut self, x: f32, y: f32, w: f32, h: f32, min_depth: f32, max_depth: f32) {
        self.inner.set_viewport(x, y, w, h, min_depth, max_depth);
    }

    /// Sets the stencil reference.
    ///
    /// Subsequent stencil tests will test against this value.
    /// If this method has not been called, the stencil reference value defaults to `0`.
    pub fn set_stencil_reference(&mut self, reference: u32) {
        self.inner.set_stencil_reference(reference);
    }

    /// Inserts debug marker.
    pub fn insert_debug_marker(&mut self, label: &str) {
        self.inner.insert_debug_marker(label);
    }

    /// Start record commands and group it into debug marker group.
    pub fn push_debug_group(&mut self, label: &str) {
        self.inner.push_debug_group(label);
    }

    /// Stops command recording and creates debug group.
    pub fn pop_debug_group(&mut self) {
        self.inner.pop_debug_group();
    }

    /// Draws primitives from the active vertex buffer(s).
    ///
    /// The active vertex buffer(s) can be set with [`RenderPass::set_vertex_buffer`].
    /// This does not use an index buffer. If you need indexed drawing, see [`RenderPass::draw_indexed`]
    ///
    /// Panics if `vertices` range is outside of the range of the vertices range of any set vertex buffer.
    ///
    /// - `vertices`: The range of vertices to draw.
    /// - `instances`: Range of instances to draw. Use `0..1` if instance buffers are not used.
    ///
    /// E.g.of how its used internally
    /// ```rust ignore
    /// for instance_id in instance_range {
    ///     for vertex_id in vertex_range {
    ///         let vertex = vertex[vertex_id];
    ///         vertex_shader(vertex, vertex_id, instance_id);
    ///     }
    /// }
    /// ```
    ///
    /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
    /// It is not affected by changes to the state that are performed after it is called.
    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.inner.draw(vertices, instances);
    }

    /// Draws indexed primitives using the active index buffer and the active vertex buffers.
    ///
    /// The active index buffer can be set with [`RenderPass::set_index_buffer`]
    /// The active vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
    ///
    /// Panics if `indices` range is outside of the range of the indices range of the set index buffer.
    ///
    /// - `indices`: The range of indices to draw.
    /// - `base_vertex`: value added to each index value before indexing into the vertex buffers.
    /// - `instances`: Range of instances to draw. Use `0..1` if instance buffers are not used.
    ///
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
    ///
    /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
    /// It is not affected by changes to the state that are performed after it is called.
    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.inner.draw_indexed(indices, base_vertex, instances);
    }

    /// Draws using a mesh pipeline.
    ///
    /// The current pipeline must be a mesh pipeline.
    ///
    /// If the current pipeline has a task shader, run it with an workgroup for
    /// every `vec3<u32>(i, j, k)` where `i`, `j`, and `k` are between `0` and
    /// `group_count_x`, `group_count_y`, and `group_count_z`. The invocation with
    /// index zero in each group is responsible for determining the mesh shader dispatch.
    /// Its return value indicates the number of workgroups of mesh shaders to invoke. It also
    /// passes a payload value for them to consume. Because each task workgroup is essentially
    /// a mesh shader draw call, mesh workgroups dispatched by different task workgroups
    /// cannot interact in any way, and `workgroup_id` corresponds to its location in the
    /// calling specific task shader's dispatch group.
    ///
    /// If the current pipeline lacks a task shader, run its mesh shader with a
    /// workgroup for every `vec3<u32>(i, j, k)` where `i`, `j`, and `k` are
    /// between `0` and `group_count_x`, `group_count_y`, and `group_count_z`.
    ///
    /// Each mesh shader workgroup outputs a set of vertices and indices for primitives.
    /// The indices outputted correspond to the vertices outputted by that same workgroup;
    /// there is no global vertex buffer. These primitives are passed to the rasterizer and
    /// essentially treated like a vertex shader output, except that the mesh shader may
    /// choose to cull specific primitives or pass per-primitive non-interpolated values
    /// to the fragment shader. As such, each primitive is then rendered with the current
    /// pipeline's fragment shader, if present. Otherwise, [No Color Output mode] is used.
    ///
    /// [No Color Output mode]: https://www.w3.org/TR/webgpu/#no-color-output
    pub fn draw_mesh_tasks(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self.inner
            .draw_mesh_tasks(group_count_x, group_count_y, group_count_z);
    }

    /// Draws primitives from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
    ///
    /// This is like calling [`RenderPass::draw`] but the contents of the call are specified in the `indirect_buffer`.
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
    ///
    /// Calling this requires the device support [`DownlevelFlags::INDIRECT_EXECUTION`].
    pub fn draw_indirect(&mut self, indirect_buffer: &Buffer, indirect_offset: BufferAddress) {
        self.inner
            .draw_indirect(&indirect_buffer.inner, indirect_offset);
    }

    /// Draws indexed primitives using the active index buffer and the active vertex buffers,
    /// based on the contents of the `indirect_buffer`.
    ///
    /// This is like calling [`RenderPass::draw_indexed`] but the contents of the call are specified in the `indirect_buffer`.
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
    ///
    /// Calling this requires the device support [`DownlevelFlags::INDIRECT_EXECUTION`].
    pub fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
    ) {
        self.inner
            .draw_indexed_indirect(&indirect_buffer.inner, indirect_offset);
    }

    /// Draws using a mesh pipeline,
    /// based on the contents of the `indirect_buffer`
    ///
    /// This is like calling [`RenderPass::draw_mesh_tasks`] but the contents of the call are specified in the `indirect_buffer`.
    /// The structure expected in the `indirect_buffer` must conform to [`DispatchIndirectArgs`](crate::util::DispatchIndirectArgs).
    ///
    /// Indirect drawing has some caveats depending on the features available. We are not currently able to validate
    /// these and issue an error.
    ///
    /// See details on the individual flags for more information.
    pub fn draw_mesh_tasks_indirect(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
    ) {
        self.inner
            .draw_mesh_tasks_indirect(&indirect_buffer.inner, indirect_offset);
    }

    impl_deferred_command_buffer_actions!();

    /// Execute a [render bundle][RenderBundle], which is a set of pre-recorded commands
    /// that can be run together.
    ///
    /// Commands in the bundle do not inherit this render pass's current render state, and after the
    /// bundle has executed, the state is **cleared** (reset to defaults, not the previous state).
    pub fn execute_bundles<'a, I: IntoIterator<Item = &'a RenderBundle>>(
        &mut self,
        render_bundles: I,
    ) {
        let mut render_bundles = render_bundles.into_iter().map(|rb| &rb.inner);

        self.inner.execute_bundles(&mut render_bundles);
    }

    /// Dispatches multiple draw calls from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
    /// `count` draw calls are issued.
    ///
    /// The active vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
    /// These draw structures are expected to be tightly packed.
    ///
    /// Calling this requires the device support [`DownlevelFlags::INDIRECT_EXECUTION`].
    ///
    /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
    /// It is not affected by changes to the state that are performed after it is called.
    pub fn multi_draw_indirect(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
        count: u32,
    ) {
        self.inner
            .multi_draw_indirect(&indirect_buffer.inner, indirect_offset, count);
    }

    /// Dispatches multiple draw calls from the active index buffer and the active vertex buffers,
    /// based on the contents of the `indirect_buffer`. `count` draw calls are issued.
    ///
    /// The active index buffer can be set with [`RenderPass::set_index_buffer`], while the active
    /// vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
    /// These draw structures are expected to be tightly packed.
    ///
    /// Calling this requires the device support [`DownlevelFlags::INDIRECT_EXECUTION`].
    ///
    /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
    /// It is not affected by changes to the state that are performed after it is called.
    pub fn multi_draw_indexed_indirect(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
        count: u32,
    ) {
        self.inner
            .multi_draw_indexed_indirect(&indirect_buffer.inner, indirect_offset, count);
    }

    /// Dispatches multiple draw calls based on the contents of the `indirect_buffer`.
    /// `count` draw calls are issued.
    ///
    /// The structure expected in the `indirect_buffer` must conform to [`DispatchIndirectArgs`](crate::util::DispatchIndirectArgs).
    ///
    /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
    /// It is not affected by changes to the state that are performed after it is called.
    pub fn multi_draw_mesh_tasks_indirect(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
        count: u32,
    ) {
        self.inner
            .multi_draw_mesh_tasks_indirect(&indirect_buffer.inner, indirect_offset, count);
    }

    #[cfg(custom)]
    /// Returns custom implementation of RenderPass (if custom backend and is internally T)
    pub fn as_custom<T: custom::RenderPassInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// [`Features::MULTI_DRAW_INDIRECT_COUNT`] must be enabled on the device in order to call these functions.
impl RenderPass<'_> {
    /// Dispatches multiple draw calls from the active vertex buffer(s) based on the contents of the `indirect_buffer`.
    /// The count buffer is read to determine how many draws to issue.
    ///
    /// The indirect buffer must be long enough to account for `max_count` draws, however only `count`
    /// draws will be read. If `count` is greater than `max_count`, `max_count` will be used.
    ///
    /// The active vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndirectArgs`](crate::util::DrawIndirectArgs).
    /// These draw structures are expected to be tightly packed.
    ///
    /// The structure expected in `count_buffer` is the following:
    ///
    /// ```rust
    /// #[repr(C)]
    /// struct DrawIndirectCount {
    ///     count: u32, // Number of draw calls to issue.
    /// }
    /// ```
    ///
    /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
    /// It is not affected by changes to the state that are performed after it is called.
    pub fn multi_draw_indirect_count(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
        count_buffer: &Buffer,
        count_offset: BufferAddress,
        max_count: u32,
    ) {
        self.inner.multi_draw_indirect_count(
            &indirect_buffer.inner,
            indirect_offset,
            &count_buffer.inner,
            count_offset,
            max_count,
        );
    }

    /// Dispatches multiple draw calls from the active index buffer and the active vertex buffers,
    /// based on the contents of the `indirect_buffer`. The count buffer is read to determine how many draws to issue.
    ///
    /// The indirect buffer must be long enough to account for `max_count` draws, however only `count`
    /// draws will be read. If `count` is greater than `max_count`, `max_count` will be used.
    ///
    /// The active index buffer can be set with [`RenderPass::set_index_buffer`], while the active
    /// vertex buffers can be set with [`RenderPass::set_vertex_buffer`].
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DrawIndexedIndirectArgs`](crate::util::DrawIndexedIndirectArgs).
    ///
    /// These draw structures are expected to be tightly packed.
    ///
    /// The structure expected in `count_buffer` is the following:
    ///
    /// ```rust
    /// #[repr(C)]
    /// struct DrawIndexedIndirectCount {
    ///     count: u32, // Number of draw calls to issue.
    /// }
    /// ```
    ///
    /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
    /// It is not affected by changes to the state that are performed after it is called.
    pub fn multi_draw_indexed_indirect_count(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
        count_buffer: &Buffer,
        count_offset: BufferAddress,
        max_count: u32,
    ) {
        self.inner.multi_draw_indexed_indirect_count(
            &indirect_buffer.inner,
            indirect_offset,
            &count_buffer.inner,
            count_offset,
            max_count,
        );
    }

    /// Dispatches multiple draw calls based on the contents of the `indirect_buffer`. The count buffer is read to determine how many draws to issue.
    ///
    /// The indirect buffer must be long enough to account for `max_count` draws, however only `count`
    /// draws will be read. If `count` is greater than `max_count`, `max_count` will be used.
    ///
    /// The structure expected in the `indirect_buffer` must conform to [`DispatchIndirectArgs`](crate::util::DispatchIndirectArgs).
    ///
    /// These draw structures are expected to be tightly packed.
    ///
    /// This drawing command uses the current render state, as set by preceding `set_*()` methods.
    /// It is not affected by changes to the state that are performed after it is called.
    pub fn multi_draw_mesh_tasks_indirect_count(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
        count_buffer: &Buffer,
        count_offset: BufferAddress,
        max_count: u32,
    ) {
        self.inner.multi_draw_mesh_tasks_indirect_count(
            &indirect_buffer.inner,
            indirect_offset,
            &count_buffer.inner,
            count_offset,
            max_count,
        );
    }
}

/// [`Features::IMMEDIATES`] must be enabled on the device in order to call these functions.
impl RenderPass<'_> {
    /// Set immediate data for subsequent draw calls.
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

/// [`Features::TIMESTAMP_QUERY_INSIDE_PASSES`] must be enabled on the device in order to call these functions.
impl RenderPass<'_> {
    /// Issue a timestamp command at this point in the queue. The
    /// timestamp will be written to the specified query set, at the specified index.
    ///
    /// Must be multiplied by [`Queue::get_timestamp_period`] to get
    /// the value in nanoseconds. Absolute values have no meaning,
    /// but timestamps can be subtracted to get the time it takes
    /// for a string of operations to complete.
    pub fn write_timestamp(&mut self, query_set: &QuerySet, query_index: u32) {
        self.inner.write_timestamp(&query_set.inner, query_index);
    }
}

impl RenderPass<'_> {
    /// Start a occlusion query on this render pass. It can be ended with
    /// [`end_occlusion_query`](Self::end_occlusion_query).
    /// Occlusion queries may not be nested.
    pub fn begin_occlusion_query(&mut self, query_index: u32) {
        self.inner.begin_occlusion_query(query_index);
    }

    /// End the occlusion query on this render pass. It can be started with
    /// [`begin_occlusion_query`](Self::begin_occlusion_query).
    /// Occlusion queries may not be nested.
    pub fn end_occlusion_query(&mut self) {
        self.inner.end_occlusion_query();
    }
}

/// [`Features::PIPELINE_STATISTICS_QUERY`] must be enabled on the device in order to call these functions.
impl RenderPass<'_> {
    /// Start a pipeline statistics query on this render pass. It can be ended with
    /// [`end_pipeline_statistics_query`](Self::end_pipeline_statistics_query).
    /// Pipeline statistics queries may not be nested.
    ///
    /// The amount of information collected by this query, and the space occupied in the query set,
    /// is determined by the [`PipelineStatisticsTypes`] the query set was created with.
    /// `query_index` is the index of the first query result slot that will be written to, and
    /// `query_set` must have sufficient size to hold all results written starting at that slot.
    pub fn begin_pipeline_statistics_query(&mut self, query_set: &QuerySet, query_index: u32) {
        self.inner
            .begin_pipeline_statistics_query(&query_set.inner, query_index);
    }

    /// End the pipeline statistics query on this render pass. It can be started with
    /// [`begin_pipeline_statistics_query`](Self::begin_pipeline_statistics_query).
    /// Pipeline statistics queries may not be nested.
    pub fn end_pipeline_statistics_query(&mut self) {
        self.inner.end_pipeline_statistics_query();
    }
}

/// Describes the timestamp writes of a render pass.
///
/// For use with [`RenderPassDescriptor`].
/// At least one of [`Self::beginning_of_pass_write_index`] and [`Self::end_of_pass_write_index`]
/// must be `Some`.
///
/// Corresponds to [WebGPU `GPURenderPassTimestampWrite`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderpasstimestampwrites).
#[derive(Clone, Debug)]
pub struct RenderPassTimestampWrites<'a> {
    /// The query set to write to.
    pub query_set: &'a QuerySet,
    /// The index of the query set at which a start timestamp of this pass is written, if any.
    pub beginning_of_pass_write_index: Option<u32>,
    /// The index of the query set at which an end timestamp of this pass is written, if any.
    pub end_of_pass_write_index: Option<u32>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(RenderPassTimestampWrites<'_>: Send, Sync);

/// Describes a color attachment to a [`RenderPass`].
///
/// For use with [`RenderPassDescriptor`].
///
/// Corresponds to [WebGPU `GPURenderPassColorAttachment`](
/// https://gpuweb.github.io/gpuweb/#color-attachments).
#[derive(Clone, Debug)]
pub struct RenderPassColorAttachment<'tex> {
    /// The view to use as an attachment.
    pub view: &'tex TextureView,
    /// The depth slice index of a 3D view. It must not be provided if the view is not 3D.
    pub depth_slice: Option<u32>,
    /// The view that will receive the resolved output if multisampling is used.
    ///
    /// If set, it is always written to, regardless of how [`Self::ops`] is configured.
    pub resolve_target: Option<&'tex TextureView>,
    /// What operations will be performed on this color attachment.
    pub ops: Operations<Color>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(RenderPassColorAttachment<'_>: Send, Sync);

/// Describes a depth/stencil attachment to a [`RenderPass`].
///
/// For use with [`RenderPassDescriptor`].
///
/// Corresponds to [WebGPU `GPURenderPassDepthStencilAttachment`](
/// https://gpuweb.github.io/gpuweb/#depth-stencil-attachments).
#[derive(Clone, Debug)]
pub struct RenderPassDepthStencilAttachment<'tex> {
    /// The view to use as an attachment.
    pub view: &'tex TextureView,
    /// What operations will be performed on the depth part of the attachment.
    pub depth_ops: Option<Operations<f32>>,
    /// What operations will be performed on the stencil part of the attachment.
    pub stencil_ops: Option<Operations<u32>>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(RenderPassDepthStencilAttachment<'_>: Send, Sync);

/// Describes the attachments of a render pass.
///
/// For use with [`CommandEncoder::begin_render_pass`].
///
/// Corresponds to [WebGPU `GPURenderPassDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderpassdescriptor).
#[derive(Clone, Debug, Default)]
pub struct RenderPassDescriptor<'a> {
    /// Debug label of the render pass. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// The color attachments of the render pass.
    pub color_attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    /// The depth and stencil attachment of the render pass, if any.
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,
    /// Defines which timestamp values will be written for this pass, and where to write them to.
    ///
    /// Requires [`Features::TIMESTAMP_QUERY`] to be enabled.
    pub timestamp_writes: Option<RenderPassTimestampWrites<'a>>,
    /// Defines where the occlusion query results will be stored for this pass.
    pub occlusion_query_set: Option<&'a QuerySet>,
    /// The mask of multiview image layers to use for this render pass. For example, if you wish
    /// to render to the first 2 layers, you would use 3=0b11. If you wanted ro render to only the
    /// 2nd layer, you would use 2=0b10. If you aren't using multiview this should be `None`.
    ///
    /// Note that setting bits higher than the number of texture layers is a validation error.
    ///
    /// This doesn't influence load/store/clear/etc operations, as those are defined for attachments,
    /// therefore affecting all attachments. Meaning, this affects only any shaders executed on the `RenderPass`.
    pub multiview_mask: Option<NonZeroU32>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(RenderPassDescriptor<'_>: Send, Sync);
