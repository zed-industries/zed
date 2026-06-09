use crate::project_search_picker::delegate::TextPickerDelegate;
use crate::{SearchOptions, project_search_picker::SearchMatch};
use std::{ops::Range, sync::Arc, time::Duration};

use editor::Editor;
use file_icons::FileIcons;
use futures::StreamExt;
use gpui::{Action, AsyncApp, DismissEvent, Entity, HighlightStyle, StyledText, Task, TextStyle};
use language::{Buffer, LanguageAwareStyling};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::{ProjectPath, SearchResults, search::SearchQuery, search::SearchResult};
use settings::Settings;
use text::Anchor;
use theme_settings::ThemeSettings;
use ui::{
    ActiveTheme, App, Color, Context, Div, Divider, FluentBuilder, HighlightedLabel, Icon,
    InteractiveElement, LabelCommon, LabelSize, ListItem, ListItemSpacing, ParentElement,
    SharedString, StatefulInteractiveElement, Styled, StyledTypography, Toggleable, Tooltip,
    Window, div, h_flex, relative, v_flex,
};
use ui_input::ErasedEditor;
use util::ResultExt;
use util::paths::PathMatcher;
use workspace::item::ItemSettings;

use super::InputPanel;

const SEARCH_DEBOUNCE_MS: u64 = 100;
const CLICK_THRESHOLD_MS: u128 = 50;
const DOUBLE_CLICK_THRESHOLD_MS: u128 = 300;
const SEARCH_RESULTS_BATCH_SIZE: usize = 256;

impl PickerDelegate for TextPickerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search all files...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn select_on_hover(&self) -> bool {
        false
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        self.last_selection_change_time = Some(std::time::Instant::now());
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.cancel_flag
            .store(true, std::sync::atomic::Ordering::SeqCst);
        self.cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let cancel_flag = self.cancel_flag.clone();

        let open_buffers = None;

        let Some(search_query) = self.build_search_query(&query, open_buffers, cx) else {
            self.matches.clear();
            self.selected_index = 0;
            self.file_count = 0;
            self.search_in_progress = false;
            cx.notify();
            return Task::ready(());
        };

        let search_results = self
            .project
            .update(cx, |project, cx| project.search(search_query, cx));

        self.search_in_progress = true;
        cx.notify();

        cx.spawn_in(window, async move |picker, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(SEARCH_DEBOUNCE_MS))
                .await;

