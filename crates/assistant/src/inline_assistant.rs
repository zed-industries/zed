use crate::{
    prompts::generate_content_prompt, AssistantPanel, CompletionProvider, Hunk,
    LanguageModelRequest, LanguageModelRequestMessage, Role, StreamingDiff,
};
use anyhow::Result;
use client::telemetry::Telemetry;
use collections::{hash_map, HashMap, HashSet, VecDeque};
use editor::{
    actions::{MoveDown, MoveUp},
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock,
    },
    scroll::{Autoscroll, AutoscrollStrategy},
    Anchor, Editor, EditorElement, EditorEvent, EditorStyle, GutterDimensions, MultiBuffer,
    MultiBufferSnapshot, ToOffset, ToPoint,
};
use futures::{channel::mpsc, SinkExt, Stream, StreamExt};
use gpui::{
    AnyWindowHandle, AppContext, EventEmitter, FocusHandle, FocusableView, FontStyle, FontWeight,
    Global, HighlightStyle, Model, ModelContext, Subscription, Task, TextStyle, UpdateGlobal, View,
    ViewContext, WeakView, WhiteSpace, WindowContext,
};
use language::{Point, TransactionId};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use rope::Rope;
use settings::Settings;
use std::{cmp, future, ops::Range, sync::Arc, time::Instant};
use theme::ThemeSettings;
use ui::{prelude::*, Tooltip};
use workspace::{notifications::NotificationId, Toast, Workspace};

pub fn init(telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    cx.set_global(InlineAssistant::new(telemetry));
}

const PROMPT_HISTORY_MAX_LEN: usize = 20;

pub struct InlineAssistant {
    next_assist_id: InlineAssistId,
    pending_assists: HashMap<InlineAssistId, PendingInlineAssist>,
    pending_assist_ids_by_editor: HashMap<WeakView<Editor>, EditorPendingAssists>,
    prompt_history: VecDeque<String>,
    telemetry: Option<Arc<Telemetry>>,
}

struct EditorPendingAssists {
    window: AnyWindowHandle,
    assist_ids: Vec<InlineAssistId>,
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
        include_conversation: bool,
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

        let inline_assist_id = self.next_assist_id.post_inc();
        let codegen = cx.new_model(|cx| {
            Codegen::new(
                editor.read(cx).buffer().clone(),
                codegen_kind,
                self.telemetry.clone(),
                cx,
            )
        });

