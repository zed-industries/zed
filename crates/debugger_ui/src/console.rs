use crate::{
    stack_frame_list::{StackFrameList, StackFrameListEvent},
    variable_list::VariableList,
};
use dap::client::DebugAdapterClientId;
use editor::{CompletionProvider, Editor, EditorElement, EditorStyle};
use fuzzy::StringMatchCandidate;
use gpui::{Model, Render, Subscription, Task, TextStyle, View, ViewContext, WeakView};
use language::{Buffer, CodeLabel, LanguageServerId, ToOffsetUtf16};
use menu::Confirm;
use parking_lot::RwLock;
use project::{dap_store::DapStore, Completion};
use settings::Settings;
use std::{collections::HashMap, sync::Arc};
use theme::ThemeSettings;
use ui::prelude::*;

pub struct Console {
    console: View<Editor>,
    query_bar: View<Editor>,
    dap_store: Model<DapStore>,
    client_id: DebugAdapterClientId,
    _subscriptions: Vec<Subscription>,
    variable_list: View<VariableList>,
    stack_frame_list: View<StackFrameList>,
}

impl Console {
    pub fn new(
        stack_frame_list: &View<StackFrameList>,
        client_id: &DebugAdapterClientId,
        variable_list: View<VariableList>,
        dap_store: Model<DapStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let console = cx.new_view(|cx| {
            let mut editor = Editor::multi_line(cx);
            editor.move_to_end(&editor::actions::MoveToEnd, cx);
            editor.set_read_only(true);
            editor.set_show_gutter(false, cx);
            editor.set_use_autoclose(false);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_inline_completions(Some(false), cx);
            editor
        });

        let this = cx.view().downgrade();
        let query_bar = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Evaluate an expression", cx);
            editor.set_use_autoclose(false);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Box::new(ConsoleQueryBarCompletionProvider(this)));

            editor
        });

        let _subscriptions =
            vec![cx.subscribe(stack_frame_list, Self::handle_stack_frame_list_events)];

        Self {
            console,
            dap_store,
            query_bar,
            variable_list,
            _subscriptions,
            client_id: *client_id,
            stack_frame_list: stack_frame_list.clone(),
        }
    }

    fn handle_stack_frame_list_events(
        &mut self,
        _: View<StackFrameList>,
        event: &StackFrameListEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            StackFrameListEvent::ChangedStackFrame => cx.notify(),
            StackFrameListEvent::StackFramesUpdated => {
                // TODO debugger: check if we need to do something here
            }
        }
    }

    pub fn add_message(&mut self, message: &str, cx: &mut ViewContext<Self>) {
        self.console.update(cx, |console, cx| {
            console.set_read_only(false);
            console.move_to_end(&editor::actions::MoveToEnd, cx);
            console.insert(format!("{}\n", message.trim_end()).as_str(), cx);
            console.set_read_only(true);
        });
    }

    fn evaluate(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        let expression = self.query_bar.update(cx, |editor, cx| {
            let expression = editor.text(cx);

            editor.clear(cx);

            expression
        });

        let evaluate_task = self.dap_store.update(cx, |store, cx| {
            store.evaluate(
                &self.client_id,
                self.stack_frame_list.read(cx).current_stack_frame_id(),
                expression,
                dap::EvaluateArgumentsContext::Variables,
                cx,
            )
        });

        cx.spawn(|this, mut cx| async move {
            let response = evaluate_task.await?;

            this.update(&mut cx, |console, cx| {
                console.add_message(&response.result, cx);

                console.variable_list.update(cx, |variable_list, cx| {
                    variable_list
                        .refetch_existing_variables(cx)
                        .detach_and_log_err(cx);
                })
            })
        })
        .detach_and_log_err(cx);
    }

    fn render_console(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.console.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: settings.buffer_font_size.into(),
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

    fn render_query_bar(&self, cx: &ViewContext<Self>) -> impl IntoElement {
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DebugConsole")
            .on_action(cx.listener(Self::evaluate))
            .size_full()
            .child(self.render_console(cx))
            .child(
                div()
                    .child(self.render_query_bar(cx))
                    .pt(Spacing::XSmall.rems(cx)),
            )
            .border_2()
    }
}

struct ConsoleQueryBarCompletionProvider(WeakView<Console>);

impl CompletionProvider for ConsoleQueryBarCompletionProvider {
    fn completions(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: language::Anchor,
        _trigger: editor::CompletionContext,
        cx: &mut ViewContext<Editor>,
    ) -> gpui::Task<gpui::Result<Vec<project::Completion>>> {
        let Some(console) = self.0.upgrade() else {
            return Task::ready(Ok(Vec::new()));
        };

        let support_completions = console.update(cx, |this, cx| {
            this.dap_store
                .read(cx)
                .capabilities_by_id(&this.client_id)
                .supports_completions_request
                .unwrap_or_default()
        });

        if support_completions {
            self.client_completions(&console, buffer, buffer_position, cx)
        } else {
            self.variable_list_completions(&console, buffer, buffer_position, cx)
        }
    }