            if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                return;
            }

            let mut first_batch = true;
            let SearchResults { rx, _task_handle } = search_results;
            let mut results_stream = std::pin::pin!(rx.ready_chunks(SEARCH_RESULTS_BATCH_SIZE));

            while let Some(results) = results_stream.next().await {
                if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }

                let mut batch_matches = Vec::new();
                let mut limit_reached = false;

                for result in results {
                    match result {
                        SearchResult::Buffer { buffer, ranges } => {
                            let matches =
                                TextPickerDelegate::process_search_result(&buffer, &ranges, cx);
                            batch_matches.extend(matches);
                        }
                        SearchResult::LimitReached => {
                            limit_reached = true;
                        }
                    }
                }

                picker
                    .update_in(cx, |picker, window, cx| {
                        let delegate = &mut picker.delegate;

                        if first_batch {
                            delegate.matches.clear();
                            delegate.file_count = 0;
                            delegate.unique_files.clear();
                            delegate.selected_index = 0;
                            first_batch = false;
                        }

                        for m in &batch_matches {
                            if delegate.unique_files.insert(m.path.clone()) {
                                delegate.file_count += 1;
                            }
                        }
                        delegate.matches.extend(batch_matches);

                        if delegate.selected_index >= delegate.matches.len()
                            && !delegate.matches.is_empty()
                        {
                            delegate.selected_index = 0;
                        }

                        cx.notify();
                    })
                    .log_err();

                if limit_reached {
                    break;
                }

                smol::future::yield_now().await;
            }

            picker
                .update_in(cx, |picker, _window, cx| {
                    picker.delegate.search_in_progress = false;
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        // Clicks (set_selected_index called immediately before confirm) require double-click.
        // Enter key proceeds immediately.
        let now = std::time::Instant::now();
        let is_click = self
            .last_selection_change_time
            .map(|t| now.duration_since(t).as_millis() < CLICK_THRESHOLD_MS)
            .unwrap_or(false);

        if is_click {
            let is_double_click = self
                .last_click
                .map(|(ix, t)| {
                    ix == self.selected_index
                        && now.duration_since(t).as_millis() < DOUBLE_CLICK_THRESHOLD_MS
                })
                .unwrap_or(false);
            self.last_click = Some((self.selected_index, now));

            if !is_double_click {
                cx.focus_self(window);
                return;
            }
        }

        let Some(selected_match) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let path = selected_match.path.clone();
        let line_number = selected_match.line_number;

        let open_task = workspace.update(cx, |workspace, cx| {
            workspace.open_path_preview(path, None, true, false, true, window, cx)
        });

        let row = line_number.saturating_sub(1);
        cx.spawn_in(window, async move |_, cx| {
            let item = open_task.await.log_err()?;
            if let Some(active_editor) = item.downcast::<editor::Editor>() {
                active_editor
                    .downgrade()
                    .update_in(cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(text::Point::new(row, 0), window, cx);
                    })
                    .log_err();
            }
            Some(())
        })
        .detach();

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn preview_layout_changed(&mut self, layout_is_horizontal: bool) {
        self.preview_layout_is_horizontal = layout_is_horizontal
    }

    fn try_get_match(&self, _cx: &App) -> Option<picker::PreviewUpdate> {
        let m = self.matches.get(self.selected_index)?;
        Some(picker::PreviewUpdate::from_buffer(
            m.buffer.clone(),
            picker::PreviewHighlight {
                anchor_range: m.anchor_range.clone(),
                range: m.range.clone(),
            },
        ))
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let search_match = self.matches.get(ix)?;
        let path = &search_match.path.path;
        let path_style = self.project.read(cx).path_style(cx);
        let file_name = path
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_default();
        let directory = path
            .parent()
            .map(|parent| parent.display(path_style))
            .map(|parent| SharedString::new(parent))
            .unwrap_or_default();
        let full_path = SharedString::new(path.display(path_style));

        let file_icon = ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(path.as_std_path(), cx))
            .flatten()
            .map(|icon| Icon::from_path(icon).color(Color::Muted));

        let file_location = h_flex()
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .id(("text-picker-path", ix))
            .tooltip(Tooltip::text(full_path))
            .child(div().flex_none().child(format!("{file_name} ")))
            .when(!directory.is_empty(), |this| {
                this.child(
                    div()
                        .min_w_0()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis_start()
                        .text_color(cx.theme().colors().text_muted)
                        .child(directory),
                )
            });

        let rendered_line = if self.preview_layout_is_horizontal {
            h_flex().gap_2().py_px().child(file_location)
        } else {
            h_flex()
                .w_full()
                .gap_4()
                .justify_between()
                .font_buffer(cx)
                .text_buffer(cx)
                .when(!self.preview_layout_is_horizontal, |d| {
                    d.child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .child(render_matched_line(search_match, cx)),
                    )
                })
                .child(
                    h_flex()
                        .w(relative(0.35))
                        .flex_none()
                        .gap_2()
                        .child(file_location),
                )
        };

        let line_number = div()
            .flex_none()
            .pr_2()
            .text_color(cx.theme().colors().text_muted)
            .child(search_match.line_number.to_string());
        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .start_slot::<Icon>(file_icon)
                .end_slot::<Div>(line_number)
                .inset(true)
                .toggle_state(selected)
                .child(rendered_line),
        )
    }
}

