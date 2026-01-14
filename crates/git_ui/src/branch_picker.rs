use anyhow::Context as _;
use editor::Editor;
use fuzzy::StringMatchCandidate;

use collections::HashSet;
use git::repository::Branch;
use gpui::http_client::Url;
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, Render,
    SharedString, Styled, Subscription, Task, WeakEntity, Window, actions, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::git_store::Repository;
use project::project_settings::ProjectSettings;
use settings::Settings;
use std::sync::Arc;
use time::OffsetDateTime;
use ui::{
    Divider, HighlightedLabel, KeyBinding, ListHeader, ListItem, ListItemSpacing, Tooltip,
    prelude::*,
};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

use crate::{branch_picker, git_panel::show_error_toast};

actions!(
    branch_picker,
    [
        /// Deletes the selected git branch or remote.
        DeleteBranch,
        /// Filter the list of remotes
        FilterRemotes
    ]
);

pub fn checkout_branch(
    workspace: &mut Workspace,
    _: &zed_actions::git::CheckoutBranch,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open(workspace, &zed_actions::git::Branch, window, cx);
}

pub fn switch(
    workspace: &mut Workspace,
    _: &zed_actions::git::Switch,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open(workspace, &zed_actions::git::Branch, window, cx);
}

pub fn open(
    workspace: &mut Workspace,
    _: &zed_actions::git::Branch,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let workspace_handle = workspace.weak_handle();
    let project = workspace.project().clone();

    // Check if there's a worktree override from the project dropdown.
    // This ensures the branch picker shows branches for the project the user
    // explicitly selected in the title bar, not just the focused file's project.
    // This is only relevant if for multi-projects workspaces.
    let repository = workspace
        .active_worktree_override()
        .and_then(|override_id| {
            let project_ref = project.read(cx);
            project_ref
                .worktree_for_id(override_id, cx)
                .and_then(|worktree| {
                    let worktree_abs_path = worktree.read(cx).abs_path();
                    let git_store = project_ref.git_store().read(cx);
                    git_store
                        .repositories()
                        .values()
                        .find(|repo| {
                            let repo_path = &repo.read(cx).work_directory_abs_path;
                            *repo_path == worktree_abs_path
                                || worktree_abs_path.starts_with(repo_path.as_ref())
                        })
                        .cloned()
                })
        })
        .or_else(|| project.read(cx).active_repository(cx));

    workspace.toggle_modal(window, cx, |window, cx| {
        BranchList::new(
            workspace_handle,
            repository,
            BranchListStyle::Modal,
            rems(34.),
            window,
            cx,
        )
    })
}

pub fn popover(
    workspace: WeakEntity<Workspace>,
    modal_style: bool,
    repository: Option<Entity<Repository>>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<BranchList> {
    let (style, width) = if modal_style {
        (BranchListStyle::Modal, rems(34.))
    } else {
        (BranchListStyle::Popover, rems(20.))
    };

    cx.new(|cx| {
        let list = BranchList::new(workspace, repository, style, width, window, cx);
        list.focus_handle(cx).focus(window, cx);
        list
    })
}

pub fn create_embedded(
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    width: Rems,
    window: &mut Window,
    cx: &mut Context<BranchList>,
) -> BranchList {
    BranchList::new_embedded(workspace, repository, width, window, cx)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BranchListStyle {
    Modal,
    Popover,
}

pub struct BranchList {
    width: Rems,
    pub picker: Entity<Picker<BranchListDelegate>>,
    picker_focus_handle: FocusHandle,
    _subscription: Option<Subscription>,
    embedded: bool,
}

impl BranchList {
    fn new(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        style: BranchListStyle,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::new_inner(workspace, repository, style, width, false, window, cx);
        this._subscription = Some(cx.subscribe(&this.picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        }));
        this
    }

    fn new_inner(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        style: BranchListStyle,
        width: Rems,
        embedded: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let all_branches_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.branches()));

        let default_branch_request = repository.clone().map(|repository| {
            repository.update(cx, |repository, _| repository.default_branch(false))
        });

        cx.spawn_in(window, async move |this, cx| {
            let mut all_branches = all_branches_request
                .context("No active repository")?
                .await??;
            let default_branch = default_branch_request
                .context("No active repository")?
                .await
                .map(Result::ok)
                .ok()
                .flatten()
                .flatten();

            let all_branches = cx
                .background_spawn(async move {
                    let remote_upstreams: HashSet<_> = all_branches
                        .iter()
                        .filter_map(|branch| {
                            branch
                                .upstream
                                .as_ref()
                                .filter(|upstream| upstream.is_remote())
                                .map(|upstream| upstream.ref_name.clone())
                        })
                        .collect();

                    all_branches.retain(|branch| !remote_upstreams.contains(&branch.ref_name));

                    all_branches.sort_by_key(|branch| {
                        (
                            !branch.is_head, // Current branch (is_head=true) comes first
                            branch
                                .most_recent_commit
                                .as_ref()
                                .map(|commit| 0 - commit.commit_timestamp),
                        )
                    });

                    all_branches
                })
                .await;

            let _ = this.update_in(cx, |this, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.default_branch = default_branch;
                    picker.delegate.all_branches = Some(all_branches);
                    picker.refresh(window, cx);
                })
            });

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let delegate = BranchListDelegate::new(workspace, repository, style, cx);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(!embedded));
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
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::new_inner(
            workspace,
            repository,
            BranchListStyle::Modal,
            width,
            true,
            window,
            cx,
        );
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

    pub fn handle_delete(
        &mut self,
        _: &branch_picker::DeleteBranch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .delete_at(picker.delegate.selected_index, window, cx)
        })
    }

    pub fn handle_filter(
        &mut self,
        _: &branch_picker::FilterRemotes,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.branch_filter = picker.delegate.branch_filter.invert();
            picker.update_matches(picker.query(cx), window, cx);
            picker.refresh_placeholder(window, cx);
            cx.notify();
        });
    }
}
impl ModalView for BranchList {}
impl EventEmitter<DismissEvent> for BranchList {}

impl Focusable for BranchList {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.picker_focus_handle.clone()
    }
}

