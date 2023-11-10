use crate::Workspace;
use gpui::{
    div, px, AnyView, Component, Div, EventEmitter, FocusHandle, ParentElement, Render,
    StatefulInteractivity, StatelessInteractive, Styled, Subscription, View, ViewContext,
    WindowContext,
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
    registered_modals: Vec<(
        TypeId,
        Box<
            dyn Fn(
                Div<Workspace, StatefulInteractivity<Workspace>>,
            ) -> Div<Workspace, StatefulInteractivity<Workspace>>,
        >,
    )>,
}

pub trait Modal: Render + EventEmitter<ModalEvent> {
    fn focus(&self, cx: &mut WindowContext);
}

pub enum ModalEvent {
    Dismissed,
}

impl ModalLayer {
    pub fn new() -> Self {
        Self {
            active_modal: None,
            registered_modals: Vec::new(),
        }
    }

    pub fn register_modal<A: 'static, V, B>(&mut self, action: A, build_view: B)
    where
        V: Modal,
        B: Fn(&mut WindowContext) -> Option<View<V>> + 'static,
    {
        let build_view = Arc::new(build_view);

        self.registered_modals.push((
            TypeId::of::<A>(),
            Box::new(move |mut div| {
                let build_view = build_view.clone();

                div.on_action(move |workspace, event: &A, cx| {
                    let previous_focus = cx.focused();
                    if let Some(active_modal) = &workspace.modal_layer().active_modal {
                        if active_modal.modal.clone().downcast::<V>().is_ok() {
                            workspace.modal_layer().hide_modal(cx);
                            return;
                        }
                    }
                    let Some(new_modal) = (build_view)(cx) else {
                        return;
                    };
                    workspace
                        .modal_layer()
                        .show_modal(previous_focus, new_modal, cx);
                })
            }),
        ));
    }

    pub fn show_modal<V>(
        &mut self,
        previous_focus: Option<FocusHandle>,
        new_modal: View<V>,
        cx: &mut ViewContext<Workspace>,
    ) where
        V: EventEmitter<ModalEvent> + Render,
    {
        self.active_modal = Some(ActiveModal {
            modal: new_modal.clone().into(),
            subscription: cx.subscribe(&new_modal, |this, modal, e, cx| match e {
                ModalEvent::Dismissed => this.modal_layer().hide_modal(cx),
            }),
            previous_focus_handle: previous_focus,
            focus_handle: cx.focus_handle(),
        });
        cx.notify();
    }

    pub fn hide_modal(&mut self, cx: &mut ViewContext<Workspace>) {
        dbg!("hiding...");
        if let Some(active_modal) = self.active_modal.take() {
            dbg!("something");
            if let Some(previous_focus) = active_modal.previous_focus_handle {
                dbg!("oohthing");
                if active_modal.focus_handle.contains_focused(cx) {
                    dbg!("aahthing");
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
        let mut parent = div().id("modal layer").relative().size_full();

        for (_, action) in self.registered_modals.iter() {
            parent = (action)(parent);
        }

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
                .track_focus(&open_modal.focus_handle);

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
