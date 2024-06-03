// Allow binary to be called Zed for a nice application menu when running executable directly
#![allow(non_snake_case)]
// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod reliability;
mod zed;

use anyhow::{anyhow, Context as _, Result};
use clap::{command, Parser};
use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
use client::{parse_zed_link, Client, DevServerToken, UserStore};
use collab_ui::channel_view::ChannelView;
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use env_logger::Builder;
use fs::RealFs;
use futures::{future, StreamExt};
use git::GitHostingProviderRegistry;
use gpui::{
    App, AppContext, AsyncAppContext, Context, Global, Task, UpdateGlobal as _, VisualContext,
};
use image_viewer;
use language::LanguageRegistry;
use log::LevelFilter;

use assets::Assets;
use node_runtime::RealNodeRuntime;
use parking_lot::Mutex;
use release_channel::AppCommitSha;
use settings::{handle_settings_file_changes, watch_config_file, Settings, SettingsStore};
use simplelog::ConfigBuilder;
use smol::process::Command;
use std::{
    env,
    fs::OpenOptions,
    io::{IsTerminal, Write},
    path::Path,
    process,
    sync::Arc,
};
use theme::{ActiveTheme, SystemAppearance, ThemeRegistry, ThemeSettings};
use util::{maybe, parse_env_output, paths, with_clone, ResultExt, TryFutureExt};
use uuid::Uuid;
use welcome::{show_welcome_view, BaseKeymap, FIRST_OPEN};
use workspace::{AppState, WorkspaceSettings, WorkspaceStore};
use zed::{
    app_menus, build_window_options, handle_cli_connection, handle_keymap_file_changes,
    initialize_workspace, open_paths_with_positions, OpenListener, OpenRequest,
};

use crate::zed::inline_completion_registry;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

enum AppMode {
    Headless(DevServerToken),
    Ui,
}
impl Global for AppMode {}

fn init_headless(
    dev_server_token: DevServerToken,
    app_state: Arc<AppState>,
    cx: &mut AppContext,
) -> Task<Result<()>> {
    match cx.try_global::<AppMode>() {
        Some(AppMode::Headless(token)) if token == &dev_server_token => return Task::ready(Ok(())),
        Some(_) => {
            return Task::ready(Err(anyhow!(
                "zed is already running. Use `kill {}` to stop it",
                process::id()
            )))
        }
        None => {
            cx.set_global(AppMode::Headless(dev_server_token.clone()));
        }
    };
    let client = app_state.client.clone();
    client.set_dev_server_token(dev_server_token);
    headless::init(
        client.clone(),
        headless::AppState {
            languages: app_state.languages.clone(),
            user_store: app_state.user_store.clone(),
            fs: app_state.fs.clone(),
            node_runtime: app_state.node_runtime.clone(),
        },
        cx,
    )
}

