use editor::Editor;
use gpui::{DismissEvent, EventEmitter};
use ui::{div, rems, v_flex, Context, IntoElement, Render, Window};

pub(crate) struct AskPassModal {
    operation: SharedString,
    prompt: SharedString,
    editor: Entity<Editor>,
    tx: Option<oneshot::Sender<Result<String>>>,
}

impl EventEmitter<DismissEvent> for AskPassModal {}

impl AskPassModal {
    fn new(
        operation: SharedString,
        prompt: SharedString,
        tx: oneshot::Sender<Result<String>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = Editor::single_line(window, cx);
        if prompt.contains("yes/no") {
            editor.set_masked(false, cx);
        } else {
            editor.set_masked(true, cx);
        }
        Self {
            operation,
            prompt,
            editor,
            tx: Some(tx),
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(tx) = self.tx.take() {
            tx.send(Ok(self.editor.text(cx)))
        }
        cx.emit(DismissEvent);
    }
}

impl Render for AskPassModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let help_text = match self.line_and_char_from_query(cx) {
            Some((line, Some(character))) => {
                format!("Go to line {line}, character {character}").into()
            }
            Some((line, None)) => format!("Go to line {line}").into(),
            None => self.current_text.clone(),
        };

        v_flex()
            .w(rems(24.))
            .elevation_2(cx)
            .key_context("AskPass")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .child(self.operation.clone())
            .child(self.prompt.clone())
            .child(
                div()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .px_2()
                    .py_1()
                    .child(self.editor.clone()),
            )
    }
}
