use dap::debugger_settings::DebuggerSettings;
use debugger_panel::{DebugPanel, ToggleFocus};
use feature_flags::{Debugger, FeatureFlagViewExt};
use gpui::App;
use session::DebugSession;
use settings::Settings;
use workspace::{
    Pause, Restart, ShutdownDebugAdapters, StepBack, StepInto, StepOver, Stop,
    ToggleIgnoreBreakpoints, Workspace,
};

pub mod attach_modal;
pub mod debugger_panel;
pub mod session;

#[cfg(test)]
mod tests;

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
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                        panel
                            .active_session(cx)
                            .and_then(|session| session.read(cx).mode().as_running().cloned())
                    }) {
                        active_item.update(cx, |item, cx| item.pause_thread(cx))
                    }
                })
                .register_action(|workspace, _: &Restart, _, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                        panel
                            .active_session(cx)
                            .and_then(|session| session.read(cx).mode().as_running().cloned())
                    }) {
                        active_item.update(cx, |item, cx| item.restart_session(cx))
                    }
                })
                .register_action(|workspace, _: &StepInto, _, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                        panel
                            .active_session(cx)
                            .and_then(|session| session.read(cx).mode().as_running().cloned())
                    }) {
                        active_item.update(cx, |item, cx| item.step_in(cx))
                    }
                })
                .register_action(|workspace, _: &StepOver, _, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                        panel
                            .active_session(cx)
                            .and_then(|session| session.read(cx).mode().as_running().cloned())
                    }) {
                        active_item.update(cx, |item, cx| item.step_over(cx))
                    }
                })
                .register_action(|workspace, _: &StepBack, _, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                        panel
                            .active_session(cx)
                            .and_then(|session| session.read(cx).mode().as_running().cloned())
                    }) {
                        active_item.update(cx, |item, cx| item.step_back(cx))
                    }
                })
                .register_action(|workspace, _: &Stop, _, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                        panel
                            .active_session(cx)
                            .and_then(|session| session.read(cx).mode().as_running().cloned())
                    }) {
                        active_item.update(cx, |item, cx| item.stop_thread(cx))
                    }
                })
                .register_action(|workspace, _: &ToggleIgnoreBreakpoints, _, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    if let Some(active_item) = debug_panel.read_with(cx, |panel, cx| {
                        panel
                            .active_session(cx)
                            .and_then(|session| session.read(cx).mode().as_running().cloned())
                    }) {
                        active_item.update(cx, |item, cx| item.toggle_ignore_breakpoints(cx))
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
                );
        })
    })
    .detach();
}
