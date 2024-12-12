use gpui::{OwnedMenu, OwnedMenuItem, View};
use smallvec::SmallVec;
use ui::{prelude::*, ContextMenu, PopoverMenu, PopoverMenuHandle, Tooltip};

#[derive(Clone)]
struct MenuEntry {
    menu: OwnedMenu,
    handle: PopoverMenuHandle<ContextMenu>,
}

pub struct ApplicationMenu {
    entries: SmallVec<[MenuEntry; 8]>,
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
        }
    }

    fn build_menu_from_items(entry: MenuEntry, cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, cx| {
            let menu = menu.when_some(cx.focused(), |menu, focused| menu.context(focused));
            entry
                .menu
                .items
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
                // Skip if menu is already open to avoid focus issue
                if *hover_enter && !current_handle.is_deployed() {
                    all_handles.iter().for_each(|h| h.hide(cx));

                    // Defer to prevent focus race condition with the previously open menu
                    let handle = current_handle.clone();
                    cx.defer(move |w| handle.show(w));
                }
            })
    }

    pub fn is_any_deployed(&self) -> bool {
        self.entries.iter().any(|entry| entry.handle.is_deployed())
    }
}

impl Render for ApplicationMenu {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_any_deployed = self.is_any_deployed();
        div()
            .flex()
            .flex_row()
            .gap_x_1()
            .when(!is_any_deployed, |this| {
                this.child(self.render_application_menu(&self.entries[0]))
            })
            .when(is_any_deployed, |this| {
                this.children(
                    self.entries
                        .iter()
                        .map(|entry| self.render_standard_menu(entry)),
                )
            })
    }
}
