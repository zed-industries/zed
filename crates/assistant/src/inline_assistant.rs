use crate::{
    prompts::generate_content_prompt, AssistantPanel, CompletionProvider, Hunk,
    LanguageModelRequest, LanguageModelRequestMessage, Role, StreamingDiff,
};
use anyhow::Result;
use client::telemetry::Telemetry;
use collections::{hash_map, HashMap, HashSet, VecDeque};
use editor::{
    actions::{MoveDown, MoveUp, SelectAll},
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock,
    },
    scroll::{Autoscroll, AutoscrollStrategy},
    Anchor, AnchorRangeExt, Editor, EditorElement, EditorEvent, EditorStyle, ExcerptRange,
    GutterDimensions, MultiBuffer, MultiBufferSnapshot, ToOffset, ToPoint,
};
use futures::{channel::mpsc, SinkExt, Stream, StreamExt};
use gpui::{
    AppContext, EventEmitter, FocusHandle, FocusableView, FontStyle, FontWeight, Global,
    HighlightStyle, Model, ModelContext, Subscription, Task, TextStyle, UpdateGlobal, View,
    ViewContext, WeakView, WhiteSpace, WindowContext,
};
use language::{Buffer, Point, TransactionId};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use rope::Rope;
use settings::Settings;
use similar::TextDiff;
use std::{
    cmp, future, mem,
    ops::{Range, RangeInclusive},
    sync::Arc,
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
    pending_assists: HashMap<InlineAssistId, PendingInlineAssist>,
    pending_assist_ids_by_editor: HashMap<WeakView<Editor>, Vec<InlineAssistId>>,
    prompt_history: VecDeque<String>,
    telemetry: Option<Arc<Telemetry>>,
}

impl Global for InlineAssistant {}

