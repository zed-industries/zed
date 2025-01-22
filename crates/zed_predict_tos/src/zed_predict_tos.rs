//! AI service Terms of Service acceptance modal.

use client::UserStore;
use gpui::{
    svg, AppContext, ClickEvent, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    MouseDownEvent, Render, View,
};
use settings::Settings;
use ui::{prelude::*, TintColor};
use workspace::{ModalView, Workspace};

/// Terms of acceptance for AI inline prediction.
pub struct ZedPredictTos {
    focus_handle: FocusHandle,
    user_store: Model<UserStore>,
    workspace: View<Workspace>,
    viewed: bool,
}

impl ZedPredictTos {
    fn new(
        workspace: View<Workspace>,
        user_store: Model<UserStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        ZedPredictTos {
            viewed: false,
            focus_handle: cx.focus_handle(),
            user_store,
            workspace,
        }
    }
    pub fn toggle(
        workspace: View<Workspace>,
        user_store: Model<UserStore>,
        cx: &mut WindowContext,
    ) {
        workspace.update(cx, |this, cx| {
            let workspace = cx.view().clone();
            this.toggle_modal(cx, |cx| ZedPredictTos::new(workspace, user_store, cx));
        });
    }

    fn view_terms(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.viewed = true;
        cx.open_url("https://zed.dev/terms-of-service");
        cx.notify();
    }

    fn accept_terms(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        let task = self
            .user_store
            .update(cx, |this, cx| this.accept_terms_of_service(cx));

        let workspace = self.workspace.clone();

        cx.spawn(|this, mut cx| async move {
            match task.await {
                Ok(_) => this.update(&mut cx, |_, cx| {
                    cx.emit(DismissEvent);
                }),
                Err(err) => workspace.update(&mut cx, |this, cx| {
                    this.show_error(&err, cx);
                }),
            }
        })
        .detach_and_log_err(cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ZedPredictTos {}

impl FocusableView for ZedPredictTos {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ZedPredictTos {}

impl Render for ZedPredictTos {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let description = if self.viewed {
            "After accepting the ToS, Zed will be set as your inline completions provider."
        } else {
            "To start using Edit Predictions, please read and accept our Terms of Service."
        };

        v_flex()
            .w_96()
            .p_4()
            .relative()
            .items_center()
            .gap_2()
            .overflow_hidden()
            .elevation_3(cx)
            .id("zed predict tos")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::cancel))
            .key_context("ZedPredictTos")
            .on_action(cx.listener(|_, _: &menu::Cancel, cx| {
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, cx| {
                cx.focus(&this.focus_handle);
            }))
            .child(
                h_flex()
                    .max_h_32()
                    .p_1()
                    .overflow_hidden()
                    .flex_wrap()
                    .gap_1()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .children((0..254).enumerate().map(|(index, _)| {
                        let opacity = 0.24 - (index as f32 * 0.0016);
                        svg()
                            .path("icons/zed_predict.svg")
                            .text_color(cx.theme().colors().icon_disabled)
                            .w(px(14.))
                            .h(px(14.))
                            .opacity(opacity.max(0.001))
                    })),
            )
            .child({
                let tab = |_n: u8| {
                    h_flex()
                        .px_4()
                        .py_0p5()
                        .bg(cx.theme().colors().editor_background)
                        .border_1()
                        .border_color(cx.theme().colors().text_accent.opacity(0.4))
                        .rounded_md()
                        .font(theme::ThemeSettings::get_global(cx).buffer_font.clone())
                        .text_size(TextSize::XSmall.rems(cx))
                        .text_color(cx.theme().colors().text)
                        .child("tab")
                };

                v_flex()
                    .items_center()
                    .gap_2()
                    .w_full()
                    .child(tab(0).ml_neg_20())
                    .child(tab(1))
                    .child(tab(2).ml_20())
            })
            .child(
                v_flex()
                    .mt_2()
                    .gap_0p5()
                    .items_center()
                    .child(
                        h_flex().child(
                            Label::new("Zed AI")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(h_flex().child(Headline::new("Edit Prediction")))
                    .child(Label::new(description).color(Color::Muted)),
            )
            .child(
                v_flex()
                    .mt_2()
                    .gap_1()
                    .w_full()
                    .child(if self.viewed {
                        Button::new(
                            "accept-tos",
                            "Accept the Terms of Service and Start Using It",
                        )
                        .style(ButtonStyle::Tinted(TintColor::Accent))
                        .full_width()
                        .on_click(cx.listener(Self::accept_terms))
                    } else {
                        Button::new("view-tos", "Read Terms of Service")
                            .style(ButtonStyle::Tinted(TintColor::Accent))
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::XSmall)
                            .icon_position(IconPosition::End)
                            .full_width()
                            .on_click(cx.listener(Self::view_terms))
                    })
                    .child(
                        Button::new("cancel", "Cancel")
                            .full_width()
                            .on_click(cx.listener(|_, _: &ClickEvent, cx| {
                                cx.emit(DismissEvent);
                            })),
                    ),
            )
    }
}
