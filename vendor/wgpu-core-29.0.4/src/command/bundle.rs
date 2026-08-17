/*! Render Bundles

A render bundle is a prerecorded sequence of commands that can be replayed on a
command encoder with a single call. A single bundle can replayed any number of
times, on different encoders. Constructing a render bundle lets `wgpu` validate
and analyze its commands up front, so that replaying a bundle can be more
efficient than simply re-recording its commands each time.

Not all commands are available in bundles; for example, a render bundle may not
contain a [`RenderCommand::SetViewport`] command.

Most of `wgpu`'s backend graphics APIs have something like bundles. For example,
Vulkan calls them "secondary command buffers", and Metal calls them "indirect
command buffers". Although we plan to take advantage of these platform features
at some point in the future, for now `wgpu`'s implementation of render bundles
does not use them: at the hal level, `wgpu` render bundles just replay the
commands.

## Render Bundle Isolation

One important property of render bundles is that the draw calls in a render
bundle depend solely on the pipeline and state established within the render
bundle itself. A draw call in a bundle will never use a vertex buffer, say, that
was set in the `RenderPass` before executing the bundle. We call this property
'isolation', in that a render bundle is somewhat isolated from the passes that
use it.

Render passes are also isolated from the effects of bundles. After executing a
render bundle, a render pass's pipeline, bind groups, and vertex and index
buffers are are unset, so the bundle cannot affect later draw calls in the pass.

A render pass is not fully isolated from a bundle's effects on immediate data
values. Draw calls following a bundle's execution will see whatever values the
bundle writes to immediate data storage. Setting a pipeline initializes any push
constant storage it could access to zero, and this initialization may also be
visible after bundle execution.

## Render Bundle Lifecycle

To create a render bundle:

1) Create a [`RenderBundleEncoder`] by calling
   [`Global::device_create_render_bundle_encoder`][Gdcrbe].

2) Record commands in the `RenderBundleEncoder` using functions from the
   [`bundle_ffi`] module.

3) Call [`Global::render_bundle_encoder_finish`][Grbef], which analyzes and cleans up
   the command stream and returns a `RenderBundleId`.

4) Then, any number of times, call [`render_pass_execute_bundles`][wrpeb] to
   execute the bundle as part of some render pass.

## Implementation

The most complex part of render bundles is the "finish" step, mostly implemented
in [`RenderBundleEncoder::finish`]. This consumes the commands stored in the
encoder's [`BasePass`], while validating everything, tracking the state,
dropping redundant or unnecessary commands, and presenting the results as a new
[`RenderBundle`]. It doesn't actually execute any commands.

This step also enforces the 'isolation' property mentioned above: every draw
call is checked to ensure that the resources it uses on were established since
the last time the pipeline was set. This means the bundle can be executed
verbatim without any state tracking.

### Execution

When the bundle is used in an actual render pass, `RenderBundle::execute` is
called. It goes through the commands and issues them into the native command
buffer. Thanks to isolation, it doesn't track any bind group invalidations or
index format changes.

[Gdcrbe]: crate::global::Global::device_create_render_bundle_encoder
[Grbef]: crate::global::Global::render_bundle_encoder_finish
[wrpeb]: crate::global::Global::render_pass_execute_bundles
!*/

#![allow(clippy::reversed_empty_ranges)]

use alloc::{
    borrow::{Cow, ToOwned as _},
    string::String,
    sync::Arc,
    vec::Vec,
};
use core::{
    convert::Infallible,
    num::{NonZeroU32, NonZeroU64},
    ops::Range,
};

use arrayvec::ArrayVec;
use thiserror::Error;

use wgpu_hal::ShouldBeNonZeroExt;
use wgt::error::{ErrorType, WebGpuError};

#[cfg(feature = "trace")]
use crate::command::ArcReferences;
use crate::{
    binding_model::{BindError, BindGroup, PipelineLayout},
    command::{
        bind::Binder, BasePass, BindGroupStateChange, ColorAttachmentError, DrawError,
        IdReferences, MapPassErr, PassErrorScope, RenderCommand, RenderCommandError, StateChange,
    },
    device::{AttachmentData, Device, DeviceError, MissingDownlevelFlags, RenderPassContext},
    hub::Hub,
    id,
    init_tracker::{BufferInitTrackerAction, MemoryInitKind, TextureInitTrackerAction},
    pipeline::{PipelineFlags, RenderPipeline, VertexStep},
    resource::{
        Buffer, DestroyedResourceError, Fallible, InvalidResourceError, Labeled, ParentDevice,
        RawResourceAccess, TrackingData,
    },
    resource_log,
    snatch::SnatchGuard,
    track::RenderBundleScope,
    Label, LabelHelpers,
};

use super::{pass, render_command::ArcRenderCommand, DrawCommandFamily, DrawKind};

/// Describes a [`RenderBundleEncoder`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderBundleEncoderDescriptor<'a> {
    /// Debug label of the render bundle encoder.
    ///
    /// This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// The formats of the color attachments that this render bundle is capable
    /// to rendering to.
    ///
    /// This must match the formats of the color attachments in the
    /// renderpass this render bundle is executed in.
    pub color_formats: Cow<'a, [Option<wgt::TextureFormat>]>,
    /// Information about the depth attachment that this render bundle is
    /// capable to rendering to.
    ///
    /// The format must match the format of the depth attachments in the
    /// renderpass this render bundle is executed in.
    pub depth_stencil: Option<wgt::RenderBundleDepthStencil>,
    /// Sample count this render bundle is capable of rendering to.
    ///
    /// This must match the pipelines and the renderpasses it is used in.
    pub sample_count: u32,
    /// If this render bundle will rendering to multiple array layers in the
    /// attachments at the same time.
    pub multiview: Option<NonZeroU32>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RenderBundleEncoder {
    base: BasePass<RenderCommand<IdReferences>, Infallible>,
    parent_id: id::DeviceId,
    pub(crate) context: RenderPassContext,
    pub(crate) is_depth_read_only: bool,
    pub(crate) is_stencil_read_only: bool,

    // Resource binding dedupe state.
    #[cfg_attr(feature = "serde", serde(skip))]
    current_bind_groups: BindGroupStateChange,
    #[cfg_attr(feature = "serde", serde(skip))]
    current_pipeline: StateChange<id::RenderPipelineId>,
}

