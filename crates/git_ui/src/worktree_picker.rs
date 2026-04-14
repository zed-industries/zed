use anyhow::Context as _;
use collections::HashSet;
use fuzzy::StringMatchCandidate;

use git::repository::Worktree as GitWorktree;
use gpui::{
    Action, App, AsyncWindowContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, Modifiers, ModifiersChangedEvent, ParentElement,
    Render, SharedString, Styled, Subscription, Task, WeakEntity, Window, actions, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::project_settings::ProjectSettings;
use project::{
    git_store::Repository,
    trusted_worktrees::{PathTrust, TrustedWorktrees},
};
use remote::{RemoteConnectionOptions, remote_client::ConnectionIdentifier};
use remote_connection::{RemoteConnectionModal, connect};
use settings::Settings;
use std::{path::PathBuf, sync::Arc};
use ui::{HighlightedLabel, KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::{ResultExt, debug_panic, paths::PathExt};
use workspace::{
    ModalView, MultiWorkspace, OpenMode, Workspace, notifications::DetachAndPromptErr,
};

use crate::git_panel::show_error_toast;

const MAIN_WORKTREE_DISPLAY_NAME: &str = "main";

actions!(
    git,
    [
        WorktreeFromDefault,
        WorktreeFromDefaultOnWindow,
        DeleteWorktree
    ]
);

pub fn open(
    workspace: &mut Workspace,
    _: &zed_actions::git::Worktree,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let repository = workspace.project().read(cx).active_repository(cx);
    let workspace_handle = workspace.weak_handle();
    workspace.toggle_modal(window, cx, |window, cx| {
        WorktreeList::new(repository, workspace_handle, rems(34.), window, cx)
    })
}

pub fn create_embedded(
    repository: Option<Entity<Repository>>,
    workspace: WeakEntity<Workspace>,
    width: Rems,
    window: &mut Window,
    cx: &mut Context<WorktreeList>,
) -> WorktreeList {
    WorktreeList::new_embedded(repository, workspace, width, window, cx)
}

pub struct WorktreeList {
    width: Rems,
    pub picker: Entity<Picker<WorktreeListDelegate>>,
    picker_focus_handle: FocusHandle,
    _subscription: Option<Subscription>,
    embedded: bool,
}

impl WorktreeList {
    fn new(
        repository: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::new_inner(repository, workspace, width, false, window, cx);
        this._subscription = Some(cx.subscribe(&this.picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        }));
        this
    }

    fn new_inner(
        repository: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        embedded: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let all_worktrees_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.worktrees()));

        let default_branch_request = repository.clone().map(|repository| {
            repository.update(cx, |repository, _| repository.default_branch(false))
        });

        cx.spawn_in(window, async move |this, cx| {
            let all_worktrees: Vec<_> = all_worktrees_request
                .context("No active repository")?
                .await??
                .into_iter()
                .filter(|worktree| worktree.ref_name.is_some()) // hide worktrees without a branch
                .collect();

            let default_branch = default_branch_request
                .context("No active repository")?
                .await
                .map(Result::ok)
                .ok()
                .flatten()
                .flatten();

            this.update_in(cx, |this, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.all_worktrees = Some(all_worktrees);
                    picker.delegate.default_branch = default_branch;
                    picker.delegate.refresh_forbidden_deletion_path(cx);
                    picker.refresh(window, cx);
                })
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let delegate = WorktreeListDelegate::new(workspace, repository, window, cx);
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .show_scrollbar(true)
                .modal(!embedded)
        });
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
        });

        Self {
            picker,
            picker_focus_handle,
            width,
            _subscription: None,
            embedded,
        }
    }

    fn new_embedded(
        repository: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::new_inner(repository, workspace, width, true, window, cx);
        this._subscription = Some(cx.subscribe(&this.picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        }));
        this
    }

    pub fn handle_modifiers_changed(
        &mut self,
        ev: &ModifiersChangedEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker
            .update(cx, |picker, _| picker.delegate.modifiers = ev.modifiers)
    }

    pub fn handle_new_worktree(
        &mut self,
        replace_current_window: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            let ix = picker.delegate.selected_index();
            let Some(entry) = picker.delegate.matches.get(ix) else {
                return;
            };
            let Some(default_branch) = picker.delegate.default_branch.clone() else {
                return;
            };
            if !entry.is_new {
                return;
            }
            picker.delegate.create_worktree(
                entry.worktree.display_name(),
                replace_current_window,
                Some(default_branch.into()),
                window,
                cx,
            );
        })
    }

    pub fn handle_delete(
        &mut self,
        _: &DeleteWorktree,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .delete_at(picker.delegate.selected_index, window, cx)
        })
    }
}
impl ModalView for WorktreeList {}
impl EventEmitter<DismissEvent> for WorktreeList {}

