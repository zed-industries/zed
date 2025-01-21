use anyhow::Result;
use assistant_settings::AssistantSettings;
use assistant_slash_command::{SlashCommand, SlashCommandOutputSection, SlashCommandWorkingSet};
use assistant_slash_commands::{
    selections_creases, DefaultSlashCommand, DocsSlashCommand, DocsSlashCommandArgs,
    FileSlashCommand,
};
use assistant_tool::ToolWorkingSet;
use client::{proto, zed_urls};
use collections::{hash_map, BTreeSet, HashMap, HashSet};
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
    div, img, percentage, point, prelude::*, pulsating_between, size, Animation, AnimationExt,
    AnyElement, AnyView, AppContext, AsyncWindowContext, ClipboardEntry, ClipboardItem,
    CursorStyle, Empty, Entity, EventEmitter, FocusHandle, FocusableView, FontWeight,
    InteractiveElement, IntoElement, Model, ParentElement, Pixels, Render, RenderImage,
    SharedString, Size, StatefulInteractiveElement, Styled, Subscription, Task, Transformation,
    View, WeakModel, WeakView,
};
use indexed_docs::IndexedDocsStore;
use language::{language_settings::SoftWrap, BufferSnapshot, LspAdapterDelegate, ToOffset};
use language_model::{LanguageModelImage, LanguageModelRegistry, LanguageModelToolUse, Role};
use language_model_selector::{LanguageModelSelector, LanguageModelSelectorPopoverMenu};
use multi_buffer::MultiBufferRow;
use picker::Picker;
use project::{Project, Worktree};
use rope::Point;
use serde::{Deserialize, Serialize};
use settings::{update_settings_file, Settings};
use std::{any::TypeId, borrow::Cow, cmp, ops::Range, path::PathBuf, sync::Arc, time::Duration};
use text::SelectionGoal;
use ui::{
    prelude::*, ButtonLike, Disclosure, ElevationIndex, KeyBinding, PopoverMenuHandle, TintColor,
    Tooltip,
};
use util::{maybe, ResultExt};
use workspace::searchable::SearchableItemHandle;
use workspace::{
    item::{self, FollowableItem, Item, ItemHandle},
    notifications::NotificationId,
    pane::{self, SaveIntent},
    searchable::{SearchEvent, SearchableItem},
    Save, ShowConfiguration, Toast, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    Workspace,
};

use crate::{
    humanize_token_count, slash_command::SlashCommandCompletionProvider, slash_command_picker,
    Assist, AssistantPanel, AssistantPatch, AssistantPatchStatus, CacheStatus, ConfirmCommand,
    Content, Context, ContextEvent, ContextId, CopyCode, CycleMessageRole, Edit,
    InsertDraggedFiles, InsertIntoEditor, InvokedSlashCommandId, InvokedSlashCommandStatus,
    Message, MessageId, MessageMetadata, MessageStatus, ParsedSlashCommand,
    PendingSlashCommandStatus, QuoteSelection, RequestType, Split, ToggleModelSelector,
};

#[derive(Copy, Clone, Debug, PartialEq)]
struct ScrollPosition {
    offset_before_cursor: gpui::Point<f32>,
    cursor: Anchor,
}

struct PatchViewState {
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
    FileRequired,
    PaymentRequired,
    MaxMonthlySpendReached,
    Message(SharedString),
}

