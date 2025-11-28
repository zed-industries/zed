use crate::{SearchOption, SearchOptions, project_search::ProjectSearch};
use editor::{Editor, EditorSettings};
use futures::StreamExt;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, ParentElement, PromptLevel, Render, SharedString, Styled, Task, WeakEntity,
    Window, prelude::*, rems,
};
use language::{Buffer, Point, ToPoint};
use picker::{Picker, PickerDelegate};
use project::{
    Project, ProjectPath,
    search::{SearchQuery, SearchResult},
};
use settings::Settings;
use theme::ThemeSettings;
use std::{
    mem,
    ops::Range,
    pin::pin,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};
use text::Anchor;
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

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .modal(false)
                .max_height(Some(rems(24.).into()))
        });

        let picker_subscription = cx.subscribe_in(&picker, window, Self::on_picker_event);

        Self {
            picker,
            _subscriptions: vec![picker_subscription],
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
    search_id: Arc<AtomicUsize>,
    cancel_flag: Arc<AtomicBool>,
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
    pub buffer: Entity<Buffer>,
    pub path: Option<ProjectPath>,
    pub range: Range<Anchor>,
    pub line_number: u32,
    pub line_text: String,
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
            search_id: Arc::new(AtomicUsize::new(0)),
            cancel_flag: Arc::new(AtomicBool::new(false)),
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

    fn find_match_positions(&self, text: &str) -> Vec<usize> {
        if self.query.is_empty() {
            return Vec::new();
        }

        let query_lower = self.query.to_lowercase();
        let text_lower = text.to_lowercase();
        let mut positions = Vec::new();

        let mut start = 0;
        while let Some(pos) = text_lower[start..].find(&query_lower) {
            let absolute_pos = start + pos;
            for i in 0..self.query.len() {
                positions.push(absolute_pos + i);
            }
            start = absolute_pos + 1;
        }

        positions
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

        let position = search_match.range.start.to_point(search_match.buffer.read(cx));

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
        let match_ranges = self.project_search.read(cx).match_ranges.clone();
        if match_ranges.is_empty() || self.selected_index >= match_ranges.len() {
            return;
        }

        let Some(query) = self.active_query.clone() else {
            return;
        };
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
        let Some(query) = self.active_query.as_ref() else {
            return;
        };
        let query = query.clone().with_replacement(self.replacement(cx));

        let match_ranges = self
            .project_search
            .update(cx, |model, _| mem::take(&mut model.match_ranges));
        if match_ranges.is_empty() {
            return;
        }

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
            cx.notify();
            return Task::ready(());
        }

        self.query = query.clone();
        self.cancel_flag.store(true, Ordering::SeqCst);
        let cancel_flag = Arc::new(AtomicBool::new(false));
        self.cancel_flag = cancel_flag.clone();

        let search_id = self.search_id.fetch_add(1, Ordering::SeqCst) + 1;

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

        self.project_search.update(cx, |project_search, cx| {
            project_search.search(search_query.clone(), cx);
        });

        let project = self.project.clone();
        let search = project.update(cx, |project, cx| project.search(search_query, cx));
        let search_id_ref = self.search_id.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let mut matches = pin!(search);
            let mut collected_matches: Vec<SearchMatch> = Vec::new();

            while let Some(result) = matches.next().await {
                if cancel_flag.load(Ordering::SeqCst) {
                    return;
                }

                match result {
                    SearchResult::Buffer { buffer, ranges } => {
                        let new_matches: Option<Vec<SearchMatch>> = picker.read_with(cx, |_, cx| {
                            let buffer_ref = buffer.read(cx);
                            let path = buffer_ref.file().map(|file| ProjectPath {
                                worktree_id: file.worktree_id(cx),
                                path: file.path().clone(),
                            });

                            ranges
                                .into_iter()
                                .take(MAX_MATCHES.saturating_sub(collected_matches.len()))
                                .map(|range| {
                                    let start_point = range.start.to_point(buffer_ref);
                                    let line_number = start_point.row + 1;

                                    let line_start = Point::new(start_point.row, 0);
                                    let line_end = Point::new(start_point.row, buffer_ref.line_len(start_point.row));
                                    let line_text = buffer_ref
                                        .text_for_range(line_start..line_end)
                                        .collect::<String>()
                                        .trim()
                                        .to_string();

                                    SearchMatch {
                                        buffer: buffer.clone(),
                                        path: path.clone(),
                                        range,
                                        line_number,
                                        line_text,
                                    }
                                })
                                .collect()
                        }).ok();

                        if let Some(new_matches) = new_matches {
                            collected_matches.extend(new_matches);
                        }

                        if collected_matches.len() >= MAX_MATCHES {
                            break;
                        }
                    }
                    SearchResult::LimitReached => {
                        break;
                    }
                }
            }

            picker
                .update_in(cx, |picker, _, cx| {
                    if search_id_ref.load(Ordering::SeqCst) == search_id {
                        picker.delegate.matches = collected_matches;
                        picker.delegate.selected_index = 0;
                        cx.notify();
                    }
                })
                .ok();
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
        let highlight_positions = self.find_match_positions(&search_match.line_text);

        let file_name = search_match
            .path
            .as_ref()
            .and_then(|p| p.path.file_name())
            .map(|name| name.to_string())
            .unwrap_or_else(|| "untitled".to_string());

        Some(
            ListItem::new(ix)
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
                                    HighlightedLabel::new(search_match.line_text.clone(), highlight_positions)
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
                )
        )
    }
}
