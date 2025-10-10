mod app_menus;
pub mod component_preview;
pub mod edit_prediction_registry;
#[cfg(target_os = "macos")]
pub(crate) mod mac_only_instance;
mod migrate;
mod open_listener;
mod quick_action_bar;
#[cfg(target_os = "windows")]
pub(crate) mod windows_only_instance;

use agent_ui::{AgentDiffToolbar, AgentPanelDelegate};
use anyhow::Context as _;
pub use app_menus::*;
use assets::Assets;
use audio::{AudioSettings, REPLAY_DURATION};
use breadcrumbs::Breadcrumbs;
use client::zed_urls;
use collections::VecDeque;
use debugger_ui::debugger_panel::DebugPanel;
use editor::ProposedChangesEditorToolbar;
use editor::{Editor, MultiBuffer};
use extension_host::ExtensionStore;
use feature_flags::{FeatureFlagAppExt, PanicFeatureFlag};
use fs::Fs;
use futures::future::Either;
use futures::{StreamExt, channel::mpsc, select_biased};
use git_ui::git_panel::GitPanel;
use git_ui::project_diff::ProjectDiffToolbar;
use gpui::{
    Action, App, AppContext as _, Context, DismissEvent, Element, Entity, Focusable, KeyBinding,
    ParentElement, PathPromptOptions, PromptLevel, ReadGlobal, SharedString, Styled, Task,
    TitlebarOptions, UpdateGlobal, Window, WindowKind, WindowOptions, actions, image_cache, point,
    px, retain_all,
};
use image_viewer::ImageInfo;
use language::Capability;
use language_onboarding::BasedPyrightBanner;
use language_tools::lsp_button::{self, LspButton};
use language_tools::lsp_log_view::LspLogToolbarItemView;
use migrate::{MigrationBanner, MigrationEvent, MigrationNotification, MigrationType};
use migrator::{migrate_keymap, migrate_settings};
use onboarding::DOCS_URL;
use onboarding::multibuffer_hint::MultibufferHint;
pub use open_listener::*;
use outline_panel::OutlinePanel;
use paths::{
    local_debug_file_relative_path, local_settings_file_relative_path,
    local_tasks_file_relative_path,
};
use project::{DirectoryLister, DisableAiSettings, ProjectItem};
use project_panel::ProjectPanel;
use prompt_store::PromptBuilder;
use quick_action_bar::QuickActionBar;
use recent_projects::open_remote_project;
use release_channel::{AppCommitSha, ReleaseChannel};
use rope::Rope;
use search::project_search::ProjectSearchBar;
use settings::{
    BaseKeymap, DEFAULT_KEYMAP_PATH, InvalidSettingsError, KeybindSource, KeymapFile,
    KeymapFileLoadResult, Settings, SettingsStore, VIM_KEYMAP_PATH,
    initial_local_debug_tasks_content, initial_project_settings_content, initial_tasks_content,
    update_settings_file,
};
use std::time::Duration;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
    sync::atomic::{self, AtomicBool},
};
use terminal_view::terminal_panel::{self, TerminalPanel};
use theme::{
    ActiveTheme, GlobalTheme, IconThemeNotFoundError, SystemAppearance, ThemeNotFoundError,
    ThemeRegistry, ThemeSettings,
};
use ui::{PopoverMenuHandle, prelude::*};
use util::markdown::MarkdownString;
use util::rel_path::RelPath;
use util::{ResultExt, asset_str};
use uuid::Uuid;
use vim_mode_setting::VimModeSetting;
use workspace::notifications::{
    NotificationId, SuppressEvent, dismiss_app_notification, show_app_notification,
};
use workspace::{
    AppState, NewFile, NewWindow, OpenLog, Toast, Workspace, WorkspaceSettings,
    create_and_open_local_file, notifications::simple_message_notification::MessageNotification,
    open_new,
};
use workspace::{
    CloseIntent, CloseWindow, NotificationFrame, RestoreBanner, with_active_or_new_workspace,
};
use workspace::{Pane, notifications::DetachAndPromptErr};
use zed_actions::{
    OpenAccountSettings, OpenBrowser, OpenDocs, OpenServerSettings, OpenSettingsFile, OpenZedUrl,
    Quit,
};

actions!(
    zed,
    [
        /// Opens the element inspector for debugging UI.
        DebugElements,
        /// Hides the application window.
        Hide,
        /// Hides all other application windows.
        HideOthers,
        /// Minimizes the current window.
        Minimize,
        /// Opens the default settings file.
        OpenDefaultSettings,
        /// Opens project-specific settings.
        OpenProjectSettings,
        /// Opens the project tasks configuration.
        OpenProjectTasks,
        /// Opens the tasks panel.
        OpenTasks,
        /// Opens debug tasks configuration.
        OpenDebugTasks,
        /// Resets the application database.
        ResetDatabase,
        /// Shows all hidden windows.
        ShowAll,
        /// Toggles fullscreen mode.
        ToggleFullScreen,
        /// Zooms the window.
        Zoom,
        /// Triggers a test panic for debugging.
        TestPanic,
        /// Triggers a hard crash for debugging.
        TestCrash,
    ]
);

actions!(
    dev,
    [
        /// Stores last 30s of audio from zed staff using the experimental rodio
        /// audio system (including yourself) on the current call in a tar file
        /// in the current working directory.
        CaptureRecentAudio,
    ]
);

pub fn init(cx: &mut App) {
    #[cfg(target_os = "macos")]
    cx.on_action(|_: &Hide, cx| cx.hide());
    #[cfg(target_os = "macos")]
    cx.on_action(|_: &HideOthers, cx| cx.hide_other_apps());
    #[cfg(target_os = "macos")]
    cx.on_action(|_: &ShowAll, cx| cx.unhide_other_apps());
    cx.on_action(quit);

    cx.on_action(|_: &RestoreBanner, cx| title_bar::restore_banner(cx));
    let flag = cx.wait_for_flag::<PanicFeatureFlag>();
    cx.spawn(async |cx| {
        if cx
            .update(|cx| ReleaseChannel::global(cx) == ReleaseChannel::Dev)
            .unwrap_or_default()
            || flag.await
        {
            cx.update(|cx| {
                cx.on_action(|_: &TestPanic, _| panic!("Ran the TestPanic action"));
                cx.on_action(|_: &TestCrash, _| {
                    unsafe extern "C" {
                        fn puts(s: *const i8);
                    }
                    unsafe {
                        puts(0xabad1d3a as *const i8);
                    }
                });
            })
            .ok();
        };
    })
    .detach();
    cx.on_action(|_: &OpenLog, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_log_file(workspace, window, cx);
        });
    });
    cx.on_action(|_: &zed_actions::OpenLicenses, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_bundled_file(
                workspace,
                asset_str::<Assets>("licenses.md"),
                "Open Source License Attribution",
                "Markdown",
                window,
                cx,
            );
        });
    });
    cx.on_action(|_: &zed_actions::OpenTelemetryLog, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_telemetry_log_file(workspace, window, cx);
        });
    });
    cx.on_action(|&zed_actions::OpenKeymapFile, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            open_settings_file(
                paths::keymap_file(),
                || settings::initial_keymap_content().as_ref().into(),
                window,
                cx,
            );
        });
    });
    cx.on_action(|_: &OpenSettingsFile, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            open_settings_file(
                paths::settings_file(),
                || settings::initial_user_settings_content().as_ref().into(),
                window,
                cx,
            );
        });
    });
    cx.on_action(|_: &OpenAccountSettings, cx| {
        with_active_or_new_workspace(cx, |_, _, cx| {
            cx.open_url(&zed_urls::account_url(cx));
        });
    });
    cx.on_action(|_: &OpenTasks, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            open_settings_file(
                paths::tasks_file(),
                || settings::initial_tasks_content().as_ref().into(),
                window,
                cx,
            );
        });
    });
    cx.on_action(|_: &OpenDebugTasks, cx| {
        with_active_or_new_workspace(cx, |_, window, cx| {
            open_settings_file(
                paths::debug_scenarios_file(),
                || settings::initial_debug_tasks_content().as_ref().into(),
                window,
                cx,
            );
        });
    });
    cx.on_action(|_: &OpenDefaultSettings, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_bundled_file(
                workspace,
                settings::default_settings(),
                "Default Settings",
                "JSON",
                window,
                cx,
            );
        });
    });
    cx.on_action(|_: &zed_actions::OpenDefaultKeymap, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            open_bundled_file(
                workspace,
                settings::default_keymap(),
                "Default Key Bindings",
                "JSON",
                window,
                cx,
            );
        });
    });
}

fn bind_on_window_closed(cx: &mut App) -> Option<gpui::Subscription> {
    WorkspaceSettings::get_global(cx)
        .on_last_window_closed
        .is_quit_app()
        .then(|| {
            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
        })
}

pub fn build_window_options(display_uuid: Option<Uuid>, cx: &mut App) -> WindowOptions {
    let display = display_uuid.and_then(|uuid| {
        cx.displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    });
    let app_id = ReleaseChannel::global(cx).app_id();
    let window_decorations = match std::env::var("ZED_WINDOW_DECORATIONS") {
        Ok(val) if val == "server" => gpui::WindowDecorations::Server,
        Ok(val) if val == "client" => gpui::WindowDecorations::Client,
        _ => gpui::WindowDecorations::Client,
    };

    let use_system_window_tabs = WorkspaceSettings::get_global(cx).use_system_window_tabs;

    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        window_bounds: None,
        focus: false,
        show: false,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
        window_background: cx.theme().window_background_appearance(),
        app_id: Some(app_id.to_owned()),
        window_decorations: Some(window_decorations),
        window_min_size: Some(gpui::Size {
            width: px(360.0),
            height: px(240.0),
        }),
        tabbing_identifier: if use_system_window_tabs {
            Some(String::from("zed"))
        } else {
            None
        },
        ..Default::default()
    }
}

