use gpui::{
    Animation, AnimationExt, App, Context, KeybindingKeystroke, Render, Subscription, Window,
};
use settings::{Settings, SettingsStore};
use std::{rc::Rc, time::Duration};
use ui::{CircularProgress, KeyBinding, Tooltip, prelude::*, text_for_keybinding_keystrokes};
use vim_mode_setting::{HelixModeSetting, VimModeSetting};
use workspace::{HideStatusItem, StatusBarSettings, StatusItemView, item::ItemHandle};

use crate::{bindings_for_pending_input, map_pending_keystrokes};

const MAX_TOOLTIP_BINDINGS: usize = 10;

/// A status bar item shown while timed pending input can complete a multi-stroke key binding.
pub struct PendingKeystrokesIndicator {
    render_state: Option<Rc<IndicatorRenderState>>,
    pending_input_generation: u64,
    _pending_input_subscription: Subscription,
    _settings_subscription: Subscription,
}

struct IndicatorRenderState {
    keystrokes: Rc<[KeybindingKeystroke]>,
    pending_input_generation: u64,
    bindings: Vec<(Rc<[KeybindingKeystroke]>, SharedString)>,
    timeout_duration: Duration,
    remaining_duration: Duration,
    timeout_paused: bool,
}

impl PendingKeystrokesIndicator {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let pending_input_subscription =
            cx.observe_pending_input(window, |this: &mut Self, window, cx| {
                if this.refresh_render_state(window, cx) {
                    cx.notify();
                }
            });

        let mut enabled = Self::enabled(cx);
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
                let new_enabled = Self::enabled(cx);
                if new_enabled == enabled {
                    return;
                }

                enabled = new_enabled;
                if this.refresh_render_state(window, cx) {
                    cx.notify();
                }
            });

        Self {
            render_state: None,
            pending_input_generation: 0,
            _pending_input_subscription: pending_input_subscription,
            _settings_subscription: settings_subscription,
        }
    }

    fn enabled(cx: &App) -> bool {
        let status_bar_settings = StatusBarSettings::get_global(cx);
        status_bar_settings.show
            && status_bar_settings.pending_keystrokes_indicator
            && !VimModeSetting::is_enabled(cx)
            && !HelixModeSetting::is_enabled(cx)
    }

    fn refresh_render_state(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if !Self::enabled(cx) {
            return self.clear_render_state(window, cx);
        }

        let Some(pending_input) = window.pending_input() else {
            return self.clear_render_state(window, cx);
        };
        let Some(timeout) = pending_input.timeout() else {
            return self.clear_render_state(window, cx);
        };
        let keystrokes = pending_input.keystrokes();

        let mut bindings = bindings_for_pending_input(window, keystrokes)
            .into_iter()
            .map(|binding| {
                let remaining_text =
                    text_for_keybinding_keystrokes(&binding.remaining_keystrokes, cx);
                (
                    remaining_text,
                    binding.remaining_keystrokes,
                    binding.action_name,
                )
            })
            .collect::<Vec<_>>();
        bindings.sort_by(|(text_a, keys_a, action_a), (text_b, keys_b, action_b)| {
            keys_a
                .len()
                .cmp(&keys_b.len())
                .then_with(|| text_a.cmp(text_b))
                .then_with(|| action_a.cmp(action_b))
        });
        bindings.dedup_by(|(text_a, _, action_a), (text_b, _, action_b)| {
            text_a == text_b && action_a == action_b
        });

        self.pending_input_generation = self.pending_input_generation.wrapping_add(1);
        self.render_state = Some(Rc::new(IndicatorRenderState {
            keystrokes: map_pending_keystrokes(keystrokes, cx.keyboard_mapper().as_ref()).into(),
            pending_input_generation: self.pending_input_generation,
            bindings: bindings
                .into_iter()
                .map(|(_, keystrokes, action)| (Rc::from(keystrokes), action))
                .collect(),
            timeout_duration: timeout.duration(),
            remaining_duration: timeout.remaining(cx),
            timeout_paused: timeout.is_paused(),
        }));
        true
    }

    fn clear_render_state(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        window.set_pending_input_timeout_paused(&cx.entity(), false, cx);
        self.render_state.take().is_some()
    }

    fn render_state(&self) -> Option<&Rc<IndicatorRenderState>> {
        self.render_state.as_ref()
    }

    fn set_hovered(&mut self, hovered: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.render_state.is_none() {
            return;
        }

        window.set_pending_input_timeout_paused(&cx.entity(), hovered, cx);
    }
}

