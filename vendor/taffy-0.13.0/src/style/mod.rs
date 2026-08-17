//! A typed representation of [CSS style properties](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) in Rust. Used as input to layout computation.
mod alignment;
mod available_space;
mod compact_length;
mod dimension;

#[cfg(feature = "block_layout")]
mod block;
#[cfg(feature = "flexbox")]
mod flex;
#[cfg(feature = "float_layout")]
mod float;
#[cfg(feature = "grid")]
mod grid;

pub use self::alignment::{
    AlignContent, AlignContentKeyword, AlignItems, AlignItemsKeyword, AlignSelf, AlignmentSafety, JustifyContent,
    JustifyItems, JustifySelf,
};
pub use self::available_space::AvailableSpace;
pub use self::compact_length::CompactLength;
pub use self::dimension::{Dimension, LengthPercentage, LengthPercentageAuto};
use crate::sys::DefaultCheapStr;

#[cfg(feature = "block_layout")]
pub use self::block::{BlockContainerStyle, BlockItemStyle, TextAlign};
#[cfg(feature = "flexbox")]
pub use self::flex::{FlexDirection, FlexWrap, FlexboxContainerStyle, FlexboxItemStyle};
#[cfg(feature = "float_layout")]
pub use self::float::{Clear, Float, FloatDirection};
#[cfg(feature = "grid")]
pub use self::grid::{
    GenericGridPlacement, GenericGridTemplateComponent, GenericRepetition, GridAutoFlow, GridAutoTracks,
    GridContainerStyle, GridItemStyle, GridPlacement, GridTemplateComponent, GridTemplateRepetition,
    GridTemplateTracks, MaxTrackSizingFunction, MinTrackSizingFunction, RepetitionCount, TrackSizingFunction,
};
#[cfg(feature = "grid")]
pub(crate) use self::grid::{GridAreaAxis, GridAreaEnd};
#[cfg(feature = "grid")]
pub use self::grid::{GridTemplateArea, GridTemplateAreas, NamedGridLine, TemplateLineNames};
#[cfg(feature = "grid")]
pub(crate) use self::grid::{NonNamedGridPlacement, OriginZeroGridPlacement};

use crate::geometry::{Point, Rect, Size};
use crate::style_helpers::TaffyAuto as _;
use core::fmt::Debug;

#[cfg(feature = "grid")]
use crate::geometry::Line;
#[cfg(feature = "serde")]
use crate::style_helpers;
#[cfg(feature = "grid")]
use crate::util::sys::GridTrackVec;

use crate::sys::String;

/// Trait that represents a cheaply clonable string. If you're unsure what to use here
/// consider `Arc<str>` or `string_cache::Atom`.
#[cfg(any(feature = "alloc", feature = "std"))]
pub trait CheapCloneStr:
    AsRef<str> + for<'a> From<&'a str> + From<String> + PartialEq + Eq + Clone + Default + Debug + 'static
{
}
#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> CheapCloneStr for T where
    T: AsRef<str> + for<'a> From<&'a str> + From<String> + PartialEq + Eq + Clone + Default + Debug + 'static
{
}

/// Trait that represents a cheaply clonable string. If you're unsure what to use here
/// consider `Arc<str>` or `string_cache::Atom`.
#[cfg(not(any(feature = "alloc", feature = "std")))]
pub trait CheapCloneStr {}
#[cfg(not(any(feature = "alloc", feature = "std")))]
impl<T> CheapCloneStr for T {}

/// The core set of styles that are shared between all CSS layout nodes
///
/// Note that all methods come with a default implementation which simply returns the default value for that style property
/// but this is a just a convenience to save on boilerplate for styles that your implementation doesn't support. You will need
/// to override the default implementation for each style property that your style type actually supports.
pub trait CoreStyle {
    /// The type of custom identifiers used to identify named grid lines and areas
    type CustomIdent: CheapCloneStr;

    /// Which box generation mode should be used
    #[inline(always)]
    fn box_generation_mode(&self) -> BoxGenerationMode {
        BoxGenerationMode::DEFAULT
    }
    /// Is block layout?
    ///
    /// This should only return `true` for `display: block`, and NOT for `display: flow-root`.
    /// Flow-root boxes establish a new block formatting context and must not be treated as
    /// being part of their parent's block formatting context (which is what this method controls).
    #[inline(always)]
    fn is_block(&self) -> bool {
        false
    }
    /// Is it a compressible replaced element?
    /// <https://drafts.csswg.org/css-sizing-3/#min-content-zero>
    #[inline(always)]
    fn is_compressible_replaced(&self) -> bool {
        false
    }
    /// Which box do size styles apply to
    #[inline(always)]
    fn box_sizing(&self) -> BoxSizing {
        BoxSizing::BorderBox
    }

    /// The direction of text, table and grid columns, and horizontal overflow.
    #[inline(always)]
    fn direction(&self) -> Direction {
        Direction::Ltr
    }

    // Overflow properties
    /// How children overflowing their container should affect layout
    #[inline(always)]
    fn overflow(&self) -> Point<Overflow> {
        Style::<Self::CustomIdent>::DEFAULT.overflow
    }
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    #[inline(always)]
    fn scrollbar_width(&self) -> f32 {
        0.0
    }

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    #[inline(always)]
    fn position(&self) -> Position {
        Style::<Self::CustomIdent>::DEFAULT.position
    }
    /// How should the position of this element be tweaked relative to the layout defined?
    #[inline(always)]
    fn inset(&self) -> Rect<LengthPercentageAuto> {
        Style::<Self::CustomIdent>::DEFAULT.inset
    }

