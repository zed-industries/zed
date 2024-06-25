use std::cmp;
use std::sync::Arc;

use client::telemetry::Telemetry;
use collections::{HashMap, VecDeque};
use editor::actions::MoveDown;
use editor::actions::MoveUp;
use editor::actions::SelectAll;
use editor::Editor;
use editor::EditorElement;
use editor::EditorEvent;
use editor::EditorMode;
use editor::EditorStyle;
use editor::MultiBuffer;
use gpui::EventEmitter;
use gpui::FocusHandle;
use gpui::FocusableView;
use gpui::FontStyle;
use gpui::FontWeight;
use gpui::Model;
use gpui::Subscription;
use gpui::TextStyle;
use gpui::UpdateGlobal;
use gpui::WhiteSpace;
use gpui::{AppContext, Global, View, WeakView};
use language::Buffer;
use settings::Settings;
use terminal_view::TerminalView;
use theme::ThemeSettings;
use ui::prelude::*;
use ui::Tooltip;
use util::ResultExt;
use workspace::Workspace;

use crate::AssistantPanel;

pub fn init(telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    cx.set_global(TerminalInlineAssistant::new(telemetry));
}

const PROMPT_HISTORY_MAX_LEN: usize = 20;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct TerminalInlineAssistId(usize);

impl TerminalInlineAssistId {
    fn post_inc(&mut self) -> TerminalInlineAssistId {
        let id = *self;
        self.0 += 1;
        id
    }
}

pub struct TerminalInlineAssistant {
    next_assist_id: TerminalInlineAssistId,
    assists: HashMap<TerminalInlineAssistId, TerminalInlineAssist>,
    assists_by_editor: HashMap<WeakView<TerminalView>, TerminalInlineAssistId>,
    prompt_history: VecDeque<String>,
    telemetry: Option<Arc<Telemetry>>,
}

impl Global for TerminalInlineAssistant {}

impl TerminalInlineAssistant {
    pub fn new(telemetry: Arc<Telemetry>) -> Self {
        Self {
            next_assist_id: TerminalInlineAssistId::default(),
            assists: HashMap::default(),
            assists_by_editor: HashMap::default(),
            prompt_history: VecDeque::default(),
            telemetry: Some(telemetry),
        }
    }

    pub fn assist(
        &mut self,
        terminal: &View<TerminalView>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) {
        let assist_id = self.next_assist_id.post_inc();
        let prompt_buffer = cx.new_model(|cx| Buffer::local("", cx));
        let prompt_buffer = cx.new_model(|cx| MultiBuffer::singleton(prompt_buffer, cx));

        let prompt_editor = cx.new_view(|cx| {
            PromptEditor::new(
                assist_id,
                self.prompt_history.clone(),
                prompt_buffer.clone(),
                workspace.clone(),
                cx,
            )
        });
        let prompt_editor_render = prompt_editor.clone();
        let block = terminal_view::BlockProperties {
            height: 1,
            render: Box::new(move |_| prompt_editor_render.clone().into_any_element()),
        };
        terminal.update(cx, |terminal, cx| {
            terminal.set_prompt(block, cx);
        });

        let terminal_assistant =
            TerminalInlineAssist::new(assist_id, terminal, prompt_editor, workspace.clone(), cx);

        self.assists.insert(assist_id, terminal_assistant);

        self.focus_assist(assist_id, cx);
    }

    fn focus_assist(&mut self, assist_id: TerminalInlineAssistId, cx: &mut WindowContext) {
        let assist = &self.assists[&assist_id];
        assist.prompt_editor.update(cx, |this, cx| {
            this.editor.update(cx, |editor, cx| {
                editor.focus(cx);
                editor.select_all(&SelectAll, cx);
            });
        })
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
                // self.start_assist(assist_id, cx);
            }
            PromptEditorEvent::StopRequested => {
                // self.stop_assist(assist_id, cx);
            }
            PromptEditorEvent::ConfirmRequested => {
                // self.finish_assist(assist_id, false, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, cx);
            }
            PromptEditorEvent::DismissRequested => {
                self.dismiss_assist(assist_id, cx);
            }
            PromptEditorEvent::Resized { height_in_lines } => {
                self.insert_prompt_editor_into_terminal(assist_id, *height_in_lines, cx);
            }
        }
    }

    fn finish_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        undo: bool,
        cx: &mut WindowContext,
    ) {
        self.dismiss_assist(assist_id, cx);

        if let Some(assist) = self.assists.remove(&assist_id) {
            assist
                .terminal
                .update(cx, |this, cx| {
                    this.clear_prompt(cx);
                    this.focus_handle(cx).focus(cx);
                })
                .log_err();

            // if undo {
            //     assist.codegen.update(cx, |codegen, cx| codegen.undo(cx));
            // }
        }
    }

    fn dismiss_assist(
        &mut self,
        assist_id: TerminalInlineAssistId,
        cx: &mut WindowContext,
    ) -> bool {
        let Some(assist) = self.assists.get_mut(&assist_id) else {
            return false;
        };
        let Some(terminal) = assist.terminal.upgrade() else {
            return false;
        };

        true
    }

    fn insert_prompt_editor_into_terminal(
        &mut self,
        assist_id: TerminalInlineAssistId,
        height: u8,
        cx: &mut WindowContext,
    ) {
        if let Some(assist) = self.assists.get_mut(&assist_id) {
            let prompt_editor = assist.prompt_editor.clone();
            assist
                .terminal
                .update(cx, |terminal, cx| {
                    terminal.clear_prompt(cx);
                    let block = terminal_view::BlockProperties {
                        height,
                        render: Box::new(move |_| prompt_editor.clone().into_any_element()),
                    };
                    terminal.set_prompt(block, cx);
                })
                .log_err();
        }
    }
}

