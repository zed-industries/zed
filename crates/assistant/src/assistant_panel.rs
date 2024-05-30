use crate::prompts::{generate_content_prompt, PromptLibrary, PromptManager};
use crate::slash_command::{rustdoc_command, search_command, tabs_command};
use crate::{
    assistant_settings::{AssistantDockPosition, AssistantSettings},
    codegen::{self, Codegen, CodegenKind},
    search::*,
    slash_command::{
        active_command, file_command, project_command, prompt_command,
        SlashCommandCompletionProvider, SlashCommandLine, SlashCommandRegistry,
    },
    ApplyEdit, Assist, CompletionProvider, ConfirmCommand, CycleMessageRole, InlineAssist,
    LanguageModelRequest, LanguageModelRequestMessage, MessageId, MessageMetadata, MessageStatus,
    QuoteSelection, ResetKey, Role, SavedConversation, SavedConversationMetadata, SavedMessage,
    Split, ToggleFocus, ToggleHistory,
};
use crate::{ModelSelector, ToggleModelSelector};
use anyhow::{anyhow, Result};
use assistant_slash_command::{SlashCommandOutput, SlashCommandOutputSection};
use client::telemetry::Telemetry;
use collections::{hash_map, BTreeSet, HashMap, HashSet, VecDeque};
use editor::actions::ShowCompletions;
use editor::{
    actions::{FoldAt, MoveDown, MoveToEndOfLine, MoveUp, Newline, UnfoldAt},
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, Flap, ToDisplayPoint,
    },
    scroll::{Autoscroll, AutoscrollStrategy},
    Anchor, Editor, EditorElement, EditorEvent, EditorStyle, MultiBufferSnapshot, RowExt,
    ToOffset as _, ToPoint,
};
use editor::{display_map::FlapId, FoldPlaceholder};
use feature_flags::{FeatureFlag, FeatureFlagAppExt, FeatureFlagViewExt};
use file_icons::FileIcons;
use fs::Fs;
use futures::future::Shared;
use futures::{FutureExt, StreamExt};
use gpui::{
    canvas, div, point, relative, rems, uniform_list, Action, AnyElement, AnyView, AppContext,
    AsyncAppContext, AsyncWindowContext, AvailableSpace, ClipboardItem, Context, Empty,
    EventEmitter, FocusHandle, FocusableView, FontStyle, FontWeight, HighlightStyle,
    InteractiveElement, IntoElement, Model, ModelContext, ParentElement, Pixels, Render,
    SharedString, StatefulInteractiveElement, Styled, Subscription, Task, TextStyle,
    UniformListScrollHandle, View, ViewContext, VisualContext, WeakModel, WeakView, WhiteSpace,
    WindowContext,
};
use language::LspAdapterDelegate;
use language::{
    language_settings::SoftWrap, AnchorRangeExt, AutoindentMode, Buffer, LanguageRegistry,
    OffsetRangeExt as _, Point, ToOffset as _,
};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use project::{Project, ProjectLspAdapterDelegate, ProjectTransaction};
use search::{buffer_search::DivRegistrar, BufferSearchBar};
use settings::Settings;
use std::{
    cmp::{self, Ordering},
    fmt::Write,
    iter,
    ops::Range,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::AssistantKind;
use theme::ThemeSettings;
use ui::{
    popover_menu, prelude::*, ButtonLike, ContextMenu, ElevationIndex, KeyBinding,
    PopoverMenuHandle, Tab, TabBar, Tooltip,
};
use util::{paths::CONVERSATIONS_DIR, post_inc, ResultExt, TryFutureExt};
use uuid::Uuid;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    searchable::Direction,
    Save, Toast, ToggleZoom, Toolbar, Workspace,
};
use workspace::{notifications::NotificationId, NewFile};

pub fn init(cx: &mut AppContext) {
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
                .register_action(AssistantPanel::cancel_last_inline_assist)
                // .register_action(ConversationEditor::insert_active_prompt)
                .register_action(ConversationEditor::quote_selection);
        },
    )
    .detach();
}

pub struct AssistantPanel {
    workspace: WeakView<Workspace>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    active_conversation_editor: Option<ActiveConversationEditor>,
    show_saved_conversations: bool,
    saved_conversations: Vec<SavedConversationMetadata>,
    saved_conversations_scroll_handle: UniformListScrollHandle,
    zoomed: bool,
    focus_handle: FocusHandle,
    toolbar: View<Toolbar>,
    languages: Arc<LanguageRegistry>,
    slash_commands: Arc<SlashCommandRegistry>,
    prompt_library: Arc<PromptLibrary>,
    fs: Arc<dyn Fs>,
    telemetry: Arc<Telemetry>,
    _subscriptions: Vec<Subscription>,
    next_inline_assist_id: usize,
    pending_inline_assists: HashMap<usize, PendingInlineAssist>,
    pending_inline_assist_ids_by_editor: HashMap<WeakView<Editor>, Vec<usize>>,
    inline_prompt_history: VecDeque<String>,
    _watch_saved_conversations: Task<Result<()>>,
    authentication_prompt: Option<AnyView>,
    model_menu_handle: PopoverMenuHandle<ContextMenu>,
}

struct ActiveConversationEditor {
    editor: View<ConversationEditor>,
    _subscriptions: Vec<Subscription>,
}

struct PromptLibraryFeatureFlag;

impl FeatureFlag for PromptLibraryFeatureFlag {
    const NAME: &'static str = "prompt-library";
}

impl AssistantPanel {
    const INLINE_PROMPT_HISTORY_MAX_LEN: usize = 20;

    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let fs = workspace.update(&mut cx, |workspace, _| workspace.app_state().fs.clone())?;
            let saved_conversations = SavedConversationMetadata::list(fs.clone())
                .await
                .log_err()
                .unwrap_or_default();

            let prompt_library = Arc::new(
                PromptLibrary::load_index(fs.clone())
                    .await
                    .log_err()
                    .unwrap_or_default(),
            );

