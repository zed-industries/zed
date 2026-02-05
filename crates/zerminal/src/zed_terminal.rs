mod terminal_welcome;

use anyhow::Result;
use futures::channel::mpsc;
use futures::future::Either;
use futures::StreamExt;
use gpui::{
    actions, point, px, Action, AnyElement, App, AppContext, AsyncApp, Context, Corner, Focusable,
    IntoElement, Menu, MenuItem, OsAction, ParentElement, Styled, TitlebarOptions, UpdateGlobal,
    Window, WindowKind, WindowOptions,
};
use platform_title_bar::PlatformTitleBar;
use settings::{DEFAULT_KEYMAP_PATH, KeymapFile, SettingsStore};
use std::{path::PathBuf, sync::Arc};
use terminal_view::TerminalView;
use ui::{
    h_flex, ButtonCommon, Clickable, ContextMenu, Disableable, DynamicSpacing, IconButton,
    IconName, IconSize, PopoverMenu, Toggleable, Tooltip,
};
use uuid::Uuid;
use workspace::{
    pane::{SplitDown, SplitLeft, SplitMode, SplitRight, SplitUp},
    register_serializable_item,
    AppState, NewTerminal, ToggleZoom, Workspace,
};

pub use terminal_welcome::{OpenRecentProject, TerminalWelcomePage};

actions!(zerminal, [
    Quit,
    About,
    ToggleFullScreen,
    ShowWelcome,
    RunClaude,
    RunClaudeInDirectory,
    RunCodex,
    RunCodexInDirectory,
    RunCopilot,
    RunCopilotInDirectory,
    OpenProject,
    NewTerminalInPlace,
]);

/// Initialize terminal view without the terminal panel.
/// This is a custom init that skips terminal_panel::init() which would
/// register a NewTerminal handler that requires TerminalPanel to exist.
pub fn init_terminal_view(cx: &mut App) {
    // Register TerminalView as a serializable item
    register_serializable_item::<TerminalView>(cx);

    // Register the deploy action for TerminalView
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(TerminalView::deploy);
    })
    .detach();
}

pub fn app_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "Zerminal".into(),
            items: vec![
                MenuItem::action("About Zerminal", About),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("New Terminal", NewTerminal { local: true }),
                MenuItem::separator(),
                MenuItem::action("Close Window", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::os_action("Copy", terminal::Copy, OsAction::Copy),
                MenuItem::os_action("Paste", terminal::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::os_action("Select All", terminal::SelectAll, OsAction::SelectAll),
                MenuItem::action("Clear", terminal::Clear),
            ],
        },
        Menu {
            name: "View".into(),
            items: vec![
                MenuItem::action("Zoom In", zed_actions::IncreaseBufferFontSize { persist: true }),
                MenuItem::action("Zoom Out", zed_actions::DecreaseBufferFontSize { persist: true }),
                MenuItem::action("Reset Zoom", zed_actions::ResetBufferFontSize { persist: true }),
                MenuItem::separator(),
                MenuItem::action("Welcome", ShowWelcome),
                MenuItem::separator(),
                MenuItem::action("Toggle Full Screen", ToggleFullScreen),
            ],
        },
    ]
}

pub fn build_window_options(_display: Option<Uuid>, cx: &mut App) -> WindowOptions {
    let display = cx.primary_display();
    let display_bounds = display.as_ref().map(|d| d.bounds());
    let bounds = display_bounds.map(|display_bounds| {
        let center = display_bounds.center();
        let width = px(800.0);
        let height = px(600.0);
        gpui::Bounds {
            origin: point(center.x - width / 2.0, center.y - height / 2.0),
            size: gpui::size(width, height),
        }
    });

    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some("Zerminal".into()),
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
            ..Default::default()
        }),
        window_bounds: bounds.map(|b| gpui::WindowBounds::Windowed(b)),
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        app_id: Some("dev.zed.Zerminal".to_string()),
        window_background: gpui::WindowBackgroundAppearance::Opaque,
        window_min_size: Some(gpui::size(px(400.0), px(300.0))),
        ..Default::default()
    }
}

