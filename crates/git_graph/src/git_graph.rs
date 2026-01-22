mod graph;

use graph::format_timestamp;

use collections::BTreeMap;
use git::{
    BuildCommitPermalinkParams, GitHostingProviderRegistry, GitRemote, ParsedGitRemote,
    parse_git_remote_url,
    repository::{CommitDiff, LogOrder, LogSource},
};
use git_ui::commit_tooltip::CommitAvatar;
use gpui::{
    AnyElement, App, Bounds, ClipboardItem, Context, Corner, DefiniteLength, ElementId, Entity,
    EventEmitter, FocusHandle, Focusable, FontWeight, Hsla, InteractiveElement, ParentElement,
    PathBuilder, Pixels, Point, Render, ScrollWheelEvent, SharedString, Styled, Subscription, Task,
    WeakEntity, Window, actions, anchored, deferred, point, px,
};
use project::{
    Project,
    git_store::{CommitDataState, GitStoreEvent, Repository, RepositoryEvent},
};
use settings::Settings;
use std::ops::Range;
use theme::{AccentColors, ThemeSettings};
use time::{OffsetDateTime, UtcOffset};
use ui::{ContextMenu, ScrollableHandle, Table, TableInteractionState, Tooltip, prelude::*};
use workspace::{
    Workspace,
    item::{Item, ItemEvent, SerializableItem},
};

use crate::graph::{AllCommitCount, CommitLineSegment, CurveKind};

const LANE_WIDTH: Pixels = px(16.0);
const LINE_WIDTH: Pixels = px(1.5);
const COMMIT_CIRCLE_RADIUS: Pixels = px(4.5);
const COMMIT_CIRCLE_STROKE_WIDTH: Pixels = px(1.5);

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

pub fn accent_colors_count(accents: &AccentColors) -> usize {
    accents.0.len()
}

fn lane_center_x(
    bounds: Bounds<Pixels>,
    left_padding: Pixels,
    lane: f32,
    horizontal_scroll_offset: Pixels,
) -> Pixels {
    bounds.origin.x + left_padding + lane * LANE_WIDTH + LANE_WIDTH / 2.0 - horizontal_scroll_offset
}

fn to_row_center(
    to_row: usize,
    row_height: Pixels,
    scroll_offset: Pixels,
    bounds: Bounds<Pixels>,
) -> Pixels {
    bounds.origin.y + to_row as f32 * row_height + row_height / 2.0 - scroll_offset
}

fn draw_commit_circle(center_x: Pixels, center_y: Pixels, color: Hsla, window: &mut Window) {
    let radius = COMMIT_CIRCLE_RADIUS;
    let stroke_width = COMMIT_CIRCLE_STROKE_WIDTH;

    let mut builder = PathBuilder::stroke(stroke_width);

    // Start at the rightmost point of the circle
    builder.move_to(point(center_x + radius, center_y));

    // Draw the circle using two arc_to calls (top half, then bottom half)
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(center_x - radius, center_y),
    );
    builder.arc_to(
        point(radius, radius),
        px(0.),
        false,
        true,
        point(center_x + radius, center_y),
    );
    builder.close();

    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
}

pub struct GitGraph {
    focus_handle: FocusHandle,
    graph_data: crate::graph::GraphData,
    project: Entity<Project>,
    loading: bool,
    error: Option<SharedString>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    row_height: Pixels,
    table_interaction_state: Entity<TableInteractionState>,
    horizontal_scroll_offset: Pixels,
    graph_viewport_width: Pixels,
    selected_entry_idx: Option<usize>,
    log_source: LogSource,
    log_order: LogOrder,
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
        let accent_colors = cx.theme().accents();
        let mut graph = crate::graph::GraphData::new(accent_colors_count(accent_colors));
        let log_source = LogSource::default();
        let log_order = LogOrder::default();

        cx.subscribe(&git_store, |this, _, event, cx| match event {
            GitStoreEvent::RepositoryUpdated(_, RepositoryEvent::BranchChanged, true) => {
                // todo! only call load data from render, we should set a bool here
                // todo! We should check that the repo actually has a change that would affect the graph
                this.graph_data.clear();
                cx.notify();
            }
            GitStoreEvent::ActiveRepositoryChanged(repo_id) => {
                this.graph_data.clear();
                this._subscriptions.clear();
                cx.notify();

                if let Some(repository) = this.project.read(cx).active_repository(cx) {
                    // todo! we can merge the repository event handler with the git store events
                    this._subscriptions =
                        vec![cx.subscribe(&repository, Self::on_repository_event)];
                }
            }
            // todo! active repository has changed we should invalidate the graph state and reset our repo subscription
            _ => {}
        })
        .detach();

