use dap::debugger_settings::DebuggerSettings;
use debugger_panel::{DebugPanel, ToggleFocus};
use debugger_panel_item::DebugPanelItem;
use gpui::AppContext;
use settings::Settings;
use ui::ViewContext;
use workspace::{StartDebugger, Workspace};

pub mod debugger_panel;
mod debugger_panel_item;
mod variable_list;

pub fn init(cx: &mut AppContext) {
    DebuggerSettings::register(cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, cx| {
                    workspace.toggle_panel_focus::<DebugPanel>(cx);
                })
                .register_action(|workspace: &mut Workspace, _: &StartDebugger, cx| {
                    tasks_ui::toggle_modal(workspace, cx, task::TaskType::Debug).detach();
                })
                .register_action(DebugPanelItem::workspace_action_handler);
        },
    )
    .detach();
}
