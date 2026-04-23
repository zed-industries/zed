mod dev_container_suggest;
pub mod disconnected_overlay;
mod remote_connections;
mod remote_servers;
pub mod sidebar_recent_projects;
mod ssh_config;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use chrono::{DateTime, Utc};

use fs::Fs;

#[cfg(target_os = "windows")]
mod wsl_picker;

use remote::RemoteConnectionOptions;
pub use remote_connection::{RemoteConnectionModal, connect};
pub use remote_connections::{navigate_to_positions, open_remote_project};

use disconnected_overlay::DisconnectedOverlay;
use fuzzy_nucleo::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, Task, WeakEntity, Window, actions, px,
};

use picker::{
    Picker, PickerDelegate,
    highlighted_match_with_paths::{HighlightedMatch, HighlightedMatchWithPaths},
};
use project::{Worktree, git_store::Repository};
pub use remote_connections::RemoteSettings;
pub use remote_servers::RemoteServerProjects;
use settings::{Settings, WorktreeId};
use ui_input::ErasedEditor;
use workspace::ProjectGroupKey;

use dev_container::{DevContainerContext, find_devcontainer_configs};
use ui::{
    ButtonLike, ContextMenu, Divider, HighlightedLabel, KeyBinding, ListItem, ListItemSpacing,
    ListSubHeader, PopoverMenu, PopoverMenuHandle, TintColor, Tooltip, prelude::*,
};
use util::{ResultExt, paths::PathExt};
use workspace::{
    HistoryManager, ModalView, MultiWorkspace, OpenMode, OpenOptions, OpenVisible, PathList,
    SerializedWorkspaceLocation, Workspace, WorkspaceDb, WorkspaceId,
    notifications::DetachAndPromptErr, with_active_or_new_workspace,
};
use zed_actions::{OpenDevContainer, OpenRecent, OpenRemote};

actions!(
    recent_projects,
    [ToggleActionsMenu, RemoveSelected, AddToWorkspace,]
);

#[derive(Clone, Debug)]
pub struct RecentProjectEntry {
    pub name: SharedString,
    pub full_path: SharedString,
    pub paths: Vec<PathBuf>,
    pub workspace_id: WorkspaceId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug)]
struct OpenFolderEntry {
    worktree_id: WorktreeId,
    name: SharedString,
    path: PathBuf,
    branch: Option<SharedString>,
    is_active: bool,
}

