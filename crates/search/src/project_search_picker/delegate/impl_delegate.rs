use crate::project_search_picker::delegate::TextPickerDelegate;
use std::{ops::Range, sync::Arc, time::Duration};

use editor::Editor;
use futures::StreamExt;
use gpui::{Action, DismissEvent, Entity, HighlightStyle, StyledText, Task};
use language::LanguageAwareStyling;
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::{SearchResults, search::SearchResult};
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{
    ActiveTheme, App, Color, Context, Div, Divider, FluentBuilder, InteractiveElement, ListItem,
    ListItemSpacing, ParentElement, StatefulInteractiveElement, Styled, StyledTypography,
    Toggleable, Tooltip, Window, div, h_flex, relative, v_flex,
};
use ui_input::ErasedEditor;
use util::ResultExt;

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
                                PickerDelegate::process_search_result(&buffer, &ranges, cx);
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

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let search_match = self.matches.get(ix)?;
        let path = &search_match.path.path;
        let file_name = path
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_default();
        let directory = path
            .parent()
            .map(|parent| parent.as_std_path().to_string_lossy().to_string())
            .unwrap_or_default();
        let full_path = path.as_std_path().to_string_lossy().to_string();

        let original_line = &search_match.line_text;
        let line_text = original_line.trim_start();
        let trim_offset = original_line.len() - line_text.len();
        let line_text_string = line_text.to_string();

        // Build search match range (merged with syntax highlighting, adding background + bold)
        let search_match_style = HighlightStyle {
            background_color: Some(cx.theme().colors().search_match_background),
            font_weight: Some(gpui::FontWeight::BOLD),
            ..Default::default()
        };

        let mut highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();

        {
            let line_start_abs = search_match.range.start - search_match.relative_range.start;
            let visible_start_abs = line_start_abs + trim_offset;
            let visible_end_abs = line_start_abs + original_line.len();
            let match_start_abs = search_match.range.start;
            let match_end_abs = search_match.range.end;

            // Determine the "effective" match range within the visible area
            let effective_match_start = match_start_abs.max(visible_start_abs);
            let effective_match_end = match_end_abs.min(visible_end_abs);

            let ranges = [
                (visible_start_abs..effective_match_start, false),
                (effective_match_start..effective_match_end, true),
                (effective_match_end..visible_end_abs, false),
            ];

            let snapshot = search_match.buffer.read(cx).snapshot();
            let syntax_theme = cx.theme().syntax();
            let mut current_offset = 0;

            for (range, is_match) in ranges {
                if range.start >= range.end {
                    continue;
                }

                for chunk in snapshot.chunks(
                    range,
                    LanguageAwareStyling {
                        tree_sitter: true,
                        diagnostics: false,
                    },
                ) {
                    let chunk_len = chunk.text.len();
                    let syntax_style = chunk
                        .syntax_highlight_id
                        .and_then(|id| syntax_theme.get(id).copied());

                    let style = if is_match {
                        let mut style = syntax_style.unwrap_or_default();
                        if let Some(bg) = search_match_style.background_color {
                            style.background_color = Some(bg);
                        }
                        if let Some(weight) = search_match_style.font_weight {
                            style.font_weight = Some(weight);
                        }
                        style
                    } else {
                        syntax_style.unwrap_or_default()
                    };

                    highlights.push((current_offset..current_offset + chunk_len, style));
                    current_offset += chunk_len;
                }
            }
        }

        let mut text_style = window.text_style();
        let settings = ThemeSettings::get_global(cx);
        text_style.font_family = settings.buffer_font.family.clone();
        text_style.font_size = settings.buffer_font_size(cx).into();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .gap_4()
                        .justify_between()
                        .font_buffer(cx)
                        .text_buffer(cx)
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .overflow_hidden()
                                .text_ellipsis()
                                .whitespace_nowrap()
                                .child(
                                    StyledText::new(line_text_string)
                                        .with_default_highlights(&text_style, highlights),
                                ),
                        )
                        .child(
                            h_flex()
                                .w(relative(0.35))
                                .flex_none()
                                .gap_2()
                                .child(
                                    h_flex()
                                        .flex_1()
                                        .min_w_0()
                                        .overflow_hidden()
                                        .id(("quick-search-path", ix))
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
                                        }),
                                )
                                .child(
                                    div()
                                        .flex_none()
                                        .pr_2()
                                        .text_color(cx.theme().colors().text_muted)
                                        .child(search_match.line_number.to_string()),
                                ),
                        ),
                ),
        )
    }
}
