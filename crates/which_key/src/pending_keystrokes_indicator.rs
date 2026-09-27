use gpui::{
    Action as _, Anchor, Animation, AnimationExt, App, Context, HoverListenerMode,
    KeybindingKeystroke, Render, ScrollHandle, Subscription, Task, Window, anchored, deferred,
};
use settings::{Settings, SettingsStore};
use std::{rc::Rc, time::Duration};
use ui::{
    ButtonLike, CircularProgress, KeyBinding, KeyBindingStyle, prelude::*, tooltip_container,
};
use util::ResultExt;
use vim_mode_setting::{HelixModeSetting, VimModeSetting};
use workspace::{HideStatusItem, StatusBarSettings, StatusItemView, item::ItemHandle};

use crate::{
    bindings_for_pending_input, map_pending_keystrokes,
    pending_bindings::{PendingBindingRow, PendingBindings, prepare_pending_bindings},
    which_key_settings::WhichKeySettings,
};

const POPOVER_HIDE_DELAY: Duration = Duration::from_millis(300);

/// A status bar item shown while pending input can complete a multi-stroke key binding.
pub struct PendingKeystrokesIndicator {
    render_state: Option<Rc<IndicatorRenderState>>,
    pending_input_generation: u64,
    popover: PopoverState,
    popover_scroll_handle: ScrollHandle,
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
    bindings: Rc<[PendingBindingRow]>,
    timeout: Option<IndicatorTimeout>,
}

struct IndicatorTimeout {
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
                this.update_pointer_over_state(window, cx);
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
            popover_scroll_handle: ScrollHandle::new(),
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
        let keystrokes = pending_input.keystrokes();

        let bindings = prepare_pending_bindings(bindings_for_pending_input(window, keystrokes), cx);

        let keystrokes = map_pending_keystrokes(keystrokes, cx.keyboard_mapper().as_ref());
        let pending_keys_changed = self
            .render_state
            .as_ref()
            .is_none_or(|previous| previous.keystrokes.as_ref() != keystrokes.as_slice());
        if pending_keys_changed {
            self.popover_scroll_handle.set_offset(Default::default());
        }
        // Pausing or resuming the timer also notifies observers.
        // Only a change in pending keys should close the popover early.
        if self.popover.visible && !self.popover.is_pointer_over() && pending_keys_changed {
            self.popover = PopoverState::default();
        }

