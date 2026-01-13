mod graph;
mod graph_rendering;

use graph::format_timestamp;

use git::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, ParsedGitRemote,
    parse_git_remote_url,
    repository::{CommitDiff, LogOrder, LogSource},
};
use git_ui::commit_tooltip::CommitAvatar;
use gpui::{
    AnyElement, App, ClipboardItem, Context, Corner, DefiniteLength, ElementId, Entity,
    EventEmitter, FocusHandle, Focusable, FontWeight, InteractiveElement, ParentElement, Pixels,
    Point, Render, ScrollWheelEvent, SharedString, Styled, Subscription, Task, WeakEntity, Window,
    actions, anchored, deferred, px,
};
use graph_rendering::accent_colors_count;
use project::{
    Project,
    git_store::{CommitDataState, GitStoreEvent, Repository, RepositoryEvent},
};
use settings::Settings;
use std::ops::Range;
use theme::ThemeSettings;
use time::{OffsetDateTime, UtcOffset};
use ui::{ContextMenu, ScrollableHandle, Table, TableInteractionState, Tooltip, prelude::*};
use util::ResultExt;
use workspace::{
    Workspace,
    item::{Item, ItemEvent, SerializableItem},
};

use crate::{graph::AllCommitCount, graph_rendering::render_graph};

actions!(
    git_graph,
    [
        /// Opens the Git Graph panel.
        OpenGitGraph,
        /// Opens the commit view for the selected commit.
        OpenCommitView,
    ]
);

pub fn init(cx: &mut App) {
    workspace::register_serializable_item::<GitGraph>(cx);

    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        // todo!: We should only register this action when project has a repo we can use to generate the graph
        workspace.register_action(|workspace, _: &OpenGitGraph, window, cx| {
            let project = workspace.project().clone();
            let git_graph = cx.new(|cx| GitGraph::new(project, window, cx));
            workspace.add_item_to_active_pane(Box::new(git_graph), None, true, window, cx);
        });
    })
    .detach();
}

pub struct GitGraph {
    focus_handle: FocusHandle,
    graph: crate::graph::GitGraph,
    project: Entity<Project>,
    max_lanes: usize,
    loading: bool,
    error: Option<SharedString>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    row_height: Pixels,
    table_interaction_state: Entity<TableInteractionState>,
    selected_entry_idx: Option<usize>,
    selected_commit_diff: Option<CommitDiff>,
    _commit_diff_task: Option<Task<()>>,
    _load_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl GitGraph {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, window, |_, _, cx| cx.notify())
            .detach();

        let git_store = project.read(cx).git_store().clone();
        let git_store_subscription = cx.subscribe(&git_store, |this, _, event, cx| match event {
            GitStoreEvent::RepositoryUpdated(_, RepositoryEvent::BranchChanged, true)
            | GitStoreEvent::ActiveRepositoryChanged(_) => {
                // todo! only call load data from render, we should set a bool here
                // todo! We should check that the repo actually has a change that would affect the graph
                this.load_data(cx);
            }
            _ => {}
        });

        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.buffer_font_size(cx);
        let row_height = font_size + px(10.0);

        let table_interaction_state = cx.new(|cx| TableInteractionState::new(cx));

        let accent_colors = cx.theme().accents();
        let mut this = GitGraph {
            focus_handle,
            project,
            graph: crate::graph::GitGraph::new(accent_colors_count(accent_colors)),
            max_lanes: 0,
            loading: true,
            error: None,
            _load_task: None,
            _commit_diff_task: None,
            context_menu: None,
            row_height,
            table_interaction_state,
            selected_entry_idx: None,
            selected_commit_diff: None,
            // todo! We can just make this a simple Subscription instead of wrapping it
            _subscriptions: vec![git_store_subscription],
        };

