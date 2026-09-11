use std::collections::VecDeque;

use gpui::{
    Context, KeybindingKeystroke, Keystroke, Modifiers, Render, Subscription, Task, Window,
};
use settings::{Settings, SettingsStore};
use ui::prelude::*;
use util::ResultExt;

use crate::ScreencastSettings;

/// Likely to become a setting later, for now, hardcode to 21
/// VS Code's default is 21, if it hits 22 it clears the queue
///
/// I'm not sure it's my decision to go based on the VS Code side of things,
/// but that's the default for now. Up to change it whenever.
const MAX_VISIBLE_KEYS: usize = 21;

enum DisplayedKey {
    Modifier(Modifiers),
    Keystroke(Keystroke),
}

pub struct ScreencastOverlay {
    enabled: bool,
    recent_keys: VecDeque<DisplayedKey>,
    previous_modifiers: Modifiers,
    pending_modifier_keys: Vec<Modifiers>,
    hide_task: Option<Task<()>>,
    _keystroke_subscription: Subscription,
    _modifier_subscription: Subscription,
    settings: ScreencastSettings,
    _settings_subscription: Subscription,
}

impl ScreencastOverlay {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let window_id = window.window_handle().window_id();
        let overlay = cx.weak_entity();

        let keystroke_window_id = window_id;
        let keystroke_subscription = cx.intercept_keystrokes(move |event, event_window, cx| {
            if event_window.window_handle().window_id() != keystroke_window_id {
                return;
            }

            overlay
                .update(cx, |overlay, cx| {
                    overlay.handle_keystroke(&event.keystroke, event.is_held, cx);
                })
                .log_err();
        });

        let overlay = cx.weak_entity();
        let modifier_window_id = window_id;
        let modifier_subscription = cx.observe_modifiers_changed(move |event, event_window, cx| {
            if event_window.window_handle().window_id() != modifier_window_id {
                return;
            }

            overlay
                .update(cx, |overlay, cx| {
                    overlay.record_modifier_change(event.modifiers, cx);
                })
                .log_err();
        });

        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, |overlay, _window, cx| {
                overlay.settings = *ScreencastSettings::get_global(cx);
                cx.notify();
            });

        Self {
            enabled: false,
            recent_keys: VecDeque::new(),
            previous_modifiers: window.modifiers(),
            pending_modifier_keys: Vec::new(),
            hide_task: None,
            _keystroke_subscription: keystroke_subscription,
            _modifier_subscription: modifier_subscription,
            settings: *ScreencastSettings::get_global(cx),
            _settings_subscription: settings_subscription,
        }
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) -> bool {
        self.enabled = !self.enabled;

        if !self.enabled {
            self.recent_keys.clear();
            self.pending_modifier_keys.clear();
            self.hide_task.take();
        }

        cx.notify();
        self.enabled
    }

    fn handle_keystroke(&mut self, keystroke: &Keystroke, is_held: bool, cx: &mut Context<Self>) {
        if is_held {
            if self.enabled && !self.recent_keys.is_empty() {
                self.refresh_hide_task(cx);
            }
            return;
        }

        self.record_keystroke(keystroke.clone(), cx);
    }

    pub fn record_keystroke(&mut self, keystroke: Keystroke, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }

        self.remove_pending_modifier_keys();

        self.record_displayed_key(DisplayedKey::Keystroke(keystroke), cx);
    }

    fn record_modifier_change(&mut self, modifiers: Modifiers, cx: &mut Context<Self>) {
        let previous_modifiers = self.previous_modifiers;
        self.previous_modifiers = modifiers;

        if !self.enabled {
            return;
        }

        let modifier_changes = [
            (
                previous_modifiers.function,
                modifiers.function,
                Modifiers::function(),
            ),
            (
                previous_modifiers.control,
                modifiers.control,
                Modifiers::control(),
            ),
            (previous_modifiers.alt, modifiers.alt, Modifiers::alt()),
            (
                previous_modifiers.platform,
                modifiers.platform,
                Modifiers::platform(),
            ),
            (
                previous_modifiers.shift,
                modifiers.shift,
                Modifiers::shift(),
            ),
        ];

        for (was_pressed, is_pressed, modifier) in modifier_changes {
            if was_pressed && !is_pressed {
                self.pending_modifier_keys
                    .retain(|pending| *pending != modifier);
            }
        }

        for (was_pressed, is_pressed, modifier) in modifier_changes {
            if !was_pressed && is_pressed {
                self.record_displayed_key(DisplayedKey::Modifier(modifier), cx);
                self.pending_modifier_keys.push(modifier);
            }
        }
    }

    fn remove_pending_modifier_keys(&mut self) {
        let pending_modifier_keys = std::mem::take(&mut self.pending_modifier_keys);
        for modifier in pending_modifier_keys {
            if let Some(index) = self.recent_keys.iter().rposition(|key| {
                matches!(key, DisplayedKey::Modifier(key_modifier) if *key_modifier == modifier)
            }) {
                self.recent_keys.remove(index);
            }
        }
    }

    fn record_displayed_key(&mut self, key: DisplayedKey, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }

        if self.recent_keys.len() >= MAX_VISIBLE_KEYS {
            self.recent_keys.clear();
            self.pending_modifier_keys.clear();
        }

        self.recent_keys.push_back(key);
        self.refresh_hide_task(cx);
    }

    fn refresh_hide_task(&mut self, cx: &mut Context<Self>) {
        self.hide_task.take();

        let overlay = cx.weak_entity();
        let keyboard_overlay_timeout = self.settings.keyboard_overlay_timeout;

        self.hide_task = Some(cx.spawn(async move |_, cx| {
            cx.background_executor()
                .timer(keyboard_overlay_timeout)
                .await;

            overlay
                .update(cx, |overlay, cx| {
                    overlay.recent_keys.clear();
                    overlay.hide_task = None;
                    cx.notify();
                })
                .log_err();
        }));

        cx.notify();
    }
}

