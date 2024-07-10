use crate::AppContext;

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
        ///
        id: &'a str,
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
    LeftClick,
    ///
    RightClick,
    ///
    MiddleClick,
    ///
    Scroll,
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
    pub title: &'a str,
    /// Detailed text.
    pub description: &'a str,
    ///
    pub submenus: Vec<TrayMenuItem<'a>>,
    ///
    pub event: Option<Box<dyn FnMut(TrayEvent, &mut AppContext)>>,
}
