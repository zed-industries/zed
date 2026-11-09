//! Production-shaped fixtures for benchmarking Zed doing ordinary things.
//!
//! Everything here follows the startup path in `crates/zed/src/main.rs` and
//! `crates/zed/src/zed.rs` (`initialize_workspace`), trimmed to the subsystems the
//! scenarios exercise: a real `RealFs` over a temporary directory, the built-in
//! tree-sitter languages, the bundled themes, fonts, icons and keymap, an on-disk
//! application database in a temporary data directory, and a client with no network
//! access. No crate in this graph may enable `test-support`; see
//! `script/check-zed-benchmarks-features`.

mod agent_thread;

pub use agent_thread::{AGENT_THREAD_ENTRIES_PER_TURN, AGENT_THREAD_TURNS, agent_thread_json};

use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result, anyhow};
use assets::Assets;
use client::{Client, UserStore};
use editor::Editor;
use fs::{Fs, RealFs};
use gpui::{
    AnyWindowHandle, App, AppContext as _, BenchAppContext, Entity, Keystroke, Modifiers,
    MouseMoveEvent, PlatformInput, Point, ScrollDelta, ScrollEvent, ScrollSource, TouchPhase,
    UpdateGlobal as _, Window, WindowHandle, profiler::FrameEvent, profiler::FrameTimingCollector,
    px,
};
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use project::Project;
use session::AppSession;
use settings::{KeybindSource, KeymapFile, SettingsStore};
use workspace::{AppState, MultiWorkspace, OpenMode, Workspace, WorkspaceStore};

/// The commit that the large source fixtures are read from with `git show`, so the
/// workload stays fixed as the repository changes. It must exist in the local clone.
pub const PINNED_FIXTURE_COMMIT: &str = "22580c6e6970de1aebf653a012765025fe8bf0f7";

/// Files read from [`PINNED_FIXTURE_COMMIT`]: (path in the Zed repository, path in
/// the generated benchmark project).
pub const PINNED_FILES: &[(&str, &str)] = &[
    ("crates/editor/src/editor.rs", "crates/editor/src/editor.rs"),
    ("script/danger/dangerfile.ts", "web/src/dangerfile.ts"),
    (
        "script/github-check-new-issue-for-duplicates.py",
        "scripts/check_new_issue_for_duplicates.py",
    ),
    (
        "docs/src/reference/all-settings.md",
        "docs/reference/all-settings.md",
    ),
];

pub const RUST_FILE: &str = "crates/editor/src/editor.rs";
pub const TYPESCRIPT_FILE: &str = "web/src/dangerfile.ts";
pub const PYTHON_FILE: &str = "scripts/check_new_issue_for_duplicates.py";
pub const MARKDOWN_FILE: &str = "docs/reference/all-settings.md";

/// Generated crates in the benchmark project, each a directory tree with a manifest,
/// a `src` directory and a few modules.
pub const GENERATED_CRATE_COUNT: usize = 24;
pub const GENERATED_MODULES_PER_CRATE: usize = 4;
const GENERATED_COMPONENT_COUNT: usize = 8;

/// User settings applied on top of the bundled defaults, as if read from
/// `settings.json`. They keep the benchmark from starting language servers,
/// sending telemetry or checking for updates, none of which is under test.
const BENCH_USER_SETTINGS: &str = r#"{
  "enable_language_server": false,
  "telemetry": { "diagnostics": false, "metrics": false },
  "auto_update": false,
  "show_edit_predictions": false
}"#;

/// A hard limit for any single wait in fixture setup or a measured session, so a
/// broken fixture fails instead of hanging the benchmark process.
pub const WAIT_TIMEOUT: Duration = Duration::from_secs(60);

