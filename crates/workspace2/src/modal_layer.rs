use crate::Workspace;
use gpui::{
    div, px, AnyView, AppContext, Component, Div, EventEmitter, ParentElement, Render,
    StatelessInteractive, Styled, Subscription, View, ViewContext, WeakView,
};
use std::{any::TypeId, sync::Arc};
use ui::v_stack;

pub struct ModalRegistry {
    registered_modals: Vec<(TypeId, Box<dyn Fn(Div<Workspace>) -> Div<Workspace>>)>,
}

pub struct ModalLayer {
    workspace: WeakView<Workspace>,
    open_modal: Option<AnyView>,
    subscription: Option<Subscription>,
}

pub fn init_modal_registry(cx: &mut AppContext) {
    cx.set_global(ModalRegistry {
        registered_modals: Vec::new(),
    });
}

pub enum ModalEvent {
    Dismissed,
}

pub trait Modal: EventEmitter + Render {
    fn to_modal_event(&self, _: &Self::Event) -> Option<ModalEvent>;
}

impl ModalRegistry {
    pub fn register_modal<A: 'static, V, B>(&mut self, action: A, build_view: B)
    where
        V: Modal,
        B: Fn(&mut Workspace, &mut ViewContext<Workspace>) -> Option<View<V>> + 'static,
    {
        let build_view = Arc::new(build_view);

        self.registered_modals.push((
            TypeId::of::<A>(),
            Box::new(move |mut div| {
                let build_view = build_view.clone();

                div.on_action(move |workspace, event: &A, cx| {
                    let Some(new_modal) =
                        (build_view)(workspace, cx) else {
                            return
                        };
                    workspace.modal_layer.update(cx, |modal_layer, cx| {
                        modal_layer.show_modal(new_modal, cx);
                    })
                })
            }),
        ));
    }
}

impl ModalLayer {
    pub fn new(workspace: WeakView<Workspace>) -> Self {
        Self {
            workspace,
            open_modal: None,
            subscription: None,
        }
    }

    pub fn show_modal<V: Modal>(&mut self, new_modal: View<V>, cx: &mut ViewContext<Self>) {
        self.subscription = Some(cx.subscribe(&new_modal, |this, modal, e, cx| {
            match modal.read(cx).to_modal_event(e) {
                Some(ModalEvent::Dismissed) => this.hide_modal(cx),
                None => {}
            }
        }));
        self.open_modal = Some(new_modal.into());
        cx.notify();
    }

    pub fn hide_modal(&mut self, cx: &mut ViewContext<Self>) {
        self.open_modal.take();
        self.subscription.take();
        cx.notify();
    }

    pub fn render(&self, cx: &ViewContext<Workspace>) -> Div<Workspace> {
        let mut parent = div().relative().size_full();

        for (_, action) in cx.global::<ModalRegistry>().registered_modals.iter() {
            parent = (action)(parent);
        }

        parent.when_some(self.open_modal.as_ref(), |parent, open_modal| {
            let container1 = div()
                .absolute()
                .flex()
                .flex_col()
                .items_center()
                .size_full()
                .top_0()
                .left_0()
                .z_index(400);

            // transparent layer
            let container2 = v_stack().h(px(0.0)).relative().top_20();

            parent.child(container1.child(container2.child(open_modal.clone())))
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