impl Render for ScreencastOverlay {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        if !self.enabled || self.recent_keys.is_empty() {
            return div().hidden().into_any_element();
        }

        let colors = cx.theme().colors();
        let font_size = rems_from_px(self.settings.font_size);

        let keycaps = self.recent_keys.iter().map(|key| {
            let contents = match key {
                DisplayedKey::Modifier(modifiers) => ui::render_modifiers(
                    modifiers,
                    ui::PlatformStyle::platform(),
                    Some(Color::Default),
                    Some(font_size.into()),
                    false,
                )
                .collect::<Vec<_>>(),
                DisplayedKey::Keystroke(keystroke) => {
                    let display_keystroke = KeybindingKeystroke::new_with_mapper(
                        keystroke.clone(),
                        false,
                        cx.keyboard_mapper().as_ref(),
                    );

                    ui::render_keybinding_keystroke(
                        &display_keystroke,
                        Some(Color::Default),
                        Some(font_size.into()),
                        ui::PlatformStyle::platform(),
                        false,
                    )
                }
            };

            div()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .min_w(font_size)
                .h(rems_from_px(20.0_f32) + font_size)
                .px_0()
                .border_1()
                .border_b_6()
                .rounded_md()
                .border_color(colors.ghost_element_selected.opacity(0.8)) // Note entirely set on this being the key's border color.
                .bg(colors.element_background.opacity(0.5))
                .shadow_sm()
                .children(contents)
        });