        let gutter_dimensions = Arc::new(Mutex::new(GutterDimensions::default()));
        let inline_assist_editor = cx.new_view(|cx| {
            InlineAssistEditor::new(
                inline_assist_id,
                gutter_dimensions.clone(),
                self.prompt_history.clone(),
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
                    style: BlockStyle::Sticky,
                    position: snapshot.anchor_before(Point::new(point_selection.head().row, 0)),
                    height: inline_assist_editor.read(cx).height_in_lines,
                    render: build_inline_assist_editor_renderer(
                        &inline_assist_editor,
                        gutter_dimensions,
                    ),
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

        self.pending_assists.insert(
            inline_assist_id,
            PendingInlineAssist {
                include_conversation,
                editor: editor.downgrade(),
                inline_assist_editor: Some((block_id, inline_assist_editor.clone())),
                codegen: codegen.clone(),
                workspace,
                _subscriptions: vec![
                    cx.subscribe(&inline_assist_editor, |inline_assist_editor, event, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            this.handle_inline_assistant_event(inline_assist_editor, event, cx)
                        })
                    }),
                    cx.subscribe(editor, {
                        let inline_assist_editor = inline_assist_editor.downgrade();
                        move |editor, event, cx| {
                            if let Some(inline_assist_editor) = inline_assist_editor.upgrade() {
                                if let EditorEvent::SelectionsChanged { local } = event {
                                    if *local
                                        && inline_assist_editor
                                            .focus_handle(cx)
                                            .contains_focused(cx)
                                    {
                                        cx.focus_view(&editor);
                                    }
                                }
                            }
                        }
                    }),
                    cx.observe(&codegen, {
                        let editor = editor.downgrade();
                        move |_, cx| {
                            if let Some(editor) = editor.upgrade() {
                                InlineAssistant::update_global(cx, |this, cx| {
                                    this.update_highlights_for_editor(&editor, cx);
                                })
                            }
                        }
                    }),
                    cx.subscribe(&codegen, move |codegen, event, cx| {
                        InlineAssistant::update_global(cx, |this, cx| match event {
                            CodegenEvent::Undone => {
                                this.finish_inline_assist(inline_assist_id, false, cx)
                            }
                            CodegenEvent::Finished => {
                                let pending_assist = if let Some(pending_assist) =
                                    this.pending_assists.get(&inline_assist_id)
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
                                    if pending_assist.inline_assist_editor.is_none() {
                                        if let Some(workspace) = pending_assist
                                            .workspace
                                            .as_ref()
                                            .and_then(|workspace| workspace.upgrade())
                                        {
                                            workspace.update(cx, |workspace, cx| {
                                                struct InlineAssistantError;

                                                let id = NotificationId::identified::<
                                                    InlineAssistantError,
                                                >(
                                                    inline_assist_id.0
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
                        })
                    }),
                ],
            },
        );

        self.pending_assist_ids_by_editor
            .entry(editor.downgrade())
            .or_insert_with(|| EditorPendingAssists {
                window: cx.window_handle(),
                assist_ids: Vec::new(),
            })
            .assist_ids
            .push(inline_assist_id);
        self.update_highlights_for_editor(editor, cx);
    }

    fn handle_inline_assistant_event(
        &mut self,
        inline_assist_editor: View<InlineAssistEditor>,
        event: &InlineAssistEditorEvent,
        cx: &mut WindowContext,
    ) {
        let assist_id = inline_assist_editor.read(cx).id;
        match event {
            InlineAssistEditorEvent::Confirmed { prompt } => {
                self.confirm_inline_assist(assist_id, prompt, cx);
            }
            InlineAssistEditorEvent::Canceled => {
                self.finish_inline_assist(assist_id, true, cx);
            }
            InlineAssistEditorEvent::Dismissed => {
                self.hide_inline_assist(assist_id, cx);
            }
            InlineAssistEditorEvent::Resized { height_in_lines } => {
                self.resize_inline_assist(assist_id, *height_in_lines, cx);
            }
        }
    }

    pub fn cancel_last_inline_assist(&mut self, cx: &mut WindowContext) -> bool {
        for (editor, pending_assists) in &self.pending_assist_ids_by_editor {
            if pending_assists.window == cx.window_handle() {
                if let Some(editor) = editor.upgrade() {
                    if editor.read(cx).is_focused(cx) {
                        if let Some(assist_id) = pending_assists.assist_ids.last().copied() {
                            self.finish_inline_assist(assist_id, true, cx);
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn finish_inline_assist(
        &mut self,
        assist_id: InlineAssistId,
        undo: bool,
        cx: &mut WindowContext,
    ) {
        self.hide_inline_assist(assist_id, cx);

        if let Some(pending_assist) = self.pending_assists.remove(&assist_id) {
            if let hash_map::Entry::Occupied(mut entry) = self
                .pending_assist_ids_by_editor
                .entry(pending_assist.editor.clone())
            {
                entry.get_mut().assist_ids.retain(|id| *id != assist_id);
                if entry.get().assist_ids.is_empty() {
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

    fn hide_inline_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        if let Some(pending_assist) = self.pending_assists.get_mut(&assist_id) {
            if let Some(editor) = pending_assist.editor.upgrade() {
                if let Some((block_id, inline_assist_editor)) =
                    pending_assist.inline_assist_editor.take()
                {
                    editor.update(cx, |editor, cx| {
                        editor.remove_blocks(HashSet::from_iter([block_id]), None, cx);
                        if inline_assist_editor.focus_handle(cx).contains_focused(cx) {
                            editor.focus(cx);
                        }
                    });
                }
            }
        }
    }

    fn resize_inline_assist(
        &mut self,
        assist_id: InlineAssistId,
        height_in_lines: u8,
        cx: &mut WindowContext,
    ) {
        if let Some(pending_assist) = self.pending_assists.get_mut(&assist_id) {
            if let Some(editor) = pending_assist.editor.upgrade() {
                if let Some((block_id, inline_assist_editor)) =
                    pending_assist.inline_assist_editor.as_ref()
                {
                    let gutter_dimensions = inline_assist_editor.read(cx).gutter_dimensions.clone();
                    let mut new_blocks = HashMap::default();
                    new_blocks.insert(
                        *block_id,
                        (
                            Some(height_in_lines),
                            build_inline_assist_editor_renderer(
                                inline_assist_editor,
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

    fn confirm_inline_assist(
        &mut self,
        assist_id: InlineAssistId,
        user_prompt: &str,
        cx: &mut WindowContext,
    ) {
        let pending_assist = if let Some(pending_assist) = self.pending_assists.get_mut(&assist_id)
        {
            pending_assist
        } else {
            return;
        };

        let conversation = if pending_assist.include_conversation {
            pending_assist.workspace.as_ref().and_then(|workspace| {
                let workspace = workspace.upgrade()?.read(cx);
                let assistant_panel = workspace.panel::<AssistantPanel>(cx)?;
                assistant_panel.read(cx).active_conversation(cx)
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

        self.prompt_history.retain(|prompt| prompt != user_prompt);
        self.prompt_history.push_back(user_prompt.into());
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

        let user_prompt = user_prompt.to_string();

        let prompt = cx.background_executor().spawn(async move {
            let language_name = language_name.as_deref();
            generate_content_prompt(user_prompt, language_name, buffer, range, project_name)
        });

        let mut messages = Vec::new();
        if let Some(conversation) = conversation {
            let request = conversation.read(cx).to_completion_request(cx);
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

    fn update_highlights_for_editor(&self, editor: &View<Editor>, cx: &mut WindowContext) {
        let mut background_ranges = Vec::new();
        let mut foreground_ranges = Vec::new();
        let empty_inline_assist_ids = Vec::new();
        let inline_assist_ids = self
            .pending_assist_ids_by_editor
            .get(&editor.downgrade())
            .map_or(&empty_inline_assist_ids, |pending_assists| {
                &pending_assists.assist_ids
            });

        for inline_assist_id in inline_assist_ids {
            if let Some(pending_assist) = self.pending_assists.get(inline_assist_id) {
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
    Confirmed { prompt: String },
    Canceled,
    Dismissed,
    Resized { height_in_lines: u8 },
}

struct InlineAssistEditor {
    id: InlineAssistId,
    height_in_lines: u8,
    prompt_editor: View<Editor>,
    confirmed: bool,
    gutter_dimensions: Arc<Mutex<GutterDimensions>>,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    codegen: Model<Codegen>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<InlineAssistEditorEvent> for InlineAssistEditor {}

impl Render for InlineAssistEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let gutter_dimensions = *self.gutter_dimensions.lock();
        let icon_size = IconSize::default();
        h_flex()
            .w_full()
            .py_1p5()
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .child(
                h_flex()
                    .w(gutter_dimensions.full_width() + (gutter_dimensions.margin / 2.0))
                    .pr(gutter_dimensions.fold_area_width())
                    .justify_end()
                    .children(if let Some(error) = self.codegen.read(cx).error() {
                        let error_message = SharedString::from(error.to_string());
                        Some(
                            div()
                                .id("error")
                                .tooltip(move |cx| Tooltip::text(error_message.clone(), cx))
                                .child(
                                    Icon::new(IconName::XCircle)
                                        .size(icon_size)
                                        .color(Color::Error),
                                ),
                        )
                    } else {
                        None
                    }),
            )
            .child(div().flex_1().child(self.render_prompt_editor(cx)))
    }
}

impl FocusableView for InlineAssistEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.prompt_editor.focus_handle(cx)
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
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let prompt_editor = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(Self::MAX_LINES as usize, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
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
            cx.observe(&prompt_editor, Self::handle_prompt_editor_changed),
            cx.subscribe(&prompt_editor, Self::handle_prompt_editor_events),
        ];

        let mut this = Self {
            id,
            height_in_lines: 1,
            prompt_editor,
            confirmed: false,
            gutter_dimensions,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            codegen,
            _subscriptions: subscriptions,
        };
        this.count_lines(cx);
        this
    }

    fn count_lines(&mut self, cx: &mut ViewContext<Self>) {
        let height_in_lines = cmp::max(
            2, // Make the editor at least two lines tall, to account for padding.
            cmp::min(
                self.prompt_editor
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
        cx.emit(InlineAssistEditorEvent::Canceled);
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if self.confirmed {
            cx.emit(InlineAssistEditorEvent::Dismissed);
        } else {
            let prompt = self.prompt_editor.read(cx).text(cx);
            self.prompt_editor
                .update(cx, |editor, _cx| editor.set_read_only(true));
            cx.emit(InlineAssistEditorEvent::Confirmed { prompt });
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

struct PendingInlineAssist {
    editor: WeakView<Editor>,
    inline_assist_editor: Option<(BlockId, View<InlineAssistEditor>)>,
    codegen: Model<Codegen>,
    _subscriptions: Vec<Subscription>,
    workspace: Option<WeakView<Workspace>>,
    include_conversation: bool,
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

pub struct Codegen {
    buffer: Model<MultiBuffer>,
    snapshot: MultiBufferSnapshot,
    kind: CodegenKind,
    last_equal_ranges: Vec<Range<Anchor>>,
    transaction_id: Option<TransactionId>,
    error: Option<anyhow::Error>,
    generation: Task<()>,
    idle: bool,
    telemetry: Option<Arc<Telemetry>>,
    _subscription: gpui::Subscription,
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
        Self {
            buffer: buffer.clone(),
            snapshot,
            kind,
            last_equal_ranges: Default::default(),
            transaction_id: Default::default(),
            error: Default::default(),
            idle: true,
            generation: Task::ready(()),
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
        match &self.kind {
            CodegenKind::Transform { range } => range.clone(),
            CodegenKind::Generate { position } => position.bias_left(&self.snapshot)..*position,
        }
    }

    pub fn kind(&self) -> &CodegenKind {
        &self.kind
    }

    pub fn last_equal_ranges(&self) -> &[Range<Anchor>] {
        &self.last_equal_ranges
    }

    pub fn idle(&self) -> bool {
        self.idle
    }

    pub fn error(&self) -> Option<&anyhow::Error> {
        self.error.as_ref()
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

                            cx.notify();
                        })?;
                    }

                    diff.await?;

                    anyhow::Ok(())
                };

                let result = generate.await;
                this.update(&mut cx, |this, cx| {
                    this.last_equal_ranges.clear();
                    this.idle = true;
                    if let Err(error) = result {
                        this.error = Some(error);
                    }
                    cx.emit(CodegenEvent::Finished);
                    cx.notify();
                })
                .ok();
            }
        });
        self.error.take();
        self.idle = false;
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(transaction_id) = self.transaction_id {
            self.buffer
                .update(cx, |buffer, cx| buffer.undo_transaction(transaction_id, cx));
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
