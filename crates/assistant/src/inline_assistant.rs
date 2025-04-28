use crate::{
    Assistant, AssistantPanel, AssistantPanelEvent, CycleNextInlineAssist,
    CyclePreviousInlineAssist,
};
use anyhow::{Context as _, Result, anyhow};
use assistant_context_editor::{RequestType, humanize_token_count};
use assistant_settings::AssistantSettings;
use client::{ErrorExt, telemetry::Telemetry};
use collections::{HashMap, HashSet, VecDeque, hash_map};
use editor::{
    Anchor, AnchorRangeExt, CodeActionProvider, Editor, EditorElement, EditorEvent, EditorMode,
    EditorStyle, ExcerptId, ExcerptRange, GutterDimensions, MultiBuffer, MultiBufferSnapshot,
    ToOffset as _, ToPoint,
    actions::{MoveDown, MoveUp, SelectAll},
    display_map::{
        BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId, RenderBlock,
        ToDisplayPoint,
    },
};
use feature_flags::{
    Assistant2FeatureFlag, FeatureFlagAppExt as _, FeatureFlagViewExt as _, ZedPro,
};
use fs::Fs;
use futures::{
    SinkExt, Stream, StreamExt, TryStreamExt as _,
    channel::mpsc,
    future::{BoxFuture, LocalBoxFuture},
    join,
};
use gpui::{
    AnyElement, App, ClickEvent, Context, CursorStyle, Entity, EventEmitter, FocusHandle,
    Focusable, FontWeight, Global, HighlightStyle, Subscription, Task, TextStyle, UpdateGlobal,
    WeakEntity, Window, anchored, deferred, point,
};
use language::{Buffer, IndentKind, Point, Selection, TransactionId, line_diff};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelTextStream, Role, report_assistant_event,
};
use language_model_selector::{LanguageModelSelector, LanguageModelSelectorPopoverMenu, ModelType};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use project::{CodeAction, LspAction, ProjectTransaction};
use prompt_store::PromptBuilder;
use rope::Rope;
use settings::{Settings, SettingsStore, update_settings_file};
use smol::future::FutureExt;
use std::{
    cmp,
    future::{self, Future},
    iter, mem,
    ops::{Range, RangeInclusive},
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{self, Poll},
    time::{Duration, Instant},
};
use streaming_diff::{CharOperation, LineDiff, LineOperation, StreamingDiff};
use telemetry_events::{AssistantEventData, AssistantKind, AssistantPhase};
use terminal_view::terminal_panel::TerminalPanel;
use text::{OffsetRangeExt, ToPoint as _};
use theme::ThemeSettings;
use ui::{
    CheckboxWithLabel, IconButtonShape, KeyBinding, Popover, Tooltip, prelude::*, text_for_action,
};
use util::{RangeExt, ResultExt};
use workspace::{ItemHandle, Toast, Workspace, notifications::NotificationId};

pub fn init(
    fs: Arc<dyn Fs>,
    prompt_builder: Arc<PromptBuilder>,
    telemetry: Arc<Telemetry>,
    cx: &mut App,
) {
    cx.set_global(InlineAssistant::new(fs, prompt_builder, telemetry));
    cx.observe_new(|_, window, cx| {
        let Some(window) = window else {
            return;
        };
        let workspace = cx.entity().clone();
        InlineAssistant::update_global(cx, |inline_assistant, cx| {
            inline_assistant.register_workspace(&workspace, window, cx)
        });

        cx.observe_flag::<Assistant2FeatureFlag, _>(window, {
            |is_assistant2_enabled, _workspace, _window, cx| {
                InlineAssistant::update_global(cx, |inline_assistant, _cx| {
                    inline_assistant.is_assistant2_enabled = is_assistant2_enabled;
                });
            }
        })
        .detach();
    })
    .detach();
}

const PROMPT_HISTORY_MAX_LEN: usize = 20;

pub struct InlineAssistant {
    next_assist_id: InlineAssistId,
    next_assist_group_id: InlineAssistGroupId,
    assists: HashMap<InlineAssistId, InlineAssist>,
    assists_by_editor: HashMap<WeakEntity<Editor>, EditorInlineAssists>,
    assist_groups: HashMap<InlineAssistGroupId, InlineAssistGroup>,
    confirmed_assists: HashMap<InlineAssistId, Entity<CodegenAlternative>>,
    prompt_history: VecDeque<String>,
    prompt_builder: Arc<PromptBuilder>,
    telemetry: Arc<Telemetry>,
    fs: Arc<dyn Fs>,
    is_assistant2_enabled: bool,
}

impl Global for InlineAssistant {}

impl InlineAssistant {
    pub fn new(
        fs: Arc<dyn Fs>,
        prompt_builder: Arc<PromptBuilder>,
        telemetry: Arc<Telemetry>,
    ) -> Self {
        Self {
            next_assist_id: InlineAssistId::default(),
            next_assist_group_id: InlineAssistGroupId::default(),
            assists: HashMap::default(),
            assists_by_editor: HashMap::default(),
            assist_groups: HashMap::default(),
            confirmed_assists: HashMap::default(),
            prompt_history: VecDeque::default(),
            prompt_builder,
            telemetry,
            fs,
            is_assistant2_enabled: false,
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
            let enabled = AssistantSettings::get_global(cx).enabled;
            terminal_panel.update(cx, |terminal_panel, cx| {
                terminal_panel.set_assistant_enabled(enabled, cx)
            });
        })
        .detach();
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
                if let Some(editor) = item.upgrade().and_then(|item| item.act_as::<Editor>(cx)) {
                    if let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) {
                        for assist_id in editor_assists.assist_ids.clone() {
                            let assist = &self.assists[&assist_id];
                            if let CodegenStatus::Done = assist.codegen.read(cx).status(cx) {
                                self.finish_assist(assist_id, false, window, cx)
                            }
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
        let is_assistant2_enabled = self.is_assistant2_enabled;

        if let Some(editor) = item.act_as::<Editor>(cx) {
            editor.update(cx, |editor, cx| {
                if is_assistant2_enabled {
                    editor.remove_code_action_provider(
                        ASSISTANT_CODE_ACTION_PROVIDER_ID.into(),
                        window,
                        cx,
                    );
                } else {
                    editor.add_code_action_provider(
                        Rc::new(AssistantCodeActionProvider {
                            editor: cx.entity().downgrade(),
                            workspace: workspace.downgrade(),
                        }),
                        window,
                        cx,
                    );
                }
            });
        }
    }

    pub fn assist(
        &mut self,
        editor: &Entity<Editor>,
        workspace: Option<WeakEntity<Workspace>>,
        assistant_panel: Option<&Entity<AssistantPanel>>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (snapshot, initial_selections) = editor.update(cx, |editor, cx| {
            (
                editor.snapshot(window, cx),
                editor.selections.all::<Point>(cx),
            )
        });

        let mut selections = Vec::<Selection<Point>>::new();
        let mut newest_selection = None;
        for mut selection in initial_selections {
            if selection.end > selection.start {
                selection.start.column = 0;
                // If the selection ends at the start of the line, we don't want to include it.
                if selection.end.column == 0 {
                    selection.end.row -= 1;
                }
                selection.end.column = snapshot
                    .buffer_snapshot
                    .line_len(MultiBufferRow(selection.end.row));
            } else if let Some(fold) =
                snapshot.crease_for_buffer_row(MultiBufferRow(selection.end.row))
            {
                selection.start = fold.range().start;
                selection.end = fold.range().end;
                if MultiBufferRow(selection.end.row) < snapshot.buffer_snapshot.max_row() {
                    let chars = snapshot
                        .buffer_snapshot
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
                                .buffer_snapshot
                                .line_len(MultiBufferRow(selection.end.row));
                        }
                    }
                }
            }

            if let Some(prev_selection) = selections.last_mut() {
                if selection.start <= prev_selection.end {
                    prev_selection.end = selection.end;
                    continue;
                }
            }

            let latest_selection = newest_selection.get_or_insert_with(|| selection.clone());
            if selection.id > latest_selection.id {
                *latest_selection = selection.clone();
            }
            selections.push(selection);
        }
        let snapshot = &snapshot.buffer_snapshot;
        let newest_selection = newest_selection.unwrap();

        let mut codegen_ranges = Vec::new();
        for (buffer, buffer_range, excerpt_id) in
            snapshot.ranges_to_buffer_ranges(selections.iter().map(|selection| {
                snapshot.anchor_before(selection.start)..snapshot.anchor_after(selection.end)
            }))
        {
            let start = buffer.anchor_before(buffer_range.start);
            let end = buffer.anchor_after(buffer_range.end);

            codegen_ranges.push(Anchor::range_in_buffer(
                excerpt_id,
                buffer.remote_id(),
                start..end,
            ));

            if let Some(ConfiguredModel { model, .. }) =
                LanguageModelRegistry::read_global(cx).default_model()
            {
                self.telemetry.report_assistant_event(AssistantEventData {
                    conversation_id: None,
                    kind: AssistantKind::Inline,
                    phase: AssistantPhase::Invoked,
                    message_id: None,
                    model: model.telemetry_id(),
                    model_provider: model.provider_id().to_string(),
                    response_latency: None,
                    error_message: None,
                    language_name: buffer.language().map(|language| language.name().to_proto()),
                });
            }
        }

        let assist_group_id = self.next_assist_group_id.post_inc();
        let prompt_buffer = cx.new(|cx| Buffer::local(initial_prompt.unwrap_or_default(), cx));
        let prompt_buffer = cx.new(|cx| MultiBuffer::singleton(prompt_buffer, cx));

        let mut assists = Vec::new();
        let mut assist_to_focus = None;
        for range in codegen_ranges {
            let assist_id = self.next_assist_id.post_inc();
            let codegen = cx.new(|cx| {
                Codegen::new(
                    editor.read(cx).buffer().clone(),
                    range.clone(),
                    None,
                    self.telemetry.clone(),
                    self.prompt_builder.clone(),
                    cx,
                )
            });

            let gutter_dimensions = Arc::new(Mutex::new(GutterDimensions::default()));
            let prompt_editor = cx.new(|cx| {
                PromptEditor::new(
                    assist_id,
                    gutter_dimensions.clone(),
                    self.prompt_history.clone(),
                    prompt_buffer.clone(),
                    codegen.clone(),
                    editor,
                    assistant_panel,
                    workspace.clone(),
                    self.fs.clone(),
                    window,
                    cx,
                )
            });

            if assist_to_focus.is_none() {
                let focus_assist = if newest_selection.reversed {
                    range.start.to_point(&snapshot) == newest_selection.start
                } else {
                    range.end.to_point(&snapshot) == newest_selection.end
                };
                if focus_assist {
                    assist_to_focus = Some(assist_id);
                }
            }

            let [prompt_block_id, end_block_id] =
                self.insert_assist_blocks(editor, &range, &prompt_editor, cx);

            assists.push((
                assist_id,
                range,
                prompt_editor,
                prompt_block_id,
                end_block_id,
            ));
        }

