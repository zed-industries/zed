use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, MouseDownEvent, Render,
};
use ui::{prelude::*, Vector, VectorName};
use workspace::{ModalView, Workspace};

pub struct ZetaTosModal {
    focus_handle: FocusHandle,
}

impl ZetaTosModal {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        ZetaTosModal {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn toggle(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        // if let Some(zeta) = Zeta::global(cx) {
        workspace.toggle_modal(cx, |cx| ZetaTosModal::new(cx));
        // }
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
            .child("Welcome! Please accept our Terms of Service to use Edit Predictions.")
            .child(
                v_flex()
                    .mt_2()
                    .gap_2()
                    .w_full()
                    .child(
                        Button::new("view-tos", "View Terms of Service")
                            .style(ButtonStyle::Filled)
                            .full_width(),
                    )
                    .child(Button::new("cancel", "Cancel").full_width()),
            )
    }
}
