use crate::{
    assistant_settings::{AssistantDockPosition, AssistantSettings},
    humanize_token_count,
    prompt_library::open_prompt_library,
    prompts::PromptBuilder,
    slash_command::{
        default_command::DefaultSlashCommand,
        docs_command::{DocsSlashCommand, DocsSlashCommandArgs},
        file_command::{self, codeblock_fence_for_path},
        SlashCommandCompletionProvider, SlashCommandRegistry,
    },
    slash_command_picker,
    terminal_inline_assistant::TerminalInlineAssistant,
    Assist, AssistantPatch, AssistantPatchStatus, CacheStatus, ConfirmCommand, Content, Context,
    ContextEvent, ContextId, ContextStore, ContextStoreEvent, CopyCode, CycleMessageRole,
    DeployHistory, DeployPromptLibrary, InlineAssistant, InsertDraggedFiles, InsertIntoEditor,
    Message, MessageId, MessageMetadata, MessageStatus, ModelPickerDelegate, ModelSelector,
    NewContext, PendingSlashCommand, PendingSlashCommandStatus, QuoteSelection,
    RemoteContextMetadata, SavedContextMetadata, Split, ToggleFocus, ToggleModelSelector,
};
use anyhow::Result;
use assistant_slash_command::{SlashCommand, SlashCommandOutputSection};
use assistant_tool::ToolRegistry;
use client::{proto, zed_urls, Client, Status};
use collections::{BTreeSet, HashMap, HashSet};
use editor::{
    actions::{FoldAt, MoveToEndOfLine, Newline, ShowCompletions, UnfoldAt},
    display_map::{
        BlockContext, BlockId, BlockPlacement, BlockProperties, BlockStyle, Crease, CreaseMetadata,
        CustomBlockId, FoldId, RenderBlock, ToDisplayPoint,
    },
    scroll::{Autoscroll, AutoscrollStrategy},
    Anchor, Editor, EditorEvent, ProposedChangeLocation, ProposedChangesEditor, RowExt,
    ToOffset as _, ToPoint,
};
use editor::{display_map::CreaseId, FoldPlaceholder};
use fs::Fs;
use futures::FutureExt;
use gpui::{
    canvas, div, img, percentage, point, pulsating_between, size, Action, Animation, AnimationExt,
    AnyElement, AnyView, AppContext, AsyncWindowContext, ClipboardEntry, ClipboardItem,
    CursorStyle, Empty, Entity, EventEmitter, ExternalPaths, FocusHandle, FocusableView,
    FontWeight, InteractiveElement, IntoElement, Model, ParentElement, Pixels, Render, RenderImage,
    SharedString, Size, StatefulInteractiveElement, Styled, Subscription, Task, Transformation,
    UpdateGlobal, View, VisualContext, WeakView, WindowContext,
};
use indexed_docs::IndexedDocsStore;
use language::{
    language_settings::SoftWrap, BufferSnapshot, LanguageRegistry, LspAdapterDelegate, ToOffset,
};
use language_model::{
    provider::cloud::PROVIDER_ID, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelRegistry, Role,
};
use language_model::{LanguageModelImage, LanguageModelToolUse};
use multi_buffer::MultiBufferRow;
use picker::{Picker, PickerDelegate};
use project::lsp_store::LocalLspAdapterDelegate;
use project::{Project, Worktree};
use rope::Point;
use search::{buffer_search::DivRegistrar, BufferSearchBar};
use serde::{Deserialize, Serialize};
use settings::{update_settings_file, Settings};
use smol::stream::StreamExt;
use std::{
    borrow::Cow,
    cmp,
    ops::{ControlFlow, Range},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use text::SelectionGoal;
use ui::TintColor;
use ui::{
    prelude::*,
    utils::{format_distance_from_now, DateTimeType},
    Avatar, ButtonLike, ContextMenu, Disclosure, ElevationIndex, KeyBinding, ListItem,
    ListItemSpacing, PopoverMenu, PopoverMenuHandle, Tooltip,
};
use util::{maybe, ResultExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::{self, FollowableItem, Item, ItemHandle},
    notifications::NotificationId,
    pane::{self, SaveIntent},
    searchable::{SearchEvent, SearchableItem},
    DraggedSelection, Pane, Save, ShowConfiguration, Toast, ToggleZoom, ToolbarItemEvent,
    ToolbarItemLocation, ToolbarItemView, Workspace,
};
use workspace::{searchable::SearchableItemHandle, DraggedTab};
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
                .register_action(ContextEditor::copy_code)
                .register_action(ContextEditor::insert_dragged_files)
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
                                    Avatar::new(host_user.avatar_uri.clone()).into_any_element(),
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
        let model_summary_editor = cx.new_view(Editor::single_line);
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
                            .tooltip(move |cx| {
                                Tooltip::for_action_in(
                                    "New Context",
                                    &NewContext,
                                    &focus_handle,
                                    cx,
                                )
                            }),
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

    fn new_context(&mut self, cx: &mut ViewContext<Self>) -> Option<View<ContextEditor>> {
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

struct PatchViewState {
    footer_block_id: CustomBlockId,
    crease_id: CreaseId,
    editor: Option<PatchEditorState>,
    update_task: Option<Task<()>>,
}

struct PatchEditorState {
    editor: WeakView<ProposedChangesEditor>,
    opened_patch: AssistantPatch,
}

type MessageHeader = MessageMetadata;

#[derive(Clone)]
enum AssistError {
    PaymentRequired,
    MaxMonthlySpendReached,
    Message(SharedString),
}

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
    pending_tool_use_creases: HashMap<Range<language::Anchor>, CreaseId>,
    _subscriptions: Vec<Subscription>,
    patches: HashMap<Range<language::Anchor>, PatchViewState>,
    active_patch: Option<Range<language::Anchor>>,
    assistant_panel: WeakView<AssistantPanel>,
    last_error: Option<AssistError>,
    show_accept_terms: bool,
    pub(crate) slash_menu_handle:
        PopoverMenuHandle<Picker<slash_command_picker::SlashCommandDelegate>>,
    // dragged_file_worktrees is used to keep references to worktrees that were added
    // when the user drag/dropped an external file onto the context editor. Since
    // the worktree is not part of the project panel, it would be dropped as soon as
    // the file is opened. In order to keep the worktree alive for the duration of the
    // context editor, we keep a reference here.
    dragged_file_worktrees: Vec<Model<Worktree>>,
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
            editor.set_completion_provider(Some(Box::new(completion_provider)));
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
        let patch_ranges = context.read(cx).patch_ranges().collect::<Vec<_>>();
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
            pending_tool_use_creases: HashMap::default(),
            _subscriptions,
            patches: HashMap::default(),
            active_patch: None,
            assistant_panel,
            last_error: None,
            show_accept_terms: false,
            slash_menu_handle: Default::default(),
            dragged_file_worktrees: Vec::new(),
        };
        this.update_message_headers(cx);
        this.update_image_blocks(cx);
        this.insert_slash_command_output_sections(sections, false, cx);
        this.patches_updated(&Vec::new(), &patch_ranges, cx);
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

        if self.focus_active_patch(cx) {
            return;
        }

        self.last_error = None;
        self.send_to_model(cx);
        cx.notify();
    }

    fn focus_active_patch(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if let Some((_range, patch)) = self.active_patch() {
            if let Some(editor) = patch
                .editor
                .as_ref()
                .and_then(|state| state.editor.upgrade())
            {
                cx.focus_view(&editor);
                return true;
            }
        }

        false
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
        self.last_error = None;

        if self
            .context
            .update(cx, |context, cx| context.cancel_last_assist(cx))
        {
            return;
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
            let context = self.context.read(cx);
            let sections = context
                .slash_command_output_sections()
                .into_iter()
                .filter(|section| section.is_valid(context.buffer().read(cx)))
                .cloned()
                .collect::<Vec<_>>();
            let snapshot = context.buffer().read(cx).snapshot();
            let output = command.run(
                arguments,
                &sections,
                snapshot,
                workspace,
                self.lsp_adapter_delegate.clone(),
                cx,
            );
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

                    let new_tool_uses = self
                        .context
                        .read(cx)
                        .pending_tool_uses()
                        .into_iter()
                        .filter(|tool_use| {
                            !self
                                .pending_tool_use_creases
                                .contains_key(&tool_use.source_range)
                        })
                        .cloned()
                        .collect::<Vec<_>>();

                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let (excerpt_id, _buffer_id, _) = buffer.as_singleton().unwrap();
                    let excerpt_id = *excerpt_id;

                    let mut buffer_rows_to_fold = BTreeSet::new();

                    let creases = new_tool_uses
                        .iter()
                        .map(|tool_use| {
                            let placeholder = FoldPlaceholder {
                                render: render_fold_icon_button(
                                    cx.view().downgrade(),
                                    IconName::PocketKnife,
                                    tool_use.name.clone().into(),
                                ),
                                constrain_width: false,
                                merge_adjacent: false,
                            };
                            let render_trailer =
                                move |_row, _unfold, _cx: &mut WindowContext| Empty.into_any();

                            let start = buffer
                                .anchor_in_excerpt(excerpt_id, tool_use.source_range.start)
                                .unwrap();
                            let end = buffer
                                .anchor_in_excerpt(excerpt_id, tool_use.source_range.end)
                                .unwrap();

                            let buffer_row = MultiBufferRow(start.to_point(&buffer).row);
                            buffer_rows_to_fold.insert(buffer_row);

                            self.context.update(cx, |context, cx| {
                                context.insert_content(
                                    Content::ToolUse {
                                        range: tool_use.source_range.clone(),
                                        tool_use: LanguageModelToolUse {
                                            id: tool_use.id.to_string(),
                                            name: tool_use.name.clone(),
                                            input: tool_use.input.clone(),
                                        },
                                    },
                                    cx,
                                );
                            });

                            Crease::new(
                                start..end,
                                placeholder,
                                fold_toggle("tool-use"),
                                render_trailer,
                            )
                        })
                        .collect::<Vec<_>>();

                    let crease_ids = editor.insert_creases(creases, cx);

                    for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                        editor.fold_at(&FoldAt { buffer_row }, cx);
                    }

                    self.pending_tool_use_creases.extend(
                        new_tool_uses
                            .iter()
                            .map(|tool_use| tool_use.source_range.clone())
                            .zip(crease_ids),
                    );
                });
            }
            ContextEvent::PatchesUpdated { removed, updated } => {
                self.patches_updated(removed, updated, cx);
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
                                height: 1,
                                placement: BlockPlacement::Below(Anchor {
                                    buffer_id: Some(buffer_id),
                                    excerpt_id,
                                    text_anchor: command.source_range.start,
                                }),
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
            ContextEvent::UsePendingTools => {
                let pending_tool_uses = self
                    .context
                    .read(cx)
                    .pending_tool_uses()
                    .into_iter()
                    .filter(|tool_use| tool_use.status.is_idle())
                    .cloned()
                    .collect::<Vec<_>>();

                for tool_use in pending_tool_uses {
                    let tool_registry = ToolRegistry::global(cx);
                    if let Some(tool) = tool_registry.tool(&tool_use.name) {
                        let task = tool.run(tool_use.input, self.workspace.clone(), cx);

                        self.context.update(cx, |context, cx| {
                            context.insert_tool_output(tool_use.id.clone(), task, cx);
                        });
                    }
                }
            }
            ContextEvent::ToolFinished {
                tool_use_id,
                output_range,
            } => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let (excerpt_id, _buffer_id, _) = buffer.as_singleton().unwrap();
                    let excerpt_id = *excerpt_id;

                    let placeholder = FoldPlaceholder {
                        render: render_fold_icon_button(
                            cx.view().downgrade(),
                            IconName::PocketKnife,
                            format!("Tool Result: {tool_use_id}").into(),
                        ),
                        constrain_width: false,
                        merge_adjacent: false,
                    };
                    let render_trailer =
                        move |_row, _unfold, _cx: &mut WindowContext| Empty.into_any();

                    let start = buffer
                        .anchor_in_excerpt(excerpt_id, output_range.start)
                        .unwrap();
                    let end = buffer
                        .anchor_in_excerpt(excerpt_id, output_range.end)
                        .unwrap();

                    let buffer_row = MultiBufferRow(start.to_point(&buffer).row);

                    let crease = Crease::new(
                        start..end,
                        placeholder,
                        fold_toggle("tool-use"),
                        render_trailer,
                    );

                    editor.insert_creases([crease], cx);
                    editor.fold_at(&FoldAt { buffer_row }, cx);
                });
            }
            ContextEvent::Operation(_) => {}
            ContextEvent::ShowAssistError(error_message) => {
                self.last_error = Some(AssistError::Message(error_message.clone()));
            }
            ContextEvent::ShowPaymentRequiredError => {
                self.last_error = Some(AssistError::PaymentRequired);
            }
            ContextEvent::ShowMaxMonthlySpendReachedError => {
                self.last_error = Some(AssistError::MaxMonthlySpendReached);
            }
        }
    }

    fn patches_updated(
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
            if let Some(state) = self.patches.remove(range) {
                editors_to_close.extend(state.editor.and_then(|state| state.editor.upgrade()));
                removed_block_ids.insert(state.footer_block_id);
                removed_crease_ids.push(state.crease_id);
            }
        }

        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let multibuffer = &snapshot.buffer_snapshot;
            let (&excerpt_id, _, _) = multibuffer.as_singleton().unwrap();

            let mut replaced_blocks = HashMap::default();
            for range in updated {
                let Some(patch) = self.context.read(cx).patch_for_range(&range, cx).cloned() else {
                    continue;
                };

                let path_count = patch.path_count();
                let patch_start = multibuffer
                    .anchor_in_excerpt(excerpt_id, patch.range.start)
                    .unwrap();
                let patch_end = multibuffer
                    .anchor_in_excerpt(excerpt_id, patch.range.end)
                    .unwrap();
                let render_block: RenderBlock = Box::new({
                    let this = this.clone();
                    let patch_range = range.clone();
                    move |cx: &mut BlockContext<'_, '_>| {
                        let max_width = cx.max_width;
                        let gutter_width = cx.gutter_dimensions.full_width();
                        let block_id = cx.block_id;
                        this.update(&mut **cx, |this, cx| {
                            this.render_patch_footer(
                                patch_range.clone(),
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
                });

                let header_placeholder = FoldPlaceholder {
                    render: {
                        let this = this.clone();
                        let patch_range = range.clone();
                        Arc::new(move |fold_id, _range, cx| {
                            this.update(cx, |this, cx| {
                                this.render_patch_header(patch_range.clone(), fold_id, cx)
                            })
                            .ok()
                            .flatten()
                            .unwrap_or_else(|| Empty.into_any())
                        })
                    },
                    constrain_width: false,
                    merge_adjacent: false,
                };

                let should_refold;
                if let Some(state) = self.patches.get_mut(&range) {
                    replaced_blocks.insert(state.footer_block_id, render_block);
                    if let Some(editor_state) = &state.editor {
                        if editor_state.opened_patch != patch {
                            state.update_task = Some({
                                let this = this.clone();
                                cx.spawn(|_, cx| async move {
                                    Self::update_patch_editor(this.clone(), patch, cx)
                                        .await
                                        .log_err();
                                })
                            });
                        }
                    }

                    should_refold =
                        snapshot.intersects_fold(patch_start.to_offset(&snapshot.buffer_snapshot));
                } else {
                    let block_ids = editor.insert_blocks(
                        [BlockProperties {
                            height: path_count as u32 + 1,
                            style: BlockStyle::Flex,
                            render: render_block,
                            placement: BlockPlacement::Below(patch_start),
                            priority: 0,
                        }],
                        None,
                        cx,
                    );

                    let new_crease_ids = editor.insert_creases(
                        [Crease::new(
                            patch_start..patch_end,
                            header_placeholder.clone(),
                            fold_toggle("patch-header"),
                            |_, _, _| Empty.into_any_element(),
                        )],
                        cx,
                    );

                    self.patches.insert(
                        range.clone(),
                        PatchViewState {
                            footer_block_id: block_ids[0],
                            crease_id: new_crease_ids[0],
                            editor: None,
                            update_task: None,
                        },
                    );

                    should_refold = true;
                }

                if should_refold {
                    editor.unfold_ranges([patch_start..patch_end], true, false, cx);
                    editor.fold_ranges([(patch_start..patch_end, header_placeholder)], false, cx);
                }
            }

            editor.remove_creases(removed_crease_ids, cx);
            editor.remove_blocks(removed_block_ids, None, cx);
            editor.replace_blocks(replaced_blocks, None, cx);
        });

        for editor in editors_to_close {
            self.close_patch_editor(editor, cx);
        }

        self.update_active_patch(cx);
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
                creases.push(
                    Crease::new(
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
                    )
                    .with_metadata(CreaseMetadata {
                        icon: section.icon,
                        label: section.label,
                    }),
                );
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
                self.update_active_patch(cx);
            }
            _ => {}
        }
        cx.emit(event.clone());
    }

    fn active_patch(&self) -> Option<(Range<text::Anchor>, &PatchViewState)> {
        let patch = self.active_patch.as_ref()?;
        Some((patch.clone(), self.patches.get(&patch)?))
    }

    fn update_active_patch(&mut self, cx: &mut ViewContext<Self>) {
        let newest_cursor = self.editor.read(cx).selections.newest::<Point>(cx).head();
        let context = self.context.read(cx);

        let new_patch = context.patch_containing(newest_cursor, cx).cloned();

        if new_patch.as_ref().map(|p| &p.range) == self.active_patch.as_ref() {
            return;
        }

        if let Some(old_patch_range) = self.active_patch.take() {
            if let Some(patch_state) = self.patches.get_mut(&old_patch_range) {
                if let Some(state) = patch_state.editor.take() {
                    if let Some(editor) = state.editor.upgrade() {
                        self.close_patch_editor(editor, cx);
                    }
                }
            }
        }

        if let Some(new_patch) = new_patch {
            self.active_patch = Some(new_patch.range.clone());

            if let Some(patch_state) = self.patches.get_mut(&new_patch.range) {
                let mut editor = None;
                if let Some(state) = &patch_state.editor {
                    if let Some(opened_editor) = state.editor.upgrade() {
                        editor = Some(opened_editor);
                    }
                }

                if let Some(editor) = editor {
                    self.workspace
                        .update(cx, |workspace, cx| {
                            workspace.activate_item(&editor, true, false, cx);
                        })
                        .ok();
                } else {
                    patch_state.update_task = Some(cx.spawn(move |this, cx| async move {
                        Self::open_patch_editor(this, new_patch, cx).await.log_err();
                    }));
                }
            }
        }
    }

    fn close_patch_editor(
        &mut self,
        editor: View<ProposedChangesEditor>,
        cx: &mut ViewContext<ContextEditor>,
    ) {
        self.workspace
            .update(cx, |workspace, cx| {
                if let Some(pane) = workspace.pane_for(&editor) {
                    pane.update(cx, |pane, cx| {
                        let item_id = editor.entity_id();
                        if !editor.read(cx).focus_handle(cx).is_focused(cx) {
                            pane.close_item_by_id(item_id, SaveIntent::Skip, cx)
                                .detach_and_log_err(cx);
                        }
                    });
                }
            })
            .ok();
    }

    async fn open_patch_editor(
        this: WeakView<Self>,
        patch: AssistantPatch,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        let project = this.update(&mut cx, |this, _| this.project.clone())?;
        let resolved_patch = patch.resolve(project.clone(), &mut cx).await;

        let editor = cx.new_view(|cx| {
            let editor = ProposedChangesEditor::new(
                patch.title.clone(),
                resolved_patch
                    .edit_groups
                    .iter()
                    .map(|(buffer, groups)| ProposedChangeLocation {
                        buffer: buffer.clone(),
                        ranges: groups
                            .iter()
                            .map(|group| group.context_range.clone())
                            .collect(),
                    })
                    .collect(),
                Some(project.clone()),
                cx,
            );
            resolved_patch.apply(&editor, cx);
            editor
        })?;

        this.update(&mut cx, |this, cx| {
            if let Some(patch_state) = this.patches.get_mut(&patch.range) {
                patch_state.editor = Some(PatchEditorState {
                    editor: editor.downgrade(),
                    opened_patch: patch,
                });
                patch_state.update_task.take();
            }

            this.workspace
                .update(cx, |workspace, cx| {
                    workspace.add_item_to_active_pane(Box::new(editor.clone()), None, false, cx)
                })
                .log_err();
        })?;

        Ok(())
    }

    async fn update_patch_editor(
        this: WeakView<Self>,
        patch: AssistantPatch,
        mut cx: AsyncWindowContext,
    ) -> Result<()> {
        let project = this.update(&mut cx, |this, _| this.project.clone())?;
        let resolved_patch = patch.resolve(project.clone(), &mut cx).await;
        this.update(&mut cx, |this, cx| {
            let patch_state = this.patches.get_mut(&patch.range)?;

            let locations = resolved_patch
                .edit_groups
                .iter()
                .map(|(buffer, groups)| ProposedChangeLocation {
                    buffer: buffer.clone(),
                    ranges: groups
                        .iter()
                        .map(|group| group.context_range.clone())
                        .collect(),
                })
                .collect();

            if let Some(state) = &mut patch_state.editor {
                if let Some(editor) = state.editor.upgrade() {
                    editor.update(cx, |editor, cx| {
                        editor.set_title(patch.title.clone(), cx);
                        editor.reset_locations(locations, cx);
                        resolved_patch.apply(editor, cx);
                    });

                    state.opened_patch = patch;
                } else {
                    patch_state.editor.take();
                }
            }
            patch_state.update_task.take();

            Some(())
        })?;
        Ok(())
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
                height: 2,
                style: BlockStyle::Sticky,
                placement: BlockPlacement::Above(
                    buffer
                        .anchor_in_excerpt(excerpt_id, message.anchor_range.start)
                        .unwrap(),
                ),
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

    /// Returns either the selected text, or the content of the Markdown code
    /// block surrounding the cursor.
    fn get_selection_or_code_block(
        context_editor_view: &View<ContextEditor>,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<(String, bool)> {
        const CODE_FENCE_DELIMITER: &'static str = "```";

        let context_editor = context_editor_view.read(cx).editor.read(cx);

        if context_editor.selections.newest::<Point>(cx).is_empty() {
            let snapshot = context_editor.buffer().read(cx).snapshot(cx);
            let (_, _, snapshot) = snapshot.as_singleton()?;

            let head = context_editor.selections.newest::<Point>(cx).head();
            let offset = snapshot.point_to_offset(head);

            let surrounding_code_block_range = find_surrounding_code_block(snapshot, offset)?;
            let mut text = snapshot
                .text_for_range(surrounding_code_block_range)
                .collect::<String>();

            // If there is no newline trailing the closing three-backticks, then
            // tree-sitter-md extends the range of the content node to include
            // the backticks.
            if text.ends_with(CODE_FENCE_DELIMITER) {
                text.drain((text.len() - CODE_FENCE_DELIMITER.len())..);
            }

            (!text.is_empty()).then_some((text, true))
        } else {
            let anchor = context_editor.selections.newest_anchor();
            let text = context_editor
                .buffer()
                .read(cx)
                .read(cx)
                .text_for_range(anchor.range())
                .collect::<String>();

            (!text.is_empty()).then_some((text, false))
        }
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

        if let Some((text, _)) = Self::get_selection_or_code_block(&context_editor_view, cx) {
            active_editor_view.update(cx, |editor, cx| {
                editor.insert(&text, cx);
                editor.focus(cx);
            })
        }
    }

    fn copy_code(workspace: &mut Workspace, _: &CopyCode, cx: &mut ViewContext<Workspace>) {
        let result = maybe!({
            let panel = workspace.panel::<AssistantPanel>(cx)?;
            let context_editor_view = panel.read(cx).active_context_editor(cx)?;
            Self::get_selection_or_code_block(&context_editor_view, cx)
        });
        let Some((text, is_code_block)) = result else {
            return;
        };

        cx.write_to_clipboard(ClipboardItem::new_string(text));

        struct CopyToClipboardToast;
        workspace.show_toast(
            Toast::new(
                NotificationId::unique::<CopyToClipboardToast>(),
                format!(
                    "{} copied to clipboard.",
                    if is_code_block {
                        "Code block"
                    } else {
                        "Selection"
                    }
                ),
            )
            .autohide(),
            cx,
        );
    }

    fn insert_dragged_files(
        workspace: &mut Workspace,
        action: &InsertDraggedFiles,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let Some(context_editor_view) = panel.read(cx).active_context_editor(cx) else {
            return;
        };

        let project = workspace.project().clone();

        let paths = match action {
            InsertDraggedFiles::ProjectPaths(paths) => Task::ready((paths.clone(), vec![])),
            InsertDraggedFiles::ExternalFiles(paths) => {
                let tasks = paths
                    .clone()
                    .into_iter()
                    .map(|path| Workspace::project_path_for_path(project.clone(), &path, false, cx))
                    .collect::<Vec<_>>();

                cx.spawn(move |_, cx| async move {
                    let mut paths = vec![];
                    let mut worktrees = vec![];

                    let opened_paths = futures::future::join_all(tasks).await;
                    for (worktree, project_path) in opened_paths.into_iter().flatten() {
                        let Ok(worktree_root_name) =
                            worktree.read_with(&cx, |worktree, _| worktree.root_name().to_string())
                        else {
                            continue;
                        };

                        let mut full_path = PathBuf::from(worktree_root_name.clone());
                        full_path.push(&project_path.path);
                        paths.push(full_path);
                        worktrees.push(worktree);
                    }

                    (paths, worktrees)
                })
            }
        };

        cx.spawn(|_, mut cx| async move {
            let (paths, dragged_file_worktrees) = paths.await;
            let cmd_name = file_command::FileSlashCommand.name();

            context_editor_view
                .update(&mut cx, |context_editor, cx| {
                    let file_argument = paths
                        .into_iter()
                        .map(|path| path.to_string_lossy().to_string())
                        .collect::<Vec<_>>()
                        .join(" ");

                    context_editor.editor.update(cx, |editor, cx| {
                        editor.insert("\n", cx);
                        editor.insert(&format!("/{} {}", cmd_name, file_argument), cx);
                    });

                    context_editor.confirm_command(&ConfirmCommand, cx);

                    context_editor
                        .dragged_file_worktrees
                        .extend(dragged_file_worktrees);
                })
                .log_err();
        })
        .detach();
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

        let mut creases = vec![];
        editor.update(cx, |editor, cx| {
            let selections = editor.selections.all_adjusted(cx);
            let buffer = editor.buffer().read(cx).snapshot(cx);
            for selection in selections {
                let range = editor::ToOffset::to_offset(&selection.start, &buffer)
                    ..editor::ToOffset::to_offset(&selection.end, &buffer);
                let selected_text = buffer.text_for_range(range.clone()).collect::<String>();
                if selected_text.is_empty() {
                    continue;
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

                    let outline_text = if let Some((start_symbols, end_symbols)) =
                        start_symbols.zip(end_symbols)
                    {
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
                        Some(selection.start.row..=selection.end.row),
                    );

                    if let Some((line_comment_prefix, outline_text)) =
                        line_comment_prefix.zip(outline_text)
                    {
                        let breadcrumb =
                            format!("{line_comment_prefix}Excerpt from: {outline_text}\n");
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
                creases.push((text, crease_title));
            }
        });
        if creases.is_empty() {
            return;
        }
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
                            for (text, crease_title) in creases {
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
                            }
                        })
                    });
                };
            });
        });
    }

    fn copy(&mut self, _: &editor::actions::Copy, cx: &mut ViewContext<Self>) {
        if self.editor.read(cx).selections.count() == 1 {
            let (copied_text, metadata, _) = self.get_clipboard_contents(cx);
            cx.write_to_clipboard(ClipboardItem::new_string_with_json_metadata(
                copied_text,
                metadata,
            ));
            cx.stop_propagation();
            return;
        }

        cx.propagate();
    }

    fn cut(&mut self, _: &editor::actions::Cut, cx: &mut ViewContext<Self>) {
        if self.editor.read(cx).selections.count() == 1 {
            let (copied_text, metadata, selections) = self.get_clipboard_contents(cx);

            self.editor.update(cx, |editor, cx| {
                editor.transact(cx, |this, cx| {
                    this.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select(selections);
                    });
                    this.insert("", cx);
                    cx.write_to_clipboard(ClipboardItem::new_string_with_json_metadata(
                        copied_text,
                        metadata,
                    ));
                });
            });

            cx.stop_propagation();
            return;
        }

        cx.propagate();
    }

    fn get_clipboard_contents(
        &mut self,
        cx: &mut ViewContext<Self>,
    ) -> (String, CopyMetadata, Vec<text::Selection<usize>>) {
        let (snapshot, selection, creases) = self.editor.update(cx, |editor, cx| {
            let mut selection = editor.selections.newest::<Point>(cx);
            let snapshot = editor.buffer().read(cx).snapshot(cx);

            let is_entire_line = selection.is_empty() || editor.selections.line_mode;
            if is_entire_line {
                selection.start = Point::new(selection.start.row, 0);
                selection.end =
                    cmp::min(snapshot.max_point(), Point::new(selection.start.row + 1, 0));
                selection.goal = SelectionGoal::None;
            }

            let selection_start = snapshot.point_to_offset(selection.start);

            (
                snapshot.clone(),
                selection.clone(),
                editor.display_map.update(cx, |display_map, cx| {
                    display_map
                        .snapshot(cx)
                        .crease_snapshot
                        .creases_in_range(
                            MultiBufferRow(selection.start.row)
                                ..MultiBufferRow(selection.end.row + 1),
                            &snapshot,
                        )
                        .filter_map(|crease| {
                            if let Some(metadata) = &crease.metadata {
                                let start = crease
                                    .range
                                    .start
                                    .to_offset(&snapshot)
                                    .saturating_sub(selection_start);
                                let end = crease
                                    .range
                                    .end
                                    .to_offset(&snapshot)
                                    .saturating_sub(selection_start);

                                let range_relative_to_selection = start..end;

                                if range_relative_to_selection.is_empty() {
                                    None
                                } else {
                                    Some(SelectedCreaseMetadata {
                                        range_relative_to_selection,
                                        crease: metadata.clone(),
                                    })
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                }),
            )
        });

        let selection = selection.map(|point| snapshot.point_to_offset(point));
        let context = self.context.read(cx);

        let mut text = String::new();
        for message in context.messages(cx) {
            if message.offset_range.start >= selection.range().end {
                break;
            } else if message.offset_range.end >= selection.range().start {
                let range = cmp::max(message.offset_range.start, selection.range().start)
                    ..cmp::min(message.offset_range.end, selection.range().end);
                if !range.is_empty() {
                    for chunk in context.buffer().read(cx).text_for_range(range) {
                        text.push_str(chunk);
                    }
                    if message.offset_range.end < selection.range().end {
                        text.push('\n');
                    }
                }
            }
        }

        (text, CopyMetadata { creases }, vec![selection])
    }

    fn paste(&mut self, action: &editor::actions::Paste, cx: &mut ViewContext<Self>) {
        cx.stop_propagation();

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

        let metadata = if let Some(item) = cx.read_from_clipboard() {
            item.entries().first().and_then(|entry| {
                if let ClipboardEntry::String(text) = entry {
                    text.metadata_json::<CopyMetadata>()
                } else {
                    None
                }
            })
        } else {
            None
        };

        if images.is_empty() {
            self.editor.update(cx, |editor, cx| {
                let paste_position = editor.selections.newest::<usize>(cx).head();
                editor.paste(action, cx);

                if let Some(metadata) = metadata {
                    let buffer = editor.buffer().read(cx).snapshot(cx);

                    let mut buffer_rows_to_fold = BTreeSet::new();
                    let weak_editor = cx.view().downgrade();
                    editor.insert_creases(
                        metadata.creases.into_iter().map(|metadata| {
                            let start = buffer.anchor_after(
                                paste_position + metadata.range_relative_to_selection.start,
                            );
                            let end = buffer.anchor_before(
                                paste_position + metadata.range_relative_to_selection.end,
                            );

                            let buffer_row = MultiBufferRow(start.to_point(&buffer).row);
                            buffer_rows_to_fold.insert(buffer_row);
                            Crease::new(
                                start..end,
                                FoldPlaceholder {
                                    constrain_width: false,
                                    render: render_fold_icon_button(
                                        weak_editor.clone(),
                                        metadata.crease.icon,
                                        metadata.crease.label.clone(),
                                    ),
                                    merge_adjacent: false,
                                },
                                render_slash_command_output_toggle,
                                |_, _, _| Empty.into_any(),
                            )
                            .with_metadata(metadata.crease.clone())
                        }),
                        cx,
                    );
                    for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                        editor.fold_at(&FoldAt { buffer_row }, cx);
                    }
                }
            });
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
                    let Some(render_image) = image.to_image_data(cx).log_err() else {
                        continue;
                    };
                    let image_id = image.id();
                    let image_task = LanguageModelImage::from_image(image, cx).shared();

                    for image_position in image_positions.iter() {
                        context.insert_content(
                            Content::Image {
                                anchor: image_position.text_anchor,
                                image_id,
                                image: image_task.clone(),
                                render_image: render_image.clone(),
                            },
                            cx,
                        );
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
                .contents(cx)
                .filter_map(|content| {
                    if let Content::Image {
                        anchor,
                        render_image,
                        ..
                    } = content
                    {
                        Some((anchor, render_image))
                    } else {
                        None
                    }
                })
                .filter_map(|(anchor, render_image)| {
                    const MAX_HEIGHT_IN_LINES: u32 = 8;
                    let anchor = buffer.anchor_in_excerpt(excerpt_id, anchor).unwrap();
                    let image = render_image.clone();
                    anchor.is_valid(&buffer).then(|| BlockProperties {
                        placement: BlockPlacement::Above(anchor),
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

    fn render_patch_header(
        &self,
        range: Range<text::Anchor>,
        _id: FoldId,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let patch = self.context.read(cx).patch_for_range(&range, cx)?;
        let theme = cx.theme().clone();
        Some(
            h_flex()
                .px_1()
                .py_0p5()
                .border_b_1()
                .border_color(theme.status().info_border)
                .gap_1()
                .child(Icon::new(IconName::Diff).size(IconSize::Small))
                .child(Label::new(patch.title.clone()).size(LabelSize::Small))
                .into_any(),
        )
    }

    fn render_patch_footer(
        &mut self,
        range: Range<text::Anchor>,
        max_width: Pixels,
        gutter_width: Pixels,
        id: BlockId,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let snapshot = self.editor.update(cx, |editor, cx| editor.snapshot(cx));
        let (excerpt_id, _buffer_id, _) = snapshot.buffer_snapshot.as_singleton().unwrap();
        let excerpt_id = *excerpt_id;
        let anchor = snapshot
            .buffer_snapshot
            .anchor_in_excerpt(excerpt_id, range.start)
            .unwrap();

        if !snapshot.intersects_fold(anchor) {
            return None;
        }

        let patch = self.context.read(cx).patch_for_range(&range, cx)?;
        let paths = patch
            .paths()
            .map(|p| SharedString::from(p.to_string()))
            .collect::<BTreeSet<_>>();

        Some(
            v_flex()
                .id(id)
                .pl(gutter_width)
                .w(max_width)
                .py_2()
                .cursor(CursorStyle::PointingHand)
                .on_click(cx.listener(move |this, _, cx| {
                    this.editor.update(cx, |editor, cx| {
                        editor.change_selections(None, cx, |selections| {
                            selections.select_ranges(vec![anchor..anchor]);
                        });
                    });
                    this.focus_active_patch(cx);
                }))
                .children(paths.into_iter().map(|path| {
                    h_flex()
                        .pl_1()
                        .gap_1()
                        .child(Icon::new(IconName::File).size(IconSize::Small))
                        .child(Label::new(path).size(LabelSize::Small))
                }))
                .when(patch.status == AssistantPatchStatus::Pending, |div| {
                    div.child(
                        Label::new("Generating")
                            .color(Color::Muted)
                            .size(LabelSize::Small)
                            .with_animation(
                                "pulsating-label",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.4, 1.)),
                                |label, delta| label.alpha(delta),
                            ),
                    )
                })
                .into_any(),
        )
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
                                Icon::new(IconName::Warning)
                                    .size(IconSize::Small)
                                    .color(Color::Warning),
                            )
                            .child(Label::new(label)),
                    )
                    .child(
                        Button::new("open-configuration", "Configure Providers")
                            .size(ButtonSize::Compact)
                            .icon(Some(IconName::SlidersVertical))
                            .icon_size(IconSize::Small)
                            .icon_position(IconPosition::Start)
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
            .child(Label::new("Send"))
            .children(
                KeyBinding::for_action_in(&Assist, &focus_handle, cx)
                    .map(|binding| binding.into_any_element()),
            )
            .on_click(move |_event, cx| {
                focus_handle.dispatch_action(&Assist, cx);
            })
    }

    fn render_last_error(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        let last_error = self.last_error.as_ref()?;

        Some(
            div()
                .absolute()
                .right_3()
                .bottom_12()
                .max_w_96()
                .py_2()
                .px_3()
                .elevation_2(cx)
                .occlude()
                .child(match last_error {
                    AssistError::PaymentRequired => self.render_payment_required_error(cx),
                    AssistError::MaxMonthlySpendReached => {
                        self.render_max_monthly_spend_reached_error(cx)
                    }
                    AssistError::Message(error_message) => {
                        self.render_assist_error(error_message, cx)
                    }
                })
                .into_any(),
        )
    }

    fn render_payment_required_error(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        const ERROR_MESSAGE: &str = "Free tier exceeded. Subscribe and add payment to continue using Zed LLMs. You'll be billed at cost for tokens used.";

        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(Label::new("Free Usage Exceeded").weight(FontWeight::MEDIUM)),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_24()
                    .overflow_y_scroll()
                    .child(Label::new(ERROR_MESSAGE)),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(Button::new("subscribe", "Subscribe").on_click(cx.listener(
                        |this, _, cx| {
                            this.last_error = None;
                            cx.open_url(&zed_urls::account_url(cx));
                            cx.notify();
                        },
                    )))
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            this.last_error = None;
                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn render_max_monthly_spend_reached_error(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        const ERROR_MESSAGE: &str = "You have reached your maximum monthly spend. Increase your spend limit to continue using Zed LLMs.";

        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::XCircle).color(Color::Error))
                    .child(Label::new("Max Monthly Spend Reached").weight(FontWeight::MEDIUM)),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_24()
                    .overflow_y_scroll()
                    .child(Label::new(ERROR_MESSAGE)),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(
                        Button::new("subscribe", "Update Monthly Spend Limit").on_click(
                            cx.listener(|this, _, cx| {
                                this.last_error = None;
                                cx.open_url(&zed_urls::account_url(cx));
                                cx.notify();
                            }),
                        ),
                    )
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            this.last_error = None;
                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }

    fn render_assist_error(
        &self,
        error_message: &SharedString,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
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
                    .child(Label::new(error_message.clone())),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, cx| {
                            this.last_error = None;
                            cx.notify();
                        },
                    ))),
            )
            .into_any()
    }
}

/// Returns the contents of the *outermost* fenced code block that contains the given offset.
fn find_surrounding_code_block(snapshot: &BufferSnapshot, offset: usize) -> Option<Range<usize>> {
    const CODE_BLOCK_NODE: &'static str = "fenced_code_block";
    const CODE_BLOCK_CONTENT: &'static str = "code_fence_content";

    let layer = snapshot.syntax_layers().next()?;

    let root_node = layer.node();
    let mut cursor = root_node.walk();

    // Go to the first child for the given offset
    while cursor.goto_first_child_for_byte(offset).is_some() {
        // If we're at the end of the node, go to the next one.
        // Example: if you have a fenced-code-block, and you're on the start of the line
        // right after the closing ```, you want to skip the fenced-code-block and
        // go to the next sibling.
        if cursor.node().end_byte() == offset {
            cursor.goto_next_sibling();
        }

        if cursor.node().start_byte() > offset {
            break;
        }

        // We found the fenced code block.
        if cursor.node().kind() == CODE_BLOCK_NODE {
            // Now we need to find the child node that contains the code.
            cursor.goto_first_child();
            loop {
                if cursor.node().kind() == CODE_BLOCK_CONTENT {
                    return Some(cursor.node().byte_range());
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    None
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CopyMetadata {
    creases: Vec<SelectedCreaseMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SelectedCreaseMetadata {
    range_relative_to_selection: Range<usize>,
    crease: CreaseMetadata,
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
            .capture_action(cx.listener(ContextEditor::cut))
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
            .children(self.render_last_error(cx))
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
                                .gap_1()
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
        self.editor.update(cx, Item::deactivated)
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
        Button::new("trigger", "Add Context")
            .icon(IconName::Plus)
            .icon_size(IconSize::Small)
            .icon_position(IconPosition::Start)
            .tooltip(|cx| Tooltip::text("Type / to insert via keyboard", cx)),
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
            // TODO display this in a nicer way, once we have a design for it.
            // .children({
            //     let project = self
            //         .workspace
            //         .upgrade()
            //         .map(|workspace| workspace.read(cx).project().downgrade());
            //
            //     let scan_items_remaining = cx.update_global(|db: &mut SemanticDb, cx| {
            //         project.and_then(|project| db.remaining_summaries(&project, cx))
            //     });

            //     scan_items_remaining
            //         .map(|remaining_items| format!("Files to scan: {}", remaining_items))
            // })
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
                    .child(Icon::new(IconName::TextSnippet))
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
                    Icon::new(IconName::Warning)
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
) -> Result<Option<Arc<dyn LspAdapterDelegate>>> {
    project.update(cx, |project, cx| {
        // TODO: Find the right worktree.
        let Some(worktree) = project.worktrees(cx).next() else {
            return Ok(None::<Arc<dyn LspAdapterDelegate>>);
        };
        let http_client = project.client().http_client().clone();
        project.lsp_store().update(cx, |lsp_store, cx| {
            Ok(Some(LocalLspAdapterDelegate::new(
                lsp_store,
                &worktree,
                http_client,
                project.fs().clone(),
                cx,
            ) as Arc<dyn LspAdapterDelegate>))
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, Context};
    use language::Buffer;
    use unindent::Unindent;

    #[gpui::test]
    fn test_find_code_blocks(cx: &mut AppContext) {
        let markdown = languages::language("markdown", tree_sitter_md::LANGUAGE.into());

        let buffer = cx.new_model(|cx| {
            let text = r#"
                line 0
                line 1
                ```rust
                fn main() {}
                ```
                line 5
                line 6
                line 7
                ```go
                func main() {}
                ```
                line 11
                ```
                this is plain text code block
                ```

                ```go
                func another() {}
                ```
                line 19
            "#
            .unindent();
            let mut buffer = Buffer::local(text, cx);
            buffer.set_language(Some(markdown.clone()), cx);
            buffer
        });
        let snapshot = buffer.read(cx).snapshot();

        let code_blocks = vec![
            Point::new(3, 0)..Point::new(4, 0),
            Point::new(9, 0)..Point::new(10, 0),
            Point::new(13, 0)..Point::new(14, 0),
            Point::new(17, 0)..Point::new(18, 0),
        ]
        .into_iter()
        .map(|range| snapshot.point_to_offset(range.start)..snapshot.point_to_offset(range.end))
        .collect::<Vec<_>>();

        let expected_results = vec![
            (0, None),
            (1, None),
            (2, Some(code_blocks[0].clone())),
            (3, Some(code_blocks[0].clone())),
            (4, Some(code_blocks[0].clone())),
            (5, None),
            (6, None),
            (7, None),
            (8, Some(code_blocks[1].clone())),
            (9, Some(code_blocks[1].clone())),
            (10, Some(code_blocks[1].clone())),
            (11, None),
            (12, Some(code_blocks[2].clone())),
            (13, Some(code_blocks[2].clone())),
            (14, Some(code_blocks[2].clone())),
            (15, None),
            (16, Some(code_blocks[3].clone())),
            (17, Some(code_blocks[3].clone())),
            (18, Some(code_blocks[3].clone())),
            (19, None),
        ];

        for (row, expected) in expected_results {
            let offset = snapshot.point_to_offset(Point::new(row, 0));
            let range = find_surrounding_code_block(&snapshot, offset);
            assert_eq!(range, expected, "unexpected result on row {:?}", row);
        }
    }
}
