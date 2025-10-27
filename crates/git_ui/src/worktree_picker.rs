use anyhow::Context as _;
use fuzzy::StringMatchCandidate;

use git::repository::Worktree as GitWorktree;
use gpui::{
    Action, App, AsyncApp, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, Modifiers, ModifiersChangedEvent, ParentElement,
    PathPromptOptions, Render, SharedString, Styled, Subscription, Task, WeakEntity, Window,
    actions, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::{DirectoryLister, git_store::Repository};
use recent_projects::{RemoteConnectionModal, connect};
use remote::{RemoteConnectionOptions, remote_client::ConnectionIdentifier};
use std::{path::PathBuf, sync::Arc};
use ui::{HighlightedLabel, KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace, notifications::DetachAndPromptErr};

actions!(git, [WorktreeFromDefault, WorktreeFromDefaultOnWindow]);

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(open);
}

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

pub struct WorktreeList {
    width: Rems,
    pub picker: Entity<Picker<WorktreeListDelegate>>,
    picker_focus_handle: FocusHandle,
    _subscription: Subscription,
}

impl WorktreeList {
    fn new(
        repository: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let all_worktrees_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.worktrees()));

        let default_branch_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.default_branch()));

        cx.spawn_in(window, async move |this, cx| {
            let all_worktrees = all_worktrees_request
                .context("No active repository")?
                .await??;

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
                    picker.refresh(window, cx);
                })
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let delegate = WorktreeListDelegate::new(workspace, repository, window, cx);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
        });

        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            picker_focus_handle,
            width,
            _subscription,
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        ev: &ModifiersChangedEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker
            .update(cx, |picker, _| picker.delegate.modifiers = ev.modifiers)
    }

    fn handle_new_worktree(
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
                entry.worktree.branch(),
                replace_current_window,
                Some(default_branch.into()),
                window,
                cx,
            );
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
            .child(self.picker.clone())
            .on_mouse_down_out({
                cx.listener(move |this, _, window, cx| {
                    this.picker.update(cx, |this, cx| {
                        this.cancel(&Default::default(), window, cx);
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
}

impl WorktreeListDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        repo: Option<Entity<Repository>>,
        _window: &mut Window,
        cx: &mut Context<WorktreeList>,
    ) -> Self {
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
        let workspace = self.workspace.clone();
        let Some(repo) = self.repo.clone() else {
            return;
        };

        let worktree_path = self
            .workspace
            .clone()
            .update(cx, |this, cx| {
                this.prompt_for_open_path(
                    PathPromptOptions {
                        files: false,
                        directories: true,
                        multiple: false,
                        prompt: Some("Select directory for new worktree".into()),
                    },
                    DirectoryLister::Project(this.project().clone()),
                    window,
                    cx,
                )
            })
            .log_err();
        let Some(worktree_path) = worktree_path else {
            return;
        };

        let branch = worktree_branch.to_string();
        let window_handle = window.window_handle();
        cx.spawn_in(window, async move |_, cx| {
            let Some(paths) = worktree_path.await? else {
                return anyhow::Ok(());
            };
            let path = paths.get(0).cloned().context("No path selected")?;

            repo.update(cx, |repo, _| {
                repo.create_worktree(branch.clone(), path.clone(), commit)
            })?
            .await??;

            let final_path = path.join(branch);

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
                            replace_current_window,
                            vec![final_path],
                            window,
                            cx,
                        )
                    })?
                    .await?;
            } else if let Some(connection_options) = connection_options {
                open_remote_worktree(
                    connection_options,
                    vec![final_path],
                    app_state,
                    window_handle,
                    replace_current_window,
                    cx,
                )
                .await?;
            }

            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed to create worktree", window, cx, |e, _, _| {
            Some(e.to_string())
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

        if is_local {
            let open_task = workspace.update(cx, |workspace, cx| {
                workspace.open_workspace_for_paths(replace_current_window, vec![path], window, cx)
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
            let window_handle = window.window_handle();
            cx.spawn_in(window, async move |_, cx| {
                open_remote_worktree(
                    connection_options,
                    vec![path],
                    app_state,
                    window_handle,
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
}

async fn open_remote_worktree(
    connection_options: RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<workspace::AppState>,
    window: gpui::AnyWindowHandle,
    replace_current_window: bool,
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    let workspace_window = window
        .downcast::<Workspace>()
        .ok_or_else(|| anyhow::anyhow!("Window is not a Workspace window"))?;

    let connect_task = workspace_window.update(cx, |workspace, window, cx| {
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

    workspace_window.update(cx, |workspace, _window, cx| {
        if let Some(prompt) = workspace.active_modal::<RemoteConnectionModal>(cx) {
            prompt.update(cx, |prompt, cx| prompt.finished(cx))
        }
    })?;

    let Some(Some(session)) = session else {
        return Ok(());
    };

    let new_project = cx.update(|cx| {
        project::Project::remote(
            session,
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            cx,
        )
    })?;

    let window_to_use = if replace_current_window {
        workspace_window
    } else {
        let workspace_position = cx
            .update(|cx| {
                workspace::remote_workspace_position_from_db(connection_options.clone(), &paths, cx)
            })?
            .await
            .context("fetching workspace position from db")?;

        let mut options =
            cx.update(|cx| (app_state.build_window_options)(workspace_position.display, cx))?;
        options.window_bounds = workspace_position.window_bounds;

        cx.open_window(options, |window, cx| {
            cx.new(|cx| {
                let mut workspace =
                    Workspace::new(None, new_project.clone(), app_state.clone(), window, cx);
                workspace.centered_layout = workspace_position.centered_layout;
                workspace
            })
        })?
    };

    workspace::open_remote_project_with_existing_connection(
        connection_options,
        new_project,
        paths,
        app_state,
        window_to_use,
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
                    .map(|(ix, worktree)| StringMatchCandidate::new(ix, worktree.branch()))
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
                        && !matches
                            .first()
                            .is_some_and(|entry| entry.worktree.branch() == query)
                    {
                        let query = query.replace(' ', "-");
                        matches.push(WorktreeEntry {
                            worktree: GitWorktree {
                                path: Default::default(),
                                ref_name: format!("refs/heads/{query}").into(),
                                sha: Default::default(),
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
            self.create_worktree(&entry.worktree.branch(), secondary, None, window, cx);
        } else {
            self.open_worktree(&entry.worktree.path, secondary, window, cx);
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
        let path = entry.worktree.path.to_string_lossy().to_string();
        let sha = entry
            .worktree
            .sha
            .clone()
            .chars()
            .take(7)
            .collect::<String>();

        let focus_handle = self.focus_handle.clone();
        let icon = if let Some(default_branch) = self.default_branch.clone()
            && entry.is_new
        {
            Some(
                IconButton::new("worktree-from-default", IconName::GitBranchAlt)
                    .on_click(|_, window, cx| {
                        window.dispatch_action(WorktreeFromDefault.boxed_clone(), cx)
                    })
                    .on_right_click(|_, window, cx| {
                        window.dispatch_action(WorktreeFromDefaultOnWindow.boxed_clone(), cx)
                    })
                    .tooltip(move |_, cx| {
                        Tooltip::for_action_in(
                            format!("From default branch {default_branch}"),
                            &WorktreeFromDefault,
                            &focus_handle,
                            cx,
                        )
                    }),
            )
        } else {
            None
        };

        let branch_name = if entry.is_new {
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::Plus)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Label::new(format!("Create worktree \"{}\"…", entry.worktree.branch()))
                        .single_line()
                        .truncate(),
                )
                .into_any_element()
        } else {
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::GitBranch)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(HighlightedLabel::new(
                    entry.worktree.branch().to_owned(),
                    entry.positions.clone(),
                ))
                .truncate()
                .into_any_element()
        };

        let sublabel = if entry.is_new {
            format!(
                "based off {}",
                self.base_branch(cx).unwrap_or("the current branch")
            )
        } else {
            format!("at {}", path)
        };

        Some(
            ListItem::new(SharedString::from(format!("worktree-menu-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .w_full()
                        .overflow_hidden()
                        .child(
                            h_flex()
                                .gap_6()
                                .justify_between()
                                .overflow_x_hidden()
                                .child(branch_name)
                                .when(!entry.is_new, |el| {
                                    el.child(
                                        Label::new(sha)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .into_element(),
                                    )
                                }),
                        )
                        .child(
                            div().max_w_96().child(
                                Label::new(sublabel)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .truncate()
                                    .into_any_element(),
                            ),
                        ),
                )
                .end_slot::<IconButton>(icon),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No worktrees found".into())
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_0p5()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("open-in-new-window", "Open in new window")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                )
                .child(
                    Button::new("open-in-window", "Open")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::SecondaryConfirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                        }),
                )
                .into_any(),
        )
    }
}
