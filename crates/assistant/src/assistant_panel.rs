use crate::{
    assistant_settings::{AssistantDockPosition, AssistantSettings},
    humanize_token_count,
    prompt_library::open_prompt_library,
    prompts::PromptBuilder,
    slash_command::{
        default_command::DefaultSlashCommand,
        docs_command::{DocsSlashCommand, DocsSlashCommandArgs},
        file_command::codeblock_fence_for_path,
        SlashCommandCompletionProvider, SlashCommandRegistry,
    },
    slash_command_picker,
    terminal_inline_assistant::TerminalInlineAssistant,
    Assist, CacheStatus, ConfirmCommand, Context, ContextEvent, ContextId, ContextStore,
    ContextStoreEvent, CycleMessageRole, DeployHistory, DeployPromptLibrary, InlineAssistId,
    InlineAssistant, InsertIntoEditor, Message, MessageId, MessageMetadata, MessageStatus,
    ModelPickerDelegate, ModelSelector, NewContext, PendingSlashCommand, PendingSlashCommandStatus,
    QuoteSelection, RemoteContextMetadata, SavedContextMetadata, Split, ToggleFocus,
    ToggleModelSelector, WorkflowStepResolution,
};
use anyhow::{anyhow, Result};
use assistant_slash_command::{SlashCommand, SlashCommandOutputSection};
use client::{proto, Client, Status};
use collections::{BTreeSet, HashMap, HashSet};
use editor::{
    actions::{FoldAt, MoveToEndOfLine, Newline, ShowCompletions, UnfoldAt},
    display_map::{
        BlockDisposition, BlockId, BlockProperties, BlockStyle, Crease, CustomBlockId, FoldId,
        RenderBlock, ToDisplayPoint,
    },
    scroll::{Autoscroll, AutoscrollStrategy, ScrollAnchor},
    Anchor, Editor, EditorEvent, ExcerptRange, MultiBuffer, RowExt, ToOffset as _, ToPoint,
};
use editor::{display_map::CreaseId, FoldPlaceholder};
use fs::Fs;
use gpui::{
    canvas, div, img, percentage, point, pulsating_between, size, Action, Animation, AnimationExt,
    AnyElement, AnyView, AppContext, AsyncWindowContext, ClipboardEntry, ClipboardItem,
    Context as _, Empty, Entity, EntityId, EventEmitter, FocusHandle, FocusableView, FontWeight,
    InteractiveElement, IntoElement, Model, ParentElement, Pixels, ReadGlobal, Render, RenderImage,
    SharedString, Size, StatefulInteractiveElement, Styled, Subscription, Task, Transformation,
    UpdateGlobal, View, VisualContext, WeakView, WindowContext,
};
use indexed_docs::IndexedDocsStore;
use language::{
    language_settings::SoftWrap, Capability, LanguageRegistry, LspAdapterDelegate, Point, ToOffset,
};
use language_model::{
    provider::cloud::PROVIDER_ID, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelRegistry, Role,
};
use multi_buffer::MultiBufferRow;
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectLspAdapterDelegate};
use search::{buffer_search::DivRegistrar, BufferSearchBar};
use settings::{update_settings_file, Settings};
use smol::stream::StreamExt;
use std::{
    borrow::Cow, cmp, collections::hash_map, fmt::Write, ops::Range, path::PathBuf, sync::Arc,
    time::Duration,
};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use ui::TintColor;
use ui::{
    prelude::*,
    utils::{format_distance_from_now, DateTimeType},
    Avatar, AvatarShape, ButtonLike, ContextMenu, Disclosure, ElevationIndex, KeyBinding, ListItem,
    ListItemSpacing, PopoverMenu, PopoverMenuHandle, Tooltip,
};
use util::ResultExt;
use workspace::searchable::SearchableItemHandle;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::{self, FollowableItem, Item, ItemHandle},
    pane::{self, SaveIntent},
    searchable::{SearchEvent, SearchableItem},
    Pane, Save, ShowConfiguration, ToggleZoom, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace,
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
                .register_action(AssistantPanel::inline_assist)
                .register_action(ContextEditor::quote_selection)
                .register_action(ContextEditor::insert_selection)
                .register_action(AssistantPanel::show_configuration)
                .register_action(AssistantPanel::create_new_context);
        },
    )
    .detach();

    cx.observe_new_views(
        |terminal_panel: &mut TerminalPanel, cx: &mut ViewContext<TerminalPanel>| {
            let settings = AssistantSettings::get_global(cx);
            terminal_panel.asssistant_enabled(settings.enabled, cx);
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
    model_selector_menu_handle: PopoverMenuHandle<Picker<ModelPickerDelegate>>,
    model_summary_editor: View<Editor>,
    authenticate_provider_task: Option<(LanguageModelProviderId, Task<()>)>,
    configuration_subscription: Option<Subscription>,
    client_status: Option<client::Status>,
    watch_client_status: Option<Task<()>>,
    show_zed_ai_notice: bool,
}

#[derive(Clone)]
enum ContextMetadata {
    Remote(RemoteContextMetadata),
    Saved(SavedContextMetadata),
}

struct SavedContextPickerDelegate {
    store: Model<ContextStore>,
    project: Model<Project>,
    matches: Vec<ContextMetadata>,
    selected_index: usize,
}

enum SavedContextPickerEvent {
    Confirmed(ContextMetadata),
}

enum InlineAssistTarget {
    Editor(View<Editor>, bool),
    Terminal(View<TerminalView>),
}

impl EventEmitter<SavedContextPickerEvent> for Picker<SavedContextPickerDelegate> {}

impl SavedContextPickerDelegate {
    fn new(project: Model<Project>, store: Model<ContextStore>) -> Self {
        Self {
            project,
            store,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for SavedContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let search = self.store.read(cx).search(query, cx);
        cx.spawn(|this, mut cx| async move {
            let matches = search.await;
            this.update(&mut cx, |this, cx| {
                let host_contexts = this.delegate.store.read(cx).host_contexts();
                this.delegate.matches = host_contexts
                    .iter()
                    .cloned()
                    .map(ContextMetadata::Remote)
                    .chain(matches.into_iter().map(ContextMetadata::Saved))
                    .collect();
                this.delegate.selected_index = 0;
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(metadata) = self.matches.get(self.selected_index) {
            cx.emit(SavedContextPickerEvent::Confirmed(metadata.clone()));
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let context = self.matches.get(ix)?;
        let item = match context {
            ContextMetadata::Remote(context) => {
                let host_user = self.project.read(cx).host().and_then(|collaborator| {
                    self.project
                        .read(cx)
                        .user_store()
                        .read(cx)
                        .get_cached_user(collaborator.user_id)
                });
                div()
                    .flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(
                        h_flex().flex_1().overflow_x_hidden().child(
                            Label::new(context.summary.clone().unwrap_or(DEFAULT_TAB_TITLE.into()))
                                .size(LabelSize::Small),
                        ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .children(if let Some(host_user) = host_user {
                                vec![
                                    Avatar::new(host_user.avatar_uri.clone())
                                        .shape(AvatarShape::Circle)
                                        .into_any_element(),
                                    Label::new(format!("Shared by @{}", host_user.github_login))
                                        .color(Color::Muted)
                                        .size(LabelSize::Small)
                                        .into_any_element(),
                                ]
                            } else {
                                vec![Label::new("Shared by host")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small)
                                    .into_any_element()]
                            }),
                    )
            }
            ContextMetadata::Saved(context) => div()
                .flex()
                .w_full()
                .justify_between()
                .gap_2()
                .child(
                    h_flex()
                        .flex_1()
                        .child(Label::new(context.title.clone()).size(LabelSize::Small))
                        .overflow_x_hidden(),
                )
                .child(
                    Label::new(format_distance_from_now(
                        DateTimeType::Local(context.mtime),
                        false,
                        true,
                        true,
                    ))
                    .color(Color::Muted)
                    .size(LabelSize::Small),
                ),
        };
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(item),
        )
    }
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let context_store = workspace
                .update(&mut cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    ContextStore::new(project, prompt_builder.clone(), cx)
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
        let model_summary_editor = cx.new_view(|cx| Editor::single_line(cx));
        let context_editor_toolbar = cx.new_view(|_| {
            ContextEditorToolbarItem::new(
                workspace,
                model_selector_menu_handle.clone(),
                model_summary_editor.clone(),
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
            pane.set_can_split(false, cx);
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
                    .tooltip(move |cx| {
                        Tooltip::for_action_in("Open History", &DeployHistory, &focus_handle, cx)
                    })
                    .selected(
                        pane.active_item()
                            .map_or(false, |item| item.downcast::<ContextHistory>().is_some()),
                    );
                let _pane = cx.view().clone();
                let right_children = h_flex()
                    .gap(Spacing::Small.rems(cx))
                    .child(
                        IconButton::new("new-context", IconName::Plus)
                            .on_click(
                                cx.listener(|_, _, cx| {
                                    cx.dispatch_action(NewContext.boxed_clone())
                                }),
                            )
                            .tooltip(|cx| Tooltip::for_action("New Context", &NewContext, cx)),
                    )
                    .child(
                        PopoverMenu::new("assistant-panel-popover-menu")
                            .trigger(
                                IconButton::new("menu", IconName::Menu).icon_size(IconSize::Small),
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
                                        .action("New Context", Box::new(NewContext))
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

            pane::Event::ActivateItem { local } => {
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
            && active_provider.map_or(true, |provider| provider.id().0 == PROVIDER_ID);

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
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx).log_err();

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
            panel.update(cx, |panel, cx| {
                panel.new_context(cx);
            });
        }
    }

    fn new_context(&mut self, cx: &mut ViewContext<Self>) -> Option<View<ContextEditor>> {
        if self.project.read(cx).is_via_collab() {
            let task = self
                .context_store
                .update(cx, |store, cx| store.create_remote_context(cx));

            cx.spawn(|this, mut cx| async move {
                let context = task.await?;

                this.update(&mut cx, |this, cx| {
                    let workspace = this.workspace.clone();
                    let project = this.project.clone();
                    let lsp_adapter_delegate = make_lsp_adapter_delegate(&project, cx).log_err();

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
            let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx).log_err();

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
            EditorEvent::Edited { .. } => cx.emit(AssistantPanelEvent::ContextEdited),
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
            let configuration = cx.new_view(|cx| ConfigurationView::new(cx));
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
        open_prompt_library(self.languages.clone(), cx).detach_and_log_err(cx);
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.model_selector_menu_handle.toggle(cx);
    }

    fn active_context_editor(&self, cx: &AppContext) -> Option<View<ContextEditor>> {
        self.pane
            .read(cx)
            .active_item()?
            .downcast::<ContextEditor>()
    }

    pub fn active_context(&self, cx: &AppContext) -> Option<Model<Context>> {
        Some(self.active_context_editor(cx)?.read(cx).context.clone())
    }

    fn open_saved_context(
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

        let lsp_adapter_delegate = make_lsp_adapter_delegate(&project, cx).log_err();

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

    fn open_remote_context(
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
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx).log_err();

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
}

impl EventEmitter<PanelEvent> for AssistantPanel {}
impl EventEmitter<AssistantPanelEvent> for AssistantPanel {}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

pub enum ContextEditorEvent {
    Edited,
    TabContentChanged,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ScrollPosition {
    offset_before_cursor: gpui::Point<f32>,
    cursor: Anchor,
}

struct WorkflowStepViewState {
    header_block_id: CustomBlockId,
    header_crease_id: CreaseId,
    footer_block_id: Option<CustomBlockId>,
    footer_crease_id: Option<CreaseId>,
    assist: Option<WorkflowAssist>,
    resolution: Option<Arc<Result<WorkflowStepResolution>>>,
}

impl WorkflowStepViewState {
    fn status(&self, cx: &AppContext) -> WorkflowStepStatus {
        if let Some(assist) = &self.assist {
            match assist.status(cx) {
                WorkflowAssistStatus::Idle => WorkflowStepStatus::Idle,
                WorkflowAssistStatus::Pending => WorkflowStepStatus::Pending,
                WorkflowAssistStatus::Done => WorkflowStepStatus::Done,
                WorkflowAssistStatus::Confirmed => WorkflowStepStatus::Confirmed,
            }
        } else if let Some(resolution) = self.resolution.as_deref() {
            match resolution {
                Err(err) => WorkflowStepStatus::Error(err),
                Ok(_) => WorkflowStepStatus::Idle,
            }
        } else {
            WorkflowStepStatus::Resolving
        }
    }
}

#[derive(Clone, Copy)]
enum WorkflowStepStatus<'a> {
    Resolving,
    Error(&'a anyhow::Error),
    Idle,
    Pending,
    Done,
    Confirmed,
}

impl<'a> WorkflowStepStatus<'a> {
    pub(crate) fn is_confirmed(&self) -> bool {
        matches!(self, Self::Confirmed)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ActiveWorkflowStep {
    range: Range<language::Anchor>,
    resolved: bool,
}

struct WorkflowAssist {
    editor: WeakView<Editor>,
    editor_was_open: bool,
    assist_ids: Vec<InlineAssistId>,
}

type MessageHeader = MessageMetadata;

pub struct ContextEditor {
    context: Model<Context>,
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
    editor: View<Editor>,
    blocks: HashMap<MessageId, (MessageHeader, CustomBlockId)>,
    image_blocks: HashSet<CustomBlockId>,
    scroll_position: Option<ScrollPosition>,
    remote_id: Option<workspace::ViewId>,
    pending_slash_command_creases: HashMap<Range<language::Anchor>, CreaseId>,
    pending_slash_command_blocks: HashMap<Range<language::Anchor>, CustomBlockId>,
    _subscriptions: Vec<Subscription>,
    workflow_steps: HashMap<Range<language::Anchor>, WorkflowStepViewState>,
    active_workflow_step: Option<ActiveWorkflowStep>,
    assistant_panel: WeakView<AssistantPanel>,
    error_message: Option<SharedString>,
    show_accept_terms: bool,
    pub(crate) slash_menu_handle:
        PopoverMenuHandle<Picker<slash_command_picker::SlashCommandDelegate>>,
}

const DEFAULT_TAB_TITLE: &str = "New Context";
const MAX_TAB_TITLE_LEN: usize = 16;

impl ContextEditor {
    fn for_context(
        context: Model<Context>,
        fs: Arc<dyn Fs>,
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
        assistant_panel: WeakView<AssistantPanel>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let completion_provider = SlashCommandCompletionProvider::new(
            Some(cx.view().downgrade()),
            Some(workspace.clone()),
        );

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::for_buffer(context.read(cx).buffer().clone(), None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Box::new(completion_provider));
            editor.set_collaboration_hub(Box::new(project.clone()));
            editor
        });

        let _subscriptions = vec![
            cx.observe(&context, |_, _, cx| cx.notify()),
            cx.subscribe(&context, Self::handle_context_event),
            cx.subscribe(&editor, Self::handle_editor_event),
            cx.subscribe(&editor, Self::handle_editor_search_event),
        ];

        let sections = context.read(cx).slash_command_output_sections().to_vec();
        let edit_step_ranges = context.read(cx).workflow_step_ranges().collect::<Vec<_>>();
        let mut this = Self {
            context,
            editor,
            lsp_adapter_delegate,
            blocks: Default::default(),
            image_blocks: Default::default(),
            scroll_position: None,
            remote_id: None,
            fs,
            workspace,
            project,
            pending_slash_command_creases: HashMap::default(),
            pending_slash_command_blocks: HashMap::default(),
            _subscriptions,
            workflow_steps: HashMap::default(),
            active_workflow_step: None,
            assistant_panel,
            error_message: None,
            show_accept_terms: false,
            slash_menu_handle: Default::default(),
        };
        this.update_message_headers(cx);
        this.update_image_blocks(cx);
        this.insert_slash_command_output_sections(sections, false, cx);
        this.workflow_steps_updated(&Vec::new(), &edit_step_ranges, cx);
        this
    }

    fn insert_default_prompt(&mut self, cx: &mut ViewContext<Self>) {
        let command_name = DefaultSlashCommand.name();
        self.editor.update(cx, |editor, cx| {
            editor.insert(&format!("/{command_name}\n\n"), cx)
        });
        let command = self.context.update(cx, |context, cx| {
            context.reparse(cx);
            context.pending_slash_commands()[0].clone()
        });
        self.run_command(
            command.source_range,
            &command.name,
            &command.arguments,
            false,
            false,
            self.workspace.clone(),
            cx,
        );
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        let provider = LanguageModelRegistry::read_global(cx).active_provider();
        if provider
            .as_ref()
            .map_or(false, |provider| provider.must_accept_terms(cx))
        {
            self.show_accept_terms = true;
            cx.notify();
            return;
        }

        if !self.apply_active_workflow_step(cx) {
            self.error_message = None;
            self.send_to_model(cx);
            cx.notify();
        }
    }

    fn apply_workflow_step(&mut self, range: Range<language::Anchor>, cx: &mut ViewContext<Self>) {
        self.show_workflow_step(range.clone(), cx);

        if let Some(workflow_step) = self.workflow_steps.get(&range) {
            if let Some(assist) = workflow_step.assist.as_ref() {
                let assist_ids = assist.assist_ids.clone();
                cx.spawn(|this, mut cx| async move {
                    for assist_id in assist_ids {
                        let mut receiver = this.update(&mut cx, |_, cx| {
                            cx.window_context().defer(move |cx| {
                                InlineAssistant::update_global(cx, |assistant, cx| {
                                    assistant.start_assist(assist_id, cx);
                                })
                            });
                            InlineAssistant::update_global(cx, |assistant, _| {
                                assistant.observe_assist(assist_id)
                            })
                        })?;
                        while !receiver.borrow().is_done() {
                            let _ = receiver.changed().await;
                        }
                    }
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
        }
    }

    fn apply_active_workflow_step(&mut self, cx: &mut ViewContext<Self>) -> bool {
        let Some((range, step)) = self.active_workflow_step() else {
            return false;
        };

        if let Some(assist) = step.assist.as_ref() {
            match assist.status(cx) {
                WorkflowAssistStatus::Pending => {}
                WorkflowAssistStatus::Confirmed => return false,
                WorkflowAssistStatus::Done => self.confirm_workflow_step(range, cx),
                WorkflowAssistStatus::Idle => self.apply_workflow_step(range, cx),
            }
        } else {
            match step.resolution.as_deref() {
                Some(Ok(_)) => self.apply_workflow_step(range, cx),
                Some(Err(_)) => self.resolve_workflow_step(range, cx),
                None => {}
            }
        }

        true
    }

    fn resolve_workflow_step(
        &mut self,
        range: Range<language::Anchor>,
        cx: &mut ViewContext<Self>,
    ) {
        self.context
            .update(cx, |context, cx| context.resolve_workflow_step(range, cx));
    }

    fn stop_workflow_step(&mut self, range: Range<language::Anchor>, cx: &mut ViewContext<Self>) {
        if let Some(workflow_step) = self.workflow_steps.get(&range) {
            if let Some(assist) = workflow_step.assist.as_ref() {
                let assist_ids = assist.assist_ids.clone();
                cx.window_context().defer(|cx| {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        for assist_id in assist_ids {
                            assistant.stop_assist(assist_id, cx);
                        }
                    })
                });
            }
        }
    }

    fn undo_workflow_step(&mut self, range: Range<language::Anchor>, cx: &mut ViewContext<Self>) {
        if let Some(workflow_step) = self.workflow_steps.get_mut(&range) {
            if let Some(assist) = workflow_step.assist.take() {
                cx.window_context().defer(|cx| {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        for assist_id in assist.assist_ids {
                            assistant.undo_assist(assist_id, cx);
                        }
                    })
                });
            }
        }
    }

    fn confirm_workflow_step(
        &mut self,
        range: Range<language::Anchor>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(workflow_step) = self.workflow_steps.get(&range) {
            if let Some(assist) = workflow_step.assist.as_ref() {
                let assist_ids = assist.assist_ids.clone();
                cx.window_context().defer(move |cx| {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        for assist_id in assist_ids {
                            assistant.finish_assist(assist_id, false, cx);
                        }
                    })
                });
            }
        }
    }

    fn reject_workflow_step(&mut self, range: Range<language::Anchor>, cx: &mut ViewContext<Self>) {
        if let Some(workflow_step) = self.workflow_steps.get_mut(&range) {
            if let Some(assist) = workflow_step.assist.take() {
                cx.window_context().defer(move |cx| {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        for assist_id in assist.assist_ids {
                            assistant.finish_assist(assist_id, true, cx);
                        }
                    })
                });
            }
        }
    }

    fn send_to_model(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(user_message) = self.context.update(cx, |context, cx| context.assist(cx)) {
            let new_selection = {
                let cursor = user_message
                    .start
                    .to_offset(self.context.read(cx).buffer().read(cx));
                cursor..cursor
            };
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    Some(Autoscroll::Strategy(AutoscrollStrategy::Fit)),
                    cx,
                    |selections| selections.select_ranges([new_selection]),
                );
            });
            // Avoid scrolling to the new cursor position so the assistant's output is stable.
            cx.defer(|this, _| this.scroll_position = None);
        }
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        self.error_message = None;

        if self
            .context
            .update(cx, |context, cx| context.cancel_last_assist(cx))
        {
            return;
        }

        if let Some((range, active_step)) = self.active_workflow_step() {
            match active_step.status(cx) {
                WorkflowStepStatus::Pending => {
                    self.stop_workflow_step(range, cx);
                    return;
                }
                WorkflowStepStatus::Done => {
                    self.reject_workflow_step(range, cx);
                    return;
                }
                _ => {}
            }
        }
        cx.propagate();
    }

    fn cycle_message_role(&mut self, _: &CycleMessageRole, cx: &mut ViewContext<Self>) {
        let cursors = self.cursors(cx);
        self.context.update(cx, |context, cx| {
            let messages = context
                .messages_for_offsets(cursors, cx)
                .into_iter()
                .map(|message| message.id)
                .collect();
            context.cycle_message_roles(messages, cx)
        });
    }

    fn cursors(&self, cx: &AppContext) -> Vec<usize> {
        let selections = self.editor.read(cx).selections.all::<usize>(cx);
        selections
            .into_iter()
            .map(|selection| selection.head())
            .collect()
    }

    pub fn insert_command(&mut self, name: &str, cx: &mut ViewContext<Self>) {
        if let Some(command) = SlashCommandRegistry::global(cx).command(name) {
            self.editor.update(cx, |editor, cx| {
                editor.transact(cx, |editor, cx| {
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| s.try_cancel());
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let newest_cursor = editor.selections.newest::<Point>(cx).head();
                    if newest_cursor.column > 0
                        || snapshot
                            .chars_at(newest_cursor)
                            .next()
                            .map_or(false, |ch| ch != '\n')
                    {
                        editor.move_to_end_of_line(
                            &MoveToEndOfLine {
                                stop_at_soft_wraps: false,
                            },
                            cx,
                        );
                        editor.newline(&Newline, cx);
                    }

                    editor.insert(&format!("/{name}"), cx);
                    if command.accepts_arguments() {
                        editor.insert(" ", cx);
                        editor.show_completions(&ShowCompletions::default(), cx);
                    }
                });
            });
            if !command.requires_argument() {
                self.confirm_command(&ConfirmCommand, cx);
            }
        }
    }

    pub fn confirm_command(&mut self, _: &ConfirmCommand, cx: &mut ViewContext<Self>) {
        if self.editor.read(cx).has_active_completions_menu() {
            return;
        }

        let selections = self.editor.read(cx).selections.disjoint_anchors();
        let mut commands_by_range = HashMap::default();
        let workspace = self.workspace.clone();
        self.context.update(cx, |context, cx| {
            context.reparse(cx);
            for selection in selections.iter() {
                if let Some(command) =
                    context.pending_command_for_position(selection.head().text_anchor, cx)
                {
                    commands_by_range
                        .entry(command.source_range.clone())
                        .or_insert_with(|| command.clone());
                }
            }
        });

        if commands_by_range.is_empty() {
            cx.propagate();
        } else {
            for command in commands_by_range.into_values() {
                self.run_command(
                    command.source_range,
                    &command.name,
                    &command.arguments,
                    true,
                    false,
                    workspace.clone(),
                    cx,
                );
            }
            cx.stop_propagation();
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn run_command(
        &mut self,
        command_range: Range<language::Anchor>,
        name: &str,
        arguments: &[String],
        ensure_trailing_newline: bool,
        expand_result: bool,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(command) = SlashCommandRegistry::global(cx).command(name) {
            let output = command.run(arguments, workspace, self.lsp_adapter_delegate.clone(), cx);
            self.context.update(cx, |context, cx| {
                context.insert_command_output(
                    command_range,
                    output,
                    ensure_trailing_newline,
                    expand_result,
                    cx,
                )
            });
        }
    }

    fn handle_context_event(
        &mut self,
        _: Model<Context>,
        event: &ContextEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let context_editor = cx.view().downgrade();

        match event {
            ContextEvent::MessagesEdited => {
                self.update_message_headers(cx);
                self.update_image_blocks(cx);
                self.context.update(cx, |context, cx| {
                    context.save(Some(Duration::from_millis(500)), self.fs.clone(), cx);
                });
            }
            ContextEvent::SummaryChanged => {
                cx.emit(EditorEvent::TitleChanged);
                self.context.update(cx, |context, cx| {
                    context.save(Some(Duration::from_millis(500)), self.fs.clone(), cx);
                });
            }
            ContextEvent::StreamedCompletion => {
                self.editor.update(cx, |editor, cx| {
                    if let Some(scroll_position) = self.scroll_position {
                        let snapshot = editor.snapshot(cx);
                        let cursor_point = scroll_position.cursor.to_display_point(&snapshot);
                        let scroll_top =
                            cursor_point.row().as_f32() - scroll_position.offset_before_cursor.y;
                        editor.set_scroll_position(
                            point(scroll_position.offset_before_cursor.x, scroll_top),
                            cx,
                        );
                    }
                });
            }
            ContextEvent::WorkflowStepsUpdated { removed, updated } => {
                self.workflow_steps_updated(removed, updated, cx);
            }
            ContextEvent::PendingSlashCommandsUpdated { removed, updated } => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let (excerpt_id, buffer_id, _) = buffer.as_singleton().unwrap();
                    let excerpt_id = *excerpt_id;

                    editor.remove_creases(
                        removed
                            .iter()
                            .filter_map(|range| self.pending_slash_command_creases.remove(range)),
                        cx,
                    );

                    editor.remove_blocks(
                        HashSet::from_iter(
                            removed.iter().filter_map(|range| {
                                self.pending_slash_command_blocks.remove(range)
                            }),
                        ),
                        None,
                        cx,
                    );

                    let crease_ids = editor.insert_creases(
                        updated.iter().map(|command| {
                            let workspace = self.workspace.clone();
                            let confirm_command = Arc::new({
                                let context_editor = context_editor.clone();
                                let command = command.clone();
                                move |cx: &mut WindowContext| {
                                    context_editor
                                        .update(cx, |context_editor, cx| {
                                            context_editor.run_command(
                                                command.source_range.clone(),
                                                &command.name,
                                                &command.arguments,
                                                false,
                                                false,
                                                workspace.clone(),
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            });
                            let placeholder = FoldPlaceholder {
                                render: Arc::new(move |_, _, _| Empty.into_any()),
                                constrain_width: false,
                                merge_adjacent: false,
                            };
                            let render_toggle = {
                                let confirm_command = confirm_command.clone();
                                let command = command.clone();
                                move |row, _, _, _cx: &mut WindowContext| {
                                    render_pending_slash_command_gutter_decoration(
                                        row,
                                        &command.status,
                                        confirm_command.clone(),
                                    )
                                }
                            };
                            let render_trailer = {
                                let command = command.clone();
                                move |row, _unfold, cx: &mut WindowContext| {
                                    // TODO: In the future we should investigate how we can expose
                                    // this as a hook on the `SlashCommand` trait so that we don't
                                    // need to special-case it here.
                                    if command.name == DocsSlashCommand::NAME {
                                        return render_docs_slash_command_trailer(
                                            row,
                                            command.clone(),
                                            cx,
                                        );
                                    }

                                    Empty.into_any()
                                }
                            };

                            let start = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.start)
                                .unwrap();
                            let end = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.end)
                                .unwrap();
                            Crease::new(start..end, placeholder, render_toggle, render_trailer)
                        }),
                        cx,
                    );

                    let block_ids = editor.insert_blocks(
                        updated
                            .iter()
                            .filter_map(|command| match &command.status {
                                PendingSlashCommandStatus::Error(error) => {
                                    Some((command, error.clone()))
                                }
                                _ => None,
                            })
                            .map(|(command, error_message)| BlockProperties {
                                style: BlockStyle::Fixed,
                                position: Anchor {
                                    buffer_id: Some(buffer_id),
                                    excerpt_id,
                                    text_anchor: command.source_range.start,
                                },
                                height: 1,
                                disposition: BlockDisposition::Below,
                                render: slash_command_error_block_renderer(error_message),
                                priority: 0,
                            }),
                        None,
                        cx,
                    );

                    self.pending_slash_command_creases.extend(
                        updated
                            .iter()
                            .map(|command| command.source_range.clone())
                            .zip(crease_ids),
                    );

                    self.pending_slash_command_blocks.extend(
                        updated
                            .iter()
                            .map(|command| command.source_range.clone())
                            .zip(block_ids),
                    );
                })
            }
            ContextEvent::SlashCommandFinished {
                output_range,
                sections,
                run_commands_in_output,
                expand_result,
            } => {
                self.insert_slash_command_output_sections(
                    sections.iter().cloned(),
                    *expand_result,
                    cx,
                );

                if *run_commands_in_output {
                    let commands = self.context.update(cx, |context, cx| {
                        context.reparse(cx);
                        context
                            .pending_commands_for_range(output_range.clone(), cx)
                            .to_vec()
                    });

                    for command in commands {
                        self.run_command(
                            command.source_range,
                            &command.name,
                            &command.arguments,
                            false,
                            false,
                            self.workspace.clone(),
                            cx,
                        );
                    }
                }
            }
            ContextEvent::Operation(_) => {}
            ContextEvent::ShowAssistError(error_message) => {
                self.error_message = Some(error_message.clone());
            }
        }
    }

    fn workflow_steps_updated(
        &mut self,
        removed: &Vec<Range<text::Anchor>>,
        updated: &Vec<Range<text::Anchor>>,
        cx: &mut ViewContext<ContextEditor>,
    ) {
        let this = cx.view().downgrade();
        let mut removed_crease_ids = Vec::new();
        let mut removed_block_ids = HashSet::default();
        let mut editors_to_close = Vec::new();
        for range in removed {
            if let Some(state) = self.workflow_steps.remove(range) {
                editors_to_close.extend(self.hide_workflow_step(range.clone(), cx));
                removed_block_ids.insert(state.header_block_id);
                removed_crease_ids.push(state.header_crease_id);
                removed_block_ids.extend(state.footer_block_id);
                removed_crease_ids.extend(state.footer_crease_id);
            }
        }

        for range in updated {
            editors_to_close.extend(self.hide_workflow_step(range.clone(), cx));
        }

        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let multibuffer = &snapshot.buffer_snapshot;
            let (&excerpt_id, _, buffer) = multibuffer.as_singleton().unwrap();

            for range in updated {
                let Some(step) = self.context.read(cx).workflow_step_for_range(&range, cx) else {
                    continue;
                };

                let resolution = step.resolution.clone();
                let header_start = step.range.start;
                let header_end = if buffer.contains_str_at(step.leading_tags_end, "\n") {
                    buffer.anchor_before(step.leading_tags_end.to_offset(&buffer) + 1)
                } else {
                    step.leading_tags_end
                };
                let header_range = multibuffer
                    .anchor_in_excerpt(excerpt_id, header_start)
                    .unwrap()
                    ..multibuffer
                        .anchor_in_excerpt(excerpt_id, header_end)
                        .unwrap();
                let footer_range = step.trailing_tag_start.map(|start| {
                    let mut step_range_end = step.range.end.to_offset(&buffer);
                    if buffer.contains_str_at(step_range_end, "\n") {
                        // Only include the newline if it belongs to the same message.
                        let messages = self
                            .context
                            .read(cx)
                            .messages_for_offsets([step_range_end, step_range_end + 1], cx);
                        if messages.len() == 1 {
                            step_range_end += 1;
                        }
                    }

                    let end = buffer.anchor_before(step_range_end);
                    multibuffer.anchor_in_excerpt(excerpt_id, start).unwrap()
                        ..multibuffer.anchor_in_excerpt(excerpt_id, end).unwrap()
                });

                let block_ids = editor.insert_blocks(
                    [BlockProperties {
                        position: header_range.start,
                        height: 1,
                        style: BlockStyle::Flex,
                        render: Box::new({
                            let this = this.clone();
                            let range = step.range.clone();
                            move |cx| {
                                let block_id = cx.block_id;
                                let max_width = cx.max_width;
                                let gutter_width = cx.gutter_dimensions.full_width();
                                this.update(&mut **cx, |this, cx| {
                                    this.render_workflow_step_header(
                                        range.clone(),
                                        max_width,
                                        gutter_width,
                                        block_id,
                                        cx,
                                    )
                                })
                                .ok()
                                .flatten()
                                .unwrap_or_else(|| Empty.into_any())
                            }
                        }),
                        disposition: BlockDisposition::Above,
                        priority: 0,
                    }]
                    .into_iter()
                    .chain(footer_range.as_ref().map(|footer_range| {
                        return BlockProperties {
                            position: footer_range.end,
                            height: 1,
                            style: BlockStyle::Flex,
                            render: Box::new({
                                let this = this.clone();
                                let range = step.range.clone();
                                move |cx| {
                                    let max_width = cx.max_width;
                                    let gutter_width = cx.gutter_dimensions.full_width();
                                    this.update(&mut **cx, |this, cx| {
                                        this.render_workflow_step_footer(
                                            range.clone(),
                                            max_width,
                                            gutter_width,
                                            cx,
                                        )
                                    })
                                    .ok()
                                    .flatten()
                                    .unwrap_or_else(|| Empty.into_any())
                                }
                            }),
                            disposition: BlockDisposition::Below,
                            priority: 0,
                        };
                    })),
                    None,
                    cx,
                );

                let header_placeholder = FoldPlaceholder {
                    render: Arc::new(move |_, _crease_range, _cx| Empty.into_any()),
                    constrain_width: false,
                    merge_adjacent: false,
                };
                let footer_placeholder = FoldPlaceholder {
                    render: render_fold_icon_button(
                        cx.view().downgrade(),
                        IconName::Code,
                        "Edits".into(),
                    ),
                    constrain_width: false,
                    merge_adjacent: false,
                };

                let new_crease_ids = editor.insert_creases(
                    [Crease::new(
                        header_range.clone(),
                        header_placeholder.clone(),
                        fold_toggle("step-header"),
                        |_, _, _| Empty.into_any_element(),
                    )]
                    .into_iter()
                    .chain(footer_range.clone().map(|footer_range| {
                        Crease::new(
                            footer_range,
                            footer_placeholder.clone(),
                            |row, is_folded, fold, cx| {
                                if is_folded {
                                    Empty.into_any_element()
                                } else {
                                    fold_toggle("step-footer")(row, is_folded, fold, cx)
                                }
                            },
                            |_, _, _| Empty.into_any_element(),
                        )
                    })),
                    cx,
                );

                let state = WorkflowStepViewState {
                    header_block_id: block_ids[0],
                    header_crease_id: new_crease_ids[0],
                    footer_block_id: block_ids.get(1).copied(),
                    footer_crease_id: new_crease_ids.get(1).copied(),
                    resolution,
                    assist: None,
                };

                let mut folds_to_insert = [(header_range.clone(), header_placeholder)]
                    .into_iter()
                    .chain(
                        footer_range
                            .clone()
                            .map(|range| (range, footer_placeholder)),
                    )
                    .collect::<Vec<_>>();

                match self.workflow_steps.entry(range.clone()) {
                    hash_map::Entry::Vacant(entry) => {
                        entry.insert(state);
                    }
                    hash_map::Entry::Occupied(mut entry) => {
                        let entry = entry.get_mut();
                        removed_block_ids.insert(entry.header_block_id);
                        removed_crease_ids.push(entry.header_crease_id);
                        removed_block_ids.extend(entry.footer_block_id);
                        removed_crease_ids.extend(entry.footer_crease_id);
                        folds_to_insert.retain(|(range, _)| snapshot.intersects_fold(range.start));
                        *entry = state;
                    }
                }

                editor.unfold_ranges(
                    [header_range.clone()]
                        .into_iter()
                        .chain(footer_range.clone()),
                    true,
                    false,
                    cx,
                );

                if !folds_to_insert.is_empty() {
                    editor.fold_ranges(folds_to_insert, false, cx);
                }
            }

            editor.remove_creases(removed_crease_ids, cx);
            editor.remove_blocks(removed_block_ids, None, cx);
        });

        for (editor, editor_was_open) in editors_to_close {
            self.close_workflow_editor(cx, editor, editor_was_open);
        }

        self.update_active_workflow_step(cx);
    }

    fn insert_slash_command_output_sections(
        &mut self,
        sections: impl IntoIterator<Item = SlashCommandOutputSection<language::Anchor>>,
        expand_result: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let mut buffer_rows_to_fold = BTreeSet::new();
            let mut creases = Vec::new();
            for section in sections {
                let start = buffer
                    .anchor_in_excerpt(excerpt_id, section.range.start)
                    .unwrap();
                let end = buffer
                    .anchor_in_excerpt(excerpt_id, section.range.end)
                    .unwrap();
                let buffer_row = MultiBufferRow(start.to_point(&buffer).row);
                buffer_rows_to_fold.insert(buffer_row);
                creases.push(Crease::new(
                    start..end,
                    FoldPlaceholder {
                        render: render_fold_icon_button(
                            cx.view().downgrade(),
                            section.icon,
                            section.label.clone(),
                        ),
                        constrain_width: false,
                        merge_adjacent: false,
                    },
                    render_slash_command_output_toggle,
                    |_, _, _| Empty.into_any_element(),
                ));
            }

            editor.insert_creases(creases, cx);

            if expand_result {
                buffer_rows_to_fold.clear();
            }
            for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                editor.fold_at(&FoldAt { buffer_row }, cx);
            }
        });
    }

    fn handle_editor_event(
        &mut self,
        _: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::ScrollPositionChanged { autoscroll, .. } => {
                let cursor_scroll_position = self.cursor_scroll_position(cx);
                if *autoscroll {
                    self.scroll_position = cursor_scroll_position;
                } else if self.scroll_position != cursor_scroll_position {
                    self.scroll_position = None;
                }
            }
            EditorEvent::SelectionsChanged { .. } => {
                self.scroll_position = self.cursor_scroll_position(cx);
                self.update_active_workflow_step(cx);
            }
            _ => {}
        }
        cx.emit(event.clone());
    }

    fn active_workflow_step(&self) -> Option<(Range<text::Anchor>, &WorkflowStepViewState)> {
        let step = self.active_workflow_step.as_ref()?;
        Some((step.range.clone(), self.workflow_steps.get(&step.range)?))
    }

    fn update_active_workflow_step(&mut self, cx: &mut ViewContext<Self>) {
        let newest_cursor = self.editor.read(cx).selections.newest::<usize>(cx).head();
        let context = self.context.read(cx);

        let new_step = context
            .workflow_step_containing(newest_cursor, cx)
            .map(|step| ActiveWorkflowStep {
                resolved: step.resolution.is_some(),
                range: step.range.clone(),
            });

        if new_step.as_ref() != self.active_workflow_step.as_ref() {
            let mut old_editor = None;
            let mut old_editor_was_open = None;
            if let Some(old_step) = self.active_workflow_step.take() {
                (old_editor, old_editor_was_open) =
                    self.hide_workflow_step(old_step.range, cx).unzip();
            }

            let mut new_editor = None;
            if let Some(new_step) = new_step {
                new_editor = self.show_workflow_step(new_step.range.clone(), cx);
                self.active_workflow_step = Some(new_step);
            }

            if new_editor != old_editor {
                if let Some((old_editor, old_editor_was_open)) = old_editor.zip(old_editor_was_open)
                {
                    self.close_workflow_editor(cx, old_editor, old_editor_was_open)
                }
            }
        }
    }

    fn hide_workflow_step(
        &mut self,
        step_range: Range<language::Anchor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<(View<Editor>, bool)> {
        if let Some(step) = self.workflow_steps.get_mut(&step_range) {
            let assist = step.assist.as_ref()?;
            let editor = assist.editor.upgrade()?;

            if matches!(step.status(cx), WorkflowStepStatus::Idle) {
                let assist = step.assist.take().unwrap();
                InlineAssistant::update_global(cx, |assistant, cx| {
                    for assist_id in assist.assist_ids {
                        assistant.finish_assist(assist_id, true, cx)
                    }
                });
                return Some((editor, assist.editor_was_open));
            }
        }

        None
    }

    fn close_workflow_editor(
        &mut self,
        cx: &mut ViewContext<ContextEditor>,
        editor: View<Editor>,
        editor_was_open: bool,
    ) {
        self.workspace
            .update(cx, |workspace, cx| {
                if let Some(pane) = workspace.pane_for(&editor) {
                    pane.update(cx, |pane, cx| {
                        let item_id = editor.entity_id();
                        if !editor_was_open && !editor.read(cx).is_focused(cx) {
                            pane.close_item_by_id(item_id, SaveIntent::Skip, cx)
                                .detach_and_log_err(cx);
                        }
                    });
                }
            })
            .ok();
    }

    fn show_workflow_step(
        &mut self,
        step_range: Range<language::Anchor>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Editor>> {
        let step = self.workflow_steps.get_mut(&step_range)?;

        let mut editor_to_return = None;
        let mut scroll_to_assist_id = None;
        match step.status(cx) {
            WorkflowStepStatus::Idle => {
                if let Some(assist) = step.assist.as_ref() {
                    scroll_to_assist_id = assist.assist_ids.first().copied();
                } else if let Some(Ok(resolved)) = step.resolution.clone().as_deref() {
                    step.assist = Self::open_assists_for_step(
                        &resolved,
                        &self.project,
                        &self.assistant_panel,
                        &self.workspace,
                        cx,
                    );
                    editor_to_return = step
                        .assist
                        .as_ref()
                        .and_then(|assist| assist.editor.upgrade());
                }
            }
            WorkflowStepStatus::Pending => {
                if let Some(assist) = step.assist.as_ref() {
                    let assistant = InlineAssistant::global(cx);
                    scroll_to_assist_id = assist
                        .assist_ids
                        .iter()
                        .copied()
                        .find(|assist_id| assistant.assist_status(*assist_id, cx).is_pending());
                }
            }
            WorkflowStepStatus::Done => {
                if let Some(assist) = step.assist.as_ref() {
                    scroll_to_assist_id = assist.assist_ids.first().copied();
                }
            }
            _ => {}
        }

        if let Some(assist_id) = scroll_to_assist_id {
            if let Some(assist_editor) = step
                .assist
                .as_ref()
                .and_then(|assists| assists.editor.upgrade())
            {
                editor_to_return = Some(assist_editor.clone());
                self.workspace
                    .update(cx, |workspace, cx| {
                        workspace.activate_item(&assist_editor, false, false, cx);
                    })
                    .ok();
                InlineAssistant::update_global(cx, |assistant, cx| {
                    assistant.scroll_to_assist(assist_id, cx)
                });
            }
        }

        editor_to_return
    }

    fn open_assists_for_step(
        resolved_step: &WorkflowStepResolution,
        project: &Model<Project>,
        assistant_panel: &WeakView<AssistantPanel>,
        workspace: &WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Option<WorkflowAssist> {
        let assistant_panel = assistant_panel.upgrade()?;
        if resolved_step.suggestion_groups.is_empty() {
            return None;
        }

        let editor;
        let mut editor_was_open = false;
        let mut suggestion_groups = Vec::new();
        if resolved_step.suggestion_groups.len() == 1
            && resolved_step
                .suggestion_groups
                .values()
                .next()
                .unwrap()
                .len()
                == 1
        {
            // If there's only one buffer and one suggestion group, open it directly
            let (buffer, groups) = resolved_step.suggestion_groups.iter().next().unwrap();
            let group = groups.into_iter().next().unwrap();
            editor = workspace
                .update(cx, |workspace, cx| {
                    let active_pane = workspace.active_pane().clone();
                    editor_was_open =
                        workspace.is_project_item_open::<Editor>(&active_pane, buffer, cx);
                    workspace.open_project_item::<Editor>(
                        active_pane,
                        buffer.clone(),
                        false,
                        false,
                        cx,
                    )
                })
                .log_err()?;
            let (&excerpt_id, _, _) = editor
                .read(cx)
                .buffer()
                .read(cx)
                .read(cx)
                .as_singleton()
                .unwrap();

            // Scroll the editor to the suggested assist
            editor.update(cx, |editor, cx| {
                let multibuffer = editor.buffer().read(cx).snapshot(cx);
                let (&excerpt_id, _, buffer) = multibuffer.as_singleton().unwrap();
                let anchor = if group.context_range.start.to_offset(buffer) == 0 {
                    Anchor::min()
                } else {
                    multibuffer
                        .anchor_in_excerpt(excerpt_id, group.context_range.start)
                        .unwrap()
                };

                editor.set_scroll_anchor(
                    ScrollAnchor {
                        offset: gpui::Point::default(),
                        anchor,
                    },
                    cx,
                );
            });

            suggestion_groups.push((excerpt_id, group));
        } else {
            // If there are multiple buffers or suggestion groups, create a multibuffer
            let multibuffer = cx.new_model(|cx| {
                let replica_id = project.read(cx).replica_id();
                let mut multibuffer = MultiBuffer::new(replica_id, Capability::ReadWrite)
                    .with_title(resolved_step.title.clone());
                for (buffer, groups) in &resolved_step.suggestion_groups {
                    let excerpt_ids = multibuffer.push_excerpts(
                        buffer.clone(),
                        groups.iter().map(|suggestion_group| ExcerptRange {
                            context: suggestion_group.context_range.clone(),
                            primary: None,
                        }),
                        cx,
                    );
                    suggestion_groups.extend(excerpt_ids.into_iter().zip(groups));
                }
                multibuffer
            });

            editor = cx.new_view(|cx| {
                Editor::for_multibuffer(multibuffer, Some(project.clone()), true, cx)
            });
            workspace
                .update(cx, |workspace, cx| {
                    workspace.add_item_to_active_pane(Box::new(editor.clone()), None, false, cx)
                })
                .log_err()?;
        }

        let mut assist_ids = Vec::new();
        for (excerpt_id, suggestion_group) in suggestion_groups {
            for suggestion in &suggestion_group.suggestions {
                assist_ids.extend(suggestion.show(
                    &editor,
                    excerpt_id,
                    workspace,
                    &assistant_panel,
                    cx,
                ));
            }
        }

        Some(WorkflowAssist {
            assist_ids,
            editor: editor.downgrade(),
            editor_was_open,
        })
    }

    fn handle_editor_search_event(
        &mut self,
        _: View<Editor>,
        event: &SearchEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(event.clone());
    }

    fn cursor_scroll_position(&self, cx: &mut ViewContext<Self>) -> Option<ScrollPosition> {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let cursor = editor.selections.newest_anchor().head();
            let cursor_row = cursor
                .to_display_point(&snapshot.display_snapshot)
                .row()
                .as_f32();
            let scroll_position = editor
                .scroll_manager
                .anchor()
                .scroll_position(&snapshot.display_snapshot);

            let scroll_bottom = scroll_position.y + editor.visible_line_count().unwrap_or(0.);
            if (scroll_position.y..scroll_bottom).contains(&cursor_row) {
                Some(ScrollPosition {
                    cursor,
                    offset_before_cursor: point(scroll_position.x, cursor_row - scroll_position.y),
                })
            } else {
                None
            }
        })
    }

    fn update_message_headers(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);

            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let mut old_blocks = std::mem::take(&mut self.blocks);
            let mut blocks_to_remove: HashMap<_, _> = old_blocks
                .iter()
                .map(|(message_id, (_, block_id))| (*message_id, *block_id))
                .collect();
            let mut blocks_to_replace: HashMap<_, RenderBlock> = Default::default();

            let render_block = |message: MessageMetadata| -> RenderBlock {
                Box::new({
                    let context = self.context.clone();
                    move |cx| {
                        let message_id = MessageId(message.timestamp);
                        let show_spinner = message.role == Role::Assistant
                            && message.status == MessageStatus::Pending;

                        let label = match message.role {
                            Role::User => {
                                Label::new("You").color(Color::Default).into_any_element()
                            }
                            Role::Assistant => {
                                // If we have model info, display it alongside the Assistant label
                                if let Some(model_info) = message.model_info.as_ref() {
                                    let label_text = format!("Assistant ({})", model_info.model_name);
                                    let label = Label::new(label_text).color(Color::Info);
                                    
                                    if show_spinner {
                                        label
                                            .with_animation(
                                                "pulsating-label",
                                                Animation::new(Duration::from_secs(2))
                                                    .repeat()
                                                    .with_easing(pulsating_between(0.4, 0.8)),
                                                |label, delta| label.alpha(delta),
                                            )
                                            .into_any_element()
                                    } else {
                                        label.into_any_element()
                                    }
                                } else {
                                    // Default behavior when no model info is available
                                    let label = Label::new("Assistant").color(Color::Info);
                                    if show_spinner {
                                        label
                                            .with_animation(
                                                "pulsating-label",
                                                Animation::new(Duration::from_secs(2))
                                                    .repeat()
                                                    .with_easing(pulsating_between(0.4, 0.8)),
                                                |label, delta| label.alpha(delta),
                                            )
                                            .into_any_element()
                                    } else {
                                        label.into_any_element()
                                    }
                                }
                            }

                            Role::System => Label::new("System")
                                .color(Color::Warning)
                                .into_any_element(),
                        };

                        let sender = ButtonLike::new("role")
                            .style(ButtonStyle::Filled)
                            .child(label)
                            .tooltip(|cx| {
                                Tooltip::with_meta(
                                    "Toggle message role",
                                    None,
                                    "Available roles: You (User), Assistant, System",
                                    cx,
                                )
                            })
                            .on_click({
                                let context = context.clone();
                                move |_, cx| {
                                    context.update(cx, |context, cx| {
                                        context.cycle_message_roles(
                                            HashSet::from_iter(Some(message_id)),
                                            cx,
                                        )
                                    })
                                }
                            });

                        h_flex()
                            .id(("message_header", message_id.as_u64()))
                            .pl(cx.gutter_dimensions.full_width())
                            .h_11()
                            .w_full()
                            .relative()
                            .gap_1()
                            .child(sender)
                            .children(match &message.cache {
                                Some(cache) if cache.is_final_anchor => match cache.status {
                                    CacheStatus::Cached => Some(
                                        div()
                                            .id("cached")
                                            .child(
                                                Icon::new(IconName::DatabaseZap)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Hint),
                                            )
                                            .tooltip(|cx| {
                                                Tooltip::with_meta(
                                                    "Context cached",
                                                    None,
                                                    "Large messages cached to optimize performance",
                                                    cx,
                                                )
                                            })
                                            .into_any_element(),
                                    ),
                                    CacheStatus::Pending => Some(
                                        div()
                                            .child(
                                                Icon::new(IconName::Ellipsis)
                                                    .size(IconSize::XSmall)
                                                    .color(Color::Hint),
                                            )
                                            .into_any_element(),
                                    ),
                                },
                                _ => None,
                            })
                            .children(match &message.status {
                                MessageStatus::Error(error) => Some(
                                    Button::new("show-error", "Error")
                                        .color(Color::Error)
                                        .selected_label_color(Color::Error)
                                        .selected_icon_color(Color::Error)
                                        .icon(IconName::XCircle)
                                        .icon_color(Color::Error)
                                        .icon_size(IconSize::Small)
                                        .icon_position(IconPosition::Start)
                                        .tooltip(move |cx| {
                                            Tooltip::with_meta(
                                                "Error interacting with language model",
                                                None,
                                                "Click for more details",
                                                cx,
                                            )
                                        })
                                        .on_click({
                                            let context = context.clone();
                                            let error = error.clone();
                                            move |_, cx| {
                                                context.update(cx, |_, cx| {
                                                    cx.emit(ContextEvent::ShowAssistError(
                                                        error.clone(),
                                                    ));
                                                });
                                            }
                                        })
                                        .into_any_element(),
                                ),
                                MessageStatus::Canceled => Some(
                                    ButtonLike::new("canceled")
                                        .child(Icon::new(IconName::XCircle).color(Color::Disabled))
                                        .child(
                                            Label::new("Canceled")
                                                .size(LabelSize::Small)
                                                .color(Color::Disabled),
                                        )
                                        .tooltip(move |cx| {
                                            Tooltip::with_meta(
                                                "Canceled",
                                                None,
                                                "Interaction with the assistant was canceled",
                                                cx,
                                            )
                                        })
                                        .into_any_element(),
                                ),
                                _ => None,
                            })
                            .into_any_element()
                    }
                })
            };
            let create_block_properties = |message: &Message| BlockProperties {
                position: buffer
                    .anchor_in_excerpt(excerpt_id, message.anchor_range.start)
                    .unwrap(),
                height: 2,
                style: BlockStyle::Sticky,
                disposition: BlockDisposition::Above,
                priority: usize::MAX,
                render: render_block(MessageMetadata::from(message)),
            };
            let mut new_blocks = vec![];
            let mut block_index_to_message = vec![];
            for message in self.context.read(cx).messages(cx) {
                if let Some(_) = blocks_to_remove.remove(&message.id) {
                    // This is an old message that we might modify.
                    let Some((meta, block_id)) = old_blocks.get_mut(&message.id) else {
                        debug_assert!(
                            false,
                            "old_blocks should contain a message_id we've just removed."
                        );
                        continue;
                    };
                    // Should we modify it?
                    let message_meta = MessageMetadata::from(&message);
                    if meta != &message_meta {
                        blocks_to_replace.insert(*block_id, render_block(message_meta.clone()));
                        *meta = message_meta;
                    }
                } else {
                    // This is a new message.
                    new_blocks.push(create_block_properties(&message));
                    block_index_to_message.push((message.id, MessageMetadata::from(&message)));
                }
            }
            editor.replace_blocks(blocks_to_replace, None, cx);
            editor.remove_blocks(blocks_to_remove.into_values().collect(), None, cx);

            let ids = editor.insert_blocks(new_blocks, None, cx);
            old_blocks.extend(ids.into_iter().zip(block_index_to_message).map(
                |(block_id, (message_id, message_meta))| (message_id, (message_meta, block_id)),
            ));
            self.blocks = old_blocks;
        });
    }

    fn insert_selection(
        workspace: &mut Workspace,
        _: &InsertIntoEditor,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let Some(context_editor_view) = panel.read(cx).active_context_editor(cx) else {
            return;
        };
        let Some(active_editor_view) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        let context_editor = context_editor_view.read(cx).editor.read(cx);
        let anchor = context_editor.selections.newest_anchor();
        let text = context_editor
            .buffer()
            .read(cx)
            .read(cx)
            .text_for_range(anchor.range())
            .collect::<String>();

        // If nothing is selected, don't delete the current selection; instead, be a no-op.
        if !text.is_empty() {
            active_editor_view.update(cx, |editor, cx| {
                editor.insert(&text, cx);
                editor.focus(cx);
            })
        }
    }

    fn quote_selection(
        workspace: &mut Workspace,
        _: &QuoteSelection,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        let selection = editor.update(cx, |editor, cx| editor.selections.newest_adjusted(cx));
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx).snapshot(cx);
        let range = editor::ToOffset::to_offset(&selection.start, &buffer)
            ..editor::ToOffset::to_offset(&selection.end, &buffer);
        let selected_text = buffer.text_for_range(range.clone()).collect::<String>();
        if selected_text.is_empty() {
            return;
        }

        let start_language = buffer.language_at(range.start);
        let end_language = buffer.language_at(range.end);
        let language_name = if start_language == end_language {
            start_language.map(|language| language.code_fence_block_name())
        } else {
            None
        };
        let language_name = language_name.as_deref().unwrap_or("");

        let filename = buffer
            .file_at(selection.start)
            .map(|file| file.full_path(cx));

        let text = if language_name == "markdown" {
            selected_text
                .lines()
                .map(|line| format!("> {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            let start_symbols = buffer
                .symbols_containing(selection.start, None)
                .map(|(_, symbols)| symbols);
            let end_symbols = buffer
                .symbols_containing(selection.end, None)
                .map(|(_, symbols)| symbols);

            let outline_text =
                if let Some((start_symbols, end_symbols)) = start_symbols.zip(end_symbols) {
                    Some(
                        start_symbols
                            .into_iter()
                            .zip(end_symbols)
                            .take_while(|(a, b)| a == b)
                            .map(|(a, _)| a.text)
                            .collect::<Vec<_>>()
                            .join(" > "),
                    )
                } else {
                    None
                };

            let line_comment_prefix = start_language
                .and_then(|l| l.default_scope().line_comment_prefixes().first().cloned());

            let fence = codeblock_fence_for_path(
                filename.as_deref(),
                Some(selection.start.row..selection.end.row),
            );

            if let Some((line_comment_prefix, outline_text)) = line_comment_prefix.zip(outline_text)
            {
                let breadcrumb = format!("{line_comment_prefix}Excerpt from: {outline_text}\n");
                format!("{fence}{breadcrumb}{selected_text}\n```")
            } else {
                format!("{fence}{selected_text}\n```")
            }
        };

        let crease_title = if let Some(path) = filename {
            let start_line = selection.start.row + 1;
            let end_line = selection.end.row + 1;
            if start_line == end_line {
                format!("{}, Line {}", path.display(), start_line)
            } else {
                format!("{}, Lines {} to {}", path.display(), start_line, end_line)
            }
        } else {
            "Quoted selection".to_string()
        };

        // Activate the panel
        if !panel.focus_handle(cx).contains_focused(cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(cx);
        }

        panel.update(cx, |_, cx| {
            // Wait to create a new context until the workspace is no longer
            // being updated.
            cx.defer(move |panel, cx| {
                if let Some(context) = panel
                    .active_context_editor(cx)
                    .or_else(|| panel.new_context(cx))
                {
                    context.update(cx, |context, cx| {
                        context.editor.update(cx, |editor, cx| {
                            editor.insert("\n", cx);

                            let point = editor.selections.newest::<Point>(cx).head();
                            let start_row = MultiBufferRow(point.row);

                            editor.insert(&text, cx);

                            let snapshot = editor.buffer().read(cx).snapshot(cx);
                            let anchor_before = snapshot.anchor_after(point);
                            let anchor_after = editor
                                .selections
                                .newest_anchor()
                                .head()
                                .bias_left(&snapshot);

                            editor.insert("\n", cx);

                            let fold_placeholder = quote_selection_fold_placeholder(
                                crease_title,
                                cx.view().downgrade(),
                            );
                            let crease = Crease::new(
                                anchor_before..anchor_after,
                                fold_placeholder,
                                render_quote_selection_output_toggle,
                                |_, _, _| Empty.into_any(),
                            );
                            editor.insert_creases(vec![crease], cx);
                            editor.fold_at(
                                &FoldAt {
                                    buffer_row: start_row,
                                },
                                cx,
                            );
                        })
                    });
                };
            });
        });
    }

    fn copy(&mut self, _: &editor::actions::Copy, cx: &mut ViewContext<Self>) {
        let editor = self.editor.read(cx);
        let context = self.context.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            let mut copied_text = String::new();
            let mut spanned_messages = 0;
            for message in context.messages(cx) {
                if message.offset_range.start >= selection.range().end {
                    break;
                } else if message.offset_range.end >= selection.range().start {
                    let range = cmp::max(message.offset_range.start, selection.range().start)
                        ..cmp::min(message.offset_range.end, selection.range().end);
                    if !range.is_empty() {
                        spanned_messages += 1;
                        write!(&mut copied_text, "## {}\n\n", message.role).unwrap();
                        for chunk in context.buffer().read(cx).text_for_range(range) {
                            copied_text.push_str(chunk);
                        }
                        copied_text.push('\n');
                    }
                }
            }

            if spanned_messages > 1 {
                cx.write_to_clipboard(ClipboardItem::new_string(copied_text));
                return;
            }
        }

        cx.propagate();
    }

    fn paste(&mut self, _: &editor::actions::Paste, cx: &mut ViewContext<Self>) {
        let images = if let Some(item) = cx.read_from_clipboard() {
            item.into_entries()
                .filter_map(|entry| {
                    if let ClipboardEntry::Image(image) = entry {
                        Some(image)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        if images.is_empty() {
            // If we didn't find any valid image data to paste, propagate to let normal pasting happen.
            cx.propagate();
        } else {
            let mut image_positions = Vec::new();
            self.editor.update(cx, |editor, cx| {
                editor.transact(cx, |editor, cx| {
                    let edits = editor
                        .selections
                        .all::<usize>(cx)
                        .into_iter()
                        .map(|selection| (selection.start..selection.end, "\n"));
                    editor.edit(edits, cx);

                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    for selection in editor.selections.all::<usize>(cx) {
                        image_positions.push(snapshot.anchor_before(selection.end));
                    }
                });
            });

            self.context.update(cx, |context, cx| {
                for image in images {
                    let image_id = image.id();
                    context.insert_image(image, cx);
                    for image_position in image_positions.iter() {
                        context.insert_image_anchor(image_id, image_position.text_anchor, cx);
                    }
                }
            });
        }
    }

    fn update_image_blocks(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let old_blocks = std::mem::take(&mut self.image_blocks);
            let new_blocks = self
                .context
                .read(cx)
                .images(cx)
                .filter_map(|image| {
                    const MAX_HEIGHT_IN_LINES: u32 = 8;
                    let anchor = buffer.anchor_in_excerpt(excerpt_id, image.anchor).unwrap();
                    let image = image.render_image.clone();
                    anchor.is_valid(&buffer).then(|| BlockProperties {
                        position: anchor,
                        height: MAX_HEIGHT_IN_LINES,
                        style: BlockStyle::Sticky,
                        render: Box::new(move |cx| {
                            let image_size = size_for_image(
                                &image,
                                size(
                                    cx.max_width - cx.gutter_dimensions.full_width(),
                                    MAX_HEIGHT_IN_LINES as f32 * cx.line_height,
                                ),
                            );
                            h_flex()
                                .pl(cx.gutter_dimensions.full_width())
                                .child(
                                    img(image.clone())
                                        .object_fit(gpui::ObjectFit::ScaleDown)
                                        .w(image_size.width)
                                        .h(image_size.height),
                                )
                                .into_any_element()
                        }),

                        disposition: BlockDisposition::Above,
                        priority: 0,
                    })
                })
                .collect::<Vec<_>>();

            editor.remove_blocks(old_blocks, None, cx);
            let ids = editor.insert_blocks(new_blocks, None, cx);
            self.image_blocks = HashSet::from_iter(ids);
        });
    }

    fn split(&mut self, _: &Split, cx: &mut ViewContext<Self>) {
        self.context.update(cx, |context, cx| {
            let selections = self.editor.read(cx).selections.disjoint_anchors();
            for selection in selections.as_ref() {
                let buffer = self.editor.read(cx).buffer().read(cx).snapshot(cx);
                let range = selection
                    .map(|endpoint| endpoint.to_offset(&buffer))
                    .range();
                context.split_message(range, cx);
            }
        });
    }

    fn save(&mut self, _: &Save, cx: &mut ViewContext<Self>) {
        self.context.update(cx, |context, cx| {
            context.save(Some(Duration::from_millis(500)), self.fs.clone(), cx)
        });
    }

    fn title(&self, cx: &AppContext) -> Cow<str> {
        self.context
            .read(cx)
            .summary()
            .map(|summary| summary.text.clone())
            .map(Cow::Owned)
            .unwrap_or_else(|| Cow::Borrowed(DEFAULT_TAB_TITLE))
    }

    fn render_workflow_step_header(
        &self,
        range: Range<text::Anchor>,
        max_width: Pixels,
        gutter_width: Pixels,
        id: BlockId,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let step_state = self.workflow_steps.get(&range)?;
        let status = step_state.status(cx);
        let this = cx.view().downgrade();

        let theme = cx.theme().status();
        let is_confirmed = status.is_confirmed();
        let border_color = if is_confirmed {
            theme.ignored_border
        } else {
            theme.info_border
        };

        let editor = self.editor.read(cx);
        let focus_handle = editor.focus_handle(cx);
        let snapshot = editor
            .buffer()
            .read(cx)
            .as_singleton()?
            .read(cx)
            .text_snapshot();
        let start_offset = range.start.to_offset(&snapshot);
        let parent_message = self
            .context
            .read(cx)
            .messages_for_offsets([start_offset], cx);
        debug_assert_eq!(parent_message.len(), 1);
        let parent_message = parent_message.first()?;

        let step_index = self
            .workflow_steps
            .keys()
            .filter(|workflow_step_range| {
                workflow_step_range
                    .start
                    .cmp(&parent_message.anchor_range.start, &snapshot)
                    .is_ge()
                    && workflow_step_range.end.cmp(&range.end, &snapshot).is_le()
            })
            .count();

        let step_label = Label::new(format!("Step {step_index}")).size(LabelSize::Small);

        let step_label = if is_confirmed {
            h_flex()
                .items_center()
                .gap_2()
                .child(step_label.strikethrough(true).color(Color::Muted))
                .child(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Created),
                )
        } else {
            div().child(step_label)
        };

        Some(
            v_flex()
                .w(max_width)
                .pl(gutter_width)
                .child(
                    h_flex()
                        .w_full()
                        .h_8()
                        .border_b_1()
                        .border_color(border_color)
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .child(h_flex().justify_start().gap_2().child(step_label))
                        .child(h_flex().w_full().justify_end().child(
                            Self::render_workflow_step_status(
                                status,
                                range.clone(),
                                focus_handle.clone(),
                                this.clone(),
                                id,
                            ),
                        )),
                )
                // todo!("do we wanna keep this?")
                // .children(edit_paths.iter().map(|path| {
                //     h_flex()
                //         .gap_1()
                //         .child(Icon::new(IconName::File))
                //         .child(Label::new(path.clone()))
                // }))
                .into_any(),
        )
    }

    fn render_workflow_step_footer(
        &self,
        step_range: Range<text::Anchor>,
        max_width: Pixels,
        gutter_width: Pixels,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let step = self.workflow_steps.get(&step_range)?;
        let current_status = step.status(cx);
        let theme = cx.theme().status();
        let border_color = if current_status.is_confirmed() {
            theme.ignored_border
        } else {
            theme.info_border
        };
        Some(
            v_flex()
                .w(max_width)
                .pt_1()
                .pl(gutter_width)
                .child(h_flex().h(px(1.)).bg(border_color))
                .into_any(),
        )
    }

    fn render_workflow_step_status(
        status: WorkflowStepStatus,
        step_range: Range<language::Anchor>,
        focus_handle: FocusHandle,
        editor: WeakView<ContextEditor>,
        id: BlockId,
    ) -> AnyElement {
        let id = EntityId::from(id).as_u64();
        fn display_keybind_in_tooltip(
            step_range: &Range<language::Anchor>,
            editor: &WeakView<ContextEditor>,
            cx: &mut WindowContext<'_>,
        ) -> bool {
            editor
                .update(cx, |this, _| {
                    this.active_workflow_step
                        .as_ref()
                        .map(|step| &step.range == step_range)
                })
                .ok()
                .flatten()
                .unwrap_or_default()
        }

        match status {
            WorkflowStepStatus::Error(error) => {
                let error = error.to_string();
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .id("step-resolution-failure")
                            .child(
                                Label::new("Step Resolution Failed")
                                    .size(LabelSize::Small)
                                    .color(Color::Error),
                            )
                            .tooltip(move |cx| Tooltip::text(error.clone(), cx)),
                    )
                    .child(
                        Button::new(("transform", id), "Retry")
                            .icon(IconName::Update)
                            .icon_position(IconPosition::Start)
                            .icon_size(IconSize::Small)
                            .label_size(LabelSize::Small)
                            .on_click({
                                let editor = editor.clone();
                                let step_range = step_range.clone();
                                move |_, cx| {
                                    editor
                                        .update(cx, |this, cx| {
                                            this.resolve_workflow_step(step_range.clone(), cx)
                                        })
                                        .ok();
                                }
                            }),
                    )
                    .into_any()
            }
            WorkflowStepStatus::Idle | WorkflowStepStatus::Resolving { .. } => {
                Button::new(("transform", id), "Transform")
                    .icon(IconName::SparkleAlt)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .label_size(LabelSize::Small)
                    .style(ButtonStyle::Tinted(TintColor::Accent))
                    .tooltip({
                        let step_range = step_range.clone();
                        let editor = editor.clone();
                        move |cx| {
                            cx.new_view(|cx| {
                                let tooltip = Tooltip::new("Transform");
                                if display_keybind_in_tooltip(&step_range, &editor, cx) {
                                    tooltip.key_binding(KeyBinding::for_action_in(
                                        &Assist,
                                        &focus_handle,
                                        cx,
                                    ))
                                } else {
                                    tooltip
                                }
                            })
                            .into()
                        }
                    })
                    .on_click({
                        let editor = editor.clone();
                        let step_range = step_range.clone();
                        let is_idle = matches!(status, WorkflowStepStatus::Idle);
                        move |_, cx| {
                            if is_idle {
                                editor
                                    .update(cx, |this, cx| {
                                        this.apply_workflow_step(step_range.clone(), cx)
                                    })
                                    .ok();
                            }
                        }
                    })
                    .map(|this| {
                        if let WorkflowStepStatus::Resolving = &status {
                            this.with_animation(
                                ("resolving-suggestion-animation", id),
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.4, 0.8)),
                                |label, delta| label.alpha(delta),
                            )
                            .into_any_element()
                        } else {
                            this.into_any_element()
                        }
                    })
            }
            WorkflowStepStatus::Pending => h_flex()
                .items_center()
                .gap_2()
                .child(
                    Label::new("Applying...")
                        .size(LabelSize::Small)
                        .with_animation(
                            ("applying-step-transformation-label", id),
                            Animation::new(Duration::from_secs(2))
                                .repeat()
                                .with_easing(pulsating_between(0.4, 0.8)),
                            |label, delta| label.alpha(delta),
                        ),
                )
                .child(
                    IconButton::new(("stop-transformation", id), IconName::Stop)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Error)
                        .style(ButtonStyle::Subtle)
                        .tooltip({
                            let step_range = step_range.clone();
                            let editor = editor.clone();
                            move |cx| {
                                cx.new_view(|cx| {
                                    let tooltip = Tooltip::new("Stop Transformation");
                                    if display_keybind_in_tooltip(&step_range, &editor, cx) {
                                        tooltip.key_binding(KeyBinding::for_action_in(
                                            &editor::actions::Cancel,
                                            &focus_handle,
                                            cx,
                                        ))
                                    } else {
                                        tooltip
                                    }
                                })
                                .into()
                            }
                        })
                        .on_click({
                            let editor = editor.clone();
                            let step_range = step_range.clone();
                            move |_, cx| {
                                editor
                                    .update(cx, |this, cx| {
                                        this.stop_workflow_step(step_range.clone(), cx)
                                    })
                                    .ok();
                            }
                        }),
                )
                .into_any_element(),
            WorkflowStepStatus::Done => h_flex()
                .gap_1()
                .child(
                    IconButton::new(("stop-transformation", id), IconName::Close)
                        .icon_size(IconSize::Small)
                        .style(ButtonStyle::Tinted(TintColor::Negative))
                        .tooltip({
                            let focus_handle = focus_handle.clone();
                            let editor = editor.clone();
                            let step_range = step_range.clone();
                            move |cx| {
                                cx.new_view(|cx| {
                                    let tooltip = Tooltip::new("Reject Transformation");
                                    if display_keybind_in_tooltip(&step_range, &editor, cx) {
                                        tooltip.key_binding(KeyBinding::for_action_in(
                                            &editor::actions::Cancel,
                                            &focus_handle,
                                            cx,
                                        ))
                                    } else {
                                        tooltip
                                    }
                                })
                                .into()
                            }
                        })
                        .on_click({
                            let editor = editor.clone();
                            let step_range = step_range.clone();
                            move |_, cx| {
                                editor
                                    .update(cx, |this, cx| {
                                        this.reject_workflow_step(step_range.clone(), cx);
                                    })
                                    .ok();
                            }
                        }),
                )
                .child(
                    Button::new(("confirm-workflow-step", id), "Accept")
                        .icon(IconName::Check)
                        .icon_position(IconPosition::Start)
                        .icon_size(IconSize::Small)
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Tinted(TintColor::Positive))
                        .tooltip({
                            let editor = editor.clone();
                            let step_range = step_range.clone();
                            move |cx| {
                                cx.new_view(|cx| {
                                    let tooltip = Tooltip::new("Accept Transformation");
                                    if display_keybind_in_tooltip(&step_range, &editor, cx) {
                                        tooltip.key_binding(KeyBinding::for_action_in(
                                            &Assist,
                                            &focus_handle,
                                            cx,
                                        ))
                                    } else {
                                        tooltip
                                    }
                                })
                                .into()
                            }
                        })
                        .on_click({
                            let editor = editor.clone();
                            let step_range = step_range.clone();
                            move |_, cx| {
                                editor
                                    .update(cx, |this, cx| {
                                        this.confirm_workflow_step(step_range.clone(), cx);
                                    })
                                    .ok();
                            }
                        }),
                )
                .into_any_element(),
            WorkflowStepStatus::Confirmed => h_flex()
                .child(
                    Button::new(("revert-workflow-step", id), "Undo")
                        .style(ButtonStyle::Filled)
                        .icon(Some(IconName::Undo))
                        .icon_position(IconPosition::Start)
                        .icon_size(IconSize::Small)
                        .label_size(LabelSize::Small)
                        .on_click({
                            let editor = editor.clone();
                            let step_range = step_range.clone();
                            move |_, cx| {
                                editor
                                    .update(cx, |this, cx| {
                                        this.undo_workflow_step(step_range.clone(), cx);
                                    })
                                    .ok();
                            }
                        }),
                )
                .into_any_element(),
        }
    }

    fn render_notice(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        use feature_flags::FeatureFlagAppExt;
        let nudge = self.assistant_panel.upgrade().map(|assistant_panel| {
            assistant_panel.read(cx).show_zed_ai_notice && cx.has_flag::<feature_flags::ZedPro>()
        });

        if nudge.map_or(false, |value| value) {
            Some(
                h_flex()
                    .p_3()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .bg(cx.theme().colors().editor_background)
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_3()
                            .child(Icon::new(IconName::ZedAssistant).color(Color::Accent))
                            .child(Label::new("Zed AI is here! Get started by signing in →")),
                    )
                    .child(
                        Button::new("sign-in", "Sign in")
                            .size(ButtonSize::Compact)
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _event, cx| {
                                let client = this
                                    .workspace
                                    .update(cx, |workspace, _| workspace.client().clone())
                                    .log_err();

                                if let Some(client) = client {
                                    cx.spawn(|this, mut cx| async move {
                                        client.authenticate_and_connect(true, &mut cx).await?;
                                        this.update(&mut cx, |_, cx| cx.notify())
                                    })
                                    .detach_and_log_err(cx)
                                }
                            })),
                    )
                    .into_any_element(),
            )
        } else if let Some(configuration_error) = configuration_error(cx) {
            let label = match configuration_error {
                ConfigurationError::NoProvider => "No LLM provider selected.",
                ConfigurationError::ProviderNotAuthenticated => "LLM provider is not configured.",
            };
            Some(
                h_flex()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .bg(cx.theme().colors().editor_background)
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_3()
                            .child(
                                Icon::new(IconName::ExclamationTriangle)
                                    .size(IconSize::Small)
                                    .color(Color::Warning),
                            )
                            .child(Label::new(label)),
                    )
                    .child(
                        Button::new("open-configuration", "Open configuration")
                            .size(ButtonSize::Compact)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Filled)
                            .on_click({
                                let focus_handle = self.focus_handle(cx).clone();
                                move |_event, cx| {
                                    focus_handle.dispatch_action(&ShowConfiguration, cx);
                                }
                            }),
                    )
                    .into_any_element(),
            )
        } else {
            None
        }
    }

    fn render_send_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();
        let button_text = match self.active_workflow_step() {
            Some((_, step)) => match step.status(cx) {
                WorkflowStepStatus::Error(_) => "Retry Step Resolution",
                WorkflowStepStatus::Resolving => "Transform",
                WorkflowStepStatus::Idle => "Transform",
                WorkflowStepStatus::Pending => "Applying...",
                WorkflowStepStatus::Done => "Accept",
                WorkflowStepStatus::Confirmed => "Send",
            },
            None => "Send",
        };

        let (style, tooltip) = match token_state(&self.context, cx) {
            Some(TokenState::NoTokensLeft { .. }) => (
                ButtonStyle::Tinted(TintColor::Negative),
                Some(Tooltip::text("Token limit reached", cx)),
            ),
            Some(TokenState::HasMoreTokens {
                over_warn_threshold,
                ..
            }) => {
                let (style, tooltip) = if over_warn_threshold {
                    (
                        ButtonStyle::Tinted(TintColor::Warning),
                        Some(Tooltip::text("Token limit is close to exhaustion", cx)),
                    )
                } else {
                    (ButtonStyle::Filled, None)
                };
                (style, tooltip)
            }
            None => (ButtonStyle::Filled, None),
        };

        let provider = LanguageModelRegistry::read_global(cx).active_provider();

        let has_configuration_error = configuration_error(cx).is_some();
        let needs_to_accept_terms = self.show_accept_terms
            && provider
                .as_ref()
                .map_or(false, |provider| provider.must_accept_terms(cx));
        let disabled = has_configuration_error || needs_to_accept_terms;

        ButtonLike::new("send_button")
            .disabled(disabled)
            .style(style)
            .when_some(tooltip, |button, tooltip| {
                button.tooltip(move |_| tooltip.clone())
            })
            .layer(ElevationIndex::ModalSurface)
            .child(Label::new(button_text))
            .children(
                KeyBinding::for_action_in(&Assist, &focus_handle, cx)
                    .map(|binding| binding.into_any_element()),
            )
            .on_click(move |_event, cx| {
                focus_handle.dispatch_action(&Assist, cx);
            })
    }
}