impl RenderBundleEncoder {
    pub fn new(
        desc: &RenderBundleEncoderDescriptor,
        parent_id: id::DeviceId,
    ) -> Result<Self, CreateRenderBundleError> {
        let (is_depth_read_only, is_stencil_read_only) = match desc.depth_stencil {
            Some(ds) => {
                let aspects = hal::FormatAspects::from(ds.format);
                (
                    !aspects.contains(hal::FormatAspects::DEPTH) || ds.depth_read_only,
                    !aspects.contains(hal::FormatAspects::STENCIL) || ds.stencil_read_only,
                )
            }
            // There's no depth/stencil attachment, so these values just don't
            // matter.  Choose the most accommodating value, to simplify
            // validation.
            None => (true, true),
        };

        // TODO: should be device.limits.max_color_attachments
        let max_color_attachments = hal::MAX_COLOR_ATTACHMENTS;

        //TODO: validate that attachment formats are renderable,
        // have expected aspects, support multisampling.
        Ok(Self {
            base: BasePass::new(&desc.label),
            parent_id,
            context: RenderPassContext {
                attachments: AttachmentData {
                    colors: if desc.color_formats.len() > max_color_attachments {
                        return Err(CreateRenderBundleError::ColorAttachment(
                            ColorAttachmentError::TooMany {
                                given: desc.color_formats.len(),
                                limit: max_color_attachments,
                            },
                        ));
                    } else {
                        desc.color_formats.iter().cloned().collect()
                    },
                    resolves: ArrayVec::new(),
                    depth_stencil: desc.depth_stencil.map(|ds| ds.format),
                },
                sample_count: {
                    let sc = desc.sample_count;
                    if sc == 0 || sc > 32 || !sc.is_power_of_two() {
                        return Err(CreateRenderBundleError::InvalidSampleCount(sc));
                    }
                    sc
                },
                multiview_mask: desc.multiview,
            },

            is_depth_read_only,
            is_stencil_read_only,
            current_bind_groups: BindGroupStateChange::new(),
            current_pipeline: StateChange::new(),
        })
    }

    pub fn dummy(parent_id: id::DeviceId) -> Self {
        Self {
            base: BasePass::new(&None),
            parent_id,
            context: RenderPassContext {
                attachments: AttachmentData {
                    colors: ArrayVec::new(),
                    resolves: ArrayVec::new(),
                    depth_stencil: None,
                },
                sample_count: 0,
                multiview_mask: None,
            },
            is_depth_read_only: false,
            is_stencil_read_only: false,

            current_bind_groups: BindGroupStateChange::new(),
            current_pipeline: StateChange::new(),
        }
    }

    pub fn parent(&self) -> id::DeviceId {
        self.parent_id
    }

    /// Convert this encoder's commands into a [`RenderBundle`].
    ///
    /// We want executing a [`RenderBundle`] to be quick, so we take
    /// this opportunity to clean up the [`RenderBundleEncoder`]'s
    /// command stream and gather metadata about it that will help
    /// keep [`ExecuteBundle`] simple and fast. We remove redundant
    /// commands (along with their side data), note resource usage,
    /// and accumulate buffer and texture initialization actions.
    ///
    /// [`ExecuteBundle`]: RenderCommand::ExecuteBundle
    pub(crate) fn finish(
        self,
        desc: &RenderBundleDescriptor,
        device: &Arc<Device>,
        hub: &Hub,
    ) -> Result<Arc<RenderBundle>, RenderBundleError> {
        let scope = PassErrorScope::Bundle;

        device.check_is_valid().map_pass_err(scope)?;

        let bind_group_guard = hub.bind_groups.read();
        let pipeline_guard = hub.render_pipelines.read();
        let buffer_guard = hub.buffers.read();

        let mut state = State {
            trackers: RenderBundleScope::new(),
            pipeline: None,
            vertex: Default::default(),
            index: None,
            flat_dynamic_offsets: Vec::new(),
            device: device.clone(),
            commands: Vec::new(),
            buffer_memory_init_actions: Vec::new(),
            texture_memory_init_actions: Vec::new(),
            next_dynamic_offset: 0,
            binder: Binder::new(),
        };

        let indices = &state.device.tracker_indices;
        state.trackers.buffers.set_size(indices.buffers.size());
        state.trackers.textures.set_size(indices.textures.size());

        let base = &self.base;

        for command in &base.commands {
            match command {
                &RenderCommand::SetBindGroup {
                    index,
                    num_dynamic_offsets,
                    bind_group,
                } => {
                    let scope = PassErrorScope::SetBindGroup;
                    set_bind_group(
                        &mut state,
                        &bind_group_guard,
                        &base.dynamic_offsets,
                        index,
                        num_dynamic_offsets,
                        bind_group,
                    )
                    .map_pass_err(scope)?;
                }
                &RenderCommand::SetPipeline(pipeline) => {
                    let scope = PassErrorScope::SetPipelineRender;
                    set_pipeline(
                        &mut state,
                        &pipeline_guard,
                        &self.context,
                        self.is_depth_read_only,
                        self.is_stencil_read_only,
                        pipeline,
                    )
                    .map_pass_err(scope)?;
                }
                &RenderCommand::SetIndexBuffer {
                    buffer,
                    index_format,
                    offset,
                    size,
                } => {
                    let scope = PassErrorScope::SetIndexBuffer;
                    set_index_buffer(
                        &mut state,
                        &buffer_guard,
                        buffer,
                        index_format,
                        offset,
                        size,
                    )
                    .map_pass_err(scope)?;
                }
                &RenderCommand::SetVertexBuffer {
                    slot,
                    buffer,
                    offset,
                    size,
                } => {
                    let scope = PassErrorScope::SetVertexBuffer;
                    set_vertex_buffer(&mut state, &buffer_guard, slot, buffer, offset, size)
                        .map_pass_err(scope)?;
                }
                &RenderCommand::SetImmediate {
                    offset,
                    size_bytes,
                    values_offset,
                } => {
                    let scope = PassErrorScope::SetImmediate;
                    set_immediates(&mut state, offset, size_bytes, values_offset)
                        .map_pass_err(scope)?;
                }
                &RenderCommand::Draw {
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: DrawKind::Draw,
                        family: DrawCommandFamily::Draw,
                    };
                    draw(
                        &mut state,
                        vertex_count,
                        instance_count,
                        first_vertex,
                        first_instance,
                    )
                    .map_pass_err(scope)?;
                }
                &RenderCommand::DrawIndexed {
                    index_count,
                    instance_count,
                    first_index,
                    base_vertex,
                    first_instance,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: DrawKind::Draw,
                        family: DrawCommandFamily::DrawIndexed,
                    };
                    draw_indexed(
                        &mut state,
                        index_count,
                        instance_count,
                        first_index,
                        base_vertex,
                        first_instance,
                    )
                    .map_pass_err(scope)?;
                }
                &RenderCommand::DrawMeshTasks {
                    group_count_x,
                    group_count_y,
                    group_count_z,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: DrawKind::Draw,
                        family: DrawCommandFamily::DrawMeshTasks,
                    };
                    draw_mesh_tasks(&mut state, group_count_x, group_count_y, group_count_z)
                        .map_pass_err(scope)?;
                }
                &RenderCommand::DrawIndirect {
                    buffer,
                    offset,
                    count: 1,
                    family,
                    vertex_or_index_limit: None,
                    instance_limit: None,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: DrawKind::DrawIndirect,
                        family,
                    };
                    multi_draw_indirect(&mut state, &buffer_guard, buffer, offset, family)
                        .map_pass_err(scope)?;
                }
                &RenderCommand::DrawIndirect {
                    count,
                    vertex_or_index_limit,
                    instance_limit,
                    ..
                } => {
                    unreachable!("unexpected (multi-)draw indirect with count {count}, vertex_or_index_limits {vertex_or_index_limit:?}, instance_limit {instance_limit:?} found in a render bundle");
                }
                &RenderCommand::MultiDrawIndirectCount { .. }
                | &RenderCommand::PushDebugGroup { color: _, len: _ }
                | &RenderCommand::InsertDebugMarker { color: _, len: _ }
                | &RenderCommand::PopDebugGroup => {
                    unimplemented!("not supported by a render bundle")
                }
                // Must check the TIMESTAMP_QUERY_INSIDE_PASSES feature
                &RenderCommand::WriteTimestamp { .. }
                | &RenderCommand::BeginOcclusionQuery { .. }
                | &RenderCommand::EndOcclusionQuery
                | &RenderCommand::BeginPipelineStatisticsQuery { .. }
                | &RenderCommand::EndPipelineStatisticsQuery => {
                    unimplemented!("not supported by a render bundle")
                }
                &RenderCommand::ExecuteBundle(_)
                | &RenderCommand::SetBlendConstant(_)
                | &RenderCommand::SetStencilReference(_)
                | &RenderCommand::SetViewport { .. }
                | &RenderCommand::SetScissor(_) => unreachable!("not supported by a render bundle"),
            }
        }

        let State {
            trackers,
            flat_dynamic_offsets,
            device,
            commands,
            buffer_memory_init_actions,
            texture_memory_init_actions,
            ..
        } = state;

        let tracker_indices = device.tracker_indices.bundles.clone();
        let discard_hal_labels = device
            .instance_flags
            .contains(wgt::InstanceFlags::DISCARD_HAL_LABELS);

        let render_bundle = RenderBundle {
            base: BasePass {
                label: desc.label.as_deref().map(str::to_owned),
                error: None,
                commands,
                dynamic_offsets: flat_dynamic_offsets,
                string_data: self.base.string_data,
                immediates_data: self.base.immediates_data,
            },
            is_depth_read_only: self.is_depth_read_only,
            is_stencil_read_only: self.is_stencil_read_only,
            device: device.clone(),
            used: trackers,
            buffer_memory_init_actions,
            texture_memory_init_actions,
            context: self.context,
            label: desc.label.to_string(),
            tracking_data: TrackingData::new(tracker_indices),
            discard_hal_labels,
        };

        let render_bundle = Arc::new(render_bundle);

        Ok(render_bundle)
    }

    pub fn set_index_buffer(
        &mut self,
        buffer: id::BufferId,
        index_format: wgt::IndexFormat,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferSize>,
    ) {
        self.base.commands.push(RenderCommand::SetIndexBuffer {
            buffer,
            index_format,
            offset,
            size,
        });
    }
}