            // TODO: deserialize state.
            let workspace_handle = workspace.clone();
            workspace.update(&mut cx, |workspace, cx| {
                cx.new_view::<Self>(|cx| {
                    cx.observe_flag::<PromptLibraryFeatureFlag, _>(|_, _, cx| cx.notify())
                        .detach();

                    const CONVERSATION_WATCH_DURATION: Duration = Duration::from_millis(100);
                    let _watch_saved_conversations = cx.spawn(move |this, mut cx| async move {
                        let mut events = fs
                            .watch(&CONVERSATIONS_DIR, CONVERSATION_WATCH_DURATION)
                            .await;
                        while events.next().await.is_some() {
                            let saved_conversations = SavedConversationMetadata::list(fs.clone())
                                .await
                                .log_err()
                                .unwrap_or_default();
                            this.update(&mut cx, |this, cx| {
                                this.saved_conversations = saved_conversations;
                                cx.notify();
                            })
                            .ok();
                        }

                        anyhow::Ok(())
                    });

                    let toolbar = cx.new_view(|cx| {
                        let mut toolbar = Toolbar::new();
                        toolbar.set_can_navigate(false, cx);
                        toolbar.add_item(cx.new_view(BufferSearchBar::new), cx);
                        toolbar
                    });

                    let focus_handle = cx.focus_handle();
                    let subscriptions = vec![
                        cx.on_focus_in(&focus_handle, Self::focus_in),
                        cx.on_focus_out(&focus_handle, Self::focus_out),
                        cx.observe_global::<CompletionProvider>({
                            let mut prev_settings_version =
                                CompletionProvider::global(cx).settings_version();
                            move |this, cx| {
                                this.completion_provider_changed(prev_settings_version, cx);
                                prev_settings_version =
                                    CompletionProvider::global(cx).settings_version();
                            }
                        }),
                    ];

                    cx.observe_global::<FileIcons>(|_, cx| {
                        cx.notify();
                    })
                    .detach();

                    let slash_command_registry = SlashCommandRegistry::global(cx);

                    slash_command_registry.register_command(file_command::FileSlashCommand, true);
                    slash_command_registry.register_command(
                        prompt_command::PromptSlashCommand::new(prompt_library.clone()),
                        true,
                    );
                    slash_command_registry
                        .register_command(active_command::ActiveSlashCommand, true);
                    slash_command_registry.register_command(tabs_command::TabsSlashCommand, true);
                    slash_command_registry
                        .register_command(project_command::ProjectSlashCommand, true);
                    slash_command_registry
                        .register_command(search_command::SearchSlashCommand, true);
                    slash_command_registry
                        .register_command(rustdoc_command::RustdocSlashCommand, false);

                    Self {
                        workspace: workspace_handle,
                        active_conversation_editor: None,
                        show_saved_conversations: false,
                        saved_conversations,
                        saved_conversations_scroll_handle: Default::default(),
                        zoomed: false,
                        focus_handle,
                        toolbar,
                        languages: workspace.app_state().languages.clone(),
                        slash_commands: slash_command_registry,
                        prompt_library,
                        fs: workspace.app_state().fs.clone(),
                        telemetry: workspace.client().telemetry().clone(),
                        width: None,
                        height: None,
                        _subscriptions: subscriptions,
                        next_inline_assist_id: 0,
                        pending_inline_assists: Default::default(),
                        pending_inline_assist_ids_by_editor: Default::default(),
                        inline_prompt_history: Default::default(),
                        _watch_saved_conversations,
                        authentication_prompt: None,
                        model_menu_handle: PopoverMenuHandle::default(),
                    }
                })
            })
        })
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        self.toolbar
            .update(cx, |toolbar, cx| toolbar.focus_changed(true, cx));
        cx.notify();
        if self.focus_handle.is_focused(cx) {
            if let Some(editor) = self.active_conversation_editor() {
                cx.focus_view(editor);
            }
        }
    }

    fn focus_out(&mut self, cx: &mut ViewContext<Self>) {
        self.toolbar
            .update(cx, |toolbar, cx| toolbar.focus_changed(false, cx));
        cx.notify();
    }

    fn completion_provider_changed(
        &mut self,
        prev_settings_version: usize,
        cx: &mut ViewContext<Self>,
    ) {
        if self.is_authenticated(cx) {
            self.authentication_prompt = None;

            if let Some(editor) = self.active_conversation_editor() {
                editor.update(cx, |active_conversation, cx| {
                    active_conversation
                        .conversation
                        .update(cx, |conversation, cx| {
                            conversation.completion_provider_changed(cx)
                        })
                })
            }

            if self.active_conversation_editor().is_none() {
                self.new_conversation(cx);
            }
            cx.notify();
        } else if self.authentication_prompt.is_none()
            || prev_settings_version != CompletionProvider::global(cx).settings_version()
        {
            self.authentication_prompt =
                Some(cx.update_global::<CompletionProvider, _>(|provider, cx| {
                    provider.authentication_prompt(cx)
                }));
            cx.notify();
        }
    }

    pub fn inline_assist(
        workspace: &mut Workspace,
        _: &InlineAssist,
        cx: &mut ViewContext<Workspace>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled {
            return;
        }

        let Some(assistant) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        let conversation_editor =
            assistant
                .read(cx)
                .active_conversation_editor()
                .and_then(|editor| {
                    let editor = &editor.read(cx).editor;
                    if editor.read(cx).is_focused(cx) {
                        Some(editor.clone())
                    } else {
                        None
                    }
                });

        let show_include_conversation;
        let active_editor;
        if let Some(conversation_editor) = conversation_editor {
            active_editor = conversation_editor;
            show_include_conversation = false;
        } else if let Some(workspace_editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        {
            active_editor = workspace_editor;
            show_include_conversation = true;
        } else {
            return;
        };
        let project = workspace.project().clone();

        if assistant.update(cx, |assistant, cx| assistant.is_authenticated(cx)) {
            assistant.update(cx, |assistant, cx| {
                assistant.new_inline_assist(&active_editor, &project, show_include_conversation, cx)
            });
        } else {
            let assistant = assistant.downgrade();
            cx.spawn(|workspace, mut cx| async move {
                assistant
                    .update(&mut cx, |assistant, cx| assistant.authenticate(cx))?
                    .await?;
                if assistant.update(&mut cx, |assistant, cx| assistant.is_authenticated(cx))? {
                    assistant.update(&mut cx, |assistant, cx| {
                        assistant.new_inline_assist(
                            &active_editor,
                            &project,
                            show_include_conversation,
                            cx,
                        )
                    })?;
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

    fn new_inline_assist(
        &mut self,
        editor: &View<Editor>,
        project: &Model<Project>,
        include_conversation: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let selection = editor.read(cx).selections.newest_anchor().clone();
        if selection.start.excerpt_id != selection.end.excerpt_id {
            return;
        }
        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);

        // Extend the selection to the start and the end of the line.
        let mut point_selection = selection.map(|selection| selection.to_point(&snapshot));
        if point_selection.end > point_selection.start {
            point_selection.start.column = 0;
            // If the selection ends at the start of the line, we don't want to include it.
            if point_selection.end.column == 0 {
                point_selection.end.row -= 1;
            }
            point_selection.end.column = snapshot.line_len(MultiBufferRow(point_selection.end.row));
        }

        let codegen_kind = if point_selection.start == point_selection.end {
            CodegenKind::Generate {
                position: snapshot.anchor_after(point_selection.start),
            }
        } else {
            CodegenKind::Transform {
                range: snapshot.anchor_before(point_selection.start)
                    ..snapshot.anchor_after(point_selection.end),
            }
        };

        let inline_assist_id = post_inc(&mut self.next_inline_assist_id);
        let telemetry = self.telemetry.clone();

        let codegen = cx.new_model(|cx| {
            Codegen::new(
                editor.read(cx).buffer().clone(),
                codegen_kind,
                Some(telemetry),
                cx,
            )
        });

        let measurements = Arc::new(Mutex::new(BlockMeasurements::default()));
        let inline_assistant = cx.new_view(|cx| {
            InlineAssistant::new(
                inline_assist_id,
                measurements.clone(),
                include_conversation,
                self.inline_prompt_history.clone(),
                codegen.clone(),
                cx,
            )
        });
        let block_id = editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |selections| {
                selections.select_anchor_ranges([selection.head()..selection.head()])
            });
            editor.insert_blocks(
                [BlockProperties {
                    style: BlockStyle::Flex,
                    position: snapshot.anchor_before(Point::new(point_selection.head().row, 0)),
                    height: 2,
                    render: Box::new({
                        let inline_assistant = inline_assistant.clone();
                        move |cx: &mut BlockContext| {
                            *measurements.lock() = BlockMeasurements {
                                gutter_width: cx.gutter_dimensions.width,
                                gutter_margin: cx.gutter_dimensions.margin,
                            };
                            inline_assistant.clone().into_any_element()
                        }
                    }),
                    disposition: if selection.reversed {
                        BlockDisposition::Above
                    } else {
                        BlockDisposition::Below
                    },
                }],
                Some(Autoscroll::Strategy(AutoscrollStrategy::Newest)),
                cx,
            )[0]
        });

        self.pending_inline_assists.insert(
            inline_assist_id,
            PendingInlineAssist {
                editor: editor.downgrade(),
                inline_assistant: Some((block_id, inline_assistant.clone())),
                codegen: codegen.clone(),
                project: project.downgrade(),
                _subscriptions: vec![
                    cx.subscribe(&inline_assistant, Self::handle_inline_assistant_event),
                    cx.subscribe(editor, {
                        let inline_assistant = inline_assistant.downgrade();
                        move |_, editor, event, cx| {
                            if let Some(inline_assistant) = inline_assistant.upgrade() {
                                if let EditorEvent::SelectionsChanged { local } = event {
                                    if *local
                                        && inline_assistant.focus_handle(cx).contains_focused(cx)
                                    {
                                        cx.focus_view(&editor);
                                    }
                                }
                            }
                        }
                    }),
                    cx.observe(&codegen, {
                        let editor = editor.downgrade();
                        move |this, _, cx| {
                            if let Some(editor) = editor.upgrade() {
                                this.update_highlights_for_editor(&editor, cx);
                            }
                        }
                    }),
                    cx.subscribe(&codegen, move |this, codegen, event, cx| match event {
                        codegen::Event::Undone => {
                            this.finish_inline_assist(inline_assist_id, false, cx)
                        }
                        codegen::Event::Finished => {
                            let pending_assist = if let Some(pending_assist) =
                                this.pending_inline_assists.get(&inline_assist_id)
                            {
                                pending_assist
                            } else {
                                return;
                            };

                            let error = codegen
                                .read(cx)
                                .error()
                                .map(|error| format!("Inline assistant error: {}", error));
                            if let Some(error) = error {
                                if pending_assist.inline_assistant.is_none() {
                                    if let Some(workspace) = this.workspace.upgrade() {
                                        workspace.update(cx, |workspace, cx| {
                                            struct InlineAssistantError;

                                            let id =
                                                NotificationId::identified::<InlineAssistantError>(
                                                    inline_assist_id,
                                                );

                                            workspace.show_toast(Toast::new(id, error), cx);
                                        })
                                    }

                                    this.finish_inline_assist(inline_assist_id, false, cx);
                                }
                            } else {
                                this.finish_inline_assist(inline_assist_id, false, cx);
                            }
                        }
                    }),
                ],
            },
        );
        self.pending_inline_assist_ids_by_editor
            .entry(editor.downgrade())
            .or_default()
            .push(inline_assist_id);
        self.update_highlights_for_editor(editor, cx);
    }

    fn handle_inline_assistant_event(
        &mut self,
        inline_assistant: View<InlineAssistant>,
        event: &InlineAssistantEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let assist_id = inline_assistant.read(cx).id;
        match event {
            InlineAssistantEvent::Confirmed {
                prompt,
                include_conversation,
            } => {
                self.confirm_inline_assist(assist_id, prompt, *include_conversation, cx);
            }
            InlineAssistantEvent::Canceled => {
                self.finish_inline_assist(assist_id, true, cx);
            }
            InlineAssistantEvent::Dismissed => {
                self.hide_inline_assist(assist_id, cx);
            }
        }
    }

    fn cancel_last_inline_assist(
        workspace: &mut Workspace,
        _: &editor::actions::Cancel,
        cx: &mut ViewContext<Workspace>,
    ) {
        if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
            if let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.downcast::<Editor>())
            {
                let handled = panel.update(cx, |panel, cx| {
                    if let Some(assist_id) = panel
                        .pending_inline_assist_ids_by_editor
                        .get(&editor.downgrade())
                        .and_then(|assist_ids| assist_ids.last().copied())
                    {
                        panel.finish_inline_assist(assist_id, true, cx);
                        true
                    } else {
                        false
                    }
                });
                if handled {
                    return;
                }
            }
        }

        cx.propagate();
    }

    fn finish_inline_assist(&mut self, assist_id: usize, undo: bool, cx: &mut ViewContext<Self>) {
        self.hide_inline_assist(assist_id, cx);

        if let Some(pending_assist) = self.pending_inline_assists.remove(&assist_id) {
            if let hash_map::Entry::Occupied(mut entry) = self
                .pending_inline_assist_ids_by_editor
                .entry(pending_assist.editor.clone())
            {
                entry.get_mut().retain(|id| *id != assist_id);
                if entry.get().is_empty() {
                    entry.remove();
                }
            }

            if let Some(editor) = pending_assist.editor.upgrade() {
                self.update_highlights_for_editor(&editor, cx);

                if undo {
                    pending_assist
                        .codegen
                        .update(cx, |codegen, cx| codegen.undo(cx));
                }
            }
        }
    }

    fn hide_inline_assist(&mut self, assist_id: usize, cx: &mut ViewContext<Self>) {
        if let Some(pending_assist) = self.pending_inline_assists.get_mut(&assist_id) {
            if let Some(editor) = pending_assist.editor.upgrade() {
                if let Some((block_id, inline_assistant)) = pending_assist.inline_assistant.take() {
                    editor.update(cx, |editor, cx| {
                        editor.remove_blocks(HashSet::from_iter([block_id]), None, cx);
                        if inline_assistant.focus_handle(cx).contains_focused(cx) {
                            editor.focus(cx);
                        }
                    });
                }
            }
        }
    }

    fn confirm_inline_assist(
        &mut self,
        inline_assist_id: usize,
        user_prompt: &str,
        include_conversation: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let conversation = if include_conversation {
            self.active_conversation_editor()
                .map(|editor| editor.read(cx).conversation.clone())
        } else {
            None
        };

        let pending_assist =
            if let Some(pending_assist) = self.pending_inline_assists.get_mut(&inline_assist_id) {
                pending_assist
            } else {
                return;
            };

        let editor = if let Some(editor) = pending_assist.editor.upgrade() {
            editor
        } else {
            return;
        };

        let project = pending_assist.project.clone();

        let project_name = project.upgrade().map(|project| {
            project
                .read(cx)
                .worktree_root_names(cx)
                .collect::<Vec<&str>>()
                .join("/")
        });

        self.inline_prompt_history
            .retain(|prompt| prompt != user_prompt);
        self.inline_prompt_history.push_back(user_prompt.into());
        if self.inline_prompt_history.len() > Self::INLINE_PROMPT_HISTORY_MAX_LEN {
            self.inline_prompt_history.pop_front();
        }

        let codegen = pending_assist.codegen.clone();
        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        let range = codegen.read(cx).range();
        let start = snapshot.point_to_buffer_offset(range.start);
        let end = snapshot.point_to_buffer_offset(range.end);
        let (buffer, range) = if let Some((start, end)) = start.zip(end) {
            let (start_buffer, start_buffer_offset) = start;
            let (end_buffer, end_buffer_offset) = end;
            if start_buffer.remote_id() == end_buffer.remote_id() {
                (start_buffer.clone(), start_buffer_offset..end_buffer_offset)
            } else {
                self.finish_inline_assist(inline_assist_id, false, cx);
                return;
            }
        } else {
            self.finish_inline_assist(inline_assist_id, false, cx);
            return;
        };

        let language = buffer.language_at(range.start);
        let language_name = if let Some(language) = language.as_ref() {
            if Arc::ptr_eq(language, &language::PLAIN_TEXT) {
                None
            } else {
                Some(language.name())
            }
        } else {
            None
        };

        // Higher Temperature increases the randomness of model outputs.
        // If Markdown or No Language is Known, increase the randomness for more creative output
        // If Code, decrease temperature to get more deterministic outputs
        let temperature = if let Some(language) = language_name.clone() {
            if language.as_ref() == "Markdown" {
                1.0
            } else {
                0.5
            }
        } else {
            1.0
        };

        let user_prompt = user_prompt.to_string();

        let prompt = cx.background_executor().spawn(async move {
            let language_name = language_name.as_deref();
            generate_content_prompt(user_prompt, language_name, buffer, range, project_name)
        });

        let mut messages = Vec::new();
        if let Some(conversation) = conversation {
            let conversation = conversation.read(cx);
            let buffer = conversation.buffer.read(cx);
            messages.extend(
                conversation
                    .messages(cx)
                    .map(|message| message.to_request_message(buffer)),
            );
        }
        let model = CompletionProvider::global(cx).model();

        cx.spawn(|_, mut cx| async move {
            // I Don't know if we want to return a ? here.
            let prompt = prompt.await?;

            messages.push(LanguageModelRequestMessage {
                role: Role::User,
                content: prompt,
            });

            let request = LanguageModelRequest {
                model,
                messages,
                stop: vec!["|END|>".to_string()],
                temperature,
            };

            codegen.update(&mut cx, |codegen, cx| codegen.start(request, cx))?;
            anyhow::Ok(())
        })
        .detach();
    }

    fn update_highlights_for_editor(&self, editor: &View<Editor>, cx: &mut ViewContext<Self>) {
        let mut background_ranges = Vec::new();
        let mut foreground_ranges = Vec::new();
        let empty_inline_assist_ids = Vec::new();
        let inline_assist_ids = self
            .pending_inline_assist_ids_by_editor
            .get(&editor.downgrade())
            .unwrap_or(&empty_inline_assist_ids);

        for inline_assist_id in inline_assist_ids {
            if let Some(pending_assist) = self.pending_inline_assists.get(inline_assist_id) {
                let codegen = pending_assist.codegen.read(cx);
                background_ranges.push(codegen.range());
                foreground_ranges.extend(codegen.last_equal_ranges().iter().cloned());
            }
        }

        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        merge_ranges(&mut background_ranges, &snapshot);
        merge_ranges(&mut foreground_ranges, &snapshot);
        editor.update(cx, |editor, cx| {
            if background_ranges.is_empty() {
                editor.clear_background_highlights::<PendingInlineAssist>(cx);
            } else {
                editor.highlight_background::<PendingInlineAssist>(
                    &background_ranges,
                    |theme| theme.editor_active_line_background, // TODO use the appropriate color
                    cx,
                );
            }

            if foreground_ranges.is_empty() {
                editor.clear_highlights::<PendingInlineAssist>(cx);
            } else {
                editor.highlight_text::<PendingInlineAssist>(
                    foreground_ranges,
                    HighlightStyle {
                        fade_out: Some(0.6),
                        ..Default::default()
                    },
                    cx,
                );
            }
        });
    }

    fn new_conversation(&mut self, cx: &mut ViewContext<Self>) -> Option<View<ConversationEditor>> {
        let workspace = self.workspace.upgrade()?;

        let editor = cx.new_view(|cx| {
            ConversationEditor::new(
                self.languages.clone(),
                self.slash_commands.clone(),
                self.fs.clone(),
                workspace,
                cx,
            )
        });

        self.show_conversation(editor.clone(), cx);
        Some(editor)
    }

    fn show_conversation(
        &mut self,
        conversation_editor: View<ConversationEditor>,
        cx: &mut ViewContext<Self>,
    ) {
        let mut subscriptions = Vec::new();
        subscriptions
            .push(cx.subscribe(&conversation_editor, Self::handle_conversation_editor_event));

        let conversation = conversation_editor.read(cx).conversation.clone();
        subscriptions.push(cx.observe(&conversation, |_, _, cx| cx.notify()));

        let editor = conversation_editor.read(cx).editor.clone();
        self.toolbar.update(cx, |toolbar, cx| {
            toolbar.set_active_item(Some(&editor), cx);
        });
        if self.focus_handle.contains_focused(cx) {
            cx.focus_view(&editor);
        }
        self.active_conversation_editor = Some(ActiveConversationEditor {
            editor: conversation_editor,
            _subscriptions: subscriptions,
        });
        self.show_saved_conversations = false;

        cx.notify();
    }

    fn handle_conversation_editor_event(
        &mut self,
        _: View<ConversationEditor>,
        event: &ConversationEditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ConversationEditorEvent::TabContentChanged => cx.notify(),
        }
    }

    fn toggle_zoom(&mut self, _: &workspace::ToggleZoom, cx: &mut ViewContext<Self>) {
        if self.zoomed {
            cx.emit(PanelEvent::ZoomOut)
        } else {
            cx.emit(PanelEvent::ZoomIn)
        }
    }

    fn toggle_history(&mut self, _: &ToggleHistory, cx: &mut ViewContext<Self>) {
        self.show_saved_conversations = !self.show_saved_conversations;
        cx.notify();
    }

    fn show_history(&mut self, cx: &mut ViewContext<Self>) {
        if !self.show_saved_conversations {
            self.show_saved_conversations = true;
            cx.notify();
        }
    }

    fn deploy(&mut self, action: &search::buffer_search::Deploy, cx: &mut ViewContext<Self>) {
        let mut propagate = true;
        if let Some(search_bar) = self.toolbar.read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |search_bar, cx| {
                if search_bar.show(cx) {
                    search_bar.search_suggested(cx);
                    if action.focus {
                        let focus_handle = search_bar.focus_handle(cx);
                        search_bar.select_query(cx);
                        cx.focus(&focus_handle);
                    }
                    propagate = false
                }
            });
        }
        if propagate {
            cx.propagate();
        }
    }

    fn handle_editor_cancel(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        if let Some(search_bar) = self.toolbar.read(cx).item_of_type::<BufferSearchBar>() {
            if !search_bar.read(cx).is_dismissed() {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.dismiss(&Default::default(), cx)
                });
                return;
            }
        }
        cx.propagate();
    }

    fn select_next_match(&mut self, _: &search::SelectNextMatch, cx: &mut ViewContext<Self>) {
        if let Some(search_bar) = self.toolbar.read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.select_match(Direction::Next, 1, cx));
        }
    }

    fn select_prev_match(&mut self, _: &search::SelectPrevMatch, cx: &mut ViewContext<Self>) {
        if let Some(search_bar) = self.toolbar.read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.select_match(Direction::Prev, 1, cx));
        }
    }

    fn reset_credentials(&mut self, _: &ResetKey, cx: &mut ViewContext<Self>) {
        CompletionProvider::global(cx)
            .reset_credentials(cx)
            .detach_and_log_err(cx);
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.model_menu_handle.toggle(cx);
    }

    fn insert_command(&mut self, name: &str, cx: &mut ViewContext<Self>) {
        if let Some(conversation_editor) = self.active_conversation_editor() {
            conversation_editor.update(cx, |conversation_editor, cx| {
                conversation_editor.insert_command(name, cx)
            });
        }
    }

    fn active_conversation_editor(&self) -> Option<&View<ConversationEditor>> {
        Some(&self.active_conversation_editor.as_ref()?.editor)
    }

    fn render_popover_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let assistant = cx.view().clone();
        let zoomed = self.zoomed;
        popover_menu("assistant-popover")
            .trigger(IconButton::new("trigger", IconName::Menu))
            .menu(move |cx| {
                let assistant = assistant.clone();
                ContextMenu::build(cx, |menu, _cx| {
                    menu.entry(
                        if zoomed { "Zoom Out" } else { "Zoom In" },
                        Some(Box::new(ToggleZoom)),
                        {
                            let assistant = assistant.clone();
                            move |cx| {
                                assistant.focus_handle(cx).dispatch_action(&ToggleZoom, cx);
                            }
                        },
                    )
                    .entry("New Context", Some(Box::new(NewFile)), {
                        let assistant = assistant.clone();
                        move |cx| {
                            assistant.focus_handle(cx).dispatch_action(&NewFile, cx);
                        }
                    })
                    .entry("History", Some(Box::new(ToggleHistory)), {
                        let assistant = assistant.clone();
                        move |cx| assistant.update(cx, |assistant, cx| assistant.show_history(cx))
                    })
                })
                .into()
            })
    }

    fn render_inject_context_menu(&self, cx: &mut ViewContext<Self>) -> impl Element {
        let commands = self.slash_commands.clone();
        let assistant_panel = cx.view().downgrade();
        let active_editor_focus_handle = self.workspace.upgrade().and_then(|workspace| {
            Some(
                workspace
                    .read(cx)
                    .active_item_as::<Editor>(cx)?
                    .focus_handle(cx),
            )
        });

        popover_menu("inject-context-menu")
            .trigger(IconButton::new("trigger", IconName::Quote).tooltip(|cx| {
                Tooltip::with_meta("Insert Context", None, "Type / to insert via keyboard", cx)
            }))
            .menu(move |cx| {
                ContextMenu::build(cx, |mut menu, _cx| {
                    for command_name in commands.featured_command_names() {
                        if let Some(command) = commands.command(&command_name) {
                            let menu_text = SharedString::from(Arc::from(command.menu_text()));
                            menu = menu.custom_entry(
                                {
                                    let command_name = command_name.clone();
                                    move |_cx| {
                                        h_flex()
                                            .w_full()
                                            .justify_between()
                                            .child(Label::new(menu_text.clone()))
                                            .child(
                                                div().ml_4().child(
                                                    Label::new(format!("/{command_name}"))
                                                        .color(Color::Muted),
                                                ),
                                            )
                                            .into_any()
                                    }
                                },
                                {
                                    let assistant_panel = assistant_panel.clone();
                                    move |cx| {
                                        assistant_panel
                                            .update(cx, |assistant_panel, cx| {
                                                assistant_panel.insert_command(&command_name, cx)
                                            })
                                            .ok();
                                    }
                                },
                            )
                        }
                    }

                    if let Some(active_editor_focus_handle) = active_editor_focus_handle.clone() {
                        menu = menu
                            .context(active_editor_focus_handle)
                            .action("Quote Selection", Box::new(QuoteSelection));
                    }

                    menu
                })
                .into()
            })
    }

    fn render_send_button(&self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        self.active_conversation_editor
            .as_ref()
            .map(|conversation| {
                let focus_handle = conversation.editor.focus_handle(cx);
                ButtonLike::new("send_button")
                    .style(ButtonStyle::Filled)
                    .layer(ElevationIndex::ModalSurface)
                    .children(
                        KeyBinding::for_action_in(&Assist, &focus_handle, cx)
                            .map(|binding| binding.into_any_element()),
                    )
                    .child(Label::new("Send"))
                    .on_click(cx.listener(|this, _event, cx| {
                        if let Some(active_editor) = this.active_conversation_editor() {
                            active_editor.update(cx, |editor, cx| editor.assist(&Assist, cx));
                        }
                    }))
            })
    }

    fn render_saved_conversation(
        &mut self,
        index: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let conversation = &self.saved_conversations[index];
        let path = conversation.path.clone();

        ButtonLike::new(index)
            .on_click(cx.listener(move |this, _, cx| {
                this.open_conversation(path.clone(), cx)
                    .detach_and_log_err(cx)
            }))
            .full_width()
            .child(
                div()
                    .flex()
                    .w_full()
                    .gap_2()
                    .child(
                        Label::new(conversation.mtime.format("%F %I:%M%p").to_string())
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .child(Label::new(conversation.title.clone()).size(LabelSize::Small)),
            )
    }

    fn open_conversation(&mut self, path: PathBuf, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        cx.focus(&self.focus_handle);

        let fs = self.fs.clone();
        let workspace = self.workspace.clone();
        let slash_commands = self.slash_commands.clone();
        let languages = self.languages.clone();
        let telemetry = self.telemetry.clone();

        let lsp_adapter_delegate = workspace
            .update(cx, |workspace, cx| {
                make_lsp_adapter_delegate(workspace.project(), cx).log_err()
            })
            .log_err()
            .flatten();

        cx.spawn(|this, mut cx| async move {
            let saved_conversation = SavedConversation::load(&path, fs.as_ref()).await?;
            let conversation = Conversation::deserialize(
                saved_conversation,
                path.clone(),
                languages,
                slash_commands,
                Some(telemetry),
                &mut cx,
            )
            .await?;

            this.update(&mut cx, |this, cx| {
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow!("workspace dropped"))?;
                let editor = cx.new_view(|cx| {
                    ConversationEditor::for_conversation(
                        conversation,
                        fs,
                        workspace,
                        lsp_adapter_delegate,
                        cx,
                    )
                });
                this.show_conversation(editor, cx);
                anyhow::Ok(())
            })??;
            Ok(())
        })
    }

    fn show_prompt_manager(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(cx, |cx| {
                    PromptManager::new(
                        self.prompt_library.clone(),
                        self.languages.clone(),
                        self.fs.clone(),
                        cx,
                    )
                })
            })
        }
    }

    fn is_authenticated(&mut self, cx: &mut ViewContext<Self>) -> bool {
        CompletionProvider::global(cx).is_authenticated()
    }

    fn authenticate(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        cx.update_global::<CompletionProvider, _>(|provider, cx| provider.authenticate(cx))
    }

    fn render_signed_in(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let header = TabBar::new("assistant_header")
            .start_child(h_flex().gap_1().child(self.render_popover_button(cx)))
            .children(self.active_conversation_editor().map(|editor| {
                h_flex()
                    .h(rems(Tab::CONTAINER_HEIGHT_IN_REMS))
                    .flex_1()
                    .px_2()
                    .child(Label::new(editor.read(cx).title(cx)).into_element())
            }))
            .end_child(
                h_flex()
                    .gap_2()
                    .when_some(self.active_conversation_editor(), |this, editor| {
                        let conversation = editor.read(cx).conversation.clone();
                        this.child(
                            h_flex()
                                .gap_1()
                                .child(ModelSelector::new(
                                    self.model_menu_handle.clone(),
                                    self.fs.clone(),
                                ))
                                .children(self.render_remaining_tokens(&conversation, cx)),
                        )
                        .child(
                            ui::Divider::vertical()
                                .inset()
                                .color(ui::DividerColor::Border),
                        )
                    })
                    .child(
                        h_flex()
                            .gap_1()
                            .child(self.render_inject_context_menu(cx))
                            .children(
                                cx.has_flag::<PromptLibraryFeatureFlag>().then_some(
                                    IconButton::new("show_prompt_manager", IconName::Library)
                                        .icon_size(IconSize::Small)
                                        .on_click(cx.listener(|this, _event, cx| {
                                            this.show_prompt_manager(cx)
                                        }))
                                        .tooltip(|cx| Tooltip::text("Prompt Library…", cx)),
                                ),
                            ),
                    ),
            );

        let contents = if self.active_conversation_editor().is_some() {
            let mut registrar = DivRegistrar::new(
                |panel, cx| panel.toolbar.read(cx).item_of_type::<BufferSearchBar>(),
                cx,
            );
            BufferSearchBar::register(&mut registrar);
            registrar.into_div()
        } else {
            div()
        };

        v_flex()
            .key_context("AssistantPanel")
            .size_full()
            .on_action(cx.listener(|this, _: &workspace::NewFile, cx| {
                this.new_conversation(cx);
            }))
            .on_action(cx.listener(AssistantPanel::toggle_zoom))
            .on_action(cx.listener(AssistantPanel::toggle_history))
            .on_action(cx.listener(AssistantPanel::deploy))
            .on_action(cx.listener(AssistantPanel::select_next_match))
            .on_action(cx.listener(AssistantPanel::select_prev_match))
            .on_action(cx.listener(AssistantPanel::handle_editor_cancel))
            .on_action(cx.listener(AssistantPanel::reset_credentials))
            .on_action(cx.listener(AssistantPanel::toggle_model_selector))
            .track_focus(&self.focus_handle)
            .child(header)
            .children(if self.toolbar.read(cx).hidden() {
                None
            } else {
                Some(self.toolbar.clone())
            })
            .child(contents.flex_1().child(
                if self.show_saved_conversations || self.active_conversation_editor().is_none() {
                    let view = cx.view().clone();
                    let scroll_handle = self.saved_conversations_scroll_handle.clone();
                    let conversation_count = self.saved_conversations.len();
                    canvas(
                        move |bounds, cx| {
                            let mut saved_conversations = uniform_list(
                                view,
                                "saved_conversations",
                                conversation_count,
                                |this, range, cx| {
                                    range
                                        .map(|ix| this.render_saved_conversation(ix, cx))
                                        .collect()
                                },
                            )
                            .track_scroll(scroll_handle)
                            .into_any_element();
                            saved_conversations.prepaint_as_root(
                                bounds.origin,
                                bounds.size.map(AvailableSpace::Definite),
                                cx,
                            );
                            saved_conversations
                        },
                        |_bounds, mut saved_conversations, cx| saved_conversations.paint(cx),
                    )
                    .size_full()
                    .into_any_element()
                } else if let Some(editor) = self.active_conversation_editor() {
                    let editor = editor.clone();
                    div()
                        .size_full()
                        .child(editor.clone())
                        .child(
                            h_flex()
                                .w_full()
                                .absolute()
                                .bottom_0()
                                .p_4()
                                .justify_end()
                                .children(self.render_send_button(cx)),
                        )
                        .into_any_element()
                } else {
                    div().into_any_element()
                },
            ))
    }

    fn render_remaining_tokens(
        &self,
        conversation: &Model<Conversation>,
        cx: &mut ViewContext<Self>,
    ) -> Option<impl IntoElement> {
        let remaining_tokens = conversation.read(cx).remaining_tokens(cx)?;
        let remaining_tokens_color = if remaining_tokens <= 0 {
            Color::Error
        } else if remaining_tokens <= 500 {
            Color::Warning
        } else {
            Color::Muted
        };
        Some(
            Label::new(remaining_tokens.to_string())
                .size(LabelSize::Small)
                .color(remaining_tokens_color),
        )
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if let Some(authentication_prompt) = self.authentication_prompt.as_ref() {
            authentication_prompt.clone().into_any()
        } else {
            self.render_signed_in(cx).into_any_element()
        }
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
        settings::update_settings_file::<AssistantSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                DockPosition::Left => AssistantDockPosition::Left,
                DockPosition::Bottom => AssistantDockPosition::Bottom,
                DockPosition::Right => AssistantDockPosition::Right,
            };
            settings.set_dock(dock);
        });
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

    fn is_zoomed(&self, _: &WindowContext) -> bool {
        self.zoomed
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.zoomed = zoomed;
        cx.notify();
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active {
            let load_credentials = self.authenticate(cx);
            cx.spawn(|this, mut cx| async move {
                load_credentials.await?;
                this.update(&mut cx, |this, cx| {
                    if this.is_authenticated(cx) && this.active_conversation_editor().is_none() {
                        this.new_conversation(cx);
                    }
                })
            })
            .detach_and_log_err(cx);
        }
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled || !settings.button {
            return None;
        }

        Some(IconName::Ai)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Clone)]
