//! Python panel — runtime detection, multi-project switching, environment/
//! venv info, installed/outdated/vulnerable package lists, framework
//! detection, quick actions, and a live process list.
//!
//! Brought up to the same level of detail as `dotnet_panel` (Projects,
//! Packages, Outdated, Vulnerabilities sections) — previously this panel
//! only showed a package *count*, no per-project switching, and no
//! outdated/vulnerability visibility at all. The multi-project scanner
//! (`python_backend::scan_python_projects`/`DetectedPythonProject`/
//! `EnvMarker`) and the vulnerability-scan HTTP plumbing
//! (`python_manager_panel::fetch_all_vulnerabilities`) are both shared with
//! `python_manager_panel` rather than duplicated — this panel already
//! depends on that crate for its "Package Manager" quick-action button, so
//! reusing its vulnerability fetcher doesn't add a new dependency edge.
//!
//! Ported standalone (no host-app wiring yet): the source panel depended on
//! a host `AppState` for four things, none of which exist here:
//!
//! - The initial/live-updating workspace root (`state.workspace_root`,
//!   `state.workspace_tx`) — this panel just uses its own process cwd
//!   instead, same as the other standalone-first ports.
//! - A live Python-process list, fed by subscribing to the Cockpit panel's
//!   own system-metrics tick broadcast and filtering it to "Python"
//!   (`state.tick_tx`). Reimplemented here as this panel's own lightweight
//!   `sysinfo` poll filtered by process name instead of cross-panel
//!   plumbing — see `spawn_processes_poll`.
//! - `run_in_script_runner` — the framework/entry-point quick-action
//!   buttons' "run this command" hook. `zed::zed::initialize_panels` hands
//!   this panel a `WeakEntity<ScriptRunnerPanel>` once both panels have
//!   loaded (see `set_script_runner`), so `dispatch_script` sends runs
//!   there for real — mirrors `node_panel`'s equivalent wiring.
//! - `open_python_manager` — re-added: opens the `python_manager_panel`
//!   workspace tab (installed/outdated/vulnerabilities, exact-name PyPI
//!   lookup, install/uninstall/update through the Script Runner), mirroring
//!   `node_panel`'s "Package Manager"/`dotnet_panel`'s "NuGet Manager"
//!   buttons. `open_task_chain_wizard` stays dropped — unrelated feature,
//!   no near-term port planned.
//!
//! Vulnerability scanning is on-demand (a button, not automatic on every
//! project switch) — same disclosed tradeoff `python_manager_panel`
//! documents: PyPI has no bulk-audit endpoint, so a scan is one HTTP
//! request per installed package.
//!
//! The actual detection/scanning/parsing logic lives in `python_backend`,
//! ported alongside this panel — see that crate's own doc comment.

mod python_panel_settings;

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use gpui::{
    Action, AnchoredPositionMode, AnyElement, App, AppContext as _, AsyncApp, AsyncWindowContext,
    Context, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, InteractiveElement as _,
    IntoElement, ParentElement as _, Pixels, Render, StatefulInteractiveElement as _, Styled as _,
    WeakEntity, Window, actions, anchored, deferred, div, point, prelude::FluentBuilder as _, px,
    svg,
};
use language::{LanguageName, Toolchain};
use gpui_component::{
    ActiveTheme as _, Disableable as _, Icon, IconName, Selectable as _, Sizable as _,
    button::{Button, ButtonCustomVariant, ButtonVariant, ButtonVariants as _},
    collapsible::Collapsible,
    h_flex,
    label::Label,
    panel_header::PanelHeader,
    spinner::Spinner,
    tag::Tag,
    v_flex,
};
use python_backend::{
    DetectedPythonProject, PyPiVulnerability, PythonFramework, PythonOutdatedPkg, PythonPackage,
    UpdateKind, count_requirements, detect_framework, find_missing_deps, list_outdated,
    list_packages, query_pip, query_python, scan_python_project, scan_python_projects,
};
use python_versions::{Download, PythonVersion};
use script_runner_panel::ScriptRunnerPanel;
use script_runner_panel::command::{check_package_name, check_version, shell_path, shell_program};
pub use python_panel_settings::PythonPanelSettings;
use settings::{DockSide, Settings as _};
use workspace::{Toast, notifications::NotificationId};
use sysinfo::System;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

/// How often the local Python-process poll re-scans the system.
const PYTHON_PROCS_TICK_INTERVAL: Duration = Duration::from_secs(2);

/// One Python-related process row — a process whose name contains "python"
/// (case-insensitive).
struct PythonProcess {
    name: String,
    pid: u32,
    cpu_percent: f32,
    memory_mb: f64,
}

/// The built-in version change the user has been asked to confirm, or is
/// waiting on. An install carries the whole [`Download`], so the question
/// names exactly what will be fetched: the version, its size and its source.
#[derive(Clone)]
enum PendingVersionAction {
    Install(Download),
    Uninstall(String),
}

impl PendingVersionAction {
    fn version(&self) -> &str {
        match self {
            PendingVersionAction::Install(download) => &download.version,
            PendingVersionAction::Uninstall(version) => version,
        }
    }

    fn verb(&self) -> &'static str {
        match self {
            PendingVersionAction::Install(_) => "install",
            PendingVersionAction::Uninstall(_) => "remove",
        }
    }
}

/// Where a built-in version change stands. Nothing is downloaded or removed
/// before `Confirm` has been answered.
#[derive(Clone)]
enum VersionStep {
    Confirm(PendingVersionAction),
    Running(PendingVersionAction),
    Done(PendingVersionAction, String),
    Error(PendingVersionAction, String),
}

/// Where the active interpreter comes from, for the header line.
#[derive(Clone, Copy, PartialEq, Eq)]
enum InterpreterSource {
    Venv,
    BuiltIn,
    System,
}

impl InterpreterSource {
    fn label(self) -> &'static str {
        match self {
            InterpreterSource::Venv => "venv",
            InterpreterSource::BuiltIn => "built-in",
            InterpreterSource::System => "system",
        }
    }
}

/// Sizes in a download question: `21.0 MB`.
fn megabytes(bytes: u64) -> String {
    format!("{:.1} MB", bytes as f64 / 1_048_576.0)
}

/// A background-loaded list with its three UI states — mirrors
/// `dotnet_panel::PackagesState`.
enum PackagesState<T> {
    Loading,
    Ready(Vec<T>),
    Error(String),
}

// ── Panel ──────────────────────────────────────────────────────────────

actions!(
    python_panel,
    [
        /// Toggles focus on the Python panel.
        ToggleFocus
    ]
);

/// Registers the Python panel's actions on every workspace. Call once at
/// app startup, alongside the other panels' `init` functions.
pub fn init(cx: &mut App) {
    // Makes the built-in Python reach past this panel: terminals, the Script
    // Runner and workflow actions all ask `ide_env` what to put first on a
    // new process's `PATH`. What goes there is the store's link to the active
    // version, the same folders whatever is active, so a terminal that is
    // already open follows a switch. A project's venv, activated in a
    // terminal or named by path, still comes before it.
    ide_env::register(|_directory| python_versions::live_bin_dirs(&python_versions::store_dir()));

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<PythonPanel>(window, cx);
        });
    })
    .detach();
}

pub struct PythonPanel {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    /// Where script runs (framework/entry-point quick actions, bulk
    /// updates) get dispatched. Handed in by `zed::zed::initialize_panels`
    /// once both this panel and the Script Runner panel are loaded — see
    /// `set_script_runner`.
    script_runner: Option<WeakEntity<ScriptRunnerPanel>>,

    /// Scan root — every detected sub-project lives under this.
    cwd: String,
    projects: Vec<DetectedPythonProject>,
    /// The currently active project's directory — packages/outdated/
    /// vulnerabilities/framework are all scoped to this, not always `cwd`.
    /// `None` when no sub-project was detected (falls back to `cwd` itself
    /// via `active_project_dir`).
    selected_project: Option<String>,
    scanning: bool,

    /// Interpreter resolved for the active project: its own venv when one
    /// exists, else the first working `python`/`python3` on PATH.
    python_exe: Option<String>,
    /// True when `python_exe` is a project-local venv interpreter rather
    /// than a global fallback.
    using_venv: bool,
    /// Where `python_exe` comes from, for the header line.
    interpreter_source: InterpreterSource,

    /// The built-in manager's installed versions, newest first.
    managed_versions: Vec<PythonVersion>,
    /// `None` while the first read of the store is pending; `Some(false)`
    /// when it failed.
    versions_loaded: Option<bool>,
    /// Whether the list of installable versions is shown.
    show_available: bool,
    /// Installable versions, once fetched. Reading them costs two small
    /// index files and no Python download.
    available_versions: Vec<Download>,
    loading_available: bool,
    available_error: Option<String>,
    /// The install or removal being asked about or carried out.
    version_step: Option<VersionStep>,

    py_ver: Option<String>,
    pip_ver: Option<String>,
    venv_path: Option<String>,
    framework: Option<PythonFramework>,
    entry_points: Vec<(String, String)>, // (file, dir)

    installed_pkgs: Vec<PythonPackage>,
    outdated: PackagesState<PythonOutdatedPkg>,

