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

    fn retained_bytes(&self) -> usize {
        self.operations.capacity() * size_of::<crate::scene::PaintOperation>()
            + self.segments.capacity() * size_of::<ViewNodeSceneSegment>()
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

/// One thing a scope produced while drawing. Kinds that are only read by walking the
/// frame live here; a `Child` marks where a child node's output of one phase belongs.
pub(crate) enum OutputItem {
    Child(crate::node_engine::ViewNodeId, MetadataPhase),
    /// A root this scope attached to the frame with `defer_draw`, drawn after the tree at
    /// the given priority. Rendering the scope emits it; replaying the scope re-attaches
    /// the same root, so a deferred draw survives exactly as long as some drawn output
    /// says it is there. Not descended into by walks: roots are walked from the frame's
    /// root list.
    Root(crate::node_engine::ViewNodeId, usize),
    Hitbox(Hitbox),
    Tooltip(TooltipRequest),
    CursorStyle(CursorStyleRequest),
    WindowControl(crate::WindowControlArea, Hitbox),
    TabStop(crate::TabStopOperation),
    /// A dispatch node pushed while prepainting. Its recorded copy lives in
    /// [`PhaseOutput::dispatch_nodes`] at the given index, refreshed once the node has
    /// painted; a reused view pushes it back into the frame's dispatch tree.
    DispatchPush(crate::DispatchNodeId, u32),
    DispatchPop,
    /// `None` while leased out for a call.
    MouseListener(Option<crate::window::AnyMouseListener>),
    InputHandler(Option<Box<dyn crate::InputHandler>>),
    #[cfg(any(test, feature = "test-support"))]
    DebugBounds(String, Bounds<Pixels>),
}

/// What one scope produced in one phase.
#[derive(Default)]
pub(crate) struct PhaseOutput {
    /// In production order.
    pub(crate) items: Vec<OutputItem>,
    /// The line layouts looked up, held so they stay shaped while the scope is reused.
    pub(crate) text: crate::text_system::TextUse,
    /// The primitives painted, with the children spliced where they were painted. Only
    /// the paint phase records one.
    pub(crate) scene: ViewNodeScene,
    /// Recorded copies of the dispatch nodes pushed in this phase, in push order. Kept out
    /// of `items` because they are wide and pushed for every element. Entries beyond
    /// `dispatch_pushes` are stale slots kept for their buffers.
    pub(crate) dispatch_nodes: Vec<crate::key_dispatch::DispatchNode>,
    pub(crate) dispatch_pushes: u32,
}

/// Everything one scope produced while drawing, by phase. A reused node keeps its output
/// untouched; a redrawn node overwrites it in place.
#[derive(Default)]
pub(crate) struct NodeOutput {
    phases: [PhaseOutput; 3],
    /// Bumped whenever the output is rebuilt, so slots issued earlier stop resolving.
    pub(crate) generation: u64,
    /// State kept for elements drawn in this scope, by element id and state type. It
    /// survives redraws; entries not accessed by a redraw are dropped when it finishes.
    pub(crate) element_states: FxHashMap<(GlobalElementId, TypeId), crate::window::ElementStateBox>,
    pub(crate) accessed_element_states: FxHashSet<(GlobalElementId, TypeId)>,
    /// How many views of each type have rendered inline in this scope so far, so siblings
    /// of one type get distinct element-id scopes.
    pub(crate) inline_views: FxHashMap<&'static str, u64>,
}

impl NodeOutput {
    /// Drops the element states a redraw did not access.
    pub(crate) fn retain_accessed_element_states(&mut self) {
        self.element_states
            .retain(|key, _| self.accessed_element_states.contains(key));
    }

    pub(crate) fn phase(&self, phase: MetadataPhase) -> &PhaseOutput {
        &self.phases[phase as usize]
    }

    pub(crate) fn phase_mut(&mut self, phase: MetadataPhase) -> &mut PhaseOutput {
        &mut self.phases[phase as usize]
    }

    pub(crate) fn phases(&self) -> impl Iterator<Item = &PhaseOutput> {
        self.phases.iter()
    }

    /// The heap this output holds on to between frames, from its containers' capacities.
    /// Boxed listeners, element states and shaped text are counted by their handles, not
    /// what they point to.
    pub(crate) fn retained_bytes(&self) -> usize {
        self.phases
            .iter()
            .map(|phase| {
                phase.items.capacity() * size_of::<OutputItem>()
                    + phase.dispatch_nodes.capacity()
                        * size_of::<crate::key_dispatch::DispatchNode>()
                    + phase
                        .dispatch_nodes
                        .iter()
                        .map(|node| node.retained_bytes())
                        .sum::<usize>()
                    + phase.text.retained_bytes()
                    + phase.scene.retained_bytes()
            })
            .sum::<usize>()
            + self.element_states.capacity()
                * (size_of::<(GlobalElementId, TypeId)>()
                    + size_of::<crate::window::ElementStateBox>())
            + self.accessed_element_states.capacity() * size_of::<(GlobalElementId, TypeId)>()
            + self.inline_views.capacity() * size_of::<(&'static str, u64)>()
    }

    /// Clears the drawn items ahead of a redraw. Element states are kept so the redraw can
    /// find them; text is kept until the redraw's own use replaces it, after the caller has
    /// seeded it back into the frame cache.
    pub(crate) fn reset(&mut self) {
        for phase in &mut self.phases {
            phase.items.clear();
            phase.dispatch_pushes = 0;
        }
        self.accessed_element_states.clear();
        self.inline_views.clear();
        self.generation += 1;
    }
}

/// The position of one item in a scope's output. Held instead of the item when the item
/// must stay in place, such as a callback that is leased out for a call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OutputSlot {
    pub(crate) owner: crate::node_engine::ViewNodeId,
    pub(crate) phase: MetadataPhase,
    pub(crate) index: usize,
    pub(crate) generation: u64,
}

pub(crate) struct ViewNode {
    pub(crate) output: NodeOutput,
    pub(crate) layout: Option<LayoutId>,
    pub(crate) occurrence: crate::node_engine::ViewOccurrence,
    pub(crate) parent: Option<super::node_engine::ViewNodeId>,
    pub(crate) children: Vec<super::node_engine::ViewNodeId>,
    pub(crate) next_children: Vec<super::node_engine::ViewNodeId>,
    /// The entity whose notification re-renders this node, once the view has mounted.
    pub(crate) view_id: Option<EntityId>,
    /// An entity the view asked the node to keep for it, such as a component's instance;
    /// dropped with the node.
    pub(crate) owned_entity: Option<crate::AnyEntity>,
    pub(crate) cache_key: ViewNodeCacheKey,
    pub(crate) previous_bounds: Bounds<Pixels>,
    pub(crate) accessed_entities: FxHashSet<EntityId>,
    /// Whether the node has painted since it was mounted, so its output is complete.
    pub(crate) painted: bool,
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
