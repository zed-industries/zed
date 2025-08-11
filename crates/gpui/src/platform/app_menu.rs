use crate::{Action, App, Platform, SharedString};
use util::ResultExt;

/// A menu of the application, either a main menu or a submenu
pub struct Menu {
    /// The name of the menu
    pub name: SharedString,

    /// The items in the menu
    pub items: Vec<MenuItem>,
}

impl Menu {
    /// Create an OwnedMenu from this Menu
    pub fn owned(self) -> OwnedMenu {
        OwnedMenu {
            name: self.name.to_string().into(),
            items: self.items.into_iter().map(|item| item.owned()).collect(),
        }
    }
}

/// OS menus are menus that are recognized by the operating system
/// This allows the operating system to provide specialized items for
/// these menus
pub struct OsMenu {
    /// The name of the menu
    pub name: SharedString,

    /// The type of menu
    pub menu_type: SystemMenuType,
}

impl OsMenu {
    /// Create an OwnedOsMenu from this OsMenu
    pub fn owned(self) -> OwnedOsMenu {
        OwnedOsMenu {
            name: self.name.to_string().into(),
            menu_type: self.menu_type,
        }
    }
}

/// The type of system menu
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SystemMenuType {
    /// The 'Services' menu in the Application menu on macOS
    Services,
}

/// The different kinds of items that can be in a menu
pub enum MenuItem {
    /// A separator between items
    Separator,

    /// A submenu
    Submenu(Menu),

    /// A menu, managed by the system (for example, the Services menu on macOS)
    SystemMenu(OsMenu),

    /// An action that can be performed
    Action {
        /// The name of this menu item
        name: SharedString,

        /// the action to perform when this menu item is selected
        action: Box<dyn Action>,

        /// The OS Action that corresponds to this action, if any
        /// See [`OsAction`] for more information
        os_action: Option<OsAction>,
    },
}

impl MenuItem {
    /// Creates a new menu item that is a separator
    pub fn separator() -> Self {
        Self::Separator
    }

    /// Creates a new menu item that is a submenu
    pub fn submenu(menu: Menu) -> Self {
        Self::Submenu(menu)
    }

    /// Creates a new submenu that is populated by the OS
    pub fn os_submenu(name: impl Into<SharedString>, menu_type: SystemMenuType) -> Self {
        Self::SystemMenu(OsMenu {
            name: name.into(),
            menu_type,
        })
    }

    /// Creates a new menu item that invokes an action
    pub fn action(name: impl Into<SharedString>, action: impl Action) -> Self {
        Self::Action {
            name: name.into(),
            action: Box::new(action),
            os_action: None,
        }
    }

    /// Creates a new menu item that invokes an action and has an OS action
    pub fn os_action(
        name: impl Into<SharedString>,
        action: impl Action,
        os_action: OsAction,
    ) -> Self {
        Self::Action {
            name: name.into(),
            action: Box::new(action),
            os_action: Some(os_action),
        }
    }

    /// Create an OwnedMenuItem from this MenuItem
    pub fn owned(self) -> OwnedMenuItem {
        match self {
            MenuItem::Separator => OwnedMenuItem::Separator,
            MenuItem::Submenu(submenu) => OwnedMenuItem::Submenu(submenu.owned()),
            MenuItem::Action {
                name,
                action,
                os_action,
            } => OwnedMenuItem::Action {
                name: name.into(),
                action,
                os_action,
            },
            MenuItem::SystemMenu(os_menu) => OwnedMenuItem::SystemMenu(os_menu.owned()),
        }
    }
}

/// OS menus are menus that are recognized by the operating system
/// This allows the operating system to provide specialized items for
/// these menus
#[derive(Clone)]
pub struct OwnedOsMenu {
    /// The name of the menu
    pub name: SharedString,

    /// The type of menu
    pub menu_type: SystemMenuType,
}

/// A menu of the application, either a main menu or a submenu
#[derive(Clone)]
pub struct OwnedMenu {
    /// The name of the menu
    pub name: SharedString,

    /// The items in the menu
    pub items: Vec<OwnedMenuItem>,
}

/// The different kinds of items that can be in a menu
pub enum OwnedMenuItem {
    /// A separator between items
    Separator,

    /// A submenu
    Submenu(OwnedMenu),

    /// A menu, managed by the system (for example, the Services menu on macOS)
    SystemMenu(OwnedOsMenu),

    /// An action that can be performed
    Action {
        /// The name of this menu item
        name: String,

        /// the action to perform when this menu item is selected
        action: Box<dyn Action>,

        /// The OS Action that corresponds to this action, if any
        /// See [`OsAction`] for more information
        os_action: Option<OsAction>,
    },
}

impl Clone for OwnedMenuItem {
    fn clone(&self) -> Self {
        match self {
            OwnedMenuItem::Separator => OwnedMenuItem::Separator,
            OwnedMenuItem::Submenu(submenu) => OwnedMenuItem::Submenu(submenu.clone()),
            OwnedMenuItem::Action {
                name,
                action,
                os_action,
            } => OwnedMenuItem::Action {
                name: name.clone(),
                action: action.boxed_clone(),
                os_action: *os_action,
            },
            OwnedMenuItem::SystemMenu(os_menu) => OwnedMenuItem::SystemMenu(os_menu.clone()),
        }
    }
}

// TODO: As part of the global selections refactor, these should
// be moved to GPUI-provided actions that make this association
// without leaking the platform details to GPUI users

/// OS actions are actions that are recognized by the operating system
/// This allows the operating system to provide specialized behavior for
/// these actions
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OsAction {
    /// The 'cut' action
    Cut,

    /// The 'copy' action
    Copy,

    /// The 'paste' action
    Paste,

    /// The 'select all' action
    SelectAll,

    /// The 'undo' action
    Undo,

    /// The 'redo' action
    Redo,
}

pub(crate) fn init_app_menus(platform: &dyn Platform, cx: &App) {
    platform.on_will_open_app_menu(Box::new({
        let cx = cx.to_async();
        move || {
            cx.update(|cx| cx.clear_pending_keystrokes()).ok();
        }
    }));

    platform.on_validate_app_menu_command(Box::new({
        let cx = cx.to_async();
        move |action| {
            cx.update(|cx| cx.is_action_available(action))
                .unwrap_or(false)
        }
    }));

    platform.on_app_menu_action(Box::new({
        let cx = cx.to_async();
        move |action| {
            cx.update(|cx| cx.dispatch_action(action)).log_err();
        }
    }));
}
