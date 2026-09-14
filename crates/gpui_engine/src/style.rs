//! The engine's layout vocabulary: the layout-affecting style fields and the
//! enums that describe them.
//!
//! These enums are copies of `taffy`'s style enums so this crate can derive
//! `JsonSchema` and stay independent of `taffy` itself.

use gpui_types::{AbsoluteLength, DefiniteLength, Edges, GridLocation, Length, Point, Size};
use refineable::Refineable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The layout-affecting subset of an element's style.
///
/// The facade resolves its `Style` into this before handing it to a layout
/// engine, so the engine never sees paint-only properties.
#[derive(Clone, Debug)]
pub struct EngineLayoutStyle {
    /// The layout algorithm used for this node's children.
    pub display: Display,
    /// How overflowing children affect this node's scrollable area.
    pub overflow: Point<Overflow>,
    /// The positioning strategy for this node.
    pub position: Position,
    /// Offsets applied relative to the layout position.
    pub inset: Edges<Length>,
    /// The preferred size.
    pub size: Size<Length>,
    /// The minimum size.
    pub min_size: Size<Length>,
    /// The maximum size.
    pub max_size: Size<Length>,
    /// The preferred aspect ratio, calculated as width divided by height.
    pub aspect_ratio: Option<f32>,
    /// Space around this node.
    pub margin: Edges<Length>,
    /// Space inside this node.
    pub padding: Edges<DefiniteLength>,
    /// Border widths.
    pub border_widths: Edges<AbsoluteLength>,
    /// How this node's children are aligned in the cross/block axis.
    pub align_items: Option<AlignItems>,
    /// How this node is aligned in the cross/block axis.
    pub align_self: Option<AlignSelf>,
    /// How this node's content is aligned in the cross/block axis.
    pub align_content: Option<AlignContent>,
    /// How this node's content is aligned in the main/inline axis.
    pub justify_content: Option<JustifyContent>,
    /// Space between this node's children.
    pub gap: Size<DefiniteLength>,
    /// Which direction the flex main axis flows in.
    pub flex_direction: FlexDirection,
    /// Whether flex children wrap or stay on a single line.
    pub flex_wrap: FlexWrap,
    /// The initial main-axis size of this node.
    pub flex_basis: Length,
    /// The rate at which this node grows when expanding to fill space.
    pub flex_grow: f32,
    /// The rate at which this node shrinks when contracting.
    pub flex_shrink: f32,
    /// The grid column template.
    pub grid_cols: Option<GridTemplate>,
    /// The grid row template.
    pub grid_rows: Option<GridTemplate>,
    /// This node's location within its parent's grid.
    pub grid_location: Option<GridLocation>,
}

/// The minimum size of a column or row in a grid layout
#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, JsonSchema, Serialize, Deserialize,
)]
pub enum GridTemplateMinSize {
    /// The column or row size may be 0
    #[default]
    Zero,
    /// The column or row size can be determined by the min content
    MinContent,
    /// The column or row size can be determined by the max content
    MaxContent,
}

/// A simplified representation of the grid-template-* value
#[derive(
    Copy,
    Clone,
    Refineable,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Default,
    JsonSchema,
    Serialize,
    Deserialize,
)]
pub struct GridTemplate {
    /// How this template directive should be repeated
    pub repeat: u16,
    /// The minimum size in the repeat(<>, minmax(_, 1fr)) equation
    pub min_size: GridTemplateMinSize,
}

/// Used to control how child nodes are aligned.
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema)]
// Copy of taffy::style type of the same name, to derive JsonSchema.
pub enum AlignItems {
    /// Items are packed toward the start of the axis
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Stretch to fill the container
    Stretch,
}
/// Used to control how child nodes are aligned.
/// Does not apply to Flexbox, and will be ignored if specified on a flex container
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items)
pub type JustifyItems = AlignItems;
/// Used to control how the specified nodes is aligned.
/// Overrides the parent Node's `AlignItems` property.
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-self)
pub type AlignSelf = AlignItems;
/// Used to control how the specified nodes is aligned.
/// Overrides the parent Node's `JustifyItems` property.
/// Does not apply to Flexbox, and will be ignored if specified on a flex child
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self)
pub type JustifySelf = AlignItems;

