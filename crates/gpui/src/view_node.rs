use crate::{
    Bounds, ContentMask, CursorStyleRequest, EntityId, GlobalElementId, Hitbox, LayoutId, Pixels,
    ScaledPixels, Scene, TextStyle, TooltipRequest,
    scene::{
        MonochromeSprite, PaintSurface, Path, PolychromeSprite, Primitive, Quad, Shadow,
        SubpixelSprite, Underline,
    },
};
use collections::FxHashMap;
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

/// The primitives one scope painted, kept so the scope can be replayed into a later
/// frame's scene without painting again. Primitives are stored by kind, so a glyph costs
/// a glyph's worth of bytes rather than the widest primitive's, and `operations` records
/// their order as small references into those lanes. A redraw overwrites the lanes in
/// place, so a node that paints the same shape every frame allocates nothing.
#[derive(Default)]
pub(crate) struct ViewNodeScene {
    operations: Vec<RecordedOperation>,
    shadows: Lane<Shadow>,
    quads: Lane<Quad>,
    paths: Lane<Path<ScaledPixels>>,
    underlines: Lane<Underline>,
    monochrome_sprites: Lane<MonochromeSprite>,
    subpixel_sprites: Lane<SubpixelSprite>,
    polychrome_sprites: Lane<PolychromeSprite>,
    surfaces: Lane<PaintSurface>,
    layers: Lane<Bounds<ScaledPixels>>,
    segments: Vec<ViewNodeSceneSegment>,
    operation_count: usize,
    local_start: usize,
}

#[derive(Clone, Copy)]
enum RecordedOperation {
    Shadow(u32),
    Quad(u32),
    Path(u32),
    Underline(u32),
    MonochromeSprite(u32),
    SubpixelSprite(u32),
    PolychromeSprite(u32),
    Surface(u32),
    StartLayer(u32),
    EndLayer,
}

/// A vector overwritten from the front on each redraw; `len` is how much of it the
/// current recording uses.
struct Lane<T> {
    items: Vec<T>,
    len: usize,
}

impl<T> Default for Lane<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            len: 0,
        }
    }
}

impl<T> Lane<T> {
    fn push(&mut self, item: T) -> u32 {
        let index = self.len;
        if let Some(slot) = self.items.get_mut(index) {
            *slot = item;
        } else {
            self.items.push(item);
        }
        self.len += 1;
        index as u32
    }

    fn get(&self, index: u32) -> &T {
        &self.items[index as usize]
    }

    fn finish(&mut self) {
        self.items.truncate(self.len);
    }