fn render_fold_icon_button(
    editor: WeakView<Editor>,
    icon: IconName,
    label: SharedString,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut WindowContext) -> AnyElement> {
    Arc::new(move |fold_id, fold_range, _cx| {
        let editor = editor.clone();
        ButtonLike::new(fold_id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(icon))
            .child(Label::new(label.clone()).single_line())
            .on_click(move |_, cx| {
                editor
                    .update(cx, |editor, cx| {
                        let buffer_start = fold_range
                            .start
                            .to_point(&editor.buffer().read(cx).read(cx));
                        let buffer_row = MultiBufferRow(buffer_start.row);
                        editor.unfold_at(&UnfoldAt { buffer_row }, cx);
                    })
                    .ok();
            })
            .into_any_element()
    })
}

impl EventEmitter<EditorEvent> for ContextEditor {}
impl EventEmitter<SearchEvent> for ContextEditor {}

impl Render for ContextEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let provider = LanguageModelRegistry::read_global(cx).active_provider();
        let accept_terms = if self.show_accept_terms {
            provider
                .as_ref()
                .and_then(|provider| provider.render_accept_terms(cx))
        } else {
            None
        };
        let focus_handle = self
            .workspace
            .update(cx, |workspace, cx| {
                Some(workspace.active_item_as::<Editor>(cx)?.focus_handle(cx))
            })
            .ok()
            .flatten();
        v_flex()
            .key_context("ContextEditor")
            .capture_action(cx.listener(ContextEditor::cancel))
            .capture_action(cx.listener(ContextEditor::save))
            .capture_action(cx.listener(ContextEditor::copy))
            .capture_action(cx.listener(ContextEditor::paste))
            .capture_action(cx.listener(ContextEditor::cycle_message_role))
            .capture_action(cx.listener(ContextEditor::confirm_command))
            .on_action(cx.listener(ContextEditor::assist))
            .on_action(cx.listener(ContextEditor::split))
            .size_full()
            .children(self.render_notice(cx))
            .child(
                div()
                    .flex_grow()
                    .bg(cx.theme().colors().editor_background)
                    .child(self.editor.clone()),
            )
            .when_some(accept_terms, |this, element| {
                this.child(
                    div()
                        .absolute()
                        .right_3()
                        .bottom_12()
                        .max_w_96()
                        .py_2()
                        .px_3()
                        .elevation_2(cx)
                        .bg(cx.theme().colors().surface_background)
                        .occlude()
                        .child(element),
                )
            })
            .when_some(self.error_message.clone(), |this, error_message| {
                this.child(
                    div()
                        .absolute()
                        .right_3()
                        .bottom_12()
                        .max_w_96()
                        .py_2()
                        .px_3()
                        .elevation_2(cx)
                        .occlude()
                        .child(
                            v_flex()
                                .gap_0p5()
                                .child(
                                    h_flex()
                                        .gap_1p5()
                                        .items_center()
                                        .child(Icon::new(IconName::XCircle).color(Color::Error))
                                        .child(
                                            Label::new("Error interacting with language model")
                                                .weight(FontWeight::MEDIUM),
                                        ),
                                )
                                .child(
                                    div()
                                        .id("error-message")
                                        .max_h_24()
                                        .overflow_y_scroll()
                                        .child(Label::new(error_message)),
                                )
                                .child(h_flex().justify_end().mt_1().child(
                                    Button::new("dismiss", "Dismiss").on_click(cx.listener(
                                        |this, _, cx| {
                                            this.error_message = None;
                                            cx.notify();
                                        },
                                    )),
                                )),
                        ),
                )
            })
            .child(
                h_flex().w_full().relative().child(
                    h_flex()
                        .p_2()
                        .w_full()
                        .border_t_1()
                        .border_color(cx.theme().colors().border_variant)
                        .bg(cx.theme().colors().editor_background)
                        .child(
                            h_flex()
                                .gap_2()
                                .child(render_inject_context_menu(cx.view().downgrade(), cx))
                                .child(
                                    IconButton::new("quote-button", IconName::Quote)
                                        .icon_size(IconSize::Small)
                                        .on_click(|_, cx| {
                                            cx.dispatch_action(QuoteSelection.boxed_clone());
                                        })
                                        .tooltip(move |cx| {
                                            cx.new_view(|cx| {
                                                Tooltip::new("Insert Selection").key_binding(
                                                    focus_handle.as_ref().and_then(|handle| {
                                                        KeyBinding::for_action_in(
                                                            &QuoteSelection,
                                                            &handle,
                                                            cx,
                                                        )
                                                    }),
                                                )
                                            })
                                            .into()
                                        }),
                                ),
                        )
                        .child(
                            h_flex()
                                .w_full()
                                .justify_end()
                                .child(div().child(self.render_send_button(cx))),
                        ),
                ),
            )
    }
}