struct ProcessFixture {
    root: PathBuf,
    pinned_contents: Vec<(&'static str, String)>,
    next_project: AtomicUsize,
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_pinned_file(repository_path: &str) -> Result<String> {
    let root = repository_root();
    let output = Command::new("git")
        .arg("-C")
        .arg(&root)
        .arg("show")
        .arg(format!("{PINNED_FIXTURE_COMMIT}:{repository_path}"))
        .output()
        .with_context(|| format!("failed to run git in {}", root.display()))?;
    if !output.status.success() {
        return Err(anyhow!(
            "could not read {repository_path} at the pinned fixture commit {PINNED_FIXTURE_COMMIT}: {}\n\
             The zed_benchmarks fixtures need that commit in the local clone; run \
             `git fetch origin {PINNED_FIXTURE_COMMIT}`.",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout).context("pinned fixture is not UTF-8")
}

fn process_fixture() -> &'static ProcessFixture {
    static FIXTURE: OnceLock<ProcessFixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        let root = std::env::temp_dir().join(format!("zed-benchmarks-{}", std::process::id()));
        if root.exists() {
            std::fs::remove_dir_all(&root).expect("failed to clear benchmark directory");
        }
        // Equivalent to `zed --user-data-dir`: settings, databases and logs stay in
        // the benchmark's own directory instead of the user's.
        let data_dir = root.join("data");
        paths::set_custom_data_dir(
            data_dir
                .to_str()
                .expect("benchmark data directory path is not UTF-8"),
        );
        let pinned_contents = PINNED_FILES
            .iter()
            .map(|(repository_path, project_path)| {
                let contents = read_pinned_file(repository_path).unwrap_or_else(|error| {
                    panic!("{error:#}");
                });
                (*project_path, contents)
            })
            .collect();
        ProcessFixture {
            root,
            pinned_contents,
            next_project: AtomicUsize::new(0),
        }
    })
}

/// Returns the pinned contents of a file in the generated project.
pub fn pinned_contents(project_path: &str) -> &'static str {
    process_fixture()
        .pinned_contents
        .iter()
        .find(|(path, _)| *path == project_path)
        .map(|(_, contents)| contents.as_str())
        .unwrap_or_else(|| panic!("{project_path} is not a pinned fixture file"))
}

fn generated_rust_module(crate_index: usize, module_index: usize) -> String {
    let mut text = format!(
        "//! Module {module_index} of generated crate {crate_index}.\n\n\
         use std::collections::HashMap;\n\n"
    );
    for item in 0..12 {
        text.push_str(&format!(
            "/// Item {item} keeps track of a small amount of state.\n\
             #[derive(Debug, Clone, Default)]\n\
             pub struct Item{item}State {{\n    \
                 pub name: String,\n    \
                 pub counts: HashMap<String, usize>,\n    \
                 pub enabled: bool,\n\
             }}\n\n\
             impl Item{item}State {{\n    \
                 pub fn record(&mut self, key: &str) -> usize {{\n        \
                     let count = self.counts.entry(key.to_string()).or_insert(0);\n        \
                     *count += {module_index} + {item};\n        \
                     *count\n    \
                 }}\n\
             }}\n\n"
        ));
    }
    text
}

/// Number of files [`create_project`] writes.
pub fn project_file_count() -> usize {
    PINNED_FILES.len()
        + 2
        + GENERATED_CRATE_COUNT * (GENERATED_MODULES_PER_CRATE + 2)
        + GENERATED_COMPONENT_COUNT
}

