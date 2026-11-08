use crate::{
    AbsoluteLength, App, Bounds, DefiniteLength, Edges, GridTemplate, Length, Pixels, Point, Size,
    Style, Window, size,
    util::{
        ceil_to_device_pixel, round_half_toward_zero, round_stroke_to_device_pixel,
        round_to_device_pixel,
    },
};
use collections::{FxHashMap, FxHashSet};
use slotmap::{DefaultKey, KeyData, SecondaryMap};
use smallvec::SmallVec;
use std::{fmt::Debug, ops::Range, sync::LazyLock};
use taffy::{
    TaffyTree, TraversePartialTree as _,
    geometry::{Point as TaffyPoint, Rect as TaffyRect, Size as TaffySize},
    prelude::{max_content, min_content},
    style::AvailableSpace as TaffyAvailableSpace,
    tree::NodeId,
};

#[cfg(feature = "stacker")]
type StackSafe<T> = stacksafe::StackSafe<T>;
#[cfg(not(feature = "stacker"))]
type StackSafe<T> = T;

type MeasureFn =
    dyn FnMut(Size<Option<Pixels>>, Size<AvailableSpace>, &mut Window, &mut App) -> Size<Pixels>;
type NodeMeasureFn = StackSafe<Box<MeasureFn>>;

enum NodeContext {
    Measure(NodeMeasureFn),
    Text(Box<crate::elements::TextMeasure>),
}

pub struct TaffyLayoutEngine {
    taffy: TaffyTree<NodeContext>,
    /// Keeps layout nodes alive from one frame to the next, so Taffy's layout cache can
    /// skip subtrees whose styles and children didn't change. Without it, the tree is
    /// rebuilt every frame.
    cache: Option<LayoutCache>,
    absolute_layout_bounds: FxHashMap<LayoutId, Bounds<Pixels>>,
    /// Unrounded absolute border-box top-left per-node coordinate in device pixels.
    absolute_outer_origins: FxHashMap<LayoutId, Point<f32>>,
    computed_layouts: FxHashSet<LayoutId>,
    layout_bounds_scratch_space: Vec<LayoutId>,
}

/// Layout nodes requested by elements under one element ID path, in request order.
///
/// Elements without IDs are told apart only by their order within the nearest ancestor
/// with one. Which node a request reuses only affects how much of Taffy's cache survives:
/// a reused node's style, children and measure function are always brought up to date.
#[derive(Default)]
struct LayoutScope {
    frame: u64,
    requested: usize,
    nodes: SmallVec<[NodeId; 4]>,
}

struct CachedNode {
    /// What the node's Taffy style was converted from, compared before converting again.
    style: LayoutStyle,
    /// The last frame the node was requested in.
    frame: u64,
    /// Whether `stretch_auto_size_to_fill` changed the Taffy style since it was converted.
    stretched: bool,
    /// Whether the node has a context in Taffy, tracked here so that requests for nodes
    /// without one don't have to look it up in Taffy's hashed context map.
    has_context: bool,
}

#[derive(Default)]
struct LayoutCache {
    scopes: FxHashMap<u64, LayoutScope>,
    nodes: SecondaryMap<DefaultKey, CachedNode>,
    /// Nodes no element requested last frame, detached from the tree. A request that
    /// misses reuses one before allocating, and those still unused at the end of this
    /// frame are removed.
    free_nodes: Vec<NodeId>,
    frame: u64,
}

fn node_key(id: NodeId) -> DefaultKey {
    KeyData::from_ffi(id.into()).into()
}

static LAYOUT_CACHE_ENABLED: LazyLock<bool> =
    LazyLock::new(|| std::env::var("GPUI_LAYOUT_CACHE").map_or(true, |value| value != "0"));

