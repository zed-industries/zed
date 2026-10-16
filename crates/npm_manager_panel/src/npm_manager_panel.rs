//! npm manager panel — virtual workspace tab opened from the Node panel's
//! "npm mgr" quick action (§6.3 of the NPM_MANAGER_PANEL_PLAN port plan).
//!
//! This is a *workspace tab* (an [`Item`]), not a dock panel — it appears as a
//! tab in the active pane, mirroring the `keymap_editor` Item precedent, with
//! the pure package logic living in `npm_backend`.
//!
//! On launch it scans the workspace root (`node_backend::scan_node_projects`)
//! for every directory containing a `package.json` and shows one tab per
//! detected project, each with its own package lists and engine check. A
//! single project is always guaranteed (the scan root itself when nothing is
//! discovered), so the panel never opens empty.
//!
//! The UI follows the settings-page layout from `gpui_component::setting`
//! (five `SettingPage`s — General, Search, Installed, Updates, Vulnerabilities
//! — rendered by a `Settings` widget), with a resizable details pane splitting
//! the right edge while a package is selected.
//!
//! Per the plan:
//! - Version/parse/CLI work is delegated to `npm_backend` (no npm handling in
//!   the panel itself).
//! - The Forge "Ecosystem security-findings aggregator" (a host `Dashboard`
//!   feed) is dropped — this Zed host has no Dashboard consumer, so the panel
//!   surfaces the local per-package `npm audit` list (`list_audit_vulns`).
//! - The engines check (`npm_backend::engine_compat`) compares the active
//!   node/npm with what the project requires — `.nvmrc`, `.node-version` or
//!   `package.json` `engines`, the same files the Node panel reads — and says
//!   which file it came from. A mismatch is a warning while browsing, and
//!   installing asks for confirmation first (`run_checked`).

use gpui::{
    App, AppContext, AsyncApp, Context, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement as _, IntoElement, KeyBinding, ParentElement as _, PromptLevel, Render,
    Styled as _, Subscription, Task, WeakEntity, Window, actions, div, prelude::FluentBuilder as _,
    px,
};
use gpui_component::{
    ActiveTheme as _, IconName, Sizable as _, Size, StyledExt as _,
    button::Button,
    h_flex,
    input::{InputEvent, InputState},
    resizable::{h_resizable, resizable_panel},
    setting::Settings,
    spinner::Spinner,
    tab::{Tab, TabBar, TabVariant},
    v_flex,
};
use npm_backend::{
    EngineCompat, NpmAuditVuln, NpmInstalledPkg, NpmOutdatedPkg, NpmPackageDetails,
    NpmSearchResult, PackageManager, UpdateKind, classify_update, engine_compat, list_audit_vulns,
    list_installed, list_outdated, run_npm_cli, version_from_output,
};
use script_runner_panel::ScriptRunnerPanel;
use script_runner_panel::command::{check_package_name, check_version};
use std::path::{Path, PathBuf};
use workspace::{Item, ItemId, SerializableItem, Workspace, WorkspaceId};
use zed_actions::npm_manager::{NpmPage, OpenNpmManager};

mod details;
mod pages;
mod search;

actions!(
    npm_manager,
    [
        ReloadNpmManager,
        /// Moves the selection down one row in whichever package list page
        /// (Search/Installed/Updates/Vulnerabilities) currently has focus.
        SelectNextPackage,
        /// Moves the selection up one row, same scope as `SelectNextPackage`.
        SelectPrevPackage,
        /// Opens the selected row's details — the keyboard equivalent of
        /// clicking its name / "More info".
        OpenSelectedPackage,
        /// Runs the selected row's primary action for the page it's on:
        /// Install (Search), Remove (Installed), Update (Updates), or Fix
        /// (Vulnerabilities, only when one is offered).
        ActSelectedPackage
    ]
);

