use gpui::{
    AnyModel, AnyView, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, Subscription,
};
use ui::prelude::*;

pub enum DismissDecision {
    Dismiss(bool),
    Pending,
}

pub trait ModalView: Render + FocusableView + EventEmitter<DismissEvent> + Sized {
    fn on_before_dismiss(
        &mut self,
        _: &Model<Self>,
        _: &mut Window,
        _: &mut AppContext,
    ) -> DismissDecision {
        DismissDecision::Dismiss(true)
    }

    fn fade_out_background(&self) -> bool {
        false
    }
}

trait ModalViewHandle {
    fn on_before_dismiss(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> DismissDecision;
    fn model(&self) -> AnyModel;
    fn view(&self) -> AnyView;
    fn fade_out_background(&self, window: &Window, cx: &AppContext) -> bool;
}

impl<V: ModalView> ModalViewHandle for Model<V> {
    fn on_before_dismiss(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> DismissDecision {
        self.update(cx, |this, model, cx| {
            this.on_before_dismiss(model, window, cx)
        })
    }

    fn model(&self) -> AnyModel {
        self.clone().into()
    }

    fn view(&self) -> AnyView {
        self.clone().into()
    }

    fn fade_out_background(&self, window: &Window, cx: &AppContext) -> bool {
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

impl Default for ModalLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl ModalLayer {
    pub fn new() -> Self {
        Self {
            active_modal: None,
            dismiss_on_focus_lost: false,
        }
    }

    pub fn toggle_modal<V, B>(
        &mut self,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
        build_view: B,
    ) where
        V: ModalView,
        B: FnOnce(&Model<V>, &mut AppContext) -> V,
    {
        if let Some(active_modal) = &self.active_modal {
            let is_close = active_modal.modal.model().downcast::<V>().is_ok();
            let did_close = self.hide_modal(model, window, cx);
            if is_close || !did_close {
                return;
            }
        }
        let new_modal = cx.new_model(build_view);
        self.show_modal(new_modal, model, window, cx);
    }

    fn show_modal<V>(
        &mut self,
        new_modal: Model<V>,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) where
        V: ModalView,
    {
        let focus_handle = window.focus_handle();
        let window_handle = window.handle();
        self.active_modal = Some(ActiveModal {
            modal: Box::new(new_modal.clone()),
            _subscriptions: [
                model.subscribe_in_window(
                    &new_modal,
                    window,
                    cx,
                    move |this, _, _: &DismissEvent, model, window, cx| {
                        window_handle
                            .update(cx, |window, cx| {
                                this.hide_modal(model, window, cx);
                            })
                            .ok();
                    },
                ),
                window.on_focus_out(
                    &focus_handle,
                    cx,
                    model.listener(|this, _event, model, window, cx| {
                        if this.dismiss_on_focus_lost {
                            this.hide_modal(model, window, cx);
                        }
                    }),
                ),
            ],
            previous_focus_handle: window.focused(),
            focus_handle,
        });
        window.defer(cx, move |window, cx| {
            window.focus_view(&new_modal, cx);
        });
        model.notify(cx);
    }

    fn hide_modal(
        &mut self,
        model: &Model<Self>,
        window: &mut Window,
        cx: &mut AppContext,
    ) -> bool {
        let Some(active_modal) = self.active_modal.as_mut() else {
            self.dismiss_on_focus_lost = false;
            return false;
        };

        match active_modal.modal.on_before_dismiss(window, cx) {
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
                if active_modal.focus_handle.contains_focused(window) {
                    previous_focus.focus(window);
                }
            }
            model.notify(cx);
        }
        true
    }

    pub fn active_modal<V>(&self) -> Option<Model<V>>
    where
        V: 'static,
    {
        let active_modal = self.active_modal.as_ref()?;
        active_modal.modal.model().downcast::<V>().ok()
    }

    pub fn has_active_modal(&self) -> bool {
        self.active_modal.is_some()
    }
}

impl Render for ModalLayer {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        let Some(active_modal) = &self.active_modal else {
            return div();
        };

        div()
            .absolute()
            .size_full()
            .top_0()
            .left_0()
            .when(active_modal.modal.fade_out_background(window, cx), |el| {
                let mut background = cx.theme().colors().elevated_surface_background;
                background.fade_out(0.2);
                el.bg(background)
                    .occlude()
                    .on_mouse_down_out(model.listener(|this, _, model, window, cx| {
                        this.hide_modal(model, window, cx);
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
