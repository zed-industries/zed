use gpui::{App, WindowOptions, TitlebarOptions, WindowKind, point, px};
use theme::ActiveTheme;
use uuid::Uuid;

pub fn build_window_options(display_uuid: Option<Uuid>, cx: &mut App) -> WindowOptions {
    let display = display_uuid.and_then(|uuid| {
        cx.displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    });

    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some("Zed Lite".into()),
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        window_bounds: None,
        focus: false,
        show: false,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
        window_background: cx.theme().window_background_appearance(),
        window_min_size: Some(gpui::Size {
            width: px(640.0),
            height: px(480.0),
        }),
        ..Default::default()
    }
}