pub fn initialize_workspace(
    app_state: Arc<AppState>,
    prompt_builder: Arc<PromptBuilder>,
    cx: &mut App,
) {
    let mut _on_close_subscription = bind_on_window_closed(cx);
    cx.observe_global::<SettingsStore>(move |cx| {
        _on_close_subscription = bind_on_window_closed(cx);
    })
    .detach();

    cx.observe_new(move |workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let workspace_handle = cx.entity();
        let center_pane = workspace.active_pane().clone();
        initialize_pane(workspace, &center_pane, window, cx);

        cx.subscribe_in(&workspace_handle, window, {
            move |workspace, _, event, window, cx| match event {
                workspace::Event::PaneAdded(pane) => {
                    initialize_pane(workspace, pane, window, cx);
                }
                workspace::Event::OpenBundledFile {
                    text,
                    title,
                    language,
                } => open_bundled_file(workspace, text.clone(), title, language, window, cx),
                _ => {}
            }
        })
        .detach();

        #[cfg(not(target_os = "macos"))]
        initialize_file_watcher(window, cx);

        if let Some(specs) = window.gpu_specs() {
            log::info!("Using GPU: {:?}", specs);
            show_software_emulation_warning_if_needed(specs.clone(), window, cx);
            if let Some((crash_server, message)) = crashes::CRASH_HANDLER
                .get()
                .zip(bincode::serialize(&specs).ok())
                && let Err(err) = crash_server.send_message(3, message)
            {
                log::warn!(
                    "Failed to store active gpu info for crash reporting: {}",
                    err
                );
            }
        }

        let edit_prediction_menu_handle = PopoverMenuHandle::default();
        let edit_prediction_button = cx.new(|cx| {
            edit_prediction_button::EditPredictionButton::new(
                app_state.fs.clone(),
                app_state.user_store.clone(),
                edit_prediction_menu_handle.clone(),
                cx,
            )
        });
        workspace.register_action({
            move |_, _: &edit_prediction_button::ToggleMenu, window, cx| {
                edit_prediction_menu_handle.toggle(window, cx);
            }
        });

        let search_button = cx.new(|_| search::search_status_button::SearchButton::new());
        let diagnostic_summary =
            cx.new(|cx| diagnostics::items::DiagnosticIndicator::new(workspace, cx));
        let activity_indicator = activity_indicator::ActivityIndicator::new(
            workspace,
            workspace.project().read(cx).languages().clone(),
            window,
            cx,
        );
        let active_buffer_language =
            cx.new(|_| language_selector::ActiveBufferLanguage::new(workspace));
        let active_toolchain_language =
            cx.new(|cx| toolchain_selector::ActiveToolchain::new(workspace, window, cx));
        let vim_mode_indicator = cx.new(|cx| vim::ModeIndicator::new(window, cx));
        let image_info = cx.new(|_cx| ImageInfo::new(workspace));

        let lsp_button_menu_handle = PopoverMenuHandle::default();
        let lsp_button =
            cx.new(|cx| LspButton::new(workspace, lsp_button_menu_handle.clone(), window, cx));
        workspace.register_action({
            move |_, _: &lsp_button::ToggleMenu, window, cx| {
                lsp_button_menu_handle.toggle(window, cx);
            }
        });

        let cursor_position =
            cx.new(|_| go_to_line::cursor_position::CursorPosition::new(workspace));
        workspace.status_bar().update(cx, |status_bar, cx| {
            status_bar.add_left_item(search_button, window, cx);
            status_bar.add_left_item(lsp_button, window, cx);
            status_bar.add_left_item(diagnostic_summary, window, cx);
            status_bar.add_left_item(activity_indicator, window, cx);
            status_bar.add_right_item(edit_prediction_button, window, cx);
            status_bar.add_right_item(active_buffer_language, window, cx);
            status_bar.add_right_item(active_toolchain_language, window, cx);
            status_bar.add_right_item(vim_mode_indicator, window, cx);
            status_bar.add_right_item(cursor_position, window, cx);
            status_bar.add_right_item(image_info, window, cx);
        });

        let handle = cx.entity().downgrade();
        window.on_window_should_close(cx, move |window, cx| {
            handle
                .update(cx, |workspace, cx| {
                    // We'll handle closing asynchronously
                    workspace.close_window(&CloseWindow, window, cx);
                    false
                })
                .unwrap_or(true)
        });

        initialize_panels(prompt_builder.clone(), window, cx);
        register_actions(app_state.clone(), workspace, window, cx);

        workspace.focus_handle(cx).focus(window);
    })
    .detach();
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn initialize_file_watcher(window: &mut Window, cx: &mut Context<Workspace>) {
    if let Err(e) = fs::fs_watcher::global(|_| {}) {
        let message = format!(
            db::indoc! {r#"
            inotify_init returned {}

            This may be due to system-wide limits on inotify instances. For troubleshooting see: https://zed.dev/docs/linux
            "#},
            e
        );
        let prompt = window.prompt(
            PromptLevel::Critical,
            "Could not start inotify",
            Some(&message),
            &["Troubleshoot and Quit"],
            cx,
        );
        cx.spawn(async move |_, cx| {
            if prompt.await == Ok(0) {
                cx.update(|cx| {
                    cx.open_url("https://zed.dev/docs/linux#could-not-start-inotify");
                    cx.quit();
                })
                .ok();
            }
        })
        .detach()
    }
}

#[cfg(target_os = "windows")]
fn initialize_file_watcher(window: &mut Window, cx: &mut Context<Workspace>) {
    if let Err(e) = fs::fs_watcher::global(|_| {}) {
        let message = format!(
            db::indoc! {r#"
            ReadDirectoryChangesW initialization failed: {}

            This may occur on network filesystems and WSL paths. For troubleshooting see: https://zed.dev/docs/windows
            "#},
            e
        );
        let prompt = window.prompt(
            PromptLevel::Critical,
            "Could not start ReadDirectoryChangesW",
            Some(&message),
            &["Troubleshoot and Quit"],
            cx,
        );
        cx.spawn(async move |_, cx| {
            if prompt.await == Ok(0) {
                cx.update(|cx| {
                    cx.open_url("https://zed.dev/docs/windows");
                    cx.quit()
                })
                .ok();
            }
        })
        .detach()
    }
}

fn show_software_emulation_warning_if_needed(
    specs: gpui::GpuSpecs,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if specs.is_software_emulated && std::env::var("ZED_ALLOW_EMULATED_GPU").is_err() {
        let (graphics_api, docs_url, open_url) = if cfg!(target_os = "windows") {
            (
                "DirectX",
                "https://zed.dev/docs/windows",
                "https://zed.dev/docs/windows",
            )
        } else {
            (
                "Vulkan",
                "https://zed.dev/docs/linux",
                "https://zed.dev/docs/linux#zed-fails-to-open-windows",
            )
        };
        let message = format!(
            db::indoc! {r#"
            Zed uses {} for rendering and requires a compatible GPU.

            Currently you are using a software emulated GPU ({}) which
            will result in awful performance.

            For troubleshooting see: {}
            Set ZED_ALLOW_EMULATED_GPU=1 env var to permanently override.
            "#},
            graphics_api, specs.device_name, docs_url
        );
        let prompt = window.prompt(
            PromptLevel::Critical,
            "Unsupported GPU",
            Some(&message),
            &["Skip", "Troubleshoot and Quit"],
            cx,
        );
        cx.spawn(async move |_, cx| {
            if prompt.await == Ok(1) {
                cx.update(|cx| {
                    cx.open_url(open_url);
                    cx.quit();
                })
                .ok();
            }
        })
        .detach()
    }
}

fn initialize_panels(
    prompt_builder: Arc<PromptBuilder>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    cx.spawn_in(window, async move |workspace_handle, cx| {
        let project_panel = ProjectPanel::load(workspace_handle.clone(), cx.clone());
        let outline_panel = OutlinePanel::load(workspace_handle.clone(), cx.clone());
        let terminal_panel = TerminalPanel::load(workspace_handle.clone(), cx.clone());
        let git_panel = GitPanel::load(workspace_handle.clone(), cx.clone());
        let channels_panel =
            collab_ui::collab_panel::CollabPanel::load(workspace_handle.clone(), cx.clone());
        let notification_panel = collab_ui::notification_panel::NotificationPanel::load(
            workspace_handle.clone(),
            cx.clone(),
        );
        let debug_panel = DebugPanel::load(workspace_handle.clone(), cx);

        let (
            project_panel,
            outline_panel,
            terminal_panel,
            git_panel,
            channels_panel,
            notification_panel,
            debug_panel,
        ) = futures::try_join!(
            project_panel,
            outline_panel,
            git_panel,
            terminal_panel,
            channels_panel,
            notification_panel,
            debug_panel,
        )?;

        workspace_handle.update_in(cx, |workspace, window, cx| {
            workspace.add_panel(project_panel, window, cx);
            workspace.add_panel(outline_panel, window, cx);
            workspace.add_panel(terminal_panel, window, cx);
            workspace.add_panel(git_panel, window, cx);
            workspace.add_panel(channels_panel, window, cx);
            workspace.add_panel(notification_panel, window, cx);
            workspace.add_panel(debug_panel, window, cx);
        })?;

        fn setup_or_teardown_agent_panel(
            workspace: &mut Workspace,
            prompt_builder: Arc<PromptBuilder>,
            window: &mut Window,
            cx: &mut Context<Workspace>,
        ) -> Task<anyhow::Result<()>> {
            let disable_ai = SettingsStore::global(cx)
                .get::<DisableAiSettings>(None)
                .disable_ai
                || cfg!(test);
            let existing_panel = workspace.panel::<agent_ui::AgentPanel>(cx);
            match (disable_ai, existing_panel) {
                (false, None) => cx.spawn_in(window, async move |workspace, cx| {
                    let panel =
                        agent_ui::AgentPanel::load(workspace.clone(), prompt_builder, cx.clone())
                            .await?;
                    workspace.update_in(cx, |workspace, window, cx| {
                        let disable_ai = SettingsStore::global(cx)
                            .get::<DisableAiSettings>(None)
                            .disable_ai;
                        let have_panel = workspace.panel::<agent_ui::AgentPanel>(cx).is_some();
                        if !disable_ai && !have_panel {
                            workspace.add_panel(panel, window, cx);
                        }
                    })
                }),
                (true, Some(existing_panel)) => {
                    workspace.remove_panel::<agent_ui::AgentPanel>(&existing_panel, window, cx);
                    Task::ready(Ok(()))
                }
                _ => Task::ready(Ok(())),
            }
        }

        workspace_handle
            .update_in(cx, |workspace, window, cx| {
                setup_or_teardown_agent_panel(workspace, prompt_builder.clone(), window, cx)
            })?
            .await?;

        workspace_handle.update_in(cx, |workspace, window, cx| {
            cx.observe_global_in::<SettingsStore>(window, {
                let prompt_builder = prompt_builder.clone();
                move |workspace, window, cx| {
                    setup_or_teardown_agent_panel(workspace, prompt_builder.clone(), window, cx)
                        .detach_and_log_err(cx);
                }
            })
            .detach();

            // Register the actions that are shared between `assistant` and `assistant2`.
            //
            // We need to do this here instead of within the individual `init`
            // functions so that we only register the actions once.
            //
            // Once we ship `assistant2` we can push this back down into `agent::agent_panel::init`.
            if !cfg!(test) {
                <dyn AgentPanelDelegate>::set_global(
                    Arc::new(agent_ui::ConcreteAssistantPanelDelegate),
                    cx,
                );

                workspace
                    .register_action(agent_ui::AgentPanel::toggle_focus)
                    .register_action(agent_ui::InlineAssistant::inline_assist);
            }
        })?;

        anyhow::Ok(())
    })
    .detach();
}

fn register_actions(
    app_state: Arc<AppState>,
    workspace: &mut Workspace,
    _: &mut Window,
    cx: &mut Context<Workspace>,
) {
    workspace
        .register_action(about)
        .register_action(|_, _: &OpenDocs, _, cx| cx.open_url(DOCS_URL))
        .register_action(|_, _: &Minimize, window, _| {
            window.minimize_window();
        })
        .register_action(|_, _: &Zoom, window, _| {
            window.zoom_window();
        })
        .register_action(|_, _: &ToggleFullScreen, window, _| {
            window.toggle_fullscreen();
        })
        .register_action(|_, action: &OpenZedUrl, _, cx| {
            OpenListener::global(cx).open(RawOpenRequest {
                urls: vec![action.url.clone()],
                ..Default::default()
            })
        })
        .register_action(|_, action: &OpenBrowser, _window, cx| cx.open_url(&action.url))
        .register_action(|workspace, _: &workspace::Open, window, cx| {
            telemetry::event!("Project Opened");
            let paths = workspace.prompt_for_open_path(
                PathPromptOptions {
                    files: true,
                    directories: true,
                    multiple: true,
                    prompt: None,
                },
                DirectoryLister::Local(
                    workspace.project().clone(),
                    workspace.app_state().fs.clone(),
                ),
                window,
                cx,
            );

            cx.spawn_in(window, async move |this, cx| {
                let Some(paths) = paths.await.log_err().flatten() else {
                    return;
                };

                if let Some(task) = this
                    .update_in(cx, |this, window, cx| {
                        this.open_workspace_for_paths(false, paths, window, cx)
                    })
                    .log_err()
                {
                    task.await.log_err();
                }
            })
            .detach()
        })
        .register_action(|workspace, action: &zed_actions::OpenRemote, window, cx| {
            if !action.from_existing_connection {
                cx.propagate();
                return;
            }
            // You need existing remote connection to open it this way
            if workspace.project().read(cx).is_local() {
                return;
            }
            telemetry::event!("Project Opened");
            let paths = workspace.prompt_for_open_path(
                PathPromptOptions {
                    files: true,
                    directories: true,
                    multiple: true,
                    prompt: None,
                },
                DirectoryLister::Project(workspace.project().clone()),
                window,
                cx,
            );
            cx.spawn_in(window, async move |this, cx| {
                let Some(paths) = paths.await.log_err().flatten() else {
                    return;
                };
                if let Some(task) = this
                    .update_in(cx, |this, window, cx| {
                        open_new_ssh_project_from_project(this, paths, window, cx)
                    })
                    .log_err()
                {
                    task.await.log_err();
                }
            })
            .detach()
        })
        .register_action({
            let fs = app_state.fs.clone();
            move |_, action: &zed_actions::IncreaseUiFontSize, _window, cx| {
                if action.persist {
                    update_settings_file(fs.clone(), cx, move |settings, cx| {
                        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx) + px(1.0);
                        let _ = settings
                            .theme
                            .ui_font_size
                            .insert(theme::clamp_font_size(ui_font_size).into());
                    });
                } else {
                    theme::adjust_ui_font_size(cx, |size| size + px(1.0));
                }
            }
        })
        .register_action({
            let fs = app_state.fs.clone();
            move |_, action: &zed_actions::DecreaseUiFontSize, _window, cx| {
                if action.persist {
                    update_settings_file(fs.clone(), cx, move |settings, cx| {
                        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx) - px(1.0);
                        let _ = settings
                            .theme
                            .ui_font_size
                            .insert(theme::clamp_font_size(ui_font_size).into());
                    });
                } else {
                    theme::adjust_ui_font_size(cx, |size| size - px(1.0));
                }
            }
        })
        .register_action({
            let fs = app_state.fs.clone();
            move |_, action: &zed_actions::ResetUiFontSize, _window, cx| {
                if action.persist {
                    update_settings_file(fs.clone(), cx, move |settings, _| {
                        settings.theme.ui_font_size = None;
                    });
                } else {
                    theme::reset_ui_font_size(cx);
                }
            }
        })
        .register_action({
            let fs = app_state.fs.clone();
            move |_, action: &zed_actions::IncreaseBufferFontSize, _window, cx| {
                if action.persist {
                    update_settings_file(fs.clone(), cx, move |settings, cx| {
                        let buffer_font_size =
                            ThemeSettings::get_global(cx).buffer_font_size(cx) + px(1.0);
                        let _ = settings
                            .theme
                            .buffer_font_size
                            .insert(theme::clamp_font_size(buffer_font_size).into());
                    });
                } else {
                    theme::adjust_buffer_font_size(cx, |size| size + px(1.0));
                }
            }
        })
        .register_action({
            let fs = app_state.fs.clone();
            move |_, action: &zed_actions::DecreaseBufferFontSize, _window, cx| {
                if action.persist {
                    update_settings_file(fs.clone(), cx, move |settings, cx| {
                        let buffer_font_size =
                            ThemeSettings::get_global(cx).buffer_font_size(cx) - px(1.0);
                        let _ = settings
                            .theme
                            .buffer_font_size
                            .insert(theme::clamp_font_size(buffer_font_size).into());
                    });
                } else {
                    theme::adjust_buffer_font_size(cx, |size| size - px(1.0));
                }
            }
        })
        .register_action({
            let fs = app_state.fs.clone();
            move |_, action: &zed_actions::ResetBufferFontSize, _window, cx| {
                if action.persist {
                    update_settings_file(fs.clone(), cx, move |settings, _| {
                        settings.theme.buffer_font_size = None;
                    });
                } else {
                    theme::reset_buffer_font_size(cx);
                }
            }
        })
        .register_action(|_, _: &install_cli::RegisterZedScheme, window, cx| {
            cx.spawn_in(window, async move |workspace, cx| {
                install_cli::register_zed_scheme(cx).await?;
                workspace.update_in(cx, |workspace, _, cx| {
                    struct RegisterZedScheme;

                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<RegisterZedScheme>(),
                            format!(
                                "zed:// links will now open in {}.",
                                ReleaseChannel::global(cx).display_name()
                            ),
                        ),
                        cx,
                    )
                })?;
                Ok(())
            })
            .detach_and_prompt_err(
                "Error registering zed:// scheme",
                window,
                cx,
                |_, _, _| None,
            );
        })
        .register_action(open_project_settings_file)
        .register_action(open_project_tasks_file)
        .register_action(open_project_debug_tasks_file)
        .register_action(
            |workspace: &mut Workspace,
             _: &project_panel::ToggleFocus,
             window: &mut Window,
             cx: &mut Context<Workspace>| {
                workspace.toggle_panel_focus::<ProjectPanel>(window, cx);
            },
        )
        .register_action(
            |workspace: &mut Workspace,
             _: &outline_panel::ToggleFocus,
             window: &mut Window,
             cx: &mut Context<Workspace>| {
                workspace.toggle_panel_focus::<OutlinePanel>(window, cx);
            },
        )
        .register_action(
            |workspace: &mut Workspace,
             _: &collab_ui::collab_panel::ToggleFocus,
             window: &mut Window,
             cx: &mut Context<Workspace>| {
                workspace.toggle_panel_focus::<collab_ui::collab_panel::CollabPanel>(window, cx);
            },
        )
        .register_action(
            |workspace: &mut Workspace,
             _: &collab_ui::notification_panel::ToggleFocus,
             window: &mut Window,
             cx: &mut Context<Workspace>| {
                workspace.toggle_panel_focus::<collab_ui::notification_panel::NotificationPanel>(
                    window, cx,
                );
            },
        )
        .register_action(
            |workspace: &mut Workspace,
             _: &terminal_panel::ToggleFocus,
             window: &mut Window,
             cx: &mut Context<Workspace>| {
                workspace.toggle_panel_focus::<TerminalPanel>(window, cx);
            },
        )
        .register_action({
            let app_state = Arc::downgrade(&app_state);
            move |_, _: &NewWindow, _, cx| {
                if let Some(app_state) = app_state.upgrade() {
                    open_new(
                        Default::default(),
                        app_state,
                        cx,
                        |workspace, window, cx| {
                            cx.activate(true);
                            Editor::new_file(workspace, &Default::default(), window, cx)
                        },
                    )
                    .detach();
                }
            }
        })
        .register_action({
            let app_state = Arc::downgrade(&app_state);
            move |_, _: &NewFile, _, cx| {
                if let Some(app_state) = app_state.upgrade() {
                    open_new(
                        Default::default(),
                        app_state,
                        cx,
                        |workspace, window, cx| {
                            Editor::new_file(workspace, &Default::default(), window, cx)
                        },
                    )
                    .detach();
                }
            }
        })
        .register_action(|workspace, _: &CaptureRecentAudio, window, cx| {
            capture_recent_audio(workspace, window, cx);
        });

    #[cfg(not(target_os = "windows"))]
    workspace.register_action(install_cli);

    if workspace.project().read(cx).is_via_remote_server() {
        workspace.register_action({
            move |workspace, _: &OpenServerSettings, window, cx| {
                let open_server_settings = workspace
                    .project()
                    .update(cx, |project, cx| project.open_server_settings(cx));

                cx.spawn_in(window, async move |workspace, cx| {
                    let buffer = open_server_settings.await?;

                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            workspace.open_path(
                                buffer
                                    .read(cx)
                                    .project_path(cx)
                                    .expect("Settings file must have a location"),
                                None,
                                true,
                                window,
                                cx,
                            )
                        })?
                        .await?;

                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
        });
    }
}

