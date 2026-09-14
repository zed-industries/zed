//! Taffy-backed layout evaluation.
//!
//! The engine owns the `taffy` tree and translates between [`LayoutId`] and
//! `taffy::NodeId`. It receives the engine's layout style and converts it here,
//! so the facade never names `taffy`. Custom measure callbacks are invoked with
//! a type-erased [`MeasureContext`], so the engine never names the facade's
//! window or application types.

use collections::{FxHashMap, FxHashSet};
use gpui_engine::{BoxedMeasureFn, EngineLayoutStyle, LayoutEngine, LayoutId, MeasureContext};
use gpui_types::{
    AvailableSpace, Bounds, Pixels, Point, Size, ceil_to_device_pixel, round_half_toward_zero,
    round_to_device_pixel, size,
};
use taffy::{TaffyTree, TraversePartialTree as _, tree::NodeId};

#[cfg(feature = "stacker")]
type StackSafe<T> = stacksafe::StackSafe<T>;
#[cfg(not(feature = "stacker"))]
type StackSafe<T> = T;

type NodeMeasureFn = StackSafe<BoxedMeasureFn>;

struct NodeContext {
    measure: NodeMeasureFn,
}

/// The `taffy`-backed layout tree for a window.
pub struct TaffyLayoutEngine {
    taffy: TaffyTree<NodeContext>,
    absolute_layout_bounds: FxHashMap<LayoutId, Bounds<Pixels>>,
    /// Unrounded absolute border-box top-left per-node coordinate in device pixels.
    absolute_outer_origins: FxHashMap<LayoutId, Point<f32>>,
    computed_layouts: FxHashSet<LayoutId>,
    layout_bounds_scratch_space: Vec<LayoutId>,
}

const EXPECT_MESSAGE: &str = "we should avoid taffy layout errors by construction if possible";

/// Creates the layout engine the default engine hands to each new window.
pub fn default_layout_engine() -> Box<dyn LayoutEngine> {
    Box::new(TaffyLayoutEngine::new())
}

fn taffy_id(id: LayoutId) -> NodeId {
    NodeId::new(id.0)
}

fn layout_id(node_id: NodeId) -> LayoutId {
    LayoutId(u64::from(node_id))
}

