use alloc::{sync::Arc, vec, vec::Vec};
use core::{iter, mem};

use crate::{
    command::{encoder::EncodingState, ArcCommand, EncoderStateError},
    device::{Device, DeviceError, MissingFeatures},
    global::Global,
    id,
    init_tracker::MemoryInitKind,
    resource::{
        Buffer, DestroyedResourceError, InvalidResourceError, MissingBufferUsageError,
        ParentDevice, QuerySet, RawResourceAccess, Trackable,
    },
    track::{StatelessTracker, TrackerIndex},
    FastHashMap,
};
use thiserror::Error;
use wgt::{
    error::{ErrorType, WebGpuError},
    BufferAddress,
};

#[derive(Debug)]
pub(crate) struct QueryResetMap {
    map: FastHashMap<TrackerIndex, (Vec<bool>, Arc<QuerySet>)>,
}
impl QueryResetMap {
    pub fn new() -> Self {
        Self {
            map: FastHashMap::default(),
        }
    }

    pub fn use_query_set(&mut self, query_set: &Arc<QuerySet>, query: u32) -> bool {
        let vec_pair = self
            .map
            .entry(query_set.tracker_index())
            .or_insert_with(|| {
                (
                    vec![false; query_set.desc.count as usize],
                    query_set.clone(),
                )
            });

        mem::replace(&mut vec_pair.0[query as usize], true)
    }

    pub fn reset_queries(&mut self, raw_encoder: &mut dyn hal::DynCommandEncoder) {
        for (_, (state, query_set)) in self.map.drain() {
            debug_assert_eq!(state.len(), query_set.desc.count as usize);

            // Need to find all "runs" of values which need resets. If the state vector is:
            // [false, true, true, false, true], we want to reset [1..3, 4..5]. This minimizes
            // the amount of resets needed.
            let mut run_start: Option<u32> = None;
            for (idx, value) in state.into_iter().chain(iter::once(false)).enumerate() {
                match (run_start, value) {
                    // We're inside of a run, do nothing
                    (Some(..), true) => {}
                    // We've hit the end of a run, dispatch a reset
                    (Some(start), false) => {
                        run_start = None;
                        unsafe { raw_encoder.reset_queries(query_set.raw(), start..idx as u32) };
                    }
                    // We're starting a run
                    (None, true) => {
                        run_start = Some(idx as u32);
                    }
                    // We're in a run of falses, do nothing.
                    (None, false) => {}
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SimplifiedQueryType {
    Occlusion,
    Timestamp,
    PipelineStatistics,
}
impl From<wgt::QueryType> for SimplifiedQueryType {
    fn from(q: wgt::QueryType) -> Self {
        match q {
            wgt::QueryType::Occlusion => SimplifiedQueryType::Occlusion,
            wgt::QueryType::Timestamp => SimplifiedQueryType::Timestamp,
            wgt::QueryType::PipelineStatistics(..) => SimplifiedQueryType::PipelineStatistics,
        }
    }
}

/// Error encountered when dealing with queries
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum QueryError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    EncoderState(#[from] EncoderStateError),
    #[error(transparent)]
    MissingFeature(#[from] MissingFeatures),
    #[error("Error encountered while trying to use queries")]
    Use(#[from] QueryUseError),
    #[error("Error encountered while trying to resolve a query")]
    Resolve(#[from] ResolveError),
    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
}

impl WebGpuError for QueryError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::EncoderState(e) => e.webgpu_error_type(),
            Self::Use(e) => e.webgpu_error_type(),
            Self::Resolve(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::Device(e) => e.webgpu_error_type(),
            Self::MissingFeature(e) => e.webgpu_error_type(),
            Self::DestroyedResource(e) => e.webgpu_error_type(),
        }
    }
}

/// Error encountered while trying to use queries
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum QueryUseError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Query {query_index} is out of bounds for a query set of size {query_set_size}")]
    OutOfBounds {
        query_index: u32,
        query_set_size: u32,
    },
    #[error("Query {query_index} has already been used within the same renderpass. Queries must only be used once per renderpass")]
    UsedTwiceInsideRenderpass { query_index: u32 },
    #[error("Query {new_query_index} was started while query {active_query_index} was already active. No more than one statistic or occlusion query may be active at once")]
    AlreadyStarted {
        active_query_index: u32,
        new_query_index: u32,
    },
    #[error("Query was stopped while there was no active query")]
    AlreadyStopped,
    #[error("A query of type {query_type:?} was started using a query set of type {set_type:?}")]
    IncompatibleType {
        set_type: SimplifiedQueryType,
        query_type: SimplifiedQueryType,
    },
    #[error("A query of type {query_type:?} was not ended before the encoder was finished")]
    MissingEnd { query_type: SimplifiedQueryType },
}

impl WebGpuError for QueryUseError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::OutOfBounds { .. }
            | Self::UsedTwiceInsideRenderpass { .. }
            | Self::AlreadyStarted { .. }
            | Self::AlreadyStopped
            | Self::IncompatibleType { .. }
            | Self::MissingEnd { .. } => ErrorType::Validation,
        }
    }
}

/// Error encountered while trying to resolve a query.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum ResolveError {
    #[error(transparent)]
    MissingBufferUsage(#[from] MissingBufferUsageError),
    #[error("Resolve buffer offset has to be aligned to `QUERY_RESOLVE_BUFFER_ALIGNMENT")]
    BufferOffsetAlignment,
    #[error("Resolving queries {start_query}..{end_query} would overrun the query set of size {query_set_size}")]
    QueryOverrun {
        start_query: u32,
        end_query: u64,
        query_set_size: u32,
    },
    #[error("Resolving queries {start_query}..{end_query} ({stride} byte queries) will end up overrunning the bounds of the destination buffer of size {buffer_size} using offsets {buffer_start_offset}..(<start> + {bytes_used})")]
    BufferOverrun {
        start_query: u32,
        end_query: u32,
        stride: u32,
        buffer_size: BufferAddress,
        buffer_start_offset: BufferAddress,
        bytes_used: BufferAddress,
    },
}

impl WebGpuError for ResolveError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::MissingBufferUsage(e) => e.webgpu_error_type(),
            Self::BufferOffsetAlignment
            | Self::QueryOverrun { .. }
            | Self::BufferOverrun { .. } => ErrorType::Validation,
        }
    }
}

