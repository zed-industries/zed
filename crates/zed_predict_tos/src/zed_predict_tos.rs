//! AI service Terms of Service acceptance modal.

use client::UserStore;
use gpui::{
    AppContext, ClickEvent, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    MouseDownEvent, Render, View,
};
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
        v_flex()
            .id("zed predict tos")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::cancel))
            .key_context("ZedPredictTos")
            .elevation_3(cx)
            .w_96()
            .items_center()
            .p_4()
            .gap_2()
            .on_action(cx.listener(|_, _: &menu::Cancel, cx| {
                cx.emit(DismissEvent);
            }))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, cx| {
                cx.focus(&this.focus_handle);
            }))
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        v_flex()
                            .gap_0p5()
                            .child(
                                Label::new("Zed AI")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Headline::new("Edit Prediction")),
                    )
                    .child(Icon::new(IconName::ZedPredict).size(IconSize::XLarge)),
            )
            .child(
                Label::new(
                    "To use Zed AI's Edit Prediction feature, please read and accept our Terms of Service.",
                )
                .color(Color::Muted),
            )
            .child(
                v_flex()
                    .mt_2()
                    .gap_0p5()
                    .w_full()
                    .child(if self.viewed {
                        Button::new("accept-tos", "I've Read and Accept the Terms of Service")
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
