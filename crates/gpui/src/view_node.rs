use crate::{
    AnyView, Bounds, ContentMask, CursorStyleRequest, EntityId, GlobalElementId, Hitbox, LayoutId,
    PaintIndex, Pixels, Scene, TextStyle, TooltipRequest,
};
use collections::{FxHashMap, FxHashSet};
use std::any::TypeId;
use std::ops::Range;

#[derive(Clone, PartialEq)]
pub(crate) struct ViewNodeCacheKey {
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) content_mask: ContentMask<Pixels>,
    pub(crate) text_style: TextStyle,
    pub(crate) rem_size: Pixels,
    pub(crate) scale_factor: f32,
    pub(crate) opacity: f32,
    pub(crate) image_cache: Option<EntityId>,
}

#[derive(Default)]
pub(crate) struct ViewNodeRecording {
    pub(crate) layout_states: Vec<(GlobalElementId, TypeId)>,
    pub(crate) prepaint_states: Vec<(GlobalElementId, TypeId)>,
    pub(crate) paint_states: Vec<(GlobalElementId, TypeId)>,
    pub(crate) layout_text: crate::text_system::LineLayoutRecording,
    pub(crate) prepaint_text: crate::text_system::LineLayoutRecording,
    pub(crate) paint_text: crate::text_system::LineLayoutRecording,
    pub(crate) tab_stops: Vec<crate::TabStopOperation>,
    pub(crate) window_controls: Vec<(crate::WindowControlArea, Hitbox)>,
    pub(crate) mouse_listeners: Vec<Option<crate::window::AnyMouseListener>>,
    pub(crate) input_handlers: Vec<Option<crate::PlatformInputHandler>>,
    pub(crate) dispatch_nodes: Vec<crate::key_dispatch::DispatchNode>,
    pub(crate) dispatch_start: usize,
    pub(crate) has_layout: bool,
    pub(crate) scene: ViewNodeScene,
    pub(crate) hitboxes: Vec<Hitbox>,
    pub(crate) tooltip_requests: Vec<Option<TooltipRequest>>,
    pub(crate) cursor_styles: Vec<CursorStyleRequest>,
}

#[derive(Default)]
pub(crate) struct ViewNodeScene {
    operations: Vec<crate::scene::PaintOperation>,
    segments: Vec<ViewNodeSceneSegment>,
}

enum ViewNodeSceneSegment {
    Local(Range<usize>),
    Child(crate::node_engine::ViewNodeId),
}

impl ViewNodeScene {
    pub(crate) fn record(
        &mut self,
        scene: &Scene,
        range: Range<usize>,
        children: &mut [(Range<usize>, crate::node_engine::ViewNodeId)],
    ) {
        children.sort_unstable_by_key(|(range, _)| (range.start, range.end));
        self.operations.clear();
        self.segments.clear();
        let mut cursor = range.start;
        for (child_range, child) in children {
            if child_range.start < cursor || child_range.end > range.end {
                continue;
            }
            if cursor < child_range.start {
                self.record_local(scene, cursor..child_range.start);
            }
            self.segments.push(ViewNodeSceneSegment::Child(*child));
            cursor = child_range.end;
        }
        if cursor < range.end {
            self.record_local(scene, cursor..range.end);
        }
    }

    fn record_local(&mut self, scene: &Scene, range: Range<usize>) {
        let start = self.operations.len();
        scene.recording(range, &mut self.operations);
        self.segments
            .push(ViewNodeSceneSegment::Local(start..self.operations.len()));
    }

    pub(crate) fn replay(
        &self,
        scene: &mut Scene,
        engine: &crate::node_engine::NodeEngine,
        cx: &crate::App,
    ) {
        for segment in &self.segments {
            match segment {
                ViewNodeSceneSegment::Local(local) => {
                    scene.replay_recording(&self.operations[local.clone()])
                }
                ViewNodeSceneSegment::Child(child) => engine.replay_scene(*child, scene, cx),
            }
        }
    }
}

pub(crate) struct NodeLocalState {
    pub(crate) entity: crate::AnyEntity,
    pub(crate) _subscription: crate::Subscription,
}

pub(crate) struct ViewNode {
    pub(crate) paint_range: Range<PaintIndex>,
    pub(crate) local_state: FxHashMap<(GlobalElementId, TypeId), NodeLocalState>,
    pub(crate) accessed_local_state: FxHashSet<(GlobalElementId, TypeId)>,
    pub(crate) layout: Option<LayoutId>,
    pub(crate) occurrence: GlobalElementId,
    pub(crate) parent: Option<super::node_engine::ViewNodeId>,
    pub(crate) children: Vec<super::node_engine::ViewNodeId>,
    pub(crate) next_children: Vec<super::node_engine::ViewNodeId>,
    pub(crate) view: AnyView,
    pub(crate) cache_key: ViewNodeCacheKey,
    pub(crate) previous_bounds: Bounds<Pixels>,
    pub(crate) accessed_entities: FxHashSet<EntityId>,
    pub(crate) dependency_revisions: Vec<(EntityId, u64)>,
    pub(crate) recording: Option<ViewNodeRecording>,
}