fn set_bind_group(
    state: &mut State,
    bind_group_guard: &crate::storage::Storage<Fallible<BindGroup>>,
    dynamic_offsets: &[u32],
    index: u32,
    num_dynamic_offsets: usize,
    bind_group_id: Option<id::Id<id::markers::BindGroup>>,
) -> Result<(), RenderBundleErrorInner> {
    let max_bind_groups = state.device.limits.max_bind_groups;
    if index >= max_bind_groups {
        return Err(
            RenderCommandError::BindGroupIndexOutOfRange(pass::BindGroupIndexOutOfRange {
                index,
                max: max_bind_groups,
            })
            .into(),
        );
    }

    // Identify the next `num_dynamic_offsets` entries from `dynamic_offsets`.
    let offsets_range = state.next_dynamic_offset..state.next_dynamic_offset + num_dynamic_offsets;
    state.next_dynamic_offset = offsets_range.end;
    let offsets = &dynamic_offsets[offsets_range.clone()];

    let bind_group = bind_group_id.map(|id| bind_group_guard.get(id));

    if let Some(bind_group) = bind_group {
        let bind_group = bind_group.get()?;
        bind_group.same_device(&state.device)?;
        bind_group.validate_dynamic_bindings(index, offsets)?;

        unsafe { state.trackers.merge_bind_group(&bind_group.used)? };
        let bind_group = state.trackers.bind_groups.insert_single(bind_group);

        state
            .binder
            .assign_group(index as usize, bind_group, offsets);
    } else {
        if !offsets.is_empty() {
            return Err(RenderBundleErrorInner::Bind(
                BindError::DynamicOffsetCountNotZero {
                    group: index,
                    actual: offsets.len(),
                },
            ));
        }

        state.binder.clear_group(index as usize);
    }

    Ok(())
}

fn set_pipeline(
    state: &mut State,
    pipeline_guard: &crate::storage::Storage<Fallible<RenderPipeline>>,
    context: &RenderPassContext,
    is_depth_read_only: bool,
    is_stencil_read_only: bool,
    pipeline_id: id::Id<id::markers::RenderPipeline>,
) -> Result<(), RenderBundleErrorInner> {
    let pipeline = pipeline_guard.get(pipeline_id).get()?;

    pipeline.same_device(&state.device)?;

    context
        .check_compatible(&pipeline.pass_context, pipeline.as_ref())
        .map_err(RenderCommandError::IncompatiblePipelineTargets)?;

    if pipeline.flags.contains(PipelineFlags::WRITES_DEPTH) && is_depth_read_only {
        return Err(RenderCommandError::IncompatibleDepthAccess(pipeline.error_ident()).into());
    }
    if pipeline.flags.contains(PipelineFlags::WRITES_STENCIL) && is_stencil_read_only {
        return Err(RenderCommandError::IncompatibleStencilAccess(pipeline.error_ident()).into());
    }

    let pipeline_state = PipelineState::new(&pipeline);

    state
        .commands
        .push(ArcRenderCommand::SetPipeline(pipeline.clone()));

    // If this pipeline uses immediates, zero out their values.
    if let Some(cmd) = pipeline_state.zero_immediates() {
        state.commands.push(cmd);
    }

    state.pipeline = Some(pipeline_state);

    state
        .binder
        .change_pipeline_layout(&pipeline.layout, &pipeline.late_sized_buffer_groups);

    state.trackers.render_pipelines.insert_single(pipeline);
    Ok(())
}

