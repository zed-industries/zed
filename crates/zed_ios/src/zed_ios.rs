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
use gpui::{AppContext as _, WindowOptions};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use project::Project;
use reqwest_client::ReqwestClient;
use session::{AppSession, Session};
use ui::App;
use workspace::{AppState, Workspace, WorkspaceStore};

fn init_zed(cx: &mut App) -> anyhow::Result<()> {
    let version = release_channel::AppVersion::load(env!("CARGO_PKG_VERSION"), None, None);
    release_channel::init(version, cx);
    cx.set_global(db::AppDatabase::new());
    Assets.load_fonts(cx)?;

    let http_client = ReqwestClient::user_agent("zed-ios")?;
    cx.set_http_client(Arc::new(http_client));

    let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
    <dyn fs::Fs>::set_global(fs.clone(), cx);

    settings::init(cx);
    // iOS has no system open/save dialogs, so route path prompts through
    // Zed's built-in picker registered by `file_finder::init`.
    use gpui::UpdateGlobal as _;
    settings::SettingsStore::update_global(cx, |store, cx| {
        let result = store.set_user_settings(r#"{"use_system_path_prompts": false}"#, cx);
        if let settings::ParseStatus::Failed { error } = &result.parse_status {
            log::error!("failed to apply iOS default settings: {error}");
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

    let project = Project::local(
        client,
        node_runtime,
        user_store,
        languages,
        fs,
        None,
        Default::default(),
        cx,
    );

    cx.open_window(WindowOptions::default(), |window, cx| {
        cx.new(|cx| Workspace::new(None, project, app_state.clone(), window, cx))
    })?;
    cx.activate(true);
    Ok(())
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