impl FocusableView for ContextEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for ContextEditor {
    type Event = editor::EditorEvent;

    fn tab_content_text(&self, cx: &WindowContext) -> Option<SharedString> {
        Some(util::truncate_and_trailoff(&self.title(cx), MAX_TAB_TITLE_LEN).into())
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(item::ItemEvent)) {
        match event {
            EditorEvent::Edited { .. } => {
                f(item::ItemEvent::Edit);
            }
            EditorEvent::TitleChanged => {
                f(item::ItemEvent::UpdateTab);
            }
            _ => {}
        }
    }

    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString> {
        Some(self.title(cx).to_string().into())
    }

    fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn set_nav_history(&mut self, nav_history: pane::ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            Item::set_nav_history(editor, nav_history, cx)
        })
    }

    fn navigate(&mut self, data: Box<dyn std::any::Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| Item::navigate(editor, data, cx))
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::deactivated(editor, cx))
    }
}

impl SearchableItem for ContextEditor {
    type Match = <Editor as SearchableItem>::Match;

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.clear_matches(cx);
        });
    }

    fn update_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.update_matches(matches, cx));
    }

    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        self.editor
            .update(cx, |editor, cx| editor.query_suggestion(cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.activate_match(index, matches, cx);
        });
    }

    fn select_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.select_matches(matches, cx));
    }

    fn replace(
        &mut self,
        identifier: &Self::Match,
        query: &project::search::SearchQuery,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor
            .update(cx, |editor, cx| editor.replace(identifier, query, cx));
    }

    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Self::Match>> {
        self.editor
            .update(cx, |editor, cx| editor.find_matches(query, cx))
    }

    fn active_match_index(
        &mut self,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        self.editor
            .update(cx, |editor, cx| editor.active_match_index(matches, cx))
    }
}

