use gpui::{impl_actions, OwnedMenu, OwnedMenuItem, View};
use schemars::JsonSchema;
use serde::Deserialize;
use smallvec::SmallVec;
use ui::{prelude::*, ContextMenu, PopoverMenu, PopoverMenuHandle, Tooltip};

impl_actions!(
    app_menu,
    [OpenApplicationMenu, NavigateApplicationMenuInDirection]
);

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Default)]
pub struct OpenApplicationMenu(String);

#[derive(Clone, Deserialize, JsonSchema, PartialEq, Default)]
pub struct NavigateApplicationMenuInDirection(String);

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
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
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

    fn build_menu_from_items(entry: MenuEntry, cx: &mut WindowContext) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, cx| {
            // Grab current focus handle so menu can shown items in context with the focused element
            let menu = menu.when_some(cx.focused(), |menu, focused| menu.context(focused));
            let sanitized_items = Self::sanitize_menu_items(entry.menu.items);

            sanitized_items
                .into_iter()
                .fold(menu, |menu, item| match item {
                    OwnedMenuItem::Separator => menu.separator(),
                    OwnedMenuItem::Action { name, action, .. } => menu.action(name, action),
                    OwnedMenuItem::Submenu(submenu) => {
                        submenu
                            .items
                            .into_iter()
                            .fold(menu, |menu, item| match item {
                                OwnedMenuItem::Separator => menu.separator(),
                                OwnedMenuItem::Action { name, action, .. } => {
                                    menu.action(name, action)
                                }
                                OwnedMenuItem::Submenu(_) => menu,
                            })
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
                    .menu(move |cx| Self::build_menu_from_items(entry.clone(), cx).into())
                    .trigger(
                        IconButton::new(
                            SharedString::from(format!("{}-menu-trigger", menu_name)),
                            ui::IconName::Menu,
                        )
                        .style(ButtonStyle::Subtle)
                        .icon_size(IconSize::Small)
                        .when(!handle.is_deployed(), |this| {
                            this.tooltip(|cx| Tooltip::text("Open Application Menu", cx))
                        }),
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
                    .menu(move |cx| Self::build_menu_from_items(entry.clone(), cx).into())
                    .trigger(
                        Button::new(
                            SharedString::from(format!("{}-menu-trigger", menu_name)),
                            menu_name.clone(),
                        )
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small),
                    )
                    .with_handle(current_handle.clone()),
            )
            .on_hover(move |hover_enter, cx| {
                if *hover_enter && !current_handle.is_deployed() {
                    all_handles.iter().for_each(|h| h.hide(cx));

                    // We need to defer this so that this menu handle can take focus from the previous menu
                    let handle = current_handle.clone();
                    cx.defer(move |cx| handle.show(cx));
                }
            })
    }

    #[cfg(not(target_os = "macos"))]
    pub fn open_menu(&mut self, action: &OpenApplicationMenu, _cx: &mut ViewContext<Self>) {
        self.pending_menu_open = Some(action.0.clone());
    }

    #[cfg(not(target_os = "macos"))]
    pub fn navigate_menus_in_direction(
        &mut self,
        action: &NavigateApplicationMenuInDirection,
        cx: &mut ViewContext<Self>,
    ) {
        let current_index = self
            .entries
            .iter()
            .position(|entry| entry.handle.is_deployed());
        let Some(current_index) = current_index else {
            return;
        };

        let next_index = match action.0.as_str() {
            "Left" => {
                if current_index == 0 {
                    self.entries.len() - 1
                } else {
                    current_index - 1
                }
            }
            "Right" => {
                if current_index == self.entries.len() - 1 {
                    0
                } else {
                    current_index + 1
                }
            }
            _ => return,
        };

        self.entries[current_index].handle.hide(cx);

        // We need to defer this so that this menu handle can take focus from the previous menu
        let next_handle = self.entries[next_index].handle.clone();
        cx.defer(move |_, cx| next_handle.show(cx));
    }

    pub fn all_menus_shown(&self) -> bool {
        self.entries.iter().any(|entry| entry.handle.is_deployed())
            || self.pending_menu_open.is_some()
    }
}

impl Render for ApplicationMenu {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let all_menus_shown = self.all_menus_shown();

        if let Some(pending_menu_open) = self.pending_menu_open.take() {
            if let Some(entry) = self
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
                    cx.window_context().on_next_frame(move |cx| {
                        handles_to_hide.iter().for_each(|handle| handle.hide(cx));
                        cx.defer(move |cx| handle_to_show.show(cx));
                    });
                } else {
                    // Since menus are already shown, we can directly handle show/hide operations
                    handles_to_hide.iter().for_each(|handle| handle.hide(cx));
                    cx.defer(move |_, cx| handle_to_show.show(cx));
                }
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
