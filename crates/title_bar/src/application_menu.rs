use gpui::{Entity, OwnedMenu, OwnedMenuItem};
use settings::Settings;

#[cfg(not(target_os = "macos"))]
use gpui::{Action, actions};

#[cfg(not(target_os = "macos"))]
use schemars::JsonSchema;
#[cfg(not(target_os = "macos"))]
use serde::Deserialize;

use smallvec::SmallVec;
use ui::{ContextMenu, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};

use crate::title_bar_settings::TitleBarSettings;

#[cfg(not(target_os = "macos"))]
actions!(
    app_menu,
    [
        /// Navigates to the menu item on the right.
        ActivateMenuRight,
        /// Navigates to the menu item on the left.
        ActivateMenuLeft
    ]
);

#[cfg(not(target_os = "macos"))]
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Default, Action)]
#[action(namespace = app_menu)]
pub struct OpenApplicationMenu(String);

#[cfg(not(target_os = "macos"))]
pub enum ActivateDirection {
    Left,
    Right,
}

#[derive(Clone)]
struct MenuEntry {
    menu: OwnedMenu,
    handle: PopoverMenuHandle<ContextMenu>,
}

pub struct ApplicationMenu {
    entries: SmallVec<[MenuEntry; 8]>,
    pending_menu_open: Option<String>,
}

impl ApplicationMenu {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let menus = cx.get_menus().unwrap_or_default();
        Self {
            entries: menus
                .into_iter()
                .map(|menu| MenuEntry {
                    menu,
                    handle: PopoverMenuHandle::default(),
                })
                .collect(),
            pending_menu_open: None,
        }
    }

    fn sanitize_menu_items(items: Vec<OwnedMenuItem>) -> Vec<OwnedMenuItem> {
        let mut cleaned = Vec::new();
        let mut last_was_separator = false;

        for item in items {
            match item {
                OwnedMenuItem::Separator => {
                    if !last_was_separator {
                        cleaned.push(item);
                        last_was_separator = true;
                    }
                }
                OwnedMenuItem::Submenu(submenu) => {
                    // Skip empty submenus
                    if !submenu.items.is_empty() {
                        cleaned.push(OwnedMenuItem::Submenu(submenu));
                        last_was_separator = false;
                    }
                }
                item => {
                    cleaned.push(item);
                    last_was_separator = false;
                }
            }
        }

        // Remove trailing separator
        if let Some(OwnedMenuItem::Separator) = cleaned.last() {
            cleaned.pop();
        }

        cleaned
    }

    fn build_menu_from_items(
        entry: MenuEntry,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |menu, window, cx| {
            // Grab current focus handle so menu can shown items in context with the focused element
            let menu = menu.when_some(window.focused(cx), |menu, focused| menu.context(focused));
            let sanitized_items = Self::sanitize_menu_items(entry.menu.items);

            sanitized_items
                .into_iter()
                .fold(menu, |menu, item| match item {
                    OwnedMenuItem::Separator => menu.separator(),
                    OwnedMenuItem::Action {
                        name,
                        action,
                        checked,
                        ..
                    } => menu.action_checked(name, action, checked),
                    OwnedMenuItem::Submenu(submenu) => {
                        submenu
                            .items
                            .into_iter()
                            .fold(menu, |menu, item| match item {
                                OwnedMenuItem::Separator => menu.separator(),
                                OwnedMenuItem::Action {
                                    name,
                                    action,
                                    checked,
                                    ..
                                } => menu.action_checked(name, action, checked),
                                OwnedMenuItem::Submenu(_) => menu,
                                OwnedMenuItem::SystemMenu(_) => {
                                    // A system menu doesn't make sense in this context, so ignore it
                                    menu
                                }
                            })
                    }
                    OwnedMenuItem::SystemMenu(_) => {
                        // A system menu doesn't make sense in this context, so ignore it
                        menu
                    }
                })
        })
    }

    fn render_application_menu(&self, entry: &MenuEntry) -> impl IntoElement {
        let handle = entry.handle.clone();

        let menu_name = entry.menu.name.clone();
        let entry = entry.clone();

        // Application menu must have same ids as first menu item in standard menu
        div()
            .id(SharedString::from(format!("{}-menu-item", menu_name)))
            .occlude()
            .child(
                PopoverMenu::new(SharedString::from(format!("{}-menu-popover", menu_name)))
                    .menu(move |window, cx| {
                        Self::build_menu_from_items(entry.clone(), window, cx).into()
                    })
                    .trigger_with_tooltip(
                        IconButton::new(
                            SharedString::from(format!("{}-menu-trigger", menu_name)),
                            ui::IconName::Menu,
                        )
                        .style(ButtonStyle::Subtle)
                        .icon_size(IconSize::Small),
                        Tooltip::text("Open Application Menu"),
                    )
                    .with_handle(handle),
            )
    }

    fn render_standard_menu(&self, entry: &MenuEntry) -> impl IntoElement {
        let current_handle = entry.handle.clone();

        let menu_name = entry.menu.name.clone();
        let entry = entry.clone();

        let all_handles: Vec<_> = self
            .entries
            .iter()
            .map(|entry| entry.handle.clone())
            .collect();

        div()
            .id(SharedString::from(format!("{}-menu-item", menu_name)))
            .occlude()
            .child(
                PopoverMenu::new(SharedString::from(format!("{}-menu-popover", menu_name)))
                    .menu(move |window, cx| {
                        Self::build_menu_from_items(entry.clone(), window, cx).into()
                    })
                    .trigger(
                        Button::new(
                            SharedString::from(format!("{}-menu-trigger", menu_name)),
                            menu_name,
                        )
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small),
                    )
                    .with_handle(current_handle.clone()),
            )
            .on_hover(move |hover_enter, window, cx| {
                if *hover_enter && !current_handle.is_deployed() {
                    all_handles.iter().for_each(|h| h.hide(cx));

                    // We need to defer this so that this menu handle can take focus from the previous menu
                    let handle = current_handle.clone();
                    window.defer(cx, move |window, cx| handle.show(window, cx));
                }
            })
    }

    #[cfg(not(target_os = "macos"))]
    pub fn open_menu(
        &mut self,
        action: &OpenApplicationMenu,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.pending_menu_open = Some(action.0.clone());
    }

    #[cfg(not(target_os = "macos"))]
    pub fn navigate_menus_in_direction(
        &mut self,
        direction: ActivateDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let current_index = self
            .entries
            .iter()
            .position(|entry| entry.handle.is_deployed());
        let Some(current_index) = current_index else {
            return;
        };

        let next_index = match direction {
            ActivateDirection::Left => {
                if current_index == 0 {
                    self.entries.len() - 1
                } else {
                    current_index - 1
                }
            }
            ActivateDirection::Right => {
                if current_index == self.entries.len() - 1 {
                    0
                } else {
                    current_index + 1
                }
            }
        };

        self.entries[current_index].handle.hide(cx);

        // We need to defer this so that this menu handle can take focus from the previous menu
        let next_handle = self.entries[next_index].handle.clone();
        cx.defer_in(window, move |_, window, cx| next_handle.show(window, cx));
    }

    pub fn all_menus_shown(&self, cx: &mut Context<Self>) -> bool {
        show_menus(cx)
            || self.entries.iter().any(|entry| entry.handle.is_deployed())
            || self.pending_menu_open.is_some()
    }
}

