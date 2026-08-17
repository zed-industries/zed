use alloc::{sync::Arc, vec::Vec};

use crate::{
    command::memory_init::CommandBufferTextureMemoryActions,
    device::{queue::TempResource, Device},
    init_tracker::BufferInitTrackerAction,
    ray_tracing::AsAction,
    snatch::SnatchGuard,
    track::Tracker,
};

/// State applicable when encoding commands onto a compute pass, render pass, or
/// directly to a command encoder.
///
/// Most encoding routines just want to receive an open encoder, write
/// command(s) to it, and leave it open for whatever is next. In this case the
/// `E` type parameter has the default value of `dyn hal::DynCommandEncoder`. To
/// avoid confusion about encoder state, we set the convention that _the encoder
/// in an `EncodingState` holding a bare HAL reference must always be open_.
///
/// Compute and render passes are more complicated. Because they record a
/// command buffer for a housekeeping pre-pass which is inserted before the pass
/// itself, the first thing they will do is close and reopen the encoder if it
/// is already open. Unnecessary empty HAL passes can be avoided by passing them
/// the encoder in whatever state it happens to be. In this case, `E` is
/// `InnerCommandEncoder`, which tracks the state of the encoder. The callee
/// (the render or compute pass) will open and close the encoder as necessary.
///
/// This structure is not supported by cbindgen because it contains a trait
/// object reference.
///
/// cbindgen:ignore
pub(crate) struct EncodingState<'snatch_guard, 'cmd_enc, E: ?Sized = dyn hal::DynCommandEncoder> {
    pub(crate) device: &'cmd_enc Arc<Device>,

    pub(crate) raw_encoder: &'cmd_enc mut E,

    pub(crate) tracker: &'cmd_enc mut Tracker,
    pub(crate) buffer_memory_init_actions: &'cmd_enc mut Vec<BufferInitTrackerAction>,
    pub(crate) texture_memory_actions: &'cmd_enc mut CommandBufferTextureMemoryActions,
    pub(crate) as_actions: &'cmd_enc mut Vec<AsAction>,
    pub(crate) temp_resources: &'cmd_enc mut Vec<TempResource>,
    pub(crate) indirect_draw_validation_resources:
        &'cmd_enc mut crate::indirect_validation::DrawResources,

    pub(crate) snatch_guard: &'snatch_guard SnatchGuard<'snatch_guard>,

    /// Current debug scope nesting depth.
    ///
    /// When encoding a compute or render pass, this is the depth of debug
    /// scopes in the pass, not the depth of debug scopes in the parent encoder.
    pub(crate) debug_scope_depth: &'cmd_enc mut u32,
}