impl FollowableItem for ContextEditor {
    fn remote_id(&self) -> Option<workspace::ViewId> {
        self.remote_id
    }

    fn to_state_proto(&self, cx: &WindowContext) -> Option<proto::view::Variant> {
        let context = self.context.read(cx);
        Some(proto::view::Variant::ContextEditor(
            proto::view::ContextEditor {
                context_id: context.id().to_proto(),
                editor: if let Some(proto::view::Variant::Editor(proto)) =
                    self.editor.read(cx).to_state_proto(cx)
                {
                    Some(proto)
                } else {
                    None
                },
            },
        ))
    }

    fn from_state_proto(
        workspace: View<Workspace>,
        id: workspace::ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut WindowContext,
    ) -> Option<Task<Result<View<Self>>>> {
        let proto::view::Variant::ContextEditor(_) = state.as_ref()? else {
            return None;
        };
        let Some(proto::view::Variant::ContextEditor(state)) = state.take() else {
            unreachable!()
        };

        let context_id = ContextId::from_proto(state.context_id);
        let editor_state = state.editor?;

        let (project, panel) = workspace.update(cx, |workspace, cx| {
            Some((
                workspace.project().clone(),
                workspace.panel::<AssistantPanel>(cx)?,
            ))
        })?;

        let context_editor =
            panel.update(cx, |panel, cx| panel.open_remote_context(context_id, cx));

        Some(cx.spawn(|mut cx| async move {
            let context_editor = context_editor.await?;
            context_editor
                .update(&mut cx, |context_editor, cx| {
                    context_editor.remote_id = Some(id);
                    context_editor.editor.update(cx, |editor, cx| {
                        editor.apply_update_proto(
                            &project,
                            proto::update_view::Variant::Editor(proto::update_view::Editor {
                                selections: editor_state.selections,
                                pending_selection: editor_state.pending_selection,
                                scroll_top_anchor: editor_state.scroll_top_anchor,
                                scroll_x: editor_state.scroll_y,
                                scroll_y: editor_state.scroll_y,
                                ..Default::default()
                            }),
                            cx,
                        )
                    })
                })?
                .await?;
            Ok(context_editor)
        }))
    }