// This function is duplicative of `render::set_index_buffer`.
fn set_index_buffer(
    state: &mut State,
    buffer_guard: &crate::storage::Storage<Fallible<Buffer>>,
    buffer_id: id::Id<id::markers::Buffer>,
    index_format: wgt::IndexFormat,
    offset: u64,
    size: Option<NonZeroU64>,
) -> Result<(), RenderBundleErrorInner> {
    let buffer = buffer_guard.get(buffer_id).get()?;

    state
        .trackers
        .buffers
        .merge_single(&buffer, wgt::BufferUses::INDEX)?;

    buffer.same_device(&state.device)?;
    buffer.check_usage(wgt::BufferUsages::INDEX)?;

    if !offset.is_multiple_of(u64::try_from(index_format.byte_size()).unwrap()) {
        return Err(RenderCommandError::UnalignedIndexBuffer {
            offset,
            alignment: index_format.byte_size(),
        }
        .into());
    }
    let end = offset + buffer.resolve_binding_size(offset, size)?;

    state
        .buffer_memory_init_actions
        .extend(buffer.initialization_status.read().create_action(
            &buffer,
            offset..end.get(),
            MemoryInitKind::NeedsInitializedMemory,
        ));
    state.set_index_buffer(buffer, index_format, offset..end.get());
    Ok(())
}

// This function is duplicative of `render::set_vertex_buffer`.
fn set_vertex_buffer(
    state: &mut State,
    buffer_guard: &crate::storage::Storage<Fallible<Buffer>>,
    slot: u32,
    buffer_id: id::Id<id::markers::Buffer>,
    offset: u64,
    size: Option<NonZeroU64>,
) -> Result<(), RenderBundleErrorInner> {
    let max_vertex_buffers = state.device.limits.max_vertex_buffers;
    if slot >= max_vertex_buffers {
        return Err(RenderCommandError::VertexBufferIndexOutOfRange {
            index: slot,
            max: max_vertex_buffers,
        }
        .into());
    }

    let buffer = buffer_guard.get(buffer_id).get()?;

    state
        .trackers
        .buffers
        .merge_single(&buffer, wgt::BufferUses::VERTEX)?;

    buffer.same_device(&state.device)?;
    buffer.check_usage(wgt::BufferUsages::VERTEX)?;

    if !offset.is_multiple_of(wgt::VERTEX_ALIGNMENT) {
        return Err(RenderCommandError::UnalignedVertexBuffer { slot, offset }.into());
    }
    let end = offset + buffer.resolve_binding_size(offset, size)?;

    state
        .buffer_memory_init_actions
        .extend(buffer.initialization_status.read().create_action(
            &buffer,
            offset..end.get(),
            MemoryInitKind::NeedsInitializedMemory,
        ));
    state.vertex[slot as usize] = Some(VertexState::new(buffer, offset..end.get()));
    Ok(())
}

fn set_immediates(
    state: &mut State,
    offset: u32,
    size_bytes: u32,
    values_offset: Option<u32>,
) -> Result<(), RenderBundleErrorInner> {
    let pipeline_state = state.pipeline()?;

    pipeline_state
        .pipeline
        .layout
        .validate_immediates_ranges(offset, size_bytes)?;

    state.commands.push(ArcRenderCommand::SetImmediate {
        offset,
        size_bytes,
        values_offset,
    });
    Ok(())
}

fn draw(
    state: &mut State,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) -> Result<(), RenderBundleErrorInner> {
    state.is_ready(DrawCommandFamily::Draw)?;
    let pipeline = state.pipeline()?;

    let vertex_limits = super::VertexLimits::new(state.vertex_buffer_sizes(), &pipeline.steps);
    vertex_limits.validate_vertex_limit(first_vertex, vertex_count)?;
    vertex_limits.validate_instance_limit(first_instance, instance_count)?;

    if instance_count > 0 && vertex_count > 0 {
        state.flush_vertices();
        state.flush_bindings();
        state.commands.push(ArcRenderCommand::Draw {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        });
    }
    Ok(())
}

fn draw_indexed(
    state: &mut State,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) -> Result<(), RenderBundleErrorInner> {
    state.is_ready(DrawCommandFamily::DrawIndexed)?;
    let pipeline = state.pipeline()?;

    let index = state.index.as_ref().unwrap();

    let vertex_limits = super::VertexLimits::new(state.vertex_buffer_sizes(), &pipeline.steps);

    let last_index = first_index as u64 + index_count as u64;
    let index_limit = index.limit();
    if last_index > index_limit {
        return Err(DrawError::IndexBeyondLimit {
            last_index,
            index_limit,
        }
        .into());
    }
    vertex_limits.validate_instance_limit(first_instance, instance_count)?;

    if instance_count > 0 && index_count > 0 {
        state.flush_index();
        state.flush_vertices();
        state.flush_bindings();
        state.commands.push(ArcRenderCommand::DrawIndexed {
            index_count,
            instance_count,
            first_index,
            base_vertex,
            first_instance,
        });
    }
    Ok(())
}

fn draw_mesh_tasks(
    state: &mut State,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) -> Result<(), RenderBundleErrorInner> {
    state.is_ready(DrawCommandFamily::DrawMeshTasks)?;

    let groups_size_limit = state.device.limits.max_task_mesh_workgroups_per_dimension;
    let max_groups = state.device.limits.max_task_mesh_workgroup_total_count;
    if group_count_x > groups_size_limit
        || group_count_y > groups_size_limit
        || group_count_z > groups_size_limit
        || group_count_x * group_count_y * group_count_z > max_groups
    {
        return Err(RenderBundleErrorInner::Draw(DrawError::InvalidGroupSize {
            current: [group_count_x, group_count_y, group_count_z],
            limit: groups_size_limit,
            max_total: max_groups,
        }));
    }

    if group_count_x > 0 && group_count_y > 0 && group_count_z > 0 {
        state.flush_bindings();
        state.commands.push(ArcRenderCommand::DrawMeshTasks {
            group_count_x,
            group_count_y,
            group_count_z,
        });
    }
    Ok(())
}

fn multi_draw_indirect(
    state: &mut State,
    buffer_guard: &crate::storage::Storage<Fallible<Buffer>>,
    buffer_id: id::Id<id::markers::Buffer>,
    offset: u64,
    family: DrawCommandFamily,
) -> Result<(), RenderBundleErrorInner> {
    state.is_ready(family)?;
    state
        .device
        .require_downlevel_flags(wgt::DownlevelFlags::INDIRECT_EXECUTION)?;

    let pipeline = state.pipeline()?;

    let buffer = buffer_guard.get(buffer_id).get()?;

    buffer.same_device(&state.device)?;
    buffer.check_usage(wgt::BufferUsages::INDIRECT)?;

    let vertex_limits = super::VertexLimits::new(state.vertex_buffer_sizes(), &pipeline.steps);

    let stride = super::get_src_stride_of_indirect_args(family);
    state
        .buffer_memory_init_actions
        .extend(buffer.initialization_status.read().create_action(
            &buffer,
            offset..(offset + stride),
            MemoryInitKind::NeedsInitializedMemory,
        ));

    let vertex_or_index_limit = if family == DrawCommandFamily::DrawIndexed {
        let index = state.index.as_mut().unwrap();
        state.commands.extend(index.flush());
        index.limit()
    } else {
        vertex_limits.vertex_limit
    };
    let instance_limit = vertex_limits.instance_limit;

    let buffer_uses = if state.device.indirect_validation.is_some() {
        wgt::BufferUses::STORAGE_READ_ONLY
    } else {
        wgt::BufferUses::INDIRECT
    };

    state.trackers.buffers.merge_single(&buffer, buffer_uses)?;

    state.flush_vertices();
    state.flush_bindings();
    state.commands.push(ArcRenderCommand::DrawIndirect {
        buffer,
        offset,
        count: 1,
        family,

        vertex_or_index_limit: Some(vertex_or_index_limit),
        instance_limit: Some(instance_limit),
    });
    Ok(())
}

