//! Node.js panel — runtime detection, Node.js version management, project scanning,
//! script editing, and a live process list.
//!
//! Ported standalone (no host-app wiring yet): the source panel depended on
//! a host `AppState` for a few things, none of which exist here:
//!
//! - The initial/live-updating workspace root (`state.workspace_root`,
//!   `state.workspace_tx`) — this panel just uses its own process cwd
//!   instead, same as the other standalone-first ports.
//! - A live Node-process list, fed by subscribing to the Cockpit panel's
//!   own system-metrics tick broadcast and filtering it to the "Node"
//!   section (`state.tick_tx`). Reimplemented here as this panel's own
//!   lightweight `sysinfo` poll filtered by process name instead of
//!   cross-panel plumbing — see `spawn_processes_poll`.
//! - `run_in_script_runner` — the script run / quick-action / SPFx buttons'
//!   "run this command" hook. `zed::zed::initialize_panels` now hands this
//!   panel a `WeakEntity<ScriptRunnerPanel>` once both panels have loaded
//!   (see `set_script_runner`), so `dispatch_script` sends runs there for
//!   real — script rows, quick actions (`dev`/`build`/`test`/`install`),
//!   and the SPFx actions all go through it.
//! - `open_npm_manager` / `open_task_chain_wizard` — both were dropped in
//!   the first port and have since been re-added now that their targets
//!   exist: "Package Manager" opens the `npm_manager_panel` tab, and "Task
//!   Chain" opens `flows_panel`'s wizard anchored to the active project
//!   (through the `WeakEntity<FlowsPanel>` `set_flows_panel` receives).
//!   `npm install`'s own dependency-counting progress UI (which ran through
//!   the script runner) stays dropped; the "install" quick action just runs
//!   plain `npm install` and streams its output like any other quick
//!   action.
//!
//! The detection/scanning logic lives in `node_backend` and version management in `node_versions`, ported
//! alongside this panel — see that crate's own doc comment.

mod node_panel_settings;
mod node_processes;

use node_processes::NodeProcess;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use gpui::{
    Action, AnchoredPositionMode, AnyElement, App, AppContext as _, AsyncApp, AsyncWindowContext,
    Context, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, InteractiveElement as _,
    IntoElement, ParentElement as _, Pixels, Render, StatefulInteractiveElement as _, Styled as _,
    Subscription, WeakEntity, Window, actions, anchored, deferred, div, point,
    prelude::FluentBuilder as _, px, relative, svg,
};
use gpui_component::{
    ActiveTheme as _, Disableable as _, Icon, IconName, Selectable as _, Sizable as _,
    button::{Button, ButtonCustomVariant, ButtonVariant, ButtonVariants as _},
    collapsible::Collapsible,
    h_flex,
    input::{Input, InputState},
    menu::ContextMenuExt,
    panel_header::PanelHeader,
    spinner::Spinner,
    tag::Tag,
    tooltip::Tooltip,
};
use indexmap::IndexMap;
use node_backend::{
    DetectedNodeProject, query_runtime, read_text_file, reveal_in_explorer, scan_node_projects,
    write_file,
};
use node_versions::{NodeVersion, ProjectNode};
use npm_backend::{
    NpmAuditVuln, NpmInstalledPkg, NpmOutdatedPkg, UpdateKind, classify_update,
    detect_package_manager, list_audit_vulns, list_installed, list_outdated,
};
use flows_panel::FlowsPanel;
use script_runner_panel::ScriptRunnerPanel;
use script_runner_panel::command::{check_package_name, check_script_name};
use serde::Deserialize;
use sysinfo::System;
pub use node_panel_settings::NodePanelSettings;
use settings::{DockSide, Settings as _};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

const PAGE_SIZE: usize = 6;

/// How often the local Node-process poll re-scans the system.
const NODE_PROCS_TICK_INTERVAL: Duration = Duration::from_secs(2);

// ── Types ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum VersionAction {
    Install,
    Uninstall,
}

impl VersionAction {
    fn title_verb(self) -> &'static str {
        match self {
            VersionAction::Install => "Install",
            VersionAction::Uninstall => "Uninstall",
        }
    }

    fn progressive(self) -> &'static str {
        match self {
            VersionAction::Install => "Installing",
            VersionAction::Uninstall => "Uninstalling",
        }
    }

    fn past(self) -> &'static str {
        match self {
            VersionAction::Install => "installed",
            VersionAction::Uninstall => "uninstalled",
        }
    }
}

#[derive(Clone)]
enum VersionStep {
    Confirm(VersionAction, String),
    Running(VersionAction, String),
    Done(VersionAction, String, String),
    Error(VersionAction, String, String),
}

/// A background-loaded package list with its three UI states — mirrors
/// `dotnet_panel::PackagesState`/`python_panel::PackagesState`.
enum PackagesState<T> {
    Loading,
    Ready(Vec<T>),
    Error(String),
}

// ── Panel ─────────────────────────────────────────────────────────────

actions!(
    node_panel,
    [
        /// Toggles focus on the Node panel.
        ToggleFocus
    ]
);

/// Opens the selected project's `package.json` in a read-only modal.
/// Dispatched by the script rows' right-click `ContextMenu`; the panel
/// root registers an `.on_action` handler for it.
#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = node_panel, no_json)]
struct ViewPackageJson;

/// Registers the Node panel's actions on every workspace. Call once at
/// app startup, alongside the other panels' `init` functions.
pub fn init(cx: &mut App) {
    // Makes the built-in Node.js reach past this panel: terminals, the
    // Script Runner and workflow actions all ask `ide_env` what to put first
    // on a new process's `PATH`.
    //
    // What goes on the `PATH` is the store's link to the active version, the
    // same folder whatever is active, so a terminal that is already open
    // follows a switch made here.
    ide_env::register(|_directory| {
        vec![node_versions::live_bin_dir(&node_versions::store_dir())]
    });

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<NodePanel>(window, cx);
        });
    })
    .detach();
}

/// Resolves the folder the Node scan should be rooted on: the **currently
/// opened** project's first worktree, mirroring the Forge original's live
/// `workspace_root`. Returns `None` when no worktree is open yet (so the
/// caller can fall back to the process cwd).
fn project_root(workspace: &Workspace, cx: &mut Context<Workspace>) -> Option<String> {
    workspace
        .project()
        .read(cx)
        .worktrees(cx)
        .next()
        .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().into_owned())
}

pub struct NodePanel {
    focus_handle: FocusHandle,

    workspace: WeakEntity<Workspace>,

    /// Where script runs (Scripts list, quick actions) get dispatched.
    /// Handed in by `zed::zed::initialize_panels` once both this panel and
    /// the Script Runner panel are loaded — see `set_script_runner`.
    script_runner: Option<WeakEntity<ScriptRunnerPanel>>,

    /// Where the "task chain" quick action opens its wizard. Handed in by
    /// `zed::zed::initialize_panels` once both this panel and the Flows
    /// panel are loaded — see `set_flows_panel`.
    flows_panel: Option<WeakEntity<FlowsPanel>>,

    /// The Node.js and npm that scripts run from this panel use: the
    /// built-in manager's choice for the selected project, or the system
    /// one when it has none.
    node_ver: Option<String>,
    npm_ver: Option<String>,
    /// Whether `node_ver` is one of the built-in manager's versions rather
    /// than the system Node.js. The header says which, since the two can be
    /// a patch release apart and look the same at a glance.
    node_is_built_in: bool,
    not_found: bool,
    /// The Node.js already on this machine's `PATH`, whoever put it there.
    /// Shown as the "System" entry; `None` when there isn't one.
    system_node_ver: Option<String>,
    /// What the selected project asks for and whether that is installed.
    project_node: ProjectNode,
    /// Set while the pending install was asked for by the project line: the
    /// folder whose version request to install from.
    install_for_project: Option<PathBuf>,

    versions_loaded: Option<bool>,
    managed_versions: Vec<NodeVersion>,
    show_install: bool,
    available_versions: Vec<String>,
    available_page: usize,
    loading_available: bool,
    version_step: Option<VersionStep>,

    cwd: String,
    projects: Vec<DetectedNodeProject>,
    selected_project: Option<String>,
    scanning: bool,

    /// Live installed packages for the selected project (`npm ls`), plus
    /// the outdated/vulnerable lists — parity with `dotnet_panel`'s
    /// Packages/Outdated/Vulnerabilities sections. All three are scoped to
    /// `selected_project` (falling back to `cwd`), reloaded by
    /// `reload_package_data` on every project switch.
    installed_pkgs: Vec<NpmInstalledPkg>,
    outdated: PackagesState<NpmOutdatedPkg>,
    vulnerable: PackagesState<NpmAuditVuln>,
    /// Label of the bulk update action currently streaming in the Script
    /// Runner, if any — guards against piling a second run onto the first.
    running_action: Option<String>,
    /// Active observer on the Script Runner dock panel, replaced on every
    /// bulk-update run — fires when it notifies (every output line and on
    /// completion), so `reload_package_data` re-runs exactly once the
    /// update finishes. Mirrors `dotnet_panel`'s `run_subscription`.
    run_subscription: Option<Subscription>,

    scripts: Option<IndexMap<String, String>>,
    editing_script: Option<String>,
    // Created fresh when editing starts, dropped on cancel/save.
    edit_name_input: Option<Entity<InputState>>,
    edit_cmd_input: Option<Entity<InputState>>,
    adding_script: bool,
    // Created when add-script form opens, dropped on cancel/save.
    new_script_name_input: Option<Entity<InputState>>,
    new_script_cmd_input: Option<Entity<InputState>>,

    /// Feedback log for version operations (install/switch version). Script runs
    /// go to the Script Runner panel, not here.
    script_output: Vec<String>,

    /// Node processes on the machine, heaviest first.
    node_procs: Vec<NodeProcess>,
    /// False until the first background process scan lands, so the
    /// Processes section shows a loading row instead of claiming nothing
    /// is running.
    procs_loaded: bool,
    /// Whether the processes outside this workspace are listed or folded.
    show_other_procs: bool,
    /// The process whose Stop is waiting for confirmation, if any.
    confirm_stop: Option<u32>,

    spfx: SpfxInfo,
    sppkg_files: Vec<String>,

    open: HashMap<String, bool>,

    view_pkg: Option<PackageJsonView>,

    /// Debounces the filesystem-watcher rescan: replaced on every relevant
    /// `WorktreeUpdatedEntries` event, so a burst of updates collapses into a
    /// single `scan_projects` run once it goes quiet. Held only for its `Drop`
    /// (replacing it cancels the pending run).
    _rescan_task: Option<gpui::Task<()>>,

    /// Same debounce, but for reloading the selected project's scripts and
    /// package data when its `package.json`/lockfile changes on disk.
    _data_reload_task: Option<gpui::Task<()>>,

    _procs_poll: gpui::Task<()>,
}

struct PackageJsonView {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    scripts: IndexMap<String, String>,
    dependencies: IndexMap<String, String>,
    dev_dependencies: IndexMap<String, String>,
}

fn parse_package_json_view(raw: &str) -> Option<PackageJsonView> {
    let json: serde_json::Value = serde_json::from_str(raw).ok()?;

    let str_field = |key: &str| json.get(key).and_then(|v| v.as_str()).map(String::from);
    let map_field = |key: &str| -> IndexMap<String, String> {
        json.get(key)
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                    .collect()
            })
            .unwrap_or_default()
    };

    Some(PackageJsonView {
        name: str_field("name"),
        version: str_field("version"),
        description: str_field("description"),
        scripts: map_field("scripts"),
        dependencies: map_field("dependencies"),
        dev_dependencies: map_field("devDependencies"),
    })
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

/// Files the panel derives package/script data from. A change to any of these
/// means the selected project's data is stale. `package.json` is included even
/// though the project scan also watches for it, since scripts live there too.
fn is_node_manifest_file(name: &str) -> bool {
    matches!(
        name,
        "package.json"
            | "package-lock.json"
            | "npm-shrinkwrap.json"
            | "yarn.lock"
            | "pnpm-lock.yaml"
    )
}

