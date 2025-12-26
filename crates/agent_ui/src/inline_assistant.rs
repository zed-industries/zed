use language_model::AnthropicEventData;
use language_model::report_anthropic_event;
use std::cmp;
use std::mem;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::load_context;
use crate::mention_set::MentionSet;
use crate::{
    AgentPanel,
    buffer_codegen::{BufferCodegen, CodegenAlternative, CodegenEvent},
    inline_prompt_editor::{CodegenStatus, InlineAssistId, PromptEditor, PromptEditorEvent},
    terminal_inline_assistant::TerminalInlineAssistant,
};
use agent::HistoryStore;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result};
use collections::{HashMap, HashSet, VecDeque, hash_map};
use editor::EditorSnapshot;
use editor::MultiBufferOffset;
use editor::RowExt;
use editor::SelectionEffects;
use editor::scroll::ScrollOffset;
use editor::{
    Anchor, AnchorRangeExt, CodeActionProvider, Editor, EditorEvent, ExcerptId, ExcerptRange,
    MultiBuffer, MultiBufferSnapshot, ToOffset as _, ToPoint,
    actions::SelectAll,
    display_map::{
        BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId, EditorMargins,
        RenderBlock, ToDisplayPoint,
    },
};
use fs::Fs;
use futures::{FutureExt, channel::mpsc};
use gpui::{
    App, Context, Entity, Focusable, Global, HighlightStyle, Subscription, Task, UpdateGlobal,
    WeakEntity, Window, point,
};
use language::{Buffer, Point, Selection, TransactionId};
use language_model::{ConfigurationError, ConfiguredModel, LanguageModelRegistry};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use project::{CodeAction, DisableAiSettings, LspAction, Project, ProjectTransaction};
use prompt_store::{PromptBuilder, PromptStore};
use settings::{Settings, SettingsStore};

use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use text::{OffsetRangeExt, ToPoint as _};
use ui::prelude::*;
use util::{RangeExt, ResultExt, maybe};
use workspace::{ItemHandle, Toast, Workspace, dock::Panel, notifications::NotificationId};
use zed_actions::agent::OpenSettings;

pub fn init(fs: Arc<dyn Fs>, prompt_builder: Arc<PromptBuilder>, cx: &mut App) {
    cx.set_global(InlineAssistant::new(fs, prompt_builder));

    cx.observe_global::<SettingsStore>(|cx| {
        if DisableAiSettings::get_global(cx).disable_ai {
            // Hide any active inline assist UI when AI is disabled
            InlineAssistant::update_global(cx, |assistant, cx| {
                assistant.cancel_all_active_completions(cx);
            });
        }
    })
    .detach();

    cx.observe_new(|_workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let workspace = cx.entity();
        InlineAssistant::update_global(cx, |inline_assistant, cx| {
            inline_assistant.register_workspace(&workspace, window, cx)
        });
    })
    .detach();
}

const PROMPT_HISTORY_MAX_LEN: usize = 20;

enum InlineAssistTarget {
    Editor(Entity<Editor>),
    Terminal(Entity<TerminalView>),
}

pub struct InlineAssistant {
    next_assist_id: InlineAssistId,
    next_assist_group_id: InlineAssistGroupId,
    assists: HashMap<InlineAssistId, InlineAssist>,
    assists_by_editor: HashMap<WeakEntity<Editor>, EditorInlineAssists>,
    assist_groups: HashMap<InlineAssistGroupId, InlineAssistGroup>,
    confirmed_assists: HashMap<InlineAssistId, Entity<CodegenAlternative>>,
    prompt_history: VecDeque<String>,
    prompt_builder: Arc<PromptBuilder>,
    fs: Arc<dyn Fs>,
    _inline_assistant_completions: Option<mpsc::UnboundedSender<anyhow::Result<InlineAssistId>>>,
}

impl Global for InlineAssistant {}

impl InlineAssistant {
    pub fn new(fs: Arc<dyn Fs>, prompt_builder: Arc<PromptBuilder>) -> Self {
        Self {
            next_assist_id: InlineAssistId::default(),
            next_assist_group_id: InlineAssistGroupId::default(),
            assists: HashMap::default(),
            assists_by_editor: HashMap::default(),
            assist_groups: HashMap::default(),
            confirmed_assists: HashMap::default(),
            prompt_history: VecDeque::default(),
            prompt_builder,
            fs,
            _inline_assistant_completions: None,
        }
    }

    pub fn register_workspace(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        window
            .subscribe(workspace, cx, |workspace, event, window, cx| {
                Self::update_global(cx, |this, cx| {
                    this.handle_workspace_event(workspace, event, window, cx)
                });
            })
            .detach();

        let workspace = workspace.downgrade();
        cx.observe_global::<SettingsStore>(move |cx| {
            let Some(workspace) = workspace.upgrade() else {
                return;
            };
            let Some(terminal_panel) = workspace.read(cx).panel::<TerminalPanel>(cx) else {
                return;
            };
            let enabled = AgentSettings::get_global(cx).enabled(cx);
            terminal_panel.update(cx, |terminal_panel, cx| {
                terminal_panel.set_assistant_enabled(enabled, cx)
            });
        })
        .detach();
    }

    /// Hides all active inline assists when AI is disabled
    pub fn cancel_all_active_completions(&mut self, cx: &mut App) {
        // Cancel all active completions in editors
        for (editor_handle, _) in self.assists_by_editor.iter() {
            if let Some(editor) = editor_handle.upgrade() {
                let windows = cx.windows();
                if !windows.is_empty() {
                    let window = windows[0];
                    let _ = window.update(cx, |_, window, cx| {
                        editor.update(cx, |editor, cx| {
                            if editor.has_active_edit_prediction() {
                                editor.cancel(&Default::default(), window, cx);
                            }
                        });
                    });
                }
            }
        }
    }

