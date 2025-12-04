use std::rc::Rc;

use crate::{App, MenuItem, SharedString};

/// System tray icon.
#[derive(Clone)]
pub struct Tray {
    /// Tooltip text.
    pub tooltip: Option<SharedString>,
    /// Tray title after the Icon.
    pub title: Option<SharedString>,
    /// Tray icon image.
    pub icon: Option<Rc<gpui::Image>>,
    /// Function to build the context menu.
    pub menu_builder: Option<Rc<dyn Fn(&mut App) -> Vec<MenuItem>>>,
    /// Visibility of the tray icon.
    pub visible: bool,

    pub(crate) rendered_icon: Option<TrayIconData>,
}

#[derive(Clone)]
pub(crate) struct TrayIconData {
    pub(crate) data: Rc<Vec<u8>>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl TrayIconData {
    pub(crate) fn from_render_image(image: &gpui::RenderImage) -> Self {
        let bytes = image.as_bytes(0).unwrap_or_default();
        let size = image.size(0);
        Self {
            data: Rc::new(bytes.to_vec()),
            width: size.width.0 as u32,
            height: size.height.0 as u32,
        }
    }
}

impl Tray {
    /// Create a new tray icon with default properties.
    pub fn new() -> Self {
        Self {
            tooltip: None,
            title: None,
            icon: None,
            menu_builder: None,
            visible: true,

            rendered_icon: None,
        }
    }

    /// Set the tooltip text, defaults to None.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set the title text, defaults to None.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the icon image, defaults to None.
    pub fn icon(mut self, icon: impl Into<gpui::Image>) -> Self {
        self.icon = Some(Rc::new(icon.into()));
        self
    }

    /// Set the context menu.
    pub fn menu<F>(mut self, builder: F) -> Self
    where
        F: Fn(&mut App) -> Vec<MenuItem> + 'static,
    {
        self.menu_builder = Some(Rc::new(builder));
        self
    }

    /// Set visibility of the tray icon, default is true.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}
