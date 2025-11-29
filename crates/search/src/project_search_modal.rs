use crate::{SearchOption, SearchOptions, project_search::ProjectSearch};
use editor::{Editor, EditorEvent, EditorSettings};
use futures::StreamExt as _;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, ParentElement, PromptLevel, Render, SharedString, Styled, Task, WeakEntity,
    Window, prelude::*, rems,
};
use language::{Point, ToPoint as _};
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectPath, search::SearchQuery};
use project::search::SearchResult;
use settings::Settings;
use std::{mem, pin::pin, sync::Arc};
use theme::ThemeSettings;
use ui::{
    ButtonStyle, Divider, HighlightedLabel, IconButton, IconButtonShape, ListItem, ListItemSpacing,
    Tooltip, prelude::*,
};
use util::{ResultExt, paths::PathMatcher};
use workspace::{DismissDecision, Item, ModalView, Workspace, WorkspaceSettings, searchable::SearchableItem};

const MAX_MATCHES: usize = 100;

pub fn init(cx: &mut App) {
    cx.observe_new(ProjectSearchModal::register).detach();
}

pub struct ProjectSearchModal {
    picker: Entity<Picker<ProjectSearchModalDelegate>>,
    _subscriptions: Vec<gpui::Subscription>,
    pending_dismiss: bool,
}

impl ProjectSearchModal {
    fn register(workspace: &mut Workspace, _window: Option<&mut Window>, _cx: &mut Context<Workspace>) {
        workspace.register_action(|workspace, _: &crate::ToggleProjectSearchModal, window, cx| {
            let weak_workspace = workspace.weak_handle();
            let project = workspace.project().clone();
            workspace.toggle_modal(window, cx, |window, cx| {
                ProjectSearchModal::new(weak_workspace, project, window, cx)
            });
        });
    }

    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ProjectSearchModalDelegate::new(workspace, project, window, cx);

        let included_files_editor = delegate.included_files_editor.clone();
        let excluded_files_editor = delegate.excluded_files_editor.clone();

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .modal(false)
                .max_height(Some(rems(24.).into()))
        });

        let picker_subscription = cx.subscribe_in(&picker, window, Self::on_picker_event);

        let included_subscription = cx.subscribe_in(&included_files_editor, window, {
            let picker = picker.clone();
            move |_this, _editor, event: &EditorEvent, window, cx| {
                if matches!(event, EditorEvent::BufferEdited { .. }) {
                    picker.update(cx, |picker, cx| {
                        let query = picker.query(cx);
                        picker.delegate.update_matches(query, window, cx).detach();
                    });
                }
            }
        });

        let excluded_subscription = cx.subscribe_in(&excluded_files_editor, window, {
            let picker = picker.clone();
            move |_this, _editor, event: &EditorEvent, window, cx| {
                if matches!(event, EditorEvent::BufferEdited { .. }) {
                    picker.update(cx, |picker, cx| {
                        let query = picker.query(cx);
                        picker.delegate.update_matches(query, window, cx).detach();
                    });
                }
            }
        });

        Self {
            picker,
            _subscriptions: vec![picker_subscription, included_subscription, excluded_subscription],
            pending_dismiss: false,
        }
    }

    fn on_picker_event(
        &mut self,
        _picker: &Entity<Picker<ProjectSearchModalDelegate>>,
        _event: &DismissEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DismissEvent);
    }

    fn has_unsaved_changes(&self, cx: &App) -> bool {
        self.picker.read(cx).delegate.results_editor.read(cx).is_dirty(cx)
    }

    fn will_autosave(&self, cx: &App) -> bool {
        let autosave_setting = WorkspaceSettings::get_global(cx).autosave;
        autosave_setting.should_save_on_close()
    }

    fn save_all(&self, cx: &mut Context<Self>) {
        let project = self.picker.read(cx).delegate.project.clone();
        let results_editor = self.picker.read(cx).delegate.results_editor.clone();
        let multi_buffer = results_editor.read(cx).buffer().clone();

        multi_buffer.update(cx, |multi_buffer, cx| {
            let buffers = multi_buffer.all_buffers();
            for buffer in buffers {
                if buffer.read(cx).is_dirty() {
                    project.update(cx, |project, cx| {
                        project.save_buffer(buffer.clone(), cx).detach_and_log_err(cx);
                    });
                }
            }
        });
    }

    fn show_save_prompt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let answer = window.prompt(
            PromptLevel::Warning,
            "Search results contain unsaved edits. Do you want to save?",
            None,
            &["Save", "Don't Save", "Cancel"],
            cx,
        );

        cx.spawn_in(window, async move |this, cx| {
            match answer.await {
                Ok(0) => {
                    this.update(cx, |this, cx| {
                        this.save_all(cx);
                        cx.emit(DismissEvent);
                    }).log_err();
                }
                Ok(1) => {
                    this.update(cx, |_, cx| {
                        cx.emit(DismissEvent);
                    }).log_err();
                }
                _ => {
                    this.update(cx, |this, _cx| {
                        this.pending_dismiss = false;
                    }).log_err();
                }
            }
        }).detach();
    }
}

