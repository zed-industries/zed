#![allow(unused_variables, dead_code, unused_mut)]
// todo!() this is to make transition easier.

// Allow binary to be called Zed for a nice application menu when running executable directly
#![allow(non_snake_case)]

use anyhow::{anyhow, Context as _, Result};
use backtrace::Backtrace;
use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
use client::UserStore;
use db::kvp::KEY_VALUE_STORE;
use fs::RealFs;
use futures::StreamExt;
use gpui::{Action, App, AppContext, AsyncAppContext, Context, SemanticVersion, Task};
use isahc::{prelude::Configurable, Request};
use language::LanguageRegistry;
use log::LevelFilter;

use node_runtime::RealNodeRuntime;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use settings::{
    default_settings, handle_keymap_file_changes, handle_settings_file_changes, watch_config_file,
    Settings, SettingsStore,
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
    time::{SystemTime, UNIX_EPOCH},
};
use theme::ActiveTheme;
use util::{
    async_maybe,
    channel::{parse_zed_link, ReleaseChannel, RELEASE_CHANNEL},
    http::{self, HttpClient},
    paths, ResultExt,
};
use uuid::Uuid;
use workspace::{AppState, WorkspaceStore};
use zed2::{
    build_window_options, ensure_only_instance, handle_cli_connection, initialize_workspace,
    languages, Assets, IsOnlyInstance, OpenListener, OpenRequest,
};

mod open_listener;

fn main() {
    //TODO!(figure out what the linker issues are here)
    // https://github.com/rust-lang/rust/issues/47384
    // https://github.com/mmastrac/rust-ctor/issues/280
    menu::unused();
    let http = http::client();
    init_paths();
    init_logger();

    if ensure_only_instance() != IsOnlyInstance::Yes {
        return;
    }

    log::info!("========== starting zed ==========");
    let app = App::production(Arc::new(Assets));

    let installation_id = app.background_executor().block(installation_id()).ok();
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
    app.on_open_urls(move |urls, _| open_listener.open_urls(urls));
    app.on_reopen(move |_cx| {
        // todo!("workspace")
        // if cx.has_global::<Weak<AppState>>() {
        // if let Some(app_state) = cx.global::<Weak<AppState>>().upgrade() {
        // workspace::open_new(&app_state, cx, |workspace, cx| {
        //     Editor::new_file(workspace, &Default::default(), cx)
        // })
        // .detach();
        // }
        // }
    });

    app.run(move |cx| {
        cx.set_global(*RELEASE_CHANNEL);
        load_embedded_fonts(cx);

        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        handle_settings_file_changes(user_settings_file_rx, cx);
        handle_keymap_file_changes(user_keymap_file_rx, cx);

        let client = client::Client::new(http.clone(), cx);
        let mut languages = LanguageRegistry::new(login_shell_env_loaded);
        let copilot_language_server_id = languages.next_language_server_id();
        languages.set_executor(cx.background_executor().clone());
        languages.set_language_server_download_dir(paths::LANGUAGES_DIR.clone());
        let languages = Arc::new(languages);
        let node_runtime = RealNodeRuntime::new(http.clone());

        language::init(cx);
        languages::init(languages.clone(), node_runtime.clone(), cx);
        let user_store = cx.build_model(|cx| UserStore::new(client.clone(), http.clone(), cx));
        let workspace_store = cx.build_model(|cx| WorkspaceStore::new(client.clone(), cx));

        cx.set_global(client.clone());

        theme::init(cx);
        // context_menu::init(cx);
        project::Project::init(&client, cx);
        client::init(&client, cx);
        // command_palette::init(cx);
        language::init(cx);
        editor::init(cx);
        copilot::init(
            copilot_language_server_id,
            http.clone(),
            node_runtime.clone(),
            cx,
        );
        // assistant::init(cx);
        // component_test::init(cx);

        // cx.spawn(|cx| watch_themes(fs.clone(), cx)).detach();
        // cx.spawn(|_| watch_languages(fs.clone(), languages.clone()))
        //     .detach();
        // watch_file_types(fs.clone(), cx);

        languages.set_theme(cx.theme().clone());
        // cx.observe_global::<SettingsStore, _>({
        //     let languages = languages.clone();
        //     move |cx| languages.set_theme(theme::current(cx).clone())
        // })
        // .detach();

        // client.telemetry().start(installation_id, session_id, cx);

        let app_state = Arc::new(AppState {
            languages,
            client: client.clone(),
            user_store,
            fs,
            build_window_options,
            initialize_workspace,
            // background_actions: todo!("ask Mikayla"),
            workspace_store,
            node_runtime,
        });
        cx.set_global(Arc::downgrade(&app_state));

        // audio::init(Assets, cx);
        // auto_update::init(http.clone(), client::ZED_SERVER_URL.clone(), cx);

        workspace::init(app_state.clone(), cx);
        // recent_projects::init(cx);

        go_to_line::init(cx);
        // file_finder::init(cx);
        // outline::init(cx);
        // project_symbols::init(cx);
        project_panel::init(Assets, cx);
        // channel::init(&client, user_store.clone(), cx);
        // diagnostics::init(cx);
        // search::init(cx);
        // semantic_index::init(fs.clone(), http.clone(), languages.clone(), cx);
        // vim::init(cx);
        // terminal_view::init(cx);

        // journal2::init(app_state.clone(), cx);
        // language_selector::init(cx);
        // theme_selector::init(cx);
        // activity_indicator::init(cx);
        // language_tools::init(cx);
        call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        // collab_ui::init(&app_state, cx);
        // feedback::init(cx);
        // welcome::init(cx);
        // zed::init(&app_state, cx);

        // cx.set_menus(menus::menus());

        if stdout_is_a_pty() {
            cx.activate(true);
            let urls = collect_url_args();
            if !urls.is_empty() {
                listener.open_urls(urls)
            }
        } else {
            upload_previous_panics(http.clone(), cx);

            // TODO Development mode that forces the CLI mode usually runs Zed binary as is instead
            // of an *app, hence gets no specific callbacks run. Emulate them here, if needed.
            if std::env::var(FORCE_CLI_MODE_ENV_VAR_NAME).ok().is_some()
                && !listener.triggered.load(Ordering::Acquire)
            {
                listener.open_urls(collect_url_args())
            }
        }

        let mut _triggered_authentication = false;

        fn open_paths_and_log_errs(
            paths: &[PathBuf],
            app_state: &Arc<AppState>,
            cx: &mut AppContext,
        ) {
            let task = workspace::open_paths(&paths, &app_state, None, cx);
            cx.spawn(|cx| async move {
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
            Ok(Some(OpenRequest::JoinChannel { channel_id: _ })) => {
                todo!()
                // triggered_authentication = true;
                // let app_state = app_state.clone();
                // let client = client.clone();
                // cx.spawn(|mut cx| async move {
                //     // ignore errors here, we'll show a generic "not signed in"
                //     let _ = authenticate(client, &cx).await;
                //     cx.update(|cx| workspace::join_channel(channel_id, app_state, None, cx))
                //         .await
                // })
                // .detach_and_log_err(cx)
            }
            Ok(Some(OpenRequest::OpenChannelNotes { channel_id: _ })) => {
                todo!()
            }
            Ok(None) | Err(_) => cx
                .spawn({
                    let app_state = app_state.clone();
                    |cx| async move { restore_or_create_workspace(&app_state, cx).await }
                })
                .detach(),
        }

        let app_state = app_state.clone();
        cx.spawn(|cx| async move {
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
                    OpenRequest::JoinChannel { channel_id: _ } => {
                        todo!()
                    }
                    OpenRequest::OpenChannelNotes { channel_id: _ } => {
                        todo!()
                    }
                }
            }
        })
        .detach();

        // if !triggered_authentication {
        //     cx.spawn(|cx| async move { authenticate(client, &cx).await })
        //         .detach_and_log_err(cx);
        // }
    });
}

