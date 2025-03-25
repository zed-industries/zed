use gpui::{
    point, App, AppContext, PlatformDisplay, Size, WindowBackgroundAppearance, WindowBounds,
    WindowDecorations, WindowHandle, WindowKind, WindowOptions,
};
use release_channel::ReleaseChannel;
use std::{marker::PhantomData, rc::Rc};
use ui::{px, Pixels, Render};
use util::ResultExt;

pub struct PopUp<V> {
    windows: Vec<WindowHandle<V>>,
    _phantom_data: PhantomData<V>,
}

impl<V> Default for PopUp<V> {
    fn default() -> Self {
        Self {
            windows: Vec::new(),
            _phantom_data: PhantomData,
        }
    }
}

impl<V: 'static + Render> PopUp<V> {
    pub fn open(&mut self, cx: &mut App, build_root_view: impl Fn(&mut App) -> V) {
        let window_size = Size {
            width: px(400.),
            height: px(72.),
        };

        for screen in cx.displays() {
            let options = window_options(screen, window_size, cx);

            if let Some(window) = cx
                .open_window(options, |_, cx| cx.new(|cx| (&build_root_view)(cx)))
                .log_err()
            {
                self.windows.push(window);
            }
        }
    }

    pub fn dismiss(self, cx: &mut App) {
        for window in self.windows {
            window
                .update(cx, |_, window, _| {
                    window.remove_window();
                })
                .ok();
        }
    }
}

fn window_options(screen: Rc<dyn PlatformDisplay>, size: Size<Pixels>, cx: &App) -> WindowOptions {
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
    }
}