    // Size properies
    /// Sets the initial size of the item
    #[inline(always)]
    fn size(&self) -> Size<Dimension> {
        Style::<Self::CustomIdent>::DEFAULT.size
    }
    /// Controls the minimum size of the item
    #[inline(always)]
    fn min_size(&self) -> Size<Dimension> {
        Style::<Self::CustomIdent>::DEFAULT.min_size
    }
    /// Controls the maximum size of the item
    #[inline(always)]
    fn max_size(&self) -> Size<Dimension> {
        Style::<Self::CustomIdent>::DEFAULT.max_size
    }
    /// Sets the preferred aspect ratio for the item
    /// The ratio is calculated as width divided by height.
    #[inline(always)]
    fn aspect_ratio(&self) -> Option<f32> {
        Style::<Self::CustomIdent>::DEFAULT.aspect_ratio
    }

    // Spacing Properties
    /// How large should the margin be on each side?
    #[inline(always)]
    fn margin(&self) -> Rect<LengthPercentageAuto> {
        Style::<Self::CustomIdent>::DEFAULT.margin
    }
    /// How large should the padding be on each side?
    #[inline(always)]
    fn padding(&self) -> Rect<LengthPercentage> {
        Style::<Self::CustomIdent>::DEFAULT.padding
    }
    /// How large should the border be on each side?
    #[inline(always)]
    fn border(&self) -> Rect<LengthPercentage> {
        Style::<Self::CustomIdent>::DEFAULT.border
    }
}

/// Sets the layout used for the children of this node
///
/// The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Display {
    /// The children will follow the block layout algorithm
    #[cfg(feature = "block_layout")]
    Block,
    /// The children will follow the block layout algorithm and establish a new block formatting context
    #[cfg(feature = "block_layout")]
    FlowRoot,
    /// The children will follow the flexbox layout algorithm
    #[cfg(feature = "flexbox")]
    Flex,
    /// The children will follow the CSS Grid layout algorithm
    #[cfg(feature = "grid")]
    Grid,
    /// The node is hidden, and it's children will also be hidden
    None,
}

impl Display {
    /// The default Display mode
    #[cfg(feature = "flexbox")]
    pub const DEFAULT: Display = Display::Flex;

    /// The default Display mode
    #[cfg(all(feature = "grid", not(feature = "flexbox")))]
    pub const DEFAULT: Display = Display::Grid;

    /// The default Display mode
    #[cfg(all(feature = "block_layout", not(feature = "flexbox"), not(feature = "grid")))]
    pub const DEFAULT: Display = Display::Block;

    /// The default Display mode
    #[cfg(all(not(feature = "flexbox"), not(feature = "grid"), not(feature = "block_layout")))]
    pub const DEFAULT: Display = Display::None;
}

impl Default for Display {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(Display,
    "none" => None,
    #[cfg(feature = "flexbox")]
    "flex" => Flex,
    #[cfg(feature = "grid")]
    "grid" => Grid,
    #[cfg(feature = "block_layout")]
    "block" => Block,
    #[cfg(feature = "block_layout")]
    "flow-root" => FlowRoot,
);

impl core::fmt::Display for Display {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Display::None => write!(f, "NONE"),
            #[cfg(feature = "block_layout")]
            Display::Block => write!(f, "BLOCK"),
            #[cfg(feature = "block_layout")]
            Display::FlowRoot => write!(f, "FLOW-ROOT"),
            #[cfg(feature = "flexbox")]
            Display::Flex => write!(f, "FLEX"),
            #[cfg(feature = "grid")]
            Display::Grid => write!(f, "GRID"),
        }
    }
}

/// An abstracted version of the CSS `display` property where any value other than "none" is represented by "normal"
/// See: <https://www.w3.org/TR/css-display-3/#box-generation>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BoxGenerationMode {
    /// The node generates a box in the regular way
    Normal,
    /// The node and it's descendants generate no boxes (they are hidden)
    None,
}

impl BoxGenerationMode {
    /// The default of BoxGenerationMode
    pub const DEFAULT: BoxGenerationMode = BoxGenerationMode::Normal;
}

impl Default for BoxGenerationMode {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The positioning strategy for this item.
///
/// This controls both how the origin is determined for the [`Style::position`] field,
/// and whether or not the item will be controlled by flexbox's layout algorithm.
///
/// WARNING: this enum follows the behavior of [CSS's `position` property](https://developer.mozilla.org/en-US/docs/Web/CSS/position),
/// which can be unintuitive.
///
/// [`Position::Relative`] is the default value, in contrast to the default behavior in CSS.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Position {
    /// The offset is computed relative to the final position given by the layout algorithm.
    /// Offsets do not affect the position of any other items; they are effectively a correction factor applied at the end.
    #[default]
    Relative,
    /// The offset is computed relative to this item's closest positioned ancestor, if any.
    /// Otherwise, it is placed relative to the origin.
    /// No space is created for the item in the page layout, and its size will not be altered.
    ///
    /// WARNING: to opt-out of layouting entirely, you must use [`Display::None`] instead on your [`Style`] object.
    Absolute,
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(Position,
    "relative" => Relative,
    "absolute" => Absolute,
);

/// Specifies whether size styles for this node are assigned to the node's "content box" or "border box"
///
/// - The "content box" is the node's inner size excluding padding, border and margin
/// - The "border box" is the node's outer size including padding and border (but still excluding margin)
///
/// This property modifies the application of the following styles:
///
///   - `size`
///   - `min_size`
///   - `max_size`
///   - `flex_basis`
///
/// See <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing>
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BoxSizing {
    /// Size styles such size, min_size, max_size specify the box's "border box" (the size excluding margin but including padding/border)
    #[default]
    BorderBox,
    /// Size styles such size, min_size, max_size specify the box's "content box" (the size excluding padding/border/margin)
    ContentBox,
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(BoxSizing,
    "border-box" => BorderBox,
    "content-box" => ContentBox,
);

/// How children overflowing their container should affect layout
///
/// In CSS the primary effect of this property is to control whether contents of a parent container that overflow that container should
/// be displayed anyway, be clipped, or trigger the container to become a scroll container. However it also has secondary effects on layout,
/// the main ones being:
///
///   - The automatic minimum size Flexbox/CSS Grid items with non-`Visible` overflow is `0` rather than being content based
///   - `Overflow::Scroll` nodes have space in the layout reserved for a scrollbar (width controlled by the `scrollbar_width` property)
///
/// In Taffy, we only implement the layout related secondary effects as we are not concerned with drawing/painting. The amount of space reserved for
/// a scrollbar is controlled by the `scrollbar_width` property. If this is `0` then `Scroll` behaves identically to `Hidden`.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Overflow {
    /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
    /// Content that overflows this node *should* contribute to the scroll region of its parent.
    #[default]
    Visible,
    /// The automatic minimum size of this node as a flexbox/grid item should be based on the size of its content.
    /// Content that overflows this node should *not* contribute to the scroll region of its parent.
    Clip,
    /// The automatic minimum size of this node as a flexbox/grid item should be `0`.
    /// Content that overflows this node should *not* contribute to the scroll region of its parent.
    Hidden,
    /// The automatic minimum size of this node as a flexbox/grid item should be `0`. Additionally, space should be reserved
    /// for a scrollbar. The amount of space reserved is controlled by the `scrollbar_width` property.
    /// Content that overflows this node should *not* contribute to the scroll region of its parent.
    Scroll,
}

impl Overflow {
    /// Returns true for overflow modes that contain their contents (`Overflow::Hidden`, `Overflow::Scroll`, `Overflow::Auto`)
    /// or else false for overflow modes that allow their contains to spill (`Overflow::Visible`).
    #[inline(always)]
    pub fn is_scroll_container(self) -> bool {
        match self {
            Self::Visible | Self::Clip => false,
            Self::Hidden | Self::Scroll => true,
        }
    }