        let editor_assists = self
            .assists_by_editor
            .entry(editor.downgrade())
            .or_insert_with(|| EditorInlineAssists::new(&editor, window, cx));
        let mut assist_group = InlineAssistGroup::new();
        for (assist_id, range, prompt_editor, prompt_block_id, end_block_id) in assists {
            self.assists.insert(
                assist_id,
                InlineAssist::new(
                    assist_id,
                    assist_group_id,
                    assistant_panel.is_some(),
                    editor,
                    &prompt_editor,
                    prompt_block_id,
                    end_block_id,
                    range,
                    prompt_editor.read(cx).codegen.clone(),
                    workspace.clone(),
                    window,
                    cx,
                ),
            );
            assist_group.assist_ids.push(assist_id);
            editor_assists.assist_ids.push(assist_id);
        }
        self.assist_groups.insert(assist_group_id, assist_group);

        if let Some(assist_id) = assist_to_focus {
            self.focus_assist(assist_id, window, cx);
        }
    }

    pub fn suggest_assist(
        &mut self,
        editor: &Entity<Editor>,
        mut range: Range<Anchor>,
        initial_prompt: String,
        initial_transaction_id: Option<TransactionId>,
        focus: bool,
        workspace: Option<WeakEntity<Workspace>>,
        assistant_panel: Option<&Entity<AssistantPanel>>,
        window: &mut Window,
        cx: &mut App,
    ) -> InlineAssistId {
        let assist_group_id = self.next_assist_group_id.post_inc();
        let prompt_buffer = cx.new(|cx| Buffer::local(&initial_prompt, cx));
        let prompt_buffer = cx.new(|cx| MultiBuffer::singleton(prompt_buffer, cx));

        let assist_id = self.next_assist_id.post_inc();

        let buffer = editor.read(cx).buffer().clone();
        {
            let snapshot = buffer.read(cx).read(cx);
            range.start = range.start.bias_left(&snapshot);
            range.end = range.end.bias_right(&snapshot);
        }

        let codegen = cx.new(|cx| {
            Codegen::new(
                editor.read(cx).buffer().clone(),
                range.clone(),
                initial_transaction_id,
                self.telemetry.clone(),
                self.prompt_builder.clone(),
                cx,
            )
        });

        let gutter_dimensions = Arc::new(Mutex::new(GutterDimensions::default()));
        let prompt_editor = cx.new(|cx| {
            PromptEditor::new(
                assist_id,
                gutter_dimensions.clone(),
                self.prompt_history.clone(),
                prompt_buffer.clone(),
                codegen.clone(),
                editor,
                assistant_panel,
                workspace.clone(),
                self.fs.clone(),
                window,
                cx,
            )
        });

        let [prompt_block_id, end_block_id] =
            self.insert_assist_blocks(editor, &range, &prompt_editor, cx);

        let editor_assists = self
            .assists_by_editor
            .entry(editor.downgrade())
            .or_insert_with(|| EditorInlineAssists::new(&editor, window, cx));

        let mut assist_group = InlineAssistGroup::new();
        self.assists.insert(
            assist_id,
            InlineAssist::new(
                assist_id,
                assist_group_id,
                assistant_panel.is_some(),
                editor,
                &prompt_editor,
                prompt_block_id,
                end_block_id,
                range,
                prompt_editor.read(cx).codegen.clone(),
                workspace.clone(),
                window,
                cx,
            ),
        );
        assist_group.assist_ids.push(assist_id);
        editor_assists.assist_ids.push(assist_id);
        self.assist_groups.insert(assist_group_id, assist_group);

        if focus {
            self.focus_assist(assist_id, window, cx);
        }

        assist_id
    }

    fn insert_assist_blocks(
        &self,
        editor: &Entity<Editor>,
        range: &Range<Anchor>,
        prompt_editor: &Entity<PromptEditor>,
        cx: &mut App,
    ) -> [CustomBlockId; 2] {
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
            [block_ids[0], block_ids[1]]
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
                let prompt_row = editor
                    .row_for_block(decorations.prompt_block_id, cx)
                    .unwrap()
                    .0 as f32;

                if (scroll_top..scroll_bottom).contains(&prompt_row) {
                    editor_assists.scroll_lock = Some(InlineAssistScrollLock {
                        assist_id,
                        distance_from_top: prompt_row - scroll_top,
                    });
                } else {
                    editor_assists.scroll_lock = None;
                }
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
        prompt_editor: Entity<PromptEditor>,
        event: &PromptEditorEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        let assist_id = prompt_editor.read(cx).id;
        match event {
            PromptEditorEvent::StartRequested => {
                self.start_assist(assist_id, window, cx);
            }
            PromptEditorEvent::StopRequested => {
                self.stop_assist(assist_id, cx);
            }
            PromptEditorEvent::ConfirmRequested => {
                self.finish_assist(assist_id, false, window, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, window, cx);
            }
            PromptEditorEvent::DismissRequested => {
                self.dismiss_assist(assist_id, window, cx);
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
                    editor.selections.newest::<usize>(cx),
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
                    editor.selections.newest::<usize>(cx),
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
                            .abs_diff(selection.start)
                            .min(assist_range.start.abs_diff(selection.end))
                            + assist_range
                                .end
                                .abs_diff(selection.start)
                                .min(assist_range.end.abs_diff(selection.end));
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
                .row_for_block(decorations.prompt_block_id, cx)
                .unwrap()
                .0 as f32
                - scroll_lock.distance_from_top;
            if target_scroll_top != scroll_position.y {
                editor.set_scroll_position(point(scroll_position.x, target_scroll_top), window, cx);
            }
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
                    buffer.edited_ranges_for_transaction::<usize>(*transaction_id, cx);
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
                                .row_for_block(decorations.prompt_block_id, cx)
                                .unwrap()
                                .0 as f32;
                            prompt_row - scroll_top
                        });

                        if distance_from_top != scroll_lock.distance_from_top {
                            editor_assists.scroll_lock = None;
                        }
                    }
                }
            }
            EditorEvent::SelectionsChanged { .. } => {
                for assist_id in editor_assists.assist_ids.clone() {
                    let assist = &self.assists[&assist_id];
                    if let Some(decorations) = assist.decorations.as_ref() {
                        if decorations
                            .prompt_editor
                            .focus_handle(cx)
                            .is_focused(window)
                        {
                            return;
                        }
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
                    entry.get().highlight_updates.send(()).ok();
                }
            }

            let active_alternative = assist.codegen.read(cx).active_alternative().clone();
            let message_id = active_alternative.read(cx).message_id.clone();

            if let Some(ConfiguredModel { model, .. }) =
                LanguageModelRegistry::read_global(cx).default_model()
            {
                let language_name = assist.editor.upgrade().and_then(|editor| {
                    let multibuffer = editor.read(cx).buffer().read(cx);
                    let multibuffer_snapshot = multibuffer.snapshot(cx);
                    let ranges = multibuffer_snapshot.range_to_buffer_ranges(assist.range.clone());
                    ranges
                        .first()
                        .and_then(|(buffer, _, _)| buffer.language())
                        .map(|language| language.name())
                });
                report_assistant_event(
                    AssistantEventData {
                        conversation_id: None,
                        kind: AssistantKind::Inline,
                        message_id,
                        phase: if undo {
                            AssistantPhase::Rejected
                        } else {
                            AssistantPhase::Accepted
                        },
                        model: model.telemetry_id(),
                        model_provider: model.provider_id().to_string(),
                        response_latency: None,
                        error_message: None,
                        language_name: language_name.map(|name| name.to_proto()),
                    },
                    Some(self.telemetry.clone()),
                    cx.http_client(),
                    model.api_key(cx),
                    cx.background_executor(),
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
                .map_or(false, |lock| lock.assist_id == assist_id)
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
            .update(cx, |editor, cx| window.focus(&editor.focus_handle(cx)))
            .ok();
    }

    fn focus_assist(&mut self, assist_id: InlineAssistId, window: &mut Window, cx: &mut App) {
        let Some(assist) = self.assists.get(&assist_id) else {
            return;
        };

        if let Some(decorations) = assist.decorations.as_ref() {
            decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                prompt_editor.editor.update(cx, |editor, cx| {
                    window.focus(&editor.focus_handle(cx));
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
            editor.change_selections(None, window, cx, |selections| {
                selections.select_anchor_ranges([position..position])
            });

            let mut scroll_target_top;
            let mut scroll_target_bottom;
            if let Some(decorations) = assist.decorations.as_ref() {
                scroll_target_top = editor
                    .row_for_block(decorations.prompt_block_id, cx)
                    .unwrap()
                    .0 as f32;
                scroll_target_bottom = editor
                    .row_for_block(decorations.end_block_id, cx)
                    .unwrap()
                    .0 as f32;
            } else {
                let snapshot = editor.snapshot(window, cx);
                let start_row = assist
                    .range
                    .start
                    .to_display_point(&snapshot.display_snapshot)
                    .row();
                scroll_target_top = start_row.0 as f32;
                scroll_target_bottom = scroll_target_top + 1.;
            }
            scroll_target_top -= editor.vertical_scroll_margin() as f32;
            scroll_target_bottom += editor.vertical_scroll_margin() as f32;

            let height_in_lines = editor.visible_line_count().unwrap_or(0.);
            let scroll_top = editor.scroll_position(cx).y;
            let scroll_bottom = scroll_top + height_in_lines;

            if scroll_target_top < scroll_top {
                editor.set_scroll_position(point(0., scroll_target_top), window, cx);
            } else if scroll_target_bottom > scroll_bottom {
                if (scroll_target_bottom - scroll_target_top) <= height_in_lines {
                    editor.set_scroll_position(
                        point(0., scroll_target_bottom - height_in_lines),
                        window,
                        cx,
                    );
                } else {
                    editor.set_scroll_position(point(0., scroll_target_top), window, cx);
                }
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

        let Some(user_prompt) = assist.user_prompt(cx) else {
            return;
        };

        self.prompt_history.retain(|prompt| *prompt != user_prompt);
        self.prompt_history.push_back(user_prompt.clone());
        if self.prompt_history.len() > PROMPT_HISTORY_MAX_LEN {
            self.prompt_history.pop_front();
        }

        let assistant_panel_context = assist.assistant_panel_context(cx);

        assist
            .codegen
            .update(cx, |codegen, cx| {
                codegen.start(user_prompt, assistant_panel_context, cx)
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
                    &gutter_pending_ranges,
                    |cx| cx.theme().status().info_background,
                    cx,
                )
            }

            enum GutterTransformedRange {}
            if gutter_transformed_ranges.is_empty() {
                editor.clear_gutter_highlights::<GutterTransformedRange>(cx);
            } else {
                editor.highlight_gutter::<GutterTransformedRange>(
                    &gutter_transformed_ranges,
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
                    false,
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
                            Some(ExcerptRange::new(buffer_start..buffer_end)),
                            cx,
                        );
                    });

                    enum DeletedLines {}
                    let mut editor = Editor::for_multibuffer(multi_buffer, None, window, cx);
                    editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
                    editor.set_show_wrap_guides(false, cx);
                    editor.set_show_gutter(false, cx);
                    editor.scroll_manager.set_forbid_vertical_scroll(true);
                    editor.set_show_scrollbars(false, cx);
                    editor.set_read_only(true);
                    editor.set_show_edit_predictions(Some(false), window, cx);
                    editor.highlight_rows::<DeletedLines>(
                        Anchor::min()..Anchor::max(),
                        cx.theme().status().deleted_background,
                        false,
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
                            .block_mouse_down()
                            .bg(cx.theme().status().deleted_background)
                            .size_full()
                            .h(height as f32 * cx.window.line_height())
                            .pl(cx.gutter_dimensions.full_width())
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
}

struct EditorInlineAssists {
    assist_ids: Vec<InlineAssistId>,
    scroll_lock: Option<InlineAssistScrollLock>,
    highlight_updates: async_watch::Sender<()>,
    _update_highlights: Task<Result<()>>,
    _subscriptions: Vec<gpui::Subscription>,
}

struct InlineAssistScrollLock {
    assist_id: InlineAssistId,
    distance_from_top: f32,
}

impl EditorInlineAssists {
    fn new(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) -> Self {
        let (highlight_updates_tx, mut highlight_updates_rx) = async_watch::channel(());
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

fn build_assist_editor_renderer(editor: &Entity<PromptEditor>) -> RenderBlock {
    let editor = editor.clone();
    Arc::new(move |cx: &mut BlockContext| {
        *editor.read(cx).gutter_dimensions.lock() = *cx.gutter_dimensions;
        editor.clone().into_any_element()
    })
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct InlineAssistId(usize);

impl InlineAssistId {
    fn post_inc(&mut self) -> InlineAssistId {
        let id = *self;
        self.0 += 1;
        id
    }
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

enum PromptEditorEvent {
    StartRequested,
    StopRequested,
    ConfirmRequested,
    CancelRequested,
    DismissRequested,
}

struct PromptEditor {
    id: InlineAssistId,
    editor: Entity<Editor>,
    language_model_selector: Entity<LanguageModelSelector>,
    edited_since_done: bool,
    gutter_dimensions: Arc<Mutex<GutterDimensions>>,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    codegen: Entity<Codegen>,
    _codegen_subscription: Subscription,
    editor_subscriptions: Vec<Subscription>,
    pending_token_count: Task<Result<()>>,
    token_counts: Option<TokenCounts>,
    _token_count_subscriptions: Vec<Subscription>,
    workspace: Option<WeakEntity<Workspace>>,
    show_rate_limit_notice: bool,
}

#[derive(Copy, Clone)]
pub struct TokenCounts {
    total: usize,
    assistant_panel: usize,
}

impl EventEmitter<PromptEditorEvent> for PromptEditor {}

impl Render for PromptEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let gutter_dimensions = *self.gutter_dimensions.lock();
        let codegen = self.codegen.read(cx);

        let mut buttons = Vec::new();
        if codegen.alternative_count(cx) > 1 {
            buttons.push(self.render_cycle_controls(cx));
        }

        let status = codegen.status(cx);
        buttons.extend(match status {
            CodegenStatus::Idle => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::for_action("Cancel Assist", &menu::Cancel, window, cx)
                        })
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        )
                        .into_any_element(),
                    IconButton::new("start", IconName::SparkleAlt)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::for_action("Transform", &menu::Confirm, window, cx)
                        })
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::StartRequested)),
                        )
                        .into_any_element(),
                ]
            }
            CodegenStatus::Pending => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(Tooltip::text("Cancel Assist"))
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        )
                        .into_any_element(),
                    IconButton::new("stop", IconName::Stop)
                        .icon_color(Color::Error)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::with_meta(
                                "Interrupt Transformation",
                                Some(&menu::Cancel),
                                "Changes won't be discarded",
                                window,
                                cx,
                            )
                        })
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::StopRequested)),
                        )
                        .into_any_element(),
                ]
            }
            CodegenStatus::Error(_) | CodegenStatus::Done => {
                let must_rerun =
                    self.edited_since_done || matches!(status, CodegenStatus::Error(_));
                // when accept button isn't visible, then restart maps to confirm
                // when accept button is visible, then restart must be mapped to an alternate keyboard shortcut
                let restart_key: &dyn gpui::Action = if must_rerun {
                    &menu::Confirm
                } else {
                    &menu::Restart
                };
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::for_action("Cancel Assist", &menu::Cancel, window, cx)
                        })
                        .on_click(
                            cx.listener(|_, _, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        )
                        .into_any_element(),
                    IconButton::new("restart", IconName::RotateCw)
                        .icon_color(Color::Muted)
                        .shape(IconButtonShape::Square)
                        .tooltip(|window, cx| {
                            Tooltip::with_meta(
                                "Regenerate Transformation",
                                Some(restart_key),
                                "Current change will be discarded",
                                window,
                                cx,
                            )
                        })
                        .on_click(cx.listener(|_, _, _, cx| {
                            cx.emit(PromptEditorEvent::StartRequested);
                        }))
                        .into_any_element(),
                    if !must_rerun {
                        IconButton::new("confirm", IconName::Check)
                            .icon_color(Color::Info)
                            .shape(IconButtonShape::Square)
                            .tooltip(|window, cx| {
                                Tooltip::for_action("Confirm Assist", &menu::Confirm, window, cx)
                            })
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(PromptEditorEvent::ConfirmRequested);
                            }))
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    },
                ]
            }
        });

        h_flex()
            .key_context("PromptEditor")
            .bg(cx.theme().colors().editor_background)
            .block_mouse_down()
            .cursor(CursorStyle::Arrow)
            .border_y_1()
            .border_color(cx.theme().status().info_border)
            .size_full()
            .py(window.line_height() / 2.5)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::restart))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .capture_action(cx.listener(Self::cycle_prev))
            .capture_action(cx.listener(Self::cycle_next))
            .child(
                h_flex()
                    .w(gutter_dimensions.full_width() + (gutter_dimensions.margin / 2.0))
                    .justify_center()
                    .gap_2()
                    .child(LanguageModelSelectorPopoverMenu::new(
                        self.language_model_selector.clone(),
                        IconButton::new("context", IconName::SettingsAlt)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted),
                        move |window, cx| {
                            Tooltip::with_meta(
                                format!(
                                    "Using {}",
                                    LanguageModelRegistry::read_global(cx)
                                        .default_model()
                                        .map(|default| default.model.name().0)
                                        .unwrap_or_else(|| "No model selected".into()),
                                ),
                                None,
                                "Change Model",
                                window,
                                cx,
                            )
                        },
                        gpui::Corner::TopRight,
                    ))
                    .map(|el| {
                        let CodegenStatus::Error(error) = self.codegen.read(cx).status(cx) else {
                            return el;
                        };

                        let error_message = SharedString::from(error.to_string());
                        if error.error_code() == proto::ErrorCode::RateLimitExceeded
                            && cx.has_flag::<ZedPro>()
                        {
                            el.child(
                                v_flex()
                                    .child(
                                        IconButton::new("rate-limit-error", IconName::XCircle)
                                            .toggle_state(self.show_rate_limit_notice)
                                            .shape(IconButtonShape::Square)
                                            .icon_size(IconSize::Small)
                                            .on_click(cx.listener(Self::toggle_rate_limit_notice)),
                                    )
                                    .children(self.show_rate_limit_notice.then(|| {
                                        deferred(
                                            anchored()
                                                .position_mode(gpui::AnchoredPositionMode::Local)
                                                .position(point(px(0.), px(24.)))
                                                .anchor(gpui::Corner::TopLeft)
                                                .child(self.render_rate_limit_notice(cx)),
                                        )
                                    })),
                            )
                        } else {
                            el.child(
                                div()
                                    .id("error")
                                    .tooltip(Tooltip::text(error_message))
                                    .child(
                                        Icon::new(IconName::XCircle)
                                            .size(IconSize::Small)
                                            .color(Color::Error),
                                    ),
                            )
                        }
                    }),
            )
            .child(div().flex_1().child(self.render_prompt_editor(cx)))
            .child(
                h_flex()
                    .gap_2()
                    .pr_6()
                    .children(self.render_token_count(cx))
                    .children(buttons),
            )
    }
}

