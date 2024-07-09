use debugger_panel::{DebugPanel, TogglePanel};
use gpui::{AppContext, Task, ViewContext};
use workspace::{StartDebugger, Workspace};

pub mod debugger_panel;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _action: &TogglePanel, cx| {
                    workspace.focus_panel::<DebugPanel>(cx);
                })
                .register_action(
                    |workspace: &mut Workspace,
                     action: &StartDebugger,
                     cx: &mut ViewContext<'_, Workspace>| {
                        tasks_ui::toggle_modal(workspace, cx, task::TaskType::Debug).detach();
                    },
                );
        },
    )
    .detach();
}