/// Whether a changed worktree-relative path (`/`-separated, relative to the
/// scan root `cwd`) sits directly in `selected` — i.e. it is that project's
/// own `package.json`/lockfile, not some other project's under the same
/// workspace. Separators in `cwd`/`selected` may be either style.
fn manifest_change_is_in_project(cwd: &str, selected: &str, rel_unix: &str) -> bool {
    let root = cwd.replace('\\', "/");
    let changed = format!("{}/{}", root.trim_end_matches('/'), rel_unix);
    changed
        .rsplit_once('/')
        .is_some_and(|(dir, _)| dir == selected.replace('\\', "/").trim_end_matches('/'))
}

impl NodePanel {
    /// Detected Node version, for a future dashboard-style "runtime
    /// versions" summary (the source panel's Cockpit dashboard exposed this
    /// the same way, via a plain accessor).
    pub fn node_version(&self) -> Option<&str> {
        self.node_ver.as_deref()
    }

    /// Every detected Node sub-project, for a future dashboard-style
    /// "runtime versions" summary.
    pub fn detected_projects(&self) -> &[DetectedNodeProject] {
        &self.projects
    }

    /// Outdated-package count for the Cockpit dashboard's Runtimes section.
    /// `0` while loading or on error — the dashboard just shows a count, not
    /// this panel's loading/error state.
    pub fn outdated_count(&self) -> usize {
        match &self.outdated {
            PackagesState::Ready(v) => v.len(),
            _ => 0,
        }
    }

    /// Vulnerable-package count, for the same dashboard use as
    /// `outdated_count`.
    pub fn vulnerable_count(&self) -> usize {
        match &self.vulnerable {
            PackagesState::Ready(v) => v.len(),
            _ => 0,
        }
    }

    /// The active scan target's display label — `selected_project`'s folder
    /// name when one was detected, else the scan root's. Falls back to the
    /// full path when it has no file-name component.
    pub fn active_project_label(&self) -> String {
        let dir = self.selected_project.as_ref().unwrap_or(&self.cwd);
        path_file_name(dir)
    }

    /// The selected project's audit findings, when the scan finished (see
    /// `vulnerable_loading`/`vulnerable_error`). `None` while loading, on
    /// error, or when there's nothing to scan — the Cockpit dashboard's
    /// Security section reads this to render per-finding rows.
    pub fn vulnerable_findings(&self) -> Option<&[NpmAuditVuln]> {
        match &self.vulnerable {
            PackagesState::Ready(list) => Some(list),
            _ => None,
        }
    }

    /// Whether the audit scan is currently in flight.
    pub fn vulnerable_loading(&self) -> bool {
        matches!(&self.vulnerable, PackagesState::Loading)
    }

    /// The audit scan's error, if the last scan failed.
    pub fn vulnerable_error(&self) -> Option<&str> {
        match &self.vulnerable {
            PackagesState::Error(e) => Some(e),
            _ => None,
        }
    }

