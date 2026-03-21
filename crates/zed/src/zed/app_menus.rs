use collab_ui::collab_panel;
use gpui::{App, Menu, MenuItem, OsAction};
use release_channel::ReleaseChannel;
use terminal_view::terminal_panel;
use zed_actions::{debug_panel, dev};

use super::super::i18n::t;

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    use zed_actions::Quit;

    let mut view_items = vec![
        MenuItem::action(t("view.zoom_in"), zed_actions::IncreaseBufferFontSize { persist: false }),
        MenuItem::action(t("view.zoom_out"), zed_actions::DecreaseBufferFontSize { persist: false }),
        MenuItem::action(t("view.reset_zoom"), zed_actions::ResetBufferFontSize { persist: false }),
        MenuItem::action(t("view.reset_all_zoom"), zed_actions::ResetAllZoom { persist: false }),
        MenuItem::separator(),
        MenuItem::action(t("view.toggle_left_dock"), workspace::ToggleLeftDock),
        MenuItem::action(t("view.toggle_right_dock"), workspace::ToggleRightDock),
        MenuItem::action(t("view.toggle_bottom_dock"), workspace::ToggleBottomDock),
        MenuItem::action(t("view.toggle_all_docks"), workspace::ToggleAllDocks),
        MenuItem::submenu(Menu {
            name: t("editor_layout").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t("view.split_up"), workspace::SplitUp::default()),
                MenuItem::action(t("view.split_down"), workspace::SplitDown::default()),
                MenuItem::action(t("view.split_left"), workspace::SplitLeft::default()),
                MenuItem::action(t("view.split_right"), workspace::SplitRight::default()),
            ],
        }),
        MenuItem::separator(),
        MenuItem::action(t("view.project_panel"), zed_actions::project_panel::ToggleFocus),
        MenuItem::action(t("view.outline_panel"), outline_panel::ToggleFocus),
        MenuItem::action(t("view.collab_panel"), collab_panel::ToggleFocus),
        MenuItem::action(t("view.terminal_panel"), terminal_panel::ToggleFocus),
        MenuItem::action(t("view.debugger_panel"), debug_panel::ToggleFocus),
        MenuItem::separator(),
        MenuItem::action(t("view.diagnostics"), diagnostics::Deploy),
        MenuItem::separator(),
    ];

    if ReleaseChannel::try_global(cx) == Some(ReleaseChannel::Dev) {
        view_items.push(MenuItem::action(t("view.toggle_gpu_inspector"), dev::ToggleInspector));
        view_items.push(MenuItem::separator());
    }

    vec![
        Menu {
            name: t("app.name").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t("app.about"), zed_actions::About),
                MenuItem::action(t("app.check_for_updates"), auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu::new(t("settings.open_settings")).items([
                    MenuItem::action(t("settings.open_settings"), zed_actions::OpenSettings),
                    MenuItem::action(t("settings.open_settings_file"), super::OpenSettingsFile),
                    MenuItem::action(t("settings.open_project_settings"), zed_actions::OpenProjectSettings),
                    MenuItem::action(t("settings.open_project_settings_file"), super::OpenProjectSettingsFile),
                    MenuItem::action(t("settings.open_default_settings"), super::OpenDefaultSettings),
                    MenuItem::separator(),
                    MenuItem::action(t("settings.open_keymap"), zed_actions::OpenKeymap),
                    MenuItem::action(t("settings.open_keymap_file"), zed_actions::OpenKeymapFile),
                    MenuItem::action(t("settings.open_default_key_bindings"), zed_actions::OpenDefaultKeymap),
                    MenuItem::separator(),
                    MenuItem::action(t("settings.select_theme"), zed_actions::theme_selector::Toggle::default()),
                    MenuItem::action(t("settings.select_icon_theme"), zed_actions::icon_theme_selector::Toggle::default()),
                ])),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu(t("services"), gpui::SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action(t("extensions"), zed_actions::Extensions::default()),
                #[cfg(not(target_os = "windows"))]
                MenuItem::action(t("install_cli"), install_cli::InstallCliBinary),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action(t("app.hide"), super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action(t("app.hide_others"), super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action(t("app.show_all"), super::ShowAll),
                MenuItem::separator(),
                MenuItem::action(t("app.quit"), Quit),
            ],
        },
        Menu {
            name: t("menu.file").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t("file.new"), workspace::NewFile),
                MenuItem::action(t("file.new_window"), workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action(t("file.open_file"), workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        t("file.open_folder")
                    } else {
                        t("file.open_folder")
                    },
                    workspace::Open::default(),
                ),
                MenuItem::action(t("file.open_recent"), zed_actions::OpenRecent {
                    create_new_window: false,
                }),
                MenuItem::action(t("file.open_remote"), zed_actions::OpenRemote {
                    create_new_window: false,
                    from_existing_connection: false,
                }),
                MenuItem::separator(),
                MenuItem::action(t("file.add_folder_to_project"), workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action(t("file.save"), workspace::Save { save_intent: None }),
                MenuItem::action(t("file.save_as"), workspace::SaveAs),
                MenuItem::action(t("file.save_all"), workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(t("file.close_editor"), workspace::CloseActiveItem {
                    save_intent: None,
                    close_pinned: true,
                }),
                MenuItem::action(t("file.close_project"), workspace::CloseProject),
                MenuItem::action(t("file.close_window"), workspace::CloseWindow),
            ],
        },
        Menu {
            name: t("menu.edit").into(),
            disabled: false,
            items: vec![
                MenuItem::os_action(t("edit.undo"), editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action(t("edit.redo"), editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action(t("edit.cut"), editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action(t("edit.copy"), editor::actions::Copy, OsAction::Copy),
                MenuItem::action(t("edit.copy_and_trim"), editor::actions::CopyAndTrim),
                MenuItem::os_action(t("edit.paste"), editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action(t("edit.find"), search::buffer_search::Deploy::find()),
                MenuItem::action(t("edit.find_in_project"), workspace::DeploySearch::find()),
                MenuItem::separator(),
                MenuItem::action(t("edit.toggle_line_comment"), editor::actions::ToggleComments::default()),
            ],
        },
        Menu {
            name: t("menu.selection").into(),
            disabled: false,
            items: vec![
                MenuItem::os_action(t("selection.select_all"), editor::actions::SelectAll, OsAction::SelectAll),
                MenuItem::action(t("selection.expand_selection"), editor::actions::SelectLargerSyntaxNode),
                MenuItem::action(t("selection.shrink_selection"), editor::actions::SelectSmallerSyntaxNode),
                MenuItem::action(t("selection.select_next_sibling"), editor::actions::SelectNextSyntaxNode),
                MenuItem::action(t("selection.select_previous_sibling"), editor::actions::SelectPreviousSyntaxNode),
                MenuItem::separator(),
                MenuItem::action(t("selection.add_cursor_above"), editor::actions::AddSelectionAbove {
                    skip_soft_wrap: true,
                }),
                MenuItem::action(t("selection.add_cursor_below"), editor::actions::AddSelectionBelow {
                    skip_soft_wrap: true,
                }),
                MenuItem::action(t("selection.select_next_occurrence"), editor::actions::SelectNext {
                    replace_newest: false,
                }),
                MenuItem::action(t("selection.select_previous_occurrence"), editor::actions::SelectPrevious {
                    replace_newest: false,
                }),
                MenuItem::action(t("selection.select_all_occurrences"), editor::actions::SelectAllMatches),
                MenuItem::separator(),
                MenuItem::action(t("selection.move_line_up"), editor::actions::MoveLineUp),
                MenuItem::action(t("selection.move_line_down"), editor::actions::MoveLineDown),
                MenuItem::action(t("selection.duplicate_selection"), editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: t("menu.view").into(),
            disabled: false,
            items: view_items,
        },
        Menu {
            name: t("menu.go").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t("go.back"), workspace::GoBack),
                MenuItem::action(t("go.forward"), workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action(t("go.command_palette"), zed_actions::command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action(t("go.go_to_file"), workspace::ToggleFileFinder::default()),
                MenuItem::action(t("go.go_to_symbol"), zed_actions::outline::ToggleOutline),
                MenuItem::action(t("go.go_to_line"), editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action(t("go.go_to_definition"), editor::actions::GoToDefinition),
                MenuItem::action(t("go.go_to_declaration"), editor::actions::GoToDeclaration),
                MenuItem::action(t("go.go_to_type_definition"), editor::actions::GoToTypeDefinition),
                MenuItem::action(t("go.find_all_references"), editor::actions::FindAllReferences::default()),
                MenuItem::separator(),
                MenuItem::action(t("go.next_problem"), editor::actions::GoToDiagnostic::default()),
                MenuItem::action(t("go.previous_problem"), editor::actions::GoToPreviousDiagnostic::default()),
            ],
        },
        Menu {
            name: t("menu.run").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t("run.spawn_task"), zed_actions::Spawn::ViaModal {
                    reveal_target: None,
                }),
                MenuItem::action(t("run.start_debugger"), debugger_ui::Start),
                MenuItem::separator(),
                MenuItem::action(t("run.edit_tasks"), crate::zed::OpenProjectTasks),
                MenuItem::action(t("run.edit_debug"), zed_actions::OpenProjectDebugTasks),
                MenuItem::separator(),
                MenuItem::action(t("run.continue"), debugger_ui::Continue),
                MenuItem::action(t("run.step_over"), debugger_ui::StepOver),
                MenuItem::action(t("run.step_into"), debugger_ui::StepInto),
                MenuItem::action(t("run.step_out"), debugger_ui::StepOut),
                MenuItem::separator(),
                MenuItem::action(t("run.toggle_breakpoint"), editor::actions::ToggleBreakpoint),
                MenuItem::action(t("run.edit_breakpoint"), editor::actions::EditLogBreakpoint),
                MenuItem::action(t("run.clear_all_breakpoints"), debugger_ui::ClearAllBreakpoints),
            ],
        },
        Menu {
            name: t("menu.window").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t("window.minimize"), super::Minimize),
                MenuItem::action(t("window.zoom"), super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: t("menu.help").into(),
            disabled: false,
            items: vec![
                MenuItem::action(t("help.view_release_notes"), auto_update_ui::ViewReleaseNotesLocally),
                MenuItem::action(t("help.view_telemetry"), zed_actions::OpenTelemetryLog),
                MenuItem::action(t("help.view_licenses"), zed_actions::OpenLicenses),
                MenuItem::action(t("help.show_welcome"), onboarding::ShowWelcome),
                MenuItem::separator(),
                MenuItem::action(t("help.file_bug_report"), zed_actions::feedback::FileBugReport),
                MenuItem::action(t("help.request_feature"), zed_actions::feedback::RequestFeature),
                MenuItem::action(t("help.email_us"), zed_actions::feedback::EmailZed),
                MenuItem::separator(),
                MenuItem::action(t("help.documentation"), super::OpenBrowser {
                    url: "https://zed.dev/docs".into(),
                }),
                MenuItem::action(t("help.zed_repository"), feedback::OpenZedRepo),
                MenuItem::action(t("help.zed_twitter"), super::OpenBrowser {
                    url: "https://twitter.com/zeddotdev".into(),
                }),
                MenuItem::action(t("help.join_the_team"), super::OpenBrowser {
                    url: "https://zed.dev/jobs".into(),
                }),
            ],
        },
    ]
}