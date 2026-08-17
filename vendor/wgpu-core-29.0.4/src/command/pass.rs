//! Generic pass functions that both compute and render passes need.

use crate::binding_model::{BindError, BindGroup, ImmediateUploadError};
use crate::command::encoder::EncodingState;
use crate::command::{
    bind::Binder, memory_init::SurfacesInDiscardState, query::QueryResetMap, DebugGroupError,
    QueryUseError,
};
use crate::device::{Device, DeviceError, MissingFeatures};
use crate::pipeline::LateSizedBufferGroup;
use crate::resource::{DestroyedResourceError, Labeled, ParentDevice, QuerySet};
use crate::track::{ResourceUsageCompatibilityError, UsageScope};
use crate::{api_log, binding_model};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::str;
use thiserror::Error;
use wgt::error::{ErrorType, WebGpuError};
use wgt::DynamicOffset;

#[derive(Clone, Debug, Error)]
#[error(
    "Bind group index {index} is greater than the device's configured `max_bind_groups` limit {max}"
)]
pub struct BindGroupIndexOutOfRange {
    pub index: u32,
    pub max: u32,
}

#[derive(Clone, Debug, Error)]
#[error("Pipeline must be set")]
pub struct MissingPipeline;

#[derive(Clone, Debug, Error)]
#[error("Setting `values_offset` to be `None` is only for internal use in render bundles")]
pub struct InvalidValuesOffset;

impl WebGpuError for InvalidValuesOffset {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

pub(crate) struct PassState<'scope, 'snatch_guard, 'cmd_enc> {
    pub(crate) base: EncodingState<'snatch_guard, 'cmd_enc>,

    /// Immediate texture inits required because of prior discards. Need to
    /// be inserted before texture reads.
    pub(crate) pending_discard_init_fixups: SurfacesInDiscardState,

    pub(crate) scope: UsageScope<'scope>,

    pub(crate) binder: Binder,

    pub(crate) temp_offsets: Vec<u32>,

    pub(crate) dynamic_offset_count: usize,

    pub(crate) string_offset: usize,
}

pub(crate) fn set_bind_group<E>(
    state: &mut PassState,
    device: &Arc<Device>,
    dynamic_offsets: &[DynamicOffset],
    index: u32,
    num_dynamic_offsets: usize,
    bind_group: Option<Arc<BindGroup>>,
    merge_bind_groups: bool,
) -> Result<(), E>
where
    E: From<DeviceError>
        + From<BindGroupIndexOutOfRange>
        + From<ResourceUsageCompatibilityError>
        + From<DestroyedResourceError>
        + From<BindError>,
{
    if let Some(ref bind_group) = bind_group {
        api_log!("Pass::set_bind_group {index} {}", bind_group.error_ident());
    } else {
        api_log!("Pass::set_bind_group {index} None");
    }

    let max_bind_groups = state.base.device.limits.max_bind_groups;
    if index >= max_bind_groups {
        return Err(BindGroupIndexOutOfRange {
            index,
            max: max_bind_groups,
        }
        .into());
    }

    state.temp_offsets.clear();
    state.temp_offsets.extend_from_slice(
        &dynamic_offsets
            [state.dynamic_offset_count..state.dynamic_offset_count + num_dynamic_offsets],
    );
    state.dynamic_offset_count += num_dynamic_offsets;

    if let Some(bind_group) = bind_group {
        // Add the bind group to the tracker. This is done for both compute and
        // render passes, and is used to fail submission of the command buffer if
        // any resource in any of the bind groups has been destroyed, whether or
        // not the bind group is actually used by the pipeline.
        let bind_group = state.base.tracker.bind_groups.insert_single(bind_group);

        bind_group.same_device(device)?;

        bind_group.validate_dynamic_bindings(index, &state.temp_offsets)?;

        if merge_bind_groups {
            // Merge the bind group's resources into the tracker. We only do this
            // for render passes. For compute passes it is done per dispatch in
            // [`flush_bindings`].
            unsafe {
                state.scope.merge_bind_group(&bind_group.used)?;
            }
        }
        //Note: stateless trackers are not merged: the lifetime reference
        // is held to the bind group itself.

        state
            .binder
            .assign_group(index as usize, bind_group, &state.temp_offsets);
    } else {
        if !state.temp_offsets.is_empty() {
            return Err(BindError::DynamicOffsetCountNotZero {
                group: index,
                actual: state.temp_offsets.len(),
            }
            .into());
        }

        state.binder.clear_group(index as usize);
    };

    Ok(())
}

/// Implementation of `flush_bindings` for both compute and render passes.
///
/// See the compute pass version of `State::flush_bindings` for an explanation
/// of some differences in handling the two types of passes.
pub(super) fn flush_bindings_helper(state: &mut PassState) -> Result<(), DestroyedResourceError> {
    let start = state.binder.take_rebind_start_index();
    let entries = state.binder.list_valid_with_start(start);
    let pipeline_layout = state.binder.pipeline_layout.as_ref().unwrap();

    for (i, bind_group, dynamic_offsets) in entries {
        state.base.buffer_memory_init_actions.extend(
            bind_group.used_buffer_ranges.iter().filter_map(|action| {
                action
                    .buffer
                    .initialization_status
                    .read()
                    .check_action(action)
            }),
        );
        for action in bind_group.used_texture_ranges.iter() {
            state.pending_discard_init_fixups.extend(
                state
                    .base
                    .texture_memory_actions
                    .register_init_action(action),
            );
        }

        let used_resource = bind_group
            .used
            .acceleration_structures
            .into_iter()
            .map(|tlas| crate::ray_tracing::AsAction::UseTlas(tlas.clone()));

        state.base.as_actions.extend(used_resource);

        let raw_bg = bind_group.try_raw(state.base.snatch_guard)?;
        unsafe {
            state.base.raw_encoder.set_bind_group(
                pipeline_layout.raw(),
                i as u32,
                raw_bg,
                dynamic_offsets,
            );
        }
    }

    Ok(())
}