impl InlineAssistant {
    pub fn new(telemetry: Arc<Telemetry>) -> Self {
        Self {
            next_assist_id: InlineAssistId::default(),
            pending_assists: HashMap::default(),
            pending_assist_ids_by_editor: HashMap::default(),
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

        let assist_id = self.next_assist_id.post_inc();
        let codegen = cx.new_model(|cx| {
            Codegen::new(
                editor.read(cx).buffer().clone(),
                codegen_kind,
                self.telemetry.clone(),
                cx,
            )
        });

        let gutter_dimensions = Arc::new(Mutex::new(GutterDimensions::default()));
        let prompt_editor = cx.new_view(|cx| {
            InlineAssistEditor::new(
                assist_id,
                gutter_dimensions.clone(),
                self.prompt_history.clone(),
                codegen.clone(),
                workspace.clone(),
                cx,
            )
        });
        let (prompt_block_id, end_block_id) = editor.update(cx, |editor, cx| {
            let start_anchor = snapshot.anchor_before(point_selection.start);
            let end_anchor = snapshot.anchor_after(point_selection.end);
            editor.change_selections(Some(Autoscroll::newest()), cx, |selections| {
                selections.select_anchor_ranges([start_anchor..start_anchor])
            });
            let block_ids = editor.insert_blocks(
                [
                    BlockProperties {
                        style: BlockStyle::Sticky,
                        position: start_anchor,
                        height: prompt_editor.read(cx).height_in_lines,
                        render: build_inline_assist_editor_renderer(
                            &prompt_editor,
                            gutter_dimensions,
                        ),
                        disposition: BlockDisposition::Above,
                    },
                    BlockProperties {
                        style: BlockStyle::Sticky,
                        position: end_anchor,
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
                    },
                ],
                Some(Autoscroll::Strategy(AutoscrollStrategy::Newest)),
                cx,
            );
            (block_ids[0], block_ids[1])
        });

        self.pending_assists.insert(
            assist_id,
            PendingInlineAssist {
                include_context,
                editor: editor.downgrade(),
                editor_decorations: Some(PendingInlineAssistDecorations {
                    prompt_block_id,
                    prompt_editor: prompt_editor.clone(),
                    removed_line_block_ids: HashSet::default(),
                    end_block_id,
                }),
                codegen: codegen.clone(),
                workspace,
                _subscriptions: vec![
                    cx.subscribe(&prompt_editor, |inline_assist_editor, event, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            this.handle_inline_assistant_editor_event(
                                inline_assist_editor,
                                event,
                                cx,
                            )
                        })
                    }),
                    editor.update(cx, |editor, _cx| {
                        editor.register_action(
                            move |_: &editor::actions::Newline, cx: &mut WindowContext| {
                                InlineAssistant::update_global(cx, |this, cx| {
                                    this.handle_editor_newline(assist_id, cx)
                                })
                            },
                        )
                    }),
                    editor.update(cx, |editor, _cx| {
                        editor.register_action(
                            move |_: &editor::actions::Cancel, cx: &mut WindowContext| {
                                InlineAssistant::update_global(cx, |this, cx| {
                                    this.handle_editor_cancel(assist_id, cx)
                                })
                            },
                        )
                    }),
                    cx.subscribe(editor, move |editor, event, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            this.handle_editor_event(assist_id, editor, event, cx)
                        })
                    }),
                    cx.observe(&codegen, {
                        let editor = editor.downgrade();
                        move |_, cx| {
                            if let Some(editor) = editor.upgrade() {
                                InlineAssistant::update_global(cx, |this, cx| {
                                    this.update_editor_highlights(&editor, cx);
                                    this.update_editor_blocks(&editor, assist_id, cx);
                                })
                            }
                        }
                    }),
                    cx.subscribe(&codegen, move |codegen, event, cx| {
                        InlineAssistant::update_global(cx, |this, cx| match event {
                            CodegenEvent::Undone => this.finish_inline_assist(assist_id, false, cx),
                            CodegenEvent::Finished => {
                                let pending_assist = if let Some(pending_assist) =
                                    this.pending_assists.get(&assist_id)
                                {
                                    pending_assist
                                } else {
                                    return;
                                };

                                if let CodegenStatus::Error(error) = &codegen.read(cx).status {
                                    if pending_assist.editor_decorations.is_none() {
                                        if let Some(workspace) = pending_assist
                                            .workspace
                                            .as_ref()
                                            .and_then(|workspace| workspace.upgrade())
                                        {
                                            let error =
                                                format!("Inline assistant error: {}", error);
                                            workspace.update(cx, |workspace, cx| {
                                                struct InlineAssistantError;

                                                let id = NotificationId::identified::<
                                                    InlineAssistantError,
                                                >(
                                                    assist_id.0
                                                );

                                                workspace.show_toast(Toast::new(id, error), cx);
                                            })
                                        }
                                    }
                                }

                                if pending_assist.editor_decorations.is_none() {
                                    this.finish_inline_assist(assist_id, false, cx);
                                }
                            }
                        })
                    }),
                ],
            },
        );

        self.pending_assist_ids_by_editor
            .entry(editor.downgrade())
            .or_default()
            .push(assist_id);
        self.update_editor_highlights(editor, cx);
    }

    fn handle_inline_assistant_editor_event(
        &mut self,
        inline_assist_editor: View<InlineAssistEditor>,
        event: &InlineAssistEditorEvent,
        cx: &mut WindowContext,
    ) {
        let assist_id = inline_assist_editor.read(cx).id;
        match event {
            InlineAssistEditorEvent::StartRequested => {
                self.start_inline_assist(assist_id, cx);
            }
            InlineAssistEditorEvent::StopRequested => {
                self.stop_inline_assist(assist_id, cx);
            }
            InlineAssistEditorEvent::ConfirmRequested => {
                self.finish_inline_assist(assist_id, false, cx);
            }
            InlineAssistEditorEvent::CancelRequested => {
                self.finish_inline_assist(assist_id, true, cx);
            }
            InlineAssistEditorEvent::DismissRequested => {
                self.dismiss_inline_assist(assist_id, cx);
            }
            InlineAssistEditorEvent::Resized { height_in_lines } => {
                self.resize_inline_assist(assist_id, *height_in_lines, cx);
            }
        }
    }

    fn handle_editor_newline(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        let Some(assist) = self.pending_assists.get(&assist_id) else {
            return;
        };
        let Some(editor) = assist.editor.upgrade() else {
            return;
        };

        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        let assist_range = assist.codegen.read(cx).range().to_offset(&buffer);
        let editor = editor.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            if assist_range.contains(&selection.start) && assist_range.contains(&selection.end) {
                if matches!(assist.codegen.read(cx).status, CodegenStatus::Pending) {
                    self.dismiss_inline_assist(assist_id, cx);
                } else {
                    self.finish_inline_assist(assist_id, false, cx);
                }

                return;
            }
        }

        cx.propagate();
    }

    fn handle_editor_cancel(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        let Some(assist) = self.pending_assists.get(&assist_id) else {
            return;
        };
        let Some(editor) = assist.editor.upgrade() else {
            return;
        };

        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        let assist_range = assist.codegen.read(cx).range().to_offset(&buffer);
        let propagate = editor.update(cx, |editor, cx| {
            if let Some(decorations) = assist.editor_decorations.as_ref() {
                if editor.selections.count() == 1 {
                    let selection = editor.selections.newest::<usize>(cx);
                    if assist_range.contains(&selection.start)
                        && assist_range.contains(&selection.end)
                    {
                        editor.change_selections(Some(Autoscroll::newest()), cx, |selections| {
                            selections.select_ranges([assist_range.start..assist_range.start]);
                        });
                        decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                            prompt_editor.editor.update(cx, |prompt_editor, cx| {
                                prompt_editor.select_all(&SelectAll, cx);
                                prompt_editor.focus(cx);
                            });
                        });
                        return false;
                    }
                }
            }
            true
        });

        if propagate {
            cx.propagate();
        }
    }

    fn handle_editor_event(
        &mut self,
        assist_id: InlineAssistId,
        editor: View<Editor>,
        event: &EditorEvent,
        cx: &mut WindowContext,
    ) {
        let Some(assist) = self.pending_assists.get(&assist_id) else {
            return;
        };

        match event {
            EditorEvent::SelectionsChanged { local } if *local => {
                if let CodegenStatus::Idle = &assist.codegen.read(cx).status {
                    self.finish_inline_assist(assist_id, true, cx);
                }
            }
            EditorEvent::Saved => {
                if let CodegenStatus::Done = &assist.codegen.read(cx).status {
                    self.finish_inline_assist(assist_id, false, cx)
                }
            }
            EditorEvent::Edited { transaction_id }
                if matches!(
                    assist.codegen.read(cx).status,
                    CodegenStatus::Error(_) | CodegenStatus::Done
                ) =>
            {
                let buffer = editor.read(cx).buffer().read(cx);
                let edited_ranges =
                    buffer.edited_ranges_for_transaction::<usize>(*transaction_id, cx);
                let assist_range = assist.codegen.read(cx).range().to_offset(&buffer.read(cx));
                if edited_ranges
                    .iter()
                    .any(|range| range.overlaps(&assist_range))
                {
                    self.finish_inline_assist(assist_id, false, cx);
                }
            }
            _ => {}
        }
    }

    fn finish_inline_assist(
        &mut self,
        assist_id: InlineAssistId,
        undo: bool,
        cx: &mut WindowContext,
    ) {
        self.dismiss_inline_assist(assist_id, cx);

        if let Some(pending_assist) = self.pending_assists.remove(&assist_id) {
            if let hash_map::Entry::Occupied(mut entry) = self
                .pending_assist_ids_by_editor
                .entry(pending_assist.editor.clone())
            {
                entry.get_mut().retain(|id| *id != assist_id);
                if entry.get().is_empty() {
                    entry.remove();
                }
            }

            if let Some(editor) = pending_assist.editor.upgrade() {
                self.update_editor_highlights(&editor, cx);

                if undo {
                    pending_assist
                        .codegen
                        .update(cx, |codegen, cx| codegen.undo(cx));
                }
            }
        }
    }

    fn dismiss_inline_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) -> bool {
        let Some(pending_assist) = self.pending_assists.get_mut(&assist_id) else {
            return false;
        };
        let Some(editor) = pending_assist.editor.upgrade() else {
            return false;
        };
        let Some(decorations) = pending_assist.editor_decorations.take() else {
            return false;
        };

        editor.update(cx, |editor, cx| {
            let mut to_remove = decorations.removed_line_block_ids;
            to_remove.insert(decorations.prompt_block_id);
            to_remove.insert(decorations.end_block_id);
            editor.remove_blocks(to_remove, None, cx);
            if decorations
                .prompt_editor
                .focus_handle(cx)
                .contains_focused(cx)
            {
                editor.focus(cx);
            }
        });

        self.update_editor_highlights(&editor, cx);
        true
    }

    fn resize_inline_assist(
        &mut self,
        assist_id: InlineAssistId,
        height_in_lines: u8,
        cx: &mut WindowContext,
    ) {
        if let Some(pending_assist) = self.pending_assists.get_mut(&assist_id) {
            if let Some(editor) = pending_assist.editor.upgrade() {
                if let Some(decorations) = pending_assist.editor_decorations.as_ref() {
                    let gutter_dimensions =
                        decorations.prompt_editor.read(cx).gutter_dimensions.clone();
                    let mut new_blocks = HashMap::default();
                    new_blocks.insert(
                        decorations.prompt_block_id,
                        (
                            Some(height_in_lines),
                            build_inline_assist_editor_renderer(
                                &decorations.prompt_editor,
                                gutter_dimensions,
                            ),
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

    fn start_inline_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        let pending_assist = if let Some(pending_assist) = self.pending_assists.get_mut(&assist_id)
        {
            pending_assist
        } else {
            return;
        };

        pending_assist
            .codegen
            .update(cx, |codegen, cx| codegen.undo(cx));

        let Some(user_prompt) = pending_assist
            .editor_decorations
            .as_ref()
            .map(|decorations| decorations.prompt_editor.read(cx).prompt(cx))
        else {
            return;
        };

        let context = if pending_assist.include_context {
            pending_assist.workspace.as_ref().and_then(|workspace| {
                let workspace = workspace.upgrade()?.read(cx);
                let assistant_panel = workspace.panel::<AssistantPanel>(cx)?;
                assistant_panel.read(cx).active_context(cx)
            })
        } else {
            None
        };

        let editor = if let Some(editor) = pending_assist.editor.upgrade() {
            editor
        } else {
            return;
        };

        let project_name = pending_assist.workspace.as_ref().and_then(|workspace| {
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
                self.finish_inline_assist(assist_id, false, cx);
                return;
            }
        } else {
            self.finish_inline_assist(assist_id, false, cx);
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

    fn stop_inline_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        let pending_assist = if let Some(pending_assist) = self.pending_assists.get_mut(&assist_id)
        {
            pending_assist
        } else {
            return;
        };

        pending_assist
            .codegen
            .update(cx, |codegen, cx| codegen.stop(cx));
    }

    fn update_editor_highlights(&self, editor: &View<Editor>, cx: &mut WindowContext) {
        let mut gutter_pending_ranges = Vec::new();
        let mut gutter_transformed_ranges = Vec::new();
        let mut foreground_ranges = Vec::new();
        let mut inserted_row_ranges = Vec::new();
        let empty_assist_ids = Vec::new();
        let assist_ids = self
            .pending_assist_ids_by_editor
            .get(&editor.downgrade())
            .unwrap_or(&empty_assist_ids);

        for assist_id in assist_ids {
            if let Some(pending_assist) = self.pending_assists.get(assist_id) {
                let codegen = pending_assist.codegen.read(cx);
                foreground_ranges.extend(codegen.last_equal_ranges().iter().cloned());

                if codegen.edit_position != codegen.range().end {
                    gutter_pending_ranges.push(codegen.edit_position..codegen.range().end);
                }

                if codegen.range().start != codegen.edit_position {
                    gutter_transformed_ranges.push(codegen.range().start..codegen.edit_position);
                }

                if pending_assist.editor_decorations.is_some() {
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

            editor.clear_row_highlights::<PendingInlineAssist>();
            for row_range in inserted_row_ranges {
                editor.highlight_rows::<PendingInlineAssist>(
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
        let Some(pending_assist) = self.pending_assists.get_mut(&assist_id) else {
            return;
        };
        let Some(decorations) = pending_assist.editor_decorations.as_mut() else {
            return;
        };

        let codegen = pending_assist.codegen.read(cx);
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

fn build_inline_assist_editor_renderer(
    editor: &View<InlineAssistEditor>,
    gutter_dimensions: Arc<Mutex<GutterDimensions>>,
) -> RenderBlock {
    let editor = editor.clone();
    Box::new(move |cx: &mut BlockContext| {
        *gutter_dimensions.lock() = *cx.gutter_dimensions;
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

enum InlineAssistEditorEvent {
    StartRequested,
    StopRequested,
    ConfirmRequested,
    CancelRequested,
    DismissRequested,
    Resized { height_in_lines: u8 },
}

struct InlineAssistEditor {
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
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<InlineAssistEditorEvent> for InlineAssistEditor {}

impl Render for InlineAssistEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let gutter_dimensions = *self.gutter_dimensions.lock();

        let buttons = match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::for_action("Cancel Assist", &menu::Cancel, cx))
                        .on_click(cx.listener(|_, _, cx| {
                            cx.emit(InlineAssistEditorEvent::CancelRequested)
                        })),
                    IconButton::new("start", IconName::Sparkle)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .icon_size(IconSize::XSmall)
                        .tooltip(|cx| Tooltip::for_action("Transform", &menu::Confirm, cx))
                        .on_click(
                            cx.listener(|_, _, cx| {
                                cx.emit(InlineAssistEditorEvent::StartRequested)
                            }),
                        ),
                ]
            }
            CodegenStatus::Pending => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::text("Cancel Assist", cx))
                        .on_click(cx.listener(|_, _, cx| {
                            cx.emit(InlineAssistEditorEvent::CancelRequested)
                        })),
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
                            cx.listener(|_, _, cx| cx.emit(InlineAssistEditorEvent::StopRequested)),
                        ),
                ]
            }
            CodegenStatus::Error(_) | CodegenStatus::Done => {
                vec![
                    IconButton::new("cancel", IconName::Close)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::None)
                        .tooltip(|cx| Tooltip::for_action("Cancel Assist", &menu::Cancel, cx))
                        .on_click(cx.listener(|_, _, cx| {
                            cx.emit(InlineAssistEditorEvent::CancelRequested)
                        })),
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
                                cx.emit(InlineAssistEditorEvent::StartRequested);
                            }))
                    } else {
                        IconButton::new("confirm", IconName::Check)
                            .icon_color(Color::Info)
                            .size(ButtonSize::None)
                            .tooltip(|cx| Tooltip::for_action("Confirm Assist", &menu::Confirm, cx))
                            .on_click(cx.listener(|_, _, cx| {
                                cx.emit(InlineAssistEditorEvent::ConfirmRequested);
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

impl FocusableView for InlineAssistEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl InlineAssistEditor {
    const MAX_LINES: u8 = 8;

    #[allow(clippy::too_many_arguments)]
    fn new(
        id: InlineAssistId,
        gutter_dimensions: Arc<Mutex<GutterDimensions>>,
        prompt_history: VecDeque<String>,
        codegen: Model<Codegen>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let prompt_editor = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(Self::MAX_LINES as usize, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text("Add a prompt…", cx);
            editor
        });
        cx.focus_view(&prompt_editor);

        let subscriptions = vec![
            cx.observe(&codegen, Self::handle_codegen_changed),
            cx.observe(&prompt_editor, Self::handle_prompt_editor_changed),
            cx.subscribe(&prompt_editor, Self::handle_prompt_editor_events),
        ];

        let mut this = Self {
            id,
            height_in_lines: 1,
            editor: prompt_editor,
            edited_since_done: false,
            gutter_dimensions,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            codegen,
            workspace,
            _subscriptions: subscriptions,
        };
        this.count_lines(cx);
        this
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
            cx.emit(InlineAssistEditorEvent::Resized { height_in_lines });
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
                cx.emit(InlineAssistEditorEvent::CancelRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(InlineAssistEditorEvent::StopRequested);
            }
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match &self.codegen.read(cx).status {
            CodegenStatus::Idle => {
                cx.emit(InlineAssistEditorEvent::StartRequested);
            }
            CodegenStatus::Pending => {
                cx.emit(InlineAssistEditorEvent::DismissRequested);
            }
            CodegenStatus::Done | CodegenStatus::Error(_) => {
                if self.edited_since_done {
                    cx.emit(InlineAssistEditorEvent::StartRequested);
                } else {
                    cx.emit(InlineAssistEditorEvent::ConfirmRequested);
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

struct PendingInlineAssist {
    editor: WeakView<Editor>,
    editor_decorations: Option<PendingInlineAssistDecorations>,
    codegen: Model<Codegen>,
    _subscriptions: Vec<Subscription>,
    workspace: Option<WeakView<Workspace>>,
    include_context: bool,
}

struct PendingInlineAssistDecorations {
    prompt_block_id: BlockId,
    prompt_editor: View<InlineAssistEditor>,
    removed_line_block_ids: HashSet<BlockId>,
    end_block_id: BlockId,
}

#[derive(Debug)]
pub enum CodegenEvent {
    Finished,
    Undone,
}

#[derive(Clone)]
pub enum CodegenKind {
    Transform { range: Range<Anchor> },
    Generate { position: Anchor },
}

impl CodegenKind {
    fn range(&self, snapshot: &MultiBufferSnapshot) -> Range<Anchor> {
        match self {
            CodegenKind::Transform { range } => range.clone(),
            CodegenKind::Generate { position } => position.bias_left(snapshot)..*position,
        }
    }
}

pub struct Codegen {
    buffer: Model<MultiBuffer>,
    old_buffer: Model<Buffer>,
    snapshot: MultiBufferSnapshot,
    kind: CodegenKind,
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
        kind: CodegenKind,
        telemetry: Option<Arc<Telemetry>>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let snapshot = buffer.read(cx).snapshot(cx);

        let (old_buffer, _, _) = buffer
            .read(cx)
            .range_to_buffer_ranges(kind.range(&snapshot), cx)
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
            edit_position: kind.range(&snapshot).start,
            snapshot,
            kind,
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

    pub fn range(&self) -> Range<Anchor> {
        self.kind.range(&self.snapshot)
    }

    pub fn last_equal_ranges(&self) -> &[Range<Anchor>] {
        &self.last_equal_ranges
    }

    pub fn start(&mut self, prompt: LanguageModelRequest, cx: &mut ModelContext<Self>) {
        let range = self.range();
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
                                let chunks = strip_invalid_spans_from_codeblock(response.await?);
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
            let old_range = self.range().to_point(&old_snapshot);
            let new_snapshot = self.buffer.read(cx).snapshot(cx);
            let new_range = self.range().to_point(&new_snapshot);

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

fn strip_invalid_spans_from_codeblock(
    stream: impl Stream<Item = Result<String>>,
) -> impl Stream<Item = Result<String>> {
    let mut first_line = true;
    let mut buffer = String::new();
    let mut starts_with_markdown_codeblock = false;
    let mut includes_start_or_end_span = false;
    stream.filter_map(move |chunk| {
        let chunk = match chunk {
            Ok(chunk) => chunk,
            Err(err) => return future::ready(Some(Err(err))),
        };
        buffer.push_str(&chunk);

        if buffer.len() > "<|S|".len() && buffer.starts_with("<|S|") {
            includes_start_or_end_span = true;

            buffer = buffer
                .strip_prefix("<|S|>")
                .or_else(|| buffer.strip_prefix("<|S|"))
                .unwrap_or(&buffer)
                .to_string();
        } else if buffer.ends_with("|E|>") {
            includes_start_or_end_span = true;
        } else if buffer.starts_with("<|")
            || buffer.starts_with("<|S")
            || buffer.starts_with("<|S|")
            || buffer.ends_with('|')
            || buffer.ends_with("|E")
            || buffer.ends_with("|E|")
        {
            return future::ready(None);
        }

        if first_line {
            if buffer.is_empty() || buffer == "`" || buffer == "``" {
                return future::ready(None);
            } else if buffer.starts_with("```") {
                starts_with_markdown_codeblock = true;
                if let Some(newline_ix) = buffer.find('\n') {
                    buffer.replace_range(..newline_ix + 1, "");
                    first_line = false;
                } else {
                    return future::ready(None);
                }
            }
        }

        let mut text = buffer.to_string();
        if starts_with_markdown_codeblock {
            text = text
                .strip_suffix("\n```\n")
                .or_else(|| text.strip_suffix("\n```"))
                .or_else(|| text.strip_suffix("\n``"))
                .or_else(|| text.strip_suffix("\n`"))
                .or_else(|| text.strip_suffix('\n'))
                .unwrap_or(&text)
                .to_string();
        }

        if includes_start_or_end_span {
            text = text
                .strip_suffix("|E|>")
                .or_else(|| text.strip_suffix("E|>"))
                .or_else(|| text.strip_prefix("|>"))
                .or_else(|| text.strip_prefix('>'))
                .unwrap_or(&text)
                .to_string();
        };

        if text.contains('\n') {
            first_line = false;
        }

        let remainder = buffer.split_off(text.len());
        let result = if buffer.is_empty() {
            None
        } else {
            Some(Ok(buffer.clone()))
        };

        buffer = remainder;
        future::ready(result)
    })
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
        let codegen = cx.new_model(|cx| {
            Codegen::new(buffer.clone(), CodegenKind::Transform { range }, None, cx)
        });

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
        let position = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 6))
        });
        let codegen = cx.new_model(|cx| {
            Codegen::new(buffer.clone(), CodegenKind::Generate { position }, None, cx)
        });

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
        let position = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 2))
        });
        let codegen = cx.new_model(|cx| {
            Codegen::new(buffer.clone(), CodegenKind::Generate { position }, None, cx)
        });

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
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("Lorem ipsum dolor", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\nLorem ipsum dolor", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\nLorem ipsum dolor\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\nLorem ipsum dolor\n```\n", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks(
                "```html\n```js\nLorem ipsum dolor\n```\n```",
                2
            ))
            .map(|chunk| chunk.unwrap())
            .collect::<String>()
            .await,
            "```js\nLorem ipsum dolor\n```"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("``\nLorem ipsum dolor\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "``\nLorem ipsum dolor\n```"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("<|S|Lorem ipsum|E|>", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum"
        );

        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("<|S|>Lorem ipsum", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum"
        );

        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\n<|S|>Lorem ipsum\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\n<|S|Lorem ipsum|E|>\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum"
        );
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
