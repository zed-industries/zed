use crate::{
    prompts::generate_content_prompt, AssistantPanel, CompletionProvider, Hunk,
    LanguageModelRequest, LanguageModelRequestMessage, Role, StreamingDiff,
};
use anyhow::{Context as _, Result};
use client::telemetry::Telemetry;
use collections::{hash_map, HashMap, HashSet, VecDeque};
use editor::{
    actions::{MoveDown, MoveUp, SelectAll},
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock,
        ToDisplayPoint,
    },
    Anchor, AnchorRangeExt, Editor, EditorElement, EditorEvent, EditorMode, EditorStyle,
    ExcerptRange, GutterDimensions, MultiBuffer, MultiBufferSnapshot, ToOffset, ToPoint,
};
use futures::{channel::mpsc, SinkExt, Stream, StreamExt};
use gpui::{
    point, AppContext, EventEmitter, FocusHandle, FocusableView, FontStyle, FontWeight, Global,
    HighlightStyle, Model, ModelContext, Subscription, Task, TextStyle, UpdateGlobal, View,
    ViewContext, WeakView, WhiteSpace, WindowContext,
};
use language::{Buffer, Point, Selection, TransactionId};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use rope::Rope;
use settings::Settings;
use similar::TextDiff;
use std::{
    cmp, mem,
    ops::{Range, RangeInclusive},
    pin::Pin,
    sync::Arc,
    task::{self, Poll},
    time::Instant,
};
use theme::ThemeSettings;
use ui::{prelude::*, Tooltip};
use util::RangeExt;
use workspace::{notifications::NotificationId, Toast, Workspace};

pub fn init(telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    cx.set_global(InlineAssistant::new(telemetry));
}

const PROMPT_HISTORY_MAX_LEN: usize = 20;

pub struct InlineAssistant {
    next_assist_id: InlineAssistId,
    next_assist_group_id: InlineAssistGroupId,
    assists: HashMap<InlineAssistId, InlineAssist>,
    assists_by_editor: HashMap<WeakView<Editor>, EditorInlineAssists>,
    assist_groups: HashMap<InlineAssistGroupId, InlineAssistGroup>,
    prompt_history: VecDeque<String>,
    telemetry: Option<Arc<Telemetry>>,
}

impl Global for InlineAssistant {}

impl InlineAssistant {
    pub fn new(telemetry: Arc<Telemetry>) -> Self {
        Self {
            next_assist_id: InlineAssistId::default(),
            next_assist_group_id: InlineAssistGroupId::default(),
            assists: HashMap::default(),
            assists_by_editor: HashMap::default(),
            assist_groups: HashMap::default(),
            prompt_history: VecDeque::default(),
            telemetry: Some(telemetry),
        }
    }

    pub fn assist(
        &mut self,
        editor: &View<Editor>,
        workspace: Option<WeakView<Workspace>>,
        include_context: bool,
        cx: &mut WindowContext,
    ) {
        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);

