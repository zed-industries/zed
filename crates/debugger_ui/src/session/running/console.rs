use super::{
    stack_frame_list::{StackFrameList, StackFrameListEvent},
    variable_list::VariableList,
};
use alacritty_terminal::vte::ansi;
use anyhow::Result;
use collections::HashMap;
use dap::OutputEvent;
use editor::{Bias, CompletionProvider, Editor, EditorElement, EditorStyle, ExcerptId};
use fuzzy::StringMatchCandidate;
use gpui::{
    Action as _, AppContext, Context, Corner, Entity, FocusHandle, Focusable, HighlightStyle, Hsla,
    Render, Subscription, Task, TextStyle, WeakEntity, actions,
};
use language::{Buffer, CodeLabel, ToOffset};
use menu::Confirm;
use project::{
    Completion, CompletionResponse,
    debugger::session::{CompletionsQuery, OutputToken, Session, SessionEvent},
};
use settings::Settings;
use std::{cell::RefCell, ops::Range, rc::Rc, usize};
use theme::{Theme, ThemeSettings};
use ui::{ContextMenu, Divider, PopoverMenu, SplitButton, Tooltip, prelude::*};

actions!(console, [WatchExpression]);

pub struct Console {
    console: Entity<Editor>,
    query_bar: Entity<Editor>,
    session: Entity<Session>,
    _subscriptions: Vec<Subscription>,
    variable_list: Entity<VariableList>,
    stack_frame_list: Entity<StackFrameList>,
    last_token: OutputToken,
    update_output_task: Task<()>,
    focus_handle: FocusHandle,
}