impl QuerySet {
    pub(crate) fn validate_query(
        self: &Arc<Self>,
        query_type: SimplifiedQueryType,
        query_index: u32,
        reset_state: Option<&mut QueryResetMap>,
    ) -> Result<(), QueryUseError> {
        // NOTE: Further code assumes the index is good, so do this first.
        if query_index >= self.desc.count {
            return Err(QueryUseError::OutOfBounds {
                query_index,
                query_set_size: self.desc.count,
            });
        }

        // We need to defer our resets because we are in a renderpass,
        // add the usage to the reset map.
        if let Some(reset) = reset_state {
            let used = reset.use_query_set(self, query_index);
            if used {
                return Err(QueryUseError::UsedTwiceInsideRenderpass { query_index });
            }
        }

        let simple_set_type = SimplifiedQueryType::from(self.desc.ty);
        if simple_set_type != query_type {
            return Err(QueryUseError::IncompatibleType {
                query_type,
                set_type: simple_set_type,
            });
        }

        Ok(())
    }

    pub(super) fn validate_and_write_timestamp(
        self: &Arc<Self>,
        raw_encoder: &mut dyn hal::DynCommandEncoder,
        query_index: u32,
        reset_state: Option<&mut QueryResetMap>,
    ) -> Result<(), QueryUseError> {
        let needs_reset = reset_state.is_none();
        self.validate_query(SimplifiedQueryType::Timestamp, query_index, reset_state)?;

        unsafe {
            // If we don't have a reset state tracker which can defer resets, we must reset now.
            if needs_reset {
                raw_encoder.reset_queries(self.raw(), query_index..(query_index + 1));
            }
            raw_encoder.write_timestamp(self.raw(), query_index);
        }

        Ok(())
    }
}

