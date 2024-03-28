pub mod channel_view;
pub mod chat_panel;
pub mod collab_panel;
mod collab_titlebar_item;
mod face_pile;
pub mod notification_panel;
pub mod notifications;
mod panel_settings;

use std::{rc::Rc, sync::Arc};

use call::{report_call_event_for_room, ActiveCall};
pub use collab_panel::CollabPanel;
pub use collab_titlebar_item::CollabTitlebarItem;
use gpui::{
    actions, point, AppContext, DevicePixels, Pixels, PlatformDisplay, Size, Task, WindowContext,
    WindowKind, WindowOptions,
};
use panel_settings::MessageEditorSettings;
pub use panel_settings::{
    ChatPanelSettings, CollaborationPanelSettings, NotificationPanelSettings,
};
use settings::Settings;
use workspace::{notifications::DetachAndPromptErr, AppState};

actions!(
    collab,
    [ToggleScreenSharing, ToggleMute, ToggleDeafen, LeaveCall]
);

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    CollaborationPanelSettings::register(cx);
    ChatPanelSettings::register(cx);
    NotificationPanelSettings::register(cx);
    MessageEditorSettings::register(cx);

    vcs_menu::init(cx);
    collab_titlebar_item::init(cx);
    collab_panel::init(cx);
    channel_view::init(cx);
    chat_panel::init(cx);
    notification_panel::init(cx);
    notifications::init(&app_state, cx);
}

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

fn notification_window_options(
    screen: Rc<dyn PlatformDisplay>,
    window_size: Size<Pixels>,
) -> WindowOptions {
    let notification_margin_width = DevicePixels::from(16);
    let notification_margin_height = DevicePixels::from(-0) - DevicePixels::from(48);

    let screen_bounds = screen.bounds();
    let size: Size<DevicePixels> = window_size.into();

    let bounds = gpui::Bounds::<DevicePixels> {
        origin: screen_bounds.upper_right()
            - point(
                size.width + notification_margin_width,
                notification_margin_height,
            ),
        size: window_size.into(),
    };

    WindowOptions {
        bounds: Some(bounds),
        titlebar: None,
        focus: false,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: false,
        display_id: Some(screen.id()),
        fullscreen: false,
    }
}