/// Error type returned from `RenderBundleEncoder::new` if the sample count is invalid.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateRenderBundleError {
    #[error(transparent)]
    ColorAttachment(#[from] ColorAttachmentError),
    #[error("Invalid number of samples {0}")]
    InvalidSampleCount(u32),
}

impl WebGpuError for CreateRenderBundleError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::ColorAttachment(e) => e.webgpu_error_type(),
            Self::InvalidSampleCount(_) => ErrorType::Validation,
        }
    }
}

/// Error type returned from `RenderBundleEncoder::new` if the sample count is invalid.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum ExecutionError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),
    #[error("Using {0} in a render bundle is not implemented")]
    Unimplemented(&'static str),
}

pub type RenderBundleDescriptor<'a> = wgt::RenderBundleDescriptor<Label<'a>>;

//Note: here, `RenderBundle` is just wrapping a raw stream of render commands.
// The plan is to back it by an actual Vulkan secondary buffer, D3D12 Bundle,
// or Metal indirect command buffer.
/// cbindgen:ignore
#[derive(Debug)]
pub struct RenderBundle {
    // Normalized command stream. It can be executed verbatim,
    // without re-binding anything on the pipeline change.
    base: BasePass<ArcRenderCommand, Infallible>,
    pub(super) is_depth_read_only: bool,
    pub(super) is_stencil_read_only: bool,
    pub(crate) device: Arc<Device>,
    pub(crate) used: RenderBundleScope,
    pub(super) buffer_memory_init_actions: Vec<BufferInitTrackerAction>,
    pub(super) texture_memory_init_actions: Vec<TextureInitTrackerAction>,
    pub(super) context: RenderPassContext,
    /// The `label` from the descriptor used to create the resource.
    label: String,
    pub(crate) tracking_data: TrackingData,
    discard_hal_labels: bool,
}

impl Drop for RenderBundle {
    fn drop(&mut self) {
        resource_log!("Drop {}", self.error_ident());
    }
}

#[cfg(send_sync)]
unsafe impl Send for RenderBundle {}
#[cfg(send_sync)]
unsafe impl Sync for RenderBundle {}

impl RenderBundle {
    #[cfg(feature = "trace")]
    pub(crate) fn to_base_pass(&self) -> BasePass<RenderCommand<ArcReferences>, Infallible> {
        self.base.clone()
    }

    /// Actually encode the contents into a native command buffer.
    ///
    /// This is partially duplicating the logic of `render_pass_end`.
    /// However the point of this function is to be lighter, since we already had
    /// a chance to go through the commands in `render_bundle_encoder_finish`.
    ///
    /// Note that the function isn't expected to fail, generally.
    /// All the validation has already been done by this point.
    /// The only failure condition is if some of the used buffers are destroyed.
    pub(super) unsafe fn execute(
        &self,
        raw: &mut dyn hal::DynCommandEncoder,
        indirect_draw_validation_resources: &mut crate::indirect_validation::DrawResources,
        indirect_draw_validation_batcher: &mut crate::indirect_validation::DrawBatcher,
        snatch_guard: &SnatchGuard,
    ) -> Result<(), ExecutionError> {
        let mut offsets = self.base.dynamic_offsets.as_slice();
        let mut pipeline_layout = None::<Arc<PipelineLayout>>;
        if !self.discard_hal_labels {
            if let Some(ref label) = self.base.label {
                unsafe { raw.begin_debug_marker(label) };
            }
        }

        use ArcRenderCommand as Cmd;
        for command in self.base.commands.iter() {
            match command {
                Cmd::SetBindGroup {
                    index,
                    num_dynamic_offsets,
                    bind_group,
                } => {
                    let raw_bg = bind_group.as_ref().unwrap().try_raw(snatch_guard)?;
                    unsafe {
                        raw.set_bind_group(
                            pipeline_layout.as_ref().unwrap().raw(),
                            *index,
                            raw_bg,
                            &offsets[..*num_dynamic_offsets],
                        )
                    };
                    offsets = &offsets[*num_dynamic_offsets..];
                }
                Cmd::SetPipeline(pipeline) => {
                    unsafe { raw.set_render_pipeline(pipeline.raw()) };

                    pipeline_layout = Some(pipeline.layout.clone());
                }
                Cmd::SetIndexBuffer {
                    buffer,
                    index_format,
                    offset,
                    size,
                } => {
                    let buffer = buffer.try_raw(snatch_guard)?;
                    // SAFETY: The binding size was checked against the buffer size
                    // in `set_index_buffer` and again in `IndexState::flush`.
                    let bb = hal::BufferBinding::new_unchecked(buffer, *offset, *size);
                    unsafe { raw.set_index_buffer(bb, *index_format) };
                }
                Cmd::SetVertexBuffer {
                    slot,
                    buffer,
                    offset,
                    size,
                } => {
                    let buffer = buffer.try_raw(snatch_guard)?;
                    // SAFETY: The binding size was checked against the buffer size
                    // in `set_vertex_buffer` and again in `VertexState::flush`.
                    let bb = hal::BufferBinding::new_unchecked(buffer, *offset, *size);
                    unsafe { raw.set_vertex_buffer(*slot, bb) };
                }
                Cmd::SetImmediate {
                    offset,
                    size_bytes,
                    values_offset,
                } => {
                    let pipeline_layout = pipeline_layout.as_ref().unwrap();

                    if let Some(values_offset) = *values_offset {
                        let values_end_offset =
                            (values_offset + size_bytes / wgt::IMMEDIATE_DATA_ALIGNMENT) as usize;
                        let data_slice =
                            &self.base.immediates_data[(values_offset as usize)..values_end_offset];

                        unsafe { raw.set_immediates(pipeline_layout.raw(), *offset, data_slice) }
                    } else {
                        super::immediates_clear(
                            *offset,
                            *size_bytes,
                            |clear_offset, clear_data| {
                                unsafe {
                                    raw.set_immediates(
                                        pipeline_layout.raw(),
                                        clear_offset,
                                        clear_data,
                                    )
                                };
                            },
                        );
                    }
                }
                Cmd::Draw {
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                } => {
                    unsafe {
                        raw.draw(
                            *first_vertex,
                            *vertex_count,
                            *first_instance,
                            *instance_count,
                        )
                    };
                }
                Cmd::DrawIndexed {
                    index_count,
                    instance_count,
                    first_index,
                    base_vertex,
                    first_instance,
                } => {
                    unsafe {
                        raw.draw_indexed(
                            *first_index,
                            *index_count,
                            *base_vertex,
                            *first_instance,
                            *instance_count,
                        )
                    };
                }
                Cmd::DrawMeshTasks {
                    group_count_x,
                    group_count_y,
                    group_count_z,
                } => unsafe {
                    raw.draw_mesh_tasks(*group_count_x, *group_count_y, *group_count_z);
                },
                Cmd::DrawIndirect {
                    buffer,
                    offset,
                    count: 1,
                    family,

                    vertex_or_index_limit,
                    instance_limit,
                } => {
                    let (buffer, offset) = if self.device.indirect_validation.is_some() {
                        let (dst_resource_index, offset) = indirect_draw_validation_batcher.add(
                            indirect_draw_validation_resources,
                            &self.device,
                            buffer,
                            *offset,
                            *family,
                            vertex_or_index_limit
                                .expect("finalized render bundle missing vertex_or_index_limit"),
                            instance_limit.expect("finalized render bundle missing instance_limit"),
                        )?;

                        let dst_buffer =
                            indirect_draw_validation_resources.get_dst_buffer(dst_resource_index);
                        (dst_buffer, offset)
                    } else {
                        (buffer.try_raw(snatch_guard)?, *offset)
                    };
                    match family {
                        DrawCommandFamily::Draw => unsafe { raw.draw_indirect(buffer, offset, 1) },
                        DrawCommandFamily::DrawIndexed => unsafe {
                            raw.draw_indexed_indirect(buffer, offset, 1)
                        },
                        DrawCommandFamily::DrawMeshTasks => unsafe {
                            raw.draw_mesh_tasks_indirect(buffer, offset, 1);
                        },
                    }
                }
                Cmd::DrawIndirect { .. } | Cmd::MultiDrawIndirectCount { .. } => {
                    return Err(ExecutionError::Unimplemented("multi-draw-indirect"))
                }
                Cmd::PushDebugGroup { .. } | Cmd::InsertDebugMarker { .. } | Cmd::PopDebugGroup => {
                    return Err(ExecutionError::Unimplemented("debug-markers"))
                }
                Cmd::WriteTimestamp { .. }
                | Cmd::BeginOcclusionQuery { .. }
                | Cmd::EndOcclusionQuery
                | Cmd::BeginPipelineStatisticsQuery { .. }
                | Cmd::EndPipelineStatisticsQuery => {
                    return Err(ExecutionError::Unimplemented("queries"))
                }
                Cmd::ExecuteBundle(_)
                | Cmd::SetBlendConstant(_)
                | Cmd::SetStencilReference(_)
                | Cmd::SetViewport { .. }
                | Cmd::SetScissor(_) => unreachable!(),
            }
        }

        if !self.discard_hal_labels {
            if let Some(_) = self.base.label {
                unsafe { raw.end_debug_marker() };
            }
        }

        Ok(())
    }
}