#[derive(Clone, Debug)]
enum ProjectPickerEntry {
    Header(SharedString),
    OpenFolder { index: usize, positions: Vec<usize> },
    ProjectGroup(StringMatch),
    RecentProject(StringMatch),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectPickerStyle {
    Modal,
    Popover,
}

pub async fn get_recent_projects(
    current_workspace_id: Option<WorkspaceId>,
    limit: Option<usize>,
    fs: Arc<dyn fs::Fs>,
    db: &WorkspaceDb,
) -> Vec<RecentProjectEntry> {
    let workspaces = db
        .recent_project_workspaces(fs.as_ref())
        .await
        .unwrap_or_default();

    let filtered: Vec<_> = workspaces
        .into_iter()
        .filter(|(id, _, _, _)| Some(*id) != current_workspace_id)
        .filter(|(_, location, _, _)| matches!(location, SerializedWorkspaceLocation::Local))
        .collect();

    let mut all_paths: Vec<PathBuf> = filtered
        .iter()
        .flat_map(|(_, _, path_list, _)| path_list.paths().iter().cloned())
        .collect();
    all_paths.sort();
    all_paths.dedup();
    let path_details =
        util::disambiguate::compute_disambiguation_details(&all_paths, |path, detail| {
            project::path_suffix(path, detail)
        });
    let path_detail_map: std::collections::HashMap<PathBuf, usize> =
        all_paths.into_iter().zip(path_details).collect();

    let entries: Vec<RecentProjectEntry> = filtered
        .into_iter()
        .map(|(workspace_id, _, path_list, timestamp)| {
            let paths: Vec<PathBuf> = path_list.paths().to_vec();
            let ordered_paths: Vec<&PathBuf> = path_list.ordered_paths().collect();

            let name = ordered_paths
                .iter()
                .map(|p| {
                    let detail = path_detail_map.get(*p).copied().unwrap_or(0);
                    project::path_suffix(p, detail)
                })
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(", ");

            let full_path = ordered_paths
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join("\n");

            RecentProjectEntry {
                name: SharedString::from(name),
                full_path: SharedString::from(full_path),
                paths,
                workspace_id,
                timestamp,
            }
        })
        .collect();

    match limit {
        Some(n) => entries.into_iter().take(n).collect(),
        None => entries,
    }
}

pub async fn delete_recent_project(workspace_id: WorkspaceId, db: &WorkspaceDb) {
    let _ = db.delete_workspace_by_id(workspace_id).await;
}

fn get_open_folders(workspace: &Workspace, cx: &App) -> Vec<OpenFolderEntry> {
    let project = workspace.project().read(cx);
    let visible_worktrees: Vec<_> = project.visible_worktrees(cx).collect();

    if visible_worktrees.len() <= 1 {
        return Vec::new();
    }

    let active_worktree_id = if let Some(repo) = project.active_repository(cx) {
        let repo = repo.read(cx);
        let repo_path = &repo.work_directory_abs_path;
        project.visible_worktrees(cx).find_map(|worktree| {
            let worktree_path = worktree.read(cx).abs_path();
            (worktree_path == *repo_path || worktree_path.starts_with(repo_path.as_ref()))
                .then(|| worktree.read(cx).id())
        })
    } else {
        project
            .visible_worktrees(cx)
            .next()
            .map(|wt| wt.read(cx).id())
    };

    let mut all_paths: Vec<PathBuf> = visible_worktrees
        .iter()
        .map(|wt| wt.read(cx).abs_path().to_path_buf())
        .collect();
    all_paths.sort();
    all_paths.dedup();
    let path_details =
        util::disambiguate::compute_disambiguation_details(&all_paths, |path, detail| {
            project::path_suffix(path, detail)
        });
    let path_detail_map: std::collections::HashMap<PathBuf, usize> =
        all_paths.into_iter().zip(path_details).collect();

    let git_store = project.git_store().read(cx);
    let repositories: Vec<_> = git_store.repositories().values().cloned().collect();

    let mut entries: Vec<OpenFolderEntry> = visible_worktrees
        .into_iter()
        .map(|worktree| {
            let worktree_ref = worktree.read(cx);
            let worktree_id = worktree_ref.id();
            let path = worktree_ref.abs_path().to_path_buf();
            let detail = path_detail_map.get(&path).copied().unwrap_or(0);
            let name = SharedString::from(project::path_suffix(&path, detail));
            let branch = get_branch_for_worktree(worktree_ref, &repositories, cx);
            let is_active = active_worktree_id == Some(worktree_id);
            OpenFolderEntry {
                worktree_id,
                name,
                path,
                branch,
                is_active,
            }
        })
        .collect();

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    entries
}

fn get_branch_for_worktree(
    worktree: &Worktree,
    repositories: &[Entity<Repository>],
    cx: &App,
) -> Option<SharedString> {
    let worktree_abs_path = worktree.abs_path();
    repositories
        .iter()
        .filter(|repo| {
            let repo_path = &repo.read(cx).work_directory_abs_path;
            *repo_path == worktree_abs_path || worktree_abs_path.starts_with(repo_path.as_ref())
        })
        .max_by_key(|repo| repo.read(cx).work_directory_abs_path.as_os_str().len())
        .and_then(|repo| {
            repo.read(cx)
                .branch
                .as_ref()
                .map(|branch| SharedString::from(branch.name().to_string()))
        })
}

pub fn init(cx: &mut App) {
    #[cfg(target_os = "windows")]
    cx.on_action(|open_wsl: &zed_actions::wsl_actions::OpenFolderInWsl, cx| {
        let create_new_window = open_wsl.create_new_window;
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            use gpui::PathPromptOptions;
            use project::DirectoryLister;

            let paths = workspace.prompt_for_open_path(
                PathPromptOptions {
                    files: true,
                    directories: true,
                    multiple: false,
                    prompt: None,
                },
                DirectoryLister::Local(
                    workspace.project().clone(),
                    workspace.app_state().fs.clone(),
                ),
                window,
                cx,
            );

            let app_state = workspace.app_state().clone();
            let window_handle = window.window_handle().downcast::<MultiWorkspace>();

            cx.spawn_in(window, async move |workspace, cx| {
                use util::paths::SanitizedPath;

                let Some(paths) = paths.await.log_err().flatten() else {
                    return;
                };

                let wsl_path = paths
                    .iter()
                    .find_map(util::paths::WslPath::from_path);

                if let Some(util::paths::WslPath { distro, path }) = wsl_path {
                    use remote::WslConnectionOptions;

                    let connection_options = RemoteConnectionOptions::Wsl(WslConnectionOptions {
                        distro_name: distro.to_string(),
                        user: None,
                    });

                    let requesting_window = match create_new_window {
                        false => window_handle,
                        true => None,
                    };

                    let open_options = workspace::OpenOptions {
                        requesting_window,
                        ..Default::default()
                    };

                    open_remote_project(connection_options, vec![path.into()], app_state, open_options, cx).await.log_err();
                    return;
                }

                let paths = paths
                    .into_iter()
                    .filter_map(|path| SanitizedPath::new(&path).local_to_wsl())
                    .collect::<Vec<_>>();

                if paths.is_empty() {
                    let message = indoc::indoc! { r#"
                        Invalid path specified when trying to open a folder inside WSL.

                        Please note that Zed currently does not support opening network share folders inside wsl.
                    "#};

                    let _ = cx.prompt(gpui::PromptLevel::Critical, "Invalid path", Some(&message), &["Ok"]).await;
                    return;
                }

                workspace.update_in(cx, |workspace, window, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        crate::wsl_picker::WslOpenModal::new(paths, create_new_window, window, cx)
                    });
                }).log_err();
            })
            .detach();
        });
    });

    #[cfg(target_os = "windows")]
    cx.on_action(|open_wsl: &zed_actions::wsl_actions::OpenWsl, cx| {
        let create_new_window = open_wsl.create_new_window;
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let handle = cx.entity().downgrade();
            let fs = workspace.project().read(cx).fs().clone();
            workspace.toggle_modal(window, cx, |window, cx| {
                RemoteServerProjects::wsl(create_new_window, fs, window, handle, cx)
            });
        });
    });

    #[cfg(target_os = "windows")]
    cx.on_action(|open_wsl: &remote::OpenWslPath, cx| {
        let open_wsl = open_wsl.clone();
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let fs = workspace.project().read(cx).fs().clone();
            add_wsl_distro(fs, &open_wsl.distro, cx);
            let open_options = OpenOptions {
                requesting_window: window.window_handle().downcast::<MultiWorkspace>(),
                ..Default::default()
            };

            let app_state = workspace.app_state().clone();

            cx.spawn_in(window, async move |_, cx| {
                open_remote_project(
                    RemoteConnectionOptions::Wsl(open_wsl.distro.clone()),
                    open_wsl.paths,
                    app_state,
                    open_options,
                    cx,
                )
                .await
            })
            .detach();
        });
    });

    cx.on_action(|open_recent: &OpenRecent, cx| {
        let create_new_window = open_recent.create_new_window;

        match cx
            .active_window()
            .and_then(|w| w.downcast::<MultiWorkspace>())
        {
            Some(multi_workspace) => {
                cx.defer(move |cx| {
                    multi_workspace
                        .update(cx, |multi_workspace, window, cx| {
                            let window_project_groups: Vec<ProjectGroupKey> =
                                multi_workspace.project_group_keys();

                            let workspace = multi_workspace.workspace().clone();
                            workspace.update(cx, |workspace, cx| {
                                let Some(recent_projects) =
                                    workspace.active_modal::<RecentProjects>(cx)
                                else {
                                    let focus_handle = workspace.focus_handle(cx);
                                    RecentProjects::open(
                                        workspace,
                                        create_new_window,
                                        window_project_groups,
                                        window,
                                        focus_handle,
                                        cx,
                                    );
                                    return;
                                };

                                recent_projects.update(cx, |recent_projects, cx| {
                                    recent_projects
                                        .picker
                                        .update(cx, |picker, cx| picker.cycle_selection(window, cx))
                                });
                            });
                        })
                        .log_err();
                });
            }
            None => {
                with_active_or_new_workspace(cx, move |workspace, window, cx| {
                    let Some(recent_projects) = workspace.active_modal::<RecentProjects>(cx) else {
                        let focus_handle = workspace.focus_handle(cx);
                        RecentProjects::open(
                            workspace,
                            create_new_window,
                            Vec::new(),
                            window,
                            focus_handle,
                            cx,
                        );
                        return;
                    };

                    recent_projects.update(cx, |recent_projects, cx| {
                        recent_projects
                            .picker
                            .update(cx, |picker, cx| picker.cycle_selection(window, cx))
                    });
                });
            }
        }
    });
    cx.on_action(|open_remote: &OpenRemote, cx| {
        let from_existing_connection = open_remote.from_existing_connection;
        let create_new_window = open_remote.create_new_window;
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            if from_existing_connection {
                cx.propagate();
                return;
            }
            let handle = cx.entity().downgrade();
            let fs = workspace.project().read(cx).fs().clone();
            workspace.toggle_modal(window, cx, |window, cx| {
                RemoteServerProjects::new(create_new_window, fs, window, handle, cx)
            })
        });
    });

    cx.observe_new(DisconnectedOverlay::register).detach();

    cx.on_action(|_: &OpenDevContainer, cx| {
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            if !workspace.project().read(cx).is_local() {
                cx.spawn_in(window, async move |_, cx| {
                    cx.prompt(
                        gpui::PromptLevel::Critical,
                        "Cannot open Dev Container from remote project",
                        None,
                        &["Ok"],
                    )
                    .await
                    .ok();
                })
                .detach();
                return;
            }

            let fs = workspace.project().read(cx).fs().clone();
            let configs = find_devcontainer_configs(workspace, cx);
            let app_state = workspace.app_state().clone();
            let dev_container_context = DevContainerContext::from_workspace(workspace, cx);
            let handle = cx.entity().downgrade();
            workspace.toggle_modal(window, cx, |window, cx| {
                RemoteServerProjects::new_dev_container(
                    fs,
                    configs,
                    app_state,
                    dev_container_context,
                    window,
                    handle,
                    cx,
                )
            });
        });
    });

    // Subscribe to worktree additions to suggest opening the project in a dev container
    cx.observe_new(
        |workspace: &mut Workspace, window: Option<&mut Window>, cx: &mut Context<Workspace>| {
            let Some(window) = window else {
                return;
            };
            cx.subscribe_in(
                workspace.project(),
                window,
                move |workspace, project, event, window, cx| {
                    if let project::Event::WorktreeUpdatedEntries(worktree_id, updated_entries) =
                        event
                    {
                        dev_container_suggest::suggest_on_worktree_updated(
                            workspace,
                            *worktree_id,
                            updated_entries,
                            project,
                            window,
                            cx,
                        );
                    }
                },
            )
            .detach();
        },
    )
    .detach();
}

#[cfg(target_os = "windows")]
pub fn add_wsl_distro(
    fs: Arc<dyn project::Fs>,
    connection_options: &remote::WslConnectionOptions,
    cx: &App,
) {
    use gpui::ReadGlobal;
    use settings::SettingsStore;

    let distro_name = connection_options.distro_name.clone();
    let user = connection_options.user.clone();
    SettingsStore::global(cx).update_settings_file(fs, move |setting, _| {
        let connections = setting
            .remote
            .wsl_connections
            .get_or_insert(Default::default());

        if !connections
            .iter()
            .any(|conn| conn.distro_name == distro_name && conn.user == user)
        {
            use std::collections::BTreeSet;

            connections.push(settings::WslConnection {
                distro_name,
                user,
                projects: BTreeSet::new(),
            })
        }
    });
}

