use crate::{Bounds, ElementId, EntityId, Pixels, SceneStats, WindowId};
use scheduler::Instant;
use std::time::Duration;

#[derive(Clone, Debug)]
pub(crate) struct NotifyEvent {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
    pub(crate) caller_file: &'static str,
    pub(crate) caller_line: u32,
    pub(crate) caller_column: u32,
    pub(crate) registered_window_count: usize,
    pub(crate) live_window_count: usize,
    pub(crate) timestamp: Instant,
}

impl NotifyEvent {
    pub(crate) fn new(
        entity_id: EntityId,
        entity_type: &'static str,
        caller: &'static std::panic::Location<'static>,
        registered_window_count: usize,
        live_window_count: usize,
    ) -> Self {
        Self {
            entity_id,
            entity_type,
            caller_file: caller.file(),
            caller_line: caller.line(),
            caller_column: caller.column(),
            registered_window_count,
            live_window_count,
            timestamp: Instant::now(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FrameEvent {
    pub(crate) window_id: WindowId,
    pub(crate) reason: &'static str,
    pub(crate) dirty_before_frame: bool,
    pub(crate) dirty_view_count: usize,
    pub(crate) invalidator_update_count: usize,
    pub(crate) rebuilt_scene: bool,
    pub(crate) draw_duration: Option<Duration>,
    pub(crate) present_duration: Duration,
    pub(crate) scene_stats: SceneStats,
    pub(crate) devtools_induced: bool,
    pub(crate) timestamp: Instant,
}

#[derive(Clone, Debug)]
pub(crate) struct DirtyPathSegment {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
}

#[derive(Clone, Debug)]
pub(crate) struct DirtyPathEvent {
    pub(crate) window_id: WindowId,
    pub(crate) invalidated_entity_id: EntityId,
    pub(crate) invalidated_entity_type: &'static str,
    pub(crate) path: Vec<DirtyPathSegment>,
    pub(crate) timestamp: Instant,
}

impl DirtyPathEvent {
    pub(crate) fn new(
        window_id: WindowId,
        invalidated_entity_id: EntityId,
        invalidated_entity_type: &'static str,
        path: Vec<DirtyPathSegment>,
    ) -> Self {
        Self {
            window_id,
            invalidated_entity_id,
            invalidated_entity_type,
            path,
            timestamp: Instant::now(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ViewRenderPhase {
    UncachedRender,
    UncachedRenderInspector,
    UncachedPrepaint,
    CachedCacheMissRefresh,
    PrepaintReuse,
    PaintReuse,
}

impl ViewRenderPhase {
    pub(super) fn flashes(self) -> bool {
        matches!(
            self,
            ViewRenderPhase::UncachedRender
                | ViewRenderPhase::UncachedRenderInspector
                | ViewRenderPhase::CachedCacheMissRefresh
        )
    }

    pub(super) fn is_reuse(self) -> bool {
        matches!(
            self,
            ViewRenderPhase::PrepaintReuse | ViewRenderPhase::PaintReuse
        )
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            ViewRenderPhase::UncachedRender => "render",
            ViewRenderPhase::UncachedRenderInspector => "render inspector",
            ViewRenderPhase::UncachedPrepaint => "prepaint",
            ViewRenderPhase::CachedCacheMissRefresh => "cache miss",
            ViewRenderPhase::PrepaintReuse => "reuse prepaint",
            ViewRenderPhase::PaintReuse => "reuse paint",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CacheMissReasons(u8);

impl CacheMissReasons {
    const MISSING_CACHE: u8 = 1 << 0;
    const BOUNDS_CHANGED: u8 = 1 << 1;
    const CONTENT_MASK_CHANGED: u8 = 1 << 2;
    const TEXT_STYLE_CHANGED: u8 = 1 << 3;
    const VIEW_DIRTY: u8 = 1 << 4;
    const WINDOW_REFRESHING: u8 = 1 << 5;

    pub(crate) fn empty() -> Self {
        Self(0)
    }

    pub(crate) fn insert_missing_cache(&mut self) {
        self.0 |= Self::MISSING_CACHE;
    }

    pub(crate) fn insert_bounds_changed(&mut self) {
        self.0 |= Self::BOUNDS_CHANGED;
    }

    pub(crate) fn insert_content_mask_changed(&mut self) {
        self.0 |= Self::CONTENT_MASK_CHANGED;
    }

    pub(crate) fn insert_text_style_changed(&mut self) {
        self.0 |= Self::TEXT_STYLE_CHANGED;
    }

    pub(crate) fn insert_view_dirty(&mut self) {
        self.0 |= Self::VIEW_DIRTY;
    }

    pub(crate) fn insert_window_refreshing(&mut self) {
        self.0 |= Self::WINDOW_REFRESHING;
    }

    pub(crate) fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub(super) fn window_refreshing(self) -> bool {
        self.0 & Self::WINDOW_REFRESHING != 0
    }

    pub(super) fn labels(self) -> Vec<&'static str> {
        let mut labels = Vec::new();
        if self.0 & Self::MISSING_CACHE != 0 {
            labels.push("first");
        }
        if self.0 & Self::BOUNDS_CHANGED != 0 {
            labels.push("bounds");
        }
        if self.0 & Self::CONTENT_MASK_CHANGED != 0 {
            labels.push("mask");
        }
        if self.0 & Self::TEXT_STYLE_CHANGED != 0 {
            labels.push("text");
        }
        if self.0 & Self::VIEW_DIRTY != 0 {
            labels.push("dirty");
        }
        if self.0 & Self::WINDOW_REFRESHING != 0 {
            labels.push("refresh");
        }
        labels
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ViewRenderEvent {
    pub(crate) window_id: WindowId,
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
    pub(crate) phase: ViewRenderPhase,
    pub(crate) duration: Option<Duration>,
    pub(crate) cache_miss_reasons: CacheMissReasons,
    pub(crate) bounds: Option<Bounds<Pixels>>,
    pub(crate) caching_disabled_by_inspector: bool,
    pub(crate) timestamp: Instant,
}

#[derive(Clone, Debug)]
pub(crate) struct AnimationEvent {
    pub(crate) window_id: WindowId,
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
    pub(crate) kind: AnimationEventKind,
    pub(crate) timestamp: Instant,
}

#[derive(Clone, Debug)]
pub(crate) enum AnimationEventKind {
    FrameRequest {
        caller_file: &'static str,
        caller_line: u32,
        caller_column: u32,
    },
    ElementTick {
        element_id: String,
        animation_index: usize,
        duration: Duration,
        repeats: bool,
    },
}

pub(crate) fn animation_frame_request_event(
    window_id: WindowId,
    entity_id: EntityId,
    entity_type: &'static str,
    caller: &'static std::panic::Location<'static>,
) -> AnimationEvent {
    AnimationEvent {
        window_id,
        entity_id,
        entity_type,
        kind: AnimationEventKind::FrameRequest {
            caller_file: caller.file(),
            caller_line: caller.line(),
            caller_column: caller.column(),
        },
        timestamp: Instant::now(),
    }
}

pub(crate) fn animation_element_tick_event(
    window_id: WindowId,
    entity_id: EntityId,
    entity_type: &'static str,
    element_id: &ElementId,
    animation_index: usize,
    duration: Duration,
    repeats: bool,
) -> AnimationEvent {
    AnimationEvent {
        window_id,
        entity_id,
        entity_type,
        kind: AnimationEventKind::ElementTick {
            element_id: format!("{element_id:?}"),
            animation_index,
            duration,
            repeats,
        },
        timestamp: Instant::now(),
    }
}