impl Render for PendingKeystrokesIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(render_state) = self.render_state().cloned() else {
            return div().hidden().into_any_element();
        };
        let remaining_fraction = if render_state.timeout_duration.is_zero() {
            0.0
        } else {
            (render_state.remaining_duration.as_secs_f32()
                / render_state.timeout_duration.as_secs_f32())
            .clamp(0.0, 1.0)
        };

        h_flex()
            .id("pending-keystrokes-indicator")
            .gap_1()
            .px_1()
            .child(if cx.reduce_motion() {
                Icon::new(IconName::CountdownTimer)
                    .size(IconSize::XSmall)
                    .color(Color::Muted)
                    .into_any_element()
            } else {
                let progress = CircularProgress::new(remaining_fraction, 1.0, px(13.), cx)
                    .stroke_width(px(2.))
                    .progress_color(cx.theme().colors().text_muted);
                if render_state.timeout_paused || render_state.remaining_duration.is_zero() {
                    progress.into_any_element()
                } else {
                    progress
                        .with_animation(
                            (
                                "pending-keystrokes-countdown",
                                render_state.pending_input_generation,
                            ),
                            Animation::new(render_state.remaining_duration).with_max_fps(30.0),
                            move |progress, delta| {
                                progress.value(remaining_fraction * (1.0 - delta))
                            },
                        )
                        .into_any_element()
                }
            })
            .child(
                KeyBinding::from_keystrokes(render_state.keystrokes.clone(), false)
                    .size(rems_from_px(12_f32)),
            )
            .tooltip(Tooltip::element(move |_, _| {
                let render_state = &render_state;
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(KeyBinding::from_keystrokes(
                                render_state.keystrokes.clone(),
                                false,
                            ))
                            .child(Label::new("is waiting for more keys").color(Color::Muted)),
                    )
                    .children(render_state.bindings.iter().take(MAX_TOOLTIP_BINDINGS).map(
                        |(keystrokes, action)| {
                            h_flex()
                                .gap_2()
                                .child(KeyBinding::from_keystrokes(keystrokes.clone(), false))
                                .child(Label::new(action.clone()).size(LabelSize::Small))
                        },
                    ))
                    .when(render_state.bindings.len() > MAX_TOOLTIP_BINDINGS, |el| {
                        el.child(
                            Label::new(format!(
                                "…and {} more",
                                render_state.bindings.len() - MAX_TOOLTIP_BINDINGS
                            ))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                        )
                    })
                    .into_any_element()
            }))
            .tooltip_show_delay(Duration::ZERO)
            .on_hover(cx.listener(|this, hovered: &bool, window, cx| {
                this.set_hovered(*hovered, window, cx);
            }))
            .into_any_element()
    }
}