/// Opens (or activates) the npm manager tab for the given workspace root, in
/// the given workspace. This is the §6.3 seam the Node panel's quick action
/// calls.
pub fn open(
    project_root: String,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Entity<NpmManagerPanel> {
    let existing = workspace
        .active_pane()
        .read(cx)
        .items()
        .find_map(|item| item.downcast::<NpmManagerPanel>());

    let panel = if let Some(existing) = existing {
        workspace.activate_item(&existing, true, true, window, cx);
        existing
    } else {
        let panel =
            cx.new(|cx| NpmManagerPanel::new(project_root, workspace.weak_handle(), window, cx));
        workspace.add_item_to_active_pane(Box::new(panel.clone()), None, true, window, cx);
        panel
    };
    panel
}

pub fn init(cx: &mut App) {
    workspace::register_serializable_item::<NpmManagerPanel>(cx);
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, action: &OpenNpmManager, window, cx| {
            let root = workspace_root(workspace, cx);
            if !root.is_empty() {
                let path = action.path.clone();
                // A search shows the Search page even when no page was named.
                let page = action
                    .page
                    .or(action.query.as_ref().map(|_| NpmPage::Search));
                let query = action.query.clone();
                let panel = open(root, workspace, window, cx);
                panel.update(cx, |panel, cx| {
                    // Pick the project first, then reload once: a reload
                    // already in flight would ignore a second one.
                    panel.show_project_containing(path);
                    if let Some(page) = page {
                        panel.show_page(page, cx);
                    }
                    if let Some(query) = query {
                        panel.search_for(query, window, cx);
                    }
                    panel.reload_visible(cx);
                });
            }
        });
    })
    .detach();

    // The four package-list pages (Search/Installed/Updates/Vulnerabilities)
    // had no keyboard path at all before this — a row could only be opened
    // or acted on with the mouse. Scoped to `NpmPackageList` (each page sets
    // that key context on its own list container), matching how
    // `gpui_component::table::data_table` scopes its own row-navigation
    // bindings to `DataTable` rather than putting them in the JSON keymap.
    cx.bind_keys([
        KeyBinding::new("down", SelectNextPackage, Some("NpmPackageList")),
        KeyBinding::new("up", SelectPrevPackage, Some("NpmPackageList")),
        KeyBinding::new("enter", OpenSelectedPackage, Some("NpmPackageList")),
        KeyBinding::new("space", ActSelectedPackage, Some("NpmPackageList")),
    ]);
}

/// The workspace's first worktree's absolute path — the scan root the npm
/// manager tab discovers node projects under. Falls back to the process cwd
/// when no project worktree is open yet (the same resolution
/// `NodePanel`/`ScriptRunnerPanel` use), rather than silently refusing to
/// open.
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

/// One discovered Node project and its per-project package state. Rendering
/// always derives the active project by index on the panel, so no page owns
/// any of this data.
#[derive(Clone)]
pub struct NpmProject {
    pub root: String,
    pub name: String,
    pub package_manager: PackageManager,
    pub installed: Vec<NpmInstalledPkg>,
    pub installed_loading: bool,
    pub outdated: Vec<NpmOutdatedPkg>,
    pub outdated_loading: bool,
    pub audit: Vec<NpmAuditVuln>,
    pub audit_loading: bool,
    pub engine: Option<EngineCompat>,
    /// Set when this project's `list_installed` fails — the render then shows
    /// an error page with a retry instead of the settings layout.
    pub load_error: Option<String>,
}

impl NpmProject {
    fn from_detected(detected: &node_backend::DetectedNodeProject) -> Self {
        let root = detected.path.clone();
        let package_manager = npm_backend::detect_package_manager(&root);
        NpmProject {
            root,
            name: detected.name.clone(),
            package_manager,
            installed: Vec::new(),
            installed_loading: true,
            outdated: Vec::new(),
            outdated_loading: true,
            audit: Vec::new(),
            audit_loading: true,
            engine: None,
            load_error: None,
        }
    }

    /// Single-project fallback so the panel never opens empty even when the
    /// scan root isn't a Node workspace.
    fn from_root(root: String) -> Self {
        let name = Path::new(&root)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "main".into());
        let package_manager = npm_backend::detect_package_manager(&root);
        NpmProject {
            root,
            name,
            package_manager,
            installed: Vec::new(),
            installed_loading: true,
            outdated: Vec::new(),
            outdated_loading: true,
            audit: Vec::new(),
            audit_loading: true,
            engine: None,
            load_error: None,
        }
    }
}

pub struct NpmManagerPanel {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    /// Active observer on the Script Runner dock panel, replaced on every run it
    /// streams for — fires when [`ScriptRunnerPanel`] notifies, which it does on
    /// each output line and when a run finishes (§4.2 wiring).
    run_subscription: Option<Subscription>,
    /// Subscription on the search input; `Enter` kicks off a fresh registry
    /// search (kept alive for the entity's lifetime, hence underscore).
    _search_sub: Subscription,
    /// Discovered node projects, one tab each. Never empty in render.
    projects: Vec<NpmProject>,
    /// Index into `projects` of the currently shown tab.
    active_project: usize,
    /// True while the workspace-root scan is still running on the background
    /// task spawned by `new` — the render shows a scanning page meanwhile.
    scanning: bool,
    /// When true, newly installed packages are saved as devDependencies.
    install_as_dev: bool,
    search_input: Entity<InputState>,

    search_results: Vec<NpmSearchResult>,
    search_loading: bool,
    search_total: usize,
    search_page: usize,
    search_error: Option<String>,