fn init_ui(app_state: Arc<AppState>, cx: &mut AppContext) -> Result<()> {
    match cx.try_global::<AppMode>() {
        Some(AppMode::Headless(_)) => {
            return Err(anyhow!(
                "zed is already running in headless mode. Use `kill {}` to stop it",
                process::id()
            ))
        }
        Some(AppMode::Ui) => return Ok(()),
        None => {
            cx.set_global(AppMode::Ui);
        }
    };

    if let Err(err) = cx.can_open_windows() {
        return Err(err);
    }

    SystemAppearance::init(cx);
    load_embedded_fonts(cx);

    theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);
    app_state.languages.set_theme(cx.theme().clone());
    command_palette::init(cx);
    editor::init(cx);
    image_viewer::init(cx);
    diagnostics::init(cx);

    audio::init(Assets, cx);
    workspace::init(app_state.clone(), cx);
    recent_projects::init(cx);

    go_to_line::init(cx);
    file_finder::init(cx);
    tab_switcher::init(cx);
    outline::init(cx);
    project_symbols::init(cx);
    project_panel::init(Assets, cx);
    tasks_ui::init(cx);
    channel::init(&app_state.client.clone(), app_state.user_store.clone(), cx);
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

    // Initialize each completion provider. Settings are used for toggling between them.
    let copilot_language_server_id = app_state.languages.next_language_server_id();
    copilot::init(
        copilot_language_server_id,
        app_state.client.http_client(),
        app_state.node_runtime.clone(),
        cx,
    );
    supermaven::init(app_state.client.clone(), cx);

    inline_completion_registry::init(app_state.client.telemetry().clone(), cx);

    assistant::init(app_state.client.clone(), cx);

    cx.observe_global::<SettingsStore>({
        let languages = app_state.languages.clone();
        let http = app_state.client.http_client();
        let client = app_state.client.clone();

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
    let telemetry = app_state.client.telemetry();
    telemetry.report_setting_event("theme", cx.theme().name.to_string());
    telemetry.report_setting_event("keymap", BaseKeymap::get_global(cx).to_string());
    telemetry.flush_events();

    extension::init(
        app_state.fs.clone(),
        app_state.client.clone(),
        app_state.node_runtime.clone(),
        app_state.languages.clone(),
        ThemeRegistry::global(cx),
        cx,
    );

    dev_server_projects::init(app_state.client.clone(), cx);

    let fs = app_state.fs.clone();
    load_user_themes_in_background(fs.clone(), cx);
    watch_themes(fs.clone(), cx);
    watch_languages(fs.clone(), app_state.languages.clone(), cx);
    watch_file_types(fs.clone(), cx);

    cx.set_menus(app_menus());
    initialize_workspace(app_state.clone(), cx);

    cx.activate(true);

    cx.spawn(|cx| async move { authenticate(app_state.client.clone(), &cx).await })
        .detach_and_log_err(cx);

    Ok(())
}

fn main() {
    menu::init();
    zed_actions::init();

    if let Err(e) = init_paths() {
        fail_to_launch(e);
        return;
    }

    init_logger();

    log::info!("========== starting zed ==========");
    let app = App::new().with_assets(Assets);

    let (installation_id, existing_installation_id_found) = app
        .background_executor()
        .block(installation_id())
        .ok()
        .unzip();
    let session_id = Uuid::new_v4().to_string();
    reliability::init_panic_hook(&app, installation_id.clone(), session_id.clone());

    let (open_listener, mut open_rx) = OpenListener::new();

    #[cfg(target_os = "linux")]
    {
        if crate::zed::listen_for_cli_connections(open_listener.clone()).is_err() {
            println!("zed is already running");
            return;
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        use zed::only_instance::*;
        if ensure_only_instance() != IsOnlyInstance::Yes {
            println!("zed is already running");
            return;
        }
    }

    let git_hosting_provider_registry = Arc::new(GitHostingProviderRegistry::new());
    let git_binary_path = if option_env!("ZED_BUNDLE").as_deref() == Some("true") {
        app.path_for_auxiliary_executable("git")
            .context("could not find git binary path")
            .log_err()
    } else {
        None
    };
    log::info!("Using git binary path: {:?}", git_binary_path);

    let fs = Arc::new(RealFs::new(
        git_hosting_provider_registry.clone(),
        git_binary_path,
    ));
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
            #[cfg(unix)]
            {
                load_shell_from_passwd().await.log_err();
            }
            load_login_shell_environment().await.log_err();
        })
    };

    app.on_open_urls(with_clone!(open_listener, move |urls| open_listener.open_urls(urls)));
    app.on_reopen(move |cx| {
        if let Some(app_state) = AppState::try_global(cx).and_then(|app_state| app_state.upgrade())
        {
            cx.spawn({
                let app_state = app_state.clone();
                |cx| async move { restore_or_create_workspace(app_state, cx).await }
            })
            .detach();
        }
    });

    app.run(move |cx| {
        release_channel::init(env!("CARGO_PKG_VERSION"), cx);
        if let Some(build_sha) = option_env!("ZED_COMMIT_SHA") {
            AppCommitSha::set_global(AppCommitSha(build_sha.into()), cx);
        }

        GitHostingProviderRegistry::set_global(git_hosting_provider_registry, cx);
        git_hosting_providers::init(cx);

        OpenListener::set_global(cx, open_listener.clone());

        settings::init(cx);
        handle_settings_file_changes(user_settings_file_rx, cx);
        handle_keymap_file_changes(user_keymap_file_rx, cx);

        client::init_settings(cx);
        let client = Client::production(cx);
        cx.update_http_client(client.http_client().clone());
        let mut languages =
            LanguageRegistry::new(login_shell_env_loaded, cx.background_executor().clone());
        languages.set_language_server_download_dir(paths::LANGUAGES_DIR.clone());
        let languages = Arc::new(languages);
        let node_runtime = RealNodeRuntime::new(client.http_client());

        language::init(cx);
        languages::init(languages.clone(), node_runtime.clone(), cx);
        let user_store = cx.new_model(|cx| UserStore::new(client.clone(), cx));
        let workspace_store = cx.new_model(|cx| WorkspaceStore::new(client.clone(), cx));

        Client::set_global(client.clone(), cx);

        zed::init(cx);
        project::Project::init(&client, cx);
        client::init(&client, cx);
        language::init(cx);
        let telemetry = client.telemetry();
        telemetry.start(installation_id.clone(), session_id, cx);
        telemetry.report_app_event(
            match existing_installation_id_found {
                Some(false) => "first open",
                _ => "open",
            }
            .to_string(),
        );
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

        auto_update::init(client.http_client(), cx);

        reliability::init(client.http_client(), installation_id, cx);

        let args = Args::parse();
        let urls: Vec<_> = args
            .paths_or_urls
            .iter()
            .filter_map(|arg| parse_url_arg(arg, cx).log_err())
            .collect();

        if !urls.is_empty() {
            open_listener.open_urls(urls)
        }

        match open_rx
            .try_next()
            .ok()
            .flatten()
            .and_then(|urls| OpenRequest::parse(urls, cx).log_err())
        {
            Some(request) => {
                handle_open_request(request, app_state.clone(), cx);
            }
            None => {
                if let Some(dev_server_token) = args.dev_server_token {
                    let task =
                        init_headless(DevServerToken(dev_server_token), app_state.clone(), cx);
                    cx.spawn(|cx| async move {
                        if let Err(e) = task.await {
                            log::error!("{}", e);
                            cx.update(|cx| cx.quit()).log_err();
                        } else {
                            log::info!("connected!");
                        }
                    })
                    .detach();
                } else {
                    init_ui(app_state.clone(), cx).unwrap();
                    cx.spawn({
                        let app_state = app_state.clone();
                        |cx| async move { restore_or_create_workspace(app_state, cx).await }
                    })
                    .detach();
                }
            }
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
    });
}

