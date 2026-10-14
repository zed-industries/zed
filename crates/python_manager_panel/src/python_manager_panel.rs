//! Python (PyPI) package manager — virtual workspace tab opened from the
//! Python panel's "Python Package Manager" quick action.
//!
//! Mirrors `npm_manager_panel`/`nuget_manager_panel` as a workspace [`Item`]
//! (a tab in the active pane, not a docked panel), rendered through the
//! `gpui_component::setting::Settings` chrome (sidebar pages + grouped
//! boxes), with the pure package/PyPI-parsing logic living in
//! `python_backend`.
//!
//! Unlike npm/NuGet, PyPI has no full-text search JSON API (it was removed
//! in 2018) — only exact-name metadata lookups via
//! `https://pypi.org/pypi/<name>/json`. The "Search" page therefore does an
//! exact-name PyPI lookup rather than a fuzzy search; entering "requests"
//! looks up the `requests` package directly. This mirrors the Forge
//! original's `python_manager_panel.rs` exactly — it's a real PyPI API
//! limitation, not a simplification made during this port.
//!
//! Install/uninstall/update commands run through the Script Runner dock
//! panel via `kick_run`, the same §4.2-style wiring `npm_manager_panel`
//! already established (`workspace.open_panel::<ScriptRunnerPanel>` +
//! `ScriptRunnerPanel::run_external` + `cx.observe` for completion) —
//! reused verbatim rather than re-derived, since this panel is a workspace
//! `Item` like npm/NuGet's managers, not a docked `Panel`, so the
//! `reveal_panel` double-lease hazard `node_panel`/`python_panel` hit
//! doesn't apply here.
//!
//! Vulnerability scanning is a disclosed deviation from a bulk-audit shape:
//! PyPI has no `npm audit`-style single endpoint, so `scan_vulnerabilities`
//! fires one `/pypi/<name>/<version>/json` request per installed package
//! (capped at `VULN_SCAN_CONCURRENCY` concurrent) and is scan-on-demand via
//! a button, not automatic on every reload — exactly matching the Forge
//! original's own documented tradeoff.

use std::collections::HashMap;
use std::path::Path;

use futures::io::AsyncReadExt as _;
use futures::stream::{self, StreamExt as _};
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, FocusHandle, Focusable, KeyBinding,
    InteractiveElement as _, IntoElement, ParentElement as _, PromptLevel, Render,
    StatefulInteractiveElement as _, Styled as _, Subscription, Task, WeakEntity, Window, actions,
    div, px,
};
use gpui_component::{
    ActiveTheme as _,
    h_flex,
    input::{InputEvent, InputState},
    resizable::{h_resizable, resizable_panel},
    setting::Settings,
    v_flex,
};
use http_client::HttpClient;
use python_backend::{
    DetectedPythonProject, EnvMarker, PyPiPackageInfo, PyPiVulnerability, PythonOutdatedPkg,
    PythonPackage, list_outdated, list_packages, parse_pypi_json, parse_pypi_vulnerabilities,
    query_python, scan_python_project, scan_python_projects,
};
use script_runner_panel::ScriptRunnerPanel;
use script_runner_panel::command::{check_package_name, check_version, shell_program};
use workspace::{Item, ItemId, SerializableItem, Workspace, WorkspaceId};

mod details;
mod pages;

actions!(
    python_manager,
    [
        OpenPythonManager,
        /// Moves the selection down one row in whichever package list page
        /// (Installed/Updates/Vulnerabilities) currently has focus.
        SelectNextPackage,
        /// Moves the selection up one row, same scope as `SelectNextPackage`.
        SelectPrevPackage,
        /// Opens (Installed/Updates) or toggles (Vulnerabilities) the
        /// selected row — the keyboard equivalent of clicking it.
        OpenSelectedPackage,
        /// Runs the selected row's primary action for the page it's on:
        /// Uninstall (Installed), Update (Updates), or Fix (Vulnerabilities,
        /// only when the advisory offers one).
        ActSelectedPackage
    ]
);

/// Max concurrent PyPI requests during a vulnerability scan — one HTTP
/// round-trip per installed package (there's no bulk endpoint, unlike `npm
/// audit`), so this caps how many are in flight at once rather than firing
/// them all simultaneously.
const VULN_SCAN_CONCURRENCY: usize = 8;