/// Writes a fresh project: the pinned files plus a few dozen generated directories
/// and files. Each call uses a new directory because the workspace database
/// remembers per-root state such as open items and dock visibility.
pub fn create_project() -> PathBuf {
    let fixture = process_fixture();
    let index = fixture.next_project.fetch_add(1, Ordering::Relaxed);
    let root = fixture
        .root
        .join("projects")
        .join(format!("project-{index}"));
    let write = |relative: &str, contents: &str| {
        let path = root.join(relative);
        std::fs::create_dir_all(path.parent().expect("fixture paths have parents"))
            .expect("failed to create fixture directory");
        std::fs::write(&path, contents).expect("failed to write fixture file");
    };
    for (project_path, contents) in &fixture.pinned_contents {
        write(project_path, contents);
    }
    write(
        "README.md",
        "# Benchmark project\n\nA generated workspace for Zed benchmarks.\n",
    );
    write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
    );
    for crate_index in 0..GENERATED_CRATE_COUNT {
        let crate_dir = format!("crates/generated_{crate_index:02}");
        write(
            &format!("{crate_dir}/Cargo.toml"),
            &format!(
                "[package]\nname = \"generated_{crate_index:02}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"
            ),
        );
        let mut lib = String::new();
        for module_index in 0..GENERATED_MODULES_PER_CRATE {
            lib.push_str(&format!("pub mod module_{module_index};\n"));
            write(
                &format!("{crate_dir}/src/module_{module_index}.rs"),
                &generated_rust_module(crate_index, module_index),
            );
        }
        write(&format!("{crate_dir}/src/lib.rs"), &lib);
    }
    for component in 0..GENERATED_COMPONENT_COUNT {
        write(
            &format!("web/src/components/component_{component}.ts"),
            &format!(
                "export interface Props{component} {{\n  label: string;\n  count: number;\n}}\n\n\
                 export function render{component}(props: Props{component}): string {{\n  \
                 return `${{props.label}}: ${{props.count + {component}}}`;\n}}\n"
            ),
        );
    }
    root
}

fn load_default_keymap(cx: &mut App) {
    // Mirrors `zed::load_default_keymap` for the default base keymap without vim
    // mode. Bindings to actions from crates this suite doesn't link (vim, terminal,
    // debugger, ...) are skipped; production binds them too, but they can't fire in
    // these scenarios.
    for path in [
        settings::DEFAULT_KEYMAP_PATH,
        settings::SPECIFIC_OVERRIDES_KEYMAP_PATH,
    ] {
        let mut bindings = KeymapFile::load_asset_allow_partial_failure(path, cx)
            .unwrap_or_else(|error| panic!("failed to load {path}: {error:#}"));
        for binding in &mut bindings {
            binding.set_meta(KeybindSource::Default.meta());
        }
        cx.bind_keys(bindings);
    }
}

/// Initializes the app the way `main.rs` does, for the subsystems the benchmarks use.
pub fn init_app(cx: &mut BenchAppContext) -> Arc<AppState> {
    init_app_with_settings(cx, "{}")
}

