use crate::{
    codegen::{self, Codegen, CodegenKind},
    prompts::generate_content_prompt,
    AssistantPanel, CompletionProvider, LanguageModelRequest, LanguageModelRequestMessage, Role,
};
use client::telemetry::Telemetry;
use collections::{hash_map, HashMap, HashSet, VecDeque};
use editor::{
    actions::{MoveDown, MoveUp},
    display_map::{BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle},
    scroll::{Autoscroll, AutoscrollStrategy},
    Anchor, Editor, EditorElement, EditorEvent, EditorStyle, GutterDimensions, MultiBufferSnapshot,
    ToPoint,
};
use gpui::{
    AppContext, EventEmitter, FocusHandle, FocusableView, FontStyle, FontWeight, Global,
    HighlightStyle, Model, Subscription, TextStyle, UpdateGlobal, View, ViewContext, WeakView,
    WhiteSpace, WindowContext,
};
use language::Point;
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use settings::Settings;
use std::{ops::Range, sync::Arc};
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

        let measurements = Arc::new(Mutex::new(GutterDimensions::default()));
        let inline_assistant = cx.new_view(|cx| {
            InlineAssistEditor::new(
                inline_assist_id,
                measurements.clone(),
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
                    style: BlockStyle::Flex,
                    position: snapshot.anchor_before(Point::new(point_selection.head().row, 0)),
                    height: 2,
                    render: Box::new({
                        let inline_assistant = inline_assistant.clone();
                        move |cx: &mut BlockContext| {
                            *measurements.lock() = *cx.gutter_dimensions;
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

        self.pending_assists.insert(
            inline_assist_id,
            PendingInlineAssist {
                include_conversation,
                editor: editor.downgrade(),
                inline_assistant: Some((block_id, inline_assistant.clone())),
                codegen: codegen.clone(),
                workspace,
                _subscriptions: vec![
                    cx.subscribe(&inline_assistant, |inline_assistant, event, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            this.handle_inline_assistant_event(inline_assistant, event, cx)
                        })
                    }),
                    cx.subscribe(editor, {
                        let inline_assistant = inline_assistant.downgrade();
                        move |editor, event, cx| {
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
                            codegen::Event::Undone => {
                                this.finish_inline_assist(inline_assist_id, false, cx)
                            }
                            codegen::Event::Finished => {
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
                                    if pending_assist.inline_assistant.is_none() {
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
            .or_default()
            .push(inline_assist_id);
        self.update_highlights_for_editor(editor, cx);
    }

    fn handle_inline_assistant_event(
        &mut self,
        inline_assistant: View<InlineAssistEditor>,
        event: &InlineAssistEditorEvent,
        cx: &mut WindowContext,
    ) {
        let assist_id = inline_assistant.read(cx).id;
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
        }
    }

    pub fn cancel_last_inline_assist(&mut self, cx: &mut WindowContext) -> bool {
        for (editor, assist_ids) in &self.pending_assist_ids_by_editor {
            if let Some(editor) = editor.upgrade() {
                if editor.read(cx).is_focused(cx) {
                    if let Some(assist_id) = assist_ids.last().copied() {
                        self.finish_inline_assist(assist_id, true, cx);
                        return true;
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

    fn hide_inline_assist(&mut self, assist_id: InlineAssistId, cx: &mut WindowContext) {
        if let Some(pending_assist) = self.pending_assists.get_mut(&assist_id) {
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
            .unwrap_or(&empty_inline_assist_ids);

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
}

struct InlineAssistEditor {
    id: InlineAssistId,
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
            .child(h_flex().flex_1().child(self.render_prompt_editor(cx)))
    }
}

impl FocusableView for InlineAssistEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.prompt_editor.focus_handle(cx)
    }
}

impl InlineAssistEditor {
    #[allow(clippy::too_many_arguments)]
    fn new(
        id: InlineAssistId,
        gutter_dimensions: Arc<Mutex<GutterDimensions>>,
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
            gutter_dimensions,
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
    inline_assistant: Option<(BlockId, View<InlineAssistEditor>)>,
    codegen: Model<Codegen>,
    _subscriptions: Vec<Subscription>,
    workspace: Option<WeakView<Workspace>>,
    include_conversation: bool,
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