impl Focusable for WorktreeList {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.picker_focus_handle.clone()
    }
}

impl Render for WorktreeList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitWorktreeSelector")
            .w(self.width)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(|this, _: &WorktreeFromDefault, w, cx| {
                this.handle_new_worktree(false, w, cx)
            }))
            .on_action(cx.listener(|this, _: &WorktreeFromDefaultOnWindow, w, cx| {
                this.handle_new_worktree(true, w, cx)
            }))
            .on_action(cx.listener(|this, _: &DeleteWorktree, window, cx| {
                this.handle_delete(&DeleteWorktree, window, cx)
            }))
            .child(self.picker.clone())
            .when(!self.embedded, |el| {
                el.on_mouse_down_out({
                    cx.listener(move |this, _, window, cx| {
                        this.picker.update(cx, |this, cx| {
                            this.cancel(&Default::default(), window, cx);
                        })
                    })
                })
            })
    }
}

#[derive(Debug, Clone)]
struct WorktreeEntry {
    worktree: GitWorktree,
    positions: Vec<usize>,
    is_new: bool,
}

impl WorktreeEntry {
    fn can_delete(&self, forbidden_deletion_path: Option<&PathBuf>) -> bool {
        !self.is_new
            && !self.worktree.is_main
            && forbidden_deletion_path != Some(&self.worktree.path)
    }
}

pub struct WorktreeListDelegate {
    matches: Vec<WorktreeEntry>,
    all_worktrees: Option<Vec<GitWorktree>>,
    workspace: WeakEntity<Workspace>,
    repo: Option<Entity<Repository>>,
    selected_index: usize,
    last_query: String,
    modifiers: Modifiers,
    focus_handle: FocusHandle,
    default_branch: Option<SharedString>,
    forbidden_deletion_path: Option<PathBuf>,
    current_worktree_path: Option<PathBuf>,
}

