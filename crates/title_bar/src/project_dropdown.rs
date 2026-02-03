use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription,
    WeakEntity, actions,
};
use menu;
use project::{Project, Worktree, git_store::Repository};
use recent_projects::{RecentProjectEntry, delete_recent_project, get_recent_projects};
use settings::WorktreeId;
use ui::{ContextMenu, DocumentationAside, DocumentationSide, Tooltip, prelude::*};
use workspace::{CloseIntent, Workspace};

actions!(project_dropdown, [RemoveSelectedFolder]);

const RECENT_PROJECTS_INLINE_LIMIT: usize = 5;

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
    _recent_projects: Rc<RefCell<Vec<RecentProjectEntry>>>,
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
        let recent_projects: Rc<RefCell<Vec<RecentProjectEntry>>> =
            Rc::new(RefCell::new(Vec::new()));

        let menu = Self::build_menu(
            project,
            workspace.clone(),
            initial_active_worktree_id,
            menu_shell.clone(),
            worktree_ids.clone(),
            recent_projects.clone(),
            window,
            cx,
        );

        *menu_shell.borrow_mut() = Some(menu.clone());

        let _subscription = cx.subscribe(&menu, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });

        let recent_projects_for_fetch = recent_projects.clone();
        let menu_shell_for_fetch = menu_shell.clone();
        let workspace_for_fetch = workspace.clone();

        cx.spawn_in(window, async move |_this, cx| {
            let current_workspace_id = cx
                .update(|_, cx| {
                    workspace_for_fetch
                        .upgrade()
                        .and_then(|ws| ws.read(cx).database_id())
                })
                .ok()
                .flatten();

            let projects = get_recent_projects(current_workspace_id, None).await;

            cx.update(|window, cx| {
                *recent_projects_for_fetch.borrow_mut() = projects;

                if let Some(menu_entity) = menu_shell_for_fetch.borrow().clone() {
                    menu_entity.update(cx, |menu, cx| {
                        menu.rebuild(window, cx);
                    });
                }
            })
            .ok()
        })
        .detach();

        Self {
            menu,
            workspace,
            worktree_ids,
            menu_shell,
            _recent_projects: recent_projects,
            _subscription,
        }
    }

    fn build_menu(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        initial_active_worktree_id: Option<WorktreeId>,
        menu_shell: Rc<RefCell<Option<Entity<ContextMenu>>>>,
        worktree_ids: Rc<RefCell<Vec<WorktreeId>>>,
        recent_projects: Rc<RefCell<Vec<RecentProjectEntry>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build_persistent(window, cx, move |menu, window, cx| {
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

                menu = menu.custom_entry(
                    move |_window, _cx| {
                        let name = name.clone();
                        let branch = branch.clone();
                        let workspace_for_remove = workspace_for_remove.clone();
                        let menu_shell = menu_shell_for_remove.clone();

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
                                .tooltip({
                                    let menu_shell = menu_shell.clone();
                                    move |window, cx| {
                                        if let Some(menu_entity) = menu_shell.borrow().as_ref() {
                                            let focus_handle = menu_entity.focus_handle(cx);
                                            Tooltip::for_action_in(
                                                "Remove Folder",
                                                &RemoveSelectedFolder,
                                                &focus_handle,
                                                cx,
                                            )
                                        } else {
                                            Tooltip::text("Remove Folder")(window, cx)
                                        }
                                    }
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

            menu = menu.separator();

            let recent = recent_projects.borrow();

            if !recent.is_empty() {
                menu = menu.header("Recent Projects");

                let enter_hint = window.keystroke_text_for(&menu::Confirm);
                let cmd_enter_hint = window.keystroke_text_for(&menu::SecondaryConfirm);

                let inline_count = recent.len().min(RECENT_PROJECTS_INLINE_LIMIT);
                for entry in recent.iter().take(inline_count) {
                    menu = Self::add_recent_project_entry(
                        menu,
                        entry.clone(),
                        workspace.clone(),
                        menu_shell.clone(),
                        recent_projects.clone(),
                        &enter_hint,
                        &cmd_enter_hint,
                    );
                }

                if recent.len() > RECENT_PROJECTS_INLINE_LIMIT {
                    let remaining_projects: Vec<RecentProjectEntry> = recent
                        .iter()
                        .skip(RECENT_PROJECTS_INLINE_LIMIT)
                        .cloned()
                        .collect();
                    let workspace_for_submenu = workspace.clone();
                    let menu_shell_for_submenu = menu_shell.clone();
                    let recent_projects_for_submenu = recent_projects.clone();

                    menu = menu.submenu("View More…", move |submenu, window, _cx| {
                        let enter_hint = window.keystroke_text_for(&menu::Confirm);
                        let cmd_enter_hint = window.keystroke_text_for(&menu::SecondaryConfirm);

                        let mut submenu = submenu;
                        for entry in &remaining_projects {
                            submenu = Self::add_recent_project_entry(
                                submenu,
                                entry.clone(),
                                workspace_for_submenu.clone(),
                                menu_shell_for_submenu.clone(),
                                recent_projects_for_submenu.clone(),
                                &enter_hint,
                                &cmd_enter_hint,
                            );
                        }
                        submenu
                    });
                }

                menu = menu.separator();
            }
            drop(recent);

            menu.action(
                "Add Folder to Workspace",
                workspace::AddFolderToProject.boxed_clone(),
            )
        })
    }

    fn add_recent_project_entry(
        menu: ContextMenu,
        entry: RecentProjectEntry,
        workspace: WeakEntity<Workspace>,
        menu_shell: Rc<RefCell<Option<Entity<ContextMenu>>>>,
        recent_projects: Rc<RefCell<Vec<RecentProjectEntry>>>,
        enter_hint: &str,
        cmd_enter_hint: &str,
    ) -> ContextMenu {
        let name = entry.name.clone();
        let full_path = entry.full_path.clone();
        let paths = entry.paths.clone();
        let workspace_id = entry.workspace_id;

        let element_id = format!("remove-recent-{}", full_path);

        let enter_hint = enter_hint.to_string();
        let cmd_enter_hint = cmd_enter_hint.to_string();
        let full_path_for_docs = full_path;
        let docs_aside = DocumentationAside {
            side: DocumentationSide::Right,
            render: Rc::new(move |cx| {
                v_flex()
                    .gap_1()
                    .child(Label::new(full_path_for_docs.clone()).size(LabelSize::Small))
                    .child(
                        h_flex()
                            .pt_1()
                            .gap_1()
                            .border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                Label::new(format!("{} reuses this window", enter_hint))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                Label::new(format!("{} opens a new one", cmd_enter_hint))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .into_any_element()
            }),
        };

        menu.custom_entry_with_docs(
            {
                let menu_shell_for_delete = menu_shell;
                let recent_projects_for_delete = recent_projects;

                move |_window, _cx| {
                    let name = name.clone();
                    let menu_shell = menu_shell_for_delete.clone();
                    let recent_projects = recent_projects_for_delete.clone();

                    h_flex()
                        .group(name.clone())
                        .w_full()
                        .justify_between()
                        .child(Label::new(name.clone()))
                        .child(
                            IconButton::new(element_id.clone(), IconName::Close)
                                .visible_on_hover(name)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .tooltip(Tooltip::text("Remove from Recent Projects"))
                                .on_click({
                                    move |_, window, cx| {
                                        let menu_shell = menu_shell.clone();
                                        let recent_projects = recent_projects.clone();

                                        recent_projects
                                            .borrow_mut()
                                            .retain(|p| p.workspace_id != workspace_id);

                                        if let Some(menu_entity) = menu_shell.borrow().clone() {
                                            menu_entity.update(cx, |menu, cx| {
                                                menu.rebuild(window, cx);
                                            });
                                        }

                                        cx.background_spawn(async move {
                                            delete_recent_project(workspace_id).await;
                                        })
                                        .detach();
                                    }
                                }),
                        )
                        .into_any_element()
                }
            },
            move |window, cx| {
                let create_new_window = window.modifiers().platform;
                Self::open_recent_project(
                    workspace.clone(),
                    paths.clone(),
                    create_new_window,
                    window,
                    cx,
                );
                window.dispatch_action(menu::Cancel.boxed_clone(), cx);
            },
            Some(docs_aside),
        )
    }

    fn open_recent_project(
        workspace: WeakEntity<Workspace>,
        paths: Vec<PathBuf>,
        create_new_window: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(workspace) = workspace.upgrade() else {
            return;
        };

        workspace.update(cx, |workspace, cx| {
            if create_new_window {
                workspace.open_workspace_for_paths(false, paths, window, cx)
            } else {
                cx.spawn_in(window, {
                    let paths = paths.clone();
                    async move |workspace, cx| {
                        let continue_replacing = workspace
                            .update_in(cx, |workspace, window, cx| {
                                workspace.prepare_to_close(CloseIntent::ReplaceWindow, window, cx)
                            })?
                            .await?;
                        if continue_replacing {
                            workspace
                                .update_in(cx, |workspace, window, cx| {
                                    workspace.open_workspace_for_paths(true, paths, window, cx)
                                })?
                                .await
                        } else {
                            Ok(())
                        }
                    }
                })
            }
            .detach_and_log_err(cx);
        });
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