#[cfg(test)]
thread_local! {
    static TEXT_MEASUREMENTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

const EXPECT_MESSAGE: &str = "we should avoid taffy layout errors by construction if possible";

impl TaffyLayoutEngine {
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();
        taffy.disable_rounding();
        TaffyLayoutEngine {
            taffy,
            cache: LAYOUT_CACHE_ENABLED.then(LayoutCache::default),
            absolute_layout_bounds: FxHashMap::default(),
            absolute_outer_origins: FxHashMap::default(),
            computed_layouts: FxHashSet::default(),
            layout_bounds_scratch_space: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn set_cache_enabled(&mut self, enabled: bool) {
        self.taffy.clear();
        self.cache = enabled.then(LayoutCache::default);
    }

    #[cfg(test)]
    pub(crate) fn node_count(&self) -> usize {
        self.taffy.total_node_count()
    }

    /// Ends the frame: nodes that weren't requested during it are evicted.
    pub fn clear(&mut self) {
        if let Some(cache) = self.cache.as_mut() {
            cache.evict_unrequested(&mut self.taffy);
        } else {
            self.taffy.clear();
        }
        self.absolute_layout_bounds.clear();
        self.absolute_outer_origins.clear();
        self.computed_layouts.clear();
    }

    pub fn request_layout(
        &mut self,
        scope: u64,
        style: &Style,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
    ) -> LayoutId {
        self.request_node(scope, style, rem_size, scale_factor, children, None)
    }

    pub fn request_measured_layout(
        &mut self,
        scope: u64,
        style: &Style,
        rem_size: Pixels,
        scale_factor: f32,
        measure: impl FnMut(
            Size<Option<Pixels>>,
            Size<AvailableSpace>,
            &mut Window,
            &mut App,
        ) -> Size<Pixels>
        + 'static,
    ) -> LayoutId {
        let measure = Box::new(measure) as Box<MeasureFn>;
        #[cfg(feature = "stacker")]
        let measure = StackSafe::new(measure);
        let context = NodeContext::Measure(measure);
        self.request_node(scope, style, rem_size, scale_factor, &[], Some(context))
    }

    pub(crate) fn request_text_layout(
        &mut self,
        scope: u64,
        rem_size: Pixels,
        scale_factor: f32,
        measure: crate::elements::TextMeasure,
    ) -> LayoutId {
        let context = NodeContext::Text(Box::new(measure));
        self.request_node(
            scope,
            &Style::default(),
            rem_size,
            scale_factor,
            &[],
            Some(context),
        )
    }

    fn request_node(
        &mut self,
        scope: u64,
        style: &Style,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
        context: Option<NodeContext>,
    ) -> LayoutId {
        // This is safe because LayoutId is repr(transparent) to taffy::tree::NodeId.
        let children = LayoutId::to_taffy_slice(children);
        let taffy = &mut self.taffy;
        let Some(cache) = self.cache.as_mut() else {
            let style = LayoutStyle::new(style, rem_size, scale_factor).to_taffy();
            let id = match context {
                Some(context) => taffy.new_leaf_with_context(style, context),
                None if children.is_empty() => taffy.new_leaf(style),
                None => taffy.new_with_children(style, children),
            }
            .expect(EXPECT_MESSAGE);
            return id.into();
        };

        let frame = cache.frame;
        let scope = cache.scopes.entry(scope).or_default();
        if scope.frame != frame {
            scope.frame = frame;
            scope.requested = 0;
        }
        let ordinal = scope.requested;
        scope.requested += 1;
        let id = if let Some(&id) = scope.nodes.get(ordinal) {
            let node = &mut cache.nodes[node_key(id)];
            debug_assert_ne!(
                node.frame, frame,
                "layout node requested twice in one frame"
            );
            if node.stretched || !node.style.matches(style, rem_size, scale_factor) {
                node.style = LayoutStyle::new(style, rem_size, scale_factor);
                node.stretched = false;
                let taffy_style = node.style.to_taffy();
                if taffy.style(id).expect(EXPECT_MESSAGE) != &taffy_style {
                    taffy.set_style(id, taffy_style).expect(EXPECT_MESSAGE);
                }
            }
            if !taffy.child_ids(id).eq(children.iter().copied()) {
                taffy.set_children(id, children).expect(EXPECT_MESSAGE);
            }
            id
        } else {
            let style = LayoutStyle::new(style, rem_size, scale_factor);
            let taffy_style = style.to_taffy();
            let id = if let Some(id) = cache.free_nodes.pop() {
                taffy.set_style(id, taffy_style).expect(EXPECT_MESSAGE);
                let node = &mut cache.nodes[node_key(id)];
                node.style = style;
                node.stretched = false;
                id
            } else {
                let id = taffy.new_leaf(taffy_style).expect(EXPECT_MESSAGE);
                let node = CachedNode {
                    style,
                    frame,
                    stretched: false,
                    has_context: false,
                };
                cache.nodes.insert(node_key(id), node);
                id
            };
            if !children.is_empty() {
                // Unlike `new_with_children`, this detaches children from their previous parents.
                taffy.set_children(id, children).expect(EXPECT_MESSAGE);
            }
            scope.nodes.push(id);
            id
        };

        let node = &mut cache.nodes[node_key(id)];
        node.frame = frame;
        let had_context = std::mem::replace(&mut node.has_context, context.is_some());
        match context {
            Some(context) if had_context => {
                let previous = taffy
                    .get_node_context_mut(id)
                    .expect("a node marked as having a context should have one in Taffy");
                // Text that hasn't changed keeps the sizes Taffy cached for it.
                let reuse = matches!(
                    (&context, &*previous),
                    (NodeContext::Text(measure), NodeContext::Text(previous))
                        if measure.reuse_measurement(previous)
                );
                // Replaced in place because `set_node_context` would always dirty the node.
                *previous = context;
                // A measure function can depend on anything, so replacing it must
                // invalidate the sizes Taffy cached from the previous one.
                if !reuse {
                    taffy.mark_dirty(id).expect(EXPECT_MESSAGE);
                }
            }
            // Adding or removing a context dirties the node.
            Some(context) => taffy
                .set_node_context(id, Some(context))
                .expect(EXPECT_MESSAGE),
            None if had_context => taffy.set_node_context(id, None).expect(EXPECT_MESSAGE),
            None => {}
        }
        id.into()
    }

    /// Treats any `auto` dimension of the given node's style as filling `size`.
    ///
    /// This is applied to window roots before layout so they behave like the
    /// root element on the web, which stretches to fill the initial containing
    /// block (the viewport) unless given an explicit size. Explicitly styled
    /// dimensions are preserved.
    pub fn stretch_auto_size_to_fill(
        &mut self,
        id: LayoutId,
        size: Size<Pixels>,
        scale_factor: f32,
    ) {
        let style = self.taffy.style(id.0).expect(EXPECT_MESSAGE);
        let stretch_width = style.size.width.is_auto();
        let stretch_height = style.size.height.is_auto();
        if !stretch_width && !stretch_height {
            return;
        }
        // The next request for this node must restore its style, unless it is stretched
        // again; it may not be a root by then.
        if let Some(node) = self
            .cache
            .as_mut()
            .and_then(|cache| cache.nodes.get_mut(node_key(id.0)))
        {
            node.stretched = true;
        }
        let mut style = style.clone();
        if stretch_width {
            style.size.width =
                taffy::style::Dimension::length(round_to_device_pixel(size.width.0, scale_factor));
        }
        if stretch_height {
            style.size.height =
                taffy::style::Dimension::length(round_to_device_pixel(size.height.0, scale_factor));
        }
        self.taffy.set_style(id.0, style).expect(EXPECT_MESSAGE);
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn count_all_children(&self, parent: LayoutId) -> anyhow::Result<u32> {
        let mut count = 0;

        for child in self.taffy.children(parent.0)? {
            // Count this child.
            count += 1;

            // Count all of this child's children.
            count += self.count_all_children(LayoutId(child))?
        }

        Ok(count)
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn max_depth(&self, depth: u32, parent: LayoutId) -> anyhow::Result<u32> {
        println!(
            "{parent:?} at depth {depth} has {} children",
            self.taffy.child_count(parent.0)
        );

        let mut max_child_depth = 0;

        for child in self.taffy.children(parent.0)? {
            max_child_depth = std::cmp::max(max_child_depth, self.max_depth(0, LayoutId(child))?);
        }

        Ok(depth + 1 + max_child_depth)
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn get_edges(&self, parent: LayoutId) -> anyhow::Result<Vec<(LayoutId, LayoutId)>> {
        let mut edges = Vec::new();

        for child in self.taffy.children(parent.0)? {
            edges.push((parent, LayoutId(child)));

            edges.extend(self.get_edges(LayoutId(child))?);
        }

        Ok(edges)
    }

    #[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
    pub fn compute_layout(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        // Leaving this here until we have a better instrumentation approach.
        // println!("Laying out {} children", self.count_all_children(id)?);
        // println!("Max layout depth: {}", self.max_depth(0, id)?);

        // Output the edges (branches) of the tree in Mermaid format for visualization.
        // println!("Edges:");
        // for (a, b) in self.get_edges(id)? {
        //     println!("N{} --> N{}", u64::from(a), u64::from(b));
        // }
        //

        // A node computed as a root may still be attached to a parent from an earlier
        // frame that no element requested in this one. Its layout must not be relative to
        // that parent.
        if let Some(cache) = self.cache.as_ref()
            && let Some(parent) = self.taffy.parent(id.0)
            && cache
                .nodes
                .get(node_key(parent))
                .is_none_or(|node| node.frame != cache.frame)
        {
            self.taffy.remove_child(parent, id.0).expect(EXPECT_MESSAGE);
        }

        if !self.computed_layouts.insert(id) {
            let stack = &mut self.layout_bounds_scratch_space;
            stack.push(id);
            while let Some(id) = stack.pop() {
                self.absolute_layout_bounds.remove(&id);
                self.absolute_outer_origins.remove(&id);
                stack.extend(
                    self.taffy
                        .children(id.into())
                        .expect(EXPECT_MESSAGE)
                        .into_iter()
                        .map(LayoutId::from),
                );
            }
        }

        let scale_factor = window.scale_factor();

        let transform = |v: AvailableSpace| match v {
            AvailableSpace::Definite(pixels) => {
                AvailableSpace::Definite(Pixels(pixels.0 * scale_factor))
            }
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        };
        let available_space = size(
            transform(available_space.width),
            transform(available_space.height),
        );

        self.taffy
            .compute_layout_with_measure(
                id.into(),
                available_space.into(),
                |input, _, node_context, style| {
                    taffy::compute_leaf_layout(
                        input,
                        style,
                        |_, _| 0.0,
                        |known_dimensions, available_space| {
                            let Some(node_context) = node_context else {
                                return taffy::geometry::Size::default();
                            };

                            let known_dimensions = Size {
                                width: known_dimensions.width.map(|e| Pixels(e / scale_factor)),
                                height: known_dimensions.height.map(|e| Pixels(e / scale_factor)),
                            };

                            let available_space: Size<AvailableSpace> = available_space.into();
                            let untransform = |ev: AvailableSpace| match ev {
                                AvailableSpace::Definite(pixels) => {
                                    AvailableSpace::Definite(Pixels(pixels.0 / scale_factor))
                                }
                                AvailableSpace::MinContent => AvailableSpace::MinContent,
                                AvailableSpace::MaxContent => AvailableSpace::MaxContent,
                            };
                            let available_space = size(
                                untransform(available_space.width),
                                untransform(available_space.height),
                            );

                            let measured_size = match node_context {
                                NodeContext::Measure(measure) => {
                                    measure(known_dimensions, available_space, window, cx)
                                }
                                NodeContext::Text(measure) => {
                                    #[cfg(test)]
                                    TEXT_MEASUREMENTS.set(TEXT_MEASUREMENTS.get() + 1);
                                    let final_layout =
                                        input.run_mode == taffy::RunMode::PerformLayout;
                                    measure.measure(
                                        known_dimensions,
                                        available_space,
                                        final_layout,
                                        window,
                                        cx,
                                    )
                                }
                            };
                            snap_measured_size_to_device_pixels(measured_size, scale_factor).into()
                        },
                    )
                },
            )
            .expect(EXPECT_MESSAGE);
    }

    // Pixel snapping
    //
    // Painting primitives at non-integer pixel coordinates produces blurry
    // output. Pixel snapping converts layout coordinates into integer
    // device-pixel coordinates so painted edges land exactly on physical
    // pixel boundaries.
    //
    // Non-integer coordinates can arise for several reasons, including:
    //   - flex distribution, percentages, centering, and text measurement
    //     can produce fractional element sizes and positions;
    //   - at fractional scale factors (for example 125% or 150%), integer
    //     logical-pixel values can map to non-integer device-pixel values.
    //
    // We pixel-snap by rounding in device-pixel space, after multiplying
    // by `scale_factor`, so that snapping targets physical pixels. Bounds
    // are divided by `scale_factor` before being returned to GPUI.
    //
    // Midpoints are rounded toward zero. This is a stylistic choice: a
    // 1-logical-pixel line at 150% scale should render as 1 dp rather than
    // 2 dp.
    //
    // Pixel snapping is done in two phases:
    //
    //  1. Pre-layout metric snapping. Before Taffy computes layout, all
    //     authored absolute lengths are rounded in `to_taffy`. This
    //     includes borders, padding, gaps, and explicit sizes.
    //     Custom-measured leaf nodes have their measured sizes rounded up
    //     to integer device-pixel lengths.
    //
    //  2. Post-layout edge snapping. After Taffy resolves the tree, layout
    //     relationships such as flex shares, grid tracks, percentages, and
    //     centering can produce new fractional edge positions. Boxes now
    //     have edges in absolute coordinates, and snapping must decide
    //     where those edges land on the device-pixel grid.
    //
    // Ideally, post-layout snapping would satisfy:
    //
    //  - Edge closure. Two raw layout edges at the same absolute position
    //    should snap to the same pixel column.
    //  - Translation stability. A component's internal geometry should not
    //    change when it moves to a new absolute position.
    //
    // These goals are in tension because rounding is not associative.
    // The simple local schemes make different tradeoffs:
    //
    //  - Absolute edge rounding gives each window coordinate one answer,
    //    so coincident edges always close globally. But a span's snapped
    //    length is `round(far) - round(near)`, which may change by 1 dp
    //    as its absolute origin moves.
    //
    //  - Parent-relative edge rounding rounds each child inside its
    //    parent's coordinate space. This guarantees translation stability,
    //    but a shared edge reached through different parents can
    //    accumulate different rounding, causing non-closure between
    //    cousins.
    //
    //  - Length rounding rounds each width, height, and thickness
    //    independently and then places boxes from those rounded lengths.
    //    Sizes stay stable under translation, but neighboring boxes derive
    //    their shared boundary from different sources, so closure is not
    //    guaranteed.
    //
    // We apply absolute edge rounding for each element's outer box in
    // post-layout rounding to preserve closure. Border and padding widths
    // are not touched by post-layout rounding; they keep their pre-layout
    // rounded value so that they remain stable under translation.
    //
    // This gives both closure and translation stability in the case that
    // all local metrics are integer device-pixel lengths. Pre-layout
    // rounding covers that in most cases. The exception is metrics
    // resolved by layout relationships, such as percentages. Outer box
    // edges will still close globally, and painted border widths are still
    // snapped independently, but the raw content-box origin can carry a
    // 1dp residual into descendants.

    pub fn layout_bounds(&mut self, id: LayoutId, scale_factor: f32) -> Bounds<Pixels> {
        if let Some(layout) = self.absolute_layout_bounds.get(&id).cloned() {
            return layout;
        }

        let layout = self.taffy.layout(id.into()).expect(EXPECT_MESSAGE);
        let layout_location = layout.location;
        let layout_size = layout.size;
        let parent = self.taffy.parent(id.0);

        let absolute_outer_origin = match parent {
            Some(parent_id) => {
                let parent_id = LayoutId::from(parent_id);
                self.layout_bounds(parent_id, scale_factor);
                let parent_origin = *self
                    .absolute_outer_origins
                    .get(&parent_id)
                    .expect("parent absolute outer origin should be cached");
                parent_origin + Point::from(layout_location)
            }
            None => Point::from(layout_location),
        };
        self.absolute_outer_origins
            .insert(id, absolute_outer_origin);

        let absolute_far = absolute_outer_origin + Point::from(Size::from(layout_size));
        let snapped_bounds = Bounds::from_corners(
            absolute_outer_origin.map(round_half_toward_zero),
            absolute_far.map(round_half_toward_zero),
        );

        let bounds = (snapped_bounds / scale_factor).map(Pixels);
        self.absolute_layout_bounds.insert(id, bounds);
        bounds
    }
}

impl LayoutCache {
    fn evict_unrequested(&mut self, taffy: &mut TaffyTree<NodeContext>) {
        for id in self.free_nodes.drain(..) {
            self.nodes.remove(node_key(id));
            taffy.remove(id).expect(EXPECT_MESSAGE);
        }

        let frame = self.frame;
        let free_nodes = &mut self.free_nodes;
        self.scopes.retain(|_, scope| {
            let requested = if scope.frame == frame {
                scope.requested
            } else {
                0
            };
            free_nodes.extend(scope.nodes.drain(requested..));
            requested > 0
        });

        // Every node an element requested this frame had its children set to nodes also
        // requested this frame, so an unrequested node's parent is unrequested too (or it
        // has none). Clearing their children therefore detaches all of them without
        // touching the live tree. Measure functions can hold on to anything, so they're
        // dropped now rather than when the node is reused or removed.
        for &id in &self.free_nodes {
            let node = &mut self.nodes[node_key(id)];
            if node.has_context {
                node.has_context = false;
                taffy.set_node_context(id, None).expect(EXPECT_MESSAGE);
            }
            if taffy.child_count(id) > 0 {
                taffy.set_children(id, &[]).expect(EXPECT_MESSAGE);
            }
        }
        debug_assert!(self.free_nodes.iter().all(|&id| taffy.parent(id).is_none()));
        self.frame += 1;
    }
}

/// A unique identifier for a layout node, generated when requesting a layout from Taffy
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct LayoutId(NodeId);

impl LayoutId {
    fn to_taffy_slice(node_ids: &[Self]) -> &[taffy::NodeId] {
        // SAFETY: LayoutId is repr(transparent) to taffy::tree::NodeId.
        unsafe { std::mem::transmute::<&[LayoutId], &[taffy::NodeId]>(node_ids) }
    }
}

impl std::hash::Hash for LayoutId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0).hash(state);
    }
}

impl From<NodeId> for LayoutId {
    fn from(node_id: NodeId) -> Self {
        Self(node_id)
    }
}

impl From<LayoutId> for NodeId {
    fn from(layout_id: LayoutId) -> NodeId {
        layout_id.0
    }
}

fn snap_measured_size_to_device_pixels(size: Size<Pixels>, scale_factor: f32) -> Size<f32> {
    size.map(|d| ceil_to_device_pixel(d.0.max(0.0), scale_factor))
}

fn border_widths_to_taffy(
    widths: &Edges<AbsoluteLength>,
    rem_size: Pixels,
    scale_factor: f32,
) -> TaffyRect<taffy::style::LengthPercentage> {
    let snap = |w: &AbsoluteLength| {
        taffy::style::LengthPercentage::length(round_stroke_to_device_pixel(
            w.to_pixels(rem_size).0,
            scale_factor,
        ))
    };
    TaffyRect {
        top: snap(&widths.top),
        right: snap(&widths.right),
        bottom: snap(&widths.bottom),
        left: snap(&widths.left),
    }
}

trait ToTaffy<Output> {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> Output;
}

// Conversion reads the copied fields, not `Style`, so a field can't affect layout
// without also being compared to decide whether to convert again.
macro_rules! layout_style {
    ($($field:ident: $field_type:ty),* $(,)?) => {
        /// The fields of a [`Style`] that affect layout, and what their lengths are
        /// resolved against.
        #[derive(Clone, Debug)]
        struct LayoutStyle {
            rem_size: Pixels,
            scale_factor: f32,
            $($field: $field_type,)*
        }

        impl LayoutStyle {
            fn new(style: &Style, rem_size: Pixels, scale_factor: f32) -> Self {
                Self {
                    rem_size,
                    scale_factor,
                    $($field: Clone::clone(&style.$field),)*
                }
            }

            fn matches(&self, style: &Style, rem_size: Pixels, scale_factor: f32) -> bool {
                self.rem_size == rem_size
                    && self.scale_factor == scale_factor
                    $(&& self.$field == style.$field)*
            }
        }
    };
}

layout_style! {
    display: crate::Display,
    overflow: Point<crate::Overflow>,
    scrollbar_width: AbsoluteLength,
    position: crate::Position,
    inset: Edges<Length>,
    size: Size<Length>,
    min_size: Size<Length>,
    max_size: Size<Length>,
    aspect_ratio: Option<f32>,
    margin: Edges<Length>,
    padding: Edges<DefiniteLength>,
    border_widths: Edges<AbsoluteLength>,
    align_items: Option<crate::AlignItems>,
    align_self: Option<crate::AlignSelf>,
    align_content: Option<crate::AlignContent>,
    justify_content: Option<crate::JustifyContent>,
    gap: Size<DefiniteLength>,
    flex_direction: crate::FlexDirection,
    flex_wrap: crate::FlexWrap,
    flex_basis: Length,
    flex_grow: f32,
    flex_shrink: f32,
    grid_rows: Option<GridTemplate>,
    grid_cols: Option<GridTemplate>,
    grid_location: Option<crate::GridLocation>,
}

impl LayoutStyle {
    fn to_taffy(&self) -> taffy::style::Style {
        use taffy::style_helpers::{fr, length, minmax, repeat};

        let rem_size = self.rem_size;
        let scale_factor = self.scale_factor;

        fn to_grid_line(
            placement: &Range<crate::GridPlacement>,
        ) -> taffy::Line<taffy::GridPlacement> {
            taffy::Line {
                start: placement.start.into(),
                end: placement.end.into(),
            }
        }

        fn to_grid_repeat<T: taffy::style::CheapCloneStr>(
            unit: &Option<GridTemplate>,
        ) -> Vec<taffy::GridTemplateComponent<T>> {
            unit.map(|template| {
                match template.min_size {
                    // grid-template-*: repeat(<number>, minmax(0, 1fr));
                    crate::GridTemplateMinSize::Zero => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(length(0.0_f32), fr(1.0_f32))],
                        )]
                    }
                    // grid-template-*: repeat(<number>, minmax(min-content, 1fr));
                    crate::GridTemplateMinSize::MinContent => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(min_content(), fr(1.0_f32))],
                        )]
                    }
                    // grid-template-*: repeat(<number>, minmax(0, max-content))
                    crate::GridTemplateMinSize::MaxContent => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(length(0.0_f32), max_content())],
                        )]
                    }
                }
            })
            .unwrap_or_default()
        }

        taffy::style::Style {
            display: self.display.into(),
            overflow: self.overflow.into(),
            scrollbar_width: self.scrollbar_width.to_taffy(rem_size, scale_factor),
            position: self.position.into(),
            inset: self.inset.to_taffy(rem_size, scale_factor),
            size: self.size.to_taffy(rem_size, scale_factor),
            min_size: self.min_size.to_taffy(rem_size, scale_factor),
            max_size: self.max_size.to_taffy(rem_size, scale_factor),
            aspect_ratio: self.aspect_ratio,
            margin: self.margin.to_taffy(rem_size, scale_factor),
            padding: self.padding.to_taffy(rem_size, scale_factor),
            border: border_widths_to_taffy(&self.border_widths, rem_size, scale_factor),
            align_items: self.align_items.map(Into::into).unwrap_or_default(),
            align_self: self.align_self.map(|x| x.into()),
            align_content: self.align_content.map(Into::into).unwrap_or_default(),
            justify_content: self.justify_content.map(Into::into).unwrap_or_default(),
            gap: self.gap.to_taffy(rem_size, scale_factor),
            flex_direction: self.flex_direction.into(),
            flex_wrap: self.flex_wrap.into(),
            flex_basis: self.flex_basis.to_taffy(rem_size, scale_factor),
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            grid_template_rows: to_grid_repeat(&self.grid_rows),
            grid_template_columns: to_grid_repeat(&self.grid_cols),
            grid_row: self
                .grid_location
                .as_ref()
                .map(|location| to_grid_line(&location.row))
                .unwrap_or_default(),
            grid_column: self
                .grid_location
                .as_ref()
                .map(|location| to_grid_line(&location.column))
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl ToTaffy<f32> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> f32 {
        round_to_device_pixel(self.to_pixels(rem_size).0, scale_factor)
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for Length {
    fn to_taffy(
        &self,
        rem_size: Pixels,
        scale_factor: f32,
    ) -> taffy::prelude::LengthPercentageAuto {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size, scale_factor),
            Length::Auto => taffy::prelude::LengthPercentageAuto::auto(),
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for Length {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::prelude::Dimension {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size, scale_factor),
            Length::Auto => taffy::prelude::Dimension::auto(),
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentage {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentage::percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentageAuto {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentageAuto::percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Dimension {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => taffy::style::Dimension::percent(*fraction),
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentage {
        taffy::style::LengthPercentage::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentageAuto {
        taffy::style::LengthPercentageAuto::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl ToTaffy<taffy::style::Dimension> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Dimension {
        taffy::style::Dimension::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl<T, T2> From<TaffyPoint<T>> for Point<T2>
where
    T: Into<T2>,
    T2: Clone + Debug + Default + PartialEq,
{
    fn from(point: TaffyPoint<T>) -> Point<T2> {
        Point {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl<T, T2> From<Point<T>> for TaffyPoint<T2>
where
    T: Into<T2> + Clone + Debug + Default + PartialEq,
{
    fn from(val: Point<T>) -> Self {
        TaffyPoint {
            x: val.x.into(),
            y: val.y.into(),
        }
    }
}

impl<T, U> ToTaffy<TaffySize<U>> for Size<T>
where
    T: ToTaffy<U> + Clone + Debug + Default + PartialEq,
{
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> TaffySize<U> {
        TaffySize {
            width: self.width.to_taffy(rem_size, scale_factor),
            height: self.height.to_taffy(rem_size, scale_factor),
        }
    }
}

impl<T, U> ToTaffy<TaffyRect<U>> for Edges<T>
where
    T: ToTaffy<U> + Clone + Debug + Default + PartialEq,
{
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> TaffyRect<U> {
        TaffyRect {
            top: self.top.to_taffy(rem_size, scale_factor),
            right: self.right.to_taffy(rem_size, scale_factor),
            bottom: self.bottom.to_taffy(rem_size, scale_factor),
            left: self.left.to_taffy(rem_size, scale_factor),
        }
    }
}

impl<T, U> From<TaffySize<T>> for Size<U>
where
    T: Into<U>,
    U: Clone + Debug + Default + PartialEq,
{
    fn from(taffy_size: TaffySize<T>) -> Self {
        Size {
            width: taffy_size.width.into(),
            height: taffy_size.height.into(),
        }
    }
}

impl<T, U> From<Size<T>> for TaffySize<U>
where
    T: Into<U> + Clone + Debug + Default + PartialEq,
{
    fn from(size: Size<T>) -> Self {
        TaffySize {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

/// The space available for an element to be laid out in
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(Pixels),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    #[default]
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}

impl AvailableSpace {
    /// Returns a `Size` with both width and height set to `AvailableSpace::MinContent`.
    ///
    /// This function is useful when you want to create a `Size` with the minimum content constraints
    /// for both dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use gpui::AvailableSpace;
    /// let min_content_size = AvailableSpace::min_size();
    /// assert_eq!(min_content_size.width, AvailableSpace::MinContent);
    /// assert_eq!(min_content_size.height, AvailableSpace::MinContent);
    /// ```
    pub const fn min_size() -> Size<Self> {
        Size {
            width: Self::MinContent,
            height: Self::MinContent,
        }
    }
}

impl From<AvailableSpace> for TaffyAvailableSpace {
    fn from(space: AvailableSpace) -> TaffyAvailableSpace {
        match space {
            AvailableSpace::Definite(Pixels(value)) => TaffyAvailableSpace::Definite(value),
            AvailableSpace::MinContent => TaffyAvailableSpace::MinContent,
            AvailableSpace::MaxContent => TaffyAvailableSpace::MaxContent,
        }
    }
}

impl From<TaffyAvailableSpace> for AvailableSpace {
    fn from(space: TaffyAvailableSpace) -> AvailableSpace {
        match space {
            TaffyAvailableSpace::Definite(value) => AvailableSpace::Definite(Pixels(value)),
            TaffyAvailableSpace::MinContent => AvailableSpace::MinContent,
            TaffyAvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

impl From<Pixels> for AvailableSpace {
    fn from(pixels: Pixels) -> Self {
        AvailableSpace::Definite(pixels)
    }
}

impl From<Size<Pixels>> for Size<AvailableSpace> {
    fn from(size: Size<Pixels>) -> Self {
        Size {
            width: AvailableSpace::Definite(size.width),
            height: AvailableSpace::Definite(size.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnyElement, AnyWindowHandle, AppContext as _, Context, Element, ElementId, GlobalElementId,
        InspectorElementId, InteractiveElement as _, IntoElement, ParentElement as _, Render,
        Styled as _, TestAppContext, div, px,
        randomized_element_tree::{RandomizedElementTree, RandomizedElementTreeConfig},
        rems,
    };
    use std::{cell::Cell, rc::Rc};

    type Frame = Box<dyn Fn(&mut Window) -> AnyElement>;

    struct FrameView {
        frame: Rc<Cell<usize>>,
        frames: Rc<Vec<Frame>>,
    }

    impl Render for FrameView {
        fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            (self.frames[self.frame.get()])(window)
        }
    }

    /// Draws each frame in a window that keeps its layout nodes and in one that rebuilds
    /// them, checking that every element with a debug selector has the same bounds in
    /// both. Returns how many times the cached window measured text in each frame.
    fn assert_frames_match_uncached(cx: &mut TestAppContext, frames: Vec<Frame>) -> Vec<usize> {
        let frames = Rc::new(frames);
        let frame = Rc::new(Cell::new(0));
        let windows = [true, false].map(|layout_cache| {
            let frames = frames.clone();
            let frame = frame.clone();
            AnyWindowHandle::from(cx.open_window(size(px(800.), px(600.)), move |window, _| {
                window.set_layout_cache_enabled(layout_cache);
                FrameView { frame, frames }
            }))
        });
        let mut text_measurements = Vec::new();
        for index in 0..frames.len() {
            frame.set(index);
            let [cached, uncached] = windows.map(|window| {
                cx.update_window(window, |_, window, cx| {
                    let measured_before = TEXT_MEASUREMENTS.get();
                    window.refresh();
                    window.draw(cx).clear(cx);
                    let mut bounds = window
                        .rendered_frame
                        .debug_bounds
                        .iter()
                        .map(|(selector, bounds)| (selector.clone(), *bounds))
                        .collect::<Vec<_>>();
                    bounds.sort_by(|a, b| a.0.cmp(&b.0));
                    (bounds, TEXT_MEASUREMENTS.get() - measured_before)
                })
                .expect("window should remain open")
            });
            assert!(
                !uncached.0.is_empty(),
                "frame {index} has no debug selectors"
            );
            assert_eq!(cached.0, uncached.0, "frame {index}");
            text_measurements.push(cached.1);
        }
        text_measurements
    }

    fn frame(render: impl Fn(&mut Window) -> AnyElement + 'static) -> Frame {
        Box::new(render)
    }

    /// A leaf whose size comes from its measure function, which captures it.
    struct MeasuredLeaf(Size<Pixels>);

    impl IntoElement for MeasuredLeaf {
        type Element = Self;

        fn into_element(self) -> Self {
            self
        }
    }

    impl Element for MeasuredLeaf {
        type RequestLayoutState = ();
        type PrepaintState = ();

        fn id(&self) -> Option<ElementId> {
            None
        }

        fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
            None
        }

        fn request_layout(
            &mut self,
            _: Option<&GlobalElementId>,
            _: Option<&InspectorElementId>,
            window: &mut Window,
            _: &mut App,
        ) -> (LayoutId, ()) {
            let measured = self.0;
            let layout_id =
                window.request_measured_layout(Style::default(), move |_, _, _, _| measured);
            (layout_id, ())
        }

        fn prepaint(
            &mut self,
            _: Option<&GlobalElementId>,
            _: Option<&InspectorElementId>,
            _: Bounds<Pixels>,
            _: &mut (),
            _: &mut Window,
            _: &mut App,
        ) {
        }

        fn paint(
            &mut self,
            _: Option<&GlobalElementId>,
            _: Option<&InspectorElementId>,
            _: Bounds<Pixels>,
            _: &mut (),
            _: &mut (),
            _: &mut Window,
            _: &mut App,
        ) {
        }
    }

    #[crate::test]
    fn stretched_root_style_is_restored_when_its_node_is_reused(cx: &mut TestAppContext) {
        // Elements without IDs are matched by request order, and children request first,
        // so the second frame's child reuses the first frame's root node.
        assert_frames_match_uncached(
            cx,
            vec![
                frame(|_| div().debug_selector(|| "root".into()).into_any_element()),
                frame(|_| {
                    div()
                        .size(px(100.))
                        .child(div().debug_selector(|| "child".into()))
                        .into_any_element()
                }),
            ],
        );
    }

    #[crate::test]
    fn root_is_detached_from_a_parent_no_longer_requested(cx: &mut TestAppContext) {
        // The second frame's root reuses the first frame's innermost node, whose parent
        // sat inside padding.
        assert_frames_match_uncached(
            cx,
            vec![
                frame(|_| {
                    div()
                        .p(px(10.))
                        .child(
                            div()
                                .p(px(7.))
                                .child(div().size(px(5.)).debug_selector(|| "leaf".into())),
                        )
                        .into_any_element()
                }),
                frame(|_| {
                    div()
                        .size(px(50.))
                        .debug_selector(|| "root".into())
                        .into_any_element()
                }),
            ],
        );
    }

    #[crate::test]
    fn measured_leaf_is_measured_again_when_its_style_is_unchanged(cx: &mut TestAppContext) {
        let measured = |width: f32, height: f32| {
            frame(move |_| {
                div()
                    .flex()
                    .child(
                        div()
                            .debug_selector(|| "wrapper".into())
                            .child(MeasuredLeaf(size(px(width), px(height)))),
                    )
                    .into_any_element()
            })
        };
        assert_frames_match_uncached(
            cx,
            vec![
                measured(10., 10.),
                measured(30., 20.),
                measured(30., 20.),
                measured(5., 40.),
            ],
        );
    }

    #[crate::test]
    fn rem_size_changes_resolve_lengths_again(cx: &mut TestAppContext) {
        let with_rem_size = |rem_size: f32| {
            frame(move |window| {
                window.set_rem_size(px(rem_size));
                div()
                    .flex()
                    .child(
                        div()
                            .w(rems(2.5))
                            .h(rems(1.25))
                            .debug_selector(|| "sized".into()),
                    )
                    .into_any_element()
            })
        };
        assert_frames_match_uncached(
            cx,
            vec![with_rem_size(16.), with_rem_size(13.), with_rem_size(13.)],
        );
    }

    #[crate::test]
    fn element_ids_reused_by_different_elements(cx: &mut TestAppContext) {
        // One ID names a measured leaf, then nothing, then a container whose children
        // have no IDs, then a measured leaf again.
        let leaf = || {
            frame(|_| {
                div()
                    .flex()
                    .child(
                        div()
                            .id("reused")
                            .debug_selector(|| "reused".into())
                            .child(MeasuredLeaf(size(px(12.), px(9.)))),
                    )
                    .into_any_element()
            })
        };
        assert_frames_match_uncached(
            cx,
            vec![
                leaf(),
                frame(|_| {
                    div()
                        .flex()
                        .child(div().size(px(3.)).debug_selector(|| "other".into()))
                        .into_any_element()
                }),
                frame(|_| {
                    div()
                        .flex()
                        .child(
                            div()
                                .id("reused")
                                .flex_col()
                                .p(px(4.))
                                .debug_selector(|| "reused".into())
                                .child(div().size(px(20.)))
                                .child(div().size(px(8.)).debug_selector(|| "inner".into())),
                        )
                        .into_any_element()
                }),
                leaf(),
            ],
        );
    }

    #[crate::test]
    fn text_is_measured_again_only_when_it_changes(cx: &mut TestAppContext) {
        let text = |width: f32, text: &'static str, font_size: f32| {
            frame(move |_| {
                div()
                    .flex()
                    .child(
                        div()
                            .w(px(width))
                            .text_size(px(font_size))
                            .debug_selector(|| "wrapper".into())
                            .child(text),
                    )
                    .child(div().size(px(10.)).debug_selector(|| "sibling".into()))
                    .into_any_element()
            })
        };
        let short = "Some text";
        let same_length = "Same text";
        let long = "Some longer text that wraps in narrow parents";
        // Opening the window draws the first frame, so drawing it again measures nothing.
        let frames = [
            (text(200., short, 14.), false),
            (text(200., short, 14.), false),
            (text(200., same_length, 14.), true),
            (text(200., long, 14.), true),
            (text(200., long, 14.), false),
            (text(60., long, 14.), true),
            (text(60., long, 18.), true),
            (text(60., long, 18.), false),
            (text(200., long, 14.), true),
        ];
        let expected = frames
            .iter()
            .map(|(_, measured)| *measured)
            .collect::<Vec<_>>();
        let measurements =
            assert_frames_match_uncached(cx, frames.into_iter().map(|(frame, _)| frame).collect());
        let measured = measurements
            .iter()
            .map(|count| *count > 0)
            .collect::<Vec<_>>();
        assert_eq!(
            measured, expected,
            "text measurements per frame: {measurements:?}"
        );
    }

    #[crate::test]
    fn evicted_nodes_do_not_accumulate_when_ids_change_every_frame(cx: &mut TestAppContext) {
        let window = cx.open_window(size(px(800.), px(600.)), |window, cx| {
            window.set_layout_cache_enabled(true);
            RandomizedElementTree::new_with_config(RandomizedElementTreeConfig::new(3, 64), cx)
        });
        let root = window.root(cx).expect("window should have a root");
        let mut node_counts = Vec::new();
        for _ in 0..20 {
            root.update(cx, |tree, cx| tree.churn_element_ids(cx));
            let node_count = cx
                .update_window(window.into(), |_, window, cx| {
                    window.draw(cx).clear(cx);
                    window.layout_node_count()
                })
                .expect("window should remain open");
            node_counts.push(node_count);
        }
        // Each frame's evicted nodes are kept for the next frame to reuse, and every node
        // a frame doesn't reuse is removed, so the tree stays the size of one frame.
        assert!(
            node_counts.windows(2).all(|pair| pair[0] == pair[1]),
            "{node_counts:?}"
        );
    }

    #[test]
    fn border_widths_to_taffy_use_stroke_snapping() {
        let border_widths = Edges {
            top: Pixels(0.0).into(),
            right: Pixels(0.4).into(),
            bottom: Pixels(0.5).into(),
            left: Pixels(1.6).into(),
        };
        let taffy_border = border_widths_to_taffy(&border_widths, Pixels(16.0), 1.0);

        assert_eq!(
            taffy_border.top,
            taffy::style::LengthPercentage::length(0.0)
        );
        assert_eq!(
            taffy_border.right,
            taffy::style::LengthPercentage::length(1.0)
        );
        assert_eq!(
            taffy_border.bottom,
            taffy::style::LengthPercentage::length(1.0)
        );
        assert_eq!(
            taffy_border.left,
            taffy::style::LengthPercentage::length(2.0)
        );
    }
}
