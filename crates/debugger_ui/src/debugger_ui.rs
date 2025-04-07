use dap::debugger_settings::DebuggerSettings;
use debugger_panel::{DebugPanel, ToggleFocus};
use feature_flags::{Debugger, FeatureFlagViewExt};
use gpui::{App, actions};
use new_session_modal::NewSessionModal;
use session::DebugSession;
use settings::Settings;
use workspace::{ShutdownDebugAdapters, Workspace};

pub mod attach_modal;
pub mod debugger_panel;
mod new_session_modal;
pub(crate) mod session;

#[cfg(test)]
pub mod tests;

actions!(
    debugger,
    [
        Start,
        Continue,
        Disconnect,
        Pause,
        Restart,
        StepInto,
        StepOver,
        StepOut,
        StepBack,
        Stop,
        ToggleIgnoreBreakpoints,
        ClearAllBreakpoints,
        CreateDebuggingSession,
    ]
);

pub fn init(cx: &mut App) {
    DebuggerSettings::register(cx);
    workspace::FollowableViewRegistry::register::<DebugSession>(cx);

    cx.observe_new(|_: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        cx.when_flag_enabled::<Debugger>(window, |workspace, _, _| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, window, cx| {
                    workspace.toggle_panel_focus::<DebugPanel>(window, cx);
                })
                .register_action(|workspace, _: &Pause, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session(cx)
                                .and_then(|session| session.read(cx).mode().as_running().cloned())
                        }) {
                            active_item.update(cx, |item, cx| item.pause_thread(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &Restart, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session(cx)
                                .and_then(|session| session.read(cx).mode().as_running().cloned())
                        }) {
                            active_item.update(cx, |item, cx| item.restart_session(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepInto, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session(cx)
                                .and_then(|session| session.read(cx).mode().as_running().cloned())
                        }) {
                            active_item.update(cx, |item, cx| item.step_in(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepOver, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session(cx)
                                .and_then(|session| session.read(cx).mode().as_running().cloned())
                        }) {
                            active_item.update(cx, |item, cx| item.step_over(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &StepBack, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session(cx)
                                .and_then(|session| session.read(cx).mode().as_running().cloned())
                        }) {
                            active_item.update(cx, |item, cx| item.step_back(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &Stop, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session(cx)
                                .and_then(|session| session.read(cx).mode().as_running().cloned())
                        }) {
                            active_item.update(cx, |item, cx| item.stop_thread(cx))
                        }
                    }
                })
                .register_action(|workspace, _: &ToggleIgnoreBreakpoints, _, cx| {
                    if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                        if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                            panel
                                .active_session(cx)
                                .and_then(|session| session.read(cx).mode().as_running().cloned())
                        }) {
                            active_item.update(cx, |item, cx| item.toggle_ignore_breakpoints(cx))
                        }
                    }
                })
                .register_action(
                    |workspace: &mut Workspace, _: &ShutdownDebugAdapters, _window, cx| {
                        workspace.project().update(cx, |project, cx| {
                            project.dap_store().update(cx, |store, cx| {
                                store.shutdown_sessions(cx).detach();
                            })
                        })
                    },
                )
                .register_action(
                    |workspace: &mut Workspace, _: &CreateDebuggingSession, window, cx| {
                        if let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) {
                            let weak_panel = debug_panel.downgrade();
                            let weak_workspace = cx.weak_entity();

                            workspace.toggle_modal(window, cx, |window, cx| {
                                NewSessionModal::new(
                                    debug_panel.read(cx).past_debug_definition.clone(),
                                    weak_panel,
                                    weak_workspace,
                                    window,
                                    cx,
                                )
                            });
                        }
                    },
                );
        })
    })
    .detach();
}