impl Focusable for PromptEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl PromptEditor {
    const MAX_LINES: u8 = 8;

    fn new(
        id: InlineAssistId,
        gutter_dimensions: Arc<Mutex<GutterDimensions>>,
        prompt_history: VecDeque<String>,
        prompt_buffer: Entity<MultiBuffer>,
        codegen: Entity<Codegen>,
        parent_editor: &Entity<Editor>,
        assistant_panel: Option<&Entity<AssistantPanel>>,
        workspace: Option<WeakEntity<Workspace>>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let prompt_editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    max_lines: Self::MAX_LINES as usize,
                },
                prompt_buffer,
                None,
                window,
                cx,
            );
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            // Since the prompt editors for all inline assistants are linked,
            // always show the cursor (even when it isn't focused) because
            // typing in one will make what you typed appear in all of them.
            editor.set_show_cursor_when_unfocused(true, cx);
            editor.set_placeholder_text(Self::placeholder_text(codegen.read(cx), window, cx), cx);
            editor
        });

        let mut token_count_subscriptions = Vec::new();
        token_count_subscriptions.push(cx.subscribe_in(
            parent_editor,
            window,
            Self::handle_parent_editor_event,
        ));
        if let Some(assistant_panel) = assistant_panel {
            token_count_subscriptions.push(cx.subscribe_in(
                assistant_panel,
                window,
                Self::handle_assistant_panel_event,
            ));
        }

        let mut this = Self {
            id,
            editor: prompt_editor,
            language_model_selector: cx.new(|cx| {
                let fs = fs.clone();
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
            edited_since_done: false,
            gutter_dimensions,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: cx.observe(&codegen, Self::handle_codegen_changed),
            editor_subscriptions: Vec::new(),
            codegen,
            pending_token_count: Task::ready(Ok(())),
            token_counts: None,
            _token_count_subscriptions: token_count_subscriptions,
            workspace,
            show_rate_limit_notice: false,
        };
        this.count_tokens(cx);
        this.subscribe_to_editor(window, cx);
        this
    }

    fn subscribe_to_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor_subscriptions.clear();
        self.editor_subscriptions.push(cx.subscribe_in(
            &self.editor,
            window,
            Self::handle_prompt_editor_events,
        ));
    }

    fn set_show_cursor_when_unfocused(
        &mut self,
        show_cursor_when_unfocused: bool,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_show_cursor_when_unfocused(show_cursor_when_unfocused, cx)
        });
    }

    fn unlink(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let prompt = self.prompt(cx);
        let focus = self.editor.focus_handle(cx).contains_focused(window, cx);
        self.editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(Self::MAX_LINES as usize, window, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text(
                Self::placeholder_text(self.codegen.read(cx), window, cx),
                cx,
            );
            editor.set_placeholder_text("Add a prompt…", cx);
            editor.set_text(prompt, window, cx);
            if focus {
                window.focus(&editor.focus_handle(cx));
            }
            editor
        });
        self.subscribe_to_editor(window, cx);
    }

    fn placeholder_text(codegen: &Codegen, window: &Window, cx: &App) -> String {
        let context_keybinding = text_for_action(&zed_actions::assistant::ToggleFocus, window, cx)
            .map(|keybinding| format!(" • {keybinding} for context"))
            .unwrap_or_default();

        let action = if codegen.is_insertion {
            "Generate"
        } else {
            "Transform"
        };

        format!("{action}…{context_keybinding} • ↓↑ for history")
    }

    fn prompt(&self, cx: &App) -> String {
        self.editor.read(cx).text(cx)
    }

    fn toggle_rate_limit_notice(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_rate_limit_notice = !self.show_rate_limit_notice;
        if self.show_rate_limit_notice {
            window.focus(&self.editor.focus_handle(cx));
        }
        cx.notify();
    }

    fn handle_parent_editor_event(
        &mut self,
        _: &Entity<Editor>,
        event: &EditorEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let EditorEvent::BufferEdited { .. } = event {
            self.count_tokens(cx);
        }
    }

    fn handle_assistant_panel_event(
        &mut self,
        _: &Entity<AssistantPanel>,
        event: &AssistantPanelEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let AssistantPanelEvent::ContextEdited { .. } = event;
        self.count_tokens(cx);
    }

    fn count_tokens(&mut self, cx: &mut Context<Self>) {
        let assist_id = self.id;
        self.pending_token_count = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let token_count = cx
                .update_global(|inline_assistant: &mut InlineAssistant, cx| {
                    let assist = inline_assistant
                        .assists
                        .get(&assist_id)
                        .context("assist not found")?;
                    anyhow::Ok(assist.count_tokens(cx))
                })??
                .await?;

            this.update(cx, |this, cx| {
                this.token_counts = Some(token_count);
                cx.notify();
            })
        })
    }

    fn handle_prompt_editor_events(
        &mut self,
        _: &Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditorEvent::Edited { .. } => {
                if let Some(workspace) = window.root::<Workspace>().flatten() {
                    workspace.update(cx, |workspace, cx| {
                        let is_via_ssh = workspace
                            .project()
                            .update(cx, |project, _| project.is_via_ssh());

                        workspace
                            .client()
                            .telemetry()
                            .log_edit_event("inline assist", is_via_ssh);
                    });
                }
                let prompt = self.editor.read(cx).text(cx);
                if self
                    .prompt_history_ix
                    .map_or(true, |ix| self.prompt_history[ix] != prompt)
                {
                    self.prompt_history_ix.take();
                    self.pending_prompt = prompt;
                }

                self.edited_since_done = true;
                cx.notify();
            }
            EditorEvent::BufferEdited => {
                self.count_tokens(cx);
            }
            EditorEvent::Blurred => {
                if self.show_rate_limit_notice {
                    self.show_rate_limit_notice = false;
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn handle_codegen_changed(&mut self, _: Entity<Codegen>, cx: &mut Context<Self>) {
        match self.codegen.read(cx).status(cx) {
            CodegenStatus::Idle => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Pending => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(true));
            }
            CodegenStatus::Done => {
                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Error(error) => {
                if cx.has_flag::<ZedPro>()
                    && error.error_code() == proto::ErrorCode::RateLimitExceeded
                    && !dismissed_rate_limit_notice()
                {
                    self.show_rate_limit_notice = true;
                    cx.notify();
                }

                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
        }
    }

    fn restart(&mut self, _: &menu::Restart, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PromptEditorEvent::StartRequested);
    }

    fn cancel(
        &mut self,
        _: &editor::actions::Cancel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.codegen.read(cx).status(cx) {
            CodegenStatus::Idle | CodegenStatus::Done | CodegenStatus::Error(_) => {
                cx.emit(PromptEditorEvent::CancelRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::StopRequested);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        match self.codegen.read(cx).status(cx) {
            CodegenStatus::Idle => {
                cx.emit(PromptEditorEvent::StartRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::DismissRequested);
            }
            CodegenStatus::Done => {
                if self.edited_since_done {
                    cx.emit(PromptEditorEvent::StartRequested);
                } else {
                    cx.emit(PromptEditorEvent::ConfirmRequested);
                }
            }
            CodegenStatus::Error(_) => {
                cx.emit(PromptEditorEvent::StartRequested);
            }
        }
    }

    fn move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix > 0 {
                self.prompt_history_ix = Some(ix - 1);
                let prompt = self.prompt_history[ix - 1].as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, window, cx);
                    editor.move_to_beginning(&Default::default(), window, cx);
                });
            }
        } else if !self.prompt_history.is_empty() {
            self.prompt_history_ix = Some(self.prompt_history.len() - 1);
            let prompt = self.prompt_history[self.prompt_history.len() - 1].as_str();
            self.editor.update(cx, |editor, cx| {
                editor.set_text(prompt, window, cx);
                editor.move_to_beginning(&Default::default(), window, cx);
            });
        }
    }

    fn move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix < self.prompt_history.len() - 1 {
                self.prompt_history_ix = Some(ix + 1);
                let prompt = self.prompt_history[ix + 1].as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, window, cx);
                    editor.move_to_end(&Default::default(), window, cx)
                });
            } else {
                self.prompt_history_ix = None;
                let prompt = self.pending_prompt.as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, window, cx);
                    editor.move_to_end(&Default::default(), window, cx)
                });
            }
        }
    }

    fn cycle_prev(
        &mut self,
        _: &CyclePreviousInlineAssist,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.codegen
            .update(cx, |codegen, cx| codegen.cycle_prev(cx));
    }

    fn cycle_next(&mut self, _: &CycleNextInlineAssist, _: &mut Window, cx: &mut Context<Self>) {
        self.codegen
            .update(cx, |codegen, cx| codegen.cycle_next(cx));
    }

    fn render_cycle_controls(&self, cx: &Context<Self>) -> AnyElement {
        let codegen = self.codegen.read(cx);
        let disabled = matches!(codegen.status(cx), CodegenStatus::Idle);

        let model_registry = LanguageModelRegistry::read_global(cx);
        let default_model = model_registry.default_model().map(|default| default.model);
        let alternative_models = model_registry.inline_alternative_models();

        let get_model_name = |index: usize| -> String {
            let name = |model: &Arc<dyn LanguageModel>| model.name().0.to_string();

            match index {
                0 => default_model.as_ref().map_or_else(String::new, name),
                index if index <= alternative_models.len() => alternative_models
                    .get(index - 1)
                    .map_or_else(String::new, name),
                _ => String::new(),
            }
        };

        let total_models = alternative_models.len() + 1;

        if total_models <= 1 {
            return div().into_any_element();
        }

        let current_index = codegen.active_alternative;
        let prev_index = (current_index + total_models - 1) % total_models;
        let next_index = (current_index + 1) % total_models;

        let prev_model_name = get_model_name(prev_index);
        let next_model_name = get_model_name(next_index);

        h_flex()
            .child(
                IconButton::new("previous", IconName::ChevronLeft)
                    .icon_color(Color::Muted)
                    .disabled(disabled || current_index == 0)
                    .shape(IconButtonShape::Square)
                    .tooltip({
                        let focus_handle = self.editor.focus_handle(cx);
                        move |window, cx| {
                            cx.new(|cx| {
                                let mut tooltip = Tooltip::new("Previous Alternative").key_binding(
                                    KeyBinding::for_action_in(
                                        &CyclePreviousInlineAssist,
                                        &focus_handle,
                                        window,
                                        cx,
                                    ),
                                );
                                if !disabled && current_index != 0 {
                                    tooltip = tooltip.meta(prev_model_name.clone());
                                }
                                tooltip
                            })
                            .into()
                        }
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.codegen
                            .update(cx, |codegen, cx| codegen.cycle_prev(cx))
                    })),
            )
            .child(
                Label::new(format!(
                    "{}/{}",
                    codegen.active_alternative + 1,
                    codegen.alternative_count(cx)
                ))
                .size(LabelSize::Small)
                .color(if disabled {
                    Color::Disabled
                } else {
                    Color::Muted
                }),
            )
            .child(
                IconButton::new("next", IconName::ChevronRight)
                    .icon_color(Color::Muted)
                    .disabled(disabled || current_index == total_models - 1)
                    .shape(IconButtonShape::Square)
                    .tooltip({
                        let focus_handle = self.editor.focus_handle(cx);
                        move |window, cx| {
                            cx.new(|cx| {
                                let mut tooltip = Tooltip::new("Next Alternative").key_binding(
                                    KeyBinding::for_action_in(
                                        &CycleNextInlineAssist,
                                        &focus_handle,
                                        window,
                                        cx,
                                    ),
                                );
                                if !disabled && current_index != total_models - 1 {
                                    tooltip = tooltip.meta(next_model_name.clone());
                                }
                                tooltip
                            })
                            .into()
                        }
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.codegen
                            .update(cx, |codegen, cx| codegen.cycle_next(cx))
                    })),
            )
            .into_any_element()
    }

    fn render_token_count(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let model = LanguageModelRegistry::read_global(cx)
            .default_model()?
            .model;
        let token_counts = self.token_counts?;
        let max_token_count = model.max_token_count();

        let remaining_tokens = max_token_count as isize - token_counts.total as isize;
        let token_count_color = if remaining_tokens <= 0 {
            Color::Error
        } else if token_counts.total as f32 / max_token_count as f32 >= 0.8 {
            Color::Warning
        } else {
            Color::Muted
        };

        let mut token_count = h_flex()
            .id("token_count")
            .gap_0p5()
            .child(
                Label::new(humanize_token_count(token_counts.total))
                    .size(LabelSize::Small)
                    .color(token_count_color),
            )
            .child(Label::new("/").size(LabelSize::Small).color(Color::Muted))
            .child(
                Label::new(humanize_token_count(max_token_count))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        if let Some(workspace) = self.workspace.clone() {
            token_count = token_count
                .tooltip(move |window, cx| {
                    Tooltip::with_meta(
                        format!(
                            "Tokens Used ({} from the Assistant Panel)",
                            humanize_token_count(token_counts.assistant_panel)
                        ),
                        None,
                        "Click to open the Assistant Panel",
                        window,
                        cx,
                    )
                })
                .cursor_pointer()
                .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_click(move |_, window, cx| {
                    cx.stop_propagation();
                    workspace
                        .update(cx, |workspace, cx| {
                            workspace.focus_panel::<AssistantPanel>(window, cx)
                        })
                        .ok();
                });
        } else {
            token_count = token_count
                .cursor_default()
                .tooltip(Tooltip::text("Tokens used"));
        }

        Some(token_count)
    }

    fn render_prompt_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(settings.buffer_line_height.value()),
            ..Default::default()
        };
        EditorElement::new(
            &self.editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_rate_limit_notice(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Popover::new().child(
            v_flex()
                .occlude()
                .p_2()
                .child(
                    Label::new("Out of Tokens")
                        .size(LabelSize::Small)
                        .weight(FontWeight::BOLD),
                )
                .child(Label::new(
                    "Try Zed Pro for higher limits, a wider range of models, and more.",
                ))
                .child(
                    h_flex()
                        .justify_between()
                        .child(CheckboxWithLabel::new(
                            "dont-show-again",
                            Label::new("Don't show again"),
                            if dismissed_rate_limit_notice() {
                                ui::ToggleState::Selected
                            } else {
                                ui::ToggleState::Unselected
                            },
                            |selection, _, cx| {
                                let is_dismissed = match selection {
                                    ui::ToggleState::Unselected => false,
                                    ui::ToggleState::Indeterminate => return,
                                    ui::ToggleState::Selected => true,
                                };

                                set_rate_limit_notice_dismissed(is_dismissed, cx)
                            },
                        ))
                        .child(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("dismiss", "Dismiss")
                                        .style(ButtonStyle::Transparent)
                                        .on_click(cx.listener(Self::toggle_rate_limit_notice)),
                                )
                                .child(Button::new("more-info", "More Info").on_click(
                                    |_event, window, cx| {
                                        window.dispatch_action(
                                            Box::new(zed_actions::OpenAccountSettings),
                                            cx,
                                        )
                                    },
                                )),
                        ),
                ),
        )
    }
}

