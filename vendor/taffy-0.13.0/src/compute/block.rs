//! Computes the CSS block layout algorithm in the case that the block container being laid out contains only block-level boxes
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{AvailableSpace, CoreStyle, LengthPercentageAuto, Overflow, Position};
use crate::style_helpers::TaffyMaxContent;
use crate::tree::{CollapsibleMarginSet, Layout, LayoutInput, LayoutOutput, RunMode, SizingMode};
use crate::tree::{LayoutPartialTree, LayoutPartialTreeExt, NodeId};
use crate::util::debug::debug_log;
use crate::util::sys::f32_max;
use crate::util::sys::Vec;
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};
use crate::{
    BlockContainerStyle, BlockItemStyle, BoxGenerationMode, BoxSizing, Direction, LayoutBlockContainer, RequestedAxis,
    TextAlign,
};

#[cfg(feature = "float_layout")]
use super::float::{BfcSlot, ContentSlot, FloatContext, FloatIntrinsicWidthCalculator};
#[cfg(feature = "float_layout")]
use crate::{Clear, Float, FloatDirection};

/// Context for positioning Block and Float boxes within a Block Formatting Context
pub struct BlockFormattingContext {
    /// The float positioning context that handles positioning floats within this Block Formatting Context
    #[cfg(feature = "float_layout")]
    float_context: FloatContext,
}

impl Default for BlockFormattingContext {
    fn default() -> Self {
        Self {
            #[cfg(feature = "float_layout")]
            float_context: FloatContext::new(),
        }
    }
}

impl BlockFormattingContext {
    /// Create a new `BlockFormattingContext` with the specified width constraint
    pub fn new() -> Self {
        Default::default()
    }

    /// Create an initial `BlockContext` for this `BlockFormattingContext`
    pub fn root_block_context(&mut self) -> BlockContext<'_> {
        BlockContext {
            bfc: self,
            y_offset: 0.0,
            insets: [0.0, 0.0],
            content_box_insets: [0.0, 0.0],
            float_content_contribution: 0.0,
            is_root: true,
            #[cfg(feature = "float_layout")]
            adjoining_floats: [false, false],
            #[cfg(feature = "float_layout")]
            top_adjoining_floats: None,
        }
    }
}

/// Context for each individual Block within a Block Formatting Context
///
/// Contains a mutable reference to the BlockFormattingContext + block-specific data
pub struct BlockContext<'bfc> {
    /// A mutable reference to the root BlockFormatttingContext that this BlockContext belongs to
    bfc: &'bfc mut BlockFormattingContext,
    /// The y-offset of the border-top of the block node, relative to the to the border-top of the
    /// root node of the Block Formatting Context it belongs to.
    y_offset: f32,
    /// The x-inset of the border-box in from each side of the block node, relative to the root node of the Block Formatting Context it belongs to.
    insets: [f32; 2],
    /// The x-insets of the content box
    content_box_insets: [f32; 2],
    /// The height that floats take up in the element
    float_content_contribution: f32,
    /// Whether the node is the root of the Block Formatting Context is belongs to.
    is_root: bool,
    /// Whether a float has been placed (on each side) whose position adjoins the current
    /// margin-collapse strut of this block (i.e. whose final position can still be moved by
    /// margins that collapse into that strut). Such floats force clearance on cleared elements
    /// whose margins adjoin the same strut.
    #[cfg(feature = "float_layout")]
    adjoining_floats: [bool; 2],
    /// The value of `adjoining_floats` frozen at the first point at which in-flow content was
    /// committed within this block (resolving the position of the block's top margin strut).
    /// `None` if no in-flow content has been committed yet.
    #[cfg(feature = "float_layout")]
    top_adjoining_floats: Option<[bool; 2]>,
}

impl BlockContext<'_> {
    /// Create a sub-`BlockContext` for a child block node
    pub fn sub_context(&mut self, additional_y_offset: f32, insets: [f32; 2]) -> BlockContext<'_> {
        let insets = [self.insets[0] + insets[0], self.insets[1] + insets[1]];
        BlockContext {
            bfc: self.bfc,
            y_offset: self.y_offset + additional_y_offset,
            insets,
            content_box_insets: insets,
            float_content_contribution: 0.0,
            is_root: false,
            // Floats adjoining the parent's current strut also adjoin this block's top strut
            // (if this block's top margin collapses with its first child's, which is checked separately)
            #[cfg(feature = "float_layout")]
            adjoining_floats: self.adjoining_floats,
            #[cfg(feature = "float_layout")]
            top_adjoining_floats: None,
        }
    }

    /// Returns whether this block is the root block of it's Block Formatting Context
    pub fn is_bfc_root(&self) -> bool {
        self.is_root
    }
}

#[cfg(feature = "float_layout")]
impl BlockContext<'_> {
    /// Set the width of the overall Block Formatting Context. This is used to resolve positions
    /// that are relative to the right of the context such as right-floated boxes.
    ///
    /// Sub-blocks within a Block Formatting Context should use the `Self::sub_context` method to create
    /// a sub-`BlockContext` with `insets` instead.
    pub fn set_width(&mut self, available_width: f32) {
        self.bfc.float_context.set_width(available_width);
    }

    /// Set the x-axis content-box insets of the `BlockContext`. These are the difference between the border-box
    /// and the content-box of the box (padding + border + scrollbar_gutter).
    pub fn apply_content_box_inset(&mut self, content_box_x_insets: [f32; 2]) {
        self.content_box_insets[0] = self.insets[0] + content_box_x_insets[0];
        self.content_box_insets[1] = self.insets[1] + content_box_x_insets[1];
    }

    /// Whether the float context contains any floats
    #[inline(always)]
    pub fn has_floats(&self) -> bool {
        self.bfc.float_context.has_floats()
    }

    /// Whether the float context contains any floats that extend to or below min_y
    #[inline(always)]
    pub fn has_active_floats(&self, min_y: f32) -> bool {
        self.bfc.float_context.has_active_floats(min_y + self.y_offset)
    }

    /// Position a floated box with the context
    pub fn place_floated_box(
        &mut self,
        floated_box: Size<f32>,
        min_y: f32,
        direction: FloatDirection,
        clear: Clear,
        adjoins_unresolved_strut: bool,
    ) -> Point<f32> {
        if adjoins_unresolved_strut {
            self.adjoining_floats[direction as usize] = true;
        }
        let mut pos = self.bfc.float_context.place_floated_box(
            floated_box,
            min_y + self.y_offset,
            self.content_box_insets,
            direction,
            clear,
        );
        pos.y -= self.y_offset;
        pos.x -= self.insets[0];

        self.float_content_contribution = self.float_content_contribution.max(pos.y + floated_box.height);

        pos
    }

    /// Search a space suitable for laying out non-floated content into
    pub fn find_content_slot(&self, min_y: f32, clear: Clear, after: Option<usize>) -> ContentSlot {
        let mut slot =
            self.bfc.float_context.find_content_slot(min_y + self.y_offset, self.content_box_insets, clear, after);
        slot.y -= self.y_offset;
        slot.x -= self.insets[0];
        slot
    }

    /// Search for a space suitable for laying out a box that establishes an independent
    /// formatting context (whose border box must not overlap floats)
    pub fn find_bfc_slot(
        &self,
        min_y: f32,
        margins: [f32; 2],
        direction: Direction,
        clear: Clear,
        after: Option<usize>,
    ) -> BfcSlot {
        let mut slot = self.bfc.float_context.find_bfc_slot(
            min_y + self.y_offset,
            self.content_box_insets,
            margins,
            direction,
            clear,
            after,
        );
        slot.y -= self.y_offset;
        slot.x -= self.insets[0];
        slot
    }

    /// Get the bottom of lowest relevant float for the specific clear property
    pub fn cleared_threshold(&self, clear: Clear) -> Option<f32> {
        self.bfc.float_context.cleared_threshold(clear).map(|threshold| threshold - self.y_offset)
    }

    /// Whether a float that is adjoining the current margin-collapse strut has been placed
    /// on the side(s) relevant to the passed clear property
    pub fn has_adjoining_float(&self, clear: Clear) -> bool {
        match clear {
            Clear::Left => self.adjoining_floats[0],
            Clear::Right => self.adjoining_floats[1],
            Clear::Both => self.adjoining_floats[0] || self.adjoining_floats[1],
            Clear::None => false,
        }
    }

    /// Merge adjoining float flags propagated from a child block into this block's flags
    fn merge_adjoining_floats(&mut self, flags: [bool; 2]) {
        self.adjoining_floats[0] |= flags[0];
        self.adjoining_floats[1] |= flags[1];
    }

    /// Record that in-flow content has been committed within this block, resolving the position of
    /// the current margin-collapse strut. Floats placed before this point no longer adjoin the
    /// current strut. The flags for the block's top strut are frozen at the first commit.
    fn commit_strut(&mut self) {
        if self.top_adjoining_floats.is_none() {
            self.top_adjoining_floats = Some(self.adjoining_floats);
        }
        self.adjoining_floats = [false, false];
    }

    /// The adjoining float flags for this block's top margin strut: floats placed while the
    /// position of the block's top strut was still unresolved
    fn top_adjoining_floats(&self) -> [bool; 2] {
        self.top_adjoining_floats.unwrap_or(self.adjoining_floats)
    }

    /// Update the height that descendent floats with the height that floats consume
    /// within a particular child
    fn add_child_floated_content_height_contribution(&mut self, child_contribution: f32) {
        self.float_content_contribution = self.float_content_contribution.max(child_contribution);
    }

    /// Returns the height that descendent floats consume
    pub fn floated_content_height_contribution(&self) -> f32 {
        self.float_content_contribution
    }
}

