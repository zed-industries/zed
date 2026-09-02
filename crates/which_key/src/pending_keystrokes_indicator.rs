use gpui::{
    Anchor, Animation, AnimationExt, App, Context, HoverListenerMode, KeybindingKeystroke, Render,
    Subscription, Task, Window, anchored, deferred,
};
use settings::{Settings, SettingsStore};
use std::{rc::Rc, time::Duration};
use ui::{
    ButtonLike, CircularProgress, KeyBinding, KeyBindingStyle, prelude::*,
    text_for_keybinding_keystrokes, tooltip_container,
};
use util::ResultExt;
use vim_mode_setting::{HelixModeSetting, VimModeSetting};
use workspace::{HideStatusItem, StatusBarSettings, StatusItemView, item::ItemHandle};

use crate::{
    bindings_for_pending_input, map_pending_keystrokes, which_key_settings::WhichKeySettings,
};

const MAX_TOOLTIP_BINDINGS: usize = 10;
const POPOVER_HIDE_DELAY: Duration = Duration::from_millis(300);

/// A status bar item shown while timed pending input can complete a multi-stroke key binding.
pub struct PendingKeystrokesIndicator {
    render_state: Option<Rc<IndicatorRenderState>>,
    pending_input_generation: u64,
    popover: PopoverState,
    _pending_input_subscription: Subscription,
    _settings_subscription: Subscription,
}

#[derive(Default)]
struct PopoverState {
    indicator_pointer_over: bool,
    pointer_over: bool,
    visible: bool,
    hide_task: Option<Task<()>>,
}

impl PopoverState {
    fn is_pointer_over(&self) -> bool {
        self.indicator_pointer_over || self.pointer_over
    }
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
        let mut popover_enabled = Self::popover_enabled(cx);
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
                let new_enabled = Self::enabled(cx);
                let new_popover_enabled = Self::popover_enabled(cx);
                if new_enabled == enabled && new_popover_enabled == popover_enabled {
                    return;
                }

                enabled = new_enabled;
                popover_enabled = new_popover_enabled;
                if !new_popover_enabled {
                    this.popover.pointer_over = false;
                    this.popover.visible = false;
                    this.popover.hide_task.take();
                }
                if this.refresh_render_state(window, cx) {
                    cx.notify();
                }
                this.update_pointer_over_state(window, cx);
            });

        Self {
            render_state: None,
            pending_input_generation: 0,
            popover: PopoverState::default(),
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

    fn popover_enabled(cx: &App) -> bool {
        !WhichKeySettings::get_global(cx).enabled
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
        self.popover = PopoverState::default();
        window.set_pending_input_timeout_paused(&cx.entity(), false, cx);
        self.render_state.take().is_some()
    }

    fn render_state(&self) -> Option<&Rc<IndicatorRenderState>> {
        self.render_state.as_ref()
    }

    fn set_indicator_pointer_over(
        &mut self,
        pointer_over: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.popover.indicator_pointer_over = pointer_over;
        self.update_pointer_over_state(window, cx);
    }

    fn set_popover_pointer_over(
        &mut self,
        pointer_over: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.popover.pointer_over = pointer_over;
        self.update_pointer_over_state(window, cx);
    }

    fn update_pointer_over_state(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.render_state.is_none() {
            return;
        }

        if self.popover.is_pointer_over() {
            self.popover.hide_task.take();
            let was_visible = self.popover.visible;
            self.popover.visible = Self::popover_enabled(cx);
            window.set_pending_input_timeout_paused(&cx.entity(), true, cx);
            if was_visible != self.popover.visible {
                cx.notify();
            }
        } else if self.popover.visible && self.popover.hide_task.is_none() {
            window.set_pending_input_timeout_paused(&cx.entity(), true, cx);
            self.popover.hide_task = Some(cx.spawn_in(window, async move |this, cx| {
                cx.background_executor().timer(POPOVER_HIDE_DELAY).await;
                this.update_in(cx, |this, window, cx| {
                    this.popover.hide_task.take();
                    if this.popover.is_pointer_over() {
                        return;
                    }

                    this.popover.visible = false;
                    window.set_pending_input_timeout_paused(&cx.entity(), false, cx);
                    cx.notify();
                })
                .log_err();
            }));
        } else if !self.popover.visible {
            window.set_pending_input_timeout_paused(&cx.entity(), false, cx);
        }
    }
}

