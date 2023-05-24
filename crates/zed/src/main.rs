// Allow binary to be called Zed for a nice application menu when running executable direcly
#![allow(non_snake_case)]

use anyhow::{anyhow, Context, Result};
use assets::Assets;
use backtrace::Backtrace;
use cli::{
    ipc::{self, IpcSender},
    CliRequest, CliResponse, IpcHandshake, FORCE_CLI_MODE_ENV_VAR_NAME,
};
use client::{self, TelemetrySettings, UserStore, ZED_APP_VERSION, ZED_SECRET_CLIENT_TOKEN};
use db::kvp::KEY_VALUE_STORE;
use editor::{scroll::autoscroll::Autoscroll, Editor};
use futures::{
    channel::{mpsc, oneshot},
    FutureExt, SinkExt, StreamExt,
};
use gpui::{Action, App, AppContext, AssetSource, AsyncAppContext, Task, ViewContext};
use isahc::{config::Configurable, Request};
use language::{LanguageRegistry, Point};
use log::LevelFilter;
use node_runtime::NodeRuntime;
use parking_lot::Mutex;
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::{default_settings, handle_settings_file_changes, watch_config_file, SettingsStore};
use simplelog::ConfigBuilder;
use smol::process::Command;
use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs::OpenOptions,
    io::Write as _,
    os::unix::prelude::OsStrExt,
    panic,
    path::{Path, PathBuf},
    str,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Weak,
    },
    thread,
    time::Duration,
};
use sum_tree::Bias;
use terminal_view::{get_working_directory, TerminalSettings, TerminalView};
use util::{
    http::{self, HttpClient},
    paths::PathLikeWithPosition,
};
use welcome::{show_welcome_experience, FIRST_OPEN};

use fs::RealFs;
#[cfg(debug_assertions)]
use staff_mode::StaffMode;
use util::{channel::RELEASE_CHANNEL, paths, ResultExt, TryFutureExt};
use workspace::{
    item::ItemHandle, notifications::NotifyResultExt, AppState, OpenSettings, Workspace,
};
use zed::{
    self, build_window_options, handle_keymap_file_changes, initialize_workspace, languages, menus,
};

