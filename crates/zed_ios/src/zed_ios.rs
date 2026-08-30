//! iOS entry point for Zed.
//!
//! This static library boots a minimal Zed session (settings, themes, fonts,
//! workspace, editor) inside a UIKit host application. The Objective-C shell
//! in `app/main.m` owns the UIKit run loop and calls [`zed_ios_run`] once the
//! window scene connects.

#![cfg(target_os = "ios")]

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use assets::Assets;
use client::{Client, UserStore};
use fs::RealFs;
use gpui::AppContext as _;
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use reqwest_client::ReqwestClient;
use session::{AppSession, Session};
use ui::App;
use workspace::{AppState, Workspace, WorkspaceStore};

fn init_zed(cx: &mut App) -> anyhow::Result<()> {
    let version = release_channel::AppVersion::load(env!("CARGO_PKG_VERSION"), None, None);
    release_channel::init(version, cx);
    gpui_tokio::init(cx);
    cx.set_global(db::AppDatabase::new());
    let db_trusted_paths = match workspace::WorkspaceDb::global(cx).fetch_trusted_worktrees() {
        Ok(trusted_paths) => trusted_paths,
        Err(error) => {
            log::error!("failed to fetch trusted worktrees: {error:#}");
            Default::default()
        }
    };
    project::trusted_worktrees::init(db_trusted_paths, cx);
    Assets.load_fonts(cx)?;

    let http_client = ReqwestClient::user_agent("zed-ios")?;
    cx.set_http_client(Arc::new(http_client));

    let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
    <dyn fs::Fs>::set_global(fs.clone(), cx);

    settings::init(cx);
    use gpui::UpdateGlobal as _;
    // Writes go through atomic renames inside this directory; without it the
    // very first "save a connection" write fails and the setting is lost.
    let settings_path = paths::settings_file();
    if let Some(parent) = settings_path.parent()
        && let Err(error) = std::fs::create_dir_all(parent)
    {
        log::error!("failed to create the settings directory: {error}");
    }
    if !settings_path.exists()
        && let Err(error) = std::fs::write(
            settings_path,
            settings::initial_user_settings_content().as_bytes(),
        )
    {
        log::error!("failed to create the settings file: {error}");
    }
    let user_settings = std::fs::read_to_string(settings_path).unwrap_or_default();
    settings::SettingsStore::update_global(cx, |store, cx| {
        // Baseline for touch devices, kept in the global layer so the user's
        // own settings file can override it without erasing it.
        let result = store.set_global_settings(IOS_BASELINE_SETTINGS, cx);
        if let settings::ParseStatus::Failed { error } = &result.parse_status {
            log::error!("failed to apply the iOS baseline settings: {error}");
        }
        if !user_settings.is_empty() {
            let result = store.set_user_settings(&user_settings, cx);
            if let settings::ParseStatus::Failed { error } = &result.parse_status {
                log::error!("failed to apply the user settings: {error}");
            }
        }
    });
    theme_settings::init(theme::LoadThemes::All(Box::new(Assets)), cx);

    // There is no file-watcher backend on iOS, so poll the settings file and
    // feed changes (e.g. a connection added through the UI, which writes the
    // file) back into the store.
    cx.spawn(async move |cx| {
        let mut last_contents: Option<String> = None;
        loop {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(2))
                .await;
            let Ok(contents) = std::fs::read_to_string(paths::settings_file()) else {
                continue;
            };
            if last_contents.as_deref() == Some(contents.as_str()) {
                continue;
            }
            last_contents = Some(contents.clone());
            cx.update(|cx| {
                settings::SettingsStore::update_global(cx, |store, cx| {
                    let result = store.set_user_settings(&contents, cx);
                    if let settings::ParseStatus::Failed { error } = &result.parse_status {
                        log::error!("failed to reload the settings file: {error}");
                    }
                })
            });
        }
    })
    .detach();

    let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
    let client = Client::production(cx);
    client::init(&client, cx);

    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    let workspace_store = cx.new(|cx| WorkspaceStore::new(client.clone(), cx));
    let session_id = uuid::Uuid::new_v4().to_string();
    let key_value_store = db::kvp::KeyValueStore::global(cx);
    let session = cx
        .foreground_executor()
        .block_on(Session::new(session_id, key_value_store));
    let session = cx.new(|cx| AppSession::new(session, cx));
    let node_runtime = NodeRuntime::unavailable();

    let app_state = Arc::new(AppState {
        languages: languages.clone(),
        client: client.clone(),
        user_store: user_store.clone(),
        workspace_store,
        fs: fs.clone(),
        build_window_options: |_, _| Default::default(),
        node_runtime: node_runtime.clone(),
        session,
    });
    AppState::set_global(app_state.clone(), cx);

    workspace::init(app_state.clone(), cx);
    editor::init(cx);
    file_finder::init(cx);
    command_palette::init(cx);
    project_panel::init(cx);
    languages::init(languages.clone(), fs.clone(), node_runtime.clone(), cx);
    menu::init();
    language_model::init(cx);
    zed_actions::init();
    theme_selector::init(cx);
    outline::init(cx);
    outline_panel::init(cx);
    tab_switcher::init(cx);
    search::init(cx);
    go_to_line::init(cx);
    markdown_preview::init(cx);
    git_ui::init(cx);
    terminal_view::init(cx);
    recent_projects::init(cx);

    // Not every action referenced by the bundled keymap is registered in this
    // trimmed-down app, so tolerate individual binding failures.
    match settings::KeymapFile::load_asset_allow_partial_failure(settings::DEFAULT_KEYMAP_PATH, cx)
    {
        Ok(key_bindings) => cx.bind_keys(key_bindings),
        Err(error) => log::error!("failed to load the default keymap: {error:#}"),
    }

    {
        use theme::ActiveTheme as _;
        languages.set_theme(cx.theme().clone());
    }
    cx.observe_global::<theme::GlobalTheme>({
        use theme::ActiveTheme as _;
        let languages = languages.clone();
        move |cx| {
            languages.set_theme(cx.theme().clone());
        }
    })
    .detach();

    cx.observe_new(
        |workspace: &mut Workspace, mut window, cx: &mut gpui::Context<Workspace>| {
            workspace.register_action(open_settings_file);
            if let Some(window) = window.as_deref_mut() {
                let touch_action_bar = cx.new(|_| TouchActionBar);
                workspace.status_bar().update(cx, |status_bar, cx| {
                    status_bar.add_left_item(touch_action_bar, window, cx);
                });
            }
            workspace.register_action(
                |workspace, _: &zed_actions::OpenSettings, window, cx| {
                    open_settings_file(workspace, &zed_actions::OpenSettingsFile, window, cx);
                },
            );

            let Some(window) = window else { return };

            let center_pane = workspace.active_pane().clone();
            initialize_pane(workspace, &center_pane, window, cx);
            cx.subscribe_in(&cx.entity(), window, |workspace, _, event, window, cx| {
                if let workspace::Event::PaneAdded(pane) = event {
                    initialize_pane(workspace, &pane.clone(), window, cx);
                }
            })
            .detach();

            async fn add_panel_when_ready(
                panel_task: impl Future<Output = anyhow::Result<gpui::Entity<impl workspace::Panel>>>
                + 'static,
                workspace_handle: gpui::WeakEntity<Workspace>,
                mut cx: gpui::AsyncWindowContext,
            ) {
                match panel_task.await {
                    Ok(panel) => {
                        workspace_handle
                            .update_in(&mut cx, |workspace, window, cx| {
                                workspace.add_panel(panel, window, cx);
                            })
                            .ok();
                    }
                    Err(error) => log::error!("failed to load panel: {error:#}"),
                }
            }

            cx.spawn_in(window, async move |workspace_handle, cx| {
                let project_panel =
                    project_panel::ProjectPanel::load(workspace_handle.clone(), cx.clone());
                let outline_panel =
                    outline_panel::OutlinePanel::load(workspace_handle.clone(), cx.clone());
                let git_panel = git_ui::git_panel::GitPanel::load(workspace_handle.clone(), cx.clone());
                let terminal_panel = terminal_view::terminal_panel::TerminalPanel::load(
                    workspace_handle.clone(),
                    cx.clone(),
                );
                futures::join!(
                    add_panel_when_ready(project_panel, workspace_handle.clone(), cx.clone()),
                    add_panel_when_ready(outline_panel, workspace_handle.clone(), cx.clone()),
                    add_panel_when_ready(git_panel, workspace_handle.clone(), cx.clone()),
                    add_panel_when_ready(terminal_panel, workspace_handle.clone(), cx.clone()),
                );
            })
            .detach();
        },
    )
    .detach();

    let open_task = Workspace::new_local(
        Vec::new(),
        app_state,
        None,
        None,
        None,
        workspace::OpenMode::Activate,
        cx,
    );
    cx.spawn(async move |cx| {
        if let Err(error) = open_task.await {
            log::error!("failed to open the initial workspace window: {error:#}");
        }
        cx.update(|cx| cx.activate(true));
    })
    .detach();
    Ok(())
}

