use super::*;

impl Editor {
    pub fn set_completion_provider(&mut self, provider: Option<Rc<dyn CompletionProvider>>) {
        self.completion_provider = provider;
    }

    pub fn set_show_completions_on_input(&mut self, show_completions_on_input: Option<bool>) {
        self.show_completions_on_input_override = show_completions_on_input;
    }

    pub fn text_layout_details(&self, window: &mut Window, cx: &mut App) -> TextLayoutDetails {
        TextLayoutDetails {
            text_system: window.text_system().clone(),
            editor_style: self.style.clone().unwrap_or_else(|| self.create_style(cx)),
            rem_size: window.rem_size(),
            scroll_anchor: self.scroll_manager.shared_scroll_anchor(cx),
            visible_rows: self.visible_line_count(),
            vertical_scroll_margin: self.scroll_manager.vertical_scroll_margin,
        }
    }

    pub fn show_word_completions(
        &mut self,
        _: &ShowWordCompletions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_or_update_completions_menu(
            Some(CompletionsMenuSource::Words {
                ignore_threshold: true,
            }),
            None,
            false,
            window,
            cx,
        );
    }

    pub fn show_completions(
        &mut self,
        _: &ShowCompletions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_or_update_completions_menu(None, None, false, window, cx);
    }

    pub fn confirm_completion(
        &mut self,
        action: &ConfirmCompletion,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.read_only(cx) {
            return None;
        }
        self.do_completion(action.item_ix, CompletionIntent::Complete, window, cx)
    }

    pub fn confirm_completion_insert(
        &mut self,
        _: &ConfirmCompletionInsert,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.read_only(cx) {
            return None;
        }
        self.do_completion(None, CompletionIntent::CompleteWithInsert, window, cx)
    }

    pub fn confirm_completion_replace(
        &mut self,
        _: &ConfirmCompletionReplace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.read_only(cx) {
            return None;
        }
        self.do_completion(None, CompletionIntent::CompleteWithReplace, window, cx)
    }

    pub fn compose_completion(
        &mut self,
        action: &ComposeCompletion,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        self.do_completion(action.item_ix, CompletionIntent::Compose, window, cx)
    }

    pub fn has_visible_completions_menu(&self) -> bool {
        !self.edit_prediction_preview_is_active()
            && self.context_menu.borrow().as_ref().is_some_and(|menu| {
                menu.visible() && matches!(menu, CodeContextMenu::Completions(_))
            })
    }

    pub(super) fn trigger_completion_on_input(
        &mut self,
        text: &str,
        trigger_in_words: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let completions_source = self
            .context_menu
            .borrow()
            .as_ref()
            .and_then(|menu| match menu {
                CodeContextMenu::Completions(completions_menu) => Some(completions_menu.source),
                CodeContextMenu::CodeActions(_) => None,
            });

        match completions_source {
            Some(CompletionsMenuSource::Words { .. }) => {
                self.open_or_update_completions_menu(
                    Some(CompletionsMenuSource::Words {
                        ignore_threshold: false,
                    }),
                    None,
                    trigger_in_words,
                    window,
                    cx,
                );
            }
            _ => self.open_or_update_completions_menu(
                None,
                Some(text.to_owned()).filter(|x| !x.is_empty()),
                trigger_in_words,
                window,
                cx,
            ),
        }
    }

    pub(super) fn is_lsp_relevant(&self, file: Option<&Arc<dyn language::File>>, cx: &App) -> bool {
        let Some(project) = self.project() else {
            return false;
        };
        let Some(buffer_file) = project::File::from_dyn(file) else {
            return false;
        };
        let Some(entry_id) = buffer_file.project_entry_id() else {
            return false;
        };
        let project = project.read(cx);
        let Some(buffer_worktree) = project.worktree_for_id(buffer_file.worktree_id(cx), cx) else {
            return false;
        };
        let Some(worktree_entry) = buffer_worktree.read(cx).entry_for_id(entry_id) else {
            return false;
        };
        !worktree_entry.is_ignored
    }

    pub(super) fn visible_buffers(&self, cx: &mut Context<Editor>) -> Vec<Entity<Buffer>> {
        let display_snapshot = self.display_snapshot(cx);
        let visible_range = self.multi_buffer_visible_range(&display_snapshot, cx);
        let multi_buffer = self.buffer().read(cx);
        display_snapshot
            .buffer_snapshot()
            .range_to_buffer_ranges(visible_range)
            .into_iter()
            .filter(|(_, excerpt_visible_range, _)| !excerpt_visible_range.is_empty())
            .filter_map(|(buffer_snapshot, _, _)| multi_buffer.buffer(buffer_snapshot.remote_id()))
            .collect()
    }

    pub(super) fn visible_buffer_ranges(
        &self,
        cx: &mut Context<Editor>,
    ) -> Vec<(
        BufferSnapshot,
        Range<BufferOffset>,
        ExcerptRange<text::Anchor>,
    )> {
        let display_snapshot = self.display_snapshot(cx);
        let visible_range = self.multi_buffer_visible_range(&display_snapshot, cx);
        display_snapshot
            .buffer_snapshot()
            .range_to_buffer_ranges(visible_range)
            .into_iter()
            .filter(|(_, excerpt_visible_range, _)| !excerpt_visible_range.is_empty())
            .collect()
    }

