use crate::Assistant;
use crate::assistant_configuration::{ConfigurationView, ConfigurationViewEvent};
use crate::{
    DeployHistory, InlineAssistant, NewChat, terminal_inline_assistant::TerminalInlineAssistant,
};
use anyhow::{Result, anyhow};
use assistant_context_editor::{
    AssistantContext, AssistantPanelDelegate, ContextEditor, ContextEditorToolbarItem,
    ContextEditorToolbarItemEvent, ContextHistory, ContextId, ContextStore, ContextStoreEvent,
    DEFAULT_TAB_TITLE, InsertDraggedFiles, SlashCommandCompletionProvider,
    make_lsp_adapter_delegate,
};
use assistant_settings::{AssistantDockPosition, AssistantSettings};
use assistant_slash_command::SlashCommandWorkingSet;
use client::{Client, Status, proto};
use editor::{Anchor, AnchorRangeExt as _, Editor, EditorEvent, MultiBuffer};
use fs::Fs;
use gpui::{
    Action, App, AsyncWindowContext, Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement, Pixels, Render, Styled, Subscription, Task,
    UpdateGlobal, WeakEntity, prelude::*,
};
use language::LanguageRegistry;
use language_model::{
    AuthenticateError, ConfiguredModel, LanguageModelProviderId, LanguageModelRegistry,
};
use project::Project;
use prompt_library::{PromptLibrary, open_prompt_library};
use prompt_store::{PromptBuilder, UserPromptId};

use search::{BufferSearchBar, buffer_search::DivRegistrar};
use settings::{Settings, update_settings_file};
use smol::stream::StreamExt;

use std::ops::Range;
use std::{ops::ControlFlow, path::PathBuf, sync::Arc};
use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use ui::{ContextMenu, PopoverMenu, Tooltip, prelude::*};
use util::{ResultExt, maybe};
use workspace::DraggedTab;
use workspace::{
    DraggedSelection, Pane, ToggleZoom, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    pane,
};
use zed_actions::assistant::{InlineAssist, OpenPromptLibrary, ShowConfiguration, ToggleFocus};

pub fn init(cx: &mut App) {
    workspace::FollowableViewRegistry::register::<ContextEditor>(cx);
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(ContextEditor::quote_selection)
                .register_action(ContextEditor::insert_selection)
                .register_action(ContextEditor::copy_code)
                .register_action(ContextEditor::insert_dragged_files)
                .register_action(AssistantPanel::show_configuration)
                .register_action(AssistantPanel::create_new_context)
                .register_action(AssistantPanel::restart_context_servers)
                .register_action(|workspace, action: &OpenPromptLibrary, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.deploy_prompt_library(action, window, cx)
                        });
                    }
                });
        },
    )
    .detach();

    cx.observe_new(
        |terminal_panel: &mut TerminalPanel, _, cx: &mut Context<TerminalPanel>| {
            terminal_panel.set_assistant_enabled(Assistant::enabled(cx), cx);
        },
    )
    .detach();
}

pub enum AssistantPanelEvent {
    ContextEdited,
}

pub struct AssistantPanel {
    pane: Entity<Pane>,
    workspace: WeakEntity<Workspace>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    project: Entity<Project>,
    context_store: Entity<ContextStore>,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    subscriptions: Vec<Subscription>,
    model_summary_editor: Entity<Editor>,
    authenticate_provider_task: Option<(LanguageModelProviderId, Task<()>)>,
    configuration_subscription: Option<Subscription>,
    client_status: Option<client::Status>,
    watch_client_status: Option<Task<()>>,
    pub(crate) show_zed_ai_notice: bool,
}