#[cfg(not(feature = "float_layout"))]
impl BlockContext<'_> {
    #[inline(always)]
    /// Returns the height that descendent floats consume (always 0.0 when the float feature is disabled)
    fn float_content_contribution(&self) -> f32 {
        0.0
    }
}

use super::common::alignment::{apply_alignment_fallback, compute_alignment_offset};
#[cfg(feature = "content_size")]
use super::common::content_size::compute_content_size_contribution;

/// Per-child data that is accumulated and modified over the course of the layout algorithm
struct BlockItem {
    /// The identifier for the associated node
    node_id: NodeId,

    /// The "source order" of the item. This is the index of the item within the children iterator,
    /// and controls the order in which the nodes are placed
    order: u32,

    /// Items that are tables don't have stretch sizing applied to them
    is_table: bool,

    /// Items that are replaced elements resolve an auto width to their intrinsic size
    /// rather than being stretch-sized
    /// <https://www.w3.org/TR/CSS22/visudet.html#block-replaced-width>
    is_replaced: bool,

    /// Whether the child is a non-independent block or inline node
    is_in_same_bfc: bool,

    #[cfg(feature = "float_layout")]
    /// The `float` style of the node
    float: Float,
    #[cfg(feature = "float_layout")]
    /// The `clear` style of the node
    clear: Clear,

    /// The base size of this item
    size: Size<Option<f32>>,
    /// The minimum allowable size of this item
    min_size: Size<Option<f32>>,
    /// The maximum allowable size of this item
    max_size: Size<Option<f32>>,

    /// The overflow style of the item
    overflow: Point<Overflow>,
    /// The width of the item's scrollbars (if it has scrollbars)
    scrollbar_width: f32,

    /// The position style of the item
    position: Position,
    /// The final offset of this item
    inset: Rect<LengthPercentageAuto>,
    /// The margin of this item
    margin: Rect<LengthPercentageAuto>,
    /// The margin of this item
    padding: Rect<f32>,
    /// The margin of this item
    border: Rect<f32>,
    /// The sum of padding and border for this item
    padding_border_sum: Size<f32>,

    /// The computed border box size of this item
    computed_size: Size<f32>,
    /// The computed "static position" of this item. The static position is the position
    /// taking into account padding, border, margins, and scrollbar_gutters but not inset
    static_position: Point<f32>,
    /// Whether margins can be collapsed through this item
    can_be_collapsed_through: bool,

    /// Pending layout for in-flow non-floated items. Held back from `set_unrounded_layout` so the
    /// post-loop `align-content` pass in `compute_inner` can shift `location.y` before commit.
    final_layout: Option<Layout>,
}

/// Computes the layout of [`LayoutPartialTree`] according to the block layout algorithm
pub fn compute_block_layout(
    tree: &mut impl LayoutBlockContainer,
    node_id: NodeId,
    inputs: LayoutInput,
    block_ctx: Option<&mut BlockContext<'_>>,
) -> LayoutOutput {
    let LayoutInput { known_dimensions, parent_size, run_mode, .. } = inputs;
    let style = tree.get_block_container_style(node_id);

    // Pull these out earlier to avoid borrowing issues
    let overflow = style.overflow();
    let is_scroll_container = overflow.x.is_scroll_container() || overflow.y.is_scroll_container();
    let aspect_ratio = style.aspect_ratio();
    let padding = style.padding().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let border = style.border().resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let padding_border_size = (padding + border).sum_axes();
    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_size } else { Size::ZERO };

    let min_size = style
        .min_size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let max_size = style
        .max_size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let clamped_style_size = if inputs.sizing_mode == SizingMode::InherentSize {
        style
            .size()
            .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment)
            .maybe_clamp(min_size, max_size)
    } else {
        Size::NONE
    };

    drop(style);

    // If both min and max in a given axis are set and max <= min then this determines the size in that axis
    let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
        (Some(min), Some(max)) if max <= min => Some(min),
        _ => None,
    });

    let styled_based_known_dimensions =
        known_dimensions.or(min_max_definite_size).or(clamped_style_size).maybe_max(padding_border_size);

    // Short-circuit layout if the container's size is fully determined by the container's size and the run mode
    // is ComputeSize (and thus the container's size is all that we're interested in)
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = styled_based_known_dimensions {
            return LayoutOutput::from_outer_size(Size { width, height });
        }

        // We can also short-circuit if the width is known and only the width has been requested.
        if inputs.axis == RequestedAxis::Horizontal {
            if let Some(width) = styled_based_known_dimensions.width {
                return LayoutOutput::from_outer_size(Size { width, height: 0.0 });
            }
        }
    }

    // Unwrap the block formatting context if one was passed, or else create a new one
    debug_log!("BLOCK");
    match block_ctx {
        Some(inherited_bfc) if !is_scroll_container => compute_inner(
            tree,
            node_id,
            LayoutInput { known_dimensions: styled_based_known_dimensions, ..inputs },
            inherited_bfc,
        ),
        _ => {
            let mut root_bfc = BlockFormattingContext::new();
            let mut root_ctx = root_bfc.root_block_context();
            compute_inner(
                tree,
                node_id,
                LayoutInput { known_dimensions: styled_based_known_dimensions, ..inputs },
                &mut root_ctx,
            )
        }
    }
}

