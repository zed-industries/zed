//! Conversion from GPUI's `Style` to the engine's layout style.
//!
//! Only layout-affecting fields cross the boundary; paint-only properties stay
//! with the authoring layer.

use crate::{EngineLayoutStyle, Style};

/// Projects a [`Style`] onto the engine's [`EngineLayoutStyle`].
pub(crate) fn to_engine_layout_style(style: &Style) -> EngineLayoutStyle {
    EngineLayoutStyle {
        display: style.display,
        overflow: style.overflow,
        scrollbar_width: style.scrollbar_width,
        position: style.position,
        inset: style.inset,
        size: style.size,
        min_size: style.min_size,
        max_size: style.max_size,
        aspect_ratio: style.aspect_ratio,
        margin: style.margin,
        padding: style.padding,
        border_widths: style.border_widths,
        align_items: style.align_items,
        align_self: style.align_self,
        align_content: style.align_content,
        justify_content: style.justify_content,
        gap: style.gap,
        flex_direction: style.flex_direction,
        flex_wrap: style.flex_wrap,
        flex_basis: style.flex_basis,
        flex_grow: style.flex_grow,
        flex_shrink: style.flex_shrink,
        grid_cols: style.grid_cols,
        grid_rows: style.grid_rows,
        grid_location: style.grid_location.clone(),
    }
}
