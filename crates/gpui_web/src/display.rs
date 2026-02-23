use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};

#[derive(Debug)]
pub struct WebDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
}

impl WebDisplay {
    pub fn new() -> Self {
        WebDisplay {
            id: DisplayId::new(1),
            uuid: uuid::Uuid::new_v4(),
        }
    }

    fn web_window() -> Option<web_sys::Window> {
        web_sys::window()
    }

    fn screen_size() -> Size<Pixels> {
        let Some(window) = Self::web_window() else {
            return Size {
                width: px(1920.),
                height: px(1080.),
            };
        };

        let Some(screen) = window.screen().ok() else {
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

    fn viewport_size() -> Size<Pixels> {
        let Some(window) = Self::web_window() else {
            return Self::screen_size();
        };

        let width = window
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1920.0) as f32;
        let height = window
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
        let size = Self::screen_size();
        Bounds {
            origin: Point::default(),
            size,
        }
    }

    fn visible_bounds(&self) -> Bounds<Pixels> {
        let size = Self::viewport_size();
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
