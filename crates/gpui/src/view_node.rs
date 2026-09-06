use crate::{
    Bounds, ContentMask, CursorStyleRequest, EntityId, GlobalElementId, Hitbox, LayoutId, Pixels,
    Scene, TextStyle, TooltipRequest,
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

impl ViewNodeCacheKey {
    /// Whether output recorded under `self` is valid for a frame whose ambient inputs are
    /// `other`. Bounds are unknown during layout, so that phase compares without them.
    pub(crate) fn matches(&self, other: &Self, ignore_bounds: bool) -> bool {
        (ignore_bounds || self.bounds == other.bounds)
            && self.content_mask == other.content_mask
            && self.rem_size == other.rem_size
            && self.scale_factor == other.scale_factor
            && self.opacity == other.opacity
            && self.image_cache == other.image_cache
            && self.text_style == other.text_style
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MetadataPhase {
    Layout,
    Prepaint,
    Paint,
}

pub(crate) struct MetadataChild {
    pub(crate) local_end: usize,
    pub(crate) node: crate::node_engine::ViewNodeId,
    pub(crate) phase: MetadataPhase,
    pub(crate) len: usize,
    pub(crate) dispatch_parent: Option<std::num::NonZeroUsize>,
}

pub(crate) struct RecordedMetadata<T> {
    pub(crate) local: Vec<T>,
    pub(crate) children: Vec<MetadataChild>,
    // Ancestors capture against the current frame, even when these local entries are older.
    pub(crate) frame_range: Range<usize>,
}

impl<T> Default for RecordedMetadata<T> {
    fn default() -> Self {
        Self {
            local: Vec::new(),
            children: Vec::new(),
            frame_range: 0..0,
        }
    }
}

impl<T> RecordedMetadata<T> {
    pub(crate) fn record<'a>(
        &mut self,
        range: Range<usize>,
        children: impl Iterator<Item = (crate::node_engine::ViewNodeId, &'a ViewNodeRecording)>,
        select: impl Fn(&ViewNodeRecording, MetadataPhase) -> Option<&Self>,
        mut capture: impl FnMut(Range<usize>, &mut Vec<T>, usize),
    ) {
        self.children.clear();
        if range.is_empty() {
            self.local.clear();
            self.frame_range = range;
            return;
        }
        // Child layout can run inside parent prepaint; include contained spans from every phase.
        for (node, recording) in children {
            for phase in [
                MetadataPhase::Layout,
                MetadataPhase::Prepaint,
                MetadataPhase::Paint,
            ] {
                let Some(recorded) = select(recording, phase) else {
                    continue;
                };
                let source = &recorded.frame_range;
                if source.start < source.end
                    && source.start >= range.start
                    && source.end <= range.end
                {
                    self.children.push(MetadataChild {
                        local_end: source.start,
                        node,
                        phase,
                        len: source.len(),
                        dispatch_parent: None,
                    });
                }
            }
        }
        self.children.sort_unstable_by_key(|child| child.local_end);
        let mut cursor = range.start;
        let mut local_end = 0;
        for child in &mut self.children {
            let child_start = child.local_end;
            assert!(
                child_start >= cursor,
                "metadata child ranges must not overlap: node {:?}, phase {:?}, start {child_start}, cursor {cursor}, length {}, parent {range:?}",
                child.node,
                child.phase,
                child.len
            );
            capture(cursor..child_start, &mut self.local, local_end);
            local_end += child_start - cursor;
            child.local_end = local_end;
            cursor = child_start + child.len;
        }
        capture(cursor..range.end, &mut self.local, local_end);
        self.local.truncate(local_end + range.end - cursor);
        self.frame_range = range;
    }

    pub(crate) fn replay(
        &self,
        engine: &crate::node_engine::NodeEngine,
        select: &impl Fn(&ViewNodeRecording, MetadataPhase) -> Option<&Self>,
        emit: &mut impl FnMut(&[T]),
    ) {
        let mut start = 0;
        for child in &self.children {
            emit(&self.local[start..child.local_end]);
            select(engine.recording(child.node), child.phase)
                .expect("child metadata phase exists")
                .replay(engine, select, emit);
            start = child.local_end;
        }
        emit(&self.local[start..]);
    }
}

pub(crate) fn capture_metadata<T: Clone>(source: &[T], target: &mut Vec<T>, start: usize) {
    for (index, value) in source.iter().enumerate() {
        if let Some(slot) = target.get_mut(start + index) {
            slot.clone_from(value);
        } else {
            target.push(value.clone());
        }
    }
}

#[derive(Default)]
pub(crate) struct ViewNodeRecording {
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) debug_bounds: RecordedMetadata<(String, Bounds<Pixels>)>,
    pub(crate) layout_states: RecordedMetadata<(GlobalElementId, TypeId)>,
    pub(crate) prepaint_states: RecordedMetadata<(GlobalElementId, TypeId)>,
    pub(crate) paint_states: RecordedMetadata<(GlobalElementId, TypeId)>,
    pub(crate) layout_text: crate::text_system::LineLayoutRecording,
    pub(crate) prepaint_text: crate::text_system::LineLayoutRecording,
    pub(crate) paint_text: crate::text_system::LineLayoutRecording,
    pub(crate) tab_stops: RecordedMetadata<crate::TabStopOperation>,
    pub(crate) window_controls: RecordedMetadata<(crate::WindowControlArea, Hitbox)>,
    pub(crate) mouse_listeners: RecordedMetadata<Option<crate::window::AnyMouseListener>>,
    pub(crate) input_handlers: RecordedMetadata<Option<crate::PlatformInputHandler>>,
    pub(crate) dispatch_nodes: RecordedMetadata<crate::key_dispatch::DispatchNode>,
    pub(crate) dispatch_start: usize,
    pub(crate) has_layout: bool,
    pub(crate) scene: ViewNodeScene,
    pub(crate) hitboxes: RecordedMetadata<Hitbox>,
    pub(crate) tooltip_requests: RecordedMetadata<Option<TooltipRequest>>,
    pub(crate) cursor_styles: RecordedMetadata<CursorStyleRequest>,
}

impl ViewNodeRecording {
    pub(crate) fn text(&self, phase: MetadataPhase) -> &crate::text_system::LineLayoutRecording {
        match phase {
            MetadataPhase::Layout => &self.layout_text,
            MetadataPhase::Prepaint => &self.prepaint_text,
            MetadataPhase::Paint => &self.paint_text,
        }
    }

    pub(crate) fn states(
        &self,
        phase: MetadataPhase,
    ) -> &RecordedMetadata<(GlobalElementId, TypeId)> {
        match phase {
            MetadataPhase::Layout => &self.layout_states,
            MetadataPhase::Prepaint => &self.prepaint_states,
            MetadataPhase::Paint => &self.paint_states,
        }
    }
}

#[derive(Default)]
pub(crate) struct ViewNodeScene {
    operations: Vec<crate::scene::PaintOperation>,
    segments: Vec<ViewNodeSceneSegment>,
    operation_count: usize,
    local_start: usize,
}

enum ViewNodeSceneSegment {
    Local(Range<usize>),
    Child(crate::node_engine::ViewNodeId),
}

impl ViewNodeScene {
    pub(crate) fn children(&self) -> impl Iterator<Item = crate::node_engine::ViewNodeId> + '_ {
        self.segments.iter().filter_map(|segment| match segment {
            ViewNodeSceneSegment::Child(child) => Some(*child),
            ViewNodeSceneSegment::Local(_) => None,
        })
    }

    pub(crate) fn begin(&mut self) {
        self.segments.clear();
        self.operation_count = 0;
        self.local_start = 0;
    }

    pub(crate) fn push(&mut self, operation: crate::scene::PaintOperation) {
        if let Some(previous) = self.operations.get_mut(self.operation_count) {
            *previous = operation;
        } else {
            self.operations.push(operation);
        }
        self.operation_count += 1;
    }

    fn finish_local(&mut self) {
        if self.local_start < self.operation_count {
            self.segments.push(ViewNodeSceneSegment::Local(
                self.local_start..self.operation_count,
            ));
        }
        self.local_start = self.operation_count;
    }

    pub(crate) fn push_child(&mut self, child: crate::node_engine::ViewNodeId) {
        self.finish_local();
        self.segments.push(ViewNodeSceneSegment::Child(child));
    }

    pub(crate) fn finish(&mut self) {
        self.finish_local();
        self.operations.truncate(self.operation_count);
    }

    pub(crate) fn replay(&self, scene: &mut Scene, engine: &crate::node_engine::NodeEngine) {
        for segment in &self.segments {
            match segment {
                ViewNodeSceneSegment::Local(local) => {
                    scene.replay_recording(&self.operations[local.clone()])
                }
                ViewNodeSceneSegment::Child(child) => engine.replay_scene(*child, scene),
            }
        }
    }
}

pub(crate) struct NodeLocalState {
    pub(crate) entity: crate::AnyEntity,
    pub(crate) _subscription: crate::Subscription,
}

pub(crate) struct ViewNode {
    pub(crate) local_state: FxHashMap<(GlobalElementId, TypeId), NodeLocalState>,
    pub(crate) accessed_local_state: FxHashSet<(GlobalElementId, TypeId)>,
    pub(crate) layout: Option<LayoutId>,
    pub(crate) occurrence: crate::node_engine::ViewOccurrence,
    pub(crate) parent: Option<super::node_engine::ViewNodeId>,
    pub(crate) children: Vec<super::node_engine::ViewNodeId>,
    pub(crate) next_children: Vec<super::node_engine::ViewNodeId>,
    pub(crate) view_id: EntityId,
    pub(crate) cache_key: ViewNodeCacheKey,
    pub(crate) previous_bounds: Bounds<Pixels>,
    pub(crate) accessed_entities: FxHashSet<EntityId>,
    pub(crate) recording: Option<ViewNodeRecording>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Path, Primitive, ScaledPixels, point, px, rgb};

    fn path_scene(vertices: usize, offset: f32) -> Scene {
        let mut path = Path::new(point(px(offset), px(0.)));
        for index in 0..vertices {
            path.line_to(point(
                px(index as f32 + offset),
                px((index % 2) as f32 + 1.),
            ));
        }
        path.color = rgb(0xabcdef).into();
        let mut path = path.scale(2.);
        path.content_mask.bounds = path.bounds;
        let mut scene = Scene::default();
        scene.insert_primitive(path);
        scene
    }

    fn recorded_path(recording: &ViewNodeScene) -> &Path<ScaledPixels> {
        match recording.operations.first() {
            Some(crate::scene::PaintOperation::Primitive(Primitive::Path(path))) => path,
            _ => panic!("expected a recorded path"),
        }
    }

    fn assert_replay(recording: &ViewNodeScene, expected: &Scene) {
        let mut replayed = Scene::default();
        for segment in &recording.segments {
            if let ViewNodeSceneSegment::Local(range) = segment {
                replayed.replay_recording(&recording.operations[range.clone()]);
            }
        }
        replayed.finish();
        assert_eq!(replayed.snapshot_for_test(), expected.snapshot_for_test());
    }

    #[test]
    fn direct_scene_recording_moves_path_buffers() {
        let mut recording = ViewNodeScene::default();
        for (vertices, offset) in [(64, 0.), (64, 3.), (8, 10.), (32, 5.)] {
            let mut expected = path_scene(vertices, offset);
            let mut scene = Scene::default();
            scene.begin_node_scene(recording);
            let path = expected.paths.first().expect("path").clone();
            let pointer = path.vertices.as_ptr();
            scene.insert_primitive(path);
            recording = scene.finish_node_scene(crate::node_engine::ViewNodeId::default());
            scene.finish();
            expected.finish();
            assert_eq!(scene.snapshot_for_test(), expected.snapshot_for_test());
            let current = recorded_path(&recording).vertices.as_ptr();
            assert_eq!(current, pointer);
            assert_replay(&recording, &expected);
        }
    }
}