/// Computes the layout of [`LayoutBlockContainer`] according to the block layout algorithm
fn compute_inner(
    tree: &mut impl LayoutBlockContainer,
    node_id: NodeId,
    inputs: LayoutInput,
    #[allow(unused_mut)] mut block_ctx: &mut BlockContext<'_>,
) -> LayoutOutput {
    let LayoutInput {
        known_dimensions, parent_size, available_space, run_mode, vertical_margins_are_collapsible, ..
    } = inputs;

    let style = tree.get_block_container_style(node_id);
    let raw_padding = style.padding();
    let raw_border = style.border();
    let raw_margin = style.margin();
    let aspect_ratio = style.aspect_ratio();
    let padding = raw_padding.resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let border = raw_border.resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
    let direction = style.direction();

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = {
        let offsets = style.overflow().transpose().map(|overflow| match overflow {
            Overflow::Scroll => style.scrollbar_width(),
            _ => 0.0,
        });
        match direction {
            Direction::Ltr => Rect { top: 0.0, left: 0.0, right: offsets.x, bottom: offsets.y },
            Direction::Rtl => Rect { top: 0.0, left: offsets.x, right: 0.0, bottom: offsets.y },
        }
    };
    let padding_border = padding + border;
    let padding_border_size = padding_border.sum_axes();
    let content_box_inset = padding_border + scrollbar_gutter;

    // Apply content box inset
    #[cfg(feature = "float_layout")]
    block_ctx.apply_content_box_inset([content_box_inset.left, content_box_inset.right]);

    let box_sizing_adjustment =
        if style.box_sizing() == BoxSizing::ContentBox { padding_border_size } else { Size::ZERO };
    let size = style
        .size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let min_size = style
        .min_size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let max_size = style
        .max_size()
        .maybe_resolve(parent_size, |val, basis| tree.calc(val, basis))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);

    // css-sizing-4: a definite size in one axis transfers through `aspect-ratio`
    // to make the other definite. Deriving it from `known_dimensions` self-gates
    // the transfer — a block parent fills an axis only when it's a real
    // constraint (e.g. the stretched width at final layout) and leaves it None
    // while probing intrinsic sizes, so measure passes stay content-based. Only a
    // newly-filled axis is adopted (and clamped); an incoming known size is left
    // as the parent resolved it (re-clamping would undo padding/border overrides).
    let known_dimensions = {
        let derived = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        Size { width: known_dimensions.width.or(derived.width), height: known_dimensions.height.or(derived.height) }
    };
    let container_content_box_size = known_dimensions.maybe_sub(content_box_inset.sum_axes());

    let overflow = style.overflow();
    let is_scroll_container = overflow.x.is_scroll_container() || overflow.y.is_scroll_container();

    // Determine margin collapsing behaviour
    let own_margins_collapse_with_children = Line {
        start: vertical_margins_are_collapsible.start
            && !is_scroll_container
            && style.position() == Position::Relative
            && padding.top == 0.0
            && border.top == 0.0,
        end: vertical_margins_are_collapsible.end
            && !is_scroll_container
            && style.position() == Position::Relative
            && padding.bottom == 0.0
            && border.bottom == 0.0
            && size.height.is_none(),
    };
    let has_styles_preventing_being_collapsed_through = !style.is_block()
        || block_ctx.is_bfc_root()
        || is_scroll_container
        || style.position() == Position::Absolute
        || padding.top > 0.0
        || padding.bottom > 0.0
        || border.top > 0.0
        || border.bottom > 0.0
        || matches!(size.height, Some(h) if h > 0.0)
        || matches!(min_size.height, Some(h) if h > 0.0);

    let text_align = style.text_align();
    let align_content = style.align_content();
    drop(style);

    // 1. Generate items
    let mut items = generate_item_list(tree, node_id, container_content_box_size);

    // 2. Compute container width
    let container_outer_width = known_dimensions.width.unwrap_or_else(|| {
        let available_width = available_space.width.maybe_sub(content_box_inset.horizontal_axis_sum());
        let intrinsic_width = determine_content_based_container_width(tree, &items, available_width)
            + content_box_inset.horizontal_axis_sum();
        intrinsic_width.maybe_clamp(min_size.width, max_size.width).maybe_max(Some(padding_border_size.width))
    });

    // Short-circuit if computing size and both dimensions known
    if let (RunMode::ComputeSize, Some(container_outer_height)) = (run_mode, known_dimensions.height) {
        return LayoutOutput::from_outer_size(Size { width: container_outer_width, height: container_outer_height });
    }

    // We can also short-circuit if the width is known and only the width has been requested.
    if run_mode == RunMode::ComputeSize && inputs.axis == RequestedAxis::Horizontal {
        return LayoutOutput::from_outer_size(Size { width: container_outer_width, height: 0.0 });
    }

    let container_percentage_resolution_height =
        known_dimensions.height.or(size.height.maybe_max(min_size.height)).or(min_size.height);

    // 3. Perform final item layout and return content height
    let resolved_padding = raw_padding.resolve_or_zero(Some(container_outer_width), |val, basis| tree.calc(val, basis));
    let resolved_border = raw_border.resolve_or_zero(Some(container_outer_width), |val, basis| tree.calc(val, basis));
    let resolved_content_box_inset = resolved_padding + resolved_border + scrollbar_gutter;
    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let (
        mut inflow_content_size,
        mut intrinsic_outer_height,
        first_child_top_margin_set,
        last_child_bottom_margin_set,
        mut first_baseline,
    ) = perform_final_layout_on_in_flow_children(
        tree,
        run_mode,
        &mut items,
        container_outer_width,
        container_percentage_resolution_height,
        content_box_inset,
        resolved_content_box_inset,
        resolved_border,
        text_align,
        direction,
        own_margins_collapse_with_children,
        block_ctx,
    );

    // Root BFCs contain floats
    #[cfg(feature = "float_layout")]
    if block_ctx.is_bfc_root() || is_scroll_container {
        intrinsic_outer_height = intrinsic_outer_height.max(block_ctx.floated_content_height_contribution());
    }

    let container_outer_height = known_dimensions
        .height
        .unwrap_or(intrinsic_outer_height.maybe_clamp(min_size.height, max_size.height))
        .maybe_max(Some(padding_border_size.height));
    let final_outer_size = Size { width: container_outer_width, height: container_outer_height };

    // Apply `align-content` to in-flow non-floated items if requested. The per-item layouts were
    // held back in `item.final_layout` so that this step can shift `location.y` before tree commit.
    //
    // For block layout the entire stack of in-flow children is treated as a single alignment
    // subject. That means distribution keywords (`space-between`, `space-around`,
    // `space-evenly`, `stretch`) must invoke the single-subject fallback unconditionally —
    // which is what passing `num_items = 1` to `apply_alignment_fallback` does. The whole
    // group then shifts by one offset, with zero inter-item gap.
    if let Some(align_content) = align_content {
        let container_inner_height = container_outer_height - resolved_content_box_inset.vertical_axis_sum();
        let inflow_content_height = intrinsic_outer_height - resolved_content_box_inset.vertical_axis_sum();
        let free_space = container_inner_height - inflow_content_height;
        let any_in_flow = items.iter().any(|item| item.final_layout.is_some());
        if any_in_flow {
            let keyword = apply_alignment_fallback(free_space, 1, align_content);
            let group_offset = compute_alignment_offset(free_space, 1, 0.0, keyword, false, true);
            first_baseline = first_baseline.map(|baseline| baseline + group_offset);
            for item in items.iter_mut() {
                if let Some(layout) = item.final_layout.as_mut() {
                    layout.location.y += group_offset;
                }
            }

            #[cfg(feature = "content_size")]
            {
                inflow_content_size = Size::ZERO;
                for item in items.iter() {
                    if let Some(layout) = item.final_layout.as_ref() {
                        let contribution_location = if direction.is_rtl() {
                            Point {
                                x: container_outer_width
                                    - (layout.location.x + layout.size.width)
                                    - resolved_border.right,
                                y: layout.location.y - resolved_border.top,
                            }
                        } else {
                            Point {
                                x: layout.location.x - resolved_border.left,
                                y: layout.location.y - resolved_border.top,
                            }
                        };
                        inflow_content_size = inflow_content_size.f32_max(compute_content_size_contribution(
                            contribution_location,
                            layout.size,
                            layout.content_size,
                            item.overflow,
                        ));
                    }
                }
            }
        }
    }

    // Determine whether this node can be collapsed through
    let all_in_flow_children_can_be_collapsed_through = items.iter().all(|item| {
        #[cfg(feature = "float_layout")]
        if item.float.is_floated() {
            return true;
        }
        item.position == Position::Absolute || item.can_be_collapsed_through
    });
    let can_be_collapsed_through =
        !has_styles_preventing_being_collapsed_through && all_in_flow_children_can_be_collapsed_through;

    let mut output = LayoutOutput {
        size: final_outer_size,
        #[cfg(feature = "content_size")]
        content_size: Size::ZERO,
        first_baselines: Point { x: None, y: first_baseline },
        top_margin: if own_margins_collapse_with_children.start {
            first_child_top_margin_set
        } else {
            let margin_top = raw_margin.top.resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
            CollapsibleMarginSet::from_margin(margin_top)
        },
        bottom_margin: if own_margins_collapse_with_children.end {
            last_child_bottom_margin_set
        } else {
            let margin_bottom =
                raw_margin.bottom.resolve_or_zero(parent_size.width, |val, basis| tree.calc(val, basis));
            CollapsibleMarginSet::from_margin(margin_bottom)
        },
        margins_can_collapse_through: can_be_collapsed_through,
    };

    // Short-circuit if computing size.
    //
    // Note: it is important that we return the margin-collapsing related outputs here as Parent block containers
    // rely on the `top_margin`/`bottom_margin` of their children to compute their own intrinsic height.
    if run_mode == RunMode::ComputeSize {
        return output;
    }

    // Commit deferred in-flow layouts to the tree. Floated items already wrote their own layouts.
    for item in items.iter() {
        if let Some(layout) = item.final_layout.as_ref() {
            tree.set_unrounded_layout(item.node_id, layout);
        }
    }

    // 4. Layout absolutely positioned children
    let absolute_position_inset = resolved_border + scrollbar_gutter;
    let absolute_position_area = final_outer_size - absolute_position_inset.sum_axes();
    let absolute_position_offset = Point { x: absolute_position_inset.left, y: absolute_position_inset.top };
    let absolute_content_size = perform_absolute_layout_on_absolute_children(
        tree,
        &items,
        absolute_position_area,
        absolute_position_offset,
        direction,
    );

    #[cfg(feature = "content_size")]
    {
        // The container's own padding at the end of the content is part of its scrollable
        // overflow region, so it is included in the in-flow content size.
        inflow_content_size.width += if direction.is_rtl() { resolved_padding.left } else { resolved_padding.right };
        inflow_content_size.height += resolved_padding.bottom;
        output.content_size = inflow_content_size.f32_max(absolute_content_size);
    }

    // 5. Perform hidden layout on hidden children
    let len = tree.child_count(node_id);
    for order in 0..len {
        let child = tree.get_child_id(node_id, order);
        let child_style = tree.get_block_child_style(child);
        if child_style.box_generation_mode() == BoxGenerationMode::None {
            drop(child_style);
            tree.set_unrounded_layout(child, &Layout::with_order(order as u32));
            tree.perform_child_layout(
                child,
                Size::NONE,
                Size::NONE,
                Size::MAX_CONTENT,
                SizingMode::InherentSize,
                Line::FALSE,
            );
        }
    }

    output
}

