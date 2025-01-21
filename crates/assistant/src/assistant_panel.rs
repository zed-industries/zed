use crate::context_editor::{
    ContextEditor, ContextEditorToolbarItem, ContextEditorToolbarItemEvent, DEFAULT_TAB_TITLE,
};
use crate::context_history::ContextHistory;
use crate::{
    slash_command::SlashCommandCompletionProvider,
    terminal_inline_assistant::TerminalInlineAssistant, Context, ContextId, ContextStore,
    ContextStoreEvent, DeployHistory, DeployPromptLibrary, InlineAssistant, InsertDraggedFiles,
    NewContext, ToggleFocus, ToggleModelSelector,
};
use anyhow::Result;
use assistant_settings::{AssistantDockPosition, AssistantSettings};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_tool::ToolWorkingSet;
use client::{proto, Client, Status};
use collections::HashMap;
use editor::{Editor, EditorEvent};
use fs::Fs;
use gpui::{
    canvas, div, prelude::*, Action, AnyView, AppContext, AsyncWindowContext, EventEmitter,
    ExternalPaths, FocusHandle, FocusableView, InteractiveElement, IntoElement, Model,
    ParentElement, Pixels, Render, SharedString, StatefulInteractiveElement, Styled, Subscription,
    Task, UpdateGlobal, View, WeakView,
};
use language::{LanguageRegistry, LspAdapterDelegate};
use language_model::{
    LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID,
};
use language_model_selector::LanguageModelSelector;
use project::lsp_store::LocalLspAdapterDelegate;
use project::Project;
use prompt_library::{open_prompt_library, PromptBuilder, PromptLibrary};
use search::{buffer_search::DivRegistrar, BufferSearchBar};
use settings::{update_settings_file, Settings};
use smol::stream::StreamExt;
use std::{ops::ControlFlow, path::PathBuf, sync::Arc};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use ui::{prelude::*, ContextMenu, ElevationIndex, PopoverMenu, PopoverMenuHandle, Tooltip};
use util::{maybe, ResultExt};
use workspace::DraggedTab;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::Item,
    pane, DraggedSelection, Pane, ShowConfiguration, ToggleZoom, Workspace,
};
use zed_actions::InlineAssist;

pub fn init(cx: &mut AppContext) {
    workspace::FollowableViewRegistry::register::<ContextEditor>(cx);
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, cx| {
                    let settings = AssistantSettings::get_global(cx);
                    if !settings.enabled {
                        return;
                    }

                    workspace.toggle_panel_focus::<AssistantPanel>(cx);
                })
                .register_action(ContextEditor::quote_selection)
                .register_action(ContextEditor::insert_selection)
                .register_action(ContextEditor::copy_code)
                .register_action(ContextEditor::insert_dragged_files)
                .register_action(AssistantPanel::show_configuration)
                .register_action(AssistantPanel::create_new_context)
                .register_action(AssistantPanel::restart_context_servers);
        },
    )
    .detach();

    cx.observe_new_views(
        |terminal_panel: &mut TerminalPanel, cx: &mut ViewContext<TerminalPanel>| {
            let settings = AssistantSettings::get_global(cx);
            terminal_panel.set_assistant_enabled(settings.enabled, cx);
        },
    )
    .detach();
}

pub enum AssistantPanelEvent {
    ContextEdited,
}

pub struct AssistantPanel {
    pane: View<Pane>,
    workspace: WeakView<Workspace>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    project: Model<Project>,
    context_store: Model<ContextStore>,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    subscriptions: Vec<Subscription>,
    model_selector_menu_handle: PopoverMenuHandle<LanguageModelSelector>,
    model_summary_editor: View<Editor>,
    authenticate_provider_task: Option<(LanguageModelProviderId, Task<()>)>,
    configuration_subscription: Option<Subscription>,
    client_status: Option<client::Status>,
    watch_client_status: Option<Task<()>>,
    pub(crate) show_zed_ai_notice: bool,
}