    /// Package currently shown in the details pane.
    selected: Option<String>,
    details: Option<NpmPackageDetails>,
    details_loading: bool,
    details_error: Option<String>,
    show_readme: bool,
    /// True when the most recent `fetch_details` call should default the
    /// details pane to the README view once the packument lands (the search
    /// card's "More info" button). Falls back to the plain details page when
    /// the package ships no README.
    open_readme_on_load: bool,

    error: Option<String>,
    running_action: Option<String>,
    /// True while `reload_visible`'s background refresh is still in flight, so
    /// the periodic auto-refresh timer and the manual reload never overlap.
    reload_in_flight: bool,
    /// True when the workspace-root scan found no `package.json`, so the panel
    /// shows a "no Node projects found" notice instead of a phantom package
    /// list. The fallback project in `projects` stays so `active()` (read
    /// synchronously by the status bar) never panics.
    no_projects_found: bool,
    /// A path (a project folder, or a file in one) whose project to show once
    /// the scan has found the projects: `OpenNpmManager { path }` can arrive
    /// before the scan lands.
    pending_path: Option<PathBuf>,
    /// The section to show, as `(request_number, page_ix)`, handed to
    /// `Settings::select_page` on every render.
    page_request: Option<(usize, usize)>,
    /// Counts `show_page` calls, so each request gets a new number.
    page_requests: usize,
    /// The five page views the `Settings` widget embeds. Their `Render` reads
    /// the panel live, so `build_all` stays O(1) per page and only the active
    /// page's rows are ever built.
    pages: pages::PageViews,
}

/// How often the tab re-runs `list_installed`/`list_outdated`/`list_audit_vulns`
/// in the background while it's open, keeping the lists fresh without any user
/// action.
const AUTO_REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(60);

