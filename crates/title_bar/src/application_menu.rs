use gpui::View;
use smallvec::SmallVec;
use ui::{prelude::*, ContextMenu, PopoverMenu, PopoverMenuHandle, Tooltip};

use install_cli;
use terminal_view::terminal_panel;
use workspace;

#[derive(Copy, Clone)]
enum MenuType {
    Zed,
    File,
    Edit,
    Selection,
    View,
    Go,
    Window,
    Help,
}

#[derive(Clone)]
struct MenuItem {
    menu_type: MenuType,
    handle: PopoverMenuHandle<ContextMenu>,
}

impl MenuItem {
    fn id(&self) -> &'static str {
        match self.menu_type {
            MenuType::Zed => "zed",
            MenuType::File => "file",
            MenuType::Edit => "edit",
            MenuType::Selection => "selection",
            MenuType::View => "view",
            MenuType::Go => "go",
            MenuType::Window => "window",
            MenuType::Help => "help",
        }
    }

    fn label(&self) -> &'static str {
        match self.menu_type {
            MenuType::Zed => "Zed",
            MenuType::File => "File",
            MenuType::Edit => "Edit",
            MenuType::Selection => "Selection",
            MenuType::View => "View",
            MenuType::Go => "Go",
            MenuType::Window => "Window",
            MenuType::Help => "Help",
        }
    }
}

pub struct ApplicationMenu {
    menu_items: SmallVec<[MenuItem; 8]>,
}

impl ApplicationMenu {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        let menu_types = [
            MenuType::Zed,
            MenuType::File,
            MenuType::Edit,
            MenuType::Selection,
            MenuType::View,
            MenuType::Go,
            MenuType::Window,
            MenuType::Help,
        ];

        let menu_items = menu_types
            .into_iter()
            .map(|menu_type| MenuItem {
                menu_type,
                handle: PopoverMenuHandle::default(),
            })
            .collect();

