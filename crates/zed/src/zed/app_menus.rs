use collab_ui::collab_panel;
use gpui::{Menu, MenuItem, OsAction};
use terminal_view::terminal_panel;

pub fn app_menus() -> Vec<Menu> {
    use zed_actions::Quit;

    vec![
        Menu {
            name: "Zed".into(),
            items: vec![
                MenuItem::action("About Zed…", zed_actions::About),
                MenuItem::action("Check for Updates", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Settings".into(),
                    items: vec![
                        MenuItem::action("Open Settings", super::OpenSettings),
                        MenuItem::action("Open Key Bindings", zed_actions::OpenKeymap),
                        MenuItem::action("Open Default Settings", super::OpenDefaultSettings),
                        MenuItem::action(
                            "Open Default Key Bindings",
                            zed_actions::OpenDefaultKeymap,
                        ),
                        MenuItem::action("Open Project Settings", super::OpenProjectSettings),
                        MenuItem::action(
                            "Select Theme...",
                            zed_actions::theme_selector::Toggle::default(),
                        ),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Services".into(),
                    items: vec![],
                }),
                MenuItem::separator(),
                MenuItem::action("Extensions", zed_actions::Extensions::default()),
                MenuItem::action("Install CLI", install_cli::Install),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action("Hide Zed", super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("Hide Others", super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action("Show All", super::ShowAll),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("New", workspace::NewFile),
                MenuItem::action("New Window", workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action("Open File...", workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        "Open Folder..."
                    } else {
                        "Open…"
                    },
                    workspace::Open,
                ),
                MenuItem::action(
                    "Open Recent...",
                    zed_actions::OpenRecent {
                        create_new_window: true,
                    },
                ),
                MenuItem::action("Open Remote...", zed_actions::OpenRemote),
                MenuItem::separator(),
                MenuItem::action("Add Folder to Project…", workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action("Save", workspace::Save { save_intent: None }),
                MenuItem::action("Save As…", workspace::SaveAs),
                MenuItem::action("Save All", workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    "Close Editor",
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action("Close Window", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::os_action("Undo", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("Redo", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Cut", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("Copy", editor::actions::Copy, OsAction::Copy),
                MenuItem::action("Copy and Trim", editor::actions::CopyAndTrim),
                MenuItem::os_action("Paste", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("Find", search::buffer_search::Deploy::find()),
                MenuItem::action("Find In Project", workspace::DeploySearch::find()),
                MenuItem::separator(),
                MenuItem::action(
                    "Toggle Line Comment",
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: "Selection".into(),
            items: vec![
                MenuItem::os_action(
                    "Select All",
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action("Expand Selection", editor::actions::SelectLargerSyntaxNode),
                MenuItem::action("Shrink Selection", editor::actions::SelectSmallerSyntaxNode),
                MenuItem::separator(),
                MenuItem::action("Add Cursor Above", editor::actions::AddSelectionAbove),
                MenuItem::action("Add Cursor Below", editor::actions::AddSelectionBelow),
                MenuItem::action(
                    "Select Next Occurrence",
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("Move Line Up", editor::actions::MoveLineUp),
                MenuItem::action("Move Line Down", editor::actions::MoveLineDown),
                MenuItem::action("Duplicate Selection", editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: "View".into(),
            items: vec![
                MenuItem::action(
                    "Zoom In",
                    zed_actions::IncreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    "Zoom Out",
                    zed_actions::DecreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    "Reset Zoom",
                    zed_actions::ResetBufferFontSize { persist: true },
                ),
                MenuItem::separator(),
                MenuItem::action("Toggle Left Dock", workspace::ToggleLeftDock),
                MenuItem::action("Toggle Right Dock", workspace::ToggleRightDock),
                MenuItem::action("Toggle Bottom Dock", workspace::ToggleBottomDock),
                MenuItem::action("Close All Docks", workspace::CloseAllDocks),
                MenuItem::submenu(Menu {
                    name: "Editor Layout".into(),
                    items: vec![
                        MenuItem::action("Split Up", workspace::SplitUp),
                        MenuItem::action("Split Down", workspace::SplitDown),
                        MenuItem::action("Split Left", workspace::SplitLeft),
                        MenuItem::action("Split Right", workspace::SplitRight),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action("Project Panel", project_panel::ToggleFocus),
                MenuItem::action("Outline Panel", outline_panel::ToggleFocus),
                MenuItem::action("Collab Panel", collab_panel::ToggleFocus),
                MenuItem::action("Terminal Panel", terminal_panel::ToggleFocus),
                MenuItem::separator(),
                MenuItem::action("Diagnostics", diagnostics::Deploy),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Go".into(),
            items: vec![
                MenuItem::action("Back", workspace::GoBack),
                MenuItem::action("Forward", workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action("Command Palette...", zed_actions::command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action("Go to File...", workspace::ToggleFileFinder::default()),
                // MenuItem::action("Go to Symbol in Project", project_symbols::Toggle),
                MenuItem::action(
                    "Go to Symbol in Editor...",
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action("Go to Line/Column...", editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action("Go to Definition", editor::actions::GoToDefinition),
                MenuItem::action("Go to Declaration", editor::actions::GoToDeclaration),
                MenuItem::action("Go to Type Definition", editor::actions::GoToTypeDefinition),
                MenuItem::action("Find All References", editor::actions::FindAllReferences),
                MenuItem::separator(),
                MenuItem::action("Next Problem", editor::actions::GoToDiagnostic),
                MenuItem::action("Previous Problem", editor::actions::GoToPreviousDiagnostic),
            ],
        },
        Menu {
            name: "Window".into(),
            items: vec![
                MenuItem::action("Minimize", super::Minimize),
                MenuItem::action("Zoom", super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Help".into(),
            items: vec![
                MenuItem::action("View Telemetry", zed_actions::OpenTelemetryLog),
                MenuItem::action("View Dependency Licenses", zed_actions::OpenLicenses),
                MenuItem::action("Show Welcome", workspace::Welcome),
                MenuItem::action("Give Feedback...", zed_actions::feedback::GiveFeedback),
                MenuItem::separator(),
                MenuItem::action(
                    "Documentation",
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    "Zed Twitter",
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    "Join the Team",
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
