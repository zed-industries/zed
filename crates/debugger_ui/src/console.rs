use dap::client::DebugAdapterClientId;
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{Model, Render, TextStyle, View, ViewContext};
use menu::Confirm;
use project::dap_store::DapStore;
use settings::Settings;
use theme::ThemeSettings;
use ui::prelude::*;

use crate::debugger_panel::ThreadState;

pub struct Console {
    console: View<Editor>,
    query_bar: View<Editor>,
    dap_store: Model<DapStore>,
    current_stack_frame_id: u64,
    client_id: DebugAdapterClientId,
    thread_state: Model<ThreadState>,
}

impl Console {
    pub fn new(
        client_id: &DebugAdapterClientId,
        current_stack_frame_id: u64,
        thread_state: Model<ThreadState>,
        dap_store: Model<DapStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let console = cx.new_view(|cx| {
            let mut editor = Editor::multi_line(cx);
            editor.move_to_end(&editor::actions::MoveToEnd, cx);
            editor.set_read_only(true);
            editor.set_show_gutter(false, cx);
            editor.set_show_inline_completions(Some(false), cx);
            editor
        });

        let query_bar = cx.new_view(Editor::single_line);

        Self {
            console,
            dap_store,
            query_bar,
            thread_state,
            client_id: *client_id,
            current_stack_frame_id,
        }
    }

    pub fn update_current_stack_frame_id(
        &mut self,
        stack_frame_id: u64,
        cx: &mut ViewContext<Self>,
    ) {
        self.current_stack_frame_id = stack_frame_id;

        cx.notify();
    }

    pub fn add_message(&mut self, message: &str, cx: &mut ViewContext<Self>) {
        self.console.update(cx, |console, cx| {
            console.set_read_only(false);
            console.move_to_end(&editor::actions::MoveToEnd, cx);
            console.insert(format!("{}\n", message.trim_end()).as_str(), cx);
            console.set_read_only(true);

            cx.notify();
        });

        cx.notify();
    }

    fn evaluate(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        let expession = self.query_bar.update(cx, |editor, cx| {
            let expession = editor.text(cx);

            editor.clear(cx);

            expession
        });

        let evaluate_task = self.dap_store.update(cx, |store, cx| {
            store.evaluate(
                &self.client_id,
                self.current_stack_frame_id,
                expession,
                dap::EvaluateArgumentsContext::Variables,
                cx,
            )
        });

        cx.spawn(|this, mut cx| async move {
            let response = evaluate_task.await?;

            this.update(&mut cx, |console, cx| {
                console.add_message(&response.result, cx);
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
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: settings.buffer_font_size.into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(settings.buffer_line_height.value()),
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