/// Opens (or activates) the Python Package Manager tab for the given
/// workspace root, in the given workspace — the seam the Python panel's
/// quick action calls.
pub fn open(
    project_root: String,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Entity<PythonManagerPanel> {
    let existing = workspace
        .active_pane()
        .read(cx)
        .items()
        .find_map(|item| item.downcast::<PythonManagerPanel>());

    if let Some(existing) = existing {
        workspace.activate_item(&existing, true, true, window, cx);
        existing
    } else {
        let panel = cx.new(|cx| PythonManagerPanel::new(project_root, workspace.weak_handle(), window, cx));
        workspace.add_item_to_active_pane(Box::new(panel.clone()), None, true, window, cx);
        panel
    }
}

pub fn init(cx: &mut App) {
    workspace::register_serializable_item::<PythonManagerPanel>(cx);
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &OpenPythonManager, window, cx| {
            let root = workspace_root(workspace, cx);
            if !root.is_empty() {
                let panel = open(root, workspace, window, cx);
                panel.update(cx, |panel, cx| {
                    panel.schedule_load(cx);
                });
            }
        });
    })
    .detach();

    // The three package-list pages (Installed/Updates/Vulnerabilities) had
    // no keyboard path at all before this — a row could only be opened or
    // acted on with the mouse. Scoped to `PythonPackageList` (each list
    // container sets that key context), matching how
    // `gpui_component::table::data_table` scopes its own row-navigation
    // bindings to `DataTable`, and identically to
    // `npm_manager_panel`/`nuget_manager_panel`'s own `PACKAGE_LIST_CONTEXT`.
    cx.bind_keys([
        KeyBinding::new("down", SelectNextPackage, Some("PythonPackageList")),
        KeyBinding::new("up", SelectPrevPackage, Some("PythonPackageList")),
        KeyBinding::new("enter", OpenSelectedPackage, Some("PythonPackageList")),
        KeyBinding::new("space", ActSelectedPackage, Some("PythonPackageList")),
    ]);
}

/// The workspace's first worktree's absolute path — same resolution
/// `NodePanel`/`NpmManagerPanel`/`ScriptRunnerPanel` use, falling back to
/// the process cwd when no project worktree is open yet.
fn workspace_root(workspace: &Workspace, cx: &Context<Workspace>) -> String {
    workspace
        .project()
        .read(cx)
        .worktrees(cx)
        .next()
        .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().into_owned())
        .unwrap_or_else(|| {
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_default()
        })
}

// ── Panel ──────────────────────────────────────────────────────────────

pub struct PythonManagerPanel {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    /// Active observer on the Script Runner dock panel, replaced on every
    /// run it streams for — fires when `ScriptRunnerPanel` notifies, which
    /// it does on each output line and when a run finishes.
    run_subscription: Option<Subscription>,
    _search_sub: Subscription,

    root: String,
    /// Every directory under the workspace that declares its own
    /// dependencies.
    projects: Vec<DetectedPythonProject>,
    /// The currently active project's directory — pip/venv commands are
    /// scoped to this, not the whole workspace.
    selected_project: Option<String>,
    /// Set when the selected project has an env-declaration file (see
    /// `EnvMarker`) but no venv yet — drives the "Create Environment"
    /// action on the General page.
    can_create_venv: Option<EnvMarker>,

    /// Interpreter used for pip commands — the selected project's own venv
    /// when one is found, else the first working "python"/"python3" on
    /// PATH.
    python_exe: Option<String>,
    /// True when `python_exe` is a project-local venv interpreter.
    using_venv: bool,

    installed: Vec<PythonPackage>,
    outdated: Vec<PythonOutdatedPkg>,
    loading: bool,
    load_error: Option<String>,
    running_action: Option<String>,
    error: Option<String>,

    /// Known vulnerabilities per installed package name, populated by
    /// `scan_vulnerabilities`.
    vulns: HashMap<String, Vec<PyPiVulnerability>>,
    vuln_scanning: bool,
    /// `None` until the first scan completes (or fails) — distinguishes
    /// "never scanned" from "scanned, zero found" in the UI.
    vuln_scanned: bool,
    vuln_scan_error: Option<String>,
    /// Ids ("{package}-{advisory id}") of vulnerabilities whose markdown
    /// body is expanded — collapsed by default.
    expanded_vulns: std::collections::HashSet<String>,

    search_input: Entity<InputState>,
    /// Result of the last exact-name PyPI lookup (there is no fuzzy
    /// search).
    search_hit: Option<PyPiPackageInfo>,
    search_loading: bool,
    search_error: Option<String>,

    /// Name of the package whose details sidebar is open, if any.
    selected: Option<String>,
    details: Option<PyPiPackageInfo>,
    details_loading: bool,
    show_readme: bool,