impl Render for PendingKeystrokesIndicator {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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

        let button = ButtonLike::new("pending-keystrokes-indicator")
            .child(if cx.reduce_motion() {
                Icon::new(IconName::CountdownTimer)
                    .size(IconSize::XSmall)
                    .color(Color::Muted)
                    .into_any_element()
            } else {
                let progress = CircularProgress::new(
                    remaining_fraction,
                    1.0,
                    rems_from_px(13_f32).to_pixels(window.rem_size()),
                    cx,
                )
                .stroke_width(rems_from_px(2_f32).to_pixels(window.rem_size()))
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
                    .size(rems_from_px(12_f32))
                    .style(KeyBindingStyle::Label),
            );

        let popover = self.popover.visible.then(|| {
            let popover_render_state = render_state.clone();
            let anchored_popover = deferred(
                anchored()
                    .anchor(Anchor::BottomRight)
                    .snap_to_window_with_margin(px(8.))
                    .child(
                        div()
                            .id("pending-keystrokes-popover")
                            .debug_selector(|| "PENDING_KEYSTROKES_POPOVER".into())
                            .pb_2()
                            .occlude()
                            .on_hover(cx.listener(|this, pointer_over: &bool, window, cx| {
                                this.set_popover_pointer_over(*pointer_over, window, cx);
                            }))
                            .hover_listener_mode(HoverListenerMode::InputModalityIndependent)
                            .child(tooltip_container(cx, |el, _| {
                                el.child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .child(KeyBinding::from_keystrokes(
                                                    popover_render_state.keystrokes.clone(),
                                                    false,
                                                ))
                                                .child(
                                                    Label::new("is waiting for more keys")
                                                        .color(Color::Muted),
                                                ),
                                        )
                                        .children(
                                            popover_render_state
                                                .bindings
                                                .iter()
                                                .take(MAX_TOOLTIP_BINDINGS)
                                                .map(|(keystrokes, action)| {
                                                    h_flex()
                                                        .gap_2()
                                                        .child(KeyBinding::from_keystrokes(
                                                            keystrokes.clone(),
                                                            false,
                                                        ))
                                                        .child(
                                                            Label::new(action.clone())
                                                                .size(LabelSize::Small),
                                                        )
                                                }),
                                        )
                                        .when(
                                            popover_render_state.bindings.len()
                                                > MAX_TOOLTIP_BINDINGS,
                                            |el| {
                                                el.child(
                                                    Label::new(format!(
                                                        "…and {} more",
                                                        popover_render_state.bindings.len()
                                                            - MAX_TOOLTIP_BINDINGS
                                                    ))
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                                )
                                            },
                                        ),
                                )
                            })),
                    ),
            )
            .with_priority(1);

            div()
                .absolute()
                .top_0()
                .right_0()
                .w_0()
                .h_0()
                .child(anchored_popover)
        });

        div()
            .id("pending-keystrokes-indicator-wrapper")
            .debug_selector(|| "PENDING_KEYSTROKES_INDICATOR".into())
            .relative()
            .child(button)
            .when_some(popover, |this, popover| this.child(popover))
            .on_hover(cx.listener(|this, pointer_over: &bool, window, cx| {
                this.set_indicator_pointer_over(*pointer_over, window, cx);
            }))
            .hover_listener_mode(HoverListenerMode::InputModalityIndependent)
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

    use super::*;
    use command_palette::humanize_action_name;
    use gpui::{
        Action as _, Entity, FocusHandle, KeyBinding, Modifiers, TestAppContext, VisualTestContext,
        actions, point,
    };

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
        indicator: Entity<PendingKeystrokesIndicator>,
    }

    #[derive(Debug, PartialEq)]
    struct IndicatorSnapshot {
        keystrokes: Vec<String>,
        generation: u64,
        bindings: Vec<(Vec<String>, String)>,
        timeout_paused: bool,
        popover_visible: bool,
        popover_pointer_over: bool,
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
                popover_visible: indicator.popover.visible,
                popover_pointer_over: indicator.popover.pointer_over,
            })
    }

    fn setup_indicator_test(
        cx: &mut TestAppContext,
        bindings: impl IntoIterator<Item = KeyBinding>,
    ) -> (Entity<PendingKeystrokesIndicator>, &mut VisualTestContext) {
        cx.update(|cx| {
            settings::init(cx);
            WhichKeySettings::register(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            cx.bind_keys(bindings);
        });

        let (test_view, cx) = cx.add_window_view(|window, cx| TestView {
            focus_handle: cx.focus_handle(),
            indicator: cx.new(|cx| PendingKeystrokesIndicator::new(window, cx)),
        });
        let (focus_handle, indicator) = test_view.read_with(cx, |test_view, _| {
            (test_view.focus_handle.clone(), test_view.indicator.clone())
        });
        cx.update(|window, cx| {
            window.focus(&focus_handle, cx);
            window.activate_window();
        });

        (indicator, cx)
    }

    fn start_pending_input_and_hover_indicator(cx: &mut VisualTestContext) {
        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();

        let indicator_bounds = cx
            .debug_bounds("PENDING_KEYSTROKES_INDICATOR")
            .expect("rendered pending keystrokes indicator");
        cx.simulate_mouse_move(indicator_bounds.center(), None, Modifiers::none());
    }

    fn move_pointer_over_popover(cx: &mut VisualTestContext) {
        let popover_bounds = cx
            .debug_bounds("PENDING_KEYSTROKES_POPOVER")
            .expect("rendered pending keystrokes popover");
        cx.simulate_mouse_move(popover_bounds.center(), None, Modifiers::none());
    }

    fn move_pointer_outside(cx: &mut VisualTestContext) {
        let outside = cx.update(|window, _| point(window.viewport_size().width - px(1.), px(1.)));
        cx.simulate_mouse_move(outside, None, Modifiers::none());
    }

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .key_context("PendingKeystrokesIndicatorTest")
                .track_focus(&self.focus_handle)
                .on_action(|_: &ShorterBinding, _, _| {})
                .on_action(|_: &LongerBinding, _, _| {})
                .on_action(|_: &LongestBinding, _, _| {})
                .child(
                    v_flex()
                        .size_full()
                        .justify_end()
                        .items_end()
                        .child(self.indicator.clone()),
                )
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
    fn test_indicator_does_not_apply_which_key_binding_filter(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(
            cx,
            [
                KeyBinding::new("g", ShorterBinding, Some("PendingKeystrokesIndicatorTest")),
                KeyBinding::new("g j", LongerBinding, Some("PendingKeystrokesIndicatorTest")),
            ],
        );

        cx.simulate_keystrokes("g");
        cx.run_until_parked();

        cx.update(|window, _| {
            let pending_keystrokes = window
                .pending_input_keystrokes()
                .expect("pending input keystrokes");
            assert!(crate::bindings_for_which_key(window, pending_keystrokes).is_empty());
        });

        let render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("pending input snapshot");
        assert_eq!(
            render_state.bindings,
            vec![(
                vec!["j".to_string()],
                humanize_action_name(LongerBinding.name()),
            )]
        );
    }

    #[gpui::test]
    fn test_which_key_disables_popover_but_keeps_indicator_and_hover_pause(
        cx: &mut TestAppContext,
    ) {
        let (indicator, cx) = setup_indicator_test(cx, timed_bindings());

        cx.update(|_, cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store
                    .set_user_settings(
                        r#"{
                            "status_bar": {"pending_keystrokes_indicator": true},
                            "which_key": {"enabled": true}
                        }"#,
                        cx,
                    )
                    .expect("valid test settings");
            });
        });
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert!(StatusBarSettings::get_global(cx).pending_keystrokes_indicator);
            assert!(WhichKeySettings::get_global(cx).enabled);
        });

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();

        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_some()));
        let indicator_bounds = cx
            .debug_bounds("PENDING_KEYSTROKES_INDICATOR")
            .expect("rendered pending keystrokes indicator");
        cx.simulate_mouse_move(indicator_bounds.center(), None, Modifiers::none());
        cx.run_until_parked();

        assert!(cx.debug_bounds("PENDING_KEYSTROKES_POPOVER").is_none());
        cx.update(|window, _| {
            let timeout = window
                .pending_input()
                .and_then(|pending_input| pending_input.timeout())
                .expect("pending input timeout");
            assert!(timeout.is_paused());
        });

        move_pointer_outside(cx);
        cx.run_until_parked();
        cx.update(|window, _| {
            let timeout = window
                .pending_input()
                .and_then(|pending_input| pending_input.timeout())
                .expect("pending input timeout");
            assert!(!timeout.is_paused());
        });
    }

    #[gpui::test]
    fn test_hovering_indicator_opens_popover_and_pauses_timeout(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(cx, nested_timed_bindings());
        start_pending_input_and_hover_indicator(cx);

        let paused_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("paused pending input");
        assert!(paused_render_state.timeout_paused);
        assert!(paused_render_state.popover_visible);
    }

    #[gpui::test]
    fn test_popover_is_positioned_above_indicator(cx: &mut TestAppContext) {
        let (_, cx) = setup_indicator_test(cx, nested_timed_bindings());
        start_pending_input_and_hover_indicator(cx);

        let indicator_bounds = cx
            .debug_bounds("PENDING_KEYSTROKES_INDICATOR")
            .expect("rendered pending keystrokes indicator");
        let popover_bounds = cx
            .debug_bounds("PENDING_KEYSTROKES_POPOVER")
            .expect("rendered pending keystrokes popover");
        assert!(
            popover_bounds.bottom() <= indicator_bounds.top(),
            "popover {popover_bounds:?} should render above indicator {indicator_bounds:?}"
        );
        assert_eq!(popover_bounds.right(), indicator_bounds.right());
    }

    #[gpui::test]
    fn test_pointer_handoff_keeps_popover_open_and_timeout_paused(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(cx, nested_timed_bindings());
        start_pending_input_and_hover_indicator(cx);
        move_pointer_over_popover(cx);

        let handoff_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("pending input during popover handoff");
        assert!(handoff_render_state.timeout_paused);
        assert!(handoff_render_state.popover_visible);
        assert!(handoff_render_state.popover_pointer_over);

        cx.executor().advance_clock(POPOVER_HIDE_DELAY);
        cx.run_until_parked();
        let stationary_popover_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("pending input while pointer remains over popover");
        assert!(stationary_popover_render_state.timeout_paused);
        assert!(stationary_popover_render_state.popover_visible);
        assert!(stationary_popover_render_state.popover_pointer_over);
    }

    #[gpui::test]
    fn test_open_popover_updates_with_pending_input(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(cx, nested_timed_bindings());
        start_pending_input_and_hover_indicator(cx);
        let initial_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("initial pending input");
        move_pointer_over_popover(cx);

        cx.simulate_keystrokes("h");
        cx.run_until_parked();
        let updated_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("updated pending input while popover is open");
        assert_eq!(updated_render_state.keystrokes, vec!["ctrl-b", "h"]);
        assert!(updated_render_state.generation > initial_render_state.generation);
        assert!(updated_render_state.timeout_paused);
        assert!(updated_render_state.popover_visible);
        assert!(cx.debug_bounds("PENDING_KEYSTROKES_POPOVER").is_some());
    }

    #[gpui::test]
    fn test_popover_hides_and_timeout_resumes_after_delay(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(cx, nested_timed_bindings());
        start_pending_input_and_hover_indicator(cx);
        move_pointer_over_popover(cx);
        move_pointer_outside(cx);

        cx.executor()
            .advance_clock(POPOVER_HIDE_DELAY - Duration::from_millis(1));
        cx.run_until_parked();
        let grace_period_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("pending input during popover dismissal grace period");
        assert!(grace_period_render_state.timeout_paused);
        assert!(grace_period_render_state.popover_visible);

        cx.executor().advance_clock(Duration::from_millis(1));
        cx.run_until_parked();
        let resumed_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("resumed pending input");
        assert!(!resumed_render_state.timeout_paused);
        assert!(!resumed_render_state.popover_visible);
    }

    #[gpui::test]
    fn test_disabling_indicator_releases_timeout_pause(cx: &mut TestAppContext) {
        let (indicator, cx) = setup_indicator_test(cx, timed_bindings());

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();
        let indicator_bounds = cx
            .debug_bounds("PENDING_KEYSTROKES_INDICATOR")
            .expect("rendered pending keystrokes indicator");
        cx.simulate_mouse_move(indicator_bounds.center(), None, Modifiers::none());

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