    /// Returns `Some(0.0)` if the overflow mode would cause the automatic minimum size of a Flexbox or CSS Grid item
    /// to be `0`. Else returns None.
    #[inline(always)]
    pub(crate) fn maybe_into_automatic_min_size(self) -> Option<f32> {
        match self.is_scroll_container() {
            true => Some(0.0),
            false => None,
        }
    }
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(Overflow,
    "visible" => Visible,
    "hidden" => Hidden,
    "clip" => Clip,
    "scroll" => Scroll,
);

/// Sets the direction of text, table and grid columns, and horizontal overflow.
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/direction>
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    #[default]
    /// Left-to-right
    Ltr,
    /// Right-to-left
    Rtl,
}

impl Direction {
    /// Returns true if the direction is right-to-left
    #[inline]
    pub(crate) fn is_rtl(&self) -> bool {
        matches!(self, Direction::Rtl)
    }
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(Direction,
    "ltr" => Ltr,
    "rtl" => Rtl,
);

/// A typed representation of the CSS style information for a single node.
///
/// The most important idea in flexbox is the notion of a "main" and "cross" axis, which are always perpendicular to each other.
/// The orientation of these axes are controlled via the [`FlexDirection`] field of this struct.
///
/// This struct follows the [CSS equivalent](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Basic_Concepts_of_Flexbox) directly;
/// information about the behavior on the web should transfer directly.
///
/// Detailed information about the exact behavior of each of these fields
/// can be found on [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS) by searching for the field name.
/// The distinction between margin, padding and border is explained well in
/// this [introduction to the box model](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Model/Introduction_to_the_CSS_box_model).
///
/// If the behavior does not match the flexbox layout algorithm on the web, please file a bug!
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Style<S: CheapCloneStr = DefaultCheapStr> {
    /// This is a dummy field which is necessary to make Taffy compile with the `grid` feature disabled
    /// It should always be set to `core::marker::PhantomData`.
    pub dummy: core::marker::PhantomData<S>,
    /// What layout strategy should be used?
    pub display: Display,
    /// Whether a child is display:table or not. This affects children of block layouts.
    /// This should really be part of `Display`, but it is currently seperate because table layout isn't implemented
    pub item_is_table: bool,
    /// Is it a replaced element like an image or form field?
    /// <https://drafts.csswg.org/css-sizing-3/#min-content-zero>
    pub item_is_replaced: bool,
    /// Should size styles apply to the content box or the border box of the node
    pub box_sizing: BoxSizing,
    /// Sets the direction of text, table and grid columns, and horizontal overflow.
    pub direction: Direction,

    // Overflow properties
    /// How children overflowing their container should affect layout
    pub overflow: Point<Overflow>,
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    pub scrollbar_width: f32,

    #[cfg(feature = "float_layout")]
    /// Should the box be floated
    pub float: Float,
    #[cfg(feature = "float_layout")]
    /// Should the box clear floats
    pub clear: Clear,

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    pub position: Position,
    /// How should the position of this element be tweaked relative to the layout defined?
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::auto"))]
    pub inset: Rect<LengthPercentageAuto>,

    // Size properties
    /// Sets the initial size of the item
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::auto"))]
    pub size: Size<Dimension>,
    /// Controls the minimum size of the item
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::auto"))]
    pub min_size: Size<Dimension>,
    /// Controls the maximum size of the item
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::auto"))]
    pub max_size: Size<Dimension>,
    /// Sets the preferred aspect ratio for the item
    ///
    /// The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,

    // Spacing Properties
    /// How large should the margin be on each side?
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::zero"))]
    pub margin: Rect<LengthPercentageAuto>,
    /// How large should the padding be on each side?
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::zero"))]
    pub padding: Rect<LengthPercentage>,
    /// How large should the border be on each side?
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::zero"))]
    pub border: Rect<LengthPercentage>,

    // Alignment properties
    /// How this node's children aligned in the cross/block axis?
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    pub align_items: Option<AlignItems>,
    /// How this node should be aligned in the cross/block axis
    /// Falls back to the parents [`AlignItems`] if not set
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    pub align_self: Option<AlignSelf>,
    /// How this node's children should be aligned in the inline axis
    #[cfg(feature = "grid")]
    pub justify_items: Option<AlignItems>,
    /// How this node should be aligned in the inline axis
    /// Falls back to the parents [`JustifyItems`] if not set
    #[cfg(feature = "grid")]
    pub justify_self: Option<AlignSelf>,
    /// How should content contained within this item be aligned in the cross/block axis
    #[cfg(any(feature = "flexbox", feature = "grid", feature = "block_layout"))]
    pub align_content: Option<AlignContent>,
    /// How should content contained within this item be aligned in the main/inline axis
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    pub justify_content: Option<JustifyContent>,
    /// How large should the gaps between items in a grid or flex container be?
    #[cfg(any(feature = "flexbox", feature = "grid"))]
    #[cfg_attr(feature = "serde", serde(default = "style_helpers::zero"))]
    pub gap: Size<LengthPercentage>,

    // Block container properties
    /// How items elements should aligned in the inline axis
    #[cfg(feature = "block_layout")]
    pub text_align: TextAlign,

    // Flexbox container properties
    /// Which direction does the main axis flow in?
    #[cfg(feature = "flexbox")]
    pub flex_direction: FlexDirection,
    /// Should elements wrap, or stay in a single line?
    #[cfg(feature = "flexbox")]
    pub flex_wrap: FlexWrap,

    // Flexbox item properties
    /// Sets the initial main axis size of the item
    #[cfg(feature = "flexbox")]
    pub flex_basis: Dimension,
    /// The relative rate at which this item grows when it is expanding to fill space
    ///
    /// 0.0 is the default value, and this value must be positive.
    #[cfg(feature = "flexbox")]
    pub flex_grow: f32,
    /// The relative rate at which this item shrinks when it is contracting to fit into space
    ///
    /// 1.0 is the default value, and this value must be positive.
    #[cfg(feature = "flexbox")]
    pub flex_shrink: f32,

    // Grid container properies
    /// Defines the track sizing functions (heights) of the grid rows
    #[cfg(feature = "grid")]
    pub grid_template_rows: GridTrackVec<GridTemplateComponent<S>>,
    /// Defines the track sizing functions (widths) of the grid columns
    #[cfg(feature = "grid")]
    pub grid_template_columns: GridTrackVec<GridTemplateComponent<S>>,
    /// Defines the size of implicitly created rows
    #[cfg(feature = "grid")]
    pub grid_auto_rows: GridTrackVec<TrackSizingFunction>,
    /// Defined the size of implicitly created columns
    #[cfg(feature = "grid")]
    pub grid_auto_columns: GridTrackVec<TrackSizingFunction>,
    /// Controls how items get placed into the grid for auto-placed items
    #[cfg(feature = "grid")]
    pub grid_auto_flow: GridAutoFlow,

    // Grid container named properties
    /// Defines the rectangular grid areas
    #[cfg(feature = "grid")]
    pub grid_template_areas: Option<GridTemplateAreas<S>>,
    /// The named lines between the columns
    #[cfg(feature = "grid")]
    pub grid_template_column_names: GridTrackVec<GridTrackVec<S>>,
    /// The named lines between the rows
    #[cfg(feature = "grid")]
    pub grid_template_row_names: GridTrackVec<GridTrackVec<S>>,

    // Grid child properties
    /// Defines which row in the grid the item should start and end at
    #[cfg(feature = "grid")]
    pub grid_row: Line<GridPlacement<S>>,
    /// Defines which column in the grid the item should start and end at
    #[cfg(feature = "grid")]
    pub grid_column: Line<GridPlacement<S>>,
}

impl<S: CheapCloneStr> Style<S> {
    /// The [`Default`] layout, in a form that can be used in const functions
    pub const DEFAULT: Style<S> = Style {
        dummy: core::marker::PhantomData,
        display: Display::DEFAULT,
        item_is_table: false,
        item_is_replaced: false,
        box_sizing: BoxSizing::BorderBox,
        direction: Direction::Ltr,
        overflow: Point { x: Overflow::Visible, y: Overflow::Visible },
        scrollbar_width: 0.0,
        #[cfg(feature = "float_layout")]
        float: Float::None,
        #[cfg(feature = "float_layout")]
        clear: Clear::None,
        position: Position::Relative,
        inset: Rect::auto(),
        margin: Rect::zero(),
        padding: Rect::zero(),
        border: Rect::zero(),
        size: Size::auto(),
        min_size: Size::auto(),
        max_size: Size::auto(),
        aspect_ratio: None,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        gap: Size::zero(),
        // Alignment
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        align_items: None,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        align_self: None,
        #[cfg(feature = "grid")]
        justify_items: None,
        #[cfg(feature = "grid")]
        justify_self: None,
        #[cfg(any(feature = "flexbox", feature = "grid", feature = "block_layout"))]
        align_content: None,
        #[cfg(any(feature = "flexbox", feature = "grid"))]
        justify_content: None,
        // Block
        #[cfg(feature = "block_layout")]
        text_align: TextAlign::Auto,
        // Flexbox
        #[cfg(feature = "flexbox")]
        flex_direction: FlexDirection::Row,
        #[cfg(feature = "flexbox")]
        flex_wrap: FlexWrap::NoWrap,
        #[cfg(feature = "flexbox")]
        flex_grow: 0.0,
        #[cfg(feature = "flexbox")]
        flex_shrink: 1.0,
        #[cfg(feature = "flexbox")]
        flex_basis: Dimension::AUTO,
        // Grid
        #[cfg(feature = "grid")]
        grid_template_rows: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_template_columns: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_template_areas: None,
        #[cfg(feature = "grid")]
        grid_template_column_names: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_template_row_names: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_auto_rows: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_auto_columns: GridTrackVec::new(),
        #[cfg(feature = "grid")]
        grid_auto_flow: GridAutoFlow::Row,
        #[cfg(feature = "grid")]
        grid_row: Line { start: GridPlacement::<S>::Auto, end: GridPlacement::<S>::Auto },
        #[cfg(feature = "grid")]
        grid_column: Line { start: GridPlacement::<S>::Auto, end: GridPlacement::<S>::Auto },
    };
}

impl<S: CheapCloneStr> Default for Style<S> {
    fn default() -> Self {
        Style::DEFAULT
    }
}

impl<S: CheapCloneStr> CoreStyle for Style<S> {
    type CustomIdent = S;