crate::impl_resource_type!(RenderBundle);
crate::impl_labeled!(RenderBundle);
crate::impl_parent_device!(RenderBundle);
crate::impl_storage_item!(RenderBundle);
crate::impl_trackable!(RenderBundle);

/// A render bundle's current index buffer state.
///
/// [`RenderBundleEncoder::finish`] records the currently set index buffer here,
/// and calls [`State::flush_index`] before any indexed draw command to produce
/// a `SetIndexBuffer` command if one is necessary.
///
/// Binding ranges must be validated against the size of the buffer before
/// being stored in `IndexState`.
#[derive(Debug)]
struct IndexState {
    buffer: Arc<Buffer>,
    format: wgt::IndexFormat,
    range: Range<wgt::BufferAddress>,
    is_dirty: bool,
}

impl IndexState {
    /// Return the number of entries in the current index buffer.
    ///
    /// Panic if no index buffer has been set.
    fn limit(&self) -> u64 {
        let bytes_per_index = self.format.byte_size() as u64;

        (self.range.end - self.range.start) / bytes_per_index
    }

    /// Generate a `SetIndexBuffer` command to prepare for an indexed draw
    /// command, if needed.
    fn flush(&mut self) -> Option<ArcRenderCommand> {
        // This was all checked before, but let's check again just in case.
        let binding_size = self
            .range
            .end
            .checked_sub(self.range.start)
            .filter(|_| self.range.end <= self.buffer.size)
            .expect("index range must be contained in buffer");

        if self.is_dirty {
            self.is_dirty = false;
            Some(ArcRenderCommand::SetIndexBuffer {
                buffer: self.buffer.clone(),
                index_format: self.format,
                offset: self.range.start,
                size: NonZeroU64::new(binding_size),
            })
        } else {
            None
        }
    }
}

/// The state of a single vertex buffer slot during render bundle encoding.
///
/// [`RenderBundleEncoder::finish`] uses this to drop redundant
/// `SetVertexBuffer` commands from the final [`RenderBundle`]. It
/// records one vertex buffer slot's state changes here, and then
/// calls this type's [`flush`] method just before any draw command to
/// produce a `SetVertexBuffer` commands if one is necessary.
///
/// Binding ranges must be validated against the size of the buffer before
/// being stored in `VertexState`.
///
/// [`flush`]: IndexState::flush
#[derive(Debug)]
struct VertexState {
    buffer: Arc<Buffer>,
    range: Range<wgt::BufferAddress>,
    is_dirty: bool,
}

impl VertexState {
    /// Create a new `VertexState`.
    ///
    /// The `range` must be contained within `buffer`.
    fn new(buffer: Arc<Buffer>, range: Range<wgt::BufferAddress>) -> Self {
        Self {
            buffer,
            range,
            is_dirty: true,
        }
    }

    /// Generate a `SetVertexBuffer` command for this slot, if necessary.
    ///
    /// `slot` is the index of the vertex buffer slot that `self` tracks.
    fn flush(&mut self, slot: u32) -> Option<ArcRenderCommand> {
        let binding_size = self
            .range
            .end
            .checked_sub(self.range.start)
            .filter(|_| self.range.end <= self.buffer.size)
            .expect("vertex range must be contained in buffer");

        if self.is_dirty {
            self.is_dirty = false;
            Some(ArcRenderCommand::SetVertexBuffer {
                slot,
                buffer: self.buffer.clone(),
                offset: self.range.start,
                size: NonZeroU64::new(binding_size),
            })
        } else {
            None
        }
    }
}

/// The bundle's current pipeline, and some cached information needed for validation.
struct PipelineState {
    /// The pipeline
    pipeline: Arc<RenderPipeline>,