    /// Re-runs the audit scan for the selected project, driven by the Cockpit
    /// dashboard's Security section (`reload_vulnerable` does the work).
    pub fn rescan_vulnerabilities(&mut self, cx: &mut Context<Self>) {
        self.reload_vulnerable(cx);
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
            NodePanel::new(workspace, window, cx)
        })
    }

    pub fn new(
        _workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        // Root the scan on the currently-opened project worktrees (matching the
        // Forge original's live `workspace_root`), falling back to the process
        // cwd when no worktree is open yet.
        let cwd = project_root(&*_workspace, cx).unwrap_or_else(|| {
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_default()
        });
        // The live worktrees watch the filesystem (same machinery the project
        // panel relies on); subscribing here lets `scan_projects` rerun when a
        // new `package.json` appears, instead of only once at startup.
        let project = _workspace.project().clone();

        let workspace = cx.entity().downgrade();

        cx.new(|cx| {
            cx.subscribe(&project, |this: &mut NodePanel, _project, event, cx| {
                if let project::Event::WorktreeUpdatedEntries(_, entries) = event {
                    let mut package_json_touched = false;
                    let mut selected_manifest_touched = false;
                    for (path, _, _) in entries.iter() {
                        let Some(name) = path.file_name() else {
                            continue;
                        };
                        if name == "package.json" {
                            package_json_touched = true;
                        }
                        if is_node_manifest_file(name)
                            && this.manifest_change_is_selected(path.as_unix_str())
                        {
                            selected_manifest_touched = true;
                        }
                    }
                    // A new/removed `package.json` can mean a new project, so
                    // the Projects list is always re-scanned for one.
                    if package_json_touched {
                        this.schedule_project_rescan(cx);
                    }
                    // Dependencies/scripts only change the *selected* project's
                    // data, so only reload that one (avoids re-running the
                    // network-bound audit for unrelated edits elsewhere).
                    if selected_manifest_touched {
                        this.schedule_data_reload(cx);
                    }
                }
            })
            .detach();

            let mut open = HashMap::new();
            // Keep all sections open by default
            open.insert("Node Versions".to_string(), true);
            open.insert("Projects".to_string(), true);
            open.insert("Quick Actions".to_string(), true);
            open.insert("Packages".to_string(), true);
            open.insert("Outdated".to_string(), true);
            open.insert("Vulnerabilities".to_string(), true);
            open.insert("Scripts".to_string(), true);
            open.insert("Processes".to_string(), true);

            let panel = NodePanel {
                focus_handle: cx.focus_handle(),
                workspace: workspace.clone(),
                script_runner: None,
                flows_panel: None,
                node_ver: None,
                npm_ver: None,
                not_found: false,
                node_is_built_in: false,
                system_node_ver: None,
                project_node: ProjectNode::default(),
                install_for_project: None,
                versions_loaded: None,
                managed_versions: Vec::new(),
                show_install: false,
                available_versions: Vec::new(),
                available_page: 0,
                loading_available: false,
                version_step: None,
                cwd: cwd.clone(),
                projects: Vec::new(),
                selected_project: None,
                scanning: false,
                installed_pkgs: Vec::new(),
                outdated: PackagesState::Ready(Vec::new()),
                vulnerable: PackagesState::Ready(Vec::new()),
                running_action: None,
                run_subscription: None,
                scripts: None,
                editing_script: None,
                edit_name_input: None,
                edit_cmd_input: None,
                adding_script: false,
                new_script_name_input: None,
                new_script_cmd_input: None,
                script_output: Vec::new(),
                node_procs: Vec::new(),
                procs_loaded: false,
                show_other_procs: false,
                confirm_stop: None,
                spfx: SpfxInfo::default(),
                sppkg_files: Vec::new(),
                open,
                view_pkg: None,
                _rescan_task: None,
                _data_reload_task: None,
                _procs_poll: Self::spawn_processes_poll(cx),
            };

            init_discovery(cx);

            panel
        })
    }

    /// Stops `pid` and everything it started, once the user has confirmed
    /// it. The list shows the result on the next poll tick.
    fn stop_process(&mut self, pid: u32, cx: &mut Context<Self>) {
        self.confirm_stop = None;
        cx.notify();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move { node_processes::stop_tree(pid) })
                .await;
            this.update(cx, |panel, cx| {
                if let Err(error) = result {
                    panel.script_output.push(format!("Error: {error}"));
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Refreshes `node_procs` from a locally-owned `sysinfo::System` every
    /// `NODE_PROCS_TICK_INTERVAL`, filtered to process names containing
    /// "node" (see `node_processes::scan`) — replaces subscribing to a host
    /// cockpit panel's broadcast tick (see the module doc).
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
                        let procs = node_processes::scan(&mut sys);
                        (sys, procs)
                    })
                    .await;
                sys = scanned;

                let alive = this
                    .update(cx, |this, cx| {
                        // A confirmation for a process that has since exited
                        // has nothing left to confirm.
                        if let Some(pid) = this.confirm_stop
                            && !procs.iter().any(|proc| proc.pid == pid)
                        {
                            this.confirm_stop = None;
                        }
                        this.node_procs = procs;
                        this.procs_loaded = true;
                        cx.notify();
                    })
                    .is_ok();
                if !alive {
                    break;
                }

                cx.background_executor()
                    .timer(NODE_PROCS_TICK_INTERVAL)
                    .await;
            }
        })
    }

    fn is_open(&self, key: &str, default: bool) -> bool {
        *self.open.get(key).unwrap_or(&default)
    }

    fn toggle(&mut self, key: &str) {
        let e = self.open.entry(key.to_string()).or_insert(true);
        *e = !*e;
    }

    fn selected_pkg_path(&self) -> Option<String> {
        let proj = self.selected_project.as_ref()?;
        Some(format!("{}/package.json", proj))
    }

    /// Called once by `zed::zed::initialize_panels` after both this panel
    /// and the Script Runner panel have loaded, so `dispatch_script` has
    /// somewhere to send runs — see the module doc's `run_in_script_runner`
    /// note.
    pub fn set_script_runner(&mut self, script_runner: WeakEntity<ScriptRunnerPanel>) {
        self.script_runner = Some(script_runner);
    }

    /// Called once by `zed::zed::initialize_panels` after both this panel
    /// and the Flows panel have loaded, so the "task chain" quick action has
    /// somewhere to open its wizard — see `open_task_chain_wizard`.
    pub fn set_flows_panel(&mut self, flows_panel: WeakEntity<FlowsPanel>) {
        self.flows_panel = Some(flows_panel);
    }

    /// "task chain" quick action — reveals the Flows panel and opens its
    /// Add Task Chain wizard anchored on the selected project (or the scan
    /// root, if nothing's selected), matching `dispatch_script`'s "reveal
    /// the target dock panel via `cx.defer`, not directly" approach for the
    /// exact same reentrant-lease reason (see that method's doc comment).
    fn open_task_chain_wizard(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(flows_panel) = self.flows_panel.as_ref().and_then(|w| w.upgrade()) else {
            return;
        };

        let workspace = self.workspace.clone();
        let window_handle = window.window_handle();
        cx.defer(move |app| {
            let _ = app.update_window(window_handle, |_, window, cx| {
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        workspace.reveal_panel::<FlowsPanel>(window, cx);
                    });
                }
            });
        });

        let project_root = self
            .selected_project
            .clone()
            .unwrap_or_else(|| self.cwd.clone());
        let anchor = (!project_root.is_empty()).then(|| PathBuf::from(project_root));
        flows_panel.update(cx, |panel, cx| {
            panel.open_task_chain_wizard_anchored(anchor, window, cx);
        });
    }

    /// Sends `command` to the Script Runner panel, matching the Forge
    /// original's `dispatch_script` (run in the selected project, falling
    /// back to the scan root).
    fn dispatch_script(&mut self, command: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(script_runner) = self.script_runner.as_ref().and_then(|w| w.upgrade()) else {
            return;
        };
        // The runner can only stream one command at a time (same constraint
        // `npm_manager_panel`'s §4.2 wiring guards against) — without this
        // check, clicking a script while another is still running (e.g. a
        // `dev` server that never exits) silently no-ops in
        // `ScriptRunnerPanel::run`, and the panel just keeps showing the
        // earlier command's output, which looks like the wrong script ran.
        if script_runner.read(cx).is_running() {
            self.script_output.push(format!(
                "Script Runner is busy — stop the current run before running \"{command}\"."
            ));
            cx.notify();
            return;
        }
        // Surface the run: reveal the Script Runner dock panel so its
        // output is actually visible, matching `npm_manager_panel`'s §4.2
        // wiring (`kick_run`). Unlike that caller, `NodePanel` is itself a
        // docked `Panel` — `reveal_panel` -> `dismiss_zoomed_items_to_reveal`
        // walks every docked panel's `is_zoomed`, which reads this very
        // entity, panicking with a "double lease" error if it's still
        // mutably leased. `Context::defer_in` isn't enough on its own: it
        // still re-leases `self` (NodePanel) to invoke its callback, so a
        // `reveal_panel` call made from *inside* that callback hits the
        // exact same conflict one level down. Use the entity-agnostic
        // `App::defer` instead (via `cx`'s `Deref<Target = App>`) — its
        // callback only gets `&mut App`, never re-leasing NodePanel, and by
        // the time it runs the original click-handler's lease has been
        // returned to the app.
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

        let cwd = self
            .selected_project
            .clone()
            .unwrap_or_else(|| self.cwd.clone());
        let command = command.to_string();
        // The Script Runner puts the active Node.js first on the command's
        // `PATH` itself (`ide_env`), as it does for everything it runs, so
        // `node`, `npm` and anything they start resolve to it.
        cx.spawn(async move |_, cx: &mut AsyncApp| {
            let _ = cx.update(|app| {
                script_runner.update(app, |panel, cx| {
                    panel.run_external(command, cwd, cx);
                });
            });
        })
        .detach();
    }

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
                .background_spawn(async move { scan_node_projects(&root, 0) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.scanning = false;
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
                            match panel.projects.first().cloned() {
                                Some(first) => panel.select_project(first.path, cx),
                                None => {
                                    panel.selected_project = None;
                                    panel.scripts = None;
                                    panel.installed_pkgs = Vec::new();
                                    panel.outdated = PackagesState::Ready(Vec::new());
                                    panel.vulnerable = PackagesState::Ready(Vec::new());
                                    panel.spfx = SpfxInfo::default();
                                    panel.sppkg_files = Vec::new();
                                }
                            }
                        }
                        cx.notify();
                    });
                }
            });
        })
        .detach();
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
    /// in the selected project — i.e. it is that project's own `package.json`
    /// or lockfile, not some other project's under the same workspace.
    fn manifest_change_is_selected(&self, rel_unix: &str) -> bool {
        let Some(selected) = self.selected_project.as_deref() else {
            return false;
        };
        manifest_change_is_in_project(&self.cwd, selected, rel_unix)
    }

    /// Reloads the selected project's scripts plus its installed/outdated/
    /// vulnerable package data after its `package.json`/lockfile changed on
    /// disk — an install, uninstall or update run outside the panel (terminal,
    /// `npm_manager_panel`, npm's own lifecycle scripts), or a hand edit.
    /// Debounced like `schedule_project_rescan`, and skipped while a bulk
    /// update is streaming so it can't race the `run_subscription` reload.
    fn schedule_data_reload(&mut self, cx: &mut Context<Self>) {
        self._data_reload_task =
            Some(cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
                cx.background_executor()
                    .timer(Duration::from_millis(500))
                    .await;
                let _ = cx.update(|app| {
                    if let Some(panel) = this.upgrade() {
                        let _ = panel.update(app, |panel, cx| {
                            if panel.running_action.is_some() {
                                return;
                            }
                            panel.load_scripts(cx);
                            panel.reload_package_data(cx);
                        });
                    }
                });
            }));
    }

    fn load_scripts(&mut self, cx: &mut Context<Self>) {
        let Some(pkg_path) = self.selected_pkg_path() else {
            self.scripts = None;
            return;
        };
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move { read_text_file(pkg_path) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.scripts = result.ok().and_then(|raw| {
                            let json: serde_json::Value = serde_json::from_str(&raw).ok()?;
                            let obj = json.get("scripts")?.as_object()?;
                            let map: IndexMap<String, String> = obj
                                .iter()
                                .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                                .collect();
                            Some(map)
                        });
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    fn save_script_edit(&mut self, cx: &mut Context<Self>) {
        let Some(pkg_path) = self.selected_pkg_path() else {
            return;
        };
        let Some(old_name) = self.editing_script.take() else {
            return;
        };
        let Some(edit_name_input) = self.edit_name_input.take() else {
            return;
        };
        let Some(edit_cmd_input) = self.edit_cmd_input.take() else {
            return;
        };

        let new_name = edit_name_input.read(cx).value().trim().to_string();
        let new_cmd = edit_cmd_input.read(cx).value().trim().to_string();
        if new_name.is_empty() {
            return;
        }

        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let _ = cx
                .background_spawn(async move {
                    let raw = read_text_file(pkg_path.clone())?;
                    let mut json: serde_json::Value =
                        serde_json::from_str(&raw).map_err(|e| e.to_string())?;
                    if let Some(obj) = json.get_mut("scripts").and_then(|v| v.as_object_mut()) {
                        obj.remove(&old_name);
                        obj.insert(new_name, serde_json::Value::String(new_cmd));
                    }
                    write_file(
                        pkg_path,
                        serde_json::to_string_pretty(&json).unwrap_or_default(),
                    )
                })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.load_scripts(cx);
                    });
                }
            });
        })
        .detach();
    }

    fn delete_script(&mut self, name: &str, cx: &mut Context<Self>) {
        let Some(pkg_path) = self.selected_pkg_path() else {
            return;
        };
        let name = name.to_string();

        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let _ = cx
                .background_spawn(async move {
                    let raw = read_text_file(pkg_path.clone())?;
                    let mut json: serde_json::Value =
                        serde_json::from_str(&raw).map_err(|e| e.to_string())?;
                    if let Some(obj) = json.get_mut("scripts").and_then(|v| v.as_object_mut()) {
                        obj.remove(&name);
                    }
                    write_file(
                        pkg_path,
                        serde_json::to_string_pretty(&json).unwrap_or_default(),
                    )
                })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.load_scripts(cx);
                    });
                }
            });
        })
        .detach();
    }

    fn create_script(&mut self, cx: &mut Context<Self>) {
        let Some(pkg_path) = self.selected_pkg_path() else {
            return;
        };
        let Some(name_input) = self.new_script_name_input.take() else {
            return;
        };
        let Some(cmd_input) = self.new_script_cmd_input.take() else {
            return;
        };

        let name = name_input.read(cx).value().trim().to_string();
        let cmd = cmd_input.read(cx).value().trim().to_string();
        if name.is_empty() {
            return;
        }
        self.adding_script = false;

        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let _ = cx
                .background_spawn(async move {
                    let raw = read_text_file(pkg_path.clone())?;
                    let mut json: serde_json::Value =
                        serde_json::from_str(&raw).map_err(|e| e.to_string())?;
                    if let Some(obj) = json.get_mut("scripts").and_then(|v| v.as_object_mut()) {
                        obj.insert(name, serde_json::Value::String(cmd));
                    }
                    write_file(
                        pkg_path,
                        serde_json::to_string_pretty(&json).unwrap_or_default(),
                    )
                })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.load_scripts(cx);
                    });
                }
            });
        })
        .detach();
    }

    /// Re-queries the Node.js and npm the selected project runs under, so the
    /// header line reflects a switch, an install, or a change of project.
    /// Unlike the startup probe in `init_discovery`, this never re-scans
    /// projects or reloads the version list.
    fn refresh_runtime_versions(&mut self, cx: &mut Context<Self>) {
        let project = self.run_dir();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let runtimes = cx
                .background_spawn(async move { probe_runtimes(project) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.apply_runtimes(runtimes);
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    /// The folder scripts run in: the selected project, or the scan root.
    fn run_dir(&self) -> Option<PathBuf> {
        let dir = self
            .selected_project
            .clone()
            .unwrap_or_else(|| self.cwd.clone());
        (!dir.is_empty()).then(|| PathBuf::from(dir))
    }

    fn apply_runtimes(&mut self, runtimes: Runtimes) {
        self.not_found = runtimes.node.is_none();
        self.node_ver = runtimes.node;
        self.npm_ver = runtimes.npm;
        self.node_is_built_in = runtimes.built_in;
        self.system_node_ver = runtimes.system_node;
        self.project_node = runtimes.project;
    }

    fn load_managed_versions(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async { node_versions::list(&node_versions::store_dir()) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        match result {
                            Ok(versions) => {
                                panel.versions_loaded = Some(true);
                                panel.managed_versions = versions;
                            }
                            Err(_) => {
                                panel.versions_loaded = Some(false);
                            }
                        }
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    /// Makes `version` the active one, or with `None` hands the choice back
    /// to the system Node.js. Either way this only changes what the IDE
    /// launches: nothing else on the machine is switched.
    fn switch_version(&mut self, version: Option<String>, cx: &mut Context<Self>) {
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move {
                    let store = node_versions::store_dir();
                    match version {
                        Some(version) => node_versions::use_version(&store, &version),
                        None => node_versions::use_system(&store),
                    }
                })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        match result {
                            Ok(output) => panel.script_output.push(output),
                            Err(e) => panel.script_output.push(format!("Error: {e}")),
                        }
                        // The active version is what the header line
                        // ("Node.js … | npm …") reports, so it is now stale —
                        // re-query it alongside the version list.
                        panel.refresh_runtime_versions(cx);
                        panel.load_managed_versions(cx);
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    fn request_install(&mut self, version: String, cx: &mut Context<Self>) {
        self.install_for_project = None;
        self.version_step = Some(VersionStep::Confirm(VersionAction::Install, version));
        cx.notify();
    }

    fn request_uninstall(&mut self, version: String, cx: &mut Context<Self>) {
        self.install_for_project = None;
        self.version_step = Some(VersionStep::Confirm(VersionAction::Uninstall, version));
        cx.notify();
    }

    fn cancel_version_step(&mut self, cx: &mut Context<Self>) {
        self.install_for_project = None;
        self.version_step = None;
        cx.notify();
    }

    fn confirm_version_action(&mut self, cx: &mut Context<Self>) {
        let Some(VersionStep::Confirm(action, version)) = self.version_step.clone() else {
            return;
        };
        self.version_step = Some(VersionStep::Running(action, version.clone()));
        cx.notify();

        let project = self.install_for_project.take();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let version_for_run = version.clone();
            let result = cx
                .background_spawn(async move {
                    let store = node_versions::store_dir();
                    match action {
                        VersionAction::Install => match project {
                            Some(project) => {
                                node_versions::install_for_project(&store, &project)
                            }
                            None => node_versions::install(&store, &version_for_run),
                        },
                        VersionAction::Uninstall => {
                            node_versions::uninstall(&store, &version_for_run)
                        }
                    }
                })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.version_step = Some(match result {
                            Ok(output) => VersionStep::Done(action, version.clone(), output),
                            Err(e) => VersionStep::Error(action, version.clone(), e),
                        });
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    fn finish_version_step(&mut self, cx: &mut Context<Self>) {
        self.version_step = None;
        self.show_install = false;
        // An install/uninstall can change the active Node, so refresh the
        // header runtime line too, not just the version list.
        self.refresh_runtime_versions(cx);
        self.load_managed_versions(cx);
        cx.notify();
    }

    fn select_project(&mut self, path: String, cx: &mut Context<Self>) {
        self.selected_project = Some(path.clone());
        self.load_scripts(cx);
        self.reload_package_data(cx);
        // Each project can ask for its own Node.js version.
        self.refresh_runtime_versions(cx);

        let proj = path.clone();
        let sppkg_proj = path.clone();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let spfx = cx.background_spawn(async move { detect_spfx(&proj) }).await;
            let sppkg = cx
                .background_spawn(async move { scan_sppkg(&sppkg_proj) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.spfx = spfx;
                        panel.sppkg_files = sppkg;
                        cx.notify();
                    });
                }
            });
        })
        .detach();

        cx.notify();
    }

    /// Re-reads everything that depends on the selected project: installed
    /// packages plus the outdated/vulnerable probes. Mirrors
    /// `dotnet_panel::reload_data`.
    fn reload_package_data(&mut self, cx: &mut Context<Self>) {
        self.reload_packages(cx);
        self.reload_outdated(cx);
        self.reload_vulnerable(cx);
    }

    fn reload_packages(&mut self, cx: &mut Context<Self>) {
        let dir = self
            .selected_project
            .clone()
            .unwrap_or_else(|| self.cwd.clone());
        if dir.is_empty() {
            self.installed_pkgs = Vec::new();
            cx.notify();
            return;
        }
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let packages = cx
                .background_spawn(async move {
                    let cli = detect_package_manager(&dir).cli_name();
                    list_installed(&dir, cli).unwrap_or_default()
                })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.installed_pkgs = packages;
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    fn reload_outdated(&mut self, cx: &mut Context<Self>) {
        let dir = self
            .selected_project
            .clone()
            .unwrap_or_else(|| self.cwd.clone());
        if dir.is_empty() {
            self.outdated = PackagesState::Ready(Vec::new());
            cx.notify();
            return;
        }
        self.outdated = PackagesState::Loading;
        cx.notify();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move {
                    let cli = detect_package_manager(&dir).cli_name();
                    list_outdated(&dir, cli)
                })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.outdated = match result {
                            Ok(list) => PackagesState::Ready(list),
                            Err(e) => PackagesState::Error(e),
                        };
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    fn reload_vulnerable(&mut self, cx: &mut Context<Self>) {
        let dir = self
            .selected_project
            .clone()
            .unwrap_or_else(|| self.cwd.clone());
        if dir.is_empty() {
            self.vulnerable = PackagesState::Ready(Vec::new());
            cx.notify();
            return;
        }
        self.vulnerable = PackagesState::Loading;
        cx.notify();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move {
                    let cli = detect_package_manager(&dir).cli_name();
                    list_audit_vulns(&dir, cli)
                })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.vulnerable = match result {
                            Ok(list) => PackagesState::Ready(list),
                            Err(e) => PackagesState::Error(e),
                        };
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    /// The live Script Runner dock panel, if `set_script_runner` has been
    /// called yet — same purpose as `dotnet_panel::runner`, but simpler
    /// since `NodePanel` already holds the `WeakEntity` directly rather
    /// than looking it up through the workspace each time.
    fn runner(&self) -> Option<Entity<ScriptRunnerPanel>> {
        self.script_runner.as_ref().and_then(|w| w.upgrade())
    }

    /// Bulk-updates every outdated package at or below `kind_ceiling`
    /// ("minor" = patch+minor, matching `dotnet_panel::update_all`'s "safe"
    /// semantics; "patch" = patch only), via `npm update` (updates within
    /// each dependency's declared semver range — matches what `npm
    /// outdated`'s "wanted" column already promises, so this never needs an
    /// explicit version list). Reuses `dispatch_script` for the actual run,
    /// then observes the Script Runner panel so `reload_package_data` re-hits
    /// once the update finishes (`dispatch_script` itself has no
    /// completion hook, since scripts/quick actions don't need one).
    fn update_all_outdated(
        &mut self,
        kind_ceiling: UpdateKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.running_action.is_some() {
            return;
        }
        let PackagesState::Ready(list) = &self.outdated else {
            return;
        };
        let command = match update_command(list, kind_ceiling) {
            Ok(Some(command)) => command,
            Ok(None) => return,
            Err(error) => {
                self.script_output.push(error);
                cx.notify();
                return;
            }
        };

        // Guard against `dispatch_script`'s own busy check silently no-oping
        // and leaving `running_action` stuck set forever with no completion
        // event to clear it.
        if let Some(runner) = self.runner()
            && runner.read(cx).is_running()
        {
            self.script_output
                .push("Script Runner is busy — stop the current run before updating.".to_string());
            cx.notify();
            return;
        }

        let label = match kind_ceiling {
            UpdateKind::Patch => "update-patch",
            _ => "update-safe",
        };
        self.running_action = Some(label.to_string());

        self.dispatch_script(&command, window, cx);

        if let Some(runner) = self.runner() {
            self.run_subscription = Some(cx.observe(&runner, |this, runner, cx| {
                if !runner.read(cx).is_running() {
                    this.running_action = None;
                    this.reload_package_data(cx);
                    cx.notify();
                }
            }));
        }
    }

    fn start_edit_script(
        &mut self,
        name: &str,
        cmd: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editing_script = Some(name.to_string());
        let name_val = name.to_string();
        let cmd_val = cmd.to_string();
        self.edit_name_input =
            Some(cx.new(|cx| InputState::new(window, cx).default_value(name_val)));
        self.edit_cmd_input = Some(cx.new(|cx| InputState::new(window, cx).default_value(cmd_val)));
        cx.notify();
    }

    fn cancel_edit_script(&mut self, cx: &mut Context<Self>) {
        self.editing_script = None;
        self.edit_name_input = None;
        self.edit_cmd_input = None;
        cx.notify();
    }

    fn view_package_json(&mut self, cx: &mut Context<Self>) {
        let Some(pkg_path) = self.selected_pkg_path() else {
            return;
        };
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move { read_text_file(pkg_path) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.view_pkg = result.ok().and_then(|raw| parse_package_json_view(&raw));
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    fn close_view_pkg(&mut self, cx: &mut Context<Self>) {
        self.view_pkg = None;
        cx.notify();
    }

    fn on_view_pkg(&mut self, _: &ViewPackageJson, _window: &mut Window, cx: &mut Context<Self>) {
        self.view_package_json(cx);
    }
}

// ── Background init ───────────────────────────────────────────────────

/// What [`probe_runtimes`] found.
struct Runtimes {
    /// The Node.js scripts will run under, `None` when there is none at all.
    node: Option<String>,
    npm: Option<String>,
    /// Whether `node` is a built-in version rather than the system one.
    built_in: bool,
    /// The Node.js on the `PATH`, independent of the built-in manager.
    system_node: Option<String>,
    project: ProjectNode,
}

/// Works out which Node.js and npm a script run in `project` will get, by
/// asking them for their versions. Blocks on several process launches, so
/// call it from a background thread.
fn probe_runtimes(project: Option<PathBuf>) -> Runtimes {
    let store = node_versions::store_dir();
    let system_node = query_runtime("node".into()).ok();
    let project_node = project
        .as_deref()
        .map(|project| node_versions::project_node(&store, project))
        .unwrap_or_default();
    let bin_dir = node_versions::active_bin_dir(&store);
    let built_in = bin_dir.is_some();
    let (node, npm) = match bin_dir {
        // A built-in version applies: ask the executables in its own folder.
        Some(bin_dir) => {
            let exe = |windows: &str, other: &str| {
                bin_dir
                    .join(if cfg!(windows) { windows } else { other })
                    .to_string_lossy()
                    .into_owned()
            };
            (
                query_runtime(exe("node.exe", "node")).ok(),
                query_runtime(exe("npm.cmd", "npm")).ok(),
            )
        }
        None => (system_node.clone(), query_runtime("npm".into()).ok()),
    };
    Runtimes {
        node,
        npm,
        built_in,
        system_node,
        project: project_node,
    }
}

fn init_discovery(cx: &mut Context<NodePanel>) {
    cx.spawn(
        async move |this: WeakEntity<NodePanel>, cx: &mut AsyncApp| {
            let runtimes = cx
                .background_spawn(async { probe_runtimes(None) })
                .await;

            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.apply_runtimes(runtimes);
                        // The built-in manager needs no Node.js on the
                        // machine: it is how one gets installed.
                        panel.load_managed_versions(cx);
                        panel.scan_projects(cx);
                        cx.notify();
                    });
                }
            });
        },
    )
    .detach();
}

