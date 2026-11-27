//! Which-key support for Zed.

mod pending_bindings;
mod pending_keystrokes_indicator;
mod which_key_modal;
mod which_key_settings;

use gpui::{
    Action, App, AppContext as _, DismissEvent, KeybindingKeystroke, Keystroke,
    PlatformKeyboardMapper, SharedString, Window,
};
pub use pending_keystrokes_indicator::PendingKeystrokesIndicator;
use settings::Settings;
use std::{sync::LazyLock, time::Duration};
use util::ResultExt;
use which_key_modal::WhichKeyModal;
use which_key_settings::WhichKeySettings;
use workspace::MultiWorkspace;

gpui::actions!(
    which_key,
    [
        /// Shows the bindings that can complete the pending key sequence. Must be bound to a
        /// single keystroke.
        ShowPendingBindings
    ]
);

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
    // Which-key uses GPUI's candidate list as is, on purpose, instead of repeating dispatch rules.
    // The list can include a longer chord that can't be typed because a shorter binding for its
    // prefix comes later in the keymap. GPUI's shortcut display shows those chords too.
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
            let action_name = binding_label(binding.action());
            Some(PendingBinding {
                remaining_keystrokes,
                action_name,
            })
        })
        .collect()
}

fn binding_label(action: &dyn Action) -> SharedString {
    match action.as_any().downcast_ref::<zed_actions::Spawn>() {
        Some(zed_actions::Spawn::ByName { task_name, .. }) => task_name.clone().into(),
        _ => command_palette::humanize_action_name(action.name()).into(),
    }
}

pub fn init(cx: &mut App) {
    WhichKeySettings::register(cx);

    cx.observe_new(|_: &mut MultiWorkspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let mut timer = None;
        cx.observe_pending_input(window, move |multi_workspace, window, cx| {
            if window.pending_input_keystrokes().is_none() {
                multi_workspace.set_pending_bindings_popup(None, cx);
                timer.take();
                return;
            }

            let which_key_settings = WhichKeySettings::get_global(cx);
            if !which_key_settings.enabled {
                return;
            }

            let delay_ms = which_key_settings.delay_ms;

            timer.replace(cx.spawn_in(window, async move |multi_workspace, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(delay_ms))
                    .await;
                multi_workspace
                    .update_in(cx, |multi_workspace, window, cx| {
                        if multi_workspace.pending_bindings_popup().is_some() {
                            return;
                        }

                        let workspace = multi_workspace.workspace().downgrade();
                        let popup = cx.new(|cx| WhichKeyModal::new(workspace, window, cx));
                        cx.subscribe(&popup, |multi_workspace, _, _: &DismissEvent, cx| {
                            multi_workspace.set_pending_bindings_popup(None, cx);
                        })
                        .detach();
                        multi_workspace.set_pending_bindings_popup(Some(popup.into()), cx);
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
    use gpui::{
        BorrowAppContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
        InteractiveElement, IntoElement, KeyBinding, PlatformKeyboardMapper, Render,
        TestAppContext, VisualTestContext, actions, div,
    };
    use workspace::{AppState, ModalView};

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

    #[test]
    fn test_binding_label_uses_task_name_for_spawn_by_name() {
        let action = zed_actions::Spawn::ByName {
            task_name: "lazygit".to_string(),
            reveal_target: None,
        };

        assert_eq!(binding_label(&action), "lazygit");
    }

    actions!(which_key_test, [TestChordAction]);

    struct TestModal {
        focus_handle: FocusHandle,
        chord_action_count: usize,
    }

    impl TestModal {
        fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
            Self {
                focus_handle: cx.focus_handle(),
                chord_action_count: 0,
            }
        }
    }

    impl EventEmitter<DismissEvent> for TestModal {}

    impl Focusable for TestModal {
        fn focus_handle(&self, _cx: &App) -> FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl ModalView for TestModal {}

    impl Render for TestModal {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div().track_focus(&self.focus_handle).on_action(cx.listener(
                |this, _: &TestChordAction, _, _| {
                    this.chord_action_count += 1;
                },
            ))
        }
    }

    async fn init_workspace_test(
        cx: &mut TestAppContext,
    ) -> (Entity<MultiWorkspace>, &mut VisualTestContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            workspace::init(app_state.clone(), cx);
            init(cx);
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(r#"{ "which_key": { "enabled": true, "delay_ms": 0 } }"#, cx)
                    .expect("valid test settings");
            });
            cx.bind_keys([KeyBinding::new("ctrl-k ctrl-x", TestChordAction, None)]);
            app_state
        });
        let project = project::Project::test(app_state.fs.clone(), [], cx).await;
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx))
    }

    #[gpui::test]
    async fn test_which_key_shows_for_pending_chord(cx: &mut TestAppContext) {
        let (multi_workspace, cx) = init_workspace_test(cx).await;

        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();
        multi_workspace.read_with(cx, |multi_workspace, _| {
            assert!(multi_workspace.pending_bindings_popup().is_some());
        });

        cx.simulate_keystrokes("escape");
        cx.run_until_parked();
        multi_workspace.read_with(cx, |multi_workspace, _| {
            assert!(multi_workspace.pending_bindings_popup().is_none());
        });
    }

    #[gpui::test]
    async fn test_which_key_shows_above_open_modal(cx: &mut TestAppContext) {
        let (multi_workspace, cx) = init_workspace_test(cx).await;
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, TestModal::new);
        });
        cx.run_until_parked();

        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();

        assert!(
            workspace.read_with(cx, |workspace, cx| workspace
                .active_modal::<TestModal>(cx)
                .is_some()),
            "which-key should not dismiss the open modal"
        );
        multi_workspace.read_with(cx, |multi_workspace, _| {
            assert!(multi_workspace.pending_bindings_popup().is_some());
        });

        cx.simulate_keystrokes("ctrl-x");
        cx.run_until_parked();

        workspace.read_with(cx, |workspace, cx| {
            let modal = workspace
                .active_modal::<TestModal>(cx)
                .expect("modal should still be open");
            assert_eq!(modal.read(cx).chord_action_count, 1);
        });
        multi_workspace.read_with(cx, |multi_workspace, _| {
            assert!(multi_workspace.pending_bindings_popup().is_none());
        });
    }

    #[gpui::test]
    async fn test_keystroke_without_pending_chord_does_not_notify(cx: &mut TestAppContext) {
        let (multi_workspace, cx) = init_workspace_test(cx).await;
        let notify_count = std::rc::Rc::new(Cell::new(0));
        let _subscription = cx.update(|_, cx| {
            let notify_count = notify_count.clone();
            cx.observe(&multi_workspace, move |_, _| {
                notify_count.set(notify_count.get() + 1);
            })
        });

        cx.simulate_keystrokes("a");
        cx.run_until_parked();

        assert_eq!(notify_count.get(), 0);
    }
}