    pub(super) fn trigger_on_type_formatting(
        &self,
        input: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if input.chars().count() != 1 {
            return None;
        }

        let project = self.project()?;
        let position = self.selections.newest_anchor().head();
        let (buffer, buffer_position) = self
            .buffer
            .read(cx)
            .text_anchor_for_position(position, cx)?;

        let settings = LanguageSettings::for_buffer_at(&buffer.read(cx), buffer_position, cx);
        if !settings.use_on_type_format {
            return None;
        }

        // OnTypeFormatting returns a list of edits, no need to pass them between Zed instances,
        // hence we do LSP request & edit on host side only — add formats to host's history.
        let push_to_lsp_host_history = true;
        // If this is not the host, append its history with new edits.
        let push_to_client_history = project.read(cx).is_via_collab();

        let on_type_formatting = project.update(cx, |project, cx| {
            project.on_type_format(
                buffer.clone(),
                buffer_position,
                input,
                push_to_lsp_host_history,
                cx,
            )
        });
        Some(cx.spawn_in(window, async move |editor, cx| {
            if let Some(transaction) = on_type_formatting.await? {
                if push_to_client_history {
                    buffer.update(cx, |buffer, _| {
                        buffer.push_transaction(transaction, Instant::now());
                        buffer.finalize_last_transaction();
                    });
                }
                editor.update(cx, |editor, cx| {
                    editor.refresh_document_highlights(cx);
                })?;
            }
            Ok(())
        }))
    }

