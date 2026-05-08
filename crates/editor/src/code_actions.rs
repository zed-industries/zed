use super::*;

impl Editor {
    /// Toggles an action selection menu for the latest selection.
    /// May show LSP code actions, code lens' command, runnables and potentially more entities applicable as actions.
    /// Previous menu toggled with this method will be closed.
    pub fn toggle_code_actions(
        &mut self,
        action: &ToggleCodeActions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let quick_launch = action.quick_launch;
        let mut context_menu = self.context_menu.borrow_mut();
        if let Some(CodeContextMenu::CodeActions(code_actions)) = context_menu.as_ref() {
            if code_actions.deployed_from == action.deployed_from {
                // Toggle if we're selecting the same one
                *context_menu = None;
                cx.notify();
                return;
            } else {
                // Otherwise, clear it and start a new one
                *context_menu = None;
                cx.notify();
            }
        }
        drop(context_menu);
        let snapshot = self.snapshot(window, cx);
        let deployed_from = action.deployed_from.clone();
        let action = action.clone();
        self.completion_tasks.clear();
        self.discard_edit_prediction(EditPredictionDiscardReason::Ignored, cx);

        let multibuffer_point = match &action.deployed_from {
            Some(CodeActionSource::Indicator(row)) | Some(CodeActionSource::RunMenu(row)) => {
                DisplayPoint::new(*row, 0).to_point(&snapshot)
            }
            _ => self
                .selections
                .newest::<Point>(&snapshot.display_snapshot)
                .head(),
        };
        let Some((buffer, buffer_row)) = snapshot
            .buffer_snapshot()
            .buffer_line_for_row(MultiBufferRow(multibuffer_point.row))
            .and_then(|(buffer_snapshot, range)| {
                self.buffer()
                    .read(cx)
                    .buffer(buffer_snapshot.remote_id())
                    .map(|buffer| (buffer, range.start.row))
            })
        else {
            return;
        };
        let buffer_id = buffer.read(cx).remote_id();
        let tasks = self
            .runnables
            .runnables((buffer_id, buffer_row))
            .map(|t| Arc::new(t.to_owned()));

        let project = self.project.clone();
        let runnable_task = match deployed_from {
            Some(CodeActionSource::Indicator(_)) => Task::ready(Ok(Default::default())),
            _ => {
                let mut task_context_task = Task::ready(Ok(None));
                let workspace = self.workspace().map(|w| w.downgrade());
                if let Some(tasks) = &tasks
                    && let Some(project) = project
                {
                    task_context_task =
                        Self::build_tasks_context(&project, &buffer, buffer_row, tasks, cx);
                }

                cx.spawn_in(window, {
                    let buffer = buffer.clone();
                    async move |editor, cx| {
                        let task_context = match workspace {
                            Some(ws) => task_context_task
                                .await
                                .notify_workspace_async_err(ws, cx)
                                .flatten(),
                            None => task_context_task.await.ok().flatten(),
                        };

                        let resolved_tasks =
                            tasks
                                .zip(task_context.clone())
                                .map(|(tasks, task_context)| ResolvedTasks {
                                    templates: tasks.resolve(&task_context).collect(),
                                    position: snapshot.buffer_snapshot().anchor_before(Point::new(
                                        multibuffer_point.row,
                                        tasks.column,
                                    )),
                                });
                        let debug_scenarios = editor
                            .update(cx, |editor, cx| {
                                editor.debug_scenarios(&resolved_tasks, &buffer, cx)
                            })?
                            .await;
                        anyhow::Ok((resolved_tasks, debug_scenarios, task_context))
                    }
                })
            }
        };

        let toggle_task = cx.spawn_in(window, async move |editor, cx| {
            let (resolved_tasks, debug_scenarios, task_context) = runnable_task.await?;

            let code_actions: Option<Rc<[AvailableCodeAction]>> =
                if let Some(CodeActionSource::RunMenu(_)) = &deployed_from {
                    None
                } else {
                    let fetch_tasks = editor.update_in(cx, |editor, window, cx| {
                        let buffer_snapshot = buffer.read(cx).snapshot();
                        let line_end_column = buffer_snapshot.line_len(buffer_row);
                        let start_offset =
                            buffer_snapshot.point_to_offset(text::Point::new(buffer_row, 0));
                        let end_offset = buffer_snapshot
                            .point_to_offset(text::Point::new(buffer_row, line_end_column));
                        let start_anchor = buffer_snapshot.anchor_before(start_offset);
                        let end_anchor = buffer_snapshot.anchor_after(end_offset);

                        let providers = editor.code_action_providers.clone();
                        let tasks: Vec<_> = providers
                            .iter()
                            .map(|provider| {
                                provider.code_actions(&buffer, start_anchor..end_anchor, window, cx)
                            })
                            .collect();
                        (providers, tasks)
                    })?;

                    let (providers, tasks) = fetch_tasks;
                    let all_results = future::join_all(tasks).await;

                    let cached = editor.update(cx, |editor, _cx| {
                        editor.cached_code_actions_for_row(buffer_id, buffer_row)
                    })?;

                    let mut actions: Vec<AvailableCodeAction> =
                        cached.map(|rc| rc.to_vec()).unwrap_or_default();

                    for (provider, provider_actions) in providers.iter().zip(all_results) {
                        if let Some(provider_actions) = provider_actions.log_err() {
                            for action in provider_actions {
                                let is_duplicate = actions.iter().any(|a| {
                                    a.action.server_id == action.server_id
                                        && a.action.lsp_action == action.lsp_action
                                });
                                if !is_duplicate {
                                    actions.push(AvailableCodeAction {
                                        action,
                                        provider: provider.clone(),
                                    });
                                }
                            }
                        }
                    }

                    if actions.is_empty() {
                        None
                    } else {
                        Some(Rc::from(actions))
                    }
                };

            editor.update_in(cx, |editor, window, cx| {
                let spawn_straight_away = quick_launch
                    && resolved_tasks
                        .as_ref()
                        .is_some_and(|tasks| tasks.templates.len() == 1)
                    && code_actions
                        .as_ref()
                        .is_none_or(|actions| actions.is_empty())
                    && debug_scenarios.is_empty();

                crate::hover_popover::hide_hover(editor, cx);
                let actions = CodeActionContents::new(
                    resolved_tasks,
                    code_actions,
                    debug_scenarios,
                    task_context.unwrap_or_default(),
                );

                // Don't show the menu if there are no actions available
                if actions.is_empty() {
                    cx.notify();
                    return Task::ready(Ok(()));
                }

                *editor.context_menu.borrow_mut() =
                    Some(CodeContextMenu::CodeActions(CodeActionsMenu {
                        buffer,
                        actions,
                        selected_item: Default::default(),
                        scroll_handle: UniformListScrollHandle::default(),
                        deployed_from,
                    }));
                cx.notify();
                if spawn_straight_away
                    && let Some(task) = editor.confirm_code_action(
                        &ConfirmCodeAction { item_ix: Some(0) },
                        window,
                        cx,
                    )
                {
                    return task;
                }

                Task::ready(Ok(()))
            })
        });
        self.runnables_for_selection_toggle = cx.background_spawn(async move {
            match toggle_task.await {
                Ok(code_action_spawn) => match code_action_spawn.await {
                    Ok(()) => {}
                    Err(e) => log::error!("failed to spawn a toggled code action: {e:#}"),
                },
                Err(e) => log::error!("failed to toggle code actions: {e:#}"),
            }
        })
    }

