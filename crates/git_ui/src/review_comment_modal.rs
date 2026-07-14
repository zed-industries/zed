use editor::Editor;
use futures::channel::oneshot;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable};
use menu::{Cancel, Confirm};
use ui::{
    App, Context, Headline, HeadlineSize, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, prelude::*,
};
use workspace::ModalView;

pub(crate) struct ReviewCommentModal {
    title: SharedString,
    editor: Entity<Editor>,
    sender: Option<oneshot::Sender<String>>,
}

impl ReviewCommentModal {
    pub fn new(
        title: SharedString,
        sender: oneshot::Sender<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| Editor::auto_height(3, 8, window, cx));
        Self {
            title,
            editor,
            sender: Some(sender),
        }
    }

    fn cancel(&mut self, _: &Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &Confirm, _: &mut Window, cx: &mut Context<Self>) {
        let body = self.editor.read(cx).text(cx).trim().to_string();
        if body.is_empty() {
            return;
        }
        if let Some(sender) = self.sender.take() {
            sender.send(body).ok();
        }
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ReviewCommentModal {}
impl ModalView for ReviewCommentModal {}

impl Focusable for ReviewCommentModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for ReviewCommentModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("ReviewCommentModal")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .w(rems(34.))
            .p_4()
            .gap_3()
            .elevation_3(cx)
            .child(Headline::new(self.title.clone()).size(HeadlineSize::Small))
            .child(
                div()
                    .p_2()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .child(self.editor.clone()),
            )
            .child(
                Label::new("Enter to submit · Escape to cancel")
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
    }
}