    pub(super) fn open_or_update_completions_menu(
        &mut self,
        requested_source: Option<CompletionsMenuSource>,
        trigger: Option<String>,
        trigger_in_words: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.pending_rename.is_some() {
            return;
        }

        let completions_source = self
            .context_menu
            .borrow()
            .as_ref()
            .and_then(|menu| match menu {
                CodeContextMenu::Completions(completions_menu) => Some(completions_menu.source),
                CodeContextMenu::CodeActions(_) => None,
            });

        let multibuffer_snapshot = self.buffer.read(cx).read(cx);

        // Typically `start` == `end`, but with snippet tabstop choices the default choice is
        // inserted and selected. To handle that case, the start of the selection is used so that
        // the menu starts with all choices.
        let position = self
            .selections
            .newest_anchor()
            .start
            .bias_right(&multibuffer_snapshot);

        if position.diff_base_anchor().is_some() {
            return;
        }
        let multibuffer_position = multibuffer_snapshot.anchor_before(position);
        let Some((buffer_position, _)) =
            multibuffer_snapshot.anchor_to_buffer_anchor(multibuffer_position)
        else {
            return;
        };
        let Some(buffer) = self.buffer.read(cx).buffer(buffer_position.buffer_id) else {
            return;
        };
        let buffer_snapshot = buffer.read(cx).snapshot();

        let menu_is_open = matches!(
            self.context_menu.borrow().as_ref(),
            Some(CodeContextMenu::Completions(_))
        );

        let language = buffer_snapshot
            .language_at(buffer_position)
            .map(|language| language.name());
        let language_settings = multibuffer_snapshot.language_settings_at(multibuffer_position, cx);
        let completion_settings = language_settings.completions.clone();

        let show_completions_on_input = self
            .show_completions_on_input_override
            .unwrap_or(language_settings.show_completions_on_input);
        if !menu_is_open && trigger.is_some() && !show_completions_on_input {
            return;
        }

        let query: Option<Arc<String>> =
            Self::completion_query(&multibuffer_snapshot, multibuffer_position)
                .map(|query| query.into());

        drop(multibuffer_snapshot);

        // Hide the current completions menu when query is empty. Without this, cached
        // completions from before the trigger char may be reused (#32774).
        if query.is_none() && menu_is_open {
            self.hide_context_menu(window, cx);
        }

        let mut ignore_word_threshold = false;
        let provider = match requested_source {
            Some(CompletionsMenuSource::Normal) | None => self.completion_provider.clone(),
            Some(CompletionsMenuSource::Words { ignore_threshold }) => {
                ignore_word_threshold = ignore_threshold;
                None
            }
            Some(CompletionsMenuSource::SnippetChoices)
            | Some(CompletionsMenuSource::SnippetsOnly) => {
                log::error!("bug: SnippetChoices requested_source is not handled");
                None
            }
        };

        let sort_completions = provider
            .as_ref()
            .is_some_and(|provider| provider.sort_completions());

        let filter_completions = provider
            .as_ref()
            .is_none_or(|provider| provider.filter_completions());

        let was_snippets_only = matches!(
            completions_source,
            Some(CompletionsMenuSource::SnippetsOnly)
        );

        if let Some(CodeContextMenu::Completions(menu)) = self.context_menu.borrow_mut().as_mut() {
            if filter_completions {
                menu.filter(
                    query.clone().unwrap_or_default(),
                    buffer_position,
                    &buffer,
                    provider.clone(),
                    window,
                    cx,
                );
            }
            // When `is_incomplete` is false, no need to re-query completions when the current query
            // is a suffix of the initial query.
            let was_complete = !menu.is_incomplete;
            if was_complete && !was_snippets_only {
                // If the new query is a suffix of the old query (typing more characters) and
                // the previous result was complete, the existing completions can be filtered.
                //
                // Note that snippet completions are always complete.
                let query_matches = match (&menu.initial_query, &query) {
                    (Some(initial_query), Some(query)) => query.starts_with(initial_query.as_ref()),
                    (None, _) => true,
                    _ => false,
                };
                if query_matches {
                    let position_matches = if menu.initial_position == position {
                        true
                    } else {
                        let snapshot = self.buffer.read(cx).read(cx);
                        menu.initial_position.to_offset(&snapshot) == position.to_offset(&snapshot)
                    };
                    if position_matches {
                        return;
                    }
                }
            }
        };

        let (word_replace_range, word_to_exclude) = if let (word_range, Some(CharKind::Word)) =
            buffer_snapshot.surrounding_word(buffer_position, None)
        {
            let word_to_exclude = buffer_snapshot
                .text_for_range(word_range.clone())
                .collect::<String>();
            (
                buffer_snapshot.anchor_before(word_range.start)
                    ..buffer_snapshot.anchor_after(buffer_position),
                Some(word_to_exclude),
            )
        } else {
            (buffer_position..buffer_position, None)
        };

        let show_completion_documentation = buffer_snapshot
            .settings_at(buffer_position, cx)
            .show_completion_documentation;

        // The document can be large, so stay in reasonable bounds when searching for words,
        // otherwise completion pop-up might be slow to appear.
        const WORD_LOOKUP_ROWS: u32 = 5_000;
        let buffer_row = text::ToPoint::to_point(&buffer_position, &buffer_snapshot).row;
        let min_word_search = buffer_snapshot.clip_point(
            Point::new(buffer_row.saturating_sub(WORD_LOOKUP_ROWS), 0),
            Bias::Left,
        );
        let max_word_search = buffer_snapshot.clip_point(
            Point::new(buffer_row + WORD_LOOKUP_ROWS, 0).min(buffer_snapshot.max_point()),
            Bias::Right,
        );
        let word_search_range = buffer_snapshot.point_to_offset(min_word_search)
            ..buffer_snapshot.point_to_offset(max_word_search);

        let skip_digits = query
            .as_ref()
            .is_none_or(|query| !query.chars().any(|c| c.is_digit(10)));

        let load_provider_completions = provider.as_ref().is_some_and(|provider| {
            trigger.as_ref().is_none_or(|trigger| {
                provider.is_completion_trigger(
                    &buffer,
                    buffer_position,
                    trigger,
                    trigger_in_words,
                    cx,
                )
            })
        });

        let provider_responses = if let Some(provider) = &provider
            && load_provider_completions
        {
            let trigger_character = trigger
                .as_ref()
                .filter(|trigger| {
                    buffer
                        .read(cx)
                        .completion_triggers()
                        .contains(trigger.as_str())
                })
                .cloned();
            let completion_context = CompletionContext {
                trigger_kind: match &trigger_character {
                    Some(_) => CompletionTriggerKind::TRIGGER_CHARACTER,
                    None => CompletionTriggerKind::INVOKED,
                },
                trigger_character,
            };

            provider.completions(&buffer, buffer_position, completion_context, window, cx)
        } else {
            Task::ready(Ok(Vec::new()))
        };

        let load_word_completions = if !self.word_completions_enabled {
            false
        } else if requested_source
            == Some(CompletionsMenuSource::Words {
                ignore_threshold: true,
            })
        {
            true
        } else {
            load_provider_completions
                && completion_settings.words != WordsCompletionMode::Disabled
                && (ignore_word_threshold || {
                    let words_min_length = completion_settings.words_min_length;
                    // check whether word has at least `words_min_length` characters
                    let query_chars = query.iter().flat_map(|q| q.chars());
                    query_chars.take(words_min_length).count() == words_min_length
                })
        };

        let mut words = if load_word_completions {
            cx.background_spawn({
                let buffer_snapshot = buffer_snapshot.clone();
                async move {
                    buffer_snapshot.words_in_range(WordsQuery {
                        fuzzy_contents: None,
                        range: word_search_range,
                        skip_digits,
                    })
                }
            })
        } else {
            Task::ready(BTreeMap::default())
        };

        let snippet_char_classifier = buffer_snapshot
            .char_classifier_at(buffer_position)
            .scope_context(Some(CharScopeContext::Completion));

        let snippets = if let Some(provider) = &provider
            && provider.show_snippets()
            && let Some(project) = self.project()
        {
            let word_trigger = trigger.as_ref().is_some_and(|trigger| {
                !trigger.is_empty()
                    && trigger
                        .chars()
                        .all(|character| snippet_char_classifier.is_word(character))
            });
            let requires_strong_snippet_match = !menu_is_open && !trigger_in_words && word_trigger;
            let load_snippet_completions = !requires_strong_snippet_match
                || query.as_ref().is_some_and(|query| {
                    let project = project.read(cx);
                    has_strong_snippet_prefix_match(
                        &project,
                        &buffer,
                        buffer_position,
                        &snippet_char_classifier,
                        query,
                        cx,
                    )
                });

            if load_snippet_completions {
                project.update(cx, |project, cx| {
                    snippet_completions(
                        project,
                        &buffer,
                        buffer_position,
                        snippet_char_classifier,
                        cx,
                    )
                })
            } else {
                Task::ready(Ok(CompletionResponse {
                    completions: Vec::new(),
                    display_options: Default::default(),
                    is_incomplete: false,
                }))
            }
        } else {
            Task::ready(Ok(CompletionResponse {
                completions: Vec::new(),
                display_options: Default::default(),
                is_incomplete: false,
            }))
        };

        let snippet_sort_order = EditorSettings::get_global(cx).snippet_sort_order;

        let id = post_inc(&mut self.next_completion_id);
        let task = cx.spawn_in(window, async move |editor, cx| {
            let Ok(()) = editor.update(cx, |this, _| {
                this.completion_tasks.retain(|(task_id, _)| *task_id >= id);
            }) else {
                return;
            };

            // TODO: Ideally completions from different sources would be selectively re-queried, so
            // that having one source with `is_incomplete: true` doesn't cause all to be re-queried.
            let mut completions = Vec::new();
            let mut is_incomplete = false;
            let mut display_options: Option<CompletionDisplayOptions> = None;
            if let Some(provider_responses) = provider_responses.await.log_err()
                && !provider_responses.is_empty()
            {
                for response in provider_responses {
                    completions.extend(response.completions);
                    is_incomplete = is_incomplete || response.is_incomplete;
                    match display_options.as_mut() {
                        None => {
                            display_options = Some(response.display_options);
                        }
                        Some(options) => options.merge(&response.display_options),
                    }
                }
                if completion_settings.words == WordsCompletionMode::Fallback {
                    words = Task::ready(BTreeMap::default());
                }
            }
            let display_options = display_options.unwrap_or_default();

            let mut words = words.await;
            if let Some(word_to_exclude) = &word_to_exclude {
                words.remove(word_to_exclude);
            }
            for lsp_completion in &completions {
                words.remove(&lsp_completion.new_text);
            }
            completions.extend(words.into_iter().map(|(word, word_range)| Completion {
                replace_range: word_replace_range.clone(),
                new_text: word.clone(),
                label: CodeLabel::plain(word, None),
                match_start: None,
                snippet_deduplication_key: None,
                icon_path: None,
                documentation: None,
                source: CompletionSource::BufferWord {
                    word_range,
                    resolved: false,
                },
                insert_text_mode: Some(InsertTextMode::AS_IS),
                confirm: None,
            }));

            completions.extend(
                snippets
                    .await
                    .into_iter()
                    .flat_map(|response| response.completions),
            );

            let menu = if completions.is_empty() {
                None
            } else {
                let Ok((mut menu, matches_task)) = editor.update(cx, |editor, cx| {
                    let languages = editor
                        .workspace
                        .as_ref()
                        .and_then(|(workspace, _)| workspace.upgrade())
                        .map(|workspace| workspace.read(cx).app_state().languages.clone());
                    let menu = CompletionsMenu::new(
                        id,
                        requested_source.unwrap_or(if load_provider_completions {
                            CompletionsMenuSource::Normal
                        } else {
                            CompletionsMenuSource::SnippetsOnly
                        }),
                        sort_completions,
                        show_completion_documentation,
                        position,
                        query.clone(),
                        is_incomplete,
                        buffer.clone(),
                        completions.into(),
                        editor
                            .context_menu()
                            .borrow_mut()
                            .as_ref()
                            .map(|menu| menu.primary_scroll_handle()),
                        display_options,
                        snippet_sort_order,
                        languages,
                        language,
                        cx,
                    );

                    let query = if filter_completions { query } else { None };
                    let matches_task = menu.do_async_filtering(
                        query.unwrap_or_default(),
                        buffer_position,
                        &buffer,
                        cx,
                    );
                    (menu, matches_task)
                }) else {
                    return;
                };

                let matches = matches_task.await;

                let Ok(()) = editor.update_in(cx, |editor, window, cx| {
                    // Newer menu already set, so exit.
                    if let Some(CodeContextMenu::Completions(prev_menu)) =
                        editor.context_menu.borrow().as_ref()
                        && prev_menu.id > id
                    {
                        return;
                    };

                    // Only valid to take prev_menu because either the new menu is immediately set
                    // below, or the menu is hidden.
                    if let Some(CodeContextMenu::Completions(prev_menu)) =
                        editor.context_menu.borrow_mut().take()
                    {
                        let position_matches =
                            if prev_menu.initial_position == menu.initial_position {
                                true
                            } else {
                                let snapshot = editor.buffer.read(cx).read(cx);
                                prev_menu.initial_position.to_offset(&snapshot)
                                    == menu.initial_position.to_offset(&snapshot)
                            };
                        if position_matches {
                            // Preserve markdown cache before `set_filter_results` because it will
                            // try to populate the documentation cache.
                            menu.preserve_markdown_cache(prev_menu);
                        }
                    };

                    menu.set_filter_results(matches, provider, window, cx);
                }) else {
                    return;
                };

                menu.visible().then_some(menu)
            };

            editor
                .update_in(cx, |editor, window, cx| {
                    if editor.focus_handle.is_focused(window)
                        && let Some(menu) = menu
                    {
                        *editor.context_menu.borrow_mut() =
                            Some(CodeContextMenu::Completions(menu));

                        crate::hover_popover::hide_hover(editor, cx);
                        if editor.show_edit_predictions_in_menu() {
                            editor.update_visible_edit_prediction(window, cx);
                        } else {
                            editor
                                .discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);
                        }

                        cx.notify();
                        return;
                    }

                    if editor.completion_tasks.len() <= 1 {
                        // If there are no more completion tasks and the last menu was empty, we should hide it.
                        let was_hidden = editor.hide_context_menu(window, cx).is_none();
                        // If it was already hidden and we don't show edit predictions in the menu,
                        // we should also show the edit prediction when available.
                        if was_hidden && editor.show_edit_predictions_in_menu() {
                            editor.update_visible_edit_prediction(window, cx);
                        }
                    }
                })
                .ok();
        });