    pub fn confirm_code_action(
        &mut self,
        action: &ConfirmCodeAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if self.read_only(cx) {
            return None;
        }

        let actions_menu =
            if let CodeContextMenu::CodeActions(menu) = self.hide_context_menu(window, cx)? {
                menu
            } else {
                return None;
            };

        let action_ix = action.item_ix.unwrap_or(actions_menu.selected_item);
        let action = actions_menu.actions.get(action_ix)?;
        let title = action.label();
        let buffer = actions_menu.buffer;
        let workspace = self.workspace()?;

        match action {
            CodeActionsItem::Task(task_source_kind, resolved_task) => {
                workspace.update(cx, |workspace, cx| {
                    workspace.schedule_resolved_task(
                        task_source_kind,
                        resolved_task,
                        false,
                        window,
                        cx,
                    );

                    Some(Task::ready(Ok(())))
                })
            }
            CodeActionsItem::CodeAction { action, provider } => {
                if code_lens::try_handle_client_command(&action, self, &workspace, window, cx) {
                    return Some(Task::ready(Ok(())));
                }

                let apply_code_action =
                    provider.apply_code_action(buffer, action, true, window, cx);
                let workspace = workspace.downgrade();
                Some(cx.spawn_in(window, async move |editor, cx| {
                    let project_transaction = apply_code_action.await?;
                    Self::open_project_transaction(
                        &editor,
                        workspace,
                        project_transaction,
                        title,
                        cx,
                    )
                    .await
                }))
            }
            CodeActionsItem::DebugScenario(scenario) => {
                let context = actions_menu.actions.context.into();

                workspace.update(cx, |workspace, cx| {
                    dap::send_telemetry(&scenario, TelemetrySpawnLocation::Gutter, cx);
                    workspace.start_debug_session(
                        scenario,
                        context,
                        Some(buffer),
                        None,
                        window,
                        cx,
                    );
                });
                Some(Task::ready(Ok(())))
            }
        }
    }

