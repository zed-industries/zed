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
    status: Option<SharedString>,
    prompt: Option<(SharedString, oneshot::Sender<Result<String>>)>,
    editor: View<Editor>,
}

impl SshConnectionModal {
    pub fn new(host: String, cx: &mut ViewContext<Self>) -> Self {
        Self {
            host: host.into(),
            prompt: None,
            status: None,
            editor: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_redact_all(true, cx);
                editor
            }),
        }
    }

    pub fn set_prompt(
        &mut self,
        prompt: String,
        tx: oneshot::Sender<Result<String>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.prompt = Some((prompt.into(), tx));
        self.status.take();
        cx.focus_view(&self.editor);
        cx.notify();
    }

    pub fn set_status(&mut self, status: Option<String>, cx: &mut ViewContext<Self>) {
        self.status = status.map(|s| s.into());
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let text = self.editor.read(cx).text(cx);
        if let Some((_, tx)) = self.prompt.take() {
            tx.send(Ok(text)).ok();
        };
        // cx.emit(DismissEvent)
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        if self.prompt.is_some() {
            cx.emit(DismissEvent)
        }
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
            .when_some(self.status.as_ref(), |el, status| {
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
