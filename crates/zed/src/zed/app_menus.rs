use collab_ui::collab_panel;
use gpui::{App, Menu, MenuItem, OsAction};
use i18n::t;
use release_channel::ReleaseChannel;
use terminal_view::terminal_panel;
use zed_actions::{Quit, assistant, debug_panel, dev, git_panel, project_panel};

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    let mut view_items = vec![
        MenuItem::action(
            t!("Zoom In"),
            zed_actions::IncreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            t!("Zoom Out"),
            zed_actions::DecreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            t!("Reset Zoom"),
            zed_actions::ResetBufferFontSize { persist: false },
        ),
        MenuItem::action(
            t!("Reset All Zoom"),
            zed_actions::ResetAllZoom { persist: false },
        ),
        MenuItem::separator(),
        MenuItem::action(t!("Toggle Left Dock"), workspace::ToggleLeftDock),
        MenuItem::action(t!("Toggle Right Dock"), workspace::ToggleRightDock),
        MenuItem::action(t!("Toggle Bottom Dock"), workspace::ToggleBottomDock),
        MenuItem::action(t!("Toggle All Docks"), workspace::ToggleAllDocks),
        MenuItem::submenu(Menu {
            name: t!("Editor Layout").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t!("Split Up"), workspace::SplitUp::default()),
                MenuItem::action(t!("Split Down"), workspace::SplitDown::default()),
                MenuItem::action(t!("Split Left"), workspace::SplitLeft::default()),
                MenuItem::action(t!("Split Right"), workspace::SplitRight::default()),
            ],
        }),
        MenuItem::separator(),
        MenuItem::action(t!("Project Panel"), project_panel::ToggleFocus),
        MenuItem::action(t!("Outline Panel"), outline_panel::ToggleFocus),
        MenuItem::action(t!("Collab Panel"), collab_panel::ToggleFocus),
        MenuItem::action(t!("Terminal Panel"), terminal_panel::Toggle),
        MenuItem::action(t!("Debugger Panel"), debug_panel::ToggleFocus),
        MenuItem::action(t!("Agent Panel"), assistant::ToggleFocus),
        MenuItem::action(t!("Git Panel"), git_panel::ToggleFocus),
        MenuItem::separator(),
        MenuItem::action(t!("Diagnostics"), diagnostics::Deploy),
        MenuItem::separator(),
    ];

    if ReleaseChannel::try_global(cx) == Some(ReleaseChannel::Dev) {
        view_items.push(MenuItem::action(
            t!("Toggle GPUI Inspector"),
            dev::ToggleInspector,
        ));
        view_items.push(MenuItem::separator());
    }

    vec![
        Menu {
            // The application menu is titled after the product, which is not
            // translated.
            name: "Zed".into(),
            disabled: false,
            items: vec![
                MenuItem::action(t!("About Zed"), zed_actions::About),
                MenuItem::action(t!("Check for Updates"), auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu::new(t!("Settings")).items([
                    MenuItem::action(t!("Open Settings"), zed_actions::OpenSettings),
                    MenuItem::action(t!("Open Settings File"), super::OpenSettingsFile),
                    MenuItem::action(
                        t!("Open Project Settings"),
                        zed_actions::OpenProjectSettings,
                    ),
                    MenuItem::action(
                        t!("Open Project Settings File"),
                        super::OpenProjectSettingsFile,
                    ),
                    MenuItem::action(t!("Open Default Settings"), super::OpenDefaultSettings),
                    MenuItem::separator(),
                    MenuItem::action(t!("Open Keymap"), zed_actions::OpenKeymap),
                    MenuItem::action(t!("Open Keymap File"), zed_actions::OpenKeymapFile),
                    MenuItem::action(
                        t!("Open Default Key Bindings"),
                        zed_actions::OpenDefaultKeymap,
                    ),
                    MenuItem::separator(),
                    MenuItem::action(
                        t!("Select Theme..."),
                        zed_actions::theme_selector::Toggle::default(),
                    ),
                    MenuItem::action(
                        t!("Select Icon Theme..."),
                        zed_actions::icon_theme_selector::Toggle::default(),
                    ),
                    MenuItem::action(
                        t!("Select Interface Language…"),
                        zed_actions::locale_selector::Toggle,
                    ),
                ])),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu(t!("Services"), gpui::SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action(t!("Extensions"), zed_actions::Extensions::default()),
                #[cfg(not(target_os = "windows"))]
                MenuItem::action(t!("Install CLI"), install_cli::InstallCliBinary),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("Hide Zed"), super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("Hide Others"), super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("Show All"), super::ShowAll),
                MenuItem::separator(),
                MenuItem::action(t!("Quit Zed"), Quit),
            ],
        },
        Menu {
            name: t!("File").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t!("New"), workspace::NewFile),
                MenuItem::action(t!("New Window"), workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action(t!("Open File..."), workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        t!("Open Folder...")
                    } else {
                        t!("Open…")
                    },
                    workspace::Open::default(),
                ),
                MenuItem::action(t!("Open Recent…"), zed_actions::OpenRecent::default()),
                MenuItem::action(t!("Open Remote…"), zed_actions::OpenRemote::default()),
                MenuItem::separator(),
                MenuItem::action(t!("Add Folder to Project…"), workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action(t!("Save"), workspace::Save { save_intent: None }),
                MenuItem::action(t!("Save As…"), workspace::SaveAs),
                MenuItem::action(t!("Save All"), workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Close Editor"),
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action(t!("Close Project"), workspace::CloseProject),
                MenuItem::action(t!("Close Window"), workspace::CloseWindow),
            ],
        },
        Menu {
            name: t!("Edit").into(),
            disabled: false,
            items: vec![
                MenuItem::os_action(t!("Undo"), editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action(t!("Redo"), editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action(t!("Cut"), editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action(t!("Copy"), editor::actions::Copy, OsAction::Copy),
                MenuItem::action(t!("Copy and Trim"), editor::actions::CopyAndTrim),
                MenuItem::os_action(t!("Paste"), editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action(t!("Find"), search::buffer_search::Deploy::find()),
                MenuItem::action(t!("Find in Project"), workspace::DeploySearch::default()),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Toggle Line Comment"),
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: t!("Selection").into(),
            disabled: false,
            items: vec![
                MenuItem::os_action(
                    t!("Select All"),
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action(
                    t!("Expand Selection"),
                    editor::actions::SelectLargerSyntaxNode,
                ),
                MenuItem::action(
                    t!("Shrink Selection"),
                    editor::actions::SelectSmallerSyntaxNode,
                ),
                MenuItem::action(
                    t!("Select Next Sibling"),
                    editor::actions::SelectNextSyntaxNode,
                ),
                MenuItem::action(
                    t!("Select Previous Sibling"),
                    editor::actions::SelectPreviousSyntaxNode,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Add Cursor Above"),
                    editor::actions::AddSelectionAbove {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    t!("Add Cursor Below"),
                    editor::actions::AddSelectionBelow {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    t!("Select Next Occurrence"),
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    t!("Select Previous Occurrence"),
                    editor::actions::SelectPrevious {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    t!("Select All Occurrences"),
                    editor::actions::SelectAllMatches,
                ),
                MenuItem::separator(),
                MenuItem::action(t!("Move Line Up"), editor::actions::MoveLineUp),
                MenuItem::action(t!("Move Line Down"), editor::actions::MoveLineDown),
                MenuItem::action(
                    t!("Duplicate Selection"),
                    editor::actions::DuplicateLineDown,
                ),
            ],
        },
        Menu {
            name: t!("View").into(),
            disabled: false,
            items: view_items,
        },
        Menu {
            name: t!("Go").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t!("Back"), workspace::GoBack),
                MenuItem::action(t!("Forward"), workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Command Palette..."),
                    zed_actions::command_palette::Toggle,
                ),
                MenuItem::separator(),
                MenuItem::action(t!("Go to File..."), workspace::ToggleFileFinder::default()),
                // MenuItem::action("Go to Symbol in Project", project_symbols::Toggle),
                MenuItem::action(
                    t!("Go to Symbol in Editor..."),
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action(t!("Go to Line/Column..."), editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Go to Definition"),
                    editor::actions::GoToDefinition::default(),
                ),
                MenuItem::action(t!("Go to Declaration"), editor::actions::GoToDeclaration),
                MenuItem::action(
                    t!("Go to Type Definition"),
                    editor::actions::GoToTypeDefinition,
                ),
                MenuItem::action(
                    t!("Find All References"),
                    editor::actions::FindAllReferences::default(),
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Next Problem"),
                    editor::actions::GoToDiagnostic::default(),
                ),
                MenuItem::action(
                    t!("Previous Problem"),
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: t!("Run").into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    t!("Spawn Task"),
                    zed_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::action(t!("Start Debugger"), debugger_ui::Start),
                MenuItem::separator(),
                MenuItem::action(t!("Edit tasks.json…"), zed_actions::OpenProjectTasks),
                MenuItem::action(t!("Edit debug.json…"), zed_actions::OpenProjectDebugTasks),
                MenuItem::separator(),
                MenuItem::action(t!("Continue"), debugger_ui::Continue),
                MenuItem::action(t!("Step Over"), debugger_ui::StepOver),
                MenuItem::action(t!("Step Into"), debugger_ui::StepInto),
                MenuItem::action(t!("Step Out"), debugger_ui::StepOut),
                MenuItem::separator(),
                MenuItem::action(t!("Toggle Breakpoint"), editor::actions::ToggleBreakpoint),
                MenuItem::action(t!("Edit Breakpoint"), editor::actions::EditLogBreakpoint),
                MenuItem::action(
                    t!("Clear All Breakpoints"),
                    debugger_ui::ClearAllBreakpoints,
                ),
            ],
        },
        Menu {
            name: t!("Window").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t!("Minimize"), super::Minimize),
                MenuItem::action(t!("Zoom"), super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: t!("Help").into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    t!("View Release Notes Locally"),
                    auto_update_ui::ViewReleaseNotesLocally,
                ),
                MenuItem::action(t!("View Telemetry"), zed_actions::OpenTelemetryLog),
                MenuItem::action(t!("View Dependency Licenses"), zed_actions::OpenLicenses),
                MenuItem::action(t!("Show Welcome"), onboarding::ShowWelcome),
                MenuItem::separator(),
                MenuItem::action(
                    t!("File Bug Report..."),
                    zed_actions::feedback::FileBugReport,
                ),
                MenuItem::action(
                    t!("Request Feature..."),
                    zed_actions::feedback::RequestFeature,
                ),
                MenuItem::action(t!("Email Us..."), zed_actions::feedback::EmailZed),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Documentation"),
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(t!("Zed Repository"), feedback::OpenZedRepo),
                MenuItem::action(
                    t!("Zed Twitter"),
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    t!("Join the Team"),
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
