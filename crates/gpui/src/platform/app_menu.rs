use crate::{Action, AppContext, Platform};
use util::ResultExt;

/// A menu of the application, either a main menu or a submenu
pub struct Menu<'a> {
    /// The name of the menu
    pub name: &'a str,

    /// The items in the menu
    pub items: Vec<MenuItem<'a>>,
}

/// The different kinds of items that can be in a menu
pub enum MenuItem<'a> {
    /// A separator between items
    Separator,

    /// A submenu
    Submenu(Menu<'a>),

    /// An action that can be performed
    Action {
        /// The name of this menu item
        name: &'a str,

        /// the action to perform when this menu item is selected
        action: Box<dyn Action>,

        /// The OS Action that corresponds to this action, if any
        /// See [`OsAction`] for more information
        os_action: Option<OsAction>,
    },
}

impl<'a> MenuItem<'a> {
    /// Creates a new menu item that is a separator
    pub fn separator() -> Self {
        Self::Separator
    }

    /// Creates a new menu item that is a submenu
    pub fn submenu(menu: Menu<'a>) -> Self {
        Self::Submenu(menu)
    }

    /// Creates a new menu item that invokes an action
    pub fn action(name: &'a str, action: impl Action) -> Self {
        Self::Action {
            name,
            action: Box::new(action),
            os_action: None,
        }
    }

    /// Creates a new menu item that invokes an action and has an OS action
    pub fn os_action(name: &'a str, action: impl Action, os_action: OsAction) -> Self {
        Self::Action {
            name,
            action: Box::new(action),
            os_action: Some(os_action),
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

pub(crate) fn init_app_menus(platform: &dyn Platform, cx: &mut AppContext) {
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