pub fn initialize_workspace(app_state: Arc<AppState>, cx: &mut App) {
    PlatformTitleBar::init(cx);

    cx.observe_new(move |workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let workspace_handle = cx.entity();

        // Set up a simple title bar for window dragging
        let title_bar = cx.new(|cx| PlatformTitleBar::new("terminal-title-bar", cx));
        workspace.set_titlebar_item(title_bar.into(), window, cx);

        cx.subscribe_in(&workspace_handle, window, {
            move |workspace, _, event, window, cx| {
                if let workspace::Event::PaneAdded(pane) = event {
                    initialize_pane(workspace, pane, window, cx);
                }
            }
        })
        .detach();

        let center_pane = workspace.active_pane().clone();
        initialize_pane(workspace, &center_pane, window, cx);

        let handle = cx.entity().downgrade();
        window.on_window_should_close(cx, move |window, cx| {
            handle
                .update(cx, |workspace, cx| {
                    workspace.close_window(&workspace::CloseWindow, window, cx);
                    false
                })
                .unwrap_or(true)
        });

        initialize_terminal_panel(app_state.clone(), window, cx);
        register_actions(workspace, window, cx);

        workspace.focus_handle(cx).focus(window, cx);
    })
    .detach();
}

fn initialize_pane(
    _workspace: &mut Workspace,
    pane: &gpui::Entity<workspace::Pane>,
    _window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    pane.update(cx, |pane, cx| {
        // Don't override can_split - workspace already sets it correctly
        pane.set_render_tab_bar_buttons(cx, render_terminal_tab_bar_buttons);
    });
}

fn render_terminal_tab_bar_buttons(
    pane: &mut workspace::Pane,
    window: &mut Window,
    cx: &mut gpui::Context<workspace::Pane>,
) -> (Option<AnyElement>, Option<AnyElement>) {
    if !pane.has_focus(window, cx) && !pane.context_menu_focused(window, cx) {
        return (None, None);
    }

    let can_split = pane.items_len() > 0;

    let right_children = h_flex()
        .gap(DynamicSpacing::Base04.rems(cx))
        .child(
            PopoverMenu::new("pane-tab-bar-popover-menu")
                .trigger_with_tooltip(
                    IconButton::new("plus", IconName::Plus).icon_size(IconSize::Small),
                    Tooltip::text("New..."),
                )
                .anchor(Corner::TopRight)
                .with_handle(pane.new_item_context_menu_handle.clone())
                .menu(move |window, cx| {
                    Some(ContextMenu::build(window, cx, |menu, _, _| {
                        menu.action("Welcome", ShowWelcome.boxed_clone())
                            .action("New Terminal", NewTerminal::default().boxed_clone())
                            .separator()
                            .action("Claude", RunClaude.boxed_clone())
                            .action("Codex", RunCodex.boxed_clone())
                            .action("Copilot", RunCopilot.boxed_clone())
                            .separator()
                            .action("Claude...", RunClaudeInDirectory.boxed_clone())
                            .action("Codex...", RunCodexInDirectory.boxed_clone())
                            .action("Copilot...", RunCopilotInDirectory.boxed_clone())
                    }))
                }),
        )
        .child(
            PopoverMenu::new("pane-tab-bar-split")
                .trigger_with_tooltip(
                    IconButton::new("split", IconName::Split)
                        .icon_size(IconSize::Small)
                        .disabled(!can_split),
                    Tooltip::text("Split Pane"),
                )
                .anchor(Corner::TopRight)
                .with_handle(pane.split_item_context_menu_handle.clone())
                .menu(move |window, cx| {
                    ContextMenu::build(window, cx, |menu, _, _| {
                        let mode = SplitMode::MovePane;
                        menu.action("Split Right", SplitRight { mode }.boxed_clone())
                            .action("Split Left", SplitLeft { mode }.boxed_clone())
                            .action("Split Up", SplitUp { mode }.boxed_clone())
                            .action("Split Down", SplitDown { mode }.boxed_clone())
                    })
                    .into()
                }),
        )
        .child({
            let zoomed = pane.is_zoomed();
            IconButton::new("toggle_zoom", IconName::Maximize)
                .icon_size(IconSize::Small)
                .toggle_state(zoomed)
                .selected_icon(IconName::Minimize)
                .on_click(cx.listener(|pane, _, window, cx| {
                    pane.toggle_zoom(&ToggleZoom, window, cx);
                }))
                .tooltip(move |_window, cx| {
                    Tooltip::for_action(
                        if zoomed { "Zoom Out" } else { "Zoom In" },
                        &ToggleZoom,
                        cx,
                    )
                })
        })
        .into_any_element()
        .into();

    (None, right_children)
}

