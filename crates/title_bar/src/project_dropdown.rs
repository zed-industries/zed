use std::cell::RefCell;
use std::rc::Rc;

use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription,
    WeakEntity, actions,
};
use menu;
use project::{Project, Worktree, git_store::Repository};
use settings::WorktreeId;
use ui::{ContextMenu, Tooltip, prelude::*};
use workspace::Workspace;

actions!(project_dropdown, [RemoveSelectedFolder]);

struct ProjectEntry {
    worktree_id: WorktreeId,
    name: SharedString,
    branch: Option<SharedString>,
    is_active: bool,
}

pub struct ProjectDropdown {
    menu: Entity<ContextMenu>,
    workspace: WeakEntity<Workspace>,
    worktree_ids: Rc<RefCell<Vec<WorktreeId>>>,
    menu_shell: Rc<RefCell<Option<Entity<ContextMenu>>>>,
    _subscription: Subscription,
}

impl ProjectDropdown {
    pub fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        initial_active_worktree_id: Option<WorktreeId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let menu_shell: Rc<RefCell<Option<Entity<ContextMenu>>>> = Rc::new(RefCell::new(None));
        let worktree_ids: Rc<RefCell<Vec<WorktreeId>>> = Rc::new(RefCell::new(Vec::new()));

        let menu = Self::build_menu(
            project,
            workspace.clone(),
            initial_active_worktree_id,
            menu_shell.clone(),
            worktree_ids.clone(),
            window,
            cx,
        );

        *menu_shell.borrow_mut() = Some(menu.clone());

        let _subscription = cx.subscribe(&menu, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            menu,
            workspace,
            worktree_ids,
            menu_shell,
            _subscription,
        }
    }

    fn build_menu(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        initial_active_worktree_id: Option<WorktreeId>,
        menu_shell: Rc<RefCell<Option<Entity<ContextMenu>>>>,
        worktree_ids: Rc<RefCell<Vec<WorktreeId>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build_persistent(window, cx, move |menu, _window, cx| {
            let active_worktree_id = if menu_shell.borrow().is_some() {
                workspace
                    .upgrade()
                    .and_then(|ws| ws.read(cx).active_worktree_override())
                    .or(initial_active_worktree_id)
            } else {
                initial_active_worktree_id
            };

            let entries = Self::get_project_entries(&project, active_worktree_id, cx);

            // Update the worktree_ids list so we can map selected_index -> worktree_id.
            {
                let mut ids = worktree_ids.borrow_mut();
                ids.clear();
                for entry in &entries {
                    ids.push(entry.worktree_id);
                }
            }

            let mut menu = menu.header("Open Folders");

            for entry in entries {
                let worktree_id = entry.worktree_id;
                let name = entry.name.clone();
                let branch = entry.branch.clone();
                let is_active = entry.is_active;

                let workspace_for_select = workspace.clone();
                let workspace_for_remove = workspace.clone();
                let menu_shell_for_remove = menu_shell.clone();

                let menu_focus_handle = menu.focus_handle(cx);

                menu = menu.custom_entry(
                    move |_window, _cx| {
                        let name = name.clone();
                        let branch = branch.clone();
                        let workspace_for_remove = workspace_for_remove.clone();
                        let menu_shell = menu_shell_for_remove.clone();
                        let menu_focus_handle = menu_focus_handle.clone();

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
                                .visible_on_hover(name)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .tooltip(move |_, cx| {
                                    Tooltip::for_action_in(
                                        "Remove Folder",
                                        &RemoveSelectedFolder,
                                        &menu_focus_handle,
                                        cx,
                                    )
                                })
                                .on_click({
                                    let workspace = workspace_for_remove;
                                    move |_, window, cx| {
                                        Self::handle_remove(
                                            workspace.clone(),
                                            worktree_id,
                                            window,
                                            cx,
                                        );

                                        if let Some(menu_entity) = menu_shell.borrow().clone() {
                                            menu_entity.update(cx, |menu, cx| {
                                                menu.rebuild(window, cx);
                                            });
                                        }
                                    }
                                }),
                            )
                            .into_any_element()
                    },
                    move |window, cx| {
                        Self::handle_select(workspace_for_select.clone(), worktree_id, window, cx);
                        window.dispatch_action(menu::Cancel.boxed_clone(), cx);
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
            workspace.update(cx, |workspace, cx| {
                workspace.set_active_worktree_override(Some(worktree_id), cx);
            });
        }
    }

    fn handle_remove(
        workspace: WeakEntity<Workspace>,
        worktree_id: WorktreeId,
        _window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(workspace) = workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                let project = workspace.project().clone();

                let current_active_id = workspace.active_worktree_override();
                let is_removing_active = current_active_id == Some(worktree_id);

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

                        workspace.set_active_worktree_override(new_active_id, cx);
                    }
                }

                project.update(cx, |project, cx| {
                    project.remove_worktree(worktree_id, cx);
                });
            });
        }
    }

    fn remove_selected_folder(
        &mut self,
        _: &RemoveSelectedFolder,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected_index = self.menu.read(cx).selected_index();

        if let Some(menu_index) = selected_index {
            // Early return because the "Open Folders" header is index 0.
            if menu_index == 0 {
                return;
            }

            let entry_index = menu_index - 1;
            let worktree_ids = self.worktree_ids.borrow();

            if entry_index < worktree_ids.len() {
                let worktree_id = worktree_ids[entry_index];
                drop(worktree_ids);

                Self::handle_remove(self.workspace.clone(), worktree_id, window, cx);

                if let Some(menu_entity) = self.menu_shell.borrow().clone() {
                    menu_entity.update(cx, |menu, cx| {
                        menu.rebuild(window, cx);
                    });
                }
            }
        }
    }
}

impl Render for ProjectDropdown {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("MultiProjectDropdown")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::remove_selected_folder))
            .child(self.menu.clone())
    }
}

impl EventEmitter<DismissEvent> for ProjectDropdown {}

impl Focusable for ProjectDropdown {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.menu.focus_handle(cx)
    }
}