    fn to_follow_event(event: &Self::Event) -> Option<item::FollowEvent> {
        Editor::to_follow_event(event)
    }

    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        cx: &WindowContext,
    ) -> bool {
        self.editor
            .read(cx)
            .add_event_to_update_proto(event, update, cx)
    }

    fn apply_update_proto(
        &mut self,
        project: &Model<Project>,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.editor.update(cx, |editor, cx| {
            editor.apply_update_proto(project, message, cx)
        })
    }

    fn is_project_item(&self, _cx: &WindowContext) -> bool {
        true
    }

    fn set_leader_peer_id(
        &mut self,
        leader_peer_id: Option<proto::PeerId>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_leader_peer_id(leader_peer_id, cx)
        })
    }

    fn dedup(&self, existing: &Self, cx: &WindowContext) -> Option<item::Dedup> {
        if existing.context.read(cx).id() == self.context.read(cx).id() {
            Some(item::Dedup::KeepExisting)
        } else {
            None
        }
    }
}

pub struct ContextEditorToolbarItem {
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    active_context_editor: Option<WeakView<ContextEditor>>,
    model_summary_editor: View<Editor>,
    model_selector_menu_handle: PopoverMenuHandle<Picker<ModelPickerDelegate>>,
}

fn active_editor_focus_handle(
    workspace: &WeakView<Workspace>,
    cx: &WindowContext<'_>,
) -> Option<FocusHandle> {
    workspace.upgrade().and_then(|workspace| {
        Some(
            workspace
                .read(cx)
                .active_item_as::<Editor>(cx)?
                .focus_handle(cx),
        )
    })
}

