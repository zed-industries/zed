use crate::{
    AbsoluteLength, App, Bounds, DefiniteLength, Edges, GridTemplate, Length, Pixels, Point, Size,
    Style, Window, size,
    util::{
        ceil_to_device_pixel, round_half_toward_zero, round_stroke_to_device_pixel,
        round_to_device_pixel,
    },
};
use collections::{FxHashMap, FxHashSet};
use std::{fmt::Debug, mem, ops::Range};
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

struct NodeContext {
    measure: NodeMeasureFn,
}
pub struct TaffyLayoutEngine {
    taffy: TaffyTree<NodeContext>,
    absolute_layout_bounds: FxHashMap<LayoutId, Bounds<Pixels>>,
    /// Unrounded absolute border-box top-left per-node coordinate in device pixels.
    absolute_outer_origins: FxHashMap<LayoutId, Point<f32>>,
    computed_layouts: FxHashSet<LayoutId>,
    layout_bounds_scratch_space: Vec<LayoutId>,
}

/// A layout tree of its own, for laying an element out while another tree computes.
///
/// Taffy computes one tree at a time. The measure closure of
/// [`Window::request_measured_layout`] runs inside that computation, so an
/// element that wants to measure a child there cannot use the window's tree.
/// This holds a second tree for the child, and [`IsolatedLayout::enter`] puts it
/// in front of the window's tree for the duration of a closure.
///
/// With that, an element can size itself from content that the window's tree
/// has not laid out yet. A panel that animates its height to the height of its
/// content is the case this exists for.
///
/// The layout ids made inside `enter` belong to this tree and address a
/// different node in any other tree. Every call that reads one has to run inside
/// `enter`, which means the child's `prepaint` as well as its layout, because
/// `prepaint` reads bounds. Two things break that rule: a child that calls
/// `Window::defer_draw`, which prepaints after `enter` returns, and a child that
/// keeps a layout id for a later frame.
pub struct IsolatedLayout(Option<TaffyLayoutEngine>);

impl Default for IsolatedLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl IsolatedLayout {
    /// An empty tree.
    pub fn new() -> Self {
        Self(Some(TaffyLayoutEngine::new()))
    }

    /// Run `f` with this tree in front of the window's tree.
    ///
    /// The window gets its own tree back when `f` returns, so calls that
    /// straddle the two stay separate.
    pub fn enter<R>(&mut self, window: &mut Window, f: impl FnOnce(&mut Window) -> R) -> R {
        let outer = mem::replace(&mut window.layout_engine, self.0.take());
        let result = f(window);
        self.0 = mem::replace(&mut window.layout_engine, outer);
        result
    }

    /// Drop everything this tree holds, keeping it usable for another frame.
    pub fn clear(&mut self) {
        if let Some(engine) = self.0.as_mut() {
            engine.clear();
        }
    }
}

const EXPECT_MESSAGE: &str = "we should avoid taffy layout errors by construction if possible";

impl TaffyLayoutEngine {
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

    pub fn clear(&mut self) {
        self.taffy.clear();
        self.absolute_layout_bounds.clear();
        self.absolute_outer_origins.clear();
        self.computed_layouts.clear();
    }

    pub fn request_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
    ) -> LayoutId {
        let taffy_style = style.to_taffy(rem_size, scale_factor);

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

        self.taffy
            .new_leaf_with_context(taffy_style, NodeContext { measure })
            .expect(EXPECT_MESSAGE)
            .into()
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

        self.taffy
            .compute_layout_with_measure(
                id.into(),
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
                        (node_context.measure)(known_dimensions, available_space, window, cx);
                    snap_measured_size_to_device_pixels(measured_size, scale_factor).into()
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

#[cfg(test)]
mod isolated_layout_tests {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use crate::{
        AnyElement, AnyWindowHandle, App, AppContext as _, AvailableSpace, Bounds, Context,
        Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, IsolatedLayout,
        LayoutId, ParentElement, Pixels, Render, Size, Style, Styled, TestAppContext, Window, div,
        px, size,
    };

    /// An element that takes its size from content it measures during layout.
    ///
    /// The content is laid out in a tree of its own, because the measure closure
    /// runs while the window's tree computes.
    struct MeasureDuringLayout {
        state: Rc<RefCell<(AnyElement, IsolatedLayout)>>,
        bounds: Rc<Cell<Bounds<Pixels>>>,
    }

    impl Element for MeasureDuringLayout {
        type RequestLayoutState = ();
        type PrepaintState = ();

        fn id(&self) -> Option<ElementId> {
            None
        }

        fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
            None
        }

        fn request_layout(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            window: &mut Window,
            _cx: &mut App,
        ) -> (LayoutId, ()) {
            let state = self.state.clone();
            let layout_id = window.request_measured_layout(
                Style::default(),
                move |known: Size<Option<Pixels>>, available: Size<AvailableSpace>, window, cx| {
                    let (content, layout) = &mut *state.borrow_mut();
                    let width = known
                        .width
                        .map_or(available.width, AvailableSpace::Definite);
                    layout.enter(window, |window| {
                        content.layout_as_root(size(width, AvailableSpace::MaxContent), window, cx)
                    })
                },
            );
            (layout_id, ())
        }

        fn prepaint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            bounds: Bounds<Pixels>,
            _request_layout: &mut (),
            _window: &mut Window,
            _cx: &mut App,
        ) {
            self.bounds.set(bounds);
        }

        fn paint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            _bounds: Bounds<Pixels>,
            _request_layout: &mut (),
            _prepaint: &mut (),
            _window: &mut Window,
            _cx: &mut App,
        ) {
        }
    }

    impl IntoElement for MeasureDuringLayout {
        type Element = Self;

        fn into_element(self) -> Self {
            self
        }
    }

    struct MeasureTestView {
        bounds: Rc<Cell<Bounds<Pixels>>>,
    }

    impl Render for MeasureTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            // Two children of 120 wrap into two rows of 30 at a width of 200, and
            // sit on one row of 30 at max content.
            let content = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .child(div().w(px(120.)).h(px(30.)))
                .child(div().w(px(120.)).h(px(30.)))
                .into_any_element();

            div()
                .w(px(200.))
                .flex()
                .flex_col()
                .child(MeasureDuringLayout {
                    state: Rc::new(RefCell::new((content, IsolatedLayout::new()))),
                    bounds: self.bounds.clone(),
                })
        }
    }

    #[crate::test]
    fn measures_content_at_the_width_layout_resolved(cx: &mut TestAppContext) {
        let bounds = Rc::new(Cell::new(Bounds::default()));
        let window = cx.add_window({
            let bounds = bounds.clone();
            move |_, _| MeasureTestView { bounds }
        });

        cx.update_window(AnyWindowHandle::from(window), |_, window, cx| {
            window.draw(cx).clear(cx)
        })
        .unwrap();

        // The width reaches the measure closure, so the content wraps the way it
        // will on screen. Without it the content would measure at max content and
        // the element would stop at 30.
        assert_eq!(bounds.get().size.height, px(60.));
    }
}