        let mut selections = Vec::<Selection<Point>>::new();
        let mut newest_selection = None;
        for mut selection in editor.read(cx).selections.all::<Point>(cx) {
            if selection.end > selection.start {
                selection.start.column = 0;
                // If the selection ends at the start of the line, we don't want to include it.
                if selection.end.column == 0 {
                    selection.end.row -= 1;
                }
                selection.end.column = snapshot.line_len(MultiBufferRow(selection.end.row));
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
        let newest_selection = newest_selection.unwrap();

        let mut codegen_ranges = Vec::new();
        for (excerpt_id, buffer, buffer_range) in
            snapshot.excerpts_in_ranges(selections.iter().map(|selection| {
                snapshot.anchor_before(selection.start)..snapshot.anchor_after(selection.end)
            }))
        {
            let start = Anchor {
                buffer_id: Some(buffer.remote_id()),
                excerpt_id,
                text_anchor: buffer.anchor_before(buffer_range.start),
            };
            let end = Anchor {
                buffer_id: Some(buffer.remote_id()),
                excerpt_id,
                text_anchor: buffer.anchor_after(buffer_range.end),
            };
            codegen_ranges.push(start..end);
        }

        let assist_group_id = self.next_assist_group_id.post_inc();
        let prompt_buffer = cx.new_model(|cx| Buffer::local("", cx));
        let prompt_buffer = cx.new_model(|cx| MultiBuffer::singleton(prompt_buffer, cx));

        let mut assists = Vec::new();
        let mut assist_blocks = Vec::new();
        let mut assist_to_focus = None;
        for range in codegen_ranges {
            let assist_id = self.next_assist_id.post_inc();
            let codegen = cx.new_model(|cx| {
                Codegen::new(
                    editor.read(cx).buffer().clone(),
                    range.clone(),
                    self.telemetry.clone(),
                    cx,
                )
            });

            let gutter_dimensions = Arc::new(Mutex::new(GutterDimensions::default()));
            let prompt_editor = cx.new_view(|cx| {
                PromptEditor::new(
                    assist_id,
                    gutter_dimensions.clone(),
                    self.prompt_history.clone(),
                    prompt_buffer.clone(),
                    codegen.clone(),
                    workspace.clone(),
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

            assist_blocks.push(BlockProperties {
                style: BlockStyle::Sticky,
                position: range.start,
                height: prompt_editor.read(cx).height_in_lines,
                render: build_assist_editor_renderer(&prompt_editor),
                disposition: BlockDisposition::Above,
            });
            assist_blocks.push(BlockProperties {
                style: BlockStyle::Sticky,
                position: range.end,
                height: 1,
                render: Box::new(|cx| {
                    v_flex()
                        .h_full()
                        .w_full()
                        .border_t_1()
                        .border_color(cx.theme().status().info_border)
                        .into_any_element()
                }),
                disposition: BlockDisposition::Below,
            });
            assists.push((assist_id, prompt_editor));
        }

        let assist_block_ids = editor.update(cx, |editor, cx| {
            editor.insert_blocks(assist_blocks, None, cx)
        });

        let editor_assists = self
            .assists_by_editor
            .entry(editor.downgrade())
            .or_insert_with(|| EditorInlineAssists::new(&editor, cx));
        let mut assist_group = InlineAssistGroup::new();
        for ((assist_id, prompt_editor), block_ids) in
            assists.into_iter().zip(assist_block_ids.chunks_exact(2))
        {
            self.assists.insert(
                assist_id,
                InlineAssist::new(
                    assist_id,
                    assist_group_id,
                    include_context,
                    editor,
                    &prompt_editor,
                    block_ids[0],
                    block_ids[1],
                    prompt_editor.read(cx).codegen.clone(),
                    workspace.clone(),
                    cx,
                ),
            );
            assist_group.assist_ids.push(assist_id);
            editor_assists.assist_ids.push(assist_id);
        }
        self.assist_groups.insert(assist_group_id, assist_group);

        if let Some(assist_id) = assist_to_focus {
            self.focus_assist(assist_id, cx);
        }
    }

    fn handle_prompt_editor_focus_in(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
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

    fn handle_prompt_editor_focus_out(
        &mut self,
        assist_id: InlineAssistId,
        cx: &mut WindowContext,
    ) {
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
        prompt_editor: View<PromptEditor>,
        event: &PromptEditorEvent,
        cx: &mut WindowContext,
    ) {
        let assist_id = prompt_editor.read(cx).id;
        match event {
            PromptEditorEvent::StartRequested => {
                self.start_assist(assist_id, cx);
            }
            PromptEditorEvent::StopRequested => {
                self.stop_assist(assist_id, cx);
            }
            PromptEditorEvent::ConfirmRequested => {
                self.finish_assist(assist_id, false, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, cx);
            }
            PromptEditorEvent::DismissRequested => {
                self.dismiss_assist(assist_id, cx);
            }
            PromptEditorEvent::Resized { height_in_lines } => {
                self.resize_assist(assist_id, *height_in_lines, cx);
            }
        }
    }

    fn handle_editor_newline(&mut self, editor: View<Editor>, cx: &mut WindowContext) {
        let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) else {
            return;
        };

        let editor = editor.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            let buffer = editor.buffer().read(cx).snapshot(cx);
            for assist_id in &editor_assists.assist_ids {
                let assist = &self.assists[assist_id];
                let assist_range = assist.codegen.read(cx).range.to_offset(&buffer);
                if assist_range.contains(&selection.start) && assist_range.contains(&selection.end)
                {
                    if matches!(assist.codegen.read(cx).status, CodegenStatus::Pending) {
                        self.dismiss_assist(*assist_id, cx);
                    } else {
                        self.finish_assist(*assist_id, false, cx);
                    }

                    return;
                }
            }
        }

        cx.propagate();
    }

    fn handle_editor_cancel(&mut self, editor: View<Editor>, cx: &mut WindowContext) {
        let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) else {
            return;
        };

        let editor = editor.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            let buffer = editor.buffer().read(cx).snapshot(cx);
            for assist_id in &editor_assists.assist_ids {
                let assist = &self.assists[assist_id];
                let assist_range = assist.codegen.read(cx).range.to_offset(&buffer);
                if assist.decorations.is_some()
                    && assist_range.contains(&selection.start)
                    && assist_range.contains(&selection.end)
                {
                    self.focus_assist(*assist_id, cx);
                    return;
                }
            }
        }

        cx.propagate();
    }

    fn handle_editor_change(&mut self, editor: View<Editor>, cx: &mut WindowContext) {
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
                editor.set_scroll_position(point(scroll_position.x, target_scroll_top), cx);
            }
        });
    }

    fn handle_editor_event(
        &mut self,
        editor: View<Editor>,
        event: &EditorEvent,
        cx: &mut WindowContext,
    ) {
        let Some(editor_assists) = self.assists_by_editor.get_mut(&editor.downgrade()) else {
            return;
        };

        match event {
            EditorEvent::Saved => {
                for assist_id in editor_assists.assist_ids.clone() {
                    let assist = &self.assists[&assist_id];
                    if let CodegenStatus::Done = &assist.codegen.read(cx).status {
                        self.finish_assist(assist_id, false, cx)
                    }
                }
            }
            EditorEvent::Edited { transaction_id } => {
                let buffer = editor.read(cx).buffer().read(cx);
                let edited_ranges =
                    buffer.edited_ranges_for_transaction::<usize>(*transaction_id, cx);
                let snapshot = buffer.snapshot(cx);

                for assist_id in editor_assists.assist_ids.clone() {
                    let assist = &self.assists[&assist_id];
                    if matches!(
                        assist.codegen.read(cx).status,
                        CodegenStatus::Error(_) | CodegenStatus::Done
                    ) {
                        let assist_range = assist.codegen.read(cx).range.to_offset(&snapshot);
                        if edited_ranges
                            .iter()
                            .any(|range| range.overlaps(&assist_range))
                        {
                            self.finish_assist(assist_id, false, cx);
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
                        if decorations.prompt_editor.focus_handle(cx).is_focused(cx) {
                            return;
                        }
                    }
                }

                editor_assists.scroll_lock = None;
            }
            _ => {}
        }
    }

    fn finish_assist(&mut self, assist_id: InlineAssistId, undo: bool, cx: &mut WindowContext) {
        if let Some(assist) = self.assists.get(&assist_id) {
            let assist_group_id = assist.group_id;
            if self.assist_groups[&assist_group_id].linked {
                for assist_id in self.unlink_assist_group(assist_group_id, cx) {
                    self.finish_assist(assist_id, undo, cx);
                }
                return;
            }
        }

        self.dismiss_assist(assist_id, cx);

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

            if undo {
                assist.codegen.update(cx, |codegen, cx| codegen.undo(cx));
            }
        }
    }

    fn dismiss_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) -> bool {
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
            .contains_focused(cx)
        {
            self.focus_next_assist(assist_id, cx);
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

    fn focus_next_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
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
                self.focus_assist(*assist_id, cx);
                return;
            }
        }

        assist.editor.update(cx, |editor, cx| editor.focus(cx)).ok();
    }

    fn focus_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        let assist = &self.assists[&assist_id];
        let Some(editor) = assist.editor.upgrade() else {
            return;
        };

        if let Some(decorations) = assist.decorations.as_ref() {
            decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                prompt_editor.editor.update(cx, |editor, cx| {
                    editor.focus(cx);
                    editor.select_all(&SelectAll, cx);
                })
            });
        }

        let position = assist.codegen.read(cx).range.start;
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |selections| {
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
                let snapshot = editor.snapshot(cx);
                let codegen = assist.codegen.read(cx);
                let start_row = codegen
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
                editor.set_scroll_position(point(0., scroll_target_top), cx);
            } else if scroll_target_bottom > scroll_bottom {
                if (scroll_target_bottom - scroll_target_top) <= height_in_lines {
                    editor
                        .set_scroll_position(point(0., scroll_target_bottom - height_in_lines), cx);
                } else {
                    editor.set_scroll_position(point(0., scroll_target_top), cx);
                }
            }
        });
    }

    fn resize_assist(
        &mut self,
        assist_id: InlineAssistId,
        height_in_lines: u8,
        cx: &mut WindowContext,
    ) {
        if let Some(assist) = self.assists.get_mut(&assist_id) {
            if let Some(editor) = assist.editor.upgrade() {
                if let Some(decorations) = assist.decorations.as_ref() {
                    let mut new_blocks = HashMap::default();
                    new_blocks.insert(
                        decorations.prompt_block_id,
                        (
                            Some(height_in_lines),
                            build_assist_editor_renderer(&decorations.prompt_editor),
                        ),
                    );
                    editor.update(cx, |editor, cx| {
                        editor
                            .display_map
                            .update(cx, |map, cx| map.replace_blocks(new_blocks, cx))
                    });
                }
            }
        }
    }

    fn unlink_assist_group(
        &mut self,
        assist_group_id: InlineAssistGroupId,
        cx: &mut WindowContext,
    ) -> Vec<InlineAssistId> {
        let assist_group = self.assist_groups.get_mut(&assist_group_id).unwrap();
        assist_group.linked = false;
        for assist_id in &assist_group.assist_ids {
            let assist = self.assists.get_mut(assist_id).unwrap();
            if let Some(editor_decorations) = assist.decorations.as_ref() {
                editor_decorations
                    .prompt_editor
                    .update(cx, |prompt_editor, cx| prompt_editor.unlink(cx));
            }
        }
        assist_group.assist_ids.clone()
    }

    fn start_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        let assist_group_id = assist.group_id;
        if self.assist_groups[&assist_group_id].linked {
            for assist_id in self.unlink_assist_group(assist_group_id, cx) {
                self.start_assist(assist_id, cx);
            }
            return;
        }

        assist.codegen.update(cx, |codegen, cx| codegen.undo(cx));

        let Some(user_prompt) = assist
            .decorations
            .as_ref()
            .map(|decorations| decorations.prompt_editor.read(cx).prompt(cx))
        else {
            return;
        };

        let context = if assist.include_context {
            assist.workspace.as_ref().and_then(|workspace| {
                let workspace = workspace.upgrade()?.read(cx);
                let assistant_panel = workspace.panel::<AssistantPanel>(cx)?;
                assistant_panel.read(cx).active_context(cx)
            })
        } else {
            None
        };

        let editor = if let Some(editor) = assist.editor.upgrade() {
            editor
        } else {
            return;
        };

        let project_name = assist.workspace.as_ref().and_then(|workspace| {
            let workspace = workspace.upgrade()?;
            Some(
                workspace
                    .read(cx)
                    .project()
                    .read(cx)
                    .worktree_root_names(cx)
                    .collect::<Vec<&str>>()
                    .join("/"),
            )
        });

        self.prompt_history.retain(|prompt| *prompt != user_prompt);
        self.prompt_history.push_back(user_prompt.clone());
        if self.prompt_history.len() > PROMPT_HISTORY_MAX_LEN {
            self.prompt_history.pop_front();
        }

        let codegen = assist.codegen.clone();
        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        let range = codegen.read(cx).range.clone();
        let start = snapshot.point_to_buffer_offset(range.start);
        let end = snapshot.point_to_buffer_offset(range.end);
        let (buffer, range) = if let Some((start, end)) = start.zip(end) {
            let (start_buffer, start_buffer_offset) = start;
            let (end_buffer, end_buffer_offset) = end;
            if start_buffer.remote_id() == end_buffer.remote_id() {
                (start_buffer.clone(), start_buffer_offset..end_buffer_offset)
            } else {
                self.finish_assist(assist_id, false, cx);
                return;
            }
        } else {
            self.finish_assist(assist_id, false, cx);
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

        let prompt = cx.background_executor().spawn(async move {
            let language_name = language_name.as_deref();
            generate_content_prompt(user_prompt, language_name, buffer, range, project_name)
        });

        let mut messages = Vec::new();
        if let Some(context) = context {
            let request = context.read(cx).to_completion_request(cx);
            messages = request.messages;
        }
        let model = CompletionProvider::global(cx).model();

        cx.spawn(|mut cx| async move {
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
        .detach_and_log_err(cx);
    }

    fn stop_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        assist.codegen.update(cx, |codegen, cx| codegen.stop(cx));
    }

    fn update_editor_highlights(&self, editor: &View<Editor>, cx: &mut WindowContext) {
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
                foreground_ranges.extend(codegen.last_equal_ranges().iter().cloned());

                if codegen.edit_position != codegen.range.end {
                    gutter_pending_ranges.push(codegen.edit_position..codegen.range.end);
                }

                if codegen.range.start != codegen.edit_position {
                    gutter_transformed_ranges.push(codegen.range.start..codegen.edit_position);
                }

                if assist.decorations.is_some() {
                    inserted_row_ranges.extend(codegen.diff.inserted_row_ranges.iter().cloned());
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
                    Some(cx.theme().status().info_background),
                    false,
                    cx,
                );
            }
        });
    }

    fn update_editor_blocks(
        &mut self,
        editor: &View<Editor>,
        assist_id: InlineAssistId,
        cx: &mut WindowContext,
    ) {
        let Some(assist) = self.assists.get_mut(&assist_id) else {
            return;
        };
        let Some(decorations) = assist.decorations.as_mut() else {
            return;
        };

        let codegen = assist.codegen.read(cx);
        let old_snapshot = codegen.snapshot.clone();
        let old_buffer = codegen.old_buffer.clone();
        let deleted_row_ranges = codegen.diff.deleted_row_ranges.clone();

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

                let deleted_lines_editor = cx.new_view(|cx| {
                    let multi_buffer = cx.new_model(|_| {
                        MultiBuffer::without_headers(0, language::Capability::ReadOnly)
                    });
                    multi_buffer.update(cx, |multi_buffer, cx| {
                        multi_buffer.push_excerpts(
                            old_buffer.clone(),
                            Some(ExcerptRange {
                                context: buffer_start..buffer_end,
                                primary: None,
                            }),
                            cx,
                        );
                    });

                    enum DeletedLines {}
                    let mut editor = Editor::for_multibuffer(multi_buffer, None, true, cx);
                    editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
                    editor.set_show_wrap_guides(false, cx);
                    editor.set_show_gutter(false, cx);
                    editor.scroll_manager.set_forbid_vertical_scroll(true);
                    editor.set_read_only(true);
                    editor.highlight_rows::<DeletedLines>(
                        Anchor::min()..=Anchor::max(),
                        Some(cx.theme().status().deleted_background),
                        false,
                        cx,
                    );
                    editor
                });

                let height = deleted_lines_editor
                    .update(cx, |editor, cx| editor.max_point(cx).row().0 as u8 + 1);
                new_blocks.push(BlockProperties {
                    position: new_row,
                    height,
                    style: BlockStyle::Flex,
                    render: Box::new(move |cx| {
                        div()
                            .bg(cx.theme().status().deleted_background)
                            .size_full()
                            .pl(cx.gutter_dimensions.full_width())
                            .child(deleted_lines_editor.clone())
                            .into_any_element()
                    }),
                    disposition: BlockDisposition::Above,
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
    #[allow(clippy::too_many_arguments)]
    fn new(editor: &View<Editor>, cx: &mut WindowContext) -> Self {
        let (highlight_updates_tx, mut highlight_updates_rx) = async_watch::channel(());
        Self {
            assist_ids: Vec::new(),
            scroll_lock: None,
            highlight_updates: highlight_updates_tx,
            _update_highlights: cx.spawn(|mut cx| {
                let editor = editor.downgrade();
                async move {
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
                cx.observe(editor, move |editor, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_editor_change(editor, cx)
                    })
                }),
                cx.subscribe(editor, move |editor, event, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_editor_event(editor, event, cx)
                    })
                }),
                editor.update(cx, |editor, cx| {
                    let editor_handle = cx.view().downgrade();
                    editor.register_action(
                        move |_: &editor::actions::Newline, cx: &mut WindowContext| {
                            InlineAssistant::update_global(cx, |this, cx| {
                                if let Some(editor) = editor_handle.upgrade() {
                                    this.handle_editor_newline(editor, cx)
                                }
                            })
                        },
                    )
                }),
                editor.update(cx, |editor, cx| {
                    let editor_handle = cx.view().downgrade();
                    editor.register_action(
                        move |_: &editor::actions::Cancel, cx: &mut WindowContext| {
                            InlineAssistant::update_global(cx, |this, cx| {
                                if let Some(editor) = editor_handle.upgrade() {
                                    this.handle_editor_cancel(editor, cx)
                                }
                            })
                        },
                    )
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

fn build_assist_editor_renderer(editor: &View<PromptEditor>) -> RenderBlock {
    let editor = editor.clone();
    Box::new(move |cx: &mut BlockContext| {
        *editor.read(cx).gutter_dimensions.lock() = *cx.gutter_dimensions;
        editor.clone().into_any_element()
    })
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct InlineAssistId(usize);

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
    Resized { height_in_lines: u8 },
}

struct PromptEditor {
    id: InlineAssistId,
    height_in_lines: u8,
    editor: View<Editor>,
    edited_since_done: bool,
    gutter_dimensions: Arc<Mutex<GutterDimensions>>,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    codegen: Model<Codegen>,
    workspace: Option<WeakView<Workspace>>,
    _codegen_subscription: Subscription,
    editor_subscriptions: Vec<Subscription>,
}

impl EventEmitter<PromptEditorEvent> for PromptEditor {}

impl Render for PromptEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let gutter_dimensions = *self.gutter_dimensions.lock();

        let buttons = match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::for_action("Cancel Assist", &menu::Cancel, cx))
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        ),
                    IconButton::new("start", IconName::Sparkle)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .icon_size(IconSize::XSmall)
                        .tooltip(|cx| Tooltip::for_action("Transform", &menu::Confirm, cx))
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::StartRequested)),
                        ),
                ]
            }
            CodegenStatus::Pending => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::text("Cancel Assist", cx))
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        ),
                    IconButton::new("stop", IconName::Stop)
                        .icon_color(Color::Error)
                        .size(ButtonSize::None)
                        .icon_size(IconSize::XSmall)
                        .tooltip(|cx| {
                            Tooltip::with_meta(
                                "Interrupt Transformation",
                                Some(&menu::Cancel),
                                "Changes won't be discarded",
                                cx,
                            )
                        })
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::StopRequested)),
                        ),
                ]
            }
            CodegenStatus::Error(_) | CodegenStatus::Done => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::for_action("Cancel Assist", &menu::Cancel, cx))
                        .on_click(
                            cx.listener(|_, _, cx| cx.emit(PromptEditorEvent::CancelRequested)),
                        ),
                    if self.edited_since_done {
                        IconButton::new("restart", IconName::RotateCw)
                            .icon_color(Color::Info)
                            .icon_size(IconSize::XSmall)
                            .size(ButtonSize::None)
                            .tooltip(|cx| {
                                Tooltip::with_meta(
                                    "Restart Transformation",
                                    Some(&menu::Confirm),
                                    "Changes will be discarded",
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|_, _, cx| {
                                cx.emit(PromptEditorEvent::StartRequested);
                            }))
                    } else {
                        IconButton::new("confirm", IconName::Check)
                            .icon_color(Color::Info)
                            .size(ButtonSize::None)
                            .tooltip(|cx| Tooltip::for_action("Confirm Assist", &menu::Confirm, cx))
                            .on_click(cx.listener(|_, _, cx| {
                                cx.emit(PromptEditorEvent::ConfirmRequested);
                            }))
                    },
                ]
            }
        };

        v_flex().h_full().w_full().justify_end().child(
            h_flex()
                .bg(cx.theme().colors().editor_background)
                .border_y_1()
                .border_color(cx.theme().status().info_border)
                .py_1p5()
                .w_full()
                .on_action(cx.listener(Self::confirm))
                .on_action(cx.listener(Self::cancel))
                .on_action(cx.listener(Self::move_up))
                .on_action(cx.listener(Self::move_down))
                .child(
                    h_flex()
                        .w(gutter_dimensions.full_width() + (gutter_dimensions.margin / 2.0))
                        // .pr(gutter_dimensions.fold_area_width())
                        .justify_center()
                        .gap_2()
                        .children(self.workspace.clone().map(|workspace| {
                            IconButton::new("context", IconName::Context)
                                .size(ButtonSize::None)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Muted)
                                .on_click({
                                    let workspace = workspace.clone();
                                    cx.listener(move |_, _, cx| {
                                        workspace
                                            .update(cx, |workspace, cx| {
                                                workspace.focus_panel::<AssistantPanel>(cx);
                                            })
                                            .ok();
                                    })
                                })
                                .tooltip(move |cx| {
                                    let token_count = workspace.upgrade().and_then(|workspace| {
                                        let panel =
                                            workspace.read(cx).panel::<AssistantPanel>(cx)?;
                                        let context = panel.read(cx).active_context(cx)?;
                                        context.read(cx).token_count()
                                    });
                                    if let Some(token_count) = token_count {
                                        Tooltip::with_meta(
                                            format!(
                                                "{} Additional Context Tokens from Assistant",
                                                token_count
                                            ),
                                            Some(&crate::ToggleFocus),
                                            "Click to open…",
                                            cx,
                                        )
                                    } else {
                                        Tooltip::for_action(
                                            "Toggle Assistant Panel",
                                            &crate::ToggleFocus,
                                            cx,
                                        )
                                    }
                                })
                        }))
                        .children(
                            if let CodegenStatus::Error(error) = &self.codegen.read(cx).status {
                                let error_message = SharedString::from(error.to_string());
                                Some(
                                    div()
                                        .id("error")
                                        .tooltip(move |cx| Tooltip::text(error_message.clone(), cx))
                                        .child(
                                            Icon::new(IconName::XCircle)
                                                .size(IconSize::Small)
                                                .color(Color::Error),
                                        ),
                                )
                            } else {
                                None
                            },
                        ),
                )
                .child(div().flex_1().child(self.render_prompt_editor(cx)))
                .child(h_flex().gap_2().pr_4().children(buttons)),
        )
    }
}