/// Like [`init_app`], with extra user settings merged over the benchmark defaults.
pub fn init_app_with_settings(cx: &mut BenchAppContext, extra_settings: &str) -> Arc<AppState> {
    process_fixture();
    cx.set_asset_source(Arc::new(Assets));
    cx.update(|cx| {
        let app_db = db::AppDatabase::new();
        let key_value_store = db::kvp::KeyValueStore::from_app_db(&app_db);
        cx.set_global(app_db);
        project::trusted_worktrees::init(Default::default(), cx);
        menu::init();
        zed_actions::init();
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        gpui_tokio::init(cx);
        settings::init(cx);
        let mut user_settings: serde_json::Value =
            serde_json::from_str(BENCH_USER_SETTINGS).expect("benchmark settings are JSON");
        let extra_settings: serde_json::Value =
            serde_json::from_str(extra_settings).expect("extra benchmark settings are JSON");
        if let (Some(user_settings), Some(extra_settings)) =
            (user_settings.as_object_mut(), extra_settings.as_object())
        {
            user_settings.extend(extra_settings.clone());
        }
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(&user_settings.to_string(), cx)
                .result()
                .expect("benchmark user settings must parse");
        });

        let fs: Arc<dyn Fs> = RealFs::new(None, cx.background_executor().clone());
        <dyn Fs>::set_global(fs.clone(), cx);

        let client = Client::production(cx);
        cx.set_http_client(client.http_client());
        let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
        let node_runtime = NodeRuntime::unavailable();
        languages::init(languages.clone(), fs.clone(), node_runtime.clone(), cx);
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let workspace_store = cx.new(|cx| WorkspaceStore::new(client.clone(), cx));
        Client::set_global(client.clone(), cx);
        Project::init(&client, cx);
        client::init(&client, cx);

        let session = cx.foreground_executor().block_on(session::Session::new(
            "zed-benchmarks".to_string(),
            key_value_store,
        ));
        let session = cx.new(|cx| AppSession::new(session, cx));
        let app_state = Arc::new(AppState {
            languages: languages.clone(),
            client: client.clone(),
            user_store,
            fs: fs.clone(),
            build_window_options: |_, _| Default::default(),
            workspace_store,
            node_runtime,
            session,
        });
        AppState::set_global(app_state.clone(), cx);

        theme_settings::init(theme::LoadThemes::All(Box::new(Assets)), cx);
        language_model::init(cx);
        let prompt_builder = prompt_store::PromptBuilder::load(fs.clone(), false, cx);
        project::AgentRegistryStore::init_global(cx, fs.clone(), client.http_client());
        agent_ui::init(fs.clone(), prompt_builder, languages, false, false, cx);

        Assets
            .load_fonts(cx)
            .expect("failed to load the bundled fonts");
        editor::init(cx);
        workspace::init(app_state.clone(), cx);
        go_to_line::init(cx);
        project_panel::init(cx);
        search::init(cx);
        cx.set_global(workspace::PaneSearchBarCallbacks {
            setup_search_bar: |languages, toolbar, window, cx| {
                let search_bar = cx.new(|cx| search::BufferSearchBar::new(languages, window, cx));
                toolbar.update(cx, |toolbar, cx| {
                    toolbar.add_item(search_bar, window, cx);
                });
            },
            wrap_div_with_search_actions: search::buffer_search::register_pane_search_actions,
        });
        language_selector::init(cx);
        settings_ui::init(cx);
        keymap_editor::init(cx);
        load_default_keymap(cx);
        initialize_workspace(cx);
        app_state
    })
}

/// The subset of `zed::initialize_workspace` the benchmarks render: the pane
/// toolbar's breadcrumbs and buffer search bar, and the status bar items that show
/// language server activity, the active language and the cursor position.
fn initialize_workspace(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let center_pane = workspace.active_pane().clone();
        initialize_pane(workspace, &center_pane, window, cx);
        cx.subscribe_in(&cx.entity(), window, |workspace, _, event, window, cx| {
            if let workspace::Event::PaneAdded(pane) = event {
                initialize_pane(workspace, pane, window, cx);
            }
        })
        .detach();

        let search_button = cx.new(|_| search::search_status_button::SearchButton::new());
        let active_file_name = cx.new(|_| workspace::active_file_name::ActiveFileName::new());
        let activity_indicator = activity_indicator::ActivityIndicator::new(workspace, window, cx);
        let active_buffer_language =
            cx.new(|_| language_selector::ActiveBufferLanguage::new(workspace));
        let cursor_position =
            cx.new(|_| go_to_line::cursor_position::CursorPosition::new(workspace));
        workspace.status_bar().update(cx, |status_bar, cx| {
            status_bar.add_left_item(search_button, window, cx);
            status_bar.add_left_item(active_file_name, window, cx);
            status_bar.add_left_item(activity_indicator, window, cx);
            status_bar.add_right_item(active_buffer_language, window, cx);
            status_bar.add_right_item(cursor_position, window, cx);
        });
    })
    .detach();
}

fn initialize_pane(
    workspace: &Workspace,
    pane: &Entity<workspace::Pane>,
    window: &mut Window,
    cx: &mut gpui::Context<Workspace>,
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
        });
    });
}

/// A workspace opened on a generated project in its own window.
pub struct ZedWindow {
    pub app_state: Arc<AppState>,
    pub project_root: PathBuf,
    pub window: WindowHandle<MultiWorkspace>,
    pub workspace: Entity<Workspace>,
}

