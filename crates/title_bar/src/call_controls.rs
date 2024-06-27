use call::{report_call_event_for_room, ActiveCall};
use gpui::{actions, AppContext, Task, WindowContext};
use workspace::notifications::DetachAndPromptErr;

actions!(
    collab,
    [ToggleScreenSharing, ToggleMute, ToggleDeafen, LeaveCall]
);

pub fn toggle_screen_sharing(_: &ToggleScreenSharing, cx: &mut WindowContext) {
    let call = ActiveCall::global(cx).read(cx);
    if let Some(room) = call.room().cloned() {
        let client = call.client();
        let toggle_screen_sharing = room.update(cx, |room, cx| {
            if room.is_screen_sharing() {
                report_call_event_for_room(
                    "disable screen share",
                    room.id(),
                    room.channel_id(),
                    &client,
                );
                Task::ready(room.unshare_screen(cx))
            } else {
                report_call_event_for_room(
                    "enable screen share",
                    room.id(),
                    room.channel_id(),
                    &client,
                );
                room.share_screen(cx)
            }
        });
        toggle_screen_sharing.detach_and_prompt_err("Sharing Screen Failed", cx, |e, _| Some(format!("{:?}\n\nPlease check that you have given Zed permissions to record your screen in Settings.", e)));
    }
}

pub fn toggle_mute(_: &ToggleMute, cx: &mut AppContext) {
    let call = ActiveCall::global(cx).read(cx);
    if let Some(room) = call.room().cloned() {
        let client = call.client();
        room.update(cx, |room, cx| {
            let operation = if room.is_muted() {
                "enable microphone"
            } else {
                "disable microphone"
            };
            report_call_event_for_room(operation, room.id(), room.channel_id(), &client);

            room.toggle_mute(cx)
        });
    }
}

pub fn toggle_deafen(_: &ToggleDeafen, cx: &mut AppContext) {
    if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
        room.update(cx, |room, cx| room.toggle_deafen(cx));
    }
}