enum ConversationEvent {
    MessagesEdited,
    SummaryChanged,
    EditSuggestionsChanged,
    StreamedCompletion,
    PendingSlashCommandsUpdated {
        removed: Vec<Range<language::Anchor>>,
        updated: Vec<PendingSlashCommand>,
    },
    SlashCommandFinished {
        sections: Vec<SlashCommandOutputSection<language::Anchor>>,
    },
}

#[derive(Default)]
struct Summary {
    text: String,
    done: bool,
}

pub struct Conversation {
    id: Option<String>,
    buffer: Model<Buffer>,
    edit_suggestions: Vec<EditSuggestion>,
    pending_slash_commands: Vec<PendingSlashCommand>,
    edits_since_last_slash_command_parse: language::Subscription,
    message_anchors: Vec<MessageAnchor>,
    messages_metadata: HashMap<MessageId, MessageMetadata>,
    next_message_id: MessageId,
    summary: Option<Summary>,
    pending_summary: Task<Option<()>>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    token_count: Option<usize>,
    pending_token_count: Task<Option<()>>,
    pending_edit_suggestion_parse: Option<Task<()>>,
    pending_save: Task<Result<()>>,
    path: Option<PathBuf>,
    _subscriptions: Vec<Subscription>,
    telemetry: Option<Arc<Telemetry>>,
    slash_command_registry: Arc<SlashCommandRegistry>,
    language_registry: Arc<LanguageRegistry>,
}

impl EventEmitter<ConversationEvent> for Conversation {}

impl Conversation {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        slash_command_registry: Arc<SlashCommandRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let buffer = cx.new_model(|cx| {
            let mut buffer = Buffer::local("", cx);
            buffer.set_language_registry(language_registry.clone());
            buffer
        });
        let edits_since_last_slash_command_parse =
            buffer.update(cx, |buffer, _| buffer.subscribe());
        let mut this = Self {
            id: Some(Uuid::new_v4().to_string()),
            message_anchors: Default::default(),
            messages_metadata: Default::default(),
            next_message_id: Default::default(),
            edit_suggestions: Vec::new(),
            pending_slash_commands: Vec::new(),
            edits_since_last_slash_command_parse,
            summary: None,
            pending_summary: Task::ready(None),
            completion_count: Default::default(),
            pending_completions: Default::default(),
            token_count: None,
            pending_token_count: Task::ready(None),
            pending_edit_suggestion_parse: None,
            _subscriptions: vec![cx.subscribe(&buffer, Self::handle_buffer_event)],
            pending_save: Task::ready(Ok(())),
            path: None,
            buffer,
            telemetry,
            language_registry,
            slash_command_registry,
        };