pub struct RecentProjects {
    pub picker: Entity<Picker<RecentProjectsDelegate>>,
    rem_width: f32,
    _subscriptions: Vec<Subscription>,
}

impl ModalView for RecentProjects {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        let submenu_focused = self.picker.update(cx, |picker, cx| {
            picker.delegate.actions_menu_handle.is_focused(window, cx)
        });
        workspace::DismissDecision::Dismiss(!submenu_focused)
    }
}

impl RecentProjects {
    fn new(
        delegate: RecentProjectsDelegate,
        fs: Option<Arc<dyn Fs>>,
        rem_width: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let style = delegate.style;
        let picker = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .list_measure_all()
                .show_scrollbar(true)
        });

        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle;
        });

        let mut subscriptions = vec![cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent))];

        if style == ProjectPickerStyle::Popover {
            let picker_focus = picker.focus_handle(cx);
            subscriptions.push(
                cx.on_focus_out(&picker_focus, window, |this, _, window, cx| {
                    let submenu_focused = this.picker.update(cx, |picker, cx| {
                        picker.delegate.actions_menu_handle.is_focused(window, cx)
                    });
                    if !submenu_focused {
                        cx.emit(DismissEvent);
                    }
                }),
            );
        }
        // We do not want to block the UI on a potentially lengthy call to DB, so we're gonna swap
        // out workspace locations once the future runs to completion.
        let db = WorkspaceDb::global(cx);
        cx.spawn_in(window, async move |this, cx| {
            let Some(fs) = fs else { return };
            let workspaces = db
                .recent_project_workspaces(fs.as_ref())
                .await
                .log_err()
                .unwrap_or_default();
            let workspaces = workspace::resolve_worktree_workspaces(workspaces, fs.as_ref()).await;
            this.update_in(cx, move |this, window, cx| {
                this.picker.update(cx, move |picker, cx| {
                    picker.delegate.set_workspaces(workspaces);
                    picker.update_matches(picker.query(cx), window, cx)
                })
            })
            .ok();
        })
        .detach();
        Self {
            picker,
            rem_width,
            _subscriptions: subscriptions,
        }
    }

    pub fn open(
        workspace: &mut Workspace,
        create_new_window: bool,
        window_project_groups: Vec<ProjectGroupKey>,
        window: &mut Window,
        focus_handle: FocusHandle,
        cx: &mut Context<Workspace>,
    ) {
        let weak = cx.entity().downgrade();
        let open_folders = get_open_folders(workspace, cx);
        let project_connection_options = workspace.project().read(cx).remote_connection_options(cx);
        let fs = Some(workspace.app_state().fs.clone());

        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate = RecentProjectsDelegate::new(
                weak,
                create_new_window,
                focus_handle,
                open_folders,
                window_project_groups,
                project_connection_options,
                ProjectPickerStyle::Modal,
            );

            Self::new(delegate, fs, 34., window, cx)
        })
    }

    pub fn popover(
        workspace: WeakEntity<Workspace>,
        window_project_groups: Vec<ProjectGroupKey>,
        create_new_window: bool,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let (open_folders, project_connection_options, fs) = workspace
            .upgrade()
            .map(|workspace| {
                let workspace = workspace.read(cx);
                (
                    get_open_folders(workspace, cx),
                    workspace.project().read(cx).remote_connection_options(cx),
                    Some(workspace.app_state().fs.clone()),
                )
            })
            .unwrap_or_else(|| (Vec::new(), None, None));

        cx.new(|cx| {
            let delegate = RecentProjectsDelegate::new(
                workspace,
                create_new_window,
                focus_handle,
                open_folders,
                window_project_groups,
                project_connection_options,
                ProjectPickerStyle::Popover,
            );
            let list = Self::new(delegate, fs, 20., window, cx);
            list.picker.focus_handle(cx).focus(window, cx);
            list
        })
    }

    fn handle_toggle_open_menu(
        &mut self,
        _: &ToggleActionsMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            let menu_handle = &picker.delegate.actions_menu_handle;
            if menu_handle.is_deployed() {
                menu_handle.hide(cx);
            } else {
                menu_handle.show(window, cx);
            }
        });
    }

    fn handle_remove_selected(
        &mut self,
        _: &RemoveSelected,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            let ix = picker.delegate.selected_index;

            match picker.delegate.filtered_entries.get(ix) {
                Some(ProjectPickerEntry::OpenFolder { index, .. }) => {
                    if let Some(folder) = picker.delegate.open_folders.get(*index) {
                        let worktree_id = folder.worktree_id;
                        let Some(workspace) = picker.delegate.workspace.upgrade() else {
                            return;
                        };
                        workspace.update(cx, |workspace, cx| {
                            let project = workspace.project().clone();
                            project.update(cx, |project, cx| {
                                project.remove_worktree(worktree_id, cx);
                            });
                        });
                        picker.delegate.open_folders = get_open_folders(workspace.read(cx), cx);
                        let query = picker.query(cx);
                        picker.update_matches(query, window, cx);
                    }
                }
                Some(ProjectPickerEntry::ProjectGroup(hit)) => {
                    if let Some(key) = picker
                        .delegate
                        .window_project_groups
                        .get(hit.candidate_id)
                        .cloned()
                    {
                        if picker.delegate.is_active_project_group(&key, cx) {
                            return;
                        }
                        picker.delegate.remove_project_group(key, window, cx);
                        let query = picker.query(cx);
                        picker.update_matches(query, window, cx);
                    }
                }
                Some(ProjectPickerEntry::RecentProject(_)) => {
                    picker.delegate.delete_recent_project(ix, window, cx);
                }
                _ => {}
            }
        });
    }

    fn handle_add_to_workspace(
        &mut self,
        _: &AddToWorkspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            let ix = picker.delegate.selected_index;

            if let Some(ProjectPickerEntry::RecentProject(hit)) =
                picker.delegate.filtered_entries.get(ix)
            {
                if let Some((_, location, paths, _)) =
                    picker.delegate.workspaces.get(hit.candidate_id)
                {
                    if matches!(location, SerializedWorkspaceLocation::Local) {
                        let paths_to_add = paths.paths().to_vec();
                        picker
                            .delegate
                            .add_paths_to_project(paths_to_add, window, cx);
                    }
                }
            }
        });
    }
}

impl EventEmitter<DismissEvent> for RecentProjects {}

impl Focusable for RecentProjects {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RecentProjects {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RecentProjects")
            .on_action(cx.listener(Self::handle_toggle_open_menu))
            .on_action(cx.listener(Self::handle_remove_selected))
            .on_action(cx.listener(Self::handle_add_to_workspace))
            .w(rems(self.rem_width))
            .child(self.picker.clone())
    }
}

pub struct RecentProjectsDelegate {
    workspace: WeakEntity<Workspace>,
    open_folders: Vec<OpenFolderEntry>,
    window_project_groups: Vec<ProjectGroupKey>,
    workspaces: Vec<(
        WorkspaceId,
        SerializedWorkspaceLocation,
        PathList,
        DateTime<Utc>,
    )>,
    filtered_entries: Vec<ProjectPickerEntry>,
    selected_index: usize,
    render_paths: bool,
    create_new_window: bool,
    // Flag to reset index when there is a new query vs not reset index when user delete an item
    reset_selected_match_index: bool,
    has_any_non_local_projects: bool,
    project_connection_options: Option<RemoteConnectionOptions>,
    focus_handle: FocusHandle,
    style: ProjectPickerStyle,
    actions_menu_handle: PopoverMenuHandle<ContextMenu>,
}

