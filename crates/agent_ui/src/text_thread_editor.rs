use crate::{
    QuoteSelection,
    language_model_selector::{LanguageModelSelector, language_model_selector},
    ui::BurnModeTooltip,
};
use agent_settings::CompletionMode;
use anyhow::Result;
use assistant_slash_command::{SlashCommand, SlashCommandOutputSection, SlashCommandWorkingSet};
use assistant_slash_commands::{DefaultSlashCommand, FileSlashCommand, selections_creases};
use client::{proto, zed_urls};
use collections::{BTreeSet, HashMap, HashSet, hash_map};
use editor::{
    Anchor, Editor, EditorEvent, MenuEditPredictionsPolicy, MultiBuffer, MultiBufferSnapshot,
    RowExt, ToOffset as _, ToPoint,
    actions::{MoveToEndOfLine, Newline, ShowCompletions},
    display_map::{
        BlockPlacement, BlockProperties, BlockStyle, Crease, CreaseMetadata, CustomBlockId, FoldId,
        RenderBlock, ToDisplayPoint,
    },
};
use editor::{FoldPlaceholder, display_map::CreaseId};
use fs::Fs;
use futures::FutureExt;
use gpui::{
    Action, Animation, AnimationExt, AnyElement, AnyView, App, ClipboardEntry, ClipboardItem,
    Empty, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, Global, InteractiveElement,
    IntoElement, ParentElement, Pixels, Render, RenderImage, SharedString, Size,
    StatefulInteractiveElement, Styled, Subscription, Task, WeakEntity, actions, div, img, point,
    prelude::*, pulsating_between, size,
};
use language::{
    BufferSnapshot, LspAdapterDelegate, ToOffset,
    language_settings::{SoftWrap, all_language_settings},
};
use language_model::{
    ConfigurationError, LanguageModelExt, LanguageModelImage, LanguageModelRegistry, Role,
};
use multi_buffer::MultiBufferRow;
use picker::{Picker, popover_menu::PickerPopoverMenu};
use project::{Project, Worktree};
use project::{ProjectPath, lsp_store::LocalLspAdapterDelegate};
use rope::Point;
use serde::{Deserialize, Serialize};
use settings::{
    LanguageModelProviderSetting, LanguageModelSelection, Settings, SettingsStore,
    update_settings_file,
};
use std::{
    any::TypeId,
    cmp,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use text::SelectionGoal;
use ui::{
    ButtonLike, CommonAnimationExt, Disclosure, ElevationIndex, KeyBinding, PopoverMenuHandle,
    TintColor, Tooltip, prelude::*,
};
use util::{ResultExt, maybe};
use workspace::{
    CollaboratorId,
    searchable::{Direction, SearchableItemHandle},
};
use workspace::{
    Save, Toast, Workspace,
    item::{self, FollowableItem, Item, ItemHandle},
    notifications::NotificationId,
    pane,
    searchable::{SearchEvent, SearchableItem},
};
use zed_actions::agent::ToggleModelSelector;

use crate::{slash_command::SlashCommandCompletionProvider, slash_command_picker};
use assistant_context::{
    AssistantContext, CacheStatus, Content, ContextEvent, ContextId, InvokedSlashCommandId,
    InvokedSlashCommandStatus, Message, MessageId, MessageMetadata, MessageStatus,
    PendingSlashCommandStatus, ThoughtProcessOutputSection,
};

actions!(
    assistant,
    [
        /// Sends the current message to the assistant.
        Assist,
        /// Confirms and executes the entered slash command.
        ConfirmCommand,
        /// Copies code from the assistant's response to the clipboard.
        CopyCode,
        /// Cycles between user and assistant message roles.
        CycleMessageRole,
        /// Inserts the selected text into the active editor.
        InsertIntoEditor,
        /// Splits the conversation at the current cursor position.
        Split,
    ]
);

/// Inserts files that were dragged and dropped into the assistant conversation.
#[derive(PartialEq, Clone, Action)]
#[action(namespace = assistant, no_json, no_register)]
pub enum InsertDraggedFiles {
    ProjectPaths(Vec<ProjectPath>),
    ExternalFiles(Vec<PathBuf>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ScrollPosition {
    offset_before_cursor: gpui::Point<f32>,
    cursor: Anchor,
}

type MessageHeader = MessageMetadata;

#[derive(Clone)]
enum AssistError {
    PaymentRequired,
    Message(SharedString),
}

pub enum ThoughtProcessStatus {
    Pending,
    Completed,
}

pub trait AgentPanelDelegate {
    fn active_context_editor(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<TextThreadEditor>>;

    fn open_saved_context(
        &self,
        workspace: &mut Workspace,
        path: Arc<Path>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>>;

    fn open_remote_context(
        &self,
        workspace: &mut Workspace,
        context_id: ContextId,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<Entity<TextThreadEditor>>>;

    fn quote_selection(
        &self,
        workspace: &mut Workspace,
        selection_ranges: Vec<Range<Anchor>>,
        buffer: Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    );
}

impl dyn AgentPanelDelegate {
    /// Returns the global [`AssistantPanelDelegate`], if it exists.
    pub fn try_global(cx: &App) -> Option<Arc<Self>> {
        cx.try_global::<GlobalAssistantPanelDelegate>()
            .map(|global| global.0.clone())
    }

    /// Sets the global [`AssistantPanelDelegate`].
    pub fn set_global(delegate: Arc<Self>, cx: &mut App) {
        cx.set_global(GlobalAssistantPanelDelegate(delegate));
    }
}

struct GlobalAssistantPanelDelegate(Arc<dyn AgentPanelDelegate>);

impl Global for GlobalAssistantPanelDelegate {}

pub struct TextThreadEditor {
    context: Entity<AssistantContext>,
    fs: Arc<dyn Fs>,
    slash_commands: Arc<SlashCommandWorkingSet>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
    editor: Entity<Editor>,
    pending_thought_process: Option<(CreaseId, language::Anchor)>,
    blocks: HashMap<MessageId, (MessageHeader, CustomBlockId)>,
    image_blocks: HashSet<CustomBlockId>,
    scroll_position: Option<ScrollPosition>,
    remote_id: Option<workspace::ViewId>,
    pending_slash_command_creases: HashMap<Range<language::Anchor>, CreaseId>,
    invoked_slash_command_creases: HashMap<InvokedSlashCommandId, CreaseId>,
    _subscriptions: Vec<Subscription>,
    last_error: Option<AssistError>,
    pub(crate) slash_menu_handle:
        PopoverMenuHandle<Picker<slash_command_picker::SlashCommandDelegate>>,
    // dragged_file_worktrees is used to keep references to worktrees that were added
    // when the user drag/dropped an external file onto the context editor. Since
    // the worktree is not part of the project panel, it would be dropped as soon as
    // the file is opened. In order to keep the worktree alive for the duration of the
    // context editor, we keep a reference here.
    dragged_file_worktrees: Vec<Entity<Worktree>>,
    language_model_selector: Entity<LanguageModelSelector>,
    language_model_selector_menu_handle: PopoverMenuHandle<LanguageModelSelector>,
}

const MAX_TAB_TITLE_LEN: usize = 16;

impl TextThreadEditor {
    pub fn init(cx: &mut App) {
        workspace::FollowableViewRegistry::register::<TextThreadEditor>(cx);

        cx.observe_new(
            |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
                workspace
                    .register_action(TextThreadEditor::quote_selection)
                    .register_action(TextThreadEditor::insert_selection)
                    .register_action(TextThreadEditor::copy_code)
                    .register_action(TextThreadEditor::handle_insert_dragged_files);
            },
        )
        .detach();
    }

    pub fn for_context(
        context: Entity<AssistantContext>,
        fs: Arc<dyn Fs>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let completion_provider = SlashCommandCompletionProvider::new(
            context.read(cx).slash_commands().clone(),
            Some(cx.entity().downgrade()),
            Some(workspace.clone()),
        );

        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_buffer(context.read(cx).buffer().clone(), None, window, cx);
            editor.disable_scrollbars_and_minimap(window, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Some(Rc::new(completion_provider)));
            editor.set_menu_edit_predictions_policy(MenuEditPredictionsPolicy::Never);
            editor.set_collaboration_hub(Box::new(project.clone()));

            let show_edit_predictions = all_language_settings(None, cx)
                .edit_predictions
                .enabled_in_text_threads;

            editor.set_show_edit_predictions(Some(show_edit_predictions), window, cx);

            editor
        });

        let _subscriptions = vec![
            cx.observe(&context, |_, _, cx| cx.notify()),
            cx.subscribe_in(&context, window, Self::handle_context_event),
            cx.subscribe_in(&editor, window, Self::handle_editor_event),
            cx.subscribe_in(&editor, window, Self::handle_editor_search_event),
            cx.observe_global_in::<SettingsStore>(window, Self::settings_changed),
        ];

        let slash_command_sections = context.read(cx).slash_command_output_sections().to_vec();
        let thought_process_sections = context.read(cx).thought_process_output_sections().to_vec();
        let slash_commands = context.read(cx).slash_commands().clone();
        let mut this = Self {
            context,
            slash_commands,
            editor,
            lsp_adapter_delegate,
            blocks: Default::default(),
            image_blocks: Default::default(),
            scroll_position: None,
            remote_id: None,
            pending_thought_process: None,
            fs: fs.clone(),
            workspace,
            project,
            pending_slash_command_creases: HashMap::default(),
            invoked_slash_command_creases: HashMap::default(),
            _subscriptions,
            last_error: None,
            slash_menu_handle: Default::default(),
            dragged_file_worktrees: Vec::new(),
            language_model_selector: cx.new(|cx| {
                language_model_selector(
                    |cx| LanguageModelRegistry::read_global(cx).default_model(),
                    move |model, cx| {
                        update_settings_file(fs.clone(), cx, move |settings, _| {
                            let provider = model.provider_id().0.to_string();
                            let model = model.id().0.to_string();
                            settings.agent.get_or_insert_default().set_model(
                                LanguageModelSelection {
                                    provider: LanguageModelProviderSetting(provider),
                                    model,
                                },
                            )
                        });
                    },
                    window,
                    cx,
                )
            }),
            language_model_selector_menu_handle: PopoverMenuHandle::default(),
        };
        this.update_message_headers(cx);
        this.update_image_blocks(cx);
        this.insert_slash_command_output_sections(slash_command_sections, false, window, cx);
        this.insert_thought_process_output_sections(
            thought_process_sections
                .into_iter()
                .map(|section| (section, ThoughtProcessStatus::Completed)),
            window,
            cx,
        );
        this
    }

    fn settings_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let show_edit_predictions = all_language_settings(None, cx)
                .edit_predictions
                .enabled_in_text_threads;

            editor.set_show_edit_predictions(Some(show_edit_predictions), window, cx);
        });
    }

    pub fn context(&self) -> &Entity<AssistantContext> {
        &self.context
    }

    pub fn editor(&self) -> &Entity<Editor> {
        &self.editor
    }

    pub fn insert_default_prompt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let command_name = DefaultSlashCommand.name();
        self.editor.update(cx, |editor, cx| {
            editor.insert(&format!("/{command_name}\n\n"), window, cx)
        });
        let command = self.context.update(cx, |context, cx| {
            context.reparse(cx);
            context.parsed_slash_commands()[0].clone()
        });
        self.run_command(
            command.source_range,
            &command.name,
            &command.arguments,
            false,
            self.workspace.clone(),
            window,
            cx,
        );
    }

    fn assist(&mut self, _: &Assist, window: &mut Window, cx: &mut Context<Self>) {
        if self.sending_disabled(cx) {
            return;
        }
        telemetry::event!("Agent Message Sent", agent = "zed-text");
        self.send_to_model(window, cx);
    }

    fn send_to_model(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.last_error = None;
        if let Some(user_message) = self.context.update(cx, |context, cx| context.assist(cx)) {
            let new_selection = {
                let cursor = user_message
                    .start
                    .to_offset(self.context.read(cx).buffer().read(cx));
                cursor..cursor
            };
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(Default::default(), window, cx, |selections| {
                    selections.select_ranges([new_selection])
                });
            });
            // Avoid scrolling to the new cursor position so the assistant's output is stable.
            cx.defer_in(window, |this, _, _| this.scroll_position = None);
        }

        cx.notify();
    }

    fn cancel(
        &mut self,
        _: &editor::actions::Cancel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.last_error = None;

        if self
            .context
            .update(cx, |context, cx| context.cancel_last_assist(cx))
        {
            return;
        }

        cx.propagate();
    }

    fn cycle_message_role(
        &mut self,
        _: &CycleMessageRole,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

    fn cursors(&self, cx: &mut App) -> Vec<usize> {
        let selections = self
            .editor
            .update(cx, |editor, cx| editor.selections.all::<usize>(cx));
        selections
            .into_iter()
            .map(|selection| selection.head())
            .collect()
    }

    pub fn insert_command(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(command) = self.slash_commands.command(name, cx) {
            self.editor.update(cx, |editor, cx| {
                editor.transact(window, cx, |editor, window, cx| {
                    editor.change_selections(Default::default(), window, cx, |s| s.try_cancel());
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let newest_cursor = editor.selections.newest::<Point>(cx).head();
                    if newest_cursor.column > 0
                        || snapshot
                            .chars_at(newest_cursor)
                            .next()
                            .is_some_and(|ch| ch != '\n')
                    {
                        editor.move_to_end_of_line(
                            &MoveToEndOfLine {
                                stop_at_soft_wraps: false,
                            },
                            window,
                            cx,
                        );
                        editor.newline(&Newline, window, cx);
                    }

                    editor.insert(&format!("/{name}"), window, cx);
                    if command.accepts_arguments() {
                        editor.insert(" ", window, cx);
                        editor.show_completions(&ShowCompletions::default(), window, cx);
                    }
                });
            });
            if !command.requires_argument() {
                self.confirm_command(&ConfirmCommand, window, cx);
            }
        }
    }

    pub fn confirm_command(
        &mut self,
        _: &ConfirmCommand,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.editor.read(cx).has_visible_completions_menu() {
            return;
        }

        let selections = self.editor.read(cx).selections.disjoint_anchors_arc();
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
                    workspace.clone(),
                    window,
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
        arguments: &[String],
        ensure_trailing_newline: bool,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(command) = self.slash_commands.command(name, cx) {
            let context = self.context.read(cx);
            let sections = context
                .slash_command_output_sections()
                .iter()
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
                window,
                cx,
            );
            self.context.update(cx, |context, cx| {
                context.insert_command_output(
                    command_range,
                    name,
                    output,
                    ensure_trailing_newline,
                    cx,
                )
            });
        }
    }

    fn handle_context_event(
        &mut self,
        _: &Entity<AssistantContext>,
        event: &ContextEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_editor = cx.entity().downgrade();

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
            ContextEvent::SummaryGenerated => {}
            ContextEvent::PathChanged { .. } => {}
            ContextEvent::StartedThoughtProcess(range) => {
                let creases = self.insert_thought_process_output_sections(
                    [(
                        ThoughtProcessOutputSection {
                            range: range.clone(),
                        },
                        ThoughtProcessStatus::Pending,
                    )],
                    window,
                    cx,
                );
                self.pending_thought_process = Some((creases[0], range.start));
            }
            ContextEvent::EndedThoughtProcess(end) => {
                if let Some((crease_id, start)) = self.pending_thought_process.take() {
                    self.editor.update(cx, |editor, cx| {
                        let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                        let (excerpt_id, _, _) = multi_buffer_snapshot.as_singleton().unwrap();
                        let start_anchor = multi_buffer_snapshot
                            .anchor_in_excerpt(*excerpt_id, start)
                            .unwrap();

                        editor.display_map.update(cx, |display_map, cx| {
                            display_map.unfold_intersecting(
                                vec![start_anchor..start_anchor],
                                true,
                                cx,
                            );
                        });
                        editor.remove_creases(vec![crease_id], cx);
                    });
                    self.insert_thought_process_output_sections(
                        [(
                            ThoughtProcessOutputSection { range: start..*end },
                            ThoughtProcessStatus::Completed,
                        )],
                        window,
                        cx,
                    );
                }
            }
            ContextEvent::StreamedCompletion => {
                self.editor.update(cx, |editor, cx| {
                    if let Some(scroll_position) = self.scroll_position {
                        let snapshot = editor.snapshot(window, cx);
                        let cursor_point = scroll_position.cursor.to_display_point(&snapshot);
                        let scroll_top =
                            cursor_point.row().as_f32() - scroll_position.offset_before_cursor.y;
                        editor.set_scroll_position(
                            point(scroll_position.offset_before_cursor.x, scroll_top),
                            window,
                            cx,
                        );
                    }
                });
            }
            ContextEvent::ParsedSlashCommandsUpdated { removed, updated } => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let (&excerpt_id, _, _) = buffer.as_singleton().unwrap();

                    editor.remove_creases(
                        removed
                            .iter()
                            .filter_map(|range| self.pending_slash_command_creases.remove(range)),
                        cx,
                    );

                    let crease_ids = editor.insert_creases(
                        updated.iter().map(|command| {
                            let workspace = self.workspace.clone();
                            let confirm_command = Arc::new({
                                let context_editor = context_editor.clone();
                                let command = command.clone();
                                move |window: &mut Window, cx: &mut App| {
                                    context_editor
                                        .update(cx, |context_editor, cx| {
                                            context_editor.run_command(
                                                command.source_range.clone(),
                                                &command.name,
                                                &command.arguments,
                                                false,
                                                workspace.clone(),
                                                window,
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            });
                            let placeholder = FoldPlaceholder {
                                render: Arc::new(move |_, _, _| Empty.into_any()),
                                ..Default::default()
                            };
                            let render_toggle = {
                                let confirm_command = confirm_command.clone();
                                let command = command.clone();
                                move |row, _, _, _window: &mut Window, _cx: &mut App| {
                                    render_pending_slash_command_gutter_decoration(
                                        row,
                                        &command.status,
                                        confirm_command.clone(),
                                    )
                                }
                            };
                            let render_trailer = {
                                move |_row, _unfold, _window: &mut Window, _cx: &mut App| {
                                    Empty.into_any()
                                }
                            };

                            let start = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.start)
                                .unwrap();
                            let end = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.end)
                                .unwrap();
                            Crease::inline(start..end, placeholder, render_toggle, render_trailer)
                        }),
                        cx,
                    );

                    self.pending_slash_command_creases.extend(
                        updated
                            .iter()
                            .map(|command| command.source_range.clone())
                            .zip(crease_ids),
                    );
                })
            }
            ContextEvent::InvokedSlashCommandChanged { command_id } => {
                self.update_invoked_slash_command(*command_id, window, cx);
            }
            ContextEvent::SlashCommandOutputSectionAdded { section } => {
                self.insert_slash_command_output_sections([section.clone()], false, window, cx);
            }
            ContextEvent::Operation(_) => {}
            ContextEvent::ShowAssistError(error_message) => {
                self.last_error = Some(AssistError::Message(error_message.clone()));
            }
            ContextEvent::ShowPaymentRequiredError => {
                self.last_error = Some(AssistError::PaymentRequired);
            }
        }
    }

    fn update_invoked_slash_command(
        &mut self,
        command_id: InvokedSlashCommandId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(invoked_slash_command) =
            self.context.read(cx).invoked_slash_command(&command_id)
            && let InvokedSlashCommandStatus::Finished = invoked_slash_command.status
        {
            let run_commands_in_ranges = invoked_slash_command.run_commands_in_ranges.clone();
            for range in run_commands_in_ranges {
                let commands = self.context.update(cx, |context, cx| {
                    context.reparse(cx);
                    context
                        .pending_commands_for_range(range.clone(), cx)
                        .to_vec()
                });

                for command in commands {
                    self.run_command(
                        command.source_range,
                        &command.name,
                        &command.arguments,
                        false,
                        self.workspace.clone(),
                        window,
                        cx,
                    );
                }
            }
        }

        self.editor.update(cx, |editor, cx| {
            if let Some(invoked_slash_command) =
                self.context.read(cx).invoked_slash_command(&command_id)
            {
                if let InvokedSlashCommandStatus::Finished = invoked_slash_command.status {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let (&excerpt_id, _buffer_id, _buffer_snapshot) =
                        buffer.as_singleton().unwrap();

                    let start = buffer
                        .anchor_in_excerpt(excerpt_id, invoked_slash_command.range.start)
                        .unwrap();
                    let end = buffer
                        .anchor_in_excerpt(excerpt_id, invoked_slash_command.range.end)
                        .unwrap();
                    editor.remove_folds_with_type(
                        &[start..end],
                        TypeId::of::<PendingSlashCommand>(),
                        false,
                        cx,
                    );

                    editor.remove_creases(
                        HashSet::from_iter(self.invoked_slash_command_creases.remove(&command_id)),
                        cx,
                    );
                } else if let hash_map::Entry::Vacant(entry) =
                    self.invoked_slash_command_creases.entry(command_id)
                {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let (&excerpt_id, _buffer_id, _buffer_snapshot) =
                        buffer.as_singleton().unwrap();
                    let context = self.context.downgrade();
                    let crease_start = buffer
                        .anchor_in_excerpt(excerpt_id, invoked_slash_command.range.start)
                        .unwrap();
                    let crease_end = buffer
                        .anchor_in_excerpt(excerpt_id, invoked_slash_command.range.end)
                        .unwrap();
                    let crease = Crease::inline(
                        crease_start..crease_end,
                        invoked_slash_command_fold_placeholder(command_id, context),
                        fold_toggle("invoked-slash-command"),
                        |_row, _folded, _window, _cx| Empty.into_any(),
                    );
                    let crease_ids = editor.insert_creases([crease.clone()], cx);
                    editor.fold_creases(vec![crease], false, window, cx);
                    entry.insert(crease_ids[0]);
                } else {
                    cx.notify()
                }
            } else {
                editor.remove_creases(
                    HashSet::from_iter(self.invoked_slash_command_creases.remove(&command_id)),
                    cx,
                );
                cx.notify();
            };
        });
    }

    fn insert_thought_process_output_sections(
        &mut self,
        sections: impl IntoIterator<
            Item = (
                ThoughtProcessOutputSection<language::Anchor>,
                ThoughtProcessStatus,
            ),
        >,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<CreaseId> {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let mut buffer_rows_to_fold = BTreeSet::new();
            let mut creases = Vec::new();
            for (section, status) in sections {
                let start = buffer
                    .anchor_in_excerpt(excerpt_id, section.range.start)
                    .unwrap();
                let end = buffer
                    .anchor_in_excerpt(excerpt_id, section.range.end)
                    .unwrap();
                let buffer_row = MultiBufferRow(start.to_point(&buffer).row);
                buffer_rows_to_fold.insert(buffer_row);
                creases.push(
                    Crease::inline(
                        start..end,
                        FoldPlaceholder {
                            render: render_thought_process_fold_icon_button(
                                cx.entity().downgrade(),
                                status,
                            ),
                            merge_adjacent: false,
                            ..Default::default()
                        },
                        render_slash_command_output_toggle,
                        |_, _, _, _| Empty.into_any_element(),
                    )
                    .with_metadata(CreaseMetadata {
                        icon_path: SharedString::from(IconName::Ai.path()),
                        label: "Thinking Process".into(),
                    }),
                );
            }

            let creases = editor.insert_creases(creases, cx);

            for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                editor.fold_at(buffer_row, window, cx);
            }

            creases
        })
    }

    fn insert_slash_command_output_sections(
        &mut self,
        sections: impl IntoIterator<Item = SlashCommandOutputSection<language::Anchor>>,
        expand_result: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
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
                    Crease::inline(
                        start..end,
                        FoldPlaceholder {
                            render: render_fold_icon_button(
                                cx.entity().downgrade(),
                                section.icon.path().into(),
                                section.label.clone(),
                            ),
                            merge_adjacent: false,
                            ..Default::default()
                        },
                        render_slash_command_output_toggle,
                        |_, _, _, _| Empty.into_any_element(),
                    )
                    .with_metadata(CreaseMetadata {
                        icon_path: section.icon.path().into(),
                        label: section.label,
                    }),
                );
            }

            editor.insert_creases(creases, cx);

            if expand_result {
                buffer_rows_to_fold.clear();
            }
            for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                editor.fold_at(buffer_row, window, cx);
            }
        });
    }

    fn handle_editor_event(
        &mut self,
        _: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::ScrollPositionChanged { autoscroll, .. } => {
                let cursor_scroll_position = self.cursor_scroll_position(window, cx);
                if *autoscroll {
                    self.scroll_position = cursor_scroll_position;
                } else if self.scroll_position != cursor_scroll_position {
                    self.scroll_position = None;
                }
            }
            EditorEvent::SelectionsChanged { .. } => {
                self.scroll_position = self.cursor_scroll_position(window, cx);
            }
            _ => {}
        }
        cx.emit(event.clone());
    }

    fn handle_editor_search_event(
        &mut self,
        _: &Entity<Editor>,
        event: &SearchEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.emit(event.clone());
    }

    fn cursor_scroll_position(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<ScrollPosition> {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(window, cx);
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

    fn esc_kbd(cx: &App) -> Div {
        let colors = cx.theme().colors().clone();

        h_flex()
            .items_center()
            .gap_1()
            .font(theme::ThemeSettings::get_global(cx).buffer_font.clone())
            .text_size(TextSize::XSmall.rems(cx))
            .text_color(colors.text_muted)
            .child("Press")
            .child(
                h_flex()
                    .rounded_sm()
                    .px_1()
                    .mr_0p5()
                    .border_1()
                    .border_color(colors.border_variant.alpha(0.6))
                    .bg(colors.element_background.alpha(0.6))
                    .child("esc"),
            )
            .child("to cancel")
    }

    fn update_message_headers(&mut self, cx: &mut Context<Self>) {
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
                Arc::new({
                    let context = self.context.clone();

                    move |cx| {
                        let message_id = MessageId(message.timestamp);
                        let llm_loading = message.role == Role::Assistant
                            && message.status == MessageStatus::Pending;

                        let (label, spinner, note) = match message.role {
                            Role::User => (
                                Label::new("You").color(Color::Default).into_any_element(),
                                None,
                                None,
                            ),
                            Role::Assistant => {
                                let base_label = Label::new("Agent").color(Color::Info);
                                let mut spinner = None;
                                let mut note = None;
                                let animated_label = if llm_loading {
                                    base_label
                                        .with_animation(
                                            "pulsating-label",
                                            Animation::new(Duration::from_secs(2))
                                                .repeat()
                                                .with_easing(pulsating_between(0.4, 0.8)),
                                            |label, delta| label.alpha(delta),
                                        )
                                        .into_any_element()
                                } else {
                                    base_label.into_any_element()
                                };
                                if llm_loading {
                                    spinner = Some(
                                        Icon::new(IconName::ArrowCircle)
                                            .size(IconSize::XSmall)
                                            .color(Color::Info)
                                            .with_rotate_animation(2)
                                            .into_any_element(),
                                    );
                                    note = Some(Self::esc_kbd(cx).into_any_element());
                                }
                                (animated_label, spinner, note)
                            }
                            Role::System => (
                                Label::new("System")
                                    .color(Color::Warning)
                                    .into_any_element(),
                                None,
                                None,
                            ),
                        };

                        let sender = h_flex()
                            .items_center()
                            .gap_2p5()
                            .child(
                                ButtonLike::new("role")
                                    .style(ButtonStyle::Filled)
                                    .child(
                                        h_flex()
                                            .items_center()
                                            .gap_1p5()
                                            .child(label)
                                            .children(spinner),
                                    )
                                    .tooltip(|window, cx| {
                                        Tooltip::with_meta(
                                            "Toggle message role",
                                            None,
                                            "Available roles: You (User), Agent, System",
                                            window,
                                            cx,
                                        )
                                    })
                                    .on_click({
                                        let context = context.clone();
                                        move |_, _window, cx| {
                                            context.update(cx, |context, cx| {
                                                context.cycle_message_roles(
                                                    HashSet::from_iter(Some(message_id)),
                                                    cx,
                                                )
                                            })
                                        }
                                    }),
                            )
                            .children(note);

                        h_flex()
                            .id(("message_header", message_id.as_u64()))
                            .pl(cx.margins.gutter.full_width())
                            .h_11()
                            .w_full()
                            .relative()
                            .gap_1p5()
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
                                            .tooltip(|window, cx| {
                                                Tooltip::with_meta(
                                                    "Context Cached",
                                                    None,
                                                    "Large messages cached to optimize performance",
                                                    window,
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
                                        .icon_size(IconSize::XSmall)
                                        .icon_position(IconPosition::Start)
                                        .tooltip(Tooltip::text("View Details"))
                                        .on_click({
                                            let context = context.clone();
                                            let error = error.clone();
                                            move |_, _window, cx| {
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
                                    h_flex()
                                        .gap_1()
                                        .items_center()
                                        .child(
                                            Icon::new(IconName::XCircle)
                                                .color(Color::Disabled)
                                                .size(IconSize::XSmall),
                                        )
                                        .child(
                                            Label::new("Canceled")
                                                .size(LabelSize::Small)
                                                .color(Color::Disabled),
                                        )
                                        .into_any_element(),
                                ),
                                _ => None,
                            })
                            .into_any_element()
                    }
                })
            };
            let create_block_properties = |message: &Message| BlockProperties {
                height: Some(2),
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
                if blocks_to_remove.remove(&message.id).is_some() {
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
        context_editor_view: &Entity<TextThreadEditor>,
        cx: &mut Context<Workspace>,
    ) -> Option<(String, bool)> {
        const CODE_FENCE_DELIMITER: &str = "```";

        let context_editor = context_editor_view.read(cx).editor.clone();
        context_editor.update(cx, |context_editor, cx| {
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
                let selection = context_editor.selections.newest_adjusted(cx);
                let buffer = context_editor.buffer().read(cx).snapshot(cx);
                let selected_text = buffer.text_for_range(selection.range()).collect::<String>();

                (!selected_text.is_empty()).then_some((selected_text, false))
            }
        })
    }

    pub fn insert_selection(
        workspace: &mut Workspace,
        _: &InsertIntoEditor,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(agent_panel_delegate) = <dyn AgentPanelDelegate>::try_global(cx) else {
            return;
        };
        let Some(context_editor_view) =
            agent_panel_delegate.active_context_editor(workspace, window, cx)
        else {
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
                editor.insert(&text, window, cx);
                editor.focus_handle(cx).focus(window);
            })
        }
    }

    pub fn copy_code(
        workspace: &mut Workspace,
        _: &CopyCode,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let result = maybe!({
            let agent_panel_delegate = <dyn AgentPanelDelegate>::try_global(cx)?;
            let context_editor_view =
                agent_panel_delegate.active_context_editor(workspace, window, cx)?;
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

    pub fn handle_insert_dragged_files(
        workspace: &mut Workspace,
        action: &InsertDraggedFiles,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(agent_panel_delegate) = <dyn AgentPanelDelegate>::try_global(cx) else {
            return;
        };
        let Some(context_editor_view) =
            agent_panel_delegate.active_context_editor(workspace, window, cx)
        else {
            return;
        };

        let project = context_editor_view.read(cx).project.clone();

        let paths = match action {
            InsertDraggedFiles::ProjectPaths(paths) => Task::ready((paths.clone(), vec![])),
            InsertDraggedFiles::ExternalFiles(paths) => {
                let tasks = paths
                    .clone()
                    .into_iter()
                    .map(|path| Workspace::project_path_for_path(project.clone(), &path, false, cx))
                    .collect::<Vec<_>>();

                cx.background_spawn(async move {
                    let mut paths = vec![];
                    let mut worktrees = vec![];

                    let opened_paths = futures::future::join_all(tasks).await;

                    for entry in opened_paths {
                        if let Some((worktree, project_path)) = entry.log_err() {
                            worktrees.push(worktree);
                            paths.push(project_path);
                        }
                    }

                    (paths, worktrees)
                })
            }
        };

        context_editor_view.update(cx, |_, cx| {
            cx.spawn_in(window, async move |this, cx| {
                let (paths, dragged_file_worktrees) = paths.await;
                this.update_in(cx, |this, window, cx| {
                    this.insert_dragged_files(paths, dragged_file_worktrees, window, cx);
                })
                .ok();
            })
            .detach();
        })
    }

    pub fn insert_dragged_files(
        &mut self,
        opened_paths: Vec<ProjectPath>,
        added_worktrees: Vec<Entity<Worktree>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut file_slash_command_args = vec![];
        for project_path in opened_paths.into_iter() {
            let Some(worktree) = self
                .project
                .read(cx)
                .worktree_for_id(project_path.worktree_id, cx)
            else {
                continue;
            };
            let worktree_root_name = worktree.read(cx).root_name().to_string();
            let mut full_path = PathBuf::from(worktree_root_name.clone());
            full_path.push(&project_path.path);
            file_slash_command_args.push(full_path.to_string_lossy().to_string());
        }

        let cmd_name = FileSlashCommand.name();

        let file_argument = file_slash_command_args.join(" ");

        self.editor.update(cx, |editor, cx| {
            editor.insert("\n", window, cx);
            editor.insert(&format!("/{} {}", cmd_name, file_argument), window, cx);
        });
        self.confirm_command(&ConfirmCommand, window, cx);
        self.dragged_file_worktrees.extend(added_worktrees);
    }

    pub fn quote_selection(
        workspace: &mut Workspace,
        _: &QuoteSelection,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(agent_panel_delegate) = <dyn AgentPanelDelegate>::try_global(cx) else {
            return;
        };

        let Some((selections, buffer)) = maybe!({
            let editor = workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))?;

            let buffer = editor.read(cx).buffer().clone();
            let snapshot = buffer.read(cx).snapshot(cx);
            let selections = editor.update(cx, |editor, cx| {
                editor
                    .selections
                    .all_adjusted(cx)
                    .into_iter()
                    .filter_map(|s| {
                        (!s.is_empty())
                            .then(|| snapshot.anchor_after(s.start)..snapshot.anchor_before(s.end))
                    })
                    .collect::<Vec<_>>()
            });
            Some((selections, buffer))
        }) else {
            return;
        };

        if selections.is_empty() {
            return;
        }

        agent_panel_delegate.quote_selection(workspace, selections, buffer, window, cx);
    }

    pub fn quote_ranges(
        &mut self,
        ranges: Vec<Range<Point>>,
        snapshot: MultiBufferSnapshot,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let creases = selections_creases(ranges, snapshot, cx);

        self.editor.update(cx, |editor, cx| {
            editor.insert("\n", window, cx);
            for (text, crease_title) in creases {
                let point = editor.selections.newest::<Point>(cx).head();
                let start_row = MultiBufferRow(point.row);

                editor.insert(&text, window, cx);

                let snapshot = editor.buffer().read(cx).snapshot(cx);
                let anchor_before = snapshot.anchor_after(point);
                let anchor_after = editor
                    .selections
                    .newest_anchor()
                    .head()
                    .bias_left(&snapshot);

                editor.insert("\n", window, cx);

                let fold_placeholder =
                    quote_selection_fold_placeholder(crease_title, cx.entity().downgrade());
                let crease = Crease::inline(
                    anchor_before..anchor_after,
                    fold_placeholder,
                    render_quote_selection_output_toggle,
                    |_, _, _, _| Empty.into_any(),
                );
                editor.insert_creases(vec![crease], cx);
                editor.fold_at(start_row, window, cx);
            }
        })
    }

    fn copy(&mut self, _: &editor::actions::Copy, _window: &mut Window, cx: &mut Context<Self>) {
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

    fn cut(&mut self, _: &editor::actions::Cut, window: &mut Window, cx: &mut Context<Self>) {
        if self.editor.read(cx).selections.count() == 1 {
            let (copied_text, metadata, selections) = self.get_clipboard_contents(cx);

            self.editor.update(cx, |editor, cx| {
                editor.transact(window, cx, |this, window, cx| {
                    this.change_selections(Default::default(), window, cx, |s| {
                        s.select(selections);
                    });
                    this.insert("", window, cx);
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
        cx: &mut Context<Self>,
    ) -> (String, CopyMetadata, Vec<text::Selection<usize>>) {
        let (mut selection, creases) = self.editor.update(cx, |editor, cx| {
            let mut selection = editor.selections.newest_adjusted(cx);
            let snapshot = editor.buffer().read(cx).snapshot(cx);

            selection.goal = SelectionGoal::None;

            let selection_start = snapshot.point_to_offset(selection.start);

            (
                selection.map(|point| snapshot.point_to_offset(point)),
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
                            if let Crease::Inline {
                                range, metadata, ..
                            } = &crease
                            {
                                let metadata = metadata.as_ref()?;
                                let start = range
                                    .start
                                    .to_offset(&snapshot)
                                    .saturating_sub(selection_start);
                                let end = range
                                    .end
                                    .to_offset(&snapshot)
                                    .saturating_sub(selection_start);

                                let range_relative_to_selection = start..end;
                                if !range_relative_to_selection.is_empty() {
                                    return Some(SelectedCreaseMetadata {
                                        range_relative_to_selection,
                                        crease: metadata.clone(),
                                    });
                                }
                            }
                            None
                        })
                        .collect::<Vec<_>>()
                }),
            )
        });

        let context = self.context.read(cx);

        let mut text = String::new();

        // If selection is empty, we want to copy the entire line
        if selection.range().is_empty() {
            let snapshot = context.buffer().read(cx).snapshot();
            let point = snapshot.offset_to_point(selection.range().start);
            selection.start = snapshot.point_to_offset(Point::new(point.row, 0));
            selection.end = snapshot
                .point_to_offset(cmp::min(Point::new(point.row + 1, 0), snapshot.max_point()));
            for chunk in context.buffer().read(cx).text_for_range(selection.range()) {
                text.push_str(chunk);
            }
        } else {
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
        }
        (text, CopyMetadata { creases }, vec![selection])
    }

    fn paste(
        &mut self,
        action: &editor::actions::Paste,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
                editor.paste(action, window, cx);

                if let Some(metadata) = metadata {
                    let buffer = editor.buffer().read(cx).snapshot(cx);

                    let mut buffer_rows_to_fold = BTreeSet::new();
                    let weak_editor = cx.entity().downgrade();
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
                            Crease::inline(
                                start..end,
                                FoldPlaceholder {
                                    render: render_fold_icon_button(
                                        weak_editor.clone(),
                                        metadata.crease.icon_path.clone(),
                                        metadata.crease.label.clone(),
                                    ),
                                    ..Default::default()
                                },
                                render_slash_command_output_toggle,
                                |_, _, _, _| Empty.into_any(),
                            )
                            .with_metadata(metadata.crease)
                        }),
                        cx,
                    );
                    for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                        editor.fold_at(buffer_row, window, cx);
                    }
                }
            });
        } else {
            let mut image_positions = Vec::new();
            self.editor.update(cx, |editor, cx| {
                editor.transact(window, cx, |editor, _window, cx| {
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
                    let Some(render_image) = image.to_image_data(cx.svg_renderer()).log_err()
                    else {
                        continue;
                    };
                    let image_id = image.id();
                    let image_task = LanguageModelImage::from_image(Arc::new(image), cx).shared();

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

    fn update_image_blocks(&mut self, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let old_blocks = std::mem::take(&mut self.image_blocks);
            let new_blocks = self
                .context
                .read(cx)
                .contents(cx)
                .map(
                    |Content::Image {
                         anchor,
                         render_image,
                         ..
                     }| (anchor, render_image),
                )
                .filter_map(|(anchor, render_image)| {
                    const MAX_HEIGHT_IN_LINES: u32 = 8;
                    let anchor = buffer.anchor_in_excerpt(excerpt_id, anchor).unwrap();
                    let image = render_image;
                    anchor.is_valid(&buffer).then(|| BlockProperties {
                        placement: BlockPlacement::Above(anchor),
                        height: Some(MAX_HEIGHT_IN_LINES),
                        style: BlockStyle::Sticky,
                        render: Arc::new(move |cx| {
                            let image_size = size_for_image(
                                &image,
                                size(
                                    cx.max_width - cx.margins.gutter.full_width(),
                                    MAX_HEIGHT_IN_LINES as f32 * cx.line_height,
                                ),
                            );
                            h_flex()
                                .pl(cx.margins.gutter.full_width())
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

    fn split(&mut self, _: &Split, _window: &mut Window, cx: &mut Context<Self>) {
        self.context.update(cx, |context, cx| {
            let selections = self.editor.read(cx).selections.disjoint_anchors_arc();
            for selection in selections.as_ref() {
                let buffer = self.editor.read(cx).buffer().read(cx).snapshot(cx);
                let range = selection
                    .map(|endpoint| endpoint.to_offset(&buffer))
                    .range();
                context.split_message(range, cx);
            }
        });
    }

    fn save(&mut self, _: &Save, _window: &mut Window, cx: &mut Context<Self>) {
        self.context.update(cx, |context, cx| {
            context.save(Some(Duration::from_millis(500)), self.fs.clone(), cx)
        });
    }

    pub fn title(&self, cx: &App) -> SharedString {
        self.context.read(cx).summary().or_default()
    }

    pub fn regenerate_summary(&mut self, cx: &mut Context<Self>) {
        self.context
            .update(cx, |context, cx| context.summarize(true, cx));
    }

    fn render_remaining_tokens(&self, cx: &App) -> Option<impl IntoElement + use<>> {
        let (token_count_color, token_count, max_token_count, tooltip) =
            match token_state(&self.context, cx)? {
                TokenState::NoTokensLeft {
                    max_token_count,
                    token_count,
                } => (
                    Color::Error,
                    token_count,
                    max_token_count,
                    Some("Token Limit Reached"),
                ),
                TokenState::HasMoreTokens {
                    max_token_count,
                    token_count,
                    over_warn_threshold,
                } => {
                    let (color, tooltip) = if over_warn_threshold {
                        (Color::Warning, Some("Token Limit is Close to Exhaustion"))
                    } else {
                        (Color::Muted, None)
                    };
                    (color, token_count, max_token_count, tooltip)
                }
            };

        Some(
            h_flex()
                .id("token-count")
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
                )
                .when_some(tooltip, |element, tooltip| {
                    element.tooltip(Tooltip::text(tooltip))
                }),
        )
    }

    fn render_send_button(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        let (style, tooltip) = match token_state(&self.context, cx) {
            Some(TokenState::NoTokensLeft { .. }) => (
                ButtonStyle::Tinted(TintColor::Error),
                Some(Tooltip::text("Token limit reached")(window, cx)),
            ),
            Some(TokenState::HasMoreTokens {
                over_warn_threshold,
                ..
            }) => {
                let (style, tooltip) = if over_warn_threshold {
                    (
                        ButtonStyle::Tinted(TintColor::Warning),
                        Some(Tooltip::text("Token limit is close to exhaustion")(
                            window, cx,
                        )),
                    )
                } else {
                    (ButtonStyle::Filled, None)
                };
                (style, tooltip)
            }
            None => (ButtonStyle::Filled, None),
        };

        Button::new("send_button", "Send")
            .label_size(LabelSize::Small)
            .disabled(self.sending_disabled(cx))
            .style(style)
            .when_some(tooltip, |button, tooltip| {
                button.tooltip(move |_, _| tooltip.clone())
            })
            .layer(ElevationIndex::ModalSurface)
            .key_binding(
                KeyBinding::for_action_in(&Assist, &focus_handle, window, cx)
                    .map(|kb| kb.size(rems_from_px(12.))),
            )
            .on_click(move |_event, window, cx| {
                focus_handle.dispatch_action(&Assist, window, cx);
            })
    }

    /// Whether or not we should allow messages to be sent.
    /// Will return false if the selected provided has a configuration error or
    /// if the user has not accepted the terms of service for this provider.
    fn sending_disabled(&self, cx: &mut Context<'_, TextThreadEditor>) -> bool {
        let model_registry = LanguageModelRegistry::read_global(cx);
        let Some(configuration_error) =
            model_registry.configuration_error(model_registry.default_model(), cx)
        else {
            return false;
        };

        match configuration_error {
            ConfigurationError::NoProvider
            | ConfigurationError::ModelNotFound
            | ConfigurationError::ProviderNotAuthenticated(_) => true,
        }
    }

    fn render_inject_context_menu(&self, cx: &mut Context<Self>) -> impl IntoElement {
        slash_command_picker::SlashCommandSelector::new(
            self.slash_commands.clone(),
            cx.entity().downgrade(),
            IconButton::new("trigger", IconName::Plus)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted),
            move |window, cx| {
                Tooltip::with_meta(
                    "Add Context",
                    None,
                    "Type / to insert via keyboard",
                    window,
                    cx,
                )
            },
        )
    }

    fn render_burn_mode_toggle(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let context = self.context().read(cx);
        let active_model = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.model)?;
        if !active_model.supports_burn_mode() {
            return None;
        }

        let active_completion_mode = context.completion_mode();
        let burn_mode_enabled = active_completion_mode == CompletionMode::Burn;
        let icon = if burn_mode_enabled {
            IconName::ZedBurnModeOn
        } else {
            IconName::ZedBurnMode
        };

        Some(
            IconButton::new("burn-mode", icon)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .toggle_state(burn_mode_enabled)
                .selected_icon_color(Color::Error)
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    this.context().update(cx, |context, _cx| {
                        context.set_completion_mode(match active_completion_mode {
                            CompletionMode::Burn => CompletionMode::Normal,
                            CompletionMode::Normal => CompletionMode::Burn,
                        });
                    });
                }))
                .tooltip(move |_window, cx| {
                    cx.new(|_| BurnModeTooltip::new().selected(burn_mode_enabled))
                        .into()
                })
                .into_any_element(),
        )
    }

    fn render_language_model_selector(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active_model = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.model);
        let model_name = match active_model {
            Some(model) => model.name().0,
            None => SharedString::from("Select Model"),
        };

        let active_provider = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.provider);

        let provider_icon = match active_provider {
            Some(provider) => provider.icon(),
            None => IconName::Ai,
        };

        let focus_handle = self.editor().focus_handle(cx);

        PickerPopoverMenu::new(
            self.language_model_selector.clone(),
            ButtonLike::new("active-model")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .gap_0p5()
                        .child(
                            Icon::new(provider_icon)
                                .color(Color::Muted)
                                .size(IconSize::XSmall),
                        )
                        .child(
                            Label::new(model_name)
                                .color(Color::Muted)
                                .size(LabelSize::Small)
                                .ml_0p5(),
                        )
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .color(Color::Muted)
                                .size(IconSize::XSmall),
                        ),
                ),
            move |window, cx| {
                Tooltip::for_action_in(
                    "Change Model",
                    &ToggleModelSelector,
                    &focus_handle,
                    window,
                    cx,
                )
            },
            gpui::Corner::BottomLeft,
            cx,
        )
        .with_handle(self.language_model_selector_menu_handle.clone())
        .render(window, cx)
    }

    fn render_last_error(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
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
                    AssistError::Message(error_message) => {
                        self.render_assist_error(error_message, cx)
                    }
                })
                .into_any(),
        )
    }

    fn render_payment_required_error(&self, cx: &mut Context<Self>) -> AnyElement {
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
                        |this, _, _window, cx| {
                            this.last_error = None;
                            cx.open_url(&zed_urls::account_url(cx));
                            cx.notify();
                        },
                    )))
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, _window, cx| {
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
        cx: &mut Context<Self>,
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
                    .max_h_32()
                    .overflow_y_scroll()
                    .child(Label::new(error_message.clone())),
            )
            .child(
                h_flex()
                    .justify_end()
                    .mt_1()
                    .child(Button::new("dismiss", "Dismiss").on_click(cx.listener(
                        |this, _, _window, cx| {
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
    const CODE_BLOCK_NODE: &str = "fenced_code_block";
    const CODE_BLOCK_CONTENT: &str = "code_fence_content";

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

fn render_thought_process_fold_icon_button(
    editor: WeakEntity<Editor>,
    status: ThoughtProcessStatus,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    Arc::new(move |fold_id, fold_range, _cx| {
        let editor = editor.clone();

        let button = ButtonLike::new(fold_id).layer(ElevationIndex::ElevatedSurface);
        let button = match status {
            ThoughtProcessStatus::Pending => button
                .child(
                    Icon::new(IconName::ToolThink)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Label::new("Thinking…").color(Color::Muted).with_animation(
                        "pulsating-label",
                        Animation::new(Duration::from_secs(2))
                            .repeat()
                            .with_easing(pulsating_between(0.4, 0.8)),
                        |label, delta| label.alpha(delta),
                    ),
                ),
            ThoughtProcessStatus::Completed => button
                .style(ButtonStyle::Filled)
                .child(Icon::new(IconName::ToolThink).size(IconSize::Small))
                .child(Label::new("Thought Process").single_line()),
        };

        button
            .on_click(move |_, window, cx| {
                editor
                    .update(cx, |editor, cx| {
                        let buffer_start = fold_range
                            .start
                            .to_point(&editor.buffer().read(cx).read(cx));
                        let buffer_row = MultiBufferRow(buffer_start.row);
                        editor.unfold_at(buffer_row, window, cx);
                    })
                    .ok();
            })
            .into_any_element()
    })
}

fn render_fold_icon_button(
    editor: WeakEntity<Editor>,
    icon_path: SharedString,
    label: SharedString,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    Arc::new(move |fold_id, fold_range, _cx| {
        let editor = editor.clone();
        ButtonLike::new(fold_id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::from_path(icon_path.clone()))
            .child(Label::new(label.clone()).single_line())
            .on_click(move |_, window, cx| {
                editor
                    .update(cx, |editor, cx| {
                        let buffer_start = fold_range
                            .start
                            .to_point(&editor.buffer().read(cx).read(cx));
                        let buffer_row = MultiBufferRow(buffer_start.row);
                        editor.unfold_at(buffer_row, window, cx);
                    })
                    .ok();
            })
            .into_any_element()
    })
}

type ToggleFold = Arc<dyn Fn(bool, &mut Window, &mut App) + Send + Sync>;

fn render_slash_command_output_toggle(
    row: MultiBufferRow,
    is_folded: bool,
    fold: ToggleFold,
    _window: &mut Window,
    _cx: &mut App,
) -> AnyElement {
    Disclosure::new(
        ("slash-command-output-fold-indicator", row.0 as u64),
        !is_folded,
    )
    .toggle_state(is_folded)
    .on_click(move |_e, window, cx| fold(!is_folded, window, cx))
    .into_any_element()
}

pub fn fold_toggle(
    name: &'static str,
) -> impl Fn(
    MultiBufferRow,
    bool,
    Arc<dyn Fn(bool, &mut Window, &mut App) + Send + Sync>,
    &mut Window,
    &mut App,
) -> AnyElement {
    move |row, is_folded, fold, _window, _cx| {
        Disclosure::new((name, row.0 as u64), !is_folded)
            .toggle_state(is_folded)
            .on_click(move |_e, window, cx| fold(!is_folded, window, cx))
            .into_any_element()
    }
}

fn quote_selection_fold_placeholder(title: String, editor: WeakEntity<Editor>) -> FoldPlaceholder {
    FoldPlaceholder {
        render: Arc::new({
            move |fold_id, fold_range, _cx| {
                let editor = editor.clone();
                ButtonLike::new(fold_id)
                    .style(ButtonStyle::Filled)
                    .layer(ElevationIndex::ElevatedSurface)
                    .child(Icon::new(IconName::TextSnippet))
                    .child(Label::new(title.clone()).single_line())
                    .on_click(move |_, window, cx| {
                        editor
                            .update(cx, |editor, cx| {
                                let buffer_start = fold_range
                                    .start
                                    .to_point(&editor.buffer().read(cx).read(cx));
                                let buffer_row = MultiBufferRow(buffer_start.row);
                                editor.unfold_at(buffer_row, window, cx);
                            })
                            .ok();
                    })
                    .into_any_element()
            }
        }),
        merge_adjacent: false,
        ..Default::default()
    }
}

fn render_quote_selection_output_toggle(
    row: MultiBufferRow,
    is_folded: bool,
    fold: ToggleFold,
    _window: &mut Window,
    _cx: &mut App,
) -> AnyElement {
    Disclosure::new(("quote-selection-indicator", row.0 as u64), !is_folded)
        .toggle_state(is_folded)
        .on_click(move |_e, window, cx| fold(!is_folded, window, cx))
        .into_any_element()
}

fn render_pending_slash_command_gutter_decoration(
    row: MultiBufferRow,
    status: &PendingSlashCommandStatus,
    confirm_command: Arc<dyn Fn(&mut Window, &mut App)>,
) -> AnyElement {
    let mut icon = IconButton::new(
        ("slash-command-gutter-decoration", row.0),
        ui::IconName::TriangleRight,
    )
    .on_click(move |_e, window, cx| confirm_command(window, cx))
    .icon_size(ui::IconSize::Small)
    .size(ui::ButtonSize::None);

    match status {
        PendingSlashCommandStatus::Idle => {
            icon = icon.icon_color(Color::Muted);
        }
        PendingSlashCommandStatus::Running { .. } => {
            icon = icon.toggle_state(true);
        }
        PendingSlashCommandStatus::Error(_) => icon = icon.icon_color(Color::Error),
    }

    icon.into_any_element()
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

impl EventEmitter<EditorEvent> for TextThreadEditor {}
impl EventEmitter<SearchEvent> for TextThreadEditor {}

impl Render for TextThreadEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let language_model_selector = self.language_model_selector_menu_handle.clone();

        v_flex()
            .key_context("ContextEditor")
            .capture_action(cx.listener(TextThreadEditor::cancel))
            .capture_action(cx.listener(TextThreadEditor::save))
            .capture_action(cx.listener(TextThreadEditor::copy))
            .capture_action(cx.listener(TextThreadEditor::cut))
            .capture_action(cx.listener(TextThreadEditor::paste))
            .capture_action(cx.listener(TextThreadEditor::cycle_message_role))
            .capture_action(cx.listener(TextThreadEditor::confirm_command))
            .on_action(cx.listener(TextThreadEditor::assist))
            .on_action(cx.listener(TextThreadEditor::split))
            .on_action(move |_: &ToggleModelSelector, window, cx| {
                language_model_selector.toggle(window, cx);
            })
            .size_full()
            .child(
                div()
                    .flex_grow()
                    .bg(cx.theme().colors().editor_background)
                    .child(self.editor.clone()),
            )
            .children(self.render_last_error(cx))
            .child(
                h_flex()
                    .relative()
                    .py_2()
                    .pl_1p5()
                    .pr_2()
                    .w_full()
                    .justify_between()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        h_flex()
                            .gap_0p5()
                            .child(self.render_inject_context_menu(cx))
                            .children(self.render_burn_mode_toggle(cx)),
                    )
                    .child(
                        h_flex()
                            .gap_2p5()
                            .children(self.render_remaining_tokens(cx))
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(self.render_language_model_selector(window, cx))
                                    .child(self.render_send_button(window, cx)),
                            ),
                    ),
            )
    }
}

impl Focusable for TextThreadEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for TextThreadEditor {
    type Event = editor::EditorEvent;

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        util::truncate_and_trailoff(&self.title(cx), MAX_TAB_TITLE_LEN).into()
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

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        Some(self.title(cx).to_string().into())
    }

    fn as_searchable(&self, handle: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn set_nav_history(
        &mut self,
        nav_history: pane::ItemNavHistory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            Item::set_nav_history(editor, nav_history, window, cx)
        })
    }

    fn navigate(
        &mut self,
        data: Box<dyn std::any::Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| Item::navigate(editor, data, window, cx))
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::deactivated(editor, window, cx))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn include_in_nav_history() -> bool {
        false
    }
}

