use super::{
    ANIMATION_CAPACITY, DIRTY_PATH_CAPACITY, FRAME_CAPACITY, NOTIFICATION_CAPACITY,
    VIEW_RENDER_CAPACITY, WINDOW_FRAME_CAPACITY,
    events::{AnimationEvent, DirtyPathEvent, FrameEvent, NotifyEvent, ViewRenderEvent},
    overlay::PreparedOverlay,
    ring_buffer::RingBuffer,
    sources::{NotifySourceKey, RenderSourceKey},
};
use crate::{Bounds, EntityId, Pixels, Point, WindowId};
use collections::{FxHashMap, FxHashSet};
use scheduler::Instant;

#[derive(Debug)]
pub(super) struct GpuiDevTools {
    pub(super) notifications: RingBuffer<NotifyEvent>,
    pub(super) frames: RingBuffer<FrameEvent>,
    pub(super) renders: RingBuffer<ViewRenderEvent>,
    pub(super) dirty_paths: RingBuffer<DirtyPathEvent>,
    pub(super) animations: RingBuffer<AnimationEvent>,
    pub(super) windows: FxHashMap<WindowId, WindowDevToolsState>,
    pub(super) hidden_notify_sources: FxHashSet<NotifySourceKey>,
    pub(super) hidden_render_sources: FxHashSet<RenderSourceKey>,
    pub(super) notify_source_total_counts: FxHashMap<NotifySourceKey, usize>,
    pub(super) pinned_notify_source: Option<NotifySourceKey>,
    pub(super) initial_pinned_notify_source_resolved: bool,
}

impl GpuiDevTools {
    pub(super) fn new() -> Self {
        Self {
            notifications: RingBuffer::new(NOTIFICATION_CAPACITY),
            frames: RingBuffer::new(FRAME_CAPACITY),
            renders: RingBuffer::new(VIEW_RENDER_CAPACITY),
            dirty_paths: RingBuffer::new(DIRTY_PATH_CAPACITY),
            animations: RingBuffer::new(ANIMATION_CAPACITY),
            windows: FxHashMap::default(),
            hidden_notify_sources: FxHashSet::default(),
            hidden_render_sources: FxHashSet::default(),
            notify_source_total_counts: FxHashMap::default(),
            pinned_notify_source: None,
            initial_pinned_notify_source_resolved: false,
        }
    }

    pub(super) fn window_state(&mut self, window_id: WindowId) -> &mut WindowDevToolsState {
        self.windows
            .entry(window_id)
            .or_insert_with(WindowDevToolsState::new)
    }
}

#[derive(Debug)]
pub(super) struct WindowDevToolsState {
    pub(super) recent_frames: RingBuffer<FrameEvent>,
    pub(super) view_bounds: FxHashMap<EntityId, Bounds<Pixels>>,
    pub(super) active_flashes: FxHashMap<EntityId, FlashState>,
    pub(super) prepared_overlay: Option<PreparedOverlay>,
    pub(super) hud_origin: Option<Point<Pixels>>,
    pub(super) hud_drag: Option<HudDragState>,
}

impl WindowDevToolsState {
    fn new() -> Self {
        Self {
            recent_frames: RingBuffer::new(WINDOW_FRAME_CAPACITY),
            view_bounds: FxHashMap::default(),
            active_flashes: FxHashMap::default(),
            prepared_overlay: None,
            hud_origin: None,
            hud_drag: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct FlashState {
    pub(super) timestamp: Instant,
    pub(super) source: RenderSourceKey,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct HudDragState {
    pub(super) cursor_offset: Point<Pixels>,
}
