#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use clap::Parser;
use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
use client::{Client, ProxySettings, UserStore};
use collections::HashMap;
use crashes::InitCrashHandler;
use db::kvp::KEY_VALUE_STORE;
use fs::{Fs, RealFs};
use futures::channel::oneshot;
use gpui::{App, AppContext as _, Application};
use gpui_tokio::Tokio;
use language::LanguageRegistry;
use node_runtime::{NodeBinaryOptions, NodeRuntime};
use parking_lot::Mutex;
use project::project_settings::ProjectSettings;
use release_channel::{AppCommitSha, AppVersion};
use reqwest_client::ReqwestClient;
use session::{AppSession, Session};
use settings::{Settings, SettingsStore, watch_config_file};
use std::{
    env,
    io::{self, IsTerminal},
    path::PathBuf,
    sync::{Arc, OnceLock},
    time::Instant,
};
use util::ResultExt;
use uuid::Uuid;
use workspace::{AppState, WorkspaceStore};

use assets::Assets;

static STARTUP_TIME: OnceLock<Instant> = OnceLock::new();

fn main() {
    STARTUP_TIME.get_or_init(|| Instant::now());

    #[cfg(unix)]
    util::prevent_root_execution();

    let args = Args::parse();

    if let Some(dir) = &args.user_data_dir {
        paths::set_custom_data_dir(dir);
    }

    let file_errors = init_paths();
    if !file_errors.is_empty() {
        eprintln!("Zerminal failed to create required directories: {:?}", file_errors);
        return;
    }

    zlog::init();

    if stdout_is_a_pty() {
        zlog::init_output_stdout();
    } else {
        let result = zlog::init_output_file(paths::log_file(), Some(paths::old_log_file()));
        if let Err(err) = result {
            eprintln!("Could not open log file: {}... Defaulting to stdout", err);
            zlog::init_output_stdout();
        };
    }
    ztracing::init();

    let version = option_env!("ZED_BUILD_ID");
    let app_commit_sha =
        option_env!("ZED_COMMIT_SHA").map(|commit_sha| AppCommitSha::new(commit_sha.to_string()));
    let app_version = AppVersion::load(env!("CARGO_PKG_VERSION"), version, app_commit_sha.clone());

    rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism().map_or(1, |n| n.get().div_ceil(2)))
        .stack_size(10 * 1024 * 1024)
        .thread_name(|ix| format!("RayonWorker{}", ix))
        .build_global()
        .unwrap();

    log::info!(
        "========== starting zerminal version {}, sha {} ==========",
        app_version,
        app_commit_sha
            .as_ref()
            .map(|sha| sha.short())
            .as_deref()
            .unwrap_or("unknown"),
    );

    let app = Application::new().with_assets(Assets);

    let system_id = app.background_executor().spawn(system_id());
    let installation_id = app.background_executor().spawn(installation_id());
    let session_id = Uuid::new_v4().to_string();
    let session = app
        .background_executor()
        .spawn(Session::new(session_id.clone()));

    app.background_executor()
        .spawn(crashes::init(InitCrashHandler {
            session_id,
            zed_version: app_version.to_string(),
            binary: "zerminal".to_string(),
            release_channel: release_channel::RELEASE_CHANNEL_NAME.clone(),
            commit_sha: app_commit_sha
                .as_ref()
                .map(|sha| sha.full())
                .unwrap_or_else(|| "no sha".to_owned()),
        }))
        .detach();

    let fs = Arc::new(RealFs::new(None, app.background_executor()));
    let (user_settings_file_rx, user_settings_watcher) = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::settings_file().clone(),
    );
    let (global_settings_file_rx, global_settings_watcher) = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::global_settings_file().clone(),
    );
    let (user_keymap_file_rx, user_keymap_watcher) = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::keymap_file().clone(),
    );

    let (shell_env_loaded_tx, shell_env_loaded_rx) = oneshot::channel();
    if !stdout_is_a_pty() {
        app.background_executor()
            .spawn(async {
                #[cfg(unix)]
                util::load_login_shell_environment().await.log_err();
                shell_env_loaded_tx.send(()).ok();
            })
            .detach()
    } else {
        drop(shell_env_loaded_tx)
    }

    app.run(move |cx| {
        menu::init();
        zed_actions::init();

        release_channel::init(app_version, cx);
        gpui_tokio::init(cx);
        if let Some(app_commit_sha) = app_commit_sha {
            AppCommitSha::set_global(app_commit_sha, cx);
        }
        settings::init(cx);
        zlog_settings::init(cx);
        zerminal::handle_settings_file_changes(
            user_settings_file_rx,
            user_settings_watcher,
            global_settings_file_rx,
            global_settings_watcher,
            cx,
        );
        zerminal::handle_keymap_file_changes(user_keymap_file_rx, user_keymap_watcher, cx);

        let user_agent = format!(
            "Zerminal/{} ({}; {})",
            AppVersion::global(cx),
            std::env::consts::OS,
            std::env::consts::ARCH
        );
        let proxy_url = ProxySettings::get_global(cx).proxy_url();
        let http = {
            let _guard = Tokio::handle(cx).enter();

            ReqwestClient::proxy_and_user_agent(proxy_url, &user_agent)
                .expect("could not start HTTP client")
        };
        cx.set_http_client(Arc::new(http));

        <dyn Fs>::set_global(fs.clone(), cx);

        let client = Client::production(cx);
        cx.set_http_client(client.http_client());
        let mut languages = LanguageRegistry::new(cx.background_executor().clone());
        languages.set_language_server_download_dir(paths::languages_dir().clone());
        let languages = Arc::new(languages);
        let (mut tx, rx) = watch::channel(None);
        cx.observe_global::<SettingsStore>(move |cx| {
            let settings = &ProjectSettings::get_global(cx).node;
            let options = NodeBinaryOptions {
                allow_path_lookup: !settings.ignore_system_version,
                allow_binary_download: true,
                use_paths: settings.path.as_ref().map(|node_path| {
                    let node_path = PathBuf::from(shellexpand::tilde(node_path).as_ref());
                    let npm_path = settings
                        .npm_path
                        .as_ref()
                        .map(|path| PathBuf::from(shellexpand::tilde(&path).as_ref()));
                    (
                        node_path.clone(),
                        npm_path.unwrap_or_else(|| {
                            let base_path = PathBuf::new();
                            node_path.parent().unwrap_or(&base_path).join("npm")
                        }),
                    )
                }),
            };
            tx.send(Some(options)).log_err();
        })
        .detach();

        let node_runtime = NodeRuntime::new(client.http_client(), Some(shell_env_loaded_rx), rx);

        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let workspace_store = cx.new(|cx| WorkspaceStore::new(client.clone(), cx));

        Client::set_global(client.clone(), cx);

        project::Project::init(&client, cx);
        client::init(&client, cx);

        let system_id = cx.foreground_executor().block_on(system_id).ok();
        let installation_id = cx.foreground_executor().block_on(installation_id).ok();
        let session = cx.foreground_executor().block_on(session);

        let telemetry = client.telemetry();
        telemetry.start(
            system_id.as_ref().map(|id| id.to_string()),
            installation_id.as_ref().map(|id| id.to_string()),
            session.id().to_owned(),
            cx,
        );

        let app_session = cx.new(|cx| AppSession::new(session, cx));

        let app_state = Arc::new(AppState {
            languages,
            client: client.clone(),
            user_store,
            fs: fs.clone(),
            build_window_options: zerminal::build_window_options,
            workspace_store,
            node_runtime,
            session: app_session,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);
        command_palette::init(cx);
        vim::init(cx);
        zerminal::init_terminal_view(cx);
        theme_selector::init(cx);

        load_embedded_fonts(cx);
        workspace::init(app_state.clone(), cx);

        let menus = zerminal::app_menus();
        cx.set_menus(menus);
        zerminal::initialize_workspace(app_state.clone(), cx);

        cx.activate(true);

        let working_directory = args.working_directory.map(PathBuf::from);
        cx.spawn({
            let app_state = app_state.clone();
            async move |cx| {
                zerminal::open_terminal_workspace(app_state, working_directory, cx).await
            }
        })
        .detach_and_log_err(cx);
    });
}

