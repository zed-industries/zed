use std::collections::HashMap;
use std::sync::Arc;

use gpui::{App, BorrowAppContext, Global, SharedString, WindowId};

use crate::{GlobalTheme, Theme};

/// A single per-window theme override: the chosen theme's name (kept for
/// persistence and re-resolution) plus its resolved value (ready to apply during
/// the window's render pass).
#[derive(Clone)]
struct WindowThemeOverride {
    theme_name: SharedString,
    theme: Arc<Theme>,
}

/// Per-window theme overrides, keyed by window.
///
/// A window present in this map renders with its override theme instead of the
/// app-wide configured theme. Overrides are stored user-side (never written to a
/// project's `.zed/settings.json`) so that the user — not an opened repository —
/// decides per-window appearance.
///
/// Resolving a theme name to an [`Arc<Theme>`] (and applying the user's theme
/// overrides) requires the `theme_settings` crate, so callers resolve first and
/// store via [`WindowThemeOverrides::set_resolved`]. The per-frame draw hook only
/// reads the already-resolved theme, keeping it cheap.
#[derive(Default)]
pub struct WindowThemeOverrides {
    overrides: HashMap<WindowId, WindowThemeOverride>,
}

impl Global for WindowThemeOverrides {}

impl WindowThemeOverrides {
    /// Stores a resolved theme override for a window. Does not trigger a redraw;
    /// callers that want the change reflected immediately should call
    /// [`App::refresh_windows`].
    pub fn set_resolved(
        cx: &mut App,
        window_id: WindowId,
        theme_name: SharedString,
        theme: Arc<Theme>,
    ) {
        cx.update_global::<Self, _>(|this, _| {
            this.overrides
                .insert(window_id, WindowThemeOverride { theme_name, theme });
        });
    }

    /// Removes the override for a window, falling back to the configured theme.
    /// Returns whether an override was present. Does not trigger a redraw.
    pub fn clear(cx: &mut App, window_id: WindowId) -> bool {
        cx.update_global::<Self, _>(|this, _| this.overrides.remove(&window_id).is_some())
    }

    /// Returns the resolved override theme for a window, if any.
    pub fn theme(cx: &App, window_id: WindowId) -> Option<Arc<Theme>> {
        cx.try_global::<Self>()
            .and_then(|this| this.overrides.get(&window_id).map(|o| o.theme.clone()))
    }

    /// Returns the override theme name for a window, if any (for persistence).
    pub fn theme_name(cx: &App, window_id: WindowId) -> Option<SharedString> {
        cx.try_global::<Self>()
            .and_then(|this| this.overrides.get(&window_id).map(|o| o.theme_name.clone()))
    }

    /// Returns all `(window, theme name)` overrides (for persistence and
    /// re-resolution).
    pub fn entries(cx: &App) -> Vec<(WindowId, SharedString)> {
        cx.try_global::<Self>()
            .map(|this| {
                this.overrides
                    .iter()
                    .map(|(id, o)| (*id, o.theme_name.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Sets the active theme for the window currently being drawn: its override
    /// if present, otherwise the app-wide configured theme. Runs every frame from
    /// the window-draw hook, so it sets the active theme without notifying
    /// observers (see [`GlobalTheme::set_active_theme`]).
    pub fn apply_for_window(cx: &mut App, window_id: WindowId) {
        let theme =
            Self::theme(cx, window_id).unwrap_or_else(|| GlobalTheme::configured_theme(cx).clone());
        GlobalTheme::set_active_theme(cx, theme);
    }
}
