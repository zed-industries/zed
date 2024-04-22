// Allow binary to be called Zed for a nice application menu when running executable directly
#![allow(non_snake_case)]
// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod zed;

use anyhow::{anyhow, Context as _, Result};
use backtrace::Backtrace;
use chrono::Utc;
use clap::{command, Parser};
use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
use client::{parse_zed_link, telemetry::Telemetry, Client, DevServerToken, UserStore};
use collab_ui::channel_view::ChannelView;
use copilot::Copilot;
use copilot_ui::CopilotCompletionProvider;
use db::kvp::KEY_VALUE_STORE;
use editor::{Editor, EditorMode};
use env_logger::Builder;
use fs::RealFs;
use futures::{future, StreamExt};
use gpui::{
    App, AppContext, AsyncAppContext, Context, SemanticVersion, Task, ViewContext, VisualContext,
};
use image_viewer;
use isahc::{prelude::Configurable, Request};
use language::LanguageRegistry;
use log::LevelFilter;

use assets::Assets;
use mimalloc::MiMalloc;
use node_runtime::RealNodeRuntime;
use parking_lot::Mutex;
use release_channel::{AppCommitSha, ReleaseChannel, RELEASE_CHANNEL};
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
    path::Path,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    thread,
};
use theme::{ActiveTheme, SystemAppearance, ThemeRegistry, ThemeSettings};
use util::{
    http::{HttpClient, HttpClientWithUrl},
    maybe, parse_env_output,
    paths::{self, CRASHES_DIR, CRASHES_RETIRED_DIR},
    ResultExt, TryFutureExt,
};
use uuid::Uuid;
use welcome::{show_welcome_view, BaseKeymap, FIRST_OPEN};
use workspace::{AppState, WorkspaceSettings, WorkspaceStore};
use zed::{
    app_menus, build_window_options, ensure_only_instance, handle_cli_connection,
    handle_keymap_file_changes, initialize_workspace, open_paths_with_positions, IsOnlyInstance,
    OpenListener, OpenRequest,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn fail_to_launch(e: anyhow::Error) {
    App::new().run(move |cx| {
        let window = cx.open_window(gpui::WindowOptions::default(), |cx| cx.new_view(|_| gpui::Empty));
        window.update(cx, |_, cx| {
            let response = cx.prompt(gpui::PromptLevel::Critical, "Zed failed to launch", Some(&format!("{}\n\nFor help resolving this, please open an issue on https://github.com/zed-industries/zed", e)), &["Exit"]);

            cx.spawn(|_, mut cx| async move {
                response.await?;
                cx.update(|cx| {
                    cx.quit()
                })
            }).detach_and_log_err(cx);
        }).log_err();
    })
}

fn init_headless(dev_server_token: DevServerToken) {
    if let Err(e) = init_paths() {
        log::error!("Failed to launch: {}", e);
        return;
    }
    init_logger();

    App::new().run(|cx| {
        release_channel::init(env!("CARGO_PKG_VERSION"), cx);
        if let Some(build_sha) = option_env!("ZED_COMMIT_SHA") {
            AppCommitSha::set_global(AppCommitSha(build_sha.into()), cx);
        }

        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);

        client::init_settings(cx);

        let clock = Arc::new(clock::RealSystemClock);
        let http = Arc::new(HttpClientWithUrl::new(
            &client::ClientSettings::get_global(cx).server_url,
        ));

        let client = client::Client::new(clock, http.clone(), cx);
        let client = client.clone();
        client.set_dev_server_token(dev_server_token);

        project::Project::init(&client, cx);
        client::init(&client, cx);

        let git_binary_path = if option_env!("ZED_BUNDLE").as_deref() == Some("true") {
            cx.path_for_auxiliary_executable("git")
                .context("could not find git binary path")
                .log_err()
        } else {
            None
        };
        let fs = Arc::new(RealFs::new(git_binary_path));

        let mut languages =
            LanguageRegistry::new(Task::ready(()), cx.background_executor().clone());
        languages.set_language_server_download_dir(paths::LANGUAGES_DIR.clone());
        let languages = Arc::new(languages);
        let node_runtime = RealNodeRuntime::new(http.clone());

        language::init(cx);
        languages::init(languages.clone(), node_runtime.clone(), cx);
        let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));

        headless::init(
            client.clone(),
            headless::AppState {
                languages: languages.clone(),
                user_store: user_store.clone(),
                fs: fs.clone(),
                node_runtime: node_runtime.clone(),
            },
            cx,
        );
    })
}

