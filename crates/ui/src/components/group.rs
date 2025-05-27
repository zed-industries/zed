use gpui::{Div, div, prelude::*};

/// Creates a horizontal group with tight, consistent spacing.
///
/// xs: ~2px @16px/rem
pub fn h_group_sm() -> Div {
    div().flex().gap_0p5()
}

/// Creates a horizontal group with consistent spacing.
///
/// s: ~4px @16px/rem
pub fn h_group() -> Div {
    div().flex().gap_1()
}

/// Creates a horizontal group with consistent spacing.
///
/// m: ~6px @16px/rem
pub fn h_group_lg() -> Div {
    div().flex().gap_1p5()
}

/// Creates a horizontal group with consistent spacing.
///
/// l: ~8px @16px/rem
pub fn h_group_xl() -> Div {
    div().flex().gap_2()
}

/// Creates a vertical group with tight, consistent spacing.
///
/// xs: ~2px @16px/rem
pub fn v_group_sm() -> Div {
    div().flex().flex_col().gap_0p5()
}

/// Creates a vertical group with consistent spacing.
///
/// s: ~4px @16px/rem
pub fn v_group() -> Div {
    div().flex().flex_col().gap_1()
}

/// Creates a vertical group with consistent spacing.
///
/// m: ~6px @16px/rem
pub fn v_group_lg() -> Div {
    div().flex().flex_col().gap_1p5()
}

/// Creates a vertical group with consistent spacing.
///
/// l: ~8px @16px/rem
pub fn v_group_xl() -> Div {
    div().flex().flex_col().gap_2()
}