    pub fn code_actions_enabled_for_toolbar(&self, cx: &App) -> bool {
        !self.code_action_providers.is_empty()
            && EditorSettings::get_global(cx).toolbar.code_actions
    }

    fn cursor_buffer_position_for_cache(&self, cx: &App) -> Option<(BufferId, BufferRow)> {
        let newest = self.selections.newest_anchor();
        if newest.head().diff_base_anchor().is_some() {
            return None;
        }
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let head_point = newest.head().to_point(&snapshot);
        let (buffer_snapshot, range) =
            snapshot.buffer_line_for_row(MultiBufferRow(head_point.row))?;
        Some((buffer_snapshot.remote_id(), range.start.row))
    }

    pub fn has_cached_code_actions_at_cursor(&self, cx: &App) -> bool {
        let Some((buffer_id, buffer_row)) = self.cursor_buffer_position_for_cache(cx) else {
            return false;
        };
        self.code_action_cache
            .buffers
            .get(&buffer_id)
            .is_some_and(|cached| {
                cached
                    .actions
                    .iter()
                    .any(|a| a.row_range.start <= buffer_row && buffer_row < a.row_range.end)
            })
    }

    pub(crate) fn cursor_within_cached_code_action_rows(&self, cx: &App) -> bool {
        let Some((buffer_id, buffer_row)) = self.cursor_buffer_position_for_cache(cx) else {
            return false;
        };
        self.code_action_cache
            .buffers
            .get(&buffer_id)
            .is_some_and(|cached| {
                cached.fetched_rows.start <= buffer_row && buffer_row < cached.fetched_rows.end
            })
    }

    pub(super) fn cached_code_actions_for_row(
        &self,
        buffer_id: BufferId,
        buffer_row: BufferRow,
    ) -> Option<Rc<[AvailableCodeAction]>> {
        let cached = self.code_action_cache.buffers.get(&buffer_id)?;
        let actions: Vec<AvailableCodeAction> = cached
            .actions
            .iter()
            .filter(|a| {
                let range = a.diagnostic_row_range.as_ref().unwrap_or(&a.row_range);
                range.start <= buffer_row && buffer_row < range.end
            })
            .map(|a| a.available.clone())
            .collect();
        if actions.is_empty() {
            None
        } else {
            Some(Rc::from(actions))
        }
    }

    pub fn context_menu(&self) -> &RefCell<Option<CodeContextMenu>> {
        &self.context_menu
    }

