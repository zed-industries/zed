//! Helix-style modal editing for the editor.
//!
//! Active when `helix_mode` is enabled together with the `helix-next` feature
//! flag; otherwise Helix mode is provided by the `vim` crate.

#[cfg(test)]
mod test;

use editor::Editor;
use gpui::{App, AppContext as _, Context, Entity, KeyContext, Window};
use vim_mode_setting::{HelixModeSetting, observe_modal_editing_changes_in};

/// Initializes the `helix` crate.
pub fn init(cx: &mut App) {
    cx.observe_new(Helix::register).detach();
}

/// Attaches the Helix state to an editor.
pub(crate) struct HelixAddon {
    pub(crate) entity: Entity<Helix>,
}

impl editor::Addon for HelixAddon {
    fn extend_key_context(&self, key_context: &mut KeyContext, cx: &App) {
        self.entity.read(cx).extend_key_context(key_context);
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// The Helix state of one editor.
pub(crate) struct Helix {}

impl Helix {
    fn extend_key_context(&self, key_context: &mut KeyContext) {
        key_context.add("Helix");
    }

    fn register(editor: &mut Editor, window: Option<&mut Window>, cx: &mut Context<Editor>) {
        let Some(window) = window else {
            return;
        };
        if !editor.use_modal_editing() {
            return;
        }

        let mut was_enabled = HelixModeSetting::uses_helix_crate(cx);
        observe_modal_editing_changes_in(window, cx, move |editor, window, cx| {
            let enabled = HelixModeSetting::uses_helix_crate(cx);
            if enabled == was_enabled {
                return;
            }
            was_enabled = enabled;
            if enabled {
                Self::activate(editor, window, cx);
            } else {
                Self::deactivate(editor, cx);
            }
        })
        .detach();
        if was_enabled {
            Self::activate(editor, window, cx);
        }
    }

    fn activate(editor: &mut Editor, _window: &mut Window, cx: &mut Context<Editor>) {
        let entity = cx.new(|_| Helix {});
        editor.register_addon(HelixAddon { entity });
    }

    fn deactivate(editor: &mut Editor, _cx: &mut Context<Editor>) {
        editor.unregister_addon::<HelixAddon>();
    }
}