fn init_ui(args: Args) {
    menu::init();
    zed_actions::init();

    if let Err(e) = init_paths() {
        fail_to_launch(e);
        return;
    }

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

    let git_binary_path = if option_env!("ZED_BUNDLE").as_deref() == Some("true") {
        app.path_for_auxiliary_executable("git")
            .context("could not find git binary path")
            .log_err()
    } else {
        None
    };
    log::info!("Using git binary path: {:?}", git_binary_path);

    let fs = Arc::new(RealFs::new(git_binary_path));
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
    app.on_open_urls(move |urls| open_listener.open_urls(urls));
    app.on_reopen(move |cx| {
        if let Some(app_state) = AppState::try_global(cx).and_then(|app_state| app_state.upgrade())
        {
            workspace::open_new(app_state, cx, |workspace, cx| {
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
        let mut languages =
            LanguageRegistry::new(login_shell_env_loaded, cx.background_executor().clone());
        let copilot_language_server_id = languages.next_language_server_id();
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
        image_viewer::init(cx);
        diagnostics::init(cx);
        copilot::init(
            copilot_language_server_id,
            http.clone(),
            node_runtime.clone(),
            cx,
        );
        assistant::init(client.clone(), cx);
        init_inline_completion_provider(client.telemetry().clone(), cx);

        extension::init(
            fs.clone(),
            client.clone(),
            node_runtime.clone(),
            languages.clone(),
            ThemeRegistry::global(cx),
            cx,
        );

        load_user_themes_in_background(fs.clone(), cx);
        watch_themes(fs.clone(), cx);

        watch_file_types(fs.clone(), cx);

        languages.set_theme(cx.theme().clone());

        cx.observe_global::<SettingsStore>({
            let languages = languages.clone();
            let http = http.clone();
            let client = client.clone();

            move |cx| {
                for &mut window in cx.windows().iter_mut() {
                    let background_appearance = cx.theme().window_background_appearance();
                    window
                        .update(cx, |_, cx| {
                            cx.set_background_appearance(background_appearance)
                        })
                        .ok();
                }
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
            node_runtime: node_runtime.clone(),
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        audio::init(Assets, cx);
        auto_update::init(http.clone(), cx);

        workspace::init(app_state.clone(), cx);
        recent_projects::init(cx);

        go_to_line::init(cx);
        file_finder::init(cx);
        tab_switcher::init(cx);
        outline::init(cx);
        project_symbols::init(cx);
        project_panel::init(Assets, cx);
        tasks_ui::init(cx);
        channel::init(&client, user_store.clone(), cx);
        search::init(cx);
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

        // todo(linux): unblock this
        upload_panics_and_crashes(http.clone(), cx);

        cx.activate(true);

        let mut triggered_authentication = false;
        let urls: Vec<_> = args
            .paths_or_urls
            .iter()
            .filter_map(|arg| parse_url_arg(arg, cx).log_err())
            .collect();

        if !urls.is_empty() {
            listener.open_urls(urls)
        }

        match open_rx
            .try_next()
            .ok()
            .flatten()
            .and_then(|urls| OpenRequest::parse(urls, cx).log_err())
        {
            Some(request) => {
                triggered_authentication = handle_open_request(request, app_state.clone(), cx)
            }
            None => cx
                .spawn({
                    let app_state = app_state.clone();
                    |cx| async move { restore_or_create_workspace(app_state, cx).await }
                })
                .detach(),
        }

        let app_state = app_state.clone();
        cx.spawn(move |cx| async move {
            while let Some(urls) = open_rx.next().await {
                cx.update(|cx| {
                    if let Some(request) = OpenRequest::parse(urls, cx).log_err() {
                        handle_open_request(request, app_state.clone(), cx);
                    }
                })
                .ok();
            }
        })
        .detach();

        if !triggered_authentication {
            cx.spawn(|cx| async move { authenticate(client, &cx).await })
                .detach_and_log_err(cx);
        }
    });
}

fn main() {
    let mut args = Args::parse();
    if let Some(dev_server_token) = args.dev_server_token.take() {
        let dev_server_token = DevServerToken(dev_server_token);
        init_headless(dev_server_token)
    } else {
        init_ui(args)
    }
}

fn handle_open_request(
    request: OpenRequest,
    app_state: Arc<AppState>,
    cx: &mut AppContext,
) -> bool {
    if let Some(connection) = request.cli_connection {
        let app_state = app_state.clone();
        cx.spawn(move |cx| handle_cli_connection(connection, app_state, cx))
            .detach();
        return false;
    }

    let mut task = None;
    if !request.open_paths.is_empty() {
        let app_state = app_state.clone();
        task = Some(cx.spawn(|mut cx| async move {
            let (_window, results) = open_paths_with_positions(
                &request.open_paths,
                app_state,
                workspace::OpenOptions::default(),
                &mut cx,
            )
            .await?;
            for result in results.into_iter().flatten() {
                if let Err(err) = result {
                    log::error!("Error opening path: {err}",);
                }
            }
            anyhow::Ok(())
        }));
    }

    if !request.open_channel_notes.is_empty() || request.join_channel.is_some() {
        cx.spawn(|mut cx| async move {
            if let Some(task) = task {
                task.await?;
            }
            let client = app_state.client.clone();
            // we continue even if authentication fails as join_channel/ open channel notes will
            // show a visible error message.
            authenticate(client, &cx).await.log_err();

            if let Some(channel_id) = request.join_channel {
                cx.update(|cx| {
                    workspace::join_channel(
                        client::ChannelId(channel_id),
                        app_state.clone(),
                        None,
                        cx,
                    )
                })?
                .await?;
            }

            let workspace_window =
                workspace::get_any_active_workspace(app_state, cx.clone()).await?;
            let workspace = workspace_window.root_view(&cx)?;

            let mut promises = Vec::new();
            for (channel_id, heading) in request.open_channel_notes {
                promises.push(cx.update_window(workspace_window.into(), |_, cx| {
                    ChannelView::open(
                        client::ChannelId(channel_id),
                        heading,
                        workspace.clone(),
                        cx,
                    )
                    .log_err()
                })?)
            }
            future::join_all(promises).await;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
        true
    } else {
        if let Some(task) = task {
            task.detach_and_log_err(cx)
        }
        false
    }
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

async fn restore_or_create_workspace(app_state: Arc<AppState>, cx: AsyncAppContext) {
    maybe!(async {
        let restore_behaviour =
            cx.update(|cx| WorkspaceSettings::get(None, cx).restore_on_startup)?;
        let location = match restore_behaviour {
            workspace::RestoreOnStartupBehaviour::LastWorkspace => {
                workspace::last_opened_workspace_paths().await
            }
            _ => None,
        };
        if let Some(location) = location {
            cx.update(|cx| {
                workspace::open_paths(
                    location.paths().as_ref(),
                    app_state,
                    workspace::OpenOptions::default(),
                    cx,
                )
            })?
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

fn init_paths() -> anyhow::Result<()> {
    for path in [
        &*util::paths::CONFIG_DIR,
        &*util::paths::EXTENSIONS_DIR,
        &*util::paths::LANGUAGES_DIR,
        &*util::paths::DB_DIR,
        &*util::paths::LOGS_DIR,
        &*util::paths::TEMP_DIR,
    ]
    .iter()
    {
        std::fs::create_dir_all(path)
            .map_err(|e| anyhow!("Could not create directory {:?}: {}", path, e))?;
    }
    Ok(())
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
            payload,
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

    let crash_report_url = http.build_zed_api_url("/telemetry/crashes", &[])?;

    // crash directories are only set on MacOS
    for dir in [&*CRASHES_DIR, &*CRASHES_RETIRED_DIR]
        .iter()
        .filter_map(|d| d.as_deref())
    {
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

            let mut request = Request::post(&crash_report_url.to_string())
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
        .map(|home| format!("cd '{home}';"));

    // The `exit 0` is the result of hours of debugging, trying to find out
    // why running this command here, without `exit 0`, would mess
    // up signal process for our process so that `ctrl-c` doesn't work
    // anymore.
    // We still don't know why `$SHELL -l -i -c '/usr/bin/env -0'`  would
    // do that, but it does, and `exit 0` helps.
    let shell_cmd = format!(
        "{}printf '%s' {marker}; /usr/bin/env; exit 0;",
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

        parse_env_output(env_output, |key, value| env::set_var(key, value));

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

#[derive(Parser, Debug)]
#[command(name = "zed", disable_version_flag = true)]
struct Args {
    /// A sequence of space-separated paths or urls that you want to open.
    ///
    /// Use `path:line:row` syntax to open a file at a specific location.
    /// Non-existing paths and directories will ignore `:line:row` suffix.
    ///
    /// URLs can either be file:// or zed:// scheme, or relative to https://zed.dev.
    paths_or_urls: Vec<String>,

    /// Instructs zed to run as a dev server on this machine. (not implemented)
    #[arg(long)]
    dev_server_token: Option<String>,
}

fn parse_url_arg(arg: &str, cx: &AppContext) -> Result<String> {
    match std::fs::canonicalize(Path::new(&arg)) {
        Ok(path) => Ok(format!("file://{}", path.to_string_lossy())),
        Err(error) => {
            if arg.starts_with("file://") || arg.starts_with("zed-cli://") {
                Ok(arg.into())
            } else if let Some(_) = parse_zed_link(&arg, cx) {
                Ok(arg.into())
            } else {
                Err(anyhow!("error parsing path argument: {}", error))
            }
        }
    }
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

/// Spawns a background task to watch the themes directory for changes.
fn watch_themes(fs: Arc<dyn fs::Fs>, cx: &mut AppContext) {
    use std::time::Duration;
    cx.spawn(|cx| async move {
        let mut events = fs
            .watch(&paths::THEMES_DIR.clone(), Duration::from_millis(100))
            .await;

        while let Some(paths) = events.next().await {
            for path in paths {
                if fs.metadata(&path).await.ok().flatten().is_some() {
                    if let Some(theme_registry) =
                        cx.update(|cx| ThemeRegistry::global(cx).clone()).log_err()
                    {
                        if let Some(()) = theme_registry
                            .load_user_theme(&path, fs.clone())
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
fn watch_file_types(fs: Arc<dyn fs::Fs>, cx: &mut AppContext) {
    use std::time::Duration;

    use gpui::BorrowAppContext;

    let path = {
        let p = Path::new("assets/icons/file_icons/file_types.json");
        let Ok(full_path) = p.canonicalize() else {
            return;
        };
        full_path
    };

    cx.spawn(|cx| async move {
        let mut events = fs.watch(path.as_path(), Duration::from_millis(100)).await;
        while (events.next().await).is_some() {
            cx.update(|cx| {
                cx.update_global(|file_types, _| {
                    *file_types = file_icons::FileIcons::new(Assets);
                });
            })
            .ok();
        }
    })
    .detach()
}

#[cfg(not(debug_assertions))]
fn watch_file_types(_fs: Arc<dyn fs::Fs>, _cx: &mut AppContext) {}

fn init_inline_completion_provider(telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    if let Some(copilot) = Copilot::global(cx) {
        cx.observe_new_views(move |editor: &mut Editor, cx: &mut ViewContext<Editor>| {
            if editor.mode() == EditorMode::Full {
                // We renamed some of these actions to not be copilot-specific, but that
                // would have not been backwards-compatible. So here we are re-registering
                // the actions with the old names to not break people's keymaps.
                editor
                    .register_action(cx.listener(
                        |editor, _: &copilot::Suggest, cx: &mut ViewContext<Editor>| {
                            editor.show_inline_completion(&Default::default(), cx);
                        },
                    ))
                    .register_action(cx.listener(
                        |editor, _: &copilot::NextSuggestion, cx: &mut ViewContext<Editor>| {
                            editor.next_inline_completion(&Default::default(), cx);
                        },
                    ))
                    .register_action(cx.listener(
                        |editor, _: &copilot::PreviousSuggestion, cx: &mut ViewContext<Editor>| {
                            editor.previous_inline_completion(&Default::default(), cx);
                        },
                    ))
                    .register_action(cx.listener(
                        |editor,
                         _: &editor::actions::AcceptPartialCopilotSuggestion,
                         cx: &mut ViewContext<Editor>| {
                            editor.accept_partial_inline_completion(&Default::default(), cx);
                        },
                    ));

                let provider = cx.new_model(|_| {
                    CopilotCompletionProvider::new(copilot.clone())
                        .with_telemetry(telemetry.clone())
                });
                editor.set_inline_completion_provider(provider, cx)
            }
        })
        .detach();
    }
}