fn main() {
    let http = http::client();
    init_paths();
    init_logger();

    log::info!("========== starting zed ==========");
    let mut app = gpui::App::new(Assets).unwrap();

    let app_version = ZED_APP_VERSION
        .or_else(|| app.platform().app_version().ok())
        .map_or("dev".to_string(), |v| v.to_string());
    init_panic_hook(app_version);

    app.background();

    load_embedded_fonts(&app);

    let fs = Arc::new(RealFs);
    let user_settings_file_rx =
        watch_config_file(app.background(), fs.clone(), paths::SETTINGS.clone());
    let user_keymap_file_rx =
        watch_config_file(app.background(), fs.clone(), paths::KEYMAP.clone());

    let login_shell_env_loaded = if stdout_is_a_pty() {
        Task::ready(())
    } else {
        app.background().spawn(async {
            load_login_shell_environment().await.log_err();
        })
    };

    let (cli_connections_tx, mut cli_connections_rx) = mpsc::unbounded();
    let cli_connections_tx = Arc::new(cli_connections_tx);
    let (open_paths_tx, mut open_paths_rx) = mpsc::unbounded();
    let open_paths_tx = Arc::new(open_paths_tx);
    let urls_callback_triggered = Arc::new(AtomicBool::new(false));

    let callback_cli_connections_tx = Arc::clone(&cli_connections_tx);
    let callback_open_paths_tx = Arc::clone(&open_paths_tx);
    let callback_urls_callback_triggered = Arc::clone(&urls_callback_triggered);
    app.on_open_urls(move |urls, _| {
        callback_urls_callback_triggered.store(true, Ordering::Release);
        open_urls(urls, &callback_cli_connections_tx, &callback_open_paths_tx);
    })
    .on_reopen(move |cx| {
        if cx.has_global::<Weak<AppState>>() {
            if let Some(app_state) = cx.global::<Weak<AppState>>().upgrade() {
                workspace::open_new(&app_state, cx, |workspace, cx| {
                    Editor::new_file(workspace, &Default::default(), cx)
                })
                .detach();
            }
        }
    });

    app.run(move |cx| {
        cx.set_global(*RELEASE_CHANNEL);

        #[cfg(debug_assertions)]
        cx.set_global(StaffMode(true));

        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        handle_settings_file_changes(user_settings_file_rx, cx);
        handle_keymap_file_changes(user_keymap_file_rx, cx);

        let client = client::Client::new(http.clone(), cx);
        let mut languages = LanguageRegistry::new(login_shell_env_loaded);
        languages.set_executor(cx.background().clone());
        languages.set_language_server_download_dir(paths::LANGUAGES_DIR.clone());
        let languages = Arc::new(languages);
        let node_runtime = NodeRuntime::new(http.clone(), cx.background().to_owned());

        languages::init(languages.clone(), node_runtime.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http.clone(), cx));

        cx.set_global(client.clone());

        theme::init(Assets, cx);
        context_menu::init(cx);
        project::Project::init(&client, cx);
        client::init(&client, cx);
        command_palette::init(cx);
        language::init(cx);
        editor::init(cx);
        go_to_line::init(cx);
        file_finder::init(cx);
        outline::init(cx);
        project_symbols::init(cx);
        project_panel::init(cx);
        diagnostics::init(cx);
        search::init(cx);
        vim::init(cx);
        terminal_view::init(cx);
        theme_testbench::init(cx);
        copilot::init(http.clone(), node_runtime, cx);
        ai::init(cx);

        cx.spawn(|cx| watch_themes(fs.clone(), cx)).detach();

        languages.set_theme(theme::current(cx).clone());
        cx.observe_global::<SettingsStore, _>({
            let languages = languages.clone();
            move |cx| languages.set_theme(theme::current(cx).clone())
        })
        .detach();

        client.telemetry().start();
        client.telemetry().report_mixpanel_event(
            "start app",
            Default::default(),
            *settings::get::<TelemetrySettings>(cx),
        );

        let app_state = Arc::new(AppState {
            languages,
            client: client.clone(),
            user_store,
            fs,
            build_window_options,
            initialize_workspace,
            background_actions,
        });
        cx.set_global(Arc::downgrade(&app_state));
        auto_update::init(http.clone(), client::ZED_SERVER_URL.clone(), cx);

        workspace::init(app_state.clone(), cx);
        recent_projects::init(cx);

        journal::init(app_state.clone(), cx);
        language_selector::init(cx);
        theme_selector::init(cx);
        activity_indicator::init(cx);
        lsp_log::init(cx);
        call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        collab_ui::init(&app_state, cx);
        feedback::init(cx);
        welcome::init(cx);
        zed::init(&app_state, cx);

        cx.set_menus(menus::menus());

        if stdout_is_a_pty() {
            cx.platform().activate(true);
            let paths = collect_path_args();
            if paths.is_empty() {
                cx.spawn(|cx| async move { restore_or_create_workspace(&app_state, cx).await })
                    .detach()
            } else {
                workspace::open_paths(&paths, &app_state, None, cx).detach_and_log_err(cx);
            }
        } else {
            upload_previous_panics(http.clone(), cx);

            // TODO Development mode that forces the CLI mode usually runs Zed binary as is instead
            // of an *app, hence gets no specific callbacks run. Emulate them here, if needed.
            if std::env::var(FORCE_CLI_MODE_ENV_VAR_NAME).ok().is_some()
                && !urls_callback_triggered.load(Ordering::Acquire)
            {
                open_urls(collect_url_args(), &cli_connections_tx, &open_paths_tx)
            }

            if let Ok(Some(connection)) = cli_connections_rx.try_next() {
                cx.spawn(|cx| handle_cli_connection(connection, app_state.clone(), cx))
                    .detach();
            } else if let Ok(Some(paths)) = open_paths_rx.try_next() {
                cx.update(|cx| workspace::open_paths(&paths, &app_state, None, cx))
                    .detach();
            } else {
                cx.spawn({
                    let app_state = app_state.clone();
                    |cx| async move { restore_or_create_workspace(&app_state, cx).await }
                })
                .detach()
            }

            cx.spawn(|cx| {
                let app_state = app_state.clone();
                async move {
                    while let Some(connection) = cli_connections_rx.next().await {
                        handle_cli_connection(connection, app_state.clone(), cx.clone()).await;
                    }
                }
            })
            .detach();

            cx.spawn(|mut cx| {
                let app_state = app_state.clone();
                async move {
                    while let Some(paths) = open_paths_rx.next().await {
                        cx.update(|cx| workspace::open_paths(&paths, &app_state, None, cx))
                            .detach();
                    }
                }
            })
            .detach();
        }

        cx.spawn(|cx| async move {
            if stdout_is_a_pty() {
                if client::IMPERSONATE_LOGIN.is_some() {
                    client.authenticate_and_connect(false, &cx).await?;
                }
            } else if client.has_keychain_credentials(&cx) {
                client.authenticate_and_connect(true, &cx).await?;
            }
            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    });
}

fn open_urls(
    urls: Vec<String>,
    cli_connections_tx: &mpsc::UnboundedSender<(
        mpsc::Receiver<CliRequest>,
        IpcSender<CliResponse>,
    )>,
    open_paths_tx: &mpsc::UnboundedSender<Vec<PathBuf>>,
) {
    if let Some(server_name) = urls.first().and_then(|url| url.strip_prefix("zed-cli://")) {
        if let Some(cli_connection) = connect_to_cli(server_name).log_err() {
            cli_connections_tx
                .unbounded_send(cli_connection)
                .map_err(|_| anyhow!("no listener for cli connections"))
                .log_err();
        };
    } else {
        let paths: Vec<_> = urls
            .iter()
            .flat_map(|url| url.strip_prefix("file://"))
            .map(|url| {
                let decoded = urlencoding::decode_binary(url.as_bytes());
                PathBuf::from(OsStr::from_bytes(decoded.as_ref()))
            })
            .collect();
        open_paths_tx
            .unbounded_send(paths)
            .map_err(|_| anyhow!("no listener for open urls requests"))
            .log_err();
    }
}

async fn restore_or_create_workspace(app_state: &Arc<AppState>, mut cx: AsyncAppContext) {
    if let Some(location) = workspace::last_opened_workspace_paths().await {
        cx.update(|cx| workspace::open_paths(location.paths().as_ref(), app_state, None, cx))
            .await
            .log_err();
    } else if matches!(KEY_VALUE_STORE.read_kvp(FIRST_OPEN), Ok(None)) {
        cx.update(|cx| show_welcome_experience(app_state, cx));
    } else {
        cx.update(|cx| {
            workspace::open_new(app_state, cx, |workspace, cx| {
                Editor::new_file(workspace, &Default::default(), cx)
            })
            .detach();
        });
    }
}

fn init_paths() {
    std::fs::create_dir_all(&*util::paths::CONFIG_DIR).expect("could not create config path");
    std::fs::create_dir_all(&*util::paths::LANGUAGES_DIR).expect("could not create languages path");
    std::fs::create_dir_all(&*util::paths::DB_DIR).expect("could not create database path");
    std::fs::create_dir_all(&*util::paths::LOGS_DIR).expect("could not create logs path");
}

fn init_logger() {
    if stdout_is_a_pty() {
        env_logger::init();
    } else {
        let level = LevelFilter::Info;

        // Prevent log file from becoming too large.
        const KIB: u64 = 1024;
        const MIB: u64 = 1024 * KIB;
        const MAX_LOG_BYTES: u64 = MIB;
        if std::fs::metadata(&*paths::LOG).map_or(false, |metadata| metadata.len() > MAX_LOG_BYTES)
        {
            let _ = std::fs::rename(&*paths::LOG, &*paths::OLD_LOG);
        }

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&*paths::LOG)
            .expect("could not open logfile");

        let config = ConfigBuilder::new()
            .set_time_format_str("%Y-%m-%dT%T") //All timestamps are UTC
            .build();

        simplelog::WriteLogger::init(level, config, log_file).expect("could not initialize logger");
    }
}

