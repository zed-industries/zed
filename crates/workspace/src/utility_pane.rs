use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, AppContext as _, EntityId, MouseButton, Pixels, Render,
    StatefulInteractiveElement, Subscription, Task, WeakEntity, deferred, ease_out_cubic, px,
};
use settings::should_reduce_motion;
use ui::{
    ActiveTheme as _, Context, FluentBuilder as _, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, Styled as _, Window, div,
};

use crate::{
    DockPosition, Workspace,
    dock::{ClosePane, MinimizePane, UtilityPane, UtilityPaneHandle},
};

pub(crate) const UTILITY_PANE_RESIZE_HANDLE_SIZE: Pixels = px(6.0);
pub(crate) const UTILITY_PANE_MIN_WIDTH: Pixels = px(20.0);
const UTILITY_PANE_OPEN_DURATION: Duration = Duration::from_millis(150);
const UTILITY_PANE_CLOSE_DURATION: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UtilityPaneSlot {
    Left,
    Right,
}

struct UtilityPaneSlotState {
    panel_id: EntityId,
    utility_pane: Box<dyn UtilityPaneHandle>,
    animation_generation: usize,
    is_closing: bool,
    _close_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Default)]
pub struct UtilityPaneState {
    left_slot: Option<UtilityPaneSlotState>,
    right_slot: Option<UtilityPaneSlotState>,
}

impl UtilityPaneState {
    fn slot(&self, slot: UtilityPaneSlot) -> &Option<UtilityPaneSlotState> {
        match slot {
            UtilityPaneSlot::Left => &self.left_slot,
            UtilityPaneSlot::Right => &self.right_slot,
        }
    }

    fn slot_mut(&mut self, slot: UtilityPaneSlot) -> &mut Option<UtilityPaneSlotState> {
        match slot {
            UtilityPaneSlot::Left => &mut self.left_slot,
            UtilityPaneSlot::Right => &mut self.right_slot,
        }
    }
}

#[derive(Clone)]
pub struct DraggedUtilityPane(pub UtilityPaneSlot);

impl Render for DraggedUtilityPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

pub fn utility_slot_for_dock_position(position: DockPosition) -> UtilityPaneSlot {
    match position {
        DockPosition::Left => UtilityPaneSlot::Left,
        DockPosition::Right => UtilityPaneSlot::Right,
        DockPosition::Bottom => UtilityPaneSlot::Left,
    }
}

impl Workspace {
    pub fn utility_pane(&self, slot: UtilityPaneSlot) -> Option<&dyn UtilityPaneHandle> {
        self.utility_panes
            .slot(slot)
            .as_ref()
            .map(|state| state.utility_pane.as_ref())
    }

    pub fn toggle_utility_pane(
        &mut self,
        slot: UtilityPaneSlot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(state) = self.utility_panes.slot_mut(slot).as_mut() {
            let current = state.utility_pane.expanded(cx);
            if current {
                state.utility_pane.set_expanded(false, cx);
            } else {
                // Cancel any pending close animation so the stale close task
                // doesn't clear the slot the user just re-expanded.
                if state.is_closing {
                    state.is_closing = false;
                    state.animation_generation = state.animation_generation.wrapping_add(1);
                    state._close_task = None;
                }
                state.utility_pane.set_expanded(true, cx);
            }
        }
        cx.notify();
        self.serialize_workspace(window, cx);
    }