    /// Known vulnerabilities per installed package name, populated by
    /// `scan_vulnerabilities`. Flat map (not `PackagesState`) since a scan
    /// is user-triggered, not part of the automatic project-load flow.
    vulns: HashMap<String, Vec<PyPiVulnerability>>,
    vuln_scanning: bool,
    /// `None`/`false` until the first scan completes — distinguishes
    /// "never scanned" from "scanned, zero found" in the UI.
    vuln_scanned: bool,
    vuln_scan_error: Option<String>,

    /// First `requirements.txt` found by the last project reload, if any —
    /// kept so [`Self::recompute_missing_deps`] can be re-run whenever
    /// either half of its inputs (this or `installed_pkgs`) changes,
    /// without re-scanning the whole project.
    requirements_path: Option<String>,
    /// `count_requirements`'s result for `requirements_path`.
    requirements_count: usize,
    missing_deps: Vec<String>,

    python_procs: Vec<PythonProcess>,
    /// False until the first background process scan lands, so the
    /// Processes section shows a loading row instead of claiming nothing
    /// is running.
    procs_loaded: bool,

    open: HashMap<String, bool>,

    /// Debounces the filesystem-watcher rescan: replaced on every relevant
    /// `WorktreeUpdatedEntries` event, so a burst of updates collapses into a
    /// single `scan_projects` run once it goes quiet. Held only for its `Drop`
    /// (replacing it cancels the pending run).
    _rescan_task: Option<gpui::Task<()>>,

    /// Same debounce, but for reloading the active project's data when its
    /// manifest changes on disk.
    _data_reload_task: Option<gpui::Task<()>>,

    _procs_poll: gpui::Task<()>,
}

/// The opened project's first worktree, which the scan is rooted on — the
/// same choice `node_panel` and `dotnet_panel` make. `None` while no worktree
/// is open.
fn project_root(workspace: &Workspace, cx: &mut Context<Workspace>) -> Option<String> {
    workspace
        .project()
        .read(cx)
        .worktrees(cx)
        .next()
        .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().into_owned())
}

/// Displays a path by its final component (folder/file name), falling back
/// to the full path when it has none — shared by the dock panels' dashboard
/// accessors.
fn path_file_name(dir: &str) -> String {
    std::path::Path::new(dir)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| dir.to_string())
}

/// The marker files `scan_python_projects` recognizes a project by. A change
/// to any of these can mean a project was added or removed, so the Projects
/// list is re-scanned for one.
fn is_python_project_marker(name: &str) -> bool {
    matches!(name, "requirements.txt" | "pyproject.toml" | "Pipfile")
}

/// Files the panel derives package/environment data from for the active
/// project — its project markers plus common lock/`setup` files. A change to
/// any of these means the installed/outdated/missing-deps state is stale.
fn is_python_manifest_file(name: &str) -> bool {
    is_python_project_marker(name)
        || matches!(
            name,
            "Pipfile.lock" | "poetry.lock" | "setup.py" | "setup.cfg" | "uv.lock"
        )
}

/// Whether a changed worktree-relative path (`/`-separated, relative to the
/// scan root `cwd`) sits directly in `selected` — i.e. it is that project's
/// own manifest, not some other project's under the same workspace.
/// Separators in `cwd`/`selected` may be either style.
fn manifest_change_is_in_project(cwd: &str, selected: &str, rel_unix: &str) -> bool {
    let root = cwd.replace('\\', "/");
    let changed = format!("{}/{}", root.trim_end_matches('/'), rel_unix);
    changed
        .rsplit_once('/')
        .is_some_and(|(dir, _)| dir == selected.replace('\\', "/").trim_end_matches('/'))
}

impl PythonPanel {
    /// Detected Python version, for a future dashboard-style "runtime
    /// versions" summary (the source panel's Cockpit dashboard exposed this
    /// the same way, via a plain accessor).
    pub fn python_version(&self) -> Option<&str> {
        self.py_ver.as_deref()
    }

    /// Every detected Python sub-project, for the Cockpit dashboard's
    /// Runtimes section (matches `node_panel`/`dotnet_panel`'s equivalent).
    pub fn detected_projects(&self) -> &[DetectedPythonProject] {
        &self.projects
    }

    /// Whether a vulnerability scan has completed at least once for the
    /// active project — Python's scan is on-demand (a button), unlike
    /// Node/`.NET`'s automatic-on-load scan, so the dashboard needs to tell
    /// "never scanned" apart from "scanned, zero found".
    pub fn vulnerabilities_scanned(&self) -> bool {
        self.vuln_scanned
    }