#[derive(Serialize, Deserialize)]
struct LocationData {
    file: String,
    line: u32,
}

#[derive(Serialize, Deserialize)]
struct Panic {
    thread: String,
    payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_data: Option<LocationData>,
    backtrace: Vec<String>,
    // TODO
    // stripped_backtrace: String,
}

#[derive(Serialize)]
struct PanicRequest {
    panic: Panic,
    version: String,
    token: String,
}

fn init_panic_hook(app_version: String) {
    let is_pty = stdout_is_a_pty();
    panic::set_hook(Box::new(move |info| {
        let backtrace = Backtrace::new();

        let thread = thread::current();
        let thread = thread.name().unwrap_or("<unnamed>");

        let payload = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        let panic_data = Panic {
            thread: thread.into(),
            payload: payload.into(),
            location_data: info.location().map(|location| LocationData {
                file: location.file().into(),
                line: location.line(),
            }),
            backtrace: format!("{:?}", backtrace)
                .split("\n")
                .map(|line| line.to_string())
                .collect(),
            // modified_backtrace: None,
        };

        if let Some(panic_data_json) = serde_json::to_string_pretty(&panic_data).log_err() {
            if is_pty {
                eprintln!("{}", panic_data_json);
                return;
            }

            let timestamp = chrono::Utc::now().format("%Y_%m_%d %H_%M_%S").to_string();
            let panic_file_path =
                paths::LOGS_DIR.join(format!("zed-{}-{}.panic", app_version, timestamp));
            let panic_file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&panic_file_path)
                .log_err();
            if let Some(mut panic_file) = panic_file {
                write!(&mut panic_file, "{}", panic_data_json).log_err();
                panic_file.flush().log_err();
            }
        }
    }));
}

