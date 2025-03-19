use gpui::{
    rems, App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, Render, Window,
};
use ui::{prelude::*, Button, ButtonStyle, IconPosition};
use workspace::{ModalView, Workspace};
use zed_actions::feedback::GiveFeedback;

use crate::{EmailZed, FileBugReport, OpenZedRepo, RequestFeature};

pub struct FeedbackModal {
    focus_handle: FocusHandle,
}

impl Focusable for FeedbackModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<DismissEvent> for FeedbackModal {}

impl ModalView for FeedbackModal {}

impl FeedbackModal {
    pub fn register(workspace: &mut Workspace, _: &mut Window, cx: &mut Context<Workspace>) {
        let _handle = cx.entity().downgrade();
        workspace.register_action(move |workspace, _: &GiveFeedback, window, cx| {
            workspace.toggle_modal(window, cx, move |_, cx| FeedbackModal::new(cx));
        });
    }

    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }
}

impl Render for FeedbackModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let open_zed_repo =
            cx.listener(|_, _, window, cx| window.dispatch_action(Box::new(OpenZedRepo), cx));

        v_flex()
            .elevation_3(cx)
            .key_context("GiveFeedback")
            .on_action(cx.listener(Self::cancel))
            .min_w(rems(40.))
            .max_w(rems(96.))
            .h(rems(32.))
            .p_4()
            .gap_2()
            .child(Headline::new("Give Feedback"))
            .child(
                Button::new("file-a-bug-report", "File a bug report")
                    .icon(IconName::Debug)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .icon_position(IconPosition::Start)
                    .on_click(cx.listener(|_, _, window, cx| {
                        window.dispatch_action(Box::new(FileBugReport), cx);
                    })),
            )
            .child(
                Button::new("request-a-feature", "Request a feature")
                    .icon(IconName::Sparkle)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .icon_position(IconPosition::Start)
                    .on_click(cx.listener(|_, _, window, cx| {
                        window.dispatch_action(Box::new(RequestFeature), cx);
                    })),
            )
            .child(
                Button::new("send-us_an-email", "Send us an email")
                    .icon(IconName::Envelope)
                    .icon_size(IconSize::XSmall)
                    .icon_color(Color::Muted)
                    .icon_position(IconPosition::Start)
                    .on_click(cx.listener(|_, _, window, cx| {
                        window.dispatch_action(Box::new(EmailZed), cx);
                    })),
            )
            .child(
                h_flex()
                    .justify_between()
                    .gap_1()
                    .child(
                        Button::new("zed_repository", "Zed Repository")
                            .style(ButtonStyle::Transparent)
                            .icon(IconName::ExternalLink)
                            .icon_position(IconPosition::End)
                            .icon_size(IconSize::Small)
                            .on_click(open_zed_repo),
                    )
                    .child(
                        h_flex().gap_1().child(
                            Button::new("cancel_feedback", "Cancel")
                                .style(ButtonStyle::Subtle)
                                .color(Color::Muted)
                                .on_click(cx.listener(move |_, _, window, cx| {
                                    cx.spawn_in(window, async move |this, cx| {
                                        this.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                                    })
                                    .detach();
                                })),
                        ),
                    ),
            )
    }
}
