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
    ActiveTheme, App, ButtonCommon, ButtonSize, Clickable, Color, Context, Div, Divider,
    FluentBuilder, IconButton, IconName, InteractiveElement, Label, LabelCommon, LabelSize,
    ListItem, ListItemSpacing, ParentElement, PopoverMenu, StatefulInteractiveElement, Styled,
    StyledTypography, Toggleable, Tooltip, Window, div, h_flex, relative, v_flex,
};
use ui_input::ErasedEditor;
use util::ResultExt;

use crate::{
    SearchOption, SearchOptions, ToggleCaseSensitive, ToggleRegex, ToggleWholeWord,
    quick_search::{
        CLICK_THRESHOLD_MS, DOUBLE_CLICK_THRESHOLD_MS, InputPanel, ReplaceAll, ReplaceNext,
        SEARCH_DEBOUNCE_MS, SEARCH_RESULTS_BATCH_SIZE, ToggleHistory,
    },
};

use super::QuickSearchDelegate;

impl PickerDelegate for QuickSearchDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search all files...".into()
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let search_options = self.search_options;
        if !self.editor_configured.get() {
            editor.set_multiline(Some(4), window, cx);
            self.editor_configured.set(true);
        }
        let focus_handle = editor.focus_handle(cx);

        if let Some(query) = self.pending_initial_query.borrow_mut().take() {
            editor.set_text(&query, window, cx);
            if let Some(editor) = editor.as_any().downcast_ref::<Entity<Editor>>() {
                editor.update(cx, |editor, cx| {
                    editor.select_all(&editor::actions::SelectAll, window, cx);
                });
            }
        }

        v_flex()
            .child(
                h_flex()
                    .flex_none()
                    .min_h_9()
                    .px_2p5()
                    .gap_1()
                    .items_center()
                    .child(
                        h_flex()
                            .flex_1()
                            .overflow_hidden()
                            .py_1p5()
                            .border_1()
                            .rounded_md()
                            .pl_0p5()
                            .pr_1()
                            .border_color(
                                if self.panels_with_errors.contains_key(&InputPanel::Query) {
                                    Color::Error.color(cx)
                                } else {
                                    gpui::transparent_black()
                                },
                            )
                            .gap_1()
                            .child(
                                PopoverMenu::new("history-menu-popover")
                                    .with_handle(self.history_popover_menu_handle.clone())
                                    .trigger(
                                        IconButton::new(
                                            "search-history",
                                            IconName::MagnifyingGlass,
                                        )
                                        .tooltip({
                                            let focus_handle = editor.focus_handle(cx);
                                            move |_window, cx| {
                                                Tooltip::for_action_in(
                                                    "Search History",
                                                    &ToggleHistory,
                                                    &focus_handle,
                                                    cx,
                                                )
                                            }
                                        }),
                                    )
                                    .menu({
                                        let editor = editor.clone();
                                        let project = self.project.clone();
                                        move |window, cx| {
                                            Self::render_history_menu(&project, &editor, window, cx)
                                        }
                                    }),
                            )
                            .child(div().flex_1().min_w_0().child(editor.render(window, cx))),
                    )
                    .child({
                        let focus_handle = focus_handle.clone();
                        h_flex()
                            .flex_none()
                            .gap_0p5()
                            .child({
                                let editor_for_click = editor.clone();
                                IconButton::new("insert-newline", IconName::Return)
                                    .size(ButtonSize::Compact)
                                    .tooltip(Tooltip::text("Insert New Line"))
                                    .on_click(move |_, window, cx| {
                                        let text = editor_for_click.text(cx);
                                        editor_for_click.set_text(&(text + "\n"), window, cx);
                                    })
                            })
                            .child({
                                let focus_handle = focus_handle.clone();
                                IconButton::new(
                                    "case-sensitive",
                                    SearchOption::CaseSensitive.icon(),
                                )
                                .size(ButtonSize::Compact)
                                .toggle_state(
                                    search_options.contains(SearchOptions::CASE_SENSITIVE),
                                )
                                .tooltip(move |_window, cx| {
                                    Tooltip::for_action_in(
                                        SearchOption::CaseSensitive.label(),
                                        &ToggleCaseSensitive,
                                        &focus_handle,
                                        cx,
                                    )
                                })
                                .on_click(cx.listener(
                                    |picker, _, window, cx| {
                                        picker
                                            .delegate
                                            .search_options
                                            .toggle(SearchOptions::CASE_SENSITIVE);
                                        picker.refresh(window, cx);
                                    },
                                ))
                            })
                            .child({
                                let focus_handle = focus_handle.clone();
                                IconButton::new("whole-word", SearchOption::WholeWord.icon())
                                    .size(ButtonSize::Compact)
                                    .toggle_state(
                                        search_options.contains(SearchOptions::WHOLE_WORD),
                                    )
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            SearchOption::WholeWord.label(),
                                            &ToggleWholeWord,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker
                                            .delegate
                                            .search_options
                                            .toggle(SearchOptions::WHOLE_WORD);
                                        picker.refresh(window, cx);
                                    }))
                            })
                            .child(
                                IconButton::new("regex", SearchOption::Regex.icon())
                                    .size(ButtonSize::Compact)
                                    .toggle_state(search_options.contains(SearchOptions::REGEX))
                                    .tooltip(move |_window, cx| {
                                        Tooltip::for_action_in(
                                            SearchOption::Regex.label(),
                                            &ToggleRegex,
                                            &focus_handle,
                                            cx,
                                        )
                                    })
                                    .on_click(cx.listener(|picker, _, window, cx| {
                                        picker.delegate.search_options.toggle(SearchOptions::REGEX);
                                        picker.refresh(window, cx);
                                    })),
                            )
                    }),
            )
            .when(self.replace_enabled, |this| {
                this.child(Divider::horizontal()).child(
                    h_flex()
                        .flex_none()
                        .h_9()
                        .px_2p5()
                        .gap_1()
                        .child(
                            div()
                                .flex_1()
                                .overflow_hidden()
                                .child(self.replacement_editor.render(window, cx)),
                        )
                        .child({
                            h_flex()
                                .flex_none()
                                .gap_0p5()
                                .child({
                                    let focus_handle = focus_handle.clone();
                                    IconButton::new("replace-next", IconName::ReplaceNext)
                                        .shape(ui::IconButtonShape::Square)
                                        .tooltip(move |_window, cx| {
                                            Tooltip::for_action_in(
                                                "Replace Next Match",
                                                &ReplaceNext,
                                                &focus_handle,
                                                cx,
                                            )
                                        })
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(ReplaceNext.boxed_clone(), cx);
                                        })
                                })
                                .child({
                                    let focus_handle = focus_handle.clone();
                                    IconButton::new("replace-all", IconName::ReplaceAll)
                                        .shape(ui::IconButtonShape::Square)
                                        .tooltip(move |_window, cx| {
                                            Tooltip::for_action_in(
                                                "Replace All",
                                                &ReplaceAll,
                                                &focus_handle,
                                                cx,
                                            )
                                        })
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(ReplaceAll.boxed_clone(), cx);
                                        })
                                })
                        }),
                )
            })
            .when(self.filters_enabled, |this| {
                this.child(Divider::horizontal()).child(
                    h_flex()
                        .flex_none()
                        .h_9()
                        .px_2p5()
                        .gap_2()
                        .child(
                            h_flex()
                                .flex_1()
                                .gap_1()
                                .child(
                                    Label::new("Include:")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .overflow_hidden()
                                        .border_1()
                                        .rounded_md()
                                        .px_1()
                                        .border_color(
                                            if self
                                                .panels_with_errors
                                                .contains_key(&InputPanel::Include)
                                            {
                                                Color::Error.color(cx)
                                            } else {
                                                gpui::transparent_black()
                                            },
                                        )
                                        .child(self.included_files_editor.render(window, cx)),
                                ),
                        )
                        .child(
                            h_flex()
                                .flex_1()
                                .gap_1()
                                .child(
                                    Label::new("Exclude:")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .overflow_hidden()
                                        .border_1()
                                        .rounded_md()
                                        .px_1()
                                        .border_color(
                                            if self
                                                .panels_with_errors
                                                .contains_key(&InputPanel::Exclude)
                                            {
                                                Color::Error.color(cx)
                                            } else {
                                                gpui::transparent_black()
                                            },
                                        )
                                        .child(self.excluded_files_editor.render(window, cx)),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_0p5()
                                .child(
                                    IconButton::new("opened-only", IconName::FolderSearch)
                                        .size(ButtonSize::Compact)
                                        .toggle_state(self.included_opened_only)
                                        .tooltip(Tooltip::text("Only Search Open Files"))
                                        .on_click(cx.listener(|picker, _, window, cx| {
                                            picker.delegate.included_opened_only =
                                                !picker.delegate.included_opened_only;
                                            picker.refresh(window, cx);
                                        })),
                                )
                                .child(
                                    IconButton::new("include-ignored", IconName::Sliders)
                                        .size(ButtonSize::Compact)
                                        .toggle_state(
                                            self.search_options
                                                .contains(SearchOptions::INCLUDE_IGNORED),
                                        )
                                        .tooltip(Tooltip::text(
                                            "Also search files ignored by configuration",
                                        ))
                                        .on_click(cx.listener(|picker, _, window, cx| {
                                            picker
                                                .delegate
                                                .search_options
                                                .toggle(SearchOptions::INCLUDE_IGNORED);
                                            picker.refresh(window, cx);
                                        })),
                                ),
                        ),
                )
            })
            .when(
                self.editor_position() == PickerEditorPosition::Start,
                |this| this.child(Divider::horizontal()),
            )
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
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        self.last_selection_change_time = Some(std::time::Instant::now());
        self.update_preview(window, cx);
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

        let open_buffers = if self.included_opened_only {
            Some(self.open_buffers(cx))
        } else {
            None
        };

        let Some(search_query) = self.build_search_query(&query, open_buffers, cx) else {
            self.matches.clear();
            self.selected_index = 0;
            self.file_count = 0;
            self.search_in_progress = false;
            self.update_preview(window, cx);
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
                                QuickSearchDelegate::process_search_result(&buffer, &ranges, cx);
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

                        if delegate.matches.len() == delegate.selected_index + 1
                            || delegate.selected_index == 0
                        {
                            delegate.update_preview(window, cx);
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

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let in_replace =
            self.replace_enabled && self.replacement_editor.focus_handle(cx).is_focused(window);

        if in_replace {
            if secondary {
                window.dispatch_action(ReplaceAll.boxed_clone(), cx);
            } else {
                window.dispatch_action(ReplaceNext.boxed_clone(), cx);
            }
            return;
        }

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