        self.completion_tasks.push((id, task));
    }

    pub(super) fn with_completions_menu_matching_id<R>(
        &self,
        id: CompletionId,
        f: impl FnOnce(Option<&mut CompletionsMenu>) -> R,
    ) -> R {
        let mut context_menu = self.context_menu.borrow_mut();
        let Some(CodeContextMenu::Completions(completions_menu)) = &mut *context_menu else {
            return f(None);
        };
        if completions_menu.id != id {
            return f(None);
        }
        f(Some(completions_menu))
    }

    fn completion_query(buffer: &MultiBufferSnapshot, position: impl ToOffset) -> Option<String> {
        let offset = position.to_offset(buffer);
        let (word_range, kind) =
            buffer.surrounding_word(offset, Some(CharScopeContext::Completion));
        if offset > word_range.start && kind == Some(CharKind::Word) {
            Some(
                buffer
                    .text_for_range(word_range.start..offset)
                    .collect::<String>(),
            )
        } else {
            None
        }
    }

    fn do_completion(
        &mut self,
        item_ix: Option<usize>,
        intent: CompletionIntent,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<Task<Result<()>>> {
        use language::ToOffset as _;

        let CodeContextMenu::Completions(completions_menu) = self.hide_context_menu(window, cx)?
        else {
            return None;
        };

        let candidate_id = {
            let entries = completions_menu.entries.borrow();
            let mat = entries.get(item_ix.unwrap_or(completions_menu.selected_item))?;
            if self.show_edit_predictions_in_menu() {
                self.discard_edit_prediction(EditPredictionDiscardReason::Rejected, cx);
            }
            mat.candidate_id
        };

        let completion = completions_menu
            .completions
            .borrow()
            .get(candidate_id)?
            .clone();
        cx.stop_propagation();

        let buffer_handle = completions_menu.buffer.clone();
        let multibuffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let (initial_position, _) =
            multibuffer_snapshot.anchor_to_buffer_anchor(completions_menu.initial_position)?;

        let CompletionEdit {
            new_text,
            snippet,
            replace_range,
        } = process_completion_for_edit(&completion, intent, &buffer_handle, &initial_position, cx);

        let buffer = buffer_handle.read(cx).snapshot();
        let newest_selection = self.selections.newest_anchor();

        let Some(replace_range_multibuffer) =
            multibuffer_snapshot.buffer_anchor_range_to_anchor_range(replace_range.clone())
        else {
            return None;
        };

        let Some((buffer_snapshot, newest_range_buffer)) =
            multibuffer_snapshot.anchor_range_to_buffer_anchor_range(newest_selection.range())
        else {
            return None;
        };

        let old_text = buffer
            .text_for_range(replace_range.clone())
            .collect::<String>();
        let lookbehind = newest_range_buffer
            .start
            .to_offset(buffer_snapshot)
            .saturating_sub(replace_range.start.to_offset(&buffer_snapshot));
        let lookahead = replace_range
            .end
            .to_offset(&buffer_snapshot)
            .saturating_sub(newest_range_buffer.end.to_offset(&buffer));
        let prefix = &old_text[..old_text.len().saturating_sub(lookahead)];
        let suffix = &old_text[lookbehind.min(old_text.len())..];

        let selections = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx));
        let mut ranges = Vec::new();
        let mut all_commit_ranges = Vec::new();
        let mut linked_edits = LinkedEdits::new();

        let text: Arc<str> = new_text.clone().into();
        for selection in &selections {
            let range = if selection.id == newest_selection.id {
                replace_range_multibuffer.clone()
            } else {
                let mut range = selection.range();

                // if prefix is present, don't duplicate it
                if multibuffer_snapshot
                    .contains_str_at(range.start.saturating_sub_usize(lookbehind), prefix)
                {
                    range.start = range.start.saturating_sub_usize(lookbehind);

                    // if suffix is also present, mimic the newest cursor and replace it
                    if selection.id != newest_selection.id
                        && multibuffer_snapshot.contains_str_at(range.end, suffix)
                    {
                        range.end += lookahead;
                    }
                }
                range.to_anchors(&multibuffer_snapshot)
            };

            ranges.push(range.clone());

            let start_anchor = multibuffer_snapshot.anchor_before(range.start);
            let end_anchor = multibuffer_snapshot.anchor_after(range.end);

            if let Some((buffer_snapshot_2, anchor_range)) =
                multibuffer_snapshot.anchor_range_to_buffer_anchor_range(start_anchor..end_anchor)
                && buffer_snapshot_2.remote_id() == buffer_snapshot.remote_id()
            {
                all_commit_ranges.push(anchor_range.clone());
                if !self.linked_edit_ranges.is_empty() {
                    linked_edits.push(&self, anchor_range, text.clone(), cx);
                }
            }
        }

        let common_prefix_len = old_text
            .chars()
            .zip(new_text.chars())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a.len_utf8())
            .sum::<usize>();

        cx.emit(EditorEvent::InputHandled {
            utf16_range_to_replace: None,
            text: new_text[common_prefix_len..].into(),
        });

        let tx_id = self.transact(window, cx, |editor, window, cx| {
            if let Some(mut snippet) = snippet {
                snippet.text = new_text.to_string();
                let offset_ranges = ranges
                    .iter()
                    .map(|range| range.to_offset(&multibuffer_snapshot))
                    .collect::<Vec<_>>();
                editor
                    .insert_snippet(&offset_ranges, snippet, window, cx)
                    .log_err();
            } else {
                editor.buffer.update(cx, |multi_buffer, cx| {
                    let auto_indent = match completion.insert_text_mode {
                        Some(InsertTextMode::AS_IS) => None,
                        _ => editor.autoindent_mode.clone(),
                    };
                    let edits = ranges.into_iter().map(|range| (range, new_text.as_str()));
                    multi_buffer.edit(edits, auto_indent, cx);
                });
            }
            linked_edits.apply(cx);
            editor.refresh_edit_prediction(true, false, window, cx);
        });
        self.invalidate_autoclose_regions(
            &self.selections.disjoint_anchors_arc(),
            &multibuffer_snapshot,
        );

        let show_new_completions_on_confirm = completion
            .confirm
            .as_ref()
            .is_some_and(|confirm| confirm(intent, window, cx));
        if show_new_completions_on_confirm {
            self.open_or_update_completions_menu(None, None, false, window, cx);
        }

        let provider = self.completion_provider.as_ref()?;

        let lsp_store = self.project().map(|project| project.read(cx).lsp_store());
        let command = lsp_store.as_ref().and_then(|lsp_store| {
            let CompletionSource::Lsp {
                lsp_completion,
                server_id,
                ..
            } = &completion.source
            else {
                return None;
            };
            let lsp_command = lsp_completion.command.as_ref()?;
            let available_commands = lsp_store
                .read(cx)
                .lsp_server_capabilities
                .get(server_id)
                .and_then(|server_capabilities| {
                    server_capabilities
                        .execute_command_provider
                        .as_ref()
                        .map(|options| options.commands.as_slice())
                })?;
            if available_commands.contains(&lsp_command.command) {
                Some(CodeAction {
                    server_id: *server_id,
                    range: language::Anchor::min_min_range_for_buffer(buffer.remote_id()),
                    lsp_action: LspAction::Command(lsp_command.clone()),
                    resolved: false,
                })
            } else {
                None
            }
        });

        drop(completion);
        let apply_edits = provider.apply_additional_edits_for_completion(
            buffer_handle.clone(),
            completions_menu.completions.clone(),
            candidate_id,
            true,
            all_commit_ranges,
            cx,
        );

        let editor_settings = EditorSettings::get_global(cx);
        if editor_settings.show_signature_help_after_edits || editor_settings.auto_signature_help {
            // After the code completion is finished, users often want to know what signatures are needed.
            // so we should automatically call signature_help
            self.show_signature_help(&ShowSignatureHelp, window, cx);
        }

        Some(cx.spawn_in(window, async move |editor, cx| {
            let additional_edits_tx = apply_edits.await?;

            if let Some((lsp_store, command)) = lsp_store.zip(command) {
                let title = command.lsp_action.title().to_owned();
                let project_transaction = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.apply_code_action(buffer_handle, command, false, cx)
                    })
                    .await
                    .context("applying post-completion command")?;
                if let Some(workspace) = editor.read_with(cx, |editor, _| editor.workspace())? {
                    Self::open_project_transaction(
                        &editor,
                        workspace.downgrade(),
                        project_transaction,
                        title,
                        cx,
                    )
                    .await?;
                }
            }

            if let Some(tx_id) = tx_id
                && let Some(additional_edits_tx) = additional_edits_tx
            {
                editor
                    .update(cx, |editor, cx| {
                        editor.buffer.update(cx, |buffer, cx| {
                            buffer.merge_transactions(additional_edits_tx.id, tx_id, cx)
                        });
                    })
                    .context("merge transactions")?;
            }

            Ok(())
        }))
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Editor {
    pub fn completion_provider(&self) -> Option<Rc<dyn CompletionProvider>> {
        self.completion_provider.clone()
    }

    pub fn current_completions(&self) -> Option<Vec<project::Completion>> {
        let menu = self.context_menu.borrow();
        if let CodeContextMenu::Completions(menu) = menu.as_ref()? {
            let completions = menu.completions.borrow();
            Some(completions.to_vec())
        } else {
            None
        }
    }

    #[cfg(test)]
    pub(super) fn disable_word_completions(&mut self) {
        self.word_completions_enabled = false;
    }
}