    #[inline(always)]
    fn box_generation_mode(&self) -> BoxGenerationMode {
        match self.display {
            Display::None => BoxGenerationMode::None,
            _ => BoxGenerationMode::Normal,
        }
    }
    #[inline(always)]
    #[cfg(feature = "block_layout")]
    fn is_block(&self) -> bool {
        matches!(self.display, Display::Block)
    }
    #[inline(always)]
    fn is_compressible_replaced(&self) -> bool {
        self.item_is_replaced
    }
    #[inline(always)]
    fn box_sizing(&self) -> BoxSizing {
        self.box_sizing
    }
    #[inline(always)]
    fn direction(&self) -> Direction {
        self.direction
    }
    #[inline(always)]
    fn overflow(&self) -> Point<Overflow> {
        self.overflow
    }
    #[inline(always)]
    fn scrollbar_width(&self) -> f32 {
        self.scrollbar_width
    }
    #[inline(always)]
    fn position(&self) -> Position {
        self.position
    }
    #[inline(always)]
    fn inset(&self) -> Rect<LengthPercentageAuto> {
        self.inset
    }
    #[inline(always)]
    fn size(&self) -> Size<Dimension> {
        self.size
    }
    #[inline(always)]
    fn min_size(&self) -> Size<Dimension> {
        self.min_size
    }
    #[inline(always)]
    fn max_size(&self) -> Size<Dimension> {
        self.max_size
    }
    #[inline(always)]
    fn aspect_ratio(&self) -> Option<f32> {
        self.aspect_ratio
    }
    #[inline(always)]
    fn margin(&self) -> Rect<LengthPercentageAuto> {
        self.margin
    }
    #[inline(always)]
    fn padding(&self) -> Rect<LengthPercentage> {
        self.padding
    }
    #[inline(always)]
    fn border(&self) -> Rect<LengthPercentage> {
        self.border
    }
}

impl<T: CoreStyle> CoreStyle for &'_ T {
    type CustomIdent = T::CustomIdent;