impl ZedWindow {
    pub fn any_window(&self) -> AnyWindowHandle {
        self.window.into()
    }

    pub fn abs_path(&self, project_path: &str) -> PathBuf {
        self.project_root.join(project_path)
    }

    pub fn update<R>(
        &self,
        cx: &mut BenchAppContext,
        update: impl FnOnce(&mut Workspace, &mut Window, &mut gpui::Context<Workspace>) -> R,
    ) -> R {
        let workspace = self.workspace.clone();
        self.update_window(cx, |window, cx| {
            workspace.update(cx, |workspace, cx| update(workspace, window, cx))
        })
    }

    pub fn update_window<R>(
        &self,
        cx: &mut BenchAppContext,
        update: impl FnOnce(&mut Window, &mut App) -> R,
    ) -> R {
        // Through the untyped handle, so the root view isn't leased while input
        // dispatched here reaches it.
        cx.update_window(self.any_window(), |_, window, cx| update(window, cx))
            .expect("benchmark window was closed")
    }

    /// Opens a project file in the active pane and waits until it is open.
    pub fn open_file(&self, cx: &mut BenchAppContext, project_path: &str) -> Entity<Editor> {
        let path = self.abs_path(project_path);
        let task = self.update(cx, |workspace, window, cx| {
            workspace.open_abs_path(
                path,
                workspace::OpenOptions {
                    visible: Some(workspace::OpenVisible::None),
                    ..Default::default()
                },
                window,
                cx,
            )
        });
        let item = wait_for_task(cx, task).expect("failed to open fixture file");
        item.downcast::<Editor>()
            .expect("fixture files open in editors")
    }
}

/// Opens a project through `Workspace::new_local`, the path `zed <dir>` takes, and
/// waits for its worktree scan and panels.
pub fn open_zed_window(cx: &mut BenchAppContext, app_state: &Arc<AppState>) -> ZedWindow {
    let project_root = create_project();
    let task = cx.update(|cx| {
        Workspace::new_local(
            vec![project_root.clone()],
            app_state.clone(),
            None,
            None,
            None,
            OpenMode::NewWindow,
            cx,
        )
    });
    let open_result = wait_for_task(cx, task).expect("failed to open benchmark workspace");
    let window = open_result.window;
    let workspace = open_result.workspace;
    // The headless platform has no window server to activate new windows.
    window
        .update(cx, |_, window, _| window.activate_window())
        .expect("benchmark window was closed");

    let zed_window = ZedWindow {
        app_state: app_state.clone(),
        project_root,
        window,
        workspace,
    };

    let panels = zed_window.update(cx, |_, window, cx| {
        cx.spawn_in(window, async move |workspace, cx| {
            let project_panel =
                project_panel::ProjectPanel::load(workspace.clone(), cx.clone()).await?;
            let agent_panel = agent_ui::AgentPanel::load(workspace.clone(), cx.clone()).await?;
            workspace.update_in(cx, |workspace, window, cx| {
                workspace.add_panel(project_panel, window, cx);
                workspace.add_panel(agent_panel, window, cx);
            })
        })
    });
    wait_for_task(cx, panels).expect("failed to load panels");

    let expected_files = project_file_count();
    let project = zed_window
        .workspace
        .read_with(cx, |workspace, _| workspace.project().clone());
    wait_until(cx, "worktree scan", |cx| {
        project.read_with(cx, |project, cx| {
            project
                .worktrees(cx)
                .next()
                .is_some_and(|worktree| worktree.read(cx).file_count() >= expected_files)
        })
    });
    cx.run_until_idle();
    zed_window
}