    fn handle_workspace_event(
        &mut self,
        workspace: Entity<Workspace>,
        event: &workspace::Event,
        window: &mut Window,
        cx: &mut App,
    ) {
        match event {
            workspace::Event::UserSavedItem { item, .. } => {
                // When the user manually saves an editor, automatically accepts all finished transformations.
                if let Some(editor) = item.upgrade().and_then(|item| item.act_as::<Editor>(cx))
                    && let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade())
                {
                    for assist_id in editor_assists.assist_ids.clone() {
                        let assist = &self.assists[&assist_id];
                        if let CodegenStatus::Done = assist.codegen.read(cx).status(cx) {
                            self.finish_assist(assist_id, false, window, cx)
                        }
                    }
                }
            }
            workspace::Event::ItemAdded { item } => {
                self.register_workspace_item(&workspace, item.as_ref(), window, cx);
            }
            _ => (),
        }
    }

    fn register_workspace_item(
        &mut self,
        workspace: &Entity<Workspace>,
        item: &dyn ItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) {
        let is_ai_enabled = !DisableAiSettings::get_global(cx).disable_ai;

        if let Some(editor) = item.act_as::<Editor>(cx) {
            editor.update(cx, |editor, cx| {
                if is_ai_enabled {
                    editor.add_code_action_provider(
                        Rc::new(AssistantCodeActionProvider {
                            editor: cx.entity().downgrade(),
                            workspace: workspace.downgrade(),
                        }),
                        window,
                        cx,
                    );

                    if DisableAiSettings::get_global(cx).disable_ai {
                        // Cancel any active edit predictions
                        if editor.has_active_edit_prediction() {
                            editor.cancel(&Default::default(), window, cx);
                        }
                    }
                } else {
                    editor.remove_code_action_provider(
                        ASSISTANT_CODE_ACTION_PROVIDER_ID.into(),
                        window,
                        cx,
                    );
                }
            });
        }
    }

    pub fn inline_assist(
        workspace: &mut Workspace,
        action: &zed_actions::assistant::InlineAssist,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if !AgentSettings::get_global(cx).enabled(cx) {
            return;
        }

        let Some(inline_assist_target) = Self::resolve_inline_assist_target(
            workspace,
            workspace.panel::<AgentPanel>(cx),
            window,
            cx,
        ) else {
            return;
        };

        let configuration_error = || {
            let model_registry = LanguageModelRegistry::read_global(cx);
            model_registry.configuration_error(model_registry.inline_assistant_model(), cx)
        };

        let Some(agent_panel) = workspace.panel::<AgentPanel>(cx) else {
            return;
        };
        let agent_panel = agent_panel.read(cx);

        let prompt_store = agent_panel.prompt_store().as_ref().cloned();
        let thread_store = agent_panel.thread_store().clone();

        let handle_assist =
            |window: &mut Window, cx: &mut Context<Workspace>| match inline_assist_target {
                InlineAssistTarget::Editor(active_editor) => {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_editor,
                            cx.entity().downgrade(),
                            workspace.project().downgrade(),
                            thread_store,
                            prompt_store,
                            action.prompt.clone(),
                            window,
                            cx,
                        );
                    })
                }
                InlineAssistTarget::Terminal(active_terminal) => {
                    TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_terminal,
                            cx.entity().downgrade(),
                            workspace.project().downgrade(),
                            thread_store,
                            prompt_store,
                            action.prompt.clone(),
                            window,
                            cx,
                        );
                    });
                }
            };

        if let Some(error) = configuration_error() {
            if let ConfigurationError::ProviderNotAuthenticated(provider) = error {
                cx.spawn(async move |_, cx| {
                    cx.update(|cx| provider.authenticate(cx))?.await?;
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);

                if configuration_error().is_none() {
                    handle_assist(window, cx);
                }
            } else {
                cx.spawn_in(window, async move |_, cx| {
                    let answer = cx
                        .prompt(
                            gpui::PromptLevel::Warning,
                            &error.to_string(),
                            None,
                            &["Configure", "Cancel"],
                        )
                        .await
                        .ok();
                    if let Some(answer) = answer
                        && answer == 0
                    {
                        cx.update(|window, cx| window.dispatch_action(Box::new(OpenSettings), cx))
                            .ok();
                    }
                    anyhow::Ok(())
                })
                .detach_and_log_err(cx);
            }
        } else {
            handle_assist(window, cx);
        }
    }

    fn codegen_ranges(
        &mut self,
        editor: &Entity<Editor>,
        snapshot: &EditorSnapshot,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(Vec<Range<Anchor>>, Selection<Point>)> {
        let (initial_selections, newest_selection) = editor.update(cx, |editor, _| {
            (
                editor.selections.all::<Point>(&snapshot.display_snapshot),
                editor
                    .selections
                    .newest::<Point>(&snapshot.display_snapshot),
            )
        });

        // Check if there is already an inline assistant that contains the
        // newest selection, if there is, focus it
        if let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) {
            for assist_id in &editor_assists.assist_ids {
                let assist = &self.assists[assist_id];
                let range = assist.range.to_point(&snapshot.buffer_snapshot());
                if range.start.row <= newest_selection.start.row
                    && newest_selection.end.row <= range.end.row
                {
                    self.focus_assist(*assist_id, window, cx);
                    return None;
                }
            }
        }

        let mut selections = Vec::<Selection<Point>>::new();
        let mut newest_selection = None;
        for mut selection in initial_selections {
            if selection.end == selection.start
                && let Some(fold) =
                    snapshot.crease_for_buffer_row(MultiBufferRow(selection.end.row))
            {
                selection.start = fold.range().start;
                selection.end = fold.range().end;
                if MultiBufferRow(selection.end.row) < snapshot.buffer_snapshot().max_row() {
                    let chars = snapshot
                        .buffer_snapshot()
                        .chars_at(Point::new(selection.end.row + 1, 0));

                    for c in chars {
                        if c == '\n' {
                            break;
                        }
                        if c.is_whitespace() {
                            continue;
                        }
                        if snapshot
                            .language_at(selection.end)
                            .is_some_and(|language| language.config().brackets.is_closing_brace(c))
                        {
                            selection.end.row += 1;
                            selection.end.column = snapshot
                                .buffer_snapshot()
                                .line_len(MultiBufferRow(selection.end.row));
                        }
                    }
                }
            } else {
                selection.start.column = 0;
                // If the selection ends at the start of the line, we don't want to include it.
                if selection.end.column == 0 && selection.start.row != selection.end.row {
                    selection.end.row -= 1;
                }
                selection.end.column = snapshot
                    .buffer_snapshot()
                    .line_len(MultiBufferRow(selection.end.row));
            }

            if let Some(prev_selection) = selections.last_mut()
                && selection.start <= prev_selection.end
            {
                prev_selection.end = selection.end;
                continue;
            }

            let latest_selection = newest_selection.get_or_insert_with(|| selection.clone());
            if selection.id > latest_selection.id {
                *latest_selection = selection.clone();
            }
            selections.push(selection);
        }
        let snapshot = &snapshot.buffer_snapshot();
        let newest_selection = newest_selection.unwrap();

        let mut codegen_ranges = Vec::new();
        for (buffer, buffer_range, excerpt_id) in
            snapshot.ranges_to_buffer_ranges(selections.iter().map(|selection| {
                snapshot.anchor_before(selection.start)..snapshot.anchor_after(selection.end)
            }))
        {
            let anchor_range = Anchor::range_in_buffer(
                excerpt_id,
                buffer.anchor_before(buffer_range.start)..buffer.anchor_after(buffer_range.end),
            );

            codegen_ranges.push(anchor_range);

            if let Some(model) = LanguageModelRegistry::read_global(cx).inline_assistant_model() {
                telemetry::event!(
                    "Assistant Invoked",
                    kind = "inline",
                    phase = "invoked",
                    model = model.model.telemetry_id(),
                    model_provider = model.provider.id().to_string(),
                    language_name = buffer.language().map(|language| language.name().to_proto())
                );

                report_anthropic_event(
                    &model.model,
                    AnthropicEventData {
                        completion_type: language_model::AnthropicCompletionType::Editor,
                        event: language_model::AnthropicEventType::Invoked,
                        language_name: buffer.language().map(|language| language.name().to_proto()),
                        message_id: None,
                    },
                    cx,
                );
            }
        }

        Some((codegen_ranges, newest_selection))
    }

    fn batch_assist(
        &mut self,
        editor: &Entity<Editor>,
        workspace: WeakEntity<Workspace>,
        project: WeakEntity<Project>,
        thread_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        initial_prompt: Option<String>,
        window: &mut Window,
        codegen_ranges: &[Range<Anchor>],
        newest_selection: Option<Selection<Point>>,
        initial_transaction_id: Option<TransactionId>,
        cx: &mut App,
    ) -> Option<InlineAssistId> {
        let snapshot = editor.update(cx, |editor, cx| editor.snapshot(window, cx));

        let assist_group_id = self.next_assist_group_id.post_inc();
        let session_id = Uuid::new_v4();
        let prompt_buffer = cx.new(|cx| {
            MultiBuffer::singleton(
                cx.new(|cx| Buffer::local(initial_prompt.unwrap_or_default(), cx)),
                cx,
            )
        });

        let mut assists = Vec::new();
        let mut assist_to_focus = None;

        for range in codegen_ranges {
            let assist_id = self.next_assist_id.post_inc();
            let codegen = cx.new(|cx| {
                BufferCodegen::new(
                    editor.read(cx).buffer().clone(),
                    range.clone(),
                    initial_transaction_id,
                    session_id,
                    self.prompt_builder.clone(),
                    cx,
                )
            });

            let editor_margins = Arc::new(Mutex::new(EditorMargins::default()));
            let prompt_editor = cx.new(|cx| {
                PromptEditor::new_buffer(
                    assist_id,
                    editor_margins,
                    self.prompt_history.clone(),
                    prompt_buffer.clone(),
                    codegen.clone(),
                    session_id,
                    self.fs.clone(),
                    thread_store.clone(),
                    prompt_store.clone(),
                    project.clone(),
                    workspace.clone(),
                    window,
                    cx,
                )
            });

            if let Some(newest_selection) = newest_selection.as_ref()
                && assist_to_focus.is_none()
            {
                let focus_assist = if newest_selection.reversed {
                    range.start.to_point(&snapshot) == newest_selection.start
                } else {
                    range.end.to_point(&snapshot) == newest_selection.end
                };
                if focus_assist {
                    assist_to_focus = Some(assist_id);
                }
            }

            let [prompt_block_id, tool_description_block_id, end_block_id] =
                self.insert_assist_blocks(&editor, &range, &prompt_editor, cx);

            assists.push((
                assist_id,
                range.clone(),
                prompt_editor,
                prompt_block_id,
                tool_description_block_id,
                end_block_id,
            ));
        }

        let editor_assists = self
            .assists_by_editor
            .entry(editor.downgrade())
            .or_insert_with(|| EditorInlineAssists::new(editor, window, cx));

        let assist_to_focus = if let Some(focus_id) = assist_to_focus {
            Some(focus_id)
        } else if assists.len() >= 1 {
            Some(assists[0].0)
        } else {
            None
        };

        let mut assist_group = InlineAssistGroup::new();
        for (
            assist_id,
            range,
            prompt_editor,
            prompt_block_id,
            tool_description_block_id,
            end_block_id,
        ) in assists
        {
            let codegen = prompt_editor.read(cx).codegen().clone();

            self.assists.insert(
                assist_id,
                InlineAssist::new(
                    assist_id,
                    assist_group_id,
                    editor,
                    &prompt_editor,
                    prompt_block_id,
                    tool_description_block_id,
                    end_block_id,
                    range,
                    codegen,
                    workspace.clone(),
                    window,
                    cx,
                ),
            );
            assist_group.assist_ids.push(assist_id);
            editor_assists.assist_ids.push(assist_id);
        }

        self.assist_groups.insert(assist_group_id, assist_group);

        assist_to_focus
    }

    pub fn assist(
        &mut self,
        editor: &Entity<Editor>,
        workspace: WeakEntity<Workspace>,
        project: WeakEntity<Project>,
        thread_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InlineAssistId> {
        let snapshot = editor.update(cx, |editor, cx| editor.snapshot(window, cx));

        let Some((codegen_ranges, newest_selection)) =
            self.codegen_ranges(editor, &snapshot, window, cx)
        else {
            return None;
        };

        let assist_to_focus = self.batch_assist(
            editor,
            workspace,
            project,
            thread_store,
            prompt_store,
            initial_prompt,
            window,
            &codegen_ranges,
            Some(newest_selection),
            None,
            cx,
        );

        if let Some(assist_id) = assist_to_focus {
            self.focus_assist(assist_id, window, cx);
        }

        assist_to_focus
    }

    pub fn suggest_assist(
        &mut self,
        editor: &Entity<Editor>,
        mut range: Range<Anchor>,
        initial_prompt: String,
        initial_transaction_id: Option<TransactionId>,
        focus: bool,
        workspace: Entity<Workspace>,
        thread_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
        cx: &mut App,
    ) -> InlineAssistId {
        let buffer = editor.read(cx).buffer().clone();
        {
            let snapshot = buffer.read(cx).read(cx);
            range.start = range.start.bias_left(&snapshot);
            range.end = range.end.bias_right(&snapshot);
        }

        let project = workspace.read(cx).project().downgrade();

        let assist_id = self
            .batch_assist(
                editor,
                workspace.downgrade(),
                project,
                thread_store,
                prompt_store,
                Some(initial_prompt),
                window,
                &[range],
                None,
                initial_transaction_id,
                cx,
            )
            .expect("batch_assist returns an id if there's only one range");

        if focus {
            self.focus_assist(assist_id, window, cx);
        }

        assist_id
    }

    fn insert_assist_blocks(
        &self,
        editor: &Entity<Editor>,
        range: &Range<Anchor>,
        prompt_editor: &Entity<PromptEditor<BufferCodegen>>,
        cx: &mut App,
    ) -> [CustomBlockId; 3] {
        let prompt_editor_height = prompt_editor.update(cx, |prompt_editor, cx| {
            prompt_editor
                .editor
                .update(cx, |editor, cx| editor.max_point(cx).row().0 + 1 + 2)
        });
        let assist_blocks = vec![
            BlockProperties {
                style: BlockStyle::Sticky,
                placement: BlockPlacement::Above(range.start),
                height: Some(prompt_editor_height),
                render: build_assist_editor_renderer(prompt_editor),
                priority: 0,
            },
            // Placeholder for tool description - will be updated dynamically
            BlockProperties {
                style: BlockStyle::Flex,
                placement: BlockPlacement::Below(range.end),
                height: Some(0),
                render: Arc::new(|_cx| div().into_any_element()),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Sticky,
                placement: BlockPlacement::Below(range.end),
                height: None,
                render: Arc::new(|cx| {
                    v_flex()
                        .h_full()
                        .w_full()
                        .border_t_1()
                        .border_color(cx.theme().status().info_border)
                        .into_any_element()
                }),
                priority: 0,
            },
        ];

        editor.update(cx, |editor, cx| {
            let block_ids = editor.insert_blocks(assist_blocks, None, cx);
            [block_ids[0], block_ids[1], block_ids[2]]
        })
    }

    fn handle_prompt_editor_focus_in(&mut self, assist_id: InlineAssistId, cx: &mut App) {
        let assist = &self.assists[&assist_id];
        let Some(decorations) = assist.decorations.as_ref() else {
            return;
        };
        let assist_group = self.assist_groups.get_mut(&assist.group_id).unwrap();
        let editor_assists = self.assists_by_editor.get_mut(&assist.editor).unwrap();

        assist_group.active_assist_id = Some(assist_id);
        if assist_group.linked {
            for assist_id in &assist_group.assist_ids {
                if let Some(decorations) = self.assists[assist_id].decorations.as_ref() {
                    decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                        prompt_editor.set_show_cursor_when_unfocused(true, cx)
                    });
                }
            }
        }

        assist
            .editor
            .update(cx, |editor, cx| {
                let scroll_top = editor.scroll_position(cx).y;
                let scroll_bottom = scroll_top + editor.visible_line_count().unwrap_or(0.);
                editor_assists.scroll_lock = editor
                    .row_for_block(decorations.prompt_block_id, cx)
                    .map(|row| row.as_f64())
                    .filter(|prompt_row| (scroll_top..scroll_bottom).contains(&prompt_row))
                    .map(|prompt_row| InlineAssistScrollLock {
                        assist_id,
                        distance_from_top: prompt_row - scroll_top,
                    });
            })
            .ok();
    }

    fn handle_prompt_editor_focus_out(&mut self, assist_id: InlineAssistId, cx: &mut App) {
        let assist = &self.assists[&assist_id];
        let assist_group = self.assist_groups.get_mut(&assist.group_id).unwrap();
        if assist_group.active_assist_id == Some(assist_id) {
            assist_group.active_assist_id = None;
            if assist_group.linked {
                for assist_id in &assist_group.assist_ids {
                    if let Some(decorations) = self.assists[assist_id].decorations.as_ref() {
                        decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                            prompt_editor.set_show_cursor_when_unfocused(false, cx)
                        });
                    }
                }
            }
        }
    }

    fn handle_prompt_editor_event(
        &mut self,
        prompt_editor: Entity<PromptEditor<BufferCodegen>>,
        event: &PromptEditorEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        let assist_id = prompt_editor.read(cx).id();
        match event {
            PromptEditorEvent::StartRequested => {
                self.start_assist(assist_id, window, cx);
            }
            PromptEditorEvent::StopRequested => {
                self.stop_assist(assist_id, cx);
            }
            PromptEditorEvent::ConfirmRequested { execute: _ } => {
                self.finish_assist(assist_id, false, window, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, window, cx);
            }
            PromptEditorEvent::Resized { .. } => {
                // This only matters for the terminal inline assistant
            }
        }
    }

    fn handle_editor_newline(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut App) {
        let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) else {
            return;
        };

        if editor.read(cx).selections.count() == 1 {
            let (selection, buffer) = editor.update(cx, |editor, cx| {
                (
                    editor
                        .selections
                        .newest::<MultiBufferOffset>(&editor.display_snapshot(cx)),
                    editor.buffer().read(cx).snapshot(cx),
                )
            });
            for assist_id in &editor_assists.assist_ids {
                let assist = &self.assists[assist_id];
                let assist_range = assist.range.to_offset(&buffer);
                if assist_range.contains(&selection.start) && assist_range.contains(&selection.end)
                {
                    if matches!(assist.codegen.read(cx).status(cx), CodegenStatus::Pending) {
                        self.dismiss_assist(*assist_id, window, cx);
                    } else {
                        self.finish_assist(*assist_id, false, window, cx);
                    }

                    return;
                }
            }
        }

        cx.propagate();
    }

    fn handle_editor_cancel(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut App) {
        let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) else {
            return;
        };

        if editor.read(cx).selections.count() == 1 {
            let (selection, buffer) = editor.update(cx, |editor, cx| {
                (
                    editor
                        .selections
                        .newest::<MultiBufferOffset>(&editor.display_snapshot(cx)),
                    editor.buffer().read(cx).snapshot(cx),
                )
            });
            let mut closest_assist_fallback = None;
            for assist_id in &editor_assists.assist_ids {
                let assist = &self.assists[assist_id];
                let assist_range = assist.range.to_offset(&buffer);
                if assist.decorations.is_some() {
                    if assist_range.contains(&selection.start)
                        && assist_range.contains(&selection.end)
                    {
                        self.focus_assist(*assist_id, window, cx);
                        return;
                    } else {
                        let distance_from_selection = assist_range
                            .start
                            .0
                            .abs_diff(selection.start.0)
                            .min(assist_range.start.0.abs_diff(selection.end.0))
                            + assist_range
                                .end
                                .0
                                .abs_diff(selection.start.0)
                                .min(assist_range.end.0.abs_diff(selection.end.0));
                        match closest_assist_fallback {
                            Some((_, old_distance)) => {
                                if distance_from_selection < old_distance {
                                    closest_assist_fallback =
                                        Some((assist_id, distance_from_selection));
                                }
                            }
                            None => {
                                closest_assist_fallback = Some((assist_id, distance_from_selection))
                            }
                        }
                    }
                }
            }

            if let Some((&assist_id, _)) = closest_assist_fallback {
                self.focus_assist(assist_id, window, cx);
            }
        }

        cx.propagate();
    }

    fn handle_editor_release(
        &mut self,
        editor: WeakEntity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(editor_assists) = self.assists_by_editor.get_mut(&editor) {
            for assist_id in editor_assists.assist_ids.clone() {
                self.finish_assist(assist_id, true, window, cx);
            }
        }
    }

    fn handle_editor_change(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut App) {
        let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) else {
            return;
        };
        let Some(scroll_lock) = editor_assists.scroll_lock.as_ref() else {
            return;
        };
        let assist = &self.assists[&scroll_lock.assist_id];
        let Some(decorations) = assist.decorations.as_ref() else {
            return;
        };

        editor.update(cx, |editor, cx| {
            let scroll_position = editor.scroll_position(cx);
            let target_scroll_top = editor
                .row_for_block(decorations.prompt_block_id, cx)?
                .as_f64()
                - scroll_lock.distance_from_top;
            if target_scroll_top != scroll_position.y {
                editor.set_scroll_position(point(scroll_position.x, target_scroll_top), window, cx);
            }
            Some(())
        });
    }

    fn handle_editor_event(
        &mut self,
        editor: Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(editor_assists) = self.assists_by_editor.get_mut(&editor.downgrade()) else {
            return;
        };

        match event {
            EditorEvent::Edited { transaction_id } => {
                let buffer = editor.read(cx).buffer().read(cx);
                let edited_ranges =
                    buffer.edited_ranges_for_transaction::<MultiBufferOffset>(*transaction_id, cx);
                let snapshot = buffer.snapshot(cx);

                for assist_id in editor_assists.assist_ids.clone() {
                    let assist = &self.assists[&assist_id];
                    if matches!(
                        assist.codegen.read(cx).status(cx),
                        CodegenStatus::Error(_) | CodegenStatus::Done
                    ) {
                        let assist_range = assist.range.to_offset(&snapshot);
                        if edited_ranges
                            .iter()
                            .any(|range| range.overlaps(&assist_range))
                        {
                            self.finish_assist(assist_id, false, window, cx);
                        }
                    }
                }
            }
            EditorEvent::ScrollPositionChanged { .. } => {
                if let Some(scroll_lock) = editor_assists.scroll_lock.as_ref() {
                    let assist = &self.assists[&scroll_lock.assist_id];
                    if let Some(decorations) = assist.decorations.as_ref() {
                        let distance_from_top = editor.update(cx, |editor, cx| {
                            let scroll_top = editor.scroll_position(cx).y;
                            let prompt_row = editor
                                .row_for_block(decorations.prompt_block_id, cx)?
                                .0 as ScrollOffset;
                            Some(prompt_row - scroll_top)
                        });

                        if distance_from_top.is_none_or(|distance_from_top| {
                            distance_from_top != scroll_lock.distance_from_top
                        }) {
                            editor_assists.scroll_lock = None;
                        }
                    }
                }
            }
            EditorEvent::SelectionsChanged { .. } => {
                for assist_id in editor_assists.assist_ids.clone() {
                    let assist = &self.assists[&assist_id];
                    if let Some(decorations) = assist.decorations.as_ref()
                        && decorations
                            .prompt_editor
                            .focus_handle(cx)
                            .is_focused(window)
                    {
                        return;
                    }
                }

                editor_assists.scroll_lock = None;
            }
            _ => {}
        }
    }

    pub fn finish_assist(
        &mut self,
        assist_id: InlineAssistId,
        undo: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(assist) = self.assists.get(&assist_id) {
            let assist_group_id = assist.group_id;
            if self.assist_groups[&assist_group_id].linked {
                for assist_id in self.unlink_assist_group(assist_group_id, window, cx) {
                    self.finish_assist(assist_id, undo, window, cx);
                }
                return;
            }
        }

        self.dismiss_assist(assist_id, window, cx);

        if let Some(assist) = self.assists.remove(&assist_id) {
            if let hash_map::Entry::Occupied(mut entry) = self.assist_groups.entry(assist.group_id)
            {
                entry.get_mut().assist_ids.retain(|id| *id != assist_id);
                if entry.get().assist_ids.is_empty() {
                    entry.remove();
                }
            }

            if let hash_map::Entry::Occupied(mut entry) =
                self.assists_by_editor.entry(assist.editor.clone())
            {
                entry.get_mut().assist_ids.retain(|id| *id != assist_id);
                if entry.get().assist_ids.is_empty() {
                    entry.remove();
                    if let Some(editor) = assist.editor.upgrade() {
                        self.update_editor_highlights(&editor, cx);
                    }
                } else {
                    entry.get_mut().highlight_updates.send(()).ok();
                }
            }

            let active_alternative = assist.codegen.read(cx).active_alternative().clone();
            if let Some(model) = LanguageModelRegistry::read_global(cx).inline_assistant_model() {
                let language_name = assist.editor.upgrade().and_then(|editor| {
                    let multibuffer = editor.read(cx).buffer().read(cx);
                    let snapshot = multibuffer.snapshot(cx);
                    let ranges = snapshot.range_to_buffer_ranges(assist.range.clone());
                    ranges
                        .first()
                        .and_then(|(buffer, _, _)| buffer.language())
                        .map(|language| language.name().0.to_string())
                });

                let codegen = assist.codegen.read(cx);
                let session_id = codegen.session_id();
                let message_id = active_alternative.read(cx).message_id.clone();
                let model_telemetry_id = model.model.telemetry_id();
                let model_provider_id = model.model.provider_id().to_string();

                let (phase, event_type, anthropic_event_type) = if undo {
                    (
                        "rejected",
                        "Assistant Response Rejected",
                        language_model::AnthropicEventType::Reject,
                    )
                } else {
                    (
                        "accepted",
                        "Assistant Response Accepted",
                        language_model::AnthropicEventType::Accept,
                    )
                };

                telemetry::event!(
                    event_type,
                    phase,
                    session_id = session_id.to_string(),
                    kind = "inline",
                    model = model_telemetry_id,
                    model_provider = model_provider_id,
                    language_name = language_name,
                    message_id = message_id.as_deref(),
                );

                report_anthropic_event(
                    &model.model,
                    language_model::AnthropicEventData {
                        completion_type: language_model::AnthropicCompletionType::Editor,
                        event: anthropic_event_type,
                        language_name,
                        message_id,
                    },
                    cx,
                );
            }

            if undo {
                assist.codegen.update(cx, |codegen, cx| codegen.undo(cx));
            } else {
                self.confirmed_assists.insert(assist_id, active_alternative);
            }
        }
    }

    fn dismiss_assist(
        &mut self,
        assist_id: InlineAssistId,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(assist) = self.assists.get_mut(&assist_id) else {
            return false;
        };
        let Some(editor) = assist.editor.upgrade() else {
            return false;
        };
        let Some(decorations) = assist.decorations.take() else {
            return false;
        };

        editor.update(cx, |editor, cx| {
            let mut to_remove = decorations.removed_line_block_ids;
            to_remove.insert(decorations.prompt_block_id);
            to_remove.insert(decorations.end_block_id);
            if let Some(tool_description_block_id) = decorations.model_explanation {
                to_remove.insert(tool_description_block_id);
            }
            editor.remove_blocks(to_remove, None, cx);
        });

        if decorations
            .prompt_editor
            .focus_handle(cx)
            .contains_focused(window, cx)
        {
            self.focus_next_assist(assist_id, window, cx);
        }

        if let Some(editor_assists) = self.assists_by_editor.get_mut(&editor.downgrade()) {
            if editor_assists
                .scroll_lock
                .as_ref()
                .is_some_and(|lock| lock.assist_id == assist_id)
            {
                editor_assists.scroll_lock = None;
            }
            editor_assists.highlight_updates.send(()).ok();
        }

        true
    }

    fn focus_next_assist(&mut self, assist_id: InlineAssistId, window: &mut Window, cx: &mut App) {
        let Some(assist) = self.assists.get(&assist_id) else {
            return;
        };

        let assist_group = &self.assist_groups[&assist.group_id];
        let assist_ix = assist_group
            .assist_ids
            .iter()
            .position(|id| *id == assist_id)
            .unwrap();
        let assist_ids = assist_group
            .assist_ids
            .iter()
            .skip(assist_ix + 1)
            .chain(assist_group.assist_ids.iter().take(assist_ix));

        for assist_id in assist_ids {
            let assist = &self.assists[assist_id];
            if assist.decorations.is_some() {
                self.focus_assist(*assist_id, window, cx);
                return;
            }
        }

        assist
            .editor
            .update(cx, |editor, cx| window.focus(&editor.focus_handle(cx), cx))
            .ok();
    }

    fn focus_assist(&mut self, assist_id: InlineAssistId, window: &mut Window, cx: &mut App) {
        let Some(assist) = self.assists.get(&assist_id) else {
            return;
        };

        if let Some(decorations) = assist.decorations.as_ref() {
            decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                prompt_editor.editor.update(cx, |editor, cx| {
                    window.focus(&editor.focus_handle(cx), cx);
                    editor.select_all(&SelectAll, window, cx);
                })
            });
        }

        self.scroll_to_assist(assist_id, window, cx);
    }

    pub fn scroll_to_assist(
        &mut self,
        assist_id: InlineAssistId,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(assist) = self.assists.get(&assist_id) else {
            return;
        };
        let Some(editor) = assist.editor.upgrade() else {
            return;
        };

        let position = assist.range.start;
        editor.update(cx, |editor, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_anchor_ranges([position..position])
            });

            let mut scroll_target_range = None;
            if let Some(decorations) = assist.decorations.as_ref() {
                scroll_target_range = maybe!({
                    let top = editor.row_for_block(decorations.prompt_block_id, cx)?.0 as f64;
                    let bottom = editor.row_for_block(decorations.end_block_id, cx)?.0 as f64;
                    Some((top, bottom))
                });
                if scroll_target_range.is_none() {
                    log::error!("bug: failed to find blocks for scrolling to inline assist");
                }
            }
            let scroll_target_range = scroll_target_range.unwrap_or_else(|| {
                let snapshot = editor.snapshot(window, cx);
                let start_row = assist
                    .range
                    .start
                    .to_display_point(&snapshot.display_snapshot)
                    .row();
                let top = start_row.0 as ScrollOffset;
                let bottom = top + 1.0;
                (top, bottom)
            });
            let height_in_lines = editor.visible_line_count().unwrap_or(0.);
            let vertical_scroll_margin = editor.vertical_scroll_margin() as ScrollOffset;
            let scroll_target_top = (scroll_target_range.0 - vertical_scroll_margin)
                // Don't scroll up too far in the case of a large vertical_scroll_margin.
                .max(scroll_target_range.0 - height_in_lines / 2.0);
            let scroll_target_bottom = (scroll_target_range.1 + vertical_scroll_margin)
                // Don't scroll down past where the top would still be visible.
                .min(scroll_target_top + height_in_lines);

            let scroll_top = editor.scroll_position(cx).y;
            let scroll_bottom = scroll_top + height_in_lines;

            if scroll_target_top < scroll_top {
                editor.set_scroll_position(point(0., scroll_target_top), window, cx);
            } else if scroll_target_bottom > scroll_bottom {
                editor.set_scroll_position(
                    point(0., scroll_target_bottom - height_in_lines),
                    window,
                    cx,
                );
            }
        });
    }

    fn unlink_assist_group(
        &mut self,
        assist_group_id: InlineAssistGroupId,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<InlineAssistId> {
        let assist_group = self.assist_groups.get_mut(&assist_group_id).unwrap();
        assist_group.linked = false;

        for assist_id in &assist_group.assist_ids {
            let assist = self.assists.get_mut(assist_id).unwrap();
            if let Some(editor_decorations) = assist.decorations.as_ref() {
                editor_decorations
                    .prompt_editor
                    .update(cx, |prompt_editor, cx| prompt_editor.unlink(window, cx));
            }
        }
        assist_group.assist_ids.clone()
    }

    pub fn start_assist(&mut self, assist_id: InlineAssistId, window: &mut Window, cx: &mut App) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        let assist_group_id = assist.group_id;
        if self.assist_groups[&assist_group_id].linked {
            for assist_id in self.unlink_assist_group(assist_group_id, window, cx) {
                self.start_assist(assist_id, window, cx);
            }
            return;
        }

        let Some((user_prompt, mention_set)) = assist.user_prompt(cx).zip(assist.mention_set(cx))
        else {
            return;
        };

        self.prompt_history.retain(|prompt| *prompt != user_prompt);
        self.prompt_history.push_back(user_prompt.clone());
        if self.prompt_history.len() > PROMPT_HISTORY_MAX_LEN {
            self.prompt_history.pop_front();
        }

        let Some(ConfiguredModel { model, .. }) =
            LanguageModelRegistry::read_global(cx).inline_assistant_model()
        else {
            return;
        };

        let context_task = load_context(&mention_set, cx).shared();
        assist
            .codegen
            .update(cx, |codegen, cx| {
                codegen.start(model, user_prompt, context_task, cx)
            })
            .log_err();
    }

    pub fn stop_assist(&mut self, assist_id: InlineAssistId, cx: &mut App) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        assist.codegen.update(cx, |codegen, cx| codegen.stop(cx));
    }

    fn update_editor_highlights(&self, editor: &Entity<Editor>, cx: &mut App) {
        let mut gutter_pending_ranges = Vec::new();
        let mut gutter_transformed_ranges = Vec::new();
        let mut foreground_ranges = Vec::new();
        let mut inserted_row_ranges = Vec::new();
        let empty_assist_ids = Vec::new();
        let assist_ids = self
            .assists_by_editor
            .get(&editor.downgrade())
            .map_or(&empty_assist_ids, |editor_assists| {
                &editor_assists.assist_ids
            });

        for assist_id in assist_ids {
            if let Some(assist) = self.assists.get(assist_id) {
                let codegen = assist.codegen.read(cx);
                let buffer = codegen.buffer(cx).read(cx).read(cx);
                foreground_ranges.extend(codegen.last_equal_ranges(cx).iter().cloned());

                let pending_range =
                    codegen.edit_position(cx).unwrap_or(assist.range.start)..assist.range.end;
                if pending_range.end.to_offset(&buffer) > pending_range.start.to_offset(&buffer) {
                    gutter_pending_ranges.push(pending_range);
                }

                if let Some(edit_position) = codegen.edit_position(cx) {
                    let edited_range = assist.range.start..edit_position;
                    if edited_range.end.to_offset(&buffer) > edited_range.start.to_offset(&buffer) {
                        gutter_transformed_ranges.push(edited_range);
                    }
                }

                if assist.decorations.is_some() {
                    inserted_row_ranges
                        .extend(codegen.diff(cx).inserted_row_ranges.iter().cloned());
                }
            }
        }

        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        merge_ranges(&mut foreground_ranges, &snapshot);
        merge_ranges(&mut gutter_pending_ranges, &snapshot);
        merge_ranges(&mut gutter_transformed_ranges, &snapshot);
        editor.update(cx, |editor, cx| {
            enum GutterPendingRange {}
            if gutter_pending_ranges.is_empty() {
                editor.clear_gutter_highlights::<GutterPendingRange>(cx);
            } else {
                editor.highlight_gutter::<GutterPendingRange>(
                    gutter_pending_ranges,
                    |cx| cx.theme().status().info_background,
                    cx,
                )
            }

            enum GutterTransformedRange {}
            if gutter_transformed_ranges.is_empty() {
                editor.clear_gutter_highlights::<GutterTransformedRange>(cx);
            } else {
                editor.highlight_gutter::<GutterTransformedRange>(
                    gutter_transformed_ranges,
                    |cx| cx.theme().status().info,
                    cx,
                )
            }

            if foreground_ranges.is_empty() {
                editor.clear_highlights::<InlineAssist>(cx);
            } else {
                editor.highlight_text::<InlineAssist>(
                    foreground_ranges,
                    HighlightStyle {
                        fade_out: Some(0.6),
                        ..Default::default()
                    },
                    cx,
                );
            }

            editor.clear_row_highlights::<InlineAssist>();
            for row_range in inserted_row_ranges {
                editor.highlight_rows::<InlineAssist>(
                    row_range,
                    cx.theme().status().info_background,
                    Default::default(),
                    cx,
                );
            }
        });
    }

    fn update_editor_blocks(
        &mut self,
        editor: &Entity<Editor>,
        assist_id: InlineAssistId,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(assist) = self.assists.get_mut(&assist_id) else {
            return;
        };
        let Some(decorations) = assist.decorations.as_mut() else {
            return;
        };

        let codegen = assist.codegen.read(cx);
        let old_snapshot = codegen.snapshot(cx);
        let old_buffer = codegen.old_buffer(cx);
        let deleted_row_ranges = codegen.diff(cx).deleted_row_ranges.clone();

        editor.update(cx, |editor, cx| {
            let old_blocks = mem::take(&mut decorations.removed_line_block_ids);
            editor.remove_blocks(old_blocks, None, cx);

            let mut new_blocks = Vec::new();
            for (new_row, old_row_range) in deleted_row_ranges {
                let (_, buffer_start) = old_snapshot
                    .point_to_buffer_offset(Point::new(*old_row_range.start(), 0))
                    .unwrap();
                let (_, buffer_end) = old_snapshot
                    .point_to_buffer_offset(Point::new(
                        *old_row_range.end(),
                        old_snapshot.line_len(MultiBufferRow(*old_row_range.end())),
                    ))
                    .unwrap();

                let deleted_lines_editor = cx.new(|cx| {
                    let multi_buffer =
                        cx.new(|_| MultiBuffer::without_headers(language::Capability::ReadOnly));
                    multi_buffer.update(cx, |multi_buffer, cx| {
                        multi_buffer.push_excerpts(
                            old_buffer.clone(),
                            // todo(lw): buffer_start and buffer_end might come from different snapshots!
                            Some(ExcerptRange::new(buffer_start..buffer_end)),
                            cx,
                        );
                    });

                    enum DeletedLines {}
                    let mut editor = Editor::for_multibuffer(multi_buffer, None, window, cx);
                    editor.disable_scrollbars_and_minimap(window, cx);
                    editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
                    editor.set_show_wrap_guides(false, cx);
                    editor.set_show_gutter(false, cx);
                    editor.set_offset_content(false, cx);
                    editor.scroll_manager.set_forbid_vertical_scroll(true);
                    editor.set_read_only(true);
                    editor.set_show_edit_predictions(Some(false), window, cx);
                    editor.highlight_rows::<DeletedLines>(
                        Anchor::min()..Anchor::max(),
                        cx.theme().status().deleted_background,
                        Default::default(),
                        cx,
                    );
                    editor
                });

                let height =
                    deleted_lines_editor.update(cx, |editor, cx| editor.max_point(cx).row().0 + 1);
                new_blocks.push(BlockProperties {
                    placement: BlockPlacement::Above(new_row),
                    height: Some(height),
                    style: BlockStyle::Flex,
                    render: Arc::new(move |cx| {
                        div()
                            .block_mouse_except_scroll()
                            .bg(cx.theme().status().deleted_background)
                            .size_full()
                            .h(height as f32 * cx.window.line_height())
                            .pl(cx.margins.gutter.full_width())
                            .child(deleted_lines_editor.clone())
                            .into_any_element()
                    }),
                    priority: 0,
                });
            }

            decorations.removed_line_block_ids = editor
                .insert_blocks(new_blocks, None, cx)
                .into_iter()
                .collect();
        })
    }

    fn resolve_inline_assist_target(
        workspace: &mut Workspace,
        agent_panel: Option<Entity<AgentPanel>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InlineAssistTarget> {
        if let Some(terminal_panel) = workspace.panel::<TerminalPanel>(cx)
            && terminal_panel
                .read(cx)
                .focus_handle(cx)
                .contains_focused(window, cx)
            && let Some(terminal_view) = terminal_panel.read(cx).pane().and_then(|pane| {
                pane.read(cx)
                    .active_item()
                    .and_then(|t| t.downcast::<TerminalView>())
            })
        {
            return Some(InlineAssistTarget::Terminal(terminal_view));
        }

        let text_thread_editor = agent_panel
            .and_then(|panel| panel.read(cx).active_text_thread_editor())
            .and_then(|editor| {
                let editor = &editor.read(cx).editor().clone();
                if editor.read(cx).is_focused(window) {
                    Some(editor.clone())
                } else {
                    None
                }
            });

        if let Some(text_thread_editor) = text_thread_editor {
            Some(InlineAssistTarget::Editor(text_thread_editor))
        } else if let Some(workspace_editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        {
            Some(InlineAssistTarget::Editor(workspace_editor))
        } else {
            workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<TerminalView>(cx))
                .map(InlineAssistTarget::Terminal)
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_completion_receiver(
        &mut self,
        sender: mpsc::UnboundedSender<anyhow::Result<InlineAssistId>>,
    ) {
        self._inline_assistant_completions = Some(sender);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn get_codegen(
        &mut self,
        assist_id: InlineAssistId,
        cx: &mut App,
    ) -> Option<Entity<CodegenAlternative>> {
        self.assists.get(&assist_id).map(|inline_assist| {
            inline_assist
                .codegen
                .update(cx, |codegen, _cx| codegen.active_alternative().clone())
        })
    }
}

struct EditorInlineAssists {
    assist_ids: Vec<InlineAssistId>,
    scroll_lock: Option<InlineAssistScrollLock>,
    highlight_updates: watch::Sender<()>,
    _update_highlights: Task<Result<()>>,
    _subscriptions: Vec<gpui::Subscription>,
}

struct InlineAssistScrollLock {
    assist_id: InlineAssistId,
    distance_from_top: ScrollOffset,
}

impl EditorInlineAssists {
    fn new(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) -> Self {
        let (highlight_updates_tx, mut highlight_updates_rx) = watch::channel(());
        Self {
            assist_ids: Vec::new(),
            scroll_lock: None,
            highlight_updates: highlight_updates_tx,
            _update_highlights: cx.spawn({
                let editor = editor.downgrade();
                async move |cx| {
                    while let Ok(()) = highlight_updates_rx.changed().await {
                        let editor = editor.upgrade().context("editor was dropped")?;
                        cx.update_global(|assistant: &mut InlineAssistant, cx| {
                            assistant.update_editor_highlights(&editor, cx);
                        })?;
                    }
                    Ok(())
                }
            }),
            _subscriptions: vec![
                cx.observe_release_in(editor, window, {
                    let editor = editor.downgrade();
                    |_, window, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            this.handle_editor_release(editor, window, cx);
                        })
                    }
                }),
                window.observe(editor, cx, move |editor, window, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_editor_change(editor, window, cx)
                    })
                }),
                window.subscribe(editor, cx, move |editor, event, window, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_editor_event(editor, event, window, cx)
                    })
                }),
                editor.update(cx, |editor, cx| {
                    let editor_handle = cx.entity().downgrade();
                    editor.register_action(move |_: &editor::actions::Newline, window, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            if let Some(editor) = editor_handle.upgrade() {
                                this.handle_editor_newline(editor, window, cx)
                            }
                        })
                    })
                }),
                editor.update(cx, |editor, cx| {
                    let editor_handle = cx.entity().downgrade();
                    editor.register_action(move |_: &editor::actions::Cancel, window, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            if let Some(editor) = editor_handle.upgrade() {
                                this.handle_editor_cancel(editor, window, cx)
                            }
                        })
                    })
                }),
            ],
        }
    }
}

