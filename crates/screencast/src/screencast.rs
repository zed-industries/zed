mod overlay;
pub use overlay::ScreencastOverlay;

mod screencast_settings;
pub use screencast_settings::ScreencastSettings;

use settings::Settings;
use util::ResultExt;
use workspace::MultiWorkspace;

use std::collections::HashMap;

use gpui::{App, AppContext, Global, WeakEntity, Window, WindowId, actions};

actions!(
    dev,
    [
        /// Toggle the keyboard screencast overlay
        ToggleScreencast,
    ]
);

#[derive(Default)]
struct ScreencastRegistry {
    overlays: HashMap<WindowId, WeakEntity<ScreencastOverlay>>,
}

impl Global for ScreencastRegistry {}

pub fn init(cx: &mut App) {
    ScreencastSettings::register(cx);
    cx.set_global(ScreencastRegistry::default());

    cx.observe_new(|multi_workspace: &mut MultiWorkspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let window_id = window.window_handle().window_id();
        let overlay = cx.new(|cx| ScreencastOverlay::new(window, cx));

        cx.default_global::<ScreencastRegistry>()
            .overlays
            .insert(window_id, overlay.downgrade());

        multi_workspace.set_window_overlay(Some(overlay.into()), cx);
    })
    .detach();
}

pub fn toggle(window: &mut Window, cx: &mut App) {
    let window_id = window.window_handle().window_id();

    let overlay = cx
        .default_global::<ScreencastRegistry>()
        .overlays
        .get(&window_id)
        .cloned();

    if let Some(overlay) = overlay {
        overlay
            .update(cx, |overlay, cx| {
                overlay.toggle(cx);
            })
            .log_err();
    }
}