impl StatusItemView for PendingKeystrokesIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .status_bar
                .get_or_insert_default()
                .pending_keystrokes_indicator = Some(false);
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use command_palette::humanize_action_name;
    use gpui::{
        Action as _, Entity, FocusHandle, KeyBinding, TestAppContext, VisualTestContext, actions,
    };

    use super::*;

    actions!(
        pending_keystrokes_indicator_test,
        [ShorterBinding, LongerBinding, LongestBinding]
    );

    fn timed_bindings() -> [KeyBinding; 2] {
        [
            KeyBinding::new(
                "ctrl-b",
                ShorterBinding,
                Some("PendingKeystrokesIndicatorTest"),
            ),
            KeyBinding::new(
                "ctrl-b h",
                LongerBinding,
                Some("PendingKeystrokesIndicatorTest"),
            ),
        ]
    }

    fn nested_timed_bindings() -> [KeyBinding; 3] {
        [
            KeyBinding::new(
                "ctrl-b",
                ShorterBinding,
                Some("PendingKeystrokesIndicatorTest"),
            ),
            KeyBinding::new(
                "ctrl-b h",
                LongerBinding,
                Some("PendingKeystrokesIndicatorTest"),
            ),
            KeyBinding::new(
                "ctrl-b h j",
                LongestBinding,
                Some("PendingKeystrokesIndicatorTest"),
            ),
        ]
    }

    struct TestView {
        focus_handle: FocusHandle,
    }

    #[derive(Debug, PartialEq)]
    struct IndicatorSnapshot {
        keystrokes: Vec<String>,
        generation: u64,
        bindings: Vec<(Vec<String>, String)>,
        timeout_paused: bool,
    }

    fn indicator_snapshot(indicator: &PendingKeystrokesIndicator) -> Option<IndicatorSnapshot> {
        indicator
            .render_state()
            .map(|render_state| IndicatorSnapshot {
                keystrokes: render_state
                    .keystrokes
                    .iter()
                    .map(|keystroke| keystroke.inner().unparse())
                    .collect(),
                generation: render_state.pending_input_generation,
                bindings: render_state
                    .bindings
                    .iter()
                    .map(|(keystrokes, action)| {
                        (
                            keystrokes
                                .iter()
                                .map(|keystroke| keystroke.inner().unparse())
                                .collect(),
                            action.to_string(),
                        )
                    })
                    .collect(),
                timeout_paused: render_state.timeout_paused,
            })
    }

    fn set_indicator_hovered(
        indicator: &Entity<PendingKeystrokesIndicator>,
        hovered: bool,
        cx: &mut VisualTestContext,
    ) {
        cx.update(|window, cx| {
            indicator.update(cx, |indicator, cx| {
                indicator.set_hovered(hovered, window, cx);
            });
        });
    }

    fn setup_indicator_test(
        cx: &mut TestAppContext,
        bindings: impl IntoIterator<Item = KeyBinding>,
    ) -> (Entity<PendingKeystrokesIndicator>, &mut VisualTestContext) {
        cx.update(|cx| {
            settings::init(cx);
            cx.bind_keys(bindings);
        });

        let (test_view, cx) = cx.add_window_view(|_, cx| TestView {
            focus_handle: cx.focus_handle(),
        });
        let indicator =
            cx.update(|window, cx| cx.new(|cx| PendingKeystrokesIndicator::new(window, cx)));
        let focus_handle = test_view.read_with(cx, |test_view, _| test_view.focus_handle.clone());
        cx.update(|window, cx| {
            window.focus(&focus_handle, cx);
            window.activate_window();
        });

        (indicator, cx)
    }

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .key_context("PendingKeystrokesIndicatorTest")
                .track_focus(&self.focus_handle)
                .on_action(|_: &ShorterBinding, _, _| {})
                .on_action(|_: &LongerBinding, _, _| {})
                .on_action(|_: &LongestBinding, _, _| {})
        }
    }

    #[gpui::test]
    fn test_indicator_tracks_pending_input(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(cx, nested_timed_bindings());

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();

        let first_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("pending input snapshot");
        assert_eq!(first_render_state.keystrokes, vec!["ctrl-b"]);
        assert_eq!(
            first_render_state.bindings,
            vec![
                (
                    vec!["h".to_string()],
                    humanize_action_name(LongerBinding.name()),
                ),
                (
                    vec!["h".to_string(), "j".to_string()],
                    humanize_action_name(LongestBinding.name()),
                ),
            ]
        );

        cx.simulate_keystrokes("h");
        cx.run_until_parked();

        let second_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("updated pending input snapshot");
        assert_eq!(second_render_state.keystrokes, vec!["ctrl-b", "h"]);
        assert!(second_render_state.generation > first_render_state.generation);
        assert_eq!(
            second_render_state.bindings,
            vec![(
                vec!["j".to_string()],
                humanize_action_name(LongestBinding.name()),
            )]
        );

        cx.simulate_keystrokes("j");
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
    }

    #[gpui::test]
    fn test_hover_pauses_and_resumes_timeout(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(cx, timed_bindings());

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();

        set_indicator_hovered(&indicator, true, cx);
        cx.run_until_parked();
        let paused_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("paused pending input");
        assert!(paused_render_state.timeout_paused);

        set_indicator_hovered(&indicator, false, cx);
        cx.run_until_parked();
        let resumed_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("resumed pending input");
        assert!(!resumed_render_state.timeout_paused);

        cx.simulate_keystrokes("h");
        cx.run_until_parked();
    }

    #[gpui::test]
    fn test_disabling_indicator_releases_timeout_pause(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(cx, timed_bindings());

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();
        set_indicator_hovered(&indicator, true, cx);
        cx.run_until_parked();

        cx.update(|_, cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(
                        r#"{"status_bar":{"pending_keystrokes_indicator":false}}"#,
                        cx,
                    )
                    .expect("valid test settings");
            });
        });
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
        cx.update(|window, _| {
            let timeout = window
                .pending_input()
                .and_then(|pending_input| pending_input.timeout())
                .expect("pending input timeout");
            assert!(!timeout.is_paused());
        });

        cx.simulate_keystrokes("h");
        cx.run_until_parked();
    }

    #[gpui::test]
    fn test_indicator_ignores_pending_input_without_timeout(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(
            cx,
            [KeyBinding::new(
                "ctrl-b h",
                LongerBinding,
                Some("PendingKeystrokesIndicatorTest"),
            )],
        );

        let notification_count = Rc::new(Cell::new(0));
        let _notification_subscription = cx.update({
            let indicator = indicator.clone();
            let notification_count = notification_count.clone();
            move |_, cx| {
                cx.observe(&indicator, move |_, _| {
                    notification_count.set(notification_count.get() + 1);
                })
            }
        });

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();

        cx.update(|window, _| {
            let pending_input = window.pending_input().expect("pending input");
            assert!(pending_input.timeout().is_none());
        });
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
        assert_eq!(notification_count.get(), 0);
    }
}
