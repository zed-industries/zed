//! Pre-resolved application menu vocabulary shared by `gpui` and its platform backends.
//!
//! `gpui` resolves its higher-level menu tree (which carries typed actions and a
//! keymap) into these plain-data structures before handing them to a platform.
//! Actionable items carry an opaque [`MenuCommandId`] that the platform reports
//! back through its menu callbacks.

use gpui_shared_string::SharedString;
use gpui_types::Keystroke;

/// Identifies a menu command whose action `gpui` will dispatch.
pub type MenuCommandId = usize;

/// The type of a system-managed menu.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SystemMenuType {
    /// The 'Services' menu in the Application menu on macOS.
    Services,
}

/// An action that the operating system recognizes and can give specialized
/// behavior to.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OsAction {
    /// The 'cut' action.
    Cut,
    /// The 'copy' action.
    Copy,
    /// The 'paste' action.
    Paste,
    /// The 'select all' action.
    SelectAll,
    /// The 'undo' action.
    Undo,
    /// The 'redo' action.
    Redo,
}

/// A menu managed by the operating system.
#[derive(Clone, Debug)]
pub struct PlatformOsMenu {
    /// The name of the menu.
    pub name: SharedString,
    /// The type of menu.
    pub menu_type: SystemMenuType,
}

/// A menu of the application, either a main menu or a submenu.
#[derive(Clone, Debug)]
pub struct PlatformMenu {
    /// The name of the menu.
    pub name: SharedString,
    /// The items in the menu.
    pub items: Vec<PlatformMenuItem>,
    /// Whether this menu is disabled.
    pub disabled: bool,
}

/// The different kinds of items that can be in a menu.
#[derive(Clone, Debug)]
pub enum PlatformMenuItem {
    /// A separator between items.
    Separator,

    /// A submenu.
    Submenu(PlatformMenu),

    /// A menu managed by the system (for example, the Services menu on macOS).
    SystemMenu(PlatformOsMenu),

    /// An action that can be performed.
    Action {
        /// The name of this menu item.
        name: String,
        /// The opaque command id that identifies the action to dispatch.
        command_id: MenuCommandId,
        /// The resolved keyboard accelerator to display, if the action has
        /// exactly one applicable keystroke.
        keystroke: Option<Keystroke>,
        /// The OS action that corresponds to this action, if any.
        os_action: Option<OsAction>,
        /// Whether this action is checked.
        checked: bool,
        /// Whether this action is disabled.
        disabled: bool,
    },
}
