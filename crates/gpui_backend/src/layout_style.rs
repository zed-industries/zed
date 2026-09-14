//! Conversions from the engine's layout vocabulary to `taffy`.
//!
//! These are free functions rather than `From` impls because both the source
//! enums (owned by `gpui_engine`) and the `taffy` enums are foreign to this
//! crate, so an impl is not permitted here.

use gpui_engine::{AlignContent, AlignItems, Display, FlexDirection, FlexWrap, Overflow, Position};

/// Converts an [`AlignItems`] into its `taffy` equivalent.
pub fn to_taffy_align_items(value: AlignItems) -> taffy::style::AlignItems {
    match value {
        AlignItems::Start => taffy::style::AlignItems::START,
        AlignItems::End => taffy::style::AlignItems::END,
        AlignItems::FlexStart => taffy::style::AlignItems::FLEX_START,
        AlignItems::FlexEnd => taffy::style::AlignItems::FLEX_END,
        AlignItems::Center => taffy::style::AlignItems::CENTER,
        AlignItems::Baseline => taffy::style::AlignItems::BASELINE,
        AlignItems::Stretch => taffy::style::AlignItems::STRETCH,
    }
}

/// Converts an [`AlignContent`] into its `taffy` equivalent.
pub fn to_taffy_align_content(value: AlignContent) -> taffy::style::AlignContent {
    match value {
        AlignContent::Start => taffy::style::AlignContent::START,
        AlignContent::End => taffy::style::AlignContent::END,
        AlignContent::FlexStart => taffy::style::AlignContent::FLEX_START,
        AlignContent::FlexEnd => taffy::style::AlignContent::FLEX_END,
        AlignContent::Center => taffy::style::AlignContent::CENTER,
        AlignContent::Stretch => taffy::style::AlignContent::STRETCH,
        AlignContent::SpaceBetween => taffy::style::AlignContent::SPACE_BETWEEN,
        AlignContent::SpaceEvenly => taffy::style::AlignContent::SPACE_EVENLY,
        AlignContent::SpaceAround => taffy::style::AlignContent::SPACE_AROUND,
    }
}

/// Converts a [`Display`] into its `taffy` equivalent.
pub fn to_taffy_display(value: Display) -> taffy::style::Display {
    match value {
        Display::Block => taffy::style::Display::Block,
        Display::Flex => taffy::style::Display::Flex,
        Display::Grid => taffy::style::Display::Grid,
        Display::None => taffy::style::Display::None,
    }
}

/// Converts a [`FlexWrap`] into its `taffy` equivalent.
pub fn to_taffy_flex_wrap(value: FlexWrap) -> taffy::style::FlexWrap {
    match value {
        FlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap,
        FlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
        FlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
    }
}

/// Converts a [`FlexDirection`] into its `taffy` equivalent.
pub fn to_taffy_flex_direction(value: FlexDirection) -> taffy::style::FlexDirection {
    match value {
        FlexDirection::Row => taffy::style::FlexDirection::Row,
        FlexDirection::Column => taffy::style::FlexDirection::Column,
        FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
        FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
    }
}

/// Converts an [`Overflow`] into its `taffy` equivalent.
pub fn to_taffy_overflow(value: Overflow) -> taffy::style::Overflow {
    match value {
        Overflow::Visible => taffy::style::Overflow::Visible,
        Overflow::Clip => taffy::style::Overflow::Clip,
        Overflow::Hidden => taffy::style::Overflow::Hidden,
        Overflow::Scroll => taffy::style::Overflow::Scroll,
    }
}

/// Converts a [`Position`] into its `taffy` equivalent.
pub fn to_taffy_position(value: Position) -> taffy::style::Position {
    match value {
        Position::Relative => taffy::style::Position::Relative,
        Position::Absolute => taffy::style::Position::Absolute,
    }
}