        let message = MessageAnchor {
            id: MessageId(post_inc(&mut this.next_message_id.0)),
            start: language::Anchor::MIN,
        };
        this.message_anchors.push(message.clone());
        this.messages_metadata.insert(
            message.id,
            MessageMetadata {
                role: Role::User,
                status: MessageStatus::Done,
            },
        );

        this.set_language(cx);
        this.count_remaining_tokens(cx);
        this
    }

    fn serialize(&self, cx: &AppContext) -> SavedConversation {
        SavedConversation {
            id: self.id.clone(),
            zed: "conversation".into(),
            version: SavedConversation::VERSION.into(),
            text: self.buffer.read(cx).text(),
            message_metadata: self.messages_metadata.clone(),
            messages: self
                .messages(cx)
                .map(|message| SavedMessage {
                    id: message.id,
                    start: message.offset_range.start,
                })
                .collect(),
            summary: self
                .summary
                .as_ref()
                .map(|summary| summary.text.clone())
                .unwrap_or_default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn deserialize(
        saved_conversation: SavedConversation,
        path: PathBuf,
        language_registry: Arc<LanguageRegistry>,
        slash_command_registry: Arc<SlashCommandRegistry>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut AsyncAppContext,
    ) -> Result<Model<Self>> {
        let id = match saved_conversation.id {
            Some(id) => Some(id),
            None => Some(Uuid::new_v4().to_string()),
        };

        let markdown = language_registry.language_for_name("Markdown");
        let mut message_anchors = Vec::new();
        let mut next_message_id = MessageId(0);
        let buffer = cx.new_model(|cx| {
            let mut buffer = Buffer::local(saved_conversation.text, cx);
            for message in saved_conversation.messages {
                message_anchors.push(MessageAnchor {
                    id: message.id,
                    start: buffer.anchor_before(message.start),
                });
                next_message_id = cmp::max(next_message_id, MessageId(message.id.0 + 1));
            }
            buffer.set_language_registry(language_registry.clone());
            cx.spawn(|buffer, mut cx| async move {
                let markdown = markdown.await?;
                buffer.update(&mut cx, |buffer: &mut Buffer, cx| {
                    buffer.set_language(Some(markdown), cx)
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            buffer
        })?;

        cx.new_model(move |cx| {
            let edits_since_last_slash_command_parse =
                buffer.update(cx, |buffer, _| buffer.subscribe());
            let mut this = Self {
                id,
                message_anchors,
                messages_metadata: saved_conversation.message_metadata,
                next_message_id,
                edit_suggestions: Vec::new(),
                pending_slash_commands: Vec::new(),
                edits_since_last_slash_command_parse,
                summary: Some(Summary {
                    text: saved_conversation.summary,
                    done: true,
                }),
                pending_summary: Task::ready(None),
                completion_count: Default::default(),
                pending_completions: Default::default(),
                token_count: None,
                pending_edit_suggestion_parse: None,
                pending_token_count: Task::ready(None),
                _subscriptions: vec![cx.subscribe(&buffer, Self::handle_buffer_event)],
                pending_save: Task::ready(Ok(())),
                path: Some(path),
                buffer,
                telemetry,
                language_registry,
                slash_command_registry,
            };
            this.set_language(cx);
            this.reparse_edit_suggestions(cx);
            this.count_remaining_tokens(cx);
            this
        })
    }

    fn set_language(&mut self, cx: &mut ModelContext<Self>) {
        let markdown = self.language_registry.language_for_name("Markdown");
        cx.spawn(|this, mut cx| async move {
            let markdown = markdown.await?;
            this.update(&mut cx, |this, cx| {
                this.buffer
                    .update(cx, |buffer, cx| buffer.set_language(Some(markdown), cx));
            })
        })
        .detach_and_log_err(cx);
    }

    fn handle_buffer_event(
        &mut self,
        _: Model<Buffer>,
        event: &language::Event,
        cx: &mut ModelContext<Self>,
    ) {
        if *event == language::Event::Edited {
            self.count_remaining_tokens(cx);
            self.reparse_edit_suggestions(cx);
            self.reparse_slash_commands(cx);
            cx.emit(ConversationEvent::MessagesEdited);
        }
    }

    pub(crate) fn count_remaining_tokens(&mut self, cx: &mut ModelContext<Self>) {
        let request = self.to_completion_request(cx);
        self.pending_token_count = cx.spawn(|this, mut cx| {
            async move {
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;

                let token_count = cx
                    .update(|cx| CompletionProvider::global(cx).count_tokens(request, cx))?
                    .await?;

                this.update(&mut cx, |this, cx| {
                    this.token_count = Some(token_count);
                    cx.notify()
                })?;
                anyhow::Ok(())
            }
            .log_err()
        });
    }

    fn reparse_slash_commands(&mut self, cx: &mut ModelContext<Self>) {
        let buffer = self.buffer.read(cx);
        let mut row_ranges = self
            .edits_since_last_slash_command_parse
            .consume()
            .into_iter()
            .map(|edit| {
                let start_row = buffer.offset_to_point(edit.new.start).row;
                let end_row = buffer.offset_to_point(edit.new.end).row + 1;
                start_row..end_row
            })
            .peekable();

        let mut removed = Vec::new();
        let mut updated = Vec::new();
        while let Some(mut row_range) = row_ranges.next() {
            while let Some(next_row_range) = row_ranges.peek() {
                if row_range.end >= next_row_range.start {
                    row_range.end = next_row_range.end;
                    row_ranges.next();
                } else {
                    break;
                }
            }

            let start = buffer.anchor_before(Point::new(row_range.start, 0));
            let end = buffer.anchor_after(Point::new(
                row_range.end - 1,
                buffer.line_len(row_range.end - 1),
            ));

            let start_ix = match self
                .pending_slash_commands
                .binary_search_by(|probe| probe.source_range.start.cmp(&start, buffer))
            {
                Ok(ix) | Err(ix) => ix,
            };
            let end_ix = match self.pending_slash_commands[start_ix..]
                .binary_search_by(|probe| probe.source_range.end.cmp(&end, buffer))
            {
                Ok(ix) => start_ix + ix + 1,
                Err(ix) => start_ix + ix,
            };

            let mut new_commands = Vec::new();
            let mut lines = buffer.text_for_range(start..end).lines();
            let mut offset = lines.offset();
            while let Some(line) = lines.next() {
                if let Some(command_line) = SlashCommandLine::parse(line) {
                    let name = &line[command_line.name.clone()];
                    let argument = command_line.argument.as_ref().and_then(|argument| {
                        (!argument.is_empty()).then_some(&line[argument.clone()])
                    });
                    if let Some(command) = self.slash_command_registry.command(name) {
                        if !command.requires_argument() || argument.is_some() {
                            let start_ix = offset + command_line.name.start - 1;
                            let end_ix = offset
                                + command_line
                                    .argument
                                    .map_or(command_line.name.end, |argument| argument.end);
                            let source_range =
                                buffer.anchor_after(start_ix)..buffer.anchor_after(end_ix);
                            let pending_command = PendingSlashCommand {
                                name: name.to_string(),
                                argument: argument.map(ToString::to_string),
                                source_range,
                                status: PendingSlashCommandStatus::Idle,
                            };
                            updated.push(pending_command.clone());
                            new_commands.push(pending_command);
                        }
                    }
                }

                offset = lines.offset();
            }

            let removed_commands = self
                .pending_slash_commands
                .splice(start_ix..end_ix, new_commands);
            removed.extend(removed_commands.map(|command| command.source_range));
        }

        if !updated.is_empty() || !removed.is_empty() {
            cx.emit(ConversationEvent::PendingSlashCommandsUpdated { removed, updated });
        }
    }

    fn reparse_edit_suggestions(&mut self, cx: &mut ModelContext<Self>) {
        self.pending_edit_suggestion_parse = Some(cx.spawn(|this, mut cx| async move {
            cx.background_executor()
                .timer(Duration::from_millis(200))
                .await;

            this.update(&mut cx, |this, cx| {
                this.reparse_edit_suggestions_in_range(0..this.buffer.read(cx).len(), cx);
            })
            .ok();
        }));
    }

    fn reparse_edit_suggestions_in_range(
        &mut self,
        range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) {
        self.buffer.update(cx, |buffer, _| {
            let range_start = buffer.anchor_before(range.start);
            let range_end = buffer.anchor_after(range.end);
            let start_ix = self
                .edit_suggestions
                .binary_search_by(|probe| {
                    probe
                        .source_range
                        .end
                        .cmp(&range_start, buffer)
                        .then(Ordering::Greater)
                })
                .unwrap_err();
            let end_ix = self
                .edit_suggestions
                .binary_search_by(|probe| {
                    probe
                        .source_range
                        .start
                        .cmp(&range_end, buffer)
                        .then(Ordering::Less)
                })
                .unwrap_err();

            let mut new_edit_suggestions = Vec::new();
            let mut message_lines = buffer.as_rope().chunks_in_range(range).lines();
            while let Some(suggestion) = parse_next_edit_suggestion(&mut message_lines) {
                let start_anchor = buffer.anchor_after(suggestion.outer_range.start);
                let end_anchor = buffer.anchor_before(suggestion.outer_range.end);
                new_edit_suggestions.push(EditSuggestion {
                    source_range: start_anchor..end_anchor,
                    full_path: suggestion.path,
                });
            }
            self.edit_suggestions
                .splice(start_ix..end_ix, new_edit_suggestions);
        });
        cx.emit(ConversationEvent::EditSuggestionsChanged);
        cx.notify();
    }

    fn pending_command_for_position(
        &mut self,
        position: language::Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Option<&mut PendingSlashCommand> {
        let buffer = self.buffer.read(cx);
        let ix = self
            .pending_slash_commands
            .binary_search_by(|probe| {
                if probe.source_range.start.cmp(&position, buffer).is_gt() {
                    Ordering::Less
                } else if probe.source_range.end.cmp(&position, buffer).is_lt() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .ok()?;
        self.pending_slash_commands.get_mut(ix)
    }

    fn insert_command_output(
        &mut self,
        command_range: Range<language::Anchor>,
        output: Task<Result<SlashCommandOutput>>,
        cx: &mut ModelContext<Self>,
    ) {
        self.reparse_slash_commands(cx);

        let insert_output_task = cx.spawn(|this, mut cx| {
            let command_range = command_range.clone();
            async move {
                let output = output.await;
                this.update(&mut cx, |this, cx| match output {
                    Ok(mut output) => {
                        if !output.text.ends_with('\n') {
                            output.text.push('\n');
                        }

                        let sections = this.buffer.update(cx, |buffer, cx| {
                            let start = command_range.start.to_offset(buffer);
                            let old_end = command_range.end.to_offset(buffer);
                            buffer.edit([(start..old_end, output.text)], None, cx);

                            let mut sections = output
                                .sections
                                .into_iter()
                                .map(|section| SlashCommandOutputSection {
                                    range: buffer.anchor_after(start + section.range.start)
                                        ..buffer.anchor_before(start + section.range.end),
                                    render_placeholder: section.render_placeholder,
                                })
                                .collect::<Vec<_>>();
                            sections.sort_by(|a, b| a.range.cmp(&b.range, buffer));
                            sections
                        });
                        cx.emit(ConversationEvent::SlashCommandFinished { sections });
                    }
                    Err(error) => {
                        if let Some(pending_command) =
                            this.pending_command_for_position(command_range.start, cx)
                        {
                            pending_command.status =
                                PendingSlashCommandStatus::Error(error.to_string());
                            cx.emit(ConversationEvent::PendingSlashCommandsUpdated {
                                removed: vec![pending_command.source_range.clone()],
                                updated: vec![pending_command.clone()],
                            });
                        }
                    }
                })
                .ok();
            }
        });

        if let Some(pending_command) = self.pending_command_for_position(command_range.start, cx) {
            pending_command.status = PendingSlashCommandStatus::Running {
                _task: insert_output_task.shared(),
            };
            cx.emit(ConversationEvent::PendingSlashCommandsUpdated {
                removed: vec![pending_command.source_range.clone()],
                updated: vec![pending_command.clone()],
            });
        }
    }

    fn remaining_tokens(&self, cx: &AppContext) -> Option<isize> {
        let model = CompletionProvider::global(cx).model();
        Some(model.max_token_count() as isize - self.token_count? as isize)
    }

    fn completion_provider_changed(&mut self, cx: &mut ModelContext<Self>) {
        self.count_remaining_tokens(cx);
    }

    fn assist(
        &mut self,
        selected_messages: HashSet<MessageId>,
        cx: &mut ModelContext<Self>,
    ) -> Vec<MessageAnchor> {
        let mut user_messages = Vec::new();

        let last_message_id = if let Some(last_message_id) =
            self.message_anchors.iter().rev().find_map(|message| {
                message
                    .start
                    .is_valid(self.buffer.read(cx))
                    .then_some(message.id)
            }) {
            last_message_id
        } else {
            return Default::default();
        };

        let mut should_assist = false;
        for selected_message_id in selected_messages {
            let selected_message_role =
                if let Some(metadata) = self.messages_metadata.get(&selected_message_id) {
                    metadata.role
                } else {
                    continue;
                };

            if selected_message_role == Role::Assistant {
                if let Some(user_message) = self.insert_message_after(
                    selected_message_id,
                    Role::User,
                    MessageStatus::Done,
                    cx,
                ) {
                    user_messages.push(user_message);
                }
            } else {
                should_assist = true;
            }
        }

        if should_assist {
            if !CompletionProvider::global(cx).is_authenticated() {
                log::info!("completion provider has no credentials");
                return Default::default();
            }

            let request = self.to_completion_request(cx);
            let stream = CompletionProvider::global(cx).complete(request);
            let assistant_message = self
                .insert_message_after(last_message_id, Role::Assistant, MessageStatus::Pending, cx)
                .unwrap();

            // Queue up the user's next reply.
            let user_message = self
                .insert_message_after(assistant_message.id, Role::User, MessageStatus::Done, cx)
                .unwrap();
            user_messages.push(user_message);

            let task = cx.spawn({
                |this, mut cx| async move {
                    let assistant_message_id = assistant_message.id;
                    let mut response_latency = None;
                    let stream_completion = async {
                        let request_start = Instant::now();
                        let mut messages = stream.await?;

                        while let Some(message) = messages.next().await {
                            if response_latency.is_none() {
                                response_latency = Some(request_start.elapsed());
                            }
                            let text = message?;

                            this.update(&mut cx, |this, cx| {
                                let message_ix = this
                                    .message_anchors
                                    .iter()
                                    .position(|message| message.id == assistant_message_id)?;
                                let message_range = this.buffer.update(cx, |buffer, cx| {
                                    let message_start_offset =
                                        this.message_anchors[message_ix].start.to_offset(buffer);
                                    let message_old_end_offset = this.message_anchors
                                        [message_ix + 1..]
                                        .iter()
                                        .find(|message| message.start.is_valid(buffer))
                                        .map_or(buffer.len(), |message| {
                                            message.start.to_offset(buffer).saturating_sub(1)
                                        });
                                    let message_new_end_offset =
                                        message_old_end_offset + text.len();
                                    buffer.edit(
                                        [(message_old_end_offset..message_old_end_offset, text)],
                                        None,
                                        cx,
                                    );
                                    message_start_offset..message_new_end_offset
                                });
                                this.reparse_edit_suggestions_in_range(message_range, cx);
                                cx.emit(ConversationEvent::StreamedCompletion);

                                Some(())
                            })?;
                            smol::future::yield_now().await;
                        }

                        this.update(&mut cx, |this, cx| {
                            this.pending_completions
                                .retain(|completion| completion.id != this.completion_count);
                            this.summarize(cx);
                        })?;

                        anyhow::Ok(())
                    };

                    let result = stream_completion.await;

                    this.update(&mut cx, |this, cx| {
                        if let Some(metadata) =
                            this.messages_metadata.get_mut(&assistant_message.id)
                        {
                            let error_message = result
                                .err()
                                .map(|error| error.to_string().trim().to_string());
                            if let Some(error_message) = error_message.as_ref() {
                                metadata.status =
                                    MessageStatus::Error(SharedString::from(error_message.clone()));
                            } else {
                                metadata.status = MessageStatus::Done;
                            }

                            if let Some(telemetry) = this.telemetry.as_ref() {
                                let model = CompletionProvider::global(cx).model();
                                telemetry.report_assistant_event(
                                    this.id.clone(),
                                    AssistantKind::Panel,
                                    model.telemetry_id(),
                                    response_latency,
                                    error_message,
                                );
                            }

                            cx.emit(ConversationEvent::MessagesEdited);
                        }
                    })
                    .ok();
                }
            });

            self.pending_completions.push(PendingCompletion {
                id: post_inc(&mut self.completion_count),
                _task: task,
            });
        }

        user_messages
    }

    fn to_completion_request(&self, cx: &mut ModelContext<Conversation>) -> LanguageModelRequest {
        let messages = self
            .messages(cx)
            .filter(|message| matches!(message.status, MessageStatus::Done))
            .map(|message| message.to_request_message(self.buffer.read(cx)));

        LanguageModelRequest {
            model: CompletionProvider::global(cx).model(),
            messages: messages.collect(),
            stop: vec![],
            temperature: 1.0,
        }
    }

    fn cancel_last_assist(&mut self) -> bool {
        self.pending_completions.pop().is_some()
    }

    fn cycle_message_roles(&mut self, ids: HashSet<MessageId>, cx: &mut ModelContext<Self>) {
        for id in ids {
            if let Some(metadata) = self.messages_metadata.get_mut(&id) {
                metadata.role.cycle();
                cx.emit(ConversationEvent::MessagesEdited);
                cx.notify();
            }
        }
    }

    fn insert_message_after(
        &mut self,
        message_id: MessageId,
        role: Role,
        status: MessageStatus,
        cx: &mut ModelContext<Self>,
    ) -> Option<MessageAnchor> {
        if let Some(prev_message_ix) = self
            .message_anchors
            .iter()
            .position(|message| message.id == message_id)
        {
            // Find the next valid message after the one we were given.
            let mut next_message_ix = prev_message_ix + 1;
            while let Some(next_message) = self.message_anchors.get(next_message_ix) {
                if next_message.start.is_valid(self.buffer.read(cx)) {
                    break;
                }
                next_message_ix += 1;
            }

            let start = self.buffer.update(cx, |buffer, cx| {
                let offset = self
                    .message_anchors
                    .get(next_message_ix)
                    .map_or(buffer.len(), |message| message.start.to_offset(buffer) - 1);
                buffer.edit([(offset..offset, "\n")], None, cx);
                buffer.anchor_before(offset + 1)
            });
            let message = MessageAnchor {
                id: MessageId(post_inc(&mut self.next_message_id.0)),
                start,
            };
            self.message_anchors
                .insert(next_message_ix, message.clone());
            self.messages_metadata
                .insert(message.id, MessageMetadata { role, status });
            cx.emit(ConversationEvent::MessagesEdited);
            Some(message)
        } else {
            None
        }
    }

    fn split_message(
        &mut self,
        range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) -> (Option<MessageAnchor>, Option<MessageAnchor>) {
        let start_message = self.message_for_offset(range.start, cx);
        let end_message = self.message_for_offset(range.end, cx);
        if let Some((start_message, end_message)) = start_message.zip(end_message) {
            // Prevent splitting when range spans multiple messages.
            if start_message.id != end_message.id {
                return (None, None);
            }

            let message = start_message;
            let role = message.role;
            let mut edited_buffer = false;

            let mut suffix_start = None;
            if range.start > message.offset_range.start && range.end < message.offset_range.end - 1
            {
                if self.buffer.read(cx).chars_at(range.end).next() == Some('\n') {
                    suffix_start = Some(range.end + 1);
                } else if self.buffer.read(cx).reversed_chars_at(range.end).next() == Some('\n') {
                    suffix_start = Some(range.end);
                }
            }

            let suffix = if let Some(suffix_start) = suffix_start {
                MessageAnchor {
                    id: MessageId(post_inc(&mut self.next_message_id.0)),
                    start: self.buffer.read(cx).anchor_before(suffix_start),
                }
            } else {
                self.buffer.update(cx, |buffer, cx| {
                    buffer.edit([(range.end..range.end, "\n")], None, cx);
                });
                edited_buffer = true;
                MessageAnchor {
                    id: MessageId(post_inc(&mut self.next_message_id.0)),
                    start: self.buffer.read(cx).anchor_before(range.end + 1),
                }
            };

            self.message_anchors
                .insert(message.index_range.end + 1, suffix.clone());
            self.messages_metadata.insert(
                suffix.id,
                MessageMetadata {
                    role,
                    status: MessageStatus::Done,
                },
            );

            let new_messages =
                if range.start == range.end || range.start == message.offset_range.start {
                    (None, Some(suffix))
                } else {
                    let mut prefix_end = None;
                    if range.start > message.offset_range.start
                        && range.end < message.offset_range.end - 1
                    {
                        if self.buffer.read(cx).chars_at(range.start).next() == Some('\n') {
                            prefix_end = Some(range.start + 1);
                        } else if self.buffer.read(cx).reversed_chars_at(range.start).next()
                            == Some('\n')
                        {
                            prefix_end = Some(range.start);
                        }
                    }

                    let selection = if let Some(prefix_end) = prefix_end {
                        cx.emit(ConversationEvent::MessagesEdited);
                        MessageAnchor {
                            id: MessageId(post_inc(&mut self.next_message_id.0)),
                            start: self.buffer.read(cx).anchor_before(prefix_end),
                        }
                    } else {
                        self.buffer.update(cx, |buffer, cx| {
                            buffer.edit([(range.start..range.start, "\n")], None, cx)
                        });
                        edited_buffer = true;
                        MessageAnchor {
                            id: MessageId(post_inc(&mut self.next_message_id.0)),
                            start: self.buffer.read(cx).anchor_before(range.end + 1),
                        }
                    };

                    self.message_anchors
                        .insert(message.index_range.end + 1, selection.clone());
                    self.messages_metadata.insert(
                        selection.id,
                        MessageMetadata {
                            role,
                            status: MessageStatus::Done,
                        },
                    );
                    (Some(selection), Some(suffix))
                };

            if !edited_buffer {
                cx.emit(ConversationEvent::MessagesEdited);
            }
            new_messages
        } else {
            (None, None)
        }
    }

    fn summarize(&mut self, cx: &mut ModelContext<Self>) {
        if self.message_anchors.len() >= 2 && self.summary.is_none() {
            if !CompletionProvider::global(cx).is_authenticated() {
                return;
            }

            let messages = self
                .messages(cx)
                .take(2)
                .map(|message| message.to_request_message(self.buffer.read(cx)))
                .chain(Some(LanguageModelRequestMessage {
                    role: Role::User,
                    content: "Summarize the conversation into a short title without punctuation"
                        .into(),
                }));
            let request = LanguageModelRequest {
                model: CompletionProvider::global(cx).model(),
                messages: messages.collect(),
                stop: vec![],
                temperature: 1.0,
            };

            let stream = CompletionProvider::global(cx).complete(request);
            self.pending_summary = cx.spawn(|this, mut cx| {
                async move {
                    let mut messages = stream.await?;

                    while let Some(message) = messages.next().await {
                        let text = message?;
                        this.update(&mut cx, |this, cx| {
                            this.summary
                                .get_or_insert(Default::default())
                                .text
                                .push_str(&text);
                            cx.emit(ConversationEvent::SummaryChanged);
                        })?;
                    }

                    this.update(&mut cx, |this, cx| {
                        if let Some(summary) = this.summary.as_mut() {
                            summary.done = true;
                            cx.emit(ConversationEvent::SummaryChanged);
                        }
                    })?;

                    anyhow::Ok(())
                }
                .log_err()
            });
        }
    }

    fn message_for_offset(&self, offset: usize, cx: &AppContext) -> Option<Message> {
        self.messages_for_offsets([offset], cx).pop()
    }

    fn messages_for_offsets(
        &self,
        offsets: impl IntoIterator<Item = usize>,
        cx: &AppContext,
    ) -> Vec<Message> {
        let mut result = Vec::new();

        let mut messages = self.messages(cx).peekable();
        let mut offsets = offsets.into_iter().peekable();
        let mut current_message = messages.next();
        while let Some(offset) = offsets.next() {
            // Locate the message that contains the offset.
            while current_message.as_ref().map_or(false, |message| {
                !message.offset_range.contains(&offset) && messages.peek().is_some()
            }) {
                current_message = messages.next();
            }
            let Some(message) = current_message.as_ref() else {
                break;
            };

            // Skip offsets that are in the same message.
            while offsets.peek().map_or(false, |offset| {
                message.offset_range.contains(offset) || messages.peek().is_none()
            }) {
                offsets.next();
            }

            result.push(message.clone());
        }
        result
    }

    fn messages<'a>(&'a self, cx: &'a AppContext) -> impl 'a + Iterator<Item = Message> {
        let buffer = self.buffer.read(cx);
        let mut message_anchors = self.message_anchors.iter().enumerate().peekable();
        iter::from_fn(move || {
            if let Some((start_ix, message_anchor)) = message_anchors.next() {
                let metadata = self.messages_metadata.get(&message_anchor.id)?;
                let message_start = message_anchor.start.to_offset(buffer);
                let mut message_end = None;
                let mut end_ix = start_ix;
                while let Some((_, next_message)) = message_anchors.peek() {
                    if next_message.start.is_valid(buffer) {
                        message_end = Some(next_message.start);
                        break;
                    } else {
                        end_ix += 1;
                        message_anchors.next();
                    }
                }
                let message_end = message_end
                    .unwrap_or(language::Anchor::MAX)
                    .to_offset(buffer);

                return Some(Message {
                    index_range: start_ix..end_ix,
                    offset_range: message_start..message_end,
                    id: message_anchor.id,
                    anchor: message_anchor.start,
                    role: metadata.role,
                    status: metadata.status.clone(),
                });
            }
            None
        })
    }

    fn save(
        &mut self,
        debounce: Option<Duration>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Conversation>,
    ) {
        self.pending_save = cx.spawn(|this, mut cx| async move {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }

            let (old_path, summary) = this.read_with(&cx, |this, _| {
                let path = this.path.clone();
                let summary = if let Some(summary) = this.summary.as_ref() {
                    if summary.done {
                        Some(summary.text.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };
                (path, summary)
            })?;

            if let Some(summary) = summary {
                let conversation = this.read_with(&cx, |this, cx| this.serialize(cx))?;
                let path = if let Some(old_path) = old_path {
                    old_path
                } else {
                    let mut discriminant = 1;
                    let mut new_path;
                    loop {
                        new_path = CONVERSATIONS_DIR.join(&format!(
                            "{} - {}.zed.json",
                            summary.trim(),
                            discriminant
                        ));
                        if fs.is_file(&new_path).await {
                            discriminant += 1;
                        } else {
                            break;
                        }
                    }
                    new_path
                };

                fs.create_dir(CONVERSATIONS_DIR.as_ref()).await?;
                fs.atomic_write(path.clone(), serde_json::to_string(&conversation).unwrap())
                    .await?;
                this.update(&mut cx, |this, _| this.path = Some(path))?;
            }

            Ok(())
        });
    }
}

#[derive(Debug)]
enum EditParsingState {
    None,
    InOldText {
        path: PathBuf,
        start_offset: usize,
        old_text_start_offset: usize,
    },
    InNewText {
        path: PathBuf,
        start_offset: usize,
        old_text_range: Range<usize>,
        new_text_start_offset: usize,
    },
}

#[derive(Clone, Debug, PartialEq)]
struct EditSuggestion {
    source_range: Range<language::Anchor>,
    full_path: PathBuf,
}

struct ParsedEditSuggestion {
    path: PathBuf,
    outer_range: Range<usize>,
    old_text_range: Range<usize>,
    new_text_range: Range<usize>,
}

fn parse_next_edit_suggestion(lines: &mut rope::Lines) -> Option<ParsedEditSuggestion> {
    let mut state = EditParsingState::None;
    loop {
        let offset = lines.offset();
        let message_line = lines.next()?;
        match state {
            EditParsingState::None => {
                if let Some(rest) = message_line.strip_prefix("```edit ") {
                    let path = rest.trim();
                    if !path.is_empty() {
                        state = EditParsingState::InOldText {
                            path: PathBuf::from(path),
                            start_offset: offset,
                            old_text_start_offset: lines.offset(),
                        };
                    }
                }
            }
            EditParsingState::InOldText {
                path,
                start_offset,
                old_text_start_offset,
            } => {
                if message_line == "---" {
                    state = EditParsingState::InNewText {
                        path,
                        start_offset,
                        old_text_range: old_text_start_offset..offset,
                        new_text_start_offset: lines.offset(),
                    };
                } else {
                    state = EditParsingState::InOldText {
                        path,
                        start_offset,
                        old_text_start_offset,
                    };
                }
            }
            EditParsingState::InNewText {
                path,
                start_offset,
                old_text_range,
                new_text_start_offset,
            } => {
                if message_line == "```" {
                    return Some(ParsedEditSuggestion {
                        path,
                        outer_range: start_offset..offset + "```".len(),
                        old_text_range,
                        new_text_range: new_text_start_offset..offset,
                    });
                } else {
                    state = EditParsingState::InNewText {
                        path,
                        start_offset,
                        old_text_range,
                        new_text_start_offset,
                    };
                }
            }
        }
    }
}

#[derive(Clone)]
struct PendingSlashCommand {
    name: String,
    argument: Option<String>,
    status: PendingSlashCommandStatus,
    source_range: Range<language::Anchor>,
}

#[derive(Clone)]
enum PendingSlashCommandStatus {
    Idle,
    Running { _task: Shared<Task<()>> },
    Error(String),
}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}

enum ConversationEditorEvent {
    TabContentChanged,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ScrollPosition {
    offset_before_cursor: gpui::Point<f32>,
    cursor: Anchor,
}

pub struct ConversationEditor {
    conversation: Model<Conversation>,
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    slash_command_registry: Arc<SlashCommandRegistry>,
    lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
    editor: View<Editor>,
    blocks: HashSet<BlockId>,
    scroll_position: Option<ScrollPosition>,
    pending_slash_command_flaps: HashMap<Range<language::Anchor>, FlapId>,
    _subscriptions: Vec<Subscription>,
}

impl ConversationEditor {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        slash_command_registry: Arc<SlashCommandRegistry>,
        fs: Arc<dyn Fs>,
        workspace: View<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let telemetry = workspace.read(cx).client().telemetry().clone();
        let project = workspace.read(cx).project().clone();
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&project, cx).log_err();

        let conversation = cx.new_model(|cx| {
            Conversation::new(
                language_registry,
                slash_command_registry,
                Some(telemetry),
                cx,
            )
        });

        Self::for_conversation(conversation, fs, workspace, lsp_adapter_delegate, cx)
    }

    fn for_conversation(
        conversation: Model<Conversation>,
        fs: Arc<dyn Fs>,
        workspace: View<Workspace>,
        lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let slash_command_registry = conversation.read(cx).slash_command_registry.clone();

        let completion_provider = SlashCommandCompletionProvider::new(
            cx.view().downgrade(),
            slash_command_registry.clone(),
            workspace.downgrade(),
        );

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::for_buffer(conversation.read(cx).buffer.clone(), None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Box::new(completion_provider));
            editor
        });

        let _subscriptions = vec![
            cx.observe(&conversation, |_, _, cx| cx.notify()),
            cx.subscribe(&conversation, Self::handle_conversation_event),
            cx.subscribe(&editor, Self::handle_editor_event),
        ];

        let mut this = Self {
            conversation,
            editor,
            slash_command_registry,
            lsp_adapter_delegate,
            blocks: Default::default(),
            scroll_position: None,
            fs,
            workspace: workspace.downgrade(),
            pending_slash_command_flaps: HashMap::default(),
            _subscriptions,
        };
        this.update_message_headers(cx);
        this
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        let cursors = self.cursors(cx);

        let user_messages = self.conversation.update(cx, |conversation, cx| {
            let selected_messages = conversation
                .messages_for_offsets(cursors, cx)
                .into_iter()
                .map(|message| message.id)
                .collect();
            conversation.assist(selected_messages, cx)
        });
        let new_selections = user_messages
            .iter()
            .map(|message| {
                let cursor = message
                    .start
                    .to_offset(self.conversation.read(cx).buffer.read(cx));
                cursor..cursor
            })
            .collect::<Vec<_>>();
        if !new_selections.is_empty() {
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    Some(Autoscroll::Strategy(AutoscrollStrategy::Fit)),
                    cx,
                    |selections| selections.select_ranges(new_selections),
                );
            });
            // Avoid scrolling to the new cursor position so the assistant's output is stable.
            cx.defer(|this, _| this.scroll_position = None);
        }
    }

    fn cancel_last_assist(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        if !self
            .conversation
            .update(cx, |conversation, _| conversation.cancel_last_assist())
        {
            cx.propagate();
        }
    }

    fn cycle_message_role(&mut self, _: &CycleMessageRole, cx: &mut ViewContext<Self>) {
        let cursors = self.cursors(cx);
        self.conversation.update(cx, |conversation, cx| {
            let messages = conversation
                .messages_for_offsets(cursors, cx)
                .into_iter()
                .map(|message| message.id)
                .collect();
            conversation.cycle_message_roles(messages, cx)
        });
    }

    fn cursors(&self, cx: &AppContext) -> Vec<usize> {
        let selections = self.editor.read(cx).selections.all::<usize>(cx);
        selections
            .into_iter()
            .map(|selection| selection.head())
            .collect()
    }

    fn insert_command(&mut self, name: &str, cx: &mut ViewContext<Self>) {
        if let Some(command) = self.slash_command_registry.command(name) {
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
                    if command.requires_argument() {
                        editor.insert(" ", cx);
                        editor.show_completions(&ShowCompletions, cx);
                    }
                });
            });
            if !command.requires_argument() {
                self.confirm_command(&ConfirmCommand, cx);
            }
        }
    }

    pub fn confirm_command(&mut self, _: &ConfirmCommand, cx: &mut ViewContext<Self>) {
        let selections = self.editor.read(cx).selections.disjoint_anchors();
        let mut commands_by_range = HashMap::default();
        let workspace = self.workspace.clone();
        self.conversation.update(cx, |conversation, cx| {
            conversation.reparse_slash_commands(cx);
            for selection in selections.iter() {
                if let Some(command) =
                    conversation.pending_command_for_position(selection.head().text_anchor, cx)
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
                    command.argument.as_deref(),
                    workspace.clone(),
                    cx,
                );
            }
            cx.stop_propagation();
        }
    }

    pub fn run_command(
        &mut self,
        command_range: Range<language::Anchor>,
        name: &str,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(command) = self.slash_command_registry.command(name) {
            if let Some(lsp_adapter_delegate) = self.lsp_adapter_delegate.clone() {
                let argument = argument.map(ToString::to_string);
                let output = command.run(argument.as_deref(), workspace, lsp_adapter_delegate, cx);
                self.conversation.update(cx, |conversation, cx| {
                    conversation.insert_command_output(command_range, output, cx)
                });
            }
        }
    }

    fn handle_conversation_event(
        &mut self,
        _: Model<Conversation>,
        event: &ConversationEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let conversation_editor = cx.view().downgrade();

        match event {
            ConversationEvent::MessagesEdited => {
                self.update_message_headers(cx);
                self.conversation.update(cx, |conversation, cx| {
                    conversation.save(Some(Duration::from_millis(500)), self.fs.clone(), cx);
                });
            }
            ConversationEvent::EditSuggestionsChanged => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let excerpt_id = *buffer.as_singleton().unwrap().0;
                    let conversation = self.conversation.read(cx);
                    let highlighted_rows = conversation
                        .edit_suggestions
                        .iter()
                        .map(|suggestion| {
                            let start = buffer
                                .anchor_in_excerpt(excerpt_id, suggestion.source_range.start)
                                .unwrap();
                            let end = buffer
                                .anchor_in_excerpt(excerpt_id, suggestion.source_range.end)
                                .unwrap();
                            start..=end
                        })
                        .collect::<Vec<_>>();

                    editor.clear_row_highlights::<EditSuggestion>();
                    for range in highlighted_rows {
                        editor.highlight_rows::<EditSuggestion>(
                            range,
                            Some(
                                cx.theme()
                                    .colors()
                                    .editor_document_highlight_read_background,
                            ),
                            false,
                            cx,
                        );
                    }
                });
            }
            ConversationEvent::SummaryChanged => {
                cx.emit(ConversationEditorEvent::TabContentChanged);
                self.conversation.update(cx, |conversation, cx| {
                    conversation.save(None, self.fs.clone(), cx);
                });
            }
            ConversationEvent::StreamedCompletion => {
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
            ConversationEvent::PendingSlashCommandsUpdated { removed, updated } => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let excerpt_id = *buffer.as_singleton().unwrap().0;

                    editor.remove_flaps(
                        removed
                            .iter()
                            .filter_map(|range| self.pending_slash_command_flaps.remove(range)),
                        cx,
                    );

                    let flap_ids = editor.insert_flaps(
                        updated.iter().map(|command| {
                            let workspace = self.workspace.clone();
                            let confirm_command = Arc::new({
                                let conversation_editor = conversation_editor.clone();
                                let command = command.clone();
                                move |cx: &mut WindowContext| {
                                    conversation_editor
                                        .update(cx, |conversation_editor, cx| {
                                            conversation_editor.run_command(
                                                command.source_range.clone(),
                                                &command.name,
                                                command.argument.as_deref(),
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
                                        command.status.clone(),
                                        confirm_command.clone(),
                                    )
                                }
                            };
                            let render_trailer = {
                                let confirm_command = confirm_command.clone();
                                move |row, _, cx: &mut WindowContext| {
                                    render_pending_slash_command_trailer(
                                        row,
                                        confirm_command.clone(),
                                        cx,
                                    )
                                }
                            };

                            let start = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.start)
                                .unwrap();
                            let end = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.end)
                                .unwrap();
                            Flap::new(start..end, placeholder, render_toggle, render_trailer)
                        }),
                        cx,
                    );

                    self.pending_slash_command_flaps.extend(
                        updated
                            .iter()
                            .map(|command| command.source_range.clone())
                            .zip(flap_ids),
                    );
                })
            }
            ConversationEvent::SlashCommandFinished { sections } => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let excerpt_id = *buffer.as_singleton().unwrap().0;
                    let mut buffer_rows_to_fold = BTreeSet::new();
                    let mut flaps = Vec::new();
                    for section in sections {
                        let start = buffer
                            .anchor_in_excerpt(excerpt_id, section.range.start)
                            .unwrap();
                        let end = buffer
                            .anchor_in_excerpt(excerpt_id, section.range.end)
                            .unwrap();
                        let buffer_row = MultiBufferRow(start.to_point(&buffer).row);
                        buffer_rows_to_fold.insert(buffer_row);
                        flaps.push(Flap::new(
                            start..end,
                            FoldPlaceholder {
                                render: Arc::new({
                                    let editor = cx.view().downgrade();
                                    let render_placeholder = section.render_placeholder.clone();
                                    move |fold_id, fold_range, cx| {
                                        let editor = editor.clone();
                                        let unfold = Arc::new(move |cx: &mut WindowContext| {
                                            editor
                                                .update(cx, |editor, cx| {
                                                    let buffer_start = fold_range.start.to_point(
                                                        &editor.buffer().read(cx).read(cx),
                                                    );
                                                    let buffer_row =
                                                        MultiBufferRow(buffer_start.row);
                                                    editor.unfold_at(&UnfoldAt { buffer_row }, cx);
                                                })
                                                .ok();
                                        });
                                        render_placeholder(fold_id.into(), unfold, cx)
                                    }
                                }),
                                constrain_width: false,
                                merge_adjacent: false,
                            },
                            render_slash_command_output_toggle,
                            |_, _, _| Empty.into_any_element(),
                        ));
                    }

                    editor.insert_flaps(flaps, cx);

                    for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                        editor.fold_at(&FoldAt { buffer_row }, cx);
                    }
                });
            }
        }
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
            }
            _ => {}
        }
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
            let old_blocks = std::mem::take(&mut self.blocks);
            let new_blocks = self
                .conversation
                .read(cx)
                .messages(cx)
                .map(|message| BlockProperties {
                    position: buffer
                        .anchor_in_excerpt(excerpt_id, message.anchor)
                        .unwrap(),
                    height: 2,
                    style: BlockStyle::Sticky,
                    render: Box::new({
                        let conversation = self.conversation.clone();
                        move |cx| {
                            let message_id = message.id;
                            let sender = ButtonLike::new("role")
                                .style(ButtonStyle::Filled)
                                .child(match message.role {
                                    Role::User => Label::new("You").color(Color::Default),
                                    Role::Assistant => Label::new("Assistant").color(Color::Info),
                                    Role::System => Label::new("System").color(Color::Warning),
                                })
                                .tooltip(|cx| {
                                    Tooltip::with_meta(
                                        "Toggle message role",
                                        None,
                                        "Available roles: You (User), Assistant, System",
                                        cx,
                                    )
                                })
                                .on_click({
                                    let conversation = conversation.clone();
                                    move |_, cx| {
                                        conversation.update(cx, |conversation, cx| {
                                            conversation.cycle_message_roles(
                                                HashSet::from_iter(Some(message_id)),
                                                cx,
                                            )
                                        })
                                    }
                                });

                            h_flex()
                                .id(("message_header", message_id.0))
                                .pl(cx.gutter_dimensions.width + cx.gutter_dimensions.margin)
                                .h_11()
                                .w_full()
                                .relative()
                                .gap_1()
                                .child(sender)
                                .children(
                                    if let MessageStatus::Error(error) = message.status.clone() {
                                        Some(
                                            div()
                                                .id("error")
                                                .tooltip(move |cx| Tooltip::text(error.clone(), cx))
                                                .child(Icon::new(IconName::XCircle)),
                                        )
                                    } else {
                                        None
                                    },
                                )
                                .into_any_element()
                        }
                    }),
                    disposition: BlockDisposition::Above,
                })
                .collect::<Vec<_>>();

            editor.remove_blocks(old_blocks, None, cx);
            let ids = editor.insert_blocks(new_blocks, None, cx);
            self.blocks = HashSet::from_iter(ids);
        });
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

        let editor = editor.read(cx);
        let range = editor.selections.newest::<usize>(cx).range();
        let buffer = editor.buffer().read(cx).snapshot(cx);
        let start_language = buffer.language_at(range.start);
        let end_language = buffer.language_at(range.end);
        let language_name = if start_language == end_language {
            start_language.map(|language| language.code_fence_block_name())
        } else {
            None
        };
        let language_name = language_name.as_deref().unwrap_or("");

        let selected_text = buffer.text_for_range(range).collect::<String>();
        let text = if selected_text.is_empty() {
            None
        } else {
            Some(if language_name == "markdown" {
                selected_text
                    .lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                format!("```{language_name}\n{selected_text}\n```")
            })
        };

        // Activate the panel
        if !panel.focus_handle(cx).contains_focused(cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(cx);
        }

        if let Some(text) = text {
            panel.update(cx, |_, cx| {
                // Wait to create a new conversation until the workspace is no longer
                // being updated.
                cx.defer(move |panel, cx| {
                    if let Some(conversation) = panel
                        .active_conversation_editor()
                        .cloned()
                        .or_else(|| panel.new_conversation(cx))
                    {
                        conversation.update(cx, |conversation, cx| {
                            conversation
                                .editor
                                .update(cx, |editor, cx| editor.insert(&text, cx))
                        });
                    };
                });
            });
        }
    }

    fn copy(&mut self, _: &editor::actions::Copy, cx: &mut ViewContext<Self>) {
        let editor = self.editor.read(cx);
        let conversation = self.conversation.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            let mut copied_text = String::new();
            let mut spanned_messages = 0;
            for message in conversation.messages(cx) {
                if message.offset_range.start >= selection.range().end {
                    break;
                } else if message.offset_range.end >= selection.range().start {
                    let range = cmp::max(message.offset_range.start, selection.range().start)
                        ..cmp::min(message.offset_range.end, selection.range().end);
                    if !range.is_empty() {
                        spanned_messages += 1;
                        write!(&mut copied_text, "## {}\n\n", message.role).unwrap();
                        for chunk in conversation.buffer.read(cx).text_for_range(range) {
                            copied_text.push_str(chunk);
                        }
                        copied_text.push('\n');
                    }
                }
            }

            if spanned_messages > 1 {
                cx.write_to_clipboard(ClipboardItem::new(copied_text));
                return;
            }
        }

        cx.propagate();
    }

    fn split(&mut self, _: &Split, cx: &mut ViewContext<Self>) {
        self.conversation.update(cx, |conversation, cx| {
            let selections = self.editor.read(cx).selections.disjoint_anchors();
            for selection in selections.as_ref() {
                let buffer = self.editor.read(cx).buffer().read(cx).snapshot(cx);
                let range = selection
                    .map(|endpoint| endpoint.to_offset(&buffer))
                    .range();
                conversation.split_message(range, cx);
            }
        });
    }

    fn apply_edit(&mut self, _: &ApplyEdit, cx: &mut ViewContext<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let project = workspace.read(cx).project().clone();

        struct Edit {
            old_text: String,
            new_text: String,
        }

        let conversation = self.conversation.read(cx);
        let conversation_buffer = conversation.buffer.read(cx);
        let conversation_buffer_snapshot = conversation_buffer.snapshot();

        let selections = self.editor.read(cx).selections.disjoint_anchors();
        let mut selections = selections.iter().peekable();
        let selected_suggestions = conversation
            .edit_suggestions
            .iter()
            .filter(|suggestion| {
                while let Some(selection) = selections.peek() {
                    if selection
                        .end
                        .text_anchor
                        .cmp(&suggestion.source_range.start, conversation_buffer)
                        .is_lt()
                    {
                        selections.next();
                        continue;
                    }
                    if selection
                        .start
                        .text_anchor
                        .cmp(&suggestion.source_range.end, conversation_buffer)
                        .is_gt()
                    {
                        break;
                    }
                    return true;
                }
                false
            })
            .cloned()
            .collect::<Vec<_>>();

        let mut opened_buffers: HashMap<PathBuf, Task<Result<Model<Buffer>>>> = HashMap::default();
        project.update(cx, |project, cx| {
            for suggestion in &selected_suggestions {
                opened_buffers
                    .entry(suggestion.full_path.clone())
                    .or_insert_with(|| {
                        project.open_buffer_for_full_path(&suggestion.full_path, cx)
                    });
            }
        });

        cx.spawn(|this, mut cx| async move {
            let mut buffers_by_full_path = HashMap::default();
            for (full_path, buffer) in opened_buffers {
                if let Some(buffer) = buffer.await.log_err() {
                    buffers_by_full_path.insert(full_path, buffer);
                }
            }

            let mut suggestions_by_buffer = HashMap::default();
            cx.update(|cx| {
                for suggestion in selected_suggestions {
                    if let Some(buffer) = buffers_by_full_path.get(&suggestion.full_path) {
                        let (_, edits) = suggestions_by_buffer
                            .entry(buffer.clone())
                            .or_insert_with(|| (buffer.read(cx).snapshot(), Vec::new()));

                        let mut lines = conversation_buffer_snapshot
                            .as_rope()
                            .chunks_in_range(
                                suggestion
                                    .source_range
                                    .to_offset(&conversation_buffer_snapshot),
                            )
                            .lines();
                        if let Some(suggestion) = parse_next_edit_suggestion(&mut lines) {
                            let old_text = conversation_buffer_snapshot
                                .text_for_range(suggestion.old_text_range)
                                .collect();
                            let new_text = conversation_buffer_snapshot
                                .text_for_range(suggestion.new_text_range)
                                .collect();
                            edits.push(Edit { old_text, new_text });
                        }
                    }
                }
            })?;

            let edits_by_buffer = cx
                .background_executor()
                .spawn(async move {
                    let mut result = HashMap::default();
                    for (buffer, (snapshot, suggestions)) in suggestions_by_buffer {
                        let edits =
                            result
                                .entry(buffer)
                                .or_insert(Vec::<(Range<language::Anchor>, _)>::new());
                        for suggestion in suggestions {
                            if let Some(range) =
                                fuzzy_search_lines(snapshot.as_rope(), &suggestion.old_text)
                            {
                                let edit_start = snapshot.anchor_after(range.start);
                                let edit_end = snapshot.anchor_before(range.end);
                                if let Err(ix) = edits.binary_search_by(|(range, _)| {
                                    range.start.cmp(&edit_start, &snapshot)
                                }) {
                                    edits.insert(
                                        ix,
                                        (edit_start..edit_end, suggestion.new_text.clone()),
                                    );
                                }
                            } else {
                                log::info!(
                                    "assistant edit did not match any text in buffer {:?}",
                                    &suggestion.old_text
                                );
                            }
                        }
                    }
                    result
                })
                .await;

            let mut project_transaction = ProjectTransaction::default();
            let (editor, workspace, title) = this.update(&mut cx, |this, cx| {
                for (buffer_handle, edits) in edits_by_buffer {
                    buffer_handle.update(cx, |buffer, cx| {
                        buffer.start_transaction();
                        buffer.edit(
                            edits,
                            Some(AutoindentMode::Block {
                                original_indent_columns: Vec::new(),
                            }),
                            cx,
                        );
                        buffer.end_transaction(cx);
                        if let Some(transaction) = buffer.finalize_last_transaction() {
                            project_transaction
                                .0
                                .insert(buffer_handle.clone(), transaction.clone());
                        }
                    });
                }

                (
                    this.editor.downgrade(),
                    this.workspace.clone(),
                    this.title(cx),
                )
            })?;

            Editor::open_project_transaction(
                &editor,
                workspace,
                project_transaction,
                format!("Edits from {}", title),
                cx,
            )
            .await
        })
        .detach_and_log_err(cx);
    }

    fn save(&mut self, _: &Save, cx: &mut ViewContext<Self>) {
        self.conversation.update(cx, |conversation, cx| {
            conversation.save(None, self.fs.clone(), cx)
        });
    }

    fn title(&self, cx: &AppContext) -> String {
        self.conversation
            .read(cx)
            .summary
            .as_ref()
            .map(|summary| summary.text.clone())
            .unwrap_or_else(|| "New Context".into())
    }
}

