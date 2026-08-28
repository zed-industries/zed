use command_palette::humanize_action_name;
use gpui::{
    Animation, AnimationExt, App, Context, KeybindingKeystroke, Render, Subscription, Window,
};
use settings::{Settings, SettingsStore};
use std::{rc::Rc, time::Duration};
use ui::{CircularProgress, KeyBinding, Tooltip, prelude::*, text_for_keystrokes};
use vim_mode_setting::{HelixModeSetting, VimModeSetting};
use workspace::{HideStatusItem, StatusBarSettings, StatusItemView, item::ItemHandle};

use crate::FILTERED_KEYSTROKES;

const MAX_TOOLTIP_BINDINGS: usize = 10;

/// A status bar item shown while timed pending input can complete a multi-stroke key binding.
pub struct PendingKeystrokesIndicator {
    pending: Option<Rc<PendingKeystrokes>>,
    pending_input_generation: u64,
    _pending_input_subscription: Subscription,
    _settings_subscription: Subscription,
}

struct PendingKeystrokes {
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
                if this.update_pending(window, cx) {
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
                if this.update_pending(window, cx) {
                    cx.notify();
                }
            });

        Self {
            pending: None,
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

    fn update_pending(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if !Self::enabled(cx) {
            return self.clear_pending(window, cx);
        }

        let Some(pending_input) = window.pending_input() else {
            return self.clear_pending(window, cx);
        };
        let Some(timeout) = pending_input.timeout() else {
            return self.clear_pending(window, cx);
        };
        let keystrokes = pending_input.keystrokes();

        let pending_len = keystrokes.len();
        let mut bindings = window
            .possible_bindings_for_input(keystrokes)
            .iter()
            .filter_map(|binding| {
                let binding_keystrokes = binding.keystrokes();
                if binding_keystrokes.len() <= pending_len {
                    return None;
                }
                if FILTERED_KEYSTROKES.iter().any(|filtered| {
                    binding_keystrokes.len() >= filtered.len()
                        && binding_keystrokes[..filtered.len()]
                            .iter()
                            .map(|keystroke| keystroke.inner())
                            .eq(filtered.iter())
                }) {
                    return None;
                }
                let remaining = &binding_keystrokes[pending_len..];
                let remaining_text = text_for_keystrokes(
                    &remaining
                        .iter()
                        .map(|keystroke| keystroke.inner().clone())
                        .collect::<Vec<_>>(),
                    cx,
                );
                Some((
                    remaining_text,
                    remaining.to_vec(),
                    SharedString::from(humanize_action_name(binding.action().name())),
                ))
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
        self.pending = Some(Rc::new(PendingKeystrokes {
            keystrokes: keystrokes
                .iter()
                .map(|keystroke| KeybindingKeystroke::from_keystroke(keystroke.clone()))
                .collect(),
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

    fn clear_pending(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        window.set_pending_input_timeout_paused(&cx.entity(), false, cx);
        self.pending.take().is_some()
    }

    fn pending(&self) -> Option<&Rc<PendingKeystrokes>> {
        self.pending.as_ref()
    }

    fn set_hovered(&mut self, hovered: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.pending.is_none() {
            return;
        }

        window.set_pending_input_timeout_paused(&cx.entity(), hovered, cx);
    }
}

impl Render for PendingKeystrokesIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(pending) = self.pending().cloned() else {
            return div().hidden().into_any_element();
        };
        let remaining_fraction = if pending.timeout_duration.is_zero() {
            0.0
        } else {
            (pending.remaining_duration.as_secs_f32() / pending.timeout_duration.as_secs_f32())
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
                if pending.timeout_paused || pending.remaining_duration.is_zero() {
                    progress.into_any_element()
                } else {
                    progress
                        .with_animation(
                            (
                                "pending-keystrokes-countdown",
                                pending.pending_input_generation,
                            ),
                            Animation::new(pending.remaining_duration).with_max_fps(30.0),
                            move |progress, delta| {
                                progress.value(remaining_fraction * (1.0 - delta))
                            },
                        )
                        .into_any_element()
                }
            })
            .child(
                KeyBinding::from_keystrokes(pending.keystrokes.clone(), false)
                    .size(rems_from_px(12_f32)),
            )
            .tooltip(Tooltip::element(move |_, _| {
                let pending = &pending;
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(KeyBinding::from_keystrokes(
                                pending.keystrokes.clone(),
                                false,
                            ))
                            .child(Label::new("is waiting for more keys").color(Color::Muted)),
                    )
                    .children(pending.bindings.iter().take(MAX_TOOLTIP_BINDINGS).map(
                        |(keystrokes, action)| {
                            h_flex()
                                .gap_2()
                                .child(KeyBinding::from_keystrokes(keystrokes.clone(), false))
                                .child(Label::new(action.clone()).size(LabelSize::Small))
                        },
                    ))
                    .when(pending.bindings.len() > MAX_TOOLTIP_BINDINGS, |el| {
                        el.child(
                            Label::new(format!(
                                "…and {} more",
                                pending.bindings.len() - MAX_TOOLTIP_BINDINGS
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

    use gpui::{
        Action as _, Entity, FocusHandle, KeyBinding, TestAppContext, VisualTestContext, actions,
    };

    use super::*;

    actions!(
        pending_keystrokes_indicator_test,
        [ShorterBinding, LongerBinding, LongestBinding]
    );

    struct TestView {
        focus_handle: FocusHandle,
        shorter_binding_count: Rc<Cell<usize>>,
        longer_binding_count: Rc<Cell<usize>>,
    }

    #[derive(Debug, Default, PartialEq)]
    struct PendingSnapshot {
        keystrokes: Vec<String>,
        generation: u64,
        bindings: Vec<(Vec<String>, String)>,
        timeout_duration: Duration,
        remaining_duration: Duration,
        timeout_paused: bool,
    }

    fn pending_snapshot(indicator: &PendingKeystrokesIndicator) -> Option<PendingSnapshot> {
        indicator.pending().map(|pending| PendingSnapshot {
            keystrokes: pending
                .keystrokes
                .iter()
                .map(|keystroke| keystroke.inner().unparse())
                .collect(),
            generation: pending.pending_input_generation,
            bindings: pending
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
            timeout_duration: pending.timeout_duration,
            remaining_duration: pending.remaining_duration,
            timeout_paused: pending.timeout_paused,
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

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let shorter_binding_count = self.shorter_binding_count.clone();
            let longer_binding_count = self.longer_binding_count.clone();
            div()
                .key_context("PendingKeystrokesIndicatorTest")
                .track_focus(&self.focus_handle)
                .on_action(move |_: &ShorterBinding, _, _| {
                    shorter_binding_count.set(shorter_binding_count.get() + 1);
                })
                .on_action(move |_: &LongerBinding, _, _| {
                    longer_binding_count.set(longer_binding_count.get() + 1);
                })
                .on_action(|_: &LongestBinding, _, _| {})
        }
    }

    #[gpui::test]
    fn test_indicator_tracks_pending_input_lifecycle(cx: &mut TestAppContext) {
        cx.update(|cx| {
            settings::init(cx);
            cx.bind_keys([
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
            ]);
        });

        let shorter_binding_count = Rc::new(Cell::new(0));
        let longer_binding_count = Rc::new(Cell::new(0));
        let (test_view, cx) = cx.add_window_view(|_, cx| TestView {
            focus_handle: cx.focus_handle(),
            shorter_binding_count: shorter_binding_count.clone(),
            longer_binding_count: longer_binding_count.clone(),
        });
        let indicator =
            cx.update(|window, cx| cx.new(|cx| PendingKeystrokesIndicator::new(window, cx)));
        let focus_handle = test_view.read_with(cx, |test_view, _| test_view.focus_handle.clone());
        cx.update(|window, cx| {
            window.focus(&focus_handle, cx);
            window.activate_window();
        });

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();

        let first_pending = indicator.read_with(cx, |indicator, _| pending_snapshot(indicator));
        assert!(first_pending.is_some(), "expected pending input snapshot");
        let first_pending = first_pending.unwrap_or_default();
        assert_eq!(first_pending.keystrokes, vec!["ctrl-b"]);
        assert_eq!(
            first_pending.bindings,
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

        let pending_before_unrelated_settings_update = indicator
            .read_with(cx, |indicator, _| indicator.pending().cloned())
            .expect("pending input");
        cx.update(|_, cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(r#"{"ui_font_size":15}"#, cx)
                    .expect("valid test settings");
            });
        });
        cx.run_until_parked();
        let pending_after_unrelated_settings_update = indicator
            .read_with(cx, |indicator, _| indicator.pending().cloned())
            .expect("pending input");
        assert!(Rc::ptr_eq(
            &pending_before_unrelated_settings_update,
            &pending_after_unrelated_settings_update
        ));

        cx.executor()
            .advance_clock(first_pending.timeout_duration / 2);
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_some()));

        set_indicator_hovered(&indicator, true, cx);
        cx.run_until_parked();
        let hovered_pending = indicator
            .read_with(cx, |indicator, _| pending_snapshot(indicator))
            .unwrap_or_default();
        assert!(hovered_pending.timeout_paused);
        assert_eq!(
            hovered_pending.remaining_duration,
            first_pending.timeout_duration / 2
        );

        cx.executor()
            .advance_clock(first_pending.timeout_duration * 2);
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_some()));
        assert_eq!(shorter_binding_count.get(), 0);

        cx.simulate_keystrokes("h");
        cx.run_until_parked();

        let second_pending = indicator.read_with(cx, |indicator, _| pending_snapshot(indicator));
        assert!(
            second_pending.is_some(),
            "expected updated pending input snapshot"
        );
        let second_pending = second_pending.unwrap_or_default();
        assert_eq!(second_pending.keystrokes, vec!["ctrl-b", "h"]);
        assert!(second_pending.generation > first_pending.generation);
        assert!(second_pending.timeout_paused);
        assert_eq!(
            second_pending.remaining_duration,
            second_pending.timeout_duration
        );
        assert_eq!(
            second_pending.bindings,
            vec![(
                vec!["j".to_string()],
                humanize_action_name(LongestBinding.name()),
            )]
        );

        cx.executor()
            .advance_clock(second_pending.timeout_duration * 2);
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_some()));
        assert_eq!(longer_binding_count.get(), 0);

        set_indicator_hovered(&indicator, false, cx);
        cx.run_until_parked();
        let resumed_pending = indicator
            .read_with(cx, |indicator, _| pending_snapshot(indicator))
            .unwrap_or_default();
        assert!(!resumed_pending.timeout_paused);
        assert_eq!(
            resumed_pending.remaining_duration,
            resumed_pending.timeout_duration
        );

        cx.executor()
            .advance_clock(resumed_pending.remaining_duration);
        cx.run_until_parked();

        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_none()));
        assert_eq!(shorter_binding_count.get(), 0);
        assert_eq!(longer_binding_count.get(), 1);

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();
        set_indicator_hovered(&indicator, true, cx);
        cx.run_until_parked();

        cx.update(|_, cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(r#"{"status_bar":{"experimental.show":false}}"#, cx)
                    .expect("valid test settings");
            });
        });
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_none()));
        let remaining_after_disabling = cx.update(|window, cx| {
            let timeout = window
                .pending_input()
                .and_then(|pending_input| pending_input.timeout())
                .expect("pending input timeout");
            assert!(!timeout.is_paused());
            timeout.remaining(cx)
        });
        cx.executor().advance_clock(remaining_after_disabling);
        cx.run_until_parked();
        assert_eq!(shorter_binding_count.get(), 1);

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_none()));

        cx.update(|_, cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(r#"{"status_bar":{"experimental.show":true}}"#, cx)
                    .expect("valid test settings");
            });
        });
        cx.run_until_parked();
        let pending_after_reenabling =
            indicator.read_with(cx, |indicator, _| pending_snapshot(indicator));
        assert!(
            pending_after_reenabling.is_some(),
            "expected the current pending input after re-enabling the indicator"
        );

        cx.update(|_, cx| {
            VimModeSetting::override_global(VimModeSetting(true), cx);
        });
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_none()));

        cx.update(|_, cx| {
            VimModeSetting::override_global(VimModeSetting(false), cx);
        });
        cx.run_until_parked();
        let pending_after_disabling_vim =
            indicator.read_with(cx, |indicator, _| pending_snapshot(indicator));
        assert!(
            pending_after_disabling_vim.is_some(),
            "expected the current pending input after disabling Vim mode"
        );
        set_indicator_hovered(&indicator, true, cx);
        cx.run_until_parked();
        let remaining_before_release = cx.update(|window, cx| {
            window
                .pending_input()
                .and_then(|pending_input| pending_input.timeout())
                .map(|timeout| timeout.remaining(cx))
                .expect("pending input timeout")
        });

        let weak_indicator = indicator.downgrade();
        drop(indicator);
        cx.update(|_, _| {});
        cx.run_until_parked();
        weak_indicator.assert_released();

        let remaining_after_release = cx.update(|window, cx| {
            let timeout = window
                .pending_input()
                .and_then(|pending_input| pending_input.timeout())
                .expect("pending input timeout");
            assert!(!timeout.is_paused());
            timeout.remaining(cx)
        });
        assert_eq!(remaining_after_release, remaining_before_release);
        cx.executor().advance_clock(remaining_after_release);
        cx.run_until_parked();
        assert_eq!(shorter_binding_count.get(), 2);
    }

    #[gpui::test]
    fn test_indicator_tracks_status_bar_and_helix_modes(cx: &mut TestAppContext) {
        cx.update(|cx| {
            settings::init(cx);
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(
                        r#"{"status_bar":{"experimental.show":false,"pending_keystrokes_indicator":true}}"#,
                        cx,
                    )
                    .expect("valid test settings");
            });
            cx.bind_keys([
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
            ]);
        });

        let shorter_binding_count = Rc::new(Cell::new(0));
        let longer_binding_count = Rc::new(Cell::new(0));
        let (test_view, cx) = cx.add_window_view(|_, cx| TestView {
            focus_handle: cx.focus_handle(),
            shorter_binding_count,
            longer_binding_count,
        });
        let indicator =
            cx.update(|window, cx| cx.new(|cx| PendingKeystrokesIndicator::new(window, cx)));
        let focus_handle = test_view.read_with(cx, |test_view, _| test_view.focus_handle.clone());
        cx.update(|window, cx| {
            window.focus(&focus_handle, cx);
            window.activate_window();
        });

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_none()));

        cx.update(|_, cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(
                        r#"{"status_bar":{"experimental.show":true,"pending_keystrokes_indicator":true}}"#,
                        cx,
                    )
                    .expect("valid test settings");
            });
        });
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_some()));

        cx.update(|_, cx| {
            HelixModeSetting::override_global(HelixModeSetting(true), cx);
        });
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_none()));

        cx.update(|_, cx| {
            HelixModeSetting::override_global(HelixModeSetting(false), cx);
        });
        cx.run_until_parked();
        let pending_after_disabling_helix =
            indicator.read_with(cx, |indicator, _| pending_snapshot(indicator));
        assert!(pending_after_disabling_helix.is_some());
        cx.executor().advance_clock(
            pending_after_disabling_helix
                .unwrap_or_default()
                .timeout_duration,
        );
        cx.run_until_parked();
    }

    #[gpui::test]
    fn test_indicator_ignores_pending_input_without_timeout(cx: &mut TestAppContext) {
        cx.update(|cx| {
            settings::init(cx);
            cx.bind_keys([KeyBinding::new(
                "ctrl-b h",
                LongerBinding,
                Some("PendingKeystrokesIndicatorTest"),
            )]);
        });

        let shorter_binding_count = Rc::new(Cell::new(0));
        let longer_binding_count = Rc::new(Cell::new(0));
        let (test_view, cx) = cx.add_window_view(|_, cx| TestView {
            focus_handle: cx.focus_handle(),
            shorter_binding_count,
            longer_binding_count,
        });
        let indicator =
            cx.update(|window, cx| cx.new(|cx| PendingKeystrokesIndicator::new(window, cx)));
        let focus_handle = test_view.read_with(cx, |test_view, _| test_view.focus_handle.clone());
        cx.update(|window, cx| {
            window.focus(&focus_handle, cx);
            window.activate_window();
        });

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
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending().is_none()));
        assert_eq!(notification_count.get(), 0);
    }
}