struct InlineAssistGroup {
    assist_ids: Vec<InlineAssistId>,
    linked: bool,
    active_assist_id: Option<InlineAssistId>,
}

impl InlineAssistGroup {
    fn new() -> Self {
        Self {
            assist_ids: Vec::new(),
            linked: true,
            active_assist_id: None,
        }
    }
}

fn build_assist_editor_renderer(editor: &Entity<PromptEditor<BufferCodegen>>) -> RenderBlock {
    let editor = editor.clone();

    Arc::new(move |cx: &mut BlockContext| {
        let editor_margins = editor.read(cx).editor_margins();

        *editor_margins.lock() = *cx.margins;
        editor.clone().into_any_element()
    })
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct InlineAssistGroupId(usize);

impl InlineAssistGroupId {
    fn post_inc(&mut self) -> InlineAssistGroupId {
        let id = *self;
        self.0 += 1;
        id
    }
}

pub struct InlineAssist {
    group_id: InlineAssistGroupId,
    range: Range<Anchor>,
    editor: WeakEntity<Editor>,
    decorations: Option<InlineAssistDecorations>,
    codegen: Entity<BufferCodegen>,
    _subscriptions: Vec<Subscription>,
    workspace: WeakEntity<Workspace>,
}

impl InlineAssist {
    fn new(
        assist_id: InlineAssistId,
        group_id: InlineAssistGroupId,
        editor: &Entity<Editor>,
        prompt_editor: &Entity<PromptEditor<BufferCodegen>>,
        prompt_block_id: CustomBlockId,
        tool_description_block_id: CustomBlockId,
        end_block_id: CustomBlockId,
        range: Range<Anchor>,
        codegen: Entity<BufferCodegen>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let prompt_editor_focus_handle = prompt_editor.focus_handle(cx);
        InlineAssist {
            group_id,
            editor: editor.downgrade(),
            decorations: Some(InlineAssistDecorations {
                prompt_block_id,
                prompt_editor: prompt_editor.clone(),
                removed_line_block_ids: Default::default(),
                model_explanation: Some(tool_description_block_id),
                end_block_id,
            }),
            range,
            codegen: codegen.clone(),
            workspace,
            _subscriptions: vec![
                window.on_focus_in(&prompt_editor_focus_handle, cx, move |_, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_focus_in(assist_id, cx)
                    })
                }),
                window.on_focus_out(&prompt_editor_focus_handle, cx, move |_, _, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_focus_out(assist_id, cx)
                    })
                }),
                window.subscribe(prompt_editor, cx, |prompt_editor, event, window, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_event(prompt_editor, event, window, cx)
                    })
                }),
                window.observe(&codegen, cx, {
                    let editor = editor.downgrade();
                    move |_, window, cx| {
                        if let Some(editor) = editor.upgrade() {
                            InlineAssistant::update_global(cx, |this, cx| {
                                if let Some(editor_assists) =
                                    this.assists_by_editor.get_mut(&editor.downgrade())
                                {
                                    editor_assists.highlight_updates.send(()).ok();
                                }

                                this.update_editor_blocks(&editor, assist_id, window, cx);
                            })
                        }
                    }
                }),
                window.subscribe(&codegen, cx, move |codegen, event, window, cx| {
                    InlineAssistant::update_global(cx, |this, cx| match event {
                        CodegenEvent::Undone => this.finish_assist(assist_id, false, window, cx),
                        CodegenEvent::Finished => {
                            let assist = if let Some(assist) = this.assists.get(&assist_id) {
                                assist
                            } else {
                                return;
                            };

                            if let CodegenStatus::Error(error) = codegen.read(cx).status(cx)
                                && assist.decorations.is_none()
                                && let Some(workspace) = assist.workspace.upgrade()
                            {
                                #[cfg(any(test, feature = "test-support"))]
                                if let Some(sender) = &mut this._inline_assistant_completions {
                                    sender
                                        .unbounded_send(Err(anyhow::anyhow!(
                                            "Inline assistant error: {}",
                                            error
                                        )))
                                        .ok();
                                }

                                let error = format!("Inline assistant error: {}", error);
                                workspace.update(cx, |workspace, cx| {
                                    struct InlineAssistantError;

                                    let id = NotificationId::composite::<InlineAssistantError>(
                                        assist_id.0,
                                    );

                                    workspace.show_toast(Toast::new(id, error), cx);
                                })
                            } else {
                                #[cfg(any(test, feature = "test-support"))]
                                if let Some(sender) = &mut this._inline_assistant_completions {
                                    sender.unbounded_send(Ok(assist_id)).ok();
                                }
                            }

                            if assist.decorations.is_none() {
                                this.finish_assist(assist_id, false, window, cx);
                            }
                        }
                    })
                }),
            ],
        }
    }

    fn user_prompt(&self, cx: &App) -> Option<String> {
        let decorations = self.decorations.as_ref()?;
        Some(decorations.prompt_editor.read(cx).prompt(cx))
    }

    fn mention_set(&self, cx: &App) -> Option<Entity<MentionSet>> {
        let decorations = self.decorations.as_ref()?;
        Some(decorations.prompt_editor.read(cx).mention_set().clone())
    }
}

