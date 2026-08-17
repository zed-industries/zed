//! Final data structures that represent the high-level UI layout
use crate::geometry::{AbsoluteAxis, Line, Point, Rect, Size};
use crate::style::AvailableSpace;
use crate::style_helpers::TaffyMaxContent;
use crate::util::sys::{f32_max, f32_min};

/// Whether we are performing a full layout, or we merely need to size the node
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum RunMode {
    /// A full layout for this node and all children should be computed
    PerformLayout,
    /// The layout algorithm should be executed such that an accurate container size for the node can be determined.
    /// Layout steps that aren't necessary for determining the container size of the current node can be skipped.
    ComputeSize,
    /// This node should have a null layout set as it has been hidden (i.e. using `Display::None`)
    PerformHiddenLayout,
}

/// Whether styles should be taken into account when computing size
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SizingMode {
    /// Only content contributions should be taken into account
    ContentSize,
    /// Inherent size styles should be taken into account in addition to content contributions
    InherentSize,
}

/// A set of margins that are available for collapsing with for block layout's margin collapsing
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CollapsibleMarginSet {
    /// The largest positive margin
    positive: f32,
    /// The smallest negative margin (with largest absolute value)
    negative: f32,
}

impl CollapsibleMarginSet {
    /// A default margin set with no collapsible margins
    pub const ZERO: Self = Self { positive: 0.0, negative: 0.0 };

    /// Create a set from a single margin
    pub fn from_margin(margin: f32) -> Self {
        if margin >= 0.0 {
            Self { positive: margin, negative: 0.0 }
        } else {
            Self { positive: 0.0, negative: margin }
        }
    }

    /// Collapse a single margin with this set
    pub fn collapse_with_margin(mut self, margin: f32) -> Self {
        if margin >= 0.0 {
            self.positive = f32_max(self.positive, margin);
        } else {
            self.negative = f32_min(self.negative, margin);
        }
        self
    }

    /// Collapse another margin set with this set
    pub fn collapse_with_set(mut self, other: CollapsibleMarginSet) -> Self {
        self.positive = f32_max(self.positive, other.positive);
        self.negative = f32_min(self.negative, other.negative);
        self
    }

    /// Resolve the resultant margin from this set once all collapsible margins
    /// have been collapsed into it
    pub fn resolve(&self) -> f32 {
        self.positive + self.negative
    }
}

/// An axis that layout algorithms can be requested to compute a size for
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum RequestedAxis {
    /// The horizontal axis
    Horizontal,
    /// The vertical axis
    Vertical,
    /// Both axes
    Both,
}

impl From<AbsoluteAxis> for RequestedAxis {
    fn from(value: AbsoluteAxis) -> Self {
        match value {
            AbsoluteAxis::Horizontal => RequestedAxis::Horizontal,
            AbsoluteAxis::Vertical => RequestedAxis::Vertical,
        }
    }
}
impl TryFrom<RequestedAxis> for AbsoluteAxis {
    type Error = ();
    fn try_from(value: RequestedAxis) -> Result<Self, Self::Error> {
        match value {
            RequestedAxis::Horizontal => Ok(AbsoluteAxis::Horizontal),
            RequestedAxis::Vertical => Ok(AbsoluteAxis::Vertical),
            RequestedAxis::Both => Err(()),
        }
    }
}

/// A struct containing the inputs constraints/hints for laying out a node, which are passed in by the parent
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LayoutInput {
    /// Whether we only need to know the Node's size, or whether we need to perform a full layout
    pub run_mode: RunMode,
    /// Whether a Node's style sizes should be taken into account or ignored
    pub sizing_mode: SizingMode,
    /// Which axis we need the size of
    pub axis: RequestedAxis,

    /// Known dimensions represent dimensions (width/height) which should be taken as fixed when performing layout.
    /// For example, if known_dimensions.width is set to Some(WIDTH) then this means something like:
    ///
    ///    "What would the height of this node be, assuming the width is WIDTH"
    ///
    /// Layout functions will be called with both known_dimensions set for final layout. Where the meaning is:
    ///
    ///   "The exact size of this node is WIDTHxHEIGHT. Please lay out your children"
    ///
    pub known_dimensions: Size<Option<f32>>,
    /// Parent size dimensions are intended to be used for percentage resolution.
    pub parent_size: Size<Option<f32>>,
    /// Available space represents an amount of space to layout into, and is used as a soft constraint
    /// for the purpose of wrapping.
    pub available_space: Size<AvailableSpace>,
    /// Specific to CSS Block layout. Used for correctly computing margin collapsing. You probably want to set this to `Line::FALSE`.
    pub vertical_margins_are_collapsible: Line<bool>,
}