enum InlineAssistTarget {
    Editor(View<Editor>, bool),
    Terminal(View<TerminalView>),
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let slash_commands = Arc::new(SlashCommandWorkingSet::default());
            let tools = Arc::new(ToolWorkingSet::default());
            let context_store = workspace
                .update(&mut cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    ContextStore::new(project, prompt_builder.clone(), slash_commands, tools, cx)
                })?
                .await?;

            workspace.update(&mut cx, |workspace, cx| {
                // TODO: deserialize state.
                cx.new_view(|cx| Self::new(workspace, context_store, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        context_store: Model<ContextStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let model_selector_menu_handle = PopoverMenuHandle::default();
        let model_summary_editor = cx.new_view(Editor::single_line);
        let context_editor_toolbar = cx.new_view(|cx| {
            ContextEditorToolbarItem::new(
                workspace,
                model_selector_menu_handle.clone(),
                model_summary_editor.clone(),
                cx,
            )
        });

        let pane = cx.new_view(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.project().clone(),
                Default::default(),
                None,
                NewContext.boxed_clone(),
                cx,
            );

            let project = workspace.project().clone();
            pane.set_custom_drop_handle(cx, move |_, dropped_item, cx| {
                let action = maybe!({
                    if project.read(cx).is_local() {
                        if let Some(paths) = dropped_item.downcast_ref::<ExternalPaths>() {
                            return Some(InsertDraggedFiles::ExternalFiles(paths.paths().to_vec()));
                        }
                    }

                    let project_paths = if let Some(tab) = dropped_item.downcast_ref::<DraggedTab>()
                    {
                        if &tab.pane == cx.view() {
                            return None;
                        }
                        let item = tab.pane.read(cx).item_for_index(tab.ix);
                        Some(
                            item.and_then(|item| item.project_path(cx))
                                .into_iter()
                                .collect::<Vec<_>>(),
                        )
                    } else if let Some(selection) = dropped_item.downcast_ref::<DraggedSelection>()
                    {
                        Some(
                            selection
                                .items()
                                .filter_map(|item| {
                                    project.read(cx).path_for_entry(item.entry_id, cx)
                                })
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }?;

                    let paths = project_paths
                        .into_iter()
                        .filter_map(|project_path| {
                            let worktree = project
                                .read(cx)
                                .worktree_for_id(project_path.worktree_id, cx)?;

                            let mut full_path = PathBuf::from(worktree.read(cx).root_name());
                            full_path.push(&project_path.path);
                            Some(full_path)
                        })
                        .collect::<Vec<_>>();

                    Some(InsertDraggedFiles::ProjectPaths(paths))
                });

                if let Some(action) = action {
                    cx.dispatch_action(action.boxed_clone());
                }

                ControlFlow::Break(())
            });

            pane.set_can_navigate(true, cx);
            pane.display_nav_history_buttons(None);
            pane.set_should_display_tab_bar(|_| true);
            pane.set_render_tab_bar_buttons(cx, move |pane, cx| {
                let focus_handle = pane.focus_handle(cx);
                let left_children = IconButton::new("history", IconName::HistoryRerun)
                    .icon_size(IconSize::Small)
                    .on_click(cx.listener({
                        let focus_handle = focus_handle.clone();
                        move |_, _, cx| {
                            focus_handle.focus(cx);
                            cx.dispatch_action(DeployHistory.boxed_clone())
                        }
                    }))
                    .tooltip({
                        let focus_handle = focus_handle.clone();
                        move |cx| {
                            Tooltip::for_action_in(
                                "Open History",
                                &DeployHistory,
                                &focus_handle,
                                cx,
                            )
                        }
                    })
                    .toggle_state(
                        pane.active_item()
                            .map_or(false, |item| item.downcast::<ContextHistory>().is_some()),
                    );
                let _pane = cx.view().clone();
                let right_children = h_flex()
                    .gap(DynamicSpacing::Base02.rems(cx))
                    .child(
                        IconButton::new("new-chat", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .on_click(
                                cx.listener(|_, _, cx| {
                                    cx.dispatch_action(NewContext.boxed_clone())
                                }),
                            )
                            .tooltip(move |cx| {
                                Tooltip::for_action_in("New Chat", &NewContext, &focus_handle, cx)
                            }),
                    )
                    .child(
                        PopoverMenu::new("assistant-panel-popover-menu")
                            .trigger(
                                IconButton::new("menu", IconName::EllipsisVertical)
                                    .icon_size(IconSize::Small)
                                    .tooltip(|cx| Tooltip::text("Toggle Assistant Menu", cx)),
                            )
                            .menu(move |cx| {
                                let zoom_label = if _pane.read(cx).is_zoomed() {
                                    "Zoom Out"
                                } else {
                                    "Zoom In"
                                };
                                let focus_handle = _pane.focus_handle(cx);
                                Some(ContextMenu::build(cx, move |menu, _| {
                                    menu.context(focus_handle.clone())
                                        .action("New Chat", Box::new(NewContext))
                                        .action("History", Box::new(DeployHistory))
                                        .action("Prompt Library", Box::new(DeployPromptLibrary))
                                        .action("Configure", Box::new(ShowConfiguration))
                                        .action(zoom_label, Box::new(ToggleZoom))
                                }))
                            }),
                    )
                    .into_any_element()
                    .into();

                (Some(left_children.into_any_element()), right_children)
            });
            pane.toolbar().update(cx, |toolbar, cx| {
                toolbar.add_item(context_editor_toolbar.clone(), cx);
                toolbar.add_item(cx.new_view(BufferSearchBar::new), cx)
            });
            pane
        });

        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe(&pane, Self::handle_pane_event),
            cx.subscribe(&context_editor_toolbar, Self::handle_toolbar_event),
            cx.subscribe(&model_summary_editor, Self::handle_summary_editor_event),
            cx.subscribe(&context_store, Self::handle_context_store_event),
            cx.subscribe(
                &LanguageModelRegistry::global(cx),
                |this, _, event: &language_model::Event, cx| match event {
                    language_model::Event::ActiveModelChanged => {
                        this.completion_provider_changed(cx);
                    }
                    language_model::Event::ProviderStateChanged => {
                        this.ensure_authenticated(cx);
                        cx.notify()
                    }
                    language_model::Event::AddedProvider(_)
                    | language_model::Event::RemovedProvider(_) => {
                        this.ensure_authenticated(cx);
                    }
                },
            ),
        ];

        let watch_client_status = Self::watch_client_status(workspace.client().clone(), cx);

        let mut this = Self {
            pane,
            workspace: workspace.weak_handle(),
            width: None,
            height: None,
            project: workspace.project().clone(),
            context_store,
            languages: workspace.app_state().languages.clone(),
            fs: workspace.app_state().fs.clone(),
            subscriptions,
            model_selector_menu_handle,
            model_summary_editor,
            authenticate_provider_task: None,
            configuration_subscription: None,
            client_status: None,
            watch_client_status: Some(watch_client_status),
            show_zed_ai_notice: false,
        };
        this.new_context(cx);
        this
    }

    fn watch_client_status(client: Arc<Client>, cx: &mut ViewContext<Self>) -> Task<()> {
        let mut status_rx = client.status();

        cx.spawn(|this, mut cx| async move {
            while let Some(status) = status_rx.next().await {
                this.update(&mut cx, |this, cx| {
                    if this.client_status.is_none()
                        || this
                            .client_status
                            .map_or(false, |old_status| old_status != status)
                    {
                        this.update_zed_ai_notice_visibility(status, cx);
                    }
                    this.client_status = Some(status);
                })
                .log_err();
            }
            this.update(&mut cx, |this, _cx| this.watch_client_status = None)
                .log_err();
        })
    }

    fn handle_pane_event(
        &mut self,
        pane: View<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        let update_model_summary = match event {
            pane::Event::Remove { .. } => {
                cx.emit(PanelEvent::Close);
                false
            }
            pane::Event::ZoomIn => {
                cx.emit(PanelEvent::ZoomIn);
                false
            }
            pane::Event::ZoomOut => {
                cx.emit(PanelEvent::ZoomOut);
                false
            }

            pane::Event::AddItem { item } => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, self.pane.clone(), cx)
                    })
                    .ok();
                true
            }

            pane::Event::ActivateItem { local, .. } => {
                if *local {
                    self.workspace
                        .update(cx, |workspace, cx| {
                            workspace.unfollow_in_pane(&pane, cx);
                        })
                        .ok();
                }
                cx.emit(AssistantPanelEvent::ContextEdited);
                true
            }
            pane::Event::RemovedItem { .. } => {
                let has_configuration_view = self
                    .pane
                    .read(cx)
                    .items_of_type::<ConfigurationView>()
                    .next()
                    .is_some();

                if !has_configuration_view {
                    self.configuration_subscription = None;
                }

                cx.emit(AssistantPanelEvent::ContextEdited);
                true
            }

            _ => false,
        };

        if update_model_summary {
            if let Some(editor) = self.active_context_editor(cx) {
                self.show_updated_summary(&editor, cx)
            }
        }
    }

    fn handle_summary_editor_event(
        &mut self,
        model_summary_editor: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if matches!(event, EditorEvent::Edited { .. }) {
            if let Some(context_editor) = self.active_context_editor(cx) {
                let new_summary = model_summary_editor.read(cx).text(cx);
                context_editor.update(cx, |context_editor, cx| {
                    context_editor.context.update(cx, |context, cx| {
                        if context.summary().is_none()
                            && (new_summary == DEFAULT_TAB_TITLE || new_summary.trim().is_empty())
                        {
                            return;
                        }
                        context.custom_summary(new_summary, cx)
                    });
                });
            }
        }
    }

    fn update_zed_ai_notice_visibility(
        &mut self,
        client_status: Status,
        cx: &mut ViewContext<Self>,
    ) {
        let active_provider = LanguageModelRegistry::read_global(cx).active_provider();

        // If we're signed out and don't have a provider configured, or we're signed-out AND Zed.dev is
        // the provider, we want to show a nudge to sign in.
        let show_zed_ai_notice = client_status.is_signed_out()
            && active_provider.map_or(true, |provider| provider.id().0 == ZED_CLOUD_PROVIDER_ID);

        self.show_zed_ai_notice = show_zed_ai_notice;
        cx.notify();
    }

    fn handle_toolbar_event(
        &mut self,
        _: View<ContextEditorToolbarItem>,
        _: &ContextEditorToolbarItemEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(context_editor) = self.active_context_editor(cx) {
            context_editor.update(cx, |context_editor, cx| {
                context_editor.context.update(cx, |context, cx| {
                    context.summarize(true, cx);
                })
            })
        }
    }

    fn handle_context_store_event(
        &mut self,
        _context_store: Model<ContextStore>,
        event: &ContextStoreEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let ContextStoreEvent::ContextCreated(context_id) = event;
        let Some(context) = self
            .context_store
            .read(cx)
            .loaded_context_for_id(&context_id, cx)
        else {
            log::error!("no context found with ID: {}", context_id.to_proto());
            return;
        };
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
            .log_err()
            .flatten();

        let assistant_panel = cx.view().downgrade();
        let editor = cx.new_view(|cx| {
            let mut editor = ContextEditor::for_context(
                context,
                self.fs.clone(),
                self.workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                assistant_panel,
                cx,
            );
            editor.insert_default_prompt(cx);
            editor
        });

        self.show_context(editor.clone(), cx);
    }

    fn completion_provider_changed(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(editor) = self.active_context_editor(cx) {
            editor.update(cx, |active_context, cx| {
                active_context
                    .context
                    .update(cx, |context, cx| context.completion_provider_changed(cx))
            })
        }

        let Some(new_provider_id) = LanguageModelRegistry::read_global(cx)
            .active_provider()
            .map(|p| p.id())
        else {
            return;
        };

        if self
            .authenticate_provider_task
            .as_ref()
            .map_or(true, |(old_provider_id, _)| {
                *old_provider_id != new_provider_id
            })
        {
            self.authenticate_provider_task = None;
            self.ensure_authenticated(cx);
        }

        if let Some(status) = self.client_status {
            self.update_zed_ai_notice_visibility(status, cx);
        }
    }

    fn ensure_authenticated(&mut self, cx: &mut ViewContext<Self>) {
        if self.is_authenticated(cx) {
            return;
        }

        let Some(provider) = LanguageModelRegistry::read_global(cx).active_provider() else {
            return;
        };

        let load_credentials = self.authenticate(cx);

        if self.authenticate_provider_task.is_none() {
            self.authenticate_provider_task = Some((
                provider.id(),
                cx.spawn(|this, mut cx| async move {
                    if let Some(future) = load_credentials {
                        let _ = future.await;
                    }
                    this.update(&mut cx, |this, _cx| {
                        this.authenticate_provider_task = None;
                    })
                    .log_err();
                }),
            ));
        }
    }

    pub fn inline_assist(
        workspace: &mut Workspace,
        action: &InlineAssist,
        cx: &mut ViewContext<Workspace>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled {
            return;
        }

        let Some(assistant_panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        let Some(inline_assist_target) =
            Self::resolve_inline_assist_target(workspace, &assistant_panel, cx)
        else {
            return;
        };

        let initial_prompt = action.prompt.clone();

        if assistant_panel.update(cx, |assistant, cx| assistant.is_authenticated(cx)) {
            match inline_assist_target {
                InlineAssistTarget::Editor(active_editor, include_context) => {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_editor,
                            Some(cx.view().downgrade()),
                            include_context.then_some(&assistant_panel),
                            initial_prompt,
                            cx,
                        )
                    })
                }
                InlineAssistTarget::Terminal(active_terminal) => {
                    TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_terminal,
                            Some(cx.view().downgrade()),
                            Some(&assistant_panel),
                            initial_prompt,
                            cx,
                        )
                    })
                }
            }
        } else {
            let assistant_panel = assistant_panel.downgrade();
            cx.spawn(|workspace, mut cx| async move {
                let Some(task) =
                    assistant_panel.update(&mut cx, |assistant, cx| assistant.authenticate(cx))?
                else {
                    let answer = cx
                        .prompt(
                            gpui::PromptLevel::Warning,
                            "No language model provider configured",
                            None,
                            &["Configure", "Cancel"],
                        )
                        .await
                        .ok();
                    if let Some(answer) = answer {
                        if answer == 0 {
                            cx.update(|cx| cx.dispatch_action(Box::new(ShowConfiguration)))
                                .ok();
                        }
                    }
                    return Ok(());
                };
                task.await?;
                if assistant_panel.update(&mut cx, |panel, cx| panel.is_authenticated(cx))? {
                    cx.update(|cx| match inline_assist_target {
                        InlineAssistTarget::Editor(active_editor, include_context) => {
                            let assistant_panel = if include_context {
                                assistant_panel.upgrade()
                            } else {
                                None
                            };
                            InlineAssistant::update_global(cx, |assistant, cx| {
                                assistant.assist(
                                    &active_editor,
                                    Some(workspace),
                                    assistant_panel.as_ref(),
                                    initial_prompt,
                                    cx,
                                )
                            })
                        }
                        InlineAssistTarget::Terminal(active_terminal) => {
                            TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                                assistant.assist(
                                    &active_terminal,
                                    Some(workspace),
                                    assistant_panel.upgrade().as_ref(),
                                    initial_prompt,
                                    cx,
                                )
                            })
                        }
                    })?
                } else {
                    workspace.update(&mut cx, |workspace, cx| {
                        workspace.focus_panel::<AssistantPanel>(cx)
                    })?;
                }

                anyhow::Ok(())
            })
            .detach_and_log_err(cx)
        }
    }

    fn resolve_inline_assist_target(
        workspace: &mut Workspace,
        assistant_panel: &View<AssistantPanel>,
        cx: &mut WindowContext,
    ) -> Option<InlineAssistTarget> {
        if let Some(terminal_panel) = workspace.panel::<TerminalPanel>(cx) {
            if terminal_panel
                .read(cx)
                .focus_handle(cx)
                .contains_focused(cx)
            {
                if let Some(terminal_view) = terminal_panel.read(cx).pane().and_then(|pane| {
                    pane.read(cx)
                        .active_item()
                        .and_then(|t| t.downcast::<TerminalView>())
                }) {
                    return Some(InlineAssistTarget::Terminal(terminal_view));
                }
            }
        }
        let context_editor =
            assistant_panel
                .read(cx)
                .active_context_editor(cx)
                .and_then(|editor| {
                    let editor = &editor.read(cx).editor;
                    if editor.read(cx).is_focused(cx) {
                        Some(editor.clone())
                    } else {
                        None
                    }
                });

        if let Some(context_editor) = context_editor {
            Some(InlineAssistTarget::Editor(context_editor, false))
        } else if let Some(workspace_editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        {
            Some(InlineAssistTarget::Editor(workspace_editor, true))
        } else if let Some(terminal_view) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<TerminalView>(cx))
        {
            Some(InlineAssistTarget::Terminal(terminal_view))
        } else {
            None
        }
    }

    pub fn create_new_context(
        workspace: &mut Workspace,
        _: &NewContext,
        cx: &mut ViewContext<Workspace>,
    ) {
        if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
            let did_create_context = panel
                .update(cx, |panel, cx| {
                    panel.new_context(cx)?;

                    Some(())
                })
                .is_some();
            if did_create_context {
                ContextEditor::quote_selection(workspace, &Default::default(), cx);
            }
        }
    }

    pub fn new_context(&mut self, cx: &mut ViewContext<Self>) -> Option<View<ContextEditor>> {
        let project = self.project.read(cx);
        if project.is_via_collab() {
            let task = self
                .context_store
                .update(cx, |store, cx| store.create_remote_context(cx));

            cx.spawn(|this, mut cx| async move {
                let context = task.await?;

                this.update(&mut cx, |this, cx| {
                    let workspace = this.workspace.clone();
                    let project = this.project.clone();
                    let lsp_adapter_delegate =
                        make_lsp_adapter_delegate(&project, cx).log_err().flatten();

                    let fs = this.fs.clone();
                    let project = this.project.clone();
                    let weak_assistant_panel = cx.view().downgrade();

                    let editor = cx.new_view(|cx| {
                        ContextEditor::for_context(
                            context,
                            fs,
                            workspace,
                            project,
                            lsp_adapter_delegate,
                            weak_assistant_panel,
                            cx,
                        )
                    });

                    this.show_context(editor, cx);

                    anyhow::Ok(())
                })??;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

            None
        } else {
            let context = self.context_store.update(cx, |store, cx| store.create(cx));
            let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
                .log_err()
                .flatten();

            let assistant_panel = cx.view().downgrade();
            let editor = cx.new_view(|cx| {
                let mut editor = ContextEditor::for_context(
                    context,
                    self.fs.clone(),
                    self.workspace.clone(),
                    self.project.clone(),
                    lsp_adapter_delegate,
                    assistant_panel,
                    cx,
                );
                editor.insert_default_prompt(cx);
                editor
            });

            self.show_context(editor.clone(), cx);
            let workspace = self.workspace.clone();
            cx.spawn(move |_, mut cx| async move {
                workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.focus_panel::<AssistantPanel>(cx);
                    })
                    .ok();
            })
            .detach();
            Some(editor)
        }
    }

    fn show_context(&mut self, context_editor: View<ContextEditor>, cx: &mut ViewContext<Self>) {
        let focus = self.focus_handle(cx).contains_focused(cx);
        let prev_len = self.pane.read(cx).items_len();
        self.pane.update(cx, |pane, cx| {
            pane.add_item(Box::new(context_editor.clone()), focus, focus, None, cx)
        });

        if prev_len != self.pane.read(cx).items_len() {
            self.subscriptions
                .push(cx.subscribe(&context_editor, Self::handle_context_editor_event));
        }

        self.show_updated_summary(&context_editor, cx);

        cx.emit(AssistantPanelEvent::ContextEdited);
        cx.notify();
    }

    fn show_updated_summary(
        &self,
        context_editor: &View<ContextEditor>,
        cx: &mut ViewContext<Self>,
    ) {
        context_editor.update(cx, |context_editor, cx| {
            let new_summary = context_editor.title(cx).to_string();
            self.model_summary_editor.update(cx, |summary_editor, cx| {
                if summary_editor.text(cx) != new_summary {
                    summary_editor.set_text(new_summary, cx);
                }
            });
        });
    }

    fn handle_context_editor_event(
        &mut self,
        context_editor: View<ContextEditor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::TitleChanged => {
                self.show_updated_summary(&context_editor, cx);
                cx.notify()
            }
            EditorEvent::Edited { .. } => {
                self.workspace
                    .update(cx, |workspace, cx| {
                        let is_via_ssh = workspace
                            .project()
                            .update(cx, |project, _| project.is_via_ssh());

                        workspace
                            .client()
                            .telemetry()
                            .log_edit_event("assistant panel", is_via_ssh);
                    })
                    .log_err();
                cx.emit(AssistantPanelEvent::ContextEdited)
            }
            _ => {}
        }
    }

    fn show_configuration(
        workspace: &mut Workspace,
        _: &ShowConfiguration,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        if !panel.focus_handle(cx).contains_focused(cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(cx);
        }

        panel.update(cx, |this, cx| {
            this.show_configuration_tab(cx);
        })
    }

    fn show_configuration_tab(&mut self, cx: &mut ViewContext<Self>) {
        let configuration_item_ix = self
            .pane
            .read(cx)
            .items()
            .position(|item| item.downcast::<ConfigurationView>().is_some());

        if let Some(configuration_item_ix) = configuration_item_ix {
            self.pane.update(cx, |pane, cx| {
                pane.activate_item(configuration_item_ix, true, true, cx);
            });
        } else {
            let configuration = cx.new_view(ConfigurationView::new);
            self.configuration_subscription = Some(cx.subscribe(
                &configuration,
                |this, _, event: &ConfigurationViewEvent, cx| match event {
                    ConfigurationViewEvent::NewProviderContextEditor(provider) => {
                        if LanguageModelRegistry::read_global(cx)
                            .active_provider()
                            .map_or(true, |p| p.id() != provider.id())
                        {
                            if let Some(model) = provider.provided_models(cx).first().cloned() {
                                update_settings_file::<AssistantSettings>(
                                    this.fs.clone(),
                                    cx,
                                    move |settings, _| settings.set_model(model),
                                );
                            }
                        }

                        this.new_context(cx);
                    }
                },
            ));
            self.pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(configuration), true, true, None, cx);
            });
        }
    }

    fn deploy_history(&mut self, _: &DeployHistory, cx: &mut ViewContext<Self>) {
        let history_item_ix = self
            .pane
            .read(cx)
            .items()
            .position(|item| item.downcast::<ContextHistory>().is_some());

        if let Some(history_item_ix) = history_item_ix {
            self.pane.update(cx, |pane, cx| {
                pane.activate_item(history_item_ix, true, true, cx);
            });
        } else {
            let assistant_panel = cx.view().downgrade();
            let history = cx.new_view(|cx| {
                ContextHistory::new(
                    self.project.clone(),
                    self.context_store.clone(),
                    assistant_panel,
                    cx,
                )
            });
            self.pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(history), true, true, None, cx);
            });
        }
    }

    fn deploy_prompt_library(&mut self, _: &DeployPromptLibrary, cx: &mut ViewContext<Self>) {
        open_prompt_library(
            self.languages.clone(),
            Box::new(PromptLibraryInlineAssist),
            Arc::new(|| {
                Box::new(SlashCommandCompletionProvider::new(
                    Arc::new(SlashCommandWorkingSet::default()),
                    None,
                    None,
                ))
            }),
            cx,
        )
        .detach_and_log_err(cx);
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.model_selector_menu_handle.toggle(cx);
    }

    pub(crate) fn active_context_editor(&self, cx: &AppContext) -> Option<View<ContextEditor>> {
        self.pane
            .read(cx)
            .active_item()?
            .downcast::<ContextEditor>()
    }

    pub fn active_context(&self, cx: &AppContext) -> Option<Model<Context>> {
        Some(self.active_context_editor(cx)?.read(cx).context.clone())
    }

    pub fn open_saved_context(
        &mut self,
        path: PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let existing_context = self.pane.read(cx).items().find_map(|item| {
            item.downcast::<ContextEditor>()
                .filter(|editor| editor.read(cx).context.read(cx).path() == Some(&path))
        });
        if let Some(existing_context) = existing_context {
            return cx.spawn(|this, mut cx| async move {
                this.update(&mut cx, |this, cx| this.show_context(existing_context, cx))
            });
        }

        let context = self
            .context_store
            .update(cx, |store, cx| store.open_local_context(path.clone(), cx));
        let fs = self.fs.clone();
        let project = self.project.clone();
        let workspace = self.workspace.clone();

        let lsp_adapter_delegate = make_lsp_adapter_delegate(&project, cx).log_err().flatten();

        cx.spawn(|this, mut cx| async move {
            let context = context.await?;
            let assistant_panel = this.clone();
            this.update(&mut cx, |this, cx| {
                let editor = cx.new_view(|cx| {
                    ContextEditor::for_context(
                        context,
                        fs,
                        workspace,
                        project,
                        lsp_adapter_delegate,
                        assistant_panel,
                        cx,
                    )
                });
                this.show_context(editor, cx);
                anyhow::Ok(())
            })??;
            Ok(())
        })
    }

    pub fn open_remote_context(
        &mut self,
        id: ContextId,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<View<ContextEditor>>> {
        let existing_context = self.pane.read(cx).items().find_map(|item| {
            item.downcast::<ContextEditor>()
                .filter(|editor| *editor.read(cx).context.read(cx).id() == id)
        });
        if let Some(existing_context) = existing_context {
            return cx.spawn(|this, mut cx| async move {
                this.update(&mut cx, |this, cx| {
                    this.show_context(existing_context.clone(), cx)
                })?;
                Ok(existing_context)
            });
        }

        let context = self
            .context_store
            .update(cx, |store, cx| store.open_remote_context(id, cx));
        let fs = self.fs.clone();
        let workspace = self.workspace.clone();
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
            .log_err()
            .flatten();

        cx.spawn(|this, mut cx| async move {
            let context = context.await?;
            let assistant_panel = this.clone();
            this.update(&mut cx, |this, cx| {
                let editor = cx.new_view(|cx| {
                    ContextEditor::for_context(
                        context,
                        fs,
                        workspace,
                        this.project.clone(),
                        lsp_adapter_delegate,
                        assistant_panel,
                        cx,
                    )
                });
                this.show_context(editor.clone(), cx);
                anyhow::Ok(editor)
            })?
        })
    }

    fn is_authenticated(&mut self, cx: &mut ViewContext<Self>) -> bool {
        LanguageModelRegistry::read_global(cx)
            .active_provider()
            .map_or(false, |provider| provider.is_authenticated(cx))
    }

    fn authenticate(&mut self, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        LanguageModelRegistry::read_global(cx)
            .active_provider()
            .map_or(None, |provider| Some(provider.authenticate(cx)))
    }

    fn restart_context_servers(
        workspace: &mut Workspace,
        _action: &context_server::Restart,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(assistant_panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        assistant_panel.update(cx, |assistant_panel, cx| {
            assistant_panel
                .context_store
                .update(cx, |context_store, cx| {
                    context_store.restart_context_servers(cx);
                });
        });
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut registrar = DivRegistrar::new(
            |panel, cx| {
                panel
                    .pane
                    .read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<BufferSearchBar>()
            },
            cx,
        );
        BufferSearchBar::register(&mut registrar);
        let registrar = registrar.into_div();

        v_flex()
            .key_context("AssistantPanel")
            .size_full()
            .on_action(cx.listener(|this, _: &NewContext, cx| {
                this.new_context(cx);
            }))
            .on_action(
                cx.listener(|this, _: &ShowConfiguration, cx| this.show_configuration_tab(cx)),
            )
            .on_action(cx.listener(AssistantPanel::deploy_history))
            .on_action(cx.listener(AssistantPanel::deploy_prompt_library))
            .on_action(cx.listener(AssistantPanel::toggle_model_selector))
            .child(registrar.size_full().child(self.pane.clone()))
            .into_any_element()
    }
}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanel"
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        match AssistantSettings::get_global(cx).dock {
            AssistantDockPosition::Left => DockPosition::Left,
            AssistantDockPosition::Bottom => DockPosition::Bottom,
            AssistantDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<AssistantSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left => AssistantDockPosition::Left,
                    DockPosition::Bottom => AssistantDockPosition::Bottom,
                    DockPosition::Right => AssistantDockPosition::Right,
                };
                settings.set_dock(dock);
            },
        );
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        let settings = AssistantSettings::get_global(cx);
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        cx.notify();
    }

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active {
            if self.pane.read(cx).items_len() == 0 {
                self.new_context(cx);
            }

            self.ensure_authenticated(cx);
        }
    }

    fn pane(&self) -> Option<View<Pane>> {
        Some(self.pane.clone())
    }

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled || !settings.button {
            return None;
        }

        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        4
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}
impl EventEmitter<AssistantPanelEvent> for AssistantPanel {}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