fn render_inject_context_menu(
    active_context_editor: WeakView<ContextEditor>,
    cx: &mut WindowContext<'_>,
) -> impl IntoElement {
    let commands = SlashCommandRegistry::global(cx);

    slash_command_picker::SlashCommandSelector::new(
        commands.clone(),
        active_context_editor,
        IconButton::new("trigger", IconName::SlashSquare)
            .icon_size(IconSize::Small)
            .tooltip(|cx| {
                Tooltip::with_meta("Insert Context", None, "Type / to insert via keyboard", cx)
            }),
    )
}

impl ContextEditorToolbarItem {
    pub fn new(
        workspace: &Workspace,
        model_selector_menu_handle: PopoverMenuHandle<Picker<ModelPickerDelegate>>,
        model_summary_editor: View<Editor>,
    ) -> Self {
        Self {
            fs: workspace.app_state().fs.clone(),
            workspace: workspace.weak_handle(),
            active_context_editor: None,
            model_summary_editor,
            model_selector_menu_handle,
        }
    }

    fn render_remaining_tokens(&self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let context = &self
            .active_context_editor
            .as_ref()?
            .upgrade()?
            .read(cx)
            .context;
        let (token_count_color, token_count, max_token_count) = match token_state(context, cx)? {
            TokenState::NoTokensLeft {
                max_token_count,
                token_count,
            } => (Color::Error, token_count, max_token_count),
            TokenState::HasMoreTokens {
                max_token_count,
                token_count,
                over_warn_threshold,
            } => {
                let color = if over_warn_threshold {
                    Color::Warning
                } else {
                    Color::Muted
                };
                (color, token_count, max_token_count)
            }
        };
        Some(
            h_flex()
                .gap_0p5()
                .child(
                    Label::new(humanize_token_count(token_count))
                        .size(LabelSize::Small)
                        .color(token_count_color),
                )
                .child(Label::new("/").size(LabelSize::Small).color(Color::Muted))
                .child(
                    Label::new(humanize_token_count(max_token_count))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }
}

impl Render for ContextEditorToolbarItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let left_side = h_flex()
            .pl_1()
            .gap_2()
            .flex_1()
            .min_w(rems(DEFAULT_TAB_TITLE.len() as f32))
            .when(self.active_context_editor.is_some(), |left_side| {
                left_side.child(self.model_summary_editor.clone())
            });
        let active_provider = LanguageModelRegistry::read_global(cx).active_provider();
        let active_model = LanguageModelRegistry::read_global(cx).active_model();
        let weak_self = cx.view().downgrade();
        let right_side = h_flex()
            .gap_2()
            .child(
                ModelSelector::new(
                    self.fs.clone(),
                    ButtonLike::new("active-model")
                        .style(ButtonStyle::Subtle)
                        .child(
                            h_flex()
                                .w_full()
                                .gap_0p5()
                                .child(
                                    div()
                                        .overflow_x_hidden()
                                        .flex_grow()
                                        .whitespace_nowrap()
                                        .child(match (active_provider, active_model) {
                                            (Some(provider), Some(model)) => h_flex()
                                                .gap_1()
                                                .child(
                                                    Icon::new(model.icon().unwrap_or_else(|| provider.icon()))
                                                        .color(Color::Muted)
                                                        .size(IconSize::XSmall),
                                                )
                                                .child(
                                                    Label::new(model.name().0)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                )
                                                .into_any_element(),
                                            _ => Label::new("No model selected")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted)
                                                .into_any_element(),
                                        }),
                                )
                                .child(
                                    Icon::new(IconName::ChevronDown)
                                        .color(Color::Muted)
                                        .size(IconSize::XSmall),
                                ),
                        )
                        .tooltip(move |cx| {
                            Tooltip::for_action("Change Model", &ToggleModelSelector, cx)
                        }),
                )
                .with_handle(self.model_selector_menu_handle.clone()),
            )
            .children(self.render_remaining_tokens(cx))
            .child(
                PopoverMenu::new("context-editor-popover")
                    .trigger(
                        IconButton::new("context-editor-trigger", IconName::EllipsisVertical)
                            .icon_size(IconSize::Small)
                            .tooltip(|cx| Tooltip::text("Open Context Options", cx)),
                    )
                    .menu({
                        let weak_self = weak_self.clone();
                        move |cx| {
                            let weak_self = weak_self.clone();
                            Some(ContextMenu::build(cx, move |menu, cx| {
                                let context = weak_self
                                    .update(cx, |this, cx| {
                                        active_editor_focus_handle(&this.workspace, cx)
                                    })
                                    .ok()
                                    .flatten();
                                menu.when_some(context, |menu, context| menu.context(context))
                                    .entry("Regenerate Context Title", None, {
                                        let weak_self = weak_self.clone();
                                        move |cx| {
                                            weak_self
                                                .update(cx, |_, cx| {
                                                    cx.emit(ContextEditorToolbarItemEvent::RegenerateSummary)
                                                })
                                                .ok();
                                        }
                                    })
                                    .custom_entry(
                                        |_| {
                                            h_flex()
                                                .w_full()
                                                .justify_between()
                                                .gap_2()
                                                .child(Label::new("Insert Context"))
                                                .child(Label::new("/ command").color(Color::Muted))
                                                .into_any()
                                        },
                                        {
                                            let weak_self = weak_self.clone();
                                            move |cx| {
                                                weak_self
                                                    .update(cx, |this, cx| {
                                                        if let Some(editor) =
                                                        &this.active_context_editor
                                                        {
                                                            editor
                                                                .update(cx, |this, cx| {
                                                                    this.slash_menu_handle
                                                                        .toggle(cx);
                                                                })
                                                                .ok();
                                                        }
                                                    })
                                                    .ok();
                                            }
                                        },
                                    )
                                    .action("Insert Selection", QuoteSelection.boxed_clone())
                            }))
                        }
                    }),
            );