/// Create a `Vec` of `BlockItem` structs where each item in the `Vec` represents a child of the current node
#[inline]
fn generate_item_list(
    tree: &impl LayoutBlockContainer,
    node: NodeId,
    node_inner_size: Size<Option<f32>>,
) -> Vec<BlockItem> {
    tree.child_ids(node)
        .map(|child_node_id| (child_node_id, tree.get_block_child_style(child_node_id)))
        .filter(|(_, style)| style.box_generation_mode() != BoxGenerationMode::None)
        .enumerate()
        .map(|(order, (child_node_id, child_style))| {
            let aspect_ratio = child_style.aspect_ratio();
            let padding = child_style.padding().resolve_or_zero(node_inner_size, |val, basis| tree.calc(val, basis));
            let border = child_style.border().resolve_or_zero(node_inner_size, |val, basis| tree.calc(val, basis));
            let pb_sum = (padding + border).sum_axes();
            let box_sizing_adjustment =
                if child_style.box_sizing() == BoxSizing::ContentBox { pb_sum } else { Size::ZERO };

            let position = child_style.position();
            let overflow = child_style.overflow();

            #[cfg(feature = "float_layout")]
            let float = child_style.float();
            #[cfg(feature = "float_layout")]
            let is_not_floated = float == Float::None;

            #[cfg(not(feature = "float_layout"))]
            let is_not_floated = true;

            let is_block = child_style.is_block();
            let is_table = child_style.is_table();
            let is_replaced = child_style.is_compressible_replaced();
            let is_scroll_container = overflow.x.is_scroll_container() || overflow.y.is_scroll_container();

            let is_in_same_bfc: bool =
                is_block && !is_table && position != Position::Absolute && is_not_floated && !is_scroll_container;

            BlockItem {
                node_id: child_node_id,
                order: order as u32,
                is_table,
                is_replaced,
                is_in_same_bfc,
                #[cfg(feature = "float_layout")]
                float,
                #[cfg(feature = "float_layout")]
                clear: child_style.clear(),
                size: child_style
                    .size()
                    .maybe_resolve(node_inner_size, |val, basis| tree.calc(val, basis))
                    .maybe_apply_aspect_ratio(aspect_ratio)
                    .maybe_add(box_sizing_adjustment),
                min_size: child_style
                    .min_size()
                    .maybe_resolve(node_inner_size, |val, basis| tree.calc(val, basis))
                    .maybe_apply_aspect_ratio(aspect_ratio)
                    .maybe_add(box_sizing_adjustment),
                max_size: child_style
                    .max_size()
                    .maybe_resolve(node_inner_size, |val, basis| tree.calc(val, basis))
                    .maybe_apply_aspect_ratio(aspect_ratio)
                    .maybe_add(box_sizing_adjustment),
                overflow,
                scrollbar_width: child_style.scrollbar_width(),
                position,
                inset: child_style.inset(),
                margin: child_style.margin(),
                padding,
                border,
                padding_border_sum: pb_sum,

                // Fields to be computed later (for now we initialise with dummy values)
                computed_size: Size::zero(),
                static_position: Point::zero(),
                can_be_collapsed_through: false,
                final_layout: None,
            }
        })
        .collect()
}

/// Compute the content-based width in the case that the width of the container is not known
#[inline]
fn determine_content_based_container_width(
    tree: &mut impl LayoutPartialTree,
    items: &[BlockItem],
    available_width: AvailableSpace,
) -> f32 {
    let available_space = Size { width: available_width, height: AvailableSpace::MinContent };

    let mut max_child_width = 0.0;
    #[cfg(feature = "float_layout")]
    let mut float_contribution = FloatIntrinsicWidthCalculator::new(available_width);
    for item in items.iter().filter(|item| item.position != Position::Absolute) {
        let known_dimensions = item.size.maybe_clamp(item.min_size, item.max_size);

        let item_x_margin_sum = item
            .margin
            .resolve_or_zero(available_space.width.into_option(), |val, basis| tree.calc(val, basis))
            .horizontal_axis_sum();
        let width = known_dimensions.width.unwrap_or_else(|| {
            tree.measure_child_size(
                item.node_id,
                known_dimensions,
                Size::NONE,
                available_space.map_width(|w| w.maybe_sub(item_x_margin_sum)),
                SizingMode::InherentSize,
                crate::AbsoluteAxis::Horizontal,
                Line::TRUE,
            )
        });

        let width = f32_max(width, item.padding_border_sum.width) + item_x_margin_sum;

        #[cfg(feature = "float_layout")]
        if let Some(direction) = item.float.float_direction() {
            float_contribution.add_float(width, direction, item.clear);
            continue;
        }

        max_child_width = f32_max(max_child_width, width);
    }

    #[cfg(feature = "float_layout")]
    {
        max_child_width = max_child_width.max(float_contribution.result());
    }

    max_child_width
}