// ── Section header helper ─────────────────────────────────────────────

fn section_container(
    cx: &Context<NodePanel>,
    title: &str,
    count: usize,
    open: bool,
    body: impl IntoElement,
) -> impl IntoElement {
    let key = title.to_string();

    let header = div()
        .id(format!("node-section-{key}"))
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
        .child(Tag::secondary().xsmall().child(count.to_string()))
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

// ── package.json modal helpers ────────────────────────────────────────

fn render_dep_list(
    title: &str,
    entries: &IndexMap<String, String>,
    theme: &gpui_component::theme::Theme,
) -> Option<impl IntoElement> {
    if entries.is_empty() {
        return None;
    }
    let mut col = div().flex().flex_col().gap_1().mb_3().child(
        div()
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(theme.muted_foreground)
            .child(title.to_string()),
    );
    for (name, value) in entries {
        col = col.child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_3()
                .py_0p5()
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .font_family("Cascadia Mono")
                        .text_xs()
                        .text_color(theme.foreground)
                        .child(name.clone()),
                )
                .child(
                    div()
                        .flex_shrink_0()
                        .font_family("Cascadia Mono")
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(value.clone()),
                ),
        );
    }
    Some(col)
}

fn render_package_json_body(
    view: &PackageJsonView,
    theme: &gpui_component::theme::Theme,
) -> impl IntoElement {
    let mut col = div().flex().flex_col();

    col = col.child(
        div()
            .flex()
            .items_baseline()
            .gap_2()
            .mb_1()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.foreground)
                    .child(view.name.clone().unwrap_or_else(|| "(unnamed)".to_string())),
            )
            .when_some(view.version.clone(), |d, v| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(format!("v{v}")),
                )
            }),
    );

    if let Some(desc) = &view.description {
        col = col.child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .mb_3()
                .child(desc.clone()),
        );
    }

    if let Some(scripts) = render_dep_list("Scripts", &view.scripts, theme) {
        col = col.child(scripts);
    }
    if let Some(deps) = render_dep_list("Dependencies", &view.dependencies, theme) {
        col = col.child(deps);
    }
    if let Some(dev_deps) = render_dep_list("Dev Dependencies", &view.dev_dependencies, theme) {
        col = col.child(dev_deps);
    }

    col
}

// ── Render ────────────────────────────────────────────────────────────

impl Render for NodePanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let panel_header = PanelHeader::new("Node").icon(
            svg()
                .path("icons/node.svg")
                .size(px(16.0))
                .text_color(cx.theme().foreground),
        );

        // No early return when Node.js is missing: the Node Versions section
        // below is where one gets installed, so the panel has to stay usable.
        let versions_section = section_container(
            cx,
            "Node Versions",
            self.managed_versions.len(),
            self.is_open("Node Versions", true),
            self.render_node_versions(cx),
        );

        let projects_section = section_container(
            cx,
            "Projects",
            self.projects.len(),
            self.is_open("Projects", true),
            self.render_projects(cx),
        );

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

        let outdated_count = match &self.outdated {
            PackagesState::Ready(list) => list.len(),
            _ => 0,
        };
        let outdated_section = section_container(
            cx,
            "Outdated",
            outdated_count,
            self.is_open("Outdated", true),
            self.render_outdated(cx),
        );

        let vulnerable_count = match &self.vulnerable {
            PackagesState::Ready(list) => list.len(),
            _ => 0,
        };
        let vulnerabilities_section = section_container(
            cx,
            "Vulnerabilities",
            vulnerable_count,
            self.is_open("Vulnerabilities", true),
            self.render_vulnerabilities(cx),
        );

        let script_count = self.scripts.as_ref().map_or(0, |s| s.len());
        let scripts_section = section_container(
            cx,
            "Scripts",
            script_count,
            self.is_open("Scripts", true),
            self.render_scripts(window, cx),
        );

        let processes_section = section_container(
            cx,
            "Processes",
            self.node_procs.len(),
            self.is_open("Processes", true),
            self.render_processes(cx),
        );

        let mut body = div()
            .flex()
            .flex_col()
            .child(self.render_versions(cx))
            .child(versions_section)
            .child(projects_section)
            .child(actions_section)
            .child(packages_section)
            .child(outdated_section)
            .child(vulnerabilities_section);

        if self.spfx.is_spfx {
            body = body.child(section_container(
                cx,
                "SharePoint Framework",
                self.sppkg_files.len(),
                self.is_open("SharePoint Framework", true),
                self.render_spfx(cx),
            ));
        }

        body = body.child(scripts_section);

        body = body.child(processes_section);

        if !self.script_output.is_empty() {
            body = body.child(self.render_output(cx));
        }

        // Coerce each modal to `AnyElement` so none of them holds the
        // `&mut Window` borrow across the successive calls (Rust 2024's
        // `impl Trait` lifetime-capture rules would otherwise reject the
        // second mutable borrow below).
        let version_modal = self
            .render_version_modal(window, cx)
            .map(|m| m.into_any_element());
        let pkg_modal = self
            .render_pkg_modal(window, cx)
            .map(|m| m.into_any_element());

        div()
            .id("node-panel")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::on_view_pkg))
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .overflow_hidden()
            .bg(cx.theme().background)
            .child(panel_header)
            .child(
                div()
                    .id("node-scroll")
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .overflow_y_scroll()
                    .child(body),
            )
            .children(pkg_modal)
            .children(version_modal)
            .into_any_element()
    }
}

impl Focusable for NodePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for NodePanel {}