const DISMISSED_RATE_LIMIT_NOTICE_KEY: &str = "dismissed-rate-limit-notice";

fn dismissed_rate_limit_notice() -> bool {
    db::kvp::KEY_VALUE_STORE
        .read_kvp(DISMISSED_RATE_LIMIT_NOTICE_KEY)
        .log_err()
        .map_or(false, |s| s.is_some())
}

fn set_rate_limit_notice_dismissed(is_dismissed: bool, cx: &mut App) {
    db::write_and_log(cx, move || async move {
        if is_dismissed {
            db::kvp::KEY_VALUE_STORE
                .write_kvp(DISMISSED_RATE_LIMIT_NOTICE_KEY.into(), "1".into())
                .await
        } else {
            db::kvp::KEY_VALUE_STORE
                .delete_kvp(DISMISSED_RATE_LIMIT_NOTICE_KEY.into())
                .await
        }
    })
}

struct InlineAssist {
    group_id: InlineAssistGroupId,
    range: Range<Anchor>,
    editor: WeakEntity<Editor>,
    decorations: Option<InlineAssistDecorations>,
    codegen: Entity<Codegen>,
    _subscriptions: Vec<Subscription>,
    workspace: Option<WeakEntity<Workspace>>,
    include_context: bool,
}

impl InlineAssist {
    fn new(
        assist_id: InlineAssistId,
        group_id: InlineAssistGroupId,
        include_context: bool,
        editor: &Entity<Editor>,
        prompt_editor: &Entity<PromptEditor>,
        prompt_block_id: CustomBlockId,
        end_block_id: CustomBlockId,
        range: Range<Anchor>,
        codegen: Entity<Codegen>,
        workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let prompt_editor_focus_handle = prompt_editor.focus_handle(cx);
        InlineAssist {
            group_id,
            include_context,
            editor: editor.downgrade(),
            decorations: Some(InlineAssistDecorations {
                prompt_block_id,
                prompt_editor: prompt_editor.clone(),
                removed_line_block_ids: HashSet::default(),
                end_block_id,
            }),
            range,
            codegen: codegen.clone(),
            workspace: workspace.clone(),
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
                window.subscribe(
                    prompt_editor,
                    cx,
                    move |prompt_editor, event, window, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            this.handle_prompt_editor_event(prompt_editor, event, window, cx)
                        })
                    },
                ),
                window.observe(&codegen, cx, {
                    let editor = editor.downgrade();
                    move |_, window, cx| {
                        if let Some(editor) = editor.upgrade() {
                            InlineAssistant::update_global(cx, |this, cx| {
                                if let Some(editor_assists) =
                                    this.assists_by_editor.get(&editor.downgrade())
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

                            if let CodegenStatus::Error(error) = codegen.read(cx).status(cx) {
                                if assist.decorations.is_none() {
                                    if let Some(workspace) = assist
                                        .workspace
                                        .as_ref()
                                        .and_then(|workspace| workspace.upgrade())
                                    {
                                        let error = format!("Inline assistant error: {}", error);
                                        workspace.update(cx, |workspace, cx| {
                                            struct InlineAssistantError;

                                            let id =
                                                NotificationId::composite::<InlineAssistantError>(
                                                    assist_id.0,
                                                );

                                            workspace.show_toast(Toast::new(id, error), cx);
                                        })
                                    }
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

    fn assistant_panel_context(&self, cx: &mut App) -> Option<LanguageModelRequest> {
        if self.include_context {
            let workspace = self.workspace.as_ref()?;
            let workspace = workspace.upgrade()?.read(cx);
            let assistant_panel = workspace.panel::<AssistantPanel>(cx)?;
            Some(
                assistant_panel
                    .read(cx)
                    .active_context(cx)?
                    .read(cx)
                    .to_completion_request(RequestType::Chat, cx),
            )
        } else {
            None
        }
    }

    pub fn count_tokens(&self, cx: &mut App) -> BoxFuture<'static, Result<TokenCounts>> {
        let Some(user_prompt) = self.user_prompt(cx) else {
            return future::ready(Err(anyhow!("no user prompt"))).boxed();
        };
        let assistant_panel_context = self.assistant_panel_context(cx);
        self.codegen
            .read(cx)
            .count_tokens(user_prompt, assistant_panel_context, cx)
    }
}

struct InlineAssistDecorations {
    prompt_block_id: CustomBlockId,
    prompt_editor: Entity<PromptEditor>,
    removed_line_block_ids: HashSet<CustomBlockId>,
    end_block_id: CustomBlockId,
}

#[derive(Copy, Clone, Debug)]
pub enum CodegenEvent {
    Finished,
    Undone,
}

pub struct Codegen {
    alternatives: Vec<Entity<CodegenAlternative>>,
    active_alternative: usize,
    seen_alternatives: HashSet<usize>,
    subscriptions: Vec<Subscription>,
    buffer: Entity<MultiBuffer>,
    range: Range<Anchor>,
    initial_transaction_id: Option<TransactionId>,
    telemetry: Arc<Telemetry>,
    builder: Arc<PromptBuilder>,
    is_insertion: bool,
}

impl Codegen {
    pub fn new(
        buffer: Entity<MultiBuffer>,
        range: Range<Anchor>,
        initial_transaction_id: Option<TransactionId>,
        telemetry: Arc<Telemetry>,
        builder: Arc<PromptBuilder>,
        cx: &mut Context<Self>,
    ) -> Self {
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                false,
                Some(telemetry.clone()),
                builder.clone(),
                cx,
            )
        });
        let mut this = Self {
            is_insertion: range.to_offset(&buffer.read(cx).snapshot(cx)).is_empty(),
            alternatives: vec![codegen],
            active_alternative: 0,
            seen_alternatives: HashSet::default(),
            subscriptions: Vec::new(),
            buffer,
            range,
            initial_transaction_id,
            telemetry,
            builder,
        };
        this.activate(0, cx);
        this
    }

    fn subscribe_to_alternative(&mut self, cx: &mut Context<Self>) {
        let codegen = self.active_alternative().clone();
        self.subscriptions.clear();
        self.subscriptions
            .push(cx.observe(&codegen, |_, _, cx| cx.notify()));
        self.subscriptions
            .push(cx.subscribe(&codegen, |_, _, event, cx| cx.emit(*event)));
    }

    fn active_alternative(&self) -> &Entity<CodegenAlternative> {
        &self.alternatives[self.active_alternative]
    }

    fn status<'a>(&self, cx: &'a App) -> &'a CodegenStatus {
        &self.active_alternative().read(cx).status
    }

    fn alternative_count(&self, cx: &App) -> usize {
        LanguageModelRegistry::read_global(cx)
            .inline_alternative_models()
            .len()
            + 1
    }

    pub fn cycle_prev(&mut self, cx: &mut Context<Self>) {
        let next_active_ix = if self.active_alternative == 0 {
            self.alternatives.len() - 1
        } else {
            self.active_alternative - 1
        };
        self.activate(next_active_ix, cx);
    }

    pub fn cycle_next(&mut self, cx: &mut Context<Self>) {
        let next_active_ix = (self.active_alternative + 1) % self.alternatives.len();
        self.activate(next_active_ix, cx);
    }

    fn activate(&mut self, index: usize, cx: &mut Context<Self>) {
        self.active_alternative()
            .update(cx, |codegen, cx| codegen.set_active(false, cx));
        self.seen_alternatives.insert(index);
        self.active_alternative = index;
        self.active_alternative()
            .update(cx, |codegen, cx| codegen.set_active(true, cx));
        self.subscribe_to_alternative(cx);
        cx.notify();
    }

    pub fn start(
        &mut self,
        user_prompt: String,
        assistant_panel_context: Option<LanguageModelRequest>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let alternative_models = LanguageModelRegistry::read_global(cx)
            .inline_alternative_models()
            .to_vec();

        self.active_alternative()
            .update(cx, |alternative, cx| alternative.undo(cx));
        self.activate(0, cx);
        self.alternatives.truncate(1);

        for _ in 0..alternative_models.len() {
            self.alternatives.push(cx.new(|cx| {
                CodegenAlternative::new(
                    self.buffer.clone(),
                    self.range.clone(),
                    false,
                    Some(self.telemetry.clone()),
                    self.builder.clone(),
                    cx,
                )
            }));
        }

        let primary_model = LanguageModelRegistry::read_global(cx)
            .default_model()
            .context("no active model")?
            .model;

        for (model, alternative) in iter::once(primary_model)
            .chain(alternative_models)
            .zip(&self.alternatives)
        {
            alternative.update(cx, |alternative, cx| {
                alternative.start(
                    user_prompt.clone(),
                    assistant_panel_context.clone(),
                    model.clone(),
                    cx,
                )
            })?;
        }

        Ok(())
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        for codegen in &self.alternatives {
            codegen.update(cx, |codegen, cx| codegen.stop(cx));
        }
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        self.active_alternative()
            .update(cx, |codegen, cx| codegen.undo(cx));

        self.buffer.update(cx, |buffer, cx| {
            if let Some(transaction_id) = self.initial_transaction_id.take() {
                buffer.undo_transaction(transaction_id, cx);
                buffer.refresh_preview(cx);
            }
        });
    }

    pub fn count_tokens(
        &self,
        user_prompt: String,
        assistant_panel_context: Option<LanguageModelRequest>,
        cx: &App,
    ) -> BoxFuture<'static, Result<TokenCounts>> {
        self.active_alternative()
            .read(cx)
            .count_tokens(user_prompt, assistant_panel_context, cx)
    }

    pub fn buffer(&self, cx: &App) -> Entity<MultiBuffer> {
        self.active_alternative().read(cx).buffer.clone()
    }

    pub fn old_buffer(&self, cx: &App) -> Entity<Buffer> {
        self.active_alternative().read(cx).old_buffer.clone()
    }

    pub fn snapshot(&self, cx: &App) -> MultiBufferSnapshot {
        self.active_alternative().read(cx).snapshot.clone()
    }

    pub fn edit_position(&self, cx: &App) -> Option<Anchor> {
        self.active_alternative().read(cx).edit_position
    }

    fn diff<'a>(&self, cx: &'a App) -> &'a Diff {
        &self.active_alternative().read(cx).diff
    }

    pub fn last_equal_ranges<'a>(&self, cx: &'a App) -> &'a [Range<Anchor>] {
        self.active_alternative().read(cx).last_equal_ranges()
    }
}