fn initialize_pane(
    workspace: &Workspace,
    pane: &Entity<Pane>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    pane.update(cx, |pane, cx| {
        pane.toolbar().update(cx, |toolbar, cx| {
            let multibuffer_hint = cx.new(|_| MultibufferHint::new());
            toolbar.add_item(multibuffer_hint, window, cx);
            let breadcrumbs = cx.new(|_| Breadcrumbs::new());
            toolbar.add_item(breadcrumbs, window, cx);
            let buffer_search_bar = cx.new(|cx| {
                search::BufferSearchBar::new(
                    Some(workspace.project().read(cx).languages().clone()),
                    window,
                    cx,
                )
            });
            toolbar.add_item(buffer_search_bar.clone(), window, cx);
            let proposed_change_bar = cx.new(|_| ProposedChangesEditorToolbar::new());
            toolbar.add_item(proposed_change_bar, window, cx);
            let quick_action_bar =
                cx.new(|cx| QuickActionBar::new(buffer_search_bar, workspace, cx));
            toolbar.add_item(quick_action_bar, window, cx);
            let diagnostic_editor_controls = cx.new(|_| diagnostics::ToolbarControls::new());
            toolbar.add_item(diagnostic_editor_controls, window, cx);
            let project_search_bar = cx.new(|_| ProjectSearchBar::new());
            toolbar.add_item(project_search_bar, window, cx);
            let lsp_log_item = cx.new(|_| LspLogToolbarItemView::new());
            toolbar.add_item(lsp_log_item, window, cx);
            let dap_log_item = cx.new(|_| debugger_tools::DapLogToolbarItemView::new());
            toolbar.add_item(dap_log_item, window, cx);
            let syntax_tree_item = cx.new(|_| language_tools::SyntaxTreeToolbarItemView::new());
            toolbar.add_item(syntax_tree_item, window, cx);
            let migration_banner = cx.new(|cx| MigrationBanner::new(workspace, cx));
            toolbar.add_item(migration_banner, window, cx);
            let project_diff_toolbar = cx.new(|cx| ProjectDiffToolbar::new(workspace, cx));
            toolbar.add_item(project_diff_toolbar, window, cx);
            let agent_diff_toolbar = cx.new(AgentDiffToolbar::new);
            toolbar.add_item(agent_diff_toolbar, window, cx);
            let basedpyright_banner = cx.new(|cx| BasedPyrightBanner::new(workspace, cx));
            toolbar.add_item(basedpyright_banner, window, cx);
        })
    });
}

fn about(
    _: &mut Workspace,
    _: &zed_actions::About,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let release_channel = ReleaseChannel::global(cx).display_name();
    let version = env!("CARGO_PKG_VERSION");
    let debug = if cfg!(debug_assertions) {
        "(debug)"
    } else {
        ""
    };
    let message = format!("{release_channel} {version} {debug}");
    let detail = AppCommitSha::try_global(cx).map(|sha| sha.full());

    let prompt = window.prompt(
        PromptLevel::Info,
        &message,
        detail.as_deref(),
        &["Copy", "OK"],
        cx,
    );
    cx.spawn(async move |_, cx| {
        if let Ok(0) = prompt.await {
            let content = format!("{}\n{}", message, detail.as_deref().unwrap_or(""));
            cx.update(|cx| {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(content));
            })
            .ok();
        }
    })
    .detach();
}

#[cfg(not(target_os = "windows"))]
fn install_cli(
    _: &mut Workspace,
    _: &install_cli::InstallCliBinary,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    install_cli::install_cli_binary(window, cx)
}

static WAITING_QUIT_CONFIRMATION: AtomicBool = AtomicBool::new(false);
fn quit(_: &Quit, cx: &mut App) {
    if WAITING_QUIT_CONFIRMATION.load(atomic::Ordering::Acquire) {
        return;
    }

    let should_confirm = WorkspaceSettings::get_global(cx).confirm_quit;
    cx.spawn(async move |cx| {
        let mut workspace_windows = cx.update(|cx| {
            cx.windows()
                .into_iter()
                .filter_map(|window| window.downcast::<Workspace>())
                .collect::<Vec<_>>()
        })?;

        // If multiple windows have unsaved changes, and need a save prompt,
        // prompt in the active window before switching to a different window.
        cx.update(|cx| {
            workspace_windows.sort_by_key(|window| window.is_active(cx) == Some(false));
        })
        .log_err();

        if should_confirm && let Some(workspace) = workspace_windows.first() {
            let answer = workspace
                .update(cx, |_, window, cx| {
                    window.prompt(
                        PromptLevel::Info,
                        "Are you sure you want to quit?",
                        None,
                        &["Quit", "Cancel"],
                        cx,
                    )
                })
                .log_err();

            if let Some(answer) = answer {
                WAITING_QUIT_CONFIRMATION.store(true, atomic::Ordering::Release);
                let answer = answer.await.ok();
                WAITING_QUIT_CONFIRMATION.store(false, atomic::Ordering::Release);
                if answer != Some(0) {
                    return Ok(());
                }
            }
        }

        // If the user cancels any save prompt, then keep the app open.
        for window in workspace_windows {
            if let Some(should_close) = window
                .update(cx, |workspace, window, cx| {
                    workspace.prepare_to_close(CloseIntent::Quit, window, cx)
                })
                .log_err()
                && !should_close.await?
            {
                return Ok(());
            }
        }
        cx.update(|cx| cx.quit())?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn open_log_file(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    const MAX_LINES: usize = 1000;
    workspace
        .with_local_workspace(window, cx, move |workspace, window, cx| {
            let app_state = workspace.app_state();
            let languages = app_state.languages.clone();
            let fs = app_state.fs.clone();
            cx.spawn_in(window, async move |workspace, cx| {
                let (old_log, new_log, log_language) = futures::join!(
                    fs.load(paths::old_log_file()),
                    fs.load(paths::log_file()),
                    languages.language_for_name("log")
                );
                let log = match (old_log, new_log) {
                    (Err(_), Err(_)) => None,
                    (old_log, new_log) => {
                        let mut lines = VecDeque::with_capacity(MAX_LINES);
                        for line in old_log
                            .iter()
                            .flat_map(|log| log.lines())
                            .chain(new_log.iter().flat_map(|log| log.lines()))
                        {
                            if lines.len() == MAX_LINES {
                                lines.pop_front();
                            }
                            lines.push_back(line);
                        }
                        Some(
                            lines
                                .into_iter()
                                .flat_map(|line| [line, "\n"])
                                .collect::<String>(),
                        )
                    }
                };
                let log_language = log_language.ok();

                workspace
                    .update_in(cx, |workspace, window, cx| {
                        let Some(log) = log else {
                            struct OpenLogError;

                            workspace.show_notification(
                                NotificationId::unique::<OpenLogError>(),
                                cx,
                                |cx| {
                                    cx.new(|cx| {
                                        MessageNotification::new(
                                            format!(
                                                "Unable to access/open log file at path {:?}",
                                                paths::log_file().as_path()
                                            ),
                                            cx,
                                        )
                                    })
                                },
                            );
                            return;
                        };
                        let project = workspace.project().clone();
                        let buffer = project.update(cx, |project, cx| {
                            project.create_local_buffer(&log, log_language, false, cx)
                        });

                        let buffer = cx
                            .new(|cx| MultiBuffer::singleton(buffer, cx).with_title("Log".into()));
                        let editor = cx.new(|cx| {
                            let mut editor =
                                Editor::for_multibuffer(buffer, Some(project), window, cx);
                            editor.set_read_only(true);
                            editor.set_breadcrumb_header(format!(
                                "Last {} lines in {}",
                                MAX_LINES,
                                paths::log_file().display()
                            ));
                            editor
                        });

                        editor.update(cx, |editor, cx| {
                            let last_multi_buffer_offset = editor.buffer().read(cx).len(cx);
                            editor.change_selections(Default::default(), window, cx, |s| {
                                s.select_ranges(Some(
                                    last_multi_buffer_offset..last_multi_buffer_offset,
                                ));
                            })
                        });

                        workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
                    })
                    .log_err();
            })
            .detach();
        })
        .detach();
}

pub fn handle_settings_file_changes(
    mut user_settings_file_rx: mpsc::UnboundedReceiver<String>,
    mut global_settings_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut App,
    settings_changed: impl Fn(Option<anyhow::Error>, &mut App) + 'static,
) {
    MigrationNotification::set_global(cx.new(|_| MigrationNotification), cx);

    // Helper function to process settings content
    let process_settings = move |content: String,
                                 is_user: bool,
                                 store: &mut SettingsStore,
                                 cx: &mut App|
          -> bool {
        let id = NotificationId::Named("failed-to-migrate-settings".into());
        // Apply migrations to both user and global settings
        let (processed_content, content_migrated) = match migrate_settings(&content) {
            Ok(result) => {
                dismiss_app_notification(&id, cx);
                if let Some(migrated_content) = result {
                    (migrated_content, true)
                } else {
                    (content, false)
                }
            }
            Err(err) => {
                show_app_notification(id, cx, move |cx| {
                    cx.new(|cx| {
                        MessageNotification::new(
                            format!(
                                "Failed to migrate settings\n\
                                    {err}"
                            ),
                            cx,
                        )
                        .primary_message("Open Settings File")
                        .primary_icon(IconName::Settings)
                        .primary_on_click(|window, cx| {
                            window.dispatch_action(zed_actions::OpenSettingsFile.boxed_clone(), cx);
                            cx.emit(DismissEvent);
                        })
                    })
                });
                // notify user here
                (content, false)
            }
        };

        let result = if is_user {
            store.set_user_settings(&processed_content, cx)
        } else {
            store.set_global_settings(&processed_content, cx)
        };

        if let Err(err) = &result {
            let settings_type = if is_user { "user" } else { "global" };
            log::error!("Failed to load {} settings: {err}", settings_type);
        }

        settings_changed(result.err(), cx);

        content_migrated
    };

    // Initial load of both settings files
    let global_content = cx
        .background_executor()
        .block(global_settings_file_rx.next())
        .unwrap();
    let user_content = cx
        .background_executor()
        .block(user_settings_file_rx.next())
        .unwrap();

    SettingsStore::update_global(cx, |store, cx| {
        process_settings(global_content, false, store, cx);
        process_settings(user_content, true, store, cx);
    });

    // Watch for changes in both files
    cx.spawn(async move |cx| {
        let mut settings_streams = futures::stream::select(
            global_settings_file_rx.map(Either::Left),
            user_settings_file_rx.map(Either::Right),
        );

        while let Some(content) = settings_streams.next().await {
            let (content, is_user) = match content {
                Either::Left(content) => (content, false),
                Either::Right(content) => (content, true),
            };

            let result = cx.update_global(|store: &mut SettingsStore, cx| {
                let migrating_in_memory = process_settings(content, is_user, store, cx);
                if let Some(notifier) = MigrationNotification::try_global(cx) {
                    notifier.update(cx, |_, cx| {
                        cx.emit(MigrationEvent::ContentChanged {
                            migration_type: MigrationType::Settings,
                            migrating_in_memory,
                        });
                    });
                }
                cx.refresh_windows();
            });

            if result.is_err() {
                break; // App dropped
            }
        }
    })
    .detach();
}

pub fn handle_keymap_file_changes(
    mut user_keymap_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut App,
) {
    BaseKeymap::register(cx);
    vim_mode_setting::init(cx);

    let (base_keymap_tx, mut base_keymap_rx) = mpsc::unbounded();
    let (keyboard_layout_tx, mut keyboard_layout_rx) = mpsc::unbounded();
    let mut old_base_keymap = *BaseKeymap::get_global(cx);
    let mut old_vim_enabled = VimModeSetting::get_global(cx).0;
    let mut old_helix_enabled = vim_mode_setting::HelixModeSetting::get_global(cx).0;

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_base_keymap = *BaseKeymap::get_global(cx);
        let new_vim_enabled = VimModeSetting::get_global(cx).0;
        let new_helix_enabled = vim_mode_setting::HelixModeSetting::get_global(cx).0;

        if new_base_keymap != old_base_keymap
            || new_vim_enabled != old_vim_enabled
            || new_helix_enabled != old_helix_enabled
        {
            old_base_keymap = new_base_keymap;
            old_vim_enabled = new_vim_enabled;
            old_helix_enabled = new_helix_enabled;

            base_keymap_tx.unbounded_send(()).unwrap();
        }
    })
    .detach();

    #[cfg(target_os = "windows")]
    {
        let mut current_layout_id = cx.keyboard_layout().id().to_string();
        cx.on_keyboard_layout_change(move |cx| {
            let next_layout_id = cx.keyboard_layout().id();
            if next_layout_id != current_layout_id {
                current_layout_id = next_layout_id.to_string();
                keyboard_layout_tx.unbounded_send(()).ok();
            }
        })
        .detach();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut current_mapping = cx.keyboard_mapper().get_key_equivalents().cloned();
        cx.on_keyboard_layout_change(move |cx| {
            let next_mapping = cx.keyboard_mapper().get_key_equivalents();
            if current_mapping.as_ref() != next_mapping {
                current_mapping = next_mapping.cloned();
                keyboard_layout_tx.unbounded_send(()).ok();
            }
        })
        .detach();
    }

    load_default_keymap(cx);

    struct KeymapParseErrorNotification;
    let notification_id = NotificationId::unique::<KeymapParseErrorNotification>();

    cx.spawn(async move |cx| {
        let mut user_keymap_content = String::new();
        let mut migrating_in_memory = false;
        loop {
            select_biased! {
                _ = base_keymap_rx.next() => {},
                _ = keyboard_layout_rx.next() => {},
                content = user_keymap_file_rx.next() => {
                    if let Some(content) = content {
                        if let Ok(Some(migrated_content)) = migrate_keymap(&content) {
                            user_keymap_content = migrated_content;
                            migrating_in_memory = true;
                        } else {
                            user_keymap_content = content;
                            migrating_in_memory = false;
                        }
                    }
                }
            };
            cx.update(|cx| {
                if let Some(notifier) = MigrationNotification::try_global(cx) {
                    notifier.update(cx, |_, cx| {
                        cx.emit(MigrationEvent::ContentChanged {
                            migration_type: MigrationType::Keymap,
                            migrating_in_memory,
                        });
                    });
                }
                let load_result = KeymapFile::load(&user_keymap_content, cx);
                match load_result {
                    KeymapFileLoadResult::Success { key_bindings } => {
                        reload_keymaps(cx, key_bindings);
                        dismiss_app_notification(&notification_id.clone(), cx);
                    }
                    KeymapFileLoadResult::SomeFailedToLoad {
                        key_bindings,
                        error_message,
                    } => {
                        if !key_bindings.is_empty() {
                            reload_keymaps(cx, key_bindings);
                        }
                        show_keymap_file_load_error(notification_id.clone(), error_message, cx);
                    }
                    KeymapFileLoadResult::JsonParseFailure { error } => {
                        show_keymap_file_json_error(notification_id.clone(), &error, cx)
                    }
                }
            })
            .ok();
        }
    })
    .detach();
}

fn show_keymap_file_json_error(
    notification_id: NotificationId,
    error: &anyhow::Error,
    cx: &mut App,
) {
    let message: SharedString =
        format!("JSON parse error in keymap file. Bindings not reloaded.\n\n{error}").into();
    show_app_notification(notification_id, cx, move |cx| {
        cx.new(|cx| {
            MessageNotification::new(message.clone(), cx)
                .primary_message("Open Keymap File")
                .primary_on_click(|window, cx| {
                    window.dispatch_action(zed_actions::OpenKeymapFile.boxed_clone(), cx);
                    cx.emit(DismissEvent);
                })
        })
    });
}

fn show_keymap_file_load_error(
    notification_id: NotificationId,
    error_message: MarkdownString,
    cx: &mut App,
) {
    show_markdown_app_notification(
        notification_id,
        error_message,
        "Open Keymap File".into(),
        |window, cx| {
            window.dispatch_action(zed_actions::OpenKeymapFile.boxed_clone(), cx);
            cx.emit(DismissEvent);
        },
        cx,
    )
}

fn show_markdown_app_notification<F>(
    notification_id: NotificationId,
    message: MarkdownString,
    primary_button_message: SharedString,
    primary_button_on_click: F,
    cx: &mut App,
) where
    F: 'static + Send + Sync + Fn(&mut Window, &mut Context<MessageNotification>),
{
    let parsed_markdown = cx.background_spawn(async move {
        let file_location_directory = None;
        let language_registry = None;
        markdown_preview::markdown_parser::parse_markdown(
            &message.0,
            file_location_directory,
            language_registry,
        )
        .await
    });

    cx.spawn(async move |cx| {
        let parsed_markdown = Arc::new(parsed_markdown.await);
        let primary_button_message = primary_button_message.clone();
        let primary_button_on_click = Arc::new(primary_button_on_click);
        cx.update(|cx| {
            show_app_notification(notification_id, cx, move |cx| {
                let workspace_handle = cx.entity().downgrade();
                let parsed_markdown = parsed_markdown.clone();
                let primary_button_message = primary_button_message.clone();
                let primary_button_on_click = primary_button_on_click.clone();
                cx.new(move |cx| {
                    MessageNotification::new_from_builder(cx, move |window, cx| {
                        image_cache(retain_all("notification-cache"))
                            .text_xs()
                            .child(markdown_preview::markdown_renderer::render_parsed_markdown(
                                &parsed_markdown.clone(),
                                Some(workspace_handle.clone()),
                                window,
                                cx,
                            ))
                            .into_any()
                    })
                    .primary_message(primary_button_message)
                    .primary_on_click_arc(primary_button_on_click)
                })
            })
        })
        .ok();
    })
    .detach();
}

fn reload_keymaps(cx: &mut App, mut user_key_bindings: Vec<KeyBinding>) {
    cx.clear_key_bindings();
    load_default_keymap(cx);

    for key_binding in &mut user_key_bindings {
        key_binding.set_meta(KeybindSource::User.meta());
    }
    cx.bind_keys(user_key_bindings);

    let menus = app_menus(cx);
    cx.set_menus(menus);
    // On Windows, this is set in the `update_jump_list` method of the `HistoryManager`.
    #[cfg(not(target_os = "windows"))]
    cx.set_dock_menu(vec![gpui::MenuItem::action(
        "New Window",
        workspace::NewWindow,
    )]);
    // todo: nicer api here?
    keymap_editor::KeymapEventChannel::trigger_keymap_changed(cx);
}

pub fn load_default_keymap(cx: &mut App) {
    let base_keymap = *BaseKeymap::get_global(cx);
    if base_keymap == BaseKeymap::None {
        return;
    }

    cx.bind_keys(
        KeymapFile::load_asset(DEFAULT_KEYMAP_PATH, Some(KeybindSource::Default), cx).unwrap(),
    );

    if let Some(asset_path) = base_keymap.asset_path() {
        cx.bind_keys(KeymapFile::load_asset(asset_path, Some(KeybindSource::Base), cx).unwrap());
    }

    if VimModeSetting::get_global(cx).0 || vim_mode_setting::HelixModeSetting::get_global(cx).0 {
        cx.bind_keys(
            KeymapFile::load_asset(VIM_KEYMAP_PATH, Some(KeybindSource::Vim), cx).unwrap(),
        );
    }
}

pub fn handle_settings_changed(error: Option<anyhow::Error>, cx: &mut App) {
    struct SettingsParseErrorNotification;
    let id = NotificationId::unique::<SettingsParseErrorNotification>();

    match error {
        Some(error) => {
            if let Some(InvalidSettingsError::LocalSettings { .. }) =
                error.downcast_ref::<InvalidSettingsError>()
            {
                // Local settings errors are displayed by the projects
                return;
            }
            show_app_notification(id, cx, move |cx| {
                cx.new(|cx| {
                    MessageNotification::new(format!("Invalid user settings file\n{error}"), cx)
                        .primary_message("Open Settings File")
                        .primary_icon(IconName::Settings)
                        .primary_on_click(|window, cx| {
                            window.dispatch_action(zed_actions::OpenSettingsFile.boxed_clone(), cx);
                            cx.emit(DismissEvent);
                        })
                })
            });
        }
        None => {
            dismiss_app_notification(&id, cx);
        }
    }
}

pub fn open_new_ssh_project_from_project(
    workspace: &mut Workspace,
    paths: Vec<PathBuf>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Task<anyhow::Result<()>> {
    let app_state = workspace.app_state().clone();
    let Some(ssh_client) = workspace.project().read(cx).remote_client() else {
        return Task::ready(Err(anyhow::anyhow!("Not an ssh project")));
    };
    let connection_options = ssh_client.read(cx).connection_options();
    cx.spawn_in(window, async move |_, cx| {
        open_remote_project(
            connection_options,
            paths,
            app_state,
            workspace::OpenOptions {
                open_new_workspace: Some(true),
                ..Default::default()
            },
            cx,
        )
        .await
    })
}

fn open_project_settings_file(
    workspace: &mut Workspace,
    _: &OpenProjectSettings,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open_local_file(
        workspace,
        local_settings_file_relative_path(),
        initial_project_settings_content(),
        window,
        cx,
    )
}

fn open_project_tasks_file(
    workspace: &mut Workspace,
    _: &OpenProjectTasks,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open_local_file(
        workspace,
        local_tasks_file_relative_path(),
        initial_tasks_content(),
        window,
        cx,
    )
}

fn open_project_debug_tasks_file(
    workspace: &mut Workspace,
    _: &zed_actions::OpenProjectDebugTasks,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open_local_file(
        workspace,
        local_debug_file_relative_path(),
        initial_local_debug_tasks_content(),
        window,
        cx,
    )
}

fn open_local_file(
    workspace: &mut Workspace,
    settings_relative_path: &'static RelPath,
    initial_contents: Cow<'static, str>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let project = workspace.project().clone();
    let worktree = project
        .read(cx)
        .visible_worktrees(cx)
        .find_map(|tree| tree.read(cx).root_entry()?.is_dir().then_some(tree));
    if let Some(worktree) = worktree {
        let tree_id = worktree.read(cx).id();
        cx.spawn_in(window, async move |workspace, cx| {
            // Check if the file actually exists on disk (even if it's excluded from worktree)
            let file_exists = {
                let full_path = worktree.read_with(cx, |tree, _| {
                    tree.abs_path().join(settings_relative_path.as_std_path())
                })?;

                let fs = project.read_with(cx, |project, _| project.fs().clone())?;

                fs.metadata(&full_path)
                    .await
                    .ok()
                    .flatten()
                    .is_some_and(|metadata| !metadata.is_dir && !metadata.is_fifo)
            };

            if !file_exists {
                if let Some(dir_path) = settings_relative_path.parent()
                    && worktree.read_with(cx, |tree, _| tree.entry_for_path(dir_path).is_none())?
                {
                    project
                        .update(cx, |project, cx| {
                            project.create_entry((tree_id, dir_path), true, cx)
                        })?
                        .await
                        .context("worktree was removed")?;
                }

                if worktree.read_with(cx, |tree, _| {
                    tree.entry_for_path(settings_relative_path).is_none()
                })? {
                    project
                        .update(cx, |project, cx| {
                            project.create_entry((tree_id, settings_relative_path), false, cx)
                        })?
                        .await
                        .context("worktree was removed")?;
                }
            }

            let editor = workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.open_path((tree_id, settings_relative_path), None, true, window, cx)
                })?
                .await?
                .downcast::<Editor>()
                .context("unexpected item type: expected editor item")?;

            editor
                .downgrade()
                .update(cx, |editor, cx| {
                    if let Some(buffer) = editor.buffer().read(cx).as_singleton()
                        && buffer.read(cx).is_empty()
                    {
                        buffer.update(cx, |buffer, cx| {
                            buffer.edit([(0..0, initial_contents)], None, cx)
                        });
                    }
                })
                .ok();

            anyhow::Ok(())
        })
        .detach();
    } else {
        struct NoOpenFolders;

        workspace.show_notification(NotificationId::unique::<NoOpenFolders>(), cx, |cx| {
            cx.new(|cx| MessageNotification::new("This project has no folders open.", cx))
        })
    }
}