    pub(super) fn render_inline_code_actions(
        &self,
        icon_size: ui::IconSize,
        display_row: DisplayRow,
        is_active: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let show_tooltip = !self.context_menu_visible();
        IconButton::new("inline_code_actions", ui::IconName::BoltFilled)
            .icon_size(icon_size)
            .shape(ui::IconButtonShape::Square)
            .icon_color(ui::Color::Hidden)
            .toggle_state(is_active)
            .when(show_tooltip, |this| {
                this.tooltip({
                    let focus_handle = self.focus_handle.clone();
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            "Toggle Code Actions",
                            &ToggleCodeActions {
                                deployed_from: None,
                                quick_launch: false,
                            },
                            &focus_handle,
                            cx,
                        )
                    }
                })
            })
            .on_click(cx.listener(move |editor, _: &ClickEvent, window, cx| {
                window.focus(&editor.focus_handle(cx), cx);
                editor.toggle_code_actions(
                    &crate::actions::ToggleCodeActions {
                        deployed_from: Some(crate::actions::CodeActionSource::Indicator(
                            display_row,
                        )),
                        quick_launch: false,
                    },
                    window,
                    cx,
                );
            }))
            .into_any_element()
    }

    pub(crate) fn refresh_code_actions_for_viewport(
        &mut self,
        reason: CodeActionRefreshReason,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.code_action_providers.is_empty() {
            return;
        }

        let debounce = match reason {
            CodeActionRefreshReason::BufferEdited => CODE_ACTIONS_DEBOUNCE_TIMEOUT,
            CodeActionRefreshReason::NewLinesShown => Duration::from_millis(100),
            CodeActionRefreshReason::ProvidersChanged => Duration::ZERO,
        };
        let invalidate = matches!(
            reason,
            CodeActionRefreshReason::BufferEdited | CodeActionRefreshReason::ProvidersChanged
        );

        let visible_ranges = self.visible_buffer_ranges(cx);
        let multi_buffer = self.buffer.read(cx);
        self.code_action_cache
            .buffers
            .retain(|id, _| multi_buffer.buffer(*id).is_some());
        self.code_action_cache
            .refresh_tasks
            .retain(|id, _| multi_buffer.buffer(*id).is_some());

        for (buffer_snapshot, visible_offset_range, _) in visible_ranges {
            let buffer_id = buffer_snapshot.remote_id();
            let Some(buffer_handle) = multi_buffer.buffer(buffer_id) else {
                continue;
            };

            let visible_start_point = buffer_snapshot.offset_to_point(visible_offset_range.start.0);
            let visible_end_point = buffer_snapshot.offset_to_point(visible_offset_range.end.0);
            let visible_rows = visible_start_point.row..visible_end_point.row + 1;

            let version = buffer_snapshot.version().clone();

            let cache_covers_visible_rows = !invalidate
                && self
                    .code_action_cache
                    .buffers
                    .get(&buffer_id)
                    .is_some_and(|cached| {
                        cached.version == version
                            && cached.fetched_rows.start <= visible_rows.start
                            && cached.fetched_rows.end >= visible_rows.end
                    });
            if cache_covers_visible_rows {
                continue;
            }

            let visible_row_count = visible_rows.end - visible_rows.start;
            let margin = visible_row_count / 2;
            let max_row = buffer_snapshot.max_point().row;
            let fetch_start_row = visible_rows.start.saturating_sub(margin);
            let fetch_end_row = (visible_rows.end + margin).min(max_row + 1);

            let fetch_end_last_row = (fetch_end_row.saturating_sub(1)).min(max_row);
            let fetch_start_offset =
                buffer_snapshot.point_to_offset(text::Point::new(fetch_start_row, 0));
            let fetch_end_offset = buffer_snapshot.point_to_offset(text::Point::new(
                fetch_end_last_row,
                buffer_snapshot.line_len(fetch_end_last_row),
            ));

            let start_anchor = buffer_snapshot.anchor_before(fetch_start_offset);
            let end_anchor = buffer_snapshot.anchor_after(fetch_end_offset);

            let task = cx.spawn_in(window, {
                let buffer_handle = buffer_handle.clone();
                async move |editor, cx| {
                    if !debounce.is_zero() {
                        cx.background_executor().timer(debounce).await;
                    }

                    let providers_and_tasks = editor
                        .update_in(cx, |editor, window, cx| {
                            let providers = editor.code_action_providers.clone();
                            let tasks = providers
                                .iter()
                                .map(|provider| {
                                    provider.code_actions(
                                        &buffer_handle,
                                        start_anchor..end_anchor,
                                        window,
                                        cx,
                                    )
                                })
                                .collect::<Vec<_>>();
                            (providers, tasks)
                        })
                        .log_err();

                    let Some((providers, tasks)) = providers_and_tasks else {
                        return;
                    };

                    let all_results = future::join_all(tasks).await;

                    editor
                        .update(cx, |editor, cx| {
                            let buffer_snapshot = buffer_handle.read(cx).snapshot();
                            let current_version = buffer_snapshot.version().clone();
                            let fetch_start_row = start_anchor.to_point(&buffer_snapshot).row;
                            let fetch_end_row = end_anchor.to_point(&buffer_snapshot).row + 1;
                            let fetch_rows = fetch_start_row..fetch_end_row;

                            let mut cached_actions = Vec::new();
                            for (provider, provider_actions) in providers.iter().zip(all_results) {
                                if let Some(provider_actions) = provider_actions.log_err() {
                                    for action in provider_actions {
                                        let diagnostic_row_range =
                                            extract_diagnostic_row_range(&action);
                                        let start_point =
                                            action.range.start.to_point(&buffer_snapshot);
                                        let end_point = action.range.end.to_point(&buffer_snapshot);
                                        cached_actions.push(CachedCodeAction {
                                            row_range: start_point.row..end_point.row + 1,
                                            diagnostic_row_range,
                                            available: AvailableCodeAction {
                                                action,
                                                provider: provider.clone(),
                                            },
                                        });
                                    }
                                }
                            }

                            editor.code_action_cache.buffers.insert(
                                buffer_id,
                                BufferCodeActions {
                                    version: current_version,
                                    fetched_rows: fetch_rows,
                                    actions: cached_actions,
                                },
                            );
                            cx.notify();
                        })
                        .log_err();
                }
            });

            self.code_action_cache.refresh_tasks.insert(buffer_id, task);
        }
    }

    fn debug_scenarios(
        &mut self,
        resolved_tasks: &Option<ResolvedTasks>,
        buffer: &Entity<Buffer>,
        cx: &mut App,
    ) -> Task<Vec<task::DebugScenario>> {
        maybe!({
            let project = self.project()?;
            let dap_store = project.read(cx).dap_store();
            let mut scenarios = vec![];
            let resolved_tasks = resolved_tasks.as_ref()?;
            let buffer = buffer.read(cx);
            let language = buffer.language()?;
            let debug_adapter = LanguageSettings::for_buffer(&buffer, cx)
                .debuggers
                .first()
                .map(SharedString::from)
                .or_else(|| language.config().debuggers.first().map(SharedString::from))?;

            dap_store.update(cx, |dap_store, cx| {
                for (_, task) in &resolved_tasks.templates {
                    let maybe_scenario = dap_store.debug_scenario_for_build_task(
                        task.original_task().clone(),
                        debug_adapter.clone().into(),
                        task.display_label().to_owned().into(),
                        cx,
                    );
                    scenarios.push(maybe_scenario);
                }
            });
            Some(cx.background_spawn(async move {
                futures::future::join_all(scenarios)
                    .await
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
            }))
        })
        .unwrap_or_else(|| Task::ready(vec![]))
    }
}