struct InlineAssistDecorations {
    prompt_block_id: CustomBlockId,
    prompt_editor: Entity<PromptEditor<BufferCodegen>>,
    removed_line_block_ids: HashSet<CustomBlockId>,
    model_explanation: Option<CustomBlockId>,
    end_block_id: CustomBlockId,
}

struct AssistantCodeActionProvider {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
}

const ASSISTANT_CODE_ACTION_PROVIDER_ID: &str = "assistant";

impl CodeActionProvider for AssistantCodeActionProvider {
    fn id(&self) -> Arc<str> {
        ASSISTANT_CODE_ACTION_PROVIDER_ID.into()
    }

    fn code_actions(
        &self,
        buffer: &Entity<Buffer>,
        range: Range<text::Anchor>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<CodeAction>>> {
        if !AgentSettings::get_global(cx).enabled(cx) {
            return Task::ready(Ok(Vec::new()));
        }

        let snapshot = buffer.read(cx).snapshot();
        let mut range = range.to_point(&snapshot);

        // Expand the range to line boundaries.
        range.start.column = 0;
        range.end.column = snapshot.line_len(range.end.row);

        let mut has_diagnostics = false;
        for diagnostic in snapshot.diagnostics_in_range::<_, Point>(range.clone(), false) {
            range.start = cmp::min(range.start, diagnostic.range.start);
            range.end = cmp::max(range.end, diagnostic.range.end);
            has_diagnostics = true;
        }
        if has_diagnostics {
            let symbols_containing_start = snapshot.symbols_containing(range.start, None);
            if let Some(symbol) = symbols_containing_start.last() {
                range.start = cmp::min(range.start, symbol.range.start.to_point(&snapshot));
                range.end = cmp::max(range.end, symbol.range.end.to_point(&snapshot));
            }
            let symbols_containing_end = snapshot.symbols_containing(range.end, None);
            if let Some(symbol) = symbols_containing_end.last() {
                range.start = cmp::min(range.start, symbol.range.start.to_point(&snapshot));
                range.end = cmp::max(range.end, symbol.range.end.to_point(&snapshot));
            }

            Task::ready(Ok(vec![CodeAction {
                server_id: language::LanguageServerId(0),
                range: snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end),
                lsp_action: LspAction::Action(Box::new(lsp::CodeAction {
                    title: "Fix with Assistant".into(),
                    ..Default::default()
                })),
                resolved: true,
            }]))
        } else {
            Task::ready(Ok(Vec::new()))
        }
    }

