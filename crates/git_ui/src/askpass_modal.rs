use askpass::EncryptedPassword;
use editor::Editor;
use futures::channel::oneshot;
use gpui::{AppContext, DismissEvent, Entity, EventEmitter, Focusable, Styled};
use ui::{
    ActiveTheme, AnyElement, App, Button, Clickable, Color, Context, DynamicSpacing, Headline,
    HeadlineSize, Icon, IconName, IconSize, InteractiveElement, IntoElement, Label, LabelCommon,
    LabelSize, ParentElement, Render, SharedString, StyledExt, StyledTypography, Window, div,
    h_flex, v_flex,
};
use util::maybe;
use workspace::ModalView;
use zeroize::Zeroize;

pub(crate) struct AskPassModal {
    operation: SharedString,
    prompt: SharedString,
    editor: Entity<Editor>,
    tx: Option<oneshot::Sender<EncryptedPassword>>,
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
        tx: oneshot::Sender<EncryptedPassword>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if prompt.contains("yes/no") || prompt.contains("Username") {
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

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        maybe!({
            let tx = self.tx.take()?;
            let mut text = self.editor.update(cx, |this, cx| {
                let text = this.text(cx);
                this.clear(window, cx);
                text
            });
            let pw = askpass::EncryptedPassword::try_from(text.as_ref()).ok()?;
            text.zeroize();
            tx.send(pw).ok();
            Some(())
        });

        cx.emit(DismissEvent);
    }

    fn render_hint(&mut self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let color = cx.theme().status().info_background;
        if (self.prompt.contains("Password") || self.prompt.contains("Username"))
            && self.prompt.contains("github.com")
        {
            return Some(
            div()
                .p_2()
                .bg(color)
                .border_t_1()
                .border_color(cx.theme().status().info_border)
                .child(
                    h_flex().gap_2()
                        .child(
                            Icon::new(IconName::Github).size(IconSize::Small)
                        )
                        .child(
                            Label::new("You may need to configure git for Github.")
                                .size(LabelSize::Small),
                        )
                        .child(Button::new("learn-more", "Learn more").color(Color::Accent).label_size(LabelSize::Small).on_click(|_, _, cx| {
                            cx.open_url("https://docs.github.com/en/get-started/git-basics/set-up-git#authenticating-with-github-from-git")
                        })),
                )
                .into_any_element(),
        );
        }
        None
    }
}

impl Render for AskPassModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("PasswordPrompt")
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
                            Headline::new(self.operation.clone()).size(HeadlineSize::XSmall),
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
                    .child(self.prompt.clone())
                    .child(self.editor.clone()),
            )
            .children(self.render_hint(cx))
    }
}
