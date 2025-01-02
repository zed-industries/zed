use crate::prelude::*;
use gpui::{Window, AppContext, WindowBackgroundAppearance, };

/// Returns the [WindowBackgroundAppearance].
fn window_appearance(window: &mut Window, cx: &mut AppContext) -> WindowBackgroundAppearance {
    cx.theme().styles.window_background_appearance
}

/// Returns if the window and it's surfaces are expected
/// to be transparent.
///
/// Helps determine if you need to take extra steps to prevent
/// transparent backgrounds.
pub fn window_is_transparent(window: &mut Window, cx: &mut AppContext) -> bool {
    matches!(
        window_appearance(cx),
        WindowBackgroundAppearance::Transparent | WindowBackgroundAppearance::Blurred
    )
}
