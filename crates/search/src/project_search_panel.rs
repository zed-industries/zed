use crate::{
    SearchOption, SearchOptions, ToggleCaseSensitive, ToggleRegex, ToggleWholeWord,
    project_search::{ProjectSearchView, split_glob_patterns},
    search_bar::{input_base_styles, render_text_input},
};
use editor::{Editor, actions::SelectAll};
use futures::{StreamExt, pin_mut};
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    IntoElement, KeyContext, ParentElement, Render, SharedString, Styled, Task, WeakEntity, Window,
    actions, div, px,
};
use language::{Anchor, Buffer, Point};
use menu::{Cancel, Confirm, SelectNext, SelectPrevious};
use project::{
    Project, ProjectPath,
    search::{SearchQuery, SearchResult},
};
use settings::Settings;
use std::ops::Range;
use ui::{CommonAnimationExt, HighlightedLabel, IconButtonShape, Tooltip, prelude::*};
use util::{ResultExt, paths::PathMatcher};
use workspace::{
    DeploySearch, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

const PROJECT_SEARCH_PANEL_KEY: &str = "ProjectSearchPanel";

actions!(
    project_search_panel,
    [
        /// Toggles focus on the project search panel.
        ToggleFocus,
        /// Opens the current project search in an editable multibuffer.
        OpenInEditor,
        /// Toggles the search filters panel.
        ToggleFilters,
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchState {
    Idle,
    Searching,
    NoResults,
    Results,
    LimitReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputPanel {
    Query,
    Include,
    Exclude,
}

#[derive(Clone)]
struct FileResults {
    path_label: SharedString,
    matches: Vec<PanelMatch>,
}

#[derive(Clone)]
struct PanelMatch {
    buffer: Entity<Buffer>,
    range: Range<Anchor>,
    line_number: u32,
    preview: SharedString,
    highlight_ranges: Vec<Range<usize>>,
}

pub struct ProjectSearchPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    query_editor: Entity<Editor>,
    included_files_editor: Entity<Editor>,
    excluded_files_editor: Entity<Editor>,
    search_options: SearchOptions,
    filters_enabled: bool,
    results: Vec<FileResults>,
    selected_match: Option<usize>,
    pending_search: Option<Task<()>>,
    search_state: SearchState,
    panels_with_errors: collections::HashMap<InputPanel, String>,
    active: bool,
    search_id: usize,
    _subscriptions: Vec<gpui::Subscription>,
}

impl ProjectSearchPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: gpui::AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        let project = workspace.read_with(&cx, |workspace, _| workspace.project().clone())?;
        workspace.update_in(&mut cx, |workspace, window, cx| {
            cx.new(|cx| Self::new(workspace.weak_handle(), project, window, cx))
        })
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let search_options =
            SearchOptions::from_settings(&editor::EditorSettings::get_global(cx).search);

        let query_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search all files...", window, cx);
            editor.set_use_autoclose(false);
            editor.set_use_selection_highlight(false);
            editor
        });

        let included_files_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("files to include", window, cx);
            editor
        });

        let excluded_files_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("files to exclude", window, cx);
            editor
        });

        let mut subscriptions = Vec::new();
        subscriptions.push(cx.subscribe(
            &query_editor,
            |this, _, event: &editor::EditorEvent, cx| {
                if matches!(event, editor::EditorEvent::Edited { .. }) {
                    this.search(cx);
                }
            },
        ));
        subscriptions.push(cx.subscribe(
            &included_files_editor,
            |this, _, event: &editor::EditorEvent, cx| {
                if matches!(event, editor::EditorEvent::Edited { .. }) && this.filters_enabled {
                    this.search(cx);
                }
            },
        ));
        subscriptions.push(cx.subscribe(
            &excluded_files_editor,
            |this, _, event: &editor::EditorEvent, cx| {
                if matches!(event, editor::EditorEvent::Edited { .. }) && this.filters_enabled {
                    this.search(cx);
                }
            },
        ));

        let focus_handle = cx.focus_handle();
        subscriptions.push(cx.on_focus(&focus_handle, window, |_, window, cx| {
            cx.on_next_frame(window, |this, window, cx| {
                if this.focus_handle.is_focused(window) {
                    this.query_editor.focus_handle(cx).focus(window, cx);
                }
            });
        }));

        Self {
            workspace,
            project,
            focus_handle,
            query_editor,
            included_files_editor,
            excluded_files_editor,
            search_options,
            filters_enabled: false,
            results: Vec::new(),
            selected_match: None,
            pending_search: None,
            search_state: SearchState::Idle,
            panels_with_errors: collections::HashMap::default(),
            active: false,
            search_id: 0,
            _subscriptions: subscriptions,
        }
    }

    pub fn deploy_search(
        &mut self,
        action: &DeploySearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(regex) = action.regex {
            self.set_search_option_enabled(SearchOptions::REGEX, regex, cx);
        }
        if let Some(case_sensitive) = action.case_sensitive {
            self.set_search_option_enabled(SearchOptions::CASE_SENSITIVE, case_sensitive, cx);
        }
        if let Some(whole_word) = action.whole_word {
            self.set_search_option_enabled(SearchOptions::WHOLE_WORD, whole_word, cx);
        }
        if let Some(include_ignored) = action.include_ignored {
            self.set_search_option_enabled(SearchOptions::INCLUDE_IGNORED, include_ignored, cx);
        }
        if let Some(query) = action.query.as_deref().filter(|query| !query.is_empty()) {
            self.query_editor
                .update(cx, |editor, cx| editor.set_text(query, window, cx));
        }
        if let Some(included_files) = action.included_files.as_deref() {
            self.included_files_editor
                .update(cx, |editor, cx| editor.set_text(included_files, window, cx));
            self.filters_enabled = true;
        }
        if let Some(excluded_files) = action.excluded_files.as_deref() {
            self.excluded_files_editor
                .update(cx, |editor, cx| editor.set_text(excluded_files, window, cx));
            self.filters_enabled = true;
        }

        self.query_editor.update(cx, |editor, cx| {
            editor.select_all(&SelectAll, window, cx);
        });
        self.query_editor.focus_handle(cx).focus(window, cx);
        self.search(cx);
    }

    fn set_search_option_enabled(
        &mut self,
        option: SearchOptions,
        enabled: bool,
        cx: &mut Context<Self>,
    ) {
        if self.search_options.contains(option) != enabled {
            self.search_options.toggle(option);
            cx.notify();
        }
    }

    fn toggle_search_option(&mut self, option: SearchOptions, cx: &mut Context<Self>) {
        self.search_options.toggle(option);
        self.search(cx);
        cx.notify();
    }

    fn toggle_filters(&mut self, cx: &mut Context<Self>) {
        self.filters_enabled = !self.filters_enabled;
        self.search(cx);
        cx.notify();
    }

    fn search(&mut self, cx: &mut Context<Self>) {
        let Some(query) = self.build_search_query(cx) else {
            self.pending_search = None;
            self.results.clear();
            self.selected_match = None;
            self.search_state = if self.query_text(cx).is_empty() {
                SearchState::Idle
            } else {
                SearchState::NoResults
            };
            cx.notify();
            return;
        };

        let search = self
            .project
            .update(cx, |project, cx| project.search(query, cx));
        self.search_id += 1;
        let search_id = self.search_id;
        self.results.clear();
        self.selected_match = None;
        self.search_state = SearchState::Searching;
        self.pending_search = Some(cx.spawn(async move |this, cx| {
            let project::SearchResults { rx, _task_handle } = search;
            let chunks = rx.ready_chunks(256);
            pin_mut!(chunks);
            let mut limit_reached = false;

            while let Some(results) = chunks.next().await {
                let mut file_results = Vec::new();
                for result in results {
                    match result {
                        SearchResult::Buffer { buffer, ranges } => {
                            if let Some(result) =
                                cx.update(|cx| build_file_results(buffer, ranges, cx))
                            {
                                file_results.push(result);
                            }
                        }
                        SearchResult::LimitReached => {
                            limit_reached = true;
                        }
                        SearchResult::WaitingForScan | SearchResult::Searching => {}
                    }
                }

                if file_results.is_empty() {
                    continue;
                }

                this.update(cx, |this, cx| {
                    if this.search_id != search_id {
                        return;
                    }
                    let had_results = !this.results.is_empty();
                    this.results.extend(file_results);
                    if !had_results && !this.results.is_empty() {
                        this.selected_match = Some(0);
                    }
                    this.search_state = SearchState::Results;
                    cx.notify();
                })
                .log_err();
            }

            this.update(cx, |this, cx| {
                if this.search_id != search_id {
                    return;
                }
                this.pending_search = None;
                this.search_state = if this.results.is_empty() {
                    SearchState::NoResults
                } else if limit_reached {
                    SearchState::LimitReached
                } else {
                    SearchState::Results
                };
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    fn build_search_query(&mut self, cx: &mut Context<Self>) -> Option<SearchQuery> {
        let text = self.query_text(cx);
        if text.is_empty() {
            self.panels_with_errors.clear();
            return None;
        }

        let included_files = if self.filters_enabled {
            match self.parse_path_matches(self.included_files_editor.read(cx).text(cx), cx) {
                Ok(matcher) => {
                    self.panels_with_errors.remove(&InputPanel::Include);
                    matcher
                }
                Err(error) => {
                    self.panels_with_errors
                        .insert(InputPanel::Include, error.to_string());
                    PathMatcher::default()
                }
            }
        } else {
            PathMatcher::default()
        };

        let excluded_files = if self.filters_enabled {
            match self.parse_path_matches(self.excluded_files_editor.read(cx).text(cx), cx) {
                Ok(matcher) => {
                    self.panels_with_errors.remove(&InputPanel::Exclude);
                    matcher
                }
                Err(error) => {
                    self.panels_with_errors
                        .insert(InputPanel::Exclude, error.to_string());
                    PathMatcher::default()
                }
            }
        } else {
            PathMatcher::default()
        };

        let match_full_paths = self.project.read(cx).visible_worktrees(cx).count() > 1;
        let query = if self.search_options.contains(SearchOptions::REGEX) {
            SearchQuery::regex(
                text,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                false,
                included_files,
                excluded_files,
                match_full_paths,
                None,
            )
        } else {
            SearchQuery::text(
                text,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                included_files,
                excluded_files,
                match_full_paths,
                None,
            )
        };

        match query {
            Ok(query) => {
                self.panels_with_errors.remove(&InputPanel::Query);
                if self.panels_with_errors.is_empty() {
                    Some(query)
                } else {
                    None
                }
            }
            Err(error) => {
                self.panels_with_errors
                    .insert(InputPanel::Query, error.to_string());
                None
            }
        }
    }

    fn parse_path_matches(&self, text: String, cx: &App) -> anyhow::Result<PathMatcher> {
        let path_style = self.project.read(cx).path_style(cx);
        let queries = split_glob_patterns(&text)
            .into_iter()
            .map(str::trim)
            .filter(|pattern| !pattern.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        Ok(PathMatcher::new(&queries, path_style)?)
    }

    fn query_text(&self, cx: &App) -> String {
        self.query_editor.read(cx).text(cx)
    }

    fn match_count(&self) -> usize {
        self.results
            .iter()
            .map(|file_result| file_result.matches.len())
            .sum()
    }

    fn selected_match_location(&self) -> Option<(usize, usize)> {
        let mut offset = self.selected_match?;
        for (file_index, file_result) in self.results.iter().enumerate() {
            if offset < file_result.matches.len() {
                return Some((file_index, offset));
            }
            offset = offset.saturating_sub(file_result.matches.len());
        }
        None
    }

    fn match_offset(&self, file_index: usize, match_index: usize) -> usize {
        self.results
            .iter()
            .take(file_index)
            .map(|file_result| file_result.matches.len())
            .sum::<usize>()
            + match_index
    }

    fn select_previous(&mut self, cx: &mut Context<Self>) {
        let match_count = self.match_count();
        if match_count == 0 {
            self.selected_match = None;
        } else {
            self.selected_match = Some(match self.selected_match {
                Some(0) | None => match_count - 1,
                Some(index) => index.saturating_sub(1),
            });
        }
        cx.notify();
    }

    fn select_next(&mut self, cx: &mut Context<Self>) {
        let match_count = self.match_count();
        if match_count == 0 {
            self.selected_match = None;
        } else {
            self.selected_match = Some(match self.selected_match {
                Some(index) => (index + 1) % match_count,
                None => 0,
            });
        }
        cx.notify();
    }

    fn open_selected_match(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((file_index, match_index)) = self.selected_match_location() {
            self.open_match(file_index, match_index, window, cx);
        }
    }

    fn open_match(
        &mut self,
        file_index: usize,
        match_index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(search_match) = self
            .results
            .get(file_index)
            .and_then(|file_result| file_result.matches.get(match_index))
            .cloned()
        else {
            return;
        };

        let PanelMatch { buffer, range, .. } = search_match;
        self.workspace
            .update(cx, |workspace, cx| {
                let pane = workspace.active_pane().clone();
                let editor = workspace.open_project_item::<Editor>(
                    pane,
                    buffer.clone(),
                    true,
                    true,
                    false,
                    true,
                    window,
                    cx,
                );
                editor.update(cx, |editor, cx| {
                    let snapshot = buffer.read(cx).snapshot();
                    let start = snapshot.summary_for_anchor::<Point>(&range.start);
                    let end = snapshot.summary_for_anchor::<Point>(&range.end);
                    editor.go_to_singleton_buffer_range(start..end, window, cx);
                });
            })
            .log_err();
    }

    fn open_in_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let action = DeploySearch {
            query: Some(self.query_text(cx)),
            regex: Some(self.search_options.contains(SearchOptions::REGEX)),
            case_sensitive: Some(self.search_options.contains(SearchOptions::CASE_SENSITIVE)),
            whole_word: Some(self.search_options.contains(SearchOptions::WHOLE_WORD)),
            include_ignored: Some(self.search_options.contains(SearchOptions::INCLUDE_IGNORED)),
            included_files: self
                .filters_enabled
                .then(|| self.included_files_editor.read(cx).text(cx)),
            excluded_files: self
                .filters_enabled
                .then(|| self.excluded_files_editor.read(cx).text(cx)),
            replace_enabled: false,
        };
        self.workspace
            .update(cx, |workspace, cx| {
                ProjectSearchView::deploy_search(workspace, &action, window, cx);
            })
            .log_err();
    }

    fn render_input_row(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let query_focus = self.query_editor.focus_handle(cx);
        let input = input_base_styles(self.border_color_for(InputPanel::Query, cx), |div| {
            div.flex_1()
        })
        .child(
            div()
                .flex_1()
                .py_1()
                .child(render_text_input(&self.query_editor, None, cx)),
        )
        .child(
            h_flex()
                .gap_1()
                .child(self.render_search_option_button(SearchOption::CaseSensitive, cx))
                .child(self.render_search_option_button(SearchOption::WholeWord, cx))
                .child(self.render_search_option_button(SearchOption::Regex, cx)),
        );

        h_flex()
            .gap_1()
            .child(input)
            .child(
                IconButton::new("project-search-panel-filters", IconName::Filter)
                    .shape(IconButtonShape::Square)
                    .toggle_state(self.filters_enabled)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action_in("Toggle Filters", &ToggleFilters, &query_focus, cx)
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_filters(cx);
                    })),
            )
            .child(
                IconButton::new(
                    "project-search-panel-open-editor",
                    IconName::FileTextOutlined,
                )
                .shape(IconButtonShape::Square)
                .tooltip(|_window, cx| {
                    Tooltip::for_action("Open Results in Editor", &OpenInEditor, cx)
                })
                .on_click(cx.listener(|this, _, window, cx| {
                    this.open_in_editor(window, cx);
                })),
            )
    }

    fn render_search_option_button(
        &self,
        option: SearchOption,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        IconButton::new(
            ("project-search-panel-option", option as usize),
            option.icon(),
        )
        .shape(IconButtonShape::Square)
        .toggle_state(self.search_options.contains(option.as_options()))
        .tooltip(Tooltip::text(option.label()))
        .on_click(cx.listener(move |this, _, _, cx| {
            this.toggle_search_option(option.as_options(), cx);
        }))
    }

    fn render_filter_row(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        self.filters_enabled.then(|| {
            h_flex()
                .gap_1()
                .child(
                    input_base_styles(self.border_color_for(InputPanel::Include, cx), |div| {
                        div.flex_1()
                    })
                    .child(render_text_input(
                        &self.included_files_editor,
                        None,
                        cx,
                    )),
                )
                .child(
                    input_base_styles(self.border_color_for(InputPanel::Exclude, cx), |div| {
                        div.flex_1()
                    })
                    .child(render_text_input(
                        &self.excluded_files_editor,
                        None,
                        cx,
                    )),
                )
                .child(self.render_search_option_button(SearchOption::IncludeIgnored, cx))
        })
    }

    fn render_results(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("project-search-panel-results")
            .flex_1()
            .overflow_y_scroll()
            .children(
                self.results
                    .iter()
                    .enumerate()
                    .map(|(file_index, file_result)| {
                        let file_header = h_flex()
                            .px_2()
                            .py_1()
                            .gap_1()
                            .child(Icon::new(IconName::File).size(IconSize::Small))
                            .child(
                                Label::new(file_result.path_label.clone()).size(LabelSize::Small),
                            )
                            .child(
                                Label::new(file_result.matches.len().to_string())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            );

                        let matches = file_result.matches.iter().enumerate().map(
                            |(match_index, search_match)| {
                                let selected = self.selected_match
                                    == Some(self.match_offset(file_index, match_index));
                                h_flex()
                                    .id((
                                        "project-search-panel-match",
                                        self.match_offset(file_index, match_index),
                                    ))
                                    .px_3()
                                    .py_1()
                                    .gap_2()
                                    .rounded_sm()
                                    .when(selected, |this| {
                                        this.bg(cx.theme().colors().element_selected)
                                    })
                                    .child(
                                        div().min_w(px(32.)).child(
                                            Label::new(search_match.line_number.to_string())
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        ),
                                    )
                                    .child(
                                        HighlightedLabel::from_ranges(
                                            search_match.preview.clone(),
                                            search_match.highlight_ranges.clone(),
                                        )
                                        .size(LabelSize::Small)
                                        .truncate(),
                                    )
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        this.selected_match =
                                            Some(this.match_offset(file_index, match_index));
                                        this.open_match(file_index, match_index, window, cx);
                                    }))
                            },
                        );

                        v_flex().child(file_header).children(matches)
                    }),
            )
    }

    fn render_message(&self, cx: &App) -> impl IntoElement {
        let (title, detail) = match self.search_state {
            SearchState::Idle => ("Search All Files", "Type a query to search the project."),
            SearchState::Searching => ("Searching...", "Results will appear here."),
            SearchState::NoResults => ("No Results", "No files matched the current query."),
            SearchState::Results | SearchState::LimitReached => ("", ""),
        };

        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_1()
            .child(Label::new(title).size(LabelSize::Large))
            .child(
                Label::new(detail)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .when(self.search_state == SearchState::Searching, |this| {
                this.child(
                    Icon::new(IconName::ArrowCircle)
                        .color(Color::Accent)
                        .with_rotate_animation(2),
                )
            })
            .when(self.search_state == SearchState::LimitReached, |this| {
                this.child(
                    Label::new("Search limits reached. Narrow the query to see fewer results.")
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                )
            })
            .bg(cx.theme().colors().editor_background)
    }

    fn border_color_for(&self, panel: InputPanel, cx: &App) -> gpui::Hsla {
        if self.panels_with_errors.contains_key(&panel) {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border
        }
    }
}

