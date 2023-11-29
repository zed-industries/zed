use gpui::{
    div, prelude::*, px, AnyView, Div, FocusHandle, ManagedView, Render, Subscription, View,
    ViewContext,
};
use ui::{h_stack, v_stack};

pub struct ActiveModal {
    modal: AnyView,
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
        V: ManagedView,
        B: FnOnce(&mut ViewContext<V>) -> V,
    {
        if let Some(active_modal) = &self.active_modal {
            let is_close = active_modal.modal.clone().downcast::<V>().is_ok();
            self.hide_modal(cx);
            if is_close {
                return;
            }
        }
        let new_modal = cx.build_view(build_view);
        self.show_modal(new_modal, cx);
    }

    pub fn show_modal<V>(&mut self, new_modal: View<V>, cx: &mut ViewContext<Self>)
    where
        V: ManagedView,
    {
        self.active_modal = Some(ActiveModal {
            modal: new_modal.clone().into(),
            subscription: cx.subscribe(&new_modal, |this, modal, e, cx| this.hide_modal(cx)),
            previous_focus_handle: cx.focused(),
            focus_handle: cx.focus_handle(),
        });
        cx.focus_view(&new_modal);
        cx.notify();
    }

    pub fn hide_modal(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_modal) = self.active_modal.take() {
            if let Some(previous_focus) = active_modal.previous_focus_handle {
                if active_modal.focus_handle.contains_focused(cx) {
                    previous_focus.focus(cx);
                }
            }
        }

        cx.notify();
    }

    pub fn active_modal<V>(&self) -> Option<View<V>>
    where
        V: 'static,
    {
        let active_modal = self.active_modal.as_ref()?;
        active_modal.modal.clone().downcast::<V>().ok()
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
                            .child(active_modal.modal.clone()),
                    ),
            )
    }
}
