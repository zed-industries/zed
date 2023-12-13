use gpui::{
    div, prelude::*, px, AnyView, DismissEvent, Div, FocusHandle, ManagedView, Render,
    Subscription, View, ViewContext, WindowContext,
};
use ui::{h_stack, v_stack};

pub trait ModalView: ManagedView {
    fn on_before_dismiss(&mut self, cx: &mut ViewContext<Self>) -> bool {
        true
    }
}

trait ModalViewHandle {
    fn on_before_dismiss(&mut self, cx: &mut WindowContext) -> bool;
    fn view(&self) -> AnyView;
}

impl<V: ModalView> ModalViewHandle for View<V> {
    fn on_before_dismiss(&mut self, cx: &mut WindowContext) -> bool {
        self.update(cx, |this, cx| this.on_before_dismiss(cx))
    }

    fn view(&self) -> AnyView {
        self.clone().into()
    }
}

pub struct ActiveModal {
    modal: Box<dyn ModalViewHandle>,
    subscription: Subscription,
    previous_focus_handle: Option<FocusHandle>,
    focus_handle: FocusHandle,
}

pub struct ModalLayer {
    active_modal: Option<ActiveModal>,
}

impl ModalLayer {
    pub fn new() -> Self {
        Self { active_modal: None }
    }

    pub fn toggle_modal<V, B>(&mut self, cx: &mut ViewContext<Self>, build_view: B)
    where
        V: ModalView,
        B: FnOnce(&mut ViewContext<V>) -> V,
    {
        if let Some(active_modal) = &self.active_modal {
            let is_close = active_modal.modal.view().downcast::<V>().is_ok();
            let did_close = self.hide_modal(cx);
            if is_close || !did_close {
                return;
            }
        }
        let new_modal = cx.build_view(build_view);
        self.show_modal(new_modal, cx);
    }

    fn show_modal<V>(&mut self, new_modal: View<V>, cx: &mut ViewContext<Self>)
    where
        V: ModalView,
    {
        self.active_modal = Some(ActiveModal {
            modal: Box::new(new_modal.clone()),
            subscription: cx.subscribe(&new_modal, |this, modal, _: &DismissEvent, cx| {
                this.hide_modal(cx);
            }),
            previous_focus_handle: cx.focused(),
            focus_handle: cx.focus_handle(),
        });
        cx.focus_view(&new_modal);
        cx.notify();
    }

    fn hide_modal(&mut self, cx: &mut ViewContext<Self>) -> bool {
        let Some(active_modal) = self.active_modal.as_mut() else {
            return false;
        };

        let dismiss = active_modal.modal.on_before_dismiss(cx);
        if !dismiss {
            return false;
        }

        if let Some(active_modal) = self.active_modal.take() {
            if let Some(previous_focus) = active_modal.previous_focus_handle {
                if active_modal.focus_handle.contains_focused(cx) {
                    previous_focus.focus(cx);
                }
            }
            cx.notify();
        }
        true
    }

    pub fn active_modal<V>(&self) -> Option<View<V>>
    where
        V: 'static,
    {
        let active_modal = self.active_modal.as_ref()?;
        active_modal.modal.view().downcast::<V>().ok()
    }
}

impl Render for ModalLayer {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let Some(active_modal) = &self.active_modal else {
            return div();
        };

        div()
            .absolute()
            .size_full()
            .top_0()
            .left_0()
            .z_index(400)
            .child(
                v_stack()
                    .h(px(0.0))
                    .top_20()
                    .flex()
                    .flex_col()
                    .items_center()
                    .track_focus(&active_modal.focus_handle)
                    .child(
                        h_stack()
                            .on_mouse_down_out(cx.listener(|this, _, cx| {
                                this.hide_modal(cx);
                            }))
                            .child(active_modal.modal.view()),
                    ),
            )
    }
}