impl TaffyLayoutEngine {
    /// Creates an empty layout engine with rounding disabled.
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();
        taffy.disable_rounding();
        TaffyLayoutEngine {
            taffy,
            absolute_layout_bounds: FxHashMap::default(),
            absolute_outer_origins: FxHashMap::default(),
            computed_layouts: FxHashSet::default(),
            layout_bounds_scratch_space: Vec::new(),
        }
    }

    /// Discards every node and cached bounds, ready for a fresh frame.
    pub fn clear(&mut self) {
        self.taffy.clear();
        self.absolute_layout_bounds.clear();
        self.absolute_outer_origins.clear();
        self.computed_layouts.clear();
    }

    /// Adds a leaf or container node built from an already-converted taffy style.
    fn request_layout_taffy(
        &mut self,
        taffy_style: taffy::style::Style,
        children: &[LayoutId],
    ) -> LayoutId {
        if children.is_empty() {
            let node = self.taffy.new_leaf(taffy_style).expect(EXPECT_MESSAGE);
            layout_id(node)
        } else {
            let child_ids: Vec<NodeId> = children.iter().copied().map(taffy_id).collect();
            let node = self
                .taffy
                .new_with_children(taffy_style, &child_ids)
                .expect(EXPECT_MESSAGE);
            layout_id(node)
        }
    }

    /// Adds a leaf whose size is resolved by `measure` during layout.
    fn request_measured_layout_taffy(
        &mut self,
        taffy_style: taffy::style::Style,
        measure: BoxedMeasureFn,
    ) -> LayoutId {
        #[cfg(feature = "stacker")]
        let measure = StackSafe::new(measure);

        let node = self
            .taffy
            .new_leaf_with_context(taffy_style, NodeContext { measure })
            .expect(EXPECT_MESSAGE);
        layout_id(node)
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
        let style = self.taffy.style(taffy_id(id)).expect(EXPECT_MESSAGE);
        let stretch_width = style.size.width.is_auto();
        let stretch_height = style.size.height.is_auto();
        if !stretch_width && !stretch_height {
            return;
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
        self.taffy
            .set_style(taffy_id(id), style)
            .expect(EXPECT_MESSAGE);
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn count_all_children(&self, parent: LayoutId) -> anyhow::Result<u32> {
        let mut count = 0;

        for child in self.taffy.children(taffy_id(parent))? {
            // Count this child.
            count += 1;

            // Count all of this child's children.
            count += self.count_all_children(layout_id(child))?
        }

        Ok(count)
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn max_depth(&self, depth: u32, parent: LayoutId) -> anyhow::Result<u32> {
        println!(
            "{parent:?} at depth {depth} has {} children",
            self.taffy.child_count(taffy_id(parent))
        );

        let mut max_child_depth = 0;

        for child in self.taffy.children(taffy_id(parent))? {
            max_child_depth = std::cmp::max(max_child_depth, self.max_depth(0, layout_id(child))?);
        }

        Ok(depth + 1 + max_child_depth)
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn get_edges(&self, parent: LayoutId) -> anyhow::Result<Vec<(LayoutId, LayoutId)>> {
        let mut edges = Vec::new();

        for child in self.taffy.children(taffy_id(parent))? {
            edges.push((parent, layout_id(child)));

            edges.extend(self.get_edges(layout_id(child))?);
        }

        Ok(edges)
    }

    /// Computes the layout of `id` within `available_space`, invoking stored
    /// measure callbacks with `context`.
    #[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
    pub fn compute_layout(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        scale_factor: f32,
        context: &mut dyn MeasureContext,
    ) {
        if !self.computed_layouts.insert(id) {
            let stack = &mut self.layout_bounds_scratch_space;
            stack.push(id);
            while let Some(id) = stack.pop() {
                self.absolute_layout_bounds.remove(&id);
                self.absolute_outer_origins.remove(&id);
                stack.extend(
                    self.taffy
                        .children(taffy_id(id))
                        .expect(EXPECT_MESSAGE)
                        .into_iter()
                        .map(layout_id),
                );
            }
        }

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
                taffy_id(id),
                available_space.into(),
                |known_dimensions, available_space, _id, node_context, _style| {
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

                    let measured_size: Size<Pixels> =
                        (node_context.measure)(known_dimensions, available_space, context);
                    snap_measured_size_to_device_pixels(measured_size, scale_factor).into()
                },
            )
            .expect(EXPECT_MESSAGE);
    }

    /// Returns the bounds of `id` relative to the window, pixel-snapped.
    ///
    /// Pixel snapping
    ///
    /// Painting primitives at non-integer pixel coordinates produces blurry
    /// output. Pixel snapping converts layout coordinates into integer
    /// device-pixel coordinates so painted edges land exactly on physical
    /// pixel boundaries.
    ///
    /// Non-integer coordinates can arise for several reasons, including:
    ///   - flex distribution, percentages, centering, and text measurement
    ///     can produce fractional element sizes and positions;
    ///   - at fractional scale factors (for example 125% or 150%), integer
    ///     logical-pixel values can map to non-integer device-pixel values.
    ///
    /// We pixel-snap by rounding in device-pixel space, after multiplying
    /// by `scale_factor`, so that snapping targets physical pixels. Bounds
    /// are divided by `scale_factor` before being returned to GPUI.
    ///
    /// Midpoints are rounded toward zero. This is a stylistic choice: a
    /// 1-logical-pixel line at 150% scale should render as 1 dp rather than
    /// 2 dp.
    ///
    /// Pixel snapping is done in two phases:
    ///
    ///  1. Pre-layout metric snapping. Before Taffy computes layout, all
    ///     authored absolute lengths are rounded in `to_taffy`. This
    ///     includes borders, padding, gaps, and explicit sizes.
    ///     Custom-measured leaf nodes have their measured sizes rounded up
    ///     to integer device-pixel lengths.
    ///
    ///  2. Post-layout edge snapping. After Taffy resolves the tree, layout
    ///     relationships such as flex shares, grid tracks, percentages, and
    ///     centering can produce new fractional edge positions. Boxes now
    ///     have edges in absolute coordinates, and snapping must decide
    ///     where those edges land on the device-pixel grid.
    ///
    /// Ideally, post-layout snapping would satisfy:
    ///
    ///  - Edge closure. Two raw layout edges at the same absolute position
    ///    should snap to the same pixel column.
    ///  - Translation stability. A component's internal geometry should not
    ///    change when it moves to a new absolute position.
    ///
    /// These goals are in tension because rounding is not associative.
    /// The simple local schemes make different tradeoffs:
    ///
    ///  - Absolute edge rounding gives each window coordinate one answer,
    ///    so coincident edges always close globally. But a span's snapped
    ///    length is `round(far) - round(near)`, which may change by 1 dp
    ///    as its absolute origin moves.
    ///
    ///  - Parent-relative edge rounding rounds each child inside its
    ///    parent's coordinate space. This guarantees translation stability,
    ///    but a shared edge reached through different parents can
    ///    accumulate different rounding, causing non-closure between
    ///    cousins.
    ///
    ///  - Length rounding rounds each width, height, and thickness
    ///    independently and then places boxes from those rounded lengths.
    ///    Sizes stay stable under translation, but neighboring boxes derive
    ///    their shared boundary from different sources, so closure is not
    ///    guaranteed.
    ///
    /// We apply absolute edge rounding for each element's outer box in
    /// post-layout rounding to preserve closure. Border and padding widths
    /// are not touched by post-layout rounding; they keep their pre-layout
    /// rounded value so that they remain stable under translation.
    ///
    /// This gives both closure and translation stability in the case that
    /// all local metrics are integer device-pixel lengths. Pre-layout
    /// rounding covers that in most cases. The exception is metrics
    /// resolved by layout relationships, such as percentages. Outer box
    /// edges will still close globally, and painted border widths are still
    /// snapped independently, but the raw content-box origin can carry a
    /// 1dp residual into descendants.
    pub fn layout_bounds(&mut self, id: LayoutId, scale_factor: f32) -> Bounds<Pixels> {
        if let Some(layout) = self.absolute_layout_bounds.get(&id).cloned() {
            return layout;
        }

        let layout = self.taffy.layout(taffy_id(id)).expect(EXPECT_MESSAGE);
        let layout_location = layout.location;
        let layout_size = layout.size;
        let parent = self.taffy.parent(taffy_id(id));

        let absolute_outer_origin = match parent {
            Some(parent_id) => {
                let parent_id = layout_id(parent_id);
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

impl LayoutEngine for TaffyLayoutEngine {
    fn clear(&mut self) {
        TaffyLayoutEngine::clear(self)
    }

    fn request_layout(
        &mut self,
        style: &EngineLayoutStyle,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
    ) -> LayoutId {
        let taffy_style = crate::layout_style::to_taffy_style(style, rem_size, scale_factor);
        TaffyLayoutEngine::request_layout_taffy(self, taffy_style, children)
    }

    fn request_measured_layout(
        &mut self,
        style: &EngineLayoutStyle,
        rem_size: Pixels,
        scale_factor: f32,
        measure: BoxedMeasureFn,
    ) -> LayoutId {
        let taffy_style = crate::layout_style::to_taffy_style(style, rem_size, scale_factor);
        TaffyLayoutEngine::request_measured_layout_taffy(self, taffy_style, measure)
    }

    fn stretch_auto_size_to_fill(&mut self, id: LayoutId, size: Size<Pixels>, scale_factor: f32) {
        TaffyLayoutEngine::stretch_auto_size_to_fill(self, id, size, scale_factor)
    }

    fn compute_layout(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        scale_factor: f32,
        context: &mut dyn MeasureContext,
    ) {
        TaffyLayoutEngine::compute_layout(self, id, available_space, scale_factor, context)
    }

    fn layout_bounds(&mut self, id: LayoutId, scale_factor: f32) -> Bounds<Pixels> {
        TaffyLayoutEngine::layout_bounds(self, id, scale_factor)
    }
}

fn snap_measured_size_to_device_pixels(size: Size<Pixels>, scale_factor: f32) -> Size<f32> {
    size.map(|d| ceil_to_device_pixel(d.0.max(0.0), scale_factor))
}
