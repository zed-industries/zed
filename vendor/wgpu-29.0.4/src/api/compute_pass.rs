use crate::{
    api::{impl_deferred_command_buffer_actions, SharedDeferredCommandBufferActions},
    *,
};

/// In-progress recording of a compute pass.
///
/// It can be created with [`CommandEncoder::begin_compute_pass`].
///
/// Corresponds to [WebGPU `GPUComputePassEncoder`](
/// https://gpuweb.github.io/gpuweb/#compute-pass-encoder).
#[derive(Debug)]
pub struct ComputePass<'encoder> {
    pub(crate) inner: dispatch::DispatchComputePass,

    /// Shared with CommandEncoder to enqueue deferred actions from within a pass.
    pub(crate) actions: SharedDeferredCommandBufferActions,

    /// This lifetime is used to protect the [`CommandEncoder`] from being used
    /// while the pass is alive. This needs to be PhantomDrop to prevent the lifetime
    /// from being shortened.
    pub(crate) _encoder_guard: crate::api::PhantomDrop<&'encoder ()>,
}

#[cfg(send_sync)]
static_assertions::assert_impl_all!(ComputePass<'_>: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(ComputePass<'_> => .inner);

impl ComputePass<'_> {
    /// Drops the lifetime relationship to the parent command encoder, making usage of
    /// the encoder while this pass is recorded a run-time error instead.
    ///
    /// Attention: As long as the compute pass has not been ended, any mutating operation on the parent
    /// command encoder will cause a run-time error and invalidate it!
    /// By default, the lifetime constraint prevents this, but it can be useful
    /// to handle this at run time, such as when storing the pass and encoder in the same
    /// data structure.
    ///
    /// This operation has no effect on pass recording.
    /// It's a safe operation, since [`CommandEncoder`] is in a locked state as long as the pass is active
    /// regardless of the lifetime constraint or its absence.
    pub fn forget_lifetime(self) -> ComputePass<'static> {
        ComputePass {
            inner: self.inner,
            actions: self.actions,
            _encoder_guard: crate::api::PhantomDrop::default(),
        }
    }

    /// Sets the active bind group for a given bind group index. The bind group layout
    /// in the active pipeline when the `dispatch()` function is called must match the layout of this bind group.
    ///
    /// If the bind group have dynamic offsets, provide them in the binding order.
    /// These offsets have to be aligned to [`Limits::min_uniform_buffer_offset_alignment`]
    /// or [`Limits::min_storage_buffer_offset_alignment`] appropriately.
    pub fn set_bind_group<'a, BG>(&mut self, index: u32, bind_group: BG, offsets: &[DynamicOffset])
    where
        Option<&'a BindGroup>: From<BG>,
    {
        let bg: Option<&BindGroup> = bind_group.into();
        let bg = bg.map(|bg| &bg.inner);
        self.inner.set_bind_group(index, bg, offsets);
    }

    /// Sets the active compute pipeline.
    pub fn set_pipeline(&mut self, pipeline: &ComputePipeline) {
        self.inner.set_pipeline(&pipeline.inner);
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

    /// Dispatches compute work operations.
    ///
    /// `x`, `y` and `z` denote the number of work groups to dispatch in each dimension.
    pub fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) {
        self.inner.dispatch_workgroups(x, y, z);
    }

    /// Dispatches compute work operations, based on the contents of the `indirect_buffer`.
    ///
    /// The structure expected in `indirect_buffer` must conform to [`DispatchIndirectArgs`](crate::util::DispatchIndirectArgs).
    pub fn dispatch_workgroups_indirect(
        &mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
    ) {
        self.inner
            .dispatch_workgroups_indirect(&indirect_buffer.inner, indirect_offset);
    }

    impl_deferred_command_buffer_actions!();

    #[cfg(custom)]
    /// Returns custom implementation of ComputePass (if custom backend and is internally T)
    pub fn as_custom<T: custom::ComputePassInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// [`Features::IMMEDIATES`] must be enabled on the device in order to call these functions.
