use crate::{
    AnyView, Bounds, ContentMask, CursorStyleRequest, EntityId, GlobalElementId, Hitbox, LayoutId,
    Pixels, Scene, TextStyle, TooltipRequest,
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
                "metadata child ranges must not overlap"
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
        cx: &crate::App,
        select: &impl Fn(&ViewNodeRecording, MetadataPhase) -> Option<&Self>,
        emit: &mut impl FnMut(&[T]),
    ) {
        let mut start = 0;
        for child in &self.children {
            emit(&self.local[start..child.local_end]);
            select(engine.recording(child.node, cx), child.phase)
                .expect("child metadata phase exists")
                .replay(engine, cx, select, emit);
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

    #[cfg(any(all(test, target_os = "macos"), feature = "test-memory"))]
    pub(crate) fn operation_buffer_bytes(&self) -> usize {
        self.operations.capacity() * std::mem::size_of::<crate::scene::PaintOperation>()
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

    #[cfg(test)]
    pub(crate) fn record(
        &mut self,
        scene: &Scene,
        range: Range<usize>,
        children: &mut [(Range<usize>, crate::node_engine::ViewNodeId)],
    ) {
        children.sort_unstable_by_key(|(range, _)| (range.start, range.end));
        // Keep existing operations alive until overwritten so path vertex buffers
        // can be reused across dirty frames.
        let mut operation_count = 0;
        self.segments.clear();
        let mut cursor = range.start;
        for (child_range, child) in children {
            if child_range.start < cursor || child_range.end > range.end {
                continue;
            }
            if cursor < child_range.start {
                operation_count =
                    self.record_local(scene, cursor..child_range.start, operation_count);
            }
            self.segments.push(ViewNodeSceneSegment::Child(*child));
            cursor = child_range.end;
        }
        if cursor < range.end {
            operation_count = self.record_local(scene, cursor..range.end, operation_count);
        }
        self.operations.truncate(operation_count);
    }

    #[cfg(test)]
    fn record_local(&mut self, scene: &Scene, range: Range<usize>, start: usize) -> usize {
        let end = scene.recording(range, &mut self.operations, start);
        self.segments.push(ViewNodeSceneSegment::Local(start..end));
        end
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
            scene.use_node_scene_storage(true);
            scene.begin_node_scene(recording);
            let path = expected.paths.first().expect("path").clone();
            let pointer = path.vertices.as_ptr();
            scene.insert_primitive(path);
            recording = scene.finish_node_scene(EntityId::from(1));
            scene.finish();
            expected.finish();
            assert_eq!(scene.snapshot_for_test(), expected.snapshot_for_test());
            assert_eq!(scene.paint_operations.capacity(), 0);
            let current = recorded_path(&recording).vertices.as_ptr();
            assert_eq!(current, pointer);
            assert_replay(&recording, &expected);
        }
    }

    #[test]
    fn path_recording_reuses_vertices_and_replaces_geometry() {
        let mut recording = ViewNodeScene::default();
        let initial = path_scene(64, 0.);
        recording.record(&initial, 0..initial.len(), &mut []);
        let pointer = recorded_path(&recording).vertices.as_ptr();
        let capacity = recorded_path(&recording).vertices.capacity();
        for (vertices, offset) in [(64, 3.), (8, 10.), (32, 5.)] {
            let mut scene = path_scene(vertices, offset);
            scene.finish();
            recording.record(&scene, 0..scene.len(), &mut []);
            assert_eq!(recorded_path(&recording).vertices.as_ptr(), pointer);
            assert_eq!(recorded_path(&recording).vertices.capacity(), capacity);
            assert_replay(&recording, &scene);
            assert_eq!(
                format!("{:?}", recorded_path(&recording)),
                format!("{:?}", scene.paths.first().expect("path"))
            );
        }
        recording.record(&Scene::default(), 0..0, &mut []);
        assert!(recording.operations.is_empty());
        assert!(recording.segments.is_empty());
    }

    #[test]
    fn path_recording_replaces_variants_and_compacts_child_ranges() {
        let mut scene = path_scene(16, 0.);
        let bounds = scene.paths.first().expect("path").bounds;
        scene.push_layer(bounds);
        scene.replay_recording(&path_scene(8, 1.).paint_operations);
        scene.pop_layer();
        let mut recording = ViewNodeScene::default();
        recording.record(&scene, 0..scene.len(), &mut []);
        let child = EntityId::from(1);
        recording.record(&scene, 0..scene.len(), &mut [(0..1, child)]);
        assert!(
            matches!(recording.segments.first(), Some(ViewNodeSceneSegment::Child(id)) if *id == child)
        );
        assert_eq!(recording.operations.len(), scene.len() - 1);
        let mut expected = Scene::default();
        expected.replay_recording(&scene.paint_operations[1..]);
        expected.finish();
        assert_replay(&recording, &expected);
        recording.record(&scene, 0..scene.len(), &mut []);
        scene.finish();
        assert_replay(&recording, &scene);
    }

    #[test]
    #[ignore = "manual capture benchmark; run with --release --ignored --nocapture"]
    fn path_recording_capture_benchmark() {
        let mut scene = Scene::default();
        for offset in 0..32 {
            scene.replay_recording(&path_scene(256, offset as f32).paint_operations);
        }
        scene.finish();
        let mut baseline = ViewNodeScene::default();
        let mut retained = ViewNodeScene::default();
        let mut samples = [Vec::new(), Vec::new()];
        for round in 0..5 {
            for index in if round % 2 == 0 { [0, 1] } else { [1, 0] } {
                let recording = if index == 0 {
                    &mut baseline
                } else {
                    &mut retained
                };
                recording.record(&scene, 0..scene.len(), &mut []);
                let started = std::time::Instant::now();
                for _ in 0..1000 {
                    if index == 0 {
                        recording.operations.clear();
                    }
                    recording.record(std::hint::black_box(&scene), 0..scene.len(), &mut []);
                    std::hint::black_box(&recording.operations);
                }
                samples[index].push(started.elapsed().as_secs_f64() * 1000.);
                assert_replay(recording, &scene);
            }
        }
        for (label, mut samples) in ["clear", "reuse"].into_iter().zip(samples) {
            samples.sort_by(f64::total_cmp);
            eprintln!("{label}: microseconds/capture {samples:?}");
        }
    }
}