impl ModalView for ProjectSearchModal {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DismissDecision {
        if self.pending_dismiss {
            return DismissDecision::Dismiss(true);
        }

        let is_dirty = self.has_unsaved_changes(cx);

        if !is_dirty {
            return DismissDecision::Dismiss(true);
        }

        let will_autosave = self.will_autosave(cx);

        if will_autosave {
            self.save_all(cx);
            return DismissDecision::Dismiss(true);
        }

        self.pending_dismiss = true;
        self.show_save_prompt(window, cx);
        DismissDecision::Pending
    }
}

impl EventEmitter<DismissEvent> for ProjectSearchModal {}

impl Focusable for ProjectSearchModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ProjectSearchModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .key_context("ProjectSearchModal")
            .elevation_3(cx)
            .w(rems(48.))
            .max_h(rems(32.))
            .overflow_hidden()
            .child(
                v_flex()
                    .w_full()
                    .h_full()
                    .overflow_hidden()
                    .child(self.picker.clone())
            )
    }
}

pub struct ProjectSearchModalDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    matches: Vec<SearchMatch>,
    selected_index: usize,
    search_options: SearchOptions,
    query: String,
    filters_enabled: bool,
    included_files_editor: Entity<Editor>,
    excluded_files_editor: Entity<Editor>,
    replace_enabled: bool,
    replacement_editor: Entity<Editor>,
    project_search: Entity<ProjectSearch>,
    results_editor: Entity<Editor>,
    active_query: Option<SearchQuery>,
}

#[derive(Clone)]
pub struct SearchMatch {
    pub path: Option<ProjectPath>,
    pub line_number: u32,
    pub line_text: String,
    pub highlight_ranges: Vec<usize>,
}

