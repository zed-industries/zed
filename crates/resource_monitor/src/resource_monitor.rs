mod process_collector;
mod resource_monitor_view;

use gpui::{actions, App};
use workspace::{SplitDirection, Workspace};

pub use resource_monitor_view::ResourceMonitorView;

actions!(
    resource_monitor,
    [
        /// Opens the Resource Monitor, a Zed-aware view of all processes
        /// that Zed is running (language servers, terminals, main process)
        /// with live CPU, memory, and uptime stats.
        OpenResourceMonitor,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        workspace.register_action(|workspace, _: &OpenResourceMonitor, window, cx| {
            let project = workspace.project().clone();
            let existing = workspace
                .items_of_type::<ResourceMonitorView>(cx)
                .next();
            if let Some(existing) = existing {
                workspace.activate_item(&existing, true, true, window, cx);
            } else {
                let view = cx.new(|cx| ResourceMonitorView::new(project, window, cx));
                workspace.add_item_to_active_pane(
                    Box::new(view),
                    None,
                    true,
                    window,
                    cx,
                );
            }
        });
    })
    .detach();
}