    // ── Keyboard row navigation ──────────────────────────────────────
    //
    // No separate page-view entities exist here (unlike
    // `npm_manager_panel`/`nuget_manager_panel`'s `PageViews`) — `pages.rs`
    // rebuilds every `SettingPage` fresh from this struct on each render, so
    // the cursor + its list's `FocusHandle` live directly on the panel,
    // one pair per list that needs it. `FocusHandle`s must be created once
    // in `new` (not per render) or focus tracking breaks.
    /// Row `up`/`down`/`enter`/`space` act on, within `installed`.
    installed_cursor: Option<usize>,
    installed_list_focus: FocusHandle,
    /// Row `up`/`down`/`enter`/`space` act on, within `outdated`.
    outdated_cursor: Option<usize>,
    outdated_list_focus: FocusHandle,
    /// Row `up`/`down`/`enter`/`space` act on, within the flattened,
    /// name-sorted `(package, vulnerability)` list `pages::flatten_vulns`
    /// also builds for rendering — kept as an index into that same order so
    /// the two never drift apart. Enter toggles the row's own
    /// expand/collapse (mirrors a click on the header, not "open details" —
    /// there is no separate details view from this list); Space runs Fix
    /// when the advisory lists one, a no-op otherwise.
    vuln_cursor: Option<usize>,
    vuln_list_focus: FocusHandle,
}

