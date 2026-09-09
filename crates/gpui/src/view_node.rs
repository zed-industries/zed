use crate::{
    Bounds, ContentMask, CursorStyleRequest, EntityId, GlobalElementId, Hitbox, LayoutId, Pixels,
    ScaledPixels, Scene, TextStyle, TooltipRequest,
    scene::{LaneCursors, PrimitiveKind},
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

/// Where in the last drawn frame's scene a scope's primitives are, so the scope can be
/// replayed into the next frame without painting again. The frame owns the primitives;
/// the record is the order they were painted in — one `PrimitiveKind` each — split into
/// runs by the children and layers between them, with each run remembering how far along
/// each kind's lane the frame was when the run began. Replaying walks the kinds, copies
/// each primitive out of the rendered frame at that cursor, and paints it, which records
/// the scope afresh at its positions in the new frame.
#[derive(Default)]
pub(crate) struct ViewNodeScene {
    kinds: Vec<PrimitiveKind>,
    segments: Vec<ViewNodeSceneSegment>,
    kind_count: usize,
    /// The run being recorded: where its kinds start, and the frame's cursors at that
    /// point. `None` between runs.
    open_run: Option<(usize, LaneCursors)>,
}

enum ViewNodeSceneSegment {
    /// A run of this scope's own primitives: their kinds, and the lane cursors at the first.
    Run(Range<usize>, LaneCursors),
    Child(crate::view_tree::ViewNodeId),
    StartLayer(Bounds<ScaledPixels>),
    EndLayer,
}

impl ViewNodeScene {
    pub(crate) fn begin(&mut self) {
        self.segments.clear();
        self.kind_count = 0;
        self.open_run = None;
    }

    /// Records a primitive the frame has just taken, given the frame's cursors before it.
    pub(crate) fn record_primitive(&mut self, kind: PrimitiveKind, cursors: LaneCursors) {
        if self.open_run.is_none() {
            self.open_run = Some((self.kind_count, cursors));
        }
        if let Some(slot) = self.kinds.get_mut(self.kind_count) {
            *slot = kind;
        } else {
            self.kinds.push(kind);
        }
        self.kind_count += 1;
    }

    pub(crate) fn record_start_layer(&mut self, bounds: Bounds<ScaledPixels>) {
        self.close_run();
        self.segments.push(ViewNodeSceneSegment::StartLayer(bounds));
    }

    pub(crate) fn record_end_layer(&mut self) {
        self.close_run();
        self.segments.push(ViewNodeSceneSegment::EndLayer);
    }

    fn close_run(&mut self) {
        if let Some((start, cursors)) = self.open_run.take() {
            self.segments
                .push(ViewNodeSceneSegment::Run(start..self.kind_count, cursors));
        }
    }

    pub(crate) fn push_child(&mut self, child: crate::view_tree::ViewNodeId) {
        self.close_run();
        self.segments.push(ViewNodeSceneSegment::Child(child));
    }

    fn retained_bytes(&self) -> usize {
        self.kinds.capacity() * size_of::<PrimitiveKind>()
            + self.segments.capacity() * size_of::<ViewNodeSceneSegment>()
    }

    pub(crate) fn finish(&mut self) {
        self.close_run();
        self.kinds.truncate(self.kind_count);
    }

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
                ViewNodeSceneSegment::Run(kinds, cursors) => {
                    self.replay_run(kinds.clone(), *cursors, rendered, scene)
                }
                ViewNodeSceneSegment::Child(child) => engine.replay_scene(*child, rendered, scene),
                ViewNodeSceneSegment::StartLayer(bounds) => scene.push_layer(*bounds),
                ViewNodeSceneSegment::EndLayer => scene.pop_layer(),
            }
        }
    }

    fn replay_run(
        &self,
        kinds: Range<usize>,
        mut cursor: LaneCursors,
        rendered: &Scene,
        scene: &mut Scene,
    ) {
        for kind in &self.kinds[kinds] {
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

/// Where a recorded dispatch node, child or root hangs in the frame's dispatch tree.
/// Recorded as the live node active when it was drawn; once the scope has painted and
/// its dispatch nodes are copied out, resolved to one of the copies or to the scope's
/// attachment point, which is whatever is active where the scope is grafted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DispatchParent {
    Live(Option<crate::DispatchNodeId>),
    /// A recorded node of this scope, by index in [`PhaseOutput::dispatch_nodes`].
    Recorded(u32),
    Attachment,
}

/// A dispatch node a scope pushed while prepainting that has listeners, a key context, a
/// focus or a view; empty ones are left out, since a walk of the tree cannot tell they
/// were there.
pub(crate) struct RecordedDispatchNode {
    pub(crate) parent: DispatchParent,
    pub(crate) node: crate::key_dispatch::DispatchNode,
}

/// What a reused scope attaches into the frame's dispatch tree besides its own recorded
/// nodes. Elements' pushes are not recorded one by one: after paint the scope copies the
/// non-empty nodes out of the live tree (they occupy `PhaseOutput::dispatch_range`), and
/// only where children and roots hang needs remembering.
#[derive(Clone, Copy)]
pub(crate) enum DispatchOp {
    Child(crate::view_tree::ViewNodeId, DispatchParent),
    /// A root this scope attached to the frame with `defer_draw`, drawn after the tree at
    /// the given priority. Rendering the scope emits it; replaying the scope re-attaches
    /// the same root, so a deferred draw survives exactly as long as some drawn output
    /// says it is there. Not descended into: roots are walked from the frame's root list.
    Root(crate::view_tree::ViewNodeId, usize, DispatchParent),
}

/// What one scope produced in one phase.
#[derive(Default)]
pub(crate) struct PhaseOutput {
    /// In production order.
    pub(crate) items: Vec<OutputItem>,
    /// The children and roots the scope attached while prepainting, in production order.
    pub(crate) dispatch: Vec<DispatchOp>,
    /// The live dispatch nodes pushed while the scope prepainted, children's included:
    /// pushes are sequential, so they are a range of the frame's tree.
    pub(crate) dispatch_range: Range<usize>,
    /// The line layouts looked up, held so they stay shaped while the scope is reused.
    pub(crate) text: crate::text_system::TextUse,
    /// The engine frame `text` was looked up in. Zero until the phase first draws.
    pub(crate) text_frame: u64,
    /// The primitives painted, with the children spliced where they were painted. Only
    /// the paint phase records one.
    pub(crate) scene: ViewNodeScene,
    /// The scope's own non-empty dispatch nodes, in push order, copied out after paint.
    pub(crate) dispatch_nodes: Vec<RecordedDispatchNode>,
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
                    + phase.dispatch_nodes.capacity() * size_of::<RecordedDispatchNode>()
                    + phase
                        .dispatch_nodes
                        .iter()
                        .map(|recorded| recorded.node.retained_bytes())
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
        }
        self.inline_views.clear();
        self.generation += 1;
    }
}

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
    /// dropped with the node.
    pub(crate) owned_entity: Option<crate::AnyEntity>,
    pub(crate) cache_key: ViewNodeCacheKey,
    pub(crate) previous_bounds: Bounds<Pixels>,
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
