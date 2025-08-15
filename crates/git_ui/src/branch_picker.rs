use anyhow::Context as _;
use fuzzy::StringMatchCandidate;

use collections::HashSet;
use git::repository::Branch;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, Render, SharedString, Styled,
    Subscription, Task, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::git_store::Repository;
use std::sync::Arc;
use time::OffsetDateTime;
use time_format::format_local_timestamp;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(open);
    workspace.register_action(switch);
    workspace.register_action(checkout_branch);
}

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
    let repository = workspace.project().read(cx).active_repository(cx).clone();
    let style = BranchListStyle::Modal;
    workspace.toggle_modal(window, cx, |window, cx| {
        BranchList::new(repository, style, rems(34.), window, cx)
    })
}

pub fn popover(
    repository: Option<Entity<Repository>>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<BranchList> {
    cx.new(|cx| {
        let list = BranchList::new(repository, BranchListStyle::Popover, rems(20.), window, cx);
        list.focus_handle(cx).focus(window);
        list
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BranchListStyle {
    Modal,
    Popover,
}

pub struct BranchList {
    width: Rems,
    pub picker: Entity<Picker<BranchListDelegate>>,
    _subscription: Subscription,
}

impl BranchList {
    fn new(
        repository: Option<Entity<Repository>>,
        style: BranchListStyle,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let all_branches_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.branches()));
        let default_branch_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.default_branch()));

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
                        branch
                            .most_recent_commit
                            .as_ref()
                            .map(|commit| 0 - commit.commit_timestamp)
                    });

                    all_branches
                })
                .await;

            this.update_in(cx, |this, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.default_branch = default_branch;
                    picker.delegate.all_branches = Some(all_branches);
                    picker.refresh(window, cx);
                })
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let delegate = BranchListDelegate::new(repository.clone(), style);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
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
}
impl ModalView for BranchList {}
impl EventEmitter<DismissEvent> for BranchList {}

impl Focusable for BranchList {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for BranchList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitBranchSelector")
            .w(self.width)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
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
struct BranchEntry {
    branch: Branch,
    positions: Vec<usize>,
    is_new: bool,
}

pub struct BranchListDelegate {
    matches: Vec<BranchEntry>,
    all_branches: Option<Vec<Branch>>,
    default_branch: Option<SharedString>,
    repo: Option<Entity<Repository>>,
    style: BranchListStyle,
    selected_index: usize,
    last_query: String,
    modifiers: Modifiers,
}

impl BranchListDelegate {
    fn new(repo: Option<Entity<Repository>>, style: BranchListStyle) -> Self {
        Self {
            matches: vec![],
            repo,
            style,
            all_branches: None,
            default_branch: None,
            selected_index: 0,
            last_query: Default::default(),
            modifiers: Default::default(),
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
        cx.spawn(async move |_, cx| {
            if let Some(based_branch) = from_branch {
                repo.update(cx, |repo, _| repo.change_branch(based_branch.to_string()))?
                    .await??;
            }

            repo.update(cx, |repo, _| {
                repo.create_branch(new_branch_name.to_string())
            })?
            .await??;
            repo.update(cx, |repo, _| {
                repo.change_branch(new_branch_name.to_string())
            })?
            .await??;

            Ok(())
        })
        .detach_and_prompt_err("Failed to create branch", window, cx, |e, _, _| {
            Some(e.to_string())
        });
        cx.emit(DismissEvent);
    }
}

impl PickerDelegate for BranchListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select branch…".into()
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