impl ProjectSearchModalDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<ProjectSearchModal>,
    ) -> Self {
        let search_settings = EditorSettings::get_global(cx).search;
        let search_options = SearchOptions::from_settings(&search_settings);

        let included_files_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Include files (e.g. *.rs, src/**)", window, cx);
            editor
        });

        let excluded_files_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Exclude files (e.g. node_modules)", window, cx);
            editor
        });

        let replacement_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Replace with...", window, cx);
            editor
        });

        let project_search = cx.new(|cx| ProjectSearch::new(project.clone(), cx));

        let excerpts = project_search.read(cx).excerpts.clone();
        let results_editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(excerpts, Some(project.clone()), window, cx);
            editor.set_searchable(false);
            editor
        });

        Self {
            workspace,
            project,
            matches: Vec::new(),
            selected_index: 0,
            search_options,
            query: String::new(),
            filters_enabled: false,
            included_files_editor,
            excluded_files_editor,
            replace_enabled: false,
            replacement_editor,
            project_search,
            results_editor,
            active_query: None,
        }
    }


    fn open_selected_match(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(search_match) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(project_path) = search_match.path.clone() else {
            return;
        };

        let position = Point::new(search_match.line_number - 1, 0);

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                let open_task = workspace.open_path_preview(
                    project_path,
                    None,
                    true,
                    false,
                    true,
                    window,
                    cx,
                );

                cx.spawn_in(window, async move |_, cx| {
                    if let Some(editor) = open_task.await.log_err().and_then(|item| item.downcast::<Editor>()) {
                        editor.update_in(cx, |editor, window, cx| {
                            editor.go_to_singleton_buffer_point(position, window, cx);
                        }).log_err();
                    }
                }).detach();
            });
        }
    }

    fn render_option_button(
        &self,
        option: SearchOption,
        cx: &mut Context<Picker<Self>>,
    ) -> impl IntoElement {
        let is_active = self.search_options.contains(option.as_options());
        let options = option.as_options();

        IconButton::new(option.label(), option.icon())
            .style(ButtonStyle::Subtle)
            .shape(IconButtonShape::Square)
            .toggle_state(is_active)
            .tooltip(Tooltip::text(option.label()))
            .on_click(cx.listener(move |picker, _, window, cx| {
                picker.delegate.search_options.toggle(options);
                let query = picker.query(cx);
                picker.delegate.update_matches(query, window, cx).detach();
            }))
    }

    fn render_filter_button(&self, cx: &mut Context<Picker<Self>>) -> impl IntoElement {
        IconButton::new("project-search-filter-button", ui::IconName::Filter)
            .style(ButtonStyle::Subtle)
            .shape(IconButtonShape::Square)
            .toggle_state(self.filters_enabled)
            .tooltip(Tooltip::text("Toggle Filters"))
            .on_click(cx.listener(|picker, _, _, cx| {
                picker.delegate.filters_enabled = !picker.delegate.filters_enabled;
                cx.notify();
            }))
    }

    fn render_replace_button(&self, cx: &mut Context<Picker<Self>>) -> impl IntoElement {
        IconButton::new("project-search-replace-button", ui::IconName::Replace)
            .style(ButtonStyle::Subtle)
            .shape(IconButtonShape::Square)
            .toggle_state(self.replace_enabled)
            .tooltip(Tooltip::text("Toggle Replace"))
            .on_click(cx.listener(|picker, _, _, cx| {
                picker.delegate.replace_enabled = !picker.delegate.replace_enabled;
                cx.notify();
            }))
    }

    fn replacement(&self, cx: &App) -> String {
        self.replacement_editor.read(cx).text(cx)
    }

    fn replace_next(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(query) = self.active_query.clone() else {
            return;
        };

        let match_ranges = self.project_search.read(cx).match_ranges.clone();

        if match_ranges.is_empty() {
            self.project_search.update(cx, |project_search, cx| {
                project_search.search(query.clone(), cx);
            });
            return;
        }

        if self.selected_index >= match_ranges.len() {
            return;
        }

        let query = query.with_replacement(self.replacement(cx));

        let match_range = match_ranges[self.selected_index].clone();
        self.results_editor.update(cx, |editor, cx| {
            editor.replace(&match_range, &query, window, cx);
        });

        if self.selected_index + 1 < self.matches.len() {
            self.selected_index += 1;
        }
        cx.notify();
    }

    fn replace_all(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(query) = self.active_query.clone() else {
            return;
        };

        let match_ranges = self.project_search.read(cx).match_ranges.clone();

        if match_ranges.is_empty() {
            self.project_search.update(cx, |project_search, cx| {
                project_search.search(query.clone(), cx);
            });
            return;
        }

        let query = query.with_replacement(self.replacement(cx));

        let match_ranges = self
            .project_search
            .update(cx, |model, _| mem::take(&mut model.match_ranges));

        self.results_editor.update(cx, |editor, cx| {
            editor.replace_all(&mut match_ranges.iter(), &query, window, cx);
        });

        self.project_search.update(cx, |model, _cx| {
            model.match_ranges = match_ranges;
        });

        self.matches.clear();
        self.selected_index = 0;
        cx.notify();
    }

    fn parse_path_matches(&self, text: String, cx: &App) -> anyhow::Result<PathMatcher> {
        let path_style = self.project.read(cx).path_style(cx);
        let queries = text
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        Ok(PathMatcher::new(&queries, path_style)?)
    }

}

impl PickerDelegate for ProjectSearchModalDelegate {
    type ListItem = ListItem;

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
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search all files...".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No matches found".into())
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Div {
        v_flex()
            .child(
                h_flex()
                    .overflow_hidden()
                    .flex_none()
                    .h_9()
                    .px_2p5()
                    .child(div().flex_1().child(editor.clone()))
                    .child(
                        h_flex()
                            .flex_shrink_0()
                            .gap_1()
                            .child(self.render_option_button(SearchOption::CaseSensitive, cx))
                            .child(self.render_option_button(SearchOption::WholeWord, cx))
                            .child(self.render_option_button(SearchOption::Regex, cx))
                            .child(self.render_option_button(SearchOption::IncludeIgnored, cx))
                            .child(self.render_filter_button(cx))
                            .child(self.render_replace_button(cx)),
                    ),
            )
            .when(self.replace_enabled, |this| {
                this.child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .px_2p5()
                        .py_1()
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .border_1()
                                .border_color(cx.theme().colors().border)
                                .rounded_md()
                                .child(self.replacement_editor.clone()),
                        )
                        .child(
                            h_flex()
                                .flex_shrink_0()
                                .gap_1()
                                .child(
                                    IconButton::new("replace-next", ui::IconName::ReplaceNext)
                                        .style(ButtonStyle::Subtle)
                                        .shape(IconButtonShape::Square)
                                        .tooltip(Tooltip::text("Replace Next"))
                                        .on_click(cx.listener(|picker, _, window, cx| {
                                            picker.delegate.replace_next(window, cx);
                                        })),
                                )
                                .child(
                                    IconButton::new("replace-all", ui::IconName::ReplaceAll)
                                        .style(ButtonStyle::Subtle)
                                        .shape(IconButtonShape::Square)
                                        .tooltip(Tooltip::text("Replace All"))
                                        .on_click(cx.listener(|picker, _, window, cx| {
                                            picker.delegate.replace_all(window, cx);
                                        })),
                                ),
                        ),
                )
            })
            .when(self.filters_enabled, |this| {
                this.child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .px_2p5()
                        .py_1()
                        .child(
                            h_flex()
                                .flex_1()
                                .gap_2()
                                .child(
                                    div()
                                        .flex_1()
                                        .px_2()
                                        .py_1()
                                        .border_1()
                                        .border_color(cx.theme().colors().border)
                                        .rounded_md()
                                        .child(self.included_files_editor.clone()),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .px_2()
                                        .py_1()
                                        .border_1()
                                        .border_color(cx.theme().colors().border)
                                        .rounded_md()
                                        .child(self.excluded_files_editor.clone()),
                                ),
                        ),
                )
            })
            .child(Divider::horizontal())
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.matches.clear();
            self.selected_index = 0;
            self.query.clear();
            self.active_query = None;
            cx.notify();
            return Task::ready(());
        }

        self.query = query.clone();

        let included_text = self.included_files_editor.read(cx).text(cx);
        let excluded_text = self.excluded_files_editor.read(cx).text(cx);

        let include_matcher = self
            .parse_path_matches(included_text, cx)
            .unwrap_or_default();
        let exclude_matcher = self
            .parse_path_matches(excluded_text, cx)
            .unwrap_or_default();

        let search_query = match SearchQuery::text(
            &query,
            self.search_options.contains(SearchOptions::WHOLE_WORD),
            self.search_options.contains(SearchOptions::CASE_SENSITIVE),
            self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
            include_matcher,
            exclude_matcher,
            false,
            None,
        ) {
            Ok(query) => query,
            Err(_) => {
                self.matches.clear();
                self.selected_index = 0;
                self.active_query = None;
                cx.notify();
                return Task::ready(());
            }
        };

        self.active_query = Some(search_query.clone());
        self.matches.clear();
        self.selected_index = 0;

        let search = self.project.update(cx, |project, cx| {
            project.search(search_query, cx)
        });

        cx.spawn_in(window, async move |picker, cx| {
            let mut search_stream = pin!(search.ready_chunks(1024));
            let mut limit_reached = false;

            while let Some(results) = search_stream.next().await {
                if limit_reached {
                    break;
                }

                let should_stop = picker.update_in(cx, |picker, _, cx| {

                    for result in results {
                        if picker.delegate.matches.len() >= MAX_MATCHES {
                            break;
                        }

                        match result {
                            SearchResult::Buffer { buffer, ranges } => {
                                let buffer_ref = buffer.read(cx);
                                let path = buffer_ref.file().map(|file| ProjectPath {
                                    worktree_id: file.worktree_id(cx),
                                    path: file.path().clone(),
                                });

                                // Track which lines we've already added from this buffer
                                // to avoid duplicates on the same line
                                let mut seen_lines = std::collections::HashSet::new();

                                for range in ranges.iter() {
                                    if picker.delegate.matches.len() >= MAX_MATCHES {
                                        break;
                                    }

                                    let start_point = range.start.to_point(&buffer_ref);
                                    let end_point = range.end.to_point(&buffer_ref);
                                    let line_row = start_point.row;

                                    // Skip if we've already added this line
                                    if !seen_lines.insert(line_row) {
                                        continue;
                                    }

                                    let match_len = (end_point.column.saturating_sub(start_point.column)) as usize;

                                    // For very long lines (like minified JSON), show context around the match
                                    let line_len = buffer_ref.line_len(line_row);
                                    const MAX_LINE_LEN: u32 = 200;
                                    const CONTEXT_CHARS: u32 = 50;

                                    let (line_text, highlight_ranges) = if line_len > MAX_LINE_LEN {
                                        // Show context around the match
                                        let context_start = start_point.column.saturating_sub(CONTEXT_CHARS);
                                        let context_end = (start_point.column + match_len as u32 + CONTEXT_CHARS).min(line_len);

                                        let range_start = Point::new(line_row, context_start);
                                        let range_end = Point::new(line_row, context_end);
                                        let context_text = buffer_ref
                                            .text_for_range(range_start..range_end)
                                            .collect::<String>();

                                        // Highlight position relative to context start
                                        let highlight_start = (start_point.column - context_start) as usize;
                                        let highlight_end = highlight_start + match_len;
                                        let highlights: Vec<usize> = (highlight_start..highlight_end).collect();

                                        let prefix = if context_start > 0 { "..." } else { "" };
                                        let suffix = if context_end < line_len { "..." } else { "" };
                                        let display_text = format!("{}{}{}", prefix, context_text.trim(), suffix);

                                        let prefix_len = prefix.len();
                                        let adjusted_highlights: Vec<usize> = highlights
                                            .iter()
                                            .map(|&h| h + prefix_len)
                                            .collect();

                                        (display_text, adjusted_highlights)
                                    } else {
                                        let line_start = Point::new(line_row, 0);
                                        let line_end = Point::new(line_row, line_len);
                                        let full_line = buffer_ref
                                            .text_for_range(line_start..line_end)
                                            .collect::<String>();
                                        let trimmed_start = full_line.len() - full_line.trim_start().len();
                                        let line_text = full_line.trim().to_string();

                                        let adjusted_column = (start_point.column as usize).saturating_sub(trimmed_start);
                                        let highlight_ranges: Vec<usize> = (adjusted_column..adjusted_column + match_len).collect();

                                        (line_text, highlight_ranges)
                                    };

                                    picker.delegate.matches.push(SearchMatch {
                                        path: path.clone(),
                                        line_number: line_row + 1,
                                        line_text,
                                        highlight_ranges,
                                    });
                                }
                            }
                            SearchResult::LimitReached => {
                                limit_reached = true;
                                break;
                            }
                        }
                    }

                    cx.notify();
                    picker.delegate.matches.len() >= MAX_MATCHES || limit_reached
                }).ok().unwrap_or(true);

                if should_stop {
                    break;
                }
            }
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.open_selected_match(window, cx);
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let search_match = self.matches.get(ix)?;
        let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size(cx);

        let file_name = search_match
            .path
            .as_ref()
            .and_then(|p| p.path.file_name())
            .map(|name| name.to_string())
            .unwrap_or_else(|| "untitled".to_string());

        let item = ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Dense)
                .selectable(false)
                .child(
                    h_flex()
                        .w_full()
                        .px_1()
                        .py_0p5()
                        .gap_2()
                        .text_size(buffer_font_size)
                        .when(selected, |this| {
                            this.bg(cx.theme().colors().ghost_element_selected)
                                .rounded_sm()
                        })
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .overflow_hidden()
                                .child(
                                    HighlightedLabel::new(
                                        search_match.line_text.clone(),
                                        search_match.highlight_ranges.clone()
                                    )
                                    .single_line()
                                )
                        )
                        .child(
                            h_flex()
                                .flex_shrink_0()
                                .child(
                                    Label::new(format!("{}:{}", file_name, search_match.line_number))
                                        .color(Color::Muted)
                                )
                        )
                );

        Some(item)
    }
}