impl Panel for NodePanel {
    fn persistent_name() -> &'static str {
        "Node Panel"
    }

    fn panel_key() -> &'static str {
        "NodePanel"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        match NodePanelSettings::get_global(cx).dock {
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
            settings.node_panel.get_or_insert_default().dock = Some(dock);
        });
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(260.)
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<ui::IconName> {
        NodePanelSettings::get_global(cx).button.then_some(ui::IconName::Node)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Node")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        26
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.node_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

// ── Section renderers ─────────────────────────────────────────────────

impl NodePanel {
    fn render_versions(&self, cx: &Context<Self>) -> impl IntoElement {
        let source = if self.node_is_built_in {
            "built-in"
        } else {
            "system"
        };
        let text = match (&self.node_ver, &self.npm_ver) {
            (Some(n), Some(pm)) => format!("Node.js {n} ({source})  |  npm {pm}"),
            (Some(n), None) => format!("Node.js {n} ({source})"),
            _ => "Node.js isn't installed".into(),
        };

        // What the selected project's own version file asks for. It is never
        // applied by itself: the version in use is the one picked below, and
        // this line only says how the two compare. When the matching version
        // is installed but isn't the one picked, switching is a click away;
        // when it isn't installed, installing it is.
        let active = self
            .managed_versions
            .iter()
            .find(|version| version.current)
            .map(|version| version.version.clone());
        let project_line = self.project_node.requested.clone().map(|requested| {
            match self.project_node.installed.clone() {
                Some(installed) if active.as_ref() == Some(&installed) => div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!("Project asks for {requested} \u{2014} using {installed}"))
                    .into_any_element(),
                Some(installed) => h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().warning)
                            .child(format!("Project asks for {requested}")),
                    )
                    .child(
                        Button::new("node-version-use-project")
                            .ghost()
                            .xsmall()
                            .label(format!("Use {installed}"))
                            .on_click(cx.listener(move |this, _e, _w, cx| {
                                this.switch_version(Some(installed.clone()), cx);
                            })),
                    )
                    .into_any_element(),
                None => h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().warning)
                            .child(format!(
                                "Project asks for {requested}, which isn't installed"
                            )),
                    )
                    .child(
                        Button::new("node-version-install-project")
                            .ghost()
                            .xsmall()
                            .icon(IconName::Plus)
                            .label("Install")
                            .on_click(cx.listener(move |this, _e, _w, cx| {
                                this.request_install(requested.clone(), cx);
                                // Install from the project's own request, not
                                // from the text shown: a `package.json` range
                                // can't be asked for by name.
                                this.install_for_project = this.run_dir();
                            })),
                    )
                    .into_any_element(),
            }
        });

        div()
            .flex()
            .flex_col()
            .gap_1()
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
            .children(project_line)
    }

    fn render_node_versions(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();

        match self.versions_loaded {
            None => {
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
                                .child("Loading Node.js versions\u{2026}"),
                        ),
                );
            }
            Some(false) => {
                col = col.child(
                    div().flex().items_center().px_3().py_2().child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("Couldn't read the installed Node.js versions"),
                    ),
                );
            }
            Some(true) => {
                col = col.child(
                    div().px_3().py_0p5().child(
                        Button::new("node-version-install-toggle")
                            .ghost()
                            .xsmall()
                            .selected(self.show_install)
                            .label(if self.show_install {
                                "Hide Available"
                            } else {
                                "Install Version\u{2026}"
                            })
                            .on_click(cx.listener(|this, _e, _w, cx| {
                                this.show_install = !this.show_install;
                                if this.show_install && this.available_versions.is_empty() {
                                    this.loading_available = true;
                                    cx.notify();
                                    let weak = cx.weak_entity();
                                    cx.spawn(async move |_, cx: &mut AsyncApp| {
                                        let result = cx
                                            .background_spawn(async {
                                                node_versions::list_available(
                                                    &node_versions::store_dir(),
                                                )
                                            })
                                            .await;
                                        let _ = cx.update(|app| {
                                            if let Some(panel) = weak.upgrade() {
                                                let _ = panel.update(app, |panel, cx| {
                                                    panel.loading_available = false;
                                                    panel.available_versions =
                                                        result.unwrap_or_default();
                                                    panel.available_page = 0;
                                                    cx.notify();
                                                });
                                            }
                                        });
                                    })
                                    .detach();
                                }
                                cx.notify();
                            })),
                    ),
                );

                // With nothing installed here yet, say what is being used
                // instead and offer the one step that changes it. Nothing
                // has to be set up first: the manager is part of the IDE.
                if self.managed_versions.is_empty() {
                    let explanation = match &self.system_node_ver {
                        Some(version) => format!(
                            "Using the Node.js already on this machine ({version}). \
                             Install a version here to switch between versions per project."
                        ),
                        None => "Node.js isn't installed. Install a version to get started."
                            .to_string(),
                    };
                    col = col
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(explanation),
                        )
                        .child(
                            div().px_3().py_0p5().child(
                                Button::new("node-version-install-lts")
                                    .primary()
                                    .xsmall()
                                    .icon(IconName::Plus)
                                    .label("Install latest LTS")
                                    .on_click(cx.listener(|this, _e, _w, cx| {
                                        this.request_install("lts".to_string(), cx);
                                    })),
                            ),
                        );
                }

                // The Node.js on the `PATH`, however it got there. It is in
                // use whenever no built-in version is active, and picking it
                // hands the choice back without removing anything.
                if let Some(system) = &self.system_node_ver {
                    let system_current = !self.managed_versions.iter().any(|v| v.current);
                    let color = if system_current {
                        cx.theme().primary
                    } else {
                        cx.theme().foreground
                    };
                    col = col.child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .px_3()
                            .py_0p5()
                            .text_xs()
                            .child(
                                div()
                                    .w_3()
                                    .flex_shrink_0()
                                    .text_color(if system_current {
                                        cx.theme().primary
                                    } else {
                                        cx.theme().muted_foreground
                                    })
                                    .child(if system_current { "\u{25CF}" } else { "\u{25CB}" }),
                            )
                            .child(
                                Button::new("node-version-system")
                                    .custom(ButtonCustomVariant::new(cx).foreground(color))
                                    .xsmall()
                                    .label(format!("System ({system})"))
                                    .tooltip(if system_current {
                                        "In use: the Node.js already on this machine"
                                    } else {
                                        "Use the Node.js already on this machine"
                                    })
                                    .on_click(cx.listener(|this, _e, _w, cx| {
                                        this.switch_version(None, cx);
                                    })),
                            ),
                    );
                }

                for v in &self.managed_versions {
                    let marker = if v.current { "\u{25CF}" } else { "\u{25CB}" };
                    let marker_color = if v.current {
                        cx.theme().primary
                    } else {
                        cx.theme().muted_foreground
                    };
                    let color = if v.current {
                        cx.theme().primary
                    } else {
                        cx.theme().foreground
                    };
                    let ver = v.version.clone();
                    let ver_for_switch = ver.clone();
                    let ver_for_delete = ver.clone();
                    col = col.child(
                        div()
                            .flex()
                            .items_center()
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
                                    .child(
                                        div()
                                            .w_3()
                                            .flex_shrink_0()
                                            .text_color(marker_color)
                                            .child(marker),
                                    )
                                    .child(
                                        Button::new(format!("node-version-ver-{ver}"))
                                            .custom(ButtonCustomVariant::new(cx).foreground(color))
                                            .xsmall()
                                            .label(v.version.clone())
                                            .tooltip(if v.current {
                                                "Current version"
                                            } else {
                                                "Use this version"
                                            })
                                            .on_click(cx.listener(move |this, _e, _w, cx| {
                                                this.switch_version(
                                                    Some(ver_for_switch.clone()),
                                                    cx,
                                                );
                                            })),
                                    ),
                            )
                            .child(
                                Button::new(format!("node-version-uninstall-{ver}"))
                                    .custom(
                                        ButtonCustomVariant::new(cx)
                                            .foreground(cx.theme().muted_foreground),
                                    )
                                    .xsmall()
                                    .icon(IconName::CircleX)
                                    .tooltip("Uninstall")
                                    .on_click(cx.listener(move |this, _e, _w, cx| {
                                        this.request_uninstall(ver_for_delete.clone(), cx);
                                    })),
                            ),
                    );
                }

                if self.show_install {
                    col = col.child(
                        div()
                            .px_3()
                            .pt_2()
                            .pb_1()
                            .border_t_1()
                            .border_color(cx.theme().border),
                    );

                    if self.loading_available {
                        col = col.child(
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
                    } else {
                        let installed: HashSet<&str> = self
                            .managed_versions
                            .iter()
                            .map(|v| v.version.as_str())
                            .collect();
                        let available: Vec<&String> = self
                            .available_versions
                            .iter()
                            // An entry may carry an LTS note after the version.
                            .filter(|v| !installed.contains(node_versions::version_of(v)))
                            .collect();

                        let start = self.available_page * PAGE_SIZE;
                        let page: Vec<String> = available
                            .iter()
                            .skip(start)
                            .take(PAGE_SIZE)
                            .map(|s| (*s).clone())
                            .collect();

                        for v in &page {
                            let v = v.clone();
                            let ver = v.clone();
                            col = col.child(
                                div().px_3().py_0p5().child(
                                    Button::new(format!("node-version-install-{v}"))
                                        .ghost()
                                        .xsmall()
                                        .icon(IconName::Plus)
                                        .label(v.clone())
                                        .on_click(cx.listener(move |this, _e, _w, cx| {
                                            this.request_install(ver.clone(), cx);
                                        })),
                                ),
                            );
                        }

                        let total_pages = (available.len() + PAGE_SIZE - 1) / PAGE_SIZE;
                        if total_pages > 1 {
                            let can_prev = self.available_page > 0;
                            let can_next = self.available_page + 1 < total_pages;
                            col = col.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .px_3()
                                    .py_1()
                                    .text_xs()
                                    .child(
                                        Button::new("node-version-available-prev")
                                            .ghost()
                                            .xsmall()
                                            .label("\u{2039} Prev")
                                            .disabled(!can_prev)
                                            .on_click(cx.listener(|this, _e, _w, cx| {
                                                if this.available_page > 0 {
                                                    this.available_page -= 1;
                                                    cx.notify();
                                                }
                                            })),
                                    )
                                    .child(div().text_color(cx.theme().muted_foreground).child(
                                        format!(
                                            "Page {} of {}",
                                            self.available_page + 1,
                                            total_pages
                                        ),
                                    ))
                                    .child(
                                        Button::new("node-version-available-next")
                                            .ghost()
                                            .xsmall()
                                            .label("Next \u{203A}")
                                            .disabled(!can_next)
                                            .on_click(cx.listener(move |this, _e, _w, cx| {
                                                if this.available_page + 1 < total_pages {
                                                    this.available_page += 1;
                                                    cx.notify();
                                                }
                                            })),
                                    ),
                            );
                        }
                    }
                }
            }
        }

        col
    }

    fn render_projects(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();

        if self.scanning {
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
        } else {
            for proj in &self.projects {
                let selected = self.selected_project.as_deref() == Some(&proj.path);
                let color = if selected {
                    cx.theme().primary
                } else {
                    cx.theme().foreground
                };
                let proj_path = proj.path.clone();
                col = col.child(
                    div()
                        .id(format!("proj-{}", proj.path))
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
                                .font_family("Cascadia Mono")
                                .text_color(color)
                                .child(proj.name.clone()),
                        ),
                );
            }
        }

        col
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
                        .when(pkg.is_dev, |d| {
                            d.child(Tag::secondary().xsmall().child("dev"))
                        })
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
                    h_flex()
                        .gap_1()
                        .items_center()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .child(
                            div()
                                .flex_1()
                                .truncate()
                                .text_color(cx.theme().danger)
                                .child(err.clone()),
                        )
                        .child(
                            Button::new("outdated-retry")
                                .secondary()
                                .xsmall()
                                .label("Retry")
                                .on_click(cx.listener(|this, _e, _w, cx| {
                                    this.reload_outdated(cx);
                                })),
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
                    let updating = self.running_action.is_some();
                    col = col.child(
                        h_flex()
                            .gap_1()
                            .flex_wrap()
                            .items_center()
                            .px_3()
                            .py_1()
                            .child(
                                Button::new("update-all-safe")
                                    .secondary()
                                    .xsmall()
                                    .label("Update all (minor)")
                                    .disabled(updating)
                                    .on_click(cx.listener(|this, _e, window, cx| {
                                        this.update_all_outdated(UpdateKind::Minor, window, cx);
                                    })),
                            )
                            .child(
                                Button::new("update-all-patch")
                                    .secondary()
                                    .xsmall()
                                    .label("Update all (patch)")
                                    .disabled(updating)
                                    .on_click(cx.listener(|this, _e, window, cx| {
                                        this.update_all_outdated(UpdateKind::Patch, window, cx);
                                    })),
                            )
                            .when(updating, |row| row.child(Spinner::new().xsmall())),
                    );

                    for pkg in list {
                        let kind = classify_update(&pkg.current, &pkg.latest);
                        // Red is reserved for the Vulnerabilities section —
                        // "major" here doesn't mean "broken", just "bigger
                        // diff to review", matching `dotnet_panel`'s
                        // corrected scheme.
                        let (kind_color, kind_label) = match kind {
                            UpdateKind::Major => (cx.theme().warning, "major"),
                            UpdateKind::Minor => (cx.theme().info, "minor"),
                            UpdateKind::Patch => (cx.theme().muted_foreground, "patch"),
                            UpdateKind::Current => (cx.theme().muted_foreground, "current"),
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
                                        .child(format!("{} \u{2192} {}", pkg.current, pkg.latest)),
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

        match &self.vulnerable {
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
                                .child("Scanning for vulnerabilities\u{2026}"),
                        ),
                );
            }
            PackagesState::Error(err) => {
                col = col.child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .child(
                            div()
                                .flex_1()
                                .truncate()
                                .text_color(cx.theme().danger)
                                .child(err.clone()),
                        )
                        .child(
                            Button::new("vulnerable-retry")
                                .secondary()
                                .xsmall()
                                .label("Retry")
                                .on_click(cx.listener(|this, _e, _w, cx| {
                                    this.reload_vulnerable(cx);
                                })),
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
                            .child("No known vulnerabilities"),
                    );
                } else {
                    for vuln in list {
                        // Same severity->color mapping `npm_manager_panel`'s
                        // own Vulnerabilities page already established —
                        // reused here rather than inventing a fourth scheme.
                        let severity_color = match vuln.severity.as_str() {
                            "critical" | "high" => cx.theme().danger,
                            "moderate" => cx.theme().warning,
                            _ => cx.theme().muted_foreground,
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
                                        .child(vuln.package.clone()),
                                )
                                .child(
                                    div()
                                        .rounded_md()
                                        .px_1()
                                        .py_0p5()
                                        .bg(severity_color.opacity(0.2))
                                        .text_color(severity_color)
                                        .child(vuln.severity.clone()),
                                ),
                        );
                        if !vuln.title.is_empty() {
                            col = col.child(
                                div()
                                    .px_3()
                                    .pb_1()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(vuln.title.clone()),
                            );
                        }
                    }
                }
            }
        }

        col
    }

    fn render_quick_actions(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut row = div().flex().flex_row().flex_wrap().gap_1().px_3().py_1();

        // `dev`/`build`/`test`/`install` dispatch a fixed command to the
        // Script Runner panel via `set_script_runner`, matching the Forge
        // original's `dispatch_script` (node_panel.rs:1523-1526 there).
        // `install` runs plain `npm install` — the source panel's own
        // dependency-counting progress UI for it was dropped in this port
        // (see the module doc), so it just streams npm's own output like
        // any other quick action instead.
        let actions = [
            ("dev", "npm run dev"),
            ("build", "npm run build"),
            ("test", "npm run test"),
            ("install", "npm install"),
        ];

        for (label, cmd) in actions {
            row = row.child(
                Button::new(format!("quick-action-{label}"))
                    .secondary()
                    .xsmall()
                    .label(label)
                    .tooltip(format!("Run \"{cmd}\""))
                    .on_click(cx.listener(move |this, _e, window, cx| {
                        this.dispatch_script(cmd, window, cx);
                    })),
            );
        }

        // "task chain" — opens the Flows panel's Add Task Chain wizard,
        // anchored on the selected project, matching Forge's node_panel
        // quick-actions row (`__task_chain__`, node_panel.rs:1555-1559
        // there). Disabled with no Flows panel wired up yet (see
        // `set_flows_panel`) rather than silently no-op on click.
        row = row.child(
            Button::new("quick-action-task-chain")
                .secondary()
                .xsmall()
                .label("Task Chain")
                .tooltip("Add this project to a task chain")
                .disabled(self.flows_panel.is_none())
                .on_click(cx.listener(|this, _e, window, cx| {
                    this.open_task_chain_wizard(window, cx);
                })),
        );

        // `open_npm_manager` (dropped in the original Ⱨubbard port) re-added:
        // opens the Node Package Manager workspace tab rooted on the current
        // project.
        row = row.child(
            Button::new("quick-action-npm-mgr")
                .secondary()
                .xsmall()
                .label("Package Manager")
                .tooltip("Open Node Package Manager")
                .on_click(cx.listener(|this, _e, window, cx| {
                    // Use the `Window` already handed to this click handler
                    // rather than `WeakEntity::update_in`'s own internal
                    // window lookup — the latter turned out unreliable for
                    // this kind of cross-entity call (see `dispatch_script`).
                    if let Some(workspace) = this.workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            // Same "first worktree, else process cwd"
                            // resolution as the Node scan itself, so the tab
                            // opens even when the click lands before a
                            // project worktree is ready.
                            let root = project_root(workspace, cx).unwrap_or_else(|| {
                                std::env::current_dir()
                                    .map(|p| p.display().to_string())
                                    .unwrap_or_default()
                            });
                            if !root.is_empty() {
                                npm_manager_panel::open(root, workspace, window, cx);
                            }
                        });
                    }
                })),
        );

        row
    }

    fn render_scripts(&self, window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
        let _ = window; // inputs created in click handlers
        let mut col = div().flex().flex_col();

        if let Some(scripts) = &self.scripts {
            for (name, cmd) in scripts {
                let name_c = name.clone();

                if self.editing_script.as_deref() == Some(name) {
                    let edit_row = h_flex()
                        .gap_1()
                        .px_3()
                        .py_0p5()
                        .text_xs()
                        .when_some(self.edit_name_input.as_ref(), |row, input| {
                            row.child(Input::new(input).xsmall().w(px(120.)))
                        })
                        .when_some(self.edit_cmd_input.as_ref(), |row, input| {
                            row.child(div().flex_1().child(Input::new(input).xsmall()))
                        })
                        .child(
                            Button::new(format!("edit-save-{name_c}"))
                                .custom(ButtonCustomVariant::new(cx).foreground(cx.theme().primary))
                                .xsmall()
                                .child("\u{2713}")
                                .tooltip("Save")
                                .on_click(cx.listener(|this, _e, _w, cx| {
                                    this.save_script_edit(cx);
                                })),
                        )
                        .child(
                            Button::new(format!("edit-cancel-{name_c}"))
                                .custom(ButtonCustomVariant::new(cx).foreground(cx.theme().danger))
                                .xsmall()
                                .child("\u{2717}")
                                .tooltip("Cancel")
                                .on_click(cx.listener(|this, _e, _w, cx| {
                                    this.cancel_edit_script(cx);
                                })),
                        );
                    col = col.child(edit_row);
                } else {
                    let name_for_edit = name.clone();
                    let cmd_for_edit = cmd.clone();
                    let name_for_delete = name.clone();
                    // Dispatch via `npm run <name>`, not the script's raw
                    // body: a body that itself chains a nested `npm run`
                    // (e.g. `"start": "npm run compile && heft start"`)
                    // would otherwise run as a separate, complete `npm.ps1`
                    // invocation partway through our own shell command —
                    // and npm's PowerShell shim ends with `exit
                    // $LASTEXITCODE`, which kills the *whole* non-interactive
                    // PowerShell host the moment that inner npm call
                    // finishes, silently dropping everything chained after
                    // it. Running `npm run <name>` instead lets npm own the
                    // entire chain in one process, exactly like typing it
                    // in a terminal does.
                    let play_command = run_script_command(&name_c);

                    let row = div()
                        .id(format!("script-{name_c}"))
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .py_0p5()
                        .text_xs()
                        .child(
                            Button::new(format!("play-{name_c}"))
                                .custom(ButtonCustomVariant::new(cx).foreground(cx.theme().primary))
                                .xsmall()
                                .child("\u{25B6}")
                                .tooltip("Run in Script Runner")
                                .on_click(cx.listener(move |this, _e, window, cx| {
                                    match &play_command {
                                        Ok(command) => this.dispatch_script(command, window, cx),
                                        Err(error) => {
                                            this.script_output.push(error.clone());
                                            cx.notify();
                                        }
                                    }
                                })),
                        )
                        .child(
                            div()
                                .flex_1()
                                .truncate()
                                .font_family("Cascadia Mono")
                                .text_color(cx.theme().foreground)
                                .child(name.clone()),
                        )
                        .child(
                            Button::new(format!("edit-{name_c}"))
                                .custom(
                                    ButtonCustomVariant::new(cx)
                                        .foreground(cx.theme().muted_foreground),
                                )
                                .xsmall()
                                .child("\u{270E}")
                                .tooltip("Edit")
                                .on_click(cx.listener(move |this, _e, window, cx| {
                                    this.start_edit_script(
                                        &name_for_edit,
                                        &cmd_for_edit,
                                        window,
                                        cx,
                                    );
                                })),
                        )
                        .child(
                            Button::new(format!("delete-{name_c}"))
                                .custom(
                                    ButtonCustomVariant::new(cx)
                                        .foreground(cx.theme().muted_foreground),
                                )
                                .xsmall()
                                .child("\u{2715}")
                                .tooltip("Delete")
                                .on_click(cx.listener(move |this, _e, _w, cx| {
                                    this.delete_script(&name_for_delete, cx);
                                })),
                        )
                        .context_menu(move |menu, _, _| {
                            menu.menu("View package.json", Box::new(ViewPackageJson))
                        });

                    col = col.child(row);
                }
            }

            col = col.child(
                div().px_3().py_1().child(
                    Button::new("add-script-btn")
                        .custom(ButtonCustomVariant::new(cx).foreground(cx.theme().primary))
                        .xsmall()
                        .icon(IconName::Plus)
                        .label(if self.adding_script {
                            "Cancel add"
                        } else {
                            "Add script"
                        })
                        .on_click(cx.listener(|this, _e, window, cx| {
                            if !this.adding_script {
                                this.adding_script = true;
                                this.new_script_name_input = Some(
                                    cx.new(|cx| InputState::new(window, cx).placeholder("Name")),
                                );
                                this.new_script_cmd_input =
                                    Some(cx.new(|cx| {
                                        InputState::new(window, cx).placeholder("Command")
                                    }));
                            } else {
                                this.adding_script = false;
                                this.new_script_name_input = None;
                                this.new_script_cmd_input = None;
                            }
                            cx.notify();
                        })),
                ),
            );

            if self.adding_script {
                col = col.child(
                    h_flex()
                        .gap_2()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .when_some(self.new_script_name_input.as_ref(), |row, input| {
                            row.child(Input::new(input).xsmall().w(px(120.)))
                        })
                        .when_some(self.new_script_cmd_input.as_ref(), |row, input| {
                            row.child(div().flex_1().child(Input::new(input).xsmall()))
                        })
                        .child(
                            Button::new("add-script-save")
                                .custom(ButtonCustomVariant::new(cx).foreground(cx.theme().primary))
                                .xsmall()
                                .child("\u{2713}")
                                .tooltip("Save")
                                .on_click(cx.listener(|this, _e, _w, cx| {
                                    this.create_script(cx);
                                })),
                        )
                        .child(
                            Button::new("add-script-cancel")
                                .custom(ButtonCustomVariant::new(cx).foreground(cx.theme().danger))
                                .xsmall()
                                .child("\u{2717}")
                                .tooltip("Cancel")
                                .on_click(cx.listener(|this, _e, _w, cx| {
                                    this.adding_script = false;
                                    this.new_script_name_input = None;
                                    this.new_script_cmd_input = None;
                                    cx.notify();
                                })),
                        ),
                );
            }
        } else {
            col = col.child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("No project selected"),
            );
        }

        col
    }

    fn render_version_modal(
        &self,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> Option<impl IntoElement> {
        let step = self.version_step.clone()?;
        let viewport = window.viewport_size();

        let title = match &step {
            VersionStep::Confirm(action, _) => match action {
                VersionAction::Install => "Install Node.js version",
                VersionAction::Uninstall => "Uninstall Node.js version",
            },
            VersionStep::Running(action, _) => match action {
                VersionAction::Install => "Installing\u{2026}",
                VersionAction::Uninstall => "Uninstalling\u{2026}",
            },
            VersionStep::Done(action, ..) => match action {
                VersionAction::Install => "Install complete",
                VersionAction::Uninstall => "Uninstall complete",
            },
            VersionStep::Error(action, ..) => match action {
                VersionAction::Install => "Install failed",
                VersionAction::Uninstall => "Uninstall failed",
            },
        };

        let body: AnyElement = match &step {
            VersionStep::Confirm(action, version) => div()
                .text_sm()
                .text_color(cx.theme().foreground)
                .child(format!(
                    "Are you sure you want to {} Node.js {version}?",
                    action.title_verb().to_ascii_lowercase()
                ))
                .into_any_element(),
            VersionStep::Running(action, version) => h_flex()
                .gap_3()
                .items_center()
                .justify_center()
                .py_2()
                .child(Spinner::new())
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child(format!(
                            "{} Node.js {version}\u{2026}",
                            action.progressive()
                        )),
                )
                .into_any_element(),
            VersionStep::Done(action, version, output) => div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().success)
                        .child(format!("Node.js {version} {}.", action.past())),
                )
                .children((!output.trim().is_empty()).then(|| {
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .font_family("Cascadia Mono")
                        .child(output.clone())
                }))
                .into_any_element(),
            VersionStep::Error(action, version, message) => div()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().text_sm().text_color(cx.theme().danger).child(format!(
                    "Failed to {} Node.js {version}.",
                    action.title_verb().to_ascii_lowercase()
                )))
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .font_family("Cascadia Mono")
                        .child(message.clone()),
                )
                .into_any_element(),
        };

        let footer: AnyElement = match &step {
            VersionStep::Confirm(action, version) => {
                let confirm_label = format!("{} {version}", action.title_verb());
                let confirm_variant = match action {
                    VersionAction::Install => ButtonVariant::Success,
                    VersionAction::Uninstall => ButtonVariant::Danger,
                };
                div()
                    .flex()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new("node-version-install-cancel")
                            .outline()
                            .xsmall()
                            .label("Cancel")
                            .on_click(cx.listener(|this, _e, _w, cx| this.cancel_version_step(cx))),
                    )
                    .child(
                        Button::new("node-version-install-confirm")
                            .with_variant(confirm_variant)
                            .xsmall()
                            .label(confirm_label)
                            .on_click(cx.listener(|this, _e, _w, cx| this.confirm_version_action(cx))),
                    )
                    .into_any_element()
            }
            VersionStep::Running(..) => div().into_any_element(),
            VersionStep::Done(..) | VersionStep::Error(..) => div()
                .flex()
                .justify_end()
                .child(
                    Button::new("node-version-install-ok")
                        .success()
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
                            .id("node-version-install-modal-backdrop")
                            .w(viewport.width)
                            .h(viewport.height)
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(gpui::rgba(0x000000aa))
                            .child(
                                div()
                                    .id("node-version-install-modal")
                                    .occlude()
                                    .w(px(400.0))
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
                                            .child(title.to_string()),
                                    )
                                    .child(body)
                                    .child(footer),
                            ),
                    ),
            )
            .with_priority(100),
        )
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
        } else if self.node_procs.is_empty() {
            col = col.child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("No Node processes running"),
            );
        } else {
            // This workspace's own processes first; the rest of the
            // machine's, including the IDE's own (language servers), folded
            // under "Other". Both keep the poll's heaviest-first order.
            let root = PathBuf::from(&self.cwd);
            let (mine, others): (Vec<&NodeProcess>, Vec<&NodeProcess>) = self
                .node_procs
                .iter()
                .partition(|proc| !proc.ide_owned && proc.belongs_to(&root));

            col = col.child(
                h_flex()
                    .gap_2()
                    .px_3()
                    .py_0p5()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(cx.theme().muted_foreground)
                    .child(div().flex_1().child("What"))
                    .child(div().w_12().text_right().child("PID"))
                    .child(div().w_12().text_right().child("CPU"))
                    .child(div().w_16().text_right().child("Mem"))
                    .child(div().w_6()),
            );

            if mine.is_empty() {
                col = col.child(
                    div()
                        .px_3()
                        .py_1()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("None started from this workspace"),
                );
            }
            for proc in &mine {
                col = col.child(self.render_process_row(proc, cx));
            }

            if !others.is_empty() {
                let open = self.show_other_procs;
                col = col.child(
                    h_flex()
                        .id("node-procs-other-toggle")
                        .gap_1()
                        .px_3()
                        .py_0p5()
                        .text_xs()
                        .cursor_pointer()
                        .text_color(cx.theme().muted_foreground)
                        .hover(|el| el.bg(cx.theme().list_hover))
                        .child(
                            Icon::new(if open {
                                IconName::ChevronDown
                            } else {
                                IconName::ChevronRight
                            })
                            .xsmall(),
                        )
                        .child(format!("Other ({})", others.len()))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.show_other_procs = !this.show_other_procs;
                            cx.notify();
                        })),
                );
                if open {
                    for proc in &others {
                        col = col.child(self.render_process_row(proc, cx));
                    }
                }
            }
        }

        col
    }

    /// One process: what it is (full command line on hover), its id, CPU and
    /// memory, and Stop. Stop asks first, in the row itself. The IDE's own
    /// processes have no Stop: ending a language server breaks an editor
    /// feature, and the IDE would only start it again.
    fn render_process_row(&self, proc: &NodeProcess, cx: &Context<Self>) -> AnyElement {
        let pid = proc.pid;
        let mono = |text: String| div().font_family("Cascadia Mono").child(text);
        let command_line = proc.command_line.clone();

        let what = h_flex()
            .id(("node-proc-what", pid as usize))
            .flex_1()
            .min_w_0()
            .gap_1()
            .child(mono(proc.label.clone()).truncate())
            .when(proc.ide_owned, |el| {
                el.child(
                    Tag::custom(
                        cx.theme().info.opacity(0.2),
                        cx.theme().info,
                        cx.theme().info.opacity(0.3),
                    )
                    .xsmall()
                    .child("IDE"),
                )
            })
            .when(!command_line.is_empty(), |el| {
                el.tooltip(move |window, cx| Tooltip::new(command_line.clone()).build(window, cx))
            });

        let action: AnyElement = if proc.ide_owned {
            div().w_6().into_any_element()
        } else if self.confirm_stop == Some(pid) {
            h_flex()
                .gap_1()
                .child(
                    Button::new(("node-proc-stop-confirm", pid as usize))
                        .danger()
                        .xsmall()
                        .label("Stop")
                        .tooltip("Stop this process and everything it started")
                        .on_click(cx.listener(move |this, _, _, cx| this.stop_process(pid, cx))),
                )
                .child(
                    Button::new(("node-proc-stop-cancel", pid as usize))
                        .ghost()
                        .xsmall()
                        .label("Cancel")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.confirm_stop = None;
                            cx.notify();
                        })),
                )
                .into_any_element()
        } else {
            Button::new(("node-proc-stop", pid as usize))
                .custom(ButtonCustomVariant::new(cx).foreground(cx.theme().muted_foreground))
                .xsmall()
                .icon(IconName::CircleX)
                .tooltip("Stop\u{2026}")
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.confirm_stop = Some(pid);
                    cx.notify();
                }))
                .into_any_element()
        };

        h_flex()
            .gap_2()
            .px_3()
            .py_0p5()
            .text_xs()
            .text_color(cx.theme().foreground)
            .child(what)
            .child(mono(pid.to_string()).w_12().text_right())
            .child(mono(format!("{:.1}%", proc.cpu_percent)).w_12().text_right())
            .child(mono(format!("{:.0}MB", proc.memory_mb)).w_16().text_right())
            .child(action)
            .into_any_element()
    }

    fn render_spfx(&self, cx: &Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col();
        let theme = cx.theme();

        let version_str = self.spfx.version.as_deref().unwrap_or("");
        let badge_text = if version_str.is_empty() {
            "SPFx".to_string()
        } else {
            format!("SPFx {version_str}")
        };

        col = col.child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_3()
                .py_1()
                .child(
                    svg()
                        .path("icons/file_icons/ms-sharepoint.svg")
                        .size(px(16.0))
                        .text_color(theme.primary),
                )
                .child(Tag::primary().child(badge_text))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("SharePoint solution detected"),
                ),
        );

        if !self.sppkg_files.is_empty() {
            for f in &self.sppkg_files {
                let fname = std::path::Path::new(f)
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| f.clone());
                let fpath = f.clone();
                col = col.child(
                    div()
                        .id(format!("sppkg-{fname}"))
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .py_0p5()
                        .text_xs()
                        .cursor_pointer()
                        .hover(|d| d.bg(theme.list_hover))
                        .child(div().text_color(theme.primary).child("\u{1F4E6}"))
                        .child(
                            div()
                                .flex_1()
                                .truncate()
                                .font_family("Cascadia Mono")
                                .text_color(theme.foreground)
                                .child(fname.clone()),
                        )
                        .child(
                            div().id(format!("sppkg-reveal-{fname}")).child(
                                Button::new(format!("sppkg-reveal-btn-{fname}"))
                                    .custom(
                                        ButtonCustomVariant::new(cx)
                                            .foreground(theme.muted_foreground),
                                    )
                                    .xsmall()
                                    .icon(IconName::Folder)
                                    .tooltip("Reveal in file explorer")
                                    .on_click(cx.listener(move |_this, _e, _w, _cx| {
                                        let _ = reveal_in_explorer(fpath.clone());
                                    })),
                            ),
                        ),
                );
            }
        }

        let spfx_actions = [
            ("serve", "npm run serve"),
            ("bundle", "npm run bundle"),
            ("package-solution", "npm run package-solution"),
            ("clean", "npm run clean"),
        ];

        let mut row = div().flex().flex_row().flex_wrap().gap_1().px_3().py_1();

        // SPFx action buttons — same `npm run <name>` dispatch as the Quick
        // Actions row above.
        for (label, cmd) in spfx_actions {
            row = row.child(
                Button::new(format!("spfx-action-{label}"))
                    .secondary()
                    .xsmall()
                    .label(label)
                    .tooltip(format!("Run \"{cmd}\""))
                    .on_click(cx.listener(move |this, _e, window, cx| {
                        this.dispatch_script(cmd, window, cx);
                    })),
            );
        }

        col = col.child(row);
        col
    }

    fn render_output(&self, cx: &Context<Self>) -> impl IntoElement {
        let lines: Vec<_> = self.script_output.iter().rev().take(50).rev().collect();

        div()
            .flex()
            .flex_col()
            .mx_3()
            .my_2()
            .rounded_md()
            .bg(cx.theme().secondary)
            .border_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .id("output-scroll")
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .overflow_y_scroll()
                    .p_2()
                    .children(lines.iter().map(|line| {
                        div()
                            .font_family("Cascadia Mono")
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(line.to_string())
                    })),
            )
    }

    fn render_pkg_modal(
        &self,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> Option<impl IntoElement> {
        let view = self.view_pkg.as_ref()?;
        let viewport = window.viewport_size();
        let theme = cx.theme();

        Some(
            deferred(
                anchored()
                    .position_mode(AnchoredPositionMode::Window)
                    .position(point(px(0.0), px(0.0)))
                    .child(
                        div()
                            .id("node-pkg-modal-backdrop")
                            .w(viewport.width)
                            .h(viewport.height)
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(gpui::rgba(0x000000aa))
                            .on_click(cx.listener(|this, _e, _w, cx| {
                                this.close_view_pkg(cx);
                            }))
                            .child(
                                div()
                                    .id("node-pkg-modal")
                                    .occlude()
                                    .w(px(560.0))
                                    .max_h(relative(0.8))
                                    .flex()
                                    .flex_col()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(theme.border)
                                    .bg(theme.background)
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .px_3()
                                            .py_2()
                                            .border_b_1()
                                            .border_color(theme.border)
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(theme.foreground)
                                                    .child("package.json"),
                                            )
                                            .child(
                                                div().id("node-pkg-modal-close").child(
                                                    Button::new("node-pkg-modal-close-btn")
                                                        .custom(
                                                            ButtonCustomVariant::new(cx)
                                                                .foreground(theme.muted_foreground),
                                                        )
                                                        .xsmall()
                                                        .icon(IconName::Close)
                                                        .tooltip("Close")
                                                        .on_click(cx.listener(
                                                            |this, _e, _w, cx| {
                                                                this.close_view_pkg(cx);
                                                            },
                                                        )),
                                                ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .id("node-pkg-modal-body")
                                            .flex_1()
                                            .min_h_0()
                                            .overflow_y_scroll()
                                            .p_3()
                                            .child(render_package_json_body(view, &theme)),
                                    ),
                            ),
                    ),
            )
            .with_priority(100),
        )
    }
}