// async fn authenticate(client: Arc<Client>, cx: &AsyncAppContext) -> Result<()> {
//     if stdout_is_a_pty() {
//         if client::IMPERSONATE_LOGIN.is_some() {
//             client.authenticate_and_connect(false, &cx).await?;
//         }
//     } else if client.has_keychain_credentials(&cx) {
//         client.authenticate_and_connect(true, &cx).await?;
//     }
//     Ok::<_, anyhow::Error>(())
// }

async fn installation_id() -> Result<String> {
    let legacy_key_name = "device_id";

    if let Ok(Some(installation_id)) = KEY_VALUE_STORE.read_kvp(legacy_key_name) {
        Ok(installation_id)
    } else {
        let installation_id = Uuid::new_v4().to_string();

        KEY_VALUE_STORE
            .write_kvp(legacy_key_name.to_string(), installation_id.clone())
            .await?;

        Ok(installation_id)
    }
}

async fn restore_or_create_workspace(app_state: &Arc<AppState>, mut cx: AsyncAppContext) {
    async_maybe!({
        if let Some(location) = workspace::last_opened_workspace_paths().await {
            cx.update(|cx| workspace::open_paths(location.paths().as_ref(), app_state, None, cx))?
                .await
                .log_err();
        } else if matches!(KEY_VALUE_STORE.read_kvp("******* THIS IS A BAD KEY PLEASE UNCOMMENT BELOW TO FIX THIS VERY LONG LINE *******"), Ok(None)) {
            // todo!(welcome)
            //} else if matches!(KEY_VALUE_STORE.read_kvp(FIRST_OPEN), Ok(None)) {
            //todo!()
            // cx.update(|cx| show_welcome_experience(app_state, cx));
        } else {
            cx.update(|cx| {
                workspace::open_new(app_state, cx, |workspace, cx| {
                    // todo!(editor)
                    // Editor::new_file(workspace, &Default::default(), cx)
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
    app_version: String,
    release_channel: String,
    os_name: String,
    os_version: Option<String>,
    architecture: String,
    panicked_on: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    installation_id: Option<String>,
    session_id: String,
}

#[derive(Serialize)]
struct PanicRequest {
    panic: Panic,
    token: String,
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

        if *util::channel::RELEASE_CHANNEL == ReleaseChannel::Dev {
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

        let app_version = client::ZED_APP_VERSION
            .or(app_metadata.app_version)
            .map_or("dev".to_string(), |v| v.to_string());

        let backtrace = Backtrace::new();
        let mut backtrace = backtrace
            .frames()
            .iter()
            .filter_map(|frame| Some(format!("{:#}", frame.symbols().first()?.name()?)))
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
            app_version: app_version.clone(),
            release_channel: RELEASE_CHANNEL.display_name().into(),
            os_name: app_metadata.os_name.into(),
            os_version: app_metadata
                .os_version
                .as_ref()
                .map(SemanticVersion::to_string),
            architecture: env::consts::ARCH.into(),
            panicked_on: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
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

fn upload_previous_panics(http: Arc<dyn HttpClient>, cx: &mut AppContext) {
    let telemetry_settings = *client::TelemetrySettings::get_global(cx);

    cx.background_executor()
        .spawn(async move {
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

                if !filename.starts_with("zed") {
                    continue;
                }

                if telemetry_settings.diagnostics {
                    let panic_file_content = smol::fs::read_to_string(&child_path)
                        .await
                        .context("error reading panic file")?;

                    let panic = serde_json::from_str(&panic_file_content)
                        .ok()
                        .or_else(|| {
                            panic_file_content
                                .lines()
                                .next()
                                .and_then(|line| serde_json::from_str(line).ok())
                        })
                        .unwrap_or_else(|| {
                            log::error!(
                                "failed to deserialize panic file {:?}",
                                panic_file_content
                            );
                            None
                        });

                    if let Some(panic) = panic {
                        let body = serde_json::to_string(&PanicRequest {
                            panic,
                            token: client::ZED_SECRET_CLIENT_TOKEN.into(),
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
                }

                // We've done what we can, delete the file
                std::fs::remove_file(child_path)
                    .context("error removing panic")
                    .log_err();
            }
            Ok::<_, anyhow::Error>(())
        })
        .detach_and_log_err(cx);
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
                let font_bytes = asset_source.load(font_path).unwrap().to_vec();
                embedded_fonts.lock().push(Arc::from(font_bytes));
            });
        }
    }));

    cx.text_system()
        .add_fonts(&embedded_fonts.into_inner())
        .unwrap();
}

// #[cfg(debug_assertions)]
// async fn watch_themes(fs: Arc<dyn Fs>, mut cx: AsyncAppContext) -> Option<()> {
//     let mut events = fs
//         .watch("styles/src".as_ref(), Duration::from_millis(100))
//         .await;
//     while (events.next().await).is_some() {
//         let output = Command::new("npm")
//             .current_dir("styles")
//             .args(["run", "build"])
//             .output()
//             .await
//             .log_err()?;
//         if output.status.success() {
//             cx.update(|cx| theme_selector::reload(cx))
//         } else {
//             eprintln!(
//                 "build script failed {}",
//                 String::from_utf8_lossy(&output.stderr)
//             );
//         }
//     }
//     Some(())
// }

// #[cfg(debug_assertions)]
// async fn watch_languages(fs: Arc<dyn Fs>, languages: Arc<LanguageRegistry>) -> Option<()> {
//     let mut events = fs
//         .watch(
//             "crates/zed/src/languages".as_ref(),
//             Duration::from_millis(100),
//         )
//         .await;
//     while (events.next().await).is_some() {
//         languages.reload();
//     }
//     Some(())
// }

// #[cfg(debug_assertions)]
// fn watch_file_types(fs: Arc<dyn Fs>, cx: &mut AppContext) {
//     cx.spawn(|mut cx| async move {
//         let mut events = fs
//             .watch(
//                 "assets/icons/file_icons/file_types.json".as_ref(),
//                 Duration::from_millis(100),
//             )
//             .await;
//         while (events.next().await).is_some() {
//             cx.update(|cx| {
//                 cx.update_global(|file_types, _| {
//                     *file_types = project_panel::file_associations::FileAssociations::new(Assets);
//                 });
//             })
//         }
//     })
//     .detach()
// }

// #[cfg(not(debug_assertions))]
// async fn watch_themes(_fs: Arc<dyn Fs>, _cx: AsyncAppContext) -> Option<()> {
//     None
// }

// #[cfg(not(debug_assertions))]
// async fn watch_languages(_: Arc<dyn Fs>, _: Arc<LanguageRegistry>) -> Option<()> {
//     None
// }

// #[cfg(not(debug_assertions))]
// fn watch_file_types(_fs: Arc<dyn Fs>, _cx: &mut AppContext) {}

pub fn background_actions() -> &'static [(&'static str, &'static dyn Action)] {
    // &[
    //     ("Go to file", &file_finder::Toggle),
    //     ("Open command palette", &command_palette::Toggle),
    //     ("Open recent projects", &recent_projects::OpenRecent),
    //     ("Change your settings", &zed_actions::OpenSettings),
    // ]
    // todo!()
    &[]
}
