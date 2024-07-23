use anyhow::Result;
use editor::Editor;
use futures::channel::oneshot;
use gpui::{
    px, DismissEvent, EventEmitter, FocusableView, ParentElement as _, Render, SharedString, View,
};
use ui::{
    v_flex, FluentBuilder as _, InteractiveElement, Label, LabelCommon, Styled, StyledExt as _,
    ViewContext, VisualContext,
};
use workspace::ModalView;

pub struct SshConnectionModal {
    host: SharedString,
    status_message: Option<SharedString>,
    prompt: Option<(SharedString, oneshot::Sender<Result<String>>)>,
    editor: View<Editor>,
}

impl SshConnectionModal {
    pub fn new(host: String, cx: &mut ViewContext<Self>) -> Self {
        Self {
            host: host.into(),
            prompt: None,
            status_message: None,
            editor: cx.new_view(|cx| Editor::single_line(cx)),
        }
    }

    pub fn set_prompt(
        &mut self,
        prompt: String,
        tx: oneshot::Sender<Result<String>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            if prompt.contains("yes/no") {
                editor.set_redact_all(false, cx);
            } else {
                editor.set_redact_all(true, cx);
            }
        });
        self.prompt = Some((prompt.into(), tx));
        self.status_message.take();
        cx.focus_view(&self.editor);
        cx.notify();
    }

    pub fn set_status(&mut self, status: Option<String>, cx: &mut ViewContext<Self>) {
        self.status_message = status.map(|s| s.into());
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if let Some((_, tx)) = self.prompt.take() {
            self.editor.update(cx, |editor, cx| {
                tx.send(Ok(editor.text(cx))).ok();
                editor.clear(cx);
            });
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.remove_window();
    }
}

impl Render for SshConnectionModal {
    fn render(&mut self, cx: &mut ui::ViewContext<Self>) -> impl ui::IntoElement {
        v_flex()
            .key_context("PasswordPrompt")
            .elevation_3(cx)
            .p_4()
            .gap_2()
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::confirm))
            .w(px(400.))
            .child(Label::new(format!("SSH: {}", self.host)).size(ui::LabelSize::Large))
            .when_some(self.status_message.as_ref(), |el, status| {
                el.child(Label::new(status.clone()))
            })
            .when_some(self.prompt.as_ref(), |el, prompt| {
                el.child(Label::new(prompt.0.clone()))
                    .child(self.editor.clone())
            })
    }
}

impl FocusableView for SshConnectionModal {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for SshConnectionModal {}

impl ModalView for SshConnectionModal {}
