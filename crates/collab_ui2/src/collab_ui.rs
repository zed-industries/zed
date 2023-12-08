pub mod channel_view;
pub mod chat_panel;
pub mod collab_panel;
mod collab_titlebar_item;
mod face_pile;
pub mod notification_panel;
pub mod notifications;
mod panel_settings;

use std::{rc::Rc, sync::Arc};

use call::{report_call_event_for_room, ActiveCall, Room};
pub use collab_panel::CollabPanel;
pub use collab_titlebar_item::CollabTitlebarItem;
use feature_flags::{ChannelsAlpha, FeatureFlagAppExt};
use gpui::{
    actions, point, AppContext, GlobalPixels, Pixels, PlatformDisplay, Size, Task, WindowBounds,
    WindowKind, WindowOptions,
};
pub use panel_settings::{
    ChatPanelSettings, CollaborationPanelSettings, NotificationPanelSettings,
};
use settings::Settings;
use util::ResultExt;
use workspace::AppState;

actions!(ToggleScreenSharing, ToggleMute, ToggleDeafen, LeaveCall);

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    CollaborationPanelSettings::register(cx);
    ChatPanelSettings::register(cx);
    NotificationPanelSettings::register(cx);

    // vcs_menu::init(cx);
    collab_titlebar_item::init(cx);
    collab_panel::init(cx);
    channel_view::init(cx);
    // chat_panel::init(cx);
    notifications::init(&app_state, cx);

    // cx.add_global_action(toggle_screen_sharing);
    // cx.add_global_action(toggle_mute);
    // cx.add_global_action(toggle_deafen);
}

pub fn toggle_screen_sharing(_: &ToggleScreenSharing, cx: &mut AppContext) {
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
                    cx,
                );
                Task::ready(room.unshare_screen(cx))
            } else {
                report_call_event_for_room(
                    "enable screen share",
                    room.id(),
                    room.channel_id(),
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
            let operation = if room.is_muted(cx) {
                "enable microphone"
            } else {
                "disable microphone"
            };
            report_call_event_for_room(operation, room.id(), room.channel_id(), &client, cx);

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
    screen: Rc<dyn PlatformDisplay>,
    window_size: Size<Pixels>,
) -> WindowOptions {
    let notification_margin_width = GlobalPixels::from(16.);
    let notification_margin_height = GlobalPixels::from(-0.) - GlobalPixels::from(48.);

    let screen_bounds = screen.bounds();
    let size: Size<GlobalPixels> = window_size.into();

    // todo!() use content bounds instead of screen.bounds and get rid of magics in point's 2nd argument.
    let bounds = gpui::Bounds::<GlobalPixels> {
        origin: screen_bounds.upper_right()
            - point(
                size.width + notification_margin_width,
                notification_margin_height,
            ),
        size: window_size.into(),
    };
    WindowOptions {
        bounds: WindowBounds::Fixed(bounds),
        titlebar: None,
        center: false,
        focus: false,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: false,
        display_id: Some(screen.id()),
    }
}

// fn render_avatar<T: 'static>(
//     avatar: Option<Arc<ImageData>>,
//     avatar_style: &AvatarStyle,
//     container: ContainerStyle,
// ) -> AnyElement<T> {
//     avatar
//         .map(|avatar| {
//             Image::from_data(avatar)
//                 .with_style(avatar_style.image)
//                 .aligned()
//                 .contained()
//                 .with_corner_radius(avatar_style.outer_corner_radius)
//                 .constrained()
//                 .with_width(avatar_style.outer_width)
//                 .with_height(avatar_style.outer_width)
//                 .into_any()
//         })
//         .unwrap_or_else(|| {
//             Empty::new()
//                 .constrained()
//                 .with_width(avatar_style.outer_width)
//                 .into_any()
//         })
//         .contained()
//         .with_style(container)
//         .into_any()
// }

fn is_channels_feature_enabled(cx: &gpui::WindowContext<'_>) -> bool {
    cx.is_staff() || cx.has_flag::<ChannelsAlpha>()
}