struct PromptLibraryInlineAssist;

impl prompt_library::InlineAssistDelegate for PromptLibraryInlineAssist {
    fn assist(
        &self,
        prompt_editor: &View<Editor>,
        initial_prompt: Option<String>,
        cx: &mut ViewContext<PromptLibrary>,
    ) {
        InlineAssistant::update_global(cx, |assistant, cx| {
            assistant.assist(&prompt_editor, None, None, initial_prompt, cx)
        })
    }

    fn focus_assistant_panel(
        &self,
        workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> bool {
        workspace.focus_panel::<AssistantPanel>(cx).is_some()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WorkflowAssistStatus {
    Pending,
    Confirmed,
    Done,
    Idle,
}

pub struct ConfigurationView {
    focus_handle: FocusHandle,
    configuration_views: HashMap<LanguageModelProviderId, AnyView>,
    _registry_subscription: Subscription,
}

impl ConfigurationView {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let registry_subscription = cx.subscribe(
            &LanguageModelRegistry::global(cx),
            |this, _, event: &language_model::Event, cx| match event {
                language_model::Event::AddedProvider(provider_id) => {
                    let provider = LanguageModelRegistry::read_global(cx).provider(provider_id);
                    if let Some(provider) = provider {
                        this.add_configuration_view(&provider, cx);
                    }
                }
                language_model::Event::RemovedProvider(provider_id) => {
                    this.remove_configuration_view(provider_id);
                }
                _ => {}
            },
        );

        let mut this = Self {
            focus_handle,
            configuration_views: HashMap::default(),
            _registry_subscription: registry_subscription,
        };
        this.build_configuration_views(cx);
        this
    }

    fn build_configuration_views(&mut self, cx: &mut ViewContext<Self>) {
        let providers = LanguageModelRegistry::read_global(cx).providers();
        for provider in providers {
            self.add_configuration_view(&provider, cx);
        }
    }

    fn remove_configuration_view(&mut self, provider_id: &LanguageModelProviderId) {
        self.configuration_views.remove(provider_id);
    }

    fn add_configuration_view(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        cx: &mut ViewContext<Self>,
    ) {
        let configuration_view = provider.configuration_view(cx);
        self.configuration_views
            .insert(provider.id(), configuration_view);
    }

    fn render_provider_view(
        &mut self,
        provider: &Arc<dyn LanguageModelProvider>,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let provider_id = provider.id().0.clone();
        let provider_name = provider.name().0.clone();
        let configuration_view = self.configuration_views.get(&provider.id()).cloned();

        let open_new_context = cx.listener({
            let provider = provider.clone();
            move |_, _, cx| {
                cx.emit(ConfigurationViewEvent::NewProviderContextEditor(
                    provider.clone(),
                ))
            }
        });

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child(Headline::new(provider_name.clone()).size(HeadlineSize::Small))
                    .when(provider.is_authenticated(cx), move |this| {
                        this.child(
                            h_flex().justify_end().child(
                                Button::new(
                                    SharedString::from(format!("new-context-{provider_id}")),
                                    "Open New Chat",
                                )
                                .icon_position(IconPosition::Start)
                                .icon(IconName::Plus)
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ModalSurface)
                                .on_click(open_new_context),
                            ),
                        )
                    }),
            )
            .child(
                div()
                    .p(DynamicSpacing::Base08.rems(cx))
                    .bg(cx.theme().colors().surface_background)
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .rounded_md()
                    .when(configuration_view.is_none(), |this| {
                        this.child(div().child(Label::new(format!(
                            "No configuration view for {}",
                            provider_name
                        ))))
                    })
                    .when_some(configuration_view, |this, configuration_view| {
                        this.child(configuration_view)
                    }),
            )
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let providers = LanguageModelRegistry::read_global(cx).providers();
        let provider_views = providers
            .into_iter()
            .map(|provider| self.render_provider_view(&provider, cx))
            .collect::<Vec<_>>();

        let mut element = v_flex()
            .id("assistant-configuration-view")
            .track_focus(&self.focus_handle(cx))
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .overflow_y_scroll()
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .gap_1()
                    .child(Headline::new("Configure your Assistant").size(HeadlineSize::Medium))
                    .child(
                        Label::new(
                            "At least one LLM provider must be configured to use the Assistant.",
                        )
                        .color(Color::Muted),
                    ),
            )
            .child(
                v_flex()
                    .p(DynamicSpacing::Base16.rems(cx))
                    .mt_1()
                    .gap_6()
                    .flex_1()
                    .children(provider_views),
            )
            .into_any();