impl FocusableView for PromptEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl PromptEditor {
    const MAX_LINES: u8 = 8;

    fn new(
        id: InlineAssistId,
        gutter_dimensions: Arc<Mutex<GutterDimensions>>,
        prompt_history: VecDeque<String>,
        prompt_buffer: Model<MultiBuffer>,
        codegen: Model<Codegen>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let prompt_editor = cx.new_view(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    max_lines: Self::MAX_LINES as usize,
                },
                prompt_buffer,
                None,
                false,
                cx,
            );
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            // Since the prompt editors for all inline assistants are linked,
            // always show the cursor (even when it isn't focused) because
            // typing in one will make what you typed appear in all of them.
            editor.set_show_cursor_when_unfocused(true, cx);
            editor.set_placeholder_text("Add a prompt…", cx);
            editor
        });
        let mut this = Self {
            id,
            height_in_lines: 1,
            editor: prompt_editor,
            edited_since_done: false,
            gutter_dimensions,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            _codegen_subscription: cx.observe(&codegen, Self::handle_codegen_changed),
            editor_subscriptions: Vec::new(),
            codegen,
            workspace,
        };
        this.count_lines(cx);
        this.subscribe_to_editor(cx);
        this
    }

    fn subscribe_to_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.editor_subscriptions.clear();
        self.editor_subscriptions
            .push(cx.observe(&self.editor, Self::handle_prompt_editor_changed));
        self.editor_subscriptions
            .push(cx.subscribe(&self.editor, Self::handle_prompt_editor_events));
    }

    fn set_show_cursor_when_unfocused(
        &mut self,
        show_cursor_when_unfocused: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_show_cursor_when_unfocused(show_cursor_when_unfocused, cx)
        });
    }

    fn unlink(&mut self, cx: &mut ViewContext<Self>) {
        let prompt = self.prompt(cx);
        let focus = self.editor.focus_handle(cx).contains_focused(cx);
        self.editor = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(Self::MAX_LINES as usize, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text("Add a prompt…", cx);
            editor.set_text(prompt, cx);
            if focus {
                editor.focus(cx);
            }
            editor
        });
        self.subscribe_to_editor(cx);
    }

    fn prompt(&self, cx: &AppContext) -> String {
        self.editor.read(cx).text(cx)
    }

    fn count_lines(&mut self, cx: &mut ViewContext<Self>) {
        let height_in_lines = cmp::max(
            2, // Make the editor at least two lines tall, to account for padding and buttons.
            cmp::min(
                self.editor
                    .update(cx, |editor, cx| editor.max_point(cx).row().0 + 1),
                Self::MAX_LINES as u32,
            ),
        ) as u8;

        if height_in_lines != self.height_in_lines {
            self.height_in_lines = height_in_lines;
            cx.emit(PromptEditorEvent::Resized { height_in_lines });
        }
    }

    fn handle_prompt_editor_changed(&mut self, _: View<Editor>, cx: &mut ViewContext<Self>) {
        self.count_lines(cx);
    }

    fn handle_prompt_editor_events(
        &mut self,
        _: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::Edited { .. } => {
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
            _ => {}
        }
    }

    fn handle_codegen_changed(&mut self, _: Model<Codegen>, cx: &mut ViewContext<Self>) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
            CodegenStatus::Pending => {
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(true));
            }
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                self.edited_since_done = false;
                self.editor
                    .update(cx, |editor, _| editor.set_read_only(false));
            }
        }
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle | CodegenStatus::Done | CodegenStatus::Error(_) => {
                cx.emit(PromptEditorEvent::CancelRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::StopRequested);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                cx.emit(PromptEditorEvent::StartRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(PromptEditorEvent::DismissRequested);
            }
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                if self.edited_since_done {
                    cx.emit(PromptEditorEvent::StartRequested);
                } else {
                    cx.emit(PromptEditorEvent::ConfirmRequested);
                }
            }
        }
    }

    fn move_up(&mut self, _: &MoveUp, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix > 0 {
                self.prompt_history_ix = Some(ix - 1);
                let prompt = self.prompt_history[ix - 1].as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, cx);
                    editor.move_to_beginning(&Default::default(), cx);
                });
            }
        } else if !self.prompt_history.is_empty() {
            self.prompt_history_ix = Some(self.prompt_history.len() - 1);
            let prompt = self.prompt_history[self.prompt_history.len() - 1].as_str();
            self.editor.update(cx, |editor, cx| {
                editor.set_text(prompt, cx);
                editor.move_to_beginning(&Default::default(), cx);
            });
        }
    }

    fn move_down(&mut self, _: &MoveDown, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.prompt_history_ix {
            if ix < self.prompt_history.len() - 1 {
                self.prompt_history_ix = Some(ix + 1);
                let prompt = self.prompt_history[ix + 1].as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, cx);
                    editor.move_to_end(&Default::default(), cx)
                });
            } else {
                self.prompt_history_ix = None;
                let prompt = self.pending_prompt.as_str();
                self.editor.update(cx, |editor, cx| {
                    editor.set_text(prompt, cx);
                    editor.move_to_end(&Default::default(), cx)
                });
            }
        }
    }

    fn render_prompt_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.editor.read(cx).read_only(cx) {
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
            &self.editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

struct InlineAssist {
    group_id: InlineAssistGroupId,
    editor: WeakView<Editor>,
    decorations: Option<InlineAssistDecorations>,
    codegen: Model<Codegen>,
    _subscriptions: Vec<Subscription>,
    workspace: Option<WeakView<Workspace>>,
    include_context: bool,
}

impl InlineAssist {
    #[allow(clippy::too_many_arguments)]
    fn new(
        assist_id: InlineAssistId,
        group_id: InlineAssistGroupId,
        include_context: bool,
        editor: &View<Editor>,
        prompt_editor: &View<PromptEditor>,
        prompt_block_id: BlockId,
        end_block_id: BlockId,
        codegen: Model<Codegen>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
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
            codegen: codegen.clone(),
            workspace: workspace.clone(),
            _subscriptions: vec![
                cx.on_focus_in(&prompt_editor_focus_handle, move |cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_focus_in(assist_id, cx)
                    })
                }),
                cx.on_focus_out(&prompt_editor_focus_handle, move |_, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_focus_out(assist_id, cx)
                    })
                }),
                cx.subscribe(prompt_editor, |prompt_editor, event, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_event(prompt_editor, event, cx)
                    })
                }),
                cx.observe(&codegen, {
                    let editor = editor.downgrade();
                    move |_, cx| {
                        if let Some(editor) = editor.upgrade() {
                            InlineAssistant::update_global(cx, |this, cx| {
                                if let Some(editor_assists) =
                                    this.assists_by_editor.get(&editor.downgrade())
                                {
                                    editor_assists.highlight_updates.send(()).ok();
                                }

                                this.update_editor_blocks(&editor, assist_id, cx);
                            })
                        }
                    }
                }),
                cx.subscribe(&codegen, move |codegen, event, cx| {
                    InlineAssistant::update_global(cx, |this, cx| match event {
                        CodegenEvent::Undone => this.finish_assist(assist_id, false, cx),
                        CodegenEvent::Finished => {
                            let assist = if let Some(assist) = this.assists.get(&assist_id) {
                                assist
                            } else {
                                return;
                            };

                            if let CodegenStatus::Error(error) = &codegen.read(cx).status {
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
                                                NotificationId::identified::<InlineAssistantError>(
                                                    assist_id.0,
                                                );

                                            workspace.show_toast(Toast::new(id, error), cx);
                                        })
                                    }
                                }
                            }

                            if assist.decorations.is_none() {
                                this.finish_assist(assist_id, false, cx);
                            }
                        }
                    })
                }),
            ],
        }
    }
}

