use crate::prelude::*;
use gpui::WindowBackgroundAppearance;

/// Returns the [WindowBackgroundAppearance].
fn window_appearance(cx: &AppContext) -> WindowBackgroundAppearance {
    cx.theme().styles.window_background_appearance
}

/// Returns if the window and it's surfaces are expected
/// to be transparent.
///
/// Helps determine if you need to take extra steps to prevent
/// transparent backgrounds.
pub fn window_is_transparent(cx: &AppContext) -> bool {
    matches!(
        window_appearance(cx),
        WindowBackgroundAppearance::Transparent | WindowBackgroundAppearance::Blurred
    )
}