fn handle_open_request(request: OpenRequest, app_state: Arc<AppState>, cx: &mut AppContext) {
    if let Some(connection) = request.cli_connection {
        let app_state = app_state.clone();
        cx.spawn(move |cx| handle_cli_connection(connection, app_state, cx))
            .detach();
        return;
    }

    if let Err(e) = init_ui(app_state.clone(), cx) {
        log::error!("{}", e);
        return;
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
    } else if let Some(task) = task {
        task.detach_and_log_err(cx)
    }
}

async fn authenticate(client: Arc<Client>, cx: &AsyncAppContext) -> Result<()> {
    if stdout_is_a_pty() {
        if *client::ZED_DEVELOPMENT_AUTH {
            client.authenticate_and_connect(true, &cx).await?;
        } else if client::IMPERSONATE_LOGIN.is_some() {
            client.authenticate_and_connect(false, &cx).await?;
        }
    } else if client.has_credentials(&cx).await {
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

#[cfg(unix)]
async fn load_shell_from_passwd() -> Result<()> {
    let buflen = match unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) } {
        n if n < 0 => 1024,
        n => n as usize,
    };
    let mut buffer = Vec::with_capacity(buflen);

    let mut pwd: std::mem::MaybeUninit<libc::passwd> = std::mem::MaybeUninit::uninit();
    let mut result: *mut libc::passwd = std::ptr::null_mut();

    let uid = unsafe { libc::getuid() };
    let status = unsafe {
        libc::getpwuid_r(
            uid,
            pwd.as_mut_ptr(),
            buffer.as_mut_ptr() as *mut libc::c_char,
            buflen,
            &mut result,
        )
    };
    let entry = unsafe { pwd.assume_init() };

    anyhow::ensure!(
        status == 0,
        "call to getpwuid_r failed. uid: {}, status: {}",
        uid,
        status
    );
    anyhow::ensure!(!result.is_null(), "passwd entry for uid {} not found", uid);
    anyhow::ensure!(
        entry.pw_uid == uid,
        "passwd entry has different uid ({}) than getuid ({}) returned",
        entry.pw_uid,
        uid,
    );

    let shell = unsafe { std::ffi::CStr::from_ptr(entry.pw_shell).to_str().unwrap() };
    if env::var("SHELL").map_or(true, |shell_env| shell_env != shell) {
        log::info!(
            "updating SHELL environment variable to value from passwd entry: {:?}",
            shell,
        );
        env::set_var("SHELL", shell);
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
                let font_bytes = asset_source.load(font_path).unwrap().unwrap();
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
fn watch_languages(fs: Arc<dyn fs::Fs>, languages: Arc<LanguageRegistry>, cx: &mut AppContext) {
    use std::time::Duration;

    let path = {
        let p = Path::new("crates/languages/src");
        let Ok(full_path) = p.canonicalize() else {
            return;
        };
        full_path
    };

    cx.spawn(|_| async move {
        let mut events = fs.watch(path.as_path(), Duration::from_millis(100)).await;
        while let Some(event) = events.next().await {
            let has_language_file = event.iter().any(|path| {
                path.extension()
                    .map(|ext| ext.to_string_lossy().as_ref() == "scm")
                    .unwrap_or(false)
            });
            if has_language_file {
                languages.reload();
            }
        }
    })
    .detach()
}

#[cfg(not(debug_assertions))]
fn watch_languages(_fs: Arc<dyn fs::Fs>, _languages: Arc<LanguageRegistry>, _cx: &mut AppContext) {}

#[cfg(debug_assertions)]
fn watch_file_types(fs: Arc<dyn fs::Fs>, cx: &mut AppContext) {
    use std::time::Duration;

    use file_icons::FileIcons;
    use gpui::UpdateGlobal;

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
                FileIcons::update_global(cx, |file_types, _cx| {
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