impl WorktreeListDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        repo: Option<Entity<Repository>>,
        _window: &mut Window,
        cx: &mut Context<WorktreeList>,
    ) -> Self {
        let current_worktree_path = repo
            .as_ref()
            .map(|r| r.read(cx).work_directory_abs_path.to_path_buf());

        Self {
            matches: vec![],
            all_worktrees: None,
            workspace,
            selected_index: 0,
            repo,
            last_query: Default::default(),
            modifiers: Default::default(),
            focus_handle: cx.focus_handle(),
            default_branch: None,
            forbidden_deletion_path: None,
            current_worktree_path,
        }
    }

    fn create_worktree(
        &self,
        worktree_branch: &str,
        replace_current_window: bool,
        commit: Option<String>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(repo) = self.repo.clone() else {
            return;
        };

        let branch = worktree_branch.to_string();
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            let (receiver, new_worktree_path) = repo.update(cx, |repo, cx| {
                let worktree_directory_setting = ProjectSettings::get_global(cx)
                    .git
                    .worktree_directory
                    .clone();
                let new_worktree_path =
                    repo.path_for_new_linked_worktree(&branch, &worktree_directory_setting)?;
                let receiver = repo.create_worktree(
                    git::repository::CreateWorktreeTarget::NewBranch {
                        branch_name: branch.clone(),
                        base_sha: commit,
                    },
                    new_worktree_path.clone(),
                );
                anyhow::Ok((receiver, new_worktree_path))
            })?;
            receiver.await??;

            workspace.update(cx, |workspace, cx| {
                if let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) {
                    let repo_path = &repo.read(cx).snapshot().work_directory_abs_path;
                    let project = workspace.project();
                    if let Some((parent_worktree, _)) =
                        project.read(cx).find_worktree(repo_path, cx)
                    {
                        let worktree_store = project.read(cx).worktree_store();
                        trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                            if trusted_worktrees.can_trust(
                                &worktree_store,
                                parent_worktree.read(cx).id(),
                                cx,
                            ) {
                                trusted_worktrees.trust(
                                    &worktree_store,
                                    HashSet::from_iter([PathTrust::AbsPath(
                                        new_worktree_path.clone(),
                                    )]),
                                    cx,
                                );
                            }
                        });
                    }
                }
            })?;

            let (connection_options, app_state, is_local) =
                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    let connection_options = project.read(cx).remote_connection_options(cx);
                    let app_state = workspace.app_state().clone();
                    let is_local = project.read(cx).is_local();
                    (connection_options, app_state, is_local)
                })?;

            if is_local {
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.open_workspace_for_paths(
                            OpenMode::Activate,
                            vec![new_worktree_path],
                            window,
                            cx,
                        )
                    })?
                    .await?;
            } else if let Some(connection_options) = connection_options {
                open_remote_worktree(
                    connection_options,
                    vec![new_worktree_path],
                    app_state,
                    workspace.clone(),
                    replace_current_window,
                    cx,
                )
                .await?;
            }

            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed to create worktree", window, cx, |e, _, _| {
            let msg = e.to_string();
            if msg.contains("git.worktree_directory") {
                Some(format!("Invalid git.worktree_directory setting: {}", e))
            } else {
                Some(msg)
            }
        });
    }

    fn open_worktree(
        &self,
        worktree_path: &PathBuf,
        replace_current_window: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let workspace = self.workspace.clone();
        let path = worktree_path.clone();

        let Some((connection_options, app_state, is_local)) = workspace
            .update(cx, |workspace, cx| {
                let project = workspace.project().clone();
                let connection_options = project.read(cx).remote_connection_options(cx);
                let app_state = workspace.app_state().clone();
                let is_local = project.read(cx).is_local();
                (connection_options, app_state, is_local)
            })
            .log_err()
        else {
            return;
        };
        let open_mode = if replace_current_window {
            OpenMode::Activate
        } else {
            OpenMode::NewWindow
        };

        if is_local {
            let open_task = workspace.update(cx, |workspace, cx| {
                workspace.open_workspace_for_paths(open_mode, vec![path], window, cx)
            });
            cx.spawn(async move |_, _| {
                open_task?.await?;
                anyhow::Ok(())
            })
            .detach_and_prompt_err(
                "Failed to open worktree",
                window,
                cx,
                |e, _, _| Some(e.to_string()),
            );
        } else if let Some(connection_options) = connection_options {
            cx.spawn_in(window, async move |_, cx| {
                open_remote_worktree(
                    connection_options,
                    vec![path],
                    app_state,
                    workspace,
                    replace_current_window,
                    cx,
                )
                .await
            })
            .detach_and_prompt_err(
                "Failed to open worktree",
                window,
                cx,
                |e, _, _| Some(e.to_string()),
            );
        }

        cx.emit(DismissEvent);
    }

    fn base_branch<'a>(&'a self, cx: &'a mut Context<Picker<Self>>) -> Option<&'a str> {
        self.repo
            .as_ref()
            .and_then(|repo| repo.read(cx).branch.as_ref().map(|b| b.name()))
    }

    fn delete_at(&self, idx: usize, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(idx).cloned() else {
            return;
        };
        if !entry.can_delete(self.forbidden_deletion_path.as_ref()) {
            return;
        }
        let Some(repo) = self.repo.clone() else {
            return;
        };
        let workspace = self.workspace.clone();
        let path = entry.worktree.path;

        cx.spawn_in(window, async move |picker, cx| {
            let result = repo
                .update(cx, |repo, _| repo.remove_worktree(path.clone(), false))
                .await?;

            if let Err(e) = result {
                log::error!("Failed to remove worktree: {}", e);
                if let Some(workspace) = workspace.upgrade() {
                    cx.update(|_window, cx| {
                        show_error_toast(
                            workspace,
                            format!("worktree remove {}", path.display()),
                            e,
                            cx,
                        )
                    })?;
                }
                return Ok(());
            }

            picker.update_in(cx, |picker, _, cx| {
                picker.delegate.matches.retain(|e| e.worktree.path != path);
                if let Some(all_worktrees) = &mut picker.delegate.all_worktrees {
                    all_worktrees.retain(|w| w.path != path);
                }
                picker.delegate.refresh_forbidden_deletion_path(cx);
                if picker.delegate.matches.is_empty() {
                    picker.delegate.selected_index = 0;
                } else if picker.delegate.selected_index >= picker.delegate.matches.len() {
                    picker.delegate.selected_index = picker.delegate.matches.len() - 1;
                }
                cx.notify();
            })?;

            anyhow::Ok(())
        })
        .detach();
    }

    fn refresh_forbidden_deletion_path(&mut self, cx: &App) {
        let Some(workspace) = self.workspace.upgrade() else {
            debug_panic!("Workspace should always be available or else the picker would be closed");
            self.forbidden_deletion_path = None;
            return;
        };

        let visible_worktree_paths = workspace.read_with(cx, |workspace, cx| {
            workspace
                .project()
                .read(cx)
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
                .collect::<Vec<_>>()
        });

        self.forbidden_deletion_path = if visible_worktree_paths.len() == 1 {
            visible_worktree_paths.into_iter().next()
        } else {
            None
        };
    }
}

