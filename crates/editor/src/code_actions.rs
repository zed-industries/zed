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

            let code_actions = if let Some(CodeActionSource::RunMenu(_)) = &deployed_from {
                None
            } else {
                editor.update(cx, |editor, _cx| match &editor.code_actions_for_selection {
                    CodeActionsForSelection::None => None,
                    CodeActionsForSelection::Fetching(task) => Some(task.clone()),
                    CodeActionsForSelection::Ready(action_fetch_ready) => {
                        Some(Task::ready(Some(action_fetch_ready.clone())).shared())
                    }
                })?
            };
            let code_actions = match code_actions {
                Some(code_actions) => code_actions
                    .await
                    .filter(|ActionFetchReady { location, .. }| {
                        let snapshot = location.buffer.read_with(cx, |buffer, _| buffer.snapshot());
                        let point_range = location.range.to_point(&snapshot);
                        (point_range.start.row..=point_range.end.row).contains(&buffer_row)
                    })
                    .map(|ActionFetchReady { actions, .. }| actions),
                None => None,
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

    pub fn has_available_code_actions_for_selection(&self) -> bool {
        if let CodeActionsForSelection::Ready(ready) = &self.code_actions_for_selection {
            !ready.actions.is_empty()
        } else {
            false
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

    pub(super) fn refresh_code_actions_for_selection(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.code_actions_for_selection = CodeActionsForSelection::Fetching(
            cx.spawn_in(window, async move |editor, cx| {
                cx.background_executor()
                    .timer(CODE_ACTIONS_DEBOUNCE_TIMEOUT)
                    .await;

                let (start_buffer, start, _, end, _newest_selection) = editor
                    .update(cx, |editor, cx| {
                        let newest_selection = editor.selections.newest_anchor().clone();
                        if newest_selection.head().diff_base_anchor().is_some() {
                            return None;
                        }
                        let display_snapshot = editor.display_snapshot(cx);
                        let newest_selection_adjusted =
                            editor.selections.newest_adjusted(&display_snapshot);
                        let buffer = editor.buffer.read(cx);

                        let (start_buffer, start) =
                            buffer.text_anchor_for_position(newest_selection_adjusted.start, cx)?;
                        let (end_buffer, end) =
                            buffer.text_anchor_for_position(newest_selection_adjusted.end, cx)?;

                        Some((start_buffer, start, end_buffer, end, newest_selection))
                    })
                    .ok()
                    .flatten()
                    .filter(|(start_buffer, _, end_buffer, _, _)| start_buffer == end_buffer)?;

                let (providers, tasks) = editor
                    .update_in(cx, |editor, window, cx| {
                        let providers = editor.code_action_providers.clone();
                        let tasks = editor
                            .code_action_providers
                            .iter()
                            .map(|provider| {
                                provider.code_actions(&start_buffer, start..end, window, cx)
                            })
                            .collect::<Vec<_>>();
                        (providers, tasks)
                    })
                    .ok()?;

                let mut actions = Vec::new();
                for (provider, provider_actions) in
                    providers.into_iter().zip(future::join_all(tasks).await)
                {
                    if let Some(provider_actions) = provider_actions.log_err() {
                        actions.extend(provider_actions.into_iter().map(|action| {
                            AvailableCodeAction {
                                action,
                                provider: provider.clone(),
                            }
                        }));
                    }
                }

                editor
                    .update(cx, |editor, cx| {
                        let new_actions = if actions.is_empty() {
                            editor.code_actions_for_selection = CodeActionsForSelection::None;
                            None
                        } else {
                            let new_actions = ActionFetchReady {
                                location: Location {
                                    buffer: start_buffer,
                                    range: start..end,
                                },
                                actions: Rc::from(actions),
                            };
                            editor.code_actions_for_selection =
                                CodeActionsForSelection::Ready(new_actions.clone());
                            Some(new_actions)
                        };
                        cx.notify();
                        new_actions
                    })
                    .ok()
                    .flatten()
            })
            .shared(),
        );
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