    #[inline(always)]
    fn box_generation_mode(&self) -> BoxGenerationMode {
        (*self).box_generation_mode()
    }
    #[inline(always)]
    fn is_block(&self) -> bool {
        (*self).is_block()
    }
    #[inline(always)]
    fn is_compressible_replaced(&self) -> bool {
        (*self).is_compressible_replaced()
    }
    #[inline(always)]
    fn box_sizing(&self) -> BoxSizing {
        (*self).box_sizing()
    }
    #[inline(always)]
    fn direction(&self) -> Direction {
        (*self).direction()
    }
    #[inline(always)]
    fn overflow(&self) -> Point<Overflow> {
        (*self).overflow()
    }
    #[inline(always)]
    fn scrollbar_width(&self) -> f32 {
        (*self).scrollbar_width()
    }
    #[inline(always)]
    fn position(&self) -> Position {
        (*self).position()
    }
    #[inline(always)]
    fn inset(&self) -> Rect<LengthPercentageAuto> {
        (*self).inset()
    }
    #[inline(always)]
    fn size(&self) -> Size<Dimension> {
        (*self).size()
    }
    #[inline(always)]
    fn min_size(&self) -> Size<Dimension> {
        (*self).min_size()
    }
    #[inline(always)]
    fn max_size(&self) -> Size<Dimension> {
        (*self).max_size()
    }
    #[inline(always)]
    fn aspect_ratio(&self) -> Option<f32> {
        (*self).aspect_ratio()
    }
    #[inline(always)]
    fn margin(&self) -> Rect<LengthPercentageAuto> {
        (*self).margin()
    }
    #[inline(always)]
    fn padding(&self) -> Rect<LengthPercentage> {
        (*self).padding()
    }
    #[inline(always)]
    fn border(&self) -> Rect<LengthPercentage> {
        (*self).border()
    }
}

#[cfg(feature = "block_layout")]
impl<S: CheapCloneStr> BlockContainerStyle for Style<S> {
    #[inline(always)]
    fn text_align(&self) -> TextAlign {
        self.text_align
    }

    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        self.align_content
    }
}

#[cfg(feature = "block_layout")]
impl<T: BlockContainerStyle> BlockContainerStyle for &'_ T {
    #[inline(always)]
    fn text_align(&self) -> TextAlign {
        (*self).text_align()
    }

    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        (*self).align_content()
    }
}

#[cfg(feature = "block_layout")]
impl<S: CheapCloneStr> BlockItemStyle for Style<S> {
    #[inline(always)]
    fn is_table(&self) -> bool {
        self.item_is_table
    }

    #[cfg(feature = "float_layout")]
    #[inline(always)]
    fn float(&self) -> Float {
        self.float
    }

    #[cfg(feature = "float_layout")]
    #[inline(always)]
    fn clear(&self) -> Clear {
        self.clear
    }
}

