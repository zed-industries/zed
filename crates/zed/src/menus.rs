use gpui::{Menu, MenuItem, OsAction};

#[cfg(target_os = "macos")]
pub fn menus() -> Vec<Menu<'static>> {
    vec![
        Menu {
            name: "Zed",
            items: vec![
                MenuItem::action("About Zed…", super::About),
                MenuItem::action("Check for Updates", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Preferences",
                    items: vec![
                        MenuItem::action("Open Settings", super::OpenSettings),
                        MenuItem::action("Open Key Bindings", super::OpenKeymap),
                        MenuItem::action("Open Default Settings", super::OpenDefaultSettings),
                        MenuItem::action("Open Default Key Bindings", super::OpenDefaultKeymap),
                        MenuItem::action("Select Theme", theme_selector::Toggle),
                    ],
                }),
                MenuItem::action("Install CLI", install_cli::Install),
                MenuItem::separator(),
                MenuItem::action("Hide Zed", super::Hide),
                MenuItem::action("Hide Others", super::HideOthers),
                MenuItem::action("Show All", super::ShowAll),
                MenuItem::action("Quit", super::Quit),
            ],
        },
        Menu {
            name: "File",
            items: vec![
                MenuItem::action("New", workspace::NewFile),
                MenuItem::action("New Window", workspace::NewWindow),
                MenuItem::separator(),
                MenuItem::action("Open…", workspace::Open),
                MenuItem::action("Open Recent...", recent_projects::OpenRecent),
                MenuItem::separator(),
                MenuItem::action("Add Folder to Project…", workspace::AddFolderToProject),
                MenuItem::action("Save", workspace::Save),
                MenuItem::action("Save As…", workspace::SaveAs),
                MenuItem::action("Save All", workspace::SaveAll),
                MenuItem::action("Close Editor", workspace::CloseActiveItem),
                MenuItem::action("Close Window", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "Edit",
            items: vec![
                MenuItem::os_action("Undo", editor::Undo, OsAction::Undo),
                MenuItem::os_action("Redo", editor::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Cut", editor::Cut, OsAction::Cut),
                MenuItem::os_action("Copy", editor::Copy, OsAction::Copy),
                MenuItem::os_action("Paste", editor::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("Find", search::buffer_search::Deploy { focus: true }),
                MenuItem::action("Find In Project", workspace::NewSearch),
                MenuItem::separator(),
                MenuItem::action("Toggle Line Comment", editor::ToggleComments::default()),
                MenuItem::action("Emoji & Symbols", editor::ShowCharacterPalette),
            ],
        },
        Menu {
            name: "Selection",
            items: vec![
                MenuItem::os_action("Select All", editor::SelectAll, OsAction::SelectAll),
                MenuItem::action("Expand Selection", editor::SelectLargerSyntaxNode),
                MenuItem::action("Shrink Selection", editor::SelectSmallerSyntaxNode),
                MenuItem::separator(),
                MenuItem::action("Add Cursor Above", editor::AddSelectionAbove),
                MenuItem::action("Add Cursor Below", editor::AddSelectionBelow),
                MenuItem::action(
                    "Select Next Occurrence",
                    editor::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("Move Line Up", editor::MoveLineUp),
                MenuItem::action("Move Line Down", editor::MoveLineDown),
                MenuItem::action("Duplicate Selection", editor::DuplicateLine),
            ],
        },
        Menu {
            name: "View",
            items: vec![
                MenuItem::action("Zoom In", super::IncreaseBufferFontSize),
                MenuItem::action("Zoom Out", super::DecreaseBufferFontSize),
                MenuItem::action("Reset Zoom", super::ResetBufferFontSize),
                MenuItem::separator(),
                MenuItem::action("Toggle Left Sidebar", workspace::ToggleLeftSidebar),
                MenuItem::submenu(Menu {
                    name: "Editor Layout",
                    items: vec![
                        MenuItem::action("Split Up", workspace::SplitUp),
                        MenuItem::action("Split Down", workspace::SplitDown),
                        MenuItem::action("Split Left", workspace::SplitLeft),
                        MenuItem::action("Split Right", workspace::SplitRight),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action("Project Panel", project_panel::ToggleFocus),
                MenuItem::action("Command Palette", command_palette::Toggle),
                MenuItem::action("Diagnostics", diagnostics::Deploy),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Go",
            items: vec![
                MenuItem::action("Back", workspace::GoBack { pane: None }),
                MenuItem::action("Forward", workspace::GoForward { pane: None }),
                MenuItem::separator(),
                MenuItem::action("Go to File", file_finder::Toggle),
                MenuItem::action("Go to Symbol in Project", project_symbols::Toggle),
                MenuItem::action("Go to Symbol in Editor", outline::Toggle),
                MenuItem::action("Go to Definition", editor::GoToDefinition),
                MenuItem::action("Go to Type Definition", editor::GoToTypeDefinition),
                MenuItem::action("Find All References", editor::FindAllReferences),
                MenuItem::action("Go to Line/Column", go_to_line::Toggle),
                MenuItem::separator(),
                MenuItem::action("Next Problem", editor::GoToDiagnostic),
                MenuItem::action("Previous Problem", editor::GoToPrevDiagnostic),
            ],
        },
        Menu {
            name: "Window",
            items: vec![
                MenuItem::action("Minimize", super::Minimize),
                MenuItem::action("Zoom", super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Help",
            items: vec![
                MenuItem::action("Command Palette", command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action("View Telemetry", crate::OpenTelemetryLog),
                MenuItem::action("View Dependency Licenses", crate::OpenLicenses),
                MenuItem::action("Show Welcome", workspace::Welcome),
                MenuItem::separator(),
                MenuItem::action("Give us feedback", feedback::feedback_editor::GiveFeedback),
                MenuItem::action(
                    "Copy System Specs Into Clipboard",
                    feedback::CopySystemSpecsIntoClipboard,
                ),
                MenuItem::action("File Bug Report", feedback::FileBugReport),
                MenuItem::action("Request Feature", feedback::RequestFeature),
                MenuItem::separator(),
                MenuItem::action(
                    "Documentation",
                    crate::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    "Zed Twitter",
                    crate::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
            ],
        },
    ]
}
