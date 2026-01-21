use gpui::{
    AppContext as _, EntityId, MouseButton, Pixels, Render, StatefulInteractiveElement,
    Subscription, WeakEntity, deferred, px,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UtilityPaneSlot {
    Left,
    Right,
}

struct UtilityPaneSlotState {
    panel_id: EntityId,
    utility_pane: Box<dyn UtilityPaneHandle>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Default)]
pub struct UtilityPaneState {
    left_slot: Option<UtilityPaneSlotState>,
    right_slot: Option<UtilityPaneSlotState>,
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
        match slot {
            UtilityPaneSlot::Left => self
                .utility_panes
                .left_slot
                .as_ref()
                .map(|s| s.utility_pane.as_ref()),
            UtilityPaneSlot::Right => self
                .utility_panes
                .right_slot
                .as_ref()
                .map(|s| s.utility_pane.as_ref()),
        }
    }

    pub fn toggle_utility_pane(
        &mut self,
        slot: UtilityPaneSlot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(handle) = self.utility_pane(slot) {
            let current = handle.expanded(cx);
            handle.set_expanded(!current, cx);
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

        match slot {
            UtilityPaneSlot::Left => {
                self.utility_panes.left_slot = Some(UtilityPaneSlotState {
                    panel_id,
                    utility_pane: boxed_handle,
                    _subscriptions: subscriptions,
                });
            }
            UtilityPaneSlot::Right => {
                self.utility_panes.right_slot = Some(UtilityPaneSlotState {
                    panel_id,
                    utility_pane: boxed_handle,
                    _subscriptions: subscriptions,
                });
            }
        }
        cx.notify();
    }

    pub fn clear_utility_pane(&mut self, slot: UtilityPaneSlot, cx: &mut Context<Self>) {
        match slot {
            UtilityPaneSlot::Left => {
                self.utility_panes.left_slot = None;
            }
            UtilityPaneSlot::Right => {
                self.utility_panes.right_slot = None;
            }
        }
        cx.notify();
    }

    pub fn clear_utility_pane_if_provider(
        &mut self,
        slot: UtilityPaneSlot,
        provider_panel_id: EntityId,
        cx: &mut Context<Self>,
    ) {
        let should_clear = match slot {
            UtilityPaneSlot::Left => self
                .utility_panes
                .left_slot
                .as_ref()
                .is_some_and(|slot| slot.panel_id == provider_panel_id),
            UtilityPaneSlot::Right => self
                .utility_panes
                .right_slot
                .as_ref()
                .is_some_and(|slot| slot.panel_id == provider_panel_id),
        };

        if should_clear {
            self.clear_utility_pane(slot, cx);
        }
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
}

impl UtilityPaneFrame {
    pub fn new(
        slot: UtilityPaneSlot,
        handle: Box<dyn UtilityPaneHandle>,
        cx: &mut Context<Workspace>,
    ) -> Self {
        let workspace = cx.weak_entity();
        Self {
            workspace,
            slot,
            handle,
        }
    }
}

impl RenderOnce for UtilityPaneFrame {
    fn render(self, _window: &mut Window, cx: &mut ui::App) -> impl IntoElement {
        let workspace = self.workspace.clone();
        let slot = self.slot;
        let width = self.handle.width(cx);

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
                    move |e: &gpui::MouseUpEvent, window, cx| {
                        if e.click_count == 2 {
                            workspace_handle
                                .update(cx, |workspace, cx| {
                                    workspace.reset_utility_pane_width(slot, window, cx);
                                })
                                .ok();
                            cx.stop_propagation();
                        }
                    },
                )
                .occlude();

            match slot {
                UtilityPaneSlot::Left => deferred(
                    handle
                        .absolute()
                        .right(-UTILITY_PANE_RESIZE_HANDLE_SIZE / 2.)
                        .top(px(0.))
                        .h_full()
                        .w(UTILITY_PANE_RESIZE_HANDLE_SIZE)
                        .cursor_col_resize(),
                ),
                UtilityPaneSlot::Right => deferred(
                    handle
                        .absolute()
                        .left(-UTILITY_PANE_RESIZE_HANDLE_SIZE / 2.)
                        .top(px(0.))
                        .h_full()
                        .w(UTILITY_PANE_RESIZE_HANDLE_SIZE)
                        .cursor_col_resize(),
                ),
            }
        };

        div()
            .h_full()
            .bg(cx.theme().colors().tab_bar_background)
            .w(width)
            .border_color(cx.theme().colors().border)
            .when(self.slot == UtilityPaneSlot::Left, |this| this.border_r_1())
            .when(self.slot == UtilityPaneSlot::Right, |this| {
                this.border_l_1()
            })
            .child(create_resize_handle())
            .child(self.handle.to_any())
            .into_any_element()
    }
}