struct TerminalInlineAssist {
    terminal: WeakView<TerminalView>,
    prompt_editor: View<PromptEditor>,
    workspace: Option<WeakView<Workspace>>,
    _subscriptions: Vec<Subscription>,
}

impl TerminalInlineAssist {
    pub fn new(
        assist_id: TerminalInlineAssistId,
        terminal: &View<TerminalView>,
        prompt_editor: View<PromptEditor>,
        workspace: Option<WeakView<Workspace>>,
        cx: &mut WindowContext,
    ) -> Self {
        Self {
            terminal: terminal.downgrade(),
            prompt_editor: prompt_editor.clone(),
            workspace: workspace.clone(),
            _subscriptions: vec![cx.subscribe(&prompt_editor, |prompt_editor, event, cx| {
                TerminalInlineAssistant::update_global(cx, |this, cx| {
                    this.handle_prompt_editor_event(prompt_editor, event, cx)
                })
            })],
        }
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
    id: TerminalInlineAssistId,
    height_in_lines: u8,
    editor: View<Editor>,
    edited_since_done: bool,
    prompt_history: VecDeque<String>,
    prompt_history_ix: Option<usize>,
    pending_prompt: String,
    // codegen: Model<Codegen>,
    workspace: Option<WeakView<Workspace>>,
    // _codegen_subscription: Subscription,
    editor_subscriptions: Vec<Subscription>,
}

impl PromptEditor {
    const MAX_LINES: u8 = 8;

    fn new(
        id: TerminalInlineAssistId,
        prompt_history: VecDeque<String>,
        prompt_buffer: Model<MultiBuffer>,
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
            editor.set_placeholder_text("Add a prompt…", cx);
            editor
        });
        let mut this = Self {
            id,
            height_in_lines: 1,
            editor: prompt_editor,
            edited_since_done: false,
            prompt_history,
            prompt_history_ix: None,
            pending_prompt: String::new(),
            editor_subscriptions: Vec::new(),
            workspace,
        };
        this.count_lines(cx);
        this.subscribe_to_editor(cx);
        this
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

    fn subscribe_to_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.editor_subscriptions.clear();
        self.editor_subscriptions
            .push(cx.observe(&self.editor, Self::handle_prompt_editor_changed));
        self.editor_subscriptions
            .push(cx.subscribe(&self.editor, Self::handle_prompt_editor_events));
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

    fn cancel(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(PromptEditorEvent::CancelRequested);
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

impl FocusableView for PromptEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl EventEmitter<PromptEditorEvent> for PromptEditor {}

impl Render for PromptEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().debug().h_full().w_full().justify_end().child(
            h_flex()
                .bg(cx.theme().colors().editor_background)
                .border_y_1()
                .border_color(cx.theme().status().info_border)
                .py_1p5()
                .w_full()
                // .on_action(cx.listener(Self::confirm))
                .on_action(cx.listener(Self::cancel))
                .on_action(cx.listener(Self::move_up))
                .on_action(cx.listener(Self::move_down))
                .child(
                    h_flex()
                        .w_10() //TODO
                        // .w(gutter_dimensions.full_width() + (gutter_dimensions.margin / 2.0))
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
                        })), // .children(
                             //     if let CodegenStatus::Error(error) = &self.codegen.read(cx).status {
                             //         let error_message = SharedString::from(error.to_string());
                             //         Some(
                             //             div()
                             //                 .id("error")
                             //                 .tooltip(move |cx| Tooltip::text(error_message.clone(), cx))
                             //                 .child(
                             //                     Icon::new(IconName::XCircle)
                             //                         .size(IconSize::Small)
                             //                         .color(Color::Error),
                             //                 ),
                             //         )
                             //     } else {
                             //         None
                             //     },
                             // ),
                )
                .child(div().flex_1().child(self.render_prompt_editor(cx))), // .child(h_flex().gap_2().pr_4().children(buttons)),
        )
    }
}
