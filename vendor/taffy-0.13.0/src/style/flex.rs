//! Style types for Flexbox layout
use super::{AlignContent, AlignItems, AlignSelf, CoreStyle, Dimension, JustifyContent, LengthPercentage, Style};
use crate::geometry::Size;

/// The set of styles required for a Flexbox container
pub trait FlexboxContainerStyle: CoreStyle {
    /// Which direction does the main axis flow in?
    #[inline(always)]
    fn flex_direction(&self) -> FlexDirection {
        Style::<Self::CustomIdent>::DEFAULT.flex_direction
    }
    /// Should elements wrap, or stay in a single line?
    #[inline(always)]
    fn flex_wrap(&self) -> FlexWrap {
        Style::<Self::CustomIdent>::DEFAULT.flex_wrap
    }

    /// How large should the gaps between items in a grid or flex container be?
    #[inline(always)]
    fn gap(&self) -> Size<LengthPercentage> {
        Style::<Self::CustomIdent>::DEFAULT.gap
    }

    // Alignment properties

    /// How should content contained within this item be aligned in the cross/block axis
    #[inline(always)]
    fn align_content(&self) -> Option<AlignContent> {
        Style::<Self::CustomIdent>::DEFAULT.align_content
    }
    /// How this node's children aligned in the cross/block axis?
    #[inline(always)]
    fn align_items(&self) -> Option<AlignItems> {
        Style::<Self::CustomIdent>::DEFAULT.align_items
    }
    /// How this node's children should be aligned in the inline axis
    #[inline(always)]
    fn justify_content(&self) -> Option<JustifyContent> {
        Style::<Self::CustomIdent>::DEFAULT.justify_content
    }
}

/// The set of styles required for a Flexbox item (child of a Flexbox container)
pub trait FlexboxItemStyle: CoreStyle {
    /// Sets the initial main axis size of the item
    #[inline(always)]
    fn flex_basis(&self) -> Dimension {
        Style::<Self::CustomIdent>::DEFAULT.flex_basis
    }
    /// The relative rate at which this item grows when it is expanding to fill space
    #[inline(always)]
    fn flex_grow(&self) -> f32 {
        Style::<Self::CustomIdent>::DEFAULT.flex_grow
    }
    /// The relative rate at which this item shrinks when it is contracting to fit into space
    #[inline(always)]
    fn flex_shrink(&self) -> f32 {
        Style::<Self::CustomIdent>::DEFAULT.flex_shrink
    }

    /// How this node should be aligned in the cross/block axis
    /// Falls back to the parents [`AlignItems`] if not set
    #[inline(always)]
    fn align_self(&self) -> Option<AlignSelf> {
        Style::<Self::CustomIdent>::DEFAULT.align_self
    }
}

use crate::geometry::AbsoluteAxis;

/// Controls whether flex items are forced onto one line or can wrap onto multiple lines.
///
/// Defaults to [`FlexWrap::NoWrap`]
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FlexWrap {
    /// Items will not wrap and stay on a single line
    #[default]
    NoWrap,
    /// Items will wrap according to this item's [`FlexDirection`]
    Wrap,
    /// Items will wrap in the opposite direction to this item's [`FlexDirection`]
    WrapReverse,
}

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(FlexWrap,
    "nowrap" => NoWrap,
    "wrap" => Wrap,
    "wrap-reverse" => WrapReverse,
);

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
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

#[cfg(feature = "parse")]
crate::util::parse::impl_parse_for_keyword_enum!(FlexDirection,
    "row" => Row,
    "column" => Column,
    "row-reverse" => RowReverse,
    "column-reverse" => ColumnReverse,
);

impl FlexDirection {
    #[inline]
    /// Is the direction [`FlexDirection::Row`] or [`FlexDirection::RowReverse`]?
    pub(crate) const fn is_row(self) -> bool {
        matches!(self, Self::Row | Self::RowReverse)
    }

    #[inline]
    /// Is the direction [`FlexDirection::Column`] or [`FlexDirection::ColumnReverse`]?
    pub(crate) const fn is_column(self) -> bool {
        matches!(self, Self::Column | Self::ColumnReverse)
    }

    #[inline]
    /// Is the direction [`FlexDirection::RowReverse`] or [`FlexDirection::ColumnReverse`]?
    pub(crate) const fn is_reverse(self) -> bool {
        matches!(self, Self::RowReverse | Self::ColumnReverse)
    }

    #[inline]
    /// The `AbsoluteAxis` that corresponds to the main axis
    pub(crate) const fn main_axis(self) -> AbsoluteAxis {
        match self {
            Self::Row | Self::RowReverse => AbsoluteAxis::Horizontal,
            Self::Column | Self::ColumnReverse => AbsoluteAxis::Vertical,
        }
    }

    #[inline]
    /// The `AbsoluteAxis` that corresponds to the cross axis
    pub(crate) const fn cross_axis(self) -> AbsoluteAxis {
        match self {
            Self::Row | Self::RowReverse => AbsoluteAxis::Vertical,
            Self::Column | Self::ColumnReverse => AbsoluteAxis::Horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_flex_direction {
        use crate::style::*;

        #[test]
        fn flex_direction_is_row() {
            assert!(FlexDirection::Row.is_row());
            assert!(FlexDirection::RowReverse.is_row());
            assert!(!FlexDirection::Column.is_row());
            assert!(!FlexDirection::ColumnReverse.is_row());
        }

        #[test]
        fn flex_direction_is_column() {
            assert!(!FlexDirection::Row.is_column());
            assert!(!FlexDirection::RowReverse.is_column());
            assert!(FlexDirection::Column.is_column());
            assert!(FlexDirection::ColumnReverse.is_column());
        }

        #[test]
        fn flex_direction_is_reverse() {
            assert!(!FlexDirection::Row.is_reverse());
            assert!(FlexDirection::RowReverse.is_reverse());
            assert!(!FlexDirection::Column.is_reverse());
            assert!(FlexDirection::ColumnReverse.is_reverse());
        }
    }
}
