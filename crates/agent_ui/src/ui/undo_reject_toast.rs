use action_log::ActionLog;
use gpui::{App, Entity};
use notifications::status_toast::{StatusToast, ToastIcon};
use ui::prelude::*;
use workspace::Workspace;

pub fn show_undo_reject_toast(
    workspace: &mut Workspace,
    action_log: Entity<ActionLog>,
    cx: &mut App,
) {
    let action_log_weak = action_log.downgrade();
    let status_toast = StatusToast::new("Agent Changes Rejected", cx, move |this, _cx| {
        this.icon(ToastIcon::new(IconName::Undo).color(Color::Muted))
            .action("Undo", move |_window, cx| {
                if let Some(action_log) = action_log_weak.upgrade() {
                    action_log
                        .update(cx, |action_log, cx| action_log.undo_last_reject(cx))
                        .detach();
                }
            })
            .dismiss_button(true)
    });
    workspace.toggle_status_toast(status_toast, cx);
}