// ── SPFx detection ────────────────────────────────────────────────────

#[derive(Clone, Default)]
struct SpfxInfo {
    is_spfx: bool,
    version: Option<String>,
}

fn detect_spfx(project_path: &str) -> SpfxInfo {
    let yo_rc = std::path::Path::new(project_path).join(".yo-rc.json");
    if let Ok(raw) = std::fs::read_to_string(&yo_rc) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(generator) = json.get("@microsoft/generator-sharepoint") {
                let version = generator
                    .get("version")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                return SpfxInfo {
                    is_spfx: true,
                    version,
                };
            }
        }
    }

    let pks = std::path::Path::new(project_path)
        .join("config")
        .join("package-solution.json");
    if pks.exists() {
        return SpfxInfo {
            is_spfx: true,
            version: None,
        };
    }

    let pkg = std::path::Path::new(project_path).join("package.json");
    if let Ok(raw) = std::fs::read_to_string(&pkg) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&raw) {
            for key in ["dependencies", "devDependencies"] {
                if let Some(obj) = json.get(key).and_then(|v| v.as_object()) {
                    for dep_name in obj.keys() {
                        if dep_name.starts_with("@microsoft/sp-") {
                            let version = obj[dep_name].as_str().map(String::from);
                            return SpfxInfo {
                                is_spfx: true,
                                version,
                            };
                        }
                    }
                }
            }
        }
    }

    SpfxInfo {
        is_spfx: false,
        version: None,
    }
}