        this.load_data(cx);
        this
    }

    fn load_data(&mut self, cx: &mut Context<Self>) {
        let project = self.project.clone();
        self.loading = true;
        self.error = None;

        if self._load_task.is_some() {
            return;
        }

        let Some(repository) = project.read_with(cx, |project, cx| project.active_repository(cx))
        else {
            return;
        };

        let commits = repository.update(cx, |repo, cx| {
            repo.initial_graph_data(LogSource::All, LogOrder::DateOrder, cx)
        });

        self._load_task = Some(cx.spawn(async move |this: WeakEntity<Self>, cx| {
            let commits = commits.await;

            this.update(cx, |this, cx| {
                this.loading = false;

                match commits {
                    Ok(commits) => {
                        this.graph.clear();
                        let commit_count = commits.len();
                        this.graph.add_commits(commits);
                        this.max_lanes = this.graph.max_lanes;
                        this.graph.max_commit_count = AllCommitCount::Loaded(commit_count);
                    }
                    Err(e) => {
                        this.error = Some(format!("{:?}", e).into());
                    }
                };

                this._load_task.take();
                cx.notify();
            })
            .log_err();
        }));
    }

    fn render_table_rows(
        &mut self,
        range: Range<usize>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Vec<AnyElement>> {
        let repository = self
            .project
            .read_with(cx, |project, cx| project.active_repository(cx));

        let row_height = self.row_height;

        // We fetch data outside the visible viewport to avoid loading entries when
        // users scroll through the git graph
        if let Some(repository) = repository.as_ref() {
            const FETCH_RANGE: usize = 100;
            repository.update(cx, |repository, cx| {
                self.graph.commits[range.start.saturating_sub(FETCH_RANGE)
                    ..(range.end + FETCH_RANGE).min(self.graph.commits.len().saturating_sub(1))]
                    .iter()
                    .for_each(|commit| {
                        repository.fetch_commit_data(commit.data.sha, cx);
                    });
            });
        }

        range
            .map(|idx| {
                let Some((commit, repository)) =
                    self.graph.commits.get(idx).zip(repository.as_ref())
                else {
                    return vec![
                        div().h(row_height).into_any_element(),
                        div().h(row_height).into_any_element(),
                        div().h(row_height).into_any_element(),
                        div().h(row_height).into_any_element(),
                    ];
                };

                let data = repository.update(cx, |repository, cx| {
                    repository.fetch_commit_data(commit.data.sha, cx).clone()
                });

                let short_sha = commit.data.sha.display_short();
                let mut formatted_time = String::new();
                let subject;
                let author_name;

                if let CommitDataState::Loaded(data) = data {
                    subject = data.subject.clone();
                    author_name = data.author_name.clone();
                    formatted_time = format_timestamp(data.commit_timestamp);
                } else {
                    subject = "Loading...".into();
                    author_name = "".into();
                }

                vec![
                    div()
                        .id(ElementId::NamedInteger("commit-subject".into(), idx as u64))
                        .overflow_hidden()
                        .tooltip(Tooltip::text(subject.clone()))
                        .child(Label::new(subject).single_line())
                        .into_any_element(),
                    Label::new(formatted_time)
                        .color(Color::Muted)
                        .single_line()
                        .into_any_element(),
                    Label::new(author_name)
                        .color(Color::Muted)
                        .single_line()
                        .into_any_element(),
                    Label::new(short_sha)
                        .color(Color::Accent)
                        .single_line()
                        .into_any_element(),
                ]
            })
            .collect()
    }

    fn select_entry(&mut self, idx: usize, cx: &mut Context<Self>) {
        if self.selected_entry_idx == Some(idx) {
            return;
        }

        self.selected_entry_idx = Some(idx);
        self.selected_commit_diff = None;

        let Some(commit) = self.graph.commits.get(idx) else {
            return;
        };

        let sha = commit.data.sha.to_string();
        let repository = self
            .project
            .read_with(cx, |project, cx| project.active_repository(cx));

        let Some(repository) = repository else {
            return;
        };

        let diff_receiver = repository.update(cx, |repo, _| repo.load_commit_diff(sha));

        self._commit_diff_task = Some(cx.spawn(async move |this, cx| {
            if let Ok(Ok(diff)) = diff_receiver.await {
                this.update(cx, |this, cx| {
                    this.selected_commit_diff = Some(diff);
                    cx.notify();
                })
                .ok();
            }
        }));

        cx.notify();
    }

    fn get_remote(
        &self,
        repository: &Repository,
        _window: &mut Window,
        cx: &mut App,
    ) -> Option<GitRemote> {
        let remote_url = repository.default_remote_url()?;
        let provider_registry = GitHostingProviderRegistry::default_global(cx);
        let (provider, parsed) = parse_git_remote_url(provider_registry, &remote_url)?;
        Some(GitRemote {
            host: provider,
            owner: parsed.owner.into(),
            repo: parsed.repo.into(),
        })
    }

    fn render_commit_detail_panel(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(selected_idx) = self.selected_entry_idx else {
            return div().into_any_element();
        };

        let Some(commit_entry) = self.graph.commits.get(selected_idx) else {
            return div().into_any_element();
        };

        let repository = self
            .project
            .read_with(cx, |project, cx| project.active_repository(cx));

        let Some(repository) = repository else {
            return div().into_any_element();
        };

        let data = repository.update(cx, |repository, cx| {
            repository
                .fetch_commit_data(commit_entry.data.sha, cx)
                .clone()
        });

        let full_sha: SharedString = commit_entry.data.sha.to_string().into();
        let truncated_sha: SharedString = {
            let sha_str = full_sha.as_ref();
            if sha_str.len() > 24 {
                format!("{}...", &sha_str[..24]).into()
            } else {
                full_sha.clone()
            }
        };
        let ref_names = commit_entry.data.ref_names.clone();

        let (author_name, author_email, commit_timestamp, subject) = match &data {
            CommitDataState::Loaded(data) => (
                data.author_name.clone(),
                data.author_email.clone(),
                Some(data.commit_timestamp),
                data.subject.clone(),
            ),
            CommitDataState::Loading => ("Loading...".into(), "".into(), None, "Loading...".into()),
        };

        let date_string = commit_timestamp
            .and_then(|ts| OffsetDateTime::from_unix_timestamp(ts).ok())
            .map(|datetime| {
                let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
                let local_datetime = datetime.to_offset(local_offset);
                let format =
                    time::format_description::parse("[month repr:short] [day], [year]").ok();
                format
                    .and_then(|f| local_datetime.format(&f).ok())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let remote = repository.update(cx, |repo, cx| self.get_remote(repo, window, cx));

        let avatar = {
            let avatar = CommitAvatar::new(&full_sha, remote.as_ref());
            v_flex()
                .w(px(64.))
                .h(px(64.))
                .border_1()
                .border_color(cx.theme().colors().border)
                .rounded_full()
                .justify_center()
                .items_center()
                .child(
                    avatar
                        .avatar(window, cx)
                        .map(|a| a.size(px(64.)).into_any_element())
                        .unwrap_or_else(|| {
                            Icon::new(IconName::Person)
                                .color(Color::Muted)
                                .size(IconSize::XLarge)
                                .into_any_element()
                        }),
                )
        };

        let changed_files_count = self
            .selected_commit_diff
            .as_ref()
            .map(|diff| diff.files.len())
            .unwrap_or(0);

        v_flex()
            .w(px(300.))
            .h_full()
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().surface_background)
            .child(
                v_flex()
                    .p_3()
                    .gap_3()
                    .child(
                        h_flex().justify_between().child(avatar).child(
                            IconButton::new("close-detail", IconName::Close)
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.selected_entry_idx = None;
                                    this.selected_commit_diff = None;
                                    this._commit_diff_task = None;
                                    cx.notify();
                                })),
                        ),
                    )
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(Label::new(author_name.clone()).weight(FontWeight::SEMIBOLD))
                            .child(
                                Label::new(date_string)
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .children((!ref_names.is_empty()).then(|| {
                        h_flex()
                            .gap_1()
                            .flex_wrap()
                            .children(ref_names.iter().map(|name| {
                                div()
                                    .px_1p5()
                                    .py_0p5()
                                    .rounded_md()
                                    .bg(cx.theme().colors().element_background)
                                    .child(
                                        Label::new(name.clone())
                                            .size(LabelSize::Small)
                                            .color(Color::Accent),
                                    )
                            }))
                    }))
                    .child(
                        v_flex()
                            .gap_1p5()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Icon::new(IconName::Person)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        Label::new(author_name)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .when(!author_email.is_empty(), |this| {
                                        this.child(
                                            Label::new(format!("<{}>", author_email))
                                                .size(LabelSize::Small)
                                                .color(Color::Ignored),
                                        )
                                    }),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Icon::new(IconName::Hash)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child({
                                        let copy_sha = full_sha.clone();
                                        Button::new("sha-button", truncated_sha)
                                            .style(ButtonStyle::Transparent)
                                            .label_size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .tooltip(Tooltip::text(format!(
                                                "Copy SHA: {}",
                                                copy_sha
                                            )))
                                            .on_click(move |_, _, cx| {
                                                cx.write_to_clipboard(ClipboardItem::new_string(
                                                    copy_sha.to_string(),
                                                ));
                                            })
                                    }),
                            )
                            .when_some(remote.clone(), |this, remote| {
                                let provider_name = remote.host.name();
                                let icon = match provider_name.as_str() {
                                    "GitHub" => IconName::Github,
                                    _ => IconName::Link,
                                };
                                let parsed_remote = ParsedGitRemote {
                                    owner: remote.owner.as_ref().into(),
                                    repo: remote.repo.as_ref().into(),
                                };
                                let params = BuildCommitPermalinkParams {
                                    sha: full_sha.as_ref(),
                                };
                                let url = remote
                                    .host
                                    .build_commit_permalink(&parsed_remote, params)
                                    .to_string();
                                this.child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Icon::new(icon)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Button::new(
                                                "view-on-provider",
                                                format!("View on {}", provider_name),
                                            )
                                            .style(ButtonStyle::Transparent)
                                            .label_size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .on_click(
                                                move |_, _, cx| {
                                                    cx.open_url(&url);
                                                },
                                            ),
                                        ),
                                )
                            })
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Icon::new(IconName::Undo)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        Button::new("uncommit", "Uncommit")
                                            .style(ButtonStyle::Transparent)
                                            .label_size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .p_3()
                    .child(
                        v_flex()
                            .gap_2()
                            .child(Label::new(subject).weight(FontWeight::MEDIUM)),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .p_3()
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                Label::new(format!("{} Changed Files", changed_files_count))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .children(self.selected_commit_diff.as_ref().map(|diff| {
                                v_flex().gap_1().children(diff.files.iter().map(|file| {
                                    let file_name: String = file
                                        .path
                                        .file_name()
                                        .map(|n| n.to_string())
                                        .unwrap_or_default();
                                    let dir_path: String = file
                                        .path
                                        .parent()
                                        .map(|p| p.as_unix_str().to_string())
                                        .unwrap_or_default();

                                    h_flex()
                                        .gap_1()
                                        .overflow_hidden()
                                        .child(
                                            Icon::new(IconName::File)
                                                .size(IconSize::Small)
                                                .color(Color::Accent),
                                        )
                                        .child(
                                            Label::new(file_name)
                                                .size(LabelSize::Small)
                                                .single_line(),
                                        )
                                        .when(!dir_path.is_empty(), |this| {
                                            this.child(
                                                Label::new(dir_path)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted)
                                                    .single_line(),
                                            )
                                        })
                                }))
                            })),
                    ),
            )
            .into_any_element()
    }

    fn handle_graph_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let line_height = window.line_height();
        let delta = event.delta.pixel_delta(line_height);

        let table_state = self.table_interaction_state.read(cx);
        let current_offset = table_state.scroll_offset();

        let viewport_height = table_state.scroll_handle.viewport().size.height;

        let commit_count = match self.graph.max_commit_count {
            AllCommitCount::Loaded(count) => count,
            AllCommitCount::NotLoaded => self.graph.commits.len(),
        };
        let content_height = self.row_height * commit_count;
        let max_scroll = (viewport_height - content_height).min(px(0.));

        let new_y = (current_offset.y + delta.y).clamp(max_scroll, px(0.));
        let new_offset = Point::new(current_offset.x, new_y);

        if new_offset != current_offset {
            table_state.set_scroll_offset(new_offset);
            cx.notify();
        }
    }
}

