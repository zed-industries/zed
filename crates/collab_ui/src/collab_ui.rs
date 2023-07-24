mod collab_titlebar_item;
mod contact_notification;
mod face_pile;
mod incoming_call_notification;
mod notifications;
mod project_shared_notification;
mod sharing_status_indicator;
pub mod panel;

use call::{ActiveCall, Room};
pub use collab_titlebar_item::CollabTitlebarItem;
use gpui::{actions, AppContext, Task};
use std::sync::Arc;
use util::ResultExt;
use workspace::AppState;

actions!(
    collab,
    [ToggleScreenSharing, ToggleMute, ToggleDeafen, LeaveCall]
);

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    vcs_menu::init(cx);
    collab_titlebar_item::init(cx);
    panel::init(app_state.client.clone(), cx);
    incoming_call_notification::init(&app_state, cx);
    project_shared_notification::init(&app_state, cx);
    sharing_status_indicator::init(cx);

    cx.add_global_action(toggle_screen_sharing);
    cx.add_global_action(toggle_mute);
    cx.add_global_action(toggle_deafen);
}

pub fn toggle_screen_sharing(_: &ToggleScreenSharing, cx: &mut AppContext) {
    let call = ActiveCall::global(cx).read(cx);
    if let Some(room) = call.room().cloned() {
        let client = call.client();
        let toggle_screen_sharing = room.update(cx, |room, cx| {
            if room.is_screen_sharing() {
                ActiveCall::report_call_event_for_room(
                    "disable screen share",
                    room.id(),
                    &client,
                    cx,
                );
                Task::ready(room.unshare_screen(cx))
            } else {
                ActiveCall::report_call_event_for_room(
                    "enable screen share",
                    room.id(),
                    &client,
                    cx,
                );
                room.share_screen(cx)
            }
        });
        toggle_screen_sharing.detach_and_log_err(cx);
    }
}

pub fn toggle_mute(_: &ToggleMute, cx: &mut AppContext) {
    let call = ActiveCall::global(cx).read(cx);
    if let Some(room) = call.room().cloned() {
        let client = call.client();
        room.update(cx, |room, cx| {
            if room.is_muted() {
                ActiveCall::report_call_event_for_room("enable microphone", room.id(), &client, cx);
            } else {
                ActiveCall::report_call_event_for_room(
                    "disable microphone",
                    room.id(),
                    &client,
                    cx,
                );
            }
            room.toggle_mute(cx)
        })
        .map(|task| task.detach_and_log_err(cx))
        .log_err();
    }
}

pub fn toggle_deafen(_: &ToggleDeafen, cx: &mut AppContext) {
    if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
        room.update(cx, Room::toggle_deafen)
            .map(|task| task.detach_and_log_err(cx))
            .log_err();
    }
}
