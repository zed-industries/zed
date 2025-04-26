use anyhow::Result;
use assistant_settings::AssistantSettings;
use assistant_slash_command::{SlashCommand, SlashCommandOutputSection, SlashCommandWorkingSet};
use assistant_slash_commands::{
    DefaultSlashCommand, DocsSlashCommand, DocsSlashCommandArgs, FileSlashCommand,
    selections_creases,
};
use client::{proto, zed_urls};
use collections::{BTreeSet, HashMap, HashSet, hash_map};
use editor::{
    Anchor, Editor, EditorEvent, MenuInlineCompletionsPolicy, MultiBuffer, MultiBufferSnapshot,
    ProposedChangeLocation, ProposedChangesEditor, RowExt, ToOffset as _, ToPoint,
    actions::{MoveToEndOfLine, Newline, ShowCompletions},
    display_map::{
        BlockContext, BlockId, BlockPlacement, BlockProperties, BlockStyle, Crease, CreaseMetadata,
        CustomBlockId, FoldId, RenderBlock, ToDisplayPoint,
    },
    scroll::Autoscroll,
};
use editor::{FoldPlaceholder, display_map::CreaseId};
use feature_flags::{Assistant2FeatureFlag, FeatureFlagAppExt as _};
use fs::Fs;
use futures::FutureExt;
use gpui::{
    Animation, AnimationExt, AnyElement, AnyView, App, AsyncWindowContext, ClipboardEntry,
    ClipboardItem, CursorStyle, Empty, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    Global, InteractiveElement, IntoElement, ParentElement, Pixels, Render, RenderImage,
    SharedString, Size, StatefulInteractiveElement, Styled, Subscription, Task, Transformation,
    WeakEntity, actions, div, img, impl_internal_actions, percentage, point, prelude::*,
    pulsating_between, size,
};
use indexed_docs::IndexedDocsStore;
use language::{
    BufferSnapshot, LspAdapterDelegate, ToOffset,
    language_settings::{SoftWrap, all_language_settings},
};
use language_model::{
    LanguageModelImage, LanguageModelProvider, LanguageModelProviderTosView, LanguageModelRegistry,
    Role,
};
use language_model_selector::{
    LanguageModelSelector, LanguageModelSelectorPopoverMenu, ModelType, ToggleModelSelector,
};
use multi_buffer::MultiBufferRow;
use picker::Picker;
use project::lsp_store::LocalLspAdapterDelegate;
use project::{Project, Worktree};
use rope::Point;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore, update_settings_file};
use std::{any::TypeId, borrow::Cow, cmp, ops::Range, path::PathBuf, sync::Arc, time::Duration};
use text::SelectionGoal;
use ui::{
    ButtonLike, Disclosure, ElevationIndex, KeyBinding, PopoverMenuHandle, TintColor, Tooltip,
    prelude::*,
};
use util::{ResultExt, maybe};
use workspace::searchable::{Direction, SearchableItemHandle};
use workspace::{
    Save, Toast, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    item::{self, FollowableItem, Item, ItemHandle},
    notifications::NotificationId,
    pane::{self, SaveIntent},
    searchable::{SearchEvent, SearchableItem},
};

use crate::{
    AssistantContext, AssistantPatch, AssistantPatchStatus, CacheStatus, Content, ContextEvent,
    ContextId, InvokedSlashCommandId, InvokedSlashCommandStatus, Message, MessageId,
    MessageMetadata, MessageStatus, ParsedSlashCommand, PendingSlashCommandStatus, RequestType,
};
use crate::{
    ThoughtProcessOutputSection, slash_command::SlashCommandCompletionProvider,
    slash_command_picker,
};

actions!(
    assistant,
    [
        Assist,
        ConfirmCommand,
        CopyCode,
        CycleMessageRole,
        Edit,
        InsertIntoEditor,
        QuoteSelection,
        Split,
    ]
);

#[derive(PartialEq, Clone)]
pub enum InsertDraggedFiles {
    ProjectPaths(Vec<PathBuf>),
    ExternalFiles(Vec<PathBuf>),
}

