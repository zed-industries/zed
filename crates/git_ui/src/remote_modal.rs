use std::str::FromStr;

use askpass::EncryptedPassword;
use editor::Editor;
use futures::channel::oneshot;
use git::RemoteUrl;
use gpui::{AppContext, DismissEvent, Entity, EventEmitter, Focusable, Styled};
use project::git_store::Repository;
use ui::{
    ActiveTheme, AnyElement, App, Button, Clickable, Color, Context, DynamicSpacing, Headline,
    HeadlineSize, Icon, IconName, IconSize, InteractiveElement, IntoElement, Label, LabelCommon,
    LabelSize, ParentElement, Render, SharedString, StyledExt, StyledTypography, Window, div,
    h_flex, v_flex,
};
use util::maybe;
use workspace::{ModalView, notifications::DetachAndPromptErr};
use zeroize::Zeroize;

pub(crate) struct RemoteModal {
    repo: Entity<Repository>,
    remote_name: SharedString,
    editor: Entity<Editor>,
}

impl EventEmitter<DismissEvent> for RemoteModal {}
impl ModalView for RemoteModal {}
impl Focusable for RemoteModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl RemoteModal {
    pub fn new(
        repo: Entity<Repository>,
        remote_name: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let editor = Editor::single_line(window, cx);
            editor
        });
        Self {
            repo,
            remote_name,
            editor,
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let mut text = self.editor.update(cx, |this, cx| {
                let text = this.text(cx);
                this.clear(window, cx);
                text
            });
            let url = RemoteUrl::from_str(&text).ok()?;
            let repo = self.repo.clone();
            let new_remote_name = self.remote_name.clone();
            cx.spawn(async move |_, cx| {
                repo.update(cx, |repo, _| {
                    repo.create_remote(new_remote_name.to_string(), url.into())
                })?
                .await??;

                Ok(())
            })
            .detach_and_prompt_err(
                "Failed to create remote",
                window,
                cx,
                |e, _, _| Some(e.to_string()),
            );
            text.zeroize();
            Some(())
        });

        cx.emit(DismissEvent);
    }
}

impl Render for RemoteModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RemoteModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_2(cx)
            .size_full()
            .child(
                h_flex()
                    .font_buffer(cx)
                    .px(DynamicSpacing::Base12.rems(cx))
                    .pt(DynamicSpacing::Base08.rems(cx))
                    .pb(DynamicSpacing::Base04.rems(cx))
                    .rounded_t_sm()
                    .w_full()
                    .gap_1p5()
                    .child(Icon::new(IconName::GitBranch).size(IconSize::XSmall))
                    .child(h_flex().gap_1().overflow_x_hidden().child(
                        div().max_w_96().overflow_x_hidden().text_ellipsis().child(
                            Headline::new(self.remote_name.clone()).size(HeadlineSize::XSmall),
                        ),
                    )),
            )
            .child(
                div()
                    .font_buffer(cx)
                    .text_buffer(cx)
                    .py_2()
                    .px_3()
                    .bg(cx.theme().colors().editor_background)
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .size_full()
                    .overflow_hidden()
                    .child(self.editor.clone()),
            )
    }
}