#[cfg(feature = "block_layout")]
impl<T: BlockItemStyle> BlockItemStyle for &'_ T {
    #[inline(always)]
    fn is_table(&self) -> bool {
        (*self).is_table()
    }

    #[cfg(feature = "float_layout")]
    #[inline(always)]
    fn float(&self) -> Float {
        (*self).float()
    }

    #[cfg(feature = "float_layout")]
    #[inline(always)]
    fn clear(&self) -> Clear {
        (*self).clear()
    }
}

#[cfg(feature = "flexbox")]
impl<S: CheapCloneStr> FlexboxContainerStyle for Style<S> {
    #[inline(always)]
    fn flex_direction(&self) -> FlexDirection {
        self.flex_direction
    }
    #[inline(always)]
    fn flex_wrap(&self) -> FlexWrap {
        self.flex_wrap
    }
    #[inline(always)]
    fn gap(&self) -> Size<LengthPercentage> {
        self.gap
    }
    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        self.align_content
    }
    #[inline(always)]
    fn align_items(&self) -> Option<AlignItems> {
        self.align_items
    }
    #[inline(always)]
    fn justify_content(&self) -> Option<JustifyContent> {
        self.justify_content
    }
}

#[cfg(feature = "flexbox")]
impl<T: FlexboxContainerStyle> FlexboxContainerStyle for &'_ T {
    #[inline(always)]
    fn flex_direction(&self) -> FlexDirection {
        (*self).flex_direction()
    }
    #[inline(always)]
    fn flex_wrap(&self) -> FlexWrap {
        (*self).flex_wrap()
    }
    #[inline(always)]
    fn gap(&self) -> Size<LengthPercentage> {
        (*self).gap()
    }
    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        (*self).align_content()
    }
    #[inline(always)]
    fn align_items(&self) -> Option<AlignItems> {
        (*self).align_items()
    }
    #[inline(always)]
    fn justify_content(&self) -> Option<JustifyContent> {
        (*self).justify_content()
    }
}

#[cfg(feature = "flexbox")]
impl<S: CheapCloneStr> FlexboxItemStyle for Style<S> {
    #[inline(always)]
    fn flex_basis(&self) -> Dimension {
        self.flex_basis
    }
    #[inline(always)]
    fn flex_grow(&self) -> f32 {
        self.flex_grow
    }
    #[inline(always)]
    fn flex_shrink(&self) -> f32 {
        self.flex_shrink
    }
    #[inline(always)]
    fn align_self(&self) -> Option<AlignSelf> {
        self.align_self
    }
}

#[cfg(feature = "flexbox")]
impl<T: FlexboxItemStyle> FlexboxItemStyle for &'_ T {
    #[inline(always)]
    fn flex_basis(&self) -> Dimension {
        (*self).flex_basis()
    }
    #[inline(always)]
    fn flex_grow(&self) -> f32 {
        (*self).flex_grow()
    }
    #[inline(always)]
    fn flex_shrink(&self) -> f32 {
        (*self).flex_shrink()
    }
    #[inline(always)]
    fn align_self(&self) -> Option<AlignSelf> {
        (*self).align_self()
    }
}

#[cfg(feature = "grid")]
impl<S: CheapCloneStr> GridContainerStyle for Style<S> {
    type Repetition<'a>
        = &'a GridTemplateRepetition<S>
    where
        Self: 'a;

    type TemplateTrackList<'a>
        = core::iter::Map<
        core::slice::Iter<'a, GridTemplateComponent<S>>,
        fn(&'a GridTemplateComponent<S>) -> GenericGridTemplateComponent<S, &'a GridTemplateRepetition<S>>,
    >
    where
        Self: 'a;

    type AutoTrackList<'a>
        = core::iter::Copied<core::slice::Iter<'a, TrackSizingFunction>>
    where
        Self: 'a;

    #[cfg(feature = "grid")]
    type TemplateLineNames<'a>
        = core::iter::Map<core::slice::Iter<'a, GridTrackVec<S>>, fn(&GridTrackVec<S>) -> core::slice::Iter<'_, S>>
    where
        Self: 'a;
    #[cfg(feature = "grid")]
    type GridTemplateAreas<'a>
        = core::iter::Cloned<core::slice::Iter<'a, GridTemplateArea<S>>>
    where
        Self: 'a;

    #[inline(always)]
    fn grid_template_rows(&self) -> Option<Self::TemplateTrackList<'_>> {
        Some(self.grid_template_rows.iter().map(|c| c.as_component_ref()))
    }
    #[inline(always)]
    fn grid_template_columns(&self) -> Option<Self::TemplateTrackList<'_>> {
        Some(self.grid_template_columns.iter().map(|c| c.as_component_ref()))
    }
    #[inline(always)]
    fn grid_auto_rows(&self) -> Self::AutoTrackList<'_> {
        self.grid_auto_rows.iter().copied()
    }
    #[inline(always)]
    fn grid_auto_columns(&self) -> Self::AutoTrackList<'_> {
        self.grid_auto_columns.iter().copied()
    }
    #[inline(always)]
    fn grid_auto_flow(&self) -> GridAutoFlow {
        self.grid_auto_flow
    }
    #[inline(always)]
    fn gap(&self) -> Size<LengthPercentage> {
        self.gap
    }
    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        self.align_content
    }
    #[inline(always)]
    fn justify_content(&self) -> Option<JustifyContent> {
        self.justify_content
    }
    #[inline(always)]
    fn align_items(&self) -> Option<AlignItems> {
        self.align_items
    }
    #[inline(always)]
    fn justify_items(&self) -> Option<AlignItems> {
        self.justify_items
    }

    #[inline(always)]
    #[cfg(feature = "grid")]
    fn grid_template_areas(&self) -> Option<Self::GridTemplateAreas<'_>> {
        self.grid_template_areas.as_ref().map(|template| template.areas.iter().cloned())
    }
    #[inline(always)]
    #[cfg(feature = "grid")]
    fn grid_template_area_row_count(&self) -> u16 {
        self.grid_template_areas.as_ref().map(|template| template.row_count).unwrap_or(0)
    }
    #[inline(always)]
    #[cfg(feature = "grid")]
    fn grid_template_area_column_count(&self) -> u16 {
        self.grid_template_areas.as_ref().map(|template| template.column_count).unwrap_or(0)
    }

    #[inline(always)]
    #[cfg(feature = "grid")]
    fn grid_template_column_names(&self) -> Option<Self::TemplateLineNames<'_>> {
        Some(self.grid_template_column_names.iter().map(|names| names.iter()))
    }

    #[inline(always)]
    #[cfg(feature = "grid")]
    fn grid_template_row_names(&self) -> Option<Self::TemplateLineNames<'_>> {
        Some(self.grid_template_row_names.iter().map(|names| names.iter()))
    }
}

