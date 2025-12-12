use gpui::WeakEntity;
use ui::{
    ActiveTheme as _, Clickable, Context, DynamicSpacing, FluentBuilder as _, IconButton, IconName,
    IconSize, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, Styled as _,
    Tab, Window, div, px,
};

use crate::Workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UtilityPaneSlot {
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct UtilityPaneState {
    pub left_slot_open: bool,
    pub right_slot_open: bool,
}

impl Workspace {
    pub fn toggle_utility_pane(
        &mut self,
        slot: UtilityPaneSlot,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        match slot {
            UtilityPaneSlot::Left => {
                self.utility_pane_state.left_slot_open = !self.utility_pane_state.left_slot_open;
            }
            UtilityPaneSlot::Right => {
                self.utility_pane_state.right_slot_open = !self.utility_pane_state.right_slot_open;
            }
        }
    }
}

#[derive(IntoElement)]
pub struct UtilityPane {
    workspace: WeakEntity<Workspace>,
    slot: UtilityPaneSlot,
}

impl UtilityPane {
    pub fn new(slot: UtilityPaneSlot, cx: &mut Context<Workspace>) -> Self {
        let workspace = cx.weak_entity();
        Self { workspace, slot }
    }
}

impl RenderOnce for UtilityPane {
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
        // .child(
        //     // todo!(put content here)
        // )
    }
}