impl ComputePass<'_> {
    /// Set immediate data for subsequent dispatch calls.
    ///
    /// Write the bytes in `data` at offset `offset` within immediate data
    /// storage.  Both `offset` and the length of `data` must be
    /// multiples of [`crate::IMMEDIATE_DATA_ALIGNMENT`], which is always 4.
    ///
    /// For example, if `offset` is `4` and `data` is eight bytes long, this
    /// call will write `data` to bytes `4..12` of immediate data storage.
    pub fn set_immediates(&mut self, offset: u32, data: &[u8]) {
        self.inner.set_immediates(offset, data);
    }
}

/// [`Features::TIMESTAMP_QUERY_INSIDE_PASSES`] must be enabled on the device in order to call these functions.
impl ComputePass<'_> {
    /// Issue a timestamp command at this point in the queue. The timestamp will be written to the specified query set, at the specified index.
    ///
    /// Must be multiplied by [`Queue::get_timestamp_period`] to get
    /// the value in nanoseconds. Absolute values have no meaning,
    /// but timestamps can be subtracted to get the time it takes
    /// for a string of operations to complete.
    pub fn write_timestamp(&mut self, query_set: &QuerySet, query_index: u32) {
        self.inner.write_timestamp(&query_set.inner, query_index);
    }
}

/// [`Features::PIPELINE_STATISTICS_QUERY`] must be enabled on the device in order to call these functions.
impl ComputePass<'_> {
    /// Start a pipeline statistics query on this compute pass. It can be ended with
    /// `end_pipeline_statistics_query`. Pipeline statistics queries may not be nested.
    ///
    /// The amount of information collected by this query, and the space occupied in the query set,
    /// is determined by the [`PipelineStatisticsTypes`] the query set was created with.
    /// `query_index` is the index of the first query result slot that will be written to, and
    /// `query_set` must have sufficient size to hold all results written starting at that slot.
    pub fn begin_pipeline_statistics_query(&mut self, query_set: &QuerySet, query_index: u32) {
        self.inner
            .begin_pipeline_statistics_query(&query_set.inner, query_index);
    }

    /// End the pipeline statistics query on this compute pass. It can be started with
    /// `begin_pipeline_statistics_query`. Pipeline statistics queries may not be nested.
    pub fn end_pipeline_statistics_query(&mut self) {
        self.inner.end_pipeline_statistics_query();
    }
}

/// Describes the timestamp writes of a compute pass.
///
/// For use with [`ComputePassDescriptor`].
/// At least one of `beginning_of_pass_write_index` and `end_of_pass_write_index` must be `Some`.
///
/// Corresponds to [WebGPU `GPUComputePassTimestampWrites`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpucomputepasstimestampwrites).
#[derive(Clone, Debug)]
pub struct ComputePassTimestampWrites<'a> {
    /// The query set to write to.
    pub query_set: &'a QuerySet,
    /// The index of the query set at which a start timestamp of this pass is written, if any.
    pub beginning_of_pass_write_index: Option<u32>,
    /// The index of the query set at which an end timestamp of this pass is written, if any.
    pub end_of_pass_write_index: Option<u32>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(ComputePassTimestampWrites<'_>: Send, Sync);

/// Describes the attachments of a compute pass.
///
/// For use with [`CommandEncoder::begin_compute_pass`].
///
/// Corresponds to [WebGPU `GPUComputePassDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpucomputepassdescriptor).
#[derive(Clone, Default, Debug)]
pub struct ComputePassDescriptor<'a> {
    /// Debug label of the compute pass. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// Defines which timestamp values will be written for this pass, and where to write them to.
    ///
    /// Requires [`Features::TIMESTAMP_QUERY`] to be enabled.
    pub timestamp_writes: Option<ComputePassTimestampWrites<'a>>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(ComputePassDescriptor<'_>: Send, Sync);