/// Sets the distribution of space between and around content items
/// For Flexbox it controls alignment in the cross axis
/// For Grid it controls alignment in the block axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/align-content)
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema)]
// Copy of taffy::style type of the same name, to derive JsonSchema.
pub enum AlignContent {
    /// Items are packed toward the start of the axis
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are centered around the middle of the axis
    Center,
    /// Items are stretched to fill the container
    Stretch,
    /// The first and last items are aligned flush with the edges of the container (no gap)
    /// The gap between items is distributed evenly.
    SpaceBetween,
    /// The gap between the first and last items is exactly THE SAME as the gap between items.
    /// The gaps are distributed evenly
    SpaceEvenly,
    /// The gap between the first and last items is exactly HALF the gap between items.
    /// The gaps are distributed evenly in proportion to these ratios.
    SpaceAround,
}

/// Sets the distribution of space between and around content items
/// For Flexbox it controls alignment in the main axis
/// For Grid it controls alignment in the inline axis
///
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)
pub type JustifyContent = AlignContent;

/// Sets the layout used for the children of this node
///
/// The default values depends on on which feature flags are enabled. The order of precedence is: Flex, Grid, Block, None.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, JsonSchema)]
// Copy of taffy::style type of the same name, to derive JsonSchema.
pub enum Display {
    /// The children will follow the block layout algorithm
    Block,
    /// The children will follow the flexbox layout algorithm
    #[default]
    Flex,
    /// The children will follow the CSS Grid layout algorithm
    Grid,
    /// The children will not be laid out, and will follow absolute positioning
    None,
}

/// Controls whether flex items are forced onto one line or can wrap onto multiple lines.
///
/// Defaults to [`FlexWrap::NoWrap`]
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, JsonSchema)]
// Copy of taffy::style type of the same name, to derive JsonSchema.
pub enum FlexWrap {
    /// Items will not wrap and stay on a single line
    #[default]
    NoWrap,
    /// Items will wrap according to this item's [`FlexDirection`]
    Wrap,
    /// Items will wrap in the opposite direction to this item's [`FlexDirection`]
    WrapReverse,
}

/// The direction of the flexbox layout main axis.
///
/// There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
/// Adding items will cause them to be positioned adjacent to each other along the main axis.
/// By varying this value throughout your tree, you can create complex axis-aligned layouts.
///
/// Items are always aligned relative to the cross axis, and justified relative to the main axis.
///
/// The default behavior is [`FlexDirection::Row`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, JsonSchema)]
// Copy of taffy::style type of the same name, to derive JsonSchema.
pub enum FlexDirection {
    /// Defines +x as the main axis
    ///
    /// Items will be added from left to right in a row.
    #[default]
    Row,
    /// Defines +y as the main axis
    ///
    /// Items will be added from top to bottom in a column.
    Column,
    /// Defines -x as the main axis
    ///
    /// Items will be added from right to left in a row.
    RowReverse,
    /// Defines -y as the main axis
    ///
    /// Items will be added from bottom to top in a column.
    ColumnReverse,
}

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
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, JsonSchema)]
// Copy of taffy::style type of the same name, to derive JsonSchema.
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

/// The positioning strategy for this item.
///
/// This controls both how the origin is determined for the [`Style::position`] field,
/// and whether or not the item will be controlled by flexbox's layout algorithm.
///
/// WARNING: this enum follows the behavior of [CSS's `position` property](https://developer.mozilla.org/en-US/docs/Web/CSS/position),
/// which can be unintuitive.
///
/// [`Position::Relative`] is the default value, in contrast to the default behavior in CSS.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, JsonSchema)]
// Copy of taffy::style type of the same name, to derive JsonSchema.
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