impl EventEmitter<ConversationEditorEvent> for ConversationEditor {}

impl Render for ConversationEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context("ConversationEditor")
            .capture_action(cx.listener(ConversationEditor::cancel_last_assist))
            .capture_action(cx.listener(ConversationEditor::save))
            .capture_action(cx.listener(ConversationEditor::copy))
            .capture_action(cx.listener(ConversationEditor::cycle_message_role))
            .capture_action(cx.listener(ConversationEditor::confirm_command))
            .on_action(cx.listener(ConversationEditor::assist))
            .on_action(cx.listener(ConversationEditor::split))
            .on_action(cx.listener(ConversationEditor::apply_edit))
            .size_full()
            .v_flex()
            .child(
                div()
                    .flex_grow()
                    .bg(cx.theme().colors().editor_background)
                    .child(self.editor.clone()),
            )
    }
}

impl FocusableView for ConversationEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

#[derive(Clone, Debug)]
struct MessageAnchor {
    id: MessageId,
    start: language::Anchor,
}

#[derive(Clone, Debug)]
pub struct Message {
    offset_range: Range<usize>,
    index_range: Range<usize>,
    id: MessageId,
    anchor: language::Anchor,
    role: Role,
    status: MessageStatus,
}

impl Message {
    fn to_request_message(&self, buffer: &Buffer) -> LanguageModelRequestMessage {
        LanguageModelRequestMessage {
            role: self.role,
            content: buffer.text_for_range(self.offset_range.clone()).collect(),
        }
    }
}

