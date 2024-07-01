pub mod channel_view;
pub mod chat_panel;
pub mod collab_panel;
pub mod notification_panel;
pub mod notifications;
mod panel_settings;

use std::{rc::Rc, sync::Arc};

pub use collab_panel::CollabPanel;
use gpui::{
    point, AppContext, Pixels, PlatformDisplay, Size, WindowBackgroundAppearance, WindowBounds,
    WindowKind, WindowOptions,
};
use panel_settings::MessageEditorSettings;
pub use panel_settings::{
    ChatPanelSettings, CollaborationPanelSettings, NotificationPanelSettings,
};
use release_channel::ReleaseChannel;
use settings::Settings;
use ui::px;
use workspace::AppState;

pub fn init(app_state: &Arc<AppState>, cx: &mut AppContext) {
    CollaborationPanelSettings::register(cx);
    ChatPanelSettings::register(cx);
    NotificationPanelSettings::register(cx);
    MessageEditorSettings::register(cx);

    channel_view::init(cx);
    chat_panel::init(cx);
    collab_panel::init(cx);
    notification_panel::init(cx);
    notifications::init(&app_state, cx);
    title_bar::init(cx);
    vcs_menu::init(cx);
}

fn notification_window_options(
    screen: Rc<dyn PlatformDisplay>,
    size: Size<Pixels>,
    cx: &AppContext,
) -> WindowOptions {
    let notification_margin_width = px(16.);
    let notification_margin_height = px(-48.);

    let bounds = gpui::Bounds::<Pixels> {
        origin: screen.bounds().upper_right()
            - point(
                size.width + notification_margin_width,
                notification_margin_height,
            ),
        size,
    };

    let app_id = ReleaseChannel::global(cx).app_id();

    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: None,
        focus: false,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: false,
        display_id: Some(screen.id()),
        window_background: WindowBackgroundAppearance::default(),
        app_id: Some(app_id.to_owned()),
        window_min_size: None,
    }
}
