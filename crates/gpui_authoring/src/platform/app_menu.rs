use crate::{
    Action, App, KeyContext, Keymap, Keystroke, OsAction, Platform, PlatformMenu, PlatformMenuItem,
    PlatformOsMenu, SharedString, SystemMenuType,
};
use itertools::Itertools as _;
use std::sync::OnceLock;

/// A menu of the application, either a main menu or a submenu
pub struct Menu {
    /// The name of the menu
    pub name: SharedString,

    /// The items in the menu
    pub items: Vec<MenuItem>,

    /// Whether this menu is disabled
    pub disabled: bool,
}

impl Menu {
    /// Create a new Menu with the given name
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            items: vec![],
            disabled: false,
        }
    }

    /// Set items to be in this menu
    pub fn items(mut self, items: impl IntoIterator<Item = MenuItem>) -> Self {
        self.items = items.into_iter().collect();
        self
    }

    /// Set whether this menu is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Create an OwnedMenu from this Menu
    pub fn owned(self) -> OwnedMenu {
        OwnedMenu {
            name: self.name.to_string().into(),
            items: self.items.into_iter().map(|item| item.owned()).collect(),
            disabled: self.disabled,
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

        /// The action to perform when this menu item is selected
        action: Box<dyn Action>,

        /// The OS Action that corresponds to this action, if any
        /// See [`OsAction`] for more information
        os_action: Option<OsAction>,

        /// Whether this action is checked
        checked: bool,

        /// Whether this action is disabled
        disabled: bool,
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
            checked: false,
            disabled: false,
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
            checked: false,
            disabled: false,
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
                checked,
                disabled,
            } => OwnedMenuItem::Action {
                name: name.into(),
                action,
                os_action,
                checked,
                disabled,
            },
            MenuItem::SystemMenu(os_menu) => OwnedMenuItem::SystemMenu(os_menu.owned()),
        }
    }

    /// Set whether this menu item is checked
    ///
    /// Only for [`MenuItem::Action`], otherwise, will be ignored
    pub fn checked(mut self, checked: bool) -> Self {
        match &mut self {
            MenuItem::Action { checked: old, .. } => {
                *old = checked;
            }
            _ => {}
        }
        self
    }

    /// Returns whether this menu item is checked
    ///
    /// Only for [`MenuItem::Action`], otherwise, returns false
    #[inline]
    pub fn is_checked(&self) -> bool {
        match self {
            MenuItem::Action { checked, .. } => *checked,
            _ => false,
        }
    }

    /// Set whether this menu item is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        match &mut self {
            MenuItem::Action { disabled: old, .. } => {
                *old = disabled;
            }
            MenuItem::Submenu(submenu) => {
                submenu.disabled = disabled;
            }
            _ => {}
        }
        self
    }

    /// Returns whether this menu item is disabled
    ///
    /// Only for [`MenuItem::Action`] and [`MenuItem::Submenu`], otherwise, returns false
    #[inline]
    pub fn is_disabled(&self) -> bool {
        match self {
            MenuItem::Action { disabled, .. } => *disabled,
            MenuItem::Submenu(submenu) => submenu.disabled,
            _ => false,
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

    /// Whether this menu is disabled
    pub disabled: bool,
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

        /// The action to perform when this menu item is selected
        action: Box<dyn Action>,

        /// The OS Action that corresponds to this action, if any
        /// See [`OsAction`] for more information
        os_action: Option<OsAction>,

        /// Whether this action is checked
        checked: bool,

        /// Whether this action is disabled
        disabled: bool,
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
                checked,
                disabled,
            } => OwnedMenuItem::Action {
                name: name.clone(),
                action: action.boxed_clone(),
                os_action: *os_action,
                checked: *checked,
                disabled: *disabled,
            },
            OwnedMenuItem::SystemMenu(os_menu) => OwnedMenuItem::SystemMenu(os_menu.clone()),
        }
    }
}

pub(crate) fn init_app_menus(platform: &dyn Platform, cx: &App) {
    platform.on_will_open_app_menu(Box::new({
        let cx = cx.to_async();
        move || {
            if let Some(app) = cx.app.upgrade() {
                app.borrow_mut().update(|cx| cx.clear_pending_keystrokes());
            }
        }
    }));

    platform.on_validate_app_menu_command(Box::new({
        let cx = cx.to_async();
        move |command_id| {
            cx.app
                .upgrade()
                .map(|app| {
                    app.borrow_mut()
                        .update(|cx| cx.is_menu_command_available(command_id))
                })
                .unwrap_or(false)
        }
    }));

    platform.on_app_menu_action(Box::new({
        let cx = cx.to_async();
        move |command_id| {
            if let Some(app) = cx.app.upgrade() {
                app.borrow_mut()
                    .update(|cx| cx.dispatch_menu_command(command_id));
            }
        }
    }));
}

/// Resolves a menu tree into the platform vocabulary, recording each action in
/// `actions` such that its index is the item's [`MenuCommandId`].
pub(crate) fn resolve_menus(
    menus: &[OwnedMenu],
    keymap: &Keymap,
    actions: &mut Vec<Box<dyn Action>>,
) -> Vec<PlatformMenu> {
    menus
        .iter()
        .map(|menu| resolve_menu(menu, keymap, actions))
        .collect()
}