    /// How this pipeline's vertex shader traverses each vertex buffer, indexed
    /// by vertex buffer slot number.
    steps: Vec<VertexStep>,

    /// Size of the immediate data ranges this pipeline uses. Copied from the pipeline layout.
    immediate_size: u32,
}

impl PipelineState {
    fn new(pipeline: &Arc<RenderPipeline>) -> Self {
        Self {
            pipeline: pipeline.clone(),
            steps: pipeline.vertex_steps.to_vec(),
            immediate_size: pipeline.layout.immediate_size,
        }
    }

    /// Return a sequence of commands to zero the immediate data ranges this
    /// pipeline uses. If no initialization is necessary, return `None`.
    fn zero_immediates(&self) -> Option<ArcRenderCommand> {
        if self.immediate_size == 0 {
            return None;
        }

        Some(ArcRenderCommand::SetImmediate {
            offset: 0,
            size_bytes: self.immediate_size,
            values_offset: None,
        })
    }
}

/// State for analyzing and cleaning up bundle command streams.
///
/// To minimize state updates, [`RenderBundleEncoder::finish`]
/// actually just applies commands like [`SetBindGroup`] and
/// [`SetIndexBuffer`] to the simulated state stored here, and then
/// calls the `flush_foo` methods before draw calls to produce the
/// update commands we actually need.
///
/// [`SetBindGroup`]: RenderCommand::SetBindGroup
/// [`SetIndexBuffer`]: RenderCommand::SetIndexBuffer
struct State {
    /// Resources used by this bundle. This will become [`RenderBundle::used`].
    trackers: RenderBundleScope,

    /// The currently set pipeline, if any.
    pipeline: Option<PipelineState>,

    /// The state of each vertex buffer slot.
    vertex: [Option<VertexState>; hal::MAX_VERTEX_BUFFERS],

    /// The current index buffer, if one has been set. We flush this state
    /// before indexed draw commands.
    index: Option<IndexState>,

    /// Dynamic offset values used by the cleaned-up command sequence.
    ///
    /// This becomes the final [`RenderBundle`]'s [`BasePass`]'s
    /// [`dynamic_offsets`] list.
    ///
    /// [`dynamic_offsets`]: BasePass::dynamic_offsets
    flat_dynamic_offsets: Vec<wgt::DynamicOffset>,

    device: Arc<Device>,
    commands: Vec<ArcRenderCommand>,
    buffer_memory_init_actions: Vec<BufferInitTrackerAction>,
    texture_memory_init_actions: Vec<TextureInitTrackerAction>,
    next_dynamic_offset: usize,
    binder: Binder,
}

impl State {
    /// Return the current pipeline state. Return an error if none is set.
    fn pipeline(&self) -> Result<&PipelineState, RenderBundleErrorInner> {
        self.pipeline
            .as_ref()
            .ok_or(DrawError::MissingPipeline(pass::MissingPipeline).into())
    }

    /// Set the bundle's current index buffer and its associated parameters.
    fn set_index_buffer(
        &mut self,
        buffer: Arc<Buffer>,
        format: wgt::IndexFormat,
        range: Range<wgt::BufferAddress>,
    ) {
        match self.index {
            Some(ref current)
                if current.buffer.is_equal(&buffer)
                    && current.format == format
                    && current.range == range =>
            {
                return
            }
            _ => (),
        }

        self.index = Some(IndexState {
            buffer,
            format,
            range,
            is_dirty: true,
        });
    }

    /// Generate a `SetIndexBuffer` command to prepare for an indexed draw
    /// command, if needed.
    fn flush_index(&mut self) {
        let commands = self.index.as_mut().and_then(|index| index.flush());
        self.commands.extend(commands);
    }

    fn flush_vertices(&mut self) {
        let commands = self
            .vertex
            .iter_mut()
            .enumerate()
            .flat_map(|(i, vs)| vs.as_mut().and_then(|vs| vs.flush(i as u32)));
        self.commands.extend(commands);
    }

    /// Validation for a draw command.
    ///
    /// This should be further deduplicated with similar validation on render/compute passes.
    fn is_ready(&mut self, family: DrawCommandFamily) -> Result<(), DrawError> {
        if let Some(pipeline) = self.pipeline.as_ref() {
            self.binder
                .check_compatibility(pipeline.pipeline.as_ref())?;
            self.binder.check_late_buffer_bindings()?;

            if family == DrawCommandFamily::DrawIndexed {
                let pipeline = &pipeline.pipeline;
                let index_format = match &self.index {
                    Some(index) => index.format,
                    None => return Err(DrawError::MissingIndexBuffer),
                };

                if pipeline.topology.is_strip() && pipeline.strip_index_format != Some(index_format)
                {
                    return Err(DrawError::UnmatchedStripIndexFormat {
                        pipeline: pipeline.error_ident(),
                        strip_index_format: pipeline.strip_index_format,
                        buffer_format: index_format,
                    });
                }
            }

            Ok(())
        } else {
            Err(DrawError::MissingPipeline(pass::MissingPipeline))
        }
    }

    /// Generate `SetBindGroup` commands for any bind groups that need to be updated.
    ///
    /// This should be further deduplicated with similar code on render/compute passes.
    fn flush_bindings(&mut self) {
        let start = self.binder.take_rebind_start_index();
        let entries = self.binder.list_valid_with_start(start);

        self.commands
            .extend(entries.map(|(i, bind_group, dynamic_offsets)| {
                self.buffer_memory_init_actions
                    .extend_from_slice(&bind_group.used_buffer_ranges);
                self.texture_memory_init_actions
                    .extend_from_slice(&bind_group.used_texture_ranges);

                self.flat_dynamic_offsets.extend_from_slice(dynamic_offsets);

                ArcRenderCommand::SetBindGroup {
                    index: i.try_into().unwrap(),
                    bind_group: Some(bind_group.clone()),
                    num_dynamic_offsets: dynamic_offsets.len(),
                }
            }));
    }

    fn vertex_buffer_sizes(&self) -> impl Iterator<Item = Option<wgt::BufferAddress>> + '_ {
        self.vertex
            .iter()
            .map(|vbs| vbs.as_ref().map(|vbs| vbs.range.end - vbs.range.start))
    }
}