impl SearchableItem for TextThreadEditor {
    type Match = <Editor as SearchableItem>::Match;

    fn clear_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.clear_matches(window, cx);
        });
    }

    fn update_matches(
        &mut self,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor
            .update(cx, |editor, cx| editor.update_matches(matches, window, cx));
    }

    fn query_suggestion(&mut self, window: &mut Window, cx: &mut Context<Self>) -> String {
        self.editor
            .update(cx, |editor, cx| editor.query_suggestion(window, cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.activate_match(index, matches, window, cx);
        });
    }

    fn select_matches(
        &mut self,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor
            .update(cx, |editor, cx| editor.select_matches(matches, window, cx));
    }

    fn replace(
        &mut self,
        identifier: &Self::Match,
        query: &project::search::SearchQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.replace(identifier, query, window, cx)
        });
    }

    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Self::Match>> {
        self.editor
            .update(cx, |editor, cx| editor.find_matches(query, window, cx))
    }

    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[Self::Match],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize> {
        self.editor.update(cx, |editor, cx| {
            editor.active_match_index(direction, matches, window, cx)
        })
    }
}

impl FollowableItem for TextThreadEditor {
    fn remote_id(&self) -> Option<workspace::ViewId> {
        self.remote_id
    }

    fn to_state_proto(&self, window: &Window, cx: &App) -> Option<proto::view::Variant> {
        let context = self.context.read(cx);
        Some(proto::view::Variant::ContextEditor(
            proto::view::ContextEditor {
                context_id: context.id().to_proto(),
                editor: if let Some(proto::view::Variant::Editor(proto)) =
                    self.editor.read(cx).to_state_proto(window, cx)
                {
                    Some(proto)
                } else {
                    None
                },
            },
        ))
    }