/// Resolves a dock menu into the platform vocabulary, appending its actions to
/// the same `actions` registry as [`resolve_menus`].
pub(crate) fn resolve_dock_menu(
    items: &[OwnedMenuItem],
    keymap: &Keymap,
    actions: &mut Vec<Box<dyn Action>>,
) -> Vec<PlatformMenuItem> {
    items
        .iter()
        .map(|item| resolve_item(item, keymap, actions))
        .collect()
}

fn resolve_menu(
    menu: &OwnedMenu,
    keymap: &Keymap,
    actions: &mut Vec<Box<dyn Action>>,
) -> PlatformMenu {
    PlatformMenu {
        name: menu.name.clone(),
        items: menu
            .items
            .iter()
            .map(|item| resolve_item(item, keymap, actions))
            .collect(),
        disabled: menu.disabled,
    }
}

fn resolve_item(
    item: &OwnedMenuItem,
    keymap: &Keymap,
    actions: &mut Vec<Box<dyn Action>>,
) -> PlatformMenuItem {
    match item {
        OwnedMenuItem::Separator => PlatformMenuItem::Separator,
        OwnedMenuItem::Submenu(menu) => {
            PlatformMenuItem::Submenu(resolve_menu(menu, keymap, actions))
        }
        OwnedMenuItem::SystemMenu(OwnedOsMenu { name, menu_type }) => {
            PlatformMenuItem::SystemMenu(PlatformOsMenu {
                name: name.clone(),
                menu_type: *menu_type,
            })
        }
        OwnedMenuItem::Action {
            name,
            action,
            os_action,
            checked,
            disabled,
        } => {
            let keystroke = resolve_keystroke(action.as_ref(), keymap);
            let command_id = actions.len();
            actions.push(action.boxed_clone());
            PlatformMenuItem::Action {
                name: name.clone(),
                command_id,
                keystroke,
                os_action: *os_action,
                checked: *checked,
                disabled: *disabled,
            }
        }
    }
}

/// Finds the accelerator to display for `action`, mirroring the precedence the
/// macOS backend used: prefer the earliest binding whose predicate holds in a
/// default Workspace/Pane/Editor context, and only show an accelerator when the
/// chosen binding is a single keystroke.
///
/// See the discussion on <https://github.com/zed-industries/zed/issues/23621>.
fn resolve_keystroke(action: &dyn Action, keymap: &Keymap) -> Option<Keystroke> {
    static DEFAULT_CONTEXT: OnceLock<Vec<KeyContext>> = OnceLock::new();

    let keystrokes = keymap
        .bindings_for_action(action)
        .find_or_first(|binding| {
            binding.predicate().is_none_or(|predicate| {
                predicate.eval(DEFAULT_CONTEXT.get_or_init(|| {
                    let mut workspace_context = KeyContext::new_with_defaults();
                    workspace_context.add("Workspace");
                    let mut pane_context = KeyContext::new_with_defaults();
                    pane_context.add("Pane");
                    let mut editor_context = KeyContext::new_with_defaults();
                    editor_context.add("Editor");

                    pane_context.extend(&editor_context);
                    workspace_context.extend(&pane_context);
                    vec![workspace_context]
                }))
            })
        })
        .map(|binding| binding.keystrokes());

    match keystrokes {
        Some(keystrokes) if keystrokes.len() == 1 => keystrokes
            .into_iter()
            .next()
            .map(|keystroke| keystroke.inner().clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::Menu;

    #[test]
    fn test_menu() {
        let menu = Menu::new("App")
            .items(vec![
                crate::MenuItem::action("Action 1", gpui::NoAction),
                crate::MenuItem::separator(),
            ])
            .disabled(true);

        assert_eq!(menu.name.as_ref(), "App");
        assert_eq!(menu.items.len(), 2);
        assert!(menu.disabled);
    }

    #[test]
    fn test_menu_item_builder() {
        use super::MenuItem;

        let item = MenuItem::action("Test Action", gpui::NoAction);
        assert_eq!(
            match &item {
                MenuItem::Action { name, .. } => name.as_ref(),
                _ => unreachable!(),
            },
            "Test Action"
        );
        assert!(matches!(
            item,
            MenuItem::Action {
                checked: false,
                disabled: false,
                ..
            }
        ));

        assert!(
            MenuItem::action("Test Action", gpui::NoAction)
                .checked(true)
                .is_checked()
        );
        assert!(
            MenuItem::action("Test Action", gpui::NoAction)
                .disabled(true)
                .is_disabled()
        );

        let submenu = MenuItem::submenu(super::Menu {
            name: "Submenu".into(),
            items: vec![],
            disabled: true,
        });
        assert_eq!(
            match &submenu {
                MenuItem::Submenu(menu) => menu.name.as_ref(),
                _ => unreachable!(),
            },
            "Submenu"
        );
        assert!(!submenu.is_checked());
        assert!(submenu.is_disabled());
    }
}