fn initialize_pane(
    workspace: &Workspace,
    pane: &gpui::Entity<workspace::Pane>,
    window: &mut gpui::Window,
    cx: &mut gpui::Context<Workspace>,
) {
    pane.update(cx, |pane, cx| {
        pane.toolbar().update(cx, |toolbar, cx| {
            let breadcrumbs = cx.new(|_| breadcrumbs::Breadcrumbs::new());
            toolbar.add_item(breadcrumbs, window, cx);
            let buffer_search_bar = cx.new(|cx| {
                search::BufferSearchBar::new(
                    Some(workspace.project().read(cx).languages().clone()),
                    window,
                    cx,
                )
            });
            toolbar.add_item(buffer_search_bar, window, cx);
        });
    });
}

// Baseline suited to a touch device: no system file dialogs, no telemetry,
// and autosave because there is no cmd-s on the on-screen keyboard.
const IOS_BASELINE_SETTINGS: &str = r#"{
  "use_system_path_prompts": false,
  "telemetry": { "diagnostics": false, "metrics": false },
  "autosave": { "after_delay": { "milliseconds": 1000 } }
}
"#;

/// Touch-reachable buttons for actions that otherwise need a keyboard:
/// the command palette (which reaches everything else) and save.
struct TouchActionBar;

impl workspace::StatusItemView for TouchActionBar {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn workspace::ItemHandle>,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        None
    }
}

