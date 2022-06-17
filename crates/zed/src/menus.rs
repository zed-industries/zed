use gpui::{Menu, MenuItem};

#[cfg(target_os = "macos")]
pub fn menus() -> Vec<Menu<'static>> {
    vec![
        Menu {
            name: "Zed",
            items: vec![
                MenuItem::Action {
                    name: "About Zed…",
                    action: Box::new(super::About),
                },
                MenuItem::Action {
                    name: "Check for Updates",
                    action: Box::new(auto_update::Check),
                },
                MenuItem::Separator,
                MenuItem::Submenu(Menu {
                    name: "Preferences",
                    items: vec![
                        MenuItem::Action {
                            name: "Open Settings",
                            action: Box::new(super::OpenSettings),
                        },
                        MenuItem::Action {
                            name: "Open Key Bindings",
                            action: Box::new(super::OpenKeymap),
                        },
                        MenuItem::Action {
                            name: "Select Theme",
                            action: Box::new(theme_selector::Toggle),
                        },
                    ],
                }),
                MenuItem::Action {
                    name: "Install CLI",
                    action: Box::new(super::InstallCommandLineInterface),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Quit",
                    action: Box::new(super::Quit),
                },
            ],
        },
        Menu {
            name: "File",
            items: vec![
                MenuItem::Action {
                    name: "New",
                    action: Box::new(workspace::NewFile),
                },
                MenuItem::Action {
                    name: "New Window",
                    action: Box::new(workspace::NewWindow),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Open…",
                    action: Box::new(workspace::Open),
                },
                MenuItem::Action {
                    name: "Add Folder to Project…",
                    action: Box::new(workspace::AddFolderToProject),
                },
                MenuItem::Action {
                    name: "Save",
                    action: Box::new(workspace::Save),
                },
                MenuItem::Action {
                    name: "Save As…",
                    action: Box::new(workspace::SaveAs),
                },
                MenuItem::Action {
                    name: "Save All",
                    action: Box::new(workspace::SaveAll),
                },
                MenuItem::Action {
                    name: "Close Editor",
                    action: Box::new(workspace::CloseActiveItem),
                },
                MenuItem::Action {
                    name: "Close Window",
                    action: Box::new(workspace::CloseWindow),
                },
            ],
        },
        Menu {
            name: "Edit",
            items: vec![
                MenuItem::Action {
                    name: "Undo",
                    action: Box::new(editor::Undo),
                },
                MenuItem::Action {
                    name: "Redo",
                    action: Box::new(editor::Redo),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Cut",
                    action: Box::new(editor::Cut),
                },
                MenuItem::Action {
                    name: "Copy",
                    action: Box::new(editor::Copy),
                },
                MenuItem::Action {
                    name: "Paste",
                    action: Box::new(editor::Paste),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Find",
                    action: Box::new(search::buffer_search::Deploy { focus: true }),
                },
                MenuItem::Action {
                    name: "Find In Project",
                    action: Box::new(search::project_search::Deploy),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Toggle Line Comment",
                    action: Box::new(editor::ToggleComments),
                },
            ],
        },
        Menu {
            name: "Selection",
            items: vec![
                MenuItem::Action {
                    name: "Select All",
                    action: Box::new(editor::SelectAll),
                },
                MenuItem::Action {
                    name: "Expand Selection",
                    action: Box::new(editor::SelectLargerSyntaxNode),
                },
                MenuItem::Action {
                    name: "Shrink Selection",
                    action: Box::new(editor::SelectSmallerSyntaxNode),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Add Cursor Above",
                    action: Box::new(editor::AddSelectionAbove),
                },
                MenuItem::Action {
                    name: "Add Cursor Below",
                    action: Box::new(editor::AddSelectionBelow),
                },
                MenuItem::Action {
                    name: "Select Next Occurrence",
                    action: Box::new(editor::SelectNext {
                        replace_newest: false,
                    }),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Move Line Up",
                    action: Box::new(editor::MoveLineUp),
                },
                MenuItem::Action {
                    name: "Move Line Down",
                    action: Box::new(editor::MoveLineDown),
                },
                MenuItem::Action {
                    name: "Duplicate Selection",
                    action: Box::new(editor::DuplicateLine),
                },
            ],
        },
        Menu {
            name: "View",
            items: vec![
                MenuItem::Action {
                    name: "Zoom In",
                    action: Box::new(super::IncreaseBufferFontSize),
                },
                MenuItem::Action {
                    name: "Zoom Out",
                    action: Box::new(super::DecreaseBufferFontSize),
                },
                MenuItem::Action {
                    name: "Reset Zoom",
                    action: Box::new(super::ResetBufferFontSize),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Project Browser",
                    action: Box::new(workspace::sidebar::ToggleSidebarItemFocus {
                        side: workspace::sidebar::Side::Left,
                        item_index: 0,
                    }),
                },
                MenuItem::Action {
                    name: "Command Palette",
                    action: Box::new(command_palette::Toggle),
                },
                MenuItem::Action {
                    name: "Diagnostics",
                    action: Box::new(diagnostics::Deploy),
                },
            ],
        },
        Menu {
            name: "Go",
            items: vec![
                MenuItem::Action {
                    name: "Back",
                    action: Box::new(workspace::GoBack { pane: None }),
                },
                MenuItem::Action {
                    name: "Forward",
                    action: Box::new(workspace::GoForward { pane: None }),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Go to File",
                    action: Box::new(file_finder::Toggle),
                },
                MenuItem::Action {
                    name: "Go to Symbol in Project",
                    action: Box::new(project_symbols::Toggle),
                },
                MenuItem::Action {
                    name: "Go to Symbol in Editor",
                    action: Box::new(outline::Toggle),
                },
                MenuItem::Action {
                    name: "Go to Definition",
                    action: Box::new(editor::GoToDefinition),
                },
                MenuItem::Action {
                    name: "Go to References",
                    action: Box::new(editor::FindAllReferences),
                },
                MenuItem::Action {
                    name: "Go to Line/Column",
                    action: Box::new(go_to_line::Toggle),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Next Problem",
                    action: Box::new(editor::GoToNextDiagnostic),
                },
                MenuItem::Action {
                    name: "Previous Problem",
                    action: Box::new(editor::GoToPrevDiagnostic),
                },
            ],
        },
        Menu {
            name: "Window",
            items: vec![MenuItem::Separator],
        },
        Menu {
            name: "Help",
            items: vec![
                MenuItem::Action {
                    name: "Command Palette",
                    action: Box::new(command_palette::Toggle),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Give Feedback",
                    action: Box::new(crate::OpenBrowser {
                        url: super::feedback::NEW_ISSUE_URL.into(),
                    }),
                },
                MenuItem::Action {
                    name: "Zed.dev",
                    action: Box::new(crate::OpenBrowser {
                        url: "https://zed.dev".into(),
                    }),
                },
                MenuItem::Action {
                    name: "Zed Twitter",
                    action: Box::new(crate::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    }),
                },
            ],
        },
    ]
}