struct InlineAssistDecorations {
    prompt_block_id: BlockId,
    prompt_editor: View<PromptEditor>,
    removed_line_block_ids: HashSet<BlockId>,
    end_block_id: BlockId,
}

#[derive(Debug)]
pub enum CodegenEvent {
    Finished,
    Undone,
}

pub struct Codegen {
    buffer: Model<MultiBuffer>,
    old_buffer: Model<Buffer>,
    snapshot: MultiBufferSnapshot,
    range: Range<Anchor>,
    edit_position: Anchor,
    last_equal_ranges: Vec<Range<Anchor>>,
    transaction_id: Option<TransactionId>,
    status: CodegenStatus,
    generation: Task<()>,
    diff: Diff,
    telemetry: Option<Arc<Telemetry>>,
    _subscription: gpui::Subscription,
}

enum CodegenStatus {
    Idle,
    Pending,
    Done,
    Error(anyhow::Error),
}

#[derive(Default)]
struct Diff {
    task: Option<Task<()>>,
    should_update: bool,
    deleted_row_ranges: Vec<(Anchor, RangeInclusive<u32>)>,
    inserted_row_ranges: Vec<RangeInclusive<Anchor>>,
}

impl EventEmitter<CodegenEvent> for Codegen {}

impl Codegen {
    pub fn new(
        buffer: Model<MultiBuffer>,
        range: Range<Anchor>,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let snapshot = buffer.read(cx).snapshot(cx);

        let (old_buffer, _, _) = buffer
            .read(cx)
            .range_to_buffer_ranges(range.clone(), cx)
            .pop()
            .unwrap();
        let old_buffer = cx.new_model(|cx| {
            let old_buffer = old_buffer.read(cx);
            let text = old_buffer.as_rope().clone();
            let line_ending = old_buffer.line_ending();
            let language = old_buffer.language().cloned();
            let language_registry = old_buffer.language_registry();

            let mut buffer = Buffer::local_normalized(text, line_ending, cx);
            buffer.set_language(language, cx);
            if let Some(language_registry) = language_registry {
                buffer.set_language_registry(language_registry)
            }
            buffer
        });

        Self {
            buffer: buffer.clone(),
            old_buffer,
            edit_position: range.start,
            range,
            snapshot,
            last_equal_ranges: Default::default(),
            transaction_id: Default::default(),
            status: CodegenStatus::Idle,
            generation: Task::ready(()),
            diff: Diff::default(),
            telemetry,
            _subscription: cx.subscribe(&buffer, Self::handle_buffer_event),
        }
    }