    /// Loads the panel for a workspace, following the same
    /// `WeakEntity<Workspace>` + `AsyncWindowContext` convention as the
    /// other dock panels' `load` functions (see `initialize_panels` in
    /// `zed::zed`), so it can be added to the dock alongside them.
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            PythonPanel::new(workspace, window, cx)
        })
    }

    pub fn new(
        _workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        // Scan the opened project, as the Node and .NET panels do. This used
        // to be the process's working directory: whatever folder the IDE was
        // started from, unrelated to the project open in it.
        let root = project_root(&*_workspace, cx).unwrap_or_else(|| {
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_default()
        });

        // The live worktrees watch the filesystem (the same machinery the
        // project panel relies on); subscribing here lets `scan_projects`
        // rerun when a new project marker appears, instead of only once at
        // startup.
        let project = _workspace.project().clone();

        let workspace = cx.entity().downgrade();

        cx.new(|cx| {
            cx.subscribe(&project, |this: &mut PythonPanel, _project, event, cx| {
                if let project::Event::WorktreeUpdatedEntries(_, entries) = event {
                    let mut marker_touched = false;
                    let mut selected_manifest_touched = false;
                    for (path, _, _) in entries.iter() {
                        let Some(name) = path.file_name() else {
                            continue;
                        };
                        if is_python_project_marker(name) {
                            marker_touched = true;
                        }
                        if is_python_manifest_file(name)
                            && this.manifest_change_is_selected(path.as_unix_str())
                        {
                            selected_manifest_touched = true;
                        }
                    }
                    // A new/removed marker can mean a new project, so the
                    // Projects list is always re-scanned for one.
                    if marker_touched {
                        this.schedule_project_rescan(cx);
                    }
                    // Packages only change the *active* project's data, so only
                    // reload that one (avoids re-running pip probes for
                    // unrelated edits elsewhere).
                    if selected_manifest_touched {
                        this.schedule_data_reload(cx);
                    }
                }
            })
            .detach();

            let cwd = root.clone();

            let mut open = HashMap::new();
            // Keep all sections open by default
            open.insert("Python Versions".to_string(), true);
            open.insert("Projects".to_string(), true);
            open.insert("Environment".to_string(), true);
            open.insert("Framework".to_string(), true);
            open.insert("Quick Actions".to_string(), true);
            open.insert("Packages".to_string(), true);
            open.insert("Outdated".to_string(), true);
            open.insert("Vulnerabilities".to_string(), true);
            open.insert("Processes".to_string(), true);

            let mut panel = PythonPanel {
                focus_handle: cx.focus_handle(),
                workspace,
                script_runner: None,
                cwd: cwd.clone(),
                projects: Vec::new(),
                selected_project: None,
                scanning: false,
                python_exe: None,
                using_venv: false,
                interpreter_source: InterpreterSource::System,
                managed_versions: Vec::new(),
                versions_loaded: None,
                show_available: false,
                available_versions: Vec::new(),
                loading_available: false,
                available_error: None,
                version_step: None,
                py_ver: None,
                pip_ver: None,
                venv_path: None,
                framework: None,
                entry_points: Vec::new(),
                installed_pkgs: Vec::new(),
                outdated: PackagesState::Ready(Vec::new()),
                vulns: HashMap::new(),
                vuln_scanning: false,
                vuln_scanned: false,
                vuln_scan_error: None,
                requirements_path: None,
                requirements_count: 0,
                missing_deps: Vec::new(),
                python_procs: Vec::new(),
                procs_loaded: false,
                open,
                _rescan_task: None,
                _data_reload_task: None,
                _procs_poll: Self::spawn_processes_poll(cx),
            };

            if !cwd.is_empty() {
                panel.scan_projects(cx);
            }
            panel.load_managed_versions(cx);

            panel
        })
    }

    // ── Built-in versions ──────────────────────────────────────────────

    /// Re-reads the installed built-in versions. Reads the disk, so off the
    /// main thread.
    fn load_managed_versions(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async { python_versions::list(&python_versions::store_dir()) })
                .await;
            this.update(cx, |panel, cx| {
                match result {
                    Ok(versions) => {
                        panel.versions_loaded = Some(true);
                        panel.managed_versions = versions;
                    }
                    Err(_) => panel.versions_loaded = Some(false),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Shows or hides the installable versions, fetching the list the first
    /// time. That is two small index files; no Python is downloaded until an
    /// install has been confirmed.
    fn toggle_available(&mut self, cx: &mut Context<Self>) {
        self.show_available = !self.show_available;
        if self.show_available && self.available_versions.is_empty() && !self.loading_available {
            self.loading_available = true;
            self.available_error = None;
            let http = cx.http_client();
            cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
                let result = cx
                    .background_spawn(async move { python_versions::list_available(http).await })
                    .await;
                this.update(cx, |panel, cx| {
                    panel.loading_available = false;
                    match result {
                        Ok(versions) => panel.available_versions = versions,
                        Err(error) => panel.available_error = Some(error),
                    }
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
        cx.notify();
    }

    /// Asks before downloading `download`. The question is shown at once;
    /// the download's size is filled in when the server has said it.
    fn request_install(&mut self, download: Download, cx: &mut Context<Self>) {
        self.version_step = Some(VersionStep::Confirm(PendingVersionAction::Install(
            download.clone(),
        )));
        cx.notify();

        let http = cx.http_client();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let sized = cx
                .background_spawn(async move { python_versions::with_size(http, download).await })
                .await;
            this.update(cx, |panel, cx| {
                // Only while the same question is still open.
                if let Some(VersionStep::Confirm(PendingVersionAction::Install(asked))) =
                    &panel.version_step
                    && asked.version == sized.version
                {
                    panel.version_step = Some(VersionStep::Confirm(
                        PendingVersionAction::Install(sized),
                    ));
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn request_uninstall(&mut self, version: String, cx: &mut Context<Self>) {
        self.version_step = Some(VersionStep::Confirm(PendingVersionAction::Uninstall(version)));
        cx.notify();
    }

    fn cancel_version_step(&mut self, cx: &mut Context<Self>) {
        self.version_step = None;
        cx.notify();
    }

    /// Carries out the confirmed install or removal. The only place a Python
    /// is downloaded.
    fn confirm_version_action(&mut self, cx: &mut Context<Self>) {
        let Some(VersionStep::Confirm(action)) = self.version_step.clone() else {
            return;
        };
        self.version_step = Some(VersionStep::Running(action.clone()));
        cx.notify();

        let http = cx.http_client();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let work = action.clone();
            let result = cx
                .background_spawn(async move {
                    let store = python_versions::store_dir();
                    match work {
                        PendingVersionAction::Install(download) => {
                            python_versions::install(&store, &download, http)
                                .await
                                .map(|installed| {
                                    format!("Installed to {}", installed.dir.to_string_lossy())
                                })
                        }
                        PendingVersionAction::Uninstall(version) => {
                            python_versions::uninstall(&store, &version)
                        }
                    }
                })
                .await;
            this.update(cx, |panel, cx| {
                panel.version_step = Some(match result {
                    Ok(output) => VersionStep::Done(action, output),
                    Err(error) => VersionStep::Error(action, error),
                });
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn finish_version_step(&mut self, cx: &mut Context<Self>) {
        let installed = matches!(
            self.version_step,
            Some(VersionStep::Done(PendingVersionAction::Install(_), _))
        );
        self.version_step = None;
        if installed {
            self.show_available = false;
        }
        self.after_version_change(cx);
    }

    /// Makes `version` the active one, or with `None` hands the choice back to
    /// the machine's own Python. Only changes what the IDE starts.
    fn switch_version(&mut self, version: Option<String>, cx: &mut Context<Self>) {
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move {
                    let store = python_versions::store_dir();
                    match version {
                        Some(version) => python_versions::use_version(&store, &version),
                        None => python_versions::use_system(&store),
                    }
                })
                .await;
            this.update(cx, |panel, cx| {
                if let Err(error) = result {
                    panel.report(error, cx);
                }
                panel.after_version_change(cx);
            })
            .ok();
        })
        .detach();
    }

    /// An install, removal or switch can change the interpreter a project
    /// without a venv uses, so its data is re-read along with the list.
    fn after_version_change(&mut self, cx: &mut Context<Self>) {
        self.load_managed_versions(cx);
        self.reload_project_data(cx);
        cx.notify();
    }

    /// The interpreter as it can be written into a command the Script Runner
    /// runs from `dir`. The built-in Python is first on that command's `PATH`,
    /// so it is named plainly, not by a path under the user's profile that
    /// may hold spaces.
    fn command_program(&self, dir: &str) -> Result<String, String> {
        let exe = self
            .python_exe
            .clone()
            .unwrap_or_else(|| "python".to_string());
        if self.interpreter_source == InterpreterSource::BuiltIn {
            return Ok("python".to_string());
        }
        shell_program(&exe, dir)
    }

    fn is_open(&self, key: &str, default: bool) -> bool {
        *self.open.get(key).unwrap_or(&default)
    }

    fn toggle(&mut self, key: &str) {
        let e = self.open.entry(key.to_string()).or_insert(true);
        *e = !*e;
    }

    /// The directory quick actions/reloads operate on: the selected
    /// sub-project when one was detected, else the whole scan root.
    fn active_project_dir(&self) -> String {
        self.selected_project
            .clone()
            .unwrap_or_else(|| self.cwd.clone())
    }

    /// Called once by `zed::zed::initialize_panels` after both this panel
    /// and the Script Runner panel have loaded, so `dispatch_script` has
    /// somewhere to send runs — mirrors `node_panel::set_script_runner`.
    pub fn set_script_runner(&mut self, script_runner: WeakEntity<ScriptRunnerPanel>) {
        self.script_runner = Some(script_runner);
    }

    /// Sends `command` to the Script Runner panel, run from `cwd`.
    fn dispatch_script(
        &mut self,
        command: &str,
        cwd: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(script_runner) = self.script_runner.as_ref().and_then(|w| w.upgrade()) else {
            return;
        };
        // The runner can only stream one command at a time — see
        // `node_panel::dispatch_script`'s identical guard for why.
        if script_runner.read(cx).is_running() {
            return;
        }
        // Surface the run: reveal the Script Runner dock panel. Uses the
        // entity-agnostic `App::defer` (not `WeakEntity::update_in`, and
        // not `Context::defer_in`) — both of those re-lease `self`
        // (PythonPanel) to invoke their callback, and `reveal_panel` reads
        // every docked panel's zoom state including this one, which
        // panics with a "double lease" error if it's still leased. See
        // `node_panel::dispatch_script`'s identical fix for the full
        // writeup.
        let workspace = self.workspace.clone();
        let window_handle = window.window_handle();
        cx.defer(move |app| {
            let _ = app.update_window(window_handle, |_, window, cx| {
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        workspace.reveal_panel::<ScriptRunnerPanel>(window, cx);
                    });
                }
            });
        });

        let command = command.to_string();
        script_runner.update(cx, |panel, cx| {
            panel.run_external(command, cwd, cx);
        });
    }

    /// Bulk-updates every outdated package whose risk is `Patch` (or
    /// `Patch`/`Minor` combined, when `ty` isn't `"patch"`) — one
    /// `pip install --upgrade "name==latest"` per target, chained with
    /// `&&` in a single Script Runner run. Mirrors
    /// `dotnet_panel::update_all`.
    fn update_all(&mut self, ty: &str, window: &mut Window, cx: &mut Context<Self>) {
        let PackagesState::Ready(list) = &self.outdated else {
            return;
        };
        let dir = self.active_project_dir();
        let command = self
            .command_program(&dir)
            .and_then(|exe| update_all_command(&exe, list, ty));
        match command {
            Ok(Some(command)) => self.dispatch_script(&command, dir, window, cx),
            Ok(None) => {}
            Err(error) => self.report(error, cx),
        }
    }

    /// Tells the user why a command was refused (see
    /// `script_runner_panel::command`). A toast, since this panel has no
    /// error row of its own for run actions.
    fn report(&self, message: String, cx: &mut App) {
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.show_toast(Toast::new(NotificationId::unique::<PythonPanel>(), message), cx);
            })
            .ok();
    }

    /// Fires one `https://pypi.org/pypi/<name>/<version>/json` request per
    /// installed package (capped concurrency, no bulk endpoint exists) via
    /// `python_manager_panel::fetch_all_vulnerabilities` — reused as-is
    /// rather than duplicated, since this panel already depends on that
    /// crate.
    fn scan_vulnerabilities(&mut self, cx: &mut Context<Self>) {
        if self.vuln_scanning || self.installed_pkgs.is_empty() {
            return;
        }
        let pkgs: Vec<(String, String)> = self
            .installed_pkgs
            .iter()
            .map(|p| (p.name.clone(), p.version.clone()))
            .collect();
        self.vuln_scanning = true;
        self.vuln_scan_error = None;
        cx.notify();

        let http_client = cx.http_client();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = python_manager_panel::fetch_all_vulnerabilities(&http_client, pkgs).await;

            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.vuln_scanning = false;
                        panel.vuln_scanned = true;
                        match result {
                            Ok(map) => {
                                panel.vulns = map;
                                panel.vuln_scan_error = None;
                            }
                            Err(e) => panel.vuln_scan_error = Some(e),
                        }
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    /// Refreshes `python_procs` from a locally-owned `sysinfo::System` every
    /// `PYTHON_PROCS_TICK_INTERVAL`, filtered to process names containing
    /// "python" — replaces subscribing to a host cockpit panel's broadcast
    /// tick (see the module doc).
    fn spawn_processes_poll(cx: &mut Context<Self>) -> gpui::Task<()> {
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            // Every sysinfo call here walks the whole process table, which
            // takes long enough to stall the UI, so the scans run on the
            // background executor and only the resulting rows hop back to
            // the main thread via `this.update`.
            let mut sys = cx.background_spawn(async { System::new_all() }).await;
            loop {
                let (scanned, procs) = cx
                    .background_spawn(async move {
                        sys.refresh_cpu_all();
                        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
                        let cpu_count = sys.cpus().len() as f32;
                        let procs: Vec<PythonProcess> = sys
                            .processes()
                            .iter()
                            .filter(|(_, p)| {
                                p.name().to_string_lossy().to_lowercase().contains("python")
                            })
                            .map(|(pid, p)| PythonProcess {
                                name: p.name().to_string_lossy().to_string(),
                                pid: pid.as_u32(),
                                cpu_percent: p.cpu_usage() / cpu_count,
                                memory_mb: p.memory() as f64 / 1_048_576.0,
                            })
                            .collect();
                        (sys, procs)
                    })
                    .await;
                sys = scanned;

                let alive = this
                    .update(cx, |this, cx| {
                        this.python_procs = procs;
                        this.procs_loaded = true;
                        cx.notify();
                    })
                    .is_ok();
                if !alive {
                    break;
                }

                cx.background_executor()
                    .timer(PYTHON_PROCS_TICK_INTERVAL)
                    .await;
            }
        })
    }

    /// Diffs `requirements_path` against `installed_pkgs` into
    /// `missing_deps`. Cheap enough (a single file read + set diff) to run
    /// synchronously.
    fn recompute_missing_deps(&mut self) {
        self.missing_deps =
            find_missing_deps(self.requirements_path.as_deref(), &self.installed_pkgs)
                .unwrap_or_default();
    }

    /// Scans `cwd` for sub-projects, picks (or keeps) the selected one, then
    /// reloads everything scoped to it. The initial entry point — replaces
    /// the old disjoint `detect_python`/`load_packages`/`load_pip_version`/
    /// `scan_project` tasks with one project-aware flow, mirroring
    /// `dotnet_panel::scan_projects` and `python_manager_panel`'s
    /// `schedule_load`.
    fn scan_projects(&mut self, cx: &mut Context<Self>) {
        if self.cwd.is_empty() {
            self.projects.clear();
            return;
        }
        self.scanning = true;
        cx.notify();

        let root = self.cwd.clone();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let projects = cx
                .background_spawn(async move { scan_python_projects(&root, 0) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.projects = projects;
                        // Keep the current selection across rescans; only
                        // (re)select when it's empty or the project it pointed
                        // at disappeared (e.g. a scaffold removed it).
                        let selection_still_exists = panel
                            .selected_project
                            .as_ref()
                            .is_some_and(|selected| {
                                panel.projects.iter().any(|p| &p.path == selected)
                            });
                        if !selection_still_exists {
                            panel.selected_project =
                                panel.projects.first().map(|p| p.path.clone());
                        }
                        panel.reload_project_data(cx);
                    });
                }
            });
        })
        .detach();
    }

    /// Selects a different detected project and rescopes everything to it.
    fn select_project(&mut self, path: String, cx: &mut Context<Self>) {
        self.selected_project = Some(path);
        self.reload_project_data(cx);
    }

    /// Coalesces the burst of `WorktreeUpdatedEntries` events a scaffold
    /// produces into one rescan: each call replaces (and so cancels) the
    /// pending task, and the last one to survive the debounce reruns
    /// `scan_projects`.
    fn schedule_project_rescan(&mut self, cx: &mut Context<Self>) {
        self._rescan_task = Some(cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            cx.background_executor()
                .timer(Duration::from_millis(500))
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.scan_projects(cx);
                    });
                }
            });
        }));
    }

    /// Whether a changed worktree-relative path (`/`-separated) sits directly
    /// in the active project — i.e. it is that project's own manifest, not
    /// some other project's under the same workspace.
    fn manifest_change_is_selected(&self, rel_unix: &str) -> bool {
        let Some(selected) = self.selected_project.as_deref() else {
            return false;
        };
        manifest_change_is_in_project(&self.cwd, selected, rel_unix)
    }

    /// Reloads the active project's interpreter/environment/packages after its
    /// manifest changed on disk — an install/remove/update run outside the
    /// panel (terminal, `python_manager_panel`, pip itself), or a hand edit.
    /// Debounced like `schedule_project_rescan`.
    fn schedule_data_reload(&mut self, cx: &mut Context<Self>) {
        self._data_reload_task =
            Some(cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
                cx.background_executor()
                    .timer(Duration::from_millis(500))
                    .await;
                let _ = cx.update(|app| {
                    if let Some(panel) = this.upgrade() {
                        let _ = panel.update(app, |panel, cx| {
                            panel.reload_project_data(cx);
                        });
                    }
                });
            }));
    }

    /// Looks up whatever interpreter the user has explicitly picked for
    /// `scan_root` via Zed's own toolchain selector (the status-bar
    /// interpreter picker, backed by `pet_core` — aware of conda/poetry/
    /// pipenv/pyenv/system installs, not just a bare `.venv` check). `None`
    /// when there's no workspace, the path isn't inside any worktree, or no
    /// toolchain has been resolved/selected for it yet — `reload_project_data`
    /// falls back to this panel's own simpler venv/PATH guess in that case.
    /// This exists so the panel's package scan never disagrees with what the
    /// language server and integrated terminal are actually running.
    fn active_toolchain_task(
        &self,
        scan_root: &str,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Task<Option<Toolchain>>> {
        let workspace = self.workspace.upgrade()?;
        workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            let project_path = project
                .read(cx)
                .project_path_for_absolute_path(Path::new(scan_root), cx)?;
            Some(project.read(cx).active_toolchain(
                project_path,
                LanguageName::new_static("Python"),
                cx,
            ))
        })
    }

    /// Re-derives everything scoped to `active_project_dir()`: the resolved
    /// interpreter (Zed's own selected toolchain when there is one, else
    /// this panel's own project venv, else global `python`/`python3`),
    /// Python/pip versions, framework/entry points, installed + outdated
    /// packages, and the missing-deps diff. One background task, mirroring
    /// `python_manager_panel::schedule_load`'s consolidated shape rather
    /// than the several separately-racing tasks this panel used to run.
    fn reload_project_data(&mut self, cx: &mut Context<Self>) {
        self.scanning = true;
        self.outdated = PackagesState::Loading;
        // Vulnerabilities are keyed by installed-package name/version and
        // become stale the moment the active project (and therefore its
        // installed set) changes — clear them rather than show a scan
        // result for a different project's packages.
        self.vulns.clear();
        self.vuln_scanned = false;
        self.vuln_scan_error = None;
        cx.notify();

        let scan_root = self.active_project_dir();
        let toolchain_task = self.active_toolchain_task(&scan_root, cx);
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let toolchain_exe = match toolchain_task {
                Some(task) => task.await.map(|toolchain| toolchain.path.to_string()),
                None => None,
            };

            let scan_root_for_task = scan_root.clone();
            let result = cx
                .background_spawn(async move {
                    let scan = scan_python_project(&scan_root_for_task).ok();
                    let venv_exe = scan.as_ref().and_then(|s| s.venvs.first().cloned());

                    // Without a venv, the built-in Python comes before the
                    // machine's when one is active: it is what this panel
                    // installed for exactly that case.
                    let store = python_versions::store_dir();
                    let (exe, using_venv) = if let Some(exe) = toolchain_exe {
                        (Some(exe), true)
                    } else if let Some(venv) = venv_exe {
                        (Some(venv), true)
                    } else if let Some(built_in) = python_versions::active(&store) {
                        (Some(built_in.python.to_string_lossy().into_owned()), false)
                    } else {
                        let mut found = None;
                        for candidate in ["python", "python3"] {
                            if query_python(candidate).is_ok() {
                                found = Some(candidate.to_string());
                                break;
                            }
                        }
                        (found, false)
                    };
                    // Zed's own interpreter picker can point at a built-in
                    // version too, so this is decided by the path, not by
                    // which branch above found it.
                    let source = match &exe {
                        Some(exe) if python_versions::is_built_in(&store, Path::new(exe)) => {
                            InterpreterSource::BuiltIn
                        }
                        _ if using_venv => InterpreterSource::Venv,
                        _ => InterpreterSource::System,
                    };

                    let (py_ver, pip_ver, installed, outdated) = match &exe {
                        Some(exe) => (
                            query_python(exe).ok(),
                            query_pip(exe).ok(),
                            list_packages(exe).unwrap_or_default(),
                            list_outdated(exe),
                        ),
                        None => (None, None, Vec::new(), Ok(Vec::new())),
                    };

                    let framework = scan
                        .as_ref()
                        .and_then(|s| detect_framework(&s.requirements, &s.entry_points));
                    let entry_points = scan
                        .as_ref()
                        .map(|s| s.entry_points.clone())
                        .unwrap_or_default();
                    let requirements_path =
                        scan.as_ref().and_then(|s| s.requirements.first().cloned());
                    let requirements_count =
                        count_requirements(requirements_path.as_deref()).unwrap_or(0);

                    (
                        exe,
                        using_venv,
                        source,
                        py_ver,
                        pip_ver,
                        installed,
                        outdated,
                        framework,
                        entry_points,
                        requirements_path,
                        requirements_count,
                    )
                })
                .await;

            let (
                exe,
                using_venv,
                source,
                py_ver,
                pip_ver,
                installed,
                outdated,
                framework,
                entry_points,
                requirements_path,
                requirements_count,
            ) = result;

            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.scanning = false;
                        panel.python_exe = exe;
                        panel.using_venv = using_venv;
                        panel.interpreter_source = source;
                        panel.venv_path = if source == InterpreterSource::Venv {
                            panel.python_exe.clone()
                        } else {
                            None
                        };
                        panel.py_ver = py_ver;
                        panel.pip_ver = pip_ver;
                        panel.installed_pkgs = installed;
                        panel.outdated = match outdated {
                            Ok(list) => PackagesState::Ready(list),
                            Err(e) => PackagesState::Error(e),
                        };
                        panel.framework = framework;
                        panel.entry_points = entry_points;
                        panel.requirements_path = requirements_path;
                        panel.requirements_count = requirements_count;
                        panel.recompute_missing_deps();
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }
}