        Self { menu_items }
    }

    fn build_zed_menu(cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _cx| {
            menu.action("About Zed", Box::new(zed_actions::About))
                .action("Check for Updates", Box::new(auto_update::Check))
                .separator()
                .action("Open Settings", Box::new(zed_actions::OpenSettings))
                .action("Open Key Bindings", Box::new(zed_actions::OpenKeymap))
                // .action(
                //     "Open Default Settings",
                //     Box::new(super::OpenDefaultSettings),
                // )
                .action(
                    "Open Default Key Bindings",
                    Box::new(zed_actions::OpenDefaultKeymap),
                )
                // .action(
                //     "Open Project Settings",
                //     Box::new(super::OpenProjectSettings),
                // )
                .action(
                    "Select Theme...",
                    Box::new(zed_actions::theme_selector::Toggle::default()),
                )
                .separator()
                .action("Extensions", Box::new(zed_actions::Extensions))
                .action("Install CLI", Box::new(install_cli::Install))
                .separator()
                // .action("Hide Zed", Box::new(super::Hide))
                // .action("Hide Others", Box::new(super::HideOthers))
                // .action("Show All", Box::new(super::ShowAll))
                .separator()
                .action("Quit", Box::new(zed_actions::Quit))
        })
    }

    fn build_file_menu(cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _cx| {
            menu.action("New", Box::new(workspace::NewFile))
                .action("New Window", Box::new(workspace::NewWindow))
                .separator()
                .action("Open...", Box::new(workspace::Open))
                .action(
                    "Open Recent...",
                    Box::new(zed_actions::OpenRecent {
                        create_new_window: true,
                    }),
                )
                .separator()
                .action(
                    "Add Folder to Project...",
                    Box::new(workspace::AddFolderToProject),
                )
                .action("Save", Box::new(workspace::Save { save_intent: None }))
                .action("Save As...", Box::new(workspace::SaveAs))
                .action(
                    "Save All",
                    Box::new(workspace::SaveAll { save_intent: None }),
                )
                .separator()
                .action(
                    "Close Editor",
                    Box::new(workspace::CloseActiveItem { save_intent: None }),
                )
                .action("Close Window", Box::new(workspace::CloseWindow))
        })
    }

    fn build_edit_menu(cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _cx| {
            menu.action("Undo", Box::new(editor::actions::Undo))
                .action("Redo", Box::new(editor::actions::Redo))
                .separator()
                .action("Cut", Box::new(editor::actions::Cut))
                .action("Copy", Box::new(editor::actions::Copy))
                .action("Paste", Box::new(editor::actions::Paste))
                .separator()
                .action("Find", Box::new(search::buffer_search::Deploy::find()))
                .action("Find In Project", Box::new(workspace::DeploySearch::find()))
                .separator()
                .action(
                    "Toggle Line Comment",
                    Box::new(editor::actions::ToggleComments::default()),
                )
        })
    }

    fn build_selection_menu(cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _cx| {
            menu.action("Select All", Box::new(editor::actions::SelectAll))
                .action(
                    "Expand Selection",
                    Box::new(editor::actions::SelectLargerSyntaxNode),
                )
                .action(
                    "Shrink Selection",
                    Box::new(editor::actions::SelectSmallerSyntaxNode),
                )
                .separator()
                .action(
                    "Add Cursor Above",
                    Box::new(editor::actions::AddSelectionAbove),
                )
                .action(
                    "Add Cursor Below",
                    Box::new(editor::actions::AddSelectionBelow),
                )
                .action(
                    "Select Next Occurrence",
                    Box::new(editor::actions::SelectNext {
                        replace_newest: false,
                    }),
                )
        })
    }

    fn build_view_menu(cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _cx| {
            menu.action("Zoom In", Box::new(zed_actions::IncreaseBufferFontSize))
                .action("Zoom Out", Box::new(zed_actions::DecreaseBufferFontSize))
                .action("Reset Zoom", Box::new(zed_actions::ResetBufferFontSize))
                .separator()
                .action("Toggle Left Dock", Box::new(workspace::ToggleLeftDock))
                .action("Toggle Right Dock", Box::new(workspace::ToggleRightDock))
                .action("Toggle Bottom Dock", Box::new(workspace::ToggleBottomDock))
                .separator()
                .action("Project Panel", Box::new(project_panel::ToggleFocus))
                .action("Outline Panel", Box::new(outline_panel::ToggleFocus))
                // .action("Collab Panel", Box::new(collab_panel::ToggleFocus))
                .action("Terminal Panel", Box::new(terminal_panel::ToggleFocus))
        })
    }

    fn build_go_menu(cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _cx| {
            menu.action("Back", Box::new(workspace::GoBack))
                .action("Forward", Box::new(workspace::GoForward))
                .separator()
                .action(
                    "Command Palette...",
                    Box::new(zed_actions::command_palette::Toggle),
                )
                .separator()
                .action(
                    "Go to File...",
                    Box::new(workspace::ToggleFileFinder::default()),
                )
                .action(
                    "Go to Symbol in Editor...",
                    Box::new(editor::actions::ToggleOutline),
                )
                .action(
                    "Go to Line/Column...",
                    Box::new(editor::actions::ToggleGoToLine),
                )
        })
    }

    fn build_window_menu(cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _cx| {
            menu

            // .action("Minimize", Box::new(super::Minimize))
            //     .action("Zoom", Box::new(super::Zoom))
        })
    }

    fn build_help_menu(cx: &mut WindowContext<'_>) -> View<ContextMenu> {
        ContextMenu::build(cx, |menu, _cx| {
            menu.action("View Telemetry", Box::new(zed_actions::OpenTelemetryLog))
                .action(
                    "View Dependency Licenses",
                    Box::new(zed_actions::OpenLicenses),
                )
                .action("Show Welcome", Box::new(workspace::Welcome))
                .action(
                    "Give Feedback...",
                    Box::new(zed_actions::feedback::GiveFeedback),
                )
                .separator()
                .action(
                    "Documentation",
                    Box::new(super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    }),
                )
        })
    }

    fn render_application_menu(&self, item: &MenuItem) -> impl IntoElement {
        let item_handle = item.handle.clone();
        div().id(item.id()).occlude().child(
            PopoverMenu::new(SharedString::from(format!("menu-{}", item.id())))
                .menu(move |cx| Some(Self::build_zed_menu(cx)))
                .trigger(
                    IconButton::new("application-menu", ui::IconName::Menu)
                        .style(ButtonStyle::Subtle)
                        .icon_size(IconSize::Small)
                        .when(!item_handle.is_deployed(), |this| {
                            this.tooltip(|cx| Tooltip::text("Open Application Menu", cx))
                        }),
                )
                .with_handle(item_handle),
        )
    }

    fn render_standard_menu(&self, item: &MenuItem) -> impl IntoElement {
        let menu_type = item.menu_type;
        let item_handle = item.handle.clone();
        let other_handles: Vec<_> = self
            .menu_items
            .iter()
            .filter(|other| other.id() != item.id())
            .map(|other| other.handle.clone())
            .collect();

        div()
            .id(item.id())
            .occlude()
            .child(
                PopoverMenu::new(SharedString::from(format!("menu-{}", item.id())))
                    .menu(move |cx| {
                        Some(match menu_type {
                            MenuType::File => Self::build_file_menu(cx),
                            MenuType::Edit => Self::build_edit_menu(cx),
                            MenuType::Selection => Self::build_selection_menu(cx),
                            MenuType::View => Self::build_view_menu(cx),
                            MenuType::Go => Self::build_go_menu(cx),
                            MenuType::Window => Self::build_window_menu(cx),
                            MenuType::Help => Self::build_help_menu(cx),
                            MenuType::Zed => Self::build_zed_menu(cx),
                        })
                    })
                    .trigger(
                        Button::new(
                            SharedString::from(format!("menu-trigger-{}", item.id())),
                            item.label(),
                        )
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small),
                    )
                    .with_handle(item_handle.clone()),
            )
            .on_hover(move |_, cx| {
                other_handles.iter().for_each(|handle| handle.hide(cx));
                item_handle.show(cx);
            })
    }
}

impl Render for ApplicationMenu {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let show_all_menu_items = self.menu_items.iter().any(|item| item.handle.is_deployed());
        div()
            .flex()
            .flex_row()
            .gap_x_1()
            .when(!show_all_menu_items, |this| {
                this.child(self.render_application_menu(&self.menu_items[0]))
            })
            .when(show_all_menu_items, |this| {
                this.child(self.render_standard_menu(&self.menu_items[0]))
                    .children(
                        self.menu_items
                            .iter()
                            .skip(1)
                            .map(|item| self.render_standard_menu(item)),
                    )
            })
    }
}