#[cfg(feature = "grid")]
impl<T: GridContainerStyle> GridContainerStyle for &'_ T {
    type Repetition<'a>
        = T::Repetition<'a>
    where
        Self: 'a;

    type TemplateTrackList<'a>
        = T::TemplateTrackList<'a>
    where
        Self: 'a;

    type AutoTrackList<'a>
        = T::AutoTrackList<'a>
    where
        Self: 'a;

    /// The type returned by grid_template_row_names and grid_template_column_names
    #[cfg(feature = "grid")]
    type TemplateLineNames<'a>
        = T::TemplateLineNames<'a>
    where
        Self: 'a;
    #[cfg(feature = "grid")]
    type GridTemplateAreas<'a>
        = T::GridTemplateAreas<'a>
    where
        Self: 'a;

    #[inline(always)]
    fn grid_template_rows(&self) -> Option<Self::TemplateTrackList<'_>> {
        (*self).grid_template_rows()
    }
    #[inline(always)]
    fn grid_template_columns(&self) -> Option<Self::TemplateTrackList<'_>> {
        (*self).grid_template_columns()
    }
    #[inline(always)]
    fn grid_auto_rows(&self) -> Self::AutoTrackList<'_> {
        (*self).grid_auto_rows()
    }
    #[inline(always)]
    fn grid_auto_columns(&self) -> Self::AutoTrackList<'_> {
        (*self).grid_auto_columns()
    }
    #[cfg(feature = "grid")]
    #[inline(always)]
    fn grid_template_areas(&self) -> Option<Self::GridTemplateAreas<'_>> {
        (*self).grid_template_areas()
    }
    #[inline(always)]
    fn grid_template_area_row_count(&self) -> u16 {
        (*self).grid_template_area_row_count()
    }
    #[inline(always)]
    fn grid_template_area_column_count(&self) -> u16 {
        (*self).grid_template_area_column_count()
    }
    #[cfg(feature = "grid")]
    #[inline(always)]
    fn grid_template_column_names(&self) -> Option<Self::TemplateLineNames<'_>> {
        (*self).grid_template_column_names()
    }
    #[cfg(feature = "grid")]
    #[inline(always)]
    fn grid_template_row_names(&self) -> Option<Self::TemplateLineNames<'_>> {
        (*self).grid_template_row_names()
    }
    #[inline(always)]
    fn grid_auto_flow(&self) -> GridAutoFlow {
        (*self).grid_auto_flow()
    }
    #[inline(always)]
    fn gap(&self) -> Size<LengthPercentage> {
        (*self).gap()
    }
    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        (*self).align_content()
    }
    #[inline(always)]
    fn justify_content(&self) -> Option<JustifyContent> {
        (*self).justify_content()
    }
    #[inline(always)]
    fn align_items(&self) -> Option<AlignItems> {
        (*self).align_items()
    }
    #[inline(always)]
    fn justify_items(&self) -> Option<AlignItems> {
        (*self).justify_items()
    }
}

#[cfg(feature = "grid")]
impl<S: CheapCloneStr> GridItemStyle for Style<S> {
    #[inline(always)]
    fn grid_row(&self) -> Line<GridPlacement<S>> {
        // TODO: Investigate eliminating clone
        self.grid_row.clone()
    }
    #[inline(always)]
    fn grid_column(&self) -> Line<GridPlacement<S>> {
        // TODO: Investigate eliminating clone
        self.grid_column.clone()
    }
    #[inline(always)]
    fn align_self(&self) -> Option<AlignSelf> {
        self.align_self
    }
    #[inline(always)]
    fn justify_self(&self) -> Option<AlignSelf> {
        self.justify_self
    }
}

