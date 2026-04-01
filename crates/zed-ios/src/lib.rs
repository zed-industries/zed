//! Zed for iPad — iOS static library entry point.
//!
//! This crate produces a static library (.a) that the Swift host app links against.
//! It provides C FFI entry points that the Swift side calls to initialize GPUI,
//! open windows, and manage the application lifecycle.
//!
//! See: docs/ios-port-plan.md for full architecture details.

#[cfg(target_os = "ios")]
mod connection_landing;

#[cfg(target_os = "ios")]
mod ios {
    #[allow(unused_imports)]
    use gpui::{
        AnyElement, App, AppContext as _, Application, ApplicationKeepAlive, Bounds, Context,
        Element, ElementId, ElementInputHandler, EntityInputHandler, FocusHandle, GlobalElementId,
        InspectorElementId, IntoElement, LayoutId, MouseButton, MouseDownEvent, Pixels, Point,
        PromptButton, PromptLevel, Render, SharedString, UTF16Selection, Window, WindowOptions,
        div, prelude::*,
    };
    use theme::ActiveTheme;
    use gpui_ios::IosPlatform;
    use std::{cell::RefCell, ops::Range, rc::Rc, sync::Arc};
    use util::ResultExt as _;
    #[allow(unused_imports)]
    use gpui::AppContext as _;

    thread_local! {
        /// Keeps the GPUI application alive for the process lifetime.
        /// On iOS, Application::run() returns immediately (UIKit owns the run loop),
        /// so we must hold this handle or the App is immediately dropped.
        static APP_KEEPALIVE: RefCell<Option<ApplicationKeepAlive>> = RefCell::new(None);
    }

    // ── Text input smoke-test view ────────────────────────────────────────────

    #[allow(dead_code)]
    /// A minimal text-input view for exercising the UITextInput pipeline.
    /// Type on the software keyboard — characters appear on screen.
    struct TextSmokeView {
        text: String,
        /// Cursor position as a byte offset into `text`.
        cursor: usize,
        focus_handle: FocusHandle,
    }

    #[allow(dead_code)]
    impl TextSmokeView {
        fn new(cx: &mut Context<Self>) -> Self {
            Self {
                text: String::new(),
                cursor: 0,
                focus_handle: cx.focus_handle(),
            }
        }
    }

