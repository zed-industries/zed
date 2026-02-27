use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};

#[derive(Debug)]
pub struct WebDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    browser_window: web_sys::Window,
}

// Safety: WASM is single-threaded — there is no concurrent access to `web_sys::Window`.
unsafe impl Send for WebDisplay {}
unsafe impl Sync for WebDisplay {}

impl WebDisplay {
    pub fn new(browser_window: web_sys::Window) -> Self {
        WebDisplay {
            id: DisplayId::new(1),
            uuid: uuid::Uuid::new_v4(),
            browser_window,
        }
    }

    fn screen_size(&self) -> Size<Pixels> {
        let Some(screen) = self.browser_window.screen().ok() else {
            return Size {
                width: px(1920.),
                height: px(1080.),
            };
        };

        let width = screen.width().unwrap_or(1920) as f32;
        let height = screen.height().unwrap_or(1080) as f32;

        Size {
            width: px(width),
            height: px(height),
        }
    }

    fn viewport_size(&self) -> Size<Pixels> {
        let width = self
            .browser_window
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1920.0) as f32;
        let height = self
            .browser_window
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1080.0) as f32;

        Size {
            width: px(width),
            height: px(height),
        }
    }
}

impl PlatformDisplay for WebDisplay {
    fn id(&self) -> DisplayId {
        self.id
    }

    fn uuid(&self) -> Result<uuid::Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        let size = self.screen_size();
        Bounds {
            origin: Point::default(),
            size,
        }
    }

    fn visible_bounds(&self) -> Bounds<Pixels> {
        let size = self.viewport_size();
        Bounds {
            origin: Point::default(),
            size,
        }
    }

    fn default_bounds(&self) -> Bounds<Pixels> {
        let visible = self.visible_bounds();
        let width = visible.size.width * 0.75;
        let height = visible.size.height * 0.75;
        let origin_x = (visible.size.width - width) / 2.0;
        let origin_y = (visible.size.height - height) / 2.0;
        Bounds {
            origin: Point::new(origin_x, origin_y),
            size: Size { width, height },
        }
    }
}