        div()
            .absolute()
            .left_0()
            .right_0()
            .bottom(relative(self.settings.vertical_offset))
            .py_2()
            .bg(colors.status_bar_background.opacity(0.85))
            .border_t_1()
            .border_color(colors.border.opacity(0.5))
            .child(
                h_flex().w_full().justify_center().child(
                    h_flex()
                        .font_buffer(cx) // Match editor font family, maybe it should match system?
                        .min_w_0()
                        .max_w_full()
                        .justify_end()
                        .px_4()
                        .gap_2()
                        .overflow_hidden()
                        .children(keycaps),
                ),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use gpui::{Entity, TestAppContext};

    use super::*;

    fn setup_overlay(cx: &mut TestAppContext) -> Entity<ScreencastOverlay> {
        cx.skip_drawing();
        cx.update(|cx| {
            settings::init(cx);
            ScreencastSettings::register(cx);
        });

        let window = cx.add_window(ScreencastOverlay::new);
        window
            .root(cx)
            .expect("screencast overlay should be the test window root")
    }

    fn parse_keystroke(value: &str) -> Keystroke {
        Keystroke::parse(value).expect("test keystroke should parse")
    }

    #[gpui::test]
    fn test_disabled_input_and_toggle_clearing(cx: &mut TestAppContext) {
        let overlay = setup_overlay(cx);
        let keystroke = parse_keystroke("a");

        overlay.update(cx, |overlay, cx| {
            overlay.record_keystroke(keystroke.clone(), cx);
            overlay.record_modifier_change(Modifiers::control(), cx);

            assert!(overlay.recent_keys.is_empty());
            assert!(overlay.pending_modifier_keys.is_empty());
            assert!(overlay.hide_task.is_none());

            overlay.record_modifier_change(Modifiers::none(), cx);
            assert!(overlay.toggle(cx));
            overlay.record_keystroke(keystroke.clone(), cx);
            overlay.record_modifier_change(Modifiers::control(), cx);

            assert_eq!(overlay.recent_keys.len(), 2);
            assert_eq!(overlay.pending_modifier_keys, vec![Modifiers::control()]);
            assert!(overlay.hide_task.is_some());

            assert!(!overlay.toggle(cx));

            assert!(overlay.recent_keys.is_empty());
            assert!(overlay.pending_modifier_keys.is_empty());
            assert!(overlay.hide_task.is_none());

            overlay.record_keystroke(keystroke, cx);
            assert!(overlay.recent_keys.is_empty());
        });
    }

    #[gpui::test]
    fn test_modifier_is_incorporated_into_modified_keystroke(cx: &mut TestAppContext) {
        let overlay = setup_overlay(cx);
        let keystroke = parse_keystroke("ctrl-k");

        overlay.update(cx, |overlay, cx| {
            overlay.toggle(cx);
            overlay.record_modifier_change(Modifiers::control(), cx);

            assert!(matches!(
                overlay.recent_keys.front(),
                Some(DisplayedKey::Modifier(modifier)) if modifier == &Modifiers::control()
            ));
            assert_eq!(overlay.pending_modifier_keys, vec![Modifiers::control()]);

            overlay.record_keystroke(keystroke.clone(), cx);

            assert!(overlay.pending_modifier_keys.is_empty());
            assert_eq!(overlay.recent_keys.len(), 1);
            assert!(matches!(
                overlay.recent_keys.front(),
                Some(DisplayedKey::Keystroke(displayed_keystroke))
                    if displayed_keystroke == &keystroke
            ));
        });
    }

    #[gpui::test]
    fn test_prior_modifier_tap_remains_before_modified_keystroke(cx: &mut TestAppContext) {
        let overlay = setup_overlay(cx);
        let keystroke = parse_keystroke("ctrl-k");

        overlay.update(cx, |overlay, cx| {
            overlay.toggle(cx);
            overlay.record_modifier_change(Modifiers::control(), cx);
            overlay.record_modifier_change(Modifiers::none(), cx);
            overlay.record_modifier_change(Modifiers::control(), cx);
            overlay.record_keystroke(keystroke.clone(), cx);

            assert!(overlay.pending_modifier_keys.is_empty());
            assert_eq!(overlay.recent_keys.len(), 2);
            assert!(matches!(
                overlay.recent_keys.front(),
                Some(DisplayedKey::Modifier(modifier)) if modifier == &Modifiers::control()
            ));
            assert!(matches!(
                overlay.recent_keys.back(),
                Some(DisplayedKey::Keystroke(displayed_keystroke))
                    if displayed_keystroke == &keystroke
            ));
        });
    }

    #[gpui::test]
    fn test_separate_repeated_modified_keystrokes_are_recorded(cx: &mut TestAppContext) {
        let overlay = setup_overlay(cx);
        let keystroke = parse_keystroke("ctrl-k");

        overlay.update(cx, |overlay, cx| {
            overlay.toggle(cx);
            overlay.record_keystroke(keystroke.clone(), cx);
            overlay.record_keystroke(keystroke.clone(), cx);

            assert_eq!(overlay.recent_keys.len(), 2);
            assert!(overlay.recent_keys.iter().all(|displayed_key| {
                matches!(
                    displayed_key,
                    DisplayedKey::Keystroke(displayed_keystroke)
                        if displayed_keystroke == &keystroke
                )
            }));
        });
    }

    #[gpui::test]
    fn test_queue_resets_after_visible_key_limit(cx: &mut TestAppContext) {
        let overlay = setup_overlay(cx);
        let keystroke = parse_keystroke("a");
        let newest_keystroke = parse_keystroke("b");

        overlay.update(cx, |overlay, cx| {
            overlay.toggle(cx);
            for _ in 0..MAX_VISIBLE_KEYS {
                overlay.record_keystroke(keystroke.clone(), cx);
            }
            assert_eq!(overlay.recent_keys.len(), MAX_VISIBLE_KEYS);

            overlay.record_keystroke(newest_keystroke.clone(), cx);
            assert_eq!(overlay.recent_keys.len(), 1);
            assert!(overlay.pending_modifier_keys.is_empty());
            assert!(matches!(
                overlay.recent_keys.front(),
                Some(DisplayedKey::Keystroke(displayed_keystroke))
                    if displayed_keystroke == &newest_keystroke
            ));

            overlay.record_keystroke(keystroke, cx);
            assert_eq!(overlay.recent_keys.len(), 2);
        });
    }

    #[gpui::test]
    fn test_held_input_refreshes_timeout_without_adding_key(cx: &mut TestAppContext) {
        let overlay = setup_overlay(cx);
        let timeout = Duration::from_secs(5);
        let keystroke = parse_keystroke("a");

        overlay.update(cx, |overlay, cx| {
            overlay.settings.keyboard_overlay_timeout = timeout;
            overlay.toggle(cx);
            overlay.handle_keystroke(&keystroke, false, cx);
        });
        cx.run_until_parked();

        cx.executor().advance_clock(Duration::from_secs(4));
        cx.run_until_parked();
        overlay.update(cx, |overlay, cx| {
            overlay.handle_keystroke(&keystroke, true, cx);
        });
        cx.run_until_parked();
        overlay.read_with(cx, |overlay, _| {
            assert_eq!(overlay.recent_keys.len(), 1);
            assert!(overlay.hide_task.is_some());
        });

        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        assert_eq!(
            overlay.read_with(cx, |overlay, _| overlay.recent_keys.len()),
            1
        );

        cx.executor().advance_clock(Duration::from_secs(4));
        cx.run_until_parked();
        overlay.read_with(cx, |overlay, _| {
            assert!(overlay.recent_keys.is_empty());
            assert!(overlay.hide_task.is_none());
        });
    }

    #[gpui::test]
    fn test_timeout_refreshes_and_clears_queue(cx: &mut TestAppContext) {
        let overlay = setup_overlay(cx);
        let timeout = Duration::from_secs(5);

        overlay.update(cx, |overlay, cx| {
            overlay.settings.keyboard_overlay_timeout = timeout;
            overlay.toggle(cx);
            overlay.record_keystroke(parse_keystroke("a"), cx);
        });
        cx.run_until_parked();

        cx.executor().advance_clock(Duration::from_secs(4));
        cx.run_until_parked();
        assert_eq!(
            overlay.read_with(cx, |overlay, _| overlay.recent_keys.len()),
            1
        );

        overlay.update(cx, |overlay, cx| {
            overlay.record_keystroke(parse_keystroke("b"), cx);
        });
        cx.run_until_parked();

        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        overlay.read_with(cx, |overlay, _| {
            assert_eq!(overlay.recent_keys.len(), 2);
            assert!(overlay.hide_task.is_some());
        });

        cx.executor().advance_clock(Duration::from_secs(4));
        cx.run_until_parked();
        overlay.read_with(cx, |overlay, _| {
            assert!(overlay.recent_keys.is_empty());
            assert!(overlay.hide_task.is_none());
        });
    }
}