/// Runs foreground work and frames until `condition` holds, panicking after
/// [`WAIT_TIMEOUT`].
pub fn wait_until(
    cx: &mut BenchAppContext,
    description: &str,
    mut condition: impl FnMut(&mut BenchAppContext) -> bool,
) {
    let started = Instant::now();
    let probe = cx.clone();
    let mut cx = cx.clone();
    probe.run_until(|| {
        if condition(&mut cx) {
            return Some(());
        }
        if started.elapsed() > WAIT_TIMEOUT {
            panic!("timed out after {WAIT_TIMEOUT:?} waiting for {description}");
        }
        // Background work such as parsing or scanning may not wake a foreground
        // task when it finishes, so poll at a modest rate.
        std::thread::sleep(Duration::from_micros(200));
        None
    });
}

pub fn wait_for_task<T: 'static>(cx: &mut BenchAppContext, task: gpui::Task<T>) -> T {
    let output = Rc::new(RefCell::new(None));
    let started = Instant::now();
    let _completion = cx.foreground_executor().spawn({
        let output = output.clone();
        async move {
            *output.borrow_mut() = Some(task.await);
        }
    });
    cx.run_until(|| {
        if started.elapsed() > WAIT_TIMEOUT {
            panic!("timed out after {WAIT_TIMEOUT:?} waiting for a task");
        }
        output.borrow_mut().take()
    })
}

/// Waits until the buffer behind `editor` has the expected language, a finished
/// parse and syntax layers, so measured frames render complete highlighting.
pub fn wait_for_highlighting(cx: &mut BenchAppContext, editor: &Entity<Editor>, language: &str) {
    let buffer = editor.read_with(cx, |editor, cx| {
        editor
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("fixture editors edit one buffer")
    });
    wait_until(cx, "syntax highlighting", |cx| {
        buffer.read_with(cx, |buffer, _| {
            buffer
                .language()
                .is_some_and(|buffer_language| buffer_language.name().as_ref() == language)
                && *buffer.parse_status().borrow() == language::ParseStatus::Idle
                && buffer.snapshot().syntax_layers().next().is_some()
        })
    });
}

/// A trackpad scroll over `position`, delivered through the window's input dispatch.
pub fn scroll_wheel(
    window: &mut Window,
    cx: &mut App,
    position: Point<gpui::Pixels>,
    delta_y: f32,
) {
    window.dispatch_event(
        PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: None,
            modifiers: Modifiers::default(),
        }),
        cx,
    );
    window.dispatch_event(
        PlatformInput::Scroll(ScrollEvent {
            position,
            delta: ScrollDelta::Pixels(gpui::point(px(0.), px(delta_y))),
            modifiers: Modifiers::default(),
            touch_phase: TouchPhase::Moved,
            source: ScrollSource::Pointer,
        }),
        cx,
    );
}

/// Types a keystroke through the keymap and the window's input handler, as a key
/// press does.
pub fn keystroke(window: &mut Window, cx: &mut App, keystroke: &str) {
    thread_local! {
        static PARSED: RefCell<std::collections::HashMap<String, Keystroke>> =
            RefCell::default();
    }
    // `Keystroke::parse` needs a `'static` source; the benchmarks use a small, fixed
    // set of keystrokes, so leaking each distinct one once is bounded.
    let parsed = PARSED.with(|parsed| {
        parsed
            .borrow_mut()
            .entry(keystroke.to_string())
            .or_insert_with(|| {
                let source: &'static str = Box::leak(keystroke.to_string().into_boxed_str());
                Keystroke::parse(source)
                    .unwrap_or_else(|_| panic!("invalid keystroke {keystroke:?}"))
            })
            .clone()
    });
    window.dispatch_keystroke(parsed, cx);
}

/// Frame statistics for one scenario, gathered from GPUI's frame profiler across
/// every measured session (including Criterion warm-up).
#[derive(Default, Debug, Clone)]
pub struct FrameStats {
    pub sessions: u64,
    pub draws: u64,
    pub request_layout: Duration,
    pub prepaint: Duration,
    pub paint: Duration,
}

