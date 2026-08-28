use command_palette::humanize_action_name;
use gpui::{Animation, AnimationExt, App, Context, KeybindingKeystroke, Render, Window};
use settings::Settings;
use std::{rc::Rc, time::Duration};
use ui::{CircularProgress, KeyBinding, Tooltip, prelude::*, text_for_keystrokes};
use workspace::{HideStatusItem, StatusBarSettings, StatusItemView, item::ItemHandle};

use crate::FILTERED_KEYSTROKES;

const MAX_TOOLTIP_BINDINGS: usize = 10;

/// A status bar item shown while pending keystrokes both match a key binding and are a prefix of
/// longer bindings, counting down until the shorter binding is applied.
pub struct PendingKeystrokesIndicator {
    pending: Option<Rc<PendingKeystrokes>>,
    pending_input_generation: u64,
}

struct PendingKeystrokes {
    keystrokes: Rc<[KeybindingKeystroke]>,
    pending_input_generation: u64,
    bindings: Vec<(Rc<[KeybindingKeystroke]>, SharedString)>,
    timeout: Duration,
}

impl PendingKeystrokesIndicator {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe_pending_input(window, |this: &mut Self, window, cx| {
            this.update_pending(window, cx);
            cx.notify();
        })
        .detach();

        Self {
            pending: None,
            pending_input_generation: 0,
        }
    }

    fn update_pending(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pending_input) = window.pending_input() else {
            self.pending = None;
            return;
        };
        let Some(timeout) = pending_input.timeout() else {
            self.pending = None;
            return;
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
            timeout,
        }));
    }
}

impl Render for PendingKeystrokesIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).pending_keystrokes_indicator {
            return div().hidden().into_any_element();
        }
        let Some(pending) = self.pending.clone() else {
            return div().hidden().into_any_element();
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
                CircularProgress::new(1.0, 1.0, px(13.), cx)
                    .stroke_width(px(2.))
                    .progress_color(cx.theme().colors().text_muted)
                    .with_animation(
                        (
                            "pending-keystrokes-countdown",
                            pending.pending_input_generation,
                        ),
                        Animation::new(pending.timeout).with_max_fps(30.0),
                        |progress, delta| progress.value(1.0 - delta),
                    )
                    .into_any_element()
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

    use gpui::{Action as _, Entity, FocusHandle, KeyBinding, TestAppContext, actions};

    use super::*;

    actions!(
        pending_keystrokes_indicator_test,
        [ShorterBinding, LongerBinding, LongestBinding]
    );

    struct TestView {
        focus_handle: FocusHandle,
        indicator: Entity<PendingKeystrokesIndicator>,
        shorter_binding_count: Rc<Cell<usize>>,
        longer_binding_count: Rc<Cell<usize>>,
    }

    #[derive(Debug, Default, PartialEq)]
    struct PendingSnapshot {
        keystrokes: Vec<String>,
        generation: u64,
        bindings: Vec<(Vec<String>, String)>,
        timeout: Duration,
    }

    fn pending_snapshot(indicator: &PendingKeystrokesIndicator) -> Option<PendingSnapshot> {
        indicator.pending.as_ref().map(|pending| PendingSnapshot {
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
            timeout: pending.timeout,
        })
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
        let (test_view, cx) = cx.add_window_view(|window, cx| TestView {
            focus_handle: cx.focus_handle(),
            indicator: cx.new(|cx| PendingKeystrokesIndicator::new(window, cx)),
            shorter_binding_count: shorter_binding_count.clone(),
            longer_binding_count: longer_binding_count.clone(),
        });
        let (indicator, focus_handle) = test_view.read_with(cx, |test_view, _| {
            (test_view.indicator.clone(), test_view.focus_handle.clone())
        });
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

        cx.executor().advance_clock(first_pending.timeout / 2);
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.pending.is_some()));

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
        assert_eq!(
            second_pending.bindings,
            vec![(
                vec!["j".to_string()],
                humanize_action_name(LongestBinding.name()),
            )]
        );

        cx.executor().advance_clock(second_pending.timeout);
        cx.run_until_parked();

        assert!(indicator.read_with(cx, |indicator, _| indicator.pending.is_none()));
        assert_eq!(shorter_binding_count.get(), 0);
        assert_eq!(longer_binding_count.get(), 1);
    }
}