impl RecentProjectsDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        create_new_window: bool,
        focus_handle: FocusHandle,
        open_folders: Vec<OpenFolderEntry>,
        window_project_groups: Vec<ProjectGroupKey>,
        project_connection_options: Option<RemoteConnectionOptions>,
        style: ProjectPickerStyle,
    ) -> Self {
        let render_paths = style == ProjectPickerStyle::Modal;
        Self {
            workspace,
            open_folders,
            window_project_groups,
            workspaces: Vec::new(),
            filtered_entries: Vec::new(),
            selected_index: 0,
            create_new_window,
            render_paths,
            reset_selected_match_index: true,
            has_any_non_local_projects: project_connection_options.is_some(),
            project_connection_options,
            focus_handle,
            style,
            actions_menu_handle: PopoverMenuHandle::default(),
        }
    }

    pub fn set_workspaces(
        &mut self,
        workspaces: Vec<(
            WorkspaceId,
            SerializedWorkspaceLocation,
            PathList,
            DateTime<Utc>,
        )>,
    ) {
        self.workspaces = workspaces;
        let has_non_local_recent = !self
            .workspaces
            .iter()
            .all(|(_, location, _, _)| matches!(location, SerializedWorkspaceLocation::Local));
        self.has_any_non_local_projects =
            self.project_connection_options.is_some() || has_non_local_recent;
    }
}
impl EventEmitter<DismissEvent> for RecentProjectsDelegate {}
impl PickerDelegate for RecentProjectsDelegate {
    type ListItem = AnyElement;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search projects…".into()
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        h_flex()
            .flex_none()
            .h_9()
            .px_2p5()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(editor.render(window, cx))
    }

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        matches!(
            self.filtered_entries.get(ix),
            Some(
                ProjectPickerEntry::OpenFolder { .. }
                    | ProjectPickerEntry::ProjectGroup(_)
                    | ProjectPickerEntry::RecentProject(_)
            )
        )
    }

    fn update_matches(
        &mut self,
        query: String,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.trim_start();
        let case = fuzzy_nucleo::Case::smart_if_uppercase_in(query);
        let is_empty_query = query.is_empty();

        let folder_matches = if self.open_folders.is_empty() {
            Vec::new()
        } else {
            let candidates: Vec<_> = self
                .open_folders
                .iter()
                .enumerate()
                .map(|(id, folder)| StringMatchCandidate::new(id, folder.name.as_ref()))
                .collect();

            match_strings(
                &candidates,
                query,
                case,
                fuzzy_nucleo::LengthPenalty::On,
                100,
            )
        };

        let project_group_candidates: Vec<_> = self
            .window_project_groups
            .iter()
            .enumerate()
            .map(|(id, key)| {
                let combined_string = key
                    .path_list()
                    .ordered_paths()
                    .map(|path| path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("");
                StringMatchCandidate::new(id, &combined_string)
            })
            .collect();

        let project_group_matches = match_strings(
            &project_group_candidates,
            query,
            case,
            fuzzy_nucleo::LengthPenalty::On,
            100,
        );

        // Build candidates for recent projects (not current, not sibling, not open folder)
        let recent_candidates: Vec<_> = self
            .workspaces
            .iter()
            .enumerate()
            .filter(|(_, (id, _, paths, _))| self.is_valid_recent_candidate(*id, paths, cx))
            .map(|(id, (_, _, paths, _))| {
                let combined_string = paths
                    .ordered_paths()
                    .map(|path| path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("");
                StringMatchCandidate::new(id, &combined_string)
            })
            .collect();

        let recent_matches = match_strings(
            &recent_candidates,
            query,
            case,
            fuzzy_nucleo::LengthPenalty::On,
            100,
        );

        let mut entries = Vec::new();

        if !self.open_folders.is_empty() {
            let matched_folders: Vec<_> = if is_empty_query {
                (0..self.open_folders.len())
                    .map(|i| (i, Vec::new()))
                    .collect()
            } else {
                folder_matches
                    .iter()
                    .map(|m| (m.candidate_id, m.positions.clone()))
                    .collect()
            };

            if !matched_folders.is_empty() {
                entries.push(ProjectPickerEntry::Header("Current Folders".into()));
                for (index, positions) in matched_folders {
                    entries.push(ProjectPickerEntry::OpenFolder { index, positions });
                }
            }
        }

        let has_projects_to_show = if is_empty_query {
            !project_group_candidates.is_empty()
        } else {
            !project_group_matches.is_empty()
        };

        if has_projects_to_show {
            entries.push(ProjectPickerEntry::Header("This Window".into()));

            if is_empty_query {
                for id in 0..self.window_project_groups.len() {
                    entries.push(ProjectPickerEntry::ProjectGroup(StringMatch {
                        candidate_id: id,
                        score: 0.0,
                        positions: Vec::new(),
                        string: Default::default(),
                    }));
                }
            } else {
                for m in project_group_matches {
                    entries.push(ProjectPickerEntry::ProjectGroup(m));
                }
            }
        }

        let has_recent_to_show = if is_empty_query {
            !recent_candidates.is_empty()
        } else {
            !recent_matches.is_empty()
        };

        if has_recent_to_show {
            entries.push(ProjectPickerEntry::Header("Recent Projects".into()));

            if is_empty_query {
                for (id, (workspace_id, _, paths, _)) in self.workspaces.iter().enumerate() {
                    if self.is_valid_recent_candidate(*workspace_id, paths, cx) {
                        entries.push(ProjectPickerEntry::RecentProject(StringMatch {
                            candidate_id: id,
                            score: 0.0,
                            positions: Vec::new(),
                            string: Default::default(),
                        }));
                    }
                }
            } else {
                for m in recent_matches {
                    entries.push(ProjectPickerEntry::RecentProject(m));
                }
            }
        }

        self.filtered_entries = entries;

        if self.reset_selected_match_index {
            self.selected_index = self
                .filtered_entries
                .iter()
                .position(|e| !matches!(e, ProjectPickerEntry::Header(_)))
                .unwrap_or(0);
        }
        self.reset_selected_match_index = true;
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        match self.filtered_entries.get(self.selected_index) {
            Some(ProjectPickerEntry::OpenFolder { index, .. }) => {
                let Some(folder) = self.open_folders.get(*index) else {
                    return;
                };
                let worktree_id = folder.worktree_id;
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        let git_store = workspace.project().read(cx).git_store().clone();
                        git_store.update(cx, |git_store, cx| {
                            git_store.set_active_repo_for_worktree(worktree_id, cx);
                        });
                    });
                }
                cx.emit(DismissEvent);
            }
            Some(ProjectPickerEntry::ProjectGroup(selected_match)) => {
                let Some(key) = self.window_project_groups.get(selected_match.candidate_id) else {
                    return;
                };

                let key = key.clone();
                let path_list = key.path_list().clone();
                if let Some(handle) = window.window_handle().downcast::<MultiWorkspace>() {
                    cx.defer(move |cx| {
                        if let Some(task) = handle
                            .update(cx, |multi_workspace, window, cx| {
                                multi_workspace.find_or_create_local_workspace(
                                    path_list,
                                    Some(key.clone()),
                                    &[],
                                    None,
                                    OpenMode::Activate,
                                    window,
                                    cx,
                                )
                            })
                            .log_err()
                        {
                            task.detach_and_log_err(cx);
                        }
                    });
                }
                cx.emit(DismissEvent);
            }
            Some(ProjectPickerEntry::RecentProject(selected_match)) => {
                let Some(workspace) = self.workspace.upgrade() else {
                    return;
                };
                let Some((
                    candidate_workspace_id,
                    candidate_workspace_location,
                    candidate_workspace_paths,
                    _,
                )) = self.workspaces.get(selected_match.candidate_id)
                else {
                    return;
                };

                let replace_current_window = self.create_new_window == secondary;
                let candidate_workspace_id = *candidate_workspace_id;
                let candidate_workspace_location = candidate_workspace_location.clone();
                let candidate_workspace_paths = candidate_workspace_paths.clone();

                workspace.update(cx, |workspace, cx| {
                    if workspace.database_id() == Some(candidate_workspace_id) {
                        return;
                    }
                    match candidate_workspace_location {
                        SerializedWorkspaceLocation::Local => {
                            let paths = candidate_workspace_paths.paths().to_vec();
                            if replace_current_window {
                                if let Some(handle) =
                                    window.window_handle().downcast::<MultiWorkspace>()
                                {
                                    cx.defer(move |cx| {
                                        if let Some(task) = handle
                                            .update(cx, |multi_workspace, window, cx| {
                                                multi_workspace.open_project(
                                                    paths,
                                                    OpenMode::Activate,
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .log_err()
                                        {
                                            task.detach_and_log_err(cx);
                                        }
                                    });
                                }
                                return;
                            } else {
                                workspace
                                    .open_workspace_for_paths(
                                        OpenMode::NewWindow,
                                        paths,
                                        window,
                                        cx,
                                    )
                                    .detach_and_prompt_err(
                                        "Failed to open project",
                                        window,
                                        cx,
                                        |_, _, _| None,
                                    );
                            }
                        }
                        SerializedWorkspaceLocation::Remote(mut connection) => {
                            let app_state = workspace.app_state().clone();
                            let replace_window = if replace_current_window {
                                window.window_handle().downcast::<MultiWorkspace>()
                            } else {
                                None
                            };
                            let open_options = OpenOptions {
                                requesting_window: replace_window,
                                ..Default::default()
                            };
                            if let RemoteConnectionOptions::Ssh(connection) = &mut connection {
                                RemoteSettings::get_global(cx)
                                    .fill_connection_options_from_settings(connection);
                            };
                            let paths = candidate_workspace_paths.paths().to_vec();
                            cx.spawn_in(window, async move |_, cx| {
                                open_remote_project(
                                    connection.clone(),
                                    paths,
                                    app_state,
                                    open_options,
                                    cx,
                                )
                                .await
                            })
                            .detach_and_prompt_err(
                                "Failed to open project",
                                window,
                                cx,
                                |_, _, _| None,
                            );
                        }
                    }
                });
                cx.emit(DismissEvent);
            }
            _ => {}
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        let text = if self.workspaces.is_empty() && self.open_folders.is_empty() {
            "Recently opened projects will show up here".into()
        } else {
            "No matches".into()
        };
        Some(text)
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.filtered_entries.get(ix)? {
            ProjectPickerEntry::Header(title) => Some(
                v_flex()
                    .w_full()
                    .gap_1()
                    .when(ix > 0, |this| this.mt_1().child(Divider::horizontal()))
                    .child(ListSubHeader::new(title.clone()).inset(true))
                    .into_any_element(),
            ),
            ProjectPickerEntry::OpenFolder { index, positions } => {
                let folder = self.open_folders.get(*index)?;
                let name = folder.name.clone();
                let path = folder.path.compact();
                let branch = folder.branch.clone();
                let is_active = folder.is_active;
                let worktree_id = folder.worktree_id;
                let positions = positions.clone();
                let show_path = self.style == ProjectPickerStyle::Modal;

                let secondary_actions = h_flex()
                    .gap_1()
                    .child(
                        IconButton::new(("remove-folder", worktree_id.to_usize()), IconName::Close)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Remove Folder from Project"))
                            .on_click(cx.listener(move |picker, _, window, cx| {
                                let Some(workspace) = picker.delegate.workspace.upgrade() else {
                                    return;
                                };
                                workspace.update(cx, |workspace, cx| {
                                    let project = workspace.project().clone();
                                    project.update(cx, |project, cx| {
                                        project.remove_worktree(worktree_id, cx);
                                    });
                                });
                                picker.delegate.open_folders =
                                    get_open_folders(workspace.read(cx), cx);
                                let query = picker.query(cx);
                                picker.update_matches(query, window, cx);
                            })),
                    )
                    .into_any_element();

                let icon = icon_for_remote_connection(self.project_connection_options.as_ref());

                let tooltip_path: SharedString = path.to_string_lossy().to_string().into();
                let tooltip_branch = branch.clone();

                Some(
                    ListItem::new(ix)
                        .toggle_state(selected)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .child(
                            h_flex()
                                .id("open_folder_item")
                                .w_full()
                                .gap_2p5()
                                .when(self.has_any_non_local_projects, |this| {
                                    this.child(Icon::new(icon).color(Color::Muted))
                                })
                                .child(
                                    v_flex()
                                        .min_w_0()
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .child(HighlightedLabel::new(
                                                    name.to_string(),
                                                    positions,
                                                ))
                                                .when_some(branch, |this, branch| {
                                                    this.child(
                                                        Label::new(branch)
                                                            .color(Color::Muted)
                                                            .truncate(),
                                                    )
                                                })
                                                .when(is_active, |this| {
                                                    this.child(
                                                        Icon::new(IconName::Check)
                                                            .size(IconSize::Small)
                                                            .color(Color::Accent),
                                                    )
                                                }),
                                        )
                                        .when(show_path, |this| {
                                            this.child(
                                                Label::new(path.to_string_lossy().to_string())
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            )
                                        }),
                                )
                                .when(!show_path, |this| {
                                    this.tooltip(move |_, cx| {
                                        if let Some(branch) = tooltip_branch.clone() {
                                            Tooltip::with_meta(
                                                format!("{}/{}", name, branch),
                                                None,
                                                tooltip_path.clone(),
                                                cx,
                                            )
                                        } else {
                                            Tooltip::simple(tooltip_path.clone(), cx)
                                        }
                                    })
                                }),
                        )
                        .end_slot(secondary_actions)
                        .show_end_slot_on_hover()
                        .into_any_element(),
                )
            }
            ProjectPickerEntry::ProjectGroup(hit) => {
                let key = self.window_project_groups.get(hit.candidate_id)?;
                let is_active = self.is_active_project_group(key, cx);
                let paths = key.path_list();
                let ordered_paths: Vec<_> = paths
                    .ordered_paths()
                    .map(|p| p.compact().to_string_lossy().to_string())
                    .collect();
                let tooltip_path: SharedString = ordered_paths.join("\n").into();
                let icon = icon_for_remote_connection(self.project_connection_options.as_ref());

                let mut path_start_offset = 0;
                let (match_labels, path_highlights): (Vec<_>, Vec<_>) = paths
                    .ordered_paths()
                    .map(|p| p.compact())
                    .map(|path| {
                        let highlighted_text =
                            highlights_for_path(path.as_ref(), &hit.positions, path_start_offset);
                        path_start_offset += highlighted_text.1.text.len();
                        highlighted_text
                    })
                    .unzip();

                let highlighted_match = HighlightedMatchWithPaths {
                    prefix: None,
                    match_label: HighlightedMatch::join(match_labels.into_iter().flatten(), ", "),
                    paths: path_highlights,
                    active: is_active,
                };

                let project_group_key = key.clone();
                let secondary_actions = h_flex()
                    .gap_1()
                    .when(!is_active, |this| {
                        this.child(
                            IconButton::new("remove_open_project", IconName::Close)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Remove Project from Window"))
                                .on_click({
                                    let project_group_key = project_group_key.clone();
                                    cx.listener(move |picker, _, window, cx| {
                                        cx.stop_propagation();
                                        window.prevent_default();
                                        picker.delegate.remove_project_group(
                                            project_group_key.clone(),
                                            window,
                                            cx,
                                        );
                                        let query = picker.query(cx);
                                        picker.update_matches(query, window, cx);
                                    })
                                }),
                        )
                    })
                    .into_any_element();

                Some(
                    ListItem::new(ix)
                        .toggle_state(selected)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .child(
                            h_flex()
                                .id("open_project_info_container")
                                .gap_2p5()
                                .when(self.has_any_non_local_projects, |this| {
                                    this.child(Icon::new(icon).color(Color::Muted))
                                })
                                .child({
                                    let mut highlighted = highlighted_match;
                                    if !self.render_paths {
                                        highlighted.paths.clear();
                                    }
                                    highlighted.render(window, cx)
                                })
                                .tooltip(Tooltip::text(tooltip_path)),
                        )
                        .end_slot(secondary_actions)
                        .show_end_slot_on_hover()
                        .into_any_element(),
                )
            }
            ProjectPickerEntry::RecentProject(hit) => {
                let (_, location, paths, _) = self.workspaces.get(hit.candidate_id)?;
                let is_local = matches!(location, SerializedWorkspaceLocation::Local);
                let paths_to_add = paths.paths().to_vec();
                let ordered_paths: Vec<_> = paths
                    .ordered_paths()
                    .map(|p| p.compact().to_string_lossy().to_string())
                    .collect();
                let tooltip_path: SharedString = match &location {
                    SerializedWorkspaceLocation::Remote(options) => {
                        let host = options.display_name();
                        if ordered_paths.len() == 1 {
                            format!("{} ({})", ordered_paths[0], host).into()
                        } else {
                            format!("{}\n({})", ordered_paths.join("\n"), host).into()
                        }
                    }
                    _ => ordered_paths.join("\n").into(),
                };

                let mut path_start_offset = 0;
                let (match_labels, paths): (Vec<_>, Vec<_>) = paths
                    .ordered_paths()
                    .map(|p| p.compact())
                    .map(|path| {
                        let highlighted_text =
                            highlights_for_path(path.as_ref(), &hit.positions, path_start_offset);
                        path_start_offset += highlighted_text.1.text.len();
                        highlighted_text
                    })
                    .unzip();

                let tooltip_title = if paths.len() > 1 {
                    "Add Folders to this Project"
                } else {
                    "Add Folder to this Project"
                };

                let prefix = match &location {
                    SerializedWorkspaceLocation::Remote(options) => {
                        Some(SharedString::from(options.display_name()))
                    }
                    _ => None,
                };

                let highlighted_match = HighlightedMatchWithPaths {
                    prefix,
                    match_label: HighlightedMatch::join(match_labels.into_iter().flatten(), ", "),
                    paths,
                    active: false,
                };

                let focus_handle = self.focus_handle.clone();

                let secondary_actions = h_flex()
                    .gap_px()
                    .when(is_local, |this| {
                        this.child(
                            IconButton::new("add_to_workspace", IconName::FolderOpenAdd)
                                .icon_size(IconSize::Small)
                                .tooltip(move |_, cx| {
                                    Tooltip::with_meta(
                                        tooltip_title,
                                        None,
                                        "As a multi-root folder",
                                        cx,
                                    )
                                })
                                .on_click({
                                    let paths_to_add = paths_to_add.clone();
                                    cx.listener(move |picker, _event, window, cx| {
                                        cx.stop_propagation();
                                        window.prevent_default();
                                        picker.delegate.add_paths_to_project(
                                            paths_to_add.clone(),
                                            window,
                                            cx,
                                        );
                                    })
                                }),
                        )
                    })
                    .child(
                        IconButton::new("open_new_window", IconName::OpenNewWindow)
                            .icon_size(IconSize::Small)
                            .tooltip({
                                move |_, cx| {
                                    Tooltip::for_action_in(
                                        "Open Project in New Window",
                                        &menu::SecondaryConfirm,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .on_click(cx.listener(move |this, _event, window, cx| {
                                cx.stop_propagation();
                                window.prevent_default();
                                this.delegate.set_selected_index(ix, window, cx);
                                this.delegate.confirm(true, window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("delete", IconName::Close)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Delete from Recent Projects"))
                            .on_click(cx.listener(move |this, _event, window, cx| {
                                cx.stop_propagation();
                                window.prevent_default();
                                this.delegate.delete_recent_project(ix, window, cx)
                            })),
                    )
                    .into_any_element();

                let icon = icon_for_remote_connection(match location {
                    SerializedWorkspaceLocation::Local => None,
                    SerializedWorkspaceLocation::Remote(options) => Some(options),
                });

                Some(
                    ListItem::new(ix)
                        .toggle_state(selected)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .child(
                            h_flex()
                                .id("project_info_container")
                                .gap_2p5()
                                .flex_grow()
                                .when(self.has_any_non_local_projects, |this| {
                                    this.child(Icon::new(icon).color(Color::Muted))
                                })
                                .child({
                                    let mut highlighted = highlighted_match;
                                    if !self.render_paths {
                                        highlighted.paths.clear();
                                    }
                                    highlighted.render(window, cx)
                                })
                                .tooltip(move |_, cx| {
                                    Tooltip::with_meta(
                                        "Open Project in This Window",
                                        None,
                                        tooltip_path.clone(),
                                        cx,
                                    )
                                }),
                        )
                        .end_slot(secondary_actions)
                        .show_end_slot_on_hover()
                        .into_any_element(),
                )
            }
        }
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();
        let popover_style = matches!(self.style, ProjectPickerStyle::Popover);
        let is_already_open_entry = matches!(
            self.filtered_entries.get(self.selected_index),
            Some(ProjectPickerEntry::OpenFolder { .. } | ProjectPickerEntry::ProjectGroup(_))
        );

        if popover_style {
            return Some(
                v_flex()
                    .flex_1()
                    .p_1p5()
                    .gap_1()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child({
                        ButtonLike::new("open_local_folder")
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_1()
                                    .justify_between()
                                    .child(Label::new("Open Local Folders"))
                                    .child(KeyBinding::for_action_in(
                                        &workspace::Open {
                                            create_new_window: self.create_new_window,
                                        },
                                        &focus_handle,
                                        cx,
                                    )),
                            )
                            .on_click({
                                let workspace = self.workspace.clone();
                                let create_new_window = self.create_new_window;
                                move |_, window, cx| {
                                    open_local_project(
                                        workspace.clone(),
                                        create_new_window,
                                        window,
                                        cx,
                                    );
                                }
                            })
                    })
                    .child(
                        ButtonLike::new("open_remote_folder")
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_1()
                                    .justify_between()
                                    .child(Label::new("Open Remote Folder"))
                                    .child(KeyBinding::for_action(
                                        &OpenRemote {
                                            from_existing_connection: false,
                                            create_new_window: false,
                                        },
                                        cx,
                                    )),
                            )
                            .on_click(|_, window, cx| {
                                window.dispatch_action(
                                    OpenRemote {
                                        from_existing_connection: false,
                                        create_new_window: false,
                                    }
                                    .boxed_clone(),
                                    cx,
                                )
                            }),
                    )
                    .into_any(),
            );
        }

        let selected_entry = self.filtered_entries.get(self.selected_index);

        let is_current_workspace_entry =
            if let Some(ProjectPickerEntry::ProjectGroup(hit)) = selected_entry {
                self.window_project_groups
                    .get(hit.candidate_id)
                    .is_some_and(|key| self.is_active_project_group(key, cx))
            } else {
                false
            };

        let secondary_footer_actions: Option<AnyElement> = match selected_entry {
            Some(ProjectPickerEntry::OpenFolder { .. }) => Some(
                Button::new("remove_selected", "Remove Folder")
                    .key_binding(KeyBinding::for_action_in(
                        &RemoveSelected,
                        &focus_handle,
                        cx,
                    ))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(RemoveSelected.boxed_clone(), cx)
                    })
                    .into_any_element(),
            ),
            Some(ProjectPickerEntry::ProjectGroup(_)) if !is_current_workspace_entry => Some(
                Button::new("remove_selected", "Remove from Window")
                    .key_binding(KeyBinding::for_action_in(
                        &RemoveSelected,
                        &focus_handle,
                        cx,
                    ))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(RemoveSelected.boxed_clone(), cx)
                    })
                    .into_any_element(),
            ),
            Some(ProjectPickerEntry::RecentProject(_)) => Some(
                Button::new("delete_recent", "Delete")
                    .key_binding(KeyBinding::for_action_in(
                        &RemoveSelected,
                        &focus_handle,
                        cx,
                    ))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(RemoveSelected.boxed_clone(), cx)
                    })
                    .into_any_element(),
            ),
            _ => None,
        };

        Some(
            h_flex()
                .flex_1()
                .p_1p5()
                .gap_1()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .when_some(secondary_footer_actions, |this, actions| {
                    this.child(actions)
                })
                .map(|this| {
                    if is_already_open_entry {
                        this.child(
                            Button::new("activate", "Activate")
                                .key_binding(KeyBinding::for_action_in(
                                    &menu::Confirm,
                                    &focus_handle,
                                    cx,
                                ))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                                }),
                        )
                    } else {
                        this.child(
                            Button::new("open_new_window", "New Window")
                                .key_binding(KeyBinding::for_action_in(
                                    &menu::SecondaryConfirm,
                                    &focus_handle,
                                    cx,
                                ))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                                }),
                        )
                        .child(
                            Button::new("open_here", "Open")
                                .key_binding(KeyBinding::for_action_in(
                                    &menu::Confirm,
                                    &focus_handle,
                                    cx,
                                ))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                                }),
                        )
                    }
                })
                .child(Divider::vertical())
                .child(
                    PopoverMenu::new("actions-menu-popover")
                        .with_handle(self.actions_menu_handle.clone())
                        .anchor(gpui::Anchor::BottomRight)
                        .offset(gpui::Point {
                            x: px(0.0),
                            y: px(-2.0),
                        })
                        .trigger(
                            Button::new("actions-trigger", "Actions")
                                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                .key_binding(KeyBinding::for_action_in(
                                    &ToggleActionsMenu,
                                    &focus_handle,
                                    cx,
                                )),
                        )
                        .menu({
                            let focus_handle = focus_handle.clone();
                            let workspace_handle = self.workspace.clone();
                            let create_new_window = self.create_new_window;
                            let open_action = workspace::Open { create_new_window };
                            let show_add_to_workspace = match selected_entry {
                                Some(ProjectPickerEntry::RecentProject(hit)) => self
                                    .workspaces
                                    .get(hit.candidate_id)
                                    .map(|(_, loc, ..)| {
                                        matches!(loc, SerializedWorkspaceLocation::Local)
                                    })
                                    .unwrap_or(false),
                                _ => false,
                            };

                            move |window, cx| {
                                Some(ContextMenu::build(window, cx, {
                                    let focus_handle = focus_handle.clone();
                                    let workspace_handle = workspace_handle.clone();
                                    let open_action = open_action.clone();
                                    move |menu, _, _| {
                                        menu.context(focus_handle)
                                            .when(show_add_to_workspace, |menu| {
                                                menu.action(
                                                    "Add Folder to this Project",
                                                    AddToWorkspace.boxed_clone(),
                                                )
                                                .separator()
                                            })
                                            .entry(
                                                "Open Local Folders",
                                                Some(open_action.boxed_clone()),
                                                {
                                                    let workspace_handle = workspace_handle.clone();
                                                    move |window, cx| {
                                                        open_local_project(
                                                            workspace_handle.clone(),
                                                            create_new_window,
                                                            window,
                                                            cx,
                                                        );
                                                    }
                                                },
                                            )
                                            .action(
                                                "Open Remote Folder",
                                                OpenRemote {
                                                    from_existing_connection: false,
                                                    create_new_window: false,
                                                }
                                                .boxed_clone(),
                                            )
                                    }
                                }))
                            }
                        }),
                )
                .into_any(),
        )
    }
}