impl PythonManagerPanel {
    pub fn new(
        root: String,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search_input = cx
            .new(|cx| InputState::new(window, cx).placeholder("Look up a package on PyPI (exact name)…"));
        let _search_sub = cx.subscribe(&search_input, |this, _, event, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                this.search_pypi(cx);
            }
        });

        let panel = PythonManagerPanel {
            focus_handle: cx.focus_handle(),
            workspace,
            run_subscription: None,
            _search_sub,
            root,
            projects: Vec::new(),
            selected_project: None,
            can_create_venv: None,
            python_exe: None,
            using_venv: false,
            installed: Vec::new(),
            outdated: Vec::new(),
            loading: true,
            load_error: None,
            running_action: None,
            error: None,
            vulns: HashMap::new(),
            vuln_scanning: false,
            vuln_scanned: false,
            vuln_scan_error: None,
            expanded_vulns: std::collections::HashSet::new(),
            search_input,
            search_hit: None,
            search_loading: false,
            search_error: None,
            selected: None,
            details: None,
            details_loading: false,
            show_readme: false,
            installed_cursor: None,
            installed_list_focus: cx.focus_handle(),
            outdated_cursor: None,
            outdated_list_focus: cx.focus_handle(),
            vuln_cursor: None,
            vuln_list_focus: cx.focus_handle(),
        };

        let mut panel = panel;
        panel.schedule_load(cx);
        panel
    }

    // ── Data loading ───────────────────────────────────────────────────

    fn schedule_load(&mut self, cx: &mut Context<Self>) {
        let root = self.root.clone();
        let prior_selection = self.selected_project.clone();
        self.loading = true;
        self.load_error = None;
        cx.notify();

        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let (projects, selected, exe, using_venv, can_create_venv, installed, outdated) = cx
                .background_spawn(async move {
                    let projects = if root.is_empty() {
                        Vec::new()
                    } else {
                        scan_python_projects(&root, 0)
                    };

                    let selected = prior_selection
                        .filter(|p| projects.iter().any(|proj| &proj.path == p))
                        .or_else(|| projects.first().map(|p| p.path.clone()));

                    let scan_root = selected.clone().unwrap_or_else(|| root.clone());
                    let env_marker = selected
                        .as_ref()
                        .and_then(|p| projects.iter().find(|proj| &proj.path == p))
                        .map(|proj| proj.marker);

                    let venv_exe = if !scan_root.is_empty() {
                        scan_python_project(&scan_root)
                            .ok()
                            .and_then(|scan| scan.venvs.into_iter().next())
                    } else {
                        None
                    };

                    let (working_exe, using_venv) = if let Some(venv) = venv_exe {
                        (Some(venv), true)
                    } else {
                        // Without a venv: the built-in Python when one is
                        // active, which is also what a new venv is made
                        // from, then whatever the machine has.
                        let mut found: Option<String> =
                            python_versions::active(&python_versions::store_dir())
                                .map(|version| version.python.to_string_lossy().into_owned());
                        for candidate in ["python", "python3"] {
                            if found.is_some() {
                                break;
                            }
                            if query_python(candidate).is_ok() {
                                found = Some(candidate.to_string());
                            }
                        }
                        (found, false)
                    };

                    let can_create_venv = if using_venv || working_exe.is_none() {
                        None
                    } else {
                        env_marker
                    };

                    let (installed, outdated) = match &working_exe {
                        Some(exe) => (
                            list_packages(exe).unwrap_or_default(),
                            list_outdated(exe).unwrap_or_default(),
                        ),
                        None => (Vec::new(), Vec::new()),
                    };

                    (
                        projects,
                        selected,
                        working_exe,
                        using_venv,
                        can_create_venv,
                        installed,
                        outdated,
                    )
                })
                .await;

            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |s, cx| {
                        s.loading = false;
                        s.load_error = if exe.is_none() {
                            Some("No Python interpreter found for this workspace".to_string())
                        } else {
                            None
                        };
                        s.projects = projects;
                        s.selected_project = selected;
                        s.python_exe = exe;
                        s.using_venv = using_venv;
                        s.can_create_venv = can_create_venv;
                        s.installed = installed;
                        s.outdated = outdated;
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    /// Selects a different detected project and rescopes pip/venv lookups
    /// to it — mirrors `NodePanel::select_project`/`NpmManagerPanel::select_project`.
    fn select_project(&mut self, path: String, cx: &mut Context<Self>) {
        if self.selected_project.as_deref() == Some(path.as_str()) {
            return;
        }
        self.selected_project = Some(path);
        self.schedule_load(cx);
    }

    /// Exact-name PyPI lookup — there is no full-text search JSON API on
    /// PyPI, so the "Search" box looks up the entered name directly.
    fn search_pypi(&mut self, cx: &mut Context<Self>) {
        let query = self.search_input.read(cx).value().trim().to_string();
        if query.is_empty() {
            self.search_hit = None;
            self.search_error = None;
            cx.notify();
            return;
        }
        self.search_loading = true;
        self.search_error = None;
        self.search_hit = None;
        cx.notify();

        let http_client = cx.http_client();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = fetch_pypi_json(&http_client, &query).await;

            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |s, cx| {
                        s.search_loading = false;
                        match result {
                            Ok(info) => s.search_hit = Some(info),
                            Err(e) => s.search_error = Some(e),
                        }
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    /// Fetches PyPI metadata for `name` and opens it in the details
    /// sidebar. Used by both the search-hit row and installed-package
    /// rows.
    fn fetch_details(&mut self, name: String, cx: &mut Context<Self>) {
        self.selected = Some(name.clone());
        self.details_loading = true;
        self.details = None;
        self.show_readme = false;
        cx.notify();

        let http_client = cx.http_client();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = fetch_pypi_json(&http_client, &name).await;

            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.details_loading = false;
                        if let Ok(info) = result {
                            panel.details = Some(info);
                        } else {
                            panel.details = None;
                        }
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    /// Opens the details sidebar directly from data we already fetched (the
    /// search hit), with no extra network round-trip.
    fn open_search_hit_details(&mut self, cx: &mut Context<Self>) {
        if let Some(hit) = self.search_hit.clone() {
            self.selected = Some(hit.name.clone());
            self.details = Some(hit);
            self.details_loading = false;
            self.show_readme = false;
            cx.notify();
        }
    }

    /// Checks every installed package's *exact installed version* against
    /// PyPI's OSV-sourced advisory data. There is no bulk endpoint like
    /// `npm audit`, so this fires one request per package (capped at
    /// `VULN_SCAN_CONCURRENCY` in flight) — explicit/on-demand rather than
    /// run automatically on every `schedule_load`, to avoid hammering PyPI
    /// on every pip install/uninstall.
    fn scan_vulnerabilities(&mut self, cx: &mut Context<Self>) {
        if self.vuln_scanning || self.installed.is_empty() {
            return;
        }
        let pkgs: Vec<(String, String)> = self
            .installed
            .iter()
            .map(|p| (p.name.clone(), p.version.clone()))
            .collect();
        self.vuln_scanning = true;
        self.vuln_scan_error = None;
        cx.notify();

        let http_client = cx.http_client();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = fetch_all_vulnerabilities(&http_client, pkgs).await;

            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |s, cx| {
                        s.vuln_scanning = false;
                        s.vuln_scanned = true;
                        match result {
                            Ok(map) => {
                                s.vulns = map;
                                s.vuln_scan_error = None;
                            }
                            Err(e) => s.vuln_scan_error = Some(e),
                        }
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    fn toggle_vuln(&mut self, id: String, cx: &mut Context<Self>) {
        if !self.expanded_vulns.remove(&id) {
            self.expanded_vulns.insert(id);
        }
        cx.notify();
    }

    // ── Command dispatch (→ Script Runner) ─────────────────────────────

    fn pip_bin(&self) -> String {
        self.python_exe.clone().unwrap_or_else(|| "python".to_string())
    }

    /// Creates a venv (or, for a `Pipfile` project, a pipenv-managed
    /// environment) in the selected project's folder and installs its
    /// declared dependencies. Only enabled (see `can_create_venv`) when the
    /// project has an env-declaration file and no venv yet. Runs through
    /// the Script Runner; when it finishes, `reload_visible` re-runs and
    /// finds the new venv, switching the tab from "global fallback" to
    /// "scoped" automatically.
    fn create_venv(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(marker) = self.can_create_venv else {
            return;
        };
        if self.selected_project.is_none() {
            return;
        }
        let command = self.pip_program().map(|exe| marker.create_command(&exe));
        self.run_or_reject("create environment", command, window, cx);
    }

    /// The interpreter as it can be written into a command run in the
    /// selected project's directory — relative when it's the project's own
    /// venv, so a project under a path with spaces still works.
    fn pip_program(&self) -> Result<String, String> {
        let exe = self.pip_bin();
        // The built-in Python is first on the `PATH` of every command the
        // Script Runner runs, so it is named plainly rather than by a path
        // under the user's profile, which may hold spaces.
        if python_versions::is_built_in(&python_versions::store_dir(), Path::new(&exe)) {
            return Ok("python".to_string());
        }
        let cwd = self.selected_project.clone().unwrap_or_else(|| self.root.clone());
        shell_program(&exe, &cwd)
    }

    /// Runs `command` if it was built successfully; otherwise shows the
    /// refusal (see `script_runner_panel::command`).
    fn run_or_reject(
        &mut self,
        label: &str,
        command: Result<String, String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match command {
            Ok(command) => self.kick_run(label, command, window, cx),
            Err(error) => {
                self.error = Some(error);
                cx.notify();
            }
        }
    }

    fn install_pkg(&mut self, name: &str, version: Option<&str>, window: &mut Window, cx: &mut Context<Self>) {
        let command = self
            .pip_program()
            .and_then(|exe| pip_install_command(&exe, name, version));
        self.run_or_reject(&format!("install {name}"), command, window, cx);
    }

    /// Confirms before uninstalling — irreversible from here (no undo, and
    /// pip doesn't remember what version was removed), and previously a
    /// single un-gated click. Uses GPUI's own `window.prompt`, matching
    /// Zed's own `project_panel::remove`'s delete-file confirmation rather
    /// than a `gpui_component` modal/dialog — see `database_panel`'s
    /// `delete_connection` for the same reasoning, applied identically here.
    fn uninstall_pkg(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) {
        let name = name.to_string();
        let answer = window.prompt(
            PromptLevel::Warning,
            &format!("Uninstall {name}?"),
            Some("This removes the package from the active interpreter and cannot be undone."),
            &["Uninstall", "Cancel"],
            cx,
        );
        cx.spawn_in(window, async move |this, cx| {
            if answer.await != Ok(0) {
                return;
            }
            this.update_in(cx, |this, window, cx| {
                let command = this
                    .pip_program()
                    .and_then(|exe| pip_uninstall_command(&exe, &name));
                this.run_or_reject(&format!("uninstall {name}"), command, window, cx);
            })
            .ok();
        })
        .detach();
    }

    fn update_all_outdated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.outdated.is_empty() {
            return;
        }
        let command = self
            .pip_program()
            .and_then(|exe| pip_update_all_command(&exe, &self.outdated));
        self.run_or_reject("update all", command, window, cx);
    }

    fn latest_for(&self, name: &str) -> Option<&str> {
        self.outdated
            .iter()
            .find(|o| o.name == name)
            .map(|o| o.latest_version.as_str())
    }

    /// The live Script Runner dock panel for this workspace, if it's been
    /// loaded yet (`initialize_panels` creates it shortly after startup).
    fn runner(&self, cx: &App) -> Option<Entity<ScriptRunnerPanel>> {
        let workspace = self.workspace.upgrade()?;
        workspace.read(cx).panel::<ScriptRunnerPanel>(cx)
    }

    /// Runs a pip command for the selected project, streaming its output
    /// through the live Script Runner dock panel — opening that dock so the
    /// output is visible — and reloading the package lists when it
    /// finishes. Falls back to a silent background run with a spinner when
    /// the Script Runner dock panel isn't loaded yet. Mirrors
    /// `NpmManagerPanel::kick_run` exactly (this panel is a workspace
    /// `Item`, not a docked `Panel`, so the `reveal_panel` double-lease
    /// hazard doesn't apply here).
    fn kick_run(&mut self, label: &str, command: String, window: &mut Window, cx: &mut Context<Self>) {
        if self.running_action.is_some() {
            return;
        }
        let cwd = self.selected_project.clone().unwrap_or_else(|| self.root.clone());
        self.running_action = Some(label.to_string());
        self.error = None;
        cx.notify();

        let Some(runner) = self.runner(cx) else {
            return self.background_run(command, cwd, cx);
        };

        // The runner can only stream one command at a time — never pile a
        // run onto an already-streaming one.
        if runner.read(cx).is_running() {
            self.running_action = None;
            self.error = Some("Script Runner is busy — wait for it to finish first.".into());
            cx.notify();
            return;
        }

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.open_panel::<ScriptRunnerPanel>(window, cx);
            });
        }

        runner.update(cx, |runner, cx| runner.run_external(command, cwd, cx));

        // Reload the lists when the run finishes: the runner notifies on
        // every output line AND on completion, so this fires exactly then.
        self.run_subscription = Some(cx.observe(&runner, |this, runner, cx| {
            if !runner.read(cx).is_running() {
                this.running_action = None;
                this.schedule_load(cx);
                cx.notify();
            }
        }));

        // A blindingly-fast run can finish between `run_external` above and
        // subscribing, which would miss the completion notify — re-check
        // once.
        if !runner.read(cx).is_running() {
            self.running_action = None;
            self.schedule_load(cx);
            cx.notify();
        }
    }

    /// Fallback when no Script Runner dock panel exists yet: run in a
    /// background task, showing only a spinner and an error on failure.
    fn background_run(&mut self, command: String, cwd: String, cx: &mut Context<Self>) {
        let this = cx.weak_entity();
        cx.spawn(async move |_: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move { run_shell_sync(&command, &cwd) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.running_action = None;
                        match result {
                            Ok(_) => panel.schedule_load(cx),
                            Err(e) => {
                                panel.error = Some(e);
                                cx.notify();
                            }
                        }
                    });
                }
            });
        })
        .detach();
    }

    /// Inline, always-visible project picker rendered above the tabbed
    /// `Settings` body — mirrors the Forge original's
    /// `render_project_picker`, since which project's venv/pip commands are
    /// active changes what every other page's actions actually do.
    fn render_project_picker(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        if self.projects.len() <= 1 {
            return div().into_any_element();
        }

        let mut row = h_flex()
            .w_full()
            .flex_wrap()
            .gap_1()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.border);
        for proj in &self.projects {
            let is_selected = self.selected_project.as_deref() == Some(proj.path.as_str());
            let path = proj.path.clone();
            let chip = div()
                .id(format!("py-proj-{}", proj.path))
                .px_2()
                .py_1()
                .rounded_md()
                .text_xs()
                .cursor_pointer()
                .bg(if is_selected {
                    theme.primary.opacity(0.15)
                } else {
                    theme.secondary
                })
                .text_color(if is_selected { theme.primary } else { theme.foreground })
                .hover(|d| d.bg(theme.list_hover))
                .on_click(cx.listener(move |this, _e, _w, cx| {
                    this.select_project(path.clone(), cx);
                }))
                .child(proj.name.clone());
            row = row.child(chip);
        }
        row.into_any_element()
    }
}

