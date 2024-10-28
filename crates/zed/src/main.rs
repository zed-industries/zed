// Allow binary to be called Zed for a nice application menu when running executable directly
#![allow(non_snake_case)]
// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod reliability;
mod zed;

use anyhow::{anyhow, Context as _, Result};
use chrono::Offset;
use clap::{command, Parser};
use cli::FORCE_CLI_MODE_ENV_VAR_NAME;
use client::{parse_zed_link, Client, ProxySettings, UserStore};
use collab_ui::channel_view::ChannelView;
use db::kvp::{GLOBAL_KEY_VALUE_STORE, KEY_VALUE_STORE};
use editor::Editor;
use env_logger::Builder;
use fs::{Fs, RealFs};
use futures::{future, StreamExt};
use git::GitHostingProviderRegistry;
use gpui::{
    Action, App, AppContext, AsyncAppContext, Context, DismissEvent, UpdateGlobal as _,
    VisualContext,
};
use http_client::{read_proxy_from_env, Uri};
use language::LanguageRegistry;
use log::LevelFilter;
use reqwest_client::ReqwestClient;

use assets::Assets;
use node_runtime::{NodeBinaryOptions, NodeRuntime};
use parking_lot::Mutex;
use project::project_settings::ProjectSettings;
use recent_projects::{open_ssh_project, SshSettings};
use release_channel::{AppCommitSha, AppVersion};
use session::{AppSession, Session};
use settings::{
    handle_settings_file_changes, watch_config_file, InvalidSettingsError, Settings, SettingsStore,
};
use simplelog::ConfigBuilder;
use smol::process::Command;
use std::{
    env,
    fs::OpenOptions,
    io::{IsTerminal, Write},
    path::{Path, PathBuf},
    process,
    sync::Arc,
};
use theme::{ActiveTheme, SystemAppearance, ThemeRegistry, ThemeSettings};
use time::UtcOffset;
use util::{maybe, parse_env_output, ResultExt, TryFutureExt};
use uuid::Uuid;
use welcome::{show_welcome_view, BaseKeymap, FIRST_OPEN};
use workspace::{
    notifications::{simple_message_notification::MessageNotification, NotificationId},
    AppState, SerializedWorkspaceLocation, WorkspaceSettings, WorkspaceStore,
};
use zed::{
    app_menus, build_window_options, derive_paths_with_position, handle_cli_connection,
    handle_keymap_file_changes, initialize_workspace, open_paths_with_positions, OpenListener,
    OpenRequest,
};

use crate::zed::inline_completion_registry;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn fail_to_launch(e: anyhow::Error) {
    eprintln!("Zed failed to launch: {e:?}");
    App::new().run(move |cx| {
        if let Ok(window) = cx.open_window(gpui::WindowOptions::default(), |cx| cx.new_view(|_| gpui::Empty)) {
            window.update(cx, |_, cx| {
                let response = cx.prompt(gpui::PromptLevel::Critical, "Zed failed to launch", Some(&format!("{e}\n\nFor help resolving this, please open an issue on https://github.com/zed-industries/zed")), &["Exit"]);

                cx.spawn(|_, mut cx| async move {
                    response.await?;
                    cx.update(|cx| {
                        cx.quit()
                    })
                }).detach_and_log_err(cx);
            }).log_err();
        } else {
            fail_to_open_window(e, cx)
        }
    })
}

fn fail_to_open_window_async(e: anyhow::Error, cx: &mut AsyncAppContext) {
    cx.update(|cx| fail_to_open_window(e, cx)).log_err();
}