impl Focusable for PythonPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for PythonPanel {}

impl Panel for PythonPanel {
    fn persistent_name() -> &'static str {
        "Python Panel"
    }

    fn panel_key() -> &'static str {
        "PythonPanel"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        match PythonPanelSettings::get_global(cx).dock {
            DockSide::Left => DockPosition::Left,
            DockSide::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        settings::update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
            let dock = match position {
                DockPosition::Left | DockPosition::Bottom => DockSide::Left,
                DockPosition::Right => DockSide::Right,
            };
            settings.python_panel.get_or_insert_default().dock = Some(dock);
        });
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(280.)
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<ui::IconName> {
        PythonPanelSettings::get_global(cx).button.then_some(ui::IconName::Python)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Python")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        25
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.python_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

impl Render for PythonPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let panel_header = PanelHeader::new("Python").icon(
            svg()
                .path("icons/python.svg")
                .size(px(16.0))
                .text_color(theme.foreground),
        );

        // No early "Python not found" screen: with no Python on the machine,
        // the versions section below is how one gets installed.
        let versions_section = section_container(
            cx,
            "Python Versions",
            self.managed_versions.len(),
            self.is_open("Python Versions", true),
            self.render_python_versions(cx),
        );

        let projects_section = section_container(
            cx,
            "Projects",
            self.projects.len(),
            self.is_open("Projects", true),
            self.render_projects(cx),
        );

        let env_section = section_container(
            cx,
            "Environment",
            0,
            self.is_open("Environment", true),
            self.render_environment(),
        );

        let framework_section = self.framework.as_ref().map(|_| {
            section_container(
                cx,
                "Framework",
                0,
                self.is_open("Framework", true),
                self.render_framework(),
            )
        });

        let actions_section = section_container(
            cx,
            "Quick Actions",
            0,
            self.is_open("Quick Actions", true),
            self.render_quick_actions(cx),
        );

        let packages_section = section_container(
            cx,
            "Packages",
            self.installed_pkgs.len(),
            self.is_open("Packages", true),
            self.render_packages(cx),
        );

        let outdated_section = section_container(
            cx,
            "Outdated",
            self.outdated_count(),
            self.is_open("Outdated", true),
            self.render_outdated(cx),
        );

        let vulnerabilities_section = section_container(
            cx,
            "Vulnerabilities",
            self.vulnerable_count(),
            self.is_open("Vulnerabilities", true),
            self.render_vulnerabilities(cx),
        );

        let processes_section = section_container(
            cx,
            "Processes",
            self.python_procs.len(),
            self.is_open("Processes", true),
            self.render_processes(cx),
        );

        let mut body = div()
            .flex()
            .flex_col()
            .child(self.render_version_info(cx))
            .child(versions_section)
            .child(projects_section)
            .child(env_section);

        if let Some(fw_section) = framework_section {
            body = body.child(fw_section);
        }

        body = body
            .child(actions_section)
            .child(packages_section)
            .child(outdated_section)
            .child(vulnerabilities_section)
            .child(processes_section);

        div()
            .id("python-panel")
            .track_focus(&self.focus_handle(cx))
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .overflow_hidden()
            .bg(theme.background)
            .child(panel_header)
            .child(
                div()
                    .id("python-scroll")
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .overflow_y_scroll()
                    .child(body),
            )
            .children(self.render_version_modal(window, cx))
            .into_any_element()
    }
}