pub(crate) fn icon_for_remote_connection(options: Option<&RemoteConnectionOptions>) -> IconName {
    match options {
        None => IconName::Screen,
        Some(options) => match options {
            RemoteConnectionOptions::Ssh(_) => IconName::Server,
            RemoteConnectionOptions::Wsl(_) => IconName::Linux,
            RemoteConnectionOptions::Docker(_) => IconName::Box,
            #[cfg(any(test, feature = "test-support"))]
            RemoteConnectionOptions::Mock(_) => IconName::Server,
        },
    }
}

// Compute the highlighted text for the name and path
pub(crate) fn highlights_for_path(
    path: &Path,
    match_positions: &Vec<usize>,
    path_start_offset: usize,
) -> (Option<HighlightedMatch>, HighlightedMatch) {
    let path_string = path.to_string_lossy();
    let path_text = path_string.to_string();
    let path_byte_len = path_text.len();
    // Get the subset of match highlight positions that line up with the given path.
    // Also adjusts them to start at the path start
    let path_positions = match_positions
        .iter()
        .copied()
        .skip_while(|position| *position < path_start_offset)
        .take_while(|position| *position < path_start_offset + path_byte_len)
        .map(|position| position - path_start_offset)
        .collect::<Vec<_>>();

    // Again subset the highlight positions to just those that line up with the file_name
    // again adjusted to the start of the file_name
    let file_name_text_and_positions = path.file_name().map(|file_name| {
        let file_name_text = file_name.to_string_lossy().into_owned();
        let file_name_start_byte = path_byte_len - file_name_text.len();
        let highlight_positions = path_positions
            .iter()
            .copied()
            .skip_while(|position| *position < file_name_start_byte)
            .take_while(|position| *position < file_name_start_byte + file_name_text.len())
            .map(|position| position - file_name_start_byte)
            .collect::<Vec<_>>();
        HighlightedMatch {
            text: file_name_text,
            highlight_positions,
            color: Color::Default,
        }
    });

    (
        file_name_text_and_positions,
        HighlightedMatch {
            text: path_text,
            highlight_positions: path_positions,
            color: Color::Default,
        },
    )
}
fn open_local_project(
    workspace: WeakEntity<Workspace>,
    create_new_window: bool,
    window: &mut Window,
    cx: &mut App,
) {
    use gpui::PathPromptOptions;
    use project::DirectoryLister;

    let Some(workspace) = workspace.upgrade() else {
        return;
    };

    let paths = workspace.update(cx, |workspace, cx| {
        workspace.prompt_for_open_path(
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
        )
    });

    let multi_workspace_handle = window.window_handle().downcast::<MultiWorkspace>();
    window
        .spawn(cx, async move |cx| {
            let Some(paths) = paths.await.log_err().flatten() else {
                return;
            };
            if !create_new_window {
                if let Some(handle) = multi_workspace_handle {
                    if let Some(task) = handle
                        .update(cx, |multi_workspace, window, cx| {
                            multi_workspace.open_project(paths, OpenMode::Activate, window, cx)
                        })
                        .log_err()
                    {
                        task.await.log_err();
                    }
                    return;
                }
            }
            if let Some(task) = workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.open_workspace_for_paths(OpenMode::NewWindow, paths, window, cx)
                })
                .log_err()
            {
                task.await.log_err();
            }
        })
        .detach();
}

