use gpui::WeakEntity;
use ui::{
    ActiveTheme as _, Clickable, Context, DynamicSpacing, IconButton, IconName, IconSize,
    InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce, Styled as _, Tab, Window,
    div, px,
};

use crate::Workspace;

impl Workspace {
    pub fn toggle_panelet(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.panelet = !self.panelet;
        // self.
    }
}

#[derive(IntoElement)]
pub struct Panelet {
    workspace: WeakEntity<Workspace>,
}

impl Panelet {
    pub fn new(cx: &mut Context<Workspace>) -> Self {
        let workspace = cx.weak_entity();
        Self { workspace }
    }
}

impl RenderOnce for Panelet {
    fn render(self, _window: &mut Window, cx: &mut ui::App) -> impl IntoElement {
        div()
            .h_full()
            .bg(cx.theme().colors().tab_bar_background)
            .w(px(400.0))
            .border_color(cx.theme().colors().border)
            .border_r_1()
            .child(
                div()
                    .pt_1()
                    .id("panelet")
                    .flex()
                    .flex_none()
                    .w_full()
                    .h(Tab::container_height(cx))
                    .child(
                        div().px(DynamicSpacing::Base06.rems(cx)).child(
                            IconButton::new("open_panelet", IconName::Thread)
                                .icon_size(IconSize::Small)
                                .on_click(move |_, window, cx| {
                                    self.workspace
                                        .update(cx, |workspace, cx| {
                                            workspace.toggle_panelet(window, cx)
                                        })
                                        .ok();
                                }),
                        ),
                    ),
            )
        // .child(
        //     // todo!(put content here)
        // )
    }
}