impl EventEmitter<CodegenEvent> for Codegen {}

pub struct CodegenAlternative {
    buffer: Entity<MultiBuffer>,
    old_buffer: Entity<Buffer>,
    snapshot: MultiBufferSnapshot,
    edit_position: Option<Anchor>,
    range: Range<Anchor>,
    last_equal_ranges: Vec<Range<Anchor>>,
    transformation_transaction_id: Option<TransactionId>,
    status: CodegenStatus,
    generation: Task<()>,
    diff: Diff,
    telemetry: Option<Arc<Telemetry>>,
    _subscription: gpui::Subscription,
    builder: Arc<PromptBuilder>,
    active: bool,
    edits: Vec<(Range<Anchor>, String)>,
    line_operations: Vec<LineOperation>,
    request: Option<LanguageModelRequest>,
    elapsed_time: Option<f64>,
    completion: Option<String>,
    message_id: Option<String>,
}

enum CodegenStatus {
    Idle,
    Pending,
    Done,
    Error(anyhow::Error),
}

#[derive(Default)]
struct Diff {
    deleted_row_ranges: Vec<(Anchor, RangeInclusive<u32>)>,
    inserted_row_ranges: Vec<Range<Anchor>>,
}

impl Diff {
    fn is_empty(&self) -> bool {
        self.deleted_row_ranges.is_empty() && self.inserted_row_ranges.is_empty()
    }
}