impl RecentProjectsDelegate {
    fn add_paths_to_project(
        &mut self,
        paths: Vec<PathBuf>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let open_paths_task = workspace.update(cx, |workspace, cx| {
            workspace.open_paths(
                paths,
                OpenOptions {
                    visible: Some(OpenVisible::All),
                    ..Default::default()
                },
                None,
                window,
                cx,
            )
        });
        cx.spawn_in(window, async move |picker, cx| {
            let _result = open_paths_task.await;
            picker
                .update_in(cx, |picker, window, cx| {
                    let Some(workspace) = picker.delegate.workspace.upgrade() else {
                        return;
                    };
                    picker.delegate.open_folders = get_open_folders(workspace.read(cx), cx);
                    let query = picker.query(cx);
                    picker.update_matches(query, window, cx);
                })
                .ok();
        })
        .detach();
    }

    fn delete_recent_project(
        &self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if let Some(ProjectPickerEntry::RecentProject(selected_match)) =
            self.filtered_entries.get(ix)
        {
            let (workspace_id, _, _, _) = &self.workspaces[selected_match.candidate_id];
            let workspace_id = *workspace_id;
            let fs = self
                .workspace
                .upgrade()
                .map(|ws| ws.read(cx).app_state().fs.clone());
            let db = WorkspaceDb::global(cx);
            cx.spawn_in(window, async move |this, cx| {
                db.delete_workspace_by_id(workspace_id).await.log_err();
                let Some(fs) = fs else { return };
                let workspaces = db
                    .recent_project_workspaces(fs.as_ref())
                    .await
                    .unwrap_or_default();
                let workspaces =
                    workspace::resolve_worktree_workspaces(workspaces, fs.as_ref()).await;
                this.update_in(cx, move |picker, window, cx| {
                    picker.delegate.set_workspaces(workspaces);
                    picker
                        .delegate
                        .set_selected_index(ix.saturating_sub(1), window, cx);
                    picker.delegate.reset_selected_match_index = false;
                    picker.update_matches(picker.query(cx), window, cx);
                    // After deleting a project, we want to update the history manager to reflect the change.
                    // But we do not emit a update event when user opens a project, because it's handled in `workspace::load_workspace`.
                    if let Some(history_manager) = HistoryManager::global(cx) {
                        history_manager
                            .update(cx, |this, cx| this.delete_history(workspace_id, cx));
                    }
                })
                .ok();
            })
            .detach();
        }
    }

