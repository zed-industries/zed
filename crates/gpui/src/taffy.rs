use crate::{
    AbsoluteLength, App, Bounds, DefiniteLength, Edges, GridTemplate, Length, Pixels, Point, Size,
    Style, Window, size,
    util::{
        ceil_to_device_pixel, round_half_toward_zero, round_stroke_to_device_pixel,
        round_to_device_pixel,
    },
};
use collections::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use std::{fmt::Debug, ops::Range};
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
    Custom(NodeMeasureFn),
    Text(Box<crate::elements::TextMeasure>),
}

#[derive(Default)]
struct LayoutCache {
    scopes: FxHashMap<u64, SmallVec<[LayoutScope; 1]>>,
    request_scope: Option<(u64, usize)>,
    font_generation: Option<usize>,
}

struct LayoutScope {
    id: crate::GlobalElementId,
    nodes: Vec<Option<LayoutId>>,
    cursor: usize,
}

pub struct TaffyLayoutEngine {
    taffy: TaffyTree<NodeContext>,
    cache: Option<Box<LayoutCache>>,
    absolute_layout_bounds: FxHashMap<LayoutId, Bounds<Pixels>>,
    /// Unrounded absolute border-box top-left per-node coordinate in device pixels.
    absolute_outer_origins: FxHashMap<LayoutId, Point<f32>>,
    computed_layouts: FxHashSet<LayoutId>,
    layout_bounds_scratch_space: Vec<LayoutId>,
}

const EXPECT_MESSAGE: &str = "we should avoid taffy layout errors by construction if possible";

fn text_layout_style() -> taffy::Style {
    taffy::Style {
        display: taffy::Display::Block,
        ..Default::default()
    }
}