impl Console {
    pub fn new(
        session: Entity<Session>,
        stack_frame_list: Entity<StackFrameList>,
        variable_list: Entity<VariableList>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let console = cx.new(|cx| {
            let mut editor = Editor::multi_line(window, cx);
            editor.move_to_end(&editor::actions::MoveToEnd, window, cx);
            editor.set_read_only(true);
            editor.disable_scrollbars_and_minimap(window, cx);
            editor.set_show_gutter(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_autoindent(false);
            editor.set_input_enabled(false);
            editor.set_use_autoclose(false);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_edit_predictions(Some(false), window, cx);
            editor.set_use_modal_editing(false);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor
        });
        let focus_handle = cx.focus_handle();

        let this = cx.weak_entity();
        let query_bar = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Evaluate an expression", cx);
            editor.set_use_autoclose(false);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Some(Rc::new(ConsoleQueryBarCompletionProvider(this))));

            editor
        });

        let _subscriptions = vec![
            cx.subscribe(&stack_frame_list, Self::handle_stack_frame_list_events),
            cx.subscribe_in(&session, window, |this, _, event, window, cx| {
                if let SessionEvent::ConsoleOutput = event {
                    this.update_output(window, cx)
                }
            }),
            cx.on_focus(&focus_handle, window, |console, window, cx| {
                if console.is_running(cx) {
                    console.query_bar.focus_handle(cx).focus(window);
                }
            }),
        ];

        Self {
            session,
            console,
            query_bar,
            variable_list,
            _subscriptions,
            stack_frame_list,
            update_output_task: Task::ready(()),
            last_token: OutputToken(0),
            focus_handle,
        }
    }

    #[cfg(test)]
    pub(crate) fn editor(&self) -> &Entity<Editor> {
        &self.console
    }

    fn is_running(&self, cx: &Context<Self>) -> bool {
        self.session.read(cx).is_started()
    }

    fn handle_stack_frame_list_events(
        &mut self,
        _: Entity<StackFrameList>,
        event: &StackFrameListEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            StackFrameListEvent::SelectedStackFrameChanged(_) => cx.notify(),
            StackFrameListEvent::BuiltEntries => {}
        }
    }

    pub(crate) fn show_indicator(&self, cx: &App) -> bool {
        self.session.read(cx).has_new_output(self.last_token)
    }

    pub fn add_messages<'a>(
        &mut self,
        events: impl Iterator<Item = &'a OutputEvent>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.console.update(cx, |console, cx| {
            console.set_read_only(false);

            for event in events {
                let to_insert = format!("{}\n", event.output.trim_end());

                let mut ansi_handler = ConsoleHandler::default();
                let mut ansi_processor = ansi::Processor::<ansi::StdSyncHandler>::default();

                let len = console.buffer().read(cx).len(cx);
                ansi_processor.advance(&mut ansi_handler, to_insert.as_bytes());
                let output = std::mem::take(&mut ansi_handler.output);
                let mut spans = std::mem::take(&mut ansi_handler.spans);
                let mut background_spans = std::mem::take(&mut ansi_handler.background_spans);
                if ansi_handler.current_range_start < output.len() {
                    spans.push((
                        ansi_handler.current_range_start..output.len(),
                        ansi_handler.current_color,
                    ));
                }
                if ansi_handler.current_background_range_start < output.len() {
                    background_spans.push((
                        ansi_handler.current_background_range_start..output.len(),
                        ansi_handler.current_background_color,
                    ));
                }
                console.move_to_end(&editor::actions::MoveToEnd, window, cx);
                console.insert(&output, window, cx);
                let buffer = console.buffer().read(cx).snapshot(cx);

                struct ConsoleAnsiHighlight;

                for (range, color) in spans {
                    let Some(color) = color else { continue };
                    let start_offset = len + range.start;
                    let range = start_offset..len + range.end;
                    let range = buffer.anchor_after(range.start)..buffer.anchor_before(range.end);
                    let style = HighlightStyle {
                        color: Some(terminal_view::terminal_element::convert_color(
                            &color,
                            cx.theme(),
                        )),
                        ..Default::default()
                    };
                    console.highlight_text_key::<ConsoleAnsiHighlight>(
                        start_offset,
                        vec![range],
                        style,
                        cx,
                    );
                }

                for (range, color) in background_spans {
                    let Some(color) = color else { continue };
                    let start_offset = len + range.start;
                    let range = start_offset..len + range.end;
                    let range = buffer.anchor_after(range.start)..buffer.anchor_before(range.end);

                    let color_fetcher: fn(&Theme) -> Hsla = match color {
                        // Named and theme defined colors
                        ansi::Color::Named(n) => match n {
                            ansi::NamedColor::Black => |theme| theme.colors().terminal_ansi_black,
                            ansi::NamedColor::Red => |theme| theme.colors().terminal_ansi_red,
                            ansi::NamedColor::Green => |theme| theme.colors().terminal_ansi_green,
                            ansi::NamedColor::Yellow => |theme| theme.colors().terminal_ansi_yellow,
                            ansi::NamedColor::Blue => |theme| theme.colors().terminal_ansi_blue,
                            ansi::NamedColor::Magenta => {
                                |theme| theme.colors().terminal_ansi_magenta
                            }
                            ansi::NamedColor::Cyan => |theme| theme.colors().terminal_ansi_cyan,
                            ansi::NamedColor::White => |theme| theme.colors().terminal_ansi_white,
                            ansi::NamedColor::BrightBlack => {
                                |theme| theme.colors().terminal_ansi_bright_black
                            }
                            ansi::NamedColor::BrightRed => {
                                |theme| theme.colors().terminal_ansi_bright_red
                            }
                            ansi::NamedColor::BrightGreen => {
                                |theme| theme.colors().terminal_ansi_bright_green
                            }
                            ansi::NamedColor::BrightYellow => {
                                |theme| theme.colors().terminal_ansi_bright_yellow
                            }
                            ansi::NamedColor::BrightBlue => {
                                |theme| theme.colors().terminal_ansi_bright_blue
                            }
                            ansi::NamedColor::BrightMagenta => {
                                |theme| theme.colors().terminal_ansi_bright_magenta
                            }
                            ansi::NamedColor::BrightCyan => {
                                |theme| theme.colors().terminal_ansi_bright_cyan
                            }
                            ansi::NamedColor::BrightWhite => {
                                |theme| theme.colors().terminal_ansi_bright_white
                            }
                            ansi::NamedColor::Foreground => {
                                |theme| theme.colors().terminal_foreground
                            }
                            ansi::NamedColor::Background => {
                                |theme| theme.colors().terminal_background
                            }
                            ansi::NamedColor::Cursor => |theme| theme.players().local().cursor,
                            ansi::NamedColor::DimBlack => {
                                |theme| theme.colors().terminal_ansi_dim_black
                            }
                            ansi::NamedColor::DimRed => {
                                |theme| theme.colors().terminal_ansi_dim_red
                            }
                            ansi::NamedColor::DimGreen => {
                                |theme| theme.colors().terminal_ansi_dim_green
                            }
                            ansi::NamedColor::DimYellow => {
                                |theme| theme.colors().terminal_ansi_dim_yellow
                            }
                            ansi::NamedColor::DimBlue => {
                                |theme| theme.colors().terminal_ansi_dim_blue
                            }
                            ansi::NamedColor::DimMagenta => {
                                |theme| theme.colors().terminal_ansi_dim_magenta
                            }
                            ansi::NamedColor::DimCyan => {
                                |theme| theme.colors().terminal_ansi_dim_cyan
                            }
                            ansi::NamedColor::DimWhite => {
                                |theme| theme.colors().terminal_ansi_dim_white
                            }
                            ansi::NamedColor::BrightForeground => {
                                |theme| theme.colors().terminal_bright_foreground
                            }
                            ansi::NamedColor::DimForeground => {
                                |theme| theme.colors().terminal_dim_foreground
                            }
                        },
                        // 'True' colors
                        ansi::Color::Spec(_) => |theme| theme.colors().editor_background,
                        // 8 bit, indexed colors
                        ansi::Color::Indexed(i) => {
                            match i {
                                // 0-15 are the same as the named colors above
                                0 => |theme| theme.colors().terminal_ansi_black,
                                1 => |theme| theme.colors().terminal_ansi_red,
                                2 => |theme| theme.colors().terminal_ansi_green,
                                3 => |theme| theme.colors().terminal_ansi_yellow,
                                4 => |theme| theme.colors().terminal_ansi_blue,
                                5 => |theme| theme.colors().terminal_ansi_magenta,
                                6 => |theme| theme.colors().terminal_ansi_cyan,
                                7 => |theme| theme.colors().terminal_ansi_white,
                                8 => |theme| theme.colors().terminal_ansi_bright_black,
                                9 => |theme| theme.colors().terminal_ansi_bright_red,
                                10 => |theme| theme.colors().terminal_ansi_bright_green,
                                11 => |theme| theme.colors().terminal_ansi_bright_yellow,
                                12 => |theme| theme.colors().terminal_ansi_bright_blue,
                                13 => |theme| theme.colors().terminal_ansi_bright_magenta,
                                14 => |theme| theme.colors().terminal_ansi_bright_cyan,
                                15 => |theme| theme.colors().terminal_ansi_bright_white,
                                // 16-231 are a 6x6x6 RGB color cube, mapped to 0-255 using steps defined by XTerm.
                                // See: https://github.com/xterm-x11/xterm-snapshots/blob/master/256colres.pl
                                // 16..=231 => {
                                //     let (r, g, b) = rgb_for_index(index as u8);
                                //     rgba_color(
                                //         if r == 0 { 0 } else { r * 40 + 55 },
                                //         if g == 0 { 0 } else { g * 40 + 55 },
                                //         if b == 0 { 0 } else { b * 40 + 55 },
                                //     )
                                // }
                                // 232-255 are a 24-step grayscale ramp from (8, 8, 8) to (238, 238, 238).
                                // 232..=255 => {
                                //     let i = index as u8 - 232; // Align index to 0..24
                                //     let value = i * 10 + 8;
                                //     rgba_color(value, value, value)
                                // }
                                // For compatibility with the alacritty::Colors interface
                                // See: https://github.com/alacritty/alacritty/blob/master/alacritty_terminal/src/term/color.rs
                                _ => |_| gpui::black(),
                            }
                        }
                    };

                    console.highlight_background_key::<ConsoleAnsiHighlight>(
                        start_offset,
                        &[range],
                        color_fetcher,
                        cx,
                    );
                }
            }

            console.set_read_only(true);
            cx.notify();
        });
    }

    pub fn watch_expression(
        &mut self,
        _: &WatchExpression,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let expression = self.query_bar.update(cx, |editor, cx| {
            let expression = editor.text(cx);
            cx.defer_in(window, |editor, window, cx| {
                editor.clear(window, cx);
            });

            expression
        });

        self.session.update(cx, |session, cx| {
            session
                .evaluate(
                    expression.clone(),
                    Some(dap::EvaluateArgumentsContext::Repl),
                    self.stack_frame_list.read(cx).opened_stack_frame_id(),
                    None,
                    cx,
                )
                .detach();

            if let Some(stack_frame_id) = self.stack_frame_list.read(cx).opened_stack_frame_id() {
                session
                    .add_watcher(expression.into(), stack_frame_id, cx)
                    .detach();
            }
        });
    }

    pub fn evaluate(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let expression = self.query_bar.update(cx, |editor, cx| {
            let expression = editor.text(cx);
            cx.defer_in(window, |editor, window, cx| {
                editor.clear(window, cx);
            });

            expression
        });

        self.session.update(cx, |session, cx| {
            session
                .evaluate(
                    expression,
                    Some(dap::EvaluateArgumentsContext::Repl),
                    self.stack_frame_list.read(cx).opened_stack_frame_id(),
                    None,
                    cx,
                )
                .detach();
        });
    }

    fn render_submit_menu(
        &self,
        id: impl Into<ElementId>,
        keybinding_target: Option<FocusHandle>,
        cx: &App,
    ) -> impl IntoElement {
        PopoverMenu::new(id.into())
            .trigger(
                ui::ButtonLike::new_rounded_right("console-confirm-split-button-right")
                    .layer(ui::ElevationIndex::ModalSurface)
                    .size(ui::ButtonSize::None)
                    .child(
                        div()
                            .px_1()
                            .child(Icon::new(IconName::ChevronDownSmall).size(IconSize::XSmall)),
                    ),
            )
            .when(
                self.stack_frame_list
                    .read(cx)
                    .opened_stack_frame_id()
                    .is_some(),
                |this| {
                    this.menu(move |window, cx| {
                        Some(ContextMenu::build(window, cx, |context_menu, _, _| {
                            context_menu
                                .when_some(keybinding_target.clone(), |el, keybinding_target| {
                                    el.context(keybinding_target.clone())
                                })
                                .action("Watch expression", WatchExpression.boxed_clone())
                        }))
                    })
                },
            )
            .anchor(Corner::TopRight)
    }

    fn render_console(&self, cx: &Context<Self>) -> impl IntoElement {
        EditorElement::new(&self.console, Self::editor_style(&self.console, cx))
    }

    fn editor_style(editor: &Entity<Editor>, cx: &Context<Self>) -> EditorStyle {
        let is_read_only = editor.read(cx).read_only(cx);
        let settings = ThemeSettings::get_global(cx);
        let theme = cx.theme();
        let text_style = TextStyle {
            color: if is_read_only {
                theme.colors().text_muted
            } else {
                theme.colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(settings.buffer_line_height.value()),
            ..Default::default()
        };
        EditorStyle {
            background: theme.colors().editor_background,
            local_player: theme.players().local(),
            text: text_style,
            ..Default::default()
        }
    }

    fn render_query_bar(&self, cx: &Context<Self>) -> impl IntoElement {
        EditorElement::new(&self.query_bar, Self::editor_style(&self.query_bar, cx))
    }

    fn update_output(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let session = self.session.clone();
        let token = self.last_token;

        self.update_output_task = cx.spawn_in(window, async move |this, cx| {
            _ = session.update_in(cx, move |session, window, cx| {
                let (output, last_processed_token) = session.output(token);

                _ = this.update(cx, |this, cx| {
                    if last_processed_token == this.last_token {
                        return;
                    }
                    this.add_messages(output, window, cx);

                    this.last_token = last_processed_token;
                });
            });
        });
    }
}

