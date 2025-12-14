use std::sync::Arc;

use gpui::{
    AnyView, App, AppContext, EntityId, MouseButton, Pixels, Render, StatefulInteractiveElement,
    WeakEntity, deferred, px,
};
use ui::{
    ActiveTheme as _, Clickable, Context, DynamicSpacing, FluentBuilder as _, IconButton, IconName,
    IconSize, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, Styled as _,
    Tab, Window, div,
};

use crate::{DockPosition, PanelHandle, Workspace};

pub(crate) const UTILITY_PANE_RESIZE_HANDLE_SIZE: Pixels = px(6.0);
pub(crate) const UTILITY_PANE_MIN_WIDTH: Pixels = px(20.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UtilityPaneSlot {
    Left,
    Right,
}

#[derive(Debug, Default, Clone)]
pub struct UtilityPaneState {
    pub left_slot: Option<EntityId>,
    pub right_slot: Option<EntityId>,
}

#[derive(Clone)]
pub struct DraggedUtilityPane(pub UtilityPaneSlot);

impl Render for DraggedUtilityPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

#[derive(Debug, Clone)]
pub struct UtilityPane {
    pub view: AnyView,
    pub expanded: bool,
}

pub fn utility_slot_for_dock_position(position: DockPosition) -> UtilityPaneSlot {
    match position {
        DockPosition::Left => UtilityPaneSlot::Left,
        DockPosition::Right => UtilityPaneSlot::Right,
        DockPosition::Bottom => UtilityPaneSlot::Left,
    }
}

impl Workspace {
    pub fn panel_for_utility_slot(
        &self,
        slot: UtilityPaneSlot,
        cx: &App,
    ) -> Option<Arc<dyn PanelHandle>> {
        let panel_id = match slot {
            UtilityPaneSlot::Left => self.utility_pane_state.left_slot?,
            UtilityPaneSlot::Right => self.utility_pane_state.right_slot?,
        };

        for dock in [&self.left_dock, &self.bottom_dock, &self.right_dock] {
            if let Some(panel) = dock.read(cx).panel_for_id(panel_id) {
                return Some(panel.clone());
            }
        }
        None
    }

    pub fn utility_pane_for_slot(
        &self,
        slot: UtilityPaneSlot,
        window: &Window,
        cx: &App,
    ) -> Option<UtilityPane> {
        let panel = self.panel_for_utility_slot(slot, cx)?;
        let view = panel.utility_pane(window, cx)?;
        let expanded = panel.utility_pane_expanded(cx);
        Some(UtilityPane { view, expanded })
    }

    pub fn toggle_utility_pane(
        &mut self,
        slot: UtilityPaneSlot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(panel) = self.panel_for_utility_slot(slot, cx) {
            let current = panel.utility_pane_expanded(cx);
            panel.set_utility_pane_expanded(!current, cx);
        }
        cx.notify();
        self.serialize_workspace(window, cx);
    }

    pub fn register_utility_pane(
        &mut self,
        slot: UtilityPaneSlot,
        provider_panel_id: EntityId,
        cx: &mut Context<Self>,
    ) {
        match slot {
            UtilityPaneSlot::Left => {
                self.utility_pane_state.left_slot = Some(provider_panel_id);
            }
            UtilityPaneSlot::Right => {
                self.utility_pane_state.right_slot = Some(provider_panel_id);
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
                .utility_pane_state
                .left_slot
                .is_some_and(|id| id == provider_panel_id),
            UtilityPaneSlot::Right => self
                .utility_pane_state
                .right_slot
                .is_some_and(|id| id == provider_panel_id),
        };

        if should_clear {
            match slot {
                UtilityPaneSlot::Left => {
                    self.utility_pane_state.left_slot = None;
                }
                UtilityPaneSlot::Right => {
                    self.utility_pane_state.right_slot = None;
                }
            }
            cx.notify();
        }
    }

    pub fn resize_utility_pane(
        &mut self,
        slot: UtilityPaneSlot,
        new_width: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(panel) = self.panel_for_utility_slot(slot, cx) {
            let max_width = self.max_utility_pane_width(window, cx);
            let width = new_width.max(UTILITY_PANE_MIN_WIDTH).min(max_width);
            panel.set_utility_pane_width(Some(width), cx);
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
        if let Some(panel) = self.panel_for_utility_slot(slot, cx) {
            panel.set_utility_pane_width(None, cx);
            cx.notify();
            self.serialize_workspace(window, cx);
        }
    }
}

#[derive(IntoElement)]
pub struct UtilityPaneFrame {
    workspace: WeakEntity<Workspace>,
    slot: UtilityPaneSlot,
    view: AnyView,
}

impl UtilityPaneFrame {
    pub fn new(slot: UtilityPaneSlot, view: AnyView, cx: &mut Context<Workspace>) -> Self {
        let workspace = cx.weak_entity();
        Self {
            workspace,
            slot,
            view,
        }
    }
}

impl RenderOnce for UtilityPaneFrame {
    fn render(self, _window: &mut Window, cx: &mut ui::App) -> impl IntoElement {
        let workspace = self.workspace.clone();
        let slot = self.slot;

        let Some(width) = workspace.upgrade().and_then(|ws| {
            ws.read_with(cx, |ws, cx| {
                ws.panel_for_utility_slot(slot, cx)
                    .map(|panel| panel.utility_pane_width(cx))
            })
        }) else {
            return gpui::Empty.into_any_element();
        };

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
            .child(
                div()
                    .pt_1()
                    .id(match self.slot {
                        UtilityPaneSlot::Left => "utility-pane-left",
                        UtilityPaneSlot::Right => "utility-pane-right",
                    })
                    .flex()
                    .flex_none()
                    .w_full()
                    .h(Tab::container_height(cx))
                    .when(self.slot == UtilityPaneSlot::Left, |this| {
                        let workspace = workspace.clone();
                        this.child(
                            div().px(DynamicSpacing::Base06.rems(cx)).child(
                                IconButton::new("open_utility_pane", IconName::Thread)
                                    .icon_size(IconSize::Small)
                                    .on_click(move |_, window, cx| {
                                        workspace
                                            .update(cx, |workspace, cx| {
                                                workspace.toggle_utility_pane(slot, window, cx)
                                            })
                                            .ok();
                                    }),
                            ),
                        )
                    })
                    .when(self.slot == UtilityPaneSlot::Right, |this| {
                        let workspace = workspace.clone();
                        this.flex_row_reverse().child(
                            div().px(DynamicSpacing::Base06.rems(cx)).child(
                                IconButton::new("open_utility_pane", IconName::Thread)
                                    .icon_size(IconSize::Small)
                                    .on_click(move |_, window, cx| {
                                        workspace
                                            .update(cx, |workspace, cx| {
                                                workspace.toggle_utility_pane(slot, window, cx)
                                            })
                                            .ok();
                                    }),
                            ),
                        )
                    }),
            )
            .child(create_resize_handle())
            .child(self.view)
            .into_any_element()
    }
}