impl EventEmitter<CodegenEvent> for CodegenAlternative {}

impl CodegenAlternative {
    pub fn new(
        multi_buffer: Entity<MultiBuffer>,
        range: Range<Anchor>,
        active: bool,
        telemetry: Option<Arc<Telemetry>>,
        builder: Arc<PromptBuilder>,
        cx: &mut Context<Self>,
    ) -> Self {
        let snapshot = multi_buffer.read(cx).snapshot(cx);

        let (buffer, _, _) = snapshot
            .range_to_buffer_ranges(range.clone())
            .pop()
            .unwrap();
        let old_buffer = cx.new(|cx| {
            let text = buffer.as_rope().clone();
            let line_ending = buffer.line_ending();
            let language = buffer.language().cloned();
            let language_registry = multi_buffer
                .read(cx)
                .buffer(buffer.remote_id())
                .unwrap()
                .read(cx)
                .language_registry();

            let mut buffer = Buffer::local_normalized(text, line_ending, cx);
            buffer.set_language(language, cx);
            if let Some(language_registry) = language_registry {
                buffer.set_language_registry(language_registry)
            }
            buffer
        });

        Self {
            buffer: multi_buffer.clone(),
            old_buffer,
            edit_position: None,
            message_id: None,
            snapshot,
            last_equal_ranges: Default::default(),
            transformation_transaction_id: None,
            status: CodegenStatus::Idle,
            generation: Task::ready(()),
            diff: Diff::default(),
            telemetry,
            _subscription: cx.subscribe(&multi_buffer, Self::handle_buffer_event),
            builder,
            active,
            edits: Vec::new(),
            line_operations: Vec::new(),
            range,
            request: None,
            elapsed_time: None,
            completion: None,
        }
    }

    fn set_active(&mut self, active: bool, cx: &mut Context<Self>) {
        if active != self.active {
            self.active = active;

            if self.active {
                let edits = self.edits.clone();
                self.apply_edits(edits, cx);
                if matches!(self.status, CodegenStatus::Pending) {
                    let line_operations = self.line_operations.clone();
                    self.reapply_line_based_diff(line_operations, cx);
                } else {
                    self.reapply_batch_diff(cx).detach();
                }
            } else if let Some(transaction_id) = self.transformation_transaction_id.take() {
                self.buffer.update(cx, |buffer, cx| {
                    buffer.undo_transaction(transaction_id, cx);
                    buffer.forget_transaction(transaction_id, cx);
                });
            }
        }
    }

    fn handle_buffer_event(
        &mut self,
        _buffer: Entity<MultiBuffer>,
        event: &multi_buffer::Event,
        cx: &mut Context<Self>,
    ) {
        if let multi_buffer::Event::TransactionUndone { transaction_id } = event {
            if self.transformation_transaction_id == Some(*transaction_id) {
                self.transformation_transaction_id = None;
                self.generation = Task::ready(());
                cx.emit(CodegenEvent::Undone);
            }
        }
    }

    pub fn last_equal_ranges(&self) -> &[Range<Anchor>] {
        &self.last_equal_ranges
    }