enum InlineAssistantEvent {
    Confirmed {
        prompt: String,
        include_conversation: bool,
    },
    Canceled,
    Dismissed,
}

struct InlineAssistant {
    id: usize,
    prompt_editor: View<Editor>,
    confirmed: bool,
    include_conversation: bool,
    measurements: Arc<Mutex<BlockMeasurements>>,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    codegen: Model<Codegen>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<InlineAssistantEvent> for InlineAssistant {}

impl Render for InlineAssistant {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let measurements = *self.measurements.lock();
        h_flex()
            .w_full()
            .py_2()
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .child(
                h_flex()
                    .w(measurements.gutter_width + measurements.gutter_margin)
                    .children(if let Some(error) = self.codegen.read(cx).error() {
                        let error_message = SharedString::from(error.to_string());
                        Some(
                            div()
                                .id("error")
                                .tooltip(move |cx| Tooltip::text(error_message.clone(), cx))
                                .child(Icon::new(IconName::XCircle).color(Color::Error)),
                        )
                    } else {
                        None
                    }),
            )
            .child(h_flex().flex_1().child(self.render_prompt_editor(cx)))
    }
}

impl FocusableView for InlineAssistant {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.prompt_editor.focus_handle(cx)
    }
}

impl InlineAssistant {
    #[allow(clippy::too_many_arguments)]
    fn new(
        id: usize,
        measurements: Arc<Mutex<BlockMeasurements>>,
        include_conversation: bool,
        prompt_history: VecDeque<String>,
        codegen: Model<Codegen>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let prompt_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            let placeholder = match codegen.read(cx).kind() {
                CodegenKind::Transform { .. } => "Enter transformation prompt…",
                CodegenKind::Generate { .. } => "Enter generation prompt…",
            };
            editor.set_placeholder_text(placeholder, cx);
            editor
        });
        cx.focus_view(&prompt_editor);

        let subscriptions = vec![
            cx.observe(&codegen, Self::handle_codegen_changed),
            cx.subscribe(&prompt_editor, Self::handle_prompt_editor_events),
        ];

        Self {
            id,
            prompt_editor,
            confirmed: false,
            include_conversation,
            measurements,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            codegen,
            _subscriptions: subscriptions,
        }
    }

    fn handle_prompt_editor_events(
        &mut self,
        _: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if let EditorEvent::Edited = event {
            self.pending_prompt = self.prompt_editor.read(cx).text(cx);
            cx.notify();
        }
    }

    fn handle_codegen_changed(&mut self, _: Model<Codegen>, cx: &mut ViewContext<Self>) {
        let is_read_only = !self.codegen.read(cx).idle();
        self.prompt_editor.update(cx, |editor, cx| {
            let was_read_only = editor.read_only(cx);
            if was_read_only != is_read_only {
                if is_read_only {
                    editor.set_read_only(true);
                } else {
                    self.confirmed = false;
                    editor.set_read_only(false);
                }
            }
        });
        cx.notify();
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(InlineAssistantEvent::Canceled);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if self.confirmed {
            cx.emit(InlineAssistantEvent::Dismissed);
        } else {
            let prompt = self.prompt_editor.read(cx).text(cx);
            self.prompt_editor
                .update(cx, |editor, _cx| editor.set_read_only(true));
            cx.emit(InlineAssistantEvent::Confirmed {
                prompt,
                include_conversation: self.include_conversation,
            });
            self.confirmed = true;
            cx.notify();
        }
    }

    fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix > 0 {
                self.prompt_history_ix = Some(ix - 1);
                let prompt = self.prompt_history[ix - 1].clone();
                self.set_prompt(&prompt, cx);
            }
        } else if !self.prompt_history.is_empty() {
            self.prompt_history_ix = Some(self.prompt_history.len() - 1);
            let prompt = self.prompt_history[self.prompt_history.len() - 1].clone();
            self.set_prompt(&prompt, cx);
        }
    }

    fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix < self.prompt_history.len() - 1 {
                self.prompt_history_ix = Some(ix + 1);
                let prompt = self.prompt_history[ix + 1].clone();
                self.set_prompt(&prompt, cx);
            } else {
                self.prompt_history_ix = None;
                let pending_prompt = self.pending_prompt.clone();
                self.set_prompt(&pending_prompt, cx);
            }
        }
    }

    fn set_prompt(&mut self, prompt: &str, cx: &mut ViewContext<Self>) {
        self.prompt_editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                let len = buffer.len(cx);
                buffer.edit([(0..len, prompt)], None, cx);
            });
        });
    }

    fn render_prompt_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.prompt_editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };
        EditorElement::new(
            &self.prompt_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

// This wouldn't need to exist if we could pass parameters when rendering child views.
#[derive(Copy, Clone, Default)]
struct BlockMeasurements {
    gutter_width: Pixels,
    gutter_margin: Pixels,
}

struct PendingInlineAssist {
    editor: WeakView<Editor>,
    inline_assistant: Option<(BlockId, View<InlineAssistant>)>,
    codegen: Model<Codegen>,
    _subscriptions: Vec<Subscription>,
    project: WeakModel<Project>,
}

type ToggleFold = Arc<dyn Fn(bool, &mut WindowContext) + Send + Sync>;

fn render_slash_command_output_toggle(
    row: MultiBufferRow,
    is_folded: bool,
    fold: ToggleFold,
    _cx: &mut WindowContext,
) -> AnyElement {
    IconButton::new(
        ("slash-command-output-fold-indicator", row.0),
        ui::IconName::ChevronDown,
    )
    .on_click(move |_e, cx| fold(!is_folded, cx))
    .icon_color(ui::Color::Muted)
    .icon_size(ui::IconSize::Small)
    .selected(is_folded)
    .selected_icon(ui::IconName::ChevronRight)
    .size(ui::ButtonSize::None)
    .into_any_element()
}

fn render_pending_slash_command_gutter_decoration(
    row: MultiBufferRow,
    status: PendingSlashCommandStatus,
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
        PendingSlashCommandStatus::Error(error) => {
            icon = icon
                .icon_color(Color::Error)
                .tooltip(move |cx| Tooltip::text(format!("error: {error}"), cx));
        }
    }

    icon.into_any_element()
}