    fn apply_code_action(
        &self,
        buffer: Entity<Buffer>,
        action: CodeAction,
        excerpt_id: ExcerptId,
        _push_to_history: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<ProjectTransaction>> {
        let editor = self.editor.clone();
        let workspace = self.workspace.clone();
        let prompt_store = PromptStore::global(cx);
        window.spawn(cx, async move |cx| {
            let workspace = workspace.upgrade().context("workspace was released")?;
            let thread_store = cx.update(|_window, cx| {
                anyhow::Ok(
                    workspace
                        .read(cx)
                        .panel::<AgentPanel>(cx)
                        .context("missing agent panel")?
                        .read(cx)
                        .thread_store()
                        .clone(),
                )
            })??;
            let editor = editor.upgrade().context("editor was released")?;
            let range = editor
                .update(cx, |editor, cx| {
                    editor.buffer().update(cx, |multibuffer, cx| {
                        let buffer = buffer.read(cx);
                        let multibuffer_snapshot = multibuffer.read(cx);

                        let old_context_range =
                            multibuffer_snapshot.context_range_for_excerpt(excerpt_id)?;
                        let mut new_context_range = old_context_range.clone();
                        if action
                            .range
                            .start
                            .cmp(&old_context_range.start, buffer)
                            .is_lt()
                        {
                            new_context_range.start = action.range.start;
                        }
                        if action.range.end.cmp(&old_context_range.end, buffer).is_gt() {
                            new_context_range.end = action.range.end;
                        }
                        drop(multibuffer_snapshot);

                        if new_context_range != old_context_range {
                            multibuffer.resize_excerpt(excerpt_id, new_context_range, cx);
                        }

                        let multibuffer_snapshot = multibuffer.read(cx);
                        multibuffer_snapshot.anchor_range_in_excerpt(excerpt_id, action.range)
                    })
                })?
                .context("invalid range")?;

            let prompt_store = prompt_store.await.ok();
            cx.update_global(|assistant: &mut InlineAssistant, window, cx| {
                let assist_id = assistant.suggest_assist(
                    &editor,
                    range,
                    "Fix Diagnostics".into(),
                    None,
                    true,
                    workspace,
                    thread_store,
                    prompt_store,
                    window,
                    cx,
                );
                assistant.start_assist(assist_id, window, cx);
            })?;

            Ok(ProjectTransaction::default())
        })
    }
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

#[cfg(any(test, feature = "unit-eval"))]
#[cfg_attr(not(test), allow(dead_code))]
pub mod test {

    use std::sync::Arc;

    use agent::HistoryStore;
    use assistant_text_thread::TextThreadStore;
    use client::{Client, UserStore};
    use editor::{Editor, MultiBuffer, MultiBufferOffset};
    use fs::FakeFs;
    use futures::channel::mpsc;
    use gpui::{AppContext, TestAppContext, UpdateGlobal as _};
    use language::Buffer;
    use project::Project;
    use prompt_store::PromptBuilder;
    use smol::stream::StreamExt as _;
    use util::test::marked_text_ranges;
    use workspace::Workspace;

    use crate::InlineAssistant;

    #[derive(Debug)]
    pub enum InlineAssistantOutput {
        Success {
            completion: Option<String>,
            description: Option<String>,
            full_buffer_text: String,
        },
        Failure {
            failure: String,
        },
        // These fields are used for logging
        #[allow(unused)]
        Malformed {
            completion: Option<String>,
            description: Option<String>,
            failure: Option<String>,
        },
    }

    pub fn run_inline_assistant_test<SetupF, TestF>(
        base_buffer: String,
        prompt: String,
        setup: SetupF,
        test: TestF,
        cx: &mut TestAppContext,
    ) -> InlineAssistantOutput
    where
        SetupF: FnOnce(&mut gpui::VisualTestContext),
        TestF: FnOnce(&mut gpui::VisualTestContext),
    {
        let fs = FakeFs::new(cx.executor());
        let app_state = cx.update(|cx| workspace::AppState::test(cx));
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let http = Arc::new(reqwest_client::ReqwestClient::user_agent("agent tests").unwrap());
        let client = cx.update(|cx| {
            cx.set_http_client(http);
            Client::production(cx)
        });
        let mut inline_assistant = InlineAssistant::new(fs.clone(), prompt_builder);

        let (tx, mut completion_rx) = mpsc::unbounded();
        inline_assistant.set_completion_receiver(tx);

        // Initialize settings and client
        cx.update(|cx| {
            gpui_tokio::init(cx);
            settings::init(cx);
            client::init(&client, cx);
            workspace::init(app_state.clone(), cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            language_model::init(client.clone(), cx);
            language_models::init(user_store, client.clone(), cx);

            cx.set_global(inline_assistant);
        });

        let project = cx
            .executor()
            .block_test(async { Project::test(fs.clone(), [], cx).await });

        // Create workspace with window
        let (workspace, cx) = cx.add_window_view(|window, cx| {
            window.activate_window();
            Workspace::new(None, project.clone(), app_state.clone(), window, cx)
        });

        setup(cx);

        let (_editor, buffer) = cx.update(|window, cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx));
            let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));
            let editor = cx.new(|cx| Editor::for_multibuffer(multibuffer, None, window, cx));
            editor.update(cx, |editor, cx| {
                let (unmarked_text, selection_ranges) = marked_text_ranges(&base_buffer, true);
                editor.set_text(unmarked_text, window, cx);
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges(
                        selection_ranges.into_iter().map(|range| {
                            MultiBufferOffset(range.start)..MultiBufferOffset(range.end)
                        }),
                    )
                })
            });

            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));

            // Add editor to workspace
            workspace.update(cx, |workspace, cx| {
                workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
            });

            // Call assist method
            InlineAssistant::update_global(cx, |inline_assistant, cx| {
                let assist_id = inline_assistant
                    .assist(
                        &editor,
                        workspace.downgrade(),
                        project.downgrade(),
                        history_store, // thread_store
                        None,          // prompt_store
                        Some(prompt),
                        window,
                        cx,
                    )
                    .unwrap();

                inline_assistant.start_assist(assist_id, window, cx);
            });

            (editor, buffer)
        });

        cx.run_until_parked();

        test(cx);

        let assist_id = cx
            .executor()
            .block_test(async { completion_rx.next().await })
            .unwrap()
            .unwrap();

        let (completion, description, failure) = cx.update(|_, cx| {
            InlineAssistant::update_global(cx, |inline_assistant, cx| {
                let codegen = inline_assistant.get_codegen(assist_id, cx).unwrap();

                let completion = codegen.read(cx).current_completion();
                let description = codegen.read(cx).current_description();
                let failure = codegen.read(cx).current_failure();

                (completion, description, failure)
            })
        });

        if failure.is_some() && (completion.is_some() || description.is_some()) {
            InlineAssistantOutput::Malformed {
                completion,
                description,
                failure,
            }
        } else if let Some(failure) = failure {
            InlineAssistantOutput::Failure { failure }
        } else {
            InlineAssistantOutput::Success {
                completion,
                description,
                full_buffer_text: buffer.read_with(cx, |buffer, _| buffer.text()),
            }
        }
    }
}

