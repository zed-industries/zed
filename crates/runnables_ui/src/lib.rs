use gpui::{AppContext, ViewContext};
use modal::RunnablesModal;
pub use panel::RunnablesPanel;
use runnables_settings::RunnablesSettings;
use settings::Settings;
use workspace::Workspace;

mod modal;
mod panel;
mod runnables_settings;
mod status_bar_icon;

pub fn init(cx: &mut AppContext) {
    RunnablesSettings::register(cx);
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &panel::ToggleFocus, cx| {
                    workspace.toggle_panel_focus::<RunnablesPanel>(cx);
                })
                .register_action(|workspace, _: &modal::Spawn, cx| {
                    let inventory = workspace.project().read(cx).runnable_inventory().clone();
                    let workspace_handle = workspace.weak_handle();
                    workspace.toggle_modal(cx, |cx| {
                        RunnablesModal::new(inventory, workspace_handle, cx)
                    })
                });
        },
    )
    .detach();
}