pub struct ContextEditor {
    pub(crate) context: Model<Context>,
    fs: Arc<dyn Fs>,
    slash_commands: Arc<SlashCommandWorkingSet>,
    tools: Arc<ToolWorkingSet>,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
    pub(crate) editor: View<Editor>,
    blocks: HashMap<MessageId, (MessageHeader, CustomBlockId)>,
    image_blocks: HashSet<CustomBlockId>,
    scroll_position: Option<ScrollPosition>,
    remote_id: Option<workspace::ViewId>,
    pending_slash_command_creases: HashMap<Range<language::Anchor>, CreaseId>,
    invoked_slash_command_creases: HashMap<InvokedSlashCommandId, CreaseId>,
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

pub const DEFAULT_TAB_TITLE: &str = "New Chat";
const MAX_TAB_TITLE_LEN: usize = 16;

impl ContextEditor {
    pub(crate) fn for_context(
        context: Model<Context>,
        fs: Arc<dyn Fs>,
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
        assistant_panel: WeakView<AssistantPanel>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let completion_provider = SlashCommandCompletionProvider::new(
            context.read(cx).slash_commands.clone(),
            Some(cx.view().downgrade()),
            Some(workspace.clone()),
        );

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::for_buffer(context.read(cx).buffer().clone(), None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_scrollbars(false, cx);
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
        let slash_commands = context.read(cx).slash_commands.clone();
        let tools = context.read(cx).tools.clone();
        let mut this = Self {
            context,
            slash_commands,
            tools,
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
            invoked_slash_command_creases: HashMap::default(),
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

    pub fn insert_default_prompt(&mut self, cx: &mut ViewContext<Self>) {
        let command_name = DefaultSlashCommand.name();
        self.editor.update(cx, |editor, cx| {
            editor.insert(&format!("/{command_name}\n\n"), cx)
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
            cx,
        );
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        self.send_to_model(RequestType::Chat, cx);
    }

    fn edit(&mut self, _: &Edit, cx: &mut ViewContext<Self>) {
        self.send_to_model(RequestType::SuggestEdits, cx);
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

    fn send_to_model(&mut self, request_type: RequestType, cx: &mut ViewContext<Self>) {
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

        if request_type == RequestType::SuggestEdits && !self.context.read(cx).contains_files(cx) {
            self.last_error = Some(AssistError::FileRequired);
            cx.notify();
        } else if let Some(user_message) = self
            .context
            .update(cx, |context, cx| context.assist(request_type, cx))
        {
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

        cx.notify();
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

    fn cursors(&self, cx: &mut WindowContext) -> Vec<usize> {
        let selections = self
            .editor
            .update(cx, |editor, cx| editor.selections.all::<usize>(cx));
        selections
            .into_iter()
            .map(|selection| selection.head())
            .collect()
    }

    pub fn insert_command(&mut self, name: &str, cx: &mut ViewContext<Self>) {
        if let Some(command) = self.slash_commands.command(name, cx) {
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
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(command) = self.slash_commands.command(name, cx) {
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
                                ..Default::default()
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
                                            id: tool_use.id.clone(),
                                            name: tool_use.name.clone(),
                                            input: tool_use.input.clone(),
                                        },
                                    },
                                    cx,
                                );
                            });

                            Crease::inline(
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
                                move |cx: &mut WindowContext| {
                                    context_editor
                                        .update(cx, |context_editor, cx| {
                                            context_editor.run_command(
                                                command.source_range.clone(),
                                                &command.name,
                                                &command.arguments,
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
                                ..Default::default()
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
                self.update_invoked_slash_command(*command_id, cx);
            }
            ContextEvent::SlashCommandOutputSectionAdded { section } => {
                self.insert_slash_command_output_sections([section.clone()], false, cx);
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
                    if let Some(tool) = self.tools.tool(&tool_use.name, cx) {
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
                        ..Default::default()
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

                    let crease = Crease::inline(
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

    fn update_invoked_slash_command(
        &mut self,
        command_id: InvokedSlashCommandId,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(invoked_slash_command) =
            self.context.read(cx).invoked_slash_command(&command_id)
        {
            if let InvokedSlashCommandStatus::Finished = invoked_slash_command.status {
                let run_commands_in_ranges = invoked_slash_command
                    .run_commands_in_ranges
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
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
                            cx,
                        );
                    }
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
                        |_row, _folded, _cx| Empty.into_any(),
                    );
                    let crease_ids = editor.insert_creases([crease.clone()], cx);
                    editor.fold_creases(vec![crease], false, cx);
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

    fn patches_updated(
        &mut self,
        removed: &Vec<Range<text::Anchor>>,
        updated: &Vec<Range<text::Anchor>>,
        cx: &mut ViewContext<ContextEditor>,
    ) {
        let this = cx.view().downgrade();
        let mut editors_to_close = Vec::new();

        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let multibuffer = &snapshot.buffer_snapshot;
            let (&excerpt_id, _, _) = multibuffer.as_singleton().unwrap();

            let mut removed_crease_ids = Vec::new();
            let mut ranges_to_unfold: Vec<Range<Anchor>> = Vec::new();
            for range in removed {
                if let Some(state) = self.patches.remove(range) {
                    let patch_start = multibuffer
                        .anchor_in_excerpt(excerpt_id, range.start)
                        .unwrap();
                    let patch_end = multibuffer
                        .anchor_in_excerpt(excerpt_id, range.end)
                        .unwrap();

                    editors_to_close.extend(state.editor.and_then(|state| state.editor.upgrade()));
                    ranges_to_unfold.push(patch_start..patch_end);
                    removed_crease_ids.push(state.crease_id);
                }
            }
            editor.unfold_ranges(&ranges_to_unfold, true, false, cx);
            editor.remove_creases(removed_crease_ids, cx);

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
                let render_block: RenderBlock = Arc::new({
                    let this = this.clone();
                    let patch_range = range.clone();
                    move |cx: &mut BlockContext<'_, '_>| {
                        let max_width = cx.max_width;
                        let gutter_width = cx.gutter_dimensions.full_width();
                        let block_id = cx.block_id;
                        let selected = cx.selected;
                        this.update(&mut **cx, |this, cx| {
                            this.render_patch_block(
                                patch_range.clone(),
                                max_width,
                                gutter_width,
                                block_id,
                                selected,
                                cx,
                            )
                        })
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| Empty.into_any())
                    }
                });

                let height = path_count as u32 + 1;
                let crease = Crease::block(
                    patch_start..patch_end,
                    height,
                    BlockStyle::Flex,
                    render_block.clone(),
                );

                let should_refold;
                if let Some(state) = self.patches.get_mut(&range) {
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
                    let crease_id = editor.insert_creases([crease.clone()], cx)[0];
                    self.patches.insert(
                        range.clone(),
                        PatchViewState {
                            crease_id,
                            editor: None,
                            update_task: None,
                        },
                    );

                    should_refold = true;
                }

                if should_refold {
                    editor.unfold_ranges(&[patch_start..patch_end], true, false, cx);
                    editor.fold_creases(vec![crease], false, cx);
                }
            }
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
                    Crease::inline(
                        start..end,
                        FoldPlaceholder {
                            render: render_fold_icon_button(
                                cx.view().downgrade(),
                                section.icon,
                                section.label.clone(),
                            ),
                            merge_adjacent: false,
                            ..Default::default()
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
        let newest_cursor = self.editor.update(cx, |editor, cx| {
            editor.selections.newest::<Point>(cx).head()
        });
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

    fn esc_kbd(cx: &WindowContext) -> Div {
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
                    .rounded_md()
                    .px_1()
                    .mr_0p5()
                    .border_1()
                    .border_color(theme::color_alpha(colors.border_variant, 0.6))
                    .bg(theme::color_alpha(colors.element_background, 0.6))
                    .child("esc"),
            )
            .child("to cancel")
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
                                let base_label = Label::new("Assistant").color(Color::Info);
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
                                            .with_animation(
                                                "arrow-circle",
                                                Animation::new(Duration::from_secs(2)).repeat(),
                                                |icon, delta| {
                                                    icon.transform(Transformation::rotate(
                                                        percentage(delta),
                                                    ))
                                                },
                                            )
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
                                    }),
                            )
                            .children(note);

                        h_flex()
                            .id(("message_header", message_id.as_u64()))
                            .pl(cx.gutter_dimensions.full_width())
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
                                            .tooltip(|cx| {
                                                Tooltip::with_meta(
                                                    "Context Cached",
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
                                        .icon_size(IconSize::XSmall)
                                        .icon_position(IconPosition::Start)
                                        .tooltip(move |cx| Tooltip::text("View Details", cx))
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
                let anchor = context_editor.selections.newest_anchor();
                let text = context_editor
                    .buffer()
                    .read(cx)
                    .read(cx)
                    .text_for_range(anchor.range())
                    .collect::<String>();

                (!text.is_empty()).then_some((text, false))
            }
        })
    }

    pub fn insert_selection(
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

    pub fn copy_code(workspace: &mut Workspace, _: &CopyCode, cx: &mut ViewContext<Workspace>) {
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

    pub fn insert_dragged_files(
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
            let cmd_name = FileSlashCommand.name();

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

    pub fn quote_selection(
        workspace: &mut Workspace,
        _: &QuoteSelection,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        let Some(creases) = selections_creases(workspace, cx) else {
            return;
        };

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
                                let crease = Crease::inline(
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
                            Crease::inline(
                                start..end,
                                FoldPlaceholder {
                                    render: render_fold_icon_button(
                                        weak_editor.clone(),
                                        metadata.crease.icon,
                                        metadata.crease.label.clone(),
                                    ),
                                    ..Default::default()
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
                    let Some(render_image) = image.to_image_data(cx.svg_renderer()).log_err()
                    else {
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
                        render: Arc::new(move |cx| {
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

    pub fn title(&self, cx: &AppContext) -> Cow<str> {
        self.context
            .read(cx)
            .summary()
            .map(|summary| summary.text.clone())
            .map(Cow::Owned)
            .unwrap_or_else(|| Cow::Borrowed(DEFAULT_TAB_TITLE))
    }

    fn render_patch_block(
        &mut self,
        range: Range<text::Anchor>,
        max_width: Pixels,
        gutter_width: Pixels,
        id: BlockId,
        selected: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        let snapshot = self.editor.update(cx, |editor, cx| editor.snapshot(cx));
        let (excerpt_id, _buffer_id, _) = snapshot.buffer_snapshot.as_singleton().unwrap();
        let excerpt_id = *excerpt_id;
        let anchor = snapshot
            .buffer_snapshot
            .anchor_in_excerpt(excerpt_id, range.start)
            .unwrap();

        let theme = cx.theme().clone();
        let patch = self.context.read(cx).patch_for_range(&range, cx)?;
        let paths = patch
            .paths()
            .map(|p| SharedString::from(p.to_string()))
            .collect::<BTreeSet<_>>();

        Some(
            v_flex()
                .id(id)
                .bg(theme.colors().editor_background)
                .ml(gutter_width)
                .pb_1()
                .w(max_width - gutter_width)
                .rounded_md()
                .border_1()
                .border_color(theme.colors().border_variant)
                .overflow_hidden()
                .hover(|style| style.border_color(theme.colors().text_accent))
                .when(selected, |this| {
                    this.border_color(theme.colors().text_accent)
                })
                .cursor(CursorStyle::PointingHand)
                .on_click(cx.listener(move |this, _, cx| {
                    this.editor.update(cx, |editor, cx| {
                        editor.change_selections(None, cx, |selections| {
                            selections.select_ranges(vec![anchor..anchor]);
                        });
                    });
                    this.focus_active_patch(cx);
                }))
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .overflow_hidden()
                        .text_ellipsis()
                        .border_b_1()
                        .border_color(theme.colors().border_variant)
                        .bg(theme.colors().element_background)
                        .child(
                            Label::new(patch.title.clone())
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .children(paths.into_iter().map(|path| {
                    h_flex()
                        .px_2()
                        .pt_1()
                        .gap_1p5()
                        .child(Icon::new(IconName::File).size(IconSize::Small))
                        .child(Label::new(path).size(LabelSize::Small))
                }))
                .when(patch.status == AssistantPatchStatus::Pending, |div| {
                    div.child(
                        h_flex()
                            .pt_1()
                            .px_2()
                            .gap_1()
                            .child(
                                Icon::new(IconName::ArrowCircle)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted)
                                    .with_animation(
                                        "arrow-circle",
                                        Animation::new(Duration::from_secs(2)).repeat(),
                                        |icon, delta| {
                                            icon.transform(Transformation::rotate(percentage(
                                                delta,
                                            )))
                                        },
                                    ),
                            )
                            .child(
                                Label::new("Generating…")
                                    .color(Color::Muted)
                                    .size(LabelSize::Small)
                                    .with_animation(
                                        "pulsating-label",
                                        Animation::new(Duration::from_secs(2))
                                            .repeat()
                                            .with_easing(pulsating_between(0.4, 0.8)),
                                        |label, delta| label.alpha(delta),
                                    ),
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
                ButtonStyle::Tinted(TintColor::Error),
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
            .child(Label::new(
                if AssistantSettings::get_global(cx).are_live_diffs_enabled(cx) {
                    "Chat"
                } else {
                    "Send"
                },
            ))
            .children(
                KeyBinding::for_action_in(&Assist, &focus_handle, cx)
                    .map(|binding| binding.into_any_element()),
            )
            .on_click(move |_event, cx| {
                focus_handle.dispatch_action(&Assist, cx);
            })
    }

    fn render_edit_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();

        let (style, tooltip) = match token_state(&self.context, cx) {
            Some(TokenState::NoTokensLeft { .. }) => (
                ButtonStyle::Tinted(TintColor::Error),
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

        ButtonLike::new("edit_button")
            .disabled(disabled)
            .style(style)
            .when_some(tooltip, |button, tooltip| {
                button.tooltip(move |_| tooltip.clone())
            })
            .layer(ElevationIndex::ModalSurface)
            .child(Label::new("Suggest Edits"))
            .children(
                KeyBinding::for_action_in(&Edit, &focus_handle, cx)
                    .map(|binding| binding.into_any_element()),
            )
            .on_click(move |_event, cx| {
                focus_handle.dispatch_action(&Edit, cx);
            })
    }

    fn render_inject_context_menu(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        slash_command_picker::SlashCommandSelector::new(
            self.slash_commands.clone(),
            cx.view().downgrade(),
            Button::new("trigger", "Add Context")
                .icon(IconName::Plus)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .icon_position(IconPosition::Start)
                .tooltip(|cx| Tooltip::text("Type / to insert via keyboard", cx)),
        )
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
                    AssistError::FileRequired => self.render_file_required_error(cx),
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

    fn render_file_required_error(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        v_flex()
            .gap_0p5()
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .child(Icon::new(IconName::Warning).color(Color::Warning))
                    .child(
                        Label::new("Suggest Edits needs a file to edit").weight(FontWeight::MEDIUM),
                    ),
            )
            .child(
                div()
                    .id("error-message")
                    .max_h_24()
                    .overflow_y_scroll()
                    .child(Label::new(
                        "To include files, type /file or /tab in your prompt.",
                    )),
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
                    .max_h_32()
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
    .toggle_state(is_folded)
    .on_click(move |_e, cx| fold(!is_folded, cx))
    .into_any_element()
}

pub fn fold_toggle(
    name: &'static str,
) -> impl Fn(
    MultiBufferRow,
    bool,
    Arc<dyn Fn(bool, &mut WindowContext) + Send + Sync>,
    &mut WindowContext,
) -> AnyElement {
    move |row, is_folded, fold, _cx| {
        Disclosure::new((name, row.0 as u64), !is_folded)
            .toggle_state(is_folded)
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
        merge_adjacent: false,
        ..Default::default()
    }
}

fn render_quote_selection_output_toggle(
    row: MultiBufferRow,
    is_folded: bool,
    fold: ToggleFold,
    _cx: &mut WindowContext,
) -> AnyElement {
    Disclosure::new(("quote-selection-indicator", row.0 as u64), !is_folded)
        .toggle_state(is_folded)
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
            icon = icon.toggle_state(true);
        }
        PendingSlashCommandStatus::Error(_) => icon = icon.icon_color(Color::Error),
    }

    icon.into_any_element()
}

fn render_docs_slash_command_trailer(
    row: MultiBufferRow,
    command: ParsedSlashCommand,
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

        v_flex()
            .key_context("ContextEditor")
            .capture_action(cx.listener(ContextEditor::cancel))
            .capture_action(cx.listener(ContextEditor::save))
            .capture_action(cx.listener(ContextEditor::copy))
            .capture_action(cx.listener(ContextEditor::cut))
            .capture_action(cx.listener(ContextEditor::paste))
            .capture_action(cx.listener(ContextEditor::cycle_message_role))
            .capture_action(cx.listener(ContextEditor::confirm_command))
            .on_action(cx.listener(ContextEditor::edit))
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
                        .child(h_flex().gap_1().child(self.render_inject_context_menu(cx)))
                        .child(
                            h_flex()
                                .w_full()
                                .justify_end()
                                .when(
                                    AssistantSettings::get_global(cx).are_live_diffs_enabled(cx),
                                    |buttons| {
                                        buttons
                                            .items_center()
                                            .gap_1p5()
                                            .child(self.render_edit_button(cx))
                                            .child(
                                                Label::new("or")
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            )
                                    },
                                )
                                .child(self.render_send_button(cx)),
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

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a View<Self>,
        _: &'a AppContext,
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
    active_context_editor: Option<WeakView<ContextEditor>>,
    model_summary_editor: View<Editor>,
    language_model_selector: View<LanguageModelSelector>,
    language_model_selector_menu_handle: PopoverMenuHandle<LanguageModelSelector>,
}

impl ContextEditorToolbarItem {
    pub fn new(
        workspace: &Workspace,
        model_selector_menu_handle: PopoverMenuHandle<LanguageModelSelector>,
        model_summary_editor: View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            active_context_editor: None,
            model_summary_editor,
            language_model_selector: cx.new_view(|cx| {
                let fs = workspace.app_state().fs.clone();
                LanguageModelSelector::new(
                    move |model, cx| {
                        update_settings_file::<AssistantSettings>(
                            fs.clone(),
                            cx,
                            move |settings, _| settings.set_model(model.clone()),
                        );
                    },
                    cx,
                )
            }),
            language_model_selector_menu_handle: model_selector_menu_handle,
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
            .group("chat-title-group")
            .gap_1()
            .items_center()
            .flex_grow()
            .child(
                div()
                    .w_full()
                    .when(self.active_context_editor.is_some(), |left_side| {
                        left_side.child(self.model_summary_editor.clone())
                    }),
            )
            .child(
                div().visible_on_hover("chat-title-group").child(
                    IconButton::new("regenerate-context", IconName::RefreshTitle)
                        .shape(ui::IconButtonShape::Square)
                        .tooltip(|cx| Tooltip::text("Regenerate Title", cx))
                        .on_click(cx.listener(move |_, _, cx| {
                            cx.emit(ContextEditorToolbarItemEvent::RegenerateSummary)
                        })),
                ),
            );
        let active_provider = LanguageModelRegistry::read_global(cx).active_provider();
        let active_model = LanguageModelRegistry::read_global(cx).active_model();
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
                LanguageModelSelectorPopoverMenu::new(
                    self.language_model_selector.clone(),
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
                                                    Icon::new(
                                                        model
                                                            .icon()
                                                            .unwrap_or_else(|| provider.icon()),
                                                    )
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
                .with_handle(self.language_model_selector_menu_handle.clone()),
            )
            .children(self.render_remaining_tokens(cx));

        h_flex()
            .px_0p5()
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

pub enum ContextEditorToolbarItemEvent {
    RegenerateSummary,
}
impl EventEmitter<ContextEditorToolbarItemEvent> for ContextEditorToolbarItem {}

enum PendingSlashCommand {}

fn invoked_slash_command_fold_placeholder(
    command_id: InvokedSlashCommandId,
    context: WeakModel<Context>,
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
                .rounded_md()
                .child(Label::new(format!("/{}", command.name.clone())))
                .map(|parent| match &command.status {
                    InvokedSlashCommandStatus::Running(_) => {
                        parent.child(Icon::new(IconName::ArrowCircle).with_animation(
                            "arrow-circle",
                            Animation::new(Duration::from_secs(4)).repeat(),
                            |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                        ))
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