        h_flex()
            .size_full()
            .gap_2()
            .justify_between()
            .child(left_side)
            .child(right_side)
    }
}

impl ToolbarItemView for ContextEditorToolbarItem {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        self.active_context_editor = active_pane_item
            .and_then(|item| item.act_as::<ContextEditor>(cx))
            .map(|editor| editor.downgrade());
        cx.notify();
        if self.active_context_editor.is_none() {
            ToolbarItemLocation::Hidden
        } else {
            ToolbarItemLocation::PrimaryRight
        }
    }

    fn pane_focus_update(&mut self, _pane_focused: bool, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}

impl EventEmitter<ToolbarItemEvent> for ContextEditorToolbarItem {}

enum ContextEditorToolbarItemEvent {
    RegenerateSummary,
}
impl EventEmitter<ContextEditorToolbarItemEvent> for ContextEditorToolbarItem {}

pub struct ContextHistory {
    picker: View<Picker<SavedContextPickerDelegate>>,
    _subscriptions: Vec<Subscription>,
    assistant_panel: WeakView<AssistantPanel>,
}

impl ContextHistory {
    fn new(
        project: Model<Project>,
        context_store: Model<ContextStore>,
        assistant_panel: WeakView<AssistantPanel>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(
                SavedContextPickerDelegate::new(project, context_store.clone()),
                cx,
            )
            .modal(false)
            .max_height(None)
        });

        let _subscriptions = vec![
            cx.observe(&context_store, |this, _, cx| {
                this.picker.update(cx, |picker, cx| picker.refresh(cx));
            }),
            cx.subscribe(&picker, Self::handle_picker_event),
        ];

        Self {
            picker,
            _subscriptions,
            assistant_panel,
        }
    }

    fn handle_picker_event(
        &mut self,
        _: View<Picker<SavedContextPickerDelegate>>,
        event: &SavedContextPickerEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let SavedContextPickerEvent::Confirmed(context) = event;
        self.assistant_panel
            .update(cx, |assistant_panel, cx| match context {
                ContextMetadata::Remote(metadata) => {
                    assistant_panel
                        .open_remote_context(metadata.id.clone(), cx)
                        .detach_and_log_err(cx);
                }
                ContextMetadata::Saved(metadata) => {
                    assistant_panel
                        .open_saved_context(metadata.path.clone(), cx)
                        .detach_and_log_err(cx);
                }
            })
            .ok();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WorkflowAssistStatus {
    Pending,
    Confirmed,
    Done,
    Idle,
}

impl WorkflowAssist {
    pub fn status(&self, cx: &AppContext) -> WorkflowAssistStatus {
        let assistant = InlineAssistant::global(cx);
        if self
            .assist_ids
            .iter()
            .any(|assist_id| assistant.assist_status(*assist_id, cx).is_pending())
        {
            WorkflowAssistStatus::Pending
        } else if self
            .assist_ids
            .iter()
            .all(|assist_id| assistant.assist_status(*assist_id, cx).is_confirmed())
        {
            WorkflowAssistStatus::Confirmed
        } else if self
            .assist_ids
            .iter()
            .all(|assist_id| assistant.assist_status(*assist_id, cx).is_done())
        {
            WorkflowAssistStatus::Done
        } else {
            WorkflowAssistStatus::Idle
        }
    }
}

impl Render for ContextHistory {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child(self.picker.clone())
    }
}