    fn remove_project_group(
        &mut self,
        key: ProjectGroupKey,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if let Some(handle) = window.window_handle().downcast::<MultiWorkspace>() {
            let key_for_remove = key.clone();
            cx.defer(move |cx| {
                handle
                    .update(cx, |multi_workspace, window, cx| {
                        multi_workspace
                            .remove_project_group(&key_for_remove, window, cx)
                            .detach_and_log_err(cx);
                    })
                    .log_err();
            });
        }

        self.window_project_groups.retain(|k| k != &key);
    }

    fn is_current_workspace(
        &self,
        workspace_id: WorkspaceId,
        cx: &mut Context<Picker<Self>>,
    ) -> bool {
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            if Some(workspace_id) == workspace.database_id() {
                return true;
            }
        }

        false
    }

    fn is_active_project_group(&self, key: &ProjectGroupKey, cx: &App) -> bool {
        if let Some(workspace) = self.workspace.upgrade() {
            return workspace.read(cx).project_group_key(cx) == *key;
        }
        false
    }

    fn is_in_current_window_groups(&self, paths: &PathList) -> bool {
        self.window_project_groups
            .iter()
            .any(|key| key.path_list() == paths)
    }

    fn is_open_folder(&self, paths: &PathList) -> bool {
        if self.open_folders.is_empty() {
            return false;
        }

        for workspace_path in paths.paths() {
            for open_folder in &self.open_folders {
                if workspace_path == &open_folder.path {
                    return true;
                }
            }
        }

        false
    }

    fn is_valid_recent_candidate(
        &self,
        workspace_id: WorkspaceId,
        paths: &PathList,
        cx: &mut Context<Picker<Self>>,
    ) -> bool {
        !self.is_current_workspace(workspace_id, cx)
            && !self.is_in_current_window_groups(paths)
            && !self.is_open_folder(paths)
    }
}

