use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription,
    WeakEntity,
};
use project::{Project, Worktree, git_store::Repository};
use settings::WorktreeId;
use ui::{ContextMenu, prelude::*};
use workspace::Workspace;

use crate::TitleBar;

struct ProjectEntry {
    worktree_id: WorktreeId,
    name: SharedString,
    branch: Option<SharedString>,
    is_active: bool,
}

pub struct ProjectDropdown {
    menu: Entity<ContextMenu>,
    _subscription: Subscription,
}

impl ProjectDropdown {
    pub fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        active_worktree_id: Option<WorktreeId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let menu = Self::build_menu(project, workspace, active_worktree_id, window, cx);

        let _subscription = cx.subscribe(&menu, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            menu,
            _subscription,
        }
    }

    fn build_menu(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        active_worktree_id: Option<WorktreeId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let entries = Self::get_project_entries(&project, active_worktree_id, cx);

        ContextMenu::build(window, cx, move |menu, _window, _cx| {
            let mut menu = menu.header("Open Folders");

            for entry in entries {
                let worktree_id = entry.worktree_id;
                let name = entry.name.clone();
                let branch = entry.branch.clone();
                let is_active = entry.is_active;

                let workspace_for_select = workspace.clone();
                let workspace_for_remove = workspace.clone();

                menu = menu.custom_entry(
                    move |_window, _cx| {
                        let name = name.clone();
                        let branch = branch.clone();
                        let workspace_for_remove = workspace_for_remove.clone();

                        h_flex()
                            .group(name.clone())
                            .w_full()
                            .justify_between()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Label::new(name.clone())
                                            .when(is_active, |label| label.color(Color::Accent)),
                                    )
                                    .when_some(branch, |this, branch| {
                                        this.child(Label::new(branch).color(Color::Muted))
                                    }),
                            )
                            .child(
                                IconButton::new(
                                    ("remove", worktree_id.to_usize()),
                                    IconName::Close,
                                )
                                .visible_on_hover(name.clone())
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .on_click({
                                    let workspace = workspace_for_remove.clone();
                                    move |_, window, cx| {
                                        Self::handle_remove(
                                            workspace.clone(),
                                            worktree_id,
                                            active_worktree_id,
                                            window,
                                            cx,
                                        );
                                    }
                                }),
                            )
                            .into_any_element()
                    },
                    move |window, cx| {
                        Self::handle_select(workspace_for_select.clone(), worktree_id, window, cx);
                    },
                );
            }

            menu.separator()
                .action(
                    "Add Folder to Workspace",
                    workspace::AddFolderToProject.boxed_clone(),
                )
                .action(
                    "Open Recent Projects",
                    zed_actions::OpenRecent {
                        create_new_window: false,
                    }
                    .boxed_clone(),
                )
        })
    }

    /// Get all projects sorted alphabetically with their branch info.
    fn get_project_entries(
        project: &Entity<Project>,
        active_worktree_id: Option<WorktreeId>,
        cx: &App,
    ) -> Vec<ProjectEntry> {
        let project = project.read(cx);
        let git_store = project.git_store().read(cx);
        let repositories: Vec<_> = git_store.repositories().values().cloned().collect();

        let mut entries: Vec<ProjectEntry> = project
            .visible_worktrees(cx)
            .map(|worktree| {
                let worktree_ref = worktree.read(cx);
                let worktree_id = worktree_ref.id();
                let name = SharedString::from(worktree_ref.root_name().as_unix_str().to_string());

                let branch = Self::get_branch_for_worktree(worktree_ref, &repositories, cx);

                let is_active = active_worktree_id == Some(worktree_id);

                ProjectEntry {
                    worktree_id,
                    name,
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

        for repo in repositories {
            let repo = repo.read(cx);
            if repo.work_directory_abs_path == worktree_abs_path
                || worktree_abs_path.starts_with(&*repo.work_directory_abs_path)
            {
                if let Some(branch) = &repo.branch {
                    return Some(SharedString::from(branch.name().to_string()));
                }
            }
        }
        None
    }

    fn handle_select(
        workspace: WeakEntity<Workspace>,
        worktree_id: WorktreeId,
        _window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(workspace) = workspace.upgrade() {
            if let Some(titlebar) = workspace
                .read(cx)
                .titlebar_item()
                .and_then(|item| item.downcast::<TitleBar>().ok())
            {
                titlebar.update(cx, |titlebar, cx| {
                    titlebar.set_active_worktree_override(worktree_id, cx);
                });
            }
        }
    }

    fn handle_remove(
        workspace: WeakEntity<Workspace>,
        worktree_id: WorktreeId,
        active_worktree_id: Option<WorktreeId>,
        _window: &mut Window,
        cx: &mut App,
    ) {
        let is_removing_active = active_worktree_id == Some(worktree_id);

        if let Some(workspace) = workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                let project = workspace.project();

                if is_removing_active {
                    let worktrees: Vec<_> = project.read(cx).visible_worktrees(cx).collect();

                    let mut sorted: Vec<_> = worktrees
                        .iter()
                        .map(|wt| {
                            let wt = wt.read(cx);
                            (wt.root_name().as_unix_str().to_string(), wt.id())
                        })
                        .collect();
                    sorted.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

                    if let Some(idx) = sorted.iter().position(|(_, id)| *id == worktree_id) {
                        let new_active_id = if idx > 0 {
                            Some(sorted[idx - 1].1)
                        } else if sorted.len() > 1 {
                            Some(sorted[1].1)
                        } else {
                            None
                        };

                        if let Some(new_id) = new_active_id {
                            if let Some(titlebar) = workspace
                                .titlebar_item()
                                .and_then(|item| item.downcast::<TitleBar>().ok())
                            {
                                titlebar.update(cx, |titlebar, cx| {
                                    titlebar.set_active_worktree_override(new_id, cx);
                                });
                            }
                        }
                    }
                }

                project.update(cx, |project, cx| {
                    project.remove_worktree(worktree_id, cx);
                });
            });
        }
    }
}

impl Render for ProjectDropdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.menu.clone()
    }
}

impl EventEmitter<DismissEvent> for ProjectDropdown {}

impl Focusable for ProjectDropdown {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.menu.focus_handle(cx)
    }
}