#[cfg(any(test, feature = "unit-eval"))]
#[cfg_attr(not(test), allow(dead_code))]
pub mod evals {
    use std::str::FromStr;

    use eval_utils::{EvalOutput, NoProcessor};
    use gpui::TestAppContext;
    use language_model::{LanguageModelRegistry, SelectedModel};
    use rand::{SeedableRng as _, rngs::StdRng};

    use crate::inline_assistant::test::{InlineAssistantOutput, run_inline_assistant_test};

    #[test]
    #[cfg_attr(not(feature = "unit-eval"), ignore)]
    fn eval_single_cursor_edit() {
        run_eval(
            20,
            1.0,
            "Rename this variable to buffer_text".to_string(),
            indoc::indoc! {"
                struct EvalExampleStruct {
                    text: Strˇing,
                    prompt: String,
                }
            "}
            .to_string(),
            exact_buffer_match(indoc::indoc! {"
                struct EvalExampleStruct {
                    buffer_text: String,
                    prompt: String,
                }
            "}),
        );
    }

    #[test]
    #[cfg_attr(not(feature = "unit-eval"), ignore)]
    fn eval_cant_do() {
        run_eval(
            20,
            0.95,
            "Rename the struct to EvalExampleStructNope",
            indoc::indoc! {"
                struct EvalExampleStruct {
                    text: Strˇing,
                    prompt: String,
                }
            "},
            uncertain_output,
        );
    }

    #[test]
    #[cfg_attr(not(feature = "unit-eval"), ignore)]
    fn eval_unclear() {
        run_eval(
            20,
            0.95,
            "Make exactly the change I want you to make",
            indoc::indoc! {"
                struct EvalExampleStruct {
                    text: Strˇing,
                    prompt: String,
                }
            "},
            uncertain_output,
        );
    }

    #[test]
    #[cfg_attr(not(feature = "unit-eval"), ignore)]
    fn eval_empty_buffer() {
        run_eval(
            20,
            1.0,
            "Write a Python hello, world program".to_string(),
            "ˇ".to_string(),
            |output| match output {
                InlineAssistantOutput::Success {
                    full_buffer_text, ..
                } => {
                    if full_buffer_text.is_empty() {
                        EvalOutput::failed("expected some output".to_string())
                    } else {
                        EvalOutput::passed(format!("Produced {full_buffer_text}"))
                    }
                }
                o @ InlineAssistantOutput::Failure { .. } => EvalOutput::failed(format!(
                    "Assistant output does not match expected output: {:?}",
                    o
                )),
                o @ InlineAssistantOutput::Malformed { .. } => EvalOutput::failed(format!(
                    "Assistant output does not match expected output: {:?}",
                    o
                )),
            },
        );
    }

    fn run_eval(
        iterations: usize,
        expected_pass_ratio: f32,
        prompt: impl Into<String>,
        buffer: impl Into<String>,
        judge: impl Fn(InlineAssistantOutput) -> eval_utils::EvalOutput<()> + Send + Sync + 'static,
    ) {
        let buffer = buffer.into();
        let prompt = prompt.into();

        eval_utils::eval(iterations, expected_pass_ratio, NoProcessor, move || {
            let dispatcher = gpui::TestDispatcher::new(StdRng::from_os_rng());
            let mut cx = TestAppContext::build(dispatcher, None);
            cx.skip_drawing();

            let output = run_inline_assistant_test(
                buffer.clone(),
                prompt.clone(),
                |cx| {
                    // Reconfigure to use a real model instead of the fake one
                    let model_name = std::env::var("ZED_AGENT_MODEL")
                        .unwrap_or("anthropic/claude-sonnet-4-latest".into());

                    let selected_model = SelectedModel::from_str(&model_name)
                        .expect("Invalid model format. Use 'provider/model-id'");

                    log::info!("Selected model: {selected_model:?}");

                    cx.update(|_, cx| {
                        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                            registry.select_inline_assistant_model(Some(&selected_model), cx);
                        });
                    });
                },
                |_cx| {
                    log::info!("Waiting for actual response from the LLM...");
                },
                &mut cx,
            );

            cx.quit();

            judge(output)
        });
    }

    fn uncertain_output(output: InlineAssistantOutput) -> EvalOutput<()> {
        match &output {
            o @ InlineAssistantOutput::Success {
                completion,
                description,
                ..
            } => {
                if description.is_some() && completion.is_none() {
                    EvalOutput::passed(format!(
                        "Assistant produced no completion, but a description:\n{}",
                        description.as_ref().unwrap()
                    ))
                } else {
                    EvalOutput::failed(format!("Assistant produced a completion:\n{:?}", o))
                }
            }
            InlineAssistantOutput::Failure {
                failure: error_message,
            } => EvalOutput::passed(format!(
                "Assistant produced a failure message: {}",
                error_message
            )),
            o @ InlineAssistantOutput::Malformed { .. } => {
                EvalOutput::failed(format!("Assistant produced a malformed response:\n{:?}", o))
            }
        }
    }

    fn exact_buffer_match(
        correct_output: impl Into<String>,
    ) -> impl Fn(InlineAssistantOutput) -> EvalOutput<()> {
        let correct_output = correct_output.into();
        move |output| match output {
            InlineAssistantOutput::Success {
                description,
                full_buffer_text,
                ..
            } => {
                if full_buffer_text == correct_output && description.is_none() {
                    EvalOutput::passed("Assistant output matches")
                } else if full_buffer_text == correct_output {
                    EvalOutput::failed(format!(
                        "Assistant output produced an unescessary description description:\n{:?}",
                        description
                    ))
                } else {
                    EvalOutput::failed(format!(
                        "Assistant output does not match expected output:\n{:?}\ndescription:\n{:?}",
                        full_buffer_text, description
                    ))
                }
            }
            o @ InlineAssistantOutput::Failure { .. } => EvalOutput::failed(format!(
                "Assistant output does not match expected output: {:?}",
                o
            )),
            o @ InlineAssistantOutput::Malformed { .. } => EvalOutput::failed(format!(
                "Assistant output does not match expected output: {:?}",
                o
            )),
        }
    }
}
