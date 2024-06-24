use debugger_panel::{DebugPanel, TogglePanel};
use gpui::{AppContext, Task, ViewContext};
use modal::DebuggerSelectModal;
use workspace::{StartDebugger, Workspace};

pub mod debugger_panel;
pub mod modal;

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
                        select_debugger(workspace, action, cx).detach();
                    },
                );
        },
    )
    .detach();
}

fn select_debugger(
    workspace: &mut Workspace,
    _: &StartDebugger,
    cx: &mut ViewContext<Workspace>,
) -> Task<()> {
    let project = workspace.project().clone();
    let workspace_handle = workspace.weak_handle();

    cx.spawn(|workspace, mut cx| async move {
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.toggle_modal(cx, |cx| {
                    DebuggerSelectModal::new(project, workspace_handle, cx)
                })
            })
            .ok();
    })
}