// ── Render sections ───────────────────────────────────────────────────

impl PythonPanel {
    /// Outdated-package count, also used by the Cockpit dashboard's
    /// Runtimes section (see `dashboard_panel`).
    pub fn outdated_count(&self) -> usize {
        match &self.outdated {
            PackagesState::Ready(list) => list.len(),
            _ => 0,
        }
    }

    /// Vulnerable-package count, summed across every installed package's
    /// findings. `0` when a scan hasn't run yet — see
    /// `vulnerabilities_scanned` to distinguish "not scanned" from
    /// "scanned, none found". Also used by the Cockpit dashboard's
    /// Runtimes section (see `dashboard_panel`).
    pub fn vulnerable_count(&self) -> usize {
        self.vulns.values().map(|v| v.len()).sum()
    }

    /// Every vulnerability finding, keyed by installed package name. Only
    /// packages with at least one finding appear in the map — the Cockpit
    /// dashboard's Security section renders per-package, per-finding rows
    /// from this.
    pub fn vulnerabilities(&self) -> &HashMap<String, Vec<PyPiVulnerability>> {
        &self.vulns
    }

    /// Whether a vulnerability scan is currently in flight.
    pub fn vulnerabilities_scanning(&self) -> bool {
        self.vuln_scanning
    }

    /// The scan's error message, if the last scan failed.
    pub fn vulnerabilities_scan_error(&self) -> Option<&str> {
        self.vuln_scan_error.as_deref()
    }

    /// The active scan target's display label — the selected sub-project's
    /// folder name when one was detected, else the scan root's.
    pub fn active_project_label(&self) -> String {
        let dir = self.selected_project.as_ref().unwrap_or(&self.cwd);
        path_file_name(dir)
    }

    /// Re-runs the on-demand vulnerability scan, driven by the Cockpit
    /// dashboard's Security section (`scan_vulnerabilities` does the work;
    /// it no-ops while a scan is in flight or no packages are installed).
    pub fn rescan_vulnerabilities(&mut self, cx: &mut Context<Self>) {
        self.scan_vulnerabilities(cx);
    }

    fn render_version_info(&self, cx: &Context<Self>) -> impl IntoElement {
        // `pip_ver` is `pip --version`'s raw stdout, e.g. "pip 26.1.2 from
        // C:\Program Files\WindowsApps\...  (python 3.13)" — the install
        // path/interpreter suffix doesn't fit this compact header line, so
        // just keep the "pip <version>" prefix, up to " from ".
        let pip_short = self
            .pip_ver
            .as_deref()
            .map(|v| v.split(" from ").next().unwrap_or(v).trim().to_string());

        // Says where the interpreter comes from, since a venv, a built-in
        // version and the machine's own can all report similar versions.
        let source = self.interpreter_source.label();
        // `python --version` already says "Python 3.13.14"; keep just the
        // number so the line doesn't read "Python Python …".
        let py_ver = self
            .py_ver
            .as_deref()
            .map(|v| v.trim().trim_start_matches("Python").trim().to_string());
        let text = match (&py_ver, &pip_short) {
            (Some(py), Some(pip)) => format!("Python {} ({source})  |  {}", py, pip),
            (Some(py), None) => format!("Python {} ({source})", py),
            _ if self.scanning => "Looking for Python\u{2026}".to_string(),
            _ => "Python not found \u{2014} install a version below".to_string(),
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .font_family("Cascadia Mono")
                    .text_xs()
                    .text_color(cx.theme().primary)
                    .child(text),
            )
    }

