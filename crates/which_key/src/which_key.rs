//! Which-key support for Zed.

mod which_key_modal;
mod which_key_settings;

use gpui::{App, Keystroke};
use settings::Settings;
use std::{sync::LazyLock, time::Duration};
use util::ResultExt;
use which_key_modal::WhichKeyModal;
use which_key_settings::WhichKeySettings;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    WhichKeySettings::register(cx);

    cx.observe_new(|_: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let mut timer = None;
        cx.observe_pending_input(window, move |workspace, window, cx| {
            if window.pending_input_keystrokes().is_none() {
                if let Some(modal) = workspace.active_modal::<WhichKeyModal>(cx) {
                    modal.update(cx, |modal, cx| modal.dismiss(cx));
                };
                timer.take();
                return;
            }

            let which_key_settings = WhichKeySettings::get_global(cx);
            if !which_key_settings.enabled {
                return;
            }

            let delay_ms = which_key_settings.delay_ms;

            timer.replace(cx.spawn_in(window, async move |workspace_handle, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(delay_ms))
                    .await;
                workspace_handle
                    .update_in(cx, |workspace, window, cx| {
                        if workspace.active_modal::<WhichKeyModal>(cx).is_some() {
                            return;
                        };

                        workspace.toggle_modal(window, cx, |window, cx| {
                            WhichKeyModal::new(workspace_handle.clone(), window, cx)
                        });
                    })
                    .log_err();
            }));
        })
        .detach();
    })
    .detach();
}

// Hard-coded list of keystrokes to filter out from which-key display
pub static FILTERED_KEYSTROKES: LazyLock<Vec<Vec<Keystroke>>> = LazyLock::new(|| {
    [
        // Modifiers on normal vim commands
        "g h",
        "g j",
        "g k",
        "g l",
        "g $",
        "g ^",
        // Duplicate keys with "ctrl" held, e.g. "ctrl-w ctrl-a" is duplicate of "ctrl-w a"
        "ctrl-w ctrl-a",
        "ctrl-w ctrl-c",
        "ctrl-w ctrl-h",
        "ctrl-w ctrl-j",
        "ctrl-w ctrl-k",
        "ctrl-w ctrl-l",
        "ctrl-w ctrl-n",
        "ctrl-w ctrl-o",
        "ctrl-w ctrl-p",
        "ctrl-w ctrl-q",
        "ctrl-w ctrl-s",
        "ctrl-w ctrl-v",
        "ctrl-w ctrl-w",
        "ctrl-w ctrl-]",
        "ctrl-w ctrl-shift-w",
        "ctrl-w ctrl-g t",
        "ctrl-w ctrl-g shift-t",
    ]
    .iter()
    .filter_map(|s| {
        let keystrokes: Result<Vec<_>, _> = s
            .split(' ')
            .map(|keystroke_str| Keystroke::parse(keystroke_str))
            .collect();
        keystrokes.ok()
    })
    .collect()
});