pub trait CompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: text::Anchor,
        trigger: CompletionContext,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>>;

    fn resolve_completions(
        &self,
        _buffer: Entity<Buffer>,
        _completion_indices: Vec<usize>,
        _completions: Rc<RefCell<Box<[Completion]>>>,
        _cx: &mut Context<Editor>,
    ) -> Task<Result<bool>> {
        Task::ready(Ok(false))
    }

    fn apply_additional_edits_for_completion(
        &self,
        _buffer: Entity<Buffer>,
        _completions: Rc<RefCell<Box<[Completion]>>>,
        _completion_index: usize,
        _push_to_history: bool,
        _all_commit_ranges: Vec<Range<language::Anchor>>,
        _cx: &mut Context<Editor>,
    ) -> Task<Result<Option<language::Transaction>>> {
        Task::ready(Ok(None))
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        text: &str,
        trigger_in_words: bool,
        cx: &mut Context<Editor>,
    ) -> bool;

    fn selection_changed(&self, _mat: Option<&StringMatch>, _window: &mut Window, _cx: &mut App) {}

    fn sort_completions(&self) -> bool {
        true
    }

    fn filter_completions(&self) -> bool {
        true
    }

    fn show_snippets(&self) -> bool {
        false
    }
}

fn has_strong_snippet_prefix_match(
    project: &Project,
    buffer: &Entity<Buffer>,
    buffer_anchor: text::Anchor,
    classifier: &CharClassifier,
    query: &str,
    cx: &App,
) -> bool {
    if query.chars().take(2).count() < 2 {
        return false;
    }

    let query = query.to_lowercase();
    let is_word_char = |character| classifier.is_word(character);
    let languages = buffer.read(cx).languages_at(buffer_anchor);
    let snippet_store = project.snippets().read(cx);

    languages.iter().any(|language| {
        snippet_store
            .snippets_for(Some(language.lsp_id()), cx)
            .iter()
            .flat_map(|snippet| snippet.prefix.iter())
            .flat_map(|prefix| snippet_candidate_suffixes(prefix, &is_word_char))
            .any(|candidate| candidate.to_lowercase().starts_with(&query))
    })
}