enum InlineAssistTarget {
    Editor(Entity<Editor>, bool),
    Terminal(Entity<TerminalView>),
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            let slash_commands = Arc::new(SlashCommandWorkingSet::default());
            let context_store = workspace
                .update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    ContextStore::new(project, prompt_builder.clone(), slash_commands, cx)
                })?
                .await?;

            workspace.update_in(cx, |workspace, window, cx| {
                // TODO: deserialize state.
                cx.new(|cx| Self::new(workspace, context_store, window, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        context_store: Entity<ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let model_summary_editor = cx.new(|cx| Editor::single_line(window, cx));
        let context_editor_toolbar =
            cx.new(|_| ContextEditorToolbarItem::new(model_summary_editor.clone()));

        let pane = cx.new(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.project().clone(),
                Default::default(),
                None,
                NewChat.boxed_clone(),
                window,
                cx,
            );

            let project = workspace.project().clone();
            pane.set_custom_drop_handle(cx, move |_, dropped_item, window, cx| {
                let action = maybe!({
                    if project.read(cx).is_local() {
                        if let Some(paths) = dropped_item.downcast_ref::<ExternalPaths>() {
                            return Some(InsertDraggedFiles::ExternalFiles(paths.paths().to_vec()));
                        }
                    }

                    let project_paths = if let Some(tab) = dropped_item.downcast_ref::<DraggedTab>()
                    {
                        if tab.pane == cx.entity() {
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
                    window.dispatch_action(action.boxed_clone(), cx);
                }

                ControlFlow::Break(())
            });

            pane.set_can_navigate(true, cx);
            pane.display_nav_history_buttons(None);
            pane.set_should_display_tab_bar(|_, _| true);
            pane.set_render_tab_bar_buttons(cx, move |pane, _window, cx| {
                let focus_handle = pane.focus_handle(cx);
                let left_children = IconButton::new("history", IconName::HistoryRerun)
                    .icon_size(IconSize::Small)
                    .on_click(cx.listener({
                        let focus_handle = focus_handle.clone();
                        move |_, _, window, cx| {
                            focus_handle.focus(window);
                            window.dispatch_action(DeployHistory.boxed_clone(), cx)
                        }
                    }))
                    .tooltip({
                        let focus_handle = focus_handle.clone();
                        move |window, cx| {
                            Tooltip::for_action_in(
                                "Open History",
                                &DeployHistory,
                                &focus_handle,
                                window,
                                cx,
                            )
                        }
                    })
                    .toggle_state(
                        pane.active_item()
                            .map_or(false, |item| item.downcast::<ContextHistory>().is_some()),
                    );
                let _pane = cx.entity().clone();
                let right_children = h_flex()
                    .gap(DynamicSpacing::Base02.rems(cx))
                    .child(
                        IconButton::new("new-chat", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.dispatch_action(NewChat.boxed_clone(), cx)
                            }))
                            .tooltip(move |window, cx| {
                                Tooltip::for_action_in(
                                    "New Chat",
                                    &NewChat,
                                    &focus_handle,
                                    window,
                                    cx,
                                )
                            }),
                    )
                    .child(
                        PopoverMenu::new("assistant-panel-popover-menu")
                            .trigger_with_tooltip(
                                IconButton::new("menu", IconName::EllipsisVertical)
                                    .icon_size(IconSize::Small),
                                Tooltip::text("Toggle Assistant Menu"),
                            )
                            .menu(move |window, cx| {
                                let zoom_label = if _pane.read(cx).is_zoomed() {
                                    "Zoom Out"
                                } else {
                                    "Zoom In"
                                };
                                let focus_handle = _pane.focus_handle(cx);
                                Some(ContextMenu::build(window, cx, move |menu, _, _| {
                                    menu.context(focus_handle.clone())
                                        .action("New Chat", Box::new(NewChat))
                                        .action("History", Box::new(DeployHistory))
                                        .action(
                                            "Prompt Library",
                                            Box::new(OpenPromptLibrary::default()),
                                        )
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
                toolbar.add_item(context_editor_toolbar.clone(), window, cx);
                toolbar.add_item(
                    cx.new(|cx| {
                        BufferSearchBar::new(
                            Some(workspace.project().read(cx).languages().clone()),
                            window,
                            cx,
                        )
                    }),
                    window,
                    cx,
                )
            });
            pane
        });

        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe_in(&pane, window, Self::handle_pane_event),
            cx.subscribe(&context_editor_toolbar, Self::handle_toolbar_event),
            cx.subscribe(&model_summary_editor, Self::handle_summary_editor_event),
            cx.subscribe_in(&context_store, window, Self::handle_context_store_event),
            cx.subscribe_in(
                &LanguageModelRegistry::global(cx),
                window,
                |this, _, event: &language_model::Event, window, cx| match event {
                    language_model::Event::DefaultModelChanged
                    | language_model::Event::InlineAssistantModelChanged
                    | language_model::Event::CommitMessageModelChanged
                    | language_model::Event::ThreadSummaryModelChanged => {
                        this.completion_provider_changed(window, cx);
                    }
                    language_model::Event::ProviderStateChanged => {
                        this.ensure_authenticated(window, cx);
                        cx.notify()
                    }
                    language_model::Event::AddedProvider(_)
                    | language_model::Event::RemovedProvider(_) => {
                        this.ensure_authenticated(window, cx);
                    }
                },
            ),
        ];

        let watch_client_status = Self::watch_client_status(workspace.client().clone(), window, cx);

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
            model_summary_editor,
            authenticate_provider_task: None,
            configuration_subscription: None,
            client_status: None,
            watch_client_status: Some(watch_client_status),
            show_zed_ai_notice: false,
        };
        this.new_context(window, cx);
        this
    }

    pub fn toggle_focus(
        workspace: &mut Workspace,
        _: &ToggleFocus,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if workspace
            .panel::<Self>(cx)
            .is_some_and(|panel| panel.read(cx).enabled(cx))
        {
            workspace.toggle_panel_focus::<Self>(window, cx);
        }
    }

    fn watch_client_status(
        client: Arc<Client>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let mut status_rx = client.status();

        cx.spawn_in(window, async move |this, cx| {
            while let Some(status) = status_rx.next().await {
                this.update(cx, |this, cx| {
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
            this.update(cx, |this, _cx| this.watch_client_status = None)
                .log_err();
        })
    }

    fn handle_pane_event(
        &mut self,
        pane: &Entity<Pane>,
        event: &pane::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
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
                        item.added_to_pane(workspace, self.pane.clone(), window, cx)
                    })
                    .ok();
                true
            }

            pane::Event::ActivateItem { local, .. } => {
                if *local {
                    self.workspace
                        .update(cx, |workspace, cx| {
                            workspace.unfollow_in_pane(&pane, window, cx);
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
                self.show_updated_summary(&editor, window, cx)
            }
        }
    }

    fn handle_summary_editor_event(
        &mut self,
        model_summary_editor: Entity<Editor>,
        event: &EditorEvent,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, EditorEvent::Edited { .. }) {
            if let Some(context_editor) = self.active_context_editor(cx) {
                let new_summary = model_summary_editor.read(cx).text(cx);
                context_editor.update(cx, |context_editor, cx| {
                    context_editor.context().update(cx, |context, cx| {
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

    fn update_zed_ai_notice_visibility(&mut self, client_status: Status, cx: &mut Context<Self>) {
        let model = LanguageModelRegistry::read_global(cx).default_model();

        // If we're signed out and don't have a provider configured, or we're signed-out AND Zed.dev is
        // the provider, we want to show a nudge to sign in.
        let show_zed_ai_notice =
            client_status.is_signed_out() && model.map_or(true, |model| model.is_provided_by_zed());

        self.show_zed_ai_notice = show_zed_ai_notice;
        cx.notify();
    }

    fn handle_toolbar_event(
        &mut self,
        _: Entity<ContextEditorToolbarItem>,
        _: &ContextEditorToolbarItemEvent,
        cx: &mut Context<Self>,
    ) {
        if let Some(context_editor) = self.active_context_editor(cx) {
            context_editor.update(cx, |context_editor, cx| {
                context_editor.context().update(cx, |context, cx| {
                    context.summarize(true, cx);
                })
            })
        }
    }

    fn handle_context_store_event(
        &mut self,
        _context_store: &Entity<ContextStore>,
        event: &ContextStoreEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
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

        let editor = cx.new(|cx| {
            let mut editor = ContextEditor::for_context(
                context,
                self.fs.clone(),
                self.workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                window,
                cx,
            );
            editor.insert_default_prompt(window, cx);
            editor
        });

        self.show_context(editor.clone(), window, cx);
    }

    fn completion_provider_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(editor) = self.active_context_editor(cx) {
            editor.update(cx, |active_context, cx| {
                active_context
                    .context()
                    .update(cx, |context, cx| context.completion_provider_changed(cx))
            })
        }

        let Some(new_provider_id) = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.provider.id())
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
            self.ensure_authenticated(window, cx);
        }

        if let Some(status) = self.client_status {
            self.update_zed_ai_notice_visibility(status, cx);
        }
    }

    fn ensure_authenticated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_authenticated(cx) {
            return;
        }

        let Some(ConfiguredModel { provider, .. }) =
            LanguageModelRegistry::read_global(cx).default_model()
        else {
            return;
        };

        let load_credentials = self.authenticate(cx);

        if self.authenticate_provider_task.is_none() {
            self.authenticate_provider_task = Some((
                provider.id(),
                cx.spawn_in(window, async move |this, cx| {
                    if let Some(future) = load_credentials {
                        let _ = future.await;
                    }
                    this.update(cx, |this, _cx| {
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
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(assistant_panel) = workspace
            .panel::<AssistantPanel>(cx)
            .filter(|panel| panel.read(cx).enabled(cx))
        else {
            return;
        };

        let Some(inline_assist_target) =
            Self::resolve_inline_assist_target(workspace, &assistant_panel, window, cx)
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
                            Some(cx.entity().downgrade()),
                            include_context.then_some(&assistant_panel),
                            initial_prompt,
                            window,
                            cx,
                        )
                    })
                }
                InlineAssistTarget::Terminal(active_terminal) => {
                    TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_terminal,
                            Some(cx.entity().downgrade()),
                            Some(&assistant_panel),
                            initial_prompt,
                            window,
                            cx,
                        )
                    })
                }
            }
        } else {
            let assistant_panel = assistant_panel.downgrade();
            cx.spawn_in(window, async move |workspace, cx| {
                let Some(task) =
                    assistant_panel.update(cx, |assistant, cx| assistant.authenticate(cx))?
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
                            cx.update(|window, cx| {
                                window.dispatch_action(Box::new(ShowConfiguration), cx)
                            })
                            .ok();
                        }
                    }
                    return Ok(());
                };
                task.await?;
                if assistant_panel.update(cx, |panel, cx| panel.is_authenticated(cx))? {
                    cx.update(|window, cx| match inline_assist_target {
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
                                    window,
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
                                    window,
                                    cx,
                                )
                            })
                        }
                    })?
                } else {
                    workspace.update_in(cx, |workspace, window, cx| {
                        workspace.focus_panel::<AssistantPanel>(window, cx)
                    })?;
                }

                anyhow::Ok(())
            })
            .detach_and_log_err(cx)
        }
    }

    fn resolve_inline_assist_target(
        workspace: &mut Workspace,
        assistant_panel: &Entity<AssistantPanel>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InlineAssistTarget> {
        if let Some(terminal_panel) = workspace.panel::<TerminalPanel>(cx) {
            if terminal_panel
                .read(cx)
                .focus_handle(cx)
                .contains_focused(window, cx)
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
                    let editor = &editor.read(cx).editor().clone();
                    if editor.read(cx).is_focused(window) {
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
        _: &NewChat,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
            let did_create_context = panel
                .update(cx, |panel, cx| {
                    panel.new_context(window, cx)?;

                    Some(())
                })
                .is_some();
            if did_create_context {
                ContextEditor::quote_selection(workspace, &Default::default(), window, cx);
            }
        }
    }

    pub fn new_context(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ContextEditor>> {
        let project = self.project.read(cx);
        if project.is_via_collab() {
            let task = self
                .context_store
                .update(cx, |store, cx| store.create_remote_context(cx));

            cx.spawn_in(window, async move |this, cx| {
                let context = task.await?;

                this.update_in(cx, |this, window, cx| {
                    let workspace = this.workspace.clone();
                    let project = this.project.clone();
                    let lsp_adapter_delegate =
                        make_lsp_adapter_delegate(&project, cx).log_err().flatten();

                    let fs = this.fs.clone();
                    let project = this.project.clone();

                    let editor = cx.new(|cx| {
                        ContextEditor::for_context(
                            context,
                            fs,
                            workspace,
                            project,
                            lsp_adapter_delegate,
                            window,
                            cx,
                        )
                    });

                    this.show_context(editor, window, cx);

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

            let editor = cx.new(|cx| {
                let mut editor = ContextEditor::for_context(
                    context,
                    self.fs.clone(),
                    self.workspace.clone(),
                    self.project.clone(),
                    lsp_adapter_delegate,
                    window,
                    cx,
                );
                editor.insert_default_prompt(window, cx);
                editor
            });

            self.show_context(editor.clone(), window, cx);
            let workspace = self.workspace.clone();
            cx.spawn_in(window, async move |_, cx| {
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                    })
                    .ok();
            })
            .detach();
            Some(editor)
        }
    }

    fn show_context(
        &mut self,
        context_editor: Entity<ContextEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let focus = self.focus_handle(cx).contains_focused(window, cx);
        let prev_len = self.pane.read(cx).items_len();
        self.pane.update(cx, |pane, cx| {
            pane.add_item(
                Box::new(context_editor.clone()),
                focus,
                focus,
                None,
                window,
                cx,
            )
        });

        if prev_len != self.pane.read(cx).items_len() {
            self.subscriptions.push(cx.subscribe_in(
                &context_editor,
                window,
                Self::handle_context_editor_event,
            ));
        }

        self.show_updated_summary(&context_editor, window, cx);

        cx.emit(AssistantPanelEvent::ContextEdited);
        cx.notify();
    }

    fn show_updated_summary(
        &self,
        context_editor: &Entity<ContextEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        context_editor.update(cx, |context_editor, cx| {
            let new_summary = context_editor.title(cx).to_string();
            self.model_summary_editor.update(cx, |summary_editor, cx| {
                if summary_editor.text(cx) != new_summary {
                    summary_editor.set_text(new_summary, window, cx);
                }
            });
        });
    }

    fn handle_context_editor_event(
        &mut self,
        context_editor: &Entity<ContextEditor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::TitleChanged => {
                self.show_updated_summary(&context_editor, window, cx);
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
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        if !panel.focus_handle(cx).contains_focused(window, cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(window, cx);
        }

        panel.update(cx, |this, cx| {
            this.show_configuration_tab(window, cx);
        })
    }

    fn show_configuration_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let configuration_item_ix = self
            .pane
            .read(cx)
            .items()
            .position(|item| item.downcast::<ConfigurationView>().is_some());

        if let Some(configuration_item_ix) = configuration_item_ix {
            self.pane.update(cx, |pane, cx| {
                pane.activate_item(configuration_item_ix, true, true, window, cx);
            });
        } else {
            let configuration = cx.new(|cx| ConfigurationView::new(window, cx));
            self.configuration_subscription = Some(cx.subscribe_in(
                &configuration,
                window,
                |this, _, event: &ConfigurationViewEvent, window, cx| match event {
                    ConfigurationViewEvent::NewProviderContextEditor(provider) => {
                        if LanguageModelRegistry::read_global(cx)
                            .default_model()
                            .map_or(true, |default| default.provider.id() != provider.id())
                        {
                            if let Some(model) = provider.default_model(cx) {
                                update_settings_file::<AssistantSettings>(
                                    this.fs.clone(),
                                    cx,
                                    move |settings, _| settings.set_model(model),
                                );
                            }
                        }

                        this.new_context(window, cx);
                    }
                },
            ));
            self.pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(configuration), true, true, None, window, cx);
            });
        }
    }

    fn deploy_history(&mut self, _: &DeployHistory, window: &mut Window, cx: &mut Context<Self>) {
        let history_item_ix = self
            .pane
            .read(cx)
            .items()
            .position(|item| item.downcast::<ContextHistory>().is_some());

        if let Some(history_item_ix) = history_item_ix {
            self.pane.update(cx, |pane, cx| {
                pane.activate_item(history_item_ix, true, true, window, cx);
            });
        } else {
            let history = cx.new(|cx| {
                ContextHistory::new(
                    self.project.clone(),
                    self.context_store.clone(),
                    self.workspace.clone(),
                    window,
                    cx,
                )
            });
            self.pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(history), true, true, None, window, cx);
            });
        }
    }

    fn deploy_prompt_library(
        &mut self,
        action: &OpenPromptLibrary,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
            action
                .prompt_to_select
                .map(|uuid| UserPromptId(uuid).into()),
            cx,
        )
        .detach_and_log_err(cx);
    }

    pub(crate) fn active_context_editor(&self, cx: &App) -> Option<Entity<ContextEditor>> {
        self.pane
            .read(cx)
            .active_item()?
            .downcast::<ContextEditor>()
    }

    pub fn active_context(&self, cx: &App) -> Option<Entity<AssistantContext>> {
        Some(self.active_context_editor(cx)?.read(cx).context().clone())
    }

    pub fn open_saved_context(
        &mut self,
        path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let existing_context = self.pane.read(cx).items().find_map(|item| {
            item.downcast::<ContextEditor>()
                .filter(|editor| editor.read(cx).context().read(cx).path() == Some(&path))
        });
        if let Some(existing_context) = existing_context {
            return cx.spawn_in(window, async move |this, cx| {
                this.update_in(cx, |this, window, cx| {
                    this.show_context(existing_context, window, cx)
                })
            });
        }

        let context = self
            .context_store
            .update(cx, |store, cx| store.open_local_context(path.clone(), cx));
        let fs = self.fs.clone();
        let project = self.project.clone();
        let workspace = self.workspace.clone();

        let lsp_adapter_delegate = make_lsp_adapter_delegate(&project, cx).log_err().flatten();

        cx.spawn_in(window, async move |this, cx| {
            let context = context.await?;
            this.update_in(cx, |this, window, cx| {
                let editor = cx.new(|cx| {
                    ContextEditor::for_context(
                        context,
                        fs,
                        workspace,
                        project,
                        lsp_adapter_delegate,
                        window,
                        cx,
                    )
                });
                this.show_context(editor, window, cx);
                anyhow::Ok(())
            })??;
            Ok(())
        })
    }

    pub fn open_remote_context(
        &mut self,
        id: ContextId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<ContextEditor>>> {
        let existing_context = self.pane.read(cx).items().find_map(|item| {
            item.downcast::<ContextEditor>()
                .filter(|editor| *editor.read(cx).context().read(cx).id() == id)
        });
        if let Some(existing_context) = existing_context {
            return cx.spawn_in(window, async move |this, cx| {
                this.update_in(cx, |this, window, cx| {
                    this.show_context(existing_context.clone(), window, cx)
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

        cx.spawn_in(window, async move |this, cx| {
            let context = context.await?;
            this.update_in(cx, |this, window, cx| {
                let editor = cx.new(|cx| {
                    ContextEditor::for_context(
                        context,
                        fs,
                        workspace,
                        this.project.clone(),
                        lsp_adapter_delegate,
                        window,
                        cx,
                    )
                });
                this.show_context(editor.clone(), window, cx);
                anyhow::Ok(editor)
            })?
        })
    }

    fn is_authenticated(&mut self, cx: &mut Context<Self>) -> bool {
        LanguageModelRegistry::read_global(cx)
            .default_model()
            .map_or(false, |default| default.provider.is_authenticated(cx))
    }

    fn authenticate(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<(), AuthenticateError>>> {
        LanguageModelRegistry::read_global(cx)
            .default_model()
            .map_or(None, |default| Some(default.provider.authenticate(cx)))
    }

    fn restart_context_servers(
        workspace: &mut Workspace,
        _action: &context_server::Restart,
        _: &mut Window,
        cx: &mut Context<Workspace>,
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut registrar = DivRegistrar::new(
            |panel, _, cx| {
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
            .on_action(cx.listener(|this, _: &NewChat, window, cx| {
                this.new_context(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ShowConfiguration, window, cx| {
                this.show_configuration_tab(window, cx)
            }))
            .on_action(cx.listener(AssistantPanel::deploy_history))
            .on_action(cx.listener(AssistantPanel::deploy_prompt_library))
            .child(registrar.size_full().child(self.pane.clone()))
            .into_any_element()
    }
}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanel"
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        match AssistantSettings::get_global(cx).dock {
            AssistantDockPosition::Left => DockPosition::Left,
            AssistantDockPosition::Bottom => DockPosition::Bottom,
            AssistantDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
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

    fn size(&self, window: &Window, cx: &App) -> Pixels {
        let settings = AssistantSettings::get_global(cx);
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        cx.notify();
    }

    fn is_zoomed(&self, _: &Window, cx: &App) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, _: &mut Window, cx: &mut Context<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {
        if active {
            if self.pane.read(cx).items_len() == 0 {
                self.new_context(window, cx);
            }

            self.ensure_authenticated(window, cx);
        }
    }

    fn pane(&self) -> Option<Entity<Pane>> {
        Some(self.pane.clone())
    }

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        (self.enabled(cx) && AssistantSettings::get_global(cx).button)
            .then_some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        4
    }

    fn enabled(&self, cx: &App) -> bool {
        Assistant::enabled(cx)
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}
impl EventEmitter<AssistantPanelEvent> for AssistantPanel {}

impl Focusable for AssistantPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

struct PromptLibraryInlineAssist;

impl prompt_library::InlineAssistDelegate for PromptLibraryInlineAssist {
    fn assist(
        &self,
        prompt_editor: &Entity<Editor>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut Context<PromptLibrary>,
    ) {
        InlineAssistant::update_global(cx, |assistant, cx| {
            assistant.assist(&prompt_editor, None, None, initial_prompt, window, cx)
        })
    }

    fn focus_assistant_panel(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        workspace
            .focus_panel::<AssistantPanel>(window, cx)
            .is_some()
    }
}

pub struct ConcreteAssistantPanelDelegate;

impl AssistantPanelDelegate for ConcreteAssistantPanelDelegate {
    fn active_context_editor(
        &self,
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<ContextEditor>> {
        let panel = workspace.panel::<AssistantPanel>(cx)?;
        panel.read(cx).active_context_editor(cx)
    }

    fn open_saved_context(
        &self,
        workspace: &mut Workspace,
        path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>> {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return Task::ready(Err(anyhow!("no Assistant panel found")));
        };

        panel.update(cx, |panel, cx| panel.open_saved_context(path, window, cx))
    }

    fn open_remote_context(
        &self,
        workspace: &mut Workspace,
        context_id: ContextId,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<Entity<ContextEditor>>> {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return Task::ready(Err(anyhow!("no Assistant panel found")));
        };

        panel.update(cx, |panel, cx| {
            panel.open_remote_context(context_id, window, cx)
        })
    }

    fn quote_selection(
        &self,
        workspace: &mut Workspace,
        selection_ranges: Vec<Range<Anchor>>,
        buffer: Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        if !panel.focus_handle(cx).contains_focused(window, cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(window, cx);
        }

        let snapshot = buffer.read(cx).snapshot(cx);
        let selection_ranges = selection_ranges
            .into_iter()
            .map(|range| range.to_point(&snapshot))
            .collect::<Vec<_>>();

        panel.update(cx, |_, cx| {
            // Wait to create a new context until the workspace is no longer
            // being updated.
            cx.defer_in(window, move |panel, window, cx| {
                if let Some(context) = panel
                    .active_context_editor(cx)
                    .or_else(|| panel.new_context(window, cx))
                {
                    context.update(cx, |context, cx| {
                        context.quote_ranges(selection_ranges, snapshot, window, cx)
                    });
                };
            });
        });
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WorkflowAssistStatus {
    Pending,
    Confirmed,
    Done,
    Idle,
}