        self.pending_input_generation = self.pending_input_generation.wrapping_add(1);
        self.render_state = Some(Rc::new(IndicatorRenderState {
            keystrokes: keystrokes.into(),
            pending_input_generation: self.pending_input_generation,
            bindings: bindings.into(),
            timeout: pending_input.timeout().map(|timeout| IndicatorTimeout {
                timeout_duration: timeout.duration(),
                remaining_duration: timeout.remaining(cx),
                timeout_paused: timeout.is_paused(),
            }),
        }));
        true
    }

    fn clear_render_state(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.popover = PopoverState::default();
        self.popover_scroll_handle.set_offset(Default::default());
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

        let button = ButtonLike::new("pending-keystrokes-indicator")
            .on_click(|_, window, cx| {
                window.dispatch_action(zed_actions::dev::OpenKeyContextView.boxed_clone(), cx);
            })
            .when_some(render_state.timeout.as_ref(), |button, timeout| {
                let remaining_fraction = if timeout.timeout_duration.is_zero() {
                    0.0
                } else {
                    (timeout.remaining_duration.as_secs_f32()
                        / timeout.timeout_duration.as_secs_f32())
                    .clamp(0.0, 1.0)
                };
                button.child(if cx.reduce_motion() {
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
                    if timeout.timeout_paused || timeout.remaining_duration.is_zero() {
                        progress.into_any_element()
                    } else {
                        progress
                            .with_animation(
                                (
                                    "pending-keystrokes-countdown",
                                    render_state.pending_input_generation,
                                ),
                                Animation::new(timeout.remaining_duration).with_max_fps(30.0),
                                move |progress, delta| {
                                    progress.value(remaining_fraction * (1.0 - delta))
                                },
                            )
                            .into_any_element()
                    }
                })
            })
            .child(
                KeyBinding::from_keystrokes(render_state.keystrokes.clone(), false)
                    .size(rems_from_px(12_f32))
                    .style(KeyBindingStyle::Label),
            );

        let popover = self.popover.visible.then(|| {
            let popover_render_state = render_state.clone();
            let viewport_size = window.viewport_size();
            let max_panel_width = px((f32::from(viewport_size.width) * 0.5).min(480.0));
            let max_content_height = px(f32::from(viewport_size.height) * 0.4);
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
                                el.p_0().max_w(max_panel_width).overflow_hidden().child(
                                    PendingBindings::new(
                                        "pending-keystrokes-popover-content",
                                        popover_render_state.keystrokes.clone(),
                                        popover_render_state.bindings.clone(),
                                        self.popover_scroll_handle.clone(),
                                        max_content_height,
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
        Entity, FocusHandle, KeyBinding, Modifiers, TestAppContext, VisualTestContext, actions,
        point,
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
        open_key_context_view_count: Rc<Cell<usize>>,
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
                    .map(|binding| {
                        (
                            binding
                                .keystrokes
                                .iter()
                                .map(|keystroke| keystroke.inner().unparse())
                                .collect(),
                            binding.action_name.to_string(),
                        )
                    })
                    .collect(),
                timeout_paused: render_state
                    .timeout
                    .as_ref()
                    .is_some_and(|timeout| timeout.timeout_paused),
                popover_visible: indicator.popover.visible,
                popover_pointer_over: indicator.popover.pointer_over,
            })
    }

    fn setup_indicator_test(
        cx: &mut TestAppContext,
        bindings: impl IntoIterator<Item = KeyBinding>,
    ) -> (
        Entity<PendingKeystrokesIndicator>,
        Rc<Cell<usize>>,
        &mut VisualTestContext,
    ) {
        cx.update(|cx| {
            settings::init(cx);
            WhichKeySettings::register(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            cx.bind_keys(bindings);
        });

        let open_key_context_view_count = Rc::new(Cell::new(0));
        let (test_view, cx) = cx.add_window_view({
            let open_key_context_view_count = open_key_context_view_count.clone();
            |window, cx| TestView {
                focus_handle: cx.focus_handle(),
                indicator: cx.new(|cx| PendingKeystrokesIndicator::new(window, cx)),
                open_key_context_view_count,
            }
        });
        let (focus_handle, indicator) = test_view.read_with(cx, |test_view, _| {
            (test_view.focus_handle.clone(), test_view.indicator.clone())
        });
        cx.update(|window, cx| {
            window.focus(&focus_handle, cx);
            window.activate_window();
        });

        (indicator, open_key_context_view_count, cx)
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

    fn start_popover_dismissal(cx: &mut VisualTestContext) {
        start_pending_input_and_hover_indicator(cx);
        move_pointer_over_popover(cx);
        move_pointer_outside(cx);
    }

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .key_context("PendingKeystrokesIndicatorTest")
                .track_focus(&self.focus_handle)
                .on_action(|_: &ShorterBinding, _, _| {})
                .on_action(|_: &LongerBinding, _, _| {})
                .on_action(|_: &LongestBinding, _, _| {})
                .on_action(
                    cx.listener(|this, _: &zed_actions::dev::OpenKeyContextView, _, _| {
                        this.open_key_context_view_count
                            .set(this.open_key_context_view_count.get() + 1);
                    }),
                )
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
    fn test_indicator_stays_hidden_without_pending_input(cx: &mut TestAppContext) {
        let (indicator, _, cx) = setup_indicator_test(cx, timed_bindings());
        cx.run_until_parked();

        // Before any input, neither the indicator's state nor its rendered element should exist.
        cx.update(|window, _| assert!(!window.has_pending_keystrokes()));
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
        assert!(cx.debug_bounds("PENDING_KEYSTROKES_INDICATOR").is_none());

        // "x" starts no chord in this keymap, so ordinary input must keep the indicator hidden.
        cx.simulate_keystrokes("x");
        cx.run_until_parked();

        cx.update(|window, _| assert!(!window.has_pending_keystrokes()));
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
        assert!(cx.debug_bounds("PENDING_KEYSTROKES_INDICATOR").is_none());
    }

    #[gpui::test]
    fn test_indicator_tracks_pending_input(cx: &mut TestAppContext) {
        let (indicator, _, cx) = setup_indicator_test(cx, nested_timed_bindings());

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();

        let first_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("pending input snapshot");
        assert_eq!(first_render_state.keystrokes, vec!["ctrl-b"]);
        assert_eq!(
            first_render_state.bindings,
            vec![(vec!["h".to_string()], "+2 keybinds".to_string())]
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
        let (indicator, _, cx) = setup_indicator_test(
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
        let (indicator, _, cx) = setup_indicator_test(cx, timed_bindings());

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
    fn test_clicking_indicator_opens_key_context_view(cx: &mut TestAppContext) {
        let (_, open_key_context_view_count, cx) = setup_indicator_test(cx, timed_bindings());

        cx.simulate_keystrokes("ctrl-b");
        cx.run_until_parked();

        let indicator_bounds = cx
            .debug_bounds("PENDING_KEYSTROKES_INDICATOR")
            .expect("rendered pending keystrokes indicator");
        cx.simulate_click(indicator_bounds.center(), Modifiers::none());

        assert_eq!(open_key_context_view_count.get(), 1);
    }

    #[gpui::test]
    fn test_hovering_indicator_opens_popover_and_pauses_timeout(cx: &mut TestAppContext) {
        let (indicator, _, cx) = setup_indicator_test(cx, nested_timed_bindings());
        start_pending_input_and_hover_indicator(cx);

        let paused_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("paused pending input");
        assert!(paused_render_state.timeout_paused);
        assert!(paused_render_state.popover_visible);
    }

    #[gpui::test]
    fn test_popover_is_positioned_above_indicator(cx: &mut TestAppContext) {
        let (_, _, cx) = setup_indicator_test(cx, nested_timed_bindings());
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
        let (indicator, _, cx) = setup_indicator_test(cx, nested_timed_bindings());
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
    fn test_open_popover_tracks_pending_input_lifecycle(cx: &mut TestAppContext) {
        let (indicator, _, cx) = setup_indicator_test(cx, nested_timed_bindings());
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

        cx.simulate_keystrokes("j");
        cx.run_until_parked();

        cx.update(|window, _| assert!(!window.has_pending_keystrokes()));
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
        assert!(cx.debug_bounds("PENDING_KEYSTROKES_POPOVER").is_none());
    }

    #[gpui::test]
    fn test_hover_pauses_timeout_when_untimed_input_becomes_timed(cx: &mut TestAppContext) {
        let (indicator, _, cx) =
            setup_indicator_test(cx, nested_timed_bindings().into_iter().skip(1));
        start_pending_input_and_hover_indicator(cx);
        move_pointer_over_popover(cx);
        cx.update(|window, _| {
            assert!(
                window
                    .pending_input()
                    .expect("untimed prefix")
                    .timeout()
                    .is_none()
            );
        });

        cx.simulate_keystrokes("h");
        cx.run_until_parked();
        cx.update(|window, _| {
            assert!(
                window
                    .pending_input()
                    .expect("timed prefix")
                    .timeout()
                    .expect("new timeout")
                    .is_paused()
            );
        });

        cx.executor().advance_clock(Duration::from_secs(2));
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_some()));
        cx.update(|window, _| assert!(window.has_pending_keystrokes()));

        move_pointer_outside(cx);
        cx.executor().advance_clock(POPOVER_HIDE_DELAY);
        cx.run_until_parked();
        cx.update(|window, _| {
            assert!(
                !window
                    .pending_input()
                    .expect("pending input after leaving popover")
                    .timeout()
                    .expect("resumed timeout")
                    .is_paused()
            );
        });

        cx.simulate_keystrokes("j");
        cx.run_until_parked();
        cx.update(|window, _| assert!(!window.has_pending_keystrokes()));
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
        assert!(cx.debug_bounds("PENDING_KEYSTROKES_POPOVER").is_none());
    }

    #[gpui::test]
    fn test_timed_chord_progress_dismisses_popover_during_dismissal_delay(cx: &mut TestAppContext) {
        // The standalone ctrl-b binding gives the initial prefix a timeout, which hovering pauses.
        assert_chord_progress_dismisses_popover(cx, nested_timed_bindings());
    }

    #[gpui::test]
    fn test_newly_timed_chord_progress_dismisses_popover_during_dismissal_delay(
        cx: &mut TestAppContext,
    ) {
        // Without standalone ctrl-b, the initial prefix has no timeout. Pressing h creates one
        // because ctrl-b h is both a complete binding and a prefix of ctrl-b h j.
        assert_chord_progress_dismisses_popover(cx, nested_timed_bindings().into_iter().skip(1));
    }

    fn assert_chord_progress_dismisses_popover(
        cx: &mut TestAppContext,
        bindings: impl IntoIterator<Item = KeyBinding>,
    ) {
        let (indicator, _, cx) = setup_indicator_test(cx, bindings);
        start_popover_dismissal(cx);

        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();

        // The pointer has left, but only 100 ms of the 300 ms dismissal delay has elapsed.
        indicator.read_with(cx, |indicator, _| {
            assert!(indicator.popover.visible);
            assert!(indicator.popover.hide_task.is_some());
        });

        // Continuing the chord must close the popover now, without waiting for the delay.
        cx.simulate_keystrokes("h");
        cx.run_until_parked();

        indicator.read_with(cx, |indicator, _| {
            assert!(!indicator.popover.visible);
            assert!(indicator.popover.hide_task.is_none());
        });

        // Both a previously paused timeout and a newly created one must run for a full
        // second from this keypress, without waiting for the popover's dismissal delay.
        cx.update(|window, cx| {
            let timeout = window
                .pending_input()
                .expect("pending chord")
                .timeout()
                .expect("timed chord");
            assert!(!timeout.is_paused());
            assert_eq!(timeout.remaining(cx), Duration::from_secs(1));
        });

        cx.executor().advance_clock(Duration::from_millis(999));
        cx.run_until_parked();

        // The chord must still be pending just before its one-second timeout.
        cx.update(|window, _| assert!(window.has_pending_keystrokes()));

        cx.executor().advance_clock(Duration::from_millis(1));
        cx.run_until_parked();

        // At exactly one second, the timeout must clear pending input and the indicator.
        cx.update(|window, _| assert!(!window.has_pending_keystrokes()));
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
    }

    #[gpui::test]
    fn test_timeout_notifications_preserve_popover_dismissal_delay(cx: &mut TestAppContext) {
        let (indicator, _, cx) = setup_indicator_test(cx, nested_timed_bindings());
        start_popover_dismissal(cx);

        cx.executor().advance_clock(Duration::from_millis(100));
        cx.run_until_parked();

        // Trigger notifications without changing the pending keys. They must neither
        // close the popover early nor restart its 300 ms dismissal delay.
        cx.update(|window, cx| {
            assert!(window.set_pending_input_timeout_paused(&indicator, false, cx));
            assert!(window.set_pending_input_timeout_paused(&indicator, true, cx));
        });
        cx.run_until_parked();

        assert!(indicator.read_with(cx, |indicator, _| indicator.popover.visible));

        cx.executor().advance_clock(Duration::from_millis(199));
        cx.run_until_parked();

        // At 299 ms since the pointer left, the popover must not have closed early.
        assert!(indicator.read_with(cx, |indicator, _| indicator.popover.visible));

        cx.executor().advance_clock(Duration::from_millis(1));
        cx.run_until_parked();

        // It must close at the original 300 ms deadline. Restarting the delay when
        // the notifications arrived at 100 ms would leave it open until 400 ms.
        assert!(!indicator.read_with(cx, |indicator, _| indicator.popover.visible));
    }

    #[gpui::test]
    fn test_popover_hides_and_timeout_resumes_after_delay(cx: &mut TestAppContext) {
        let (indicator, _, cx) = setup_indicator_test(cx, nested_timed_bindings());
        start_popover_dismissal(cx);

        cx.executor()
            .advance_clock(POPOVER_HIDE_DELAY - Duration::from_millis(1));
        cx.run_until_parked();

        let dismissing_render_state = indicator
            .read_with(cx, |indicator, _| indicator_snapshot(indicator))
            .expect("pending input during popover dismissal delay");
        assert!(dismissing_render_state.timeout_paused);
        assert!(dismissing_render_state.popover_visible);

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
        let (indicator, _, cx) = setup_indicator_test(cx, timed_bindings());

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
        cx.update(|window, _| assert!(!window.has_pending_keystrokes()));
    }

    #[gpui::test]
    fn test_indicator_shows_pending_input_without_timeout(cx: &mut TestAppContext) {
        let (indicator, _, cx) = setup_indicator_test(
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
        // Untimed input must still show the pending keys and matching binding,
        // without countdown state.
        let snapshot = indicator
            .read_with(cx, |indicator, _| {
                assert!(
                    indicator
                        .render_state()
                        .expect("pending input")
                        .timeout
                        .is_none()
                );
                indicator_snapshot(indicator)
            })
            .expect("pending input snapshot");
        assert_eq!(snapshot.keystrokes, vec!["ctrl-b"]);
        assert_eq!(
            snapshot.bindings,
            vec![(
                vec!["h".to_string()],
                humanize_action_name(LongerBinding.name()),
            )]
        );
        assert!(cx.debug_bounds("PENDING_KEYSTROKES_INDICATOR").is_some());
        assert!(notification_count.get() > 0);

        // Waiting longer than the usual one-second timeout must not hide an untimed chord.
        cx.executor().advance_clock(Duration::from_secs(2));
        cx.run_until_parked();
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_some()));

        // Completing the chord must clear pending input and hide the indicator.
        cx.simulate_keystrokes("h");
        cx.run_until_parked();
        cx.update(|window, _| assert!(!window.has_pending_keystrokes()));
        assert!(indicator.read_with(cx, |indicator, _| indicator.render_state().is_none()));
    }
}
