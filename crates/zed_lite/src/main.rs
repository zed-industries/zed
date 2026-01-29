// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod zed_lite;

use anyhow::Result;
use clap::Parser;
use fs::{Fs, RealFs};
use gpui::{App, AppContext, Application, AsyncApp};
use gpui_tokio;
use parking_lot::Mutex;
use release_channel::{AppCommitSha, AppVersion};
use reqwest_client::ReqwestClient;
use settings::Settings;
use std::{
    env,
    sync::{Arc, OnceLock},
    time::Instant,
};
use util::ResultExt;
use watch;
use workspace::{AppState, WorkspaceSettings};
use zed_lite::build_window_options;

use assets::Assets;

static STARTUP_TIME: OnceLock<Instant> = OnceLock::new();

fn main() {
    STARTUP_TIME.get_or_init(|| Instant::now());

    let args = Args::parse();

    // Set custom data directory.
    if let Some(dir) = &args.user_data_dir {
        paths::set_custom_data_dir(dir);
    }

    let file_errors = init_paths();
    if !file_errors.is_empty() {
        eprintln!("Failed to create required directories: {:?}", file_errors);
        return;
    }

    env_logger::init();

    let version = option_env!("ZED_LITE_BUILD_ID");
    let app_commit_sha =
        option_env!("ZED_LITE_COMMIT_SHA").map(|commit_sha| AppCommitSha::new(commit_sha.to_string()));
    let app_version = AppVersion::load(env!("CARGO_PKG_VERSION"), version, app_commit_sha.clone());

    log::info!(
        "========== starting zed_lite version {} ==========",
        app_version,
    );

    let app = Application::new().with_assets(Assets);

    let fs = Arc::new(RealFs::new(None, app.background_executor()));

    app.run(move |cx| {
        release_channel::init(app_version, cx);
        gpui_tokio::init(cx);
        if let Some(app_commit_sha) = app_commit_sha {
            AppCommitSha::set_global(app_commit_sha, cx);
        }
        settings::init(cx);

        // Set up HTTP client
        let user_agent = format!(
            "ZedLite/{} ({}; {})",
            AppVersion::global(cx),
            std::env::consts::OS,
            std::env::consts::ARCH
        );
        let http = ReqwestClient::user_agent(&user_agent)
            .expect("could not start HTTP client");
        cx.set_http_client(Arc::new(http));

        <dyn Fs>::set_global(fs.clone(), cx);

        theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);
        load_embedded_fonts(cx);

        // Initialize basic components
        menu::init();
        zed_actions::init();
        
        // Initialize UI components
        editor::init(cx);

        // Create minimal app state
        let languages = Arc::new(language::LanguageRegistry::new(cx.background_executor().clone()));
        let client = client::Client::production(cx);
        cx.set_http_client(client.http_client());
        let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
        let workspace_store = cx.new(|cx| workspace::WorkspaceStore::new(client.clone(), cx));
        let session = cx.new(|cx| session::AppSession::new(
            futures::executor::block_on(session::Session::new("zed_lite".to_string())),
            cx
        ));

        // Initialize call system (required by workspace)
        call::init(client.clone(), user_store.clone(), cx);
        
        // Initialize title bar after call system
        title_bar::init(cx);

        let app_state = Arc::new(AppState {
            languages,
            client,
            user_store,
            fs: fs.clone(),
            build_window_options,
            workspace_store,
            node_runtime: node_runtime::NodeRuntime::new(
                client::Client::production(cx).http_client(),
                None,
                watch::channel(None).1,
            ),
            session,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        workspace::init(app_state.clone(), cx);

        cx.activate(true);

        // Create initial workspace window
        cx.spawn(async move |cx| {
            if let Err(e) = create_empty_workspace(app_state, cx).await {
                log::error!("Failed to create workspace: {}", e);
            }
        })
        .detach();
    });
}

async fn create_empty_workspace(app_state: Arc<AppState>, cx: &mut AsyncApp) -> Result<()> {
    cx.update(|cx| {
        workspace::open_new(
            Default::default(),
            app_state,
            cx,
            |workspace, window, cx| {
                // Create an empty workspace with Zed's full UI framework
                // This will show the title bar, panels, and workspace layout
                let restore_on_startup = WorkspaceSettings::get_global(cx).restore_on_startup;
                match restore_on_startup {
                    workspace::RestoreOnStartupBehavior::Launchpad => {
                        // Show launchpad if configured
                    }
                    _ => {
                        // Create a new empty file to show the editor
                        editor::Editor::new_file(workspace, &Default::default(), window, cx);
                    }
                }
            },
        )
    })
    .await?;

    Ok(())
}

fn init_paths() -> Vec<std::io::Error> {
    let mut errors = Vec::new();
    
    for path in [
        paths::config_dir(),
        paths::database_dir(),
        paths::logs_dir(),
        paths::temp_dir(),
    ] {
        if let Err(e) = std::fs::create_dir_all(path) {
            errors.push(e);
        }
    }
    
    errors
}

fn load_embedded_fonts(cx: &App) {
    let asset_source = cx.asset_source();
    let font_paths = asset_source.list("fonts").unwrap_or_default();
    let embedded_fonts = Mutex::new(Vec::new());
    let executor = cx.background_executor();

    cx.foreground_executor().block_on(executor.scoped(|scope| {
        for font_path in &font_paths {
            if !font_path.ends_with(".ttf") {
                continue;
            }

            scope.spawn(async {
                if let Ok(Some(font_bytes)) = asset_source.load(font_path) {
                    embedded_fonts.lock().push(font_bytes);
                }
            });
        }
    }));

    let fonts = embedded_fonts.into_inner();
    if !fonts.is_empty() {
        cx.text_system().add_fonts(fonts).log_err();
    }
}

#[derive(Parser, Debug)]
#[command(name = "zed_lite", disable_version_flag = true)]
struct Args {
    /// Sets a custom directory for all user data
    #[arg(long, value_name = "DIR")]
    user_data_dir: Option<String>,
}