impl Render for Console {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let query_focus_handle = self.query_bar.focus_handle(cx);

        v_flex()
            .track_focus(&self.focus_handle)
            .key_context("DebugConsole")
            .on_action(cx.listener(Self::evaluate))
            .on_action(cx.listener(Self::watch_expression))
            .size_full()
            .child(self.render_console(cx))
            .when(self.is_running(cx), |this| {
                this.child(Divider::horizontal()).child(
                    h_flex()
                        .gap_1()
                        .bg(cx.theme().colors().editor_background)
                        .child(self.render_query_bar(cx))
                        .child(SplitButton::new(
                            ui::ButtonLike::new_rounded_all(ElementId::Name(
                                "split-button-left-confirm-button".into(),
                            ))
                            .on_click(move |_, window, cx| {
                                window.dispatch_action(Box::new(Confirm), cx)
                            })
                            .tooltip({
                                let query_focus_handle = query_focus_handle.clone();

                                move |window, cx| {
                                    Tooltip::for_action_in(
                                        "Evaluate",
                                        &Confirm,
                                        &query_focus_handle,
                                        window,
                                        cx,
                                    )
                                }
                            })
                            .layer(ui::ElevationIndex::ModalSurface)
                            .size(ui::ButtonSize::Compact)
                            .child(Label::new("Evaluate")),
                            self.render_submit_menu(
                                ElementId::Name("split-button-right-confirm-button".into()),
                                Some(query_focus_handle.clone()),
                                cx,
                            )
                            .into_any_element(),
                        )),
                )
            })
            .border_2()
    }
}

