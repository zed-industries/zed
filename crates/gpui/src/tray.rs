use crate::{App, MenuItem, SharedString};
use anyhow::Result;
use std::rc::Rc;

/// System tray icon.
#[derive(Clone)]
pub struct Tray {
    /// Tooltip text.
    pub tooltip: Option<SharedString>,
    /// Tray title after the Icon.
    pub title: Option<SharedString>,
    /// Tray icon image.
    pub icon: Option<Rc<gpui::Image>>,
    pub(crate) icon_data: Option<TrayIconData>,

    /// Function to build the context menu.
    pub menu_builder: Option<Rc<dyn Fn(&mut App) -> Vec<MenuItem>>>,
    /// Visibility of the tray icon.
    pub visible: bool,
}

impl Tray {
    pub(crate) fn render_icon(&mut self, cx: &App) -> Result<()> {
        if let Some(icon) = &self.icon {
            let image = icon.to_image_data(cx.svg_renderer())?;
            let bytes = image.as_bytes(0).unwrap_or_default();
            let size = image.size(0);

            self.icon_data = Some(TrayIconData {
                data: Rc::new(bytes.to_vec()),
                width: size.width.0 as u32,
                height: size.height.0 as u32,
            })
        }
        Ok(())
    }
}

#[derive(Clone)]
#[allow(unused)]
pub(crate) struct TrayIconData {
    pub(crate) data: Rc<Vec<u8>>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Tray {
    /// Create a new tray icon with default properties.
    pub fn new() -> Self {
        Self {
            tooltip: None,
            title: None,
            icon: None,
            icon_data: None,
            menu_builder: None,
            visible: true,
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