pub(super) fn validate_and_begin_occlusion_query(
    query_set: Arc<QuerySet>,
    raw_encoder: &mut dyn hal::DynCommandEncoder,
    tracker: &mut StatelessTracker<QuerySet>,
    query_index: u32,
    reset_state: Option<&mut QueryResetMap>,
    active_query: &mut Option<(Arc<QuerySet>, u32)>,
) -> Result<(), QueryUseError> {
    let needs_reset = reset_state.is_none();
    query_set.validate_query(SimplifiedQueryType::Occlusion, query_index, reset_state)?;

    tracker.insert_single(query_set.clone());

    if let Some((_old, old_idx)) = active_query.take() {
        return Err(QueryUseError::AlreadyStarted {
            active_query_index: old_idx,
            new_query_index: query_index,
        });
    }
    let (query_set, _) = &active_query.insert((query_set, query_index));

    unsafe {
        // If we don't have a reset state tracker which can defer resets, we must reset now.
        if needs_reset {
            raw_encoder.reset_queries(query_set.raw(), query_index..(query_index + 1));
        }
        raw_encoder.begin_query(query_set.raw(), query_index);
    }

    Ok(())
}

pub(super) fn end_occlusion_query(
    raw_encoder: &mut dyn hal::DynCommandEncoder,
    active_query: &mut Option<(Arc<QuerySet>, u32)>,
) -> Result<(), QueryUseError> {
    if let Some((query_set, query_index)) = active_query.take() {
        unsafe { raw_encoder.end_query(query_set.raw(), query_index) };
        Ok(())
    } else {
        Err(QueryUseError::AlreadyStopped)
    }
}

pub(super) fn validate_and_begin_pipeline_statistics_query(
    query_set: Arc<QuerySet>,
    raw_encoder: &mut dyn hal::DynCommandEncoder,
    tracker: &mut StatelessTracker<QuerySet>,
    device: &Arc<Device>,
    query_index: u32,
    reset_state: Option<&mut QueryResetMap>,
    active_query: &mut Option<(Arc<QuerySet>, u32)>,
) -> Result<(), QueryUseError> {
    query_set.same_device(device)?;

    let needs_reset = reset_state.is_none();
    query_set.validate_query(
        SimplifiedQueryType::PipelineStatistics,
        query_index,
        reset_state,
    )?;

    tracker.insert_single(query_set.clone());

    if let Some((_old, old_idx)) = active_query.take() {
        return Err(QueryUseError::AlreadyStarted {
            active_query_index: old_idx,
            new_query_index: query_index,
        });
    }
    let (query_set, _) = &active_query.insert((query_set, query_index));

    unsafe {
        // If we don't have a reset state tracker which can defer resets, we must reset now.
        if needs_reset {
            raw_encoder.reset_queries(query_set.raw(), query_index..(query_index + 1));
        }
        raw_encoder.begin_query(query_set.raw(), query_index);
    }

    Ok(())
}

pub(super) fn end_pipeline_statistics_query(
    raw_encoder: &mut dyn hal::DynCommandEncoder,
    active_query: &mut Option<(Arc<QuerySet>, u32)>,
) -> Result<(), QueryUseError> {
    if let Some((query_set, query_index)) = active_query.take() {
        unsafe { raw_encoder.end_query(query_set.raw(), query_index) };
        Ok(())
    } else {
        Err(QueryUseError::AlreadyStopped)
    }
}