    fn retained_bytes(&self) -> usize {
        self.items.capacity() * size_of::<T>()
    }
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
        self.shadows.len = 0;
        self.quads.len = 0;
        self.paths.len = 0;
        self.underlines.len = 0;
        self.monochrome_sprites.len = 0;
        self.subpixel_sprites.len = 0;
        self.polychrome_sprites.len = 0;
        self.surfaces.len = 0;
        self.layers.len = 0;
    }

    /// Records a primitive the frame has just taken a copy of. Not-`Copy` kinds are moved
    /// in, so the recording does not clone them a second time.
    pub(crate) fn record_primitive(&mut self, primitive: Primitive) {
        let operation = match primitive {
            Primitive::Shadow(shadow) => RecordedOperation::Shadow(self.shadows.push(shadow)),
            Primitive::Quad(quad) => RecordedOperation::Quad(self.quads.push(quad)),
            Primitive::Path(path) => RecordedOperation::Path(self.paths.push(path)),
            Primitive::Underline(underline) => {
                RecordedOperation::Underline(self.underlines.push(underline))
            }
            Primitive::MonochromeSprite(sprite) => {
                RecordedOperation::MonochromeSprite(self.monochrome_sprites.push(sprite))
            }
            Primitive::SubpixelSprite(sprite) => {
                RecordedOperation::SubpixelSprite(self.subpixel_sprites.push(sprite))
            }
            Primitive::PolychromeSprite(sprite) => {
                RecordedOperation::PolychromeSprite(self.polychrome_sprites.push(sprite))
            }
            Primitive::Surface(surface) => RecordedOperation::Surface(self.surfaces.push(surface)),
        };
        self.push(operation);
    }

    pub(crate) fn record_start_layer(&mut self, bounds: Bounds<ScaledPixels>) {
        let index = self.layers.push(bounds);
        self.push(RecordedOperation::StartLayer(index));
    }

    pub(crate) fn record_end_layer(&mut self) {
        self.push(RecordedOperation::EndLayer);
    }

    fn push(&mut self, operation: RecordedOperation) {
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
        self.operations.capacity() * size_of::<RecordedOperation>()
            + self.segments.capacity() * size_of::<ViewNodeSceneSegment>()
            + self.shadows.retained_bytes()
            + self.quads.retained_bytes()
            + self.paths.retained_bytes()
            + self.underlines.retained_bytes()
            + self.monochrome_sprites.retained_bytes()
            + self.subpixel_sprites.retained_bytes()
            + self.polychrome_sprites.retained_bytes()
            + self.surfaces.retained_bytes()
            + self.layers.retained_bytes()
    }

    pub(crate) fn finish(&mut self) {
        self.finish_local();
        self.operations.truncate(self.operation_count);
        self.shadows.finish();
        self.quads.finish();
        self.paths.finish();
        self.underlines.finish();
        self.monochrome_sprites.finish();
        self.subpixel_sprites.finish();
        self.polychrome_sprites.finish();
        self.surfaces.finish();
        self.layers.finish();
    }

    pub(crate) fn replay(&self, scene: &mut Scene, engine: &crate::node_engine::NodeEngine) {
        for segment in &self.segments {
            match segment {
                ViewNodeSceneSegment::Local(local) => self.replay_local(local.clone(), scene),
                ViewNodeSceneSegment::Child(child) => engine.replay_scene(*child, scene),
            }
        }
    }

    fn replay_local(&self, operations: Range<usize>, scene: &mut Scene) {
        for operation in &self.operations[operations] {
            match *operation {
                RecordedOperation::Shadow(index) => {
                    scene.insert_primitive(*self.shadows.get(index))
                }
                RecordedOperation::Quad(index) => scene.insert_primitive(*self.quads.get(index)),
                RecordedOperation::Path(index) => {
                    scene.insert_primitive(self.paths.get(index).clone())
                }
                RecordedOperation::Underline(index) => {
                    scene.insert_primitive(*self.underlines.get(index))
                }
                RecordedOperation::MonochromeSprite(index) => {
                    scene.insert_primitive(*self.monochrome_sprites.get(index))
                }
                RecordedOperation::SubpixelSprite(index) => {
                    scene.insert_primitive(*self.subpixel_sprites.get(index))
                }
                RecordedOperation::PolychromeSprite(index) => {
                    scene.insert_primitive(*self.polychrome_sprites.get(index))
                }
                RecordedOperation::Surface(index) => {
                    scene.insert_primitive(self.surfaces.get(index).clone())
                }
                RecordedOperation::StartLayer(index) => scene.push_layer(*self.layers.get(index)),
                RecordedOperation::EndLayer => scene.pop_layer(),
            }
        }
    }
}

/// One thing a scope produced while drawing. Kinds that are only read by walking the
/// frame live here; a `Child` marks where a child node's output of one phase belongs.
pub(crate) enum OutputItem {
    Child(crate::node_engine::ViewNodeId, MetadataPhase),
    Hitbox(Hitbox),
    /// Boxed: at 80 bytes the request would otherwise set the size of every item.
    Tooltip(Box<TooltipRequest>),
    CursorStyle(CursorStyleRequest),
    WindowControl(crate::WindowControlArea, Hitbox),
    TabStop(crate::TabStopOperation),
    /// `None` while leased out for a call.
    MouseListener(Option<crate::window::AnyMouseListener>),
    InputHandler(Option<Box<dyn crate::InputHandler>>),
    #[cfg(any(test, feature = "test-support"))]
    DebugBounds(String, Bounds<Pixels>),
}

// Every element pushes items, so their size is paid per element per frame. `Hitbox` is
// the widest common variant; anything wider is boxed.
const _: () = assert!(size_of::<OutputItem>() <= 56);

/// One step of rebuilding the frame's dispatch tree from a reused scope. Every element
/// pushes a dispatch node, so these are kept apart from `items`, which the frame's other
/// walks (hit testing, mouse listeners, cursor styles) would otherwise step over; only
/// where children and roots fall between the pushes matters, so those are repeated here.
#[derive(Clone, Copy)]
pub(crate) enum DispatchOp {
    /// A node pushed by the element being drawn, by its id in the frame's tree. Replaced
    /// by `Push` once the scope has painted and the node has been copied out.
    PushLive(crate::DispatchNodeId),
    /// A recorded node, by its index in [`PhaseOutput::dispatch_nodes`].
    Push(u32),
    Pop,
    Child(crate::node_engine::ViewNodeId),
    /// A root this scope attached to the frame with `defer_draw`, drawn after the tree at
    /// the given priority under the dispatch node active here. Rendering the scope emits
    /// it; replaying the scope re-attaches the same root, so a deferred draw survives
    /// exactly as long as some drawn output says it is there. Not descended into: roots
    /// are walked from the frame's root list.
    Root(crate::node_engine::ViewNodeId, usize),
}