fn upload_previous_panics(http: Arc<dyn HttpClient>, cx: &mut AppContext) {
    let telemetry_settings = *settings::get::<TelemetrySettings>(cx);

    cx.background()
        .spawn({
            async move {
                let panic_report_url = format!("{}/api/panic", &*client::ZED_SERVER_URL);
                let mut children = smol::fs::read_dir(&*paths::LOGS_DIR).await?;
                while let Some(child) = children.next().await {
                    let child = child?;
                    let child_path = child.path();

                    if child_path.extension() != Some(OsStr::new("panic")) {
                        continue;
                    }
                    let filename = if let Some(filename) = child_path.file_name() {
                        filename.to_string_lossy()
                    } else {
                        continue;
                    };

                    let mut components = filename.split('-');
                    if components.next() != Some("zed") {
                        continue;
                    }
                    let version = if let Some(version) = components.next() {
                        version
                    } else {
                        continue;
                    };

                    if telemetry_settings.diagnostics {
                        let panic_data_text = smol::fs::read_to_string(&child_path)
                            .await
                            .context("error reading panic file")?;

                        let body = serde_json::to_string(&PanicRequest {
                            panic: serde_json::from_str(&panic_data_text)?,
                            version: version.to_string(),
                            token: ZED_SECRET_CLIENT_TOKEN.into(),
                        })
                        .unwrap();

                        let request = Request::post(&panic_report_url)
                            .redirect_policy(isahc::config::RedirectPolicy::Follow)
                            .header("Content-Type", "application/json")
                            .body(body.into())?;
                        let response = http.send(request).await.context("error sending panic")?;
                        if !response.status().is_success() {
                            log::error!("Error uploading panic to server: {}", response.status());
                        }
                    }

                    // We've done what we can, delete the file
                    std::fs::remove_file(child_path)
                        .context("error removing panic")
                        .log_err();
                }
                Ok::<_, anyhow::Error>(())
            }
            .log_err()
        })
        .detach();
}

