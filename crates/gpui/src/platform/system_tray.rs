use crate::{AppContext, MouseButton, Point};

/// An icon displayed in a tray menu
pub enum TrayIcon<'a> {
    /// Name of the icon, this is platform dependent.
    Name(&'a str),
    /// Icon image
    Image {
        /// Width of the image
        width: u32,
        /// Height of the image
        height: u32,
        /// ARGB32 representation of the image
        bytes: Vec<u8>,
    },
}

/// Input type
pub enum TrayToggleType {
    /// Checkbox
    Checkbox(bool),
    /// Radio
    Radio(bool),
}

/// Item used to describe a System tray context menu.
pub enum TrayMenuItem<'a> {
    /// This item represents a line dividing submenus.
    /// Some desktop environments can display a label on top of the separator.
    Separator {
        /// Text displayed on top of the separator.
        label: Option<&'a str>,
    },
    /// This item represents a menu
    Submenu {
        /// ID of the menu item.
        id: &'a str,
        /// Text displayed in the menu.
        label: &'a str,
        /// Type of the input.
        toggle_type: Option<TrayToggleType>,
        /// Recursive menu item.
        children: Vec<TrayMenuItem<'a>>,
    },
}

impl<'a> TrayMenuItem<'a> {
    /// Create a separator with no label
    pub fn separator() -> Self {
        Self::Separator { label: None }
    }

    /// Create a separator with a label
    pub fn labeled_separator(label: &'a str) -> Self {
        Self::Separator { label: Some(label) }
    }

    /// Creates a menu item with a label and submenus.
    pub fn menu(id: &'a str, label: &'a str, children: Vec<TrayMenuItem<'a>>) -> Self {
        Self::Submenu {
            id,
            label,
            toggle_type: None,
            children,
        }
    }

    /// Creates a checkbox with a label.
    pub fn checkbox(id: &'a str, label: &'a str, checked: bool) -> Self {
        Self::Submenu {
            id,
            label,
            toggle_type: Some(TrayToggleType::Checkbox(checked)),
            children: Vec::default(),
        }
    }

    /// Creates a radio button with a label.
    pub fn radio(id: &'a str, label: &'a str, checked: bool) -> Self {
        Self::Submenu {
            id,
            label,
            toggle_type: Some(TrayToggleType::Radio(checked)),
            children: Vec::default(),
        }
    }
}

/// Events triggered by user action
pub enum TrayEvent {
    /// Represents a user click.
    TrayClick {
        /// A mouse button could be left, right or middle click.
        button: MouseButton,
        /// Position of the click.
        position: Point<i32>,
    },
    /// Scroll
    Scroll {
        /// Direction Scrolled.
        scroll_detal: Point<i32>,
    },
    /// Menu or submenu left click.
    MenuClick {
        /// ID of the clicked menu.
        id: String,
    },
}

/// A tray item.
pub struct TrayItem<'a> {
    pub(crate) icon: TrayIcon<'a>,
    pub(crate) title: String,
    pub(crate) tooltip: String,
    pub(crate) description: String,
    pub(crate) submenus: Vec<TrayMenuItem<'a>>,
    pub(crate) event: Option<Box<dyn FnMut(TrayEvent, &mut AppContext)>>,
}

impl<'a> TrayItem<'a> {
    /// Creates a new default tray item
    pub fn new() -> Self {
        Self {
            icon: TrayIcon::Name(""),
            title: String::default(),
            tooltip: String::default(),
            description: String::default(),
            submenus: Vec::default(),
            event: None,
        }
    }

    /// Sets the tray displayed icon.
    pub fn icon(mut self, icon: TrayIcon<'a>) -> Self {
        self.icon = icon;
        self
    }

    /// Sets the tray title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the text shown when hovered.
    pub fn tooltip(mut self, header: impl Into<String>) -> Self {
        self.tooltip = header.into();
        self
    }

    /// Sets a detailed text displayed together with a tooltip.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Adds a submenu to the tray item
    pub fn submenu(mut self, submenu: TrayMenuItem<'a>) -> Self {
        self.submenus.push(submenu);
        self
    }

    /// Sets a function to be called when an event happens with a tray item.
    pub fn on_event(mut self, event: impl FnMut(TrayEvent, &mut AppContext) + 'static) -> Self {
        self.event = Some(Box::new(event));
        self
    }
}
