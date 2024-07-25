use gpui::{AnyView, DismissEvent, FocusHandle, ManagedView, Subscription, View};
use ui::prelude::*;

pub enum DismissDecision {
    Dismiss(bool),
    Pending,
}

pub trait ModalView: ManagedView {
    fn on_before_dismiss(&mut self, _: &mut ViewContext<Self>) -> DismissDecision {
        DismissDecision::Dismiss(true)
    }

    fn fade_out_background(&self) -> bool {
        false
    }
}

trait ModalViewHandle {
    fn on_before_dismiss(&mut self, cx: &mut WindowContext) -> DismissDecision;
    fn view(&self) -> AnyView;
    fn fade_out_background(&self, cx: &WindowContext) -> bool;
}

impl<V: ModalView> ModalViewHandle for View<V> {
    fn on_before_dismiss(&mut self, cx: &mut WindowContext) -> DismissDecision {
        self.update(cx, |this, cx| this.on_before_dismiss(cx))
    }

    fn view(&self) -> AnyView {
        self.clone().into()
    }

    fn fade_out_background(&self, cx: &WindowContext) -> bool {
        self.read(cx).fade_out_background()
    }
}

pub struct ActiveModal {
    modal: Box<dyn ModalViewHandle>,
    _subscriptions: [Subscription; 2],
    previous_focus_handle: Option<FocusHandle>,
    focus_handle: FocusHandle,
}

pub struct ModalLayer {
    active_modal: Option<ActiveModal>,
    dismiss_on_focus_lost: bool,
}

impl ModalLayer {
    pub fn new() -> Self {
        Self {
            active_modal: None,
            dismiss_on_focus_lost: false,
        }
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
        let new_modal = cx.new_view(build_view);
        self.show_modal(new_modal, cx);
    }

    fn show_modal<V>(&mut self, new_modal: View<V>, cx: &mut ViewContext<Self>)
    where
        V: ModalView,
    {
        let focus_handle = cx.focus_handle();
        self.active_modal = Some(ActiveModal {
            modal: Box::new(new_modal.clone()),
            _subscriptions: [
                cx.subscribe(&new_modal, |this, _, _: &DismissEvent, cx| {
                    this.hide_modal(cx);
                }),
                cx.on_focus_out(&focus_handle, |this, _event, cx| {
                    if this.dismiss_on_focus_lost {
                        this.hide_modal(cx);
                    }
                }),
            ],
            previous_focus_handle: cx.focused(),
            focus_handle,
        });
        cx.defer(move |_, cx| {
            cx.focus_view(&new_modal);
        });
        cx.notify();
    }

    fn hide_modal(&mut self, cx: &mut ViewContext<Self>) -> bool {
        let Some(active_modal) = self.active_modal.as_mut() else {
            self.dismiss_on_focus_lost = false;
            return false;
        };

        match active_modal.modal.on_before_dismiss(cx) {
            DismissDecision::Dismiss(dismiss) => {
                self.dismiss_on_focus_lost = !dismiss;
                if !dismiss {
                    return false;
                }
            }
            DismissDecision::Pending => {
                self.dismiss_on_focus_lost = false;
                return false;
            }
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

    pub fn has_active_modal(&self) -> bool {
        self.active_modal.is_some()
    }
}

impl Render for ModalLayer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(active_modal) = &self.active_modal else {
            return div();
        };

        div()
            .absolute()
            .size_full()
            .top_0()
            .left_0()
            .when(active_modal.modal.fade_out_background(cx), |el| {
                let mut background = cx.theme().colors().elevated_surface_background;
                background.fade_out(0.2);
                el.bg(background)
                    .occlude()
                    .on_mouse_down_out(cx.listener(|this, _, cx| {
                        this.hide_modal(cx);
                    }))
            })
            .child(
                v_flex()
                    .h(px(0.0))
                    .top_20()
                    .flex()
                    .flex_col()
                    .items_center()
                    .track_focus(&active_modal.focus_handle)
                    .child(h_flex().occlude().child(active_modal.modal.view())),
            )
    }
}