impl Render for GitGraph {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let description_width_fraction = 0.72;
        let date_width_fraction = 0.12;
        let author_width_fraction = 0.10;
        let commit_width_fraction = 0.06;

        let error_banner = self.error.as_ref().map(|error| {
            h_flex()
                .id("error-banner")
                .w_full()
                .px_2()
                .py_1()
                .bg(cx.theme().colors().surface_background)
                .border_b_1()
                .border_color(cx.theme().colors().border)
                .justify_between()
                .items_center()
                .child(
                    h_flex()
                        .gap_2()
                        .overflow_hidden()
                        .child(Icon::new(IconName::Warning).color(Color::Error))
                        .child(Label::new(error.clone()).color(Color::Error).single_line()),
                )
                .child(
                    IconButton::new("dismiss-error", IconName::Close)
                        .icon_size(IconSize::Small)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.error = None;
                            cx.notify();
                        })),
                )
        });

        let commit_count = match self.graph.max_commit_count {
            AllCommitCount::Loaded(count) => count,
            AllCommitCount::NotLoaded => self.graph.commits.len(),
        };

        let content = if self.loading && self.graph.commits.is_empty() && false {
            let message = if self.loading {
                "Loading commits..."
            } else {
                "No commits found"
            };
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new(message).color(Color::Muted))
        } else {
            let graph_width = px(16.0) * (4 as f32) + px(24.0);
            div()
                .size_full()
                .flex()
                .flex_row()
                .child(
                    div()
                        .w(graph_width)
                        .h_full()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .p_2()
                                .border_b_1()
                                .border_color(cx.theme().colors().border)
                                .child(Label::new("Graph").color(Color::Muted)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .overflow_hidden()
                                .child(render_graph(&self, cx))
                                .on_scroll_wheel(cx.listener(Self::handle_graph_scroll)),
                        ),
                )
                .child({
                    let row_height = self.row_height;
                    let selected_entry_idx = self.selected_entry_idx;
                    let weak_self = cx.weak_entity();
                    div().flex_1().size_full().child(
                        Table::new(4)
                            .interactable(&self.table_interaction_state)
                            .hide_row_borders()
                            .header(vec![
                                Label::new("Description")
                                    .color(Color::Muted)
                                    .into_any_element(),
                                Label::new("Date").color(Color::Muted).into_any_element(),
                                Label::new("Author").color(Color::Muted).into_any_element(),
                                Label::new("Commit").color(Color::Muted).into_any_element(),
                            ])
                            .column_widths(
                                [
                                    DefiniteLength::Fraction(description_width_fraction),
                                    DefiniteLength::Fraction(date_width_fraction),
                                    DefiniteLength::Fraction(author_width_fraction),
                                    DefiniteLength::Fraction(commit_width_fraction),
                                ]
                                .to_vec(),
                            )
                            .map_row(move |(index, row), _window, cx| {
                                let is_selected = selected_entry_idx == Some(index);
                                let weak = weak_self.clone();
                                row.h(row_height)
                                    .when(is_selected, |row| {
                                        row.bg(cx.theme().colors().element_selected)
                                    })
                                    .on_click(move |_, _, cx| {
                                        weak.update(cx, |this, cx| {
                                            this.select_entry(index, cx);
                                        })
                                        .ok();
                                    })
                                    .into_any_element()
                            })
                            .uniform_list(
                                "git-graph-commits",
                                commit_count,
                                cx.processor(Self::render_table_rows),
                            ),
                    )
                })
                .when(self.selected_entry_idx.is_some(), |this| {
                    this.child(self.render_commit_detail_panel(window, cx))
                })
        };

        div()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .key_context("GitGraph")
            .track_focus(&self.focus_handle)
            .child(v_flex().size_full().children(error_banner).child(content))
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl EventEmitter<ItemEvent> for GitGraph {}