fn snippet_completions(
    project: &Project,
    buffer: &Entity<Buffer>,
    buffer_anchor: text::Anchor,
    classifier: CharClassifier,
    cx: &mut App,
) -> Task<Result<CompletionResponse>> {
    let languages = buffer.read(cx).languages_at(buffer_anchor);
    let snippet_store = project.snippets().read(cx);

    let scopes: Vec<_> = languages
        .iter()
        .filter_map(|language| {
            let language_name = language.lsp_id();
            let snippets = snippet_store.snippets_for(Some(language_name), cx);

            if snippets.is_empty() {
                None
            } else {
                Some((language.default_scope(), snippets))
            }
        })
        .collect();

    if scopes.is_empty() {
        return Task::ready(Ok(CompletionResponse {
            completions: vec![],
            display_options: CompletionDisplayOptions::default(),
            is_incomplete: false,
        }));
    }

    let snapshot = buffer.read(cx).text_snapshot();
    let executor = cx.background_executor().clone();

    cx.background_spawn(async move {
        let is_word_char = |c| classifier.is_word(c);

        let mut is_incomplete = false;
        let mut completions: Vec<Completion> = Vec::new();

        const MAX_PREFIX_LEN: usize = 128;
        let buffer_offset = text::ToOffset::to_offset(&buffer_anchor, &snapshot);
        let window_start = buffer_offset.saturating_sub(MAX_PREFIX_LEN);
        let window_start = snapshot.clip_offset(window_start, Bias::Left);

        let max_buffer_window: String = snapshot
            .text_for_range(window_start..buffer_offset)
            .collect();

        if max_buffer_window.is_empty() {
            return Ok(CompletionResponse {
                completions: vec![],
                display_options: CompletionDisplayOptions::default(),
                is_incomplete: true,
            });
        }

        for (_scope, snippets) in scopes.into_iter() {
            // Sort snippets by word count to match longer snippet prefixes first.
            let mut sorted_snippet_candidates = snippets
                .iter()
                .enumerate()
                .flat_map(|(snippet_ix, snippet)| {
                    snippet
                        .prefix
                        .iter()
                        .enumerate()
                        .map(move |(prefix_ix, prefix)| {
                            let word_count =
                                snippet_candidate_suffixes(prefix, &is_word_char).count();
                            ((snippet_ix, prefix_ix), prefix, word_count)
                        })
                })
                .collect_vec();
            sorted_snippet_candidates
                .sort_unstable_by_key(|(_, _, word_count)| Reverse(*word_count));

            // Each prefix may be matched multiple times; the completion menu must filter out duplicates.

            let buffer_windows = snippet_candidate_suffixes(&max_buffer_window, &is_word_char)
                .take(
                    sorted_snippet_candidates
                        .first()
                        .map(|(_, _, word_count)| *word_count)
                        .unwrap_or_default(),
                )
                .collect_vec();

            const MAX_RESULTS: usize = 100;
            // Each match also remembers how many characters from the buffer it consumed
            let mut matches: Vec<(StringMatch, usize)> = vec![];

            let mut snippet_list_cutoff_index = 0;
            for (buffer_index, buffer_window) in buffer_windows.iter().enumerate().rev() {
                let word_count = buffer_index + 1;
                // Increase `snippet_list_cutoff_index` until we have all of the
                // snippets with sufficiently many words.
                while sorted_snippet_candidates
                    .get(snippet_list_cutoff_index)
                    .is_some_and(|(_ix, _prefix, snippet_word_count)| {
                        *snippet_word_count >= word_count
                    })
                {
                    snippet_list_cutoff_index += 1;
                }

                // Take only the candidates with at least `word_count` many words
                let snippet_candidates_at_word_len =
                    &sorted_snippet_candidates[..snippet_list_cutoff_index];

                let candidates = snippet_candidates_at_word_len
                    .iter()
                    .map(|(_snippet_ix, prefix, _snippet_word_count)| prefix)
                    .enumerate() // index in `sorted_snippet_candidates`
                    // First char must match
                    .filter(|(_ix, prefix)| {
                        itertools::equal(
                            prefix
                                .chars()
                                .next()
                                .into_iter()
                                .flat_map(|c| c.to_lowercase()),
                            buffer_window
                                .chars()
                                .next()
                                .into_iter()
                                .flat_map(|c| c.to_lowercase()),
                        )
                    })
                    .map(|(ix, prefix)| StringMatchCandidate::new(ix, prefix))
                    .collect::<Vec<StringMatchCandidate>>();

                matches.extend(
                    fuzzy::match_strings(
                        &candidates,
                        &buffer_window,
                        buffer_window.chars().any(|c| c.is_uppercase()),
                        true,
                        MAX_RESULTS - matches.len(), // always prioritize longer snippets
                        &Default::default(),
                        executor.clone(),
                    )
                    .await
                    .into_iter()
                    .map(|string_match| (string_match, buffer_window.len())),
                );

                if matches.len() >= MAX_RESULTS {
                    break;
                }
            }

            let to_lsp = |point: &text::Anchor| {
                let end = text::ToPointUtf16::to_point_utf16(point, &snapshot);
                point_to_lsp(end)
            };
            let lsp_end = to_lsp(&buffer_anchor);

            if matches.len() >= MAX_RESULTS {
                is_incomplete = true;
            }

            completions.extend(matches.iter().map(|(string_match, buffer_window_len)| {
                let ((snippet_index, prefix_index), matching_prefix, _snippet_word_count) =
                    sorted_snippet_candidates[string_match.candidate_id];
                let snippet = &snippets[snippet_index];
                let start = buffer_offset - buffer_window_len;
                let start = snapshot.anchor_before(start);
                let range = start..buffer_anchor;
                let lsp_start = to_lsp(&start);
                let lsp_range = lsp::Range {
                    start: lsp_start,
                    end: lsp_end,
                };
                Completion {
                    replace_range: range,
                    new_text: snippet.body.clone(),
                    source: CompletionSource::Lsp {
                        insert_range: None,
                        server_id: LanguageServerId(usize::MAX),
                        resolved: true,
                        lsp_completion: Box::new(lsp::CompletionItem {
                            label: matching_prefix.clone(),
                            kind: Some(CompletionItemKind::SNIPPET),
                            label_details: snippet.description.as_ref().map(|description| {
                                lsp::CompletionItemLabelDetails {
                                    detail: Some(description.clone()),
                                    description: None,
                                }
                            }),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            text_edit: Some(lsp::CompletionTextEdit::InsertAndReplace(
                                lsp::InsertReplaceEdit {
                                    new_text: snippet.body.clone(),
                                    insert: lsp_range,
                                    replace: lsp_range,
                                },
                            )),
                            filter_text: Some(snippet.body.clone()),
                            sort_text: Some(char::MAX.to_string()),
                            ..lsp::CompletionItem::default()
                        }),
                        lsp_defaults: None,
                    },
                    label: CodeLabel {
                        text: matching_prefix.clone(),
                        runs: Vec::new(),
                        filter_range: 0..matching_prefix.len(),
                    },
                    icon_path: None,
                    documentation: Some(CompletionDocumentation::SingleLineAndMultiLinePlainText {
                        single_line: snippet.name.clone().into(),
                        plain_text: snippet
                            .description
                            .clone()
                            .map(|description| description.into()),
                    }),
                    insert_text_mode: None,
                    confirm: None,
                    match_start: Some(start),
                    snippet_deduplication_key: Some((snippet_index, prefix_index)),
                }
            }));
        }

        Ok(CompletionResponse {
            completions,
            display_options: CompletionDisplayOptions::default(),
            is_incomplete,
        })
    })
}