impl LayoutInput {
    /// A LayoutInput that can be used to request hidden layout
    pub const HIDDEN: LayoutInput = LayoutInput {
        // The important property for hidden layout
        run_mode: RunMode::PerformHiddenLayout,
        // The rest will be ignored
        known_dimensions: Size::NONE,
        parent_size: Size::NONE,
        available_space: Size::MAX_CONTENT,
        sizing_mode: SizingMode::InherentSize,
        axis: RequestedAxis::Both,
        vertical_margins_are_collapsible: Line::FALSE,
    };
}

/// A struct containing the result of laying a single node, which is returned up to the parent node
///
/// A baseline is the line on which text sits. Your node likely has a baseline if it is a text node, or contains
/// children that may be text nodes. See <https://www.w3.org/TR/css-writing-modes-3/#intro-baselines> for details.
/// If your node does not have a baseline (or you are unsure how to compute it), then simply return `Point::NONE`
/// for the first_baselines field
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LayoutOutput {
    /// The size of the node
    pub size: Size<f32>,
    #[cfg(feature = "content_size")]
    /// The size of the content within the node
    pub content_size: Size<f32>,
    /// The first baseline of the node in each dimension, if any
    pub first_baselines: Point<Option<f32>>,
    /// Top margin that can be collapsed with. This is used for CSS block layout and can be set to
    /// `CollapsibleMarginSet::ZERO` for other layout modes that don't support margin collapsing
    pub top_margin: CollapsibleMarginSet,
    /// Bottom margin that can be collapsed with. This is used for CSS block layout and can be set to
    /// `CollapsibleMarginSet::ZERO` for other layout modes that don't support margin collapsing
    pub bottom_margin: CollapsibleMarginSet,
    /// Whether margins can be collapsed through this node. This is used for CSS block layout and can
    /// be set to `false` for other layout modes that don't support margin collapsing
    pub margins_can_collapse_through: bool,
}

impl LayoutOutput {
    /// An all-zero `LayoutOutput` for hidden nodes
    pub const HIDDEN: Self = Self {
        size: Size::ZERO,
        #[cfg(feature = "content_size")]
        content_size: Size::ZERO,
        first_baselines: Point::NONE,
        top_margin: CollapsibleMarginSet::ZERO,
        bottom_margin: CollapsibleMarginSet::ZERO,
        margins_can_collapse_through: false,
    };

    /// A blank layout output
    pub const DEFAULT: Self = Self::HIDDEN;

    /// Constructor to create a `LayoutOutput` from just the size and baselines
    pub fn from_sizes_and_baselines(
        size: Size<f32>,
        #[cfg_attr(not(feature = "content_size"), allow(unused_variables))] content_size: Size<f32>,
        first_baselines: Point<Option<f32>>,
    ) -> Self {
        Self {
            size,
            #[cfg(feature = "content_size")]
            content_size,
            first_baselines,
            top_margin: CollapsibleMarginSet::ZERO,
            bottom_margin: CollapsibleMarginSet::ZERO,
            margins_can_collapse_through: false,
        }
    }

    /// Construct a `LayoutOutput` from just the container and content sizes
    pub fn from_sizes(size: Size<f32>, content_size: Size<f32>) -> Self {
        Self::from_sizes_and_baselines(size, content_size, Point::NONE)
    }

    /// Construct a `LayoutOutput` from just the container's size.
    pub fn from_outer_size(size: Size<f32>) -> Self {
        Self::from_sizes(size, Size::zero())
    }
}