fn render_pending_slash_command_trailer(
    _row: MultiBufferRow,
    _confirm_command: Arc<dyn Fn(&mut WindowContext)>,
    _cx: &mut WindowContext,
) -> AnyElement {
    Empty.into_any()
    // ButtonLike::new(("run_button", row.0))
    //     .style(ButtonStyle::Filled)
    //     .size(ButtonSize::Compact)
    //     .layer(ElevationIndex::ModalSurface)
    //     .children(
    //         KeyBinding::for_action(&Confirm, cx)
    //             .map(|binding| binding.icon_size(IconSize::XSmall).into_any_element()),
    //     )
    //     .child(Label::new("Run").size(LabelSize::XSmall))
    //     .on_click(move |_, cx| confirm_command(cx))
    //     .into_any_element()
}

fn merge_ranges(ranges: &mut Vec<Range<Anchor>>, buffer: &MultiBufferSnapshot) {
    ranges.sort_unstable_by(|a, b| {
        a.start
            .cmp(&b.start, buffer)
            .then_with(|| b.end.cmp(&a.end, buffer))
    });

    let mut ix = 0;
    while ix + 1 < ranges.len() {
        let b = ranges[ix + 1].clone();
        let a = &mut ranges[ix];
        if a.end.cmp(&b.start, buffer).is_gt() {
            if a.end.cmp(&b.end, buffer).is_lt() {
                a.end = b.end;
            }
            ranges.remove(ix + 1);
        } else {
            ix += 1;
        }
    }
}