fn initialize_terminal_panel(
    _app_state: Arc<AppState>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    cx.spawn_in(window, async move |workspace_handle, cx| {
        let terminal = workspace_handle.update_in(&mut cx.clone(), |workspace, _window, cx| {
            workspace
                .project()
                .update(cx, |project, cx| project.create_terminal_shell(None, cx))
        })?.await?;

        workspace_handle.update_in(&mut cx.clone(), |workspace, window, cx| {
            let terminal_view = cx.new(|cx| {
                TerminalView::new(
                    terminal.clone(),
                    workspace.weak_handle(),
                    workspace.database_id(),
                    workspace.project().downgrade(),
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(Box::new(terminal_view), None, true, window, cx);
        })?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn register_actions(
    workspace: &mut Workspace,
    _window: &mut Window,
    _cx: &mut Context<Workspace>,
) {
    workspace
        .register_action(|_, _: &Quit, _, cx| {
            cx.quit();
        })
        .register_action(|_, _: &About, _, _cx| {
            log::info!("Zerminal - A terminal-focused application built on Zed");
        })
        .register_action(|_, _: &ToggleFullScreen, window, _cx| {
            window.toggle_fullscreen();
        })
        .register_action(show_welcome)
        .register_action(new_terminal)
        .register_action(open_project)
        .register_action(open_recent_project)
        .register_action(new_terminal_in_place)
        .register_action(run_claude)
        .register_action(run_claude_in_directory)
        .register_action(run_codex)
        .register_action(run_codex_in_directory)
        .register_action(run_copilot)
        .register_action(run_copilot_in_directory);
}

fn show_welcome(workspace: &mut Workspace, _: &ShowWelcome, window: &mut Window, cx: &mut Context<Workspace>) {
    let workspace_handle = workspace.weak_handle();
    let welcome = cx.new(|cx| {
        TerminalWelcomePage::new(workspace_handle, window, cx)
    });
    workspace.add_item_to_active_pane(Box::new(welcome), None, true, window, cx);
}

fn new_terminal(workspace: &mut Workspace, _: &NewTerminal, window: &mut Window, cx: &mut Context<Workspace>) {
    let working_dir = get_current_terminal_working_directory(workspace, cx);
    let project = workspace.project().downgrade();
    let workspace_handle = workspace.weak_handle();
    let database_id = workspace.database_id();
    cx.spawn_in(window, async move |_, cx| {
        let terminal = project
            .update(cx, |project, cx| project.create_terminal_shell(working_dir, cx))?
            .await?;
        workspace_handle.update_in(cx, |workspace, window, cx| {
            let terminal_view = cx.new(|cx| {
                TerminalView::new(
                    terminal.clone(),
                    workspace.weak_handle(),
                    database_id,
                    workspace.project().downgrade(),
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(Box::new(terminal_view), None, true, window, cx);
        })?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn new_terminal_in_place(workspace: &mut Workspace, _: &NewTerminalInPlace, window: &mut Window, cx: &mut Context<Workspace>) {
    let active_pane = workspace.active_pane().clone();
    let current_item_id = active_pane.read(cx).active_item().map(|item| item.item_id());
    let current_item_index = active_pane.read(cx).active_item_index();

    let project = workspace.project().downgrade();
    let workspace_handle = workspace.weak_handle();
    let database_id = workspace.database_id();
    cx.spawn_in(window, async move |_, cx| {
        let terminal = project
            .update(cx, |project, cx| project.create_terminal_shell(None, cx))?
            .await?;
        workspace_handle.update_in(cx, |workspace, window, cx| {
            let terminal_view = cx.new(|cx| {
                TerminalView::new(
                    terminal.clone(),
                    workspace.weak_handle(),
                    database_id,
                    workspace.project().downgrade(),
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(Box::new(terminal_view), Some(current_item_index), true, window, cx);
            if let Some(item_id) = current_item_id {
                workspace.active_pane().update(cx, |pane, cx| {
                    pane.remove_item(item_id, false, false, window, cx);
                });
            }
        })?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn open_terminal_in_directory(
    workspace: &mut Workspace,
    working_directory: PathBuf,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let active_pane = workspace.active_pane().clone();
    let current_item_id = active_pane.read(cx).active_item().map(|item| item.item_id());
    let current_item_index = active_pane.read(cx).active_item_index();

    let project = workspace.project().downgrade();
    let workspace_handle = workspace.weak_handle();
    let database_id = workspace.database_id();
    cx.spawn_in(window, async move |_, cx| {
        let terminal = project
            .update(cx, |project, cx| project.create_terminal_shell(Some(working_directory), cx))?
            .await?;
        workspace_handle.update_in(cx, |workspace, window, cx| {
            let terminal_view = cx.new(|cx| {
                TerminalView::new(
                    terminal.clone(),
                    workspace.weak_handle(),
                    database_id,
                    workspace.project().downgrade(),
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(Box::new(terminal_view), Some(current_item_index), true, window, cx);
            if let Some(item_id) = current_item_id {
                workspace.active_pane().update(cx, |pane, cx| {
                    pane.remove_item(item_id, false, false, window, cx);
                });
            }
        })?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn open_project(workspace: &mut Workspace, _: &OpenProject, window: &mut Window, cx: &mut Context<Workspace>) {
    let workspace_handle = workspace.weak_handle();
    let abs_path = cx.prompt_for_paths(gpui::PathPromptOptions {
        files: false,
        directories: true,
        multiple: false,
        prompt: Some("Open Project".into()),
    });
    cx.spawn_in(window, async move |_, cx| {
        if let Ok(Ok(Some(paths))) = abs_path.await {
            if let Some(path) = paths.into_iter().next() {
                workspace_handle.update_in(cx, |workspace, window, cx| {
                    open_terminal_in_directory(workspace, path, window, cx);
                })?;
            }
        }
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn open_recent_project(workspace: &mut Workspace, action: &OpenRecentProject, window: &mut Window, cx: &mut Context<Workspace>) {
    let workspace_handle = workspace.weak_handle();
    let index = action.index;
    cx.spawn_in(window, async move |_, cx| {
        let workspaces = workspace::WORKSPACE_DB
            .recent_workspaces_on_disk()
            .await
            .unwrap_or_default();

        if let Some((_workspace_id, location, paths)) = workspaces.get(index) {
            if matches!(location, workspace::SerializedWorkspaceLocation::Local) {
                if let Some(path) = paths.paths().first() {
                    let path = path.clone();
                    workspace_handle.update_in(cx, |workspace, window, cx| {
                        open_terminal_in_directory(workspace, path, window, cx);
                    })?;
                }
            }
        }
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn get_current_terminal_working_directory(workspace: &Workspace, cx: &App) -> Option<PathBuf> {
    let active_item = workspace.active_item(cx)?;
    let terminal_view = active_item.downcast::<TerminalView>()?;
    terminal_view.read(cx).terminal().read(cx).working_directory()
}

fn run_ai_tool_in_terminal(
    workspace: &mut Workspace,
    command: &str,
    working_directory: Option<PathBuf>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().downgrade();
    let workspace_handle = workspace.weak_handle();
    let database_id = workspace.database_id();
    let command = command.to_string();
    cx.spawn_in(window, async move |_, cx| {
        let terminal = project
            .update(cx, |project, cx| project.create_terminal_shell(working_directory, cx))?
            .await?;

        let command_bytes: Vec<u8> = format!("{}\n", command).into_bytes();
        terminal.update(cx, |terminal, _cx| {
            terminal.input(command_bytes);
        });

        workspace_handle.update_in(cx, |workspace, window, cx| {
            let terminal_view = cx.new(|cx| {
                TerminalView::new(
                    terminal.clone(),
                    workspace.weak_handle(),
                    database_id,
                    workspace.project().downgrade(),
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(Box::new(terminal_view), None, true, window, cx);
        })?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn run_claude(workspace: &mut Workspace, _: &RunClaude, window: &mut Window, cx: &mut Context<Workspace>) {
    let working_dir = get_current_terminal_working_directory(workspace, cx);
    run_ai_tool_in_terminal(workspace, "claude", working_dir, window, cx);
}

fn run_claude_in_directory(workspace: &mut Workspace, _: &RunClaudeInDirectory, window: &mut Window, cx: &mut Context<Workspace>) {
    let workspace_handle = workspace.weak_handle();
    let abs_path = cx.prompt_for_paths(gpui::PathPromptOptions {
        files: false,
        directories: true,
        multiple: false,
        prompt: Some("Select directory for Claude".into()),
    });
    cx.spawn_in(window, async move |_, cx| {
        if let Ok(Ok(Some(paths))) = abs_path.await {
            if let Some(path) = paths.into_iter().next() {
                workspace_handle.update_in(cx, |workspace, window, cx| {
                    run_ai_tool_in_terminal(workspace, "claude", Some(path), window, cx);
                })?;
            }
        }
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn run_codex(workspace: &mut Workspace, _: &RunCodex, window: &mut Window, cx: &mut Context<Workspace>) {
    let working_dir = get_current_terminal_working_directory(workspace, cx);
    run_ai_tool_in_terminal(workspace, "codex", working_dir, window, cx);
}

fn run_codex_in_directory(workspace: &mut Workspace, _: &RunCodexInDirectory, window: &mut Window, cx: &mut Context<Workspace>) {
    let workspace_handle = workspace.weak_handle();
    let abs_path = cx.prompt_for_paths(gpui::PathPromptOptions {
        files: false,
        directories: true,
        multiple: false,
        prompt: Some("Select directory for Codex".into()),
    });
    cx.spawn_in(window, async move |_, cx| {
        if let Ok(Ok(Some(paths))) = abs_path.await {
            if let Some(path) = paths.into_iter().next() {
                workspace_handle.update_in(cx, |workspace, window, cx| {
                    run_ai_tool_in_terminal(workspace, "codex", Some(path), window, cx);
                })?;
            }
        }
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn run_copilot(workspace: &mut Workspace, _: &RunCopilot, window: &mut Window, cx: &mut Context<Workspace>) {
    let working_dir = get_current_terminal_working_directory(workspace, cx);
    run_ai_tool_in_terminal(workspace, "gh copilot", working_dir, window, cx);
}

fn run_copilot_in_directory(workspace: &mut Workspace, _: &RunCopilotInDirectory, window: &mut Window, cx: &mut Context<Workspace>) {
    let workspace_handle = workspace.weak_handle();
    let abs_path = cx.prompt_for_paths(gpui::PathPromptOptions {
        files: false,
        directories: true,
        multiple: false,
        prompt: Some("Select directory for Copilot".into()),
    });
    cx.spawn_in(window, async move |_, cx| {
        if let Ok(Ok(Some(paths))) = abs_path.await {
            if let Some(path) = paths.into_iter().next() {
                workspace_handle.update_in(cx, |workspace, window, cx| {
                    run_ai_tool_in_terminal(workspace, "gh copilot", Some(path), window, cx);
                })?;
            }
        }
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

pub fn handle_settings_file_changes(
    mut user_settings_file_rx: mpsc::UnboundedReceiver<String>,
    user_settings_watcher: gpui::Task<()>,
    mut global_settings_file_rx: mpsc::UnboundedReceiver<String>,
    global_settings_watcher: gpui::Task<()>,
    cx: &mut App,
) {
    let global_content = cx
        .foreground_executor()
        .block_on(global_settings_file_rx.next())
        .unwrap_or_default();
    let user_content = cx
        .foreground_executor()
        .block_on(user_settings_file_rx.next())
        .unwrap_or_default();

    SettingsStore::update_global(cx, |store, cx| {
        let _ = store.set_user_settings(&user_content, cx);
        let _ = store.set_global_settings(&global_content, cx);
    });

    cx.spawn(async move |cx| {
        let _user_settings_watcher = user_settings_watcher;
        let _global_settings_watcher = global_settings_watcher;
        let mut settings_streams = futures::stream::select(
            global_settings_file_rx.map(Either::Left),
            user_settings_file_rx.map(Either::Right),
        );

        while let Some(content) = settings_streams.next().await {
            let (content, is_user) = match content {
                Either::Left(content) => (content, false),
                Either::Right(content) => (content, true),
            };

            cx.update_global(|store: &mut SettingsStore, cx| {
                if is_user {
                    let _ = store.set_user_settings(&content, cx);
                } else {
                    let _ = store.set_global_settings(&content, cx);
                }
                cx.refresh_windows();
            });
        }
    })
    .detach();
}

pub fn handle_keymap_file_changes(
    mut user_keymap_file_rx: mpsc::UnboundedReceiver<String>,
    user_keymap_watcher: gpui::Task<()>,
    cx: &mut App,
) {
    load_default_keymap(cx);

    cx.spawn(async move |cx| {
        let _user_keymap_watcher = user_keymap_watcher;
        while let Some(content) = user_keymap_file_rx.next().await {
            cx.update(|cx| {
                if let settings::KeymapFileLoadResult::Success { key_bindings } =
                    KeymapFile::load(&content, cx)
                {
                    cx.clear_key_bindings();
                    load_default_keymap(cx);
                    for binding in key_bindings {
                        cx.bind_keys([binding]);
                    }
                }
            });
        }
    })
    .detach();
}

pub fn load_default_keymap(cx: &mut App) {
    match KeymapFile::load_asset_allow_partial_failure(DEFAULT_KEYMAP_PATH, cx) {
        Ok(bindings) => {
            log::info!("Loaded {} key bindings from default keymap", bindings.len());
            cx.bind_keys(bindings);
        }
        Err(err) => {
            log::error!("Failed to load default keymap: {}", err);
        }
    }
}

pub async fn open_terminal_workspace(
    app_state: Arc<AppState>,
    working_directory: Option<PathBuf>,
    cx: &mut AsyncApp,
) -> Result<()> {
    let working_dir = working_directory.clone();
    let window = cx.update(move |cx| {
        workspace::open_new(
            Default::default(),
            app_state.clone(),
            cx,
            move |workspace, _window, cx| {
                if let Some(working_dir) = working_dir.clone() {
                    workspace.project().update(cx, |project, cx| {
                        project.create_worktree(&working_dir, true, cx).detach();
                    });
                }
            },
        )
    });

    window.await?;
    Ok(())
}
