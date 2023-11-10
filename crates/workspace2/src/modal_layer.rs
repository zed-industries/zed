use crate::Workspace;
use gpui::{
    div, px, AnyView, Component, Div, EventEmitter, FocusHandle, ParentElement, Render,
    StatefulInteractivity, StatelessInteractive, Styled, Subscription, View, ViewContext,
    VisualContext, WindowContext,
};
use std::{any::TypeId, sync::Arc};
use ui::v_stack;

pub struct ActiveModal {
    modal: AnyView,
    subscription: Subscription,
    previous_focus_handle: Option<FocusHandle>,
    focus_handle: FocusHandle,
}

pub struct ModalLayer {
    active_modal: Option<ActiveModal>,
}

pub trait Modal: Render + EventEmitter<ModalEvent> {
    fn focus(&self, cx: &mut WindowContext);
}

pub enum ModalEvent {
    Dismissed,
}

impl ModalLayer {
    pub fn new() -> Self {
        Self { active_modal: None }
    }

    pub fn toggle_modal<V, B>(&mut self, cx: &mut ViewContext<Workspace>, build_view: B)
    where
        V: Modal,
        B: FnOnce(&mut ViewContext<V>) -> V,
    {
        let previous_focus = cx.focused();

        if let Some(active_modal) = &self.active_modal {
            if active_modal.modal.clone().downcast::<V>().is_ok() {
                self.hide_modal(cx);
                return;
            }
        }
        let new_modal = cx.build_view(build_view);
        self.show_modal(new_modal, cx);
    }

    pub fn show_modal<V>(&mut self, new_modal: View<V>, cx: &mut ViewContext<Workspace>)
    where
        V: Modal,
    {
        self.active_modal = Some(ActiveModal {
            modal: new_modal.clone().into(),
            subscription: cx.subscribe(&new_modal, |workspace, modal, e, cx| match e {
                ModalEvent::Dismissed => workspace.modal_layer.hide_modal(cx),
            }),
            previous_focus_handle: cx.focused(),
            focus_handle: cx.focus_handle(),
        });
        new_modal.update(cx, |modal, cx| modal.focus(cx));
        cx.notify();
    }

    pub fn hide_modal(&mut self, cx: &mut ViewContext<Workspace>) {
        if let Some(active_modal) = self.active_modal.take() {
            if let Some(previous_focus) = active_modal.previous_focus_handle {
                if active_modal.focus_handle.contains_focused(cx) {
                    previous_focus.focus(cx);
                }
            }
        }

        cx.notify();
    }

    pub fn wrapper_element(
        &self,
        cx: &ViewContext<Workspace>,
    ) -> Div<Workspace, StatefulInteractivity<Workspace>> {
        let parent = div().id("boop");
        parent.when_some(self.active_modal.as_ref(), |parent, open_modal| {
            let container1 = div()
                .absolute()
                .flex()
                .flex_col()
                .items_center()
                .size_full()
                .top_0()
                .left_0()
                .z_index(400);

            let container2 = v_stack()
                .h(px(0.0))
                .relative()
                .top_20()
                .track_focus(&open_modal.focus_handle)
                .on_mouse_down_out(|workspace: &mut Workspace, event, cx| {
                    workspace.modal_layer.hide_modal(cx);
                });

            parent.child(container1.child(container2.child(open_modal.modal.clone())))
        })
    }
}

// impl Render for ModalLayer {
//     type Element = Div<Self>;

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
//         let mut div = div();
//         for (type_id, build_view) in cx.global::<ModalRegistry>().registered_modals {
//             div = div.useful_on_action(
//                 type_id,
//                 Box::new(|this, _: dyn Any, phase, cx: &mut ViewContext<Self>| {
//                     if phase == DispatchPhase::Capture {
//                         return;
//                     }
//                     self.workspace.update(cx, |workspace, cx| {
//                         self.open_modal = Some(build_view(workspace, cx));
//                     });
//                     cx.notify();
//                 }),
//             )
//         }

//         div
//     }
// }