#[cfg(test)]
mod tests {
    use gpui::{TestAppContext, UpdateGlobal};

    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use workspace::{AppState, open_paths};

    use super::*;

    #[gpui::test]
    async fn test_open_dev_container_action_with_single_config(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/project"),
                json!({
                    ".devcontainer": {
                        "devcontainer.json": "{}"
                    },
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                }),
            )
            .await;

        // Open a file path (not a directory) so that the worktree root is a
        // file. This means `active_project_directory` returns `None`, which
        // causes `DevContainerContext::from_workspace` to return `None`,
        // preventing `open_dev_container` from spawning real I/O (docker
        // commands, shell environment loading) that is incompatible with the
        // test scheduler. The modal is still created and the re-entrancy
        // guard that this test validates is still exercised.
        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/project/src/main.rs"))],
                app_state,
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        assert_eq!(cx.update(|cx| cx.windows().len()), 1);
        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());

        cx.run_until_parked();

        // This dispatch triggers with_active_or_new_workspace -> MultiWorkspace::update
        // -> Workspace::update -> toggle_modal -> new_dev_container.
        // Before the fix, this panicked with "cannot read workspace::Workspace while
        // it is already being updated" because new_dev_container and open_dev_container
        // tried to read the Workspace entity through a WeakEntity handle while it was
        // already leased by the outer update.
        cx.dispatch_action(*multi_workspace, OpenDevContainer);

        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let modal = multi_workspace
                    .workspace()
                    .read(cx)
                    .active_modal::<RemoteServerProjects>(cx);
                assert!(
                    modal.is_some(),
                    "Dev container modal should be open after dispatching OpenDevContainer"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_dev_container_action_with_multiple_configs(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/project"),
                json!({
                    ".devcontainer": {
                        "rust": {
                            "devcontainer.json": "{}"
                        },
                        "python": {
                            "devcontainer.json": "{}"
                        }
                    },
                    "src": {
                        "main.rs": "fn main() {}"
                    }
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/project"))],
                app_state,
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        assert_eq!(cx.update(|cx| cx.windows().len()), 1);
        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());

        cx.run_until_parked();

        cx.dispatch_action(*multi_workspace, OpenDevContainer);

        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                let modal = multi_workspace
                    .workspace()
                    .read(cx)
                    .active_modal::<RemoteServerProjects>(cx);
                assert!(
                    modal.is_some(),
                    "Dev container modal should be open after dispatching OpenDevContainer with multiple configs"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_open_local_project_reuses_multi_workspace_window(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        // Disable system path prompts so the injected mock is used.
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.workspace.use_system_path_prompts = Some(false);
                });
            });
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/initial-project"),
                json!({ "src": { "main.rs": "" } }),
            )
            .await;
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/new-project"), json!({ "lib": { "mod.rs": "" } }))
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/initial-project"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let initial_window_count = cx.update(|cx| cx.windows().len());
        assert_eq!(initial_window_count, 1);

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        cx.run_until_parked();

        let workspace = multi_workspace
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        // Set up the prompt mock to return the new project path.
        workspace.update(cx, |workspace, _cx| {
            workspace.set_prompt_for_open_path(Box::new(|_, _, _, _| {
                let (tx, rx) = futures::channel::oneshot::channel();
                tx.send(Some(vec![PathBuf::from(path!("/new-project"))]))
                    .ok();
                rx
            }));
        });

        // Call open_local_project with create_new_window: false.
        let weak_workspace = workspace.downgrade();
        multi_workspace
            .update(cx, |_, window, cx| {
                open_local_project(weak_workspace, false, window, cx);
            })
            .unwrap();

        cx.run_until_parked();

        // Should NOT have opened a new window.
        let final_window_count = cx.update(|cx| cx.windows().len());
        assert_eq!(
            final_window_count, initial_window_count,
            "open_local_project with create_new_window=false should reuse the current multi-workspace window"
        );
    }

    #[gpui::test]
    async fn test_open_local_project_new_window_creates_new_window(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        // Disable system path prompts so the injected mock is used.
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.workspace.use_system_path_prompts = Some(false);
                });
            });
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/initial-project"),
                json!({ "src": { "main.rs": "" } }),
            )
            .await;
        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/new-project"), json!({ "lib": { "mod.rs": "" } }))
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/initial-project"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let initial_window_count = cx.update(|cx| cx.windows().len());
        assert_eq!(initial_window_count, 1);

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        cx.run_until_parked();

        let workspace = multi_workspace
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        // Set up the prompt mock to return the new project path.
        workspace.update(cx, |workspace, _cx| {
            workspace.set_prompt_for_open_path(Box::new(|_, _, _, _| {
                let (tx, rx) = futures::channel::oneshot::channel();
                tx.send(Some(vec![PathBuf::from(path!("/new-project"))]))
                    .ok();
                rx
            }));
        });

        // Call open_local_project with create_new_window: true.
        let weak_workspace = workspace.downgrade();
        multi_workspace
            .update(cx, |_, window, cx| {
                open_local_project(weak_workspace, true, window, cx);
            })
            .unwrap();

        cx.run_until_parked();

        // Should have opened a new window.
        let final_window_count = cx.update(|cx| cx.windows().len());
        assert_eq!(
            final_window_count,
            initial_window_count + 1,
            "open_local_project with create_new_window=true should open a new window"
        );
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            state
        })
    }
}
