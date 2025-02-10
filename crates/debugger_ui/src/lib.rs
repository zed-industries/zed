use dap::debugger_settings::DebuggerSettings;
use debugger_panel::{DebugPanel, ToggleFocus};
use debugger_panel_item::DebugPanelItem;
use gpui::App;
use settings::Settings;
use workspace::{
    Continue, Pause, Restart, ShutdownDebugAdapters, Start, StepBack, StepInto, StepOut, StepOver,
    Stop, ToggleIgnoreBreakpoints, Workspace,
};

pub mod attach_modal;
pub mod console;
pub mod debugger_panel;
pub mod debugger_panel_item;
pub mod loaded_source_list;
pub mod module_list;
pub mod stack_frame_list;
pub mod variable_list;

#[cfg(test)]
mod tests;

pub fn init(cx: &mut App) {
    DebuggerSettings::register(cx);
    workspace::FollowableViewRegistry::register::<DebugPanelItem>(cx);

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
                            store.shutdown_sessions(cx).detach();
                        })
                    })
                },
            )
            .register_action(|workspace: &mut Workspace, _: &Stop, _window, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                debug_panel.update(cx, |panel, cx| {
                    let Some(active_item) = panel.active_debug_panel_item(cx) else {
                        return;
                    };

                    active_item.update(cx, |item, cx| item.stop_thread(cx))
                });
            })
            .register_action(|workspace: &mut Workspace, _: &Continue, _window, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                debug_panel.update(cx, |panel, cx| {
                    let Some(active_item) = panel.active_debug_panel_item(cx) else {
                        return;
                    };

                    active_item.update(cx, |item, cx| item.continue_thread(cx))
                });
            })
            .register_action(|workspace: &mut Workspace, _: &StepInto, _window, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                debug_panel.update(cx, |panel, cx| {
                    let Some(active_item) = panel.active_debug_panel_item(cx) else {
                        return;
                    };

                    active_item.update(cx, |item, cx| item.step_in(cx))
                });
            })
            .register_action(|workspace: &mut Workspace, _: &StepBack, _window, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                debug_panel.update(cx, |panel, cx| {
                    let Some(active_item) = panel.active_debug_panel_item(cx) else {
                        return;
                    };

                    active_item.update(cx, |item, cx| item.step_back(cx))
                });
            })
            .register_action(|workspace: &mut Workspace, _: &StepOut, _window, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                debug_panel.update(cx, |panel, cx| {
                    let Some(active_item) = panel.active_debug_panel_item(cx) else {
                        return;
                    };

                    active_item.update(cx, |item, cx| item.step_out(cx))
                });
            })
            .register_action(|workspace: &mut Workspace, _: &StepOver, _window, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                debug_panel.update(cx, |panel, cx| {
                    let Some(active_item) = panel.active_debug_panel_item(cx) else {
                        return;
                    };

                    active_item.update(cx, |item, cx| item.step_over(cx))
                });
            })
            .register_action(|workspace: &mut Workspace, _: &Restart, _window, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                debug_panel.update(cx, |panel, cx| {
                    let Some(active_item) = panel.active_debug_panel_item(cx) else {
                        return;
                    };

                    active_item.update(cx, |item, cx| item.restart_client(cx))
                });
            })
            .register_action(
                |workspace: &mut Workspace, _: &ToggleIgnoreBreakpoints, _window, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    debug_panel.update(cx, |panel, cx| {
                        let Some(active_item) = panel.active_debug_panel_item(cx) else {
                            return;
                        };

                        active_item.update(cx, |item, cx| item.toggle_ignore_breakpoints(cx))
                    });
                },
            )
            .register_action(|workspace: &mut Workspace, _: &Pause, _window, cx| {
                let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                debug_panel.update(cx, |panel, cx| {
                    let Some(active_item) = panel.active_debug_panel_item(cx) else {
                        return;
                    };

                    active_item.update(cx, |item, cx| item.pause_thread(cx))
                });
            });
    })
    .detach();
}