impl Focusable for Console {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

struct ConsoleQueryBarCompletionProvider(WeakEntity<Console>);

impl CompletionProvider for ConsoleQueryBarCompletionProvider {
    fn completions(
        &self,
        _excerpt_id: ExcerptId,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        _trigger: editor::CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let Some(console) = self.0.upgrade() else {
            return Task::ready(Ok(Vec::new()));
        };

        let support_completions = console
            .read(cx)
            .session
            .read(cx)
            .capabilities()
            .supports_completions_request
            .unwrap_or_default();

        if support_completions {
            self.client_completions(&console, buffer, buffer_position, cx)
        } else {
            self.variable_list_completions(&console, buffer, buffer_position, cx)
        }
    }

    fn apply_additional_edits_for_completion(
        &self,
        _buffer: Entity<Buffer>,
        _completions: Rc<RefCell<Box<[Completion]>>>,
        _completion_index: usize,
        _push_to_history: bool,
        _cx: &mut Context<Editor>,
    ) -> gpui::Task<anyhow::Result<Option<language::Transaction>>> {
        Task::ready(Ok(None))
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        text: &str,
        _trigger_in_words: bool,
        menu_is_open: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        let snapshot = buffer.read(cx).snapshot();
        if !menu_is_open && !snapshot.settings_at(position, cx).show_completions_on_input {
            return false;
        }

        self.0
            .read_with(cx, |console, cx| {
                console
                    .session
                    .read(cx)
                    .capabilities()
                    .completion_trigger_characters
                    .as_ref()
                    .map(|triggers| triggers.contains(&text.to_string()))
            })
            .ok()
            .flatten()
            .unwrap_or(true)
    }
}

impl ConsoleQueryBarCompletionProvider {
    fn variable_list_completions(
        &self,
        console: &Entity<Console>,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let (variables, string_matches) = console.update(cx, |console, cx| {
            let mut variables = HashMap::default();
            let mut string_matches = Vec::default();

            for variable in console.variable_list.update(cx, |variable_list, cx| {
                variable_list.completion_variables(cx)
            }) {
                if let Some(evaluate_name) = &variable.evaluate_name {
                    variables.insert(evaluate_name.clone(), variable.value.clone());
                    string_matches.push(StringMatchCandidate {
                        id: 0,
                        string: evaluate_name.clone(),
                        char_bag: evaluate_name.chars().collect(),
                    });
                }

                variables.insert(variable.name.clone(), variable.value.clone());

                string_matches.push(StringMatchCandidate {
                    id: 0,
                    string: variable.name.clone(),
                    char_bag: variable.name.chars().collect(),
                });
            }

            (variables, string_matches)
        });

        let snapshot = buffer.read(cx).text_snapshot();
        let query = snapshot.text();
        let replace_range = {
            let buffer_offset = buffer_position.to_offset(&snapshot);
            let reversed_chars = snapshot.reversed_chars_for_range(0..buffer_offset);
            let mut word_len = 0;
            for ch in reversed_chars {
                if ch.is_alphanumeric() || ch == '_' {
                    word_len += 1;
                } else {
                    break;
                }
            }
            let word_start_offset = buffer_offset - word_len;
            let start_anchor = snapshot.anchor_at(word_start_offset, Bias::Left);
            start_anchor..buffer_position
        };
        cx.spawn(async move |_, cx| {
            const LIMIT: usize = 10;
            let matches = fuzzy::match_strings(
                &string_matches,
                &query,
                true,
                true,
                LIMIT,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            let completions = matches
                .iter()
                .filter_map(|string_match| {
                    let variable_value = variables.get(&string_match.string)?;

                    Some(project::Completion {
                        replace_range: replace_range.clone(),
                        new_text: string_match.string.clone(),
                        label: CodeLabel {
                            filter_range: 0..string_match.string.len(),
                            text: format!("{} {}", string_match.string, variable_value),
                            runs: Vec::new(),
                        },
                        icon_path: None,
                        documentation: None,
                        confirm: None,
                        source: project::CompletionSource::Custom,
                        insert_text_mode: None,
                    })
                })
                .collect::<Vec<_>>();

            Ok(vec![project::CompletionResponse {
                is_incomplete: completions.len() >= LIMIT,
                completions,
            }])
        })
    }

    fn client_completions(
        &self,
        console: &Entity<Console>,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let completion_task = console.update(cx, |console, cx| {
            console.session.update(cx, |state, cx| {
                let frame_id = console.stack_frame_list.read(cx).opened_stack_frame_id();

                state.completions(
                    CompletionsQuery::new(buffer.read(cx), buffer_position, frame_id),
                    cx,
                )
            })
        });
        let snapshot = buffer.read(cx).text_snapshot();
        cx.background_executor().spawn(async move {
            let completions = completion_task.await?;

            let completions = completions
                .into_iter()
                .map(|completion| {
                    let new_text = completion
                        .text
                        .as_ref()
                        .unwrap_or(&completion.label)
                        .to_owned();
                    let buffer_text = snapshot.text();
                    let buffer_bytes = buffer_text.as_bytes();
                    let new_bytes = new_text.as_bytes();

                    let mut prefix_len = 0;
                    for i in (0..new_bytes.len()).rev() {
                        if buffer_bytes.ends_with(&new_bytes[0..i]) {
                            prefix_len = i;
                            break;
                        }
                    }

                    let buffer_offset = buffer_position.to_offset(&snapshot);
                    let start = buffer_offset - prefix_len;
                    let start = snapshot.clip_offset(start, Bias::Left);
                    let start = snapshot.anchor_before(start);
                    let replace_range = start..buffer_position;

                    project::Completion {
                        replace_range,
                        new_text,
                        label: CodeLabel {
                            filter_range: 0..completion.label.len(),
                            text: completion.label,
                            runs: Vec::new(),
                        },
                        icon_path: None,
                        documentation: None,
                        confirm: None,
                        source: project::CompletionSource::BufferWord {
                            word_range: buffer_position..language::Anchor::MAX,
                            resolved: false,
                        },
                        insert_text_mode: None,
                    }
                })
                .collect();

            Ok(vec![project::CompletionResponse {
                completions,
                is_incomplete: false,
            }])
        })
    }
}

#[derive(Default)]
struct ConsoleHandler {
    output: String,
    spans: Vec<(Range<usize>, Option<ansi::Color>)>,
    background_spans: Vec<(Range<usize>, Option<ansi::Color>)>,
    current_range_start: usize,
    current_background_range_start: usize,
    current_color: Option<ansi::Color>,
    current_background_color: Option<ansi::Color>,
    pos: usize,
}

impl ConsoleHandler {
    fn break_span(&mut self, color: Option<ansi::Color>) {
        self.spans.push((
            self.current_range_start..self.output.len(),
            self.current_color,
        ));
        self.current_color = color;
        self.current_range_start = self.pos;
    }

    fn break_background_span(&mut self, color: Option<ansi::Color>) {
        self.background_spans.push((
            self.current_background_range_start..self.output.len(),
            self.current_background_color,
        ));
        self.current_background_color = color;
        self.current_background_range_start = self.pos;
    }
}

impl ansi::Handler for ConsoleHandler {
    fn input(&mut self, c: char) {
        self.output.push(c);
        self.pos += c.len_utf8();
    }

    fn linefeed(&mut self) {
        self.output.push('\n');
        self.pos += 1;
    }

    fn put_tab(&mut self, count: u16) {
        self.output
            .extend(std::iter::repeat('\t').take(count as usize));
        self.pos += count as usize;
    }

    fn terminal_attribute(&mut self, attr: ansi::Attr) {
        match attr {
            ansi::Attr::Foreground(color) => {
                self.break_span(Some(color));
            }
            ansi::Attr::Background(color) => {
                self.break_background_span(Some(color));
            }
            ansi::Attr::Reset => {
                self.break_span(None);
                self.break_background_span(None);
            }
            _ => {}
        }
    }
}
