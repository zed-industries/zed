use dap::debugger_settings::DebuggerSettings;
use debugger_panel::{DebugPanel, ToggleFocus};
use gpui::AppContext;
use settings::Settings;
use ui::ViewContext;
use workspace::{
    Continue, Pause, Restart, Start, StepInto, StepOut, StepOver, Stop, StopDebugAdapters,
    Workspace,
};

mod console;
pub mod debugger_panel;
mod debugger_panel_item;
mod module_list;
mod stack_frame_list;
mod variable_list;

pub fn init(cx: &mut AppContext) {
    DebuggerSettings::register(cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, cx| {
                    workspace.toggle_panel_focus::<DebugPanel>(cx);
                })
                .register_action(|workspace: &mut Workspace, _: &Start, cx| {
                    tasks_ui::toggle_modal(workspace, cx, task::TaskModal::DebugModal).detach();
                })
                .register_action(|workspace: &mut Workspace, _: &StopDebugAdapters, cx| {
                    workspace.project().update(cx, |project, cx| {
                        project.dap_store().update(cx, |store, cx| {
                            store.shutdown_clients(cx).detach();
                        })
                    })
                })
                .register_action(|workspace: &mut Workspace, _: &Stop, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    debug_panel.update(cx, |panel, cx| {
                        let Some(active_item) = panel.active_debug_panel_item(cx) else {
                            return;
                        };

                        active_item.update(cx, |item, cx| item.stop_thread(cx))
                    });
                })
                .register_action(|workspace: &mut Workspace, _: &Continue, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    debug_panel.update(cx, |panel, cx| {
                        let Some(active_item) = panel.active_debug_panel_item(cx) else {
                            return;
                        };

                        active_item.update(cx, |item, cx| item.continue_thread(cx))
                    });
                })
                .register_action(|workspace: &mut Workspace, _: &StepInto, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    debug_panel.update(cx, |panel, cx| {
                        let Some(active_item) = panel.active_debug_panel_item(cx) else {
                            return;
                        };

                        active_item.update(cx, |item, cx| item.step_in(cx))
                    });
                })
                .register_action(|workspace: &mut Workspace, _: &StepOut, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    debug_panel.update(cx, |panel, cx| {
                        let Some(active_item) = panel.active_debug_panel_item(cx) else {
                            return;
                        };

                        active_item.update(cx, |item, cx| item.step_out(cx))
                    });
                })
                .register_action(|workspace: &mut Workspace, _: &StepOver, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    debug_panel.update(cx, |panel, cx| {
                        let Some(active_item) = panel.active_debug_panel_item(cx) else {
                            return;
                        };

                        active_item.update(cx, |item, cx| item.step_over(cx))
                    });
                })
                .register_action(|workspace: &mut Workspace, _: &Restart, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    debug_panel.update(cx, |panel, cx| {
                        let Some(active_item) = panel.active_debug_panel_item(cx) else {
                            return;
                        };

                        active_item.update(cx, |item, cx| item.restart_client(cx))
                    });
                })
                .register_action(|workspace: &mut Workspace, _: &Pause, cx| {
                    let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

                    debug_panel.update(cx, |panel, cx| {
                        let Some(active_item) = panel.active_debug_panel_item(cx) else {
                            return;
                        };

                        active_item.update(cx, |item, cx| item.pause_thread(cx))
                    });
                });
        },
    )
    .detach();
}
