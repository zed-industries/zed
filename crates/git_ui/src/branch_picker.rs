use anyhow::Context as _;
use editor::Editor;
use fuzzy::StringMatchCandidate;

use collections::HashSet;
use git::repository::Branch;
use gpui::http_client::Url;
use gpui::{
    App, AppContext, AsyncApp, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, Render,
    SharedString, Styled, Subscription, Task, WeakEntity, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::git_store::Repository;
use project::project_settings::ProjectSettings;
use settings::Settings;
use std::sync::Arc;
use time::OffsetDateTime;
use ui::{
    CommonAnimationExt, Divider, HighlightedLabel, ListItem, ListItemSpacing, ToggleButtonGroup,
    ToggleButtonSimple, Tooltip, prelude::*,
};
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
    let repository = workspace.project().read(cx).active_repository(cx);
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

        let delegate = BranchListDelegate::new(repository, style);
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
    is_url: bool,
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
    display_remotes: bool,
    state: PickerState,
    loading: bool,
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
            display_remotes: false,
            state: PickerState::List,
            loading: false,
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
            })?
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
        cx.spawn(async move |this, cx| {
            this.update(cx, |picker, cx| {
                picker.delegate.loading = true;
                cx.notify();
            })
            .log_err();

            let stop_loader = |this: &WeakEntity<Picker<BranchListDelegate>>, cx: &mut AsyncApp| {
                this.update(cx, |picker, cx| {
                    picker.delegate.loading = false;
                    cx.notify();
                })
                .log_err();
            };
            repo.update(cx, |repo, _| repo.create_remote(remote_name, remote_url))
                .inspect_err(|_err| {
                    stop_loader(&this, cx);
                })?
                .await
                .inspect_err(|_err| {
                    stop_loader(&this, cx);
                })?
                .inspect_err(|_err| {
                    stop_loader(&this, cx);
                })?;
            stop_loader(&this, cx);
            Ok(())
        })
        .detach_and_prompt_err("Failed to create remote", window, cx, |e, _, _cx| {
            Some(e.to_string())
        });
        cx.emit(DismissEvent);
    }

    fn loader(&self) -> AnyElement {
        Icon::new(IconName::LoadCircle)
            .size(IconSize::Small)
            .with_rotate_animation(3)
            .into_any_element()
    }
}

