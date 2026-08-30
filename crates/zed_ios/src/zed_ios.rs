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
    let user_settings = std::fs::read_to_string(paths::settings_file()).unwrap_or_default();
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
        |workspace: &mut Workspace, window, cx: &mut gpui::Context<Workspace>| {
            workspace.register_action(open_settings_file);
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
                futures::join!(
                    add_panel_when_ready(project_panel, workspace_handle.clone(), cx.clone()),
                    add_panel_when_ready(outline_panel, workspace_handle.clone(), cx.clone()),
                    add_panel_when_ready(git_panel, workspace_handle.clone(), cx.clone()),
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