/// What one scope produced in one phase.
#[derive(Default)]
pub(crate) struct PhaseOutput {
    /// In production order.
    pub(crate) items: Vec<OutputItem>,
    /// The dispatch tree the scope built while prepainting, in production order.
    pub(crate) dispatch: Vec<DispatchOp>,
    /// The line layouts looked up, held so they stay shaped while the scope is reused.
    pub(crate) text: crate::text_system::TextUse,
    /// The engine frame `text` was looked up in. Zero until the phase first draws.
    pub(crate) text_frame: u64,
    /// The primitives painted, with the children spliced where they were painted. Only
    /// the paint phase records one.
    pub(crate) scene: ViewNodeScene,
    /// Recorded copies of the dispatch nodes `dispatch` pushes, in push order. Entries
    /// beyond `dispatch_pushes` are stale slots kept for their buffers.
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
    /// Each state is stamped with the output generation that last stored it, so the sweep
    /// after a redraw needs no separate record of what the redraw accessed.
    pub(crate) element_states:
        FxHashMap<(GlobalElementId, TypeId), (u64, crate::window::ElementStateBox)>,
    /// How many views of each type have rendered inline in this scope so far, so siblings
    /// of one type get distinct element-id scopes.
    pub(crate) inline_views: FxHashMap<&'static str, u64>,
}

impl NodeOutput {
    /// Drops the element states a redraw did not access.
    pub(crate) fn retain_accessed_element_states(&mut self) {
        let generation = self.generation;
        self.element_states
            .retain(|_, (stored_in, _)| *stored_in == generation);
    }

    pub(crate) fn phase(&self, phase: MetadataPhase) -> &PhaseOutput {
        &self.phases[phase as usize]
    }

    pub(crate) fn phase_mut(&mut self, phase: MetadataPhase) -> &mut PhaseOutput {
        &mut self.phases[phase as usize]
    }

    pub(crate) fn phases_mut(&mut self) -> impl Iterator<Item = &mut PhaseOutput> {
        self.phases.iter_mut()
    }

    /// The heap this output holds on to between frames, from its containers' capacities.
    /// Boxed listeners, element states and shaped text are counted by their handles, not
    /// what they point to.
    pub(crate) fn retained_bytes(&self) -> usize {
        self.phases
            .iter()
            .map(|phase| {
                phase.items.capacity() * size_of::<OutputItem>()
                    + phase.dispatch.capacity() * size_of::<DispatchOp>()
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
                    + size_of::<(u64, crate::window::ElementStateBox)>())
            + self.inline_views.capacity() * size_of::<(&'static str, u64)>()
    }

    /// Clears the drawn items ahead of a redraw. Element states are kept so the redraw can
    /// find them; text is kept until the redraw's own use replaces it, after the caller has
    /// seeded it back into the frame cache.
    pub(crate) fn reset(&mut self) {
        for phase in &mut self.phases {
            phase.items.clear();
            phase.dispatch.clear();
            phase.dispatch_pushes = 0;
        }
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
    pub(crate) accessed_entities: crate::node_engine::DependencySet,
    /// Whether the node has painted since it was mounted, so its output is complete.
    pub(crate) painted: bool,
    /// Whether the node's recorded output is stale and must be rendered again. Set on
    /// mount, on a notification of something it read, and on every node under a full
    /// refresh; cleared when the node stores a render. The engine counts dirty nodes.
    pub(crate) dirty: bool,
    /// Whether the node's output cannot be reused past this frame: it produced something a
    /// recording cannot hold, such as a measurement closure that may capture the frame
    /// arena. Cleared when the node next renders.
    pub(crate) frame_bound: bool,
    /// The engine frame the node was last mounted in, so a repeated element id in one
    /// frame gets the next occurrence rather than this node.
    pub(crate) mounted_frame: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Path, ScaledPixels, point, px, rgb};

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
            Some(RecordedOperation::Path(index)) => recording.paths.get(*index),
            _ => panic!("expected a recorded path"),
        }
    }

    fn assert_replay(recording: &ViewNodeScene, expected: &Scene) {
        let mut replayed = Scene::default();
        for segment in &recording.segments {
            if let ViewNodeSceneSegment::Local(range) = segment {
                recording.replay_local(range.clone(), &mut replayed);
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