async fn open_remote_worktree(
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<workspace::AppState>,
    workspace: WeakEntity<Workspace>,
    replace_current_window: bool,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<()> {
    let workspace_window = cx
        .window_handle()
        .downcast::<MultiWorkspace>()
        .ok_or_else(|| anyhow::anyhow!("Window is not a Workspace window"))?;

    let connect_task = workspace.update_in(cx, |workspace, window, cx| {
        workspace.toggle_modal(window, cx, |window, cx| {
            RemoteConnectionModal::new(&connection_options, Vec::new(), window, cx)
        });

        let prompt = workspace
            .active_modal::<RemoteConnectionModal>(cx)
            .expect("Modal just created")
            .read(cx)
            .prompt
            .clone();

        connect(
            ConnectionIdentifier::setup(),
            connection_options.clone(),
            prompt,
            window,
            cx,
        )
        .prompt_err("Failed to connect", window, cx, |_, _, _| None)
    })?;

    let session = connect_task.await;

    workspace
        .update_in(cx, |workspace, _window, cx| {
            if let Some(prompt) = workspace.active_modal::<RemoteConnectionModal>(cx) {
                prompt.update(cx, |prompt, cx| prompt.finished(cx))
            }
        })
        .ok();

    let Some(Some(session)) = session else {
        return Ok(());
    };

    let new_project: Entity<project::Project> = cx.update(|_, cx| {
        project::Project::remote(
            session,
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            true,
            cx,
        )
    })?;

    let window_to_use = if replace_current_window {
        workspace_window
    } else {
        let workspace_position = cx
            .update(|_, cx| {
                workspace::remote_workspace_position_from_db(connection_options.clone(), &paths, cx)
            })?
            .await
            .context("fetching workspace position from db")?;

        let mut options =
            cx.update(|_, cx| (app_state.build_window_options)(workspace_position.display, cx))?;
        options.window_bounds = workspace_position.window_bounds;

        cx.open_window(options, |window, cx| {
            let workspace = cx.new(|cx| {
                let mut workspace =
                    Workspace::new(None, new_project.clone(), app_state.clone(), window, cx);
                workspace.centered_layout = workspace_position.centered_layout;
                workspace
            });
            cx.new(|cx| MultiWorkspace::new(workspace, window, cx))
        })?
    };

    workspace::open_remote_project_with_existing_connection(
        connection_options,
        new_project,
        paths,
        app_state,
        window_to_use,
        None,
        cx,
    )
    .await?;

    Ok(())
}

impl PickerDelegate for WorktreeListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select worktree…".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::Start
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let Some(all_worktrees) = self.all_worktrees.clone() else {
            return Task::ready(());
        };

        cx.spawn_in(window, async move |picker, cx| {
            let mut matches: Vec<WorktreeEntry> = if query.is_empty() {
                all_worktrees
                    .into_iter()
                    .map(|worktree| WorktreeEntry {
                        worktree,
                        positions: Vec::new(),
                        is_new: false,
                    })
                    .collect()
            } else {
                let candidates = all_worktrees
                    .iter()
                    .enumerate()
                    .map(|(ix, worktree)| {
                        let name = if worktree.is_main {
                            MAIN_WORKTREE_DISPLAY_NAME
                        } else {
                            worktree.display_name()
                        };
                        StringMatchCandidate::new(ix, name)
                    })
                    .collect::<Vec<StringMatchCandidate>>();
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .into_iter()
                .map(|candidate| WorktreeEntry {
                    worktree: all_worktrees[candidate.candidate_id].clone(),
                    positions: candidate.positions,
                    is_new: false,
                })
                .collect()
            };
            picker
                .update(cx, |picker, _| {
                    if !query.is_empty()
                        && !matches.first().is_some_and(|entry| {
                            let name = if entry.worktree.is_main {
                                MAIN_WORKTREE_DISPLAY_NAME
                            } else {
                                entry.worktree.display_name()
                            };
                            name == query
                        })
                    {
                        let query = query.replace(' ', "-");
                        matches.push(WorktreeEntry {
                            worktree: GitWorktree {
                                path: Default::default(),
                                ref_name: Some(format!("refs/heads/{query}").into()),
                                sha: Default::default(),
                                is_main: false,
                            },
                            positions: Vec::new(),
                            is_new: true,
                        })
                    }
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            core::cmp::min(delegate.selected_index, delegate.matches.len() - 1);
                    }
                    delegate.last_query = query;
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index()) else {
            return;
        };
        if entry.is_new {
            self.create_worktree(&entry.worktree.display_name(), secondary, None, window, cx);
        } else {
            self.open_worktree(&entry.worktree.path, !secondary, window, cx);
        }

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = &self.matches.get(ix)?;
        let path = entry.worktree.path.compact().to_string_lossy().to_string();
        let sha = entry
            .worktree
            .sha
            .clone()
            .chars()
            .take(7)
            .collect::<String>();

        let (branch_name, sublabel) = if entry.is_new {
            (
                Label::new(format!(
                    "Create Worktree: \"{}\"…",
                    entry.worktree.display_name()
                ))
                .truncate()
                .into_any_element(),
                format!(
                    "based off {}",
                    self.base_branch(cx).unwrap_or("the current branch")
                ),
            )
        } else {
            let display_name = if entry.worktree.is_main {
                MAIN_WORKTREE_DISPLAY_NAME
            } else {
                entry.worktree.display_name()
            };
            let first_line = display_name.lines().next().unwrap_or(display_name);
            let positions: Vec<_> = entry
                .positions
                .iter()
                .copied()
                .filter(|&pos| pos < first_line.len())
                .collect();

            (
                HighlightedLabel::new(first_line.to_owned(), positions)
                    .truncate()
                    .into_any_element(),
                path,
            )
        };

        let focus_handle = self.focus_handle.clone();

        let can_delete = entry.can_delete(self.forbidden_deletion_path.as_ref());

        let delete_button = |entry_ix: usize| {
            IconButton::new(("delete-worktree", entry_ix), IconName::Trash)
                .icon_size(IconSize::Small)
                .tooltip(move |_, cx| {
                    Tooltip::for_action_in("Delete Worktree", &DeleteWorktree, &focus_handle, cx)
                })
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.delegate.delete_at(entry_ix, window, cx);
                }))
        };

        let is_current = !entry.is_new
            && self
                .current_worktree_path
                .as_ref()
                .is_some_and(|current| *current == entry.worktree.path);

        let entry_icon = if entry.is_new {
            IconName::Plus
        } else if is_current {
            IconName::Check
        } else {
            IconName::GitWorktree
        };

        Some(
            ListItem::new(format!("worktree-menu-{ix}"))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .gap_2p5()
                        .child(
                            Icon::new(entry_icon)
                                .color(if is_current {
                                    Color::Accent
                                } else {
                                    Color::Muted
                                })
                                .size(IconSize::Small),
                        )
                        .child(v_flex().w_full().min_w_0().child(branch_name).map(|this| {
                            if entry.is_new {
                                this.child(
                                    Label::new(sublabel)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .truncate(),
                                )
                            } else {
                                this.child(
                                    h_flex()
                                        .w_full()
                                        .min_w_0()
                                        .gap_1p5()
                                        .child(
                                            Label::new(sha)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Label::new("•")
                                                .alpha(0.5)
                                                .color(Color::Muted)
                                                .size(LabelSize::Small),
                                        )
                                        .child(
                                            Label::new(sublabel)
                                                .truncate_start()
                                                .color(Color::Muted)
                                                .size(LabelSize::Small)
                                                .flex_1(),
                                        )
                                        .into_any_element(),
                                )
                            }
                        })),
                )
                .when(!entry.is_new, |this| {
                    let focus_handle = self.focus_handle.clone();
                    let open_in_new_window_button =
                        IconButton::new(("open-new-window", ix), IconName::ArrowUpRight)
                            .icon_size(IconSize::Small)
                            .tooltip(move |_, cx| {
                                Tooltip::for_action_in(
                                    "Open in New Window",
                                    &menu::SecondaryConfirm,
                                    &focus_handle,
                                    cx,
                                )
                            })
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx);
                            });

                    this.end_slot(
                        h_flex()
                            .gap_0p5()
                            .child(open_in_new_window_button)
                            .when(can_delete, |this| this.child(delete_button(ix))),
                    )
                    .show_end_slot_on_hover()
                }),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No worktrees found".into())
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();
        let selected_entry = self.matches.get(self.selected_index);
        let is_creating = selected_entry.is_some_and(|entry| entry.is_new);
        let can_delete = selected_entry
            .is_some_and(|entry| entry.can_delete(self.forbidden_deletion_path.as_ref()));

        let footer_container = h_flex()
            .w_full()
            .p_1p5()
            .gap_0p5()
            .justify_end()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant);

        if is_creating {
            let from_default_button = self.default_branch.as_ref().map(|default_branch| {
                Button::new(
                    "worktree-from-default",
                    format!("Create from: {default_branch}"),
                )
                .key_binding(
                    KeyBinding::for_action_in(&WorktreeFromDefault, &focus_handle, cx)
                        .map(|kb| kb.size(rems_from_px(12.))),
                )
                .on_click(|_, window, cx| {
                    window.dispatch_action(WorktreeFromDefault.boxed_clone(), cx)
                })
            });

            let current_branch = self.base_branch(cx).unwrap_or("current branch");

            Some(
                footer_container
                    .when_some(from_default_button, |this, button| this.child(button))
                    .child(
                        Button::new(
                            "worktree-from-current",
                            format!("Create from: {current_branch}"),
                        )
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                    )
                    .into_any(),
            )
        } else {
            Some(
                footer_container
                    .when(can_delete, |this| {
                        this.child(
                            Button::new("delete-worktree", "Delete")
                                .key_binding(
                                    KeyBinding::for_action_in(&DeleteWorktree, &focus_handle, cx)
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(DeleteWorktree.boxed_clone(), cx)
                                }),
                        )
                    })
                    .child(
                        Button::new("open-in-new-window", "Open in New Window")
                            .key_binding(
                                KeyBinding::for_action_in(
                                    &menu::SecondaryConfirm,
                                    &focus_handle,
                                    cx,
                                )
                                .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                            }),
                    )
                    .child(
                        Button::new("open-in-window", "Open")
                            .key_binding(
                                KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                    .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                            }),
                    )
                    .into_any(),
            )
        }
    }
}
