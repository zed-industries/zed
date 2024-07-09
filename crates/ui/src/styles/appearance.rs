use crate::prelude::*;
use gpui::{WindowBackgroundAppearance, WindowContext};
use theme::Appearance;

/// Returns the current [Appearance].
pub fn appearance(cx: &WindowContext) -> Appearance {
    cx.theme().appearance
}

/// Returns the [WindowBackgroundAppearance].
pub fn window_appereance(cx: &WindowContext) -> WindowBackgroundAppearance {
    cx.theme().styles.window_background_appearance
}

/// Returns if the window and it's surfaces are expected
/// to be transparent.
///
/// Helps determine if you need to take extra steps to prevent
/// transparent backgrounds.
pub fn window_is_transparent(cx: &WindowContext) -> bool {
    match window_appereance(cx) {
        WindowBackgroundAppearance::Transparent => true,
        WindowBackgroundAppearance::Blurred => true,
        _ => false,
    }
}