impl FocusableView for ContextHistory {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<()> for ContextHistory {}

impl Item for ContextHistory {
    type Event = ();

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("History".into())
    }
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
                                    "Open new context",
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
                    .p(Spacing::Large.rems(cx))
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
            .track_focus(&self.focus_handle)
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .overflow_y_scroll()
            .child(
                v_flex()
                    .p(Spacing::XXLarge.rems(cx))
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
                    .p(Spacing::XXLarge.rems(cx))
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

type ToggleFold = Arc<dyn Fn(bool, &mut WindowContext) + Send + Sync>;

fn render_slash_command_output_toggle(
    row: MultiBufferRow,
    is_folded: bool,
    fold: ToggleFold,
    _cx: &mut WindowContext,
) -> AnyElement {
    Disclosure::new(
        ("slash-command-output-fold-indicator", row.0 as u64),
        !is_folded,
    )
    .selected(is_folded)
    .on_click(move |_e, cx| fold(!is_folded, cx))
    .into_any_element()
}

fn fold_toggle(
    name: &'static str,
) -> impl Fn(
    MultiBufferRow,
    bool,
    Arc<dyn Fn(bool, &mut WindowContext<'_>) + Send + Sync>,
    &mut WindowContext<'_>,
) -> AnyElement {
    move |row, is_folded, fold, _cx| {
        Disclosure::new((name, row.0 as u64), !is_folded)
            .selected(is_folded)
            .on_click(move |_e, cx| fold(!is_folded, cx))
            .into_any_element()
    }
}

fn quote_selection_fold_placeholder(title: String, editor: WeakView<Editor>) -> FoldPlaceholder {
    FoldPlaceholder {
        render: Arc::new({
            move |fold_id, fold_range, _cx| {
                let editor = editor.clone();
                ButtonLike::new(fold_id)
                    .style(ButtonStyle::Filled)
                    .layer(ElevationIndex::ElevatedSurface)
                    .child(Icon::new(IconName::TextSelect))
                    .child(Label::new(title.clone()).single_line())
                    .on_click(move |_, cx| {
                        editor
                            .update(cx, |editor, cx| {
                                let buffer_start = fold_range
                                    .start
                                    .to_point(&editor.buffer().read(cx).read(cx));
                                let buffer_row = MultiBufferRow(buffer_start.row);
                                editor.unfold_at(&UnfoldAt { buffer_row }, cx);
                            })
                            .ok();
                    })
                    .into_any_element()
            }
        }),
        constrain_width: false,
        merge_adjacent: false,
    }
}

fn render_quote_selection_output_toggle(
    row: MultiBufferRow,
    is_folded: bool,
    fold: ToggleFold,
    _cx: &mut WindowContext,
) -> AnyElement {
    Disclosure::new(("quote-selection-indicator", row.0 as u64), !is_folded)
        .selected(is_folded)
        .on_click(move |_e, cx| fold(!is_folded, cx))
        .into_any_element()
}

fn render_pending_slash_command_gutter_decoration(
    row: MultiBufferRow,
    status: &PendingSlashCommandStatus,
    confirm_command: Arc<dyn Fn(&mut WindowContext)>,
) -> AnyElement {
    let mut icon = IconButton::new(
        ("slash-command-gutter-decoration", row.0),
        ui::IconName::TriangleRight,
    )
    .on_click(move |_e, cx| confirm_command(cx))
    .icon_size(ui::IconSize::Small)
    .size(ui::ButtonSize::None);

    match status {
        PendingSlashCommandStatus::Idle => {
            icon = icon.icon_color(Color::Muted);
        }
        PendingSlashCommandStatus::Running { .. } => {
            icon = icon.selected(true);
        }
        PendingSlashCommandStatus::Error(_) => icon = icon.icon_color(Color::Error),
    }

    icon.into_any_element()
}

fn render_docs_slash_command_trailer(
    row: MultiBufferRow,
    command: PendingSlashCommand,
    cx: &mut WindowContext,
) -> AnyElement {
    if command.arguments.is_empty() {
        return Empty.into_any();
    }
    let args = DocsSlashCommandArgs::parse(&command.arguments);

    let Some(store) = args
        .provider()
        .and_then(|provider| IndexedDocsStore::try_global(provider, cx).ok())
    else {
        return Empty.into_any();
    };

    let Some(package) = args.package() else {
        return Empty.into_any();
    };

    let mut children = Vec::new();

    if store.is_indexing(&package) {
        children.push(
            div()
                .id(("crates-being-indexed", row.0))
                .child(Icon::new(IconName::ArrowCircle).with_animation(
                    "arrow-circle",
                    Animation::new(Duration::from_secs(4)).repeat(),
                    |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                ))
                .tooltip({
                    let package = package.clone();
                    move |cx| Tooltip::text(format!("Indexing {package}…"), cx)
                })
                .into_any_element(),
        );
    }

    if let Some(latest_error) = store.latest_error_for_package(&package) {
        children.push(
            div()
                .id(("latest-error", row.0))
                .child(
                    Icon::new(IconName::ExclamationTriangle)
                        .size(IconSize::Small)
                        .color(Color::Warning),
                )
                .tooltip(move |cx| Tooltip::text(format!("Failed to index: {latest_error}"), cx))
                .into_any_element(),
        )
    }

    let is_indexing = store.is_indexing(&package);
    let latest_error = store.latest_error_for_package(&package);

    if !is_indexing && latest_error.is_none() {
        return Empty.into_any();
    }

    h_flex().gap_2().children(children).into_any_element()
}

fn make_lsp_adapter_delegate(
    project: &Model<Project>,
    cx: &mut AppContext,
) -> Result<Arc<dyn LspAdapterDelegate>> {
    project.update(cx, |project, cx| {
        // TODO: Find the right worktree.
        let worktree = project
            .worktrees(cx)
            .next()
            .ok_or_else(|| anyhow!("no worktrees when constructing ProjectLspAdapterDelegate"))?;
        project.lsp_store().update(cx, |lsp_store, cx| {
            Ok(ProjectLspAdapterDelegate::new(lsp_store, &worktree, cx)
                as Arc<dyn LspAdapterDelegate>)
        })
    })
}

fn slash_command_error_block_renderer(message: String) -> RenderBlock {
    Box::new(move |_| {
        div()
            .pl_6()
            .child(
                Label::new(format!("error: {}", message))
                    .single_line()
                    .color(Color::Error),
            )
            .into_any()
    })
}

enum TokenState {
    NoTokensLeft {
        max_token_count: usize,
        token_count: usize,
    },
    HasMoreTokens {
        max_token_count: usize,
        token_count: usize,
        over_warn_threshold: bool,
    },
}

fn token_state(context: &Model<Context>, cx: &AppContext) -> Option<TokenState> {
    const WARNING_TOKEN_THRESHOLD: f32 = 0.8;

    let model = LanguageModelRegistry::read_global(cx).active_model()?;
    let token_count = context.read(cx).token_count()?;
    let max_token_count = model.max_token_count();

    let remaining_tokens = max_token_count as isize - token_count as isize;
    let token_state = if remaining_tokens <= 0 {
        TokenState::NoTokensLeft {
            max_token_count,
            token_count,
        }
    } else {
        let over_warn_threshold =
            token_count as f32 / max_token_count as f32 >= WARNING_TOKEN_THRESHOLD;
        TokenState::HasMoreTokens {
            max_token_count,
            token_count,
            over_warn_threshold,
        }
    };
    Some(token_state)
}

fn size_for_image(data: &RenderImage, max_size: Size<Pixels>) -> Size<Pixels> {
    let image_size = data
        .size(0)
        .map(|dimension| Pixels::from(u32::from(dimension)));
    let image_ratio = image_size.width / image_size.height;
    let bounds_ratio = max_size.width / max_size.height;

    if image_size.width > max_size.width || image_size.height > max_size.height {
        if bounds_ratio > image_ratio {
            size(
                image_size.width * (max_size.height / image_size.height),
                max_size.height,
            )
        } else {
            size(
                max_size.width,
                image_size.height * (max_size.width / image_size.width),
            )
        }
    } else {
        size(image_size.width, image_size.height)
    }
}

enum ConfigurationError {
    NoProvider,
    ProviderNotAuthenticated,
}

fn configuration_error(cx: &AppContext) -> Option<ConfigurationError> {
    let provider = LanguageModelRegistry::read_global(cx).active_provider();
    let is_authenticated = provider
        .as_ref()
        .map_or(false, |provider| provider.is_authenticated(cx));

    if provider.is_some() && is_authenticated {
        return None;
    }

    if provider.is_none() {
        return Some(ConfigurationError::NoProvider);
    }

    if !is_authenticated {
        return Some(ConfigurationError::ProviderNotAuthenticated);
    }

    None
}
