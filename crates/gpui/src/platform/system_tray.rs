use crate::{AppContext, Point};

///
pub enum TrayIcon<'a> {
    ///
    Name(&'a str),
}

impl<'a> Default for TrayIcon<'a> {
    fn default() -> Self {
        Self::Name("")
    }
}

///
pub enum TrayToggleType {
    ///
    Checkbox(bool),
    ///
    Radio(bool),
}

///
pub enum TrayMenuItem<'a> {
    /// This item usually represents a line dividing submenus.
    /// Some desktop environments can display a label on top of the separator.
    Separator {
        /// Text displayed on top of the separator.
        label: Option<&'a str>,
    },
    ///
    Submenu {
        ///
        id: &'a str,
        ///
        label: &'a str,
        ///
        icon: Option<TrayIcon<'a>>,
        ///
        toggle_type: Option<TrayToggleType>,
        ///
        children: Vec<TrayMenuItem<'a>>,
    },
}

///
pub enum TrayEvent {
    ///
    LeftClick {
        ///
        position: Point<i32>,
    },
    ///
    RightClick {
        ///
        position: Point<i32>,
    },
    ///
    MiddleClick {
        ///
        position: Point<i32>,
    },
    ///
    Scroll,
    ///
    MenuClick {
        ///
        id: String,
    },
}

///
#[derive(Default)]
pub struct TrayItem<'a> {
    /// Icon displayed
    pub icon: TrayIcon<'a>,
    /// Smaller icon displayed on top of the main icon.
    /// Some desktops environments support this feature.
    #[cfg(target_os = "linux")]
    pub overlay: Option<TrayIcon<'a>>,
    /// Title of this item.
    pub title: String,
    /// Text displayed when a mouse is hovered.
    pub tooltip: String,
    /// Detailed text displayed with a tooltip.
    pub description: String,
    ///
    pub submenus: Vec<TrayMenuItem<'a>>,
    ///
    pub event: Option<Box<dyn FnMut(TrayEvent, &mut AppContext)>>,
}

impl<'a> TrayItem<'a> {
    ///
    pub fn new() -> Self {
        TrayItem::default()
    }

    ///
    pub fn icon(mut self, icon: TrayIcon<'a>) -> Self {
        self.icon = icon;
        self
    }

    ///
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    ///
    pub fn tooltip(mut self, header: impl Into<String>) -> Self {
        self.tooltip = header.into();
        self
    }

    ///
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    ///
    pub fn on_event(mut self, event: impl FnMut(TrayEvent, &mut AppContext) + 'static) -> Self {
        self.event = Some(Box::new(event));
        self
    }
}