fn open_telemetry_log_file(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    workspace.with_local_workspace(window, cx, move |workspace, window, cx| {
        let app_state = workspace.app_state().clone();
        cx.spawn_in(window, async move |workspace, cx| {
            async fn fetch_log_string(app_state: &Arc<AppState>) -> Option<String> {
                let path = client::telemetry::Telemetry::log_file_path();
                app_state.fs.load(&path).await.log_err()
            }

            let log = fetch_log_string(&app_state).await.unwrap_or_else(|| "// No data has been collected yet".to_string());

            const MAX_TELEMETRY_LOG_LEN: usize = 5 * 1024 * 1024;
            let mut start_offset = log.len().saturating_sub(MAX_TELEMETRY_LOG_LEN);
            if let Some(newline_offset) = log[start_offset..].find('\n') {
                start_offset += newline_offset + 1;
            }
            let log_suffix = &log[start_offset..];
            let header = concat!(
                "// Zed collects anonymous usage data to help us understand how people are using the app.\n",
                "// Telemetry can be disabled via the `settings.json` file.\n",
                "// Here is the data that has been reported for the current session:\n",
            );
            let content = format!("{}\n{}", header, log_suffix);
            let json = app_state.languages.language_for_name("JSON").await.log_err();

            workspace.update_in( cx, |workspace, window, cx| {
                let project = workspace.project().clone();
                let buffer = project.update(cx, |project, cx| project.create_local_buffer(&content, json,false, cx));
                let buffer = cx.new(|cx| {
                    MultiBuffer::singleton(buffer, cx).with_title("Telemetry Log".into())
                });
                workspace.add_item_to_active_pane(
                    Box::new(cx.new(|cx| {
                        let mut editor = Editor::for_multibuffer(buffer, Some(project), window, cx);
                        editor.set_read_only(true);
                        editor.set_breadcrumb_header("Telemetry Log".into());
                        editor
                    })),
                    None,
                    true,
                    window, cx,
                );
            }).log_err()?;

            Some(())
        })
        .detach();
    }).detach();
}

fn open_bundled_file(
    workspace: &Workspace,
    text: Cow<'static, str>,
    title: &'static str,
    language: &'static str,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let language = workspace.app_state().languages.language_for_name(language);
    cx.spawn_in(window, async move |workspace, cx| {
        let language = language.await.log_err();
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.with_local_workspace(window, cx, |workspace, window, cx| {
                    let project = workspace.project();
                    let buffer = project.update(cx, move |project, cx| {
                        let buffer =
                            project.create_local_buffer(text.as_ref(), language, false, cx);
                        buffer.update(cx, |buffer, cx| {
                            buffer.set_capability(Capability::ReadOnly, cx);
                        });
                        buffer
                    });
                    let buffer =
                        cx.new(|cx| MultiBuffer::singleton(buffer, cx).with_title(title.into()));
                    workspace.add_item_to_active_pane(
                        Box::new(cx.new(|cx| {
                            let mut editor =
                                Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                            editor.set_read_only(true);
                            editor.set_breadcrumb_header(title.into());
                            editor
                        })),
                        None,
                        true,
                        window,
                        cx,
                    );
                })
            })?
            .await
    })
    .detach_and_log_err(cx);
}

fn open_settings_file(
    abs_path: &'static Path,
    default_content: impl FnOnce() -> Rope + Send + 'static,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    cx.spawn_in(window, async move |workspace, cx| {
        let (worktree_creation_task, settings_open_task) = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.with_local_workspace(window, cx, move |workspace, window, cx| {
                    let worktree_creation_task = workspace.project().update(cx, |project, cx| {
                        // Set up a dedicated worktree for settings, since
                        // otherwise we're dropping and re-starting LSP servers
                        // for each file inside on every settings file
                        // close/open

                        // TODO: Do note that all other external files (e.g.
                        // drag and drop from OS) still have their worktrees
                        // released on file close, causing LSP servers'
                        // restarts.
                        project.find_or_create_worktree(paths::config_dir().as_path(), false, cx)
                    });
                    let settings_open_task =
                        create_and_open_local_file(abs_path, window, cx, default_content);
                    (worktree_creation_task, settings_open_task)
                })
            })?
            .await?;
        let _ = worktree_creation_task.await?;
        let _ = settings_open_task.await?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn capture_recent_audio(workspace: &mut Workspace, _: &mut Window, cx: &mut Context<Workspace>) {
    struct CaptureRecentAudioNotification {
        focus_handle: gpui::FocusHandle,
        save_result: Option<Result<(PathBuf, Duration), anyhow::Error>>,
        _save_task: Task<anyhow::Result<()>>,
    }

    impl gpui::EventEmitter<DismissEvent> for CaptureRecentAudioNotification {}
    impl gpui::EventEmitter<SuppressEvent> for CaptureRecentAudioNotification {}
    impl gpui::Focusable for CaptureRecentAudioNotification {
        fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
            self.focus_handle.clone()
        }
    }
    impl workspace::notifications::Notification for CaptureRecentAudioNotification {}

    impl Render for CaptureRecentAudioNotification {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let message = match &self.save_result {
                None => format!(
                    "Saving up to {} seconds of recent audio",
                    REPLAY_DURATION.as_secs(),
                ),
                Some(Ok((path, duration))) => format!(
                    "Saved {} seconds of all audio to {}",
                    duration.as_secs(),
                    path.display(),
                ),
                Some(Err(e)) => format!("Error saving audio replays: {e:?}"),
            };

            NotificationFrame::new()
                .with_title(Some("Saved Audio"))
                .show_suppress_button(false)
                .on_close(cx.listener(|_, _, _, cx| {
                    cx.emit(DismissEvent);
                }))
                .with_content(message)
        }
    }

    impl CaptureRecentAudioNotification {
        fn new(cx: &mut Context<Self>) -> Self {
            if AudioSettings::get_global(cx).rodio_audio {
                let executor = cx.background_executor().clone();
                let save_task = cx.default_global::<audio::Audio>().save_replays(executor);
                let _save_task = cx.spawn(async move |this, cx| {
                    let res = save_task.await;
                    this.update(cx, |this, cx| {
                        this.save_result = Some(res);
                        cx.notify();
                    })
                });

                Self {
                    focus_handle: cx.focus_handle(),
                    _save_task,
                    save_result: None,
                }
            } else {
                Self {
                    focus_handle: cx.focus_handle(),
                    _save_task: Task::ready(Ok(())),
                    save_result: Some(Err(anyhow::anyhow!(
                        "Capturing recent audio is only supported on the experimental rodio audio pipeline"
                    ))),
                }
            }
        }
    }

    workspace.show_notification(
        NotificationId::unique::<CaptureRecentAudioNotification>(),
        cx,
        |cx| cx.new(CaptureRecentAudioNotification::new),
    );
}

