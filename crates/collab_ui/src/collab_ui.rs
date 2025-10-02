pub mod channel_view;
pub mod collab_panel;
pub mod notification_panel;
pub mod notifications;
mod panel_settings;

use std::{rc::Rc, sync::Arc};

pub use collab_panel::CollabPanel;
use gpui::{
    App, Pixels, PlatformDisplay, Size, WindowBackgroundAppearance, WindowBounds,
    WindowDecorations, WindowKind, WindowOptions, point,
};
pub use panel_settings::{CollaborationPanelSettings, NotificationPanelSettings};
use release_channel::ReleaseChannel;
use settings::Settings;
use ui::px;
use workspace::AppState;

pub fn init(app_state: &Arc<AppState>, cx: &mut App) {
    CollaborationPanelSettings::register(cx);
    NotificationPanelSettings::register(cx);

    channel_view::init(cx);
    collab_panel::init(cx);
    notification_panel::init(cx);
    notifications::init(app_state, cx);
    title_bar::init(cx);
}

fn notification_window_options(
    screen: Rc<dyn PlatformDisplay>,
    size: Size<Pixels>,
    cx: &App,
) -> WindowOptions {
    let notification_margin_width = px(16.);
    let notification_margin_height = px(-48.);

    let bounds = gpui::Bounds::<Pixels> {
        origin: screen.bounds().top_right()
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
        window_background: WindowBackgroundAppearance::Transparent,
        app_id: Some(app_id.to_owned()),
        window_min_size: None,
        window_decorations: Some(WindowDecorations::Client),
        tabbing_identifier: None,
        ..Default::default()
    }
}
