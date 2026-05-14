use anyhow::Context as _;
use editor::Editor;
use fuzzy_nucleo::StringMatchCandidate;

use collections::HashSet;
use git::repository::{Branch, delete_branch_flag};
use gpui::http_client::Url;
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, PromptLevel,
    Render, SharedString, Styled, Subscription, Task, TaskExt, WeakEntity, Window, actions, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::git_store::{Repository, RepositoryEvent};
use project::project_settings::ProjectSettings;
use settings::Settings;
use std::sync::Arc;
use time::OffsetDateTime;
use ui::{Divider, HighlightedLabel, KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*};
use ui_input::ErasedEditor;
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

use crate::{branch_picker, git_panel::show_error_toast};

actions!(
    branch_picker,
    [
        /// Deletes the selected git branch or remote.
        DeleteBranch,
        /// Force deletes the selected git branch or remote.
        ForceDeleteBranch,
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
    let repository = workspace.project().read(cx).active_repository(cx);

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

pub fn select_popover(
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    selected_branch: Option<SharedString>,
    on_select: SelectBranchCallback,
    window: &mut Window,
    cx: &mut App,
) -> Entity<BranchList> {
    cx.new(|cx| {
        let list = BranchList::new_select(
            workspace,
            repository,
            BranchListStyle::Popover,
            rems(20.),
            selected_branch,
            on_select,
            window,
            cx,
        );
        list.focus_handle(cx).focus(window, cx);
        list
    })
}

pub type SelectBranchCallback = Arc<dyn Fn(Branch, &mut App)>;

pub fn create_embedded(
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    width: Rems,
    show_footer: bool,
    window: &mut Window,
    cx: &mut Context<BranchList>,
) -> BranchList {
    BranchList::new_embedded(workspace, repository, width, show_footer, window, cx)
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
    _subscriptions: Vec<Subscription>,
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
        this._subscriptions
            .push(cx.subscribe(&this.picker, |_, _, _, cx| {
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
        Self::new_inner_with_behavior(
            workspace,
            repository,
            style,
            width,
            embedded,
            BranchSelectionBehavior::Checkout,
            window,
            cx,
        )
    }

    fn new_select(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        style: BranchListStyle,
        width: Rems,
        selected_branch: Option<SharedString>,
        on_select: SelectBranchCallback,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::new_inner_with_behavior(
            workspace,
            repository,
            style,
            width,
            false,
            BranchSelectionBehavior::Select {
                selected_branch,
                on_select,
            },
            window,
            cx,
        );
        this._subscriptions
            .push(cx.subscribe(&this.picker, |_, _, _, cx| {
                cx.emit(DismissEvent);
            }));
        this
    }

    fn new_inner_with_behavior(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        style: BranchListStyle,
        width: Rems,
        embedded: bool,
        branch_selection_behavior: BranchSelectionBehavior,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let all_branches = repository
            .as_ref()
            .map(|repo| {
                process_branches(
                    &repo.read(cx).branch_list,
                    branch_selection_behavior.selected_branch(),
                )
            })
            .unwrap_or_default();

        let default_branch_request = repository.clone().map(|repository| {
            repository.update(cx, |repository, _| repository.default_branch(false))
        });

        let mut delegate = BranchListDelegate::new(
            workspace,
            repository.clone(),
            style,
            branch_selection_behavior,
            cx,
        );
        delegate.all_branches = all_branches;

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .show_scrollbar(true)
                .modal(!embedded)
        });
        let picker_focus_handle = picker.focus_handle(cx);

        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
            picker.delegate.show_footer = !embedded && !picker.delegate.is_select_only();
        });

        let mut subscriptions = Vec::new();

        if let Some(repo) = &repository {
            subscriptions.push(cx.subscribe_in(
                repo,
                window,
                move |this, repo, event, window, cx| {
                    if matches!(event, RepositoryEvent::BranchListChanged) {
                        let branch_list = repo.read(cx).branch_list.clone();
                        this.picker.update(cx, |picker, cx| {
                            picker.delegate.restore_selected_branch = picker
                                .delegate
                                .matches
                                .get(picker.delegate.selected_index)
                                .and_then(|entry| entry.as_branch().map(|b| b.ref_name.clone()));
                            picker.delegate.all_branches = process_branches(
                                &branch_list,
                                picker.delegate.branch_selection_behavior.selected_branch(),
                            );
                            picker.refresh(window, cx);
                        });
                    }
                },
            ));
        }

        // Fetch default branch asynchronously since it requires a git operation
        cx.spawn_in(window, async move |this, cx| {
            let default_branch = default_branch_request
                .context("No active repository")?
                .await
                .map(Result::ok)
                .ok()
                .flatten()
                .flatten();

            let _ = this.update_in(cx, |this, _window, cx| {
                this.picker.update(cx, |picker, _cx| {
                    picker.delegate.default_branch = default_branch;
                });
            });

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        Self {
            picker,
            picker_focus_handle,
            width,
            _subscriptions: subscriptions,
            embedded,
        }
    }

    fn new_embedded(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        width: Rems,
        show_footer: bool,
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
        this.picker.update(cx, |picker, _| {
            picker.delegate.show_footer = show_footer;
        });
        this._subscriptions
            .push(cx.subscribe(&this.picker, |_, _, _, cx| {
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
        self.picker.update(cx, |picker, cx| {
            picker.delegate.modifiers = ev.modifiers;
            cx.notify();
        })
    }

    pub fn handle_delete(
        &mut self,
        _: &branch_picker::DeleteBranch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            if picker.delegate.is_select_only() {
                return;
            }
            picker
                .delegate
                .delete_at(picker.delegate.selected_index, false, window, cx)
        })
    }

    pub fn handle_force_delete(
        &mut self,
        _: &branch_picker::ForceDeleteBranch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            if picker.delegate.is_select_only() {
                return;
            }
            picker
                .delegate
                .delete_at(picker.delegate.selected_index, true, window, cx)
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
            .on_action(cx.listener(Self::handle_force_delete))
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
    all_branches: Vec<Branch>,
    default_branch: Option<SharedString>,
    repo: Option<Entity<Repository>>,
    style: BranchListStyle,
    selected_index: usize,
    last_query: String,
    modifiers: Modifiers,
    branch_filter: BranchFilter,
    state: PickerState,
    branch_selection_behavior: BranchSelectionBehavior,
    focus_handle: FocusHandle,
    restore_selected_branch: Option<SharedString>,
    show_footer: bool,
    hovered_delete_index: Option<usize>,
}

enum BranchSelectionBehavior {
    Checkout,
    Select {
        selected_branch: Option<SharedString>,
        on_select: SelectBranchCallback,
    },
}

impl BranchSelectionBehavior {
    fn selected_branch(&self) -> Option<&SharedString> {
        match self {
            Self::Checkout => None,
            Self::Select {
                selected_branch, ..
            } => selected_branch.as_ref(),
        }
    }

    fn is_select_only(&self) -> bool {
        matches!(self, Self::Select { .. })
    }
}

#[derive(Clone)]
struct BranchSelectionContext {
    selected_branch: Option<SharedString>,
    active_branch_ref_name: Option<SharedString>,
    active_branch_upstream_ref_name: Option<SharedString>,
    active_branch_remote_name: Option<SharedString>,
}

impl BranchSelectionContext {
    fn new(
        selected_branch: Option<SharedString>,
        repo: Option<&Entity<Repository>>,
        cx: &App,
    ) -> Self {
        let active_branch = repo.and_then(|repo| repo.read(cx).branch.clone());
        let active_branch_ref_name = active_branch.as_ref().map(|branch| branch.ref_name.clone());
        let active_branch_upstream_ref_name = active_branch.as_ref().and_then(|branch| {
            branch
                .upstream
                .as_ref()
                .map(|upstream| upstream.ref_name.clone())
        });
        let active_branch_remote_name = active_branch.as_ref().and_then(|branch| {
            branch
                .upstream
                .as_ref()
                .and_then(|upstream| upstream.remote_name())
                .or_else(|| branch.remote_name())
                .map(SharedString::from)
        });

        Self {
            selected_branch,
            active_branch_ref_name,
            active_branch_upstream_ref_name,
            active_branch_remote_name,
        }
    }

    fn priority(&self, branch: &Branch) -> usize {
        if self
            .selected_branch
            .as_ref()
            .is_some_and(|selected_branch| branch_matches_ref(branch, selected_branch))
        {
            0
        } else if self.is_on_active_branch_remote(branch) {
            1
        } else if self.is_active_branch(branch) || self.is_active_upstream(branch) {
            3
        } else {
            2
        }
    }

    fn is_active_branch(&self, branch: &Branch) -> bool {
        self.active_branch_ref_name
            .as_ref()
            .is_some_and(|ref_name| branch.ref_name.as_ref() == ref_name.as_ref())
    }

    fn is_active_upstream(&self, branch: &Branch) -> bool {
        self.active_branch_upstream_ref_name
            .as_ref()
            .is_some_and(|ref_name| branch.ref_name.as_ref() == ref_name.as_ref())
    }

    fn is_on_active_branch_remote(&self, branch: &Branch) -> bool {
        if self.is_active_branch(branch) || self.is_active_upstream(branch) {
            return false;
        }

        let Some(active_branch_remote_name) = &self.active_branch_remote_name else {
            return false;
        };

        branch_remote_name(branch)
            .is_some_and(|remote_name| remote_name == active_branch_remote_name.as_ref())
    }
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

fn delete_branch_command(is_remote: bool, branch_name: &str, force: bool) -> String {
    format!(
        "branch {} {branch_name}",
        delete_branch_flag(is_remote, force)
    )
}

struct BranchDeleteForceDeletePrompt {
    required_error_substrings: &'static [&'static str],
    message: fn(&str) -> String,
}

impl BranchDeleteForceDeletePrompt {
    fn matches(&self, normalized_error_message: &str) -> bool {
        self.required_error_substrings
            .iter()
            .all(|substring| normalized_error_message.contains(substring))
    }
}

const BRANCH_DELETE_FORCE_DELETE_PROMPTS: &[BranchDeleteForceDeletePrompt] =
    &[BranchDeleteForceDeletePrompt {
        required_error_substrings: &["not fully merged"],
        message: unmerged_branch_force_delete_prompt,
    }];

fn unmerged_branch_force_delete_prompt(branch_name: &str) -> String {
    format!("Branch \"{branch_name}\" is not fully merged. Force delete it?")
}

// Git only reports these cases via localized stderr, so this best-effort check
// may miss some locales and fall back to the raw error toast.
fn force_delete_prompt_for_branch_delete_error(
    error: &anyhow::Error,
    branch_name: &str,
) -> Option<String> {
    let normalized_error_message = error.to_string().to_lowercase();
    BRANCH_DELETE_FORCE_DELETE_PROMPTS
        .iter()
        .find(|prompt| prompt.matches(&normalized_error_message))
        .map(|prompt| (prompt.message)(branch_name))
}

struct DeleteBranchTooltip {
    picker: WeakEntity<Picker<BranchListDelegate>>,
    focus_handle: FocusHandle,
    delete_index: usize,
    _subscription: Subscription,
}

impl DeleteBranchTooltip {
    fn new(
        picker: Entity<Picker<BranchListDelegate>>,
        focus_handle: FocusHandle,
        delete_index: usize,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe(&picker, |_, _, cx| cx.notify());
        Self {
            picker: picker.downgrade(),
            focus_handle,
            delete_index,
            _subscription: subscription,
        }
    }
}

impl Render for DeleteBranchTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let force_delete = self
            .picker
            .read_with(cx, |picker, _| {
                picker
                    .delegate
                    .is_force_delete_hovering_index(self.delete_index)
            })
            .unwrap_or(false);
        if force_delete {
            Tooltip::for_action_in(
                "Force Delete Branch",
                &branch_picker::ForceDeleteBranch,
                &self.focus_handle,
                cx,
            )
            .into_any_element()
        } else {
            Tooltip::with_meta_in(
                "Delete Branch",
                Some(&branch_picker::DeleteBranch),
                "Hold alt to force delete",
                &self.focus_handle,
                cx,
            )
            .into_any_element()
        }
    }
}

fn branch_matches_ref(branch: &Branch, branch_ref: &SharedString) -> bool {
    branch.ref_name.as_ref() == branch_ref.as_ref() || branch.name() == branch_ref.as_ref()
}

fn branch_remote_name(branch: &Branch) -> Option<&str> {
    branch.remote_name().or_else(|| {
        branch
            .upstream
            .as_ref()
            .and_then(|upstream| upstream.remote_name())
    })
}

fn sort_branch_entries(
    matches: &mut [Entry],
    branch_selection_context: Option<&BranchSelectionContext>,
) {
    matches.sort_by_key(|entry| {
        let Some(branch) = entry.as_branch() else {
            return (4, false);
        };

        let priority = branch_selection_context
            .map(|context| context.priority(branch))
            .unwrap_or(0);
        (priority, branch.is_remote())
    });
}

fn process_branches(
    branches: &Arc<[Branch]>,
    preserved_branch: Option<&SharedString>,
) -> Vec<Branch> {
    let remote_upstreams: HashSet<_> = branches
        .iter()
        .filter_map(|branch| {
            branch
                .upstream
                .as_ref()
                .filter(|upstream| upstream.is_remote())
                .map(|upstream| upstream.ref_name.clone())
        })
        .collect();

    let mut result: Vec<Branch> = branches
        .iter()
        .filter(|branch| {
            !remote_upstreams.contains(&branch.ref_name)
                || preserved_branch
                    .as_ref()
                    .is_some_and(|preserved_branch| branch_matches_ref(branch, preserved_branch))
        })
        .cloned()
        .collect();

    result.sort_by_key(|branch| {
        (
            !branch.is_head,
            branch
                .most_recent_commit
                .as_ref()
                .map(|commit| 0 - commit.commit_timestamp),
        )
    });

    result
}

impl BranchListDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        repo: Option<Entity<Repository>>,
        style: BranchListStyle,
        branch_selection_behavior: BranchSelectionBehavior,
        cx: &mut Context<BranchList>,
    ) -> Self {
        let restore_selected_branch = match &branch_selection_behavior {
            BranchSelectionBehavior::Checkout => None,
            BranchSelectionBehavior::Select {
                selected_branch, ..
            } => selected_branch.clone(),
        };

        Self {
            workspace,
            matches: vec![],
            repo,
            style,
            all_branches: Vec::new(),
            default_branch: None,
            selected_index: 0,
            last_query: Default::default(),
            modifiers: Default::default(),
            branch_filter: BranchFilter::All,
            state: PickerState::List,
            branch_selection_behavior,
            focus_handle: cx.focus_handle(),
            restore_selected_branch,
            show_footer: false,
            hovered_delete_index: None,
        }
    }

    fn is_select_only(&self) -> bool {
        self.branch_selection_behavior.is_select_only()
    }

    fn is_force_delete_hovering_index(&self, index: usize) -> bool {
        self.modifiers.alt && self.hovered_delete_index == Some(index)
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

    fn delete_at(
        &self,
        idx: usize,
        force: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(entry) = self.matches.get(idx).cloned() else {
            return;
        };
        let Some(repo) = self.repo.clone() else {
            return;
        };

        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let Entry::Branch { branch, .. } = &entry else {
                log::error!("Failed to delete entry: wrong entry to delete");
                return Ok(());
            };

            if branch.is_head {
                return Ok(());
            }

            let is_remote = branch.is_remote();
            let branch_name = branch.name().to_string();
            let initial_result = repo
                .update(cx, |repo, _| {
                    repo.delete_branch(is_remote, branch_name.clone(), force)
                })
                .await?;

            let (result, attempted_force) = match initial_result {
                Ok(()) => (Ok(()), force),
                Err(error) => {
                    if is_remote {
                        log::error!("Failed to delete remote branch: {error}");
                    } else {
                        log::error!("Failed to delete branch: {error}");
                    }

                    let force_delete_prompt = (!force)
                        .then(|| force_delete_prompt_for_branch_delete_error(&error, entry.name()))
                        .flatten();

                    if let Some(prompt_message) = force_delete_prompt {
                        let answer = cx.update(|window, cx| {
                            window.prompt(
                                PromptLevel::Warning,
                                &prompt_message,
                                None,
                                &["Force Delete", "Cancel"],
                                cx,
                            )
                        })?;

                        if answer.await != Ok(0) {
                            return Ok(());
                        }

                        let retry = repo
                            .update(cx, |repo, _| {
                                repo.delete_branch(is_remote, branch_name, true)
                            })
                            .await?;

                        if let Err(error) = &retry {
                            log::error!("Failed to force delete branch: {error}");
                        }
                        (retry, true)
                    } else {
                        (Err(error), force)
                    }
                }
            };

            if let Err(error) = result {
                if let Some(workspace) = workspace.upgrade() {
                    cx.update(|_window, cx| {
                        show_error_toast(
                            workspace,
                            delete_branch_command(is_remote, entry.name(), attempted_force),
                            error,
                            cx,
                        )
                    })?;
                }

                return Ok(());
            }

            picker.update_in(cx, |picker, _, cx| {
                picker.delegate.matches.retain(|e| e != &entry);

                if let Entry::Branch { branch, .. } = &entry {
                    picker
                        .delegate
                        .all_branches
                        .retain(|e| e.ref_name != branch.ref_name);
                }

                if picker.delegate.matches.is_empty() {
                    picker.delegate.selected_index = 0;
                } else if picker.delegate.selected_index >= picker.delegate.matches.len() {
                    picker.delegate.selected_index = picker.delegate.matches.len() - 1;
                }

                picker.delegate.hovered_delete_index = None;

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
                if self.is_select_only() {
                    "Select branch…"
                } else {
                    "Switch branch…"
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
        editor: &Arc<dyn ErasedEditor>,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let focus_handle = self.focus_handle.clone();
        let editor = editor.as_any().downcast_ref::<Entity<Editor>>().unwrap();

        let show_inline_filter =
            self.editor_position() == PickerEditorPosition::End || !self.show_footer;

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
                    .when(show_inline_filter, |this| {
                        let tooltip_label = match self.branch_filter {
                            BranchFilter::All => "Filter Remote Branches",
                            BranchFilter::Remote => "Show All Branches",
                        };

                        this.gap_1().justify_between().child({
                            IconButton::new("filter-remotes", IconName::Filter)
                                .toggle_state(self.branch_filter == BranchFilter::Remote)
                                .icon_size(IconSize::Small)
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
                    }),
            )
            .when(
                self.editor_position() == PickerEditorPosition::Start,
                |this| this.child(Divider::horizontal()),
            )
    }

    fn editor_position(&self) -> PickerEditorPosition {
        if self.is_select_only() {
            return PickerEditorPosition::Start;
        }

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
        let all_branches = self.all_branches.clone();
        let branch_selection_context = self.is_select_only().then(|| {
            BranchSelectionContext::new(
                self.branch_selection_behavior.selected_branch().cloned(),
                self.repo.as_ref(),
                cx,
            )
        });

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

                sort_branch_entries(&mut matches, branch_selection_context.as_ref());

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
                let mut matches: Vec<Entry> = fuzzy_nucleo::match_strings_async(
                    &candidates,
                    &query,
                    fuzzy_nucleo::Case::Smart,
                    fuzzy_nucleo::LengthPenalty::On,
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

                sort_branch_entries(&mut matches, branch_selection_context.as_ref());

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

                    if !picker.delegate.is_select_only()
                        && !query.is_empty()
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
                    } else if let Some(ref_name) = delegate.restore_selected_branch.take() {
                        delegate.selected_index = delegate
                            .matches
                            .iter()
                            .position(|entry| {
                                entry.as_branch().is_some_and(|branch| {
                                    branch.ref_name == ref_name
                                        || branch.name() == ref_name.as_ref()
                                })
                            })
                            .unwrap_or(0);
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
                if let BranchSelectionBehavior::Select { on_select, .. } =
                    &self.branch_selection_behavior
                {
                    on_select(branch.clone(), cx);
                    cx.emit(DismissEvent);
                    return;
                }

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

        let (commit_time, absolute_time, author_name, subject) = entry
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
                    let absolute_time = time_format::format_localized_timestamp(
                        commit_time,
                        OffsetDateTime::now_utc(),
                        local_offset,
                        time_format::TimestampFormat::EnhancedAbsolute,
                    );
                    let author = commit.author_name.clone();
                    (
                        Some(formatted_time),
                        Some(absolute_time),
                        Some(author),
                        Some(subject),
                    )
                })
            })
            .unwrap_or_else(|| (None, None, None, None));

        let is_head_branch = entry.as_branch().is_some_and(|branch| branch.is_head);
        let is_checked_branch = entry.as_branch().is_some_and(|branch| {
            if self.is_select_only() {
                self.branch_selection_behavior
                    .selected_branch()
                    .is_some_and(|selected_branch| branch_matches_ref(branch, selected_branch))
            } else {
                branch.is_head
            }
        });

        let entry_icon = match entry {
            Entry::NewUrl { .. } | Entry::NewBranch { .. } | Entry::NewRemoteName { .. } => {
                IconName::Plus
            }
            Entry::Branch { branch, .. } => {
                if is_checked_branch {
                    IconName::Check
                } else if branch.is_remote() {
                    IconName::Screen
                } else {
                    IconName::GitBranch
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
        let picker = cx.entity();
        let is_new_items = matches!(
            entry,
            Entry::NewUrl { .. } | Entry::NewBranch { .. } | Entry::NewRemoteName { .. }
        );

        let deleted_branch_icon = |entry_ix: usize| {
            let picker = picker.clone();
            let focus_handle = focus_handle.clone();
            let force_delete = self.is_force_delete_hovering_index(entry_ix);

            div()
                .id(("delete-hover", entry_ix))
                .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                    if *hovered {
                        this.delegate.hovered_delete_index = Some(entry_ix);
                    } else if this.delegate.hovered_delete_index == Some(entry_ix) {
                        this.delegate.hovered_delete_index = None;
                    }
                    cx.notify();
                }))
                .child(
                    IconButton::new(("delete", entry_ix), IconName::Trash)
                        .icon_size(IconSize::Small)
                        .when(force_delete, |this| this.icon_color(Color::Error))
                        .tooltip(move |_, cx| {
                            cx.new(|cx| {
                                DeleteBranchTooltip::new(
                                    picker.clone(),
                                    focus_handle.clone(),
                                    entry_ix,
                                    cx,
                                )
                            })
                            .into()
                        })
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.delegate.delete_at(
                                entry_ix,
                                this.delegate.modifiers.alt,
                                window,
                                cx,
                            );
                        })),
                )
        };

        let create_from_default_button = self.default_branch.as_ref().map(|default_branch| {
            let tooltip_label: SharedString = format!("Create New From: {default_branch}").into();
            let focus_handle = self.focus_handle.clone();

            IconButton::new("create_from_default", IconName::GitBranchPlus)
                .icon_size(IconSize::Small)
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
                        .gap_2p5()
                        .flex_grow()
                        .child(
                            Icon::new(entry_icon)
                                .color(if is_checked_branch {
                                    Color::Accent
                                } else {
                                    Color::Muted
                                })
                                .size(IconSize::Small),
                        )
                        .child(
                            v_flex()
                                .id("info_container")
                                .w_full()
                                .child(entry_title)
                                .child({
                                    let message = match entry {
                                        Entry::NewUrl { url } => format!("Based off {url}"),
                                        Entry::NewRemoteName { url, .. } => {
                                            format!("Based off {url}")
                                        }
                                        Entry::NewBranch { .. } => {
                                            if let Some(current_branch) =
                                                self.repo.as_ref().and_then(|repo| {
                                                    repo.read(cx).branch.as_ref().map(|b| b.name())
                                                })
                                            {
                                                format!("Based off {}", current_branch)
                                            } else {
                                                "Based off the current branch".to_string()
                                            }
                                        }
                                        Entry::Branch { .. } => String::new(),
                                    };

                                    if matches!(entry, Entry::Branch { .. }) {
                                        let show_author_name = ProjectSettings::get_global(cx)
                                            .git
                                            .branch_picker
                                            .show_author_name;
                                        let has_author = show_author_name && author_name.is_some();
                                        let has_commit = commit_time.is_some();
                                        let author_for_meta =
                                            if show_author_name { author_name } else { None };

                                        let dot = || {
                                            Label::new("•")
                                                .alpha(0.5)
                                                .color(Color::Muted)
                                                .size(LabelSize::Small)
                                        };

                                        h_flex()
                                            .w_full()
                                            .min_w_0()
                                            .gap_1p5()
                                            .when_some(author_for_meta, |this, author| {
                                                this.child(
                                                    Label::new(author)
                                                        .color(Color::Muted)
                                                        .size(LabelSize::Small),
                                                )
                                            })
                                            .when_some(commit_time, |this, time| {
                                                this.when(has_author, |this| this.child(dot()))
                                                    .child(
                                                        Label::new(time)
                                                            .color(Color::Muted)
                                                            .size(LabelSize::Small),
                                                    )
                                            })
                                            .when_some(subject, |this, subj| {
                                                this.when(has_commit, |this| this.child(dot()))
                                                    .child(
                                                        Label::new(subj.to_string())
                                                            .color(Color::Muted)
                                                            .size(LabelSize::Small)
                                                            .truncate()
                                                            .flex_1(),
                                                    )
                                            })
                                            .when(!has_commit, |this| {
                                                this.child(
                                                    Label::new("No commits found")
                                                        .color(Color::Muted)
                                                        .size(LabelSize::Small),
                                                )
                                            })
                                            .into_any_element()
                                    } else {
                                        Label::new(message)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .truncate()
                                            .into_any_element()
                                    }
                                })
                                .when_some(
                                    entry.as_branch().map(|b| b.name().to_string()),
                                    |this, branch_name| {
                                        let absolute_time = absolute_time.clone();
                                        this.tooltip({
                                            let is_head = is_head_branch;
                                            let is_checked = is_checked_branch;
                                            let is_select_only = self.is_select_only();
                                            Tooltip::element(move |_, _| {
                                                v_flex()
                                                    .child(Label::new(branch_name.clone()))
                                                    .when(is_select_only && is_checked, |this| {
                                                        this.child(
                                                            Label::new("Selected Branch")
                                                                .size(LabelSize::Small)
                                                                .color(Color::Muted),
                                                        )
                                                    })
                                                    .when(is_head, |this| {
                                                        this.child(
                                                            Label::new("Current Branch")
                                                                .size(LabelSize::Small)
                                                                .color(Color::Muted),
                                                        )
                                                    })
                                                    .when_some(
                                                        absolute_time.clone(),
                                                        |this, time| {
                                                            this.child(
                                                                Label::new(time)
                                                                    .size(LabelSize::Small)
                                                                    .color(Color::Muted),
                                                            )
                                                        },
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    },
                                ),
                        ),
                )
                .when(
                    !self.is_select_only() && !is_new_items && !is_head_branch,
                    |this| {
                        this.end_slot(deleted_branch_icon(ix))
                            .show_end_slot_on_hover()
                    },
                )
                .when_some(
                    if is_new_items {
                        create_from_default_button
                    } else {
                        None
                    },
                    |this, create_from_default_button| {
                        this.end_slot(create_from_default_button)
                            .show_end_slot_on_hover()
                    },
                ),
        )
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        if self.is_select_only()
            || !self.show_footer
            || self.editor_position() == PickerEditorPosition::End
        {
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
                    .when(
                        !selected_entry
                            .and_then(|entry| entry.as_branch())
                            .is_some_and(|branch| branch.is_head),
                        |this| {
                            this.child(
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
                                        window.dispatch_action(
                                            branch_picker::DeleteBranch.boxed_clone(),
                                            cx,
                                        );
                                    }),
                            )
                        },
                    )
                    .child(
                        Button::new("switch_branch", "Switch")
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
                                        let filter_label = match self.branch_filter {
                                            BranchFilter::All => "Filter Remote",
                                            BranchFilter::Remote => "Show All",
                                        };
                                        Button::new("filter-remotes", filter_label)
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
                            Button::new("create-new-branch", "Create")
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
                        Button::new("confirm-create-remote", "Confirm")
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
    use git::repository::{
        CommitSummary, Remote, Upstream, UpstreamTracking, UpstreamTrackingStatus,
    };
    use gpui::{AppContext, TestAppContext, VisualTestContext};
    use project::{FakeFs, Project};
    use rand::{Rng, rngs::StdRng};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
    }

    fn create_test_branch(
        name: &str,
        is_head: bool,
        remote_name: Option<&str>,
        timestamp: Option<i64>,
    ) -> Branch {
        create_test_branch_with_upstream(name, is_head, remote_name, timestamp, None)
    }

    fn create_test_branch_with_upstream(
        name: &str,
        is_head: bool,
        remote_name: Option<&str>,
        timestamp: Option<i64>,
        upstream_ref_name: Option<&str>,
    ) -> Branch {
        let ref_name = match remote_name {
            Some(remote_name) => format!("refs/remotes/{remote_name}/{name}"),
            None => format!("refs/heads/{name}"),
        };

        Branch {
            is_head,
            ref_name: ref_name.into(),
            upstream: upstream_ref_name.map(|ref_name| Upstream {
                ref_name: ref_name.into(),
                tracking: UpstreamTracking::Tracked(UpstreamTrackingStatus {
                    ahead: 0,
                    behind: 0,
                }),
            }),
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

    #[test]
    fn test_select_branch_preserves_selected_remote_upstream_and_prioritizes_active_remote_branches()
     {
        let selected_branch = SharedString::from("origin/main");
        let branches: Arc<[Branch]> = Arc::from([
            create_test_branch_with_upstream(
                "feature",
                true,
                None,
                Some(1200),
                Some("refs/remotes/origin/feature"),
            ),
            create_test_branch_with_upstream(
                "main",
                false,
                None,
                Some(1100),
                Some("refs/remotes/origin/main"),
            ),
            create_test_branch("main", false, Some("origin"), Some(1000)),
            create_test_branch("feature", false, Some("origin"), Some(900)),
            create_test_branch("main", false, Some("fork"), Some(800)),
        ]);

        let processed_branches = process_branches(&branches, Some(&selected_branch));
        assert!(
            processed_branches
                .iter()
                .any(|branch| branch.name() == "origin/main"),
            "the selected remote branch should be preserved even when a local branch tracks it"
        );
        assert!(
            processed_branches
                .iter()
                .all(|branch| branch.name() != "origin/feature"),
            "the active branch's unselected remote upstream should still be collapsed"
        );

        let mut entries = processed_branches
            .into_iter()
            .map(|branch| Entry::Branch {
                branch,
                positions: Vec::new(),
            })
            .collect::<Vec<_>>();
        let selection_context = BranchSelectionContext {
            selected_branch: Some(selected_branch),
            active_branch_ref_name: Some("refs/heads/feature".into()),
            active_branch_upstream_ref_name: Some("refs/remotes/origin/feature".into()),
            active_branch_remote_name: Some("origin".into()),
        };

        sort_branch_entries(&mut entries, Some(&selection_context));

        let ordered_branch_names = entries.iter().map(Entry::name).collect::<Vec<_>>();
        assert_eq!(ordered_branch_names.first(), Some(&"origin/main"));
        assert!(
            ordered_branch_names.iter().position(|name| *name == "main")
                < ordered_branch_names
                    .iter()
                    .position(|name| *name == "fork/main"),
            "branches on the active branch's remote should be prioritized"
        );
    }

    async fn init_branch_list_test(
        repository: Option<Entity<Repository>>,
        branches: Vec<Branch>,
        cx: &mut TestAppContext,
    ) -> (Entity<BranchList>, VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window_handle
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let branch_list = window_handle
            .update(cx, |_multi_workspace, window, cx| {
                cx.new(|cx| {
                    let mut delegate = BranchListDelegate::new(
                        workspace.downgrade(),
                        repository,
                        BranchListStyle::Modal,
                        BranchSelectionBehavior::Checkout,
                        cx,
                    );
                    delegate.all_branches = branches;
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
                        _subscriptions: vec![_subscription],
                        embedded: false,
                    }
                })
            })
            .unwrap();

        let cx = VisualTestContext::from_window(window_handle.into(), cx);

        (branch_list, cx)
    }

    async fn init_fake_repository_with_fs(
        cx: &mut TestAppContext,
    ) -> (Arc<FakeFs>, Entity<Project>, Entity<Repository>) {
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

        (fs, project, repository.unwrap())
    }

    async fn init_fake_repository(
        cx: &mut TestAppContext,
    ) -> (Entity<Project>, Entity<Repository>) {
        let (_, project, repository) = init_fake_repository_with_fs(cx).await;
        (project, repository)
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
        let (_project, repository) = init_fake_repository(cx).await;

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
                picker.delegate.delete_at(1, false, window, cx);
                branch_to_delete
            })
        });
        cx.run_until_parked();

        let expected_branches = ["main", "feature-auth", "feature-ui", "develop"]
            .into_iter()
            .filter(|name| name != &branch_to_delete)
            .collect::<HashSet<_>>();
        let repo_branches = branch_list
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
        let repo_branches = repo_branches
            .iter()
            .map(|b| b.name())
            .collect::<HashSet<_>>();
        assert_eq!(&repo_branches, &expected_branches);

        branch_list.update(cx, move |branch_list, cx| {
            branch_list.picker.update(cx, move |picker, _cx| {
                assert_eq!(picker.delegate.matches.len(), 3);
                let branches = picker
                    .delegate
                    .matches
                    .iter()
                    .map(|be| be.name())
                    .collect::<HashSet<_>>();
                assert_eq!(branches, expected_branches);
            })
        });
    }

    #[gpui::test]
    async fn test_delete_unmerged_branch_prompts_for_force_delete(cx: &mut TestAppContext) {
        init_test(cx);
        let (fs, _project, repository) = init_fake_repository_with_fs(cx).await;

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

        let branch_to_delete = "feature-auth";
        fs.with_git_state(path!("/dir/.git").as_ref(), true, |state| {
            state
                .branches_requiring_force_delete
                .insert(branch_to_delete.to_string());
        })
        .expect("failed to mark test branch as requiring force delete");

        let (branch_list, mut ctx) = init_branch_list_test(repository.into(), branches, cx).await;
        let cx = &mut ctx;
        update_branch_list_matches_with_empty_query(&branch_list, cx).await;

        branch_list.update_in(cx, |branch_list, window, cx| {
            branch_list.picker.update(cx, |picker, cx| {
                let branch_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|entry| entry.name() == branch_to_delete)
                    .unwrap();
                picker.delegate.delete_at(branch_index, false, window, cx);
            })
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());

        cx.simulate_prompt_answer("Force Delete");
        cx.run_until_parked();

        let repo_branches = branch_list
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
        assert!(
            repo_branches
                .iter()
                .all(|branch| branch.name() != branch_to_delete)
        );
    }

    #[gpui::test]
    async fn test_delete_unmerged_branch_cancel_keeps_branch(cx: &mut TestAppContext) {
        init_test(cx);
        let (fs, _project, repository) = init_fake_repository_with_fs(cx).await;

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

        let branch_to_delete = "feature-auth";
        fs.with_git_state(path!("/dir/.git").as_ref(), true, |state| {
            state
                .branches_requiring_force_delete
                .insert(branch_to_delete.to_string());
        })
        .expect("failed to mark test branch as requiring force delete");

        let (branch_list, mut ctx) = init_branch_list_test(repository.into(), branches, cx).await;
        let cx = &mut ctx;
        update_branch_list_matches_with_empty_query(&branch_list, cx).await;

        let initial_match_count = branch_list.update(cx, |branch_list, cx| {
            branch_list
                .picker
                .update(cx, |picker, _| picker.delegate.matches.len())
        });

        branch_list.update_in(cx, |branch_list, window, cx| {
            branch_list.picker.update(cx, |picker, cx| {
                let branch_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|entry| entry.name() == branch_to_delete)
                    .unwrap();
                picker.delegate.delete_at(branch_index, false, window, cx);
            })
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());

        cx.simulate_prompt_answer("Cancel");
        cx.run_until_parked();
        assert!(!cx.has_pending_prompt());

        let repo_branches = branch_list
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
        assert!(
            repo_branches
                .iter()
                .any(|branch| branch.name() == branch_to_delete),
            "branch should still exist after cancelling the force-delete prompt"
        );

        let final_match_count = branch_list.update(cx, |branch_list, cx| {
            branch_list
                .picker
                .update(cx, |picker, _| picker.delegate.matches.len())
        });
        assert_eq!(
            initial_match_count, final_match_count,
            "picker matches should be unchanged after cancel"
        );
    }

    #[gpui::test]
    async fn test_force_delete_click_deletes_branch_without_prompt(cx: &mut TestAppContext) {
        init_test(cx);
        let (fs, _project, repository) = init_fake_repository_with_fs(cx).await;

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

        let branch_to_delete = "feature-auth";
        fs.with_git_state(path!("/dir/.git").as_ref(), true, |state| {
            state
                .branches_requiring_force_delete
                .insert(branch_to_delete.to_string());
        })
        .expect("failed to mark test branch as requiring force delete");

        let (branch_list, mut ctx) = init_branch_list_test(repository.into(), branches, cx).await;
        let cx = &mut ctx;
        update_branch_list_matches_with_empty_query(&branch_list, cx).await;

        branch_list.update_in(cx, |branch_list, window, cx| {
            branch_list.picker.update(cx, |picker, cx| {
                picker.delegate.modifiers = Modifiers::alt();
                let branch_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|entry| entry.name() == branch_to_delete)
                    .unwrap();
                picker.delegate.delete_at(branch_index, true, window, cx);
            })
        });
        cx.run_until_parked();
        assert!(!cx.has_pending_prompt());

        let repo_branches = branch_list
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
        assert!(
            repo_branches
                .iter()
                .all(|branch| branch.name() != branch_to_delete)
        );
    }

    #[gpui::test]
    async fn test_delete_remote_branch(cx: &mut TestAppContext) {
        init_test(cx);
        let (_project, repository) = init_fake_repository(cx).await;
        let branches = vec![
            create_test_branch("main", true, Some("origin"), Some(1000)),
            create_test_branch("feature-auth", false, Some("origin"), Some(900)),
            create_test_branch("feature-ui", false, Some("fork"), Some(800)),
            create_test_branch("develop", false, Some("private"), Some(700)),
        ];

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
                picker.delegate.delete_at(1, false, window, cx);
                branch_to_delete
            })
        });
        cx.run_until_parked();

        let expected_branches = [
            "origin/main",
            "origin/feature-auth",
            "fork/feature-ui",
            "private/develop",
        ]
        .into_iter()
        .filter(|name| name != &branch_to_delete)
        .collect::<HashSet<_>>();
        let repo_branches = branch_list
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
        let repo_branches = repo_branches
            .iter()
            .map(|b| b.name())
            .collect::<HashSet<_>>();
        assert_eq!(&repo_branches, &expected_branches);

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
                assert_eq!(branches, expected_branches);
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
        let (_project, repository) = init_fake_repository(test_cx).await;

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
        let (_project, repository) = init_fake_repository(cx).await;
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
                name: SharedString::from("my_new_remote")
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