impl CompletionProvider for Entity<Project> {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: text::Anchor,
        options: CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        self.update(cx, |project, cx| {
            let task = project.completions(buffer, buffer_position, options, cx);
            cx.background_spawn(task)
        })
    }

    fn resolve_completions(
        &self,
        buffer: Entity<Buffer>,
        completion_indices: Vec<usize>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        cx: &mut Context<Editor>,
    ) -> Task<Result<bool>> {
        self.update(cx, |project, cx| {
            project.lsp_store().update(cx, |lsp_store, cx| {
                lsp_store.resolve_completions(buffer, completion_indices, completions, cx)
            })
        })
    }

    fn apply_additional_edits_for_completion(
        &self,
        buffer: Entity<Buffer>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        completion_index: usize,
        push_to_history: bool,
        all_commit_ranges: Vec<Range<language::Anchor>>,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Option<language::Transaction>>> {
        self.update(cx, |project, cx| {
            project.lsp_store().update(cx, |lsp_store, cx| {
                lsp_store.apply_additional_edits_for_completion(
                    buffer,
                    completions,
                    completion_index,
                    push_to_history,
                    all_commit_ranges,
                    cx,
                )
            })
        })
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        text: &str,
        trigger_in_words: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        let mut chars = text.chars();
        let char = if let Some(char) = chars.next() {
            char
        } else {
            return false;
        };
        if chars.next().is_some() {
            return false;
        }

        let buffer = buffer.read(cx);
        let snapshot = buffer.snapshot();
        let classifier = snapshot
            .char_classifier_at(position)
            .scope_context(Some(CharScopeContext::Completion));
        if trigger_in_words && classifier.is_word(char) {
            return true;
        }

        buffer.completion_triggers().contains(text)
    }

    fn show_snippets(&self) -> bool {
        true
    }
}

