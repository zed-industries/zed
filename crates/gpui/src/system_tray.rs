use std::rc::Rc;

use crate::{App, Image, MenuItem, SharedString};

/// System tray icon.
#[derive(Clone)]
pub struct SystemTray {
    pub(crate) tooltip: Option<SharedString>,
    pub(crate) title: Option<SharedString>,
    pub(crate) icon: Option<Rc<Image>>,
    pub(crate) menu_builder: Option<Rc<dyn Fn(&mut App) -> Vec<MenuItem>>>,
    pub(crate) visible: bool,
}

impl SystemTray {
    /// Create a new tray icon with default properties.
    pub fn new() -> Self {
        Self {
            tooltip: None,
            title: None,
            icon: None,
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
    pub fn icon(mut self, icon: Image) -> Self {
        self.icon = Some(Rc::new(icon));
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

    /// Set the tooltip text.
    pub fn set_tooltip(&mut self, tooltip: impl Into<SharedString>) {
        self.tooltip = Some(tooltip.into());
    }

    /// Set the title text.
    pub fn set_title(&mut self, title: impl Into<SharedString>) {
        self.title = Some(title.into());
    }

    /// Set the icon image.
    pub fn set_icon(&mut self, icon: Image) {
        self.icon = Some(Rc::new(icon));
    }
}
