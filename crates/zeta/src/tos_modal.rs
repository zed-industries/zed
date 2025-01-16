use client::UserStore;
use gpui::{
    AppContext, ClickEvent, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    MouseDownEvent, Render,
};
use ui::prelude::*;
use workspace::{ModalView, Workspace};

pub struct ZetaTosModal {
    focus_handle: FocusHandle,
    user_store: Model<UserStore>,
    viewed: bool,
}

impl ZetaTosModal {
    fn new(user_store: Model<UserStore>, cx: &mut ViewContext<Self>) -> Self {
        ZetaTosModal {
            viewed: false,
            focus_handle: cx.focus_handle(),
            user_store,
        }
    }

    pub fn toggle(
        workspace: &mut Workspace,
        user_store: Model<UserStore>,
        cx: &mut ViewContext<Workspace>,
    ) {
        workspace.toggle_modal(cx, |cx| ZetaTosModal::new(user_store, cx));
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

        cx.spawn(|this, mut cx| async move {
            task.await?;

            this.update(&mut cx, |_, cx| {
                cx.emit(DismissEvent);
            })
        })
        .detach_and_log_err(cx);
    }
}

impl EventEmitter<DismissEvent> for ZetaTosModal {}

impl FocusableView for ZetaTosModal {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ZetaTosModal {}

impl Render for ZetaTosModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("zeta accept tos")
            .track_focus(&self.focus_handle(cx))
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
                    .child(Headline::new("Zed AI").size(HeadlineSize::Large))
                    .child(Icon::new(IconName::ZedPredict).size(IconSize::Humongous)),
            )
            .child("Please read and accept our Terms of Service to use Edit Predictions.")
            .child(
                v_flex()
                    .mt_2()
                    .gap_2()
                    .w_full()
                    .child(if self.viewed {
                        Button::new("accept-tos", "Accept Terms of Service")
                            .style(ButtonStyle::Filled)
                            .full_width()
                            .on_click(cx.listener(Self::accept_terms))
                    } else {
                        Button::new("view-tos", "View Terms of Service")
                            .style(ButtonStyle::Filled)
                            .full_width()
                            .on_click(cx.listener(Self::view_terms))
                    })
                    .child(Button::new("cancel", "Cancel").full_width()),
            )
    }
}