        // We use a canvas here to get scrolling to work in the ConfigurationView. It's a workaround
        // because we couldn't the element to take up the size of the parent.
        canvas(
            move |bounds, cx| {
                element.prepaint_as_root(bounds.origin, bounds.size.into(), cx);
                element
            },
            |_, mut element, cx| {
                element.paint(cx);
            },
        )
        .flex_1()
        .w_full()
    }
}

pub enum ConfigurationViewEvent {
    NewProviderContextEditor(Arc<dyn LanguageModelProvider>),
}

impl EventEmitter<ConfigurationViewEvent> for ConfigurationView {}

impl FocusableView for ConfigurationView {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ConfigurationView {
    type Event = ConfigurationViewEvent;

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("Configuration".into())
    }
}

fn make_lsp_adapter_delegate(
    project: &Model<Project>,
    cx: &mut AppContext,
) -> Result<Option<Arc<dyn LspAdapterDelegate>>> {
    project.update(cx, |project, cx| {
        // TODO: Find the right worktree.
        let Some(worktree) = project.worktrees(cx).next() else {
            return Ok(None::<Arc<dyn LspAdapterDelegate>>);
        };
        let http_client = project.client().http_client().clone();
        project.lsp_store().update(cx, |_, cx| {
            Ok(Some(LocalLspAdapterDelegate::new(
                project.languages().clone(),
                project.environment(),
                cx.weak_model(),
                &worktree,
                http_client,
                project.fs().clone(),
                cx,
            ) as Arc<dyn LspAdapterDelegate>))
        })
    })
}