async fn load_login_shell_environment() -> Result<()> {
    let marker = "ZED_LOGIN_SHELL_START";
    let shell = env::var("SHELL").context(
        "SHELL environment variable is not assigned so we can't source login environment variables",
    )?;
    let output = Command::new(&shell)
        .args(["-lic", &format!("echo {marker} && /usr/bin/env -0")])
        .output()
        .await
        .context("failed to spawn login shell to source login environment variables")?;
    if !output.status.success() {
        Err(anyhow!("login shell exited with error"))?;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if let Some(env_output_start) = stdout.find(marker) {
        let env_output = &stdout[env_output_start + marker.len()..];
        for line in env_output.split_terminator('\0') {
            if let Some(separator_index) = line.find('=') {
                let key = &line[..separator_index];
                let value = &line[separator_index + 1..];
                env::set_var(key, value);
            }
        }
        log::info!(
            "set environment variables from shell:{}, path:{}",
            shell,
            env::var("PATH").unwrap_or_default(),
        );
    }

    Ok(())
}

fn stdout_is_a_pty() -> bool {
    std::env::var(FORCE_CLI_MODE_ENV_VAR_NAME).ok().is_none()
        && unsafe { libc::isatty(libc::STDOUT_FILENO as i32) != 0 }
}

fn collect_path_args() -> Vec<PathBuf> {
    env::args()
        .skip(1)
        .filter_map(|arg| match std::fs::canonicalize(arg) {
            Ok(path) => Some(path),
            Err(error) => {
                log::error!("error parsing path argument: {}", error);
                None
            }
        })
        .collect()
}

fn collect_url_args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn load_embedded_fonts(app: &App) {
    let font_paths = Assets.list("fonts");
    let embedded_fonts = Mutex::new(Vec::new());
    smol::block_on(app.background().scoped(|scope| {
        for font_path in &font_paths {
            scope.spawn(async {
                let font_path = &*font_path;
                let font_bytes = Assets.load(font_path).unwrap().to_vec();
                embedded_fonts.lock().push(Arc::from(font_bytes));
            });
        }
    }));
    app.platform()
        .fonts()
        .add_fonts(&embedded_fonts.into_inner())
        .unwrap();
}

#[cfg(debug_assertions)]
async fn watch_themes(fs: Arc<dyn Fs>, mut cx: AsyncAppContext) -> Option<()> {
    let mut events = fs
        .watch("styles/src".as_ref(), Duration::from_millis(100))
        .await;
    while (events.next().await).is_some() {
        let output = Command::new("npm")
            .current_dir("styles")
            .args(["run", "build"])
            .output()
            .await
            .log_err()?;
        if output.status.success() {
            cx.update(|cx| theme_selector::reload(cx))
        } else {
            eprintln!(
                "build script failed {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    Some(())
}

#[cfg(not(debug_assertions))]
async fn watch_themes(_fs: Arc<dyn Fs>, _cx: AsyncAppContext) -> Option<()> {
    None
}

fn connect_to_cli(
    server_name: &str,
) -> Result<(mpsc::Receiver<CliRequest>, IpcSender<CliResponse>)> {
    let handshake_tx = cli::ipc::IpcSender::<IpcHandshake>::connect(server_name.to_string())
        .context("error connecting to cli")?;
    let (request_tx, request_rx) = ipc::channel::<CliRequest>()?;
    let (response_tx, response_rx) = ipc::channel::<CliResponse>()?;

    handshake_tx
        .send(IpcHandshake {
            requests: request_tx,
            responses: response_rx,
        })
        .context("error sending ipc handshake")?;

    let (mut async_request_tx, async_request_rx) =
        futures::channel::mpsc::channel::<CliRequest>(16);
    thread::spawn(move || {
        while let Ok(cli_request) = request_rx.recv() {
            if smol::block_on(async_request_tx.send(cli_request)).is_err() {
                break;
            }
        }
        Ok::<_, anyhow::Error>(())
    });

    Ok((async_request_rx, response_tx))
}

async fn handle_cli_connection(
    (mut requests, responses): (mpsc::Receiver<CliRequest>, IpcSender<CliResponse>),
    app_state: Arc<AppState>,
    mut cx: AsyncAppContext,
) {
    if let Some(request) = requests.next().await {
        match request {
            CliRequest::Open { paths, wait } => {
                let mut caret_positions = HashMap::new();

                let paths = if paths.is_empty() {
                    workspace::last_opened_workspace_paths()
                        .await
                        .map(|location| location.paths().to_vec())
                        .unwrap_or_default()
                } else {
                    paths
                        .into_iter()
                        .filter_map(|path_with_position_string| {
                            let path_with_position = PathLikeWithPosition::parse_str(
                                &path_with_position_string,
                                |path_str| {
                                    Ok::<_, std::convert::Infallible>(
                                        Path::new(path_str).to_path_buf(),
                                    )
                                },
                            )
                            .expect("Infallible");
                            let path = path_with_position.path_like;
                            if let Some(row) = path_with_position.row {
                                if path.is_file() {
                                    let row = row.saturating_sub(1);
                                    let col =
                                        path_with_position.column.unwrap_or(0).saturating_sub(1);
                                    caret_positions.insert(path.clone(), Point::new(row, col));
                                }
                            }
                            Some(path)
                        })
                        .collect()
                };

                let mut errored = false;
                match cx
                    .update(|cx| workspace::open_paths(&paths, &app_state, None, cx))
                    .await
                {
                    Ok((workspace, items)) => {
                        let mut item_release_futures = Vec::new();

                        for (item, path) in items.into_iter().zip(&paths) {
                            match item {
                                Some(Ok(item)) => {
                                    if let Some(point) = caret_positions.remove(path) {
                                        if let Some(active_editor) = item.downcast::<Editor>() {
                                            active_editor
                                                .downgrade()
                                                .update(&mut cx, |editor, cx| {
                                                    let snapshot =
                                                        editor.snapshot(cx).display_snapshot;
                                                    let point = snapshot
                                                        .buffer_snapshot
                                                        .clip_point(point, Bias::Left);
                                                    editor.change_selections(
                                                        Some(Autoscroll::center()),
                                                        cx,
                                                        |s| s.select_ranges([point..point]),
                                                    );
                                                })
                                                .log_err();
                                        }
                                    }

                                    let released = oneshot::channel();
                                    cx.update(|cx| {
                                        item.on_release(
                                            cx,
                                            Box::new(move |_| {
                                                let _ = released.0.send(());
                                            }),
                                        )
                                        .detach();
                                    });
                                    item_release_futures.push(released.1);
                                }
                                Some(Err(err)) => {
                                    responses
                                        .send(CliResponse::Stderr {
                                            message: format!("error opening {:?}: {}", path, err),
                                        })
                                        .log_err();
                                    errored = true;
                                }
                                None => {}
                            }
                        }

                        if wait {
                            let background = cx.background();
                            let wait = async move {
                                if paths.is_empty() {
                                    let (done_tx, done_rx) = oneshot::channel();
                                    if let Some(workspace) = workspace.upgrade(&cx) {
                                        let _subscription = cx.update(|cx| {
                                            cx.observe_release(&workspace, move |_, _| {
                                                let _ = done_tx.send(());
                                            })
                                        });
                                        drop(workspace);
                                        let _ = done_rx.await;
                                    }
                                } else {
                                    let _ =
                                        futures::future::try_join_all(item_release_futures).await;
                                };
                            }
                            .fuse();
                            futures::pin_mut!(wait);

                            loop {
                                // Repeatedly check if CLI is still open to avoid wasting resources
                                // waiting for files or workspaces to close.
                                let mut timer = background.timer(Duration::from_secs(1)).fuse();
                                futures::select_biased! {
                                    _ = wait => break,
                                    _ = timer => {
                                        if responses.send(CliResponse::Ping).is_err() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        errored = true;
                        responses
                            .send(CliResponse::Stderr {
                                message: format!("error opening {:?}: {}", paths, error),
                            })
                            .log_err();
                    }
                }

                responses
                    .send(CliResponse::Exit {
                        status: i32::from(errored),
                    })
                    .log_err();
            }
        }
    }
}

pub fn dock_default_item_factory(
    workspace: &mut Workspace,
    cx: &mut ViewContext<Workspace>,
) -> Option<Box<dyn ItemHandle>> {
    let strategy = settings::get::<TerminalSettings>(cx)
        .working_directory
        .clone();
    let working_directory = get_working_directory(workspace, cx, strategy);

    let window_id = cx.window_id();
    let terminal = workspace
        .project()
        .update(cx, |project, cx| {
            project.create_terminal(working_directory, window_id, cx)
        })
        .notify_err(workspace, cx)?;

    let terminal_view = cx.add_view(|cx| TerminalView::new(terminal, workspace.database_id(), cx));

    Some(Box::new(terminal_view))
}

pub fn background_actions() -> &'static [(&'static str, &'static dyn Action)] {
    &[
        ("Go to file", &file_finder::Toggle),
        ("Open command palette", &command_palette::Toggle),
        ("Open recent projects", &recent_projects::OpenRecent),
        ("Change your settings", &OpenSettings),
    ]
}