    pub fn count_tokens(
        &self,
        user_prompt: String,
        assistant_panel_context: Option<LanguageModelRequest>,
        cx: &App,
    ) -> BoxFuture<'static, Result<TokenCounts>> {
        if let Some(ConfiguredModel { model, .. }) =
            LanguageModelRegistry::read_global(cx).inline_assistant_model()
        {
            let request = self.build_request(user_prompt, assistant_panel_context.clone(), cx);
            match request {
                Ok(request) => {
                    let total_count = model.count_tokens(request.clone(), cx);
                    let assistant_panel_count = assistant_panel_context
                        .map(|context| model.count_tokens(context, cx))
                        .unwrap_or_else(|| future::ready(Ok(0)).boxed());

                    async move {
                        Ok(TokenCounts {
                            total: total_count.await?,
                            assistant_panel: assistant_panel_count.await?,
                        })
                    }
                    .boxed()
                }
                Err(error) => futures::future::ready(Err(error)).boxed(),
            }
        } else {
            future::ready(Err(anyhow!("no active model"))).boxed()
        }
    }

    pub fn start(
        &mut self,
        user_prompt: String,
        assistant_panel_context: Option<LanguageModelRequest>,
        model: Arc<dyn LanguageModel>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let Some(transformation_transaction_id) = self.transformation_transaction_id.take() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.undo_transaction(transformation_transaction_id, cx);
            });
        }

        self.edit_position = Some(self.range.start.bias_right(&self.snapshot));

        let api_key = model.api_key(cx);
        let telemetry_id = model.telemetry_id();
        let provider_id = model.provider_id();
        let stream: LocalBoxFuture<Result<LanguageModelTextStream>> =
            if user_prompt.trim().to_lowercase() == "delete" {
                async { Ok(LanguageModelTextStream::default()) }.boxed_local()
            } else {
                let request = self.build_request(user_prompt, assistant_panel_context, cx)?;
                self.request = Some(request.clone());

                cx.spawn(async move |_, cx| model.stream_completion_text(request, &cx).await)
                    .boxed_local()
            };
        self.handle_stream(telemetry_id, provider_id.to_string(), api_key, stream, cx);
        Ok(())
    }

    fn build_request(
        &self,
        user_prompt: String,
        assistant_panel_context: Option<LanguageModelRequest>,
        cx: &App,
    ) -> Result<LanguageModelRequest> {
        let buffer = self.buffer.read(cx).snapshot(cx);
        let language = buffer.language_at(self.range.start);
        let language_name = if let Some(language) = language.as_ref() {
            if Arc::ptr_eq(language, &language::PLAIN_TEXT) {
                None
            } else {
                Some(language.name())
            }
        } else {
            None
        };

        let language_name = language_name.as_ref();
        let start = buffer.point_to_buffer_offset(self.range.start);
        let end = buffer.point_to_buffer_offset(self.range.end);
        let (buffer, range) = if let Some((start, end)) = start.zip(end) {
            let (start_buffer, start_buffer_offset) = start;
            let (end_buffer, end_buffer_offset) = end;
            if start_buffer.remote_id() == end_buffer.remote_id() {
                (start_buffer.clone(), start_buffer_offset..end_buffer_offset)
            } else {
                return Err(anyhow::anyhow!("invalid transformation range"));
            }
        } else {
            return Err(anyhow::anyhow!("invalid transformation range"));
        };

        let prompt = self
            .builder
            .generate_inline_transformation_prompt(user_prompt, language_name, buffer, range)
            .map_err(|e| anyhow::anyhow!("Failed to generate content prompt: {}", e))?;

        let mut messages = Vec::new();
        if let Some(context_request) = assistant_panel_context {
            messages = context_request.messages;
        }

        messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![prompt.into()],
            cache: false,
        });

        Ok(LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            mode: None,
            messages,
            tools: Vec::new(),
            stop: Vec::new(),
            temperature: None,
        })
    }

    pub fn handle_stream(
        &mut self,
        model_telemetry_id: String,
        model_provider_id: String,
        model_api_key: Option<String>,
        stream: impl 'static + Future<Output = Result<LanguageModelTextStream>>,
        cx: &mut Context<Self>,
    ) {
        let start_time = Instant::now();
        let snapshot = self.snapshot.clone();
        let selected_text = snapshot
            .text_for_range(self.range.start..self.range.end)
            .collect::<Rope>();

        let selection_start = self.range.start.to_point(&snapshot);

        // Start with the indentation of the first line in the selection
        let mut suggested_line_indent = snapshot
            .suggested_indents(selection_start.row..=selection_start.row, cx)
            .into_values()
            .next()
            .unwrap_or_else(|| snapshot.indent_size_for_line(MultiBufferRow(selection_start.row)));

        // If the first line in the selection does not have indentation, check the following lines
        if suggested_line_indent.len == 0 && suggested_line_indent.kind == IndentKind::Space {
            for row in selection_start.row..=self.range.end.to_point(&snapshot).row {
                let line_indent = snapshot.indent_size_for_line(MultiBufferRow(row));
                // Prefer tabs if a line in the selection uses tabs as indentation
                if line_indent.kind == IndentKind::Tab {
                    suggested_line_indent.kind = IndentKind::Tab;
                    break;
                }
            }
        }

        let http_client = cx.http_client().clone();
        let telemetry = self.telemetry.clone();
        let language_name = {
            let multibuffer = self.buffer.read(cx);
            let snapshot = multibuffer.snapshot(cx);
            let ranges = snapshot.range_to_buffer_ranges(self.range.clone());
            ranges
                .first()
                .and_then(|(buffer, _, _)| buffer.language())
                .map(|language| language.name())
        };

        self.diff = Diff::default();
        self.status = CodegenStatus::Pending;
        let mut edit_start = self.range.start.to_offset(&snapshot);
        let completion = Arc::new(Mutex::new(String::new()));
        let completion_clone = completion.clone();

        self.generation = cx.spawn(async move |codegen, cx| {
            let stream = stream.await;
            let message_id = stream
                .as_ref()
                .ok()
                .and_then(|stream| stream.message_id.clone());
            let generate = async {
                let (mut diff_tx, mut diff_rx) = mpsc::channel(1);
                let executor = cx.background_executor().clone();
                let message_id = message_id.clone();
                let line_based_stream_diff: Task<anyhow::Result<()>> =
                    cx.background_spawn(async move {
                        let mut response_latency = None;
                        let request_start = Instant::now();
                        let diff = async {
                            let chunks =
                                StripInvalidSpans::new(stream?.stream.map_err(|e| e.into()));
                            futures::pin_mut!(chunks);
                            let mut diff = StreamingDiff::new(selected_text.to_string());
                            let mut line_diff = LineDiff::default();

                            let mut new_text = String::new();
                            let mut base_indent = None;
                            let mut line_indent = None;
                            let mut first_line = true;

                            while let Some(chunk) = chunks.next().await {
                                if response_latency.is_none() {
                                    response_latency = Some(request_start.elapsed());
                                }
                                let chunk = chunk?;
                                completion_clone.lock().push_str(&chunk);

                                let mut lines = chunk.split('\n').peekable();
                                while let Some(line) = lines.next() {
                                    new_text.push_str(line);
                                    if line_indent.is_none() {
                                        if let Some(non_whitespace_ch_ix) =
                                            new_text.find(|ch: char| !ch.is_whitespace())
                                        {
                                            line_indent = Some(non_whitespace_ch_ix);
                                            base_indent = base_indent.or(line_indent);

                                            let line_indent = line_indent.unwrap();
                                            let base_indent = base_indent.unwrap();
                                            let indent_delta =
                                                line_indent as i32 - base_indent as i32;
                                            let mut corrected_indent_len = cmp::max(
                                                0,
                                                suggested_line_indent.len as i32 + indent_delta,
                                            )
                                                as usize;
                                            if first_line {
                                                corrected_indent_len = corrected_indent_len
                                                    .saturating_sub(
                                                        selection_start.column as usize,
                                                    );
                                            }

                                            let indent_char = suggested_line_indent.char();
                                            let mut indent_buffer = [0; 4];
                                            let indent_str =
                                                indent_char.encode_utf8(&mut indent_buffer);
                                            new_text.replace_range(
                                                ..line_indent,
                                                &indent_str.repeat(corrected_indent_len),
                                            );
                                        }
                                    }

                                    if line_indent.is_some() {
                                        let char_ops = diff.push_new(&new_text);
                                        line_diff.push_char_operations(&char_ops, &selected_text);
                                        diff_tx
                                            .send((char_ops, line_diff.line_operations()))
                                            .await?;
                                        new_text.clear();
                                    }

                                    if lines.peek().is_some() {
                                        let char_ops = diff.push_new("\n");
                                        line_diff.push_char_operations(&char_ops, &selected_text);
                                        diff_tx
                                            .send((char_ops, line_diff.line_operations()))
                                            .await?;
                                        if line_indent.is_none() {
                                            // Don't write out the leading indentation in empty lines on the next line
                                            // This is the case where the above if statement didn't clear the buffer
                                            new_text.clear();
                                        }
                                        line_indent = None;
                                        first_line = false;
                                    }
                                }
                            }

                            let mut char_ops = diff.push_new(&new_text);
                            char_ops.extend(diff.finish());
                            line_diff.push_char_operations(&char_ops, &selected_text);
                            line_diff.finish(&selected_text);
                            diff_tx
                                .send((char_ops, line_diff.line_operations()))
                                .await?;

                            anyhow::Ok(())
                        };

                        let result = diff.await;

                        let error_message = result.as_ref().err().map(|error| error.to_string());
                        report_assistant_event(
                            AssistantEventData {
                                conversation_id: None,
                                message_id,
                                kind: AssistantKind::Inline,
                                phase: AssistantPhase::Response,
                                model: model_telemetry_id,
                                model_provider: model_provider_id.to_string(),
                                response_latency,
                                error_message,
                                language_name: language_name.map(|name| name.to_proto()),
                            },
                            telemetry,
                            http_client,
                            model_api_key,
                            &executor,
                        );

                        result?;
                        Ok(())
                    });

                while let Some((char_ops, line_ops)) = diff_rx.next().await {
                    codegen.update(cx, |codegen, cx| {
                        codegen.last_equal_ranges.clear();

                        let edits = char_ops
                            .into_iter()
                            .filter_map(|operation| match operation {
                                CharOperation::Insert { text } => {
                                    let edit_start = snapshot.anchor_after(edit_start);
                                    Some((edit_start..edit_start, text))
                                }
                                CharOperation::Delete { bytes } => {
                                    let edit_end = edit_start + bytes;
                                    let edit_range = snapshot.anchor_after(edit_start)
                                        ..snapshot.anchor_before(edit_end);
                                    edit_start = edit_end;
                                    Some((edit_range, String::new()))
                                }
                                CharOperation::Keep { bytes } => {
                                    let edit_end = edit_start + bytes;
                                    let edit_range = snapshot.anchor_after(edit_start)
                                        ..snapshot.anchor_before(edit_end);
                                    edit_start = edit_end;
                                    codegen.last_equal_ranges.push(edit_range);
                                    None
                                }
                            })
                            .collect::<Vec<_>>();

                        if codegen.active {
                            codegen.apply_edits(edits.iter().cloned(), cx);
                            codegen.reapply_line_based_diff(line_ops.iter().cloned(), cx);
                        }
                        codegen.edits.extend(edits);
                        codegen.line_operations = line_ops;
                        codegen.edit_position = Some(snapshot.anchor_after(edit_start));

                        cx.notify();
                    })?;
                }

                // Streaming stopped and we have the new text in the buffer, and a line-based diff applied for the whole new buffer.
                // That diff is not what a regular diff is and might look unexpected, ergo apply a regular diff.
                // It's fine to apply even if the rest of the line diffing fails, as no more hunks are coming through `diff_rx`.
                let batch_diff_task =
                    codegen.update(cx, |codegen, cx| codegen.reapply_batch_diff(cx))?;
                let (line_based_stream_diff, ()) = join!(line_based_stream_diff, batch_diff_task);
                line_based_stream_diff?;

                anyhow::Ok(())
            };

            let result = generate.await;
            let elapsed_time = start_time.elapsed().as_secs_f64();

            codegen
                .update(cx, |this, cx| {
                    this.message_id = message_id;
                    this.last_equal_ranges.clear();
                    if let Err(error) = result {
                        this.status = CodegenStatus::Error(error);
                    } else {
                        this.status = CodegenStatus::Done;
                    }
                    this.elapsed_time = Some(elapsed_time);
                    this.completion = Some(completion.lock().clone());
                    cx.emit(CodegenEvent::Finished);
                    cx.notify();
                })
                .ok();
        });
        cx.notify();
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.last_equal_ranges.clear();
        if self.diff.is_empty() {
            self.status = CodegenStatus::Idle;
        } else {
            self.status = CodegenStatus::Done;
        }
        self.generation = Task::ready(());
        cx.emit(CodegenEvent::Finished);
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        self.buffer.update(cx, |buffer, cx| {
            if let Some(transaction_id) = self.transformation_transaction_id.take() {
                buffer.undo_transaction(transaction_id, cx);
                buffer.refresh_preview(cx);
            }
        });
    }

    fn apply_edits(
        &mut self,
        edits: impl IntoIterator<Item = (Range<Anchor>, String)>,
        cx: &mut Context<CodegenAlternative>,
    ) {
        let transaction = self.buffer.update(cx, |buffer, cx| {
            // Avoid grouping assistant edits with user edits.
            buffer.finalize_last_transaction(cx);
            buffer.start_transaction(cx);
            buffer.edit(edits, None, cx);
            buffer.end_transaction(cx)
        });

        if let Some(transaction) = transaction {
            if let Some(first_transaction) = self.transformation_transaction_id {
                // Group all assistant edits into the first transaction.
                self.buffer.update(cx, |buffer, cx| {
                    buffer.merge_transactions(transaction, first_transaction, cx)
                });
            } else {
                self.transformation_transaction_id = Some(transaction);
                self.buffer
                    .update(cx, |buffer, cx| buffer.finalize_last_transaction(cx));
            }
        }
    }

    fn reapply_line_based_diff(
        &mut self,
        line_operations: impl IntoIterator<Item = LineOperation>,
        cx: &mut Context<Self>,
    ) {
        let old_snapshot = self.snapshot.clone();
        let old_range = self.range.to_point(&old_snapshot);
        let new_snapshot = self.buffer.read(cx).snapshot(cx);
        let new_range = self.range.to_point(&new_snapshot);

        let mut old_row = old_range.start.row;
        let mut new_row = new_range.start.row;

        self.diff.deleted_row_ranges.clear();
        self.diff.inserted_row_ranges.clear();
        for operation in line_operations {
            match operation {
                LineOperation::Keep { lines } => {
                    old_row += lines;
                    new_row += lines;
                }
                LineOperation::Delete { lines } => {
                    let old_end_row = old_row + lines - 1;
                    let new_row = new_snapshot.anchor_before(Point::new(new_row, 0));

                    if let Some((_, last_deleted_row_range)) =
                        self.diff.deleted_row_ranges.last_mut()
                    {
                        if *last_deleted_row_range.end() + 1 == old_row {
                            *last_deleted_row_range = *last_deleted_row_range.start()..=old_end_row;
                        } else {
                            self.diff
                                .deleted_row_ranges
                                .push((new_row, old_row..=old_end_row));
                        }
                    } else {
                        self.diff
                            .deleted_row_ranges
                            .push((new_row, old_row..=old_end_row));
                    }

                    old_row += lines;
                }
                LineOperation::Insert { lines } => {
                    let new_end_row = new_row + lines - 1;
                    let start = new_snapshot.anchor_before(Point::new(new_row, 0));
                    let end = new_snapshot.anchor_before(Point::new(
                        new_end_row,
                        new_snapshot.line_len(MultiBufferRow(new_end_row)),
                    ));
                    self.diff.inserted_row_ranges.push(start..end);
                    new_row += lines;
                }
            }

            cx.notify();
        }
    }

    fn reapply_batch_diff(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let old_snapshot = self.snapshot.clone();
        let old_range = self.range.to_point(&old_snapshot);
        let new_snapshot = self.buffer.read(cx).snapshot(cx);
        let new_range = self.range.to_point(&new_snapshot);

        cx.spawn(async move |codegen, cx| {
            let (deleted_row_ranges, inserted_row_ranges) = cx
                .background_spawn(async move {
                    let old_text = old_snapshot
                        .text_for_range(
                            Point::new(old_range.start.row, 0)
                                ..Point::new(
                                    old_range.end.row,
                                    old_snapshot.line_len(MultiBufferRow(old_range.end.row)),
                                ),
                        )
                        .collect::<String>();
                    let new_text = new_snapshot
                        .text_for_range(
                            Point::new(new_range.start.row, 0)
                                ..Point::new(
                                    new_range.end.row,
                                    new_snapshot.line_len(MultiBufferRow(new_range.end.row)),
                                ),
                        )
                        .collect::<String>();

                    let old_start_row = old_range.start.row;
                    let new_start_row = new_range.start.row;
                    let mut deleted_row_ranges: Vec<(Anchor, RangeInclusive<u32>)> = Vec::new();
                    let mut inserted_row_ranges = Vec::new();
                    for (old_rows, new_rows) in line_diff(&old_text, &new_text) {
                        let old_rows = old_start_row + old_rows.start..old_start_row + old_rows.end;
                        let new_rows = new_start_row + new_rows.start..new_start_row + new_rows.end;
                        if !old_rows.is_empty() {
                            deleted_row_ranges.push((
                                new_snapshot.anchor_before(Point::new(new_rows.start, 0)),
                                old_rows.start..=old_rows.end - 1,
                            ));
                        }
                        if !new_rows.is_empty() {
                            let start = new_snapshot.anchor_before(Point::new(new_rows.start, 0));
                            let new_end_row = new_rows.end - 1;
                            let end = new_snapshot.anchor_before(Point::new(
                                new_end_row,
                                new_snapshot.line_len(MultiBufferRow(new_end_row)),
                            ));
                            inserted_row_ranges.push(start..end);
                        }
                    }
                    (deleted_row_ranges, inserted_row_ranges)
                })
                .await;

            codegen
                .update(cx, |codegen, cx| {
                    codegen.diff.deleted_row_ranges = deleted_row_ranges;
                    codegen.diff.inserted_row_ranges = inserted_row_ranges;
                    cx.notify();
                })
                .ok();
        })
    }
}

struct StripInvalidSpans<T> {
    stream: T,
    stream_done: bool,
    buffer: String,
    first_line: bool,
    line_end: bool,
    starts_with_code_block: bool,
}

impl<T> StripInvalidSpans<T>
where
    T: Stream<Item = Result<String>>,
{
    fn new(stream: T) -> Self {
        Self {
            stream,
            stream_done: false,
            buffer: String::new(),
            first_line: true,
            line_end: false,
            starts_with_code_block: false,
        }
    }
}