impl Global {
    pub fn command_encoder_write_timestamp(
        &self,
        command_encoder_id: id::CommandEncoderId,
        query_set_id: id::QuerySetId,
        query_index: u32,
    ) -> Result<(), EncoderStateError> {
        let hub = &self.hub;

        let cmd_enc = hub.command_encoders.get(command_encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();

        cmd_buf_data.push_with(|| -> Result<_, QueryError> {
            Ok(ArcCommand::WriteTimestamp {
                query_set: self.resolve_query_set(query_set_id)?,
                query_index,
            })
        })
    }

    pub fn command_encoder_resolve_query_set(
        &self,
        command_encoder_id: id::CommandEncoderId,
        query_set_id: id::QuerySetId,
        start_query: u32,
        query_count: u32,
        destination: id::BufferId,
        destination_offset: BufferAddress,
    ) -> Result<(), EncoderStateError> {
        let hub = &self.hub;

        let cmd_enc = hub.command_encoders.get(command_encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();

        cmd_buf_data.push_with(|| -> Result<_, QueryError> {
            Ok(ArcCommand::ResolveQuerySet {
                query_set: self.resolve_query_set(query_set_id)?,
                start_query,
                query_count,
                destination: self.resolve_buffer_id(destination)?,
                destination_offset,
            })
        })
    }
}

pub(super) fn write_timestamp(
    state: &mut EncodingState,
    query_set: Arc<QuerySet>,
    query_index: u32,
) -> Result<(), QueryError> {
    state
        .device
        .require_features(wgt::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS)?;

    query_set.same_device(state.device)?;

    query_set.validate_and_write_timestamp(state.raw_encoder, query_index, None)?;

    state.tracker.query_sets.insert_single(query_set);

    Ok(())
}

pub(super) fn resolve_query_set(
    state: &mut EncodingState,
    query_set: Arc<QuerySet>,
    start_query: u32,
    query_count: u32,
    dst_buffer: Arc<Buffer>,
    destination_offset: BufferAddress,
) -> Result<(), QueryError> {
    if !destination_offset.is_multiple_of(wgt::QUERY_RESOLVE_BUFFER_ALIGNMENT) {
        return Err(QueryError::Resolve(ResolveError::BufferOffsetAlignment));
    }

    query_set.same_device(state.device)?;
    dst_buffer.same_device(state.device)?;

    dst_buffer.check_destroyed(state.snatch_guard)?;

    let dst_pending = state
        .tracker
        .buffers
        .set_single(&dst_buffer, wgt::BufferUses::COPY_DST);
    let dst_barrier = dst_pending.map(|pending| pending.into_hal(&dst_buffer, state.snatch_guard));

    dst_buffer
        .check_usage(wgt::BufferUsages::QUERY_RESOLVE)
        .map_err(ResolveError::MissingBufferUsage)?;

    let end_query = u64::from(start_query)
        .checked_add(u64::from(query_count))
        .expect("`u64` overflow from adding two `u32`s, should be unreachable");
    if end_query > u64::from(query_set.desc.count) {
        return Err(ResolveError::QueryOverrun {
            start_query,
            end_query,
            query_set_size: query_set.desc.count,
        }
        .into());
    }
    let end_query =
        u32::try_from(end_query).expect("`u32` overflow for `end_query`, which should be `u32`");

    let elements_per_query = match query_set.desc.ty {
        wgt::QueryType::Occlusion => 1,
        wgt::QueryType::PipelineStatistics(ps) => ps.bits().count_ones(),
        wgt::QueryType::Timestamp => 1,
    };
    let stride = elements_per_query * wgt::QUERY_SIZE;
    let bytes_used: BufferAddress = u64::from(stride)
        .checked_mul(u64::from(query_count))
        .expect("`stride` * `query_count` overflowed `u32`, should be unreachable");

    let buffer_start_offset = destination_offset;
    let buffer_end_offset = buffer_start_offset
        .checked_add(bytes_used)
        .filter(|buffer_end_offset| *buffer_end_offset <= dst_buffer.size)
        .ok_or(ResolveError::BufferOverrun {
            start_query,
            end_query,
            stride,
            buffer_size: dst_buffer.size,
            buffer_start_offset,
            bytes_used,
        })?;

    // TODO(https://github.com/gfx-rs/wgpu/issues/3993): Need to track initialization state.
    state
        .buffer_memory_init_actions
        .extend(dst_buffer.initialization_status.read().create_action(
            &dst_buffer,
            buffer_start_offset..buffer_end_offset,
            MemoryInitKind::ImplicitlyInitialized,
        ));

    let raw_dst_buffer = dst_buffer.try_raw(state.snatch_guard)?;
    unsafe {
        state.raw_encoder.transition_buffers(dst_barrier.as_slice());
        state.raw_encoder.copy_query_results(
            query_set.raw(),
            start_query..end_query,
            raw_dst_buffer,
            destination_offset,
            wgt::BufferSize::new_unchecked(stride as u64),
        );
    }

    if matches!(query_set.desc.ty, wgt::QueryType::Timestamp) {
        // Timestamp normalization is only needed for timestamps.
        state.device.timestamp_normalizer.get().unwrap().normalize(
            state.snatch_guard,
            state.raw_encoder,
            &mut state.tracker.buffers,
            dst_buffer
                .timestamp_normalization_bind_group
                .get(state.snatch_guard)
                .unwrap(),
            &dst_buffer,
            destination_offset,
            query_count,
        );
    }

    state.tracker.query_sets.insert_single(query_set);

    Ok(())
}
