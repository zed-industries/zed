use gpui::{IntoElement, Render, View, WeakView};
use settings::Settings;
use ui::{
    div, ButtonCommon, Clickable, Color, FluentBuilder, IconButton, IconName, Tooltip,
    VisualContext, WindowContext,
};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

use crate::{modal::Spawn, settings::TaskSettings};

enum TaskStatus {
    Failed,
    Running,
    Succeeded,
}

/// A status bar icon that surfaces the status of running tasks.
/// It has a different color depending on the state of running tasks:
/// - red if any open task tab failed
/// - else, yellow if any open task tab is still running
/// - else, green if there tasks tabs open, and they have all succeeded
/// - else, no indicator if there are no open task tabs
pub struct TaskStatusIndicator {
    workspace: WeakView<Workspace>,
}

impl TaskStatusIndicator {
    pub fn new(workspace: WeakView<Workspace>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_| Self { workspace })
    }
    fn current_status(&self, cx: &mut WindowContext) -> Option<TaskStatus> {
        self.workspace
            .update(cx, |this, cx| {
                let mut status = None;
                let project = this.project().read(cx);

                for handle in project.local_terminal_handles() {
                    let Some(handle) = handle.upgrade() else {
                        continue;
                    };
                    let handle = handle.read(cx);
                    let task_state = handle.task();
                    if let Some(state) = task_state {
                        match state.status {
                            terminal::TaskStatus::Running => {
                                let _ = status.insert(TaskStatus::Running);
                            }
                            terminal::TaskStatus::Completed { success } => {
                                if !success {
                                    let _ = status.insert(TaskStatus::Failed);
                                    return status;
                                }
                                status.get_or_insert(TaskStatus::Succeeded);
                            }
                            _ => {}
                        };
                    }
                }
                status
            })
            .ok()
            .flatten()
    }
}

impl Render for TaskStatusIndicator {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        if !TaskSettings::get_global(cx).show_status_indicator {
            return div().into_any_element();
        }
        let current_status = self.current_status(cx);
        let color = current_status.map(|status| match status {
            TaskStatus::Failed => Color::Error,
            TaskStatus::Running => Color::Warning,
            TaskStatus::Succeeded => Color::Success,
        });
        IconButton::new("tasks-activity-indicator", IconName::Play)
            .when_some(color, |this, color| this.icon_color(color))
            .on_click(cx.listener(|this, _, cx| {
                this.workspace
                    .update(cx, |this, cx| {
                        crate::spawn_task_or_modal(this, &Spawn::modal(), cx)
                    })
                    .ok();
            }))
            .tooltip(|cx| Tooltip::for_action("Spawn tasks", &Spawn { task_name: None }, cx))
            .into_any_element()
    }
}

impl StatusItemView for TaskStatusIndicator {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _: &mut ui::prelude::ViewContext<Self>,
    ) {
    }
}