impl<T> Stream for StripInvalidSpans<T>
where
    T: Stream<Item = Result<String>>,
{
    type Item = Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Option<Self::Item>> {
        const CODE_BLOCK_DELIMITER: &str = "```";
        const CURSOR_SPAN: &str = "<|CURSOR|>";

        let this = unsafe { self.get_unchecked_mut() };
        loop {
            if !this.stream_done {
                let mut stream = unsafe { Pin::new_unchecked(&mut this.stream) };
                match stream.as_mut().poll_next(cx) {
                    Poll::Ready(Some(Ok(chunk))) => {
                        this.buffer.push_str(&chunk);
                    }
                    Poll::Ready(Some(Err(error))) => return Poll::Ready(Some(Err(error))),
                    Poll::Ready(None) => {
                        this.stream_done = true;
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }

            let mut chunk = String::new();
            let mut consumed = 0;
            if !this.buffer.is_empty() {
                let mut lines = this.buffer.split('\n').enumerate().peekable();
                while let Some((line_ix, line)) = lines.next() {
                    if line_ix > 0 {
                        this.first_line = false;
                    }

                    if this.first_line {
                        let trimmed_line = line.trim();
                        if lines.peek().is_some() {
                            if trimmed_line.starts_with(CODE_BLOCK_DELIMITER) {
                                consumed += line.len() + 1;
                                this.starts_with_code_block = true;
                                continue;
                            }
                        } else if trimmed_line.is_empty()
                            || prefixes(CODE_BLOCK_DELIMITER)
                                .any(|prefix| trimmed_line.starts_with(prefix))
                        {
                            break;
                        }
                    }

                    let line_without_cursor = line.replace(CURSOR_SPAN, "");
                    if lines.peek().is_some() {
                        if this.line_end {
                            chunk.push('\n');
                        }

                        chunk.push_str(&line_without_cursor);
                        this.line_end = true;
                        consumed += line.len() + 1;
                    } else if this.stream_done {
                        if !this.starts_with_code_block
                            || !line_without_cursor.trim().ends_with(CODE_BLOCK_DELIMITER)
                        {
                            if this.line_end {
                                chunk.push('\n');
                            }

                            chunk.push_str(&line);
                        }

                        consumed += line.len();
                    } else {
                        let trimmed_line = line.trim();
                        if trimmed_line.is_empty()
                            || prefixes(CURSOR_SPAN).any(|prefix| trimmed_line.ends_with(prefix))
                            || prefixes(CODE_BLOCK_DELIMITER)
                                .any(|prefix| trimmed_line.ends_with(prefix))
                        {
                            break;
                        } else {
                            if this.line_end {
                                chunk.push('\n');
                                this.line_end = false;
                            }

                            chunk.push_str(&line_without_cursor);
                            consumed += line.len();
                        }
                    }
                }
            }

            this.buffer = this.buffer.split_off(consumed);
            if !chunk.is_empty() {
                return Poll::Ready(Some(Ok(chunk)));
            } else if this.stream_done {
                return Poll::Ready(None);
            }
        }
    }
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
        if !Assistant::enabled(cx) {
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
            if let Some(symbols_containing_start) = snapshot.symbols_containing(range.start, None) {
                if let Some(symbol) = symbols_containing_start.last() {
                    range.start = cmp::min(range.start, symbol.range.start.to_point(&snapshot));
                    range.end = cmp::max(range.end, symbol.range.end.to_point(&snapshot));
                }
            }

            if let Some(symbols_containing_end) = snapshot.symbols_containing(range.end, None) {
                if let Some(symbol) = symbols_containing_end.last() {
                    range.start = cmp::min(range.start, symbol.range.start.to_point(&snapshot));
                    range.end = cmp::max(range.end, symbol.range.end.to_point(&snapshot));
                }
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
        window.spawn(cx, async move |cx| {
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
                        Some(
                            multibuffer_snapshot
                                .anchor_in_excerpt(excerpt_id, action.range.start)?
                                ..multibuffer_snapshot
                                    .anchor_in_excerpt(excerpt_id, action.range.end)?,
                        )
                    })
                })?
                .context("invalid range")?;
            let assistant_panel = workspace.update(cx, |workspace, cx| {
                workspace
                    .panel::<AssistantPanel>(cx)
                    .context("assistant panel was released")
            })??;

            cx.update_global(|assistant: &mut InlineAssistant, window, cx| {
                let assist_id = assistant.suggest_assist(
                    &editor,
                    range,
                    "Fix Diagnostics".into(),
                    None,
                    true,
                    Some(workspace),
                    Some(&assistant_panel),
                    window,
                    cx,
                );
                assistant.start_assist(assist_id, window, cx);
            })?;

            Ok(ProjectTransaction::default())
        })
    }
}

fn prefixes(text: &str) -> impl Iterator<Item = &str> {
    (0..text.len() - 1).map(|ix| &text[..ix + 1])
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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::{self};
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::{
        Buffer, Language, LanguageConfig, LanguageMatcher, Point, language_settings,
        tree_sitter_rust,
    };
    use language_model::{LanguageModelRegistry, TokenUsage};
    use rand::prelude::*;
    use serde::Serialize;
    use settings::SettingsStore;
    use std::{future, sync::Arc};

    #[derive(Serialize)]
    pub struct DummyCompletionRequest {
        pub name: String,
    }

    #[gpui::test(iterations = 10)]
    async fn test_transform_autoindent(cx: &mut TestAppContext, mut rng: StdRng) {
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_model::LanguageModelRegistry::test);
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                let x = 0;
                for _ in 0..10 {
                    x += 1;
                }
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(4, 5))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);

        let mut new_text = concat!(
            "       let mut x = 0;\n",
            "       while x < 10 {\n",
            "           x += 1;\n",
            "       }",
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.gen_range(1..=max_len);
            let (chunk, suffix) = new_text.split_at(len);
            chunks_tx.unbounded_send(chunk.to_string()).unwrap();
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        drop(chunks_tx);
        cx.background_executor.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    while x < 10 {
                        x += 1;
                    }
                }
            "}
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_autoindent_when_generating_past_indentation(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                le
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 6))..snapshot.anchor_after(Point::new(1, 6))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);

        cx.background_executor.run_until_parked();

        let mut new_text = concat!(
            "t mut x = 0;\n",
            "while x < 10 {\n",
            "    x += 1;\n",
            "}", //
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.gen_range(1..=max_len);
            let (chunk, suffix) = new_text.split_at(len);
            chunks_tx.unbounded_send(chunk.to_string()).unwrap();
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        drop(chunks_tx);
        cx.background_executor.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    while x < 10 {
                        x += 1;
                    }
                }
            "}
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_autoindent_when_generating_before_indentation(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        cx.update(LanguageModelRegistry::test);
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = concat!(
            "fn main() {\n",
            "  \n",
            "}\n" //
        );
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 2))..snapshot.anchor_after(Point::new(1, 2))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);

        cx.background_executor.run_until_parked();

        let mut new_text = concat!(
            "let mut x = 0;\n",
            "while x < 10 {\n",
            "    x += 1;\n",
            "}", //
        );
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.gen_range(1..=max_len);
            let (chunk, suffix) = new_text.split_at(len);
            chunks_tx.unbounded_send(chunk.to_string()).unwrap();
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        drop(chunks_tx);
        cx.background_executor.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    while x < 10 {
                        x += 1;
                    }
                }
            "}
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_autoindent_respects_tabs_in_selection(cx: &mut TestAppContext) {
        cx.update(LanguageModelRegistry::test);
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            func main() {
            \tx := 0
            \tfor i := 0; i < 10; i++ {
            \t\tx++
            \t}
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(0, 0))..snapshot.anchor_after(Point::new(4, 2))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                true,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);
        let new_text = concat!(
            "func main() {\n",
            "\tx := 0\n",
            "\tfor x < 10 {\n",
            "\t\tx++\n",
            "\t}", //
        );
        chunks_tx.unbounded_send(new_text.to_string()).unwrap();
        drop(chunks_tx);
        cx.background_executor.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                func main() {
                \tx := 0
                \tfor x < 10 {
                \t\tx++
                \t}
                }
            "}
        );
    }

    #[gpui::test]
    async fn test_inactive_codegen_alternative(cx: &mut TestAppContext) {
        cx.update(LanguageModelRegistry::test);
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                let x = 0;
            }
        "};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(1, 14))
        });
        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let codegen = cx.new(|cx| {
            CodegenAlternative::new(
                buffer.clone(),
                range.clone(),
                false,
                None,
                prompt_builder,
                cx,
            )
        });

        let chunks_tx = simulate_response_stream(codegen.clone(), cx);
        chunks_tx
            .unbounded_send("let mut x = 0;\nx += 1;".to_string())
            .unwrap();
        drop(chunks_tx);
        cx.run_until_parked();

        // The codegen is inactive, so the buffer doesn't get modified.
        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            text
        );

        // Activating the codegen applies the changes.
        codegen.update(cx, |codegen, cx| codegen.set_active(true, cx));
        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    x += 1;
                }
            "}
        );

        // Deactivating the codegen undoes the changes.
        codegen.update(cx, |codegen, cx| codegen.set_active(false, cx));
        cx.run_until_parked();
        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            text
        );
    }

    #[gpui::test]
    async fn test_strip_invalid_spans_from_codeblock() {
        assert_chunks("Lorem ipsum dolor", "Lorem ipsum dolor").await;
        assert_chunks("```\nLorem ipsum dolor", "Lorem ipsum dolor").await;
        assert_chunks("```\nLorem ipsum dolor\n```", "Lorem ipsum dolor").await;
        assert_chunks(
            "```html\n```js\nLorem ipsum dolor\n```\n```",
            "```js\nLorem ipsum dolor\n```",
        )
        .await;
        assert_chunks("``\nLorem ipsum dolor\n```", "``\nLorem ipsum dolor\n```").await;
        assert_chunks("Lorem<|CURSOR|> ipsum", "Lorem ipsum").await;
        assert_chunks("Lorem ipsum", "Lorem ipsum").await;
        assert_chunks("```\n<|CURSOR|>Lorem ipsum\n```", "Lorem ipsum").await;

        async fn assert_chunks(text: &str, expected_text: &str) {
            for chunk_size in 1..=text.len() {
                let actual_text = StripInvalidSpans::new(chunks(text, chunk_size))
                    .map(|chunk| chunk.unwrap())
                    .collect::<String>()
                    .await;
                assert_eq!(
                    actual_text, expected_text,
                    "failed to strip invalid spans, chunk size: {}",
                    chunk_size
                );
            }
        }

        fn chunks(text: &str, size: usize) -> impl Stream<Item = Result<String>> {
            stream::iter(
                text.chars()
                    .collect::<Vec<_>>()
                    .chunks(size)
                    .map(|chunk| Ok(chunk.iter().collect::<String>()))
                    .collect::<Vec<_>>(),
            )
        }
    }

    fn simulate_response_stream(
        codegen: Entity<CodegenAlternative>,
        cx: &mut TestAppContext,
    ) -> mpsc::UnboundedSender<String> {
        let (chunks_tx, chunks_rx) = mpsc::unbounded();
        codegen.update(cx, |codegen, cx| {
            codegen.handle_stream(
                String::new(),
                String::new(),
                None,
                future::ready(Ok(LanguageModelTextStream {
                    message_id: None,
                    stream: chunks_rx.map(Ok).boxed(),
                    last_token_usage: Arc::new(Mutex::new(TokenUsage::default())),
                })),
                cx,
            );
        });
        chunks_tx
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_indents_query(
            r#"
            (call_expression) @indent
            (field_expression) @indent
            (_ "(" ")" @end) @indent
            (_ "{" "}" @end) @indent
            "#,
        )
        .unwrap()
    }
}