pub(crate) fn show_menus(cx: &mut App) -> bool {
    TitleBarSettings::get_global(cx).show_menus
        && (cfg!(not(target_os = "macos")) || option_env!("ZED_USE_CROSS_PLATFORM_MENU").is_some())
}

impl Render for ApplicationMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_menus_shown = self.all_menus_shown(cx);

        if let Some(pending_menu_open) = self.pending_menu_open.take()
            && let Some(entry) = self
                .entries
                .iter()
                .find(|entry| entry.menu.name == pending_menu_open && !entry.handle.is_deployed())
        {
            let handle_to_show = entry.handle.clone();
            let handles_to_hide: Vec<_> = self
                .entries
                .iter()
                .filter(|e| e.menu.name != pending_menu_open && e.handle.is_deployed())
                .map(|e| e.handle.clone())
                .collect();

            if handles_to_hide.is_empty() {
                // We need to wait for the next frame to show all menus first,
                // before we can handle show/hide operations
                window.on_next_frame(move |window, cx| {
                    handles_to_hide.iter().for_each(|handle| handle.hide(cx));
                    window.defer(cx, move |window, cx| handle_to_show.show(window, cx));
                });
            } else {
                // Since menus are already shown, we can directly handle show/hide operations
                handles_to_hide.iter().for_each(|handle| handle.hide(cx));
                cx.defer_in(window, move |_, window, cx| handle_to_show.show(window, cx));
            }
        }

        div()
            .key_context("ApplicationMenu")
            .flex()
            .flex_row()
            .gap_x_1()
            .when(!all_menus_shown && !self.entries.is_empty(), |this| {
                this.child(self.render_application_menu(&self.entries[0]))
            })
            .when(all_menus_shown, |this| {
                this.children(
                    self.entries
                        .iter()
                        .map(|entry| self.render_standard_menu(entry)),
                )
            })
    }
}