    fn resolve_completions(
        &self,
        _buffer: Model<Buffer>,
        _completion_indices: Vec<usize>,
        _completions: Arc<RwLock<Box<[Completion]>>>,
        _cx: &mut ViewContext<Editor>,
    ) -> gpui::Task<gpui::Result<bool>> {
        Task::ready(Ok(false))
    }

    fn apply_additional_edits_for_completion(
        &self,
        _buffer: Model<Buffer>,
        _completion: project::Completion,
        _push_to_history: bool,
        _cx: &mut ViewContext<Editor>,
    ) -> gpui::Task<gpui::Result<Option<language::Transaction>>> {
        Task::ready(Ok(None))
    }

    fn is_completion_trigger(
        &self,
        _buffer: &Model<Buffer>,
        _position: language::Anchor,
        _text: &str,
        _trigger_in_words: bool,
        _cx: &mut ViewContext<Editor>,
    ) -> bool {
        true
    }
}

impl ConsoleQueryBarCompletionProvider {
    fn variable_list_completions(
        &self,
        console: &View<Console>,
        buffer: &Model<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut ViewContext<Editor>,
    ) -> gpui::Task<gpui::Result<Vec<project::Completion>>> {
        let (variables, string_matches) = console.update(cx, |console, cx| {
            let mut variables = HashMap::new();
            let mut string_matches = Vec::new();

            for variable in console.variable_list.update(cx, |v, cx| v.variables(cx)) {
                if let Some(evaluate_name) = &variable.variable.evaluate_name {
                    variables.insert(evaluate_name.clone(), variable.variable.value.clone());
                    string_matches.push(StringMatchCandidate {
                        id: 0,
                        string: evaluate_name.clone(),
                        char_bag: evaluate_name.chars().collect(),
                    });
                }

                variables.insert(
                    variable.variable.name.clone(),
                    variable.variable.value.clone(),
                );

                string_matches.push(StringMatchCandidate {
                    id: 0,
                    string: variable.variable.name.clone(),
                    char_bag: variable.variable.name.chars().collect(),
                });
            }

            (variables, string_matches)
        });

        let query = buffer.read(cx).text();
        let start_position = buffer.read(cx).anchor_before(0);

        cx.spawn(|_, cx| async move {
            let matches = fuzzy::match_strings(
                &string_matches,
                &query,
                true,
                10,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            Ok(matches
                .iter()
                .filter_map(|string_match| {
                    let variable_value = variables.get(&string_match.string)?;

                    Some(project::Completion {
                        old_range: start_position..buffer_position,
                        new_text: string_match.string.clone(),
                        label: CodeLabel {
                            filter_range: 0..string_match.string.len(),
                            text: format!("{} {}", string_match.string.clone(), variable_value),
                            runs: Vec::new(),
                        },
                        server_id: LanguageServerId(0), // TODO debugger: read from client
                        documentation: None,
                        lsp_completion: Default::default(),
                        confirm: None,
                    })
                })
                .collect())
        })
    }

    fn client_completions(
        &self,
        console: &View<Console>,
        buffer: &Model<Buffer>,
        buffer_position: language::Anchor,
        cx: &mut ViewContext<Editor>,
    ) -> gpui::Task<gpui::Result<Vec<project::Completion>>> {
        let text = buffer.read(cx).text();
        let start_position = buffer.read(cx).anchor_before(0);
        let snapshot = buffer.read(cx).snapshot();

        let completion_task = console.update(cx, |console, cx| {
            console.dap_store.update(cx, |store, cx| {
                store.completions(
                    &console.client_id,
                    console.stack_frame_list.read(cx).current_stack_frame_id(),
                    text,
                    buffer_position.to_offset_utf16(&snapshot).0 as u64,
                    cx,
                )
            })
        });

        cx.background_executor().spawn(async move {
            Ok(completion_task
                .await?
                .iter()
                .map(|completion| project::Completion {
                    old_range: start_position..buffer_position,
                    new_text: completion.text.clone().unwrap_or(completion.label.clone()),
                    label: CodeLabel {
                        filter_range: 0..completion.label.len(),
                        text: completion.label.clone(),
                        runs: Vec::new(),
                    },
                    server_id: LanguageServerId(0), // TODO debugger: read from client
                    documentation: None,
                    lsp_completion: Default::default(),
                    confirm: None,
                })
                .collect())
        })
    }
}
