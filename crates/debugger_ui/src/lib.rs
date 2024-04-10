use debugger_panel::{DebugPanel, TogglePanel};
use gpui::{AppContext, ViewContext};
use serde::{Deserialize, Serialize};
use ui::Pixels;
use workspace::Workspace;

pub mod debugger_panel;

#[derive(Serialize, Deserialize)]
struct SerializedDebugPanel {
    width: Option<Pixels>,
}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _action: &TogglePanel, cx| {
                workspace.focus_panel::<DebugPanel>(cx);
            });
        },
    )
    .detach();
}