fn build_file_results(
    buffer: Entity<Buffer>,
    ranges: Vec<Range<Anchor>>,
    cx: &mut App,
) -> Option<FileResults> {
    let buffer_snapshot = buffer.read(cx).snapshot();
    let file = buffer_snapshot.file()?;
    let project_path = ProjectPath {
        worktree_id: file.worktree_id(cx),
        path: file.path().clone(),
    };
    let path_label: SharedString = project_path
        .path
        .display(file.path_style(cx))
        .to_string()
        .into();

    let matches = ranges
        .into_iter()
        .map(|range| {
            let start = buffer_snapshot.summary_for_anchor::<Point>(&range.start);
            let end = buffer_snapshot.summary_for_anchor::<Point>(&range.end);
            let line_start = Point::new(start.row, 0);
            let line_end = Point::new(start.row, buffer_snapshot.line_len(start.row));
            let preview = buffer_snapshot
                .text_for_range(line_start..line_end)
                .collect::<String>();
            let highlight_end = if start.row == end.row {
                end.column
            } else {
                buffer_snapshot.line_len(start.row)
            };
            PanelMatch {
                buffer: buffer.clone(),
                range,
                line_number: start.row + 1,
                preview: preview.into(),
                highlight_ranges: vec![start.column as usize..highlight_end as usize],
            }
        })
        .collect();

    Some(FileResults {
        path_label,
        matches,
    })
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<ProjectSearchPanel>(window, cx);
        });
    })
    .detach();
}