/// Compute each child's final size and position
#[inline]
#[allow(clippy::too_many_arguments)]
fn perform_final_layout_on_in_flow_children(
    tree: &mut impl LayoutBlockContainer,
    run_mode: RunMode,
    items: &mut [BlockItem],
    container_outer_width: f32,
    container_percentage_resolution_height: Option<f32>,
    content_box_inset: Rect<f32>,
    resolved_content_box_inset: Rect<f32>,
    resolved_border: Rect<f32>,
    text_align: TextAlign,
    direction: Direction,
    own_margins_collapse_with_children: Line<bool>,
    block_ctx: &mut BlockContext<'_>,
) -> (Size<f32>, f32, CollapsibleMarginSet, CollapsibleMarginSet, Option<f32>) {
    // Resolve container_inner_width for sizing child nodes using initial content_box_inset
    let container_inner_width = container_outer_width - resolved_content_box_inset.horizontal_axis_sum();
    let container_percentage_resolution_height =
        container_percentage_resolution_height.maybe_sub(resolved_content_box_inset.vertical_axis_sum());
    let parent_size = Size { width: Some(container_inner_width), height: container_percentage_resolution_height };
    // Vertical available space in block flow is indefinite, NOT a min-content
    // constraint: MaxContent is taffy's representation of "indefinite".
    // Passing MinContent here made every descendant grid believe it was being
    // sized under a min-content constraint, in which the maximize-tracks step
    // has zero free space — so auto rows containing only scroll-container
    // items (overflow != visible, automatic minimum size = 0) collapsed to
    // zero height. Browsers size such rows to the item's content.
    let available_space =
        Size { width: AvailableSpace::Definite(container_inner_width), height: AvailableSpace::MaxContent };

    // TODO: handle nested blocks with different widths
    #[cfg(feature = "float_layout")]
    if block_ctx.is_bfc_root() {
        block_ctx.set_width(container_outer_width);
        block_ctx.apply_content_box_inset([resolved_content_box_inset.left, resolved_content_box_inset.right]);
    }

    // If this block's top margin does not collapse with its children's then the position of its
    // top margin strut is resolved relative to it, and floats adjoining ancestor struts do not
    // adjoin this block's strut.
    #[cfg(feature = "float_layout")]
    if !own_margins_collapse_with_children.start {
        block_ctx.commit_strut();
    }

    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut inflow_content_size = Size::ZERO;
    let mut committed_y_offset = resolved_content_box_inset.top;
    let mut y_offset_for_absolute = resolved_content_box_inset.top;
    let mut first_child_top_margin_set = CollapsibleMarginSet::ZERO;
    let mut active_collapsible_margin_set = CollapsibleMarginSet::ZERO;
    let mut is_collapsing_with_first_margin_set = true;
    let mut first_baseline: Option<f32> = None;
    // Whether the active margin set contains the margins of a self-collapsing element with
    // clearance. Such margins collapse with the margins of following siblings but the resulting
    // margin does not collapse with the bottom margin of the parent block.
    let mut active_margin_set_has_clearance = false;

    #[cfg(feature = "float_layout")]
    let mut has_active_floats = block_ctx.has_active_floats(committed_y_offset);
    #[cfg(not(feature = "float_layout"))]
    let has_active_floats = false;

    for item in items.iter_mut() {
        if item.position == Position::Absolute {
            let x = match direction {
                Direction::Ltr => resolved_content_box_inset.left,
                Direction::Rtl => container_outer_width - resolved_content_box_inset.right,
            };
            item.static_position = Point { x, y: y_offset_for_absolute }
        } else {
            let item_margin = item
                .margin
                .map(|margin| margin.resolve_to_option(container_inner_width, |val, basis| tree.calc(val, basis)));
            let item_non_auto_margin = item_margin.map(|m| m.unwrap_or(0.0));
            let item_non_auto_x_margin_sum = item_non_auto_margin.horizontal_axis_sum();

            let scrollbar_size = Size {
                width: if item.overflow.y == Overflow::Scroll { item.scrollbar_width } else { 0.0 },
                height: if item.overflow.x == Overflow::Scroll { item.scrollbar_width } else { 0.0 },
            };

            // Handle floated boxes
            #[cfg(feature = "float_layout")]
            if let Some(float_direction) = item.float.float_direction() {
                has_active_floats = true;

                // A float with `width: auto` is shrink-to-fit (fit-content) sized: the available
                // space clamped between its min-content and max-content sizes.
                let available_width = container_inner_width - item_non_auto_x_margin_sum;
                let item_layout = tree.perform_child_layout(
                    item.node_id,
                    Size::NONE,
                    parent_size,
                    Size { width: AvailableSpace::Definite(available_width), height: AvailableSpace::MaxContent },
                    SizingMode::InherentSize,
                    // A float establishes a new block formatting context: its margins do not
                    // collapse with the margins of its children
                    Line::FALSE,
                );
                let margin_box = item_layout.size + item_non_auto_margin.sum_axes();

                // Floats that occur between collapsing margins are positioned as if they had an otherwise
                // empty anonymous block parent taking part in the flow, so the pending collapsible margins
                // contribute to the float's minimum y position (unless those margins collapse with the
                // container's own top margin, in which case they are applied outside the container).
                //
                // In the latter case the position of the float is not fully resolved: margins contributed
                // by later siblings can still collapse into the strut and move the container (and float).
                // Such floats force clearance on cleared elements whose margins adjoin the same strut.
                let adjoins_unresolved_strut =
                    is_collapsing_with_first_margin_set && own_margins_collapse_with_children.start;
                let y_offset_for_float = if adjoins_unresolved_strut {
                    committed_y_offset
                } else {
                    committed_y_offset + active_collapsible_margin_set.resolve()
                };

                let mut location = block_ctx.place_floated_box(
                    margin_box,
                    y_offset_for_float,
                    float_direction,
                    item.clear,
                    adjoins_unresolved_strut,
                );

                // Ensure that content that appears after a float does not get positioned before/above the float
                //
                // FIXME: this isn't quite right, because a second float at the same location
                // shouldn't cause content to push down to it's level
                // committed_y_offset = committed_y_offset.max(location.y);
                // y_offset_for_absolute = y_offset_for_absolute.max(location.y);

                // Convert the margin-box location returned by float placement into a border-box location
                // for the output Layout
                location.y += item_non_auto_margin.top;
                location.x += item_non_auto_margin.left;

                // println!("BLOCK FLOATED BOX ({:?}) {:?}", item.node_id, float_direction);
                // println!("w:{} h:{} x:{}, y:{}", margin_box.width, margin_box.height, location.x, location.y);

                tree.set_unrounded_layout(
                    item.node_id,
                    &Layout {
                        order: item.order,
                        size: item_layout.size,
                        #[cfg(feature = "content_size")]
                        content_size: item_layout.content_size,
                        scrollbar_size,
                        location,
                        padding: item.padding,
                        border: item.border,
                        margin: item_non_auto_margin,
                    },
                );

                #[cfg(feature = "content_size")]
                {
                    // TODO: Should content size of floated boxes count as "inflow_content_size"
                    // or should it be counted separately?
                    let contribution_location = if direction.is_rtl() {
                        Point {
                            x: container_outer_width - (location.x + item_layout.size.width) - resolved_border.right,
                            y: location.y - resolved_border.top,
                        }
                    } else {
                        Point { x: location.x - resolved_border.left, y: location.y - resolved_border.top }
                    };
                    inflow_content_size = inflow_content_size.f32_max(compute_content_size_contribution(
                        contribution_location,
                        item_layout.size,
                        item_layout.content_size,
                        item.overflow,
                    ));
                }

                continue;
            }

            // Handle non-floated boxes

            let mut y_margin_offset: f32 = 0.0;
            #[cfg(feature = "float_layout")]
            let mut item_avoids_floats = false;
            #[cfg(feature = "float_layout")]
            let mut item_pushed_below_float = false;

            let (stretch_width, float_avoiding_position, float_avoiding_width) = if item.is_in_same_bfc {
                let stretch_width = container_inner_width - item_non_auto_x_margin_sum;
                let position = Point { x: 0.0, y: 0.0 };
                let width = 0.0;

                (stretch_width, position, width)
            } else {
                'block: {
                    // Set y_margin_offset (different bfc child)
                    if !is_collapsing_with_first_margin_set || !own_margins_collapse_with_children.start {
                        y_margin_offset =
                            active_collapsible_margin_set.collapse_with_margin(item_non_auto_margin.top).resolve();
                    };
                    let min_y = committed_y_offset + y_margin_offset;

                    // In addition to the running flag, check the float context directly:
                    // floats placed by the subtree of a preceding in-flow sibling (in the same
                    // BFC) are not reflected in the flag
                    #[cfg(feature = "float_layout")]
                    if has_active_floats || block_ctx.has_active_floats(min_y) {
                        let x_margins = [item_non_auto_margin.left, item_non_auto_margin.right];
                        // An auto width resolves to at least the negation of the margin sum
                        // (so that the margin box width is non-negative, per CSS2 §10.3.3)
                        let min_auto_width = -item_non_auto_x_margin_sum;

                        // Find the highest slot (at or below `min_y`) with enough horizontal space
                        // for the item's border box, which must not overlap any float
                        let mut slot_segment = None;
                        let slot = loop {
                            let slot = block_ctx.find_bfc_slot(min_y, x_margins, direction, item.clear, slot_segment);
                            let Some(segment_id) = slot.segment_id else { break slot };
                            let width = item
                                .size
                                .width
                                .unwrap_or(slot.stretch_width.max(min_auto_width))
                                .maybe_clamp(item.min_size.width, item.max_size.width);
                            if width <= slot.border_width + 0.001 {
                                break slot;
                            }
                            slot_segment = Some(segment_id);
                        };

                        // If the item had to move down to avoid floats then it "separates from the
                        // float": similarly to clearance, its top margin no longer collapses with
                        // the parent's margins.
                        if slot.y > min_y {
                            item_pushed_below_float = true;
                        }

                        has_active_floats = slot.segment_id.is_some();
                        item_avoids_floats = true;
                        let stretch_width = slot.stretch_width.max(min_auto_width);
                        break 'block (stretch_width, Point { x: slot.x, y: slot.y }, slot.border_width);
                    }

                    if !has_active_floats {
                        let stretch_width = container_inner_width - item_non_auto_x_margin_sum;
                        break 'block (
                            stretch_width,
                            Point { x: resolved_content_box_inset.left, y: min_y },
                            container_inner_width,
                        );
                    }

                    unreachable!("One of the above cases will always be hit");
                }
            };

            // Tables and replaced elements are not stretch-sized: they resolve their own
            // size (for replaced elements an auto width resolves to the intrinsic size
            // <https://www.w3.org/TR/CSS22/visudet.html#block-replaced-width>)
            let known_dimensions = if item.is_table || item.is_replaced {
                Size::NONE
            } else {
                item.size
                    .map_width(|width| {
                        Some(width.unwrap_or(stretch_width).maybe_clamp(item.min_size.width, item.max_size.width))
                    })
                    .maybe_clamp(item.min_size, item.max_size)
            };

            //

            let inputs = LayoutInput {
                run_mode,
                sizing_mode: SizingMode::InherentSize,
                axis: RequestedAxis::Both,
                known_dimensions,
                parent_size,
                available_space: available_space.map_width(|_| AvailableSpace::Definite(stretch_width)),
                vertical_margins_are_collapsible: if item.is_in_same_bfc { Line::TRUE } else { Line::FALSE },
            };

            #[cfg(feature = "float_layout")]
            let clear_threshold = block_ctx.cleared_threshold(item.clear);
            #[cfg(feature = "float_layout")]
            let clear_pos = clear_threshold.unwrap_or(f32::NEG_INFINITY);
            #[cfg(not(feature = "float_layout"))]
            let clear_pos = f32::NEG_INFINITY;

            let item_layout = if item.is_in_same_bfc {
                // Replaced elements may not have a known width (they are sized by their
                // measure function rather than stretch-sized)
                let width = known_dimensions.width.unwrap_or(stretch_width);

                // TODO: account for auto margins
                let inset_left = item_non_auto_margin.left + content_box_inset.left;
                let inset_right = container_outer_width - width - inset_left;
                let insets = [inset_left, inset_right];

                // Compute child layout
                let mut child_block_ctx =
                    block_ctx.sub_context((y_offset_for_absolute + item_non_auto_margin.top).max(clear_pos), insets);
                let output = tree.compute_block_child_layout(item.node_id, inputs, Some(&mut child_block_ctx));

                // Extract float contribution from child block context
                #[cfg(feature = "float_layout")]
                {
                    let child_contribution = child_block_ctx.floated_content_height_contribution();
                    let child_top_adjoining_floats = child_block_ctx.top_adjoining_floats();
                    block_ctx.add_child_floated_content_height_contribution(y_offset_for_absolute + child_contribution);
                    // Floats placed while the position of the child's top margin strut was unresolved
                    // also adjoin this block's current strut
                    block_ctx.merge_adjoining_floats(child_top_adjoining_floats);
                }

                output
            } else {
                tree.compute_child_layout(item.node_id, inputs)
            };
            let final_size = item_layout.size;

            let top_margin_set = item_layout.top_margin.collapse_with_margin(item_margin.top.unwrap_or(0.0));
            let bottom_margin_set = item_layout.bottom_margin.collapse_with_margin(item_margin.bottom.unwrap_or(0.0));

            // Expand auto margins to fill available space
            // Note: Vertical auto-margins for relatively positioned block items simply resolve to 0.
            // See: https://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width
            let free_x_space = f32_max(0.0, stretch_width - final_size.width);
            let x_axis_auto_margin_size = {
                let auto_margin_count = item_margin.left.is_none() as u8 + item_margin.right.is_none() as u8;
                if auto_margin_count > 0 {
                    free_x_space / auto_margin_count as f32
                } else {
                    0.0
                }
            };
            let resolved_margin = Rect {
                left: item_margin.left.unwrap_or(x_axis_auto_margin_size),
                right: item_margin.right.unwrap_or(x_axis_auto_margin_size),
                top: top_margin_set.resolve(),
                bottom: bottom_margin_set.resolve(),
            };

            // Resolve item inset
            let inset = item.inset.zip_size(Size { width: container_inner_width, height: 0.0 }, |p, s| {
                p.maybe_resolve(s, |val, basis| tree.calc(val, basis))
            });
            let inset_offset = Point {
                x: if direction.is_rtl() {
                    inset.right.map(|x| -x).or(inset.left).unwrap_or(0.0)
                } else {
                    inset.left.or(inset.right.map(|x| -x)).unwrap_or(0.0)
                },
                y: inset.top.or(inset.bottom.map(|x| -x)).unwrap_or(0.0),
            };

            // Set y_margin_offset (same bfc child)
            if item.is_in_same_bfc
                && (!is_collapsing_with_first_margin_set || !own_margins_collapse_with_children.start)
            {
                y_margin_offset = active_collapsible_margin_set.collapse_with_set(top_margin_set).resolve()
            };

            // Compute clearance (CSS2.2 9.5.2). Clearance is introduced if the hypothetical position of the
            // item's top border edge (the position it would have with normal margin collapsing) is not past
            // the bottom of the relevant floats. When clearance is introduced the item's border edge is
            // placed at `max(float bottom, hypothetical position)`.
            #[cfg(feature = "float_layout")]
            let mut has_clearance = false;
            #[cfg(not(feature = "float_layout"))]
            let has_clearance = false;
            #[cfg(feature = "float_layout")]
            if item.is_in_same_bfc {
                if let Some(threshold) = clear_threshold {
                    // The hypothetical position always includes the item's collapsed top margin set, even
                    // when those margins collapse with the container's own top margin (and are thus applied
                    // outside the container): in that case they still move the container (and hence the item)
                    // relative to the floats.
                    let hypothetical_y =
                        committed_y_offset + active_collapsible_margin_set.collapse_with_set(top_margin_set).resolve();
                    // Clearance is forced (regardless of the hypothetical position) if a relevant float is
                    // adjoining the margin-collapse strut that the item's top margin would collapse into:
                    // if the margins were allowed to collapse they would pull the float down with the item,
                    // so clearance is inserted to separate the two, placing the item just below the float.
                    let forced_clearance = block_ctx.has_adjoining_float(item.clear);
                    if forced_clearance || hypothetical_y < threshold {
                        has_clearance = true;
                        // Clearance stops the item's top margin collapsing with preceding margins. If those
                        // preceding margins collapse with the container's own top margin they are applied
                        // outside the container (moving it down), so the item's cleared position within the
                        // container must be reduced by that amount to keep its absolute position correct.
                        let escaped_margin =
                            if is_collapsing_with_first_margin_set && own_margins_collapse_with_children.start {
                                active_collapsible_margin_set.resolve()
                            } else {
                                0.0
                            };
                        y_margin_offset = threshold - committed_y_offset - escaped_margin;
                    }
                }
            }

            item.computed_size = item_layout.size;
            item.can_be_collapsed_through = item_layout.margins_can_collapse_through && !has_clearance;
            item.static_position = if item.is_in_same_bfc {
                let uncleared_y = committed_y_offset + active_collapsible_margin_set.resolve();
                Point {
                    x: match direction {
                        Direction::Ltr => resolved_content_box_inset.left,
                        Direction::Rtl => container_outer_width - resolved_content_box_inset.right - final_size.width,
                    },
                    y: uncleared_y.max(clear_pos),
                }
            } else {
                // TODO: handle inset and margins
                Point {
                    x: match direction {
                        Direction::Ltr => float_avoiding_position.x,
                        Direction::Rtl => float_avoiding_position.x + float_avoiding_width - final_size.width,
                    },
                    y: float_avoiding_position.y,
                }
            };
            let mut location = if item.is_in_same_bfc {
                Point {
                    x: match direction {
                        Direction::Ltr => resolved_content_box_inset.left + inset_offset.x + resolved_margin.left,
                        Direction::Rtl => {
                            container_outer_width
                                - resolved_content_box_inset.right
                                - final_size.width
                                - resolved_margin.right
                                + inset_offset.x
                        }
                    },
                    y: committed_y_offset + y_margin_offset + inset_offset.y,
                }
            } else {
                // When the item avoids floats, its non-auto margins are already accounted for in the
                // slot's border-box position/width (margins may overlap floats), so only the auto
                // portion of the resolved margin is added here.
                #[cfg(feature = "float_layout")]
                let (extra_margin_left, extra_margin_right) = if item_avoids_floats {
                    (
                        resolved_margin.left - item_non_auto_margin.left,
                        resolved_margin.right - item_non_auto_margin.right,
                    )
                } else {
                    (resolved_margin.left, resolved_margin.right)
                };
                #[cfg(not(feature = "float_layout"))]
                let (extra_margin_left, extra_margin_right) = (resolved_margin.left, resolved_margin.right);

                // TODO: handle inset and margins
                Point {
                    x: match direction {
                        Direction::Ltr => float_avoiding_position.x + extra_margin_left + inset_offset.x,
                        Direction::Rtl => {
                            float_avoiding_position.x + float_avoiding_width - final_size.width - extra_margin_right
                                + inset_offset.x
                        }
                    },
                    y: float_avoiding_position.y + inset_offset.y,
                }
            };

            // Apply alignment
            let item_outer_width = item_layout.size.width + resolved_margin.horizontal_axis_sum();
            if item_outer_width < container_inner_width {
                let free_x_space = container_inner_width - item_outer_width;
                match (text_align, direction) {
                    (TextAlign::Auto, _) => {
                        // Do nothing
                    }
                    (TextAlign::LegacyLeft, Direction::Ltr) => {
                        // Do nothing. Left aligned by default.
                    }
                    (TextAlign::LegacyLeft, Direction::Rtl) => location.x -= free_x_space,
                    (TextAlign::LegacyRight, Direction::Ltr) => location.x += free_x_space,
                    (TextAlign::LegacyRight, Direction::Rtl) => {
                        // Do nothing. Right aligned by default.
                    }
                    (TextAlign::LegacyCenter, Direction::Ltr) => location.x += free_x_space / 2.0,
                    (TextAlign::LegacyCenter, Direction::Rtl) => location.x -= free_x_space / 2.0,
                }
            }

            // A block container's first baseline is the first baseline of its first in-flow child
            // that has one.
            if first_baseline.is_none() {
                first_baseline = item_layout.first_baselines.y.map(|baseline| location.y + baseline);
            }

            // Defer `set_unrounded_layout` to the post-loop pass in `compute_inner` so that
            // `align-content` can shift `location.y` before the layout is committed to the tree.
            item.final_layout = Some(Layout {
                order: item.order,
                size: item_layout.size,
                #[cfg(feature = "content_size")]
                content_size: item_layout.content_size,
                scrollbar_size,
                location,
                padding: item.padding,
                border: item.border,
                margin: resolved_margin,
            });

            #[cfg(feature = "content_size")]
            {
                let contribution_location = if direction.is_rtl() {
                    Point {
                        x: container_outer_width - (location.x + final_size.width) - resolved_border.right,
                        y: location.y - resolved_border.top,
                    }
                } else {
                    Point { x: location.x - resolved_border.left, y: location.y - resolved_border.top }
                };
                inflow_content_size = inflow_content_size.f32_max(compute_content_size_contribution(
                    contribution_location,
                    final_size,
                    item_layout.content_size,
                    item.overflow,
                ));
            }

            // Update first_child_top_margin_set
            //
            // The top margin of an item with clearance does not collapse with the container's top margin,
            // so clearance terminates collapsing without contributing the item's own margins.
            #[cfg(feature = "float_layout")]
            if is_collapsing_with_first_margin_set && item_pushed_below_float {
                // The item's top margin "separated from the float" and must not
                // propagate to the parent
                is_collapsing_with_first_margin_set = false;
            }
            if is_collapsing_with_first_margin_set && has_clearance {
                is_collapsing_with_first_margin_set = false;
            } else if is_collapsing_with_first_margin_set {
                if item.can_be_collapsed_through {
                    first_child_top_margin_set = first_child_top_margin_set
                        .collapse_with_set(top_margin_set)
                        .collapse_with_set(bottom_margin_set);
                } else {
                    first_child_top_margin_set = first_child_top_margin_set.collapse_with_set(top_margin_set);
                    is_collapsing_with_first_margin_set = false;
                }
            }

            // Update active_collapsible_margin_set
            if item.can_be_collapsed_through {
                active_collapsible_margin_set = active_collapsible_margin_set
                    .collapse_with_set(top_margin_set)
                    .collapse_with_set(bottom_margin_set);
                y_offset_for_absolute = committed_y_offset + item_layout.size.height + y_margin_offset;
            } else {
                committed_y_offset = location.y - inset_offset.y + item_layout.size.height;
                // A self-collapsing item with clearance is not collapsed through (its margins do not collapse
                // with margins of preceding siblings), but its top and bottom margins still collapse with each
                // other and with the margins of following siblings.
                if has_clearance && item_layout.margins_can_collapse_through {
                    // The element's border edge stays at the cleared position, but its collapsed margin
                    // extends below it: the border edge sits `top margin` inside the collapsed margin, so
                    // following content is offset by `collapsed margin - top margin` from the border edge.
                    committed_y_offset -= top_margin_set.resolve();
                    active_collapsible_margin_set = top_margin_set.collapse_with_set(bottom_margin_set);
                    active_margin_set_has_clearance = true;
                } else {
                    active_collapsible_margin_set = bottom_margin_set;
                    active_margin_set_has_clearance = false;
                }
                y_offset_for_absolute = committed_y_offset + active_collapsible_margin_set.resolve();
                // Committing in-flow content resolves the position of the current margin-collapse strut,
                // so floats placed before this point no longer force clearance
                #[cfg(feature = "float_layout")]
                block_ctx.commit_strut();
            }
        }
    }

    // The margins of a self-collapsing element with clearance do not collapse with the bottom
    // margin of the parent block: they extend the parent's content height instead of escaping it
    let last_child_bottom_margin_set =
        if active_margin_set_has_clearance { CollapsibleMarginSet::ZERO } else { active_collapsible_margin_set };
    let bottom_y_margin_offset = if active_margin_set_has_clearance {
        active_collapsible_margin_set.resolve()
    } else if own_margins_collapse_with_children.end {
        0.0
    } else {
        last_child_bottom_margin_set.resolve()
    };

    committed_y_offset += resolved_content_box_inset.bottom + bottom_y_margin_offset;
    let content_height = f32_max(0.0, committed_y_offset);
    (inflow_content_size, content_height, first_child_top_margin_set, last_child_bottom_margin_set, first_baseline)
}

