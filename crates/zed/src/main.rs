// Allow binary to be called Zed for a nice application menu when running executable directly
#![allow(non_snake_case)]

use anyhow::{anyhow, Context as _, Result};
use backtrace::Backtrace;
use chrono::Utc;
use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
use client::{Client, UserStore};
use collab_ui::channel_view::ChannelView;
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use env_logger::Builder;
use fs::RealFs;
#[cfg(target_os = "macos")]
use fsevent::StreamFlags;
use futures::StreamExt;
use gpui::{App, AppContext, AsyncAppContext, Context, SemanticVersion, Task};
use isahc::{prelude::Configurable, Request};
use language::LanguageRegistry;
use log::LevelFilter;

use assets::Assets;
use mimalloc::MiMalloc;
use node_runtime::RealNodeRuntime;
use parking_lot::Mutex;
use release_channel::{parse_zed_link, AppCommitSha, ReleaseChannel, RELEASE_CHANNEL};
use serde::{Deserialize, Serialize};
use settings::{
    default_settings, handle_settings_file_changes, watch_config_file, Settings, SettingsStore,
};
use simplelog::ConfigBuilder;
use smol::process::Command;
use std::{
    env,
    ffi::OsStr,
    fs::OpenOptions,
    io::{IsTerminal, Write},
    panic,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    thread,
};
use theme::{ActiveTheme, SystemAppearance, ThemeRegistry, ThemeSettings};
use util::{
    async_maybe,
    http::{HttpClient, HttpClientWithUrl},
    paths::{self, CRASHES_DIR, CRASHES_RETIRED_DIR},
    ResultExt,
};
use uuid::Uuid;
use welcome::{show_welcome_view, BaseKeymap, FIRST_OPEN};
use workspace::{AppState, WorkspaceStore};
use zed::{
    app_menus, build_window_options, ensure_only_instance, handle_cli_connection,
    handle_keymap_file_changes, initialize_workspace, IsOnlyInstance, OpenListener, OpenRequest,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    menu::init();
    zed_actions::init();

    init_paths();
    init_logger();

    if ensure_only_instance() != IsOnlyInstance::Yes {
        return;
    }

    log::info!("========== starting zed ==========");
    let app = App::new().with_assets(Assets);

    let (installation_id, existing_installation_id_found) = app
        .background_executor()
        .block(installation_id())
        .ok()
        .unzip();
    let session_id = Uuid::new_v4().to_string();
    init_panic_hook(&app, installation_id.clone(), session_id.clone());

    let fs = Arc::new(RealFs);
    let user_settings_file_rx = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::SETTINGS.clone(),
    );
    let user_keymap_file_rx = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::KEYMAP.clone(),
    );

    let login_shell_env_loaded = if stdout_is_a_pty() {
        Task::ready(())
    } else {
        app.background_executor().spawn(async {
            load_login_shell_environment().await.log_err();
        })
    };

    let (listener, mut open_rx) = OpenListener::new();
    let listener = Arc::new(listener);
    let open_listener = listener.clone();
    app.on_open_urls(move |urls, _| open_listener.open_urls(&urls));
    app.on_reopen(move |cx| {
        if let Some(app_state) = AppState::try_global(cx)
            .map(|app_state| app_state.upgrade())
            .flatten()
        {
            workspace::open_new(&app_state, cx, |workspace, cx| {
                Editor::new_file(workspace, &Default::default(), cx)
            })
            .detach();
        }
    });

    app.run(move |cx| {
        release_channel::init(env!("CARGO_PKG_VERSION"), cx);
        if let Some(build_sha) = option_env!("ZED_COMMIT_SHA") {
            AppCommitSha::set_global(AppCommitSha(build_sha.into()), cx);
        }

        SystemAppearance::init(cx);
        OpenListener::set_global(listener.clone(), cx);

        load_embedded_fonts(cx);

        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        handle_settings_file_changes(user_settings_file_rx, cx);
        handle_keymap_file_changes(user_keymap_file_rx, cx);
        client::init_settings(cx);

        let clock = Arc::new(clock::RealSystemClock);
        let http = Arc::new(HttpClientWithUrl::new(
            &client::ClientSettings::get_global(cx).server_url,
        ));

        let client = client::Client::new(clock, http.clone(), cx);
        let mut languages = LanguageRegistry::new(login_shell_env_loaded);
        let copilot_language_server_id = languages.next_language_server_id();
        languages.set_executor(cx.background_executor().clone());
        languages.set_language_server_download_dir(paths::LANGUAGES_DIR.clone());
        let languages = Arc::new(languages);
        let node_runtime = RealNodeRuntime::new(http.clone());

        language::init(cx);
        languages::init(languages.clone(), node_runtime.clone(), cx);
        let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
        let workspace_store = cx.new_model(|cx| WorkspaceStore::new(client.clone(), cx));

        Client::set_global(client.clone(), cx);

        zed::init(cx);
        theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);
        project::Project::init(&client, cx);
        client::init(&client, cx);
        command_palette::init(cx);
        language::init(cx);
        editor::init(cx);
        diagnostics::init(cx);
        copilot::init(
            copilot_language_server_id,
            http.clone(),
            node_runtime.clone(),
            cx,
        );
        assistant::init(cx);

        extension::init(
            fs.clone(),
            http.clone(),
            languages.clone(),
            ThemeRegistry::global(cx),
            cx,
        );

        load_user_themes_in_background(fs.clone(), cx);
        #[cfg(target_os = "macos")]
        watch_themes(fs.clone(), cx);

        cx.spawn(|_| watch_languages(fs.clone(), languages.clone()))
            .detach();
        watch_file_types(fs.clone(), cx);

        languages.set_theme(cx.theme().clone());
        cx.observe_global::<SettingsStore>({
            let languages = languages.clone();
            let http = http.clone();
            let client = client.clone();

            move |cx| {
                languages.set_theme(cx.theme().clone());
                let new_host = &client::ClientSettings::get_global(cx).server_url;
                if &http.base_url() != new_host {
                    http.set_base_url(new_host);
                    if client.status().borrow().is_connected() {
                        client.reconnect(&cx.to_async());
                    }
                }
            }
        })
        .detach();

        let telemetry = client.telemetry();
        telemetry.start(installation_id, session_id, cx);
        telemetry.report_setting_event("theme", cx.theme().name.to_string());
        telemetry.report_setting_event("keymap", BaseKeymap::get_global(cx).to_string());
        telemetry.report_app_event(
            match existing_installation_id_found {
                Some(false) => "first open",
                _ => "open",
            }
            .to_string(),
        );
        telemetry.flush_events();

        let app_state = Arc::new(AppState {
            languages: languages.clone(),
            client: client.clone(),
            user_store: user_store.clone(),
            fs: fs.clone(),
            build_window_options,
            workspace_store,
            node_runtime,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        audio::init(Assets, cx);
        auto_update::init(http.clone(), cx);

        workspace::init(app_state.clone(), cx);
        recent_projects::init(cx);

        go_to_line::init(cx);
        file_finder::init(cx);
        outline::init(cx);
        project_symbols::init(cx);
        project_panel::init(Assets, cx);
        tasks_ui::init(cx);
        channel::init(&client, user_store.clone(), cx);
        search::init(cx);
        semantic_index::init(fs.clone(), http.clone(), languages.clone(), cx);
        vim::init(cx);
        terminal_view::init(cx);

        journal::init(app_state.clone(), cx);
        language_selector::init(cx);
        theme_selector::init(cx);
        language_tools::init(cx);
        call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        notifications::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        collab_ui::init(&app_state, cx);
        feedback::init(cx);
        markdown_preview::init(cx);
        welcome::init(cx);
        extensions_ui::init(cx);

        cx.set_menus(app_menus());
        initialize_workspace(app_state.clone(), cx);

        if stdout_is_a_pty() {
            //todo!(linux): unblock this
            #[cfg(not(target_os = "linux"))]
            upload_panics_and_crashes(http.clone(), cx);
            cx.activate(true);
            let urls = collect_url_args();
            if !urls.is_empty() {
                listener.open_urls(&urls)
            }
        } else {
            upload_panics_and_crashes(http.clone(), cx);
            // TODO Development mode that forces the CLI mode usually runs Zed binary as is instead
            // of an *app, hence gets no specific callbacks run. Emulate them here, if needed.
            if std::env::var(FORCE_CLI_MODE_ENV_VAR_NAME).ok().is_some()
                && !listener.triggered.load(Ordering::Acquire)
            {
                listener.open_urls(&collect_url_args())
            }
        }

        let mut triggered_authentication = false;

        fn open_paths_and_log_errs(
            paths: &[PathBuf],
            app_state: &Arc<AppState>,
            cx: &mut AppContext,
        ) {
            let task = workspace::open_paths(&paths, &app_state, None, cx);
            cx.spawn(|_| async move {
                if let Some((_window, results)) = task.await.log_err() {
                    for result in results {
                        if let Some(Err(e)) = result {
                            log::error!("Error opening path: {}", e);
                        }
                    }
                }
            })
            .detach();
        }

        match open_rx.try_next() {
            Ok(Some(OpenRequest::Paths { paths })) => {
                open_paths_and_log_errs(&paths, &app_state, cx)
            }
            Ok(Some(OpenRequest::CliConnection { connection })) => {
                let app_state = app_state.clone();
                cx.spawn(move |cx| handle_cli_connection(connection, app_state, cx))
                    .detach();
            }
            Ok(Some(OpenRequest::JoinChannel { channel_id })) => {
                triggered_authentication = true;
                let app_state = app_state.clone();
                let client = client.clone();
                cx.spawn(|cx| async move {
                    // ignore errors here, we'll show a generic "not signed in"
                    let _ = authenticate(client, &cx).await;
                    cx.update(|cx| {
                        workspace::join_channel(client::ChannelId(channel_id), app_state, None, cx)
                    })?
                    .await?;
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
            Ok(Some(OpenRequest::OpenChannelNotes {
                channel_id,
                heading,
            })) => {
                triggered_authentication = true;
                let app_state = app_state.clone();
                let client = client.clone();
                cx.spawn(|mut cx| async move {
                    // ignore errors here, we'll show a generic "not signed in"
                    let _ = authenticate(client, &cx).await;
                    let workspace_window =
                        workspace::get_any_active_workspace(app_state, cx.clone()).await?;
                    let workspace = workspace_window.root_view(&cx)?;
                    cx.update_window(workspace_window.into(), |_, cx| {
                        ChannelView::open(client::ChannelId(channel_id), heading, workspace, cx)
                    })?
                    .await?;
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
            Ok(None) | Err(_) => cx
                .spawn({
                    let app_state = app_state.clone();
                    |cx| async move { restore_or_create_workspace(&app_state, cx).await }
                })
                .detach(),
        }

        let app_state = app_state.clone();
        cx.spawn(move |cx| async move {
            while let Some(request) = open_rx.next().await {
                match request {
                    OpenRequest::Paths { paths } => {
                        cx.update(|cx| open_paths_and_log_errs(&paths, &app_state, cx))
                            .ok();
                    }
                    OpenRequest::CliConnection { connection } => {
                        let app_state = app_state.clone();
                        cx.spawn(move |cx| {
                            handle_cli_connection(connection, app_state.clone(), cx)
                        })
                        .detach();
                    }
                    OpenRequest::JoinChannel { channel_id } => {
                        let app_state = app_state.clone();
                        cx.update(|mut cx| {
                            cx.spawn(|cx| async move {
                                cx.update(|cx| {
                                    workspace::join_channel(
                                        client::ChannelId(channel_id),
                                        app_state,
                                        None,
                                        cx,
                                    )
                                })?
                                .await?;
                                anyhow::Ok(())
                            })
                            .detach_and_log_err(&mut cx);
                        })
                        .log_err();
                    }
                    OpenRequest::OpenChannelNotes {
                        channel_id,
                        heading,
                    } => {
                        let app_state = app_state.clone();
                        let open_notes_task = cx.spawn(|mut cx| async move {
                            let workspace_window =
                                workspace::get_any_active_workspace(app_state, cx.clone()).await?;
                            let workspace = workspace_window.root_view(&cx)?;
                            cx.update_window(workspace_window.into(), |_, cx| {
                                ChannelView::open(
                                    client::ChannelId(channel_id),
                                    heading,
                                    workspace,
                                    cx,
                                )
                            })?
                            .await?;
                            anyhow::Ok(())
                        });
                        cx.update(|cx| open_notes_task.detach_and_log_err(cx))
                            .log_err();
                    }
                }
            }
        })
        .detach();

        if !triggered_authentication {
            cx.spawn(|cx| async move { authenticate(client, &cx).await })
                .detach_and_log_err(cx);
        }
    });
}

async fn authenticate(client: Arc<Client>, cx: &AsyncAppContext) -> Result<()> {
    if stdout_is_a_pty() {
        if client::IMPERSONATE_LOGIN.is_some() {
            client.authenticate_and_connect(false, &cx).await?;
        }
    } else if client.has_keychain_credentials(&cx).await {
        client.authenticate_and_connect(true, &cx).await?;
    }
    Ok::<_, anyhow::Error>(())
}

async fn installation_id() -> Result<(String, bool)> {
    let legacy_key_name = "device_id".to_string();
    let key_name = "installation_id".to_string();

    // Migrate legacy key to new key
    if let Ok(Some(installation_id)) = KEY_VALUE_STORE.read_kvp(&legacy_key_name) {
        KEY_VALUE_STORE
            .write_kvp(key_name, installation_id.clone())
            .await?;
        KEY_VALUE_STORE.delete_kvp(legacy_key_name).await?;
        return Ok((installation_id, true));
    }

    if let Ok(Some(installation_id)) = KEY_VALUE_STORE.read_kvp(&key_name) {
        return Ok((installation_id, true));
    }

    let installation_id = Uuid::new_v4().to_string();

    KEY_VALUE_STORE
        .write_kvp(key_name, installation_id.clone())
        .await?;

    Ok((installation_id, false))
}

async fn restore_or_create_workspace(app_state: &Arc<AppState>, cx: AsyncAppContext) {
    async_maybe!({
        if let Some(location) = workspace::last_opened_workspace_paths().await {
            cx.update(|cx| workspace::open_paths(location.paths().as_ref(), app_state, None, cx))?
                .await
                .log_err();
        } else if matches!(KEY_VALUE_STORE.read_kvp(FIRST_OPEN), Ok(None)) {
            cx.update(|cx| show_welcome_view(app_state, cx)).log_err();
        } else {
            cx.update(|cx| {
                workspace::open_new(app_state, cx, |workspace, cx| {
                    Editor::new_file(workspace, &Default::default(), cx)
                })
                .detach();
            })?;
        }
        anyhow::Ok(())
    })
    .await
    .log_err();
}

fn init_paths() {
    std::fs::create_dir_all(&*util::paths::CONFIG_DIR).expect("could not create config path");
    std::fs::create_dir_all(&*util::paths::LANGUAGES_DIR).expect("could not create languages path");
    std::fs::create_dir_all(&*util::paths::DB_DIR).expect("could not create database path");
    std::fs::create_dir_all(&*util::paths::LOGS_DIR).expect("could not create logs path");
    #[cfg(target_os = "linux")]
    std::fs::create_dir_all(&*util::paths::TEMP_DIR).expect("could not create tmp path");
}

fn init_logger() {
    if stdout_is_a_pty() {
        init_stdout_logger();
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

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&*paths::LOG)
        {
            Ok(log_file) => {
                let config = ConfigBuilder::new()
                    .set_time_format_str("%Y-%m-%dT%T%:z")
                    .set_time_to_local(true)
                    .build();

                simplelog::WriteLogger::init(level, config, log_file)
                    .expect("could not initialize logger");
            }
            Err(err) => {
                init_stdout_logger();
                log::error!(
                    "could not open log file, defaulting to stdout logging: {}",
                    err
                );
            }
        }
    }
}

fn init_stdout_logger() {
    Builder::new()
        .parse_default_env()
        .format(|buf, record| {
            use env_logger::fmt::Color;

            let subtle = buf
                .style()
                .set_color(Color::Black)
                .set_intense(true)
                .clone();
            write!(buf, "{}", subtle.value("["))?;
            write!(
                buf,
                "{} ",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z")
            )?;
            write!(buf, "{:<5}", buf.default_styled_level(record.level()))?;
            if let Some(path) = record.module_path() {
                write!(buf, " {}", path)?;
            }
            write!(buf, "{}", subtle.value("]"))?;
            writeln!(buf, " {}", record.args())
        })
        .init();
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
    app_version: String,
    release_channel: String,
    os_name: String,
    os_version: Option<String>,
    architecture: String,
    panicked_on: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    installation_id: Option<String>,
    session_id: String,
}

#[derive(Serialize)]
struct PanicRequest {
    panic: Panic,
}

static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);

fn init_panic_hook(app: &App, installation_id: Option<String>, session_id: String) {
    let is_pty = stdout_is_a_pty();
    let app_metadata = app.metadata();

    panic::set_hook(Box::new(move |info| {
        let prior_panic_count = PANIC_COUNT.fetch_add(1, Ordering::SeqCst);
        if prior_panic_count > 0 {
            // Give the panic-ing thread time to write the panic file
            loop {
                std::thread::yield_now();
            }
        }

        let thread = thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");

        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().map(|s| s.clone()))
            .unwrap_or_else(|| "Box<Any>".to_string());

        if *release_channel::RELEASE_CHANNEL == ReleaseChannel::Dev {
            let location = info.location().unwrap();
            let backtrace = Backtrace::new();
            eprintln!(
                "Thread {:?} panicked with {:?} at {}:{}:{}\n{:?}",
                thread_name,
                payload,
                location.file(),
                location.line(),
                location.column(),
                backtrace,
            );
            std::process::exit(-1);
        }

        let app_version = if let Some(version) = app_metadata.app_version {
            version.to_string()
        } else {
            option_env!("CARGO_PKG_VERSION")
                .unwrap_or("dev")
                .to_string()
        };

        let backtrace = Backtrace::new();
        let mut backtrace = backtrace
            .frames()
            .iter()
            .flat_map(|frame| {
                frame
                    .symbols()
                    .iter()
                    .filter_map(|frame| Some(format!("{:#}", frame.name()?)))
            })
            .collect::<Vec<_>>();

        // Strip out leading stack frames for rust panic-handling.
        if let Some(ix) = backtrace
            .iter()
            .position(|name| name == "rust_begin_unwind")
        {
            backtrace.drain(0..=ix);
        }

        let panic_data = Panic {
            thread: thread_name.into(),
            payload: payload.into(),
            location_data: info.location().map(|location| LocationData {
                file: location.file().into(),
                line: location.line(),
            }),
            app_version: app_version.to_string(),
            release_channel: RELEASE_CHANNEL.display_name().into(),
            os_name: app_metadata.os_name.into(),
            os_version: app_metadata
                .os_version
                .as_ref()
                .map(SemanticVersion::to_string),
            architecture: env::consts::ARCH.into(),
            panicked_on: Utc::now().timestamp_millis(),
            backtrace,
            installation_id: installation_id.clone(),
            session_id: session_id.clone(),
        };

        if let Some(panic_data_json) = serde_json::to_string_pretty(&panic_data).log_err() {
            log::error!("{}", panic_data_json);
        }

        if !is_pty {
            if let Some(panic_data_json) = serde_json::to_string(&panic_data).log_err() {
                let timestamp = chrono::Utc::now().format("%Y_%m_%d %H_%M_%S").to_string();
                let panic_file_path = paths::LOGS_DIR.join(format!("zed-{}.panic", timestamp));
                let panic_file = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&panic_file_path)
                    .log_err();
                if let Some(mut panic_file) = panic_file {
                    writeln!(&mut panic_file, "{}", panic_data_json).log_err();
                    panic_file.flush().log_err();
                }
            }
        }

        std::process::abort();
    }));
}

fn upload_panics_and_crashes(http: Arc<HttpClientWithUrl>, cx: &mut AppContext) {
    let telemetry_settings = *client::TelemetrySettings::get_global(cx);
    cx.background_executor()
        .spawn(async move {
            let most_recent_panic = upload_previous_panics(http.clone(), telemetry_settings)
                .await
                .log_err()
                .flatten();
            upload_previous_crashes(http, most_recent_panic, telemetry_settings)
                .await
                .log_err()
        })
        .detach()
}

/// Uploads panics via `zed.dev`.
async fn upload_previous_panics(
    http: Arc<HttpClientWithUrl>,
    telemetry_settings: client::TelemetrySettings,
) -> Result<Option<(i64, String)>> {
    let panic_report_url = http.build_url("/api/panic");
    let mut children = smol::fs::read_dir(&*paths::LOGS_DIR).await?;

    let mut most_recent_panic = None;

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

        if !filename.starts_with("zed") {
            continue;
        }

        if telemetry_settings.diagnostics {
            let panic_file_content = smol::fs::read_to_string(&child_path)
                .await
                .context("error reading panic file")?;

            let panic: Option<Panic> = serde_json::from_str(&panic_file_content)
                .ok()
                .or_else(|| {
                    panic_file_content
                        .lines()
                        .next()
                        .and_then(|line| serde_json::from_str(line).ok())
                })
                .unwrap_or_else(|| {
                    log::error!("failed to deserialize panic file {:?}", panic_file_content);
                    None
                });

            if let Some(panic) = panic {
                most_recent_panic = Some((panic.panicked_on, panic.payload.clone()));

                let body = serde_json::to_string(&PanicRequest { panic }).unwrap();

                let request = Request::post(&panic_report_url)
                    .redirect_policy(isahc::config::RedirectPolicy::Follow)
                    .header("Content-Type", "application/json")
                    .body(body.into())?;
                let response = http.send(request).await.context("error sending panic")?;
                if !response.status().is_success() {
                    log::error!("Error uploading panic to server: {}", response.status());
                }
            }
        }

        // We've done what we can, delete the file
        std::fs::remove_file(child_path)
            .context("error removing panic")
            .log_err();
    }
    Ok::<_, anyhow::Error>(most_recent_panic)
}

static LAST_CRASH_UPLOADED: &'static str = "LAST_CRASH_UPLOADED";

/// upload crashes from apple's diagnostic reports to our server.
/// (only if telemetry is enabled)
async fn upload_previous_crashes(
    http: Arc<HttpClientWithUrl>,
    most_recent_panic: Option<(i64, String)>,
    telemetry_settings: client::TelemetrySettings,
) -> Result<()> {
    if !telemetry_settings.diagnostics {
        return Ok(());
    }
    let last_uploaded = KEY_VALUE_STORE
        .read_kvp(LAST_CRASH_UPLOADED)?
        .unwrap_or("zed-2024-01-17-221900.ips".to_string()); // don't upload old crash reports from before we had this.
    let mut uploaded = last_uploaded.clone();

    let crash_report_url = http.build_url("/api/crash");

    for dir in [&*CRASHES_DIR, &*CRASHES_RETIRED_DIR] {
        let mut children = smol::fs::read_dir(&dir).await?;
        while let Some(child) = children.next().await {
            let child = child?;
            let Some(filename) = child
                .path()
                .file_name()
                .map(|f| f.to_string_lossy().to_lowercase())
            else {
                continue;
            };

            if !filename.starts_with("zed-") || !filename.ends_with(".ips") {
                continue;
            }

            if filename <= last_uploaded {
                continue;
            }

            let body = smol::fs::read_to_string(&child.path())
                .await
                .context("error reading crash file")?;

            let mut request = Request::post(&crash_report_url)
                .redirect_policy(isahc::config::RedirectPolicy::Follow)
                .header("Content-Type", "text/plain");

            if let Some((panicked_on, payload)) = most_recent_panic.as_ref() {
                request = request
                    .header("x-zed-panicked-on", format!("{}", panicked_on))
                    .header("x-zed-panic", payload)
            }

            let request = request.body(body.into())?;

            let response = http.send(request).await.context("error sending crash")?;
            if !response.status().is_success() {
                log::error!("Error uploading crash to server: {}", response.status());
            }

            if uploaded < filename {
                uploaded = filename.clone();
                KEY_VALUE_STORE
                    .write_kvp(LAST_CRASH_UPLOADED.to_string(), filename)
                    .await?;
            }
        }
    }

    Ok(())
}

async fn load_login_shell_environment() -> Result<()> {
    let marker = "ZED_LOGIN_SHELL_START";
    let shell = env::var("SHELL").context(
        "SHELL environment variable is not assigned so we can't source login environment variables",
    )?;

    // If possible, we want to `cd` in the user's `$HOME` to trigger programs
    // such as direnv, asdf, mise, ... to adjust the PATH. These tools often hook
    // into shell's `cd` command (and hooks) to manipulate env.
    // We do this so that we get the env a user would have when spawning a shell
    // in home directory.
    let shell_cmd_prefix = std::env::var_os("HOME")
        .and_then(|home| home.into_string().ok())
        .map(|home| format!("cd {home};"));

    // The `exit 0` is the result of hours of debugging, trying to find out
    // why running this command here, without `exit 0`, would mess
    // up signal process for our process so that `ctrl-c` doesn't work
    // anymore.
    // We still don't know why `$SHELL -l -i -c '/usr/bin/env -0'`  would
    // do that, but it does, and `exit 0` helps.
    let shell_cmd = format!(
        "{}echo {marker}; /usr/bin/env -0; exit 0;",
        shell_cmd_prefix.as_deref().unwrap_or("")
    );

    let output = Command::new(&shell)
        .args(["-l", "-i", "-c", &shell_cmd])
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
    std::env::var(FORCE_CLI_MODE_ENV_VAR_NAME).ok().is_none() && std::io::stdout().is_terminal()
}

fn collect_url_args() -> Vec<String> {
    env::args()
        .skip(1)
        .filter_map(|arg| match std::fs::canonicalize(Path::new(&arg)) {
            Ok(path) => Some(format!("file://{}", path.to_string_lossy())),
            Err(error) => {
                if let Some(_) = parse_zed_link(&arg) {
                    Some(arg)
                } else {
                    log::error!("error parsing path argument: {}", error);
                    None
                }
            }
        })
        .collect()
}

fn load_embedded_fonts(cx: &AppContext) {
    let asset_source = cx.asset_source();
    let font_paths = asset_source.list("fonts").unwrap();
    let embedded_fonts = Mutex::new(Vec::new());
    let executor = cx.background_executor();

    executor.block(executor.scoped(|scope| {
        for font_path in &font_paths {
            if !font_path.ends_with(".ttf") {
                continue;
            }

            scope.spawn(async {
                let font_bytes = asset_source.load(font_path).unwrap();
                embedded_fonts.lock().push(font_bytes);
            });
        }
    }));

    cx.text_system()
        .add_fonts(embedded_fonts.into_inner())
        .unwrap();
}

/// Spawns a background task to load the user themes from the themes directory.
fn load_user_themes_in_background(fs: Arc<dyn fs::Fs>, cx: &mut AppContext) {
    cx.spawn({
        let fs = fs.clone();
        |cx| async move {
            if let Some(theme_registry) =
                cx.update(|cx| ThemeRegistry::global(cx).clone()).log_err()
            {
                let themes_dir = paths::THEMES_DIR.as_ref();
                match fs
                    .metadata(themes_dir)
                    .await
                    .ok()
                    .flatten()
                    .map(|m| m.is_dir)
                {
                    Some(is_dir) => {
                        anyhow::ensure!(is_dir, "Themes dir path {themes_dir:?} is not a directory")
                    }
                    None => {
                        fs.create_dir(themes_dir).await.with_context(|| {
                            format!("Failed to create themes dir at path {themes_dir:?}")
                        })?;
                    }
                }
                theme_registry.load_user_themes(themes_dir, fs).await?;
                cx.update(|cx| ThemeSettings::reload_current_theme(cx))?;
            }
            anyhow::Ok(())
        }
    })
    .detach_and_log_err(cx);
}

//todo!(linux): Port fsevents to linux
/// Spawns a background task to watch the themes directory for changes.
#[cfg(target_os = "macos")]
fn watch_themes(fs: Arc<dyn fs::Fs>, cx: &mut AppContext) {
    use std::time::Duration;
    cx.spawn(|cx| async move {
        let mut events = fs
            .watch(&paths::THEMES_DIR.clone(), Duration::from_millis(100))
            .await;

        while let Some(events) = events.next().await {
            for event in events {
                if event.flags.contains(StreamFlags::ITEM_REMOVED) {
                    // Theme was removed, don't need to reload.
                    // We may want to remove the theme from the registry, in this case.
                } else {
                    if let Some(theme_registry) =
                        cx.update(|cx| ThemeRegistry::global(cx).clone()).log_err()
                    {
                        if let Some(()) = theme_registry
                            .load_user_theme(&event.path, fs.clone())
                            .await
                            .log_err()
                        {
                            cx.update(|cx| ThemeSettings::reload_current_theme(cx))
                                .log_err();
                        }
                    }
                }
            }
        }
    })
    .detach()
}

#[cfg(debug_assertions)]
async fn watch_languages(fs: Arc<dyn fs::Fs>, languages: Arc<LanguageRegistry>) {
    use std::time::Duration;

    let reload_debounce = Duration::from_millis(250);

    let mut events = fs
        .watch("crates/zed/src/languages".as_ref(), reload_debounce)
        .await;

    while (events.next().await).is_some() {
        languages.reload();
    }
}

#[cfg(debug_assertions)]
fn watch_file_types(fs: Arc<dyn fs::Fs>, cx: &mut AppContext) {
    use std::time::Duration;

    cx.spawn(|cx| async move {
        let mut events = fs
            .watch(
                "assets/icons/file_icons/file_types.json".as_ref(),
                Duration::from_millis(100),
            )
            .await;
        while (events.next().await).is_some() {
            cx.update(|cx| {
                cx.update_global(|file_types, _| {
                    *file_types = project_panel::file_associations::FileAssociations::new(Assets);
                });
            })
            .ok();
        }
    })
    .detach()
}

#[cfg(not(debug_assertions))]
fn watch_file_types(_fs: Arc<dyn fs::Fs>, _cx: &mut AppContext) {}

#[cfg(not(debug_assertions))]
async fn watch_languages(_fs: Arc<dyn fs::Fs>, _languages: Arc<LanguageRegistry>) {}