    fn handle_buffer_event(
        &mut self,
        _buffer: Model<MultiBuffer>,
        event: &multi_buffer::Event,
        cx: &mut ModelContext<Self>,
    ) {
        if let multi_buffer::Event::TransactionUndone { transaction_id } = event {
            if self.transaction_id == Some(*transaction_id) {
                self.transaction_id = None;
                self.generation = Task::ready(());
                cx.emit(CodegenEvent::Undone);
            }
        }
    }

    pub fn last_equal_ranges(&self) -> &[Range<Anchor>] {
        &self.last_equal_ranges
    }

    pub fn start(&mut self, prompt: LanguageModelRequest, cx: &mut ModelContext<Self>) {
        let range = self.range.clone();
        let snapshot = self.snapshot.clone();
        let selected_text = snapshot
            .text_for_range(range.start..range.end)
            .collect::<Rope>();

        let selection_start = range.start.to_point(&snapshot);
        let suggested_line_indent = snapshot
            .suggested_indents(selection_start.row..selection_start.row + 1, cx)
            .into_values()
            .next()
            .unwrap_or_else(|| snapshot.indent_size_for_line(MultiBufferRow(selection_start.row)));

        let model_telemetry_id = prompt.model.telemetry_id();
        let response = CompletionProvider::global(cx).complete(prompt);
        let telemetry = self.telemetry.clone();
        self.edit_position = range.start;
        self.diff = Diff::default();
        self.status = CodegenStatus::Pending;
        self.generation = cx.spawn(|this, mut cx| {
            async move {
                let generate = async {
                    let mut edit_start = range.start.to_offset(&snapshot);

                    let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);
                    let diff: Task<anyhow::Result<()>> =
                        cx.background_executor().spawn(async move {
                            let mut response_latency = None;
                            let request_start = Instant::now();
                            let diff = async {
                                let chunks = StripInvalidSpans::new(response.await?);
                                futures::pin_mut!(chunks);
                                let mut diff = StreamingDiff::new(selected_text.to_string());

                                let mut new_text = String::new();
                                let mut base_indent = None;
                                let mut line_indent = None;
                                let mut first_line = true;

                                while let Some(chunk) = chunks.next().await {
                                    if response_latency.is_none() {
                                        response_latency = Some(request_start.elapsed());
                                    }
                                    let chunk = chunk?;

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
                                            hunks_tx.send(diff.push_new(&new_text)).await?;
                                            new_text.clear();
                                        }

                                        if lines.peek().is_some() {
                                            hunks_tx.send(diff.push_new("\n")).await?;
                                            line_indent = None;
                                            first_line = false;
                                        }
                                    }
                                }
                                hunks_tx.send(diff.push_new(&new_text)).await?;
                                hunks_tx.send(diff.finish()).await?;

                                anyhow::Ok(())
                            };

                            let result = diff.await;

                            let error_message =
                                result.as_ref().err().map(|error| error.to_string());
                            if let Some(telemetry) = telemetry {
                                telemetry.report_assistant_event(
                                    None,
                                    telemetry_events::AssistantKind::Inline,
                                    model_telemetry_id,
                                    response_latency,
                                    error_message,
                                );
                            }

                            result?;
                            Ok(())
                        });

                    while let Some(hunks) = hunks_rx.next().await {
                        this.update(&mut cx, |this, cx| {
                            this.last_equal_ranges.clear();

                            let transaction = this.buffer.update(cx, |buffer, cx| {
                                // Avoid grouping assistant edits with user edits.
                                buffer.finalize_last_transaction(cx);

                                buffer.start_transaction(cx);
                                buffer.edit(
                                    hunks.into_iter().filter_map(|hunk| match hunk {
                                        Hunk::Insert { text } => {
                                            let edit_start = snapshot.anchor_after(edit_start);
                                            Some((edit_start..edit_start, text))
                                        }
                                        Hunk::Remove { len } => {
                                            let edit_end = edit_start + len;
                                            let edit_range = snapshot.anchor_after(edit_start)
                                                ..snapshot.anchor_before(edit_end);
                                            edit_start = edit_end;
                                            Some((edit_range, String::new()))
                                        }
                                        Hunk::Keep { len } => {
                                            let edit_end = edit_start + len;
                                            let edit_range = snapshot.anchor_after(edit_start)
                                                ..snapshot.anchor_before(edit_end);
                                            edit_start = edit_end;
                                            this.last_equal_ranges.push(edit_range);
                                            None
                                        }
                                    }),
                                    None,
                                    cx,
                                );
                                this.edit_position = snapshot.anchor_after(edit_start);

                                buffer.end_transaction(cx)
                            });

                            if let Some(transaction) = transaction {
                                if let Some(first_transaction) = this.transaction_id {
                                    // Group all assistant edits into the first transaction.
                                    this.buffer.update(cx, |buffer, cx| {
                                        buffer.merge_transactions(
                                            transaction,
                                            first_transaction,
                                            cx,
                                        )
                                    });
                                } else {
                                    this.transaction_id = Some(transaction);
                                    this.buffer.update(cx, |buffer, cx| {
                                        buffer.finalize_last_transaction(cx)
                                    });
                                }
                            }

                            this.update_diff(cx);
                            cx.notify();
                        })?;
                    }

                    diff.await?;

                    anyhow::Ok(())
                };

                let result = generate.await;
                this.update(&mut cx, |this, cx| {
                    this.last_equal_ranges.clear();
                    if let Err(error) = result {
                        this.status = CodegenStatus::Error(error);
                    } else {
                        this.status = CodegenStatus::Done;
                    }
                    cx.emit(CodegenEvent::Finished);
                    cx.notify();
                })
                .ok();
            }
        });
        cx.notify();
    }

    pub fn stop(&mut self, cx: &mut ModelContext<Self>) {
        self.last_equal_ranges.clear();
        self.status = CodegenStatus::Done;
        self.generation = Task::ready(());
        cx.emit(CodegenEvent::Finished);
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(transaction_id) = self.transaction_id.take() {
            self.buffer
                .update(cx, |buffer, cx| buffer.undo_transaction(transaction_id, cx));
        }
    }

    fn update_diff(&mut self, cx: &mut ModelContext<Self>) {
        if self.diff.task.is_some() {
            self.diff.should_update = true;
        } else {
            self.diff.should_update = false;

            let old_snapshot = self.snapshot.clone();
            let old_range = self.range.to_point(&old_snapshot);
            let new_snapshot = self.buffer.read(cx).snapshot(cx);
            let new_range = self.range.to_point(&new_snapshot);

            self.diff.task = Some(cx.spawn(|this, mut cx| async move {
                let (deleted_row_ranges, inserted_row_ranges) = cx
                    .background_executor()
                    .spawn(async move {
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

                        let mut old_row = old_range.start.row;
                        let mut new_row = new_range.start.row;
                        let diff = TextDiff::from_lines(old_text.as_str(), new_text.as_str());

                        let mut deleted_row_ranges: Vec<(Anchor, RangeInclusive<u32>)> = Vec::new();
                        let mut inserted_row_ranges = Vec::new();
                        for change in diff.iter_all_changes() {
                            let line_count = change.value().lines().count() as u32;
                            match change.tag() {
                                similar::ChangeTag::Equal => {
                                    old_row += line_count;
                                    new_row += line_count;
                                }
                                similar::ChangeTag::Delete => {
                                    let old_end_row = old_row + line_count - 1;
                                    let new_row =
                                        new_snapshot.anchor_before(Point::new(new_row, 0));

                                    if let Some((_, last_deleted_row_range)) =
                                        deleted_row_ranges.last_mut()
                                    {
                                        if *last_deleted_row_range.end() + 1 == old_row {
                                            *last_deleted_row_range =
                                                *last_deleted_row_range.start()..=old_end_row;
                                        } else {
                                            deleted_row_ranges
                                                .push((new_row, old_row..=old_end_row));
                                        }
                                    } else {
                                        deleted_row_ranges.push((new_row, old_row..=old_end_row));
                                    }

                                    old_row += line_count;
                                }
                                similar::ChangeTag::Insert => {
                                    let new_end_row = new_row + line_count - 1;
                                    let start = new_snapshot.anchor_before(Point::new(new_row, 0));
                                    let end = new_snapshot.anchor_before(Point::new(
                                        new_end_row,
                                        new_snapshot.line_len(MultiBufferRow(new_end_row)),
                                    ));
                                    inserted_row_ranges.push(start..=end);
                                    new_row += line_count;
                                }
                            }
                        }

                        (deleted_row_ranges, inserted_row_ranges)
                    })
                    .await;

                this.update(&mut cx, |this, cx| {
                    this.diff.deleted_row_ranges = deleted_row_ranges;
                    this.diff.inserted_row_ranges = inserted_row_ranges;
                    this.diff.task = None;
                    if this.diff.should_update {
                        this.update_diff(cx);
                    }
                    cx.notify();
                })
                .ok();
            }));
        }
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
    use std::sync::Arc;

    use crate::FakeCompletionProvider;

    use super::*;
    use futures::stream::{self};
    use gpui::{Context, TestAppContext};
    use indoc::indoc;
    use language::{
        language_settings, tree_sitter_rust, Buffer, Language, LanguageConfig, LanguageMatcher,
        Point,
    };
    use rand::prelude::*;
    use serde::Serialize;
    use settings::SettingsStore;

    #[derive(Serialize)]
    pub struct DummyCompletionRequest {
        pub name: String,
    }

    #[gpui::test(iterations = 10)]
    async fn test_transform_autoindent(cx: &mut TestAppContext, mut rng: StdRng) {
        let provider = FakeCompletionProvider::default();
        cx.set_global(cx.update(SettingsStore::test));
        cx.set_global(CompletionProvider::Fake(provider.clone()));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                let x = 0;
                for _ in 0..10 {
                    x += 1;
                }
            }
        "};
        let buffer =
            cx.new_model(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(4, 5))
        });
        let codegen = cx.new_model(|cx| Codegen::new(buffer.clone(), range, None, cx));

        let request = LanguageModelRequest::default();
        codegen.update(cx, |codegen, cx| codegen.start(request, cx));

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
            provider.send_completion(chunk.into());
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        provider.finish_completion();
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
        let provider = FakeCompletionProvider::default();
        cx.set_global(CompletionProvider::Fake(provider.clone()));
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                le
            }
        "};
        let buffer =
            cx.new_model(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 6))..snapshot.anchor_after(Point::new(1, 6))
        });
        let codegen = cx.new_model(|cx| Codegen::new(buffer.clone(), range, None, cx));

        let request = LanguageModelRequest::default();
        codegen.update(cx, |codegen, cx| codegen.start(request, cx));

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
            provider.send_completion(chunk.into());
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        provider.finish_completion();
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
        let provider = FakeCompletionProvider::default();
        cx.set_global(CompletionProvider::Fake(provider.clone()));
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = concat!(
            "fn main() {\n",
            "  \n",
            "}\n" //
        );
        let buffer =
            cx.new_model(|cx| Buffer::local(text, cx).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 2))..snapshot.anchor_after(Point::new(1, 2))
        });
        let codegen = cx.new_model(|cx| Codegen::new(buffer.clone(), range, None, cx));

        let request = LanguageModelRequest::default();
        codegen.update(cx, |codegen, cx| codegen.start(request, cx));

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
            provider.send_completion(chunk.into());
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        provider.finish_completion();
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
            Some(tree_sitter_rust::language()),
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
