//! Which-key support for Zed.

mod pending_keystrokes_indicator;
mod which_key_modal;
mod which_key_settings;

use gpui::{App, KeybindingKeystroke, Keystroke, PlatformKeyboardMapper, SharedString, Window};
pub use pending_keystrokes_indicator::PendingKeystrokesIndicator;
use settings::Settings;
use std::{sync::LazyLock, time::Duration};
use util::ResultExt;
use which_key_modal::WhichKeyModal;
use which_key_settings::WhichKeySettings;
use workspace::Workspace;

pub(crate) struct PendingBinding {
    pub(crate) remaining_keystrokes: Vec<KeybindingKeystroke>,
    pub(crate) action_name: SharedString,
}

pub(crate) fn map_pending_keystrokes(
    keystrokes: &[Keystroke],
    keyboard_mapper: &dyn PlatformKeyboardMapper,
) -> Vec<KeybindingKeystroke> {
    keystrokes
        .iter()
        .cloned()
        .map(|keystroke| KeybindingKeystroke::new_with_mapper(keystroke, false, keyboard_mapper))
        .collect()
}

pub(crate) fn bindings_for_pending_input(
    window: &Window,
    pending_keystrokes: &[Keystroke],
) -> Vec<PendingBinding> {
    collect_bindings_for_pending_input(window, pending_keystrokes, |_| true)
}

pub(crate) fn bindings_for_which_key(
    window: &Window,
    pending_keystrokes: &[Keystroke],
) -> Vec<PendingBinding> {
    collect_bindings_for_pending_input(window, pending_keystrokes, |binding| {
        let binding_keystrokes = binding.keystrokes();
        !FILTERED_KEYSTROKES.iter().any(|filtered| {
            binding_keystrokes.len() >= filtered.len()
                && binding_keystrokes[..filtered.len()]
                    .iter()
                    .map(|keystroke| keystroke.inner())
                    .eq(filtered.iter())
        })
    })
}

fn collect_bindings_for_pending_input(
    window: &Window,
    pending_keystrokes: &[Keystroke],
    mut include_binding: impl FnMut(&gpui::KeyBinding) -> bool,
) -> Vec<PendingBinding> {
    window
        .possible_bindings_for_input(pending_keystrokes)
        .into_iter()
        .filter(|binding| include_binding(binding))
        .filter_map(|binding| {
            let remaining_keystrokes = binding.keystrokes().get(pending_keystrokes.len()..)?;
            if remaining_keystrokes.is_empty() {
                return None;
            }
            let remaining_keystrokes = remaining_keystrokes.to_vec();
            let action_name = command_palette::humanize_action_name(binding.action().name()).into();
            Some(PendingBinding {
                remaining_keystrokes,
                action_name,
            })
        })
        .collect()
}

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
        "g j",
        "g k",
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

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use collections::HashMap;
    use gpui::PlatformKeyboardMapper;

    use super::*;

    struct TestKeyboardMapper {
        call_count: Cell<usize>,
    }

    impl PlatformKeyboardMapper for TestKeyboardMapper {
        fn map_key_equivalent(
            &self,
            keystroke: Keystroke,
            use_key_equivalents: bool,
        ) -> KeybindingKeystroke {
            assert!(!use_key_equivalents);
            self.call_count.set(self.call_count.get() + 1);

            #[cfg(target_os = "windows")]
            {
                KeybindingKeystroke::new(
                    keystroke,
                    gpui::Modifiers::control_shift(),
                    "2".to_owned(),
                )
            }
            #[cfg(not(target_os = "windows"))]
            {
                KeybindingKeystroke::from_keystroke(keystroke)
            }
        }

        fn get_key_equivalents(&self) -> Option<&HashMap<char, char>> {
            None
        }
    }

    #[gpui::test]
    fn test_map_pending_keystrokes_uses_platform_mapper(cx: &mut App) {
        let keyboard_mapper = TestKeyboardMapper {
            call_count: Cell::new(0),
        };
        let keystroke = Keystroke::parse("ctrl-@").expect("valid test keystroke");

        let mapped = map_pending_keystrokes(std::slice::from_ref(&keystroke), &keyboard_mapper);

        assert_eq!(keyboard_mapper.call_count.get(), 1);
        let Some(mapped_keystroke) = mapped.first() else {
            panic!("expected mapped pending keystroke");
        };
        assert_eq!(mapped_keystroke.inner(), &keystroke);
        #[cfg(target_os = "windows")]
        {
            assert_eq!(
                mapped_keystroke.modifiers(),
                &gpui::Modifiers::control_shift()
            );
            assert_eq!(mapped_keystroke.key(), "2");
        }

        let expected_display_text = if cfg!(target_os = "windows") {
            "Ctrl-Shift-2"
        } else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            "Ctrl-@"
        } else {
            "Control-@"
        };
        assert_eq!(
            ui::text_for_keybinding_keystrokes(&mapped, cx),
            expected_display_text
        );
    }
}