/// Fetches `https://pypi.org/pypi/<name>/json` and parses it via
/// `cx.http_client()` — the same established pattern
/// `npm_manager_panel::search::fetch_json` uses for its registry calls.
async fn fetch_pypi_json(
    http_client: &std::sync::Arc<dyn HttpClient>,
    name: &str,
) -> Result<PyPiPackageInfo, String> {
    let url = format!("https://pypi.org/pypi/{}/json", urlencoding::encode(name.trim()));
    let json = fetch_json(http_client, &url).await?;
    parse_pypi_json(&json)
}

/// Fetches `https://pypi.org/pypi/<name>/<version>/json` for every
/// `(name, version)` pair — the *per-version* endpoint, whose top-level
/// `vulnerabilities` array is what actually carries OSV advisory data for
/// that exact release. Runs up to `VULN_SCAN_CONCURRENCY` requests
/// concurrently; a failed/missing lookup for one package is treated as "no
/// known vulnerabilities" for it rather than failing the whole scan.
///
/// `pub` (not `pub(crate)`): `python_panel`'s own Vulnerabilities section
/// reuses this exact function rather than duplicating the HTTP/concurrency
/// logic — `python_panel` already depends on this crate for the "Package
/// Manager" quick-action button, so this doesn't add a new dependency edge.
pub async fn fetch_all_vulnerabilities(
    http_client: &std::sync::Arc<dyn HttpClient>,
    packages: Vec<(String, String)>,
) -> Result<HashMap<String, Vec<PyPiVulnerability>>, String> {
    let results: Vec<(String, Vec<PyPiVulnerability>)> = stream::iter(packages)
        .map(|(name, version)| {
            let http_client = http_client.clone();
            async move {
                let url = format!(
                    "https://pypi.org/pypi/{}/{}/json",
                    urlencoding::encode(name.trim()),
                    urlencoding::encode(version.trim())
                );
                let vulns = match fetch_json(&http_client, &url).await {
                    Ok(body) => parse_pypi_vulnerabilities(&body),
                    Err(_) => Vec::new(),
                };
                (name, vulns)
            }
        })
        .buffer_unordered(VULN_SCAN_CONCURRENCY)
        .collect()
        .await;

    Ok(results.into_iter().filter(|(_, v)| !v.is_empty()).collect())
}

