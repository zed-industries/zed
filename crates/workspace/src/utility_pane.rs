use std::sync::Arc;

use gpui::{AnyView, App, EntityId, WeakEntity};
use ui::{
    ActiveTheme as _, Clickable, Context, DynamicSpacing, FluentBuilder as _, IconButton, IconName,
    IconSize, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, Styled as _,
    Tab, Window, div, px,
};

use crate::{DockPosition, PanelHandle, Workspace};

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

        div()
            .h_full()
            .bg(cx.theme().colors().tab_bar_background)
            .w(px(400.0))
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
            .child(self.view)
    }
}