    pub fn register_utility_pane<T: UtilityPane>(
        &mut self,
        slot: UtilityPaneSlot,
        panel_id: EntityId,
        handle: gpui::Entity<T>,
        cx: &mut Context<Self>,
    ) {
        let minimize_subscription =
            cx.subscribe(&handle, move |this, _, _event: &MinimizePane, cx| {
                if let Some(handle) = this.utility_pane(slot) {
                    handle.set_expanded(false, cx);
                }
                cx.notify();
            });

        let close_subscription = cx.subscribe(&handle, move |this, _, _event: &ClosePane, cx| {
            this.clear_utility_pane(slot, cx);
        });

        let subscriptions = vec![minimize_subscription, close_subscription];
        let boxed_handle: Box<dyn UtilityPaneHandle> = Box::new(handle);

        let next_generation = self
            .utility_panes
            .slot(slot)
            .as_ref()
            .map(|state| state.animation_generation.wrapping_add(1))
            .unwrap_or(0);

        *self.utility_panes.slot_mut(slot) = Some(UtilityPaneSlotState {
            panel_id,
            utility_pane: boxed_handle,
            animation_generation: next_generation,
            is_closing: false,
            _close_task: None,
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    pub fn clear_utility_pane(&mut self, slot: UtilityPaneSlot, cx: &mut Context<Self>) {
        let Some(state) = self.utility_panes.slot_mut(slot).as_mut() else {
            return;
        };

        if state.is_closing {
            return;
        }

        if should_reduce_motion(cx) {
            *self.utility_panes.slot_mut(slot) = None;
            cx.notify();
            return;
        }

        state.is_closing = true;
        // Prevents stale close tasks from clearing state after a new open/close cycle has begun.
        state.animation_generation = state.animation_generation.wrapping_add(1);
        let close_generation = state.animation_generation;
        state._close_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(UTILITY_PANE_CLOSE_DURATION)
                .await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |workspace, cx| {
                    let matches_generation = workspace
                        .utility_panes
                        .slot(slot)
                        .as_ref()
                        .is_some_and(|state| state.animation_generation == close_generation);
                    if matches_generation {
                        *workspace.utility_panes.slot_mut(slot) = None;
                        cx.notify();
                    }
                });
            }
        }));
        cx.notify();
    }

    pub fn clear_utility_pane_if_provider(
        &mut self,
        slot: UtilityPaneSlot,
        provider_panel_id: EntityId,
        cx: &mut Context<Self>,
    ) {
        let should_clear = self
            .utility_panes
            .slot(slot)
            .as_ref()
            .is_some_and(|state| state.panel_id == provider_panel_id && !state.is_closing);

        if should_clear {
            self.clear_utility_pane(slot, cx);
        }
    }

    pub(crate) fn utility_pane_frame(
        &self,
        slot: UtilityPaneSlot,
        cx: &mut Context<Self>,
    ) -> Option<UtilityPaneFrame> {
        let state = self.utility_panes.slot(slot).as_ref()?;
        let pane = &state.utility_pane;
        let should_show = pane.expanded(cx) || state.is_closing;
        if !should_show {
            return None;
        }
        Some(UtilityPaneFrame::new(
            slot,
            pane.box_clone(),
            state.animation_generation,
            state.is_closing,
            cx,
        ))
    }

    pub fn resize_utility_pane(
        &mut self,
        slot: UtilityPaneSlot,
        new_width: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(handle) = self.utility_pane(slot) {
            let max_width = self.max_utility_pane_width(window, cx);
            let width = new_width.max(UTILITY_PANE_MIN_WIDTH).min(max_width);
            handle.set_width(Some(width), cx);
            cx.notify();
            self.serialize_workspace(window, cx);
        }
    }

    pub fn reset_utility_pane_width(
        &mut self,
        slot: UtilityPaneSlot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(handle) = self.utility_pane(slot) {
            handle.set_width(None, cx);
            cx.notify();
            self.serialize_workspace(window, cx);
        }
    }
}

#[derive(IntoElement)]
pub struct UtilityPaneFrame {
    workspace: WeakEntity<Workspace>,
    slot: UtilityPaneSlot,
    handle: Box<dyn UtilityPaneHandle>,
    animation_generation: usize,
    is_closing: bool,
}