/// The final result of a layout algorithm for a single node.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Layout {
    /// The relative ordering of the node
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// This is effectively a topological sort of each tree.
    pub order: u32,
    /// The top-left corner of the node
    pub location: Point<f32>,
    /// The width and height of the node
    pub size: Size<f32>,
    #[cfg(feature = "content_size")]
    /// The width and height of the content inside the node. This may be larger than the size of the node in the case of
    /// overflowing content and is useful for computing a "scroll width/height" for scrollable nodes
    pub content_size: Size<f32>,
    /// The size of the scrollbars in each dimension. If there is no scrollbar then the size will be zero.
    pub scrollbar_size: Size<f32>,
    /// The size of the borders of the node
    pub border: Rect<f32>,
    /// The size of the padding of the node
    pub padding: Rect<f32>,
    /// The size of the margin of the node
    pub margin: Rect<f32>,
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout {
    /// Creates a new zero-[`Layout`].
    ///
    /// The Zero-layout has size and location set to ZERO.
    /// The `order` value of this layout is set to the minimum value of 0.
    /// This means it should be rendered below all other [`Layout`]s.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            order: 0,
            location: Point::ZERO,
            size: Size::zero(),
            #[cfg(feature = "content_size")]
            content_size: Size::zero(),
            scrollbar_size: Size::zero(),
            border: Rect::zero(),
            padding: Rect::zero(),
            margin: Rect::zero(),
        }
    }

    /// Creates a new zero-[`Layout`] with the supplied `order` value.
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// The Zero-layout has size and location set to ZERO.
    #[must_use]
    pub const fn with_order(order: u32) -> Self {
        Self {
            order,
            size: Size::zero(),
            location: Point::ZERO,
            #[cfg(feature = "content_size")]
            content_size: Size::zero(),
            scrollbar_size: Size::zero(),
            border: Rect::zero(),
            padding: Rect::zero(),
            margin: Rect::zero(),
        }
    }

    /// Get the width of the node's content box
    #[inline]
    pub fn content_box_width(&self) -> f32 {
        self.size.width - self.padding.left - self.padding.right - self.border.left - self.border.right
    }

    /// Get the height of the node's content box
    #[inline]
    pub fn content_box_height(&self) -> f32 {
        self.size.height - self.padding.top - self.padding.bottom - self.border.top - self.border.bottom
    }

    /// Get the size of the node's content box
    #[inline]
    pub fn content_box_size(&self) -> Size<f32> {
        Size { width: self.content_box_width(), height: self.content_box_height() }
    }

    /// Get x offset of the node's content box relative to it's parent's border box
    pub fn content_box_x(&self) -> f32 {
        self.location.x + self.border.left + self.padding.left
    }

    /// Get x offset of the node's content box relative to it's parent's border box
    pub fn content_box_y(&self) -> f32 {
        self.location.y + self.border.top + self.padding.top
    }
}

#[cfg(feature = "content_size")]
impl Layout {
    /// Return the maximum horizontal scroll offset of the node.
    /// This is the content width less the width of the padding box, floored at zero.
    pub fn scroll_width(&self) -> f32 {
        f32_max(
            0.0,
            self.content_size.width + f32_min(self.scrollbar_size.width, self.size.width) - self.size.width
                + self.border.left
                + self.border.right,
        )
    }

    /// Return the maximum vertical scroll offset of the node.
    /// This is the content height less the height of the padding box, floored at zero.
    pub fn scroll_height(&self) -> f32 {
        f32_max(
            0.0,
            self.content_size.height + f32_min(self.scrollbar_size.height, self.size.height) - self.size.height
                + self.border.top
                + self.border.bottom,
        )
    }
}

/// The additional information from layout algorithm
#[cfg(feature = "detailed_layout_info")]
#[derive(Debug, Clone, PartialEq)]
pub enum DetailedLayoutInfo {
    /// Enum variant for [`DetailedGridInfo`](crate::compute::grid::DetailedGridInfo)
    #[cfg(feature = "grid")]
    Grid(Box<crate::compute::grid::DetailedGridInfo>),
    /// For node that hasn't had any detailed information yet
    None,
}