    /// The built-in versions: the machine's own Python, every installed
    /// version (click to make it active), and the installable ones. Picking
    /// one to install only opens the question; nothing is downloaded before
    /// it has been answered.
    fn render_python_versions(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();

        match self.versions_loaded {
            None => {
                return col.child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .px_3()
                        .py_2()
                        .child(Spinner::new().xsmall())
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Loading Python versions\u{2026}"),
                        ),
                );
            }
            Some(false) => {
                return col.child(
                    div()
                        .px_3()
                        .py_2()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("Couldn't read the installed Python versions"),
                );
            }
            Some(true) => {}
        }

        col = col.child(
            div().px_3().py_0p5().child(
                Button::new("python-version-available-toggle")
                    .ghost()
                    .xsmall()
                    .selected(self.show_available)
                    .label(if self.show_available {
                        "Hide Available"
                    } else {
                        "Install Version\u{2026}"
                    })
                    .on_click(cx.listener(|this, _e, _w, cx| this.toggle_available(cx))),
            ),
        );

        if self.managed_versions.is_empty() {
            col = col.child(
                div()
                    .px_3()
                    .py_1()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(
                        "No built-in versions yet. One can be downloaded into the IDE's own \
                         folder, without installing anything on this machine.",
                    ),
            );
        }

        let marker = |current: bool| {
            div()
                .w_3()
                .flex_shrink_0()
                .text_color(if current {
                    cx.theme().primary
                } else {
                    cx.theme().muted_foreground
                })
                .child(if current { "\u{25CF}" } else { "\u{25CB}" })
        };
        let row_color = |current: bool| {
            if current {
                cx.theme().primary
            } else {
                cx.theme().foreground
            }
        };

        // The machine's own Python, in use whenever no built-in version is
        // active (and no venv takes over).
        let system_current = !self.managed_versions.iter().any(|v| v.current);
        col = col.child(
            h_flex()
                .gap_2()
                .items_center()
                .px_3()
                .py_0p5()
                .text_xs()
                .child(marker(system_current))
                .child(
                    Button::new("python-version-system")
                        .custom(ButtonCustomVariant::new(cx).foreground(row_color(system_current)))
                        .xsmall()
                        .label("System")
                        .tooltip(if system_current {
                            "In use: the Python already on this machine"
                        } else {
                            "Use the Python already on this machine"
                        })
                        .on_click(cx.listener(|this, _e, _w, cx| this.switch_version(None, cx))),
                ),
        );

        for version in &self.managed_versions {
            let name = version.version.clone();
            let for_switch = name.clone();
            let for_remove = name.clone();
            col = col.child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .px_3()
                    .py_0p5()
                    .text_xs()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .flex_1()
                            .min_w_0()
                            .child(marker(version.current))
                            .child(
                                Button::new(format!("python-version-use-{name}"))
                                    .custom(
                                        ButtonCustomVariant::new(cx)
                                            .foreground(row_color(version.current)),
                                    )
                                    .xsmall()
                                    .label(if python_versions::is_prerelease(&name) {
                                        format!("{name} (pre-release)")
                                    } else {
                                        name.clone()
                                    })
                                    .tooltip(if version.current {
                                        "Active version"
                                    } else {
                                        "Use this version"
                                    })
                                    .on_click(cx.listener(move |this, _e, _w, cx| {
                                        this.switch_version(Some(for_switch.clone()), cx);
                                    })),
                            ),
                    )
                    .child(
                        Button::new(format!("python-version-remove-{name}"))
                            .custom(
                                ButtonCustomVariant::new(cx)
                                    .foreground(cx.theme().muted_foreground),
                            )
                            .xsmall()
                            .icon(IconName::CircleX)
                            .tooltip("Remove")
                            .on_click(cx.listener(move |this, _e, _w, cx| {
                                this.request_uninstall(for_remove.clone(), cx);
                            })),
                    ),
            );
        }

        if !self.show_available {
            return col;
        }

        col = col.child(div().mx_3().mt_1().border_t_1().border_color(cx.theme().border));
        if self.loading_available {
            return col.child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .px_3()
                    .py_1()
                    .child(Spinner::new().xsmall())
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("Loading available versions\u{2026}"),
                    ),
            );
        }
        if let Some(error) = &self.available_error {
            return col.child(
                div()
                    .px_3()
                    .py_1()
                    .text_xs()
                    .text_color(cx.theme().danger)
                    .child(error.clone()),
            );
        }

        let installed: Vec<&str> = self
            .managed_versions
            .iter()
            .map(|v| v.version.as_str())
            .collect();
        // Releases first, newest first; pre-releases (alphas, betas,
        // release candidates) after them and marked, since they are for
        // trying out, not for a project to depend on.
        let (releases, prereleases): (Vec<&Download>, Vec<&Download>) = self
            .available_versions
            .iter()
            .filter(|d| !installed.contains(&d.version.as_str()))
            .partition(|d| !python_versions::is_prerelease(&d.version));
        let mut any = false;
        for download in releases.into_iter().chain(prereleases) {
            any = true;
            let label = if python_versions::is_prerelease(&download.version) {
                format!("{} (pre-release)", download.version)
            } else {
                download.version.clone()
            };
            let for_install = download.clone();
            col = col.child(
                div().px_3().py_0p5().child(
                    Button::new(format!("python-version-install-{}", download.version))
                        .ghost()
                        .xsmall()
                        .icon(IconName::Plus)
                        .label(label)
                        .on_click(cx.listener(move |this, _e, _w, cx| {
                            this.request_install(for_install.clone(), cx);
                        })),
                ),
            );
        }
        if !any {
            col = col.child(
                div()
                    .px_3()
                    .py_1()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("Every available version is installed"),
            );
        }
        col
    }

    /// The question before an install or removal, then its progress and
    /// result. Drawn over the whole window so it can't be missed.
    fn render_version_modal(&self, window: &Window, cx: &Context<Self>) -> Option<AnyElement> {
        let step = self.version_step.clone()?;
        let viewport = window.viewport_size();

        let title = match &step {
            VersionStep::Confirm(PendingVersionAction::Install(_)) => {
                "Download Python?".to_string()
            }
            VersionStep::Confirm(PendingVersionAction::Uninstall(_)) => {
                "Remove Python version?".to_string()
            }
            VersionStep::Running(action) => format!(
                "{} Python {}\u{2026}",
                match action {
                    PendingVersionAction::Install(_) => "Installing",
                    PendingVersionAction::Uninstall(_) => "Removing",
                },
                action.version()
            ),
            VersionStep::Done(action, _) => format!("Python {} {}", action.version(), match action {
                PendingVersionAction::Install(_) => "installed",
                PendingVersionAction::Uninstall(_) => "removed",
            }),
            VersionStep::Error(action, _) => {
                format!("Couldn't {} Python {}", action.verb(), action.version())
            }
        };

        let muted = |text: String| {
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(text)
        };

        let body: AnyElement = match &step {
            VersionStep::Confirm(PendingVersionAction::Install(download)) => {
                let size = match download.size {
                    Some(bytes) => megabytes(bytes),
                    None => "size not known yet".to_string(),
                };
                v_flex()
                    .gap_1()
                    .child(div().text_sm().text_color(cx.theme().foreground).child(format!(
                        "Python {} will be downloaded ({size}) and unpacked into the IDE's own \
                         folder. Nothing is installed on this machine.",
                        download.version
                    )))
                    .when(python_versions::is_prerelease(&download.version), |el| {
                        el.child(div().text_xs().text_color(cx.theme().warning).child(
                            "This is a pre-release of Python, published for testing. \
                             Packages may not support it yet.",
                        ))
                    })
                    .child(muted(format!("From: {}", python_versions::SOURCE)))
                    .child(muted(format!("File: {}", download.file_name)))
                    .child(muted(format!(
                        "Into: {}",
                        python_versions::store_dir().to_string_lossy()
                    )))
                    .into_any_element()
            }
            VersionStep::Confirm(PendingVersionAction::Uninstall(version)) => v_flex()
                .gap_1()
                .child(div().text_sm().text_color(cx.theme().foreground).child(format!(
                    "Python {version} will be deleted from the IDE's folder."
                )))
                .child(div().text_xs().text_color(cx.theme().warning).child(
                    "Any virtual environment created from this version stops working, \
                     and has to be created again.",
                ))
                .into_any_element(),
            VersionStep::Running(action) => h_flex()
                .gap_3()
                .items_center()
                .py_2()
                .child(Spinner::new())
                .child(muted(match action {
                    PendingVersionAction::Install(_) => {
                        "Downloading, checking and unpacking\u{2026}".to_string()
                    }
                    PendingVersionAction::Uninstall(_) => "Deleting\u{2026}".to_string(),
                }))
                .into_any_element(),
            VersionStep::Done(_, output) => div()
                .text_xs()
                .text_color(cx.theme().success)
                .font_family("Cascadia Mono")
                .child(output.clone())
                .into_any_element(),
            VersionStep::Error(_, message) => div()
                .text_xs()
                .text_color(cx.theme().danger)
                .font_family("Cascadia Mono")
                .child(message.clone())
                .into_any_element(),
        };

        let footer: AnyElement = match &step {
            VersionStep::Confirm(action) => {
                let (label, variant) = match action {
                    PendingVersionAction::Install(_) => ("Download", ButtonVariant::Success),
                    PendingVersionAction::Uninstall(_) => ("Remove", ButtonVariant::Danger),
                };
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new("python-version-cancel")
                            .outline()
                            .xsmall()
                            .label("Cancel")
                            .on_click(cx.listener(|this, _e, _w, cx| this.cancel_version_step(cx))),
                    )
                    .child(
                        Button::new("python-version-confirm")
                            .with_variant(variant)
                            .xsmall()
                            .label(label)
                            .on_click(
                                cx.listener(|this, _e, _w, cx| this.confirm_version_action(cx)),
                            ),
                    )
                    .into_any_element()
            }
            VersionStep::Running(_) => div().into_any_element(),
            VersionStep::Done(..) | VersionStep::Error(..) => h_flex()
                .justify_end()
                .child(
                    Button::new("python-version-ok")
                        .outline()
                        .xsmall()
                        .label("OK")
                        .on_click(cx.listener(|this, _e, _w, cx| this.finish_version_step(cx))),
                )
                .into_any_element(),
        };

        Some(
            deferred(
                anchored()
                    .position_mode(AnchoredPositionMode::Window)
                    .position(point(px(0.0), px(0.0)))
                    .child(
                        div()
                            .id("python-version-modal-backdrop")
                            .w(viewport.width)
                            .h(viewport.height)
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(gpui::rgba(0x000000aa))
                            .child(
                                div()
                                    .id("python-version-modal")
                                    .occlude()
                                    .w(px(420.0))
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .p_4()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .bg(cx.theme().background)
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(cx.theme().foreground)
                                            .child(title),
                                    )
                                    .child(body)
                                    .child(footer),
                            ),
                    ),
            )
            .with_priority(100)
            .into_any_element(),
        )
    }

    fn render_projects(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();

        if self.scanning && self.projects.is_empty() {
            col = col.child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .px_3()
                    .py_2()
                    .child(Spinner::new().xsmall())
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("Scanning\u{2026}"),
                    ),
            );
        } else if self.projects.is_empty() {
            col = col.child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("No Python sub-projects found — using the scan root"),
            );
        } else {
            for proj in &self.projects {
                let selected = self.selected_project.as_deref() == Some(&proj.path);
                let color = if selected {
                    cx.theme().primary
                } else {
                    cx.theme().foreground
                };
                let proj_path = proj.path.clone();
                let marker_label = proj.marker.file_name();
                col = col.child(
                    div()
                        .id(format!("py-proj-{}", proj.path))
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .py_0p5()
                        .text_xs()
                        .cursor_pointer()
                        .when(selected, |d| d.bg(cx.theme().list_active))
                        .hover(|d| d.bg(cx.theme().list_hover))
                        .on_click(cx.listener(move |this, _e, _w, cx| {
                            this.select_project(proj_path.clone(), cx);
                        }))
                        .child(
                            div()
                                .flex_1()
                                .truncate()
                                .font_family("Cascadia Mono")
                                .text_color(color)
                                .child(proj.name.clone()),
                        )
                        .child(Tag::secondary().xsmall().child(marker_label)),
                );
            }
        }

        col
    }

    fn render_environment(&self) -> impl IntoElement {
        let mut content = v_flex().w_full().gap_1();

        if !self.missing_deps.is_empty() {
            content = content.child(div().w_full().flex().items_center().gap_1().child(
                Tag::danger().xsmall().child(format!(
                    "{} missing from requirements.txt",
                    self.missing_deps.len()
                )),
            ));
        }
        if self.requirements_count > 0 {
            content = content.child(div().w_full().child(
                Label::new(format!("{} in requirements.txt", self.requirements_count)).text_xs(),
            ));
        }

        if let Some(venv) = &self.venv_path {
            content = content.child(
                div()
                    .w_full()
                    .child(Label::new("venv").text_xs().secondary(venv.as_str())),
            );
        } else if self.interpreter_source == InterpreterSource::BuiltIn {
            content = content.child(div().w_full().child(
                Label::new("Using the built-in Python (no project venv found)").text_xs(),
            ));
        } else if self.python_exe.is_some() {
            content = content.child(div().w_full().child(
                Label::new("Using the global interpreter (no project venv found)").text_xs(),
            ));
        }

        if self.missing_deps.is_empty() && self.requirements_count == 0 && self.venv_path.is_none()
        {
            content = content.child(
                div()
                    .w_full()
                    .child(Label::new("No environment details for this project").text_xs()),
            );
        }

        content.w_full().px_3().py_2()
    }

    fn render_framework(&self) -> impl IntoElement {
        if let Some(fw) = &self.framework {
            h_flex()
                .gap_2()
                .items_center()
                .px_3()
                .py_2()
                .child(div().text_base().child(fw.icon.clone()))
                .child(Tag::primary().child(format!("{} detected", fw.name)))
                .into_any_element()
        } else {
            div().into_any_element()
        }
    }

    fn render_quick_actions(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut row = div().flex().flex_row().flex_wrap().gap_1().px_3().py_1();

        // Framework quick action button: runs the framework's dev command
        // in the active project's directory via the Script Runner panel.
        if let Some(fw) = &self.framework {
            let label = fw.name.to_lowercase();
            let cmd = fw.dev_command.clone();
            let cwd = self.active_project_dir();
            row = row.child(
                Button::new(format!("py-quick-{label}"))
                    .secondary()
                    .xsmall()
                    .label(label)
                    .tooltip(format!("Run \"{cmd}\""))
                    .on_click(cx.listener(move |this, _e, window, cx| {
                        this.dispatch_script(&cmd, cwd.clone(), window, cx);
                    })),
            );
        }

        // Entry point buttons: run `python <file>` from that entry point's
        // own directory, matching the source's `dispatch(&cmd, dir)`.
        for (idx, (file, dir)) in self.entry_points.iter().enumerate() {
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file)
                .to_string();
            // Runs from the entry point's own directory, so the file is
            // normally just its name — which is what keeps a project under a
            // path with spaces runnable.
            let cmd = shell_path(file, dir).map(|file| format!("python {file}"));
            let cwd = dir.clone();
            row = row.child(
                Button::new(format!("py-quick-entry-{idx}"))
                    .secondary()
                    .xsmall()
                    .label(filename)
                    .tooltip(match &cmd {
                        Ok(cmd) => format!("Run \"{cmd}\""),
                        Err(error) => error.clone(),
                    })
                    .on_click(cx.listener(move |this, _e, window, cx| match &cmd {
                        Ok(cmd) => this.dispatch_script(cmd, cwd.clone(), window, cx),
                        Err(error) => this.report(error.clone(), cx),
                    })),
            );
        }

        // `open_python_manager` re-added: opens the Python Package Manager
        // workspace tab rooted on the active project. Mirrors
        // `node_panel`'s "Package Manager" button exactly — uses the
        // `Window` already handed to this click handler rather than
        // `WeakEntity::update_in`'s own window lookup.
        row = row.child(
            Button::new("py-quick-pymgr")
                .secondary()
                .xsmall()
                .label("Package Manager")
                .tooltip("Open Python Package Manager")
                .on_click(cx.listener(|this, _e, window, cx| {
                    if let Some(workspace) = this.workspace.upgrade() {
                        let root = this.active_project_dir();
                        workspace.update(cx, |workspace, cx| {
                            if !root.is_empty() {
                                python_manager_panel::open(root, workspace, window, cx);
                            }
                        });
                    }
                })),
        );

        // Show scanning state or no actions message
        if self.framework.is_none() && self.entry_points.is_empty() {
            if self.scanning {
                row = row.child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .px_3()
                        .py_2()
                        .child(Spinner::new().xsmall())
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Scanning\u{2026}"),
                        ),
                );
            } else {
                row = row.child(
                    div()
                        .px_3()
                        .py_2()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("No framework or entry points detected"),
                );
            }
        }

        row
    }

    fn render_packages(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();

        if self.installed_pkgs.is_empty() {
            col = col.child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("No installed packages"),
            );
        } else {
            for pkg in &self.installed_pkgs {
                col = col.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .py_0p5()
                        .text_xs()
                        .child(
                            div()
                                .flex_1()
                                .truncate()
                                .font_family("Cascadia Mono")
                                .text_color(cx.theme().foreground)
                                .child(pkg.name.clone()),
                        )
                        .child(
                            div()
                                .font_family("Cascadia Mono")
                                .text_color(cx.theme().muted_foreground)
                                .child(format!("@{}", pkg.version)),
                        ),
                );
            }
        }

        col
    }

    fn render_outdated(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();

        match &self.outdated {
            PackagesState::Loading => {
                col = col.child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .px_3()
                        .py_2()
                        .child(Spinner::new().xsmall())
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Checking for updates\u{2026}"),
                        ),
                );
            }
            PackagesState::Error(err) => {
                col = col.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .child(
                            div()
                                .flex_1()
                                .truncate()
                                .text_color(cx.theme().danger)
                                .child(err.clone()),
                        ),
                );
            }
            PackagesState::Ready(list) => {
                if list.is_empty() {
                    col = col.child(
                        div()
                            .px_3()
                            .py_2()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("All packages up to date"),
                    );
                } else {
                    col = col.child(
                        h_flex()
                            .gap_1()
                            .flex_wrap()
                            .items_center()
                            .px_3()
                            .py_1()
                            .child(
                                Button::new("py-update-all-minor")
                                    .secondary()
                                    .xsmall()
                                    .label("Update all (minor)")
                                    .on_click(cx.listener(|this, _e, window, cx| {
                                        this.update_all("minor", window, cx);
                                    })),
                            )
                            .child(
                                Button::new("py-update-all-patch")
                                    .secondary()
                                    .xsmall()
                                    .label("Update all (patch)")
                                    .on_click(cx.listener(|this, _e, window, cx| {
                                        this.update_all("patch", window, cx);
                                    })),
                            ),
                    );

                    for pkg in list {
                        // Red is reserved for the Vulnerabilities section —
                        // "major" here doesn't mean "broken", just "bigger
                        // diff to review", so it gets amber instead. Minor
                        // and patch use two distinct, quieter colors rather
                        // than both reading as "fine" in the same shade.
                        let (kind_color, kind_label) = match pkg.kind {
                            UpdateKind::Major => (cx.theme().warning, "major"),
                            UpdateKind::Minor => (cx.theme().info, "minor"),
                            UpdateKind::Patch => (cx.theme().muted_foreground, "patch"),
                        };
                        col = col.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .px_3()
                                .py_0p5()
                                .text_xs()
                                .child(
                                    div()
                                        .flex_1()
                                        .truncate()
                                        .font_family("Cascadia Mono")
                                        .text_color(cx.theme().foreground)
                                        .child(pkg.name.clone()),
                                )
                                .child(
                                    div()
                                        .flex_shrink_0()
                                        .font_family("Cascadia Mono")
                                        .text_color(cx.theme().muted_foreground)
                                        .child(format!(
                                            "{} \u{2192} {}",
                                            pkg.version, pkg.latest_version
                                        )),
                                )
                                .child(
                                    div()
                                        .rounded_md()
                                        .px_1()
                                        .py_0p5()
                                        .bg(kind_color.opacity(0.2))
                                        .text_color(kind_color)
                                        .child(kind_label),
                                ),
                        );
                    }
                }
            }
        }

        col
    }

    fn render_vulnerabilities(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();

        col = col.child(
            h_flex()
                .gap_2()
                .items_center()
                .px_3()
                .py_1()
                .child(
                    Button::new("py-scan-vulns")
                        .secondary()
                        .xsmall()
                        .label(if self.vuln_scanning {
                            "Scanning\u{2026}"
                        } else {
                            "Scan for vulnerabilities"
                        })
                        .disabled(self.vuln_scanning || self.installed_pkgs.is_empty())
                        .on_click(cx.listener(|this, _e, _w, cx| {
                            this.scan_vulnerabilities(cx);
                        })),
                )
                .when(self.vuln_scanning, |row| row.child(Spinner::new().xsmall())),
        );

        if let Some(err) = &self.vuln_scan_error {
            col = col.child(
                div()
                    .px_3()
                    .py_1()
                    .text_xs()
                    .text_color(cx.theme().danger)
                    .child(err.clone()),
            );
        } else if self.vuln_scanned {
            if self.vulns.is_empty() {
                col = col.child(
                    div()
                        .px_3()
                        .py_2()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("No known vulnerabilities"),
                );
            } else {
                // No severity tiers here — PyPI's per-version advisory data
                // (unlike `dotnet list package --vulnerable`) doesn't carry
                // a severity field, only an OSV advisory id/summary/fixed-in
                // list. A known vulnerability is exactly what red is for.
                let mut names: Vec<&String> = self.vulns.keys().collect();
                names.sort();
                for name in names {
                    let Some(findings) = self.vulns.get(name) else {
                        continue;
                    };
                    for vuln in findings {
                        col = col.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .px_3()
                                .py_0p5()
                                .text_xs()
                                .child(
                                    div()
                                        .flex_1()
                                        .truncate()
                                        .font_family("Cascadia Mono")
                                        .text_color(cx.theme().foreground)
                                        .child(format!("{name} — {}", vuln.id)),
                                )
                                .child(
                                    div()
                                        .rounded_md()
                                        .px_1()
                                        .py_0p5()
                                        .bg(cx.theme().danger.opacity(0.2))
                                        .text_color(cx.theme().danger)
                                        .child("vulnerable"),
                                ),
                        );
                        if let Some(summary) = &vuln.summary {
                            col = col.child(
                                div()
                                    .px_3()
                                    .pb_1()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(summary.clone()),
                            );
                        }
                    }
                }
            }
        } else {
            col = col.child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("Not scanned yet"),
            );
        }

        col
    }

    fn render_processes(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();

        if !self.procs_loaded {
            col = col.child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(Spinner::new().xsmall())
                    .child("Loading processes\u{2026}"),
            );
        } else if self.python_procs.is_empty() {
            col = col.child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("No Python processes running"),
            );
        } else {
            col = col.child(
                div()
                    .flex()
                    .px_3()
                    .py_0p5()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(cx.theme().muted_foreground)
                    .child(div().w_16().child("PID"))
                    .child(div().flex_1().child("Name"))
                    .child(div().w_16().text_right().child("CPU"))
                    .child(div().w_16().text_right().child("Mem")),
            );

            for proc in &self.python_procs {
                col = col.child(
                    div()
                        .flex()
                        .px_3()
                        .py_0p5()
                        .text_xs()
                        .text_color(cx.theme().foreground)
                        .child(
                            div()
                                .w_16()
                                .font_family("Cascadia Mono")
                                .child(proc.pid.to_string()),
                        )
                        .child(
                            div()
                                .flex_1()
                                .truncate()
                                .font_family("Cascadia Mono")
                                .child(proc.name.clone()),
                        )
                        .child(
                            div()
                                .w_16()
                                .text_right()
                                .font_family("Cascadia Mono")
                                .child(format!("{:.1}%", proc.cpu_percent)),
                        )
                        .child(
                            div()
                                .w_16()
                                .text_right()
                                .font_family("Cascadia Mono")
                                .child(format!("{:.0}MB", proc.memory_mb)),
                        ),
                );
            }
        }

        col
    }
}