impl Focusable for GitGraph {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for GitGraph {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Git Graph".into()
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }
}

impl SerializableItem for GitGraph {
    fn serialized_item_kind() -> &'static str {
        "GitGraph"
    }

    fn cleanup(
        workspace_id: workspace::WorkspaceId,
        alive_items: Vec<workspace::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        workspace::delete_unloaded_items(
            alive_items,
            workspace_id,
            "git_graphs",
            &persistence::GIT_GRAPHS,
            cx,
        )
    }

    fn deserialize(
        project: Entity<Project>,
        _: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<Entity<Self>>> {
        if persistence::GIT_GRAPHS
            .get_git_graph(item_id, workspace_id)
            .ok()
            .is_some_and(|is_open| is_open)
        {
            let git_graph = cx.new(|cx| GitGraph::new(project, window, cx));
            Task::ready(Ok(git_graph))
        } else {
            Task::ready(Err(anyhow::anyhow!("No git graph to deserialize")))
        }
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        Some(cx.background_spawn(async move {
            persistence::GIT_GRAPHS
                .save_git_graph(item_id, workspace_id, true)
                .await
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        event == &ItemEvent::UpdateTab
    }
}

mod persistence {
    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::WorkspaceDb;

    pub struct GitGraphsDb(ThreadSafeConnection);

    impl Domain for GitGraphsDb {
        const NAME: &str = stringify!(GitGraphsDb);

        const MIGRATIONS: &[&str] = (&[sql!(
            CREATE TABLE git_graphs (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                is_open INTEGER DEFAULT FALSE,

                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )]);
    }

    db::static_connection!(GIT_GRAPHS, GitGraphsDb, [WorkspaceDb]);

    impl GitGraphsDb {
        query! {
            pub async fn save_git_graph(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId,
                is_open: bool
            ) -> Result<()> {
                INSERT OR REPLACE INTO git_graphs(item_id, workspace_id, is_open)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_git_graph(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId
            ) -> Result<bool> {
                SELECT is_open
                FROM git_graphs
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
