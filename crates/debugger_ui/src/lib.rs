use dap::debugger_settings::DebuggerSettings;
use debugger_panel::{DebugPanel, ToggleFocus};
use gpui::App;
use session::DebugSession;
use settings::Settings;
use workspace::{
    Continue, Pause, Restart, ShutdownDebugAdapters, Start, StepBack, StepInto, StepOut, StepOver,
    Stop, ToggleIgnoreBreakpoints, Workspace,
};

pub mod attach_modal;
pub mod debugger_panel;
pub mod session;

#[cfg(test)]
mod tests;

pub fn init(cx: &mut App) {
    DebuggerSettings::register(cx);
    workspace::FollowableViewRegistry::register::<DebugSession>(cx);

    cx.observe_new(|workspace: &mut Workspace, window, _cx| {
        let Some(_) = window else {
            return;
        };

        workspace
            .register_action(|workspace, _: &ToggleFocus, window, cx| {
                workspace.toggle_panel_focus::<DebugPanel>(window, cx);
            })
            .register_action(|workspace: &mut Workspace, _: &Start, window, cx| {
                tasks_ui::toggle_modal(workspace, None, task::TaskModal::DebugModal, window, cx)
                    .detach();
            })
            .register_action(
                |workspace: &mut Workspace, _: &ShutdownDebugAdapters, _window, cx| {
                    workspace.project().update(cx, |project, cx| {
                        project.dap_store().update(cx, |store, cx| {
                            store.shutdown_clients(cx).detach();
                        })
                    })
                },
            );
    })
    .detach();
}
