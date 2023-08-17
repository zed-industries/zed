pub mod collab_panel;
mod collab_titlebar_item;
mod contact_notification;
mod face_pile;
mod incoming_call_notification;
mod notifications;
mod project_shared_notification;
mod sharing_status_indicator;

use call::{ActiveCall, Room};
pub use collab_titlebar_item::CollabTitlebarItem;
use gpui::{
    actions,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    platform::{Screen, WindowBounds, WindowKind, WindowOptions},
    AppContext, Task,
};
use std::{rc::Rc, sync::Arc};
use util::ResultExt;
use workspace::AppState;

actions!(
    collab,
    [ToggleScreenSharing, ToggleMute, ToggleDeafen, LeaveCall]
);

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    vcs_menu::init(cx);
    collab_titlebar_item::init(cx);
    collab_panel::init(app_state.client.clone(), cx);
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
            if room.is_muted(cx) {
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

fn notification_window_options(
    screen: Rc<dyn Screen>,
    window_size: Vector2F,
) -> WindowOptions<'static> {
    const NOTIFICATION_PADDING: f32 = 16.;

    let screen_bounds = screen.content_bounds();
    WindowOptions {
        bounds: WindowBounds::Fixed(RectF::new(
            screen_bounds.upper_right()
                + vec2f(
                    -NOTIFICATION_PADDING - window_size.x(),
                    NOTIFICATION_PADDING,
                ),
            window_size,
        )),
        titlebar: None,
        center: false,
        focus: false,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: false,
        screen: Some(screen),
    }
}