impl Render for ProjectSearchPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::default();
        key_context.add("ProjectSearchPanel");

        let errors = self.panels_with_errors.values().next().map(|error| {
            Label::new(error.clone())
                .size(LabelSize::Small)
                .color(Color::Error)
        });

        v_flex()
            .key_context(key_context)
            .track_focus(&self.focus_handle)
            .size_full()
            .gap_2()
            .p_2()
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(|this, _: &Confirm, window, cx| {
                this.open_selected_match(window, cx);
            }))
            .on_action(cx.listener(|this, _: &Cancel, window, cx| {
                if let Some(workspace) = this.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        workspace.focus_center_pane(window, cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &SelectPrevious, _, cx| {
                this.select_previous(cx);
            }))
            .on_action(cx.listener(|this, _: &SelectNext, _, cx| {
                this.select_next(cx);
            }))
            .on_action(cx.listener(|this, _: &OpenInEditor, window, cx| {
                this.open_in_editor(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleFilters, _, cx| {
                this.toggle_filters(cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleCaseSensitive, _, cx| {
                this.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleWholeWord, _, cx| {
                this.toggle_search_option(SearchOptions::WHOLE_WORD, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleRegex, _, cx| {
                this.toggle_search_option(SearchOptions::REGEX, cx);
            }))
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Search").size(LabelSize::Default))
                    .child(
                        Label::new(format!("{} results", self.match_count()))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(self.render_input_row(cx))
            .children(self.render_filter_row(cx))
            .children(errors)
            .when(self.results.is_empty(), |this| {
                this.child(self.render_message(cx))
            })
            .when(!self.results.is_empty(), |this| {
                this.child(self.render_results(cx))
            })
    }
}

impl Focusable for ProjectSearchPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for ProjectSearchPanel {}

impl Panel for ProjectSearchPanel {
    fn persistent_name() -> &'static str {
        "Project Search"
    }

    fn panel_key() -> &'static str {
        PROJECT_SEARCH_PANEL_KEY
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(
        &mut self,
        _position: DockPosition,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn default_size(&self, _: &Window, _: &App) -> gpui::Pixels {
        px(360.)
    }

    fn icon(&self, _: &Window, _: &App) -> Option<IconName> {
        Some(IconName::MagnifyingGlass)
    }

    fn icon_tooltip(&self, _window: &Window, _: &App) -> Option<&'static str> {
        Some("Project Search")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _window: &Window, _: &App) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool, _window: &mut Window, cx: &mut Context<Self>) {
        self.active = active;
        cx.notify();
    }

    fn activation_priority(&self) -> u32 {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext};
    use pretty_assertions::assert_eq;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use workspace::MultiWorkspace;

    #[gpui::test]
    async fn test_deploy_search_updates_panel_results(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const NEEDLE: usize = 1;",
                "two.rs": "const TWO: usize = NEEDLE;",
                "three.rs": "const THREE: usize = 3;",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .expect("workspace should exist");
        let cx = &mut VisualTestContext::from_window(window.into(), cx);

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| {
                ProjectSearchPanel::new(workspace.weak_handle(), project.clone(), window, cx)
            });
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.focus_panel::<ProjectSearchPanel>(window, cx);
            panel.update(cx, |panel, cx| {
                panel.deploy_search(
                    &DeploySearch {
                        query: Some("NEEDLE".into()),
                        ..Default::default()
                    },
                    window,
                    cx,
                );
            });
        });
        cx.run_until_parked();

        panel.read_with(cx, |panel, cx| {
            assert_eq!(panel.query_text(cx), "NEEDLE");
            assert_eq!(panel.match_count(), 2);
            assert_eq!(panel.selected_match, Some(0));
            assert_eq!(panel.search_state, SearchState::Results);
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            theme_settings::init(theme::LoadThemes::JustBase, cx);

            editor::init(cx);
            crate::init(cx);
        });
    }
}