impl TaffyLayoutEngine {
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();
        taffy.disable_rounding();
        TaffyLayoutEngine {
            taffy,
            cache: None,
            absolute_layout_bounds: FxHashMap::default(),
            absolute_outer_origins: FxHashMap::default(),
            computed_layouts: FxHashSet::default(),
            layout_bounds_scratch_space: Vec::new(),
        }
    }

    pub(crate) fn enable_cache(&mut self) {
        self.cache = Some(Box::default());
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn cache_enabled(&self) -> bool {
        self.cache.is_some()
    }

    #[cfg(test)]
    pub(crate) fn text_context_identity_and_dirty(
        &self,
        id: LayoutId,
    ) -> (Option<*const crate::elements::TextMeasure>, bool) {
        let identity = match self.taffy.get_node_context(id.0) {
            Some(NodeContext::Text(measure)) => Some(&**measure as *const _),
            _ => None,
        };
        (identity, self.taffy.dirty(id.0).expect(EXPECT_MESSAGE))
    }

    pub(crate) fn set_request_scope(&mut self, stack: &crate::window::ElementIdStack) {
        let Some(cache) = self.cache.as_mut() else {
            return;
        };
        // Hashes select a bucket, never an identity. Anonymous requests consume
        // ordinals in their nearest identified execution scope.
        let scopes = cache.scopes.entry(stack.hash()).or_default();
        let index = scopes
            .iter()
            .position(|scope| &*scope.id == &**stack)
            .unwrap_or_else(|| {
                scopes.push(LayoutScope {
                    id: stack.global_id(),
                    nodes: Vec::new(),
                    cursor: 0,
                });
                scopes.len() - 1
            });
        cache.request_scope = Some((stack.hash(), index));
    }

    pub fn clear(&mut self) {
        if let Some(cache) = self.cache.as_mut() {
            let reachable = &mut self.computed_layouts;
            let stack = &mut self.layout_bounds_scratch_space;
            stack.extend(reachable.iter().copied());
            while let Some(id) = stack.pop() {
                for child in self.taffy.child_ids(id.0).map(LayoutId) {
                    if reachable.insert(child) {
                        stack.push(child);
                    }
                }
            }
            cache.scopes.retain(|_, scopes| {
                scopes.retain_mut(|scope| {
                    for (index, node) in scope.nodes.iter_mut().enumerate() {
                        if let Some(id) = *node {
                            if index >= scope.cursor || !reachable.contains(&id) {
                                // Taffy 0.14's remove does not remove node context data.
                                self.taffy
                                    .set_node_context(id.0, None)
                                    .expect(EXPECT_MESSAGE);
                                self.taffy.remove(id.0).expect(EXPECT_MESSAGE);
                                *node = None;
                            } else if matches!(
                                self.taffy.get_node_context(id.0),
                                Some(NodeContext::Custom(_))
                            ) {
                                self.taffy
                                    .set_node_context(id.0, None)
                                    .expect(EXPECT_MESSAGE);
                            }
                        }
                    }
                    scope.nodes.truncate(scope.cursor);
                    scope.cursor = 0;
                    scope.nodes.iter().any(Option::is_some)
                });
                !scopes.is_empty()
            });
            cache.request_scope = None;
        } else {
            self.taffy.clear();
        }
        self.absolute_layout_bounds.clear();
        self.absolute_outer_origins.clear();
        self.computed_layouts.clear();
    }

    fn claim_node(&mut self, style: taffy::Style, children: &[LayoutId]) -> LayoutId {
        let scope = self.cache.as_mut().map(|cache| {
            let (hash, index) = cache
                .request_scope
                .take()
                .expect("cached layout requests require an execution scope");
            let scope = &mut cache.scopes.get_mut(&hash).expect(EXPECT_MESSAGE)[index];
            let ordinal = scope.cursor;
            scope.cursor += 1;
            if ordinal == scope.nodes.len() {
                scope.nodes.push(None);
            }
            &mut scope.nodes[ordinal]
        });
        let existing = scope.as_deref().copied().flatten();
        let id = if let Some(id) = existing {
            if self.taffy.style(id.0).expect(EXPECT_MESSAGE) != &style {
                self.taffy.set_style(id.0, style).expect(EXPECT_MESSAGE);
            }
            if !self
                .taffy
                .child_ids(id.0)
                .eq(children.iter().map(|id| id.0))
            {
                self.taffy
                    .set_children(id.0, LayoutId::to_taffy_slice(children))
                    .expect(EXPECT_MESSAGE);
            }
            id
        } else {
            let id = self.taffy.new_leaf(style).expect(EXPECT_MESSAGE);
            if !children.is_empty() {
                // Unlike new_with_children, set_children detaches previous parents.
                self.taffy
                    .set_children(id, LayoutId::to_taffy_slice(children))
                    .expect(EXPECT_MESSAGE);
            }
            LayoutId(id)
        };
        if let Some(scope) = scope {
            *scope = Some(id);
        }
        id
    }

    fn request_node(
        &mut self,
        style: taffy::Style,
        children: &[LayoutId],
        context: Option<NodeContext>,
    ) -> LayoutId {
        let id = self.claim_node(style, children);
        if context.is_some() || self.taffy.get_node_context(id.0).is_some() {
            // Opaque measurements can observe arbitrary external state.
            // Replacing the callback must invalidate every cached constraint.
            self.taffy
                .set_node_context(id.0, context)
                .expect(EXPECT_MESSAGE);
        }
        id
    }

    pub fn request_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
    ) -> LayoutId {
        let taffy_style = style.to_taffy(rem_size, scale_factor);

        if self.cache.is_some() {
            return self.request_node(taffy_style, children, None);
        }
        if children.is_empty() {
            self.taffy
                .new_leaf(taffy_style)
                .expect(EXPECT_MESSAGE)
                .into()
        } else {
            self.taffy
                // This is safe because LayoutId is repr(transparent) to taffy::tree::NodeId.
                .new_with_children(taffy_style, LayoutId::to_taffy_slice(children))
                .expect(EXPECT_MESSAGE)
                .into()
        }
    }

    pub fn request_measured_layout(
        &mut self,
        style: Style,
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
        let taffy_style = style.to_taffy(rem_size, scale_factor);
        let measure = Box::new(measure) as Box<MeasureFn>;
        #[cfg(feature = "stacker")]
        let measure = StackSafe::new(measure);

        if self.cache.is_some() {
            return self.request_node(taffy_style, &[], Some(NodeContext::Custom(measure)));
        }
        self.taffy
            .new_leaf_with_context(taffy_style, NodeContext::Custom(measure))
            .expect(EXPECT_MESSAGE)
            .into()
    }

    pub(crate) fn request_text_layout(
        &mut self,
        measure: crate::elements::TextMeasure,
    ) -> LayoutId {
        let style = text_layout_style();
        if self.cache.is_some() {
            let id = self.claim_node(style, &[]);
            if let Some(NodeContext::Text(previous)) = self.taffy.get_node_context_mut(id.0) {
                let carried = measure.carry_from(previous);
                if !carried {
                    measure.reset();
                }
                // Keep the authoritative allocation, not the old descriptor.
                // In-place replacement bypasses the setter's invalidation.
                **previous = measure;
                if !carried {
                    self.taffy.mark_dirty(id.0).expect(EXPECT_MESSAGE);
                }
            } else {
                measure.reset();
                self.taffy
                    .set_node_context(id.0, Some(NodeContext::Text(Box::new(measure))))
                    .expect(EXPECT_MESSAGE);
            }
            id
        } else {
            self.taffy
                .new_leaf_with_context(style, NodeContext::Text(Box::new(measure)))
                .expect(EXPECT_MESSAGE)
                .into()
        }
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
        if let Some(cache) = self.cache.as_mut() {
            let generation = window.text_system().font_generation();
            if cache.font_generation != Some(generation) {
                // Fonts may be installed after request_layout, before this root
                // is computed. Invalidating only at descriptor creation is too early.
                for scope in cache.scopes.values().flatten() {
                    for id in scope.nodes.iter().flatten() {
                        if let Some(NodeContext::Text(measure)) =
                            self.taffy.get_node_context_mut(id.0)
                            && measure.refresh_fonts(generation)
                        {
                            self.taffy.mark_dirty(id.0).expect(EXPECT_MESSAGE);
                        }
                    }
                }
                cache.font_generation = Some(generation);
            }
        }
        if let Some(cache) = self.cache.as_ref()
            && let Some(parent) = self.taffy.parent(id.0)
            && !cache
                .scopes
                .values()
                .flatten()
                .any(|scope| scope.nodes[..scope.cursor].contains(&Some(LayoutId(parent))))
        {
            // A root may still have a parent from the previous frame. Preserve
            // current-frame parentage when a custom element computes a subtree.
            self.taffy.remove_child(parent, id.0).expect(EXPECT_MESSAGE);
        }
        // Leaving this here until we have a better instrumentation approach.
        // println!("Laying out {} children", self.count_all_children(id)?);
        // println!("Max layout depth: {}", self.max_depth(0, id)?);

        // Output the edges (branches) of the tree in Mermaid format for visualization.
        // println!("Edges:");
        // for (a, b) in self.get_edges(id)? {
        //     println!("N{} --> N{}", u64::from(a), u64::from(b));
        // }
        //

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

        let cache_enabled = self.cache.is_some();
        self.taffy
            .compute_layout_with_measure(
                id.into(),
                available_space.into(),
                |inputs, _id, node_context, style| {
                    taffy::compute_leaf_layout(
                        inputs,
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
                                NodeContext::Custom(measure) => {
                                    measure(known_dimensions, available_space, window, cx)
                                }
                                NodeContext::Text(measure) => measure.measure(
                                    known_dimensions,
                                    available_space,
                                    cache_enabled.then_some(inputs.run_mode),
                                    window,
                                    cx,
                                ),
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

impl ToTaffy<taffy::style::Style> for Style {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Style {
        use taffy::style_helpers::{fr, length, minmax, repeat};

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
            align_items: self.align_items.map(|x| x.into()),
            align_self: self.align_self.map(|x| x.into()),
            align_content: self.align_content.map(|x| x.into()),
            justify_content: self.justify_content.map(|x| x.into()),
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

    #[test]
    fn text_layout_style_matches_default_conversion() {
        for rem_size in [Pixels(12.0), Pixels(16.0), Pixels(24.0)] {
            for scale_factor in [1.0, 1.25, 1.5, 2.0] {
                assert_eq!(
                    text_layout_style(),
                    Style::default().to_taffy(rem_size, scale_factor),
                    "rem size {rem_size:?}, scale {scale_factor}"
                );
            }
        }
    }

    fn scoped_request(
        engine: &mut TaffyLayoutEngine,
        scope: u64,
        style: Style,
        scale: f32,
        children: &[LayoutId],
    ) -> LayoutId {
        let mut stack = crate::window::ElementIdStack::default();
        stack.push(scope.into());
        engine.set_request_scope(&stack);
        engine.request_layout(style, Pixels(16.0), scale, children)
    }

    #[gpui::test]
    fn layout_cache_restores_root_style_when_reparented(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            window.set_scale_factor(1.0);
            let mut engine = TaffyLayoutEngine::new();
            engine.enable_cache();
            let root = scoped_request(&mut engine, 1, Style::default(), 1.0, &[]);
            let viewport = size(Pixels(200.0), Pixels(100.0));
            engine.stretch_auto_size_to_fill(root, viewport, 1.0);
            engine.compute_layout(root, viewport.into(), window, cx);
            assert_eq!(engine.layout_bounds(root, 1.0).size, viewport);
            engine.clear();

            let child = scoped_request(&mut engine, 1, Style::default(), 1.0, &[]);
            assert_eq!(child, root);
            let root = scoped_request(&mut engine, 2, Style::default(), 1.0, &[child]);
            engine.compute_layout(root, AvailableSpace::min_size(), window, cx);
            assert_eq!(engine.layout_bounds(child, 1.0).size, Size::default());
            engine.clear();
        });
    }

    #[test]
    fn layout_cache_compares_full_scope_identity_in_hash_buckets() {
        let mut engine = TaffyLayoutEngine::new();
        engine.enable_cache();
        let mut stack = crate::window::ElementIdStack::default();
        stack.push(1_u64.into());
        let hash = stack.hash();
        let cache = engine.cache.as_mut().expect("cache enabled");
        // Inject a distinct identity into the target bucket to simulate a collision.
        cache.scopes.entry(hash).or_default().push(LayoutScope {
            id: crate::GlobalElementId::new(&[2_u64.into()]),
            nodes: Vec::new(),
            cursor: 0,
        });
        engine.set_request_scope(&stack);
        let cache = engine.cache.as_ref().expect("cache enabled");
        assert_eq!(cache.request_scope, Some((hash, 1)));
        assert_eq!(cache.scopes[&hash].len(), 2);
    }

    #[gpui::test(iterations = 10)]
    fn layout_cache_matches_fresh_geometry(
        mut rng: rand::rngs::StdRng,
        cx: &mut crate::TestAppContext,
    ) {
        use rand::{Rng as _, seq::SliceRandom as _};

        cx.add_empty_window().update(|window, cx| {
            let mut fresh = TaffyLayoutEngine::new();
            let mut cached = TaffyLayoutEngine::new();
            cached.enable_cache();
            for frame in 0..40 {
                let scale = [1.0, 1.25, 2.0][frame % 3];
                window.set_scale_factor(scale);
                let mut scopes: Vec<u64> = (10..26).collect();
                scopes.shuffle(&mut rng);
                scopes.truncate(rng.random_range(0..=16));
                let styles: Vec<_> = scopes
                    .iter()
                    .map(|_| {
                        let mut style = Style::default();
                        style.size = size(
                            Pixels(rng.random_range(5.0..80.0)).into(),
                            Pixels(rng.random_range(5.0..30.0)).into(),
                        );
                        style.padding = Edges::all(Pixels(1.3).into());
                        style
                    })
                    .collect();
                let split = rng.random_range(0..=scopes.len());
                let viewport = size(Pixels(400.0 + frame as f32), Pixels(300.0));
                let mut results = Vec::new();
                for engine in [&mut fresh, &mut cached] {
                    let leaves: Vec<_> = scopes
                        .iter()
                        .zip(&styles)
                        .map(|(scope, style)| {
                            scoped_request(engine, *scope, style.clone(), scale, &[])
                        })
                        .collect();
                    let left = scoped_request(engine, 1, Style::default(), scale, &leaves[..split]);
                    let right =
                        scoped_request(engine, 2, Style::default(), scale, &leaves[split..]);
                    let root = scoped_request(engine, 0, Style::default(), scale, &[left, right]);
                    engine.stretch_auto_size_to_fill(root, viewport, scale);
                    engine.compute_layout(root, viewport.into(), window, cx);
                    results.push(
                        leaves
                            .into_iter()
                            .chain([left, right, root])
                            .map(|id| engine.layout_bounds(id, scale))
                            .collect::<Vec<_>>(),
                    );
                    engine.clear();
                }
                assert_eq!(results[0], results[1], "frame {frame}");
                assert_eq!(cached.taffy.total_node_count(), scopes.len() + 3);
            }
            cached.clear();
            assert_eq!(cached.taffy.total_node_count(), 0);
        });
    }

    #[gpui::test]
    fn layout_cache_reuses_ordinals_and_drops_measurements(cx: &mut crate::TestAppContext) {
        use std::{cell::Cell, rc::Rc};

        cx.add_empty_window().update(|window, cx| {
            window.set_scale_factor(1.0);
            let mut engine = TaffyLayoutEngine::new();
            engine.enable_cache();
            let scope = crate::window::ElementIdStack::default();
            let calls = Rc::new(Cell::new(0));
            let mut previous = None;
            for frame in 0..4 {
                let capture = Rc::new(());
                let weak_capture = Rc::downgrade(&capture);
                engine.set_request_scope(&scope);
                let measured =
                    engine.request_measured_layout(Style::default(), Pixels(16.0), 1.0, {
                        let calls = calls.clone();
                        move |_, _, _, _| {
                            let _capture = &capture;
                            calls.set(calls.get() + 1);
                            size(Pixels(20.0 + frame as f32), Pixels(10.0))
                        }
                    });
                engine.set_request_scope(&scope);
                let root = engine.request_layout(Style::default(), Pixels(16.0), 1.0, &[measured]);
                if let Some(previous) = previous {
                    assert_eq!((measured, root), previous);
                }
                previous = Some((measured, root));
                let before = calls.get();
                engine.compute_layout(root, AvailableSpace::min_size(), window, cx);
                assert!(calls.get() > before);
                assert_eq!(
                    engine.layout_bounds(measured, 1.0).size,
                    size(Pixels(20.0 + frame as f32), Pixels(10.0))
                );
                engine.clear();
                assert!(weak_capture.upgrade().is_none());
            }
            engine.set_request_scope(&scope);
            let unmeasured = engine.request_layout(Style::default(), Pixels(16.0), 1.0, &[]);
            assert_eq!(unmeasured, previous.expect("previous frame").0);
            engine.compute_layout(unmeasured, AvailableSpace::min_size(), window, cx);
            assert_eq!(engine.layout_bounds(unmeasured, 1.0).size, Size::default());
            engine.clear();
            assert_eq!(engine.taffy.total_node_count(), 1);

            let abandoned_capture = Rc::new(());
            let weak_capture = Rc::downgrade(&abandoned_capture);
            engine.set_request_scope(&scope);
            engine.request_measured_layout(
                Style::default(),
                Pixels(16.0),
                1.0,
                move |_, _, _, _| {
                    let _capture = &abandoned_capture;
                    Size::default()
                },
            );
            engine.clear();
            assert!(weak_capture.upgrade().is_none());
            assert_eq!(engine.taffy.total_node_count(), 0);
        });
    }

    #[gpui::test]
    fn unmeasured_leaf_preserves_box_styles(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            window.set_scale_factor(2.0);
            let mut engine = TaffyLayoutEngine::new();
            for (min_width, max_width, aspect_ratio, expected_size) in [
                (
                    Length::Auto,
                    Length::Auto,
                    None,
                    size(Pixels(10.0), Pixels(10.0)),
                ),
                (
                    Pixels(40.0).into(),
                    Length::Auto,
                    Some(2.0),
                    size(Pixels(40.0), Pixels(20.0)),
                ),
                (
                    Length::Auto,
                    Pixels(6.0).into(),
                    None,
                    size(Pixels(10.0), Pixels(10.0)),
                ),
            ] {
                let mut style = Style::default();
                style.padding = Edges::all(Pixels(4.0).into());
                style.border_widths = Edges::all(Pixels(1.0).into());
                style.min_size.width = min_width;
                style.max_size.width = max_width;
                style.aspect_ratio = aspect_ratio;
                let id = engine.request_layout(style, Pixels(16.0), 2.0, &[]);
                engine.compute_layout(
                    id,
                    size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                    window,
                    cx,
                );
                assert_eq!(engine.layout_bounds(id, 2.0).size, expected_size);
            }
        });
    }

    #[gpui::test]
    fn measured_leaf_preserves_logical_units_and_ceil_snapping(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            window.set_scale_factor(2.0);
            let mut engine = TaffyLayoutEngine::new();
            for (width, expected_available_width, expected_width) in [
                (Length::Auto, AvailableSpace::MaxContent, Pixels(20.5)),
                (
                    Pixels(30.0).into(),
                    AvailableSpace::Definite(Pixels(20.0)),
                    Pixels(30.0),
                ),
            ] {
                let mut style = Style::default();
                style.size.width = width;
                style.padding = Edges::all(Pixels(4.0).into());
                style.border_widths = Edges::all(Pixels(1.0).into());
                let measured = std::rc::Rc::new(std::cell::Cell::new(false));
                let id = engine.request_measured_layout(style, Pixels(16.0), 2.0, {
                    let measured = measured.clone();
                    move |_, available_space, _, _| {
                        measured.set(true);
                        assert_eq!(available_space.width, expected_available_width);
                        assert_eq!(available_space.height, AvailableSpace::MaxContent);
                        size(Pixels(10.2), Pixels(20.2))
                    }
                });
                engine.compute_layout(
                    id,
                    size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                    window,
                    cx,
                );
                assert!(measured.get());
                assert_eq!(
                    engine.layout_bounds(id, 2.0).size,
                    size(expected_width, Pixels(30.5))
                );
            }
        });
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

    #[gpui::test]
    fn fractional_scale_root_dimensions(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            for (scale_factor, explicit_size, percentage_size) in [
                (1.25, size(26.0, 16.0), size(63.0, 42.0)),
                (1.5, size(31.0, 19.0), size(76.0, 50.0)),
            ] {
                window.set_scale_factor(scale_factor);
                for percentage in [false, true] {
                    let mut engine = TaffyLayoutEngine::new();
                    let mut style = Style::default();
                    style.size = if percentage {
                        size(
                            DefiniteLength::Fraction(0.5).into(),
                            DefiniteLength::Fraction(0.5).into(),
                        )
                    } else {
                        size(Pixels(21.0).into(), Pixels(13.0).into())
                    };
                    let root = engine.request_layout(style, Pixels(16.0), scale_factor, &[]);
                    let available_size = size(Pixels(101.0), Pixels(67.0));
                    engine.stretch_auto_size_to_fill(root, available_size, scale_factor);
                    engine.compute_layout(root, available_size.into(), window, cx);

                    // Author absolute sizes in already-snapped device pixels, independently
                    // of Style::to_taffy; percentages resolve against unsnapped available space.
                    let mut direct = TaffyTree::<()>::new();
                    direct.disable_rounding();
                    let direct_root = direct
                        .new_leaf(taffy::Style {
                            display: taffy::Display::Block,
                            size: if percentage {
                                TaffySize {
                                    width: taffy::Dimension::percent(0.5),
                                    height: taffy::Dimension::percent(0.5),
                                }
                            } else {
                                TaffySize {
                                    width: taffy::Dimension::length(explicit_size.width),
                                    height: taffy::Dimension::length(explicit_size.height),
                                }
                            },
                            ..Default::default()
                        })
                        .expect(EXPECT_MESSAGE);
                    direct
                        .compute_layout(
                            direct_root,
                            TaffySize {
                                width: TaffyAvailableSpace::Definite(101.0 * scale_factor),
                                height: TaffyAvailableSpace::Definite(67.0 * scale_factor),
                            },
                        )
                        .expect(EXPECT_MESSAGE);
                    let direct_layout = direct.layout(direct_root).expect(EXPECT_MESSAGE);
                    let adapter_layout = engine.taffy.layout(root.into()).expect(EXPECT_MESSAGE);
                    assert_eq!(adapter_layout.location, direct_layout.location);
                    assert_eq!(adapter_layout.size, direct_layout.size);

                    let expected_size = if percentage {
                        percentage_size
                    } else {
                        explicit_size
                    };
                    assert_eq!(
                        engine.layout_bounds(root, scale_factor),
                        Bounds::new(
                            Point::default(),
                            expected_size.map(|value| Pixels(value / scale_factor)),
                        ),
                        "scale={scale_factor}, percentage={percentage}",
                    );
                }
            }
        });
    }

    #[gpui::test]
    fn fractional_scale_nested_flex_edges_close(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            // 101 logical pixels snap to 126 or 151 device pixels before flex
            // distribution. Snap the resulting absolute quarter edges, not local widths.
            for (scale_factor, width, height, edges) in [
                (1.25, 126.0, 11.0, [0.0, 31.0, 63.0, 94.0, 126.0]),
                (1.5, 151.0, 13.0, [0.0, 38.0, 75.0, 113.0, 151.0]),
            ] {
                window.set_scale_factor(scale_factor);
                let mut engine = TaffyLayoutEngine::new();
                let flex_item = Style {
                    display: crate::Display::Flex,
                    flex_grow: 1.0,
                    flex_basis: Pixels(0.0).into(),
                    min_size: size(Pixels(0.0).into(), Pixels(0.0).into()),
                    ..Default::default()
                };
                let leaves: Vec<_> = (0..4)
                    .map(|_| {
                        engine.request_layout(flex_item.clone(), Pixels(16.0), scale_factor, &[])
                    })
                    .collect();
                let parents: Vec<_> = leaves
                    .chunks_exact(2)
                    .map(|children| {
                        engine.request_layout(
                            flex_item.clone(),
                            Pixels(16.0),
                            scale_factor,
                            children,
                        )
                    })
                    .collect();
                let root = engine.request_layout(
                    Style {
                        display: crate::Display::Flex,
                        size: size(Pixels(101.0).into(), Pixels(9.0).into()),
                        ..Default::default()
                    },
                    Pixels(16.0),
                    scale_factor,
                    &parents,
                );
                engine.compute_layout(
                    root,
                    size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                    window,
                    cx,
                );

                for (&leaf, edge_pair) in leaves.iter().zip(edges.windows(2)) {
                    let raw = engine.taffy.layout(leaf.into()).expect(EXPECT_MESSAGE);
                    assert_eq!(raw.size.width, width / 4.0);
                    assert_eq!(
                        engine.layout_bounds(leaf, scale_factor),
                        Bounds::new(
                            Point::new(Pixels(edge_pair[0] / scale_factor), Pixels(0.0)),
                            size(
                                Pixels((edge_pair[1] - edge_pair[0]) / scale_factor),
                                Pixels(height / scale_factor),
                            ),
                        ),
                        "scale={scale_factor}, edges={edge_pair:?}",
                    );
                }
                for (&parent, edge_pair) in parents.iter().zip(edges.windows(3).step_by(2)) {
                    assert_eq!(
                        engine.layout_bounds(parent, scale_factor),
                        Bounds::new(
                            Point::new(Pixels(edge_pair[0] / scale_factor), Pixels(0.0)),
                            size(
                                Pixels((edge_pair[2] - edge_pair[0]) / scale_factor),
                                Pixels(height / scale_factor),
                            ),
                        ),
                    );
                }
                assert_eq!(
                    engine.layout_bounds(root, scale_factor).size,
                    size(Pixels(width / scale_factor), Pixels(height / scale_factor)),
                );
            }
        });
    }

    #[gpui::test]
    fn fractional_scale_padding_and_overflow_origins(cx: &mut crate::TestAppContext) {
        cx.add_empty_window().update(|window, cx| {
            for (scale_factor, root_size, child_size, leaf_size) in [
                (1.25, size(26.0, 16.0), size(39.0, 21.0), 2.0),
                (1.5, size(31.0, 19.0), size(46.0, 25.0), 3.0),
            ] {
                window.set_scale_factor(scale_factor);
                for overflow in [crate::Overflow::Visible, crate::Overflow::Hidden] {
                    let mut engine = TaffyLayoutEngine::new();
                    let leaf = engine.request_layout(
                        Style {
                            size: size(Pixels(2.0).into(), Pixels(2.0).into()),
                            ..Default::default()
                        },
                        Pixels(16.0),
                        scale_factor,
                        &[],
                    );
                    let child = engine.request_layout(
                        Style {
                            size: size(Pixels(31.0).into(), Pixels(17.0).into()),
                            flex_shrink: 0.0,
                            padding: Edges::all(Pixels(1.0).into()),
                            ..Default::default()
                        },
                        Pixels(16.0),
                        scale_factor,
                        &[leaf],
                    );
                    let root = engine.request_layout(
                        Style {
                            display: crate::Display::Flex,
                            size: size(Pixels(21.0).into(), Pixels(13.0).into()),
                            padding: Edges {
                                top: Pixels(3.0).into(),
                                ..Edges::all(Pixels(1.0).into())
                            },
                            border_widths: Edges::all(Pixels(1.0).into()),
                            overflow: Point::new(overflow, overflow),
                            ..Default::default()
                        },
                        Pixels(16.0),
                        scale_factor,
                        &[child],
                    );
                    engine.compute_layout(
                        root,
                        size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                        window,
                        cx,
                    );

                    // At both scales, 1 logical pixel snaps to 1 device pixel,
                    // and the top padding of 3 snaps to 4. Overflow must not
                    // clip layout bounds or add padding again at each ancestor.
                    for (id, origin, expected_size) in [
                        (root, Point::new(0.0, 0.0), root_size),
                        (child, Point::new(2.0, 5.0), child_size),
                        (leaf, Point::new(3.0, 6.0), size(leaf_size, leaf_size)),
                    ] {
                        assert_eq!(
                            engine.layout_bounds(id, scale_factor),
                            Bounds::new(
                                origin.map(|value| Pixels(value / scale_factor)),
                                expected_size.map(|value| Pixels(value / scale_factor)),
                            ),
                            "scale={scale_factor}, overflow={overflow:?}, node={id:?}",
                        );
                    }
                }
            }
        });
    }
}
