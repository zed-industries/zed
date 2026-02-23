use gpui::{Action, Context, IntoElement, Render, WeakEntity, Window};
use ui::{ButtonLike, Icon, IconName, IconSize, Tooltip, h_flex, prelude::*};
use workspace::{StatusItemView, Workspace};

use crate::git_panel::Open;

pub struct GitGraphIndicator {
    workspace: WeakEntity<Workspace>,
}

impl GitGraphIndicator {
    pub fn new(workspace: &Workspace, _cx: &mut Context<Self>) -> Self {
        Self {
            workspace: workspace.weak_handle(),
        }
    }
}

impl Render for GitGraphIndicator {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .child(
                ButtonLike::new("git-graph-indicator")
                    .child(
                        Icon::new(IconName::GitGraph)
                            .size(IconSize::Small),
                    )
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action("Open Git Graph", &Open, cx)
                    })
                    .on_click({
                        let workspace = self.workspace.clone();
                        move |_, window, cx| {
                            if let Some(workspace) = workspace.upgrade() {
                                workspace.update(cx, |_workspace, cx| {
                                    window.dispatch_action(Open.boxed_clone(), cx);
                                });
                            }
                        }
                    }),
            )
    }
}

impl StatusItemView for GitGraphIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn workspace::ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
