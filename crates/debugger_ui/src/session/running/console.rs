use super::{
    stack_frame_list::{StackFrameList, StackFrameListEvent},
    variable_list::VariableList,
};
use anyhow::Result;
use collections::HashMap;
use dap::OutputEvent;
use editor::{CompletionProvider, Editor, EditorElement, EditorStyle, ExcerptId};
use fuzzy::StringMatchCandidate;
use gpui::{Context, Entity, Render, Subscription, Task, TextStyle, WeakEntity};
use language::{Buffer, CodeLabel};
use menu::Confirm;
use project::{
    debugger::session::{CompletionsQuery, OutputToken, Session},
    Completion,
};
use settings::Settings;
use std::{cell::RefCell, rc::Rc, usize};
use theme::ThemeSettings;
use ui::prelude::*;

pub struct Console {
    console: Entity<Editor>,
    query_bar: Entity<Editor>,
    session: Entity<Session>,
    _subscriptions: Vec<Subscription>,
    variable_list: Entity<VariableList>,
    stack_frame_list: Entity<StackFrameList>,
    last_token: OutputToken,
    update_output_task: Task<()>,
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
            editor.set_show_gutter(true, cx);
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
            editor
        });

        let this = cx.weak_entity();
        let query_bar = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Evaluate an expression", cx);
            editor.set_use_autoclose(false);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Some(Box::new(ConsoleQueryBarCompletionProvider(this))));

            editor
        });

        let _subscriptions =
            vec![cx.subscribe(&stack_frame_list, Self::handle_stack_frame_list_events)];

        Self {
            session,
            console,
            query_bar,
            variable_list,
            _subscriptions,
            stack_frame_list,
            update_output_task: Task::ready(()),
            last_token: OutputToken(0),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn editor(&self) -> &Entity<Editor> {
        &self.console
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn query_bar(&self) -> &Entity<Editor> {
        &self.query_bar
    }

    fn is_local(&self, cx: &Context<Self>) -> bool {
        self.session.read(cx).is_local()
    }

    fn handle_stack_frame_list_events(
        &mut self,
        _: Entity<StackFrameList>,
        event: &StackFrameListEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            StackFrameListEvent::SelectedStackFrameChanged(_) => cx.notify(),
        }
    }

    pub fn add_messages<'a>(
        &mut self,
        events: impl Iterator<Item = &'a OutputEvent>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.console.update(cx, |console, cx| {
            let mut to_insert = String::default();
            for event in events {
                use std::fmt::Write;

                _ = write!(to_insert, "{}\n", event.output.trim_end());
            }

            console.set_read_only(false);
            console.move_to_end(&editor::actions::MoveToEnd, window, cx);
            console.insert(&to_insert, window, cx);
            console.set_read_only(true);

            cx.notify();
        });
    }

    pub fn evaluate(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let expression = self.query_bar.update(cx, |editor, cx| {
            let expression = editor.text(cx);

            editor.clear(window, cx);

            expression
        });

        self.session.update(cx, |state, cx| {
            state.evaluate(
                expression,
                Some(dap::EvaluateArgumentsContext::Variables),
                self.stack_frame_list.read(cx).current_stack_frame_id(),
                None,
                cx,
            );
        });
    }

    fn render_console(&self, cx: &Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.console.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(settings.buffer_line_height.value()),
            ..Default::default()
        };

        EditorElement::new(
            &self.console,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_query_bar(&self, cx: &Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.console.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: TextSize::Editor.rems(cx).into(),
            font_weight: settings.ui_font.weight,
            line_height: relative(1.3),
            ..Default::default()
        };

        EditorElement::new(
            &self.query_bar,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

impl Render for Console {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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

        v_flex()
            .key_context("DebugConsole")
            .on_action(cx.listener(Self::evaluate))
            .size_full()
            .child(self.render_console(cx))
            .when(self.is_local(cx), |this| {
                this.child(self.render_query_bar(cx))
                    .pt(DynamicSpacing::Base04.rems(cx))
            })
            .border_2()
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
    ) -> Task<Result<Option<Vec<Completion>>>> {
        let Some(console) = self.0.upgrade() else {
            return Task::ready(Ok(None));
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

    fn resolve_completions(
        &self,
        _buffer: Entity<Buffer>,
        _completion_indices: Vec<usize>,
        _completions: Rc<RefCell<Box<[Completion]>>>,
        _cx: &mut Context<Editor>,
    ) -> gpui::Task<gpui::Result<bool>> {
        Task::ready(Ok(false))
    }

    fn apply_additional_edits_for_completion(
        &self,
        _buffer: Entity<Buffer>,
        _completions: Rc<RefCell<Box<[Completion]>>>,
        _completion_index: usize,
        _push_to_history: bool,
        _cx: &mut Context<Editor>,
    ) -> gpui::Task<gpui::Result<Option<language::Transaction>>> {
        Task::ready(Ok(None))
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Entity<Buffer>,
        _position: language::Anchor,
        _text: &str,
        _trigger_in_words: bool,
        _cx: &mut Context<Editor>,
    ) -> bool {
        true
    }
}

impl ConsoleQueryBarCompletionProvider {
    fn variable_list_completions(
        &self,
        console: &Entity<Console>,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Option<Vec<Completion>>>> {
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

        let query = buffer.read(cx).text();

        cx.spawn(async move |_, cx| {
            let matches = fuzzy::match_strings(
                &string_matches,
                &query,
                true,
                10,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            Ok(Some(
                matches
                    .iter()
                    .filter_map(|string_match| {
                        let variable_value = variables.get(&string_match.string)?;

                        Some(project::Completion {
                            old_range: buffer_position..buffer_position,
                            new_text: string_match.string.clone(),
                            label: CodeLabel {
                                filter_range: 0..string_match.string.len(),
                                text: format!("{} {}", string_match.string.clone(), variable_value),
                                runs: Vec::new(),
                            },
                            icon_path: None,
                            documentation: None,
                            confirm: None,
                            source: project::CompletionSource::Custom,
                        })
                    })
                    .collect(),
            ))
        })
    }

    fn client_completions(
        &self,
        console: &Entity<Console>,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Option<Vec<Completion>>>> {
        let completion_task = console.update(cx, |console, cx| {
            console.session.update(cx, |state, cx| {
                let frame_id = console.stack_frame_list.read(cx).current_stack_frame_id();

                state.completions(
                    CompletionsQuery::new(buffer.read(cx), buffer_position, frame_id),
                    cx,
                )
            })
        });

        cx.background_executor().spawn(async move {
            Ok(Some(
                completion_task
                    .await?
                    .iter()
                    .map(|completion| project::Completion {
                        old_range: buffer_position..buffer_position, // TODO(debugger): change this
                        new_text: completion.text.clone().unwrap_or(completion.label.clone()),
                        label: CodeLabel {
                            filter_range: 0..completion.label.len(),
                            text: completion.label.clone(),
                            runs: Vec::new(),
                        },
                        icon_path: None,
                        documentation: None,
                        confirm: None,
                        source: project::CompletionSource::Custom,
                    })
                    .collect(),
            ))
        })
    }
}