impl_internal_actions!(assistant, [InsertDraggedFiles]);

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
    editor: WeakEntity<ProposedChangesEditor>,
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

pub enum ThoughtProcessStatus {
    Pending,
    Completed,
}

pub trait AssistantPanelDelegate {
    fn active_context_editor(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<ContextEditor>>;

    fn open_saved_context(
        &self,
        workspace: &mut Workspace,
        path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>>;

    fn open_remote_context(
        &self,
        workspace: &mut Workspace,
        context_id: ContextId,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<Entity<ContextEditor>>>;

    fn quote_selection(
        &self,
        workspace: &mut Workspace,
        selection_ranges: Vec<Range<Anchor>>,
        buffer: Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    );
}

impl dyn AssistantPanelDelegate {
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

struct GlobalAssistantPanelDelegate(Arc<dyn AssistantPanelDelegate>);

impl Global for GlobalAssistantPanelDelegate {}

pub struct ContextEditor {
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
    patches: HashMap<Range<language::Anchor>, PatchViewState>,
    active_patch: Option<Range<language::Anchor>>,
    last_error: Option<AssistError>,
    show_accept_terms: bool,
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

pub const DEFAULT_TAB_TITLE: &str = "New Chat";
const MAX_TAB_TITLE_LEN: usize = 16;

impl ContextEditor {
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
            editor.disable_scrollbars_and_minimap(cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Some(Box::new(completion_provider)));
            editor.set_menu_inline_completions_policy(MenuInlineCompletionsPolicy::Never);
            editor.set_collaboration_hub(Box::new(project.clone()));

            let show_edit_predictions = all_language_settings(None, cx)
                .edit_predictions
                .enabled_in_assistant;

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
        let patch_ranges = context.read(cx).patch_ranges().collect::<Vec<_>>();
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
            patches: HashMap::default(),
            active_patch: None,
            last_error: None,
            show_accept_terms: false,
            slash_menu_handle: Default::default(),
            dragged_file_worktrees: Vec::new(),
            language_model_selector: cx.new(|cx| {
                LanguageModelSelector::new(
                    move |model, cx| {
                        update_settings_file::<AssistantSettings>(
                            fs.clone(),
                            cx,
                            move |settings, _| settings.set_model(model.clone()),
                        );
                    },
                    ModelType::Default,
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
        this.patches_updated(&Vec::new(), &patch_ranges, window, cx);
        this
    }

    fn settings_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            let show_edit_predictions = all_language_settings(None, cx)
                .edit_predictions
                .enabled_in_assistant;

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
        self.send_to_model(RequestType::Chat, window, cx);
    }

    fn edit(&mut self, _: &Edit, window: &mut Window, cx: &mut Context<Self>) {
        self.send_to_model(RequestType::SuggestEdits, window, cx);
    }

    fn focus_active_patch(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if let Some((_range, patch)) = self.active_patch() {
            if let Some(editor) = patch
                .editor
                .as_ref()
                .and_then(|state| state.editor.upgrade())
            {
                editor.focus_handle(cx).focus(window);
                return true;
            }
        }

        false
    }

    fn send_to_model(
        &mut self,
        request_type: RequestType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let provider = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.provider);
        if provider
            .as_ref()
            .map_or(false, |provider| provider.must_accept_terms(cx))
        {
            self.show_accept_terms = true;
            cx.notify();
            return;
        }

        if self.focus_active_patch(window, cx) {
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
                editor.change_selections(Some(Autoscroll::fit()), window, cx, |selections| {
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
                    editor
                        .change_selections(Some(Autoscroll::fit()), window, cx, |s| s.try_cancel());
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
            ContextEvent::PatchesUpdated { removed, updated } => {
                self.patches_updated(removed, updated, window, cx);
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
                                let command = command.clone();
                                move |row, _unfold, _window: &mut Window, cx: &mut App| {
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
            ContextEvent::ShowMaxMonthlySpendReachedError => {
                self.last_error = Some(AssistError::MaxMonthlySpendReached);
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
                            window,
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

    fn patches_updated(
        &mut self,
        removed: &Vec<Range<text::Anchor>>,
        updated: &Vec<Range<text::Anchor>>,
        window: &mut Window,
        cx: &mut Context<ContextEditor>,
    ) {
        let this = cx.entity().downgrade();
        let mut editors_to_close = Vec::new();

        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(window, cx);
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
                    move |cx: &mut BlockContext| {
                        let max_width = cx.max_width;
                        let gutter_width = cx.gutter_dimensions.full_width();
                        let block_id = cx.block_id;
                        let selected = cx.selected;
                        let window = &mut cx.window;
                        this.update(cx.app, |this, cx| {
                            this.render_patch_block(
                                patch_range.clone(),
                                max_width,
                                gutter_width,
                                block_id,
                                selected,
                                window,
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
                                cx.spawn_in(window, async move |_, cx| {
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
                    editor.fold_creases(vec![crease], false, window, cx);
                }
            }
        });

        for editor in editors_to_close {
            self.close_patch_editor(editor, window, cx);
        }

        self.update_active_patch(window, cx);
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
                        icon: IconName::Ai,
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
                                section.icon,
                                section.label.clone(),
                            ),
                            merge_adjacent: false,
                            ..Default::default()
                        },
                        render_slash_command_output_toggle,
                        |_, _, _, _| Empty.into_any_element(),
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
                self.update_active_patch(window, cx);
            }
            _ => {}
        }
        cx.emit(event.clone());
    }

    fn active_patch(&self) -> Option<(Range<text::Anchor>, &PatchViewState)> {
        let patch = self.active_patch.as_ref()?;
        Some((patch.clone(), self.patches.get(&patch)?))
    }

    fn update_active_patch(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
                        self.close_patch_editor(editor, window, cx);
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
                            workspace.activate_item(&editor, true, false, window, cx);
                        })
                        .ok();
                } else {
                    patch_state.update_task = Some(cx.spawn_in(window, async move |this, cx| {
                        Self::open_patch_editor(this, new_patch, cx).await.log_err();
                    }));
                }
            }
        }
    }

    fn close_patch_editor(
        &mut self,
        editor: Entity<ProposedChangesEditor>,
        window: &mut Window,
        cx: &mut Context<ContextEditor>,
    ) {
        self.workspace
            .update(cx, |workspace, cx| {
                if let Some(pane) = workspace.pane_for(&editor) {
                    pane.update(cx, |pane, cx| {
                        let item_id = editor.entity_id();
                        if !editor.read(cx).focus_handle(cx).is_focused(window) {
                            pane.close_item_by_id(item_id, SaveIntent::Skip, window, cx)
                                .detach_and_log_err(cx);
                        }
                    });
                }
            })
            .ok();
    }

    async fn open_patch_editor(
        this: WeakEntity<Self>,
        patch: AssistantPatch,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let project = this.read_with(cx, |this, _| this.project.clone())?;
        let resolved_patch = patch.resolve(project.clone(), cx).await;

        let editor = cx.new_window_entity(|window, cx| {
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
                window,
                cx,
            );
            resolved_patch.apply(&editor, cx);
            editor
        })?;

        this.update(cx, |this, _| {
            if let Some(patch_state) = this.patches.get_mut(&patch.range) {
                patch_state.editor = Some(PatchEditorState {
                    editor: editor.downgrade(),
                    opened_patch: patch,
                });
                patch_state.update_task.take();
            }
        })?;
        this.read_with(cx, |this, _| this.workspace.clone())?
            .update_in(cx, |workspace, window, cx| {
                workspace.add_item_to_active_pane(Box::new(editor.clone()), None, false, window, cx)
            })
            .log_err();

        Ok(())
    }

    async fn update_patch_editor(
        this: WeakEntity<Self>,
        patch: AssistantPatch,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let project = this.update(cx, |this, _| this.project.clone())?;
        let resolved_patch = patch.resolve(project.clone(), cx).await;
        this.update_in(cx, |this, window, cx| {
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
                        editor.reset_locations(locations, window, cx);
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
                                    .tooltip(|window, cx| {
                                        Tooltip::with_meta(
                                            "Toggle message role",
                                            None,
                                            "Available roles: You (User), Assistant, System",
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
        context_editor_view: &Entity<ContextEditor>,
        cx: &mut Context<Workspace>,
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
        let Some(assistant_panel_delegate) = <dyn AssistantPanelDelegate>::try_global(cx) else {
            return;
        };
        let Some(context_editor_view) =
            assistant_panel_delegate.active_context_editor(workspace, window, cx)
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
            let assistant_panel_delegate = <dyn AssistantPanelDelegate>::try_global(cx)?;
            let context_editor_view =
                assistant_panel_delegate.active_context_editor(workspace, window, cx)?;
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
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(assistant_panel_delegate) = <dyn AssistantPanelDelegate>::try_global(cx) else {
            return;
        };
        let Some(context_editor_view) =
            assistant_panel_delegate.active_context_editor(workspace, window, cx)
        else {
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

                cx.spawn(async move |_, cx| {
                    let mut paths = vec![];
                    let mut worktrees = vec![];

                    let opened_paths = futures::future::join_all(tasks).await;
                    for (worktree, project_path) in opened_paths.into_iter().flatten() {
                        let Ok(worktree_root_name) =
                            worktree.read_with(cx, |worktree, _| worktree.root_name().to_string())
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

        window
            .spawn(cx, async move |cx| {
                let (paths, dragged_file_worktrees) = paths.await;
                let cmd_name = FileSlashCommand.name();

                context_editor_view
                    .update_in(cx, |context_editor, window, cx| {
                        let file_argument = paths
                            .into_iter()
                            .map(|path| path.to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                            .join(" ");

                        context_editor.editor.update(cx, |editor, cx| {
                            editor.insert("\n", window, cx);
                            editor.insert(&format!("/{} {}", cmd_name, file_argument), window, cx);
                        });

                        context_editor.confirm_command(&ConfirmCommand, window, cx);

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
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(assistant_panel_delegate) = <dyn AssistantPanelDelegate>::try_global(cx) else {
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

        assistant_panel_delegate.quote_selection(workspace, selections, buffer, window, cx);
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
                    this.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
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
        let (selection, creases) = self.editor.update(cx, |editor, cx| {
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
                                        metadata.crease.icon,
                                        metadata.crease.label.clone(),
                                    ),
                                    ..Default::default()
                                },
                                render_slash_command_output_toggle,
                                |_, _, _, _| Empty.into_any(),
                            )
                            .with_metadata(metadata.crease.clone())
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
                    let image = render_image.clone();
                    anchor.is_valid(&buffer).then(|| BlockProperties {
                        placement: BlockPlacement::Above(anchor),
                        height: Some(MAX_HEIGHT_IN_LINES),
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

    fn split(&mut self, _: &Split, _window: &mut Window, cx: &mut Context<Self>) {
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

    fn save(&mut self, _: &Save, _window: &mut Window, cx: &mut Context<Self>) {
        self.context.update(cx, |context, cx| {
            context.save(Some(Duration::from_millis(500)), self.fs.clone(), cx)
        });
    }

    pub fn title(&self, cx: &App) -> Cow<str> {
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let snapshot = self
            .editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));
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
                .rounded_sm()
                .border_1()
                .border_color(theme.colors().border_variant)
                .overflow_hidden()
                .hover(|style| style.border_color(theme.colors().text_accent))
                .when(selected, |this| {
                    this.border_color(theme.colors().text_accent)
                })
                .cursor(CursorStyle::PointingHand)
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.editor.update(cx, |editor, cx| {
                        editor.change_selections(None, window, cx, |selections| {
                            selections.select_ranges(vec![anchor..anchor]);
                        });
                    });
                    this.focus_active_patch(window, cx);
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

    fn render_notice(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        // This was previously gated behind the `zed-pro` feature flag. Since we
        // aren't planning to ship that right now, we're just hard-coding this
        // value to not show the nudge.
        let nudge = Some(false);

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
                            .on_click(cx.listener(|this, _event, _window, cx| {
                                let client = this
                                    .workspace
                                    .update(cx, |workspace, _| workspace.client().clone())
                                    .log_err();

                                if let Some(client) = client {
                                    cx.spawn(async move |this, cx| {
                                        client.authenticate_and_connect(true, cx).await?;
                                        this.update(cx, |_, cx| cx.notify())
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
                ConfigurationError::ProviderPendingTermsAcceptance(_) => {
                    "LLM provider requires accepting the Terms of Service."
                }
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
                                move |_event, window, cx| {
                                    if cx.has_flag::<Assistant2FeatureFlag>() {
                                        focus_handle.dispatch_action(
                                            &zed_actions::agent::OpenConfiguration,
                                            window,
                                            cx,
                                        );
                                    } else {
                                        focus_handle.dispatch_action(
                                            &zed_actions::assistant::ShowConfiguration,
                                            window,
                                            cx,
                                        );
                                    };
                                }
                            }),
                    )
                    .into_any_element(),
            )
        } else {
            None
        }
    }

    fn render_send_button(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();

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

        let model = LanguageModelRegistry::read_global(cx).default_model();

        let has_configuration_error = configuration_error(cx).is_some();
        let needs_to_accept_terms = self.show_accept_terms
            && model
                .as_ref()
                .map_or(false, |model| model.provider.must_accept_terms(cx));
        let disabled = has_configuration_error || needs_to_accept_terms;

        ButtonLike::new("send_button")
            .disabled(disabled)
            .style(style)
            .when_some(tooltip, |button, tooltip| {
                button.tooltip(move |_, _| tooltip.clone())
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
                KeyBinding::for_action_in(&Assist, &focus_handle, window, cx)
                    .map(|binding| binding.into_any_element()),
            )
            .on_click(move |_event, window, cx| {
                focus_handle.dispatch_action(&Assist, window, cx);
            })
    }

    fn render_edit_button(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();

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

        let provider = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.provider);

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
                button.tooltip(move |_, _| tooltip.clone())
            })
            .layer(ElevationIndex::ModalSurface)
            .child(Label::new("Suggest Edits"))
            .children(
                KeyBinding::for_action_in(&Edit, &focus_handle, window, cx)
                    .map(|binding| binding.into_any_element()),
            )
            .on_click(move |_event, window, cx| {
                focus_handle.dispatch_action(&Edit, window, cx);
            })
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

    fn render_language_model_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_model = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.model);
        let focus_handle = self.editor().focus_handle(cx).clone();
        let model_name = match active_model {
            Some(model) => model.name().0,
            None => SharedString::from("No model selected"),
        };

        LanguageModelSelectorPopoverMenu::new(
            self.language_model_selector.clone(),
            ButtonLike::new("active-model")
                .style(ButtonStyle::Subtle)
                .child(
                    h_flex()
                        .gap_0p5()
                        .child(
                            Label::new(model_name)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
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
        )
        .with_handle(self.language_model_selector_menu_handle.clone())
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

    fn render_file_required_error(&self, cx: &mut Context<Self>) -> AnyElement {
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
                        |this, _, _window, cx| {
                            this.last_error = None;
                            cx.notify();
                        },
                    ))),
            )
            .into_any()
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

    fn render_max_monthly_spend_reached_error(&self, cx: &mut Context<Self>) -> AnyElement {
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
                            cx.listener(|this, _, _window, cx| {
                                this.last_error = None;
                                cx.open_url(&zed_urls::account_url(cx));
                                cx.notify();
                            }),
                        ),
                    )
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
                    Icon::new(IconName::LightBulb)
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
                .child(Icon::new(IconName::LightBulb).size(IconSize::Small))
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
    icon: IconName,
    label: SharedString,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    Arc::new(move |fold_id, fold_range, _cx| {
        let editor = editor.clone();
        ButtonLike::new(fold_id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(icon))
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

fn render_docs_slash_command_trailer(
    row: MultiBufferRow,
    command: ParsedSlashCommand,
    cx: &mut App,
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
                    Tooltip::text(format!("Indexing {package}…"))
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
                .tooltip(Tooltip::text(format!("Failed to index: {latest_error}")))
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let provider = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.provider);
        let accept_terms = if self.show_accept_terms {
            provider.as_ref().and_then(|provider| {
                provider.render_accept_terms(LanguageModelProviderTosView::PromptEditorPopup, cx)
            })
        } else {
            None
        };

        let language_model_selector = self.language_model_selector_menu_handle.clone();
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
            .on_action(move |_: &ToggleModelSelector, window, cx| {
                language_model_selector.toggle(window, cx);
            })
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
                                .child(self.render_inject_context_menu(cx))
                                .child(ui::Divider::vertical())
                                .child(
                                    div()
                                        .pl_0p5()
                                        .child(self.render_language_model_selector(cx)),
                                ),
                        )
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
                                            .child(self.render_edit_button(window, cx))
                                            .child(
                                                Label::new("or")
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            )
                                    },
                                )
                                .child(self.render_send_button(window, cx)),
                        ),
                ),
            )
    }
}

impl Focusable for ContextEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for ContextEditor {
    type Event = editor::EditorEvent;

    fn tab_content_text(&self, _window: &Window, cx: &App) -> Option<SharedString> {
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

impl SearchableItem for ContextEditor {
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

impl FollowableItem for ContextEditor {
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
        let assistant_panel_delegate = <dyn AssistantPanelDelegate>::try_global(cx)?;

        let context_editor_task = workspace.update(cx, |workspace, cx| {
            assistant_panel_delegate.open_remote_context(workspace, context_id, window, cx)
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

    fn set_leader_peer_id(
        &mut self,
        leader_peer_id: Option<proto::PeerId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_leader_peer_id(leader_peer_id, window, cx)
        })
    }

    fn dedup(&self, existing: &Self, _window: &Window, cx: &App) -> Option<item::Dedup> {
        if existing.context.read(cx).id() == self.context.read(cx).id() {
            Some(item::Dedup::KeepExisting)
        } else {
            None
        }
    }
}

pub struct ContextEditorToolbarItem {
    active_context_editor: Option<WeakEntity<ContextEditor>>,
    model_summary_editor: Entity<Editor>,
}

impl ContextEditorToolbarItem {
    pub fn new(model_summary_editor: Entity<Editor>) -> Self {
        Self {
            active_context_editor: None,
            model_summary_editor,
        }
    }
}

pub fn render_remaining_tokens(
    context_editor: &Entity<ContextEditor>,
    cx: &App,
) -> Option<impl IntoElement + use<>> {
    let context = &context_editor.read(cx).context;

    let (token_count_color, token_count, max_token_count, tooltip) = match token_state(context, cx)?
    {
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

impl Render for ContextEditorToolbarItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                        .tooltip(Tooltip::text("Regenerate Title"))
                        .on_click(cx.listener(move |_, _, _window, cx| {
                            cx.emit(ContextEditorToolbarItemEvent::RegenerateSummary)
                        })),
                ),
            );

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
            .children(
                self.active_context_editor
                    .as_ref()
                    .and_then(|editor| editor.upgrade())
                    .and_then(|editor| render_remaining_tokens(&editor, cx)),
            );

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
        _window: &mut Window,
        cx: &mut Context<Self>,
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

    fn pane_focus_update(
        &mut self,
        _pane_focused: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

fn token_state(context: &Entity<AssistantContext>, cx: &App) -> Option<TokenState> {
    const WARNING_TOKEN_THRESHOLD: f32 = 0.8;

    let model = LanguageModelRegistry::read_global(cx)
        .default_model()?
        .model;
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

pub enum ConfigurationError {
    NoProvider,
    ProviderNotAuthenticated,
    ProviderPendingTermsAcceptance(Arc<dyn LanguageModelProvider>),
}

fn configuration_error(cx: &App) -> Option<ConfigurationError> {
    let model = LanguageModelRegistry::read_global(cx).default_model();
    let is_authenticated = model
        .as_ref()
        .map_or(false, |model| model.provider.is_authenticated(cx));

    if model.is_some() && is_authenticated {
        return None;
    }

    if model.is_none() {
        return Some(ConfigurationError::NoProvider);
    }

    if !is_authenticated {
        return Some(ConfigurationError::ProviderNotAuthenticated);
    }

    None
}

pub fn humanize_token_count(count: usize) -> String {
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
        let http_client = project.client().http_client().clone();
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
    use gpui::App;
    use language::Buffer;
    use unindent::Unindent;

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
}
