use std::collections::VecDeque;

use gpui::{Context, KeybindingKeystroke, Keystroke, Render, Subscription, Task, Window};
use settings::{Settings, SettingsStore};
use ui::prelude::*;
use util::ResultExt;

use crate::ScreencastSettings;

// Likely to become a setting later, for now, hardcode to 21.
const MAX_VISIBLE_KEYS: usize = 21;

pub struct ScreencastOverlay {
    enabled: bool,
    recent_keystrokes: VecDeque<Keystroke>,
    hide_task: Option<Task<()>>,
    _keystroke_subscription: Subscription,
    settings: ScreencastSettings,
    _settings_subscription: Subscription,
}

impl ScreencastOverlay {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let window_id = window.window_handle().window_id();
        let overlay = cx.weak_entity();

        let keystroke_subscription = cx.intercept_keystrokes(move |event, event_window, cx| {
            if event_window.window_handle().window_id() != window_id || event.is_held {
                return;
            }

            overlay
                .update(cx, |overlay, cx| {
                    overlay.record_keystroke(event.keystroke.clone(), cx);
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
            recent_keystrokes: VecDeque::new(),
            hide_task: None,
            _keystroke_subscription: keystroke_subscription,
            settings: ScreencastSettings::get_global(cx).clone(),
            _settings_subscription: settings_subscription,
        }
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.enabled = !self.enabled;

        if !self.enabled {
            self.recent_keystrokes.clear();
            self.hide_task.take();
        }

        cx.notify();
    }

    pub fn record_keystroke(&mut self, keystroke: Keystroke, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }

        self.recent_keystrokes.push_back(keystroke);

        while self.recent_keystrokes.len() > MAX_VISIBLE_KEYS {
            self.recent_keystrokes.pop_front();
        }

        self.hide_task.take();

        let overlay = cx.weak_entity();
        let keyboard_overlay_timeout = self.settings.keyboard_overlay_timeout;

        self.hide_task = Some(cx.spawn(async move |_, cx| {
            cx.background_executor()
                .timer(keyboard_overlay_timeout)
                .await;

            overlay
                .update(cx, |overlay, cx| {
                    overlay.recent_keystrokes.clear();
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
        if !self.enabled || self.recent_keystrokes.is_empty() {
            return div().hidden().into_any_element();
        }

        let colors = cx.theme().colors();
        let font_size = rems_from_px(self.settings.font_size);

        let keycaps: _ = self.recent_keystrokes.iter().map(|keystroke| {
            let display_keystroke = KeybindingKeystroke::new_with_mapper(
                keystroke.clone(),
                false,
                cx.keyboard_mapper().as_ref(),
            );

            div()
                .flex()
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
                .children(ui::render_keybinding_keystroke(
                    &display_keystroke,
                    Some(Color::Default),
                    Some(font_size.into()),
                    ui::PlatformStyle::platform(),
                    false,
                ))
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
            .child(h_flex().w_full().justify_center().gap_1().children(keycaps))
            .into_any_element()
    }
}