/// Error encountered when finishing recording a render bundle.
#[derive(Clone, Debug, Error)]
pub enum RenderBundleErrorInner {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    RenderCommand(RenderCommandError),
    #[error(transparent)]
    Draw(#[from] DrawError),
    #[error(transparent)]
    MissingDownlevelFlags(#[from] MissingDownlevelFlags),
    #[error(transparent)]
    Bind(#[from] BindError),
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
}

impl<T> From<T> for RenderBundleErrorInner
where
    T: Into<RenderCommandError>,
{
    fn from(t: T) -> Self {
        Self::RenderCommand(t.into())
    }
}

/// Error encountered when finishing recording a render bundle.
#[derive(Clone, Debug, Error)]
#[error("{scope}")]
pub struct RenderBundleError {
    pub scope: PassErrorScope,
    #[source]
    inner: RenderBundleErrorInner,
}

impl WebGpuError for RenderBundleError {
    fn webgpu_error_type(&self) -> ErrorType {
        let Self { scope: _, inner } = self;
        match inner {
            RenderBundleErrorInner::Device(e) => e.webgpu_error_type(),
            RenderBundleErrorInner::RenderCommand(e) => e.webgpu_error_type(),
            RenderBundleErrorInner::Draw(e) => e.webgpu_error_type(),
            RenderBundleErrorInner::MissingDownlevelFlags(e) => e.webgpu_error_type(),
            RenderBundleErrorInner::Bind(e) => e.webgpu_error_type(),
            RenderBundleErrorInner::InvalidResource(e) => e.webgpu_error_type(),
        }
    }
}

impl RenderBundleError {
    pub fn from_device_error(e: DeviceError) -> Self {
        Self {
            scope: PassErrorScope::Bundle,
            inner: e.into(),
        }
    }
}

impl<E> MapPassErr<RenderBundleError> for E
where
    E: Into<RenderBundleErrorInner>,
{
    fn map_pass_err(self, scope: PassErrorScope) -> RenderBundleError {
        RenderBundleError {
            scope,
            inner: self.into(),
        }
    }
}

pub mod bundle_ffi {
    use super::{RenderBundleEncoder, RenderCommand};
    use crate::{command::DrawCommandFamily, id, RawString};
    use core::{convert::TryInto, slice};
    use wgt::{BufferAddress, BufferSize, DynamicOffset, IndexFormat};

    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is
    /// valid for `offset_length` elements.
    pub unsafe fn wgpu_render_bundle_set_bind_group(
        bundle: &mut RenderBundleEncoder,
        index: u32,
        bind_group_id: Option<id::BindGroupId>,
        offsets: *const DynamicOffset,
        offset_length: usize,
    ) {
        let offsets = unsafe { slice::from_raw_parts(offsets, offset_length) };

        let redundant = bundle.current_bind_groups.set_and_check_redundant(
            bind_group_id,
            index,
            &mut bundle.base.dynamic_offsets,
            offsets,
        );

        if redundant {
            return;
        }

        bundle.base.commands.push(RenderCommand::SetBindGroup {
            index,
            num_dynamic_offsets: offset_length,
            bind_group: bind_group_id,
        });
    }

    pub fn wgpu_render_bundle_set_pipeline(
        bundle: &mut RenderBundleEncoder,
        pipeline_id: id::RenderPipelineId,
    ) {
        if bundle.current_pipeline.set_and_check_redundant(pipeline_id) {
            return;
        }

        bundle
            .base
            .commands
            .push(RenderCommand::SetPipeline(pipeline_id));
    }

    pub fn wgpu_render_bundle_set_vertex_buffer(
        bundle: &mut RenderBundleEncoder,
        slot: u32,
        buffer_id: id::BufferId,
        offset: BufferAddress,
        size: Option<BufferSize>,
    ) {
        bundle.base.commands.push(RenderCommand::SetVertexBuffer {
            slot,
            buffer: buffer_id,
            offset,
            size,
        });
    }

    pub fn wgpu_render_bundle_set_index_buffer(
        encoder: &mut RenderBundleEncoder,
        buffer: id::BufferId,
        index_format: IndexFormat,
        offset: BufferAddress,
        size: Option<BufferSize>,
    ) {
        encoder.set_index_buffer(buffer, index_format, offset, size);
    }

    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is
    /// valid for `data` elements.
    pub unsafe fn wgpu_render_bundle_set_immediates(
        pass: &mut RenderBundleEncoder,
        offset: u32,
        size_bytes: u32,
        data: *const u8,
    ) {
        assert_eq!(
            offset & (wgt::IMMEDIATE_DATA_ALIGNMENT - 1),
            0,
            "Immediate data offset must be aligned to 4 bytes."
        );
        assert_eq!(
            size_bytes & (wgt::IMMEDIATE_DATA_ALIGNMENT - 1),
            0,
            "Immediate data size must be aligned to 4 bytes."
        );
        let data_slice = unsafe { slice::from_raw_parts(data, size_bytes as usize) };
        let value_offset = pass.base.immediates_data.len().try_into().expect(
            "Ran out of immediate data space. Don't set 4gb of immediates per RenderBundle.",
        );

        pass.base.immediates_data.extend(
            data_slice
                .chunks_exact(wgt::IMMEDIATE_DATA_ALIGNMENT as usize)
                .map(|arr| u32::from_ne_bytes([arr[0], arr[1], arr[2], arr[3]])),
        );

        pass.base.commands.push(RenderCommand::SetImmediate {
            offset,
            size_bytes,
            values_offset: Some(value_offset),
        });
    }

    pub fn wgpu_render_bundle_draw(
        bundle: &mut RenderBundleEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        bundle.base.commands.push(RenderCommand::Draw {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        });
    }

    pub fn wgpu_render_bundle_draw_indexed(
        bundle: &mut RenderBundleEncoder,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    ) {
        bundle.base.commands.push(RenderCommand::DrawIndexed {
            index_count,
            instance_count,
            first_index,
            base_vertex,
            first_instance,
        });
    }

    pub fn wgpu_render_bundle_draw_indirect(
        bundle: &mut RenderBundleEncoder,
        buffer_id: id::BufferId,
        offset: BufferAddress,
    ) {
        bundle.base.commands.push(RenderCommand::DrawIndirect {
            buffer: buffer_id,
            offset,
            count: 1,
            family: DrawCommandFamily::Draw,
            vertex_or_index_limit: None,
            instance_limit: None,
        });
    }

    pub fn wgpu_render_bundle_draw_indexed_indirect(
        bundle: &mut RenderBundleEncoder,
        buffer_id: id::BufferId,
        offset: BufferAddress,
    ) {
        bundle.base.commands.push(RenderCommand::DrawIndirect {
            buffer: buffer_id,
            offset,
            count: 1,
            family: DrawCommandFamily::DrawIndexed,
            vertex_or_index_limit: None,
            instance_limit: None,
        });
    }

    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given `label`
    /// is a valid null-terminated string.
    pub unsafe fn wgpu_render_bundle_push_debug_group(
        _bundle: &mut RenderBundleEncoder,
        _label: RawString,
    ) {
        //TODO
    }

    pub fn wgpu_render_bundle_pop_debug_group(_bundle: &mut RenderBundleEncoder) {
        //TODO
    }

    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given `label`
    /// is a valid null-terminated string.
    pub unsafe fn wgpu_render_bundle_insert_debug_marker(
        _bundle: &mut RenderBundleEncoder,
        _label: RawString,
    ) {
        //TODO
    }
}