impl PickerDelegate for BranchListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select branch…".into()
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        cx.update_entity(editor, move |editor, cx| {
            let placeholder = match self.state {
                PickerState::List | PickerState::NewRemote | PickerState::NewBranch => {
                    if self.display_remotes {
                        "Select remote…"
                    } else {
                        "Select branch…"
                    }
                }
                PickerState::CreateRemote(_) => "Choose a name…",
            };
            editor.set_placeholder_text(placeholder, window, cx);
        });

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
                    .child(editor.clone()),
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

        const RECENT_BRANCHES_COUNT: usize = 10;
        let display_remotes = self.display_remotes;
        cx.spawn_in(window, async move |picker, cx| {
            let mut matches: Vec<BranchEntry> = if query.is_empty() {
                all_branches
                    .into_iter()
                    .filter(|branch| {
                        if display_remotes {
                            branch.is_remote()
                        } else {
                            !branch.is_remote()
                        }
                    })
                    .take(RECENT_BRANCHES_COUNT)
                    .map(|branch| BranchEntry {
                        branch,
                        positions: Vec::new(),
                        is_url: false,
                        is_new: false,
                    })
                    .collect()
            } else {
                let branches = all_branches
                    .iter()
                    .filter(|branch| {
                        if display_remotes {
                            branch.is_remote()
                        } else {
                            !branch.is_remote()
                        }
                    })
                    .collect::<Vec<_>>();
                let candidates = branches
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
                    branch: branches[candidate.candidate_id].clone(),
                    positions: candidate.positions,
                    is_url: false,
                    is_new: false,
                })
                .collect()
            };
            picker
                .update(cx, |picker, _| {
                    if matches!(picker.delegate.state, PickerState::CreateRemote(_)) {
                        picker.delegate.last_query = query;
                        picker.delegate.matches = Vec::new();
                        picker.delegate.selected_index = 0;

                        return;
                    }

                    if !query.is_empty()
                        && !matches
                            .first()
                            .is_some_and(|entry| entry.branch.name() == query)
                    {
                        let query = query.replace(' ', "-");
                        let is_url = query.trim_start_matches("git@").parse::<Url>().is_ok();
                        let ref_name = if is_url {
                            query.into()
                        } else {
                            if display_remotes {
                                format!("refs/heads/{query}").into()
                            } else {
                                format!("refs/remotes/{query}").into()
                            }
                        };
                        picker.delegate.state = if is_url {
                            PickerState::NewRemote
                        } else {
                            PickerState::NewBranch
                        };
                        matches.push(BranchEntry {
                            branch: Branch {
                                ref_name,
                                is_head: false,
                                upstream: None,
                                most_recent_commit: None,
                            },
                            positions: Vec::new(),
                            is_url,
                            is_new: true,
                        })
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
        if let PickerState::CreateRemote(remote_url) = &self.state {
            self.create_remote(self.last_query.clone(), remote_url.to_string(), window, cx);
            self.state = PickerState::List;
            cx.notify();
            return;
        }

        let Some(entry) = self.matches.get(self.selected_index()) else {
            return;
        };
        if entry.is_new {
            if entry.is_url {
                self.state = PickerState::CreateRemote(entry.branch.ref_name.clone());
                self.matches = Vec::new();
                self.selected_index = 0;
                cx.spawn_in(window, async move |this, cx| {
                    this.update_in(cx, |picker, window, cx| {
                        picker.set_query("", window, cx);
                    })
                })
                .detach_and_log_err(cx);
                cx.notify();
            } else {
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
            }

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

        let Some(repo) = self.repo.clone() else {
            return;
        };

        let branch = entry.branch.clone();
        cx.spawn(async move |_, cx| {
            repo.update(cx, |repo, _| repo.change_branch(branch.name().to_string()))?
                .await??;

            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed to change branch", window, cx, |_, _, _| None);

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
            .branch
            .most_recent_commit
            .as_ref()
            .map(|commit| {
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
            .unwrap_or_else(|| (None, None, None));

        let icon = if let Some(default_branch) = self.default_branch.clone()
            && entry.is_new
        {
            let (icon, tooltip_text) = if entry.is_url {
                let remote_url = entry.branch.ref_name.clone();
                (
                    IconName::Screen,
                    format!("Create remote based off {remote_url}"),
                )
            } else {
                (
                    IconName::GitBranchAlt,
                    format!("Create branch based off default: {default_branch}"),
                )
            };

            Some(
                IconButton::new("branch-from-default", icon)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.delegate.set_selected_index(ix, window, cx);
                        this.delegate.confirm(true, window, cx);
                    }))
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action(tooltip_text.clone(), &menu::SecondaryConfirm, cx)
                    }),
            )
        } else {
            None
        };

        let icon_elt = if self.display_remotes {
            Icon::new(IconName::Screen)
        } else {
            Icon::new(IconName::GitBranchAlt)
        };
        let branch_name = if entry.is_new {
            let label = if entry.is_url {
                "Create remote repository".to_string()
            } else {
                format!("Create branch \"{}\"…", entry.branch.name())
            };
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::Plus)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(Label::new(label).single_line().truncate())
                .into_any_element()
        } else {
            h_flex()
                .max_w_48()
                .child(h_flex().mr_1().child(icon_elt))
                .child(
                    HighlightedLabel::new(entry.branch.name().to_owned(), entry.positions.clone())
                        .truncate(),
                )
                .into_any_element()
        };

        Some(
            ListItem::new(SharedString::from(format!("vcs-menu-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .tooltip({
                    let branch_name = entry.branch.name().to_string();
                    if entry.is_new {
                        if entry.is_url {
                            Tooltip::text("Create remote repository".to_string())
                        } else {
                            Tooltip::text(format!("Create branch \"{}\"", branch_name))
                        }
                    } else {
                        Tooltip::text(branch_name)
                    }
                })
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
                                    if entry.is_url {
                                        format!("based off {}", entry.branch.ref_name)
                                    } else {
                                        if let Some(current_branch) =
                                            self.repo.as_ref().and_then(|repo| {
                                                repo.read(cx).branch.as_ref().map(|b| b.name())
                                            })
                                        {
                                            format!("based off {}", current_branch)
                                        } else {
                                            "based off the current branch".to_string()
                                        }
                                    }
                                } else {
                                    let show_author_name = ProjectSettings::get_global(cx)
                                        .git
                                        .branch_picker
                                        .show_author_name;

                                    subject.map_or("no commits found".into(), |subject| {
                                        if show_author_name && author_name.is_some() {
                                            format!("{} • {}", author_name.unwrap(), subject)
                                        } else {
                                            subject.to_string()
                                        }
                                    })
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

    fn render_header(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        if matches!(
            self.state,
            PickerState::CreateRemote(_) | PickerState::NewRemote | PickerState::NewBranch
        ) {
            return None;
        }
        let label = if self.display_remotes {
            "Remote"
        } else {
            "Local"
        };
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(Label::new(label).size(LabelSize::Small).color(Color::Muted))
                .into_any(),
        )
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        if self.loading {
            return Some(
                h_flex()
                    .w_full()
                    .p_1p5()
                    .gap_1()
                    .justify_end()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(self.loader())
                    .into_any(),
            );
        }
        match self.state {
            PickerState::List => Some(
                h_flex()
                    .w_full()
                    .p_1p5()
                    .gap_1()
                    .justify_end()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        h_flex().gap_0p5().child(
                            ToggleButtonGroup::single_row(
                                "filter-remotes",
                                [ToggleButtonSimple::new(
                                    "Filter remotes",
                                    cx.listener(move |this, _, window, cx| {
                                        this.delegate.display_remotes =
                                            !this.delegate.display_remotes;
                                        cx.spawn_in(window, async move |this, cx| {
                                            this.update_in(cx, |this, window, cx| {
                                                let last_query = this.delegate.last_query.clone();
                                                this.delegate.update_matches(last_query, window, cx)
                                            })?
                                            .await;

                                            Result::Ok::<_, anyhow::Error>(())
                                        })
                                        .detach_and_log_err(cx);
                                        cx.notify();
                                    }),
                                )
                                .selected(self.display_remotes)],
                            )
                            .style(ui::ToggleButtonGroupStyle::Transparent),
                        ),
                    )
                    .when(self.loading, |this| this.child(self.loader()))
                    .into_any(),
            ),
            PickerState::CreateRemote(_) => Some(
                h_flex()
                    .w_full()
                    .p_1p5()
                    .gap_1()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Label::new("Choose a name for this remote repository")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        h_flex().w_full().justify_end().child(
                            Label::new("Save")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .into_any(),
            ),
            PickerState::NewRemote | PickerState::NewBranch => None,
        }
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        None
    }
}