// ── Section Container Helper ──────────────────────────────────────────

fn section_container(
    cx: &Context<PythonPanel>,
    title: &str,
    count: usize,
    open: bool,
    body: impl IntoElement,
) -> impl IntoElement {
    let key = title.to_string();

    let header = div()
        .id(format!("py-section-{key}"))
        .flex()
        .items_center()
        .gap_1()
        .px_3()
        .py_1()
        .text_xs()
        .cursor_pointer()
        .hover(|d| d.bg(cx.theme().list_hover))
        .child(
            Icon::new(if open {
                IconName::ChevronDown
            } else {
                IconName::ChevronRight
            })
            .xsmall()
            .text_color(cx.theme().muted_foreground),
        )
        .child(
            div()
                .flex_1()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(cx.theme().foreground)
                .child(title.to_string()),
        )
        .when(count > 0, |el| {
            el.child(Tag::secondary().xsmall().child(count.to_string()))
        })
        .on_click(cx.listener(move |this, _e, _w, _cx| {
            this.toggle(&key);
        }));

    Collapsible::new()
        .open(open)
        .child(header)
        .content(body)
        .flex()
        .flex_col()
        .w_full()
        .border_b_1()
        .border_color(cx.theme().border)
}

/// The command for the Outdated section's bulk buttons: one
/// `pip install --upgrade "<name>==<latest>"` per qualifying package,
/// chained with `&&` — patch-only updates when `ty` is `"patch"`, otherwise
/// patch and minor. `Ok(None)` when nothing qualifies; an error (and nothing
/// run) if any qualifying entry isn't shell-plain. `exe` must already be
/// shell-ready (`shell_program`).
fn update_all_command(
    exe: &str,
    outdated: &[PythonOutdatedPkg],
    ty: &str,
) -> Result<Option<String>, String> {
    let mut segments = Vec::new();
    for package in outdated.iter().filter(|o| match ty {
        "patch" => o.kind == UpdateKind::Patch,
        _ => matches!(o.kind, UpdateKind::Patch | UpdateKind::Minor),
    }) {
        check_package_name(&package.name)?;
        check_version(&package.latest_version)?;
        segments.push(format!(
            "{exe} -m pip install --upgrade \"{}=={}\"",
            package.name, package.latest_version
        ));
    }
    Ok((!segments.is_empty()).then(|| segments.join(" && ")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outdated(name: &str, version: &str, latest: &str) -> PythonOutdatedPkg {
        PythonOutdatedPkg {
            name: name.to_string(),
            version: version.to_string(),
            latest_version: latest.to_string(),
            kind: python_backend::classify_update(version, latest),
        }
    }

    #[test]
    fn path_file_name_is_the_last_component() {
        assert_eq!(path_file_name("C:/work/services/api"), "api");
        assert_eq!(path_file_name("single"), "single");
        assert_eq!(path_file_name("/"), "/");
    }

    #[test]
    fn update_all_pins_each_package_to_its_latest() {
        let list = [
            outdated("idna", "3.6", "3.6.1"),
            outdated("urllib3", "2.1.0", "2.2.0"),
            outdated("django", "4.2.0", "5.0.1"),
        ];
        assert_eq!(
            update_all_command("python", &list, "minor").unwrap().as_deref(),
            Some(
                "python -m pip install --upgrade \"idna==3.6.1\" && \
                 python -m pip install --upgrade \"urllib3==2.2.0\""
            )
        );
        assert_eq!(
            update_all_command("py", &list, "patch").unwrap().as_deref(),
            Some("py -m pip install --upgrade \"idna==3.6.1\"")
        );
    }

    #[test]
    fn update_all_with_nothing_safe_to_update() {
        assert_eq!(
            update_all_command("python", &[outdated("django", "4.2.0", "5.0.1")], "minor"),
            Ok(None)
        );
        assert_eq!(update_all_command("python", &[], "patch"), Ok(None));
    }

    #[test]
    fn update_all_runs_nothing_if_any_entry_is_unsafe() {
        let list = [outdated("idna", "3.6", "3.6.1"), outdated("x\" && calc", "1.0.0", "1.0.1")];
        assert!(update_all_command("python", &list, "minor").is_err());
        let list = [outdated("idna", "3.6", "3.6.1 && calc")];
        assert!(update_all_command("python", &list, "minor").is_err());
    }

    #[test]
    fn python_manifest_files_are_recognized() {
        assert!(is_python_project_marker("requirements.txt"));
        assert!(is_python_project_marker("pyproject.toml"));
        assert!(is_python_project_marker("Pipfile"));
        assert!(!is_python_project_marker("Pipfile.lock"));
        assert!(is_python_manifest_file("Pipfile.lock"));
        assert!(is_python_manifest_file("poetry.lock"));
        assert!(is_python_manifest_file("setup.py"));
        assert!(is_python_manifest_file("setup.cfg"));
        assert!(!is_python_manifest_file("main.py"));
        assert!(!is_python_manifest_file("requirements.txt.bak"));
    }

    #[test]
    fn manifest_changes_match_only_the_selected_project() {
        let cwd = "C:/work";
        assert!(manifest_change_is_in_project(cwd, "C:/work", "requirements.txt"));
        assert!(manifest_change_is_in_project(cwd, "C:\\work", "pyproject.toml"));
        assert!(manifest_change_is_in_project(
            cwd,
            "C:/work/services/api",
            "services/api/Pipfile"
        ));
        assert!(!manifest_change_is_in_project(
            cwd,
            "C:/work/services/api",
            "services/worker/Pipfile"
        ));
        assert!(!manifest_change_is_in_project(cwd, "C:/work", "services/api/pyproject.toml"));
    }
}
