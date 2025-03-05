use editor::Editor;
use futures::channel::oneshot;
use gpui::{AppContext, DismissEvent, Entity, EventEmitter, Focusable, Styled};
use ui::{
    div, rems, v_flex, ActiveTheme, App, Context, InteractiveElement, IntoElement, ParentElement,
    Render, SharedString, StyledExt, Window,
};
use workspace::ModalView;

pub(crate) struct AskPassModal {
    operation: SharedString,
    prompt: SharedString,
    editor: Entity<Editor>,
    tx: Option<oneshot::Sender<String>>,
}

impl EventEmitter<DismissEvent> for AskPassModal {}
impl ModalView for AskPassModal {}
impl Focusable for AskPassModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl AskPassModal {
    pub fn new(
        operation: SharedString,
        prompt: SharedString,
        tx: oneshot::Sender<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if prompt.contains("yes/no") {
                editor.set_masked(false, cx);
            } else {
                editor.set_masked(true, cx);
            }
            editor
        });
        Self {
            operation,
            prompt,
            editor,
            tx: Some(tx),
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(tx) = self.tx.take() {
            tx.send(self.editor.read(cx).text(cx)).ok();
        }
        cx.emit(DismissEvent);
    }
}

impl Render for AskPassModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
