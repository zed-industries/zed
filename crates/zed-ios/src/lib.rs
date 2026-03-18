//! Zed for iPad — iOS static library entry point.
//!
//! This crate produces a static library (.a) that the Swift host app links against.
//! It provides C FFI entry points that the Swift side calls to initialize GPUI,
//! open windows, and manage the application lifecycle.
//!
//! See: docs/ios-port-plan.md for full architecture details.

#[cfg(target_os = "ios")]
mod ios {
    use gpui::{
        AnyElement, App, AppContext as _, Application, ApplicationKeepAlive, Bounds, Context,
        Element, ElementId, ElementInputHandler, EntityInputHandler, FocusHandle, GlobalElementId,
        InspectorElementId, IntoElement, LayoutId, MouseButton, MouseDownEvent, Pixels, Point,
        PromptButton, PromptLevel, Render, SharedString, UTF16Selection, Window, WindowOptions,
        div, prelude::*,
    };
    use gpui_ios::IosPlatform;
    use std::{cell::RefCell, ops::Range, rc::Rc, sync::Arc};
    use util::ResultExt as _;

    thread_local! {
        /// Keeps the GPUI application alive for the process lifetime.
        /// On iOS, Application::run() returns immediately (UIKit owns the run loop),
        /// so we must hold this handle or the App is immediately dropped.
        static APP_KEEPALIVE: RefCell<Option<ApplicationKeepAlive>> = RefCell::new(None);
    }

    // ── Text input smoke-test view ────────────────────────────────────────────

    /// A minimal text-input view for exercising the UITextInput pipeline.
    /// Type on the software keyboard — characters appear on screen.
    struct TextSmokeView {
        text: String,
        /// Cursor position as a byte offset into `text`.
        cursor: usize,
        focus_handle: FocusHandle,
    }

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

    fn init_zed(cx: &mut App) {
        use fs::{Fs, RealFs};
        use language::LanguageRegistry;
        use node_runtime::NodeRuntime;
        use session::{AppSession, Session};
        use workspace::{AppState, WorkspaceStore};

        release_channel::init(semver::Version::new(0, 1, 0), cx);

        // Settings — use empty default settings for now
        settings::init(cx);

        // HTTP client
        let http = Arc::new(
            reqwest_client::ReqwestClient::new()
        );
        cx.set_http_client(http);

        // Theme and fonts
        theme::init(theme::LoadThemes::JustBase, cx);
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

        // Menu and actions
        menu::init();
        zed_actions::init();

        // Session::new is async (reads KVP store). Spawn the rest of init
        // to run after the session is created.
        let client_for_state = client.clone();
        cx.spawn(async move |cx| {
            let session_id = format!("ios-{}", std::process::id());
            let session_data = Session::new(session_id).await;
            cx.update(|cx| {
                let session = cx.new(|cx| AppSession::new(session_data, cx));
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

                workspace::init(app_state.clone(), cx);

                // Register no-op path prompts. The thin client doesn't open local
                // projects — all file access goes through the remote host. Without
                // this, clicking "Open Project" panics on unwrap().
                cx.observe_new(|workspace: &mut workspace::Workspace, _window, _cx| {
                    workspace.set_prompt_for_open_path(Box::new(|_, _, _, _| {
                        let (_tx, rx) = futures::channel::oneshot::channel();
                        rx
                    }));
                })
                .detach();

                APP_STATE.with(|cell| *cell.borrow_mut() = Some(app_state.clone()));
                log::info!("[zed-ios] Zed initialized successfully");

                // Open the workspace window now that init is complete.
                let task = workspace::Workspace::new_local(
                    vec![],
                    app_state,
                    None,
                    None,
                    None,
                    true,
                    cx,
                );
                cx.spawn(async move |_cx| {
                    match task.await {
                        Ok(_) => log::info!("[zed-ios] Workspace opened"),
                        Err(err) => log::error!("[zed-ios] Failed to open workspace: {err:?}"),
                    }
                })
                .detach();
            });
        })
        .detach();
    }

    thread_local! {
        static APP_STATE: RefCell<Option<Arc<workspace::AppState>>> = RefCell::new(None);
    }

    pub fn ios_open_window() {
        // The first workspace window is opened by init_zed after async Session
        // creation completes. Subsequent calls from SceneDelegate (e.g. Stage
        // Manager multi-window) would open additional workspaces here.
        log::info!("[zed-ios] ios_open_window called");
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

// Submodules — uncomment as implemented:
// pub mod keychain;         // Phase 2.1: SSH key storage via Security.framework
// pub mod network_monitor;  // Phase 2.3: NWPathMonitor connectivity events
// pub mod ssh_transport;    // Phase 2.0: russh-based SSH transport (CRITICAL)