impl UtilityPaneFrame {
    pub fn new(
        slot: UtilityPaneSlot,
        handle: Box<dyn UtilityPaneHandle>,
        animation_generation: usize,
        is_closing: bool,
        cx: &mut Context<Workspace>,
    ) -> Self {
        let workspace = cx.weak_entity();
        Self {
            workspace,
            slot,
            handle,
            animation_generation,
            is_closing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utility_pane_state_slots() {
        let state = UtilityPaneState::default();
        assert!(state.slot(UtilityPaneSlot::Left).is_none());
        assert!(std::ptr::eq(state.slot(UtilityPaneSlot::Left), &state.left_slot));
        assert!(state.slot(UtilityPaneSlot::Right).is_none());
        assert!(std::ptr::eq(state.slot(UtilityPaneSlot::Right), &state.right_slot));
    }

    #[test]
    fn test_utility_slot_for_dock_position() {
        assert_eq!(
            utility_slot_for_dock_position(DockPosition::Left),
            UtilityPaneSlot::Left
        );
        assert_eq!(
            utility_slot_for_dock_position(DockPosition::Right),
            UtilityPaneSlot::Right
        );
        assert_eq!(
            utility_slot_for_dock_position(DockPosition::Bottom),
            UtilityPaneSlot::Left
        );
    }

    #[test]
    fn test_utility_pane_slot_state_initial_values() {
        let state = UtilityPaneState::default();
        assert!(state.slot(UtilityPaneSlot::Left).is_none());
        assert!(state.slot(UtilityPaneSlot::Right).is_none());
    }

    #[test]
    fn test_utility_pane_slot_mut_independence() {
        let mut state = UtilityPaneState::default();
        assert!(state.slot(UtilityPaneSlot::Left).is_none());
        assert!(state.slot(UtilityPaneSlot::Right).is_none());

        let left = state.slot_mut(UtilityPaneSlot::Left);
        assert!(left.is_none());

        let right = state.slot_mut(UtilityPaneSlot::Right);
        assert!(right.is_none());
    }

    #[test]
    fn test_utility_pane_slot_returns_correct_field() {
        let state = UtilityPaneState::default();
        assert!(std::ptr::eq(
            state.slot(UtilityPaneSlot::Left),
            &state.left_slot
        ));
        assert!(std::ptr::eq(
            state.slot(UtilityPaneSlot::Right),
            &state.right_slot
        ));
    }

    #[test]
    fn test_utility_pane_slot_mut_returns_correct_field() {
        let mut state = UtilityPaneState::default();
        assert!(std::ptr::eq(
            state.slot_mut(UtilityPaneSlot::Left),
            &state.left_slot
        ));
        assert!(std::ptr::eq(
            state.slot_mut(UtilityPaneSlot::Right),
            &state.right_slot
        ));
    }

    // Animation lifecycle tests (is_closing, animation_generation, _close_task)
    // require a full Workspace test fixture because clear_utility_pane and
    // register_utility_pane operate on &mut Workspace with a Context. These
    // transitions are best tested via integration tests that can construct a
    // Workspace, similar to the patterns in dock.rs tests.
}

impl RenderOnce for UtilityPaneFrame {
    fn render(self, _window: &mut Window, cx: &mut ui::App) -> impl IntoElement {
        let workspace = self.workspace.clone();
        let slot = self.slot;
        let width = self.handle.width(cx);
        let is_closing = self.is_closing;
        let animation_generation = self.animation_generation;
        let reduce_motion = should_reduce_motion(cx);

        let create_resize_handle = || {
            let workspace_handle = workspace.clone();
            let handle = div()
                .id(match slot {
                    UtilityPaneSlot::Left => "utility-pane-resize-handle-left",
                    UtilityPaneSlot::Right => "utility-pane-resize-handle-right",
                })
                .on_drag(DraggedUtilityPane(slot), move |pane, _, _, cx| {
                    cx.stop_propagation();
                    cx.new(|_| pane.clone())
                })
                .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    move |event: &gpui::MouseUpEvent, window, cx| {
                        if event.click_count == 2 {
                            workspace_handle
                                .update(cx, |workspace, cx| {
                                    workspace.reset_utility_pane_width(slot, window, cx);
                                })
                                .ok();
                            cx.stop_propagation();
                        }
                    },
                )
                .occlude()
                .absolute()
                .top(px(0.))
                .h_full()
                .w(UTILITY_PANE_RESIZE_HANDLE_SIZE)
                .cursor_col_resize()
                .when(slot == UtilityPaneSlot::Left, |this| {
                    this.right(-UTILITY_PANE_RESIZE_HANDLE_SIZE / 2.)
                })
                .when(slot == UtilityPaneSlot::Right, |this| {
                    this.left(-UTILITY_PANE_RESIZE_HANDLE_SIZE / 2.)
                });

            deferred(handle)
        };

        let pane_div = div()
            .h_full()
            .bg(cx.theme().colors().tab_bar_background)
            .w(width)
            .border_color(cx.theme().colors().border)
            .overflow_hidden()
            .when(self.slot == UtilityPaneSlot::Left, |this| this.border_r_1())
            .when(self.slot == UtilityPaneSlot::Right, |this| {
                this.border_l_1()
            })
            .child(
                div()
                    .min_w(width)
                    .h_full()
                    .child(self.handle.to_any()),
            )
            .when(!is_closing, |this| this.child(create_resize_handle()));

        if reduce_motion {
            pane_div.into_any_element()
        } else {
            pane_div
                .with_animation(
                    ("utility-pane-anim", animation_generation as u64),
                    Animation::new(if is_closing {
                        UTILITY_PANE_CLOSE_DURATION
                    } else {
                        UTILITY_PANE_OPEN_DURATION
                    })
                        .with_easing(ease_out_cubic),
                    {
                        let target_width = f32::from(width);
                        move |this, delta| {
                            let progress = if is_closing { 1.0 - delta } else { delta };
                            let animated_width = px(target_width * progress);
                            this.w(animated_width)
                        }
                    },
                )
                .into_any_element()
        }
    }
}