pub(super) fn change_pipeline_layout<E, F: FnOnce()>(
    state: &mut PassState,
    pipeline_layout: &Arc<binding_model::PipelineLayout>,
    late_sized_buffer_groups: &[LateSizedBufferGroup],
    f: F,
) -> Result<(), E>
where
    E: From<DestroyedResourceError>,
{
    if state
        .binder
        .change_pipeline_layout(pipeline_layout, late_sized_buffer_groups)
    {
        f();

        super::immediates_clear(
            0,
            pipeline_layout.immediate_size,
            |clear_offset, clear_data| unsafe {
                state.base.raw_encoder.set_immediates(
                    pipeline_layout.raw(),
                    clear_offset,
                    clear_data,
                );
            },
        );
    }
    Ok(())
}

pub(crate) fn set_immediates<E, F: FnOnce(&[u32])>(
    state: &mut PassState,
    immediates_data: &[u32],
    offset: u32,
    size_bytes: u32,
    values_offset: Option<u32>,
    f: F,
) -> Result<(), E>
where
    E: From<ImmediateUploadError> + From<InvalidValuesOffset> + From<MissingPipeline>,
{
    api_log!("Pass::set_immediates");

    let values_offset = values_offset.ok_or(InvalidValuesOffset)?;

    let pipeline_layout = state
        .binder
        .pipeline_layout
        .as_ref()
        .ok_or(MissingPipeline)?;

    pipeline_layout.validate_immediates_ranges(offset, size_bytes)?;

    let values_offset_usize = usize::try_from(values_offset)
        .expect("`values_offset` is outside the bounds of `usize` (!?)");
    if values_offset_usize > immediates_data.len() {
        return Err(ImmediateUploadError::ValueStartIndexOverrun {
            start_index: values_offset,
            data_size: immediates_data.len(),
        }
        .into());
    }

    // NOTE: The `validate_immediates_ranges` call above validates `size_bytes` is aligned.
    let size_immediate_elements = size_bytes / wgt::IMMEDIATE_DATA_ALIGNMENT;
    let size_immediate_elements_usize = usize::try_from(size_immediate_elements)
        .expect("`size_immediate_elements` is outside the bounds of `usize` (!?)");
    if size_immediate_elements_usize > immediates_data.len() - values_offset_usize {
        return Err(ImmediateUploadError::ValueEndIndexOverrun {
            start_index: values_offset,
            count: size_immediate_elements,
            data_size: immediates_data.len(),
        }
        .into());
    }

    // NOTE: These additions are will not overflow, because we've validated the range above.
    let values_end_offset = values_offset_usize + size_immediate_elements_usize;
    let data_slice = &immediates_data[(values_offset_usize)..values_end_offset];

    f(data_slice);

    unsafe {
        state
            .base
            .raw_encoder
            .set_immediates(pipeline_layout.raw(), offset, data_slice)
    }
    Ok(())
}

pub(crate) fn write_timestamp<E>(
    state: &mut PassState,
    device: &Arc<Device>,
    pending_query_resets: Option<&mut QueryResetMap>,
    query_set: Arc<QuerySet>,
    query_index: u32,
) -> Result<(), E>
where
    E: From<MissingFeatures> + From<QueryUseError> + From<DeviceError>,
{
    api_log!(
        "Pass::write_timestamps {query_index} {}",
        query_set.error_ident()
    );

    query_set.same_device(device)?;

    state
        .base
        .device
        .require_features(wgt::Features::TIMESTAMP_QUERY_INSIDE_PASSES)?;

    let query_set = state.base.tracker.query_sets.insert_single(query_set);

    query_set.validate_and_write_timestamp(
        state.base.raw_encoder,
        query_index,
        pending_query_resets,
    )?;
    Ok(())
}

pub(crate) fn push_debug_group(state: &mut PassState, string_data: &[u8], len: usize) {
    *state.base.debug_scope_depth += 1;
    if !state
        .base
        .device
        .instance_flags
        .contains(wgt::InstanceFlags::DISCARD_HAL_LABELS)
    {
        let label =
            str::from_utf8(&string_data[state.string_offset..state.string_offset + len]).unwrap();

        api_log!("Pass::push_debug_group {label:?}");
        unsafe {
            state.base.raw_encoder.begin_debug_marker(label);
        }
    }
    state.string_offset += len;
}

pub(crate) fn pop_debug_group<E>(state: &mut PassState) -> Result<(), E>
where
    E: From<DebugGroupError>,
{
    api_log!("Pass::pop_debug_group");

    if *state.base.debug_scope_depth == 0 {
        return Err(DebugGroupError::InvalidPop.into());
    }
    *state.base.debug_scope_depth -= 1;
    if !state
        .base
        .device
        .instance_flags
        .contains(wgt::InstanceFlags::DISCARD_HAL_LABELS)
    {
        unsafe {
            state.base.raw_encoder.end_debug_marker();
        }
    }
    Ok(())
}

pub(crate) fn insert_debug_marker(state: &mut PassState, string_data: &[u8], len: usize) {
    if !state
        .base
        .device
        .instance_flags
        .contains(wgt::InstanceFlags::DISCARD_HAL_LABELS)
    {
        let label =
            str::from_utf8(&string_data[state.string_offset..state.string_offset + len]).unwrap();
        api_log!("Pass::insert_debug_marker {label:?}");
        unsafe {
            state.base.raw_encoder.insert_debug_marker(label);
        }
    }
    state.string_offset += len;
}