#[cfg(feature = "grid")]
impl<T: GridItemStyle> GridItemStyle for &'_ T {
    #[inline(always)]
    fn grid_row(&self) -> Line<GridPlacement<Self::CustomIdent>> {
        (*self).grid_row()
    }
    #[inline(always)]
    fn grid_column(&self) -> Line<GridPlacement<Self::CustomIdent>> {
        (*self).grid_column()
    }
    #[inline(always)]
    fn align_self(&self) -> Option<AlignSelf> {
        (*self).align_self()
    }
    #[inline(always)]
    fn justify_self(&self) -> Option<AlignSelf> {
        (*self).justify_self()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::Style;
    use crate::sys::DefaultCheapStr;
    use crate::{geometry::*, style_helpers::TaffyAuto as _};

    #[test]
    fn defaults_match() {
        #[cfg(feature = "grid")]
        use super::GridPlacement;

        let old_defaults: Style<DefaultCheapStr> = Style {
            dummy: core::marker::PhantomData,
            display: Default::default(),
            item_is_table: false,
            item_is_replaced: false,
            box_sizing: Default::default(),
            #[cfg(feature = "float_layout")]
            float: Default::default(),
            #[cfg(feature = "float_layout")]
            clear: Default::default(),
            direction: Default::default(),
            overflow: Default::default(),
            scrollbar_width: 0.0,
            position: Default::default(),
            #[cfg(feature = "flexbox")]
            flex_direction: Default::default(),
            #[cfg(feature = "flexbox")]
            flex_wrap: Default::default(),
            #[cfg(any(feature = "flexbox", feature = "grid"))]
            align_items: Default::default(),
            #[cfg(any(feature = "flexbox", feature = "grid"))]
            align_self: Default::default(),
            #[cfg(feature = "grid")]
            justify_items: Default::default(),
            #[cfg(feature = "grid")]
            justify_self: Default::default(),
            #[cfg(any(feature = "flexbox", feature = "grid", feature = "block_layout"))]
            align_content: Default::default(),
            #[cfg(any(feature = "flexbox", feature = "grid"))]
            justify_content: Default::default(),
            inset: Rect::auto(),
            margin: Rect::zero(),
            padding: Rect::zero(),
            border: Rect::zero(),
            gap: Size::zero(),
            #[cfg(feature = "block_layout")]
            text_align: Default::default(),
            #[cfg(feature = "flexbox")]
            flex_grow: 0.0,
            #[cfg(feature = "flexbox")]
            flex_shrink: 1.0,
            #[cfg(feature = "flexbox")]
            flex_basis: super::Dimension::AUTO,
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: Default::default(),
            #[cfg(feature = "grid")]
            grid_template_rows: Default::default(),
            #[cfg(feature = "grid")]
            grid_template_columns: Default::default(),
            #[cfg(feature = "grid")]
            grid_template_row_names: Default::default(),
            #[cfg(feature = "grid")]
            grid_template_column_names: Default::default(),
            #[cfg(feature = "grid")]
            grid_template_areas: Default::default(),
            #[cfg(feature = "grid")]
            grid_auto_rows: Default::default(),
            #[cfg(feature = "grid")]
            grid_auto_columns: Default::default(),
            #[cfg(feature = "grid")]
            grid_auto_flow: Default::default(),
            #[cfg(feature = "grid")]
            grid_row: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
            #[cfg(feature = "grid")]
            grid_column: Line { start: GridPlacement::Auto, end: GridPlacement::Auto },
        };

        assert_eq!(Style::DEFAULT, Style::<DefaultCheapStr>::default());
        assert_eq!(Style::DEFAULT, old_defaults);
    }

    // NOTE: Please feel free the update the sizes in this test as required. This test is here to prevent unintentional size changes
    // and to serve as accurate up-to-date documentation on the sizes.
    #[test]
    fn style_sizes() {
        use super::*;
        type S = crate::sys::DefaultCheapStr;

        fn assert_type_size<T>(expected_size: usize) {
            let name = ::core::any::type_name::<T>();
            let name = name.replace("taffy::geometry::", "");
            let name = name.replace("taffy::style::dimension::", "");
            let name = name.replace("taffy::style::alignment::", "");
            let name = name.replace("taffy::style::flex::", "");
            let name = name.replace("taffy::style::grid::", "");

            assert_eq!(
                ::core::mem::size_of::<T>(),
                expected_size,
                "Expected {} for be {} byte(s) but it was {} byte(s)",
                name,
                expected_size,
                ::core::mem::size_of::<T>(),
            );
        }

        // Display and Position
        assert_type_size::<Display>(1);
        assert_type_size::<BoxSizing>(1);
        assert_type_size::<Position>(1);
        assert_type_size::<Overflow>(1);

        // Dimensions and aggregations of Dimensions
        assert_type_size::<f32>(4);
        assert_type_size::<LengthPercentage>(8);
        assert_type_size::<LengthPercentageAuto>(8);
        assert_type_size::<Dimension>(8);
        assert_type_size::<Size<LengthPercentage>>(16);
        assert_type_size::<Size<LengthPercentageAuto>>(16);
        assert_type_size::<Size<Dimension>>(16);
        assert_type_size::<Rect<LengthPercentage>>(32);
        assert_type_size::<Rect<LengthPercentageAuto>>(32);
        assert_type_size::<Rect<Dimension>>(32);

        // Alignment — `AlignContent` and `AlignItems` are structs of two `#[repr(u8)]` enums
        // (position keyword + safety modifier). Niche-packing in the safety byte (only 2 of
        // 256 values used) lets `Option<_>` stay the same size as the bare struct.
        assert_type_size::<AlignContentKeyword>(1);
        assert_type_size::<AlignItemsKeyword>(1);
        assert_type_size::<AlignmentSafety>(1);
        assert_type_size::<AlignContent>(2);
        assert_type_size::<AlignItems>(2);
        assert_type_size::<Option<AlignItems>>(2);
        assert_type_size::<Option<AlignContent>>(2);

        // Flexbox Container
        assert_type_size::<FlexDirection>(1);
        assert_type_size::<FlexWrap>(1);

        // CSS Grid Container
        assert_type_size::<GridAutoFlow>(1);
        assert_type_size::<MinTrackSizingFunction>(8);
        assert_type_size::<MaxTrackSizingFunction>(8);
        assert_type_size::<TrackSizingFunction>(16);
        assert_type_size::<Vec<TrackSizingFunction>>(24);
        assert_type_size::<Vec<GridTemplateComponent<S>>>(24);

        // String-type dependent (String)
        assert_type_size::<GridTemplateComponent<String>>(56);
        assert_type_size::<GridPlacement<String>>(32);
        assert_type_size::<Line<GridPlacement<String>>>(64);
        assert_type_size::<Style<String>>(552);

        // String-type dependent (Arc<str>)
        assert_type_size::<GridTemplateComponent<Arc<str>>>(56);
        assert_type_size::<GridPlacement<Arc<str>>>(24);
        assert_type_size::<Line<GridPlacement<Arc<str>>>>(48);
        assert_type_size::<Style<Arc<str>>>(520);
    }
}