impl gpui::Render for TouchActionBar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        use ui::prelude::*;

        ui::h_flex()
            .gap_1()
            .child(
                ui::IconButton::new("touch-command-palette", ui::IconName::ListCollapse)
                    .icon_size(ui::IconSize::Small)
                    .tooltip(ui::Tooltip::text("Command Palette"))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(zed_actions::command_palette::Toggle), cx);
                    }),
            )
            .child(
                ui::IconButton::new("touch-save", ui::IconName::Check)
                    .icon_size(ui::IconSize::Small)
                    .tooltip(ui::Tooltip::text("Save"))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(workspace::Save { save_intent: None }), cx);
                    }),
            )
    }
}

fn open_settings_file(
    workspace: &mut Workspace,
    _: &zed_actions::OpenSettingsFile,
    window: &mut gpui::Window,
    cx: &mut gpui::Context<Workspace>,
) {
    let settings_path = paths::settings_file().clone();
    if !settings_path.exists() {
        if let Some(parent) = settings_path.parent()
            && let Err(error) = std::fs::create_dir_all(parent)
        {
            log::error!("failed to create the settings directory: {error}");
            return;
        }
        if let Err(error) = std::fs::write(
            &settings_path,
            settings::initial_user_settings_content().as_bytes(),
        ) {
            log::error!("failed to create the settings file: {error}");
            return;
        }
    }
    let open_task = workspace.open_abs_path(settings_path, Default::default(), window, cx);
    cx.spawn(async move |_, _| {
        if let Err(error) = open_task.await {
            log::error!("failed to open the settings file: {error:#}");
        }
    })
    .detach();
}

#[unsafe(no_mangle)]
pub extern "C" fn zed_ios_run() -> bool {
    zlog::init();
    zlog::init_output_stdout();

    let did_start = Rc::new(Cell::new(false));
    gpui_ios::ios::ffi::set_app_callback(Box::new({
        let did_start = did_start.clone();
        move |cx: &mut App| match init_zed(cx) {
            Ok(()) => did_start.set(true),
            Err(error) => log::error!("failed to start Zed on iOS: {error:#}"),
        }
    }));
    gpui_ios::ios::ffi::run_app_with_assets(Assets);
    did_start.get()
}