fn scan_sppkg(project_path: &str) -> Vec<String> {
    let sol_dir = std::path::Path::new(project_path)
        .join("sharepoint")
        .join("solution");
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&sol_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.to_lowercase().ends_with(".sppkg") {
                files.push(entry.path().to_string_lossy().into_owned());
            }
        }
    }
    files
}

/// The `npm update …` command for the Outdated section's bulk buttons:
/// every package whose update is no bigger than `kind_ceiling` (patch only
/// for `Patch`, otherwise patch and minor). `Ok(None)` when nothing qualifies.
///
/// An error (and nothing run) if any qualifying name isn't shell-plain.
fn update_command(
    outdated: &[NpmOutdatedPkg],
    kind_ceiling: UpdateKind,
) -> Result<Option<String>, String> {
    let mut targets = Vec::new();
    for package in outdated.iter().filter(|pkg| {
        let kind = classify_update(&pkg.current, &pkg.latest);
        match kind_ceiling {
            UpdateKind::Patch => kind == UpdateKind::Patch,
            _ => matches!(kind, UpdateKind::Patch | UpdateKind::Minor),
        }
    }) {
        check_package_name(&package.name)?;
        targets.push(package.name.as_str());
    }
    Ok((!targets.is_empty()).then(|| format!("npm update {}", targets.join(" "))))
}