/// Perform absolute layout on all absolutely positioned children.
#[inline]
fn perform_absolute_layout_on_absolute_children(
    tree: &mut impl LayoutBlockContainer,
    items: &[BlockItem],
    area_size: Size<f32>,
    area_offset: Point<f32>,
    direction: Direction,
) -> Size<f32> {
    let area_width = area_size.width;
    let area_height = area_size.height;

    #[cfg_attr(not(feature = "content_size"), allow(unused_mut))]
    let mut absolute_content_size = Size::ZERO;

    for item in items.iter().filter(|item| item.position == Position::Absolute) {
        let child_style = tree.get_block_child_style(item.node_id);

        // Skip items that are display:none or are not position:absolute
        if child_style.box_generation_mode() == BoxGenerationMode::None || child_style.position() != Position::Absolute
        {
            continue;
        }

        let aspect_ratio = child_style.aspect_ratio();
        let margin =
            child_style.margin().map(|margin| margin.resolve_to_option(area_width, |val, basis| tree.calc(val, basis)));
        let padding = child_style.padding().resolve_or_zero(Some(area_width), |val, basis| tree.calc(val, basis));
        let border = child_style.border().resolve_or_zero(Some(area_width), |val, basis| tree.calc(val, basis));
        let padding_border_sum = (padding + border).sum_axes();
        let box_sizing_adjustment =
            if child_style.box_sizing() == BoxSizing::ContentBox { padding_border_sum } else { Size::ZERO };

        // Resolve inset
        let left = child_style.inset().left.maybe_resolve(area_width, |val, basis| tree.calc(val, basis));
        let right = child_style.inset().right.maybe_resolve(area_width, |val, basis| tree.calc(val, basis));
        let top = child_style.inset().top.maybe_resolve(area_height, |val, basis| tree.calc(val, basis));
        let bottom = child_style.inset().bottom.maybe_resolve(area_height, |val, basis| tree.calc(val, basis));

        // Compute known dimensions from min/max/inherent size styles
        let style_size = child_style
            .size()
            .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let min_size = child_style
            .min_size()
            .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment)
            .or(padding_border_sum.map(Some))
            .maybe_max(padding_border_sum);
        let max_size = child_style
            .max_size()
            .maybe_resolve(area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let mut known_dimensions = style_size.maybe_clamp(min_size, max_size);

        drop(child_style);

        // Fill in width from left/right and reapply aspect ratio if:
        //   - Width is not already known
        //   - Item has both left and right inset properties set
        if let (None, Some(left), Some(right)) = (known_dimensions.width, left, right) {
            let new_width_raw = area_width.maybe_sub(margin.left).maybe_sub(margin.right) - left - right;
            known_dimensions.width = Some(f32_max(new_width_raw, 0.0));
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        // Fill in height from top/bottom and reapply aspect ratio if:
        //   - Height is not already known
        //   - Item has both top and bottom inset properties set
        if let (None, Some(top), Some(bottom)) = (known_dimensions.height, top, bottom) {
            let new_height_raw = area_height.maybe_sub(margin.top).maybe_sub(margin.bottom) - top - bottom;
            known_dimensions.height = Some(f32_max(new_height_raw, 0.0));
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        let measured_size = tree.measure_child_size_both(
            item.node_id,
            known_dimensions,
            area_size.map(Some),
            Size {
                width: AvailableSpace::Definite(area_width.maybe_clamp(min_size.width, max_size.width)),
                height: AvailableSpace::Definite(area_height.maybe_clamp(min_size.height, max_size.height)),
            },
            SizingMode::ContentSize,
            Line::FALSE,
        );

        let final_size = known_dimensions.unwrap_or(measured_size).maybe_clamp(min_size, max_size);

        let layout_output = tree.perform_child_layout(
            item.node_id,
            final_size.map(Some),
            area_size.map(Some),
            Size {
                width: AvailableSpace::Definite(area_width.maybe_clamp(min_size.width, max_size.width)),
                height: AvailableSpace::Definite(area_height.maybe_clamp(min_size.height, max_size.height)),
            },
            SizingMode::ContentSize,
            Line::FALSE,
        );

        let non_auto_margin = Rect {
            left: if left.is_some() { margin.left.unwrap_or(0.0) } else { 0.0 },
            right: if right.is_some() { margin.right.unwrap_or(0.0) } else { 0.0 },
            top: if top.is_some() { margin.top.unwrap_or(0.0) } else { 0.0 },
            bottom: if bottom.is_some() { margin.bottom.unwrap_or(0.0) } else { 0.0 },
        };

        // Expand auto margins to fill available space
        // https://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width
        let auto_margin = {
            // Auto margins for absolutely positioned elements in block containers only resolve
            // if inset is set. Otherwise they resolve to 0.
            let absolute_auto_margin_space = Point {
                x: right.map(|right| area_size.width - right - left.unwrap_or(0.0)).unwrap_or(final_size.width),
                y: bottom.map(|bottom| area_size.height - bottom - top.unwrap_or(0.0)).unwrap_or(final_size.height),
            };
            let free_space = Size {
                width: absolute_auto_margin_space.x - final_size.width - non_auto_margin.horizontal_axis_sum(),
                height: absolute_auto_margin_space.y - final_size.height - non_auto_margin.vertical_axis_sum(),
            };

            let auto_margin_size = Size {
                // If all three of 'left', 'width', and 'right' are 'auto': First set any 'auto' values for 'margin-left' and 'margin-right' to 0.
                // Then, if the 'direction' property of the element establishing the static-position containing block is 'ltr' set 'left' to the
                // static position and apply rule number three below; otherwise, set 'right' to the static position and apply rule number one below.
                //
                // If none of the three is 'auto': If both 'margin-left' and 'margin-right' are 'auto', solve the equation under the extra constraint
                // that the two margins get equal values, unless this would make them negative, in which case when direction of the containing block is
                // 'ltr' ('rtl'), set 'margin-left' ('margin-right') to zero and solve for 'margin-right' ('margin-left'). If one of 'margin-left' or
                // 'margin-right' is 'auto', solve the equation for that value. If the values are over-constrained, ignore the value for 'left' (in case
                // the 'direction' property of the containing block is 'rtl') or 'right' (in case 'direction' is 'ltr') and solve for that value.
                width: {
                    let auto_margin_count = margin.left.is_none() as u8 + margin.right.is_none() as u8;
                    if auto_margin_count == 2
                        && (style_size.width.is_none() || style_size.width.unwrap() >= free_space.width)
                    {
                        0.0
                    } else if auto_margin_count > 0 {
                        free_space.width / auto_margin_count as f32
                    } else {
                        0.0
                    }
                },
                height: {
                    let auto_margin_count = margin.top.is_none() as u8 + margin.bottom.is_none() as u8;
                    if auto_margin_count == 2
                        && (style_size.height.is_none() || style_size.height.unwrap() >= free_space.height)
                    {
                        0.0
                    } else if auto_margin_count > 0 {
                        free_space.height / auto_margin_count as f32
                    } else {
                        0.0
                    }
                },
            };

            Rect {
                left: margin.left.map(|_| 0.0).unwrap_or(auto_margin_size.width),
                right: margin.right.map(|_| 0.0).unwrap_or(auto_margin_size.width),
                top: margin.top.map(|_| 0.0).unwrap_or(auto_margin_size.height),
                bottom: margin.bottom.map(|_| 0.0).unwrap_or(auto_margin_size.height),
            }
        };

        let resolved_margin = Rect {
            left: margin.left.unwrap_or(auto_margin.left),
            right: margin.right.unwrap_or(auto_margin.right),
            top: margin.top.unwrap_or(auto_margin.top),
            bottom: margin.bottom.unwrap_or(auto_margin.bottom),
        };

        let x_offset = match (left, right) {
            (Some(left), Some(right)) => {
                if direction.is_rtl() {
                    area_size.width - final_size.width - right - resolved_margin.right
                } else {
                    left + resolved_margin.left
                }
            }
            (Some(left), None) => left + resolved_margin.left,
            (None, Some(right)) => area_size.width - final_size.width - right - resolved_margin.right,
            (None, None) => {
                if direction.is_rtl() {
                    item.static_position.x - final_size.width - resolved_margin.right - area_offset.x
                } else {
                    item.static_position.x + resolved_margin.left - area_offset.x
                }
            }
        };
        let location = Point {
            x: x_offset + area_offset.x,
            y: top
                .map(|top| top + resolved_margin.top)
                .or(bottom.map(|bottom| area_size.height - final_size.height - bottom - resolved_margin.bottom))
                .maybe_add(area_offset.y)
                .unwrap_or(item.static_position.y + resolved_margin.top),
        };
        // Note: axis intentionally switched here as scrollbars take up space in the opposite axis
        // to the axis in which scrolling is enabled.
        let scrollbar_size = Size {
            width: if item.overflow.y == Overflow::Scroll { item.scrollbar_width } else { 0.0 },
            height: if item.overflow.x == Overflow::Scroll { item.scrollbar_width } else { 0.0 },
        };

        tree.set_unrounded_layout(
            item.node_id,
            &Layout {
                order: item.order,
                size: final_size,
                #[cfg(feature = "content_size")]
                content_size: layout_output.content_size,
                scrollbar_size,
                location,
                padding,
                border,
                margin: resolved_margin,
            },
        );

        #[cfg(feature = "content_size")]
        {
            let relative_location = Point { x: location.x - area_offset.x, y: location.y - area_offset.y };
            absolute_content_size = absolute_content_size.f32_max(compute_content_size_contribution(
                relative_location,
                final_size,
                layout_output.content_size,
                item.overflow,
            ));
        }
    }

    absolute_content_size
}