/// Synchronous fallback shell runner used only by `background_run` (no
/// Script Runner dock panel loaded yet). Mirrors the shape of
/// `npm_backend::run_npm_cli`'s own fallback, but takes a full shell command
/// string (this panel's commands can be `&&`-chained, e.g. `create_venv`)
/// rather than a program + args list, so it runs through the system shell:
/// `cmd /C` on Windows, `sh -c` elsewhere (there is no `cmd` there).
fn run_shell_sync(command: &str, cwd: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    let mut c = {
        use std::os::windows::process::CommandExt as _;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let mut c = std::process::Command::new("cmd");
        c.args(["/C", command]).creation_flags(CREATE_NO_WINDOW);
        c
    };
    #[cfg(not(target_os = "windows"))]
    let mut c = {
        let mut c = std::process::Command::new("sh");
        c.args(["-c", command]);
        c
    };
    c.current_dir(cwd);
    // The IDE's own runtimes first, as the Script Runner gives its commands:
    // a command naming the built-in Python as plain `python` (see
    // `pip_program`) needs it on the `PATH` to find it.
    if let Some(path) = ide_env::path_env(Some(Path::new(cwd))) {
        c.env("PATH", path);
    }
    let out = c.output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr);
        Err(if stderr.trim().is_empty() {
            format!("command exited with {}", out.status)
        } else {
            stderr.into_owned()
        })
    }
}