    impl Render for TextSmokeView {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            TextSmokeElement {
                view: cx.entity().clone(),
                focus_handle: self.focus_handle.clone(),
            }
        }
    }

    impl EntityInputHandler for TextSmokeView {
        fn text_for_range(
            &mut self,
            range: Range<usize>,
            adjusted_range: &mut Option<Range<usize>>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<String> {
            let start = range.start.min(self.text.len());
            let end = range.end.min(self.text.len());
            *adjusted_range = Some(start..end);
            Some(self.text[start..end].to_owned())
        }

        fn selected_text_range(
            &mut self,
            _ignore_disabled_input: bool,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<UTF16Selection> {
            Some(UTF16Selection { range: self.cursor..self.cursor, reversed: false })
        }

        fn marked_text_range(
            &self,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<Range<usize>> {
            None
        }

        fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

        fn replace_text_in_range(
            &mut self,
            range: Option<Range<usize>>,
            text: &str,
            _window: &mut Window,
            cx: &mut Context<Self>,
        ) {
            let start = range.as_ref().map(|r| r.start).unwrap_or(self.cursor).min(self.text.len());
            let end = range.as_ref().map(|r| r.end).unwrap_or(self.cursor).min(self.text.len());
            self.text.replace_range(start..end, text);
            self.cursor = start + text.len();
            cx.notify();
        }

        fn replace_and_mark_text_in_range(
            &mut self,
            range: Option<Range<usize>>,
            new_text: &str,
            new_selected_range: Option<Range<usize>>,
            _window: &mut Window,
            cx: &mut Context<Self>,
        ) {
            let start = range.as_ref().map(|r| r.start).unwrap_or(self.cursor).min(self.text.len());
            let end = range.as_ref().map(|r| r.end).unwrap_or(self.cursor).min(self.text.len());
            self.text.replace_range(start..end, new_text);
            let new_end = start + new_text.len();
            self.cursor = new_selected_range
                .map(|r| (start + r.end).min(new_end))
                .unwrap_or(new_end);
            cx.notify();
        }

        fn bounds_for_range(
            &mut self,
            _range_utf16: Range<usize>,
            element_bounds: Bounds<Pixels>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<Bounds<Pixels>> {
            Some(element_bounds)
        }

        fn character_index_for_point(
            &mut self,
            _point: Point<Pixels>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<usize> {
            None
        }
    }

    // ── Element that paints the text and installs the input handler ───────────

    #[allow(dead_code)]
    struct TextSmokeElement {
        view: gpui::Entity<TextSmokeView>,
        focus_handle: FocusHandle,
    }

    impl IntoElement for TextSmokeElement {
        type Element = Self;
        fn into_element(self) -> Self { self }
    }

    impl Element for TextSmokeElement {
        type RequestLayoutState = AnyElement;
        type PrepaintState = ();

        fn id(&self) -> Option<ElementId> { None }

        fn source_location(&self) -> Option<&'static std::panic::Location<'static>> { None }

        fn request_layout(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            window: &mut Window,
            cx: &mut App,
        ) -> (LayoutId, Self::RequestLayoutState) {
            let text = self.view.read(cx).text.clone();
            let input_line = SharedString::from(format!("> {text}▌"));

            // A column of numbered lines above the input gives the view enough
            // content to scroll so two-finger pan can be verified.
            let lines = (1..=50).map(|n| {
                div()
                    .text_color(gpui::rgb(0xa6adc8))
                    .child(SharedString::from(format!("Line {n:02}: lorem ipsum dolor sit amet")))
            });

            let focus_handle = self.focus_handle.clone();
            let mut inner = div()
                .id("smoke-scroll")
                .size_full()
                .bg(gpui::rgb(0x1e1e2e))
                .text_color(gpui::rgb(0xcdd6f4))
                .text_xl()
                .flex()
                .flex_col()
                .overflow_y_scroll()
                .p_4()
                .children(lines)
                .child(
                    div()
                        .id("text-input")
                        .mt_4()
                        .text_color(gpui::rgb(0xcdd6f4))
                        .on_mouse_down(
                            MouseButton::Left,
                            move |_: &MouseDownEvent, window, cx| {
                                window.focus(&focus_handle, cx);
                            },
                        )
                        .child(input_line),
                )
                .child(
                    div()
                        .id("test-prompt")
                        .mt_4()
                        .px_4()
                        .py_2()
                        .bg(gpui::rgb(0x585b70))
                        .rounded_md()
                        .text_color(gpui::rgb(0xcdd6f4))
                        .on_mouse_down(
                            MouseButton::Left,
                            |_: &MouseDownEvent, window, cx| {
                                let _receiver = window.prompt(
                                    PromptLevel::Info,
                                    "Test Prompt",
                                    Some("Phase 1.3 UIAlertController is working!"),
                                    &[
                                        PromptButton::Ok("Nice".into()),
                                        PromptButton::Cancel("Cancel".into()),
                                    ],
                                    cx,
                                );
                            },
                        )
                        .child("Test Prompt"),
                )
                .into_any_element();
            let layout_id = inner.request_layout(window, cx);
            (layout_id, inner)
        }

        fn prepaint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            _bounds: Bounds<Pixels>,
            inner: &mut Self::RequestLayoutState,
            window: &mut Window,
            cx: &mut App,
        ) -> Self::PrepaintState {
            inner.prepaint(window, cx);
        }

        fn paint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            bounds: Bounds<Pixels>,
            inner: &mut Self::RequestLayoutState,
            _prepaint: &mut Self::PrepaintState,
            window: &mut Window,
            cx: &mut App,
        ) {
            window.handle_input(
                &self.focus_handle,
                ElementInputHandler::new(bounds, self.view.clone()),
                cx,
            );
            inner.paint(window, cx);
        }
    }

    // ── App lifecycle ─────────────────────────────────────────────────────────

    pub fn ios_main() {
        // Initialize logging to stderr — captured by Xcode console and `log stream`.
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .try_init();

        let platform = Rc::new(IosPlatform::new());
        let app = Application::with_platform(platform)
            .with_assets(assets::Assets);

        // Keep the app alive — Application::run() returns immediately on iOS
        // because UIKit owns the run loop.
        let keepalive = app.keep_alive();
        APP_KEEPALIVE.with(|cell| *cell.borrow_mut() = Some(keepalive));

        app.run(|cx: &mut App| {
            init_zed(cx);
        });
    }

    fn initialize_pane(
        workspace: &workspace::Workspace,
        pane: &gpui::Entity<workspace::Pane>,
        window: &mut Window,
        cx: &mut gpui::Context<workspace::Workspace>,
    ) {
        pane.update(cx, |pane, cx| {
            pane.toolbar().update(cx, |toolbar, cx| {
                let breadcrumbs = cx.new(|_| breadcrumbs::Breadcrumbs::new());
                toolbar.add_item(breadcrumbs, window, cx);
                let buffer_search_bar = cx.new(|cx| {
                    search::BufferSearchBar::new(
                        Some(workspace.project().read(cx).languages().clone()),
                        window,
                        cx,
                    )
                });
                toolbar.add_item(buffer_search_bar, window, cx);
                let diagnostic_controls =
                    cx.new(|_| diagnostics::ToolbarControls::new());
                toolbar.add_item(diagnostic_controls, window, cx);
                let project_search_bar = cx.new(|_| search::project_search::ProjectSearchBar::new());
                toolbar.add_item(project_search_bar, window, cx);
            })
        });
    }

    fn init_zed(cx: &mut App) {
        use fs::{Fs, RealFs};
        use futures::StreamExt as _;
        use language::LanguageRegistry;
        use node_runtime::NodeRuntime;
        use session::{AppSession, Session};
        use gpui::UpdateGlobal as _;
        use settings::{SettingsStore, watch_config_file};
        use workspace::{AppState, WorkspaceStore};

        release_channel::init(semver::Version::new(0, 1, 0), cx);

        // Database (must be set before anything that uses KeyValueStore)
        let app_db = db::AppDatabase::new();
        cx.set_global(app_db);

        // Tokio runtime for russh SSH transport
        gpui_tokio::init(cx);

        // Settings
        settings::init(cx);

        // HTTP client
        let http = Arc::new(
            reqwest_client::ReqwestClient::new()
        );
        cx.set_http_client(http);

        // Theme and fonts
        theme_settings::init(theme::LoadThemes::All(Box::new(assets::Assets)), cx);
        assets::Assets.load_fonts(cx).log_err();

        // Filesystem
        let fs = Arc::new(RealFs::new(
            None,
            cx.background_executor().clone(),
        ));
        <dyn Fs>::set_global(fs.clone(), cx);

        // Core services
        let client = client::Client::production(cx);
        cx.set_http_client(client.http_client());
        client::init(&client, cx);

        let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
        let workspace_store = cx.new(|cx| WorkspaceStore::new(client.clone(), cx));
        let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
        languages::init(
            languages.clone(),
            fs.clone(),
            node_runtime::NodeRuntime::unavailable(),
            cx,
        );
        languages.set_theme(cx.theme().clone());
        cx.observe_global::<theme::GlobalTheme>({
            let languages = languages.clone();
            move |cx| {
                languages.set_theme(cx.theme().clone());
            }
        })
        .detach();

        // Menu and actions
        menu::init();
        zed_actions::init();

        // Session::new is async (reads KVP store). Spawn the rest of init
        // to run after the session is created.
        let client_for_state = client.clone();
        cx.spawn(async move |cx| {
            let session_id = format!("ios-{}", std::process::id());
            let kvp = cx.update(|cx| db::kvp::KeyValueStore::global(cx));
            let session_data = Session::new(session_id, kvp).await;
            cx.update(|cx| {
                let session = cx.new(|cx| AppSession::new(session_data, cx));
                let settings_fs = fs.clone();
                let app_state = Arc::new(AppState {
                    client: client_for_state,
                    fs,
                    languages,
                    user_store,
                    workspace_store,
                    node_runtime: NodeRuntime::unavailable(),
                    build_window_options: |_, _| Default::default(),
                    session,
                });

                AppState::set_global(app_state.clone(), cx);

                git::GitHostingProviderRegistry::set_global(
                    git::GitHostingProviderRegistry::default_global(cx),
                    cx,
                );
                language_model::init_settings(cx);
                command_palette::init(cx);
                editor::init(cx);
                go_to_line::init(cx);
                file_finder::init(cx);
                diagnostics::init(cx);
                search::init(cx);
                git_hosting_providers::init(cx);
                git_ui::init(cx);
                language_tools::init(cx);
                vim::init(cx);
                terminal_view::init(cx);
                outline_panel::init(cx);
                language_selector::init(cx);
                theme_selector::init(cx);
                settings_profile_selector::init(cx);
                workspace::init(app_state.clone(), cx);
                project_panel::init(cx);
                recent_projects::init(cx);

                // Load user settings AFTER all init calls so that observers
                // (e.g. vim's SettingsStore observer) are already registered.
                let (mut user_settings_rx, user_settings_watcher) = watch_config_file(
                    &cx.background_executor(),
                    settings_fs,
                    paths::settings_file().clone(),
                );
                if let Some(user_content) = cx
                    .foreground_executor()
                    .block_on(user_settings_rx.next())
                {
                    SettingsStore::update_global(cx, |store: &mut SettingsStore, cx: &mut gpui::App| {
                        let _ = store.set_user_settings(&user_content, cx);
                    });
                }
                cx.spawn(async move |cx| {
                    let _watcher = user_settings_watcher;
                    while let Some(content) = user_settings_rx.next().await {
                        let _ = cx.update(|cx: &mut gpui::App| {
                            SettingsStore::update_global(cx, |store: &mut SettingsStore, cx: &mut gpui::App| {
                                let _ = store.set_user_settings(&content, cx);
                            });
                            cx.refresh_windows();
                        });
                    }
                })
                .detach();

                load_default_keymap(cx);

                // Reload keymaps when vim/helix mode toggles (the vim keymap is
                // conditionally loaded, so we must re-run when the setting changes).
                {
                    use settings::Settings as _;
                    let mut old_vim = vim_mode_setting::VimModeSetting::get_global(cx).0;
                    let mut old_helix = vim_mode_setting::HelixModeSetting::get_global(cx).0;
                    cx.observe_global::<settings::SettingsStore>(move |cx| {
                        let new_vim = vim_mode_setting::VimModeSetting::get_global(cx).0;
                        let new_helix = vim_mode_setting::HelixModeSetting::get_global(cx).0;
                        if new_vim != old_vim || new_helix != old_helix {
                            old_vim = new_vim;
                            old_helix = new_helix;
                            cx.clear_key_bindings();
                            load_default_keymap(cx);
                        }
                    })
                    .detach();
                }

                // Register no-op path prompts. The thin client doesn't open local
                // projects — all file access goes through the remote host. Without
                // this, clicking "Open Project" panics on unwrap().
                cx.observe_new(|workspace: &mut workspace::Workspace, window, cx| {
                    workspace.set_prompt_for_open_path(Box::new(|_, _, _, _| {
                        let (_tx, rx) = futures::channel::oneshot::channel();
                        rx
                    }));

                    let Some(window) = window else { return };

                    // Status bar items
                    let search_button = cx.new(|_| {
                        search::search_status_button::SearchButton::new()
                    });
                    let diagnostic_summary = cx.new(|cx| {
                        diagnostics::items::DiagnosticIndicator::new(workspace, cx)
                    });
                    let activity_indicator =
                        activity_indicator::ActivityIndicator::new(
                            workspace,
                            workspace.project().read(cx).languages().clone(),
                            window,
                            cx,
                        );
                    let lsp_menu_handle = ui::PopoverMenuHandle::default();
                    let lsp_button = cx.new(|cx| {
                        language_tools::lsp_button::LspButton::new(
                            workspace,
                            lsp_menu_handle,
                            window,
                            cx,
                        )
                    });
                    let active_buffer_language = cx.new(|_| {
                        language_selector::ActiveBufferLanguage::new(workspace)
                    });
                    let cursor_position = cx.new(|_| {
                        go_to_line::cursor_position::CursorPosition::new(workspace)
                    });
                    let vim_mode_indicator = cx.new(|cx| {
                        vim::ModeIndicator::new(window, cx)
                    });

                    workspace.status_bar().update(cx, |status_bar, cx| {
                        status_bar.add_left_item(search_button, window, cx);
                        status_bar.add_left_item(lsp_button, window, cx);
                        status_bar.add_left_item(diagnostic_summary, window, cx);
                        status_bar.add_left_item(activity_indicator, window, cx);
                        status_bar.add_right_item(active_buffer_language, window, cx);
                        status_bar.add_right_item(vim_mode_indicator, window, cx);
                        status_bar.add_right_item(cursor_position, window, cx);
                    });

                    // Status bar prefix/suffix for remote workspaces.
                    // Set here (in addition to create_workspace_for_path) so they
                    // survive SSH reconnection, which may recreate the workspace.
                    if let Some(conn) = workspace.project().read(cx).remote_connection_options(cx) {
                        if let remote::RemoteConnectionOptions::Ssh(ref opts) = conn {
                            let host = opts.host.to_string();
                            let username = opts.username.clone().unwrap_or_default();
                            let port = opts.port.unwrap_or(22);
                            let path = workspace
                                .worktrees(cx)
                                .next()
                                .map(|wt| wt.read(cx).abs_path().to_string_lossy().to_string())
                                .unwrap_or_default();
                            let switcher = cx.new(|_cx| {
                                crate::connection_landing::WorkspaceSwitcher::new(
                                    &path, &host, &username, port,
                                )
                            });
                            let suffix = cx.new(|_cx| crate::connection_landing::StatusBarSuffix);
                            workspace.set_status_bar_prefix(switcher.into(), cx);
                            workspace.set_status_bar_suffix(suffix.into(), cx);
                        }
                    }

                    // Toolbar items for panes
                    let center_pane = workspace.active_pane().clone();
                    initialize_pane(workspace, &center_pane, window, cx);
                    let workspace_handle = cx.entity();
                    cx.subscribe_in(&workspace_handle, window, {
                        move |workspace, _, event, window, cx| {
                            if let workspace::Event::PaneAdded(pane) = event {
                                initialize_pane(workspace, pane, window, cx);
                            }
                        }
                    })
                    .detach();

                    // Panels
                    let panels_task = cx.spawn_in(window, async move |workspace_handle, cx| {
                        if let Some(panel) = project_panel::ProjectPanel::load(
                            workspace_handle.clone(), cx.clone(),
                        ).await.log_err() {
                            workspace_handle.update_in(
                                &mut cx.clone(),
                                |workspace, window, cx| {
                                    workspace.add_panel(panel, window, cx);
                                    workspace.left_dock().update(cx, |dock, cx| {
                                        dock.set_open(true, window, cx);
                                    });
                                },
                            ).log_err();
                        }
                        if let Some(panel) = outline_panel::OutlinePanel::load(
                            workspace_handle.clone(), cx.clone(),
                        ).await.log_err() {
                            workspace_handle.update_in(
                                &mut cx.clone(),
                                |workspace, window, cx| workspace.add_panel(panel, window, cx),
                            ).log_err();
                        }
                        if let Some(panel) = git_ui::git_panel::GitPanel::load(
                            workspace_handle.clone(), cx.clone(),
                        ).await.log_err() {
                            workspace_handle.update_in(
                                &mut cx.clone(),
                                |workspace, window, cx| workspace.add_panel(panel, window, cx),
                            ).log_err();
                        }
                        if let Some(panel) = terminal_view::terminal_panel::TerminalPanel::load(
                            workspace_handle.clone(), cx.clone(),
                        ).await.log_err() {
                            workspace_handle.update_in(
                                &mut cx.clone(),
                                |workspace, window, cx| workspace.add_panel(panel, window, cx),
                            ).log_err();
                        }
                        anyhow::Ok(())
                    });
                    workspace.set_panels_task(panels_task);
                })
                .detach();

                APP_STATE.with(|cell| *cell.borrow_mut() = Some(app_state));
                log::info!("[zed-ios] Zed initialized successfully");

                // Register Zed's menu structure with the iPadOS menu bar.
                cx.set_menus(ios_app_menus());
                // Wire up the dispatcher so UIKit menu taps reach GPUI.
                {
                    let async_cx = cx.to_async();
                    gpui_ios::set_menu_action_dispatcher(Box::new(move |action_name: &str| {
                        let name = action_name.to_owned();
                        async_cx.update(|cx: &mut gpui::App| {
                            match cx.build_action(&name, None) {
                                Ok(action) => cx.dispatch_action(action.as_ref()),
                                Err(err) => log::warn!(
                                    "[zed-ios] Unknown menu action `{}`: {err:?}",
                                    name
                                ),
                            }
                        });
                    }));
                }

                // Store AsyncApp handle for FFI entry points (e.g. background persist).
                store_async_app(cx);

                // Initialize connection tracking and show the landing screen.
                crate::connection_landing::init_active_connections(cx);
                if let Err(err) = crate::connection_landing::ConnectionLanding::open(cx) {
                    log::error!("[zed-ios] Failed to open connection landing: {err:?}");
                }
            });
        })
        .detach();
    }

    /// Returns the subset of Zed's menu structure supported on iPad.
    ///
    /// Only includes crates that are actually initialized in `init_zed()` and
    /// actions that are meaningful on a remote thin client (no local terminals,
    /// debugger, auto-update, collab panel, etc.).
    fn ios_app_menus() -> Vec<gpui::Menu> {
        use gpui::{Menu, MenuItem};

        vec![
            Menu {
                name: "File".into(),
                disabled: false,
                items: vec![
                    MenuItem::action("New", workspace::NewFile),
                    MenuItem::separator(),
                    MenuItem::action("Save", workspace::Save { save_intent: None }),
                    MenuItem::action("Save As…", workspace::SaveAs),
                    MenuItem::action("Save All", workspace::SaveAll { save_intent: None }),
                    MenuItem::separator(),
                    MenuItem::action(
                        "Close Editor",
                        workspace::CloseActiveItem {
                            save_intent: None,
                            close_pinned: true,
                        },
                    ),
                ],
            },
            Menu {
                name: "Edit".into(),
                disabled: false,
                items: vec![
                    MenuItem::os_action("Undo", editor::actions::Undo, gpui::OsAction::Undo),
                    MenuItem::os_action("Redo", editor::actions::Redo, gpui::OsAction::Redo),
                    MenuItem::separator(),
                    MenuItem::os_action("Cut", editor::actions::Cut, gpui::OsAction::Cut),
                    MenuItem::os_action("Copy", editor::actions::Copy, gpui::OsAction::Copy),
                    MenuItem::os_action("Paste", editor::actions::Paste, gpui::OsAction::Paste),
                    MenuItem::separator(),
                    MenuItem::action("Find", search::buffer_search::Deploy::find()),
                    MenuItem::action("Find in Project", workspace::DeploySearch::find()),
                    MenuItem::separator(),
                    MenuItem::action(
                        "Toggle Line Comment",
                        editor::actions::ToggleComments::default(),
                    ),
                ],
            },
            Menu {
                name: "Selection".into(),
                disabled: false,
                items: vec![
                    MenuItem::os_action(
                        "Select All",
                        editor::actions::SelectAll,
                        gpui::OsAction::SelectAll,
                    ),
                    MenuItem::action(
                        "Expand Selection",
                        editor::actions::SelectLargerSyntaxNode,
                    ),
                    MenuItem::action(
                        "Shrink Selection",
                        editor::actions::SelectSmallerSyntaxNode,
                    ),
                    MenuItem::separator(),
                    MenuItem::action(
                        "Add Cursor Above",
                        editor::actions::AddSelectionAbove { skip_soft_wrap: true },
                    ),
                    MenuItem::action(
                        "Add Cursor Below",
                        editor::actions::AddSelectionBelow { skip_soft_wrap: true },
                    ),
                    MenuItem::action(
                        "Select Next Occurrence",
                        editor::actions::SelectNext { replace_newest: false },
                    ),
                    MenuItem::action(
                        "Select All Occurrences",
                        editor::actions::SelectAllMatches,
                    ),
                    MenuItem::separator(),
                    MenuItem::action("Move Line Up", editor::actions::MoveLineUp),
                    MenuItem::action("Move Line Down", editor::actions::MoveLineDown),
                    MenuItem::action("Duplicate Selection", editor::actions::DuplicateLineDown),
                ],
            },
            Menu {
                name: "View".into(),
                disabled: false,
                items: vec![
                    MenuItem::action(
                        "Zoom In",
                        zed_actions::IncreaseBufferFontSize { persist: false },
                    ),
                    MenuItem::action(
                        "Zoom Out",
                        zed_actions::DecreaseBufferFontSize { persist: false },
                    ),
                    MenuItem::action(
                        "Reset Zoom",
                        zed_actions::ResetBufferFontSize { persist: false },
                    ),
                    MenuItem::separator(),
                    MenuItem::action("Toggle Left Dock", workspace::ToggleLeftDock),
                    MenuItem::action("Toggle Right Dock", workspace::ToggleRightDock),
                    MenuItem::separator(),
                    MenuItem::action("Project Panel", zed_actions::project_panel::ToggleFocus),
                    MenuItem::action("Outline Panel", outline_panel::ToggleFocus),
                    MenuItem::action("Git Panel", git_ui::git_panel::ToggleFocus),
                    MenuItem::separator(),
                    MenuItem::action("Diagnostics", diagnostics::Deploy),
                ],
            },
            Menu {
                name: "Go".into(),
                disabled: false,
                items: vec![
                    MenuItem::action("Back", workspace::GoBack),
                    MenuItem::action("Forward", workspace::GoForward),
                    MenuItem::separator(),
                    MenuItem::action(
                        "Command Palette…",
                        zed_actions::command_palette::Toggle,
                    ),
                    MenuItem::separator(),
                    MenuItem::action(
                        "Go to File…",
                        workspace::ToggleFileFinder::default(),
                    ),
                    MenuItem::action(
                        "Go to Symbol in Editor…",
                        zed_actions::outline::ToggleOutline,
                    ),
                    MenuItem::action(
                        "Go to Line/Column…",
                        editor::actions::ToggleGoToLine,
                    ),
                    MenuItem::separator(),
                    MenuItem::action("Go to Definition", editor::actions::GoToDefinition),
                    MenuItem::action("Go to Type Definition", editor::actions::GoToTypeDefinition),
                    MenuItem::action(
                        "Find All References",
                        editor::actions::FindAllReferences::default(),
                    ),
                    MenuItem::separator(),
                    MenuItem::action(
                        "Next Problem",
                        editor::actions::GoToDiagnostic::default(),
                    ),
                    MenuItem::action(
                        "Previous Problem",
                        editor::actions::GoToPreviousDiagnostic::default(),
                    ),
                ],
            },
            Menu {
                name: "Settings".into(),
                disabled: false,
                items: vec![
                    MenuItem::action("Open Settings", zed_actions::OpenSettings),
                    MenuItem::action(
                        "Select Theme…",
                        zed_actions::theme_selector::Toggle::default(),
                    ),
                    MenuItem::action(
                        "Select Language…",
                        language_selector::Toggle,
                    ),
                    MenuItem::action(
                        "Select Profile…",
                        zed_actions::settings_profile_selector::Toggle,
                    ),
                    MenuItem::separator(),
                    MenuItem::action("Open Keymap", zed_actions::OpenKeymap),
                ],
            },
        ]
    }

    thread_local! {
        static APP_STATE: RefCell<Option<Arc<workspace::AppState>>> = RefCell::new(None);
    }

    pub fn app_state() -> Option<Arc<workspace::AppState>> {
        APP_STATE.with(|cell| cell.borrow().clone())
    }

    fn load_default_keymap(cx: &mut gpui::App) {
        use settings::{DEFAULT_KEYMAP_PATH, KeybindSource, KeymapFile, Settings as _, VIM_KEYMAP_PATH};
        use vim_mode_setting::{HelixModeSetting, VimModeSetting};

        match KeymapFile::load_asset_allow_partial_failure(DEFAULT_KEYMAP_PATH, cx) {
            Ok(mut key_bindings) => {
                for binding in &mut key_bindings {
                    binding.set_meta(KeybindSource::Default.meta());
                }
                cx.bind_keys(key_bindings);
            }
            Err(error) => log::error!("Failed to load default keymap: {error}"),
        }

        if VimModeSetting::get_global(cx).0 || HelixModeSetting::get_global(cx).0 {
            match KeymapFile::load_asset_allow_partial_failure(VIM_KEYMAP_PATH, cx) {
                Ok(mut key_bindings) => {
                    for binding in &mut key_bindings {
                        binding.set_meta(KeybindSource::Vim.meta());
                    }
                    cx.bind_keys(key_bindings);
                }
                Err(error) => log::error!("Failed to load vim keymap: {error}"),
            }
        }
    }

    pub fn ios_open_window() {
        // The first workspace window is opened by init_zed after async Session
        // creation completes. Subsequent calls from SceneDelegate (e.g. Stage
        // Manager multi-window) would open additional workspaces here.
        log::info!("[zed-ios] ios_open_window called");
    }

    pub fn ios_will_resign_active() {
        log::info!("[zed-ios] app will resign active — persisting sessions");
        ASYNC_APP.with(|cell| {
            if let Some(ref async_cx) = *cell.borrow() {
                async_cx.update(|cx| {
                    crate::connection_landing::persist_sessions_for_background(cx);
                });
            }
        });
    }

    thread_local! {
        static ASYNC_APP: RefCell<Option<gpui::AsyncApp>> = RefCell::new(None);
    }

    /// Store the AsyncApp handle for use by FFI entry points that need App access.
    fn store_async_app(cx: &App) {
        let async_cx = cx.to_async();
        ASYNC_APP.with(|cell| *cell.borrow_mut() = Some(async_cx));
    }
}

/// Main entry point called by AppDelegate.swift after UIApplicationMain.
///
/// # Safety
/// Called from Swift via C FFI. Must be called exactly once on the main thread.
#[unsafe(no_mangle)]
pub extern "C" fn zed_ios_main() {
    #[cfg(target_os = "ios")]
    ios::ios_main();
}

/// Called by SceneDelegate.swift when a new UIWindowScene activates.
///
/// # Safety
/// Called from Swift via C FFI. `scene_id` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_ios_open_window(_scene_id: *const std::ffi::c_char) {
    #[cfg(target_os = "ios")]
    ios::ios_open_window();
}

/// Called by SceneDelegate.swift when a UIWindowScene disconnects.
///
/// # Safety
/// Called from Swift via C FFI. `scene_id` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_ios_close_window(_scene_id: *const std::ffi::c_char) {
    // TODO: Clean up the GPUI window, disconnect if last window.
}

/// Called by `AppDelegate.buildMenu(with:)` to install Zed's menus into the
/// iPadOS menu bar. `builder` is a `UIMenuBuilder*` passed as an opaque pointer.
///
/// # Safety
/// Must be called on the UIKit main thread inside `buildMenuWithBuilder:`.
/// The `builder` pointer must be a valid `UIMenuBuilder` instance.
#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_ios_build_menus(builder: *mut std::ffi::c_void) {
    unsafe { gpui_ios::build_ios_menus(builder) }
}

/// Called by AppDelegate or SceneDelegate when the app is about to enter
/// the background. Persists active SSH sessions so they can be restored
/// on next launch.
///
/// # Safety
/// Called from Swift via C FFI. Must be called on the main thread.
#[unsafe(no_mangle)]
pub extern "C" fn zed_ios_will_resign_active() {
    #[cfg(target_os = "ios")]
    ios::ios_will_resign_active();
}

// Submodules — uncomment as implemented:
// pub mod keychain;         // Phase 2.1: SSH key storage via Security.framework
// pub mod network_monitor;  // Phase 2.3: NWPathMonitor connectivity events
// pub mod ssh_transport;    // Phase 2.0: russh-based SSH transport (CRITICAL)