    fn from_state_proto(
        workspace: Entity<Workspace>,
        id: workspace::ViewId,
        state: &mut Option<proto::view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        let proto::view::Variant::ContextEditor(_) = state.as_ref()? else {
            return None;
        };
        let Some(proto::view::Variant::ContextEditor(state)) = state.take() else {
            unreachable!()
        };

        let context_id = ContextId::from_proto(state.context_id);
        let editor_state = state.editor?;

        let project = workspace.read(cx).project().clone();
        let agent_panel_delegate = <dyn AgentPanelDelegate>::try_global(cx)?;

        let context_editor_task = workspace.update(cx, |workspace, cx| {
            agent_panel_delegate.open_remote_context(workspace, context_id, window, cx)
        });

        Some(window.spawn(cx, async move |cx| {
            let context_editor = context_editor_task.await?;
            context_editor
                .update_in(cx, |context_editor, window, cx| {
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
                            window,
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
        window: &Window,
        cx: &App,
    ) -> bool {
        self.editor
            .read(cx)
            .add_event_to_update_proto(event, update, window, cx)
    }

    fn apply_update_proto(
        &mut self,
        project: &Entity<Project>,
        message: proto::update_view::Variant,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor.update(cx, |editor, cx| {
            editor.apply_update_proto(project, message, window, cx)
        })
    }

    fn is_project_item(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn set_leader_id(
        &mut self,
        leader_id: Option<CollaboratorId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor
            .update(cx, |editor, cx| editor.set_leader_id(leader_id, window, cx))
    }

    fn dedup(&self, existing: &Self, _window: &Window, cx: &App) -> Option<item::Dedup> {
        if existing.context.read(cx).id() == self.context.read(cx).id() {
            Some(item::Dedup::KeepExisting)
        } else {
            None
        }
    }
}

enum PendingSlashCommand {}

fn invoked_slash_command_fold_placeholder(
    command_id: InvokedSlashCommandId,
    context: WeakEntity<AssistantContext>,
) -> FoldPlaceholder {
    FoldPlaceholder {
        constrain_width: false,
        merge_adjacent: false,
        render: Arc::new(move |fold_id, _, cx| {
            let Some(context) = context.upgrade() else {
                return Empty.into_any();
            };

            let Some(command) = context.read(cx).invoked_slash_command(&command_id) else {
                return Empty.into_any();
            };

            h_flex()
                .id(fold_id)
                .px_1()
                .ml_6()
                .gap_2()
                .bg(cx.theme().colors().surface_background)
                .rounded_sm()
                .child(Label::new(format!("/{}", command.name)))
                .map(|parent| match &command.status {
                    InvokedSlashCommandStatus::Running(_) => {
                        parent.child(Icon::new(IconName::ArrowCircle).with_rotate_animation(4))
                    }
                    InvokedSlashCommandStatus::Error(message) => parent.child(
                        Label::new(format!("error: {message}"))
                            .single_line()
                            .color(Color::Error),
                    ),
                    InvokedSlashCommandStatus::Finished => parent,
                })
                .into_any_element()
        }),
        type_tag: Some(TypeId::of::<PendingSlashCommand>()),
    }
}

enum TokenState {
    NoTokensLeft {
        max_token_count: u64,
        token_count: u64,
    },
    HasMoreTokens {
        max_token_count: u64,
        token_count: u64,
        over_warn_threshold: bool,
    },
}

fn token_state(context: &Entity<AssistantContext>, cx: &App) -> Option<TokenState> {
    const WARNING_TOKEN_THRESHOLD: f32 = 0.8;

    let model = LanguageModelRegistry::read_global(cx)
        .default_model()?
        .model;
    let token_count = context.read(cx).token_count()?;
    let max_token_count = model.max_token_count_for_mode(context.read(cx).completion_mode().into());
    let token_state = if max_token_count.saturating_sub(token_count) == 0 {
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

pub fn humanize_token_count(count: u64) -> String {
    match count {
        0..=999 => count.to_string(),
        1000..=9999 => {
            let thousands = count / 1000;
            let hundreds = (count % 1000 + 50) / 100;
            if hundreds == 0 {
                format!("{}k", thousands)
            } else if hundreds == 10 {
                format!("{}k", thousands + 1)
            } else {
                format!("{}.{}k", thousands, hundreds)
            }
        }
        1_000_000..=9_999_999 => {
            let millions = count / 1_000_000;
            let hundred_thousands = (count % 1_000_000 + 50_000) / 100_000;
            if hundred_thousands == 0 {
                format!("{}M", millions)
            } else if hundred_thousands == 10 {
                format!("{}M", millions + 1)
            } else {
                format!("{}.{}M", millions, hundred_thousands)
            }
        }
        10_000_000.. => format!("{}M", (count + 500_000) / 1_000_000),
        _ => format!("{}k", (count + 500) / 1000),
    }
}

pub fn make_lsp_adapter_delegate(
    project: &Entity<Project>,
    cx: &mut App,
) -> Result<Option<Arc<dyn LspAdapterDelegate>>> {
    project.update(cx, |project, cx| {
        // TODO: Find the right worktree.
        let Some(worktree) = project.worktrees(cx).next() else {
            return Ok(None::<Arc<dyn LspAdapterDelegate>>);
        };
        let http_client = project.client().http_client();
        project.lsp_store().update(cx, |_, cx| {
            Ok(Some(LocalLspAdapterDelegate::new(
                project.languages().clone(),
                project.environment(),
                cx.weak_entity(),
                &worktree,
                http_client,
                project.fs().clone(),
                cx,
            ) as Arc<dyn LspAdapterDelegate>))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::SelectionEffects;
    use fs::FakeFs;
    use gpui::{App, TestAppContext, VisualTestContext};
    use indoc::indoc;
    use language::{Buffer, LanguageRegistry};
    use pretty_assertions::assert_eq;
    use prompt_store::PromptBuilder;
    use text::OffsetRangeExt;
    use unindent::Unindent;
    use util::path;

    #[gpui::test]
    async fn test_copy_paste_whole_message(cx: &mut TestAppContext) {
        let (context, context_editor, mut cx) = setup_context_editor_text(vec![
            (Role::User, "What is the Zed editor?"),
            (
                Role::Assistant,
                "Zed is a modern, high-performance code editor designed from the ground up for speed and collaboration.",
            ),
            (Role::User, ""),
        ],cx).await;

        // Select & Copy whole user message
        assert_copy_paste_context_editor(
            &context_editor,
            message_range(&context, 0, &mut cx),
            indoc! {"
                What is the Zed editor?
                Zed is a modern, high-performance code editor designed from the ground up for speed and collaboration.
                What is the Zed editor?
            "},
            &mut cx,
        );

        // Select & Copy whole assistant message
        assert_copy_paste_context_editor(
            &context_editor,
            message_range(&context, 1, &mut cx),
            indoc! {"
                What is the Zed editor?
                Zed is a modern, high-performance code editor designed from the ground up for speed and collaboration.
                What is the Zed editor?
                Zed is a modern, high-performance code editor designed from the ground up for speed and collaboration.
            "},
            &mut cx,
        );
    }

    #[gpui::test]
    async fn test_copy_paste_no_selection(cx: &mut TestAppContext) {
        let (context, context_editor, mut cx) = setup_context_editor_text(
            vec![
                (Role::User, "user1"),
                (Role::Assistant, "assistant1"),
                (Role::Assistant, "assistant2"),
                (Role::User, ""),
            ],
            cx,
        )
        .await;

        // Copy and paste first assistant message
        let message_2_range = message_range(&context, 1, &mut cx);
        assert_copy_paste_context_editor(
            &context_editor,
            message_2_range.start..message_2_range.start,
            indoc! {"
                user1
                assistant1
                assistant2
                assistant1
            "},
            &mut cx,
        );

        // Copy and cut second assistant message
        let message_3_range = message_range(&context, 2, &mut cx);
        assert_copy_paste_context_editor(
            &context_editor,
            message_3_range.start..message_3_range.start,
            indoc! {"
                user1
                assistant1
                assistant2
                assistant1
                assistant2
            "},
            &mut cx,
        );
    }

    #[gpui::test]
    fn test_find_code_blocks(cx: &mut App) {
        let markdown = languages::language("markdown", tree_sitter_md::LANGUAGE.into());

        let buffer = cx.new(|cx| {
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

    async fn setup_context_editor_text(
        messages: Vec<(Role, &str)>,
        cx: &mut TestAppContext,
    ) -> (
        Entity<AssistantContext>,
        Entity<TextThreadEditor>,
        VisualTestContext,
    ) {
        cx.update(init_test);

        let fs = FakeFs::new(cx.executor());
        let context = create_context_with_messages(messages, cx);

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();
        let mut cx = VisualTestContext::from_window(*window, cx);

        let context_editor = window
            .update(&mut cx, |_, window, cx| {
                cx.new(|cx| {
                    TextThreadEditor::for_context(
                        context.clone(),
                        fs,
                        workspace.downgrade(),
                        project,
                        None,
                        window,
                        cx,
                    )
                })
            })
            .unwrap();

        (context, context_editor, cx)
    }

    fn message_range(
        context: &Entity<AssistantContext>,
        message_ix: usize,
        cx: &mut TestAppContext,
    ) -> Range<usize> {
        context.update(cx, |context, cx| {
            context
                .messages(cx)
                .nth(message_ix)
                .unwrap()
                .anchor_range
                .to_offset(&context.buffer().read(cx).snapshot())
        })
    }

    fn assert_copy_paste_context_editor<T: editor::ToOffset>(
        context_editor: &Entity<TextThreadEditor>,
        range: Range<T>,
        expected_text: &str,
        cx: &mut VisualTestContext,
    ) {
        context_editor.update_in(cx, |context_editor, window, cx| {
            context_editor.editor.update(cx, |editor, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([range])
                });
            });

            context_editor.copy(&Default::default(), window, cx);

            context_editor.editor.update(cx, |editor, cx| {
                editor.move_to_end(&Default::default(), window, cx);
            });

            context_editor.paste(&Default::default(), window, cx);

            context_editor.editor.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), expected_text);
            });
        });
    }

    fn create_context_with_messages(
        mut messages: Vec<(Role, &str)>,
        cx: &mut TestAppContext,
    ) -> Entity<AssistantContext> {
        let registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        cx.new(|cx| {
            let mut context = AssistantContext::local(
                registry,
                None,
                None,
                prompt_builder.clone(),
                Arc::new(SlashCommandWorkingSet::default()),
                cx,
            );
            let mut message_1 = context.messages(cx).next().unwrap();
            let (role, text) = messages.remove(0);

            loop {
                if role == message_1.role {
                    context.buffer().update(cx, |buffer, cx| {
                        buffer.edit([(message_1.offset_range, text)], None, cx);
                    });
                    break;
                }
                let mut ids = HashSet::default();
                ids.insert(message_1.id);
                context.cycle_message_roles(ids, cx);
                message_1 = context.messages(cx).next().unwrap();
            }

            let mut last_message_id = message_1.id;
            for (role, text) in messages {
                context.insert_message_after(last_message_id, role, MessageStatus::Done, cx);
                let message = context.messages(cx).last().unwrap();
                last_message_id = message.id;
                context.buffer().update(cx, |buffer, cx| {
                    buffer.edit([(message.offset_range, text)], None, cx);
                })
            }

            context
        })
    }

    fn init_test(cx: &mut App) {
        let settings_store = SettingsStore::test(cx);
        prompt_store::init(cx);
        LanguageModelRegistry::test(cx);
        cx.set_global(settings_store);
        language::init(cx);
        agent_settings::init(cx);
        Project::init_settings(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        workspace::init_settings(cx);
        editor::init_settings(cx);
    }
}