pub(crate) fn split_words(text: &str) -> impl std::iter::Iterator<Item = &str> + '_ {
    let mut prev_index = 0;
    let mut prev_codepoint: Option<char> = None;
    text.char_indices()
        .chain([(text.len(), '\0')])
        .filter_map(move |(index, codepoint)| {
            let prev_codepoint = prev_codepoint.replace(codepoint)?;
            let is_boundary = index == text.len()
                || !prev_codepoint.is_uppercase() && codepoint.is_uppercase()
                || !prev_codepoint.is_alphanumeric() && codepoint.is_alphanumeric();
            if is_boundary {
                let chunk = &text[prev_index..index];
                prev_index = index;
                Some(chunk)
            } else {
                None
            }
        })
}

/// Given a string of text immediately before the cursor, iterates over possible
/// strings a snippet could match to. More precisely: returns an iterator over
/// suffixes of `text` created by splitting at word boundaries (before & after
/// every non-word character).
///
/// Shorter suffixes are returned first.
pub(crate) fn snippet_candidate_suffixes<'a>(
    text: &'a str,
    is_word_char: &'a dyn Fn(char) -> bool,
) -> impl std::iter::Iterator<Item = &'a str> + 'a {
    let mut prev_index = text.len();
    let mut prev_codepoint = None;
    text.char_indices()
        .rev()
        .chain([(0, '\0')])
        .filter_map(move |(index, codepoint)| {
            let prev_index = std::mem::replace(&mut prev_index, index);
            let prev_codepoint = prev_codepoint.replace(codepoint)?;
            if is_word_char(prev_codepoint) && is_word_char(codepoint) {
                None
            } else {
                let chunk = &text[prev_index..]; // go to end of string
                Some(chunk)
            }
        })
}
