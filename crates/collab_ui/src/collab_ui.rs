mod collab_titlebar_item;
mod contact_finder;
mod contact_list;
mod contact_notification;
mod contacts_popover;
mod face_pile;
mod incoming_call_notification;
mod notifications;
mod project_shared_notification;
mod sharing_status_indicator;

use call::ActiveCall;
pub use collab_titlebar_item::{CollabTitlebarItem, ToggleContactsMenu};
use gpui::{actions, AppContext, Task};
use std::sync::Arc;
use workspace::AppState;

actions!(collab, [ToggleScreenSharing]);

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    collab_titlebar_item::init(cx);
    contact_list::init(cx);
    contact_finder::init(cx);
    contacts_popover::init(cx);
    incoming_call_notification::init(&app_state, cx);
    project_shared_notification::init(&app_state, cx);
    sharing_status_indicator::init(cx);

    cx.add_global_action(toggle_screen_sharing);
}

pub fn toggle_screen_sharing(_: &ToggleScreenSharing, cx: &mut AppContext) {
    if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
        let toggle_screen_sharing = room.update(cx, |room, cx| {
            if room.is_screen_sharing() {
                Task::ready(room.unshare_screen(cx))
            } else {
                room.share_screen(cx)
            }
        });
        toggle_screen_sharing.detach_and_log_err(cx);
    }
}