pub trait CodeActionProvider {
    fn id(&self) -> Arc<str>;

    fn code_actions(
        &self,
        buffer: &Entity<Buffer>,
        range: Range<text::Anchor>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<CodeAction>>>;

    fn apply_code_action(
        &self,
        buffer_handle: Entity<Buffer>,
        action: CodeAction,
        push_to_history: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<ProjectTransaction>>;
}

impl CodeActionProvider for Entity<Project> {
    fn id(&self) -> Arc<str> {
        "project".into()
    }

    fn code_actions(
        &self,
        buffer: &Entity<Buffer>,
        range: Range<text::Anchor>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<CodeAction>>> {
        self.update(cx, |project, cx| {
            let code_lens_actions = if EditorSettings::get_global(cx).code_lens.show_in_menu() {
                Some(project.code_lens_actions(buffer, range.clone(), cx))
            } else {
                None
            };
            let code_actions = project.code_actions(buffer, range, None, cx);
            cx.background_spawn(async move {
                let code_lens_actions = match code_lens_actions {
                    Some(task) => task.await.context("code lens fetch")?.unwrap_or_default(),
                    None => Vec::new(),
                };
                let code_actions = code_actions
                    .await
                    .context("code action fetch")?
                    .unwrap_or_default();
                Ok(code_lens_actions.into_iter().chain(code_actions).collect())
            })
        })
    }

    fn apply_code_action(
        &self,
        buffer_handle: Entity<Buffer>,
        action: CodeAction,
        push_to_history: bool,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<ProjectTransaction>> {
        self.update(cx, |project, cx| {
            project.apply_code_action(buffer_handle, action, push_to_history, cx)
        })
    }
}