/// Renders the matched source line with syntax highlighting, overlaying the
/// search match with a highlighted background and bold weight.
fn render_matched_line(search_match: &SearchMatch, cx: &App) -> StyledText {
    let settings = ThemeSettings::get_global(cx);
    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: settings.buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.),
        ..Default::default()
    };
    let original_line = &search_match.line_text;
    let line_text = original_line.trim_start();
    let trim_offset = original_line.len() - line_text.len();

    let search_match_style = HighlightStyle {
        background_color: Some(cx.theme().colors().search_match_background),
        font_weight: Some(gpui::FontWeight::BOLD),
        ..Default::default()
    };

    let line_start_abs = search_match.range.start - search_match.relative_range.start;
    let visible_start_abs = line_start_abs + trim_offset;
    let visible_end_abs = line_start_abs + original_line.len();

    // Syntax highlights for the visible (trimmed) portion of the line, with
    // ranges relative to the start of the rendered text.
    let snapshot = search_match.buffer.read(cx).snapshot();
    let syntax_theme = cx.theme().syntax();
    let mut syntax_highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();
    let mut current_offset = 0;
    for chunk in snapshot.chunks(
        visible_start_abs..visible_end_abs,
        LanguageAwareStyling {
            tree_sitter: true,
            diagnostics: false,
        },
    ) {
        let chunk_len = chunk.text.len();
        if let Some(style) = chunk
            .syntax_highlight_id
            .and_then(|id| syntax_theme.get(id).copied())
        {
            syntax_highlights.push((current_offset..current_offset + chunk_len, style));
        }
        current_offset += chunk_len;
    }

    // The search match range, clamped to the visible area and made relative to
    // the start of the rendered text.
    let match_start = search_match
        .range
        .start
        .clamp(visible_start_abs, visible_end_abs);
    let match_end = search_match
        .range
        .end
        .clamp(visible_start_abs, visible_end_abs);
    let match_highlight = (
        match_start - visible_start_abs..match_end - visible_start_abs,
        search_match_style,
    );

    let highlights = gpui::combine_highlights(syntax_highlights, [match_highlight]);

    StyledText::new(line_text.to_string()).with_default_highlights(&text_style, highlights)
}

impl TextPickerDelegate {
    pub(crate) fn build_search_query(
        &mut self,
        query: &str,
        open_buffers: Option<Vec<Entity<Buffer>>>,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<SearchQuery> {
        if query.is_empty() {
            return None;
        }

        let files_to_include = PathMatcher::default();
        let files_to_exclude = PathMatcher::default();

        // If the project contains multiple visible worktrees, we match the
        // include/exclude patterns against full paths to allow them to be
        // disambiguated. For single worktree projects we use worktree relative
        // paths for convenience.
        let match_full_paths = self.project.read(cx).visible_worktrees(cx).count() > 1;

        let result = if self.search_options.contains(SearchOptions::REGEX) {
            SearchQuery::regex(
                query,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                self.search_options
                    .contains(SearchOptions::ONE_MATCH_PER_LINE),
                files_to_include,
                files_to_exclude,
                match_full_paths,
                open_buffers,
            )
        } else {
            SearchQuery::text(
                query,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                files_to_include,
                files_to_exclude,
                match_full_paths,
                open_buffers,
            )
        };

        result.log_err()
    }

    pub(crate) fn process_search_result(
        buffer: &Entity<Buffer>,
        ranges: &[Range<Anchor>],
        cx: &AsyncApp,
    ) -> Vec<SearchMatch> {
        if ranges.is_empty() {
            return Vec::new();
        }

        buffer.read_with(cx, |buf, cx| {
            let file = buf.file();
            let path = file.map(|f| ProjectPath {
                worktree_id: f.worktree_id(cx),
                path: f.path().clone(),
            });
            let text = buf.text();

            let mut matches = Vec::new();
            for anchor_range in ranges {
                let start_offset: usize = buf.summary_for_anchor(&anchor_range.start);
                let end_offset: usize = buf.summary_for_anchor(&anchor_range.end);
                let match_row = buf.offset_to_point(start_offset).row;
                let line_number = match_row + 1;
                let line_start = text[..start_offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
                let line_end = text[start_offset..]
                    .find('\n')
                    .map(|i| start_offset + i)
                    .unwrap_or(text.len());
                let line_text = text[line_start..line_end].to_string();

                let relative_start = start_offset - line_start;
                let relative_end = end_offset - line_start;

                if let Some(path) = &path {
                    matches.push(SearchMatch {
                        path: path.clone(),
                        buffer: buffer.clone(),
                        anchor_range: anchor_range.clone(),
                        range: start_offset..end_offset,
                        relative_range: relative_start..relative_end,
                        line_text,
                        line_number,
                    });
                }
            }
            matches
        })
    }
}