impl FrameStats {
    pub fn print(&self, name: &str) {
        let per_draw = |duration: Duration| {
            if self.draws == 0 {
                0.0
            } else {
                duration.as_secs_f64() * 1000.0 / self.draws as f64
            }
        };
        let draws_per_session = if self.sessions == 0 {
            0.0
        } else {
            self.draws as f64 / self.sessions as f64
        };
        eprintln!(
            "zed bench frames: {name}: {} sessions, {} draws ({draws_per_session:.1}/session); \
             mean per draw: request_layout {:.3}ms, prepaint {:.3}ms, paint {:.3}ms",
            self.sessions,
            self.draws,
            per_draw(self.request_layout),
            per_draw(self.prepaint),
            per_draw(self.paint),
        );
    }
}

/// One measured session's state. Frame events are collected and `finish` runs when
/// it is dropped, which happens after the timed interval.
pub struct SessionInput<S> {
    pub state: S,
    stop: Arc<AtomicBool>,
    collector: FrameTimingCollector,
    stats: Rc<RefCell<FrameStats>>,
    window_id: gpui::WindowId,
    on_finish: Option<Box<dyn FnOnce(&mut S)>>,
}

impl<S> Drop for SessionInput<S> {
    fn drop(&mut self) {
        let mut stats = self.stats.borrow_mut();
        stats.sessions += 1;
        for event in self.collector.collect_unseen() {
            if let FrameEvent::Draw(timing) = event
                && timing.window_id == self.window_id
            {
                stats.draws += 1;
                stats.request_layout += timing.phases.request_layout;
                stats.prepaint += timing.phases.prepaint;
                stats.paint += timing.phases.paint;
            }
        }
        drop(stats);
        if let Some(on_finish) = self.on_finish.take() {
            on_finish(&mut self.state);
        }
    }
}

/// What a session wants after a step.
pub enum Step {
    Continue,
    Stop,
}

/// Measures repeated sessions of input against a Zed window with
/// `bench_renderer_session`: every turn runs ready foreground work, delivers one
/// step of input and draws any frame it scheduled. `setup` runs before each
/// session and `finish` after it, both outside the timed interval; `finish` should
/// assert that the session did its work.
pub fn bench_sessions<S: 'static>(
    cx: &mut BenchAppContext,
    window: AnyWindowHandle,
    name: &str,
    mut setup: impl FnMut(&mut BenchAppContext) -> S,
    mut step: impl FnMut(&mut S, u64, &mut Window, &mut App) -> Step,
    finish: impl Fn(&mut S) + Clone + 'static,
) -> FrameStats {
    let stats = Rc::new(RefCell::new(FrameStats::default()));
    let window_id = window.window_id();
    // The harness clears the profiler's frame buffer when each measurement's trace
    // scope ends, before session inputs are dropped; keeping the trace setting on
    // preserves the events until `SessionInput` collects them.
    let enabled_tracing = gpui::profiler::set_trace_enabled(true);
    cx.bench_renderer_session(
        WAIT_TIMEOUT,
        |cx| {
            let state = setup(cx);
            let stop = Arc::new(AtomicBool::new(false));
            let finish = finish.clone();
            (
                SessionInput {
                    state,
                    stop: stop.clone(),
                    collector: FrameTimingCollector::new(),
                    stats: stats.clone(),
                    window_id,
                    on_finish: Some(Box::new(move |state| finish(state))),
                },
                window,
                stop,
            )
        },
        |input, turn, window, cx| {
            if let Step::Stop = step(&mut input.state, turn, window, cx) {
                input.stop.store(true, Ordering::Release);
            }
        },
    );
    if enabled_tracing {
        gpui::profiler::set_trace_enabled(false);
    }
    let stats = stats.borrow().clone();
    stats.print(name);
    stats
}

/// Sleeps until `turn` frame intervals at `fps` have passed since `started`, so
/// idle sessions run at a display's cadence instead of spinning.
pub fn pace_frame(started: Instant, turn: u64, fps: u64) {
    let target = started + Duration::from_nanos(1_000_000_000 / fps * turn);
    let now = Instant::now();
    if target > now {
        std::thread::sleep(target - now);
    }
}