fn make_lsp_adapter_delegate(
    project: &Model<Project>,
    cx: &mut AppContext,
) -> Result<Arc<dyn LspAdapterDelegate>> {
    project.update(cx, |project, cx| {
        // TODO: Find the right worktree.
        let worktree = project
            .worktrees()
            .next()
            .ok_or_else(|| anyhow!("no worktrees when constructing ProjectLspAdapterDelegate"))?;
        Ok(ProjectLspAdapterDelegate::new(project, &worktree, cx) as Arc<dyn LspAdapterDelegate>)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FakeCompletionProvider, MessageId};
    use fs::FakeFs;
    use gpui::{AppContext, TestAppContext};
    use rope::Rope;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{cell::RefCell, path::Path, rc::Rc};
    use unindent::Unindent;
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_inserting_and_removing_messages(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        cx.set_global(settings_store);
        init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

        let conversation =
            cx.new_model(|cx| Conversation::new(registry, Default::default(), None, cx));
        let buffer = conversation.read(cx).buffer.clone();

        let message_1 = conversation.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&conversation, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        let message_2 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_1.id, Role::Assistant, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..1),
                (message_2.id, Role::Assistant, 1..1)
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "1"), (1..1, "2")], None, cx)
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..3)
            ]
        );

        let message_3 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_3.id, Role::User, 4..4)
            ]
        );

        let message_4 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..5),
                (message_3.id, Role::User, 5..5),
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(4..4, "C"), (5..5, "D")], None, cx)
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..6),
                (message_3.id, Role::User, 6..7),
            ]
        );

        // Deleting across message boundaries merges the messages.
        buffer.update(cx, |buffer, cx| buffer.edit([(1..4, "")], None, cx));
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_3.id, Role::User, 3..4),
            ]
        );

        // Undoing the deletion should also undo the merge.
        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..6),
                (message_3.id, Role::User, 6..7),
            ]
        );

        // Redoing the deletion should also redo the merge.
        buffer.update(cx, |buffer, cx| buffer.redo(cx));
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_3.id, Role::User, 3..4),
            ]
        );

        // Ensure we can still insert after a merged message.
        let message_5 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_5.id, Role::System, 3..4),
                (message_3.id, Role::User, 4..5)
            ]
        );
    }

    #[gpui::test]
    fn test_message_splitting(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

        let conversation =
            cx.new_model(|cx| Conversation::new(registry, Default::default(), None, cx));
        let buffer = conversation.read(cx).buffer.clone();

        let message_1 = conversation.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&conversation, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "aaa\nbbb\nccc\nddd\n")], None, cx)
        });

        let (_, message_2) =
            conversation.update(cx, |conversation, cx| conversation.split_message(3..3, cx));
        let message_2 = message_2.unwrap();

        // We recycle newlines in the middle of a split message
        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..16),
            ]
        );

        let (_, message_3) =
            conversation.update(cx, |conversation, cx| conversation.split_message(3..3, cx));
        let message_3 = message_3.unwrap();

        // We don't recycle newlines at the end of a split message
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..17),
            ]
        );

        let (_, message_4) =
            conversation.update(cx, |conversation, cx| conversation.split_message(9..9, cx));
        let message_4 = message_4.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..17),
            ]
        );

        let (_, message_5) =
            conversation.update(cx, |conversation, cx| conversation.split_message(9..9, cx));
        let message_5 = message_5.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\nddd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..10),
                (message_5.id, Role::User, 10..18),
            ]
        );

        let (message_6, message_7) = conversation.update(cx, |conversation, cx| {
            conversation.split_message(14..16, cx)
        });
        let message_6 = message_6.unwrap();
        let message_7 = message_7.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\ndd\nd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..10),
                (message_5.id, Role::User, 10..14),
                (message_6.id, Role::User, 14..17),
                (message_7.id, Role::User, 17..19),
            ]
        );
    }

    #[gpui::test]
    fn test_messages_for_offsets(cx: &mut AppContext) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        cx.set_global(settings_store);
        init(cx);
        let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let conversation =
            cx.new_model(|cx| Conversation::new(registry, Default::default(), None, cx));
        let buffer = conversation.read(cx).buffer.clone();

        let message_1 = conversation.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&conversation, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "aaa")], None, cx));
        let message_2 = conversation
            .update(cx, |conversation, cx| {
                conversation.insert_message_after(message_1.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "bbb")], None, cx));

        let message_3 = conversation
            .update(cx, |conversation, cx| {
                conversation.insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        buffer.update(cx, |buffer, cx| buffer.edit([(8..8, "ccc")], None, cx));

        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..8),
                (message_3.id, Role::User, 8..11)
            ]
        );

        assert_eq!(
            message_ids_for_offsets(&conversation, &[0, 4, 9], cx),
            [message_1.id, message_2.id, message_3.id]
        );
        assert_eq!(
            message_ids_for_offsets(&conversation, &[0, 1, 11], cx),
            [message_1.id, message_3.id]
        );

        let message_4 = conversation
            .update(cx, |conversation, cx| {
                conversation.insert_message_after(message_3.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..8),
                (message_3.id, Role::User, 8..12),
                (message_4.id, Role::User, 12..12)
            ]
        );
        assert_eq!(
            message_ids_for_offsets(&conversation, &[0, 4, 8, 12], cx),
            [message_1.id, message_2.id, message_3.id, message_4.id]
        );

        fn message_ids_for_offsets(
            conversation: &Model<Conversation>,
            offsets: &[usize],
            cx: &AppContext,
        ) -> Vec<MessageId> {
            conversation
                .read(cx)
                .messages_for_offsets(offsets.iter().copied(), cx)
                .into_iter()
                .map(|message| message.id)
                .collect()
        }
    }

    #[gpui::test]
    async fn test_slash_commands(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        cx.update(Project::init_settings);
        cx.update(init);
        let fs = FakeFs::new(cx.background_executor.clone());

        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "lib.rs": "fn one() -> usize { 1 }",
                    "main.rs": "
                        use crate::one;
                        fn main() { one(); }
                    ".unindent(),
                }
            }),
        )
        .await;

        let prompt_library = Arc::new(PromptLibrary::default());
        let slash_command_registry = SlashCommandRegistry::new();

        slash_command_registry.register_command(file_command::FileSlashCommand, false);
        slash_command_registry.register_command(
            prompt_command::PromptSlashCommand::new(prompt_library.clone()),
            false,
        );

        let registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let conversation = cx
            .new_model(|cx| Conversation::new(registry.clone(), slash_command_registry, None, cx));

        let output_ranges = Rc::new(RefCell::new(HashSet::default()));
        conversation.update(cx, |_, cx| {
            cx.subscribe(&conversation, {
                let ranges = output_ranges.clone();
                move |_, _, event, _| match event {
                    ConversationEvent::PendingSlashCommandsUpdated { removed, updated } => {
                        for range in removed {
                            ranges.borrow_mut().remove(range);
                        }
                        for command in updated {
                            ranges.borrow_mut().insert(command.source_range.clone());
                        }
                    }
                    _ => {}
                }
            })
            .detach();
        });

        let buffer = conversation.read_with(cx, |conversation, _| conversation.buffer.clone());

        // Insert a slash command
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "/file src/lib.rs")], None, cx);
        });
        assert_text_and_output_ranges(
            &buffer,
            &output_ranges.borrow(),
            "
            «/file src/lib.rs»
            "
            .unindent()
            .trim_end(),
            cx,
        );

        // Edit the argument of the slash command.
        buffer.update(cx, |buffer, cx| {
            let edit_offset = buffer.text().find("lib.rs").unwrap();
            buffer.edit([(edit_offset..edit_offset + "lib".len(), "main")], None, cx);
        });
        assert_text_and_output_ranges(
            &buffer,
            &output_ranges.borrow(),
            "
            «/file src/main.rs»
            "
            .unindent()
            .trim_end(),
            cx,
        );

        // Edit the name of the slash command, using one that doesn't exist.
        buffer.update(cx, |buffer, cx| {
            let edit_offset = buffer.text().find("/file").unwrap();
            buffer.edit(
                [(edit_offset..edit_offset + "/file".len(), "/unknown")],
                None,
                cx,
            );
        });
        assert_text_and_output_ranges(
            &buffer,
            &output_ranges.borrow(),
            "
            /unknown src/main.rs
            "
            .unindent()
            .trim_end(),
            cx,
        );

        #[track_caller]
        fn assert_text_and_output_ranges(
            buffer: &Model<Buffer>,
            ranges: &HashSet<Range<language::Anchor>>,
            expected_marked_text: &str,
            cx: &mut TestAppContext,
        ) {
            let (expected_text, expected_ranges) = marked_text_ranges(expected_marked_text, false);
            let (actual_text, actual_ranges) = buffer.update(cx, |buffer, _| {
                let mut ranges = ranges
                    .iter()
                    .map(|range| range.to_offset(buffer))
                    .collect::<Vec<_>>();
                ranges.sort_by_key(|a| a.start);
                (buffer.text(), ranges)
            });

            assert_eq!(actual_text, expected_text);
            assert_eq!(actual_ranges, expected_ranges);
        }
    }

    #[test]
    fn test_parse_next_edit_suggestion() {
        let text = "
            some output:

            ```edit src/foo.rs
                let a = 1;
                let b = 2;
            ---
                let w = 1;
                let x = 2;
                let y = 3;
                let z = 4;
            ```

            some more output:

            ```edit src/foo.rs
                let c = 1;
            ---
            ```

            and the conclusion.
        "
        .unindent();

        let rope = Rope::from(text.as_str());
        let mut lines = rope.chunks().lines();
        let mut suggestions = vec![];
        while let Some(suggestion) = parse_next_edit_suggestion(&mut lines) {
            suggestions.push((
                suggestion.path.clone(),
                text[suggestion.old_text_range].to_string(),
                text[suggestion.new_text_range].to_string(),
            ));
        }

        assert_eq!(
            suggestions,
            vec![
                (
                    Path::new("src/foo.rs").into(),
                    [
                        "    let a = 1;", //
                        "    let b = 2;",
                        "",
                    ]
                    .join("\n"),
                    [
                        "    let w = 1;",
                        "    let x = 2;",
                        "    let y = 3;",
                        "    let z = 4;",
                        "",
                    ]
                    .join("\n"),
                ),
                (
                    Path::new("src/foo.rs").into(),
                    [
                        "    let c = 1;", //
                        "",
                    ]
                    .join("\n"),
                    String::new(),
                )
            ]
        );
    }

    #[gpui::test]
    async fn test_serialization(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.set_global(CompletionProvider::Fake(FakeCompletionProvider::default()));
        cx.update(init);
        let registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let conversation =
            cx.new_model(|cx| Conversation::new(registry.clone(), Default::default(), None, cx));
        let buffer = conversation.read_with(cx, |conversation, _| conversation.buffer.clone());
        let message_0 =
            conversation.read_with(cx, |conversation, _| conversation.message_anchors[0].id);
        let message_1 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_0, Role::Assistant, MessageStatus::Done, cx)
                .unwrap()
        });
        let message_2 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "a"), (1..1, "b\nc")], None, cx);
            buffer.finalize_last_transaction();
        });
        let _message_3 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_2.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(buffer.read_with(cx, |buffer, _| buffer.text()), "a\nb\nc\n");
        assert_eq!(
            cx.read(|cx| messages(&conversation, cx)),
            [
                (message_0, Role::User, 0..2),
                (message_1.id, Role::Assistant, 2..6),
                (message_2.id, Role::System, 6..6),
            ]
        );

        let deserialized_conversation = Conversation::deserialize(
            conversation.read_with(cx, |conversation, cx| conversation.serialize(cx)),
            Default::default(),
            registry.clone(),
            Default::default(),
            None,
            &mut cx.to_async(),
        )
        .await
        .unwrap();
        let deserialized_buffer =
            deserialized_conversation.read_with(cx, |conversation, _| conversation.buffer.clone());
        assert_eq!(
            deserialized_buffer.read_with(cx, |buffer, _| buffer.text()),
            "a\nb\nc\n"
        );
        assert_eq!(
            cx.read(|cx| messages(&deserialized_conversation, cx)),
            [
                (message_0, Role::User, 0..2),
                (message_1.id, Role::Assistant, 2..6),
                (message_2.id, Role::System, 6..6),
            ]
        );
    }

    fn messages(
        conversation: &Model<Conversation>,
        cx: &AppContext,
    ) -> Vec<(MessageId, Role, Range<usize>)> {
        conversation
            .read(cx)
            .messages(cx)
            .map(|message| (message.id, message.role, message.offset_range))
            .collect()
    }
}