impl NpmManagerPanel {
    pub fn new(
        scan_root: String,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search npmjs.com for a package…"));
        let _search_sub = cx.subscribe(&search_input, |this, _, event, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                this.search_page = 0;
                this.search_npm(cx);
            }
        });

        // Kick off the workspace scan in the background, exactly like
        // `NodePanel::scan_projects`: populating the panel's project tabs
        // without stalling the first frame. Until it lands, `projects` holds a
        // single fallback project rooted on the scan root so `active()` is
        // never empty (the status bar reads `tab_tooltip_text` synchronously)
        // and the panel always has something to render.
        let fallback = NpmProject::from_root(scan_root.clone());
        let this = cx.weak_entity();
        let scan_root_task = scan_root.clone();
        cx.spawn(async move |_: WeakEntity<Self>, cx: &mut AsyncApp| {
            let detected = cx
                .background_spawn(
                    async move { node_backend::scan_node_projects(&scan_root_task, 0) },
                )
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.populate_projects(scan_root, detected, cx);
                    });
                }
            });
        })
        .detach();

        // Periodic auto-refresh: while the panel is alive, re-run the active
        // project's `installed`/`outdated`/`audit` lists every
        // `AUTO_REFRESH_INTERVAL`, unless a reload (manual or automatic) is
        // already in flight or an install/update action is running right now.
        let auto_refresh_this = cx.weak_entity();
        cx.spawn(async move |_: WeakEntity<Self>, cx: &mut AsyncApp| {
            loop {
                cx.background_executor().timer(AUTO_REFRESH_INTERVAL).await;
                let alive = cx.update(|app| {
                    if let Some(panel) = auto_refresh_this.upgrade() {
                        let _ = panel.update(app, |panel, cx| {
                            if !panel.reload_in_flight && panel.running_action.is_none() {
                                panel.reload_active(true, cx);
                            }
                        });
                        true
                    } else {
                        false
                    }
                });
                if !alive {
                    break;
                }
            }
        })
        .detach();

        let pages = {
            let panel = cx.weak_entity();
            pages::PageViews::new(panel, cx)
        };

        NpmManagerPanel {
            focus_handle: cx.focus_handle(),
            workspace,
            run_subscription: None,
            _search_sub,
            projects: vec![fallback],
            active_project: 0,
            scanning: true,
            install_as_dev: false,
            search_input,
            pages,
            search_results: Vec::new(),
            search_loading: false,
            search_total: 0,
            search_page: 0,
            search_error: None,
            selected: None,
            details: None,
            details_loading: false,
            details_error: None,
            show_readme: false,
            open_readme_on_load: false,
            pending_path: None,
            page_request: None,
            page_requests: 0,
            error: None,
            running_action: None,
            reload_in_flight: false,
            no_projects_found: false,
        }
    }

    fn active(&self) -> &NpmProject {
        self.projects
            .get(self.active_project)
            .expect("projects is never empty in render")
    }

    fn active_mut(&mut self) -> &mut NpmProject {
        self.projects
            .get_mut(self.active_project)
            .expect("projects is never empty once populated")
    }

    /// Turns a scan result into the per-project tab set (falling back to a
    /// single project rooted on the scan root), then loads the active one.
    fn populate_projects(
        &mut self,
        scan_root: String,
        detected: Vec<node_backend::DetectedNodeProject>,
        cx: &mut Context<Self>,
    ) {
        let projects = if detected.is_empty() {
            vec![NpmProject::from_root(scan_root.clone())]
        } else {
            detected.iter().map(NpmProject::from_detected).collect()
        };
        self.no_projects_found = detected.is_empty();
        self.active_project = projects
            .iter()
            .position(|p| p.root == scan_root)
            .unwrap_or(0);
        self.projects = projects;
        // A project asked for before the scan landed (`show_project_containing`).
        if let Some(path) = self.pending_path.take()
            && let Some(index) = self.project_index_for(&path)
        {
            self.active_project = index;
        }
        self.scanning = false;
        self.reload_visible(cx);
    }

    /// Re-runs the workspace-root scan. Used by the notice shown when no Node
    /// projects were found, so discovering a new `package.json` doesn't force
    /// an app restart (or a new tab).
    fn rescan(&mut self, cx: &mut Context<Self>) {
        let Some(root) = self.projects.first().map(|p| p.root.clone()) else {
            return;
        };
        self.scanning = true;
        self.no_projects_found = false;
        cx.notify();

        let this = cx.weak_entity();
        let scan_root = root.clone();
        let scan_root_task = scan_root.clone();
        cx.spawn(async move |_: WeakEntity<Self>, cx: &mut AsyncApp| {
            let detected = cx
                .background_spawn(
                    async move { node_backend::scan_node_projects(&scan_root_task, 0) },
                )
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.populate_projects(scan_root, detected, cx);
                    });
                }
            });
        })
        .detach();
    }

    fn select_project(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.projects.len() || index == self.active_project {
            return;
        }
        self.active_project = index;
        self.reload_visible(cx);
    }

    /// Shows the project `path` is in: the deepest project folder containing
    /// it. While the scan is still running, the path is kept and applied when
    /// it lands (`populate_projects`). Doesn't reload: the caller does, once.
    pub fn show_project_containing(&mut self, path: Option<PathBuf>) {
        let Some(path) = path else {
            return;
        };
        if self.scanning {
            self.pending_path = Some(path);
            return;
        }
        if let Some(index) = self.project_index_for(&path) {
            self.active_project = index;
        }
    }

    /// The project whose folder `path` is in (or is), the deepest one when
    /// projects are nested.
    fn project_index_for(&self, path: &Path) -> Option<usize> {
        self.projects
            .iter()
            .enumerate()
            .filter(|(_, project)| path.starts_with(&project.root))
            .max_by_key(|(_, project)| Path::new(&project.root).components().count())
            .map(|(index, _)| index)
    }

    /// Shows `page`. The `Settings` widget applies it on its next render.
    pub fn show_page(&mut self, page: NpmPage, cx: &mut Context<Self>) {
        self.page_requests += 1;
        self.page_request = Some((self.page_requests, page_index(page)));
        cx.notify();
    }

    /// Puts `query` in the search box and searches the registry for it.
    pub fn search_for(&mut self, query: String, window: &mut Window, cx: &mut Context<Self>) {
        self.search_input
            .update(cx, |input, cx| input.set_value(query, window, cx));
        self.search_page = 0;
        self.search_npm(cx);
    }

    fn cli(&self) -> &'static str {
        self.active().package_manager.cli_name()
    }

    // ── Loading ─────────────────────────────────────────────────────────

    /// Reloads whichever project is currently active.
    ///
    /// `silent` is used by the periodic auto-refresh: the old lists stay on
    /// screen while the probes run, then the new data lands in a single
    /// notification — no spinner flash, no full page rebuild in the user's
    /// peripheral vision. Manual reloads pass `false` so the loading state is
    /// shown immediately.
    fn reload_visible(&mut self, cx: &mut Context<Self>) {
        self.reload_active(false, cx);
    }

    fn reload_active(&mut self, silent: bool, cx: &mut Context<Self>) {
        self.error = None;
        // Nothing to load when the scan found no Node projects — the panel is
        // showing the "no projects" notice, not a phantom package list.
        if self.scanning || self.reload_in_flight || self.no_projects_found {
            return;
        }
        self.reload_in_flight = true;
        let ix = self.active_project;
        if !silent {
            let active = self.active_mut();
            active.load_error = None;
            active.installed_loading = true;
            active.outdated_loading = true;
            active.audit_loading = true;
        }
        cx.notify();

        let root = self.active().root.clone();
        let manager = self.active().package_manager;
        let cli = manager.cli_name().to_string();

        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            // All five probes run concurrently, so the refresh waits on the
            // slowest one instead of serializing them end to end.
            let (installed, outdated, audit, node_ver, npm_ver) = futures::join!(
                cx.background_spawn({
                    let root = root.clone();
                    let cli = cli.clone();
                    async move { list_installed(&root, &cli) }
                }),
                cx.background_spawn({
                    let root = root.clone();
                    let cli = cli.clone();
                    async move { list_outdated(&root, &cli) }
                }),
                cx.background_spawn({
                    let root = root.clone();
                    let cli = cli.clone();
                    async move { list_audit_vulns(&root, &cli) }
                }),
                cx.background_spawn({
                    let root = root.clone();
                    async move {
                        run_npm_cli(&root, "node", &["--version".to_string()])
                            .ok()
                            .map(|s| version_from_output(&s))
                    }
                }),
                cx.background_spawn({
                    let root = root.clone();
                    let cli = cli.clone();
                    async move {
                        run_npm_cli(&root, &cli, &["--version".to_string()])
                            .ok()
                            .map(|s| version_from_output(&s))
                    }
                }),
            );
            // Reads the project's version files, so off the main thread too.
            let engine = cx
                .background_spawn({
                    let root = root.clone();
                    async move {
                        Self::compute_engine(&root, node_ver.as_deref(), npm_ver.as_deref())
                    }
                })
                .await;

            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        let Some(active) = panel.projects.get_mut(ix) else {
                            panel.reload_in_flight = false;
                            return;
                        };
                        if !silent {
                            match installed {
                                Ok(list) => active.installed = list,
                                Err(e) => active.load_error = Some(e),
                            }
                            active.outdated = outdated.unwrap_or_default();
                            active.audit = audit.unwrap_or_default();
                        } else {
                            // Silent refresh: keep existing lists and just
                            // merge in whatever the probes produced, so a
                            // temporarily failed probe never nukes a page.
                            if let Ok(list) = installed {
                                active.installed = list;
                            }
                            if let Ok(list) = outdated {
                                active.outdated = list;
                            }
                            if let Ok(list) = audit {
                                active.audit = list;
                            }
                        }
                        active.installed_loading = false;
                        active.outdated_loading = false;
                        active.audit_loading = false;
                        active.engine = engine;
                        panel.reload_in_flight = false;
                        cx.notify();
                    });
                }
            });
        })
        .detach();
    }

    /// The active node/npm against what the project requires. Reads the
    /// project's files: call it off the main thread.
    fn compute_engine(
        root: &str,
        node_ver: Option<&str>,
        npm_ver: Option<&str>,
    ) -> Option<EngineCompat> {
        let (Some(node), Some(npm)) = (node_ver, npm_ver) else {
            return None;
        };
        Some(engine_compat(Path::new(root), node, npm))
    }

    // ── Actions ─────────────────────────────────────────────────────────

    /// The live Script Runner dock panel for this workspace, if it's been
    /// loaded yet (`initialize_panels` creates it shortly after startup).
    fn runner(&self, cx: &App) -> Option<Entity<ScriptRunnerPanel>> {
        let workspace = self.workspace.upgrade()?;
        workspace.read(cx).panel::<ScriptRunnerPanel>(cx)
    }

    /// Installs a new package to an exact version in the active project,
    /// appending `--save-dev` when the dev-dependency toggle is on.
    fn install_pkg(
        &mut self,
        name: &str,
        version: &str,
        dev: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let label = format!("install {name}");
        self.run_checked(&label, install_args(name, version, dev), window, cx);
    }

    /// Runs an install, asking first when the active Node.js or npm doesn't
    /// fit what the project requires. Browsing only warns (the status line);
    /// an install is where a wrong version does lasting harm — a lockfile and
    /// `node_modules` built for the wrong runtime — so it is confirmed.
    /// Nothing is asked when it fits, or when that can't be told.
    fn run_checked(
        &mut self,
        label: &str,
        args: Result<Vec<String>, String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(warning) = self
            .active()
            .engine
            .as_ref()
            .and_then(|engine| engine.install_warning())
        else {
            return self.run_or_reject(label, args, window, cx);
        };
        let answer = window.prompt(
            PromptLevel::Warning,
            &format!("Run {label} with an unsupported version?"),
            Some(&warning),
            &["Run anyway", "Cancel"],
            cx,
        );
        let label = label.to_string();
        cx.spawn_in(window, async move |this, cx| {
            if answer.await != Ok(0) {
                return;
            }
            this.update_in(cx, |this, window, cx| {
                this.run_or_reject(&label, args, window, cx);
            })
            .ok();
        })
        .detach();
    }

    /// Updates an installed package to the given version in the active project
    /// (never the dev flag — it keeps the dependency where it already is).
    fn update_pkg(
        &mut self,
        name: &str,
        version: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.install_pkg(name, version, false, window, cx);
    }

    /// Confirms before removing — irreversible from here (no undo, and the
    /// version being removed isn't remembered anywhere), and previously a
    /// single un-gated click. Same `window.prompt` pattern as
    /// `python_manager_panel::uninstall_pkg` / `database_panel`'s
    /// `delete_connection` — see either's doc comment for why this uses
    /// GPUI's own prompt rather than a `gpui_component` modal/dialog.
    fn remove_package(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) {
        let name = name.to_string();
        let answer = window.prompt(
            PromptLevel::Warning,
            &format!("Remove {name}?"),
            Some("This removes the package from the active project and cannot be undone."),
            &["Remove", "Cancel"],
            cx,
        );
        cx.spawn_in(window, async move |this, cx| {
            if answer.await != Ok(0) {
                return;
            }
            this.update_in(cx, |this, window, cx| {
                let label = format!("remove {name}");
                this.run_or_reject(&label, remove_args(&name), window, cx);
            })
            .ok();
        })
        .detach();
    }

    /// Bulk-updates every outdated package in the active project — or only the
    /// patch-only ones when `ty` is "patch", else every patch/minor — by
    /// installing each to its latest in a single Script Runner run.
    fn update_all(&mut self, ty: &str, window: &mut Window, cx: &mut Context<Self>) {
        match update_all_args(&self.active().outdated, ty) {
            Ok(Some(args)) => self.run_checked("update all", Ok(args), window, cx),
            Ok(None) => {}
            Err(error) => self.reject(error, cx),
        }
    }

    /// Shows why a command was refused instead of running it.
    fn reject(&mut self, error: String, cx: &mut Context<Self>) {
        self.error = Some(error);
        cx.notify();
    }

    /// Runs `args` if they were built successfully; otherwise shows the
    /// refusal (see `script_runner_panel::command`).
    fn run_or_reject(
        &mut self,
        label: &str,
        args: Result<Vec<String>, String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match args {
            Ok(args) => self.kick_run(label, args, window, cx),
            Err(error) => self.reject(error, cx),
        }
    }

    /// Runs an npm command for the active project, streaming its output through
    /// the live Script Runner dock panel (§4.2 wiring) — opening that dock so
    /// the output is visible — and reloads the package lists when it finishes.
    ///
    /// Falls back to a silent background run with a spinner when the Script
    /// Runner dock panel isn't loaded yet (e.g. the very first click of a
    /// session, before `initialize_panels` has finished).
    fn kick_run(
        &mut self,
        label: &str,
        args: Vec<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.running_action.is_some() {
            return;
        }
        let root = self.active().root.clone();
        let cli = self.cli().to_string();
        self.running_action = Some(label.to_string());
        self.error = None;
        cx.notify();

        let Some(runner) = self.runner(cx) else {
            return self.background_run(root, cli, args, cx);
        };

        // The runner can only stream one command at a time — never pile a run
        // onto an already-streaming one.
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

        let command = command_line(&cli, &args);
        runner.update(cx, |runner, cx| runner.run_external(command, root, cx));

        // Reload the lists when the run finishes: the runner notifies on every
        // output line AND on completion, so this fires exactly then.
        self.run_subscription = Some(cx.observe(&runner, |this, runner, cx| {
            if !runner.read(cx).is_running() {
                this.running_action = None;
                this.reload_visible(cx);
                cx.notify();
            }
        }));

        // A blindingly-fast run can finish between `run_external` above and
        // subscribing, which would miss the completion notify — re-check once.
        if !runner.read(cx).is_running() {
            self.running_action = None;
            self.reload_visible(cx);
            cx.notify();
        }
    }

    /// Fallback when no Script Runner dock panel exists yet: run in a
    /// background task, showing only a spinner and an error on failure.
    fn background_run(
        &mut self,
        root: String,
        cli: String,
        args: Vec<String>,
        cx: &mut Context<Self>,
    ) {
        let this = cx.weak_entity();
        cx.spawn(async move |_: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move { run_npm_cli(&root, &cli, &args) })
                .await;
            let _ = cx.update(|app| {
                if let Some(panel) = this.upgrade() {
                    let _ = panel.update(app, |panel, cx| {
                        panel.running_action = None;
                        match result {
                            Ok(_) => panel.reload_visible(cx),
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

    /// The project-tab strip, only when more than one project was found.
    fn project_tabs(&self, view: &Entity<Self>) -> TabBar {
        let view = view.clone();
        TabBar::new("npm-project-tabs")
            .with_variant(TabVariant::Underline)
            .selected_index(self.active_project)
            .children(
                self.projects
                    .iter()
                    .map(|p| Tab::new().label(p.name.clone())),
            )
            .on_click(move |ix, _window, cx| {
                view.update(cx, |panel, cx| panel.select_project(*ix, cx));
            })
    }
}

impl EventEmitter<()> for NpmManagerPanel {}

impl Focusable for NpmManagerPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for NpmManagerPanel {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> gpui::SharedString {
        "Node Package Manager".into()
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<gpui::SharedString> {
        Some(format!("Node Package Manager — {}", self.active().root).into())
    }
}

impl SerializableItem for NpmManagerPanel {
    fn serialized_item_kind() -> &'static str {
        "npm_manager_panel"
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
            cx.update(|window, cx| {
                Ok(cx.new(|cx| NpmManagerPanel::new(root, workspace, window, cx)))
            })?
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

impl Render for NpmManagerPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let view = cx.entity();

        let body = if self.scanning {
            h_flex()
                .gap_2()
                .items_center()
                .p_4()
                .text_xs()
                .child(Spinner::new().with_size(Size::XSmall))
                .child(
                    div()
                        .text_color(theme.muted_foreground)
                        .child("Scanning for Node projects…"),
                )
                .into_any_element()
        } else if self.no_projects_found {
            // The scan finished but found no package.json anywhere under the
            // scan root — show a clear notice instead of a phantom project
            // with three empty (or error-swallowed) package lists.
            v_flex()
                .gap_2()
                .p_4()
                .text_xs()
                .child(
                    div()
                        .font_semibold()
                        .text_color(theme.muted_foreground)
                        .child("No Node projects found in this project."),
                )
                .child(
                    Button::new("npm-rescan")
                        .icon(IconName::Redo2)
                        .label("Rescan")
                        .with_size(Size::Small)
                        .on_click({
                            let view = view.clone();
                            move |_, _, cx| {
                                view.update(cx, |panel, cx| panel.rescan(cx));
                            }
                        }),
                )
                .into_any_element()
        } else if let Some(err) = self.active().load_error.clone() {
            v_flex()
                .gap_2()
                .p_4()
                .text_xs()
                .child(
                    div()
                        .font_semibold()
                        .text_color(theme.danger)
                        .child("Failed to reload the package lists:"),
                )
                .child(div().text_color(theme.muted_foreground).child(err))
                .child(
                    Button::new("npm-reload-error")
                        .icon(IconName::Redo2)
                        .label("Retry")
                        .with_size(Size::Small)
                        .on_click({
                            let view = view.clone();
                            move |_, _, cx| {
                                view.update(cx, |panel, cx| panel.reload_visible(cx));
                            }
                        }),
                )
                .into_any_element()
        } else {
            let active = self.active();
            let settings = Settings::new("npm-package-manager")
                .select_page(self.page_request)
                .pages(pages::build_all(active, &self.pages));
            if self.selected.is_some() {
                h_resizable("npm-details-split")
                    .child(
                        resizable_panel()
                            .child(v_flex().flex_1().min_w_0().h_full().child(settings)),
                    )
                    .child(
                        resizable_panel()
                            .size(px(400.0))
                            .size_range(px(280.0)..px(900.0))
                            .flex_none()
                            .child(details::details_pane(self, active, &view, cx)),
                    )
                    .into_any_element()
            } else {
                settings.into_any_element()
            }
        };

        let view = view.clone();
        v_flex()
            .id("npm-manager-panel")
            .track_focus(&self.focus_handle)
            .size_full()
            .pl_2()
            .bg(theme.background)
            .when(self.projects.len() > 1, move |this| {
                this.child(self.project_tabs(&view))
            })
            .child(body)
    }
}

/// `page`'s position among the `Settings` pages: the order
/// `pages::build_all` builds them in.
fn page_index(page: NpmPage) -> usize {
    match page {
        NpmPage::General => 0,
        NpmPage::Search => 1,
        NpmPage::Installed => 2,
        NpmPage::Updates => 3,
        NpmPage::Vulnerabilities => 4,
    }
}

/// The arguments that install one package pinned to an exact version,
/// with `--save-dev` when it should land in `devDependencies`. Refuses a
/// name or version that isn't shell-plain.
fn install_args(name: &str, version: &str, dev: bool) -> Result<Vec<String>, String> {
    check_package_name(name)?;
    check_version(version)?;
    let mut args = vec!["install".to_string(), format!("{name}@{version}")];
    if dev {
        args.push("--save-dev".to_string());
    }
    Ok(args)
}

/// The arguments that remove one package.
fn remove_args(name: &str) -> Result<Vec<String>, String> {
    check_package_name(name)?;
    Ok(vec!["remove".to_string(), name.to_string()])
}

/// The arguments for "Update all": one `install` followed by a
/// `name@latest` spec per qualifying package — patch-only updates when `ty`
/// is `"patch"`, otherwise patch and minor. `Ok(None)` when nothing
/// qualifies; an error (and nothing run) if any qualifying entry isn't
/// shell-plain.
///
/// A single `install` verb, not one per package: `npm install a@1 install
/// b@2` would ask npm for a package literally named `install`.
fn update_all_args(outdated: &[NpmOutdatedPkg], ty: &str) -> Result<Option<Vec<String>>, String> {
    let mut args = vec!["install".to_string()];
    for package in outdated.iter().filter(|o| {
        let kind = classify_update(&o.current, &o.latest);
        match ty {
            "patch" => kind == UpdateKind::Patch,
            _ => matches!(kind, UpdateKind::Patch | UpdateKind::Minor),
        }
    }) {
        check_package_name(&package.name)?;
        check_version(&package.latest)?;
        args.push(format!("{}@{}", package.name, package.latest));
    }
    Ok((args.len() > 1).then_some(args))
}

/// Joins a package-manager CLI and its arguments into the one command line
/// the Script Runner executes.
fn command_line(cli: &str, args: &[String]) -> String {
    let mut command = cli.to_string();
    for arg in args {
        command.push(' ');
        command.push_str(arg);
    }
    command
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

    #[test]
    fn install_pins_the_exact_version() {
        assert_eq!(
            install_args("react", "18.2.0", false).unwrap(),
            vec!["install", "react@18.2.0"]
        );
        assert_eq!(
            install_args("@types/node", "20.11.0", true).unwrap(),
            vec!["install", "@types/node@20.11.0", "--save-dev"]
        );
    }

    #[test]
    fn install_and_remove_refuse_shell_syntax() {
        assert!(install_args("react; calc", "18.2.0", false).is_err());
        assert!(install_args("react", "18.2.0 && calc", false).is_err());
        assert!(install_args("--registry=http://evil.test", "1.0.0", false).is_err());
        assert!(install_args("", "1.0.0", false).is_err());
        assert!(remove_args("react|more").is_err());
        assert_eq!(
            remove_args("@scope/widget").unwrap(),
            vec!["remove", "@scope/widget"]
        );
    }

    #[test]
    fn update_all_uses_one_install_verb() {
        let list = [
            outdated("a", "1.0.0", "1.0.5"),
            outdated("b", "2.0.0", "2.3.0"),
            outdated("c", "3.0.0", "4.0.0"),
        ];
        let args = update_all_args(&list, "minor").unwrap().unwrap();
        assert_eq!(args, vec!["install", "a@1.0.5", "b@2.3.0"]);
        assert_eq!(args.iter().filter(|arg| *arg == "install").count(), 1);
    }

    #[test]
    fn update_all_patch_only_skips_minor_and_major() {
        let list = [
            outdated("a", "1.0.0", "1.0.5"),
            outdated("b", "2.0.0", "2.3.0"),
            outdated("c", "3.0.0", "4.0.0"),
        ];
        assert_eq!(
            update_all_args(&list, "patch").unwrap().unwrap(),
            vec!["install", "a@1.0.5"]
        );
    }

    #[test]
    fn update_all_never_includes_major_or_unparseable_versions() {
        let list = [
            outdated("major", "1.0.0", "2.0.0"),
            outdated("git-dep", "github:user/repo", "1.0.0"),
            outdated("current", "1.2.3", "1.2.3"),
        ];
        assert_eq!(update_all_args(&list, "minor"), Ok(None));
        assert_eq!(update_all_args(&list, "patch"), Ok(None));
        assert_eq!(update_all_args(&[], "minor"), Ok(None));
    }

    #[test]
    fn update_all_runs_nothing_if_any_entry_is_unsafe() {
        let list = [
            outdated("a", "1.0.0", "1.0.5"),
            outdated("b && calc", "2.0.0", "2.0.1"),
        ];
        assert!(update_all_args(&list, "minor").is_err());
        // An unsafe entry that wouldn't be updated anyway doesn't block the rest.
        let list = [
            outdated("a", "1.0.0", "1.0.5"),
            outdated("b && calc", "2.0.0", "3.0.0"),
        ];
        assert_eq!(
            update_all_args(&list, "minor").unwrap().unwrap(),
            vec!["install", "a@1.0.5"]
        );
    }

    #[test]
    fn command_line_joins_cli_and_arguments() {
        let args = install_args("react", "18.2.0", true).unwrap();
        assert_eq!(
            command_line("pnpm", &args),
            "pnpm install react@18.2.0 --save-dev"
        );
        assert_eq!(command_line("npm", &[]), "npm");

        let list = [
            outdated("a", "1.0.0", "1.0.5"),
            outdated("b", "2.0.0", "2.3.0"),
        ];
        assert_eq!(
            command_line("npm", &update_all_args(&list, "minor").unwrap().unwrap()),
            "npm install a@1.0.5 b@2.3.0"
        );
    }
}
