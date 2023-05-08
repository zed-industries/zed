use gpui::{elements::*, Entity, ModelHandle, View, ViewContext, ViewHandle, WeakViewHandle};
use project::Project;
use settings::{Settings, WorkingDirectory};
use util::ResultExt;
use workspace::{dock::Panel, Pane, Workspace};

use crate::TerminalView;

pub struct TerminalPanel {
    project: ModelHandle<Project>,
    pane: ViewHandle<Pane>,
    workspace: WeakViewHandle<Workspace>,
}

impl TerminalPanel {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        Self {
            project: workspace.project().clone(),
            pane: cx.add_view(|cx| {
                Pane::new(
                    workspace.weak_handle(),
                    workspace.app_state().background_actions,
                    cx,
                )
            }),
            workspace: workspace.weak_handle(),
        }
    }
}

impl Entity for TerminalPanel {
    type Event = ();
}

impl View for TerminalPanel {
    fn ui_name() -> &'static str {
        "TerminalPanel"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> gpui::AnyElement<Self> {
        ChildView::new(&self.pane, cx).into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if self.pane.read(cx).items_len() == 0 {
            if let Some(workspace) = self.workspace.upgrade(cx) {
                let working_directory_strategy = cx
                    .global::<Settings>()
                    .terminal_overrides
                    .working_directory
                    .clone()
                    .unwrap_or(WorkingDirectory::CurrentProjectDirectory);
                let working_directory = crate::get_working_directory(
                    workspace.read(cx),
                    cx,
                    working_directory_strategy,
                );
                let window_id = cx.window_id();
                if let Some(terminal) = self.project.update(cx, |project, cx| {
                    project
                        .create_terminal(working_directory, window_id, cx)
                        .log_err()
                }) {
                    workspace.update(cx, |workspace, cx| {
                        let terminal = Box::new(cx.add_view(|cx| {
                            TerminalView::new(terminal, workspace.database_id(), cx)
                        }));
                        Pane::add_item(workspace, &self.pane, terminal, true, true, None, cx);
                    });
                }
            }
        }
    }
}

impl Panel for TerminalPanel {}
