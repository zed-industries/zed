use anyhow::Result;
use editor::Editor;
use futures::channel::oneshot;
use gpui::{
    px, DismissEvent, EventEmitter, FocusableView, ParentElement as _, Render, SharedString, View,
};
use ui::{v_flex, InteractiveElement, Label, Styled, StyledExt as _, ViewContext, VisualContext};
use workspace::ModalView;

pub struct PasswordPrompt {
    prompt: SharedString,
    tx: Option<oneshot::Sender<Result<String>>>,
    editor: View<Editor>,
}

impl PasswordPrompt {
    pub fn new(
        prompt: String,
        tx: oneshot::Sender<Result<String>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            prompt: SharedString::from(prompt),
            tx: Some(tx),
            editor: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_redact_all(true, cx);
                editor
            }),
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let text = self.editor.read(cx).text(cx);
        if let Some(tx) = self.tx.take() {
            tx.send(Ok(text)).ok();
        };
        cx.emit(DismissEvent)
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent)
    }
}

impl Render for PasswordPrompt {
    fn render(&mut self, cx: &mut ui::ViewContext<Self>) -> impl ui::IntoElement {
        v_flex()
            .key_context("PasswordPrompt")
            .elevation_3(cx)
            .p_4()
            .gap_2()
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::confirm))
            .w(px(400.))
            .child(Label::new(self.prompt.clone()))
            .child(self.editor.clone())
    }
}

impl FocusableView for PasswordPrompt {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for PasswordPrompt {}

impl ModalView for PasswordPrompt {}