        const RECENT_BRANCHES_COUNT: usize = 10;
        cx.spawn_in(window, async move |picker, cx| {
            let mut matches: Vec<BranchEntry> = if query.is_empty() {
                all_branches
                    .into_iter()
                    .filter(|branch| !branch.is_remote())
                    .take(RECENT_BRANCHES_COUNT)
                    .map(|branch| BranchEntry {
                        branch,
                        positions: Vec::new(),
                        is_new: false,
                    })
                    .collect()
            } else {
                let candidates = all_branches
                    .iter()
                    .enumerate()
                    .map(|(ix, branch)| StringMatchCandidate::new(ix, branch.name()))
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
                .map(|candidate| BranchEntry {
                    branch: all_branches[candidate.candidate_id].clone(),
                    positions: candidate.positions,
                    is_new: false,
                })
                .collect()
            };
            picker
                .update(cx, |picker, _| {
                    #[allow(clippy::nonminimal_bool)]
                    if !query.is_empty()
                        && !matches
                            .first()
                            .is_some_and(|entry| entry.branch.name() == query)
                    {
                        let query = query.replace(' ', "-");
                        matches.push(BranchEntry {
                            branch: Branch {
                                ref_name: format!("refs/heads/{query}").into(),
                                is_head: false,
                                upstream: None,
                                most_recent_commit: None,
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
            let from_branch = if secondary {
                self.default_branch.clone()
            } else {
                None
            };
            self.create_branch(
                from_branch,
                entry.branch.name().to_owned().into(),
                window,
                cx,
            );
            return;
        }

        let current_branch = self.repo.as_ref().map(|repo| {
            repo.read_with(cx, |repo, _| {
                repo.branch.as_ref().map(|branch| branch.ref_name.clone())
            })
        });

        if current_branch
            .flatten()
            .is_some_and(|current_branch| current_branch == entry.branch.ref_name)
        {
            cx.emit(DismissEvent);
            return;
        }

        cx.spawn_in(window, {
            let branch = entry.branch.clone();
            async move |picker, cx| {
                let branch_change_task = picker.update(cx, |this, cx| {
                    let repo = this
                        .delegate
                        .repo
                        .as_ref()
                        .context("No active repository")?
                        .clone();

                    let mut cx = cx.to_async();

                    anyhow::Ok(async move {
                        repo.update(&mut cx, |repo, _| {
                            repo.change_branch(branch.name().to_string())
                        })?
                        .await?
                    })
                })??;

                branch_change_task.await?;

                picker.update(cx, |_, cx| {
                    cx.emit(DismissEvent);

                    anyhow::Ok(())
                })
            }
        })
        .detach_and_prompt_err("Failed to change branch", window, cx, |_, _, _| None);
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
        let entry = &self.matches[ix];

        let (commit_time, subject) = entry
            .branch
            .most_recent_commit
            .as_ref()
            .map(|commit| {
                let subject = commit.subject.clone();
                let commit_time = OffsetDateTime::from_unix_timestamp(commit.commit_timestamp)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc());
                let formatted_time = format_local_timestamp(
                    commit_time,
                    OffsetDateTime::now_utc(),
                    time_format::TimestampFormat::Relative,
                );
                (Some(formatted_time), Some(subject))
            })
            .unwrap_or_else(|| (None, None));

        let icon = if let Some(default_branch) = self.default_branch.clone()
            && entry.is_new
        {
            Some(
                IconButton::new("branch-from-default", IconName::GitBranchAlt)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.delegate.set_selected_index(ix, window, cx);
                        this.delegate.confirm(true, window, cx);
                    }))
                    .tooltip(move |window, cx| {
                        Tooltip::for_action(
                            format!("Create branch based off default: {default_branch}"),
                            &menu::SecondaryConfirm,
                            window,
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
                    Label::new(format!("Create branch \"{}\"…", entry.branch.name()))
                        .single_line()
                        .truncate(),
                )
                .into_any_element()
        } else {
            HighlightedLabel::new(entry.branch.name().to_owned(), entry.positions.clone())
                .truncate()
                .into_any_element()
        };

        Some(
            ListItem::new(SharedString::from(format!("vcs-menu-{ix}")))
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
                                .when_some(commit_time, |label, commit_time| {
                                    label.child(
                                        Label::new(commit_time)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .into_element(),
                                    )
                                }),
                        )
                        .when(self.style == BranchListStyle::Modal, |el| {
                            el.child(div().max_w_96().child({
                                let message = if entry.is_new {
                                    if let Some(current_branch) =
                                        self.repo.as_ref().and_then(|repo| {
                                            repo.read(cx).branch.as_ref().map(|b| b.name())
                                        })
                                    {
                                        format!("based off {}", current_branch)
                                    } else {
                                        "based off the current branch".to_string()
                                    }
                                } else {
                                    subject.unwrap_or("no commits found".into()).to_string()
                                };
                                Label::new(message)
                                    .size(LabelSize::Small)
                                    .truncate()
                                    .color(Color::Muted)
                            }))
                        }),
                )
                .end_slot::<IconButton>(icon),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        None
    }
}