/// `npm run <script>` for a row in the Scripts section. Script names come
/// straight from `package.json`, which can hold anything, so one that isn't
/// shell-plain is refused rather than run.
fn run_script_command(name: &str) -> Result<String, String> {
    check_script_name(name)?;
    Ok(format!("npm run {name}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outdated(name: &str, current: &str, latest: &str) -> NpmOutdatedPkg {
        NpmOutdatedPkg {
            name: name.to_string(),
            current: current.to_string(),
            wanted: latest.to_string(),
            latest: latest.to_string(),
            location: None,
        }
    }

    fn write(path: &std::path::Path, content: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn package_json_view_reads_fields_in_file_order() {
        let view = parse_package_json_view(
            r#"{
                "name": "web",
                "version": "1.2.3",
                "description": "The site",
                "scripts": { "dev": "vite", "build": "vite build", "broken": 5 },
                "dependencies": { "react": "^18.2.0" },
                "devDependencies": { "vite": "^5.0.0", "typescript": "^5.4.0" }
            }"#,
        )
        .unwrap();

        assert_eq!(view.name.as_deref(), Some("web"));
        assert_eq!(view.version.as_deref(), Some("1.2.3"));
        assert_eq!(view.description.as_deref(), Some("The site"));
        // Scripts keep the order they have in package.json; a non-string
        // value becomes an empty command rather than dropping the row.
        let scripts: Vec<(&str, &str)> = view
            .scripts
            .iter()
            .map(|(name, command)| (name.as_str(), command.as_str()))
            .collect();
        assert_eq!(scripts, vec![("dev", "vite"), ("build", "vite build"), ("broken", "")]);
        assert_eq!(view.dependencies.get("react").map(String::as_str), Some("^18.2.0"));
        assert_eq!(
            view.dev_dependencies.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["vite", "typescript"]
        );
    }

    #[test]
    fn package_json_view_tolerates_missing_sections_but_not_bad_json() {
        let view = parse_package_json_view("{}").unwrap();
        assert_eq!(view.name, None);
        assert!(view.scripts.is_empty());
        assert!(view.dependencies.is_empty());
        assert!(view.dev_dependencies.is_empty());

        assert!(parse_package_json_view("{ not json").is_none());
        assert!(parse_package_json_view("").is_none());
    }

    #[test]
    fn path_file_name_is_the_last_component() {
        assert_eq!(path_file_name("C:/work/apps/web"), "web");
        assert_eq!(path_file_name("/home/me/api"), "api");
        assert_eq!(path_file_name("single"), "single");
        // No final component: fall back to the input.
        assert_eq!(path_file_name("/"), "/");
    }

    #[test]
    fn node_manifest_files_are_recognized() {
        assert!(is_node_manifest_file("package.json"));
        assert!(is_node_manifest_file("package-lock.json"));
        assert!(is_node_manifest_file("npm-shrinkwrap.json"));
        assert!(is_node_manifest_file("yarn.lock"));
        assert!(is_node_manifest_file("pnpm-lock.yaml"));
        assert!(!is_node_manifest_file("tsconfig.json"));
        assert!(!is_node_manifest_file("package.json.bak"));
    }

    #[test]
    fn manifest_changes_match_only_the_selected_project() {
        let cwd = "C:/work";
        // The scan root's own manifest.
        assert!(manifest_change_is_in_project(cwd, "C:/work", "package.json"));
        assert!(manifest_change_is_in_project(cwd, "C:\\work", "yarn.lock"));
        // A nested project's own lockfile.
        assert!(manifest_change_is_in_project(
            cwd,
            "C:/work/apps/web",
            "apps/web/package-lock.json"
        ));
        // A different project's manifest must not match.
        assert!(!manifest_change_is_in_project(
            cwd,
            "C:/work/apps/web",
            "apps/api/package.json"
        ));
        // A file one level below the project root is not its own manifest.
        assert!(!manifest_change_is_in_project(cwd, "C:/work", "apps/web/package.json"));
    }

    #[test]
    fn update_command_respects_the_ceiling() {
        let list = [
            outdated("a", "1.0.0", "1.0.5"),
            outdated("b", "2.0.0", "2.3.0"),
            outdated("c", "3.0.0", "4.0.0"),
        ];
        assert_eq!(update_command(&list, UpdateKind::Patch).unwrap().as_deref(), Some("npm update a"));
        assert_eq!(update_command(&list, UpdateKind::Minor).unwrap().as_deref(), Some("npm update a b"));
        // A major ceiling is still capped at minor: bulk updates never
        // cross a major version.
        assert_eq!(update_command(&list, UpdateKind::Major).unwrap().as_deref(), Some("npm update a b"));
    }

    #[test]
    fn update_command_with_nothing_to_update() {
        assert_eq!(update_command(&[outdated("c", "3.0.0", "4.0.0")], UpdateKind::Minor), Ok(None));
        assert_eq!(update_command(&[], UpdateKind::Patch), Ok(None));
    }

    #[test]
    fn update_command_runs_nothing_if_any_name_is_unsafe() {
        let list = [outdated("a", "1.0.0", "1.0.5"), outdated("b && calc", "2.0.0", "2.0.1")];
        assert!(update_command(&list, UpdateKind::Minor).is_err());
    }

    #[test]
    fn script_commands() {
        assert_eq!(run_script_command("dev").as_deref(), Ok("npm run dev"));
        assert_eq!(run_script_command("build:prod").as_deref(), Ok("npm run build:prod"));
        assert!(run_script_command("dev && calc").is_err());
        assert!(run_script_command("say hi").is_err());
        assert!(run_script_command("").is_err());
    }

    #[test]
    fn spfx_detected_from_yeoman_config() {
        let dir = tempfile::tempdir().unwrap();
        write(
            &dir.path().join(".yo-rc.json"),
            r#"{ "@microsoft/generator-sharepoint": { "version": "1.18.2" } }"#,
        );
        let info = detect_spfx(&dir.path().to_string_lossy());
        assert!(info.is_spfx);
        assert_eq!(info.version.as_deref(), Some("1.18.2"));
    }

    #[test]
    fn spfx_detected_from_package_solution_or_dependencies() {
        let dir = tempfile::tempdir().unwrap();
        write(&dir.path().join("config/package-solution.json"), "{}");
        let info = detect_spfx(&dir.path().to_string_lossy());
        assert!(info.is_spfx);
        assert_eq!(info.version, None);

        let dir = tempfile::tempdir().unwrap();
        write(
            &dir.path().join("package.json"),
            r#"{ "devDependencies": { "@microsoft/sp-build-web": "1.18.2", "gulp": "4.0.2" } }"#,
        );
        let info = detect_spfx(&dir.path().to_string_lossy());
        assert!(info.is_spfx);
        assert_eq!(info.version.as_deref(), Some("1.18.2"));
    }

    #[test]
    fn ordinary_projects_are_not_spfx() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!detect_spfx(&dir.path().to_string_lossy()).is_spfx);

        write(
            &dir.path().join("package.json"),
            r#"{ "dependencies": { "react": "18.2.0", "@microsoft/signalr": "8.0.0" } }"#,
        );
        write(&dir.path().join(".yo-rc.json"), r#"{ "generator-other": {} }"#);
        assert!(!detect_spfx(&dir.path().to_string_lossy()).is_spfx);
    }

    #[test]
    fn sppkg_files_are_listed_from_the_solution_folder() {
        let dir = tempfile::tempdir().unwrap();
        assert!(scan_sppkg(&dir.path().to_string_lossy()).is_empty());

        write(&dir.path().join("sharepoint/solution/webpart.sppkg"), "");
        write(&dir.path().join("sharepoint/solution/UPPER.SPPKG"), "");
        write(&dir.path().join("sharepoint/solution/debug/notes.txt"), "");
        write(&dir.path().join("sharepoint/solution/readme.md"), "");

        let mut names: Vec<String> = scan_sppkg(&dir.path().to_string_lossy())
            .iter()
            .map(|path| path_file_name(path).to_lowercase())
            .collect();
        names.sort();
        assert_eq!(names, vec!["upper.sppkg", "webpart.sppkg"]);
    }
}