fn init_paths() -> HashMap<io::ErrorKind, Vec<&'static std::path::Path>> {
    [
        paths::config_dir(),
        paths::database_dir(),
        paths::logs_dir(),
        paths::temp_dir(),
    ]
    .into_iter()
    .fold(HashMap::default(), |mut errors, path| {
        if let Err(e) = std::fs::create_dir_all(path) {
            errors.entry(e.kind()).or_insert_with(Vec::new).push(path);
        }
        errors
    })
}

fn stdout_is_a_pty() -> bool {
    std::env::var(FORCE_CLI_MODE_ENV_VAR_NAME).ok().is_none() && io::stdout().is_terminal()
}

#[derive(Parser, Debug)]
#[command(name = "zerminal", disable_version_flag = true, max_term_width = 100)]
struct Args {
    /// The working directory to start the terminal in.
    #[arg(long, short = 'w')]
    working_directory: Option<String>,

    /// Sets a custom directory for all user data (e.g., database, logs).
    #[arg(long, value_name = "DIR")]
    user_data_dir: Option<String>,
}

#[derive(Clone, Debug)]
enum IdType {
    New(String),
    Existing(String),
}

impl ToString for IdType {
    fn to_string(&self) -> String {
        match self {
            IdType::New(id) | IdType::Existing(id) => id.clone(),
        }
    }
}

async fn system_id() -> Result<IdType> {
    let key_name = "system_id".to_string();

    if let Ok(Some(system_id)) = db::kvp::GLOBAL_KEY_VALUE_STORE.read_kvp(&key_name) {
        return Ok(IdType::Existing(system_id));
    }

    let system_id = Uuid::new_v4().to_string();

    db::kvp::GLOBAL_KEY_VALUE_STORE
        .write_kvp(key_name, system_id.clone())
        .await?;

    Ok(IdType::New(system_id))
}

async fn installation_id() -> Result<IdType> {
    let key_name = "installation_id".to_string();

    if let Ok(Some(installation_id)) = KEY_VALUE_STORE.read_kvp(&key_name) {
        return Ok(IdType::Existing(installation_id));
    }

    let installation_id = Uuid::new_v4().to_string();

    KEY_VALUE_STORE
        .write_kvp(key_name, installation_id.clone())
        .await?;

    Ok(IdType::New(installation_id))
}

fn load_embedded_fonts(cx: &App) {
    let asset_source = cx.asset_source();
    let font_paths = asset_source.list("fonts").unwrap();
    let embedded_fonts = Mutex::new(Vec::new());
    let executor = cx.background_executor();

    cx.foreground_executor().block_on(executor.scoped(|scope| {
        for font_path in &font_paths {
            if !font_path.ends_with(".ttf") {
                continue;
            }

            scope.spawn(async {
                let font_bytes = asset_source.load(font_path).unwrap().unwrap();
                embedded_fonts.lock().push(font_bytes);
            });
        }
    }));

    cx.text_system()
        .add_fonts(embedded_fonts.into_inner())
        .unwrap();
}