async fn fetch_json(
    http_client: &std::sync::Arc<dyn HttpClient>,
    url: &str,
) -> Result<serde_json::Value, String> {
    let mut response = http_client
        .get(url, Default::default(), true)
        .await
        .map_err(|e| format!("request failed: {e}"))?;
    if response.status() == http_client::http::StatusCode::NOT_FOUND {
        return Err("not found on PyPI".to_string());
    }
    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }
    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&body).map_err(|e| format!("invalid JSON: {e}"))
}

impl EventEmitter<()> for PythonManagerPanel {}

impl Focusable for PythonManagerPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for PythonManagerPanel {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> gpui::SharedString {
        "Python Package Manager".into()
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<gpui::SharedString> {
        Some(format!("Python Package Manager — {}", self.root).into())
    }
}

impl SerializableItem for PythonManagerPanel {
    fn serialized_item_kind() -> &'static str {
        "python_manager_panel"
    }

    /// Persisting the tab just records its presence in the pane; the scan
    /// root is always the project's first worktree, so nothing extra is
    /// written and nothing needs cleaning up.
    fn cleanup(
        _workspace_id: WorkspaceId,
        _alive_items: Vec<ItemId>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<anyhow::Result<()>> {
        Task::ready(Ok(()))
    }

    fn deserialize(
        project: gpui::Entity<project::Project>,
        workspace: WeakEntity<Workspace>,
        _workspace_id: WorkspaceId,
        _item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<gpui::Entity<Self>>> {
        let root = project
            .read(cx)
            .worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().into_owned())
            .unwrap_or_else(|| {
                std::env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default()
            });

        window.spawn(cx, async move |cx| {
            cx.update(|window, cx| Ok(cx.new(|cx| PythonManagerPanel::new(root, workspace, window, cx))))?
        })
    }

    /// Nothing is persisted beyond the item's presence in the pane.
    fn serialize(
        &mut self,
        _workspace: &mut Workspace,
        _item_id: ItemId,
        _closing: bool,
        _cx: &mut gpui::Context<Self>,
    ) -> Option<Task<anyhow::Result<()>>> {
        None
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

impl Render for PythonManagerPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let view = cx.entity();

        if let Some(err) = &self.load_error {
            if self.installed.is_empty() && self.python_exe.is_none() {
                return v_flex()
                    .id("python-manager-panel-error")
                    .track_focus(&self.focus_handle)
                    .size_full()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .p_4()
                    .bg(theme.background)
                    .text_color(theme.muted_foreground)
                    .child(err.clone())
                    .into_any_element();
            }
        }

        let settings = Settings::new("python-package-manager").pages(pages::build_all(self, &view, cx));

        let body = if self.selected.is_some() {
            h_resizable("python-details-split")
                .child(resizable_panel().child(v_flex().flex_1().min_w_0().h_full().child(settings)))
                .child(
                    resizable_panel()
                        .size(px(400.0))
                        .size_range(px(280.0)..px(900.0))
                        .flex_none()
                        .child(details::details_pane(self, &view, cx)),
                )
                .into_any_element()
        } else {
            settings.into_any_element()
        };

        v_flex()
            .id("python-manager-panel")
            .track_focus(&self.focus_handle)
            .size_full()
            .pl_2()
            .bg(theme.background)
            .child(self.render_project_picker(cx))
            .child(div().flex_1().min_h_0().w_full().child(body))
            .into_any_element()
    }
}

/// `<exe> -m pip install <name>`, pinned with `==<version>` when one is
/// given. Going through `-m pip` rather than a bare `pip` keeps the install
/// tied to the interpreter the panel is showing. `exe` must already be
/// shell-ready (see `PythonManagerPanel::pip_program`); the name and version
/// are checked here.
fn pip_install_command(exe: &str, name: &str, version: Option<&str>) -> Result<String, String> {
    check_package_name(name)?;
    Ok(match version {
        Some(version) => {
            check_version(version)?;
            format!("{exe} -m pip install {name}=={version}")
        }
        None => format!("{exe} -m pip install {name}"),
    })
}

/// `<exe> -m pip uninstall -y <name>` — `-y` because the panel has already
/// asked for confirmation and the runner has no stdin to answer pip's prompt.
fn pip_uninstall_command(exe: &str, name: &str) -> Result<String, String> {
    check_package_name(name)?;
    Ok(format!("{exe} -m pip uninstall -y {name}"))
}

/// One `pip install --upgrade` per outdated package, chained with `&&` so a
/// failure stops the rest. An error (and nothing run) if any name isn't
/// shell-plain.
fn pip_update_all_command(exe: &str, outdated: &[PythonOutdatedPkg]) -> Result<String, String> {
    let mut segments = Vec::new();
    for package in outdated {
        check_package_name(&package.name)?;
        segments.push(format!("{exe} -m pip install --upgrade {}", package.name));
    }
    Ok(segments.join(" && "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outdated(name: &str) -> PythonOutdatedPkg {
        PythonOutdatedPkg {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            latest_version: "1.0.1".to_string(),
            kind: python_backend::classify_update("1.0.0", "1.0.1"),
        }
    }

    #[test]
    fn install_command_pins_a_version_only_when_given() {
        assert_eq!(
            pip_install_command("python", "requests", None).as_deref(),
            Ok("python -m pip install requests")
        );
        assert_eq!(
            pip_install_command("python", "requests", Some("2.31.0")).as_deref(),
            Ok("python -m pip install requests==2.31.0")
        );
    }

    #[test]
    fn commands_run_through_the_given_interpreter() {
        let exe = "venv\\Scripts\\python.exe";
        assert!(
            pip_install_command(exe, "flask", None)
                .unwrap()
                .starts_with("venv\\Scripts\\python.exe -m pip ")
        );
        assert!(
            pip_uninstall_command(exe, "flask")
                .unwrap()
                .starts_with("venv\\Scripts\\python.exe -m pip ")
        );
    }

    #[test]
    fn uninstall_command_never_prompts() {
        assert_eq!(
            pip_uninstall_command("python", "flask").as_deref(),
            Ok("python -m pip uninstall -y flask")
        );
    }

    #[test]
    fn commands_refuse_shell_syntax() {
        assert!(pip_install_command("python", "requests; calc", None).is_err());
        assert!(pip_install_command("python", "requests", Some("1.0 && calc")).is_err());
        assert!(pip_install_command("python", "--index-url=http://evil.test", None).is_err());
        assert!(pip_uninstall_command("python", "flask|more").is_err());
        assert!(pip_update_all_command("python", &[outdated("flask"), outdated("x && calc")]).is_err());
    }

    #[test]
    fn update_all_chains_one_upgrade_per_package() {
        assert_eq!(
            pip_update_all_command("py", &[outdated("flask"), outdated("requests")]).as_deref(),
            Ok("py -m pip install --upgrade flask && py -m pip install --upgrade requests")
        );
        assert_eq!(pip_update_all_command("py", &[]).as_deref(), Ok(""));
    }
}
