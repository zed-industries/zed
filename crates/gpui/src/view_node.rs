use crate::{
    Bounds, ContentMask, CursorStyleRequest, EntityId, GlobalElementId, Hitbox, LayoutId, Pixels,
    ScaledPixels, Scene, TooltipRequest,
    scene::{LaneCursors, PrimitiveKind},
};
use collections::FxHashMap;
use std::any::TypeId;
use std::ops::Range;

/// The ambient inputs a node's output was recorded under. Every node holds one, so the
/// text style is kept as a 64-bit hash rather than by value; a collision would reuse
/// output under a different style, the same bet occurrence identity makes on its path hash.
#[derive(Clone, PartialEq)]
pub(crate) struct ViewNodeCacheKey {
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) content_mask: ContentMask<Pixels>,
    pub(crate) text_style_hash: u64,
    pub(crate) rem_size: Pixels,
    pub(crate) scale_factor: f32,
    pub(crate) opacity: f32,
    pub(crate) image_cache: Option<EntityId>,
}

impl ViewNodeCacheKey {
    /// Whether output recorded under `self` is valid for a frame whose ambient inputs are
    /// `other`. Bounds and the content mask come from prepaint, so layout leaves them out
    /// and prepaint restarts the render if they differ.
    pub(crate) fn matches(&self, other: &Self, at_layout: bool) -> bool {
        (at_layout || (self.bounds == other.bounds && self.content_mask == other.content_mask))
            && self.image_cache == other.image_cache
            && self.rem_size == other.rem_size
            && self.scale_factor == other.scale_factor
            && self.opacity == other.opacity
            && self.text_style_hash == other.text_style_hash
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MetadataPhase {
    Layout,
    Prepaint,
    Paint,
}

/// Where in the last drawn frame's scene a scope's primitives are, so the scope can be
/// replayed into the next frame without painting again. The frame owns the primitives and
/// their kinds in paint order; the record is the scope's runs of that order, split by the
/// children and layers between them, with each run remembering how far along each kind's
/// lane the frame was when the run began. Replaying walks the run's kinds, copies each
/// primitive out of the rendered frame at that cursor, and paints it, which records the
/// scope afresh at its positions in the new frame.
#[derive(Default)]
pub(crate) struct ViewNodeScene {
    segments: Vec<ViewNodeSceneSegment>,
}

enum ViewNodeSceneSegment {
    /// A run of this scope's own primitives: their range of the frame's paint order, and
    /// the lane cursors at the first.
    Run(Range<u32>, LaneCursors),
    Child(crate::view_tree::ViewNodeId),
    StartLayer(Bounds<ScaledPixels>),
    EndLayer,
}

/// Records a [`ViewNodeScene`] while its scope paints. The run being recorded — where its
/// kinds start and the frame's cursors at that point — lives here rather than in the
/// record, since it only exists mid-paint and every node holds a record between frames.
#[derive(Default)]
pub(crate) struct ViewNodeSceneRecorder {
    scene: ViewNodeScene,
    open_run: Option<(u32, LaneCursors)>,
    open_run_end: u32,
}

impl ViewNodeSceneRecorder {
    pub(crate) fn begin(mut scene: ViewNodeScene) -> Self {
        scene.segments.clear();
        Self {
            scene,
            open_run: None,
            open_run_end: 0,
        }
    }

    /// Records that the frame is taking its `index`th primitive, given its cursors before it.
    pub(crate) fn record_primitive(&mut self, index: u32, cursors: LaneCursors) {
        if self.open_run.is_none() {
            self.open_run = Some((index, cursors));
        }
        self.open_run_end = index + 1;
    }

    pub(crate) fn record_start_layer(&mut self, bounds: Bounds<ScaledPixels>) {
        self.close_run();
        self.scene
            .segments
            .push(ViewNodeSceneSegment::StartLayer(bounds));
    }

    pub(crate) fn record_end_layer(&mut self) {
        self.close_run();
        self.scene.segments.push(ViewNodeSceneSegment::EndLayer);
    }

    fn close_run(&mut self) {
        if let Some((start, cursors)) = self.open_run.take() {
            self.scene
                .segments
                .push(ViewNodeSceneSegment::Run(start..self.open_run_end, cursors));
        }
    }

    pub(crate) fn push_child(&mut self, child: crate::view_tree::ViewNodeId) {
        self.close_run();
        self.scene.segments.push(ViewNodeSceneSegment::Child(child));
    }

    pub(crate) fn finish(mut self) -> ViewNodeScene {
        self.close_run();
        self.scene
    }
}

impl ViewNodeScene {
    /// Paints the scope's primitives from `rendered`, the frame they were last drawn in,
    /// into `scene`, descending into children where they were painted. `scene` is
    /// recording the scope anew, so the record comes out addressing the new frame.
    pub(crate) fn replay(
        &self,
        rendered: &Scene,
        scene: &mut Scene,
        engine: &mut crate::view_tree::ViewTree,
    ) {
        for segment in &self.segments {
            match segment {
                ViewNodeSceneSegment::Run(range, cursors) => {
                    Self::replay_run(range.clone(), *cursors, rendered, scene)
                }
                ViewNodeSceneSegment::Child(child) => engine.replay_scene(*child, rendered, scene),
                ViewNodeSceneSegment::StartLayer(bounds) => scene.push_layer(*bounds),
                ViewNodeSceneSegment::EndLayer => scene.pop_layer(),
            }
        }
    }

    fn replay_run(range: Range<u32>, mut cursor: LaneCursors, rendered: &Scene, scene: &mut Scene) {
        for kind in rendered.painted_kinds(range) {
            match kind {
                PrimitiveKind::Shadow => {
                    scene.insert_primitive(*rendered.painted_shadow(cursor.shadows));
                    cursor.shadows += 1;
                }
                PrimitiveKind::Quad => {
                    scene.insert_primitive(*rendered.painted_quad(cursor.quads));
                    cursor.quads += 1;
                }
                PrimitiveKind::Path => {
                    scene.insert_primitive(rendered.painted_path(cursor.paths).clone());
                    cursor.paths += 1;
                }
                PrimitiveKind::Underline => {
                    scene.insert_primitive(*rendered.painted_underline(cursor.underlines));
                    cursor.underlines += 1;
                }
                PrimitiveKind::MonochromeSprite => {
                    scene.insert_primitive(
                        *rendered.painted_monochrome_sprite(cursor.monochrome_sprites),
                    );
                    cursor.monochrome_sprites += 1;
                }
                PrimitiveKind::SubpixelSprite => {
                    scene.insert_primitive(
                        *rendered.painted_subpixel_sprite(cursor.subpixel_sprites),
                    );
                    cursor.subpixel_sprites += 1;
                }
                PrimitiveKind::PolychromeSprite => {
                    scene.insert_primitive(
                        *rendered.painted_polychrome_sprite(cursor.polychrome_sprites),
                    );
                    cursor.polychrome_sprites += 1;
                }
                PrimitiveKind::Surface => {
                    scene.insert_primitive(rendered.painted_surface(cursor.surfaces).clone());
                    cursor.surfaces += 1;
                }
            }
        }
    }
}

/// One thing a scope produced while drawing. Kinds that are only read by walking the
/// frame live here; a `Child` marks where a child node's output of one phase belongs.
pub(crate) enum OutputItem {
    Child(crate::view_tree::ViewNodeId, MetadataPhase),
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

/// Where a recorded dispatch node, child or root hangs when the scope is replayed: under
/// one of the scope's recorded nodes, or at the scope's attachment point, which is
/// whatever dispatch node is active where the scope is grafted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DispatchParent {
    /// A recorded node of this scope, by index in [`NodeOutput::dispatch_nodes`].
    Recorded(u32),
    Attachment,
}

/// Where a child or root a scope attached hangs in the dispatch tree: the live node it
/// was attached under while the scope drew, and that node resolved against the scope's
/// recorded nodes when they were last copied out, which is what replay uses.
#[derive(Clone, Copy)]
pub(crate) struct DispatchLink {
    pub(crate) live: Option<crate::DispatchNodeId>,
    pub(crate) resolved: DispatchParent,
    /// How many of the scope's recorded nodes were pushed before the attachment. Replay
    /// copies the scope's nodes in one block, but registers their focus and view with
    /// the children's in drawing order, since the last registration of a handle wins.
    pub(crate) preceding: u32,
}

impl DispatchLink {
    pub(crate) fn live(under: Option<crate::DispatchNodeId>) -> Self {
        Self {
            live: under,
            resolved: DispatchParent::Attachment,
            preceding: 0,
        }
    }
}

/// A dispatch node a scope pushed while prepainting that has listeners, a key context, a
/// focus or a view; empty ones are left out, since a walk of the tree cannot tell they
/// were there. Replay follows the phases that produced it: grafting the scope's prepaint
/// adds the node with its focus and view, grafting its paint adds the key context and
/// listeners. A scope that is prepainted but not painted (under `visibility: hidden`)
/// therefore gets the same nodes a fresh frame would give it.
///
/// Like the scene, the node is not copied out of the frame it was drawn in: `source` is its
/// index in that frame's dispatch tree, which replay copies from. Reuse only grafts nodes
/// drawn in the previous frame, and a grafted scope's records are moved onto the copies
/// once its frame is drawn, so `source` always addresses the previous frame's tree.
#[derive(Clone, Copy)]
pub(crate) struct RecordedDispatchNode {
    pub(crate) parent: DispatchParent,
    pub(crate) source: crate::DispatchNodeId,
}

/// What a reused scope attaches into the frame's dispatch tree besides its own recorded
/// nodes. Elements' pushes are not recorded one by one: after paint the scope copies the
/// non-empty nodes out of the live tree (they occupy `PhaseOutput::dispatch_range`), and
/// only where children and roots hang needs remembering.
#[derive(Clone, Copy)]
pub(crate) enum DispatchOp {
    Child(crate::view_tree::ViewNodeId, DispatchLink),
    /// A root this scope attached to the frame with `defer_draw`, drawn after the tree at
    /// the given priority. Rendering the scope emits it; replaying the scope re-attaches
    /// the same root, so a deferred draw survives exactly as long as some drawn output
    /// says it is there. Not descended into: roots are walked from the frame's root list.
    Root(crate::view_tree::ViewNodeId, usize, DispatchLink),
}

/// Identifies an element's state by element id and state type. A state is taken out and put
/// back on every access, so the id path, which can be long, is hashed once when the key is
/// made rather than on each map operation. Equality still compares the full path.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct ElementStateKey {
    hash: u64,
    id: GlobalElementId,
    type_id: TypeId,
}

impl ElementStateKey {
    pub(crate) fn new(id: GlobalElementId, type_id: TypeId) -> Self {
        let mut hasher = collections::FxHasher::default();
        std::hash::Hash::hash(&id, &mut hasher);
        std::hash::Hash::hash(&type_id, &mut hasher);
        Self {
            hash: std::hash::Hasher::finish(&hasher),
            id,
            type_id,
        }
    }
}

impl std::hash::Hash for ElementStateKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

/// Inline views counted by the hash of the element path they render at and their type.
pub(crate) type InlineViewCounts = FxHashMap<(u64, &'static str), u64>;

/// What one scope produced in one phase.
#[derive(Default)]
pub(crate) struct PhaseOutput {
    /// In production order.
    pub(crate) items: Vec<OutputItem>,
    /// The line layouts looked up, held so they stay shaped while the scope is reused.
    pub(crate) text: crate::text_system::TextUse,
    /// The engine frame `text` was looked up in. Zero until the phase first draws.
    pub(crate) text_frame: u64,
}

/// Everything one scope produced while drawing. Items and text are produced by each phase;
/// the dispatch record is prepaint's and the scene is paint's, so they live once. A reused
/// node keeps its output untouched; a redrawn node overwrites it in place.
#[derive(Default)]
pub(crate) struct NodeOutput {
    phases: [PhaseOutput; 3],
    /// The children and roots the scope attached while prepainting, in production order.
    pub(crate) dispatch: Vec<DispatchOp>,
    /// The live dispatch nodes pushed while the scope prepainted, children's included:
    /// pushes are sequential, so they are a range of the frame's tree.
    pub(crate) dispatch_range: Range<u32>,
    /// The scope's own non-empty dispatch nodes, in push order, recorded after paint, or
    /// at the end of a frame the scope prepainted in without painting.
    pub(crate) dispatch_nodes: Vec<RecordedDispatchNode>,
    /// The primitives painted, with the children spliced where they were painted.
    pub(crate) scene: ViewNodeScene,
    /// Bumped whenever the output is rebuilt, so slots issued earlier stop resolving.
    pub(crate) generation: u64,
    /// State kept for elements drawn in this scope, by element id and state type. It
    /// survives redraws; entries not accessed by a redraw are dropped when it finishes.
    /// Each state is stamped with the output generation that last stored it, so the sweep
    /// after a redraw needs no separate record of what the redraw accessed.
    pub(crate) element_states: FxHashMap<ElementStateKey, (u64, crate::window::ElementStateBox)>,
    /// What only some scopes record, boxed so the rest do not pay for it.
    extras: Option<Box<ScopeExtras>>,
}

#[derive(Default)]
struct ScopeExtras {
    /// How many views of each type have rendered inline in this scope so far at each
    /// element path, so siblings of one type get distinct element-id scopes, and a keyed
    /// element's components keep theirs when its siblings change.
    inline_views: InlineViewCounts,
    /// The groups drawn outside this scope that its elements, or its children's, resolved
    /// while painting. Their listeners hold the groups' hitbox ids, so the scope is only
    /// reusable while each name still resolves to the same hitbox.
    group_reads: Vec<GroupRead>,
}

/// A group name a scope resolved, to the hitbox it found (`None` if no enclosing element
/// had the group) and the scope whose element pushed that hitbox.
#[derive(Clone, PartialEq)]
pub(crate) struct GroupRead {
    pub(crate) name: crate::SharedString,
    pub(crate) hitbox: Option<crate::HitboxId>,
    pub(crate) owner: Option<crate::view_tree::ViewNodeId>,
}

impl NodeOutput {
    /// Drops the element states a redraw did not access.
    pub(crate) fn retain_accessed_element_states(&mut self) {
        let generation = self.generation;
        self.element_states
            .retain(|_, (stored_in, _)| *stored_in == generation);
    }

    pub(crate) fn inline_views(&self) -> Option<&InlineViewCounts> {
        self.extras.as_ref().map(|extras| &extras.inline_views)
    }

    pub(crate) fn inline_views_mut(&mut self) -> &mut InlineViewCounts {
        &mut self.extras.get_or_insert_default().inline_views
    }

    pub(crate) fn group_reads(&self) -> &[GroupRead] {
        self.extras
            .as_ref()
            .map_or(&[], |extras| extras.group_reads.as_slice())
    }

    pub(crate) fn group_reads_mut(&mut self) -> &mut Vec<GroupRead> {
        &mut self.extras.get_or_insert_default().group_reads
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

    /// Clears the drawn items ahead of a redraw. Element states are kept so the redraw can
    /// find them; text is kept until the redraw's own use replaces it, after the caller has
    /// seeded it back into the frame cache.
    pub(crate) fn reset(&mut self) {
        for phase in &mut self.phases {
            phase.items.clear();
        }
        self.dispatch.clear();
        if let Some(extras) = &mut self.extras {
            extras.inline_views.clear();
            extras.group_reads.clear();
        }
        self.generation += 1;
    }
}

// Every node holds one of these between frames, so its size is a floor on memory per view.
const _: () = assert!(size_of::<ViewNode>() <= 560);

/// The position of one item in a scope's output. Held instead of the item when the item
/// must stay in place, such as a callback that is leased out for a call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OutputSlot {
    pub(crate) owner: crate::view_tree::ViewNodeId,
    pub(crate) phase: MetadataPhase,
    pub(crate) index: usize,
    pub(crate) generation: u64,
}

pub(crate) struct ViewNode {
    pub(crate) output: NodeOutput,
    pub(crate) layout: Option<LayoutId>,
    pub(crate) occurrence: crate::view_tree::ViewOccurrence,
    pub(crate) parent: Option<super::view_tree::ViewNodeId>,
    pub(crate) children: Vec<super::view_tree::ViewNodeId>,
    pub(crate) next_children: Vec<super::view_tree::ViewNodeId>,
    /// The entity whose notification re-renders this node, once the view has mounted.
    pub(crate) view_id: Option<EntityId>,
    /// An entity the view asked the node to keep for it, such as a component's instance;
    /// dropped with the node. Boxed: most views keep none.
    pub(crate) owned_entity: Option<Box<crate::AnyEntity>>,
    /// The ambient inputs of the last stored render; its bounds are also the node's last
    /// known bounds, which the window repaints when the node is retired.
    pub(crate) cache_key: ViewNodeCacheKey,
    pub(crate) accessed_entities: crate::view_tree::DependencySet,
    /// The engine frame the node's scene record was last stored in, by a paint or a
    /// replay; zero until it first paints. The record addresses that frame's scene, so the
    /// node can only be reused in the frame right after it; a node prepainted but not
    /// painted in a frame renders again the frame after that. Painting also records the
    /// node's layout root as retained, which is what keeps it in the layout tree.
    pub(crate) painted_frame: u64,
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
    use crate::{Path, Quad, point, px, rgb, size};

    fn path(vertices: usize, offset: f32) -> Path<ScaledPixels> {
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
        path
    }

    fn quad(x: f32, width: f32) -> Quad {
        let bounds = Bounds::new(point(px(x), px(0.)), size(px(width), px(10.))).scale(1.);
        let mut quad = Quad::default();
        quad.bounds = bounds;
        quad.content_mask.bounds = bounds;
        quad.background = rgb(0x336699).into();
        quad
    }

    /// A node paints a mix of kinds in a fixed order; replaying its record out of the
    /// finished frame into a fresh scene reproduces the frame, including the draw orders
    /// that depend on that mix (the path is painted over the quad it overlaps), and paths
    /// come out of the frame's lanes rather than a copy of their buffers.
    #[test]
    fn record_replays_a_node_from_the_frame_it_was_drawn_in() {
        let mut rendered = Scene::default();
        rendered.begin_node_scene(ViewNodeScene::default());
        rendered.insert_primitive(quad(0., 40.));
        rendered.insert_primitive(quad(100., 40.));
        let painted_path = path(16, 5.);
        let vertices = painted_path.vertices.as_ptr();
        rendered.insert_primitive(painted_path);
        rendered.insert_primitive(quad(10., 40.));
        let record = rendered.finish_node_scene(crate::view_tree::ViewNodeId::default());
        rendered.finish();

        assert_eq!(rendered.painted_path(0).vertices.as_ptr(), vertices);
        assert_eq!(rendered.quads.len(), 3);
        assert!(
            rendered.painted_quad(2).order > rendered.painted_path(0).order,
            "the last quad overlaps the path, so it is drawn after it"
        );

        let mut replayed = Scene::default();
        let mut engine = crate::view_tree::ViewTree::new();
        record.replay(&rendered, &mut replayed, &mut engine);
        replayed.finish();
        assert_eq!(replayed.snapshot_for_test(), rendered.snapshot_for_test());
    }
}