/// Eagerly loads the active theme and icon theme based on the selections in the
/// theme settings.
///
/// This fast path exists to load these themes as soon as possible so the user
/// doesn't see the default themes while waiting on extensions to load.
pub(crate) fn eager_load_active_theme_and_icon_theme(fs: Arc<dyn Fs>, cx: &mut App) {
    let extension_store = ExtensionStore::global(cx);
    let theme_registry = ThemeRegistry::global(cx);
    let theme_settings = ThemeSettings::get_global(cx);
    let appearance = SystemAppearance::global(cx).0;

    let theme_name = theme_settings.theme.name(appearance);
    if matches!(
        theme_registry.get(&theme_name.0),
        Err(ThemeNotFoundError(_))
    ) && let Some(theme_path) = extension_store
        .read(cx)
        .path_to_extension_theme(&theme_name.0)
    {
        if cx
            .background_executor()
            .block(theme_registry.load_user_theme(&theme_path, fs.clone()))
            .log_err()
            .is_some()
        {
            GlobalTheme::reload_theme(cx);
        }
    }

    let theme_settings = ThemeSettings::get_global(cx);
    let icon_theme_name = theme_settings.icon_theme.name(appearance);
    if matches!(
        theme_registry.get_icon_theme(&icon_theme_name.0),
        Err(IconThemeNotFoundError(_))
    ) && let Some((icon_theme_path, icons_root_path)) = extension_store
        .read(cx)
        .path_to_extension_icon_theme(&icon_theme_name.0)
    {
        if cx
            .background_executor()
            .block(theme_registry.load_icon_theme(&icon_theme_path, &icons_root_path, fs))
            .log_err()
            .is_some()
        {
            GlobalTheme::reload_icon_theme(cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assets::Assets;
    use collections::HashSet;
    use editor::{DisplayPoint, Editor, SelectionEffects, display_map::DisplayRow};
    use gpui::{
        Action, AnyWindowHandle, App, AssetSource, BorrowAppContext, SemanticVersion,
        TestAppContext, UpdateGlobal, VisualTestContext, WindowHandle, actions,
    };
    use language::{LanguageMatcher, LanguageRegistry};
    use pretty_assertions::{assert_eq, assert_ne};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use settings::{SettingsStore, watch_config_file};
    use std::{
        path::{Path, PathBuf},
        time::Duration,
    };
    use theme::ThemeRegistry;
    use util::{
        path,
        rel_path::{RelPath, rel_path},
    };
    use workspace::{
        NewFile, OpenOptions, OpenVisible, SERIALIZATION_THROTTLE_TIME, SaveIntent, SplitDirection,
        WorkspaceHandle,
        item::SaveOptions,
        item::{Item, ItemHandle},
        open_new, open_paths, pane,
    };

    #[gpui::test]
    async fn test_open_non_existing_file(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "a": {
                    },
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/a/new"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.read(|cx| cx.windows().len()), 1);

        let workspace = cx.windows()[0].downcast::<Workspace>().unwrap();
        workspace
            .update(cx, |workspace, _, cx| {
                assert!(workspace.active_item_as::<Editor>(cx).is_some())
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_paths_action(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "a": {
                        "aa": null,
                        "ab": null,
                    },
                    "b": {
                        "ba": null,
                        "bb": null,
                    },
                    "c": {
                        "ca": null,
                        "cb": null,
                    },
                    "d": {
                        "da": null,
                        "db": null,
                    },
                    "e": {
                        "ea": null,
                        "eb": null,
                    }
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/a"), PathBuf::from("/root/b")],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.read(|cx| cx.windows().len()), 1);

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/a")],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.read(|cx| cx.windows().len()), 1);
        let workspace_1 = cx
            .read(|cx| cx.windows()[0].downcast::<Workspace>())
            .unwrap();
        cx.run_until_parked();
        workspace_1
            .update(cx, |workspace, window, cx| {
                assert_eq!(workspace.worktrees(cx).count(), 2);
                assert!(workspace.left_dock().read(cx).is_open());
                assert!(
                    workspace
                        .active_pane()
                        .read(cx)
                        .focus_handle(cx)
                        .is_focused(window)
                );
            })
            .unwrap();

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/c"), PathBuf::from("/root/d")],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.read(|cx| cx.windows().len()), 2);

        // Replace existing windows
        let window = cx
            .update(|cx| cx.windows()[0].downcast::<Workspace>())
            .unwrap();
        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/e")],
                app_state,
                workspace::OpenOptions {
                    replace_window: Some(window),
                    ..Default::default()
                },
                cx,
            )
        })
        .await
        .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(cx.read(|cx| cx.windows().len()), 2);
        let workspace_1 = cx
            .update(|cx| cx.windows()[0].downcast::<Workspace>())
            .unwrap();
        workspace_1
            .update(cx, |workspace, window, cx| {
                assert_eq!(
                    workspace
                        .worktrees(cx)
                        .map(|w| w.read(cx).abs_path())
                        .collect::<Vec<_>>(),
                    &[Path::new("/root/e").into()]
                );
                assert!(workspace.left_dock().read(cx).is_open());
                assert!(workspace.active_pane().focus_handle(cx).is_focused(window));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_add_new(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({"a": "hey", "b": "", "dir": {"c": "f"}}),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/dir"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/a"))],
                app_state.clone(),
                workspace::OpenOptions {
                    open_new_workspace: Some(false),
                    ..Default::default()
                },
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/dir/c"))],
                app_state.clone(),
                workspace::OpenOptions {
                    open_new_workspace: Some(true),
                    ..Default::default()
                },
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 2);
    }

    #[gpui::test]
    async fn test_open_file_in_many_spaces(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({"dir1": {"a": "b"}, "dir2": {"c": "d"}}),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/dir1/a"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);
        let window1 = cx.update(|cx| cx.active_window().unwrap());

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/dir2/c"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/dir2"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 2);
        let window2 = cx.update(|cx| cx.active_window().unwrap());
        assert!(window1 != window2);
        cx.update_window(window1, |_, window, _| window.activate_window())
            .unwrap();

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/dir2/c"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 2);
        // should have opened in window2 because that has dir2 visibly open (window1 has it open, but not in the project panel)
        assert!(cx.update(|cx| cx.active_window().unwrap()) == window2);
    }

    #[gpui::test]
    async fn test_window_edit_state_restoring_disabled(cx: &mut TestAppContext) {
        let executor = cx.executor();
        let app_state = init_test(cx);

        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .session
                        .get_or_insert_default()
                        .restore_unsaved_buffers = Some(false)
                });
            });
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/root"), json!({"a": "hey"}))
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/a"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);

        // When opening the workspace, the window is not in a edited state.
        let window = cx.update(|cx| cx.windows()[0].downcast::<Workspace>().unwrap());

        let window_is_edited = |window: WindowHandle<Workspace>, cx: &mut TestAppContext| {
            cx.update(|cx| window.read(cx).unwrap().is_edited())
        };
        let pane = window
            .read_with(cx, |workspace, _| workspace.active_pane().clone())
            .unwrap();
        let editor = window
            .read_with(cx, |workspace, cx| {
                workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap()
            })
            .unwrap();

        assert!(!window_is_edited(window, cx));

        // Editing a buffer marks the window as edited.
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| editor.insert("EDIT", window, cx));
            })
            .unwrap();

        assert!(window_is_edited(window, cx));

        // Undoing the edit restores the window's edited state.
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.undo(&Default::default(), window, cx)
                });
            })
            .unwrap();
        assert!(!window_is_edited(window, cx));

        // Redoing the edit marks the window as edited again.
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.redo(&Default::default(), window, cx)
                });
            })
            .unwrap();
        assert!(window_is_edited(window, cx));
        let weak = editor.downgrade();

        // Closing the item restores the window's edited state.
        let close = window
            .update(cx, |_, window, cx| {
                pane.update(cx, |pane, cx| {
                    drop(editor);
                    pane.close_active_item(&Default::default(), window, cx)
                })
            })
            .unwrap();
        executor.run_until_parked();

        cx.simulate_prompt_answer("Don't Save");
        close.await.unwrap();

        // Advance the clock to ensure that the item has been serialized and dropped from the queue
        cx.executor().advance_clock(Duration::from_secs(1));

        weak.assert_released();
        assert!(!window_is_edited(window, cx));
        // Opening the buffer again doesn't impact the window's edited state.
        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/a"))],
                app_state,
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        executor.run_until_parked();

        window
            .update(cx, |workspace, _, cx| {
                let editor = workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap();

                editor.update(cx, |editor, cx| {
                    assert_eq!(editor.text(cx), "hey");
                });
            })
            .unwrap();

        let editor = window
            .read_with(cx, |workspace, cx| {
                workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap()
            })
            .unwrap();
        assert!(!window_is_edited(window, cx));

        // Editing the buffer marks the window as edited.
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| editor.insert("EDIT", window, cx));
            })
            .unwrap();
        executor.run_until_parked();
        assert!(window_is_edited(window, cx));

        // Ensure closing the window via the mouse gets preempted due to the
        // buffer having unsaved changes.
        assert!(!VisualTestContext::from_window(window.into(), cx).simulate_close());
        executor.run_until_parked();
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);

        // The window is successfully closed after the user dismisses the prompt.
        cx.simulate_prompt_answer("Don't Save");
        executor.run_until_parked();
        assert_eq!(cx.update(|cx| cx.windows().len()), 0);
    }

    #[gpui::test]
    async fn test_window_edit_state_restoring_enabled(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/root"), json!({"a": "hey"}))
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/a"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        assert_eq!(cx.update(|cx| cx.windows().len()), 1);

        // When opening the workspace, the window is not in a edited state.
        let window = cx.update(|cx| cx.windows()[0].downcast::<Workspace>().unwrap());

        let window_is_edited = |window: WindowHandle<Workspace>, cx: &mut TestAppContext| {
            cx.update(|cx| window.read(cx).unwrap().is_edited())
        };

        let editor = window
            .read_with(cx, |workspace, cx| {
                workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap()
            })
            .unwrap();

        assert!(!window_is_edited(window, cx));

        // Editing a buffer marks the window as edited.
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| editor.insert("EDIT", window, cx));
            })
            .unwrap();

        assert!(window_is_edited(window, cx));
        cx.run_until_parked();

        // Advance the clock to make sure the workspace is serialized
        cx.executor().advance_clock(Duration::from_secs(1));

        // When closing the window, no prompt shows up and the window is closed.
        // buffer having unsaved changes.
        assert!(!VisualTestContext::from_window(window.into(), cx).simulate_close());
        cx.run_until_parked();
        assert_eq!(cx.update(|cx| cx.windows().len()), 0);

        // When we now reopen the window, the edited state and the edited buffer are back
        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/root/a"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        assert_eq!(cx.update(|cx| cx.windows().len()), 1);
        assert!(cx.update(|cx| cx.active_window().is_some()));

        cx.run_until_parked();

        // When opening the workspace, the window is not in a edited state.
        let window = cx.update(|cx| cx.active_window().unwrap().downcast::<Workspace>().unwrap());
        assert!(window_is_edited(window, cx));

        window
            .update(cx, |workspace, _, cx| {
                let editor = workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<editor::Editor>()
                    .unwrap();
                editor.update(cx, |editor, cx| {
                    assert_eq!(editor.text(cx), "EDIThey");
                    assert!(editor.is_dirty(cx));
                });

                editor
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_new_empty_workspace(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        cx.update(|cx| {
            open_new(
                Default::default(),
                app_state.clone(),
                cx,
                |workspace, window, cx| {
                    Editor::new_file(workspace, &Default::default(), window, cx)
                },
            )
        })
        .await
        .unwrap();
        cx.run_until_parked();

        let workspace = cx
            .update(|cx| cx.windows().first().unwrap().downcast::<Workspace>())
            .unwrap();

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                let editor = workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<editor::Editor>()
                    .unwrap();
                editor.update(cx, |editor, cx| {
                    assert!(editor.text(cx).is_empty());
                    assert!(!editor.is_dirty(cx));
                });

                editor
            })
            .unwrap();

        let save_task = workspace
            .update(cx, |workspace, window, cx| {
                workspace.save_active_item(SaveIntent::Save, window, cx)
            })
            .unwrap();
        app_state.fs.create_dir(Path::new("/root")).await.unwrap();
        cx.background_executor.run_until_parked();
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/the-new-name")));
        save_task.await.unwrap();
        workspace
            .update(cx, |_, _, cx| {
                editor.update(cx, |editor, cx| {
                    assert!(!editor.is_dirty(cx));
                    assert_eq!(editor.title(cx), "the-new-name");
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_entry(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "a": {
                        "file1": "contents 1",
                        "file2": "contents 2",
                        "file3": "contents 3",
                    },
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.languages().add(markdown_language())
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window.root(cx).unwrap();

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        // Open the first entry
        let entry_1 = window
            .update(cx, |w, window, cx| {
                w.open_path(file1.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items_len(), 1);
        });

        // Open the second entry
        window
            .update(cx, |w, window, cx| {
                w.open_path(file2.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file2.clone())
            );
            assert_eq!(pane.items_len(), 2);
        });

        // Open the first entry again. The existing pane item is activated.
        let entry_1b = window
            .update(cx, |w, window, cx| {
                w.open_path(file1.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(entry_1.item_id(), entry_1b.item_id());

        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items_len(), 2);
        });

        // Split the pane with the first entry, then open the second entry again.
        window
            .update(cx, |w, window, cx| {
                w.split_and_clone(w.active_pane().clone(), SplitDirection::Right, window, cx);
                w.open_path(file2.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap();

        window
            .read_with(cx, |w, cx| {
                assert_eq!(
                    w.active_pane()
                        .read(cx)
                        .active_item()
                        .unwrap()
                        .project_path(cx),
                    Some(file2.clone())
                );
            })
            .unwrap();

        // Open the third entry twice concurrently. Only one pane item is added.
        let (t1, t2) = window
            .update(cx, |w, window, cx| {
                (
                    w.open_path(file3.clone(), None, true, window, cx),
                    w.open_path(file3.clone(), None, true, window, cx),
                )
            })
            .unwrap();
        t1.await.unwrap();
        t2.await.unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file3.clone())
            );
            let pane_entries = pane
                .items()
                .map(|i| i.project_path(cx).unwrap())
                .collect::<Vec<_>>();
            assert_eq!(pane_entries, &[file1, file2, file3]);
        });
    }

    #[gpui::test]
    async fn test_open_paths(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/"),
                json!({
                    "dir1": {
                        "a.txt": ""
                    },
                    "dir2": {
                        "b.txt": ""
                    },
                    "dir3": {
                        "c.txt": ""
                    },
                    "d.txt": ""
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir1/"))],
                app_state,
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();
        cx.run_until_parked();
        assert_eq!(cx.update(|cx| cx.windows().len()), 1);
        let window = cx.update(|cx| cx.windows()[0].downcast::<Workspace>().unwrap());
        let workspace = window.root(cx).unwrap();

        #[track_caller]
        fn assert_project_panel_selection(
            workspace: &Workspace,
            expected_worktree_path: &Path,
            expected_entry_path: &RelPath,
            cx: &App,
        ) {
            let project_panel = [
                workspace.left_dock().read(cx).panel::<ProjectPanel>(),
                workspace.right_dock().read(cx).panel::<ProjectPanel>(),
                workspace.bottom_dock().read(cx).panel::<ProjectPanel>(),
            ]
            .into_iter()
            .find_map(std::convert::identity)
            .expect("found no project panels")
            .read(cx);
            let (selected_worktree, selected_entry) = project_panel
                .selected_entry(cx)
                .expect("project panel should have a selected entry");
            assert_eq!(
                selected_worktree.abs_path().as_ref(),
                expected_worktree_path,
                "Unexpected project panel selected worktree path"
            );
            assert_eq!(
                selected_entry.path.as_ref(),
                expected_entry_path,
                "Unexpected project panel selected entry path"
            );
        }

        // Open a file within an existing worktree.
        window
            .update(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![path!("/dir1/a.txt").into()],
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await;
        cx.run_until_parked();
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            assert_project_panel_selection(
                workspace,
                Path::new(path!("/dir1")),
                rel_path("a.txt"),
                cx,
            );
            assert_eq!(
                workspace
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .act_as::<Editor>(cx)
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "a.txt"
            );
        });

        // Open a file outside of any existing worktree.
        window
            .update(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![path!("/dir2/b.txt").into()],
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await;
        cx.run_until_parked();
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            assert_project_panel_selection(
                workspace,
                Path::new(path!("/dir2/b.txt")),
                rel_path(""),
                cx,
            );
            let worktree_roots = workspace
                .worktrees(cx)
                .map(|w| w.read(cx).as_local().unwrap().abs_path().as_ref())
                .collect::<HashSet<_>>();
            assert_eq!(
                worktree_roots,
                vec![path!("/dir1"), path!("/dir2/b.txt")]
                    .into_iter()
                    .map(Path::new)
                    .collect(),
            );
            assert_eq!(
                workspace
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .act_as::<Editor>(cx)
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "b.txt"
            );
        });

        // Ensure opening a directory and one of its children only adds one worktree.
        window
            .update(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![path!("/dir3").into(), path!("/dir3/c.txt").into()],
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await;
        cx.run_until_parked();
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            assert_project_panel_selection(
                workspace,
                Path::new(path!("/dir3")),
                rel_path("c.txt"),
                cx,
            );
            let worktree_roots = workspace
                .worktrees(cx)
                .map(|w| w.read(cx).as_local().unwrap().abs_path().as_ref())
                .collect::<HashSet<_>>();
            assert_eq!(
                worktree_roots,
                vec![path!("/dir1"), path!("/dir2/b.txt"), path!("/dir3")]
                    .into_iter()
                    .map(Path::new)
                    .collect(),
            );
            assert_eq!(
                workspace
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .act_as::<Editor>(cx)
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "c.txt"
            );
        });

        // Ensure opening invisibly a file outside an existing worktree adds a new, invisible worktree.
        window
            .update(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![path!("/d.txt").into()],
                    OpenOptions {
                        visible: Some(OpenVisible::None),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await;
        cx.run_until_parked();
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            assert_project_panel_selection(workspace, Path::new(path!("/d.txt")), rel_path(""), cx);
            let worktree_roots = workspace
                .worktrees(cx)
                .map(|w| w.read(cx).as_local().unwrap().abs_path().as_ref())
                .collect::<HashSet<_>>();
            assert_eq!(
                worktree_roots,
                vec![
                    path!("/dir1"),
                    path!("/dir2/b.txt"),
                    path!("/dir3"),
                    path!("/d.txt")
                ]
                .into_iter()
                .map(Path::new)
                .collect(),
            );

            let visible_worktree_roots = workspace
                .visible_worktrees(cx)
                .map(|w| w.read(cx).as_local().unwrap().abs_path().as_ref())
                .collect::<HashSet<_>>();
            assert_eq!(
                visible_worktree_roots,
                vec![path!("/dir1"), path!("/dir2/b.txt"), path!("/dir3")]
                    .into_iter()
                    .map(Path::new)
                    .collect(),
            );

            assert_eq!(
                workspace
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .act_as::<Editor>(cx)
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "d.txt"
            );
        });
    }

    #[gpui::test]
    async fn test_opening_excluded_paths(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |project_settings| {
                    project_settings.project.worktree.file_scan_exclusions =
                        Some(vec!["excluded_dir".to_string(), "**/.git".to_string()]);
                });
            });
        });
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    ".gitignore": "ignored_dir\n",
                    ".git": {
                        "HEAD": "ref: refs/heads/main",
                    },
                    "regular_dir": {
                        "file": "regular file contents",
                    },
                    "ignored_dir": {
                        "ignored_subdir": {
                            "file": "ignored subfile contents",
                        },
                        "file": "ignored file contents",
                    },
                    "excluded_dir": {
                        "file": "excluded file contents",
                        "ignored_subdir": {
                            "file": "ignored subfile contents",
                        },
                    },
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.languages().add(markdown_language())
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window.root(cx).unwrap();

        let initial_entries = cx.read(|cx| workspace.file_project_paths(cx));
        let paths_to_open = [
            PathBuf::from(path!("/root/excluded_dir/file")),
            PathBuf::from(path!("/root/.git/HEAD")),
            PathBuf::from(path!("/root/excluded_dir/ignored_subdir")),
        ];
        let (opened_workspace, new_items) = cx
            .update(|cx| {
                workspace::open_paths(
                    &paths_to_open,
                    app_state,
                    workspace::OpenOptions::default(),
                    cx,
                )
            })
            .await
            .unwrap();

        assert_eq!(
            opened_workspace.root(cx).unwrap().entity_id(),
            workspace.entity_id(),
            "Excluded files in subfolders of a workspace root should be opened in the workspace"
        );
        let mut opened_paths = cx.read(|cx| {
            assert_eq!(
                new_items.len(),
                paths_to_open.len(),
                "Expect to get the same number of opened items as submitted paths to open"
            );
            new_items
                .iter()
                .zip(paths_to_open.iter())
                .map(|(i, path)| {
                    match i {
                        Some(Ok(i)) => Some(i.project_path(cx).map(|p| p.path)),
                        Some(Err(e)) => panic!("Excluded file {path:?} failed to open: {e:?}"),
                        None => None,
                    }
                    .flatten()
                })
                .collect::<Vec<_>>()
        });
        opened_paths.sort();
        assert_eq!(
            opened_paths,
            vec![
                None,
                Some(rel_path(".git/HEAD").into()),
                Some(rel_path("excluded_dir/file").into()),
            ],
            "Excluded files should get opened, excluded dir should not get opened"
        );

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        assert_eq!(
            initial_entries, entries,
            "Workspace entries should not change after opening excluded files and directories paths"
        );

        cx.read(|cx| {
                let pane = workspace.read(cx).active_pane().read(cx);
                let mut opened_buffer_paths = pane
                    .items()
                    .map(|i| {
                        i.project_path(cx)
                            .expect("all excluded files that got open should have a path")
                            .path
                    })
                    .collect::<Vec<_>>();
                opened_buffer_paths.sort();
                assert_eq!(
                    opened_buffer_paths,
                    vec![rel_path(".git/HEAD").into(), rel_path("excluded_dir/file").into()],
                    "Despite not being present in the worktrees, buffers for excluded files are opened and added to the pane"
                );
            });
    }

    #[gpui::test]
    async fn test_save_conflicting_item(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/root"), json!({ "a.txt": "" }))
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.languages().add(markdown_language())
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window.root(cx).unwrap();

        // Open a file within an existing worktree.
        window
            .update(cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![PathBuf::from(path!("/root/a.txt"))],
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    None,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await;
        let editor = cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            let item = pane.active_item().unwrap();
            item.downcast::<Editor>().unwrap()
        });

        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| editor.handle_input("x", window, cx));
            })
            .unwrap();

        app_state
            .fs
            .as_fake()
            .insert_file(path!("/root/a.txt"), b"changed".to_vec())
            .await;

        cx.run_until_parked();
        cx.read(|cx| assert!(editor.is_dirty(cx)));
        cx.read(|cx| assert!(editor.has_conflict(cx)));

        let save_task = window
            .update(cx, |workspace, window, cx| {
                workspace.save_active_item(SaveIntent::Save, window, cx)
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        cx.simulate_prompt_answer("Overwrite");
        save_task.await.unwrap();
        window
            .update(cx, |_, _, cx| {
                editor.update(cx, |editor, cx| {
                    assert!(!editor.is_dirty(cx));
                    assert!(!editor.has_conflict(cx));
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_and_save_new_file(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .create_dir(Path::new(path!("/root")))
            .await
            .unwrap();

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        project.update(cx, |project, _| {
            project.languages().add(markdown_language());
            project.languages().add(rust_lang());
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let worktree = cx.update(|cx| window.read(cx).unwrap().worktrees(cx).next().unwrap());

        // Create a new untitled buffer
        cx.dispatch_action(window.into(), NewFile);
        let editor = window
            .read_with(cx, |workspace, cx| {
                workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap()
            })
            .unwrap();

        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    assert!(!editor.is_dirty(cx));
                    assert_eq!(editor.title(cx), "untitled");
                    assert!(Arc::ptr_eq(
                        &editor.buffer().read(cx).language_at(0, cx).unwrap(),
                        &languages::PLAIN_TEXT
                    ));
                    editor.handle_input("hi", window, cx);
                    assert!(editor.is_dirty(cx));
                });
            })
            .unwrap();

        // Save the buffer. This prompts for a filename.
        let save_task = window
            .update(cx, |workspace, window, cx| {
                workspace.save_active_item(SaveIntent::Save, window, cx)
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        cx.simulate_new_path_selection(|parent_dir| {
            assert_eq!(parent_dir, Path::new(path!("/root")));
            Some(parent_dir.join("the-new-name.rs"))
        });
        cx.read(|cx| {
            assert!(editor.is_dirty(cx));
            assert_eq!(editor.read(cx).title(cx), "hi");
        });

        // When the save completes, the buffer's title is updated and the language is assigned based
        // on the path.
        save_task.await.unwrap();
        window
            .update(cx, |_, _, cx| {
                editor.update(cx, |editor, cx| {
                    assert!(!editor.is_dirty(cx));
                    assert_eq!(editor.title(cx), "the-new-name.rs");
                    assert_eq!(
                        editor.buffer().read(cx).language_at(0, cx).unwrap().name(),
                        "Rust".into()
                    );
                });
            })
            .unwrap();

        // Edit the file and save it again. This time, there is no filename prompt.
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.handle_input(" there", window, cx);
                    assert!(editor.is_dirty(cx));
                });
            })
            .unwrap();

        let save_task = window
            .update(cx, |workspace, window, cx| {
                workspace.save_active_item(SaveIntent::Save, window, cx)
            })
            .unwrap();
        save_task.await.unwrap();

        assert!(!cx.did_prompt_for_new_path());
        window
            .update(cx, |_, _, cx| {
                editor.update(cx, |editor, cx| {
                    assert!(!editor.is_dirty(cx));
                    assert_eq!(editor.title(cx), "the-new-name.rs")
                });
            })
            .unwrap();

        // Open the same newly-created file in another pane item. The new editor should reuse
        // the same buffer.
        cx.dispatch_action(window.into(), NewFile);
        window
            .update(cx, |workspace, window, cx| {
                workspace.split_and_clone(
                    workspace.active_pane().clone(),
                    SplitDirection::Right,
                    window,
                    cx,
                );
                workspace.open_path(
                    (worktree.read(cx).id(), rel_path("the-new-name.rs")),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();
        let editor2 = window
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap()
            })
            .unwrap();
        cx.read(|cx| {
            assert_eq!(
                editor2.read(cx).buffer().read(cx).as_singleton().unwrap(),
                editor.read(cx).buffer().read(cx).as_singleton().unwrap()
            );
        })
    }

    #[gpui::test]
    async fn test_setting_language_when_saving_as_single_file_worktree(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state.fs.create_dir(Path::new("/root")).await.unwrap();

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        project.update(cx, |project, _| {
            project.languages().add(rust_lang());
            project.languages().add(markdown_language());
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));

        // Create a new untitled buffer
        cx.dispatch_action(window.into(), NewFile);
        let editor = window
            .read_with(cx, |workspace, cx| {
                workspace
                    .active_item(cx)
                    .unwrap()
                    .downcast::<Editor>()
                    .unwrap()
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    assert!(Arc::ptr_eq(
                        &editor.buffer().read(cx).language_at(0, cx).unwrap(),
                        &languages::PLAIN_TEXT
                    ));
                    editor.handle_input("hi", window, cx);
                    assert!(editor.is_dirty(cx));
                });
            })
            .unwrap();

        // Save the buffer. This prompts for a filename.
        let save_task = window
            .update(cx, |workspace, window, cx| {
                workspace.save_active_item(SaveIntent::Save, window, cx)
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/the-new-name.rs")));
        save_task.await.unwrap();
        // The buffer is not dirty anymore and the language is assigned based on the path.
        window
            .update(cx, |_, _, cx| {
                editor.update(cx, |editor, cx| {
                    assert!(!editor.is_dirty(cx));
                    assert_eq!(
                        editor.buffer().read(cx).language_at(0, cx).unwrap().name(),
                        "Rust".into()
                    )
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_pane_actions(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "a": {
                        "file1": "contents 1",
                        "file2": "contents 2",
                        "file3": "contents 3",
                    },
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.languages().add(markdown_language())
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window.root(cx).unwrap();

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();

        let pane_1 = cx.read(|cx| workspace.read(cx).active_pane().clone());

        window
            .update(cx, |w, window, cx| {
                w.open_path(file1.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap();

        let (editor_1, buffer) = window
            .update(cx, |_, window, cx| {
                pane_1.update(cx, |pane_1, cx| {
                    let editor = pane_1.active_item().unwrap().downcast::<Editor>().unwrap();
                    assert_eq!(editor.project_path(cx), Some(file1.clone()));
                    let buffer = editor.update(cx, |editor, cx| {
                        editor.insert("dirt", window, cx);
                        editor.buffer().downgrade()
                    });
                    (editor.downgrade(), buffer)
                })
            })
            .unwrap();

        cx.dispatch_action(window.into(), pane::SplitRight);
        let editor_2 = cx.update(|cx| {
            let pane_2 = workspace.read(cx).active_pane().clone();
            assert_ne!(pane_1, pane_2);

            let pane2_item = pane_2.read(cx).active_item().unwrap();
            assert_eq!(pane2_item.project_path(cx), Some(file1.clone()));

            pane2_item.downcast::<Editor>().unwrap().downgrade()
        });
        cx.dispatch_action(
            window.into(),
            workspace::CloseActiveItem {
                save_intent: None,
                close_pinned: false,
            },
        );

        cx.background_executor.run_until_parked();
        window
            .read_with(cx, |workspace, _| {
                assert_eq!(workspace.panes().len(), 1);
                assert_eq!(workspace.active_pane(), &pane_1);
            })
            .unwrap();

        cx.dispatch_action(
            window.into(),
            workspace::CloseActiveItem {
                save_intent: None,
                close_pinned: false,
            },
        );
        cx.background_executor.run_until_parked();
        cx.simulate_prompt_answer("Don't Save");
        cx.background_executor.run_until_parked();

        window
            .update(cx, |workspace, _, cx| {
                assert_eq!(workspace.panes().len(), 1);
                assert!(workspace.active_item(cx).is_none());
            })
            .unwrap();

        cx.background_executor
            .advance_clock(SERIALIZATION_THROTTLE_TIME);
        cx.update(|_| {});
        editor_1.assert_released();
        editor_2.assert_released();
        buffer.assert_released();
    }

    #[gpui::test]
    async fn test_navigation(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "a": {
                        "file1": "contents 1\n".repeat(20),
                        "file2": "contents 2\n".repeat(20),
                        "file3": "contents 3\n".repeat(20),
                    },
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.languages().add(markdown_language())
        });
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let pane = workspace
            .read_with(cx, |workspace, _| workspace.active_pane().clone())
            .unwrap();

        let entries = cx.update(|cx| workspace.root(cx).unwrap().file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        let editor1 = workspace
            .update(cx, |w, window, cx| {
                w.open_path(file1.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        workspace
            .update(cx, |_, window, cx| {
                editor1.update(cx, |editor, cx| {
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_display_ranges([DisplayPoint::new(DisplayRow(10), 0)
                            ..DisplayPoint::new(DisplayRow(10), 0)])
                    });
                });
            })
            .unwrap();

        let editor2 = workspace
            .update(cx, |w, window, cx| {
                w.open_path(file2.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let editor3 = workspace
            .update(cx, |w, window, cx| {
                w.open_path(file3.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        workspace
            .update(cx, |_, window, cx| {
                editor3.update(cx, |editor, cx| {
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_display_ranges([DisplayPoint::new(DisplayRow(12), 0)
                            ..DisplayPoint::new(DisplayRow(12), 0)])
                    });
                    editor.newline(&Default::default(), window, cx);
                    editor.newline(&Default::default(), window, cx);
                    editor.move_down(&Default::default(), window, cx);
                    editor.move_down(&Default::default(), window, cx);
                    editor.save(
                        SaveOptions {
                            format: true,
                            autosave: false,
                        },
                        project.clone(),
                        window,
                        cx,
                    )
                })
            })
            .unwrap()
            .await
            .unwrap();
        workspace
            .update(cx, |_, window, cx| {
                editor3.update(cx, |editor, cx| {
                    editor.set_scroll_position(point(0., 12.5), window, cx)
                });
            })
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(DisplayRow(16), 0), 12.5)
        );

        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );

        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );

        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(DisplayRow(10), 0), 0.)
        );

        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );

        // Go back one more time and ensure we don't navigate past the first item in the history.
        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );

        workspace
            .update(cx, |w, window, cx| {
                w.go_forward(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(DisplayRow(10), 0), 0.)
        );

        workspace
            .update(cx, |w, window, cx| {
                w.go_forward(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );

        // Go forward to an item that has been closed, ensuring it gets re-opened at the same
        // location.
        workspace
            .update(cx, |_, window, cx| {
                pane.update(cx, |pane, cx| {
                    let editor3_id = editor3.entity_id();
                    drop(editor3);
                    pane.close_item_by_id(editor3_id, SaveIntent::Close, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        workspace
            .update(cx, |w, window, cx| {
                w.go_forward(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );

        workspace
            .update(cx, |w, window, cx| {
                w.go_forward(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(DisplayRow(16), 0), 12.5)
        );

        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );

        // Go back to an item that has been closed and removed from disk
        workspace
            .update(cx, |_, window, cx| {
                pane.update(cx, |pane, cx| {
                    let editor2_id = editor2.entity_id();
                    drop(editor2);
                    pane.close_item_by_id(editor2_id, SaveIntent::Close, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        app_state
            .fs
            .remove_file(Path::new(path!("/root/a/file2")), Default::default())
            .await
            .unwrap();
        cx.background_executor.run_until_parked();

        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );
        workspace
            .update(cx, |w, window, cx| {
                w.go_forward(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(DisplayRow(0), 0), 0.)
        );

        // Modify file to collapse multiple nav history entries into the same location.
        // Ensure we don't visit the same location twice when navigating.
        workspace
            .update(cx, |_, window, cx| {
                editor1.update(cx, |editor, cx| {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.select_display_ranges([DisplayPoint::new(DisplayRow(15), 0)
                            ..DisplayPoint::new(DisplayRow(15), 0)])
                    })
                });
            })
            .unwrap();
        for _ in 0..5 {
            workspace
                .update(cx, |_, window, cx| {
                    editor1.update(cx, |editor, cx| {
                        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.select_display_ranges([DisplayPoint::new(DisplayRow(3), 0)
                                ..DisplayPoint::new(DisplayRow(3), 0)])
                        });
                    });
                })
                .unwrap();

            workspace
                .update(cx, |_, window, cx| {
                    editor1.update(cx, |editor, cx| {
                        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.select_display_ranges([DisplayPoint::new(DisplayRow(13), 0)
                                ..DisplayPoint::new(DisplayRow(13), 0)])
                        })
                    });
                })
                .unwrap();
        }
        workspace
            .update(cx, |_, window, cx| {
                editor1.update(cx, |editor, cx| {
                    editor.transact(window, cx, |editor, window, cx| {
                        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.select_display_ranges([DisplayPoint::new(DisplayRow(2), 0)
                                ..DisplayPoint::new(DisplayRow(14), 0)])
                        });
                        editor.insert("", window, cx);
                    })
                });
            })
            .unwrap();

        workspace
            .update(cx, |_, window, cx| {
                editor1.update(cx, |editor, cx| {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.select_display_ranges([DisplayPoint::new(DisplayRow(1), 0)
                            ..DisplayPoint::new(DisplayRow(1), 0)])
                    })
                });
            })
            .unwrap();
        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(DisplayRow(2), 0), 0.)
        );
        workspace
            .update(cx, |w, window, cx| {
                w.go_back(w.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(DisplayRow(3), 0), 0.)
        );

        fn active_location(
            workspace: &WindowHandle<Workspace>,
            cx: &mut TestAppContext,
        ) -> (ProjectPath, DisplayPoint, f64) {
            workspace
                .update(cx, |workspace, _, cx| {
                    let item = workspace.active_item(cx).unwrap();
                    let editor = item.downcast::<Editor>().unwrap();
                    let (selections, scroll_position) = editor.update(cx, |editor, cx| {
                        (
                            editor.selections.display_ranges(cx),
                            editor.scroll_position(cx),
                        )
                    });
                    (
                        item.project_path(cx).unwrap(),
                        selections[0].start,
                        scroll_position.y,
                    )
                })
                .unwrap()
        }
    }

    #[gpui::test]
    async fn test_reopening_closed_items(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/root"),
                json!({
                    "a": {
                        "file1": "",
                        "file2": "",
                        "file3": "",
                        "file4": "",
                    },
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        project.update(cx, |project, _cx| {
            project.languages().add(markdown_language())
        });
        let workspace = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let pane = workspace
            .read_with(cx, |workspace, _| workspace.active_pane().clone())
            .unwrap();

        let entries = cx.update(|cx| workspace.root(cx).unwrap().file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();
        let file4 = entries[3].clone();

        let file1_item_id = workspace
            .update(cx, |w, window, cx| {
                w.open_path(file1.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap()
            .item_id();
        let file2_item_id = workspace
            .update(cx, |w, window, cx| {
                w.open_path(file2.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap()
            .item_id();
        let file3_item_id = workspace
            .update(cx, |w, window, cx| {
                w.open_path(file3.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap()
            .item_id();
        let file4_item_id = workspace
            .update(cx, |w, window, cx| {
                w.open_path(file4.clone(), None, true, window, cx)
            })
            .unwrap()
            .await
            .unwrap()
            .item_id();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        // Close all the pane items in some arbitrary order.
        workspace
            .update(cx, |_, window, cx| {
                pane.update(cx, |pane, cx| {
                    pane.close_item_by_id(file1_item_id, SaveIntent::Close, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, |_, window, cx| {
                pane.update(cx, |pane, cx| {
                    pane.close_item_by_id(file4_item_id, SaveIntent::Close, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |_, window, cx| {
                pane.update(cx, |pane, cx| {
                    pane.close_item_by_id(file2_item_id, SaveIntent::Close, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));
        workspace
            .update(cx, |_, window, cx| {
                pane.update(cx, |pane, cx| {
                    pane.close_item_by_id(file3_item_id, SaveIntent::Close, window, cx)
                })
            })
            .unwrap()
            .await
            .unwrap();

        assert_eq!(active_path(&workspace, cx), None);

        // Reopen all the closed items, ensuring they are reopened in the same order
        // in which they were closed.
        workspace
            .update(cx, Workspace::reopen_closed_item)
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, Workspace::reopen_closed_item)
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace
            .update(cx, Workspace::reopen_closed_item)
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, Workspace::reopen_closed_item)
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        // Reopening past the last closed item is a no-op.
        workspace
            .update(cx, Workspace::reopen_closed_item)
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        // Reopening closed items doesn't interfere with navigation history.
        workspace
            .update(cx, |workspace, window, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        fn active_path(
            workspace: &WindowHandle<Workspace>,
            cx: &TestAppContext,
        ) -> Option<ProjectPath> {
            workspace
                .read_with(cx, |workspace, cx| {
                    let item = workspace.active_item(cx)?;
                    item.project_path(cx)
                })
                .unwrap()
        }
    }

    fn init_keymap_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let app_state = AppState::test(cx);

            theme::init(theme::LoadThemes::JustBase, cx);
            client::init(&app_state.client, cx);
            language::init(cx);
            workspace::init(app_state.clone(), cx);
            onboarding::init(cx);
            Project::init_settings(cx);
            app_state
        })
    }

    actions!(test_only, [ActionA, ActionB]);

    #[gpui::test]
    async fn test_base_keymap(cx: &mut gpui::TestAppContext) {
        let executor = cx.executor();
        let app_state = init_keymap_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // From the Atom keymap
        use workspace::ActivatePreviousPane;
        // From the JetBrains keymap
        use workspace::ActivatePreviousItem;

        app_state
            .fs
            .save(
                "/settings.json".as_ref(),
                &r#"{"base_keymap": "Atom"}"#.into(),
                Default::default(),
            )
            .await
            .unwrap();

        app_state
            .fs
            .save(
                "/keymap.json".as_ref(),
                &r#"[{"bindings": {"backspace": "test_only::ActionA"}}]"#.into(),
                Default::default(),
            )
            .await
            .unwrap();
        executor.run_until_parked();
        cx.update(|cx| {
            let settings_rx = watch_config_file(
                &executor,
                app_state.fs.clone(),
                PathBuf::from("/settings.json"),
            );
            let keymap_rx = watch_config_file(
                &executor,
                app_state.fs.clone(),
                PathBuf::from("/keymap.json"),
            );
            let global_settings_rx = watch_config_file(
                &executor,
                app_state.fs.clone(),
                PathBuf::from("/global_settings.json"),
            );
            handle_settings_file_changes(settings_rx, global_settings_rx, cx, |_, _| {});
            handle_keymap_file_changes(keymap_rx, cx);
        });
        workspace
            .update(cx, |workspace, _, cx| {
                workspace.register_action(|_, _: &ActionA, _window, _cx| {});
                workspace.register_action(|_, _: &ActionB, _window, _cx| {});
                workspace.register_action(|_, _: &ActivatePreviousPane, _window, _cx| {});
                workspace.register_action(|_, _: &ActivatePreviousItem, _window, _cx| {});
                cx.notify();
            })
            .unwrap();
        executor.run_until_parked();
        // Test loading the keymap base at all
        assert_key_bindings_for(
            workspace.into(),
            cx,
            vec![("backspace", &ActionA), ("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test modifying the users keymap, while retaining the base keymap
        app_state
            .fs
            .save(
                "/keymap.json".as_ref(),
                &r#"[{"bindings": {"backspace": "test_only::ActionB"}}]"#.into(),
                Default::default(),
            )
            .await
            .unwrap();

        executor.run_until_parked();

        assert_key_bindings_for(
            workspace.into(),
            cx,
            vec![("backspace", &ActionB), ("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test modifying the base, while retaining the users keymap
        app_state
            .fs
            .save(
                "/settings.json".as_ref(),
                &r#"{"base_keymap": "JetBrains"}"#.into(),
                Default::default(),
            )
            .await
            .unwrap();

        executor.run_until_parked();

        assert_key_bindings_for(
            workspace.into(),
            cx,
            vec![("backspace", &ActionB), ("{", &ActivatePreviousItem)],
            line!(),
        );
    }

    #[gpui::test]
    async fn test_disabled_keymap_binding(cx: &mut gpui::TestAppContext) {
        let executor = cx.executor();
        let app_state = init_keymap_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

        // From the Atom keymap
        use workspace::ActivatePreviousPane;
        // From the JetBrains keymap
        use diagnostics::Deploy;

        workspace
            .update(cx, |workspace, _, _| {
                workspace.register_action(|_, _: &ActionA, _window, _cx| {});
                workspace.register_action(|_, _: &ActionB, _window, _cx| {});
                workspace.register_action(|_, _: &Deploy, _window, _cx| {});
            })
            .unwrap();
        app_state
            .fs
            .save(
                "/settings.json".as_ref(),
                &r#"{"base_keymap": "Atom"}"#.into(),
                Default::default(),
            )
            .await
            .unwrap();
        app_state
            .fs
            .save(
                "/keymap.json".as_ref(),
                &r#"[{"bindings": {"backspace": "test_only::ActionA"}}]"#.into(),
                Default::default(),
            )
            .await
            .unwrap();

        cx.update(|cx| {
            let settings_rx = watch_config_file(
                &executor,
                app_state.fs.clone(),
                PathBuf::from("/settings.json"),
            );
            let keymap_rx = watch_config_file(
                &executor,
                app_state.fs.clone(),
                PathBuf::from("/keymap.json"),
            );

            let global_settings_rx = watch_config_file(
                &executor,
                app_state.fs.clone(),
                PathBuf::from("/global_settings.json"),
            );
            handle_settings_file_changes(settings_rx, global_settings_rx, cx, |_, _| {});
            handle_keymap_file_changes(keymap_rx, cx);
        });

        cx.background_executor.run_until_parked();

        cx.background_executor.run_until_parked();
        // Test loading the keymap base at all
        assert_key_bindings_for(
            workspace.into(),
            cx,
            vec![("backspace", &ActionA), ("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test disabling the key binding for the base keymap
        app_state
            .fs
            .save(
                "/keymap.json".as_ref(),
                &r#"[{"bindings": {"backspace": null}}]"#.into(),
                Default::default(),
            )
            .await
            .unwrap();

        cx.background_executor.run_until_parked();

        assert_key_bindings_for(
            workspace.into(),
            cx,
            vec![("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test modifying the base, while retaining the users keymap
        app_state
            .fs
            .save(
                "/settings.json".as_ref(),
                &r#"{"base_keymap": "JetBrains"}"#.into(),
                Default::default(),
            )
            .await
            .unwrap();

        cx.background_executor.run_until_parked();

        assert_key_bindings_for(workspace.into(), cx, vec![("6", &Deploy)], line!());
    }

    #[gpui::test]
    async fn test_generate_keymap_json_schema_for_registered_actions(
        cx: &mut gpui::TestAppContext,
    ) {
        init_keymap_test(cx);
        cx.update(|cx| {
            // Make sure it doesn't panic.
            KeymapFile::generate_json_schema_for_registered_actions(cx);
        });
    }

    /// Actions that don't build from empty input won't work from command palette invocation.
    #[gpui::test]
    async fn test_actions_build_with_empty_input(cx: &mut gpui::TestAppContext) {
        init_keymap_test(cx);
        cx.update(|cx| {
            let all_actions = cx.all_action_names();
            let mut failing_names = Vec::new();
            let mut errors = Vec::new();
            for action in all_actions {
                match action.to_string().as_str() {
                    "vim::FindCommand"
                    | "vim::Literal"
                    | "vim::ResizePane"
                    | "vim::PushObject"
                    | "vim::PushFindForward"
                    | "vim::PushFindBackward"
                    | "vim::PushSneak"
                    | "vim::PushSneakBackward"
                    | "vim::PushChangeSurrounds"
                    | "vim::PushJump"
                    | "vim::PushDigraph"
                    | "vim::PushLiteral"
                    | "vim::PushHelixNext"
                    | "vim::PushHelixPrevious"
                    | "vim::Number"
                    | "vim::SelectRegister"
                    | "git::StageAndNext"
                    | "git::UnstageAndNext"
                    | "terminal::SendText"
                    | "terminal::SendKeystroke"
                    | "app_menu::OpenApplicationMenu"
                    | "picker::ConfirmInput"
                    | "editor::HandleInput"
                    | "editor::FoldAtLevel"
                    | "pane::ActivateItem"
                    | "workspace::ActivatePane"
                    | "workspace::MoveItemToPane"
                    | "workspace::MoveItemToPaneInDirection"
                    | "workspace::OpenTerminal"
                    | "workspace::SendKeystrokes"
                    | "agent::NewNativeAgentThreadFromSummary"
                    | "action::Sequence"
                    | "zed::OpenBrowser"
                    | "zed::OpenZedUrl"
                    | "settings_editor::FocusFile" => {}
                    _ => {
                        let result = cx.build_action(action, None);
                        match &result {
                            Ok(_) => {}
                            Err(err) => {
                                failing_names.push(action);
                                errors.push(format!("{action} failed to build: {err:?}"));
                            }
                        }
                    }
                }
            }
            if !errors.is_empty() {
                panic!(
                    "Failed to build actions using {{}} as input: {:?}. Errors:\n{}",
                    failing_names,
                    errors.join("\n")
                );
            }
        });
    }

    /// Checks that action namespaces are the expected set. The purpose of this is to prevent typos
    /// and let you know when introducing a new namespace.
    #[gpui::test]
    async fn test_action_namespaces(cx: &mut gpui::TestAppContext) {
        use itertools::Itertools;

        init_keymap_test(cx);
        cx.update(|cx| {
            let all_actions = cx.all_action_names();

            let mut actions_without_namespace = Vec::new();
            let all_namespaces = all_actions
                .iter()
                .filter_map(|action_name| {
                    let namespace = action_name
                        .split("::")
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .skip(1)
                        .rev()
                        .join("::");
                    if namespace.is_empty() {
                        actions_without_namespace.push(*action_name);
                    }
                    if &namespace == "test_only" || &namespace == "stories" {
                        None
                    } else {
                        Some(namespace)
                    }
                })
                .sorted()
                .dedup()
                .collect::<Vec<_>>();
            assert_eq!(actions_without_namespace, Vec::<&str>::new());

            let expected_namespaces = vec![
                "action",
                "activity_indicator",
                "agent",
                #[cfg(not(target_os = "macos"))]
                "app_menu",
                "assistant",
                "assistant2",
                "auto_update",
                "branches",
                "buffer_search",
                "channel_modal",
                "cli",
                "client",
                "collab",
                "collab_panel",
                "command_palette",
                "console",
                "context_server",
                "copilot",
                "debug_panel",
                "debugger",
                "dev",
                "diagnostics",
                "edit_prediction",
                "editor",
                "feedback",
                "file_finder",
                "git",
                "git_onboarding",
                "git_panel",
                "go_to_line",
                "icon_theme_selector",
                "journal",
                "keymap_editor",
                "keystroke_input",
                "language_selector",
                "line_ending",
                "lsp_tool",
                "markdown",
                "menu",
                "notebook",
                "notification_panel",
                "onboarding",
                "outline",
                "outline_panel",
                "pane",
                "panel",
                "picker",
                "project_panel",
                "project_search",
                "project_symbols",
                "projects",
                "repl",
                "rules_library",
                "search",
                "settings_editor",
                "settings_profile_selector",
                "snippets",
                "stash_picker",
                "supermaven",
                "svg",
                "syntax_tree_view",
                "tab_switcher",
                "task",
                "terminal",
                "terminal_panel",
                "theme_selector",
                "toast",
                "toolchain",
                "variable_list",
                "vim",
                "window",
                "workspace",
                "zed",
                "zed_actions",
                "zed_predict_onboarding",
                "zeta",
            ];
            assert_eq!(
                all_namespaces,
                expected_namespaces
                    .into_iter()
                    .map(|namespace| namespace.to_string())
                    .sorted()
                    .collect::<Vec<_>>()
            );
        });
    }

    #[gpui::test]
    fn test_bundled_settings_and_themes(cx: &mut App) {
        cx.text_system()
            .add_fonts(vec![
                Assets
                    .load("fonts/lilex/Lilex-Regular.ttf")
                    .unwrap()
                    .unwrap(),
                Assets
                    .load("fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf")
                    .unwrap()
                    .unwrap(),
            ])
            .unwrap();
        let themes = ThemeRegistry::default();
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);

        let mut has_default_theme = false;
        for theme_name in themes.list().into_iter().map(|meta| meta.name) {
            let theme = themes.get(&theme_name).unwrap();
            assert_eq!(theme.name, theme_name);
            if theme.name.as_ref() == "One Dark" {
                has_default_theme = true;
            }
        }
        assert!(has_default_theme);
    }

    #[gpui::test]
    async fn test_bundled_files_editor(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        cx.update(init);

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let _window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));

        cx.update(|cx| {
            cx.dispatch_action(&OpenDefaultSettings);
        });
        cx.run_until_parked();

        assert_eq!(cx.read(|cx| cx.windows().len()), 1);

        let workspace = cx.windows()[0].downcast::<Workspace>().unwrap();
        let active_editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace.active_item_as::<Editor>(cx)
            })
            .unwrap();
        assert!(
            active_editor.is_some(),
            "Settings action should have opened an editor with the default file contents"
        );

        let active_editor = active_editor.unwrap();
        assert!(
            active_editor.read_with(cx, |editor, cx| editor.read_only(cx)),
            "Default settings should be readonly"
        );
        assert!(
            active_editor.read_with(cx, |editor, cx| editor.buffer().read(cx).read_only()),
            "The underlying buffer should also be readonly for the shipped default settings"
        );
    }

    #[gpui::test]
    async fn test_bundled_languages(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        env_logger::builder().is_test(true).try_init().ok();
        let settings = cx.update(SettingsStore::test);
        cx.set_global(settings);
        let languages = LanguageRegistry::test(cx.executor());
        let languages = Arc::new(languages);
        let node_runtime = node_runtime::NodeRuntime::unavailable();
        cx.update(|cx| {
            languages::init(languages.clone(), fs, node_runtime, cx);
        });
        for name in languages.language_names() {
            languages
                .language_for_name(name.as_ref())
                .await
                .with_context(|| format!("language name {name}"))
                .unwrap();
        }
        cx.run_until_parked();
    }

    pub(crate) fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        init_test_with_state(cx, cx.update(AppState::test))
    }

    fn init_test_with_state(
        cx: &mut TestAppContext,
        mut app_state: Arc<AppState>,
    ) -> Arc<AppState> {
        cx.update(move |cx| {
            env_logger::builder().is_test(true).try_init().ok();

            let state = Arc::get_mut(&mut app_state).unwrap();
            state.build_window_options = build_window_options;

            app_state.languages.add(markdown_language());

            gpui_tokio::init(cx);
            vim_mode_setting::init(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            audio::init(cx);
            channel::init(&app_state.client, app_state.user_store.clone(), cx);
            call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
            notifications::init(app_state.client.clone(), app_state.user_store.clone(), cx);
            workspace::init(app_state.clone(), cx);
            Project::init_settings(cx);
            release_channel::init(SemanticVersion::default(), cx);
            command_palette::init(cx);
            language::init(cx);
            editor::init(cx);
            collab_ui::init(&app_state, cx);
            git_ui::init(cx);
            project_panel::init(cx);
            outline_panel::init(cx);
            terminal_view::init(cx);
            copilot::copilot_chat::init(
                app_state.fs.clone(),
                app_state.client.http_client(),
                copilot::copilot_chat::CopilotChatConfiguration::default(),
                cx,
            );
            image_viewer::init(cx);
            language_model::init(app_state.client.clone(), cx);
            language_models::init(app_state.user_store.clone(), app_state.client.clone(), cx);
            web_search::init(cx);
            web_search_providers::init(app_state.client.clone(), cx);
            let prompt_builder = PromptBuilder::load(app_state.fs.clone(), false, cx);
            agent_ui::init(
                app_state.fs.clone(),
                app_state.client.clone(),
                prompt_builder.clone(),
                app_state.languages.clone(),
                false,
                cx,
            );
            repl::init(app_state.fs.clone(), cx);
            repl::notebook::init(cx);
            tasks_ui::init(cx);
            project::debugger::breakpoint_store::BreakpointStore::init(
                &app_state.client.clone().into(),
            );
            project::debugger::dap_store::DapStore::init(&app_state.client.clone().into(), cx);
            debugger_ui::init(cx);
            initialize_workspace(app_state.clone(), prompt_builder, cx);
            search::init(cx);
            app_state
        })
    }

    fn rust_lang() -> Arc<language::Language> {
        Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        ))
    }

    fn markdown_language() -> Arc<language::Language> {
        Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "Markdown".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["md".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_md::LANGUAGE.into()),
        ))
    }

    #[track_caller]
    fn assert_key_bindings_for(
        window: AnyWindowHandle,
        cx: &TestAppContext,
        actions: Vec<(&'static str, &dyn Action)>,
        line: u32,
    ) {
        let available_actions = cx
            .update(|cx| window.update(cx, |_, window, cx| window.available_actions(cx)))
            .unwrap();
        for (key, action) in actions {
            let bindings = cx
                .update(|cx| window.update(cx, |_, window, _| window.bindings_for_action(action)))
                .unwrap();
            // assert that...
            assert!(
                available_actions.iter().any(|bound_action| {
                    // actions match...
                    bound_action.partial_eq(action)
                }),
                "On {} Failed to find {}",
                line,
                action.name(),
            );
            assert!(
                // and key strokes contain the given key
                bindings
                    .into_iter()
                    .any(|binding| binding.keystrokes().iter().any(|k| k.key() == key)),
                "On {} Failed to find {} with key binding {}",
                line,
                action.name(),
                key
            );
        }
    }

    #[gpui::test]
    async fn test_opening_project_settings_when_excluded(cx: &mut gpui::TestAppContext) {
        // Use the proper initialization for runtime state
        let app_state = init_keymap_test(cx);

        eprintln!("Running test_opening_project_settings_when_excluded");

        // 1. Set up a project with some project settings
        let settings_init =
            r#"{ "UNIQUEVALUE": true, "git": { "inline_blame": { "enabled": false } } }"#;
        app_state
            .fs
            .as_fake()
            .insert_tree(
                Path::new("/root"),
                json!({
                    ".zed": {
                        "settings.json": settings_init
                    }
                }),
            )
            .await;

        eprintln!("Created project with .zed/settings.json containing UNIQUEVALUE");

        // 2. Create a project with the file system and load it
        let project = Project::test(app_state.fs.clone(), [Path::new("/root")], cx).await;

        // Save original settings content for comparison
        let original_settings = app_state
            .fs
            .load(Path::new("/root/.zed/settings.json"))
            .await
            .unwrap();

        let original_settings_str = original_settings.clone();

        // Verify settings exist on disk and have expected content
        eprintln!("Original settings content: {}", original_settings_str);
        assert!(
            original_settings_str.contains("UNIQUEVALUE"),
            "Test setup failed - settings file doesn't contain our marker"
        );

        // 3. Add .zed to file scan exclusions in user settings
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |worktree_settings| {
                worktree_settings.project.worktree.file_scan_exclusions =
                    Some(vec![".zed".to_string()]);
            });
        });

        eprintln!("Added .zed to file_scan_exclusions in settings");

        // 4. Run tasks to apply settings
        cx.background_executor.run_until_parked();

        // 5. Critical: Verify .zed is actually excluded from worktree
        let worktree = cx.update(|cx| project.read(cx).worktrees(cx).next().unwrap());

        let has_zed_entry =
            cx.update(|cx| worktree.read(cx).entry_for_path(rel_path(".zed")).is_some());

        eprintln!(
            "Is .zed directory visible in worktree after exclusion: {}",
            has_zed_entry
        );

        // This assertion verifies the test is set up correctly to show the bug
        // If .zed is not excluded, the test will fail here
        assert!(
            !has_zed_entry,
            "Test precondition failed: .zed directory should be excluded but was found in worktree"
        );

        // 6. Create workspace and trigger the actual function that causes the bug
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        window
            .update(cx, |workspace, window, cx| {
                // Call the exact function that contains the bug
                eprintln!("About to call open_project_settings_file");
                open_project_settings_file(workspace, &OpenProjectSettings, window, cx);
            })
            .unwrap();

        // 7. Run background tasks until completion
        cx.background_executor.run_until_parked();

        // 8. Verify file contents after calling function
        let new_content = app_state
            .fs
            .load(Path::new("/root/.zed/settings.json"))
            .await
            .unwrap();

        let new_content_str = new_content;
        eprintln!("New settings content: {}", new_content_str);

        // The bug causes the settings to be overwritten with empty settings
        // So if the unique value is no longer present, the bug has been reproduced
        let bug_exists = !new_content_str.contains("UNIQUEVALUE");
        eprintln!("Bug reproduced: {}", bug_exists);

        // This assertion should fail if the bug exists - showing the bug is real
        assert!(
            new_content_str.contains("UNIQUEVALUE"),
            "BUG FOUND: Project settings were overwritten when opening via command - original custom content was lost"
        );
    }
}