        let _subscriptions = if let Some(repository) = project.read(cx).active_repository(cx) {
            repository.update(cx, |repository, cx| {
                let commits =
                    repository.graph_data(log_source.clone(), log_order, 0..usize::MAX, cx);
                graph.add_commits(commits);
            });

            vec![cx.subscribe(&repository, Self::on_repository_event)]
        } else {
            vec![]
        };

        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.buffer_font_size(cx);
        let row_height = font_size + px(12.0);

        let table_interaction_state = cx.new(|cx| TableInteractionState::new(cx));

        GitGraph {
            focus_handle,
            project,
            graph_data: graph,
            loading: true,
            error: None,
            _load_task: None,
            _commit_diff_task: None,
            context_menu: None,
            row_height,
            table_interaction_state,
            horizontal_scroll_offset: px(0.),
            graph_viewport_width: px(88.),
            selected_entry_idx: None,
            selected_commit_diff: None,
            log_source,
            log_order,
            _subscriptions,
        }
    }

    fn on_repository_event(
        &mut self,
        repository: Entity<Repository>,
        event: &RepositoryEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            RepositoryEvent::GitGraphCountUpdated(_, commit_count) => {
                let old_count = self.graph_data.commits.len();

                repository.update(cx, |repository, cx| {
                    let commits = repository.graph_data(
                        self.log_source.clone(),
                        self.log_order,
                        old_count..*commit_count,
                        cx,
                    );
                    self.graph_data.add_commits(commits);
                });

                self.graph_data.max_commit_count = AllCommitCount::Loaded(*commit_count);
            }
            _ => {}
        }
    }

    fn render_badge(&self, name: &SharedString, accent_color: gpui::Hsla) -> impl IntoElement {
        div()
            .px_1p5()
            .py_0p5()
            // todo! height should probably be based off of font size
            .h(px(22.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .bg(accent_color.opacity(0.18))
            .border_1()
            .border_color(accent_color.opacity(0.55))
            .child(
                Label::new(name.clone())
                    .size(LabelSize::Small)
                    .color(Color::Default)
                    .single_line(),
            )
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
                self.graph_data.commits[range.start.saturating_sub(FETCH_RANGE)
                    ..(range.end + FETCH_RANGE)
                        .min(self.graph_data.commits.len().saturating_sub(1))]
                    .iter()
                    .for_each(|commit| {
                        repository.fetch_commit_data(commit.data.sha, cx);
                    });
            });
        }

        range
            .map(|idx| {
                let Some((commit, repository)) =
                    self.graph_data.commits.get(idx).zip(repository.as_ref())
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

                let accent_colors = cx.theme().accents();
                let accent_color = accent_colors
                    .0
                    .get(commit.color_idx)
                    .copied()
                    .unwrap_or_else(|| accent_colors.0.first().copied().unwrap_or_default());
                let is_selected = self.selected_entry_idx == Some(idx);
                let text_color = if is_selected {
                    Color::Default
                } else {
                    Color::Muted
                };

                vec![
                    div()
                        .id(ElementId::NamedInteger("commit-subject".into(), idx as u64))
                        .overflow_hidden()
                        .tooltip(Tooltip::text(subject.clone()))
                        .child(
                            h_flex()
                                .gap_1()
                                .items_center()
                                .overflow_hidden()
                                .children((!commit.data.ref_names.is_empty()).then(|| {
                                    h_flex().flex_shrink().gap_2().items_center().children(
                                        commit
                                            .data
                                            .ref_names
                                            .iter()
                                            .map(|name| self.render_badge(name, accent_color)),
                                    )
                                }))
                                .child(
                                    Label::new(subject)
                                        .color(text_color)
                                        .truncate()
                                        .single_line(),
                                ),
                        )
                        .into_any_element(),
                    Label::new(formatted_time)
                        .color(text_color)
                        .single_line()
                        .into_any_element(),
                    Label::new(author_name)
                        .color(text_color)
                        .single_line()
                        .into_any_element(),
                    Label::new(short_sha)
                        .color(text_color)
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

        let Some(commit) = self.graph_data.commits.get(idx) else {
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

        let Some(commit_entry) = self.graph_data.commits.get(selected_idx) else {
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
        let accent_colors = cx.theme().accents();
        let accent_color = accent_colors
            .0
            .get(commit_entry.color_idx)
            .copied()
            .unwrap_or_else(|| accent_colors.0.first().copied().unwrap_or_default());

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
                        h_flex().gap_1().flex_wrap().children(
                            ref_names
                                .iter()
                                .map(|name| self.render_badge(name, accent_color)),
                        )
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

    pub fn render_graph(&self, cx: &mut Context<GitGraph>) -> impl IntoElement {
        let row_height = self.row_height;
        let table_state = self.table_interaction_state.read(cx);
        let viewport_height = table_state
            .scroll_handle
            .0
            .borrow()
            .last_item_size
            .map(|size| size.item.height)
            .unwrap_or(px(600.0));
        let loaded_commit_count = self.graph_data.commits.len();

        let content_height = row_height * loaded_commit_count;
        let max_scroll = (content_height - viewport_height).max(px(0.));
        let scroll_offset_y = (-table_state.scroll_offset().y).clamp(px(0.), max_scroll);

        let first_visible_row = (scroll_offset_y / row_height).floor() as usize;
        let vertical_scroll_offset = scroll_offset_y - (first_visible_row as f32 * row_height);
        let horizontal_scroll_offset = self.horizontal_scroll_offset;

        let left_padding = px(12.0);
        let max_lanes = self.graph_data.max_lanes.max(6);
        let graph_width = LANE_WIDTH * max_lanes as f32 + left_padding * 2;
        let last_visible_row =
            first_visible_row + (viewport_height / row_height).ceil() as usize + 1;

        let viewport_range = first_visible_row.min(loaded_commit_count.saturating_sub(1))
            ..(last_visible_row).min(loaded_commit_count);
        let rows = self.graph_data.commits[viewport_range.clone()].to_vec();
        let commit_lines: Vec<_> = self
            .graph_data
            .lines
            .iter()
            .filter(|line| {
                line.full_interval.start <= viewport_range.end
                    && line.full_interval.end >= viewport_range.start
            })
            .cloned()
            .collect();

        let mut lines: BTreeMap<usize, Vec<_>> = BTreeMap::new();

        gpui::canvas(
            move |_bounds, _window, _cx| {},
            move |bounds: Bounds<Pixels>, _: (), window: &mut Window, cx: &mut App| {
                window.paint_layer(bounds, |window| {
                    let accent_colors = cx.theme().accents();

                    for (row_idx, row) in rows.into_iter().enumerate() {
                        let row_color = accent_colors.color_for_index(row.color_idx as u32);
                        let row_y_center =
                            bounds.origin.y + row_idx as f32 * row_height + row_height / 2.0
                                - vertical_scroll_offset;

                        let commit_x = lane_center_x(
                            bounds,
                            left_padding,
                            row.lane as f32,
                            horizontal_scroll_offset,
                        );

                        draw_commit_circle(commit_x, row_y_center, row_color, window);
                    }

                    for line in commit_lines {
                        let Some((start_segment_idx, start_column)) =
                            line.get_first_visible_segment_idx(first_visible_row)
                        else {
                            continue;
                        };

                        let line_color = accent_colors.color_for_index(line.color_idx as u32);
                        let line_x = lane_center_x(
                            bounds,
                            left_padding,
                            start_column as f32,
                            horizontal_scroll_offset,
                        );

                        let start_row = line.full_interval.start as i32 - first_visible_row as i32;

                        let from_y =
                            bounds.origin.y + start_row as f32 * row_height + row_height / 2.0
                                - vertical_scroll_offset
                                + COMMIT_CIRCLE_RADIUS;

                        let mut current_row = from_y;
                        let mut current_column = line_x;

                        let mut builder = PathBuilder::stroke(LINE_WIDTH);
                        builder.move_to(point(line_x, from_y));

                        let segments = &line.segments[start_segment_idx..];

                        for (segment_idx, segment) in segments.iter().enumerate() {
                            let is_last = segment_idx + 1 == segments.len();

                            match segment {
                                CommitLineSegment::Straight { to_row } => {
                                    let mut dest_row = to_row_center(
                                        to_row - first_visible_row,
                                        row_height,
                                        vertical_scroll_offset,
                                        bounds,
                                    );
                                    if is_last {
                                        dest_row -= COMMIT_CIRCLE_RADIUS;
                                    }

                                    let dest_point = point(current_column, dest_row);

                                    current_row = dest_point.y;
                                    builder.line_to(dest_point);
                                    builder.move_to(dest_point);
                                }
                                CommitLineSegment::Curve {
                                    to_column,
                                    on_row,
                                    curve_kind,
                                } => {
                                    let mut to_column = lane_center_x(
                                        bounds,
                                        left_padding,
                                        *to_column as f32,
                                        horizontal_scroll_offset,
                                    );

                                    let mut to_row = to_row_center(
                                        *on_row - first_visible_row,
                                        row_height,
                                        vertical_scroll_offset,
                                        bounds,
                                    );

                                    // This means that this branch was a checkout
                                    let going_right = to_column > current_column;
                                    let column_shift = if going_right {
                                        COMMIT_CIRCLE_RADIUS + COMMIT_CIRCLE_STROKE_WIDTH
                                    } else {
                                        -COMMIT_CIRCLE_RADIUS - COMMIT_CIRCLE_STROKE_WIDTH
                                    };

                                    let control = match curve_kind {
                                        CurveKind::Checkout => {
                                            if is_last {
                                                to_column -= column_shift;
                                            }
                                            builder.move_to(point(current_column, current_row));
                                            point(current_column, to_row)
                                        }
                                        CurveKind::Merge => {
                                            if is_last {
                                                to_row -= COMMIT_CIRCLE_RADIUS;
                                            }
                                            builder.move_to(point(
                                                current_column + column_shift,
                                                current_row - COMMIT_CIRCLE_RADIUS,
                                            ));
                                            point(to_column, current_row)
                                        }
                                    };

                                    match curve_kind {
                                        CurveKind::Checkout
                                            if (to_row - current_row).abs() > row_height =>
                                        {
                                            let start_curve =
                                                point(current_column, current_row + row_height);
                                            builder.line_to(start_curve);
                                            builder.move_to(start_curve);
                                        }
                                        CurveKind::Merge
                                            if (to_column - current_column).abs() > LANE_WIDTH =>
                                        {
                                            let column_shift =
                                                if going_right { LANE_WIDTH } else { -LANE_WIDTH };

                                            let start_curve = point(
                                                current_column + column_shift,
                                                current_row - COMMIT_CIRCLE_RADIUS,
                                            );

                                            builder.line_to(start_curve);
                                            builder.move_to(start_curve);
                                        }
                                        _ => {}
                                    };

                                    builder.curve_to(point(to_column, to_row), control);
                                    current_row = to_row;
                                    current_column = to_column;
                                    builder.move_to(point(current_column, current_row));
                                }
                            }
                        }

                        builder.close();
                        lines.entry(line.color_idx).or_default().push(builder);
                    }

                    for (color_idx, builders) in lines {
                        let line_color = accent_colors.color_for_index(color_idx as u32);

                        for builder in builders {
                            if let Ok(path) = builder.build() {
                                // we paint each color on it's own layer to stop overlapping lines
                                // of different colors changing the color of a line
                                window.paint_layer(bounds, |window| {
                                    window.paint_path(path, line_color);
                                });
                            }
                        }
                    }
                })
            },
        )
        .w(graph_width)
        .h_full()
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

        let commit_count = match self.graph_data.max_commit_count {
            AllCommitCount::Loaded(count) => count,
            AllCommitCount::NotLoaded => self.graph_data.commits.len(),
        };
        let content_height = self.row_height * commit_count;
        let max_vertical_scroll = (viewport_height - content_height).min(px(0.));

        let new_y = (current_offset.y + delta.y).clamp(max_vertical_scroll, px(0.));
        let new_offset = Point::new(current_offset.x, new_y);

        let left_padding = px(12.0);
        let lane_width = px(16.0);
        let max_lanes = self.graph_data.max_lanes.max(1);
        let graph_content_width = lane_width * max_lanes as f32 + left_padding * 2.0;
        let max_horizontal_scroll = (graph_content_width - self.graph_viewport_width).max(px(0.));

        let new_horizontal_offset =
            (self.horizontal_scroll_offset - delta.x).clamp(px(0.), max_horizontal_scroll);

        let vertical_changed = new_offset != current_offset;
        let horizontal_changed = new_horizontal_offset != self.horizontal_scroll_offset;

        if vertical_changed {
            table_state.set_scroll_offset(new_offset);
        }

        if horizontal_changed {
            self.horizontal_scroll_offset = new_horizontal_offset;
        }

        if vertical_changed || horizontal_changed {
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

        let commit_count = match self.graph_data.max_commit_count {
            AllCommitCount::Loaded(count) => count,
            AllCommitCount::NotLoaded => {
                self.project.update(cx, |project, cx| {
                    if let Some(repository) = project.active_repository(cx) {
                        repository.update(cx, |repository, cx| {
                            // Start loading the graph data if we haven't started already
                            repository.graph_data(
                                self.log_source.clone(),
                                self.log_order,
                                0..0,
                                cx,
                            );
                        })
                    }
                });

                self.graph_data.commits.len()
            }
        };

        let content = if self.loading && self.graph_data.commits.is_empty() && false {
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
            let graph_viewport_width = self.graph_viewport_width;
            div()
                .size_full()
                .flex()
                .flex_row()
                .child(
                    div()
                        .w(graph_viewport_width)
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
                                .id("graph-canvas")
                                .flex_1()
                                .overflow_hidden()
                                .child(self.render_graph(cx))
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

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result, bail};
    use collections::{HashMap, HashSet};
    use fs::FakeFs;
    use git::Oid;
    use git::repository::InitialGraphCommitData;
    use gpui::TestAppContext;
    use project::Project;
    use rand::prelude::*;
    use serde_json::json;
    use settings::SettingsStore;
    use smallvec::{SmallVec, smallvec};
    use std::path::Path;
    use std::sync::Arc;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    /// Generates a random commit DAG suitable for testing git graph rendering.
    ///
    /// The commits are ordered newest-first (like git log output), so:
    /// - Index 0 = most recent commit (HEAD)
    /// - Last index = oldest commit (root, has no parents)
    /// - Parents of commit at index I must have index > I
    ///
    /// When `adversarial` is true, generates complex topologies with many branches
    /// and octopus merges. Otherwise generates more realistic linear histories
    /// with occasional branches.
    fn generate_random_commit_dag(
        rng: &mut StdRng,
        num_commits: usize,
        adversarial: bool,
    ) -> Vec<Arc<InitialGraphCommitData>> {
        if num_commits == 0 {
            return Vec::new();
        }

        let mut commits: Vec<Arc<InitialGraphCommitData>> = Vec::with_capacity(num_commits);
        let oids: Vec<Oid> = (0..num_commits).map(|_| Oid::random(rng)).collect();

        for i in 0..num_commits {
            let sha = oids[i];

            let parents = if i == num_commits - 1 {
                smallvec![]
            } else {
                generate_parents_from_oids(rng, &oids, i, num_commits, adversarial)
            };

            let ref_names = if i == 0 {
                vec!["HEAD".into(), "main".into()]
            } else if adversarial && rng.random_bool(0.1) {
                vec![format!("branch-{}", i).into()]
            } else {
                Vec::new()
            };

            commits.push(Arc::new(InitialGraphCommitData {
                sha,
                parents,
                ref_names,
            }));
        }

        commits
    }

    fn generate_parents_from_oids(
        rng: &mut StdRng,
        oids: &[Oid],
        current_idx: usize,
        num_commits: usize,
        adversarial: bool,
    ) -> SmallVec<[Oid; 1]> {
        let remaining = num_commits - current_idx - 1;
        if remaining == 0 {
            return smallvec![];
        }

        if adversarial {
            let merge_chance = 0.4;
            let octopus_chance = 0.15;

            if remaining >= 3 && rng.random_bool(octopus_chance) {
                let num_parents = rng.random_range(3..=remaining.min(5));
                let mut parent_indices: Vec<usize> = (current_idx + 1..num_commits).collect();
                parent_indices.shuffle(rng);
                parent_indices
                    .into_iter()
                    .take(num_parents)
                    .map(|idx| oids[idx])
                    .collect()
            } else if remaining >= 2 && rng.random_bool(merge_chance) {
                let mut parent_indices: Vec<usize> = (current_idx + 1..num_commits).collect();
                parent_indices.shuffle(rng);
                parent_indices
                    .into_iter()
                    .take(2)
                    .map(|idx| oids[idx])
                    .collect()
            } else {
                let parent_idx = rng.random_range(current_idx + 1..num_commits);
                smallvec![oids[parent_idx]]
            }
        } else {
            let merge_chance = 0.15;
            let skip_chance = 0.1;

            if remaining >= 2 && rng.random_bool(merge_chance) {
                let first_parent = current_idx + 1;
                let second_parent = rng.random_range(current_idx + 2..num_commits);
                smallvec![oids[first_parent], oids[second_parent]]
            } else if rng.random_bool(skip_chance) && remaining >= 2 {
                let skip = rng.random_range(1..remaining.min(3));
                smallvec![oids[current_idx + 1 + skip]]
            } else {
                smallvec![oids[current_idx + 1]]
            }
        }
    }

    fn build_oid_to_row_map(graph: &crate::graph::GraphData) -> HashMap<Oid, usize> {
        graph
            .commits
            .iter()
            .enumerate()
            .map(|(idx, entry)| (entry.data.sha, idx))
            .collect()
    }

    fn verify_commit_order(
        graph: &crate::graph::GraphData,
        commits: &[Arc<InitialGraphCommitData>],
    ) -> Result<()> {
        if graph.commits.len() != commits.len() {
            bail!(
                "Commit count mismatch: graph has {} commits, expected {}",
                graph.commits.len(),
                commits.len()
            );
        }

        for (idx, (graph_commit, expected_commit)) in
            graph.commits.iter().zip(commits.iter()).enumerate()
        {
            if graph_commit.data.sha != expected_commit.sha {
                bail!(
                    "Commit order mismatch at index {}: graph has {:?}, expected {:?}",
                    idx,
                    graph_commit.data.sha,
                    expected_commit.sha
                );
            }
        }

        Ok(())
    }

    fn verify_line_endpoints(
        graph: &crate::graph::GraphData,
        oid_to_row: &HashMap<Oid, usize>,
    ) -> Result<()> {
        for line in &graph.lines {
            let child_row = *oid_to_row
                .get(&line.child)
                .context("Line references non-existent child commit")?;

            let parent_row = *oid_to_row
                .get(&line.parent)
                .context("Line references non-existent parent commit")?;

            if child_row >= parent_row {
                bail!(
                    "child_row ({}) must be < parent_row ({})",
                    child_row,
                    parent_row
                );
            }

            if line.full_interval.start != child_row {
                bail!(
                    "full_interval.start ({}) != child_row ({})",
                    line.full_interval.start,
                    child_row
                );
            }

            if line.full_interval.end != parent_row {
                bail!(
                    "full_interval.end ({}) != parent_row ({})",
                    line.full_interval.end,
                    parent_row
                );
            }

            if let Some(last_segment) = line.segments.last() {
                let segment_end_row = match last_segment {
                    crate::graph::CommitLineSegment::Straight { to_row } => *to_row,
                    crate::graph::CommitLineSegment::Curve { on_row, .. } => *on_row,
                };

                if segment_end_row != line.full_interval.end {
                    bail!(
                        "last segment ends at row {} but full_interval.end is {}",
                        segment_end_row,
                        line.full_interval.end
                    );
                }
            }
        }

        Ok(())
    }

    fn verify_column_correctness(
        graph: &crate::graph::GraphData,
        oid_to_row: &HashMap<Oid, usize>,
    ) -> Result<()> {
        for line in &graph.lines {
            let child_row = *oid_to_row
                .get(&line.child)
                .context("Line references non-existent child commit")?;

            let parent_row = *oid_to_row
                .get(&line.parent)
                .context("Line references non-existent parent commit")?;

            let child_lane = graph.commits[child_row].lane;
            if line.child_column != child_lane {
                bail!(
                    "child_column ({}) != child's lane ({})",
                    line.child_column,
                    child_lane
                );
            }

            let mut current_column = line.child_column;
            for segment in &line.segments {
                if let crate::graph::CommitLineSegment::Curve { to_column, .. } = segment {
                    current_column = *to_column;
                }
            }

            let parent_lane = graph.commits[parent_row].lane;
            if current_column != parent_lane {
                bail!(
                    "ending column ({}) != parent's lane ({})",
                    current_column,
                    parent_lane
                );
            }
        }

        Ok(())
    }

    fn verify_segment_continuity(graph: &crate::graph::GraphData) -> Result<()> {
        for line in &graph.lines {
            if line.segments.is_empty() {
                bail!("Line has no segments");
            }

            let mut current_row = line.full_interval.start;

            for (idx, segment) in line.segments.iter().enumerate() {
                let segment_end_row = match segment {
                    crate::graph::CommitLineSegment::Straight { to_row } => *to_row,
                    crate::graph::CommitLineSegment::Curve { on_row, .. } => *on_row,
                };

                if segment_end_row < current_row {
                    bail!(
                        "segment {} ends at row {} which is before current row {}",
                        idx,
                        segment_end_row,
                        current_row
                    );
                }

                current_row = segment_end_row;
            }
        }

        Ok(())
    }

    fn verify_line_overlaps(graph: &crate::graph::GraphData) -> Result<()> {
        for line in &graph.lines {
            let child_row = line.full_interval.start;

            let mut current_column = line.child_column;
            let mut current_row = child_row;

            for segment in &line.segments {
                match segment {
                    crate::graph::CommitLineSegment::Straight { to_row } => {
                        for row in (current_row + 1)..*to_row {
                            if row < graph.commits.len() {
                                let commit_at_row = &graph.commits[row];
                                if commit_at_row.lane == current_column {
                                    bail!(
                                        "straight segment from row {} to {} in column {} passes through commit {:?} at row {}",
                                        current_row,
                                        to_row,
                                        current_column,
                                        commit_at_row.data.sha,
                                        row
                                    );
                                }
                            }
                        }
                        current_row = *to_row;
                    }
                    crate::graph::CommitLineSegment::Curve {
                        to_column, on_row, ..
                    } => {
                        current_column = *to_column;
                        current_row = *on_row;
                    }
                }
            }
        }

        Ok(())
    }

    fn verify_coverage(graph: &crate::graph::GraphData) -> Result<()> {
        let mut expected_edges: HashSet<(Oid, Oid)> = HashSet::default();
        for entry in &graph.commits {
            for parent in &entry.data.parents {
                expected_edges.insert((entry.data.sha, *parent));
            }
        }

        let mut found_edges: HashSet<(Oid, Oid)> = HashSet::default();
        for line in &graph.lines {
            let edge = (line.child, line.parent);

            if !found_edges.insert(edge) {
                bail!(
                    "Duplicate line found for edge {:?} -> {:?}",
                    line.child,
                    line.parent
                );
            }

            if !expected_edges.contains(&edge) {
                bail!(
                    "Orphan line found: {:?} -> {:?} is not in the commit graph",
                    line.child,
                    line.parent
                );
            }
        }

        for (child, parent) in &expected_edges {
            if !found_edges.contains(&(*child, *parent)) {
                bail!("Missing line for edge {:?} -> {:?}", child, parent);
            }
        }

        assert_eq!(
            expected_edges.symmetric_difference(&found_edges).count(),
            0,
            "The symmetric difference should be zero"
        );

        Ok(())
    }

    fn verify_merge_line_optimality(
        graph: &crate::graph::GraphData,
        oid_to_row: &HashMap<Oid, usize>,
    ) -> Result<()> {
        for line in &graph.lines {
            let first_segment = line.segments.first();
            let is_merge_line = matches!(
                first_segment,
                Some(crate::graph::CommitLineSegment::Curve {
                    curve_kind: crate::graph::CurveKind::Merge,
                    ..
                })
            );

            if !is_merge_line {
                continue;
            }

            let child_row = *oid_to_row
                .get(&line.child)
                .context("Line references non-existent child commit")?;

            let parent_row = *oid_to_row
                .get(&line.parent)
                .context("Line references non-existent parent commit")?;

            let parent_lane = graph.commits[parent_row].lane;

            let Some(crate::graph::CommitLineSegment::Curve { to_column, .. }) = first_segment
            else {
                continue;
            };

            let curves_directly_to_parent = *to_column == parent_lane;

            if !curves_directly_to_parent {
                continue;
            }

            let curve_row = child_row + 1;
            let has_commits_in_path = graph.commits[curve_row..parent_row]
                .iter()
                .any(|c| c.lane == parent_lane);

            if has_commits_in_path {
                bail!(
                    "Merge line from {:?} to {:?} curves directly to parent lane {} but there are commits in that lane between rows {} and {}",
                    line.child,
                    line.parent,
                    parent_lane,
                    curve_row,
                    parent_row
                );
            }

            let curve_ends_at_parent = curve_row == parent_row;

            if curve_ends_at_parent {
                if line.segments.len() != 1 {
                    bail!(
                        "Merge line from {:?} to {:?} curves directly to parent (curve_row == parent_row), but has {} segments instead of 1 [MergeCurve]",
                        line.child,
                        line.parent,
                        line.segments.len()
                    );
                }
            } else {
                if line.segments.len() != 2 {
                    bail!(
                        "Merge line from {:?} to {:?} curves directly to parent lane without overlap, but has {} segments instead of 2 [MergeCurve, Straight]",
                        line.child,
                        line.parent,
                        line.segments.len()
                    );
                }

                let is_straight_segment = matches!(
                    line.segments.get(1),
                    Some(crate::graph::CommitLineSegment::Straight { .. })
                );

                if !is_straight_segment {
                    bail!(
                        "Merge line from {:?} to {:?} curves directly to parent lane without overlap, but second segment is not a Straight segment",
                        line.child,
                        line.parent
                    );
                }
            }
        }

        Ok(())
    }

    fn verify_all_invariants(
        graph: &crate::graph::GraphData,
        commits: &[Arc<InitialGraphCommitData>],
    ) -> Result<()> {
        let oid_to_row = build_oid_to_row_map(graph);

        verify_commit_order(graph, commits).context("commit order")?;
        verify_line_endpoints(graph, &oid_to_row).context("line endpoints")?;
        verify_column_correctness(graph, &oid_to_row).context("column correctness")?;
        verify_segment_continuity(graph).context("segment continuity")?;
        verify_merge_line_optimality(graph, &oid_to_row).context("merge line optimality")?;
        verify_coverage(graph).context("coverage")?;
        verify_line_overlaps(graph).context("line overlaps")?;
        Ok(())
    }

    #[test]
    fn test_git_graph_merge_commits() {
        let mut rng = StdRng::seed_from_u64(42);

        let oid1 = Oid::random(&mut rng);
        let oid2 = Oid::random(&mut rng);
        let oid3 = Oid::random(&mut rng);
        let oid4 = Oid::random(&mut rng);

        let commits = vec![
            Arc::new(InitialGraphCommitData {
                sha: oid1,
                parents: smallvec![oid2, oid3],
                ref_names: vec!["HEAD".into()],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid2,
                parents: smallvec![oid4],
                ref_names: vec![],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid3,
                parents: smallvec![oid4],
                ref_names: vec![],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid4,
                parents: smallvec![],
                ref_names: vec![],
            }),
        ];

        let mut graph_data = crate::graph::GraphData::new(8);
        graph_data.add_commits(&commits);

        if let Err(error) = verify_all_invariants(&graph_data, &commits) {
            panic!("Graph invariant violation for merge commits:\n{}", error);
        }
    }

    #[test]
    fn test_git_graph_linear_commits() {
        let mut rng = StdRng::seed_from_u64(42);

        let oid1 = Oid::random(&mut rng);
        let oid2 = Oid::random(&mut rng);
        let oid3 = Oid::random(&mut rng);

        let commits = vec![
            Arc::new(InitialGraphCommitData {
                sha: oid1,
                parents: smallvec![oid2],
                ref_names: vec!["HEAD".into()],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid2,
                parents: smallvec![oid3],
                ref_names: vec![],
            }),
            Arc::new(InitialGraphCommitData {
                sha: oid3,
                parents: smallvec![],
                ref_names: vec![],
            }),
        ];

        let mut graph_data = crate::graph::GraphData::new(8);
        graph_data.add_commits(&commits);

        if let Err(error) = verify_all_invariants(&graph_data, &commits) {
            panic!("Graph invariant violation for linear commits:\n{}", error);
        }
    }

    #[test]
    fn test_git_graph_random_commits() {
        for seed in 0..100 {
            let mut rng = StdRng::seed_from_u64(seed);

            let adversarial = rng.random_bool(0.2);
            let num_commits = if adversarial {
                rng.random_range(10..100)
            } else {
                rng.random_range(5..50)
            };

            let commits = generate_random_commit_dag(&mut rng, num_commits, adversarial);

            assert_eq!(
                num_commits,
                commits.len(),
                "seed={}: Generate random commit dag didn't generate the correct amount of commits",
                seed
            );

            let mut graph_data = crate::graph::GraphData::new(8);
            graph_data.add_commits(&commits);

            if let Err(error) = verify_all_invariants(&graph_data, &commits) {
                panic!(
                    "Graph invariant violation (seed={}, adversarial={}, num_commits={}):\n{:#}",
                    seed, adversarial, num_commits, error
                );
            }
        }
    }

    // The full integration test has less iterations because it's significantly slower
    // than the random commit test
    #[gpui::test(iterations = 5)]
    async fn test_git_graph_random_integration(mut rng: StdRng, cx: &mut TestAppContext) {
        init_test(cx);

        let adversarial = rng.random_bool(0.2);
        let num_commits = if adversarial {
            rng.random_range(10..100)
        } else {
            rng.random_range(5..50)
        };

        let commits = generate_random_commit_dag(&mut rng, num_commits, adversarial);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            Path::new("/project"),
            json!({
                ".git": {},
                "file.txt": "content",
            }),
        )
        .await;

        fs.set_graph_commits(Path::new("/project/.git"), commits.clone());

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        cx.run_until_parked();

        let repository = project.read_with(cx, |project, cx| {
            project
                .active_repository(cx)
                .expect("should have a repository")
        });

        repository.update(cx, |repo, cx| {
            repo.graph_data(
                crate::LogSource::default(),
                crate::LogOrder::default(),
                0..usize::MAX,
                cx,
            );
        });
        cx.run_until_parked();

        let graph_commits: Vec<Arc<InitialGraphCommitData>> = repository.update(cx, |repo, cx| {
            repo.graph_data(
                crate::LogSource::default(),
                crate::LogOrder::default(),
                0..usize::MAX,
                cx,
            )
            .to_vec()
        });

        let mut graph_data = crate::graph::GraphData::new(8);
        graph_data.add_commits(&graph_commits);

        if let Err(error) = verify_all_invariants(&graph_data, &commits) {
            panic!(
                "Graph invariant violation (adversarial={}, num_commits={}):\n{:#}",
                adversarial, num_commits, error
            );
        }
    }
}
