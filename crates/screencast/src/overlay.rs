use std::collections::VecDeque;

use gpui::{
    Context, KeybindingKeystroke, Keystroke, Modifiers, Render, Subscription, Task, Window,
};
use settings::{Settings, SettingsStore};
use ui::prelude::*;
use util::ResultExt;

use crate::ScreencastSettings;

// Likely to become a setting later, for now, hardcode to 15.
const MAX_VISIBLE_KEYS: usize = 15;

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
            if event_window.window_handle().window_id() != keystroke_window_id || event.is_held {
                return;
            }

            overlay
                .update(cx, |overlay, cx| {
                    overlay.record_keystroke(event.keystroke.clone(), cx);
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
                overlay.settings = ScreencastSettings::get_global(cx).clone();
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
            settings: ScreencastSettings::get_global(cx).clone(),
            _settings_subscription: settings_subscription,
        }
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.enabled = !self.enabled;

        if !self.enabled {
            self.recent_keys.clear();
            self.pending_modifier_keys.clear();
            self.hide_task.take();
        }

        cx.notify();
    }

    pub fn record_keystroke(&mut self, keystroke: Keystroke, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }

        self.remove_pending_modifier_keys();

        if keystroke.modifiers.modified()
            && matches!(
                self.recent_keys.back(),
                Some(DisplayedKey::Keystroke(previous_keystroke))
                    if previous_keystroke == &keystroke
            )
        {
            self.refresh_hide_task(cx);
            return;
        }

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
                if !self.pending_modifier_keys.contains(&modifier) {
                    self.pending_modifier_keys.push(modifier);
                }

                if matches!(
                    self.recent_keys.back(),
                    Some(DisplayedKey::Modifier(previous_modifier))
                        if previous_modifier == &modifier
                ) {
                    self.refresh_hide_task(cx);
                } else {
                    self.record_displayed_key(DisplayedKey::Modifier(modifier), cx);
                }
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

        self.recent_keys.push_back(key);

        if self.recent_keys.len() > MAX_VISIBLE_KEYS {
            self.recent_keys.clear();
            self.pending_modifier_keys.clear();
        }

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
                .px_2()
                .rounded_sm()
                .border_1()
                .rounded_md()
                .border_color(colors.border_variant.opacity(0.8))
                .bg(colors.element_background.opacity(0.55))
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
                h_flex()
                    .w_full()
                    .min_w_0()
                    .justify_center()
                    .gap_2()
                    .overflow_hidden()
                    .children(keycaps),
            )
            .into_any_element()
    }
}
