//! Which-key support for Zed.

mod which_key_modal;
mod which_key_settings;

use gpui::{App, AppContext, Context, Keystroke, Task, WeakEntity, Window};
use settings::Settings;
use std::{sync::LazyLock, time::Duration};
use util::ResultExt;
use which_key_modal::WhichKeyModal;
use which_key_settings::WhichKeySettings;
use workspace::Workspace;

pub fn init(cx: &mut App) {
    WhichKeySettings::register(cx);

    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let workspace_handle = cx.entity();
        let which_key =
            cx.new::<WhichKey>(|cx| WhichKey::new(workspace_handle.downgrade(), window, cx));
        workspace.which_key = Some(which_key.into_any());
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

pub struct WhichKey {
    timer: Option<Task<()>>,
    workspace: WeakEntity<Workspace>,
}

impl WhichKey {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe_pending_input(window, |this: &mut Self, window, cx| {
            this.update_pending_keys(window, cx);
        })
        .detach();

        Self {
            timer: None,
            workspace,
        }
    }

    fn update_pending_keys(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if window.pending_input_keystrokes().is_none() {
            // Hide modal
            self.workspace
                .update(cx, |workspace, cx| {
                    if workspace.active_modal::<WhichKeyModal>(cx).is_none() {
                        return;
                    };

                    workspace.hide_modal(window, cx);
                })
                .log_err();

            self.timer = None;
            return;
        }

        let which_key_settings = WhichKeySettings::get_global(cx);
        if !which_key_settings.enabled {
            return;
        }

        let delay_ms = which_key_settings.delay_ms;
        let workspace_handle = self.workspace.clone();

        self.timer = Some(cx.spawn_in(window, async move |_, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(delay_ms))
                .await;

            // Open modal
            workspace_handle
                .clone()
                .update_in(cx, |workspace, window, cx| {
                    if workspace.active_modal::<WhichKeyModal>(cx).is_some() {
                        return;
                    };

                    workspace.toggle_modal(window, cx, |window, cx| {
                        WhichKeyModal::new(workspace_handle, window, cx)
                    });
                })
                .log_err();
        }));
    }
}
