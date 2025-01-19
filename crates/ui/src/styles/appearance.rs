use crate::prelude::*;
use gpui::{AppContext, WindowBackgroundAppearance};

/// Returns the [WindowBackgroundAppearance].
fn window_appearance(cx: &mut AppContext) -> WindowBackgroundAppearance {
    cx.theme().styles.window_background_appearance
}

/// Returns if the window and it's surfaces are expected
/// to be transparent.
///
/// Helps determine if you need to take extra steps to prevent
/// transparent backgrounds.
pub fn theme_is_transparent(cx: &mut AppContext) -> bool {
    matches!(
        window_appearance(cx),
        WindowBackgroundAppearance::Transparent | WindowBackgroundAppearance::Blurred
    )
}