fn fail_to_open_window(e: anyhow::Error, _cx: &mut AppContext) {
    eprintln!(
        "Zed failed to open a window: {e:?}. See https://zed.dev/docs/linux for troubleshooting steps."
    );
    #[cfg(not(target_os = "linux"))]
    {
        process::exit(1);
    }

    #[cfg(target_os = "linux")]
    {
        use ashpd::desktop::notification::{Notification, NotificationProxy, Priority};
        _cx.spawn(|_cx| async move {
            let Ok(proxy) = NotificationProxy::new().await else {
                process::exit(1);
            };

            let notification_id = "dev.zed.Oops";
            proxy
                .add_notification(
                    notification_id,
                    Notification::new("Zed failed to launch")
                        .body(Some(
                            format!(
                                "{e:?}. See https://zed.dev/docs/linux for troubleshooting steps."
                            )
                            .as_str(),
                        ))
                        .priority(Priority::High)
                        .icon(ashpd::desktop::Icon::with_names(&[
                            "dialog-question-symbolic",
                        ])),
                )
                .await
                .ok();

            process::exit(1);
        })
        .detach();
    }
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

    let system_id = app.background_executor().block(system_id()).ok();
    let installation_id = app.background_executor().block(installation_id()).ok();
    let session_id = Uuid::new_v4().to_string();
    let session = app.background_executor().block(Session::new());
    let app_version = AppVersion::init(env!("CARGO_PKG_VERSION"));

    reliability::init_panic_hook(
        app_version,
        system_id.as_ref().map(|id| id.to_string()),
        installation_id.as_ref().map(|id| id.to_string()),
        session_id.clone(),
    );

    let (open_listener, mut open_rx) = OpenListener::new();

    #[cfg(target_os = "linux")]
    {
        if env::var("ZED_STATELESS").is_err() {
            if crate::zed::listen_for_cli_connections(open_listener.clone()).is_err() {
                println!("zed is already running");
                return;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        use zed::windows_only_instance::*;
        if !check_single_instance() {
            println!("zed is already running");
            return;
        }
    }

    #[cfg(target_os = "macos")]
    {
        use zed::mac_only_instance::*;
        if ensure_only_instance() != IsOnlyInstance::Yes {
            println!("zed is already running");
            return;
        }
    }

    let git_hosting_provider_registry = Arc::new(GitHostingProviderRegistry::new());
    let git_binary_path =
        if cfg!(target_os = "macos") && option_env!("ZED_BUNDLE").as_deref() == Some("true") {
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
        paths::settings_file().clone(),
    );
    let user_keymap_file_rx = watch_config_file(
        &app.background_executor(),
        fs.clone(),
        paths::keymap_file().clone(),
    );

    if !stdout_is_a_pty() {
        app.background_executor()
            .spawn(async {
                #[cfg(unix)]
                {
                    load_shell_from_passwd().await.log_err();
                }
                load_login_shell_environment().await.log_err();
            })
            .detach()
    };

    app.on_open_urls({
        let open_listener = open_listener.clone();
        move |urls| open_listener.open_urls(urls)
    });
    app.on_reopen(move |cx| {
        if let Some(app_state) = AppState::try_global(cx).and_then(|app_state| app_state.upgrade())
        {
            cx.spawn({
                let app_state = app_state.clone();
                |mut cx| async move {
                    if let Err(e) = restore_or_create_workspace(app_state, &mut cx).await {
                        fail_to_open_window_async(e, &mut cx)
                    }
                }
            })
            .detach();
        }
    });

    app.run(move |cx| {
        release_channel::init(app_version, cx);
        if let Some(build_sha) = option_env!("ZED_COMMIT_SHA") {
            AppCommitSha::set_global(AppCommitSha(build_sha.into()), cx);
        }
        settings::init(cx);
        handle_settings_file_changes(user_settings_file_rx, cx, handle_settings_changed);
        handle_keymap_file_changes(user_keymap_file_rx, cx, handle_keymap_changed);
        client::init_settings(cx);
        let user_agent = format!(
            "Zed/{} ({}; {})",
            AppVersion::global(cx),
            std::env::consts::OS,
            std::env::consts::ARCH
        );
        let proxy_str = ProxySettings::get_global(cx).proxy.to_owned();
        let proxy_url = proxy_str
            .as_ref()
            .and_then(|input| {
                input
                    .parse::<Uri>()
                    .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                    .ok()
            })
            .or_else(read_proxy_from_env);
        let http = ReqwestClient::proxy_and_user_agent(proxy_url, &user_agent)
            .expect("could not start HTTP client");
        cx.set_http_client(Arc::new(http));

        <dyn Fs>::set_global(fs.clone(), cx);

        GitHostingProviderRegistry::set_global(git_hosting_provider_registry, cx);
        git_hosting_providers::init(cx);

        OpenListener::set_global(cx, open_listener.clone());

        let client = Client::production(cx);
        cx.set_http_client(client.http_client().clone());
        let mut languages = LanguageRegistry::new(cx.background_executor().clone());
        languages.set_language_server_download_dir(paths::languages_dir().clone());
        let languages = Arc::new(languages);
        let (tx, rx) = async_watch::channel(None);
        cx.observe_global::<SettingsStore>(move |cx| {
            let settings = &ProjectSettings::get_global(cx).node;
            let options = NodeBinaryOptions {
                allow_path_lookup: !settings.ignore_system_version.unwrap_or_default(),
                // TODO: Expose this setting
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
        let node_runtime = NodeRuntime::new(client.http_client(), rx);

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
        telemetry.start(
            system_id.as_ref().map(|id| id.to_string()),
            installation_id.as_ref().map(|id| id.to_string()),
            session_id,
            cx,
        );

        // We should rename these in the future to `first app open`, `first app open for release channel`, and `app open`
        if let (Some(system_id), Some(installation_id)) = (&system_id, &installation_id) {
            match (&system_id, &installation_id) {
                (IdType::New(_), IdType::New(_)) => {
                    telemetry.report_app_event("first open".to_string());
                    telemetry.report_app_event("first open for release channel".to_string());
                }
                (IdType::Existing(_), IdType::New(_)) => {
                    telemetry.report_app_event("first open for release channel".to_string());
                }
                (_, IdType::Existing(_)) => {
                    telemetry.report_app_event("open".to_string());
                }
            }
        }
        let app_session = cx.new_model(|cx| AppSession::new(session, cx));

        let app_state = Arc::new(AppState {
            languages: languages.clone(),
            client: client.clone(),
            user_store: user_store.clone(),
            fs: fs.clone(),
            build_window_options,
            workspace_store,
            node_runtime: node_runtime.clone(),
            session: app_session,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        auto_update::init(client.http_client(), cx);
        reliability::init(
            client.http_client(),
            installation_id.clone().map(|id| id.to_string()),
            cx,
        );

        SystemAppearance::init(cx);
        theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);
        command_palette::init(cx);
        let copilot_language_server_id = app_state.languages.next_language_server_id();
        copilot::init(
            copilot_language_server_id,
            app_state.fs.clone(),
            app_state.client.http_client(),
            app_state.node_runtime.clone(),
            cx,
        );
        supermaven::init(app_state.client.clone(), cx);
        language_model::init(
            app_state.user_store.clone(),
            app_state.client.clone(),
            app_state.fs.clone(),
            cx,
        );
        snippet_provider::init(cx);
        inline_completion_registry::init(app_state.client.telemetry().clone(), cx);
        let prompt_builder = assistant::init(
            app_state.fs.clone(),
            app_state.client.clone(),
            stdout_is_a_pty(),
            cx,
        );
        repl::init(
            app_state.fs.clone(),
            app_state.client.telemetry().clone(),
            cx,
        );
        extension::init(
            app_state.fs.clone(),
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.languages.clone(),
            ThemeRegistry::global(cx),
            cx,
        );
        recent_projects::init(cx);

        load_embedded_fonts(cx);

        #[cfg(target_os = "linux")]
        crate::zed::linux_prompts::init(cx);

        app_state.languages.set_theme(cx.theme().clone());
        editor::init(cx);
        image_viewer::init(cx);
        diagnostics::init(cx);

        audio::init(Assets, cx);
        workspace::init(app_state.clone(), cx);

        go_to_line::init(cx);
        file_finder::init(cx);
        tab_switcher::init(cx);
        outline::init(cx);
        project_symbols::init(cx);
        project_panel::init(Assets, cx);
        outline_panel::init(Assets, cx);
        tasks_ui::init(cx);
        snippets_ui::init(cx);
        channel::init(&app_state.client.clone(), app_state.user_store.clone(), cx);
        search::init(cx);
        vim::init(cx);
        terminal_view::init(cx);
        journal::init(app_state.clone(), cx);
        language_selector::init(cx);
        toolchain_selector::init(cx);
        theme_selector::init(cx);
        language_tools::init(cx);
        call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        notifications::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        collab_ui::init(&app_state, cx);
        feedback::init(cx);
        markdown_preview::init(cx);
        welcome::init(cx);
        settings_ui::init(cx);
        extensions_ui::init(cx);

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

        let fs = app_state.fs.clone();
        load_user_themes_in_background(fs.clone(), cx);
        watch_themes(fs.clone(), cx);
        watch_languages(fs.clone(), app_state.languages.clone(), cx);
        watch_file_types(fs.clone(), cx);

        cx.set_menus(app_menus());
        initialize_workspace(app_state.clone(), prompt_builder, cx);

        cx.activate(true);

        cx.spawn({
            let client = app_state.client.clone();
            |cx| async move { authenticate(client, &cx).await }
        })
        .detach_and_log_err(cx);

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
                cx.spawn({
                    let app_state = app_state.clone();
                    |mut cx| async move {
                        if let Err(e) = restore_or_create_workspace(app_state, &mut cx).await {
                            fail_to_open_window_async(e, &mut cx)
                        }
                    }
                })
                .detach();
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

fn handle_keymap_changed(error: Option<anyhow::Error>, cx: &mut AppContext) {
    struct KeymapParseErrorNotification;
    let id = NotificationId::unique::<KeymapParseErrorNotification>();

    for workspace in workspace::local_workspace_windows(cx) {
        workspace
            .update(cx, |workspace, cx| match &error {
                Some(error) => {
                    workspace.show_notification(id.clone(), cx, |cx| {
                        cx.new_view(|_| {
                            MessageNotification::new(format!("Invalid keymap file\n{error}"))
                                .with_click_message("Open keymap file")
                                .on_click(|cx| {
                                    cx.dispatch_action(zed_actions::OpenKeymap.boxed_clone());
                                    cx.emit(DismissEvent);
                                })
                        })
                    });
                }
                None => workspace.dismiss_notification(&id, cx),
            })
            .log_err();
    }
}

fn handle_settings_changed(error: Option<anyhow::Error>, cx: &mut AppContext) {
    struct SettingsParseErrorNotification;
    let id = NotificationId::unique::<SettingsParseErrorNotification>();

    for workspace in workspace::local_workspace_windows(cx) {
        workspace
            .update(cx, |workspace, cx| {
                match error.as_ref() {
                    Some(error) => {
                        if let Some(InvalidSettingsError::LocalSettings { .. }) =
                            error.downcast_ref::<InvalidSettingsError>()
                        {
                            // Local settings will be displayed by the projects
                        } else {
                            workspace.show_notification(id.clone(), cx, |cx| {
                                cx.new_view(|_| {
                                    MessageNotification::new(format!(
                                        "Invalid user settings file\n{error}"
                                    ))
                                    .with_click_message("Open settings file")
                                    .on_click(|cx| {
                                        cx.dispatch_action(zed_actions::OpenSettings.boxed_clone());
                                        cx.emit(DismissEvent);
                                    })
                                })
                            });
                        }
                    }
                    None => workspace.dismiss_notification(&id, cx),
                }
            })
            .log_err();
    }
}

fn handle_open_request(request: OpenRequest, app_state: Arc<AppState>, cx: &mut AppContext) {
    if let Some(connection) = request.cli_connection {
        let app_state = app_state.clone();
        cx.spawn(move |cx| handle_cli_connection(connection, app_state, cx))
            .detach();
        return;
    }

    if let Some(connection_options) = request.ssh_connection {
        cx.spawn(|mut cx| async move {
            let paths_with_position =
                derive_paths_with_position(app_state.fs.as_ref(), request.open_paths).await;
            open_ssh_project(
                connection_options,
                paths_with_position.into_iter().map(|p| p.path).collect(),
                app_state,
                workspace::OpenOptions::default(),
                &mut cx,
            )
            .await
        })
        .detach_and_log_err(cx);
        return;
    }

    let mut task = None;
    if !request.open_paths.is_empty() {
        let app_state = app_state.clone();
        task = Some(cx.spawn(|mut cx| async move {
            let paths_with_position =
                derive_paths_with_position(app_state.fs.as_ref(), request.open_paths).await;
            let (_window, results) = open_paths_with_positions(
                &paths_with_position,
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
            let result = maybe!(async {
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
            .await;
            if let Err(err) = result {
                fail_to_open_window_async(err, &mut cx);
            }
        })
        .detach()
    } else if let Some(task) = task {
        cx.spawn(|mut cx| async move {
            if let Err(err) = task.await {
                fail_to_open_window_async(err, &mut cx);
            }
        })
        .detach();
    }
}

async fn authenticate(client: Arc<Client>, cx: &AsyncAppContext) -> Result<()> {
    if stdout_is_a_pty() {
        if *client::ZED_DEVELOPMENT_AUTH {
            client.authenticate_and_connect(true, cx).await?;
        } else if client::IMPERSONATE_LOGIN.is_some() {
            client.authenticate_and_connect(false, cx).await?;
        }
    } else if client.has_credentials(cx).await {
        client.authenticate_and_connect(true, cx).await?;
    }
    Ok::<_, anyhow::Error>(())
}

async fn system_id() -> Result<IdType> {
    let key_name = "system_id".to_string();

    if let Ok(Some(system_id)) = GLOBAL_KEY_VALUE_STORE.read_kvp(&key_name) {
        return Ok(IdType::Existing(system_id));
    }

    let system_id = Uuid::new_v4().to_string();

    GLOBAL_KEY_VALUE_STORE
        .write_kvp(key_name, system_id.clone())
        .await?;

    Ok(IdType::New(system_id))
}

async fn installation_id() -> Result<IdType> {
    let legacy_key_name = "device_id".to_string();
    let key_name = "installation_id".to_string();

    // Migrate legacy key to new key
    if let Ok(Some(installation_id)) = KEY_VALUE_STORE.read_kvp(&legacy_key_name) {
        KEY_VALUE_STORE
            .write_kvp(key_name, installation_id.clone())
            .await?;
        KEY_VALUE_STORE.delete_kvp(legacy_key_name).await?;
        return Ok(IdType::Existing(installation_id));
    }

    if let Ok(Some(installation_id)) = KEY_VALUE_STORE.read_kvp(&key_name) {
        return Ok(IdType::Existing(installation_id));
    }

    let installation_id = Uuid::new_v4().to_string();

    KEY_VALUE_STORE
        .write_kvp(key_name, installation_id.clone())
        .await?;

    Ok(IdType::New(installation_id))
}

async fn restore_or_create_workspace(
    app_state: Arc<AppState>,
    cx: &mut AsyncAppContext,
) -> Result<()> {
    if let Some(locations) = restorable_workspace_locations(cx, &app_state).await {
        for location in locations {
            match location {
                SerializedWorkspaceLocation::Local(location, _) => {
                    let task = cx.update(|cx| {
                        workspace::open_paths(
                            location.paths().as_ref(),
                            app_state.clone(),
                            workspace::OpenOptions::default(),
                            cx,
                        )
                    })?;
                    task.await?;
                }
                SerializedWorkspaceLocation::Ssh(ssh) => {
                    let connection_options = cx.update(|cx| {
                        SshSettings::get_global(cx)
                            .connection_options_for(ssh.host, ssh.port, ssh.user)
                    })?;
                    let app_state = app_state.clone();
                    cx.spawn(move |mut cx| async move {
                        recent_projects::open_ssh_project(
                            connection_options,
                            ssh.paths.into_iter().map(PathBuf::from).collect(),
                            app_state,
                            workspace::OpenOptions::default(),
                            &mut cx,
                        )
                        .await
                        .log_err();
                    })
                    .detach();
                }
            }
        }
    } else if matches!(KEY_VALUE_STORE.read_kvp(FIRST_OPEN), Ok(None)) {
        cx.update(|cx| show_welcome_view(app_state, cx))?.await?;
    } else {
        cx.update(|cx| {
            workspace::open_new(Default::default(), app_state, cx, |workspace, cx| {
                Editor::new_file(workspace, &Default::default(), cx)
            })
        })?
        .await?;
    }

    Ok(())
}

pub(crate) async fn restorable_workspace_locations(
    cx: &mut AsyncAppContext,
    app_state: &Arc<AppState>,
) -> Option<Vec<SerializedWorkspaceLocation>> {
    let mut restore_behavior = cx
        .update(|cx| WorkspaceSettings::get(None, cx).restore_on_startup)
        .ok()?;

    let session_handle = app_state.session.clone();
    let (last_session_id, last_session_window_stack) = cx
        .update(|cx| {
            let session = session_handle.read(cx);

            (
                session.last_session_id().map(|id| id.to_string()),
                session.last_session_window_stack(),
            )
        })
        .ok()?;

    if last_session_id.is_none()
        && matches!(
            restore_behavior,
            workspace::RestoreOnStartupBehavior::LastSession
        )
    {
        restore_behavior = workspace::RestoreOnStartupBehavior::LastWorkspace;
    }

    match restore_behavior {
        workspace::RestoreOnStartupBehavior::LastWorkspace => {
            workspace::last_opened_workspace_location()
                .await
                .map(|location| vec![location])
        }
        workspace::RestoreOnStartupBehavior::LastSession => {
            if let Some(last_session_id) = last_session_id {
                let ordered = last_session_window_stack.is_some();

                let mut locations = workspace::last_session_workspace_locations(
                    &last_session_id,
                    last_session_window_stack,
                )
                .filter(|locations| !locations.is_empty());

                // Since last_session_window_order returns the windows ordered front-to-back
                // we need to open the window that was frontmost last.
                if ordered {
                    if let Some(locations) = locations.as_mut() {
                        locations.reverse();
                    }
                }

                locations
            } else {
                None
            }
        }
        _ => None,
    }
}

fn init_paths() -> anyhow::Result<()> {
    for path in [
        paths::config_dir(),
        paths::extensions_dir(),
        paths::languages_dir(),
        paths::database_dir(),
        paths::logs_dir(),
        paths::temp_dir(),
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
        if std::fs::metadata(paths::log_file())
            .map_or(false, |metadata| metadata.len() > MAX_LOG_BYTES)
        {
            let _ = std::fs::rename(paths::log_file(), paths::old_log_file());
        }

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(paths::log_file())
        {
            Ok(log_file) => {
                let mut config_builder = ConfigBuilder::new();

                config_builder.set_time_format_rfc3339();
                let local_offset = chrono::Local::now().offset().fix().local_minus_utc();
                if let Ok(offset) = UtcOffset::from_whole_seconds(local_offset) {
                    config_builder.set_time_offset(offset);
                }

                #[cfg(target_os = "linux")]
                {
                    config_builder.add_filter_ignore_str("zbus");
                    config_builder.add_filter_ignore_str("blade_graphics::hal::resource");
                    config_builder.add_filter_ignore_str("naga::back::spv::writer");
                }

                let config = config_builder.build();
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
            use env_logger::fmt::style::{AnsiColor, Style};

            let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
            write!(buf, "{subtle}[{subtle:#}")?;
            write!(
                buf,
                "{} ",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z")
            )?;
            let level_style = buf.default_level_style(record.level());
            write!(buf, "{level_style}{:<5}{level_style:#}", record.level())?;
            if let Some(path) = record.module_path() {
                write!(buf, " {path}")?;
            }
            write!(buf, "{subtle}]{subtle:#}")?;
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
    /// URLs can either be `file://` or `zed://` scheme, or relative to <https://zed.dev>.
    paths_or_urls: Vec<String>,

    /// Instructs zed to run as a dev server on this machine. (not implemented)
    #[arg(long)]
    dev_server_token: Option<String>,
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

fn parse_url_arg(arg: &str, cx: &AppContext) -> Result<String> {
    match std::fs::canonicalize(Path::new(&arg)) {
        Ok(path) => Ok(format!(
            "file://{}",
            path.to_string_lossy().trim_start_matches(r#"\\?\"#)
        )),
        Err(error) => {
            if arg.starts_with("file://")
                || arg.starts_with("zed-cli://")
                || arg.starts_with("ssh://")
                || parse_zed_link(arg, cx).is_some()
            {
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
                let themes_dir = paths::themes_dir().as_ref();
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
                cx.update(ThemeSettings::reload_current_theme)?;
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
        let (mut events, _) = fs
            .watch(paths::themes_dir(), Duration::from_millis(100))
            .await;

        while let Some(paths) = events.next().await {
            for event in paths {
                if fs.metadata(&event.path).await.ok().flatten().is_some() {
                    if let Some(theme_registry) =
                        cx.update(|cx| ThemeRegistry::global(cx).clone()).log_err()
                    {
                        if let Some(()) = theme_registry
                            .load_user_theme(&event.path, fs.clone())
                            .await
                            .log_err()
                        {
                            cx.update(ThemeSettings::reload_current_theme).log_err();
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
        let (mut events, _) = fs.watch(path.as_path(), Duration::from_millis(100)).await;
        while let Some(event) = events.next().await {
            let has_language_file = event.iter().any(|event| {
                event
                    .path
                    .extension()
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
        let (mut events, _) = fs.watch(path.as_path(), Duration::from_millis(100)).await;
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