impl Render for BranchList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitBranchSelector")
            .w(self.width)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(Self::handle_delete))
            .on_action(cx.listener(Self::handle_filter))
            .child(self.picker.clone())
            .when(!self.embedded, |this| {
                this.on_mouse_down_out({
                    cx.listener(move |this, _, window, cx| {
                        this.picker.update(cx, |this, cx| {
                            this.cancel(&Default::default(), window, cx);
                        })
                    })
                })
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Entry {
    Branch {
        branch: Branch,
        positions: Vec<usize>,
    },
    NewUrl {
        url: String,
    },
    NewBranch {
        name: String,
    },
    NewRemoteName {
        name: String,
        url: SharedString,
    },
}

impl Entry {
    fn as_branch(&self) -> Option<&Branch> {
        match self {
            Entry::Branch { branch, .. } => Some(branch),
            _ => None,
        }
    }

    fn name(&self) -> &str {
        match self {
            Entry::Branch { branch, .. } => branch.name(),
            Entry::NewUrl { url, .. } => url.as_str(),
            Entry::NewBranch { name, .. } => name.as_str(),
            Entry::NewRemoteName { name, .. } => name.as_str(),
        }
    }

    #[cfg(test)]
    fn is_new_url(&self) -> bool {
        matches!(self, Self::NewUrl { .. })
    }

    #[cfg(test)]
    fn is_new_branch(&self) -> bool {
        matches!(self, Self::NewBranch { .. })
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BranchFilter {
    /// Show both local and remote branches.
    All,
    /// Only show remote branches.
    Remote,
}

impl BranchFilter {
    fn invert(&self) -> Self {
        match self {
            BranchFilter::All => BranchFilter::Remote,
            BranchFilter::Remote => BranchFilter::All,
        }
    }
}

pub struct BranchListDelegate {
    workspace: WeakEntity<Workspace>,
    matches: Vec<Entry>,
    all_branches: Option<Vec<Branch>>,
    default_branch: Option<SharedString>,
    repo: Option<Entity<Repository>>,
    style: BranchListStyle,
    selected_index: usize,
    last_query: String,
    modifiers: Modifiers,
    branch_filter: BranchFilter,
    state: PickerState,
    focus_handle: FocusHandle,
}

#[derive(Debug)]
enum PickerState {
    /// When we display list of branches/remotes
    List,
    /// When we set an url to create a new remote
    NewRemote,
    /// When we confirm the new remote url (after NewRemote)
    CreateRemote(SharedString),
    /// When we set a new branch to create
    NewBranch,
}

impl BranchListDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        repo: Option<Entity<Repository>>,
        style: BranchListStyle,
        cx: &mut Context<BranchList>,
    ) -> Self {
        Self {
            workspace,
            matches: vec![],
            repo,
            style,
            all_branches: None,
            default_branch: None,
            selected_index: 0,
            last_query: Default::default(),
            modifiers: Default::default(),
            branch_filter: BranchFilter::All,
            state: PickerState::List,
            focus_handle: cx.focus_handle(),
        }
    }

    fn create_branch(
        &self,
        from_branch: Option<SharedString>,
        new_branch_name: SharedString,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(repo) = self.repo.clone() else {
            return;
        };
        let new_branch_name = new_branch_name.to_string().replace(' ', "-");
        let base_branch = from_branch.map(|b| b.to_string());
        cx.spawn(async move |_, cx| {
            repo.update(cx, |repo, _| {
                repo.create_branch(new_branch_name, base_branch)
            })
            .await??;

            Ok(())
        })
        .detach_and_prompt_err("Failed to create branch", window, cx, |e, _, _| {
            Some(e.to_string())
        });
        cx.emit(DismissEvent);
    }

    fn create_remote(
        &self,
        remote_name: String,
        remote_url: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(repo) = self.repo.clone() else {
            return;
        };

        let receiver = repo.update(cx, |repo, _| repo.create_remote(remote_name, remote_url));

        cx.background_spawn(async move { receiver.await? })
            .detach_and_prompt_err("Failed to create remote", window, cx, |e, _, _cx| {
                Some(e.to_string())
            });
        cx.emit(DismissEvent);
    }

    fn delete_at(&self, idx: usize, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(idx).cloned() else {
            return;
        };
        let Some(repo) = self.repo.clone() else {
            return;
        };

        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let mut is_remote = false;
            let result = match &entry {
                Entry::Branch { branch, .. } => match branch.remote_name() {
                    Some(remote_name) => {
                        is_remote = true;
                        repo.update(cx, |repo, _| repo.remove_remote(remote_name.to_string()))
                            .await?
                    }
                    None => {
                        repo.update(cx, |repo, _| repo.delete_branch(branch.name().to_string()))
                            .await?
                    }
                },
                _ => {
                    log::error!("Failed to delete remote: wrong entry to delete");
                    return Ok(());
                }
            };

            if let Err(e) = result {
                if is_remote {
                    log::error!("Failed to delete remote: {}", e);
                } else {
                    log::error!("Failed to delete branch: {}", e);
                }

                if let Some(workspace) = workspace.upgrade() {
                    cx.update(|_window, cx| {
                        if is_remote {
                            show_error_toast(
                                workspace,
                                format!("remote remove {}", entry.name()),
                                e,
                                cx,
                            )
                        } else {
                            show_error_toast(
                                workspace,
                                format!("branch -d {}", entry.name()),
                                e,
                                cx,
                            )
                        }
                    })?;
                }

                return Ok(());
            }

            picker.update_in(cx, |picker, _, cx| {
                picker.delegate.matches.retain(|e| e != &entry);

                if let Entry::Branch { branch, .. } = &entry {
                    if let Some(all_branches) = &mut picker.delegate.all_branches {
                        all_branches.retain(|e| e.ref_name != branch.ref_name);
                    }
                }

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
}

impl PickerDelegate for BranchListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        match self.state {
            PickerState::List | PickerState::NewRemote | PickerState::NewBranch => {
                match self.branch_filter {
                    BranchFilter::All => "Select branch or remote…",
                    BranchFilter::Remote => "Select remote…",
                }
            }
            PickerState::CreateRemote(_) => "Enter a name for this remote…",
        }
        .into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        match self.state {
            PickerState::CreateRemote(_) => {
                Some(SharedString::new_static("Remote name can't be empty"))
            }
            _ => None,
        }
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let focus_handle = self.focus_handle.clone();

        v_flex()
            .when(
                self.editor_position() == PickerEditorPosition::End,
                |this| this.child(Divider::horizontal()),
            )
            .child(
                h_flex()
                    .overflow_hidden()
                    .flex_none()
                    .h_9()
                    .px_2p5()
                    .child(editor.clone())
                    .when(
                        self.editor_position() == PickerEditorPosition::End,
                        |this| {
                            let tooltip_label = match self.branch_filter {
                                BranchFilter::All => "Filter Remote Branches",
                                BranchFilter::Remote => "Show All Branches",
                            };

                            this.gap_1().justify_between().child({
                                IconButton::new("filter-remotes", IconName::Filter)
                                    .toggle_state(self.branch_filter == BranchFilter::Remote)
                                    .tooltip(move |_, cx| {
                                        Tooltip::for_action_in(
                                            tooltip_label,
                                            &branch_picker::FilterRemotes,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(|_click, window, cx| {
                                        window.dispatch_action(
                                            branch_picker::FilterRemotes.boxed_clone(),
                                            cx,
                                        );
                                    })
                            })
                        },
                    ),
            )
            .when(
                self.editor_position() == PickerEditorPosition::Start,
                |this| this.child(Divider::horizontal()),
            )
    }

    fn editor_position(&self) -> PickerEditorPosition {
        match self.style {
            BranchListStyle::Modal => PickerEditorPosition::Start,
            BranchListStyle::Popover => PickerEditorPosition::End,
        }
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
        let Some(all_branches) = self.all_branches.clone() else {
            return Task::ready(());
        };

        let branch_filter = self.branch_filter;
        cx.spawn_in(window, async move |picker, cx| {
            let branch_matches_filter = |branch: &Branch| match branch_filter {
                BranchFilter::All => true,
                BranchFilter::Remote => branch.is_remote(),
            };

            let mut matches: Vec<Entry> = if query.is_empty() {
                let mut matches: Vec<Entry> = all_branches
                    .into_iter()
                    .filter(|branch| branch_matches_filter(branch))
                    .map(|branch| Entry::Branch {
                        branch,
                        positions: Vec::new(),
                    })
                    .collect();

                // Keep the existing recency sort within each group, but show local branches first.
                matches.sort_by_key(|entry| entry.as_branch().is_some_and(|b| b.is_remote()));

                matches
            } else {
                let branches = all_branches
                    .iter()
                    .filter(|branch| branch_matches_filter(branch))
                    .collect::<Vec<_>>();
                let candidates = branches
                    .iter()
                    .enumerate()
                    .map(|(ix, branch)| StringMatchCandidate::new(ix, branch.name()))
                    .collect::<Vec<StringMatchCandidate>>();
                let mut matches: Vec<Entry> = fuzzy::match_strings(
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
                .map(|candidate| Entry::Branch {
                    branch: branches[candidate.candidate_id].clone(),
                    positions: candidate.positions,
                })
                .collect();

                // Keep fuzzy-relevance ordering within local/remote groups, but show locals first.
                matches.sort_by_key(|entry| entry.as_branch().is_some_and(|b| b.is_remote()));

                matches
            };
            picker
                .update(cx, |picker, _| {
                    if let PickerState::CreateRemote(url) = &picker.delegate.state {
                        let query = query.replace(' ', "-");
                        if !query.is_empty() {
                            picker.delegate.matches = vec![Entry::NewRemoteName {
                                name: query.clone(),
                                url: url.clone(),
                            }];
                            picker.delegate.selected_index = 0;
                        } else {
                            picker.delegate.matches = Vec::new();
                            picker.delegate.selected_index = 0;
                        }
                        picker.delegate.last_query = query;
                        return;
                    }

                    if !query.is_empty()
                        && !matches.first().is_some_and(|entry| entry.name() == query)
                    {
                        let query = query.replace(' ', "-");
                        let is_url = query.trim_start_matches("git@").parse::<Url>().is_ok();
                        let entry = if is_url {
                            Entry::NewUrl { url: query }
                        } else {
                            Entry::NewBranch { name: query }
                        };
                        // Only transition to NewBranch/NewRemote states when we only show their list item
                        // Otherwise, stay in List state so footer buttons remain visible
                        picker.delegate.state = if matches.is_empty() {
                            if is_url {
                                PickerState::NewRemote
                            } else {
                                PickerState::NewBranch
                            }
                        } else {
                            PickerState::List
                        };
                        matches.push(entry);
                    } else {
                        picker.delegate.state = PickerState::List;
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

        match entry {
            Entry::Branch { branch, .. } => {
                let current_branch = self.repo.as_ref().map(|repo| {
                    repo.read_with(cx, |repo, _| {
                        repo.branch.as_ref().map(|branch| branch.ref_name.clone())
                    })
                });

                if current_branch
                    .flatten()
                    .is_some_and(|current_branch| current_branch == branch.ref_name)
                {
                    cx.emit(DismissEvent);
                    return;
                }

                let Some(repo) = self.repo.clone() else {
                    return;
                };

                let branch = branch.clone();
                cx.spawn(async move |_, cx| {
                    repo.update(cx, |repo, _| repo.change_branch(branch.name().to_string()))
                        .await??;

                    anyhow::Ok(())
                })
                .detach_and_prompt_err(
                    "Failed to change branch",
                    window,
                    cx,
                    |_, _, _| None,
                );
            }
            Entry::NewUrl { url } => {
                self.state = PickerState::CreateRemote(url.clone().into());
                self.matches = Vec::new();
                self.selected_index = 0;

                cx.defer_in(window, |picker, window, cx| {
                    picker.refresh_placeholder(window, cx);
                    picker.set_query("", window, cx);
                    cx.notify();
                });

                // returning early to prevent dismissing the modal, so a user can enter
                // a remote name first.
                return;
            }
            Entry::NewRemoteName { name, url } => {
                self.create_remote(name.clone(), url.to_string(), window, cx);
            }
            Entry::NewBranch { name } => {
                let from_branch = if secondary {
                    self.default_branch.clone()
                } else {
                    None
                };
                self.create_branch(from_branch, name.into(), window, cx);
            }
        }

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.state = PickerState::List;
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

        let (commit_time, author_name, subject) = entry
            .as_branch()
            .and_then(|branch| {
                branch.most_recent_commit.as_ref().map(|commit| {
                    let subject = commit.subject.clone();
                    let commit_time = OffsetDateTime::from_unix_timestamp(commit.commit_timestamp)
                        .unwrap_or_else(|_| OffsetDateTime::now_utc());
                    let local_offset =
                        time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
                    let formatted_time = time_format::format_localized_timestamp(
                        commit_time,
                        OffsetDateTime::now_utc(),
                        local_offset,
                        time_format::TimestampFormat::Relative,
                    );
                    let author = commit.author_name.clone();
                    (Some(formatted_time), Some(author), Some(subject))
                })
            })
            .unwrap_or_else(|| (None, None, None));

        let entry_icon = match entry {
            Entry::NewUrl { .. } | Entry::NewBranch { .. } | Entry::NewRemoteName { .. } => {
                Icon::new(IconName::Plus).color(Color::Muted)
            }
            Entry::Branch { branch, .. } => {
                if branch.is_remote() {
                    Icon::new(IconName::Screen).color(Color::Muted)
                } else {
                    Icon::new(IconName::GitBranchAlt).color(Color::Muted)
                }
            }
        };

        let entry_title = match entry {
            Entry::NewUrl { .. } => Label::new("Create Remote Repository")
                .single_line()
                .truncate()
                .into_any_element(),
            Entry::NewBranch { name } => Label::new(format!("Create Branch: \"{name}\"…"))
                .single_line()
                .truncate()
                .into_any_element(),
            Entry::NewRemoteName { name, .. } => Label::new(format!("Create Remote: \"{name}\""))
                .single_line()
                .truncate()
                .into_any_element(),
            Entry::Branch { branch, positions } => {
                HighlightedLabel::new(branch.name().to_string(), positions.clone())
                    .single_line()
                    .truncate()
                    .into_any_element()
            }
        };

        let focus_handle = self.focus_handle.clone();
        let is_new_items = matches!(
            entry,
            Entry::NewUrl { .. } | Entry::NewBranch { .. } | Entry::NewRemoteName { .. }
        );

        let deleted_branch_icon = |entry_ix: usize, is_head_branch: bool| {
            IconButton::new(("delete", entry_ix), IconName::Trash)
                .tooltip(move |_, cx| {
                    Tooltip::for_action_in(
                        "Delete Branch",
                        &branch_picker::DeleteBranch,
                        &focus_handle,
                        cx,
                    )
                })
                .disabled(is_head_branch)
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.delegate.delete_at(entry_ix, window, cx);
                }))
        };

        let create_from_default_button = self.default_branch.as_ref().map(|default_branch| {
            let tooltip_label: SharedString = format!("Create New From: {default_branch}").into();
            let focus_handle = self.focus_handle.clone();

            IconButton::new("create_from_default", IconName::GitBranchPlus)
                .tooltip(move |_, cx| {
                    Tooltip::for_action_in(
                        tooltip_label.clone(),
                        &menu::SecondaryConfirm,
                        &focus_handle,
                        cx,
                    )
                })
                .on_click(cx.listener(|this, _, window, cx| {
                    this.delegate.confirm(true, window, cx);
                }))
                .into_any_element()
        });

        Some(
            ListItem::new(format!("vcs-menu-{ix}"))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .gap_3()
                        .flex_grow()
                        .child(entry_icon)
                        .child(
                            v_flex()
                                .id("info_container")
                                .w_full()
                                .child(entry_title)
                                .child(
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .gap_1p5()
                                        .when(self.style == BranchListStyle::Modal, |el| {
                                            el.child(div().max_w_96().child({
                                                let message = match entry {
                                                    Entry::NewUrl { url } => {
                                                        format!("Based off {url}")
                                                    }
                                                    Entry::NewRemoteName { url, .. } => {
                                                        format!("Based off {url}")
                                                    }
                                                    Entry::NewBranch { .. } => {
                                                        if let Some(current_branch) =
                                                            self.repo.as_ref().and_then(|repo| {
                                                                repo.read(cx)
                                                                    .branch
                                                                    .as_ref()
                                                                    .map(|b| b.name())
                                                            })
                                                        {
                                                            format!("Based off {}", current_branch)
                                                        } else {
                                                            "Based off the current branch"
                                                                .to_string()
                                                        }
                                                    }
                                                    Entry::Branch { .. } => {
                                                        let show_author_name =
                                                            ProjectSettings::get_global(cx)
                                                                .git
                                                                .branch_picker
                                                                .show_author_name;

                                                        subject.map_or(
                                                            "No commits found".into(),
                                                            |subject| {
                                                                if show_author_name
                                                                    && let Some(author) =
                                                                        author_name
                                                                {
                                                                    format!(
                                                                        "{}  •  {}",
                                                                        author, subject
                                                                    )
                                                                } else {
                                                                    subject.to_string()
                                                                }
                                                            },
                                                        )
                                                    }
                                                };

                                                Label::new(message)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted)
                                                    .truncate()
                                            }))
                                        })
                                        .when_some(commit_time, |label, commit_time| {
                                            label.child(
                                                Label::new(commit_time)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            )
                                        }),
                                )
                                .when_some(
                                    entry.as_branch().map(|b| b.name().to_string()),
                                    |this, branch_name| this.tooltip(Tooltip::text(branch_name)),
                                ),
                        ),
                )
                .when(
                    self.editor_position() == PickerEditorPosition::End && !is_new_items,
                    |this| {
                        this.map(|this| {
                            let is_head_branch =
                                entry.as_branch().is_some_and(|branch| branch.is_head);
                            if self.selected_index() == ix {
                                this.end_slot(deleted_branch_icon(ix, is_head_branch))
                            } else {
                                this.end_hover_slot(deleted_branch_icon(ix, is_head_branch))
                            }
                        })
                    },
                )
                .when_some(
                    if self.editor_position() == PickerEditorPosition::End && is_new_items {
                        create_from_default_button
                    } else {
                        None
                    },
                    |this, create_from_default_button| {
                        this.map(|this| {
                            if self.selected_index() == ix {
                                this.end_slot(create_from_default_button)
                            } else {
                                this.end_hover_slot(create_from_default_button)
                            }
                        })
                    },
                ),
        )
    }

    fn render_header(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        matches!(self.state, PickerState::List).then(|| {
            let label = match self.branch_filter {
                BranchFilter::All => "Branches",
                BranchFilter::Remote => "Remotes",
            };

            ListHeader::new(label).inset(true).into_any_element()
        })
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        if self.editor_position() == PickerEditorPosition::End {
            return None;
        }
        let focus_handle = self.focus_handle.clone();

        let footer_container = || {
            h_flex()
                .w_full()
                .p_1p5()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
        };

        match self.state {
            PickerState::List => {
                let selected_entry = self.matches.get(self.selected_index);

                let branch_from_default_button = self
                    .default_branch
                    .as_ref()
                    .filter(|_| matches!(selected_entry, Some(Entry::NewBranch { .. })))
                    .map(|default_branch| {
                        let button_label = format!("Create New From: {default_branch}");

                        Button::new("branch-from-default", button_label)
                            .key_binding(
                                KeyBinding::for_action_in(
                                    &menu::SecondaryConfirm,
                                    &focus_handle,
                                    cx,
                                )
                                .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.delegate.confirm(true, window, cx);
                            }))
                    });

                let delete_and_select_btns = h_flex()
                    .gap_1()
                    .child(
                        Button::new("delete-branch", "Delete")
                            .key_binding(
                                KeyBinding::for_action_in(
                                    &branch_picker::DeleteBranch,
                                    &focus_handle,
                                    cx,
                                )
                                .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(|_, window, cx| {
                                window
                                    .dispatch_action(branch_picker::DeleteBranch.boxed_clone(), cx);
                            }),
                    )
                    .child(
                        Button::new("select_branch", "Select")
                            .key_binding(
                                KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                    .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.delegate.confirm(false, window, cx);
                            })),
                    );

                Some(
                    footer_container()
                        .map(|this| {
                            if branch_from_default_button.is_some() {
                                this.justify_end().when_some(
                                    branch_from_default_button,
                                    |this, button| {
                                        this.child(button).child(
                                            Button::new("create", "Create")
                                                .key_binding(
                                                    KeyBinding::for_action_in(
                                                        &menu::Confirm,
                                                        &focus_handle,
                                                        cx,
                                                    )
                                                    .map(|kb| kb.size(rems_from_px(12.))),
                                                )
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.delegate.confirm(false, window, cx);
                                                })),
                                        )
                                    },
                                )
                            } else {
                                this.justify_between()
                                    .child({
                                        let focus_handle = focus_handle.clone();
                                        Button::new("filter-remotes", "Filter Remotes")
                                            .toggle_state(matches!(
                                                self.branch_filter,
                                                BranchFilter::Remote
                                            ))
                                            .key_binding(
                                                KeyBinding::for_action_in(
                                                    &branch_picker::FilterRemotes,
                                                    &focus_handle,
                                                    cx,
                                                )
                                                .map(|kb| kb.size(rems_from_px(12.))),
                                            )
                                            .on_click(|_click, window, cx| {
                                                window.dispatch_action(
                                                    branch_picker::FilterRemotes.boxed_clone(),
                                                    cx,
                                                );
                                            })
                                    })
                                    .child(delete_and_select_btns)
                            }
                        })
                        .into_any_element(),
                )
            }
            PickerState::NewBranch => {
                let branch_from_default_button =
                    self.default_branch.as_ref().map(|default_branch| {
                        let button_label = format!("Create New From: {default_branch}");

                        Button::new("branch-from-default", button_label)
                            .key_binding(
                                KeyBinding::for_action_in(
                                    &menu::SecondaryConfirm,
                                    &focus_handle,
                                    cx,
                                )
                                .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.delegate.confirm(true, window, cx);
                            }))
                    });

                Some(
                    footer_container()
                        .gap_1()
                        .justify_end()
                        .when_some(branch_from_default_button, |this, button| {
                            this.child(button)
                        })
                        .child(
                            Button::new("branch-from-default", "Create")
                                .key_binding(
                                    KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.delegate.confirm(false, window, cx);
                                })),
                        )
                        .into_any_element(),
                )
            }
            PickerState::CreateRemote(_) => Some(
                footer_container()
                    .justify_end()
                    .child(
                        Button::new("branch-from-default", "Confirm")
                            .key_binding(
                                KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                    .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.delegate.confirm(false, window, cx);
                            }))
                            .disabled(self.last_query.is_empty()),
                    )
                    .into_any_element(),
            ),
            PickerState::NewRemote => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use git::repository::{CommitSummary, Remote};
    use gpui::{AppContext, TestAppContext, VisualTestContext};
    use project::{FakeFs, Project};
    use rand::{Rng, rngs::StdRng};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }

    fn create_test_branch(
        name: &str,
        is_head: bool,
        remote_name: Option<&str>,
        timestamp: Option<i64>,
    ) -> Branch {
        let ref_name = match remote_name {
            Some(remote_name) => format!("refs/remotes/{remote_name}/{name}"),
            None => format!("refs/heads/{name}"),
        };

        Branch {
            is_head,
            ref_name: ref_name.into(),
            upstream: None,
            most_recent_commit: timestamp.map(|ts| CommitSummary {
                sha: "abc123".into(),
                commit_timestamp: ts,
                author_name: "Test Author".into(),
                subject: "Test commit".into(),
                has_parent: true,
            }),
        }
    }

    fn create_test_branches() -> Vec<Branch> {
        vec![
            create_test_branch("main", true, None, Some(1000)),
            create_test_branch("feature-auth", false, None, Some(900)),
            create_test_branch("feature-ui", false, None, Some(800)),
            create_test_branch("develop", false, None, Some(700)),
        ]
    }

    async fn init_branch_list_test(
        repository: Option<Entity<Repository>>,
        branches: Vec<Branch>,
        cx: &mut TestAppContext,
    ) -> (Entity<BranchList>, VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let workspace = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));

        let branch_list = workspace
            .update(cx, |workspace, window, cx| {
                cx.new(|cx| {
                    let mut delegate = BranchListDelegate::new(
                        workspace.weak_handle(),
                        repository,
                        BranchListStyle::Modal,
                        cx,
                    );
                    delegate.all_branches = Some(branches);
                    let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
                    let picker_focus_handle = picker.focus_handle(cx);
                    picker.update(cx, |picker, _| {
                        picker.delegate.focus_handle = picker_focus_handle.clone();
                    });

                    let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
                        cx.emit(DismissEvent);
                    });

                    BranchList {
                        picker,
                        picker_focus_handle,
                        width: rems(34.),
                        _subscription: Some(_subscription),
                        embedded: false,
                    }
                })
            })
            .unwrap();

        let cx = VisualTestContext::from_window(*workspace, cx);

        (branch_list, cx)
    }

    async fn init_fake_repository(cx: &mut TestAppContext) -> Entity<Repository> {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                ".git": {},
                "file.txt": "buffer_text".to_string()
            }),
        )
        .await;
        fs.set_head_for_repo(
            path!("/dir/.git").as_ref(),
            &[("file.txt", "test".to_string())],
            "deadbeef",
        );
        fs.set_index_for_repo(
            path!("/dir/.git").as_ref(),
            &[("file.txt", "index_text".to_string())],
        );

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let repository = cx.read(|cx| project.read(cx).active_repository(cx));

        repository.unwrap()
    }

    #[gpui::test]
    async fn test_update_branch_matches_with_query(cx: &mut TestAppContext) {
        init_test(cx);

        let branches = create_test_branches();
        let (branch_list, mut ctx) = init_branch_list_test(None, branches, cx).await;
        let cx = &mut ctx;

        branch_list
            .update_in(cx, |branch_list, window, cx| {
                let query = "feature".to_string();
                branch_list.picker.update(cx, |picker, cx| {
                    picker.delegate.update_matches(query, window, cx)
                })
            })
            .await;
        cx.run_until_parked();

        branch_list.update(cx, |branch_list, cx| {
            branch_list.picker.update(cx, |picker, _cx| {
                // Should have 2 existing branches + 1 "create new branch" entry = 3 total
                assert_eq!(picker.delegate.matches.len(), 3);
                assert!(
                    picker
                        .delegate
                        .matches
                        .iter()
                        .any(|m| m.name() == "feature-auth")
                );
                assert!(
                    picker
                        .delegate
                        .matches
                        .iter()
                        .any(|m| m.name() == "feature-ui")
                );
                // Verify the last entry is the "create new branch" option
                let last_match = picker.delegate.matches.last().unwrap();
                assert!(last_match.is_new_branch());
            })
        });
    }

    async fn update_branch_list_matches_with_empty_query(
        branch_list: &Entity<BranchList>,
        cx: &mut VisualTestContext,
    ) {
        branch_list
            .update_in(cx, |branch_list, window, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    picker.delegate.update_matches(String::new(), window, cx)
                })
            })
            .await;
        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_delete_branch(cx: &mut TestAppContext) {
        init_test(cx);
        let repository = init_fake_repository(cx).await;

        let branches = create_test_branches();

        let branch_names = branches
            .iter()
            .map(|branch| branch.name().to_string())
            .collect::<Vec<String>>();
        let repo = repository.clone();
        cx.spawn(async move |mut cx| {
            for branch in branch_names {
                repo.update(&mut cx, |repo, _| repo.create_branch(branch, None))
                    .await
                    .unwrap()
                    .unwrap();
            }
        })
        .await;
        cx.run_until_parked();

        let (branch_list, mut ctx) = init_branch_list_test(repository.into(), branches, cx).await;
        let cx = &mut ctx;

        update_branch_list_matches_with_empty_query(&branch_list, cx).await;

        let branch_to_delete = branch_list.update_in(cx, |branch_list, window, cx| {
            branch_list.picker.update(cx, |picker, cx| {
                assert_eq!(picker.delegate.matches.len(), 4);
                let branch_to_delete = picker.delegate.matches.get(1).unwrap().name().to_string();
                picker.delegate.delete_at(1, window, cx);
                branch_to_delete
            })
        });
        cx.run_until_parked();

        branch_list.update(cx, move |branch_list, cx| {
            branch_list.picker.update(cx, move |picker, _cx| {
                assert_eq!(picker.delegate.matches.len(), 3);
                let branches = picker
                    .delegate
                    .matches
                    .iter()
                    .map(|be| be.name())
                    .collect::<HashSet<_>>();
                assert_eq!(
                    branches,
                    ["main", "feature-auth", "feature-ui", "develop"]
                        .into_iter()
                        .filter(|name| name != &branch_to_delete)
                        .collect::<HashSet<_>>()
                );
            })
        });
    }

    #[gpui::test]
    async fn test_delete_remote(cx: &mut TestAppContext) {
        init_test(cx);
        let repository = init_fake_repository(cx).await;
        let branches = vec![
            create_test_branch("main", true, Some("origin"), Some(1000)),
            create_test_branch("feature-auth", false, Some("origin"), Some(900)),
            create_test_branch("feature-ui", false, Some("fork"), Some(800)),
            create_test_branch("develop", false, Some("private"), Some(700)),
        ];

        let remote_names = branches
            .iter()
            .filter_map(|branch| branch.remote_name().map(|r| r.to_string()))
            .collect::<Vec<String>>();
        let repo = repository.clone();
        cx.spawn(async move |mut cx| {
            for branch in remote_names {
                repo.update(&mut cx, |repo, _| {
                    repo.create_remote(branch, String::from("test"))
                })
                .await
                .unwrap()
                .unwrap();
            }
        })
        .await;
        cx.run_until_parked();

        let (branch_list, mut ctx) = init_branch_list_test(repository.into(), branches, cx).await;
        let cx = &mut ctx;
        // Enable remote filter
        branch_list.update(cx, |branch_list, cx| {
            branch_list.picker.update(cx, |picker, _cx| {
                picker.delegate.branch_filter = BranchFilter::Remote;
            });
        });
        update_branch_list_matches_with_empty_query(&branch_list, cx).await;

        // Check matches, it should match all existing branches and no option to create new branch
        let branch_to_delete = branch_list.update_in(cx, |branch_list, window, cx| {
            branch_list.picker.update(cx, |picker, cx| {
                assert_eq!(picker.delegate.matches.len(), 4);
                let branch_to_delete = picker.delegate.matches.get(1).unwrap().name().to_string();
                picker.delegate.delete_at(1, window, cx);
                branch_to_delete
            })
        });
        cx.run_until_parked();

        // Check matches, it should match one less branch than before
        branch_list.update(cx, move |branch_list, cx| {
            branch_list.picker.update(cx, move |picker, _cx| {
                assert_eq!(picker.delegate.matches.len(), 3);
                let branches = picker
                    .delegate
                    .matches
                    .iter()
                    .map(|be| be.name())
                    .collect::<HashSet<_>>();
                assert_eq!(
                    branches,
                    [
                        "origin/main",
                        "origin/feature-auth",
                        "fork/feature-ui",
                        "private/develop"
                    ]
                    .into_iter()
                    .filter(|name| name != &branch_to_delete)
                    .collect::<HashSet<_>>()
                );
            })
        });
    }

    #[gpui::test]
    async fn test_branch_filter_shows_all_then_remotes_and_applies_query(cx: &mut TestAppContext) {
        init_test(cx);

        let branches = vec![
            create_test_branch("main", true, Some("origin"), Some(1000)),
            create_test_branch("feature-auth", false, Some("fork"), Some(900)),
            create_test_branch("feature-ui", false, None, Some(800)),
            create_test_branch("develop", false, None, Some(700)),
        ];

        let (branch_list, mut ctx) = init_branch_list_test(None, branches, cx).await;
        let cx = &mut ctx;

        update_branch_list_matches_with_empty_query(&branch_list, cx).await;

        branch_list.update(cx, |branch_list, cx| {
            branch_list.picker.update(cx, |picker, _cx| {
                assert_eq!(picker.delegate.matches.len(), 4);

                let branches = picker
                    .delegate
                    .matches
                    .iter()
                    .map(|be| be.name())
                    .collect::<HashSet<_>>();
                assert_eq!(
                    branches,
                    ["origin/main", "fork/feature-auth", "feature-ui", "develop"]
                        .into_iter()
                        .collect::<HashSet<_>>()
                );

                // Locals should be listed before remotes.
                let ordered = picker
                    .delegate
                    .matches
                    .iter()
                    .map(|be| be.name())
                    .collect::<Vec<_>>();
                assert_eq!(
                    ordered,
                    vec!["feature-ui", "develop", "origin/main", "fork/feature-auth"]
                );

                // Verify the last entry is NOT the "create new branch" option
                let last_match = picker.delegate.matches.last().unwrap();
                assert!(!last_match.is_new_branch());
                assert!(!last_match.is_new_url());
            })
        });

        branch_list.update(cx, |branch_list, cx| {
            branch_list.picker.update(cx, |picker, _cx| {
                picker.delegate.branch_filter = BranchFilter::Remote;
            })
        });

        update_branch_list_matches_with_empty_query(&branch_list, cx).await;

        branch_list
            .update_in(cx, |branch_list, window, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    assert_eq!(picker.delegate.matches.len(), 2);
                    let branches = picker
                        .delegate
                        .matches
                        .iter()
                        .map(|be| be.name())
                        .collect::<HashSet<_>>();
                    assert_eq!(
                        branches,
                        ["origin/main", "fork/feature-auth"]
                            .into_iter()
                            .collect::<HashSet<_>>()
                    );

                    // Verify the last entry is NOT the "create new branch" option
                    let last_match = picker.delegate.matches.last().unwrap();
                    assert!(!last_match.is_new_url());
                    picker.delegate.branch_filter = BranchFilter::Remote;
                    picker
                        .delegate
                        .update_matches(String::from("fork"), window, cx)
                })
            })
            .await;
        cx.run_until_parked();

        branch_list.update(cx, |branch_list, cx| {
            branch_list.picker.update(cx, |picker, _cx| {
                // Should have 1 existing branch + 1 "create new branch" entry = 2 total
                assert_eq!(picker.delegate.matches.len(), 2);
                assert!(
                    picker
                        .delegate
                        .matches
                        .iter()
                        .any(|m| m.name() == "fork/feature-auth")
                );
                // Verify the last entry is the "create new branch" option
                let last_match = picker.delegate.matches.last().unwrap();
                assert!(last_match.is_new_branch());
            })
        });
    }

    #[gpui::test]
    async fn test_new_branch_creation_with_query(test_cx: &mut TestAppContext) {
        const MAIN_BRANCH: &str = "main";
        const FEATURE_BRANCH: &str = "feature";
        const NEW_BRANCH: &str = "new-feature-branch";

        init_test(test_cx);
        let repository = init_fake_repository(test_cx).await;

        let branches = vec![
            create_test_branch(MAIN_BRANCH, true, None, Some(1000)),
            create_test_branch(FEATURE_BRANCH, false, None, Some(900)),
        ];

        let (branch_list, mut ctx) =
            init_branch_list_test(repository.into(), branches, test_cx).await;
        let cx = &mut ctx;

        branch_list
            .update_in(cx, |branch_list, window, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .update_matches(NEW_BRANCH.to_string(), window, cx)
                })
            })
            .await;

        cx.run_until_parked();

        branch_list.update_in(cx, |branch_list, window, cx| {
            branch_list.picker.update(cx, |picker, cx| {
                let last_match = picker.delegate.matches.last().unwrap();
                assert!(last_match.is_new_branch());
                assert_eq!(last_match.name(), NEW_BRANCH);
                // State is NewBranch because no existing branches fuzzy-match the query
                assert!(matches!(picker.delegate.state, PickerState::NewBranch));
                picker.delegate.confirm(false, window, cx);
            })
        });
        cx.run_until_parked();

        let branches = branch_list
            .update(cx, |branch_list, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .repo
                        .as_ref()
                        .unwrap()
                        .update(cx, |repo, _cx| repo.branches())
                })
            })
            .await
            .unwrap()
            .unwrap();

        let new_branch = branches
            .into_iter()
            .find(|branch| branch.name() == NEW_BRANCH)
            .expect("new-feature-branch should exist");
        assert_eq!(
            new_branch.ref_name.as_ref(),
            &format!("refs/heads/{NEW_BRANCH}"),
            "branch ref_name should not have duplicate refs/heads/ prefix"
        );
    }

    #[gpui::test]
    async fn test_remote_url_detection_https(cx: &mut TestAppContext) {
        init_test(cx);
        let repository = init_fake_repository(cx).await;
        let branches = vec![create_test_branch("main", true, None, Some(1000))];

        let (branch_list, mut ctx) = init_branch_list_test(repository.into(), branches, cx).await;
        let cx = &mut ctx;

        branch_list
            .update_in(cx, |branch_list, window, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    let query = "https://github.com/user/repo.git".to_string();
                    picker.delegate.update_matches(query, window, cx)
                })
            })
            .await;

        cx.run_until_parked();

        branch_list
            .update_in(cx, |branch_list, window, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    let last_match = picker.delegate.matches.last().unwrap();
                    assert!(last_match.is_new_url());
                    assert!(matches!(picker.delegate.state, PickerState::NewRemote));
                    picker.delegate.confirm(false, window, cx);
                    assert_eq!(picker.delegate.matches.len(), 0);
                    if let PickerState::CreateRemote(remote_url) = &picker.delegate.state
                        && remote_url.as_ref() == "https://github.com/user/repo.git"
                    {
                    } else {
                        panic!("wrong picker state");
                    }
                    picker
                        .delegate
                        .update_matches("my_new_remote".to_string(), window, cx)
                })
            })
            .await;

        cx.run_until_parked();

        branch_list.update_in(cx, |branch_list, window, cx| {
            branch_list.picker.update(cx, |picker, cx| {
                assert_eq!(picker.delegate.matches.len(), 1);
                assert!(matches!(
                    picker.delegate.matches.first(),
                    Some(Entry::NewRemoteName { name, url })
                        if name == "my_new_remote" && url.as_ref() == "https://github.com/user/repo.git"
                ));
                picker.delegate.confirm(false, window, cx);
            })
        });
        cx.run_until_parked();

        // List remotes
        let remotes = branch_list
            .update(cx, |branch_list, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .repo
                        .as_ref()
                        .unwrap()
                        .update(cx, |repo, _cx| repo.get_remotes(None, false))
                })
            })
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            remotes,
            vec![Remote {
                name: SharedString::from("my_new_remote".to_string())
            }]
        );
    }

    #[gpui::test]
    async fn test_confirm_remote_url_transitions(cx: &mut TestAppContext) {
        init_test(cx);

        let branches = vec![create_test_branch("main_branch", true, None, Some(1000))];
        let (branch_list, mut ctx) = init_branch_list_test(None, branches, cx).await;
        let cx = &mut ctx;

        branch_list
            .update_in(cx, |branch_list, window, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    let query = "https://github.com/user/repo.git".to_string();
                    picker.delegate.update_matches(query, window, cx)
                })
            })
            .await;
        cx.run_until_parked();

        // Try to create a new remote but cancel in the middle of the process
        branch_list
            .update_in(cx, |branch_list, window, cx| {
                branch_list.picker.update(cx, |picker, cx| {
                    picker.delegate.selected_index = picker.delegate.matches.len() - 1;
                    picker.delegate.confirm(false, window, cx);

                    assert!(matches!(
                        picker.delegate.state,
                        PickerState::CreateRemote(_)
                    ));
                    if let PickerState::CreateRemote(ref url) = picker.delegate.state {
                        assert_eq!(url.as_ref(), "https://github.com/user/repo.git");
                    }
                    assert_eq!(picker.delegate.matches.len(), 0);
                    picker.delegate.dismissed(window, cx);
                    assert!(matches!(picker.delegate.state, PickerState::List));
                    let query = "main".to_string();
                    picker.delegate.update_matches(query, window, cx)
                })
            })
            .await;
        cx.run_until_parked();

        // Try to search a branch again to see if the state is restored properly
        branch_list.update(cx, |branch_list, cx| {
            branch_list.picker.update(cx, |picker, _cx| {
                // Should have 1 existing branch + 1 "create new branch" entry = 2 total
                assert_eq!(picker.delegate.matches.len(), 2);
                assert!(
                    picker
                        .delegate
                        .matches
                        .iter()
                        .any(|m| m.name() == "main_branch")
                );
                // Verify the last entry is the "create new branch" option
                let last_match = picker.delegate.matches.last().unwrap();
                assert!(last_match.is_new_branch());
            })
        });
    }

    #[gpui::test]
    async fn test_confirm_remote_url_does_not_dismiss(cx: &mut TestAppContext) {
        const REMOTE_URL: &str = "https://github.com/user/repo.git";

        init_test(cx);
        let branches = vec![create_test_branch("main", true, None, Some(1000))];

        let (branch_list, mut ctx) = init_branch_list_test(None, branches, cx).await;
        let cx = &mut ctx;

        let subscription = cx.update(|_, cx| {
            cx.subscribe(&branch_list, |_, _: &DismissEvent, _| {
                panic!("DismissEvent should not be emitted when confirming a remote URL");
            })
        });

        branch_list
            .update_in(cx, |branch_list, window, cx| {
                window.focus(&branch_list.picker_focus_handle, cx);
                assert!(
                    branch_list.picker_focus_handle.is_focused(window),
                    "Branch picker should be focused when selecting an entry"
                );

                branch_list.picker.update(cx, |picker, cx| {
                    picker
                        .delegate
                        .update_matches(REMOTE_URL.to_string(), window, cx)
                })
            })
            .await;

        cx.run_until_parked();

        branch_list.update_in(cx, |branch_list, window, cx| {
            // Re-focus the picker since workspace initialization during run_until_parked
            window.focus(&branch_list.picker_focus_handle, cx);

            branch_list.picker.update(cx, |picker, cx| {
                let last_match = picker.delegate.matches.last().unwrap();
                assert!(last_match.is_new_url());
                assert!(matches!(picker.delegate.state, PickerState::NewRemote));

                picker.delegate.confirm(false, window, cx);

                assert!(
                    matches!(picker.delegate.state, PickerState::CreateRemote(ref url) if url.as_ref() == REMOTE_URL),
                    "State should transition to CreateRemote with the URL"
                );
            });

            assert!(
                branch_list.picker_focus_handle.is_focused(window),
                "Branch list picker should still be focused after confirming remote URL"
            );
        });

        cx.run_until_parked();

        drop(subscription);
    }

    #[gpui::test(iterations = 10)]
    async fn test_empty_query_displays_all_branches(mut rng: StdRng, cx: &mut TestAppContext) {
        init_test(cx);
        let branch_count = rng.random_range(13..540);

        let branches: Vec<Branch> = (0..branch_count)
            .map(|i| create_test_branch(&format!("branch-{:02}", i), i == 0, None, Some(i * 100)))
            .collect();

        let (branch_list, mut ctx) = init_branch_list_test(None, branches, cx).await;
        let cx = &mut ctx;

        update_branch_list_matches_with_empty_query(&branch_list, cx).await;

        branch_list.update(cx, |branch_list, cx| {
            branch_list.picker.update(cx, |picker, _cx| {
                assert_eq!(picker.delegate.matches.len(), branch_count as usize);
            })
        });
    }
}
