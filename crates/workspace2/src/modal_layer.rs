use std::{any::TypeId, sync::Arc};

use gpui::{
    div, AnyView, AppContext, Component, DispatchPhase, Div, ParentElement, Render,
    StatelessInteractive, Styled, View, ViewContext,
};

use crate::Workspace;

pub struct ModalRegistry {
    registered_modals: Vec<(TypeId, Box<dyn Fn(Div<Workspace>) -> Div<Workspace>>)>,
}

pub trait Modal {}

#[derive(Clone)]
pub struct ModalLayer {
    open_modal: Option<AnyView>,
}

pub fn init_modal_registry(cx: &mut AppContext) {
    cx.set_global(ModalRegistry {
        registered_modals: Vec::new(),
    });
}

struct ToggleModal {
    name: String,
}

impl ModalRegistry {
    pub fn register_modal<A: 'static, V, B>(&mut self, action: A, build_view: B)
    where
        V: Render,
        B: Fn(&Workspace, &mut ViewContext<Workspace>) -> View<V> + 'static,
    {
        let build_view = Arc::new(build_view);

        self.registered_modals.push((
            TypeId::of::<A>(),
            Box::new(move |mut div| {
                let build_view = build_view.clone();

                div.on_action(
                    move |workspace: &mut Workspace,
                          event: &A,
                          phase: DispatchPhase,
                          cx: &mut ViewContext<Workspace>| {
                        dbg!("GOT HERE");
                        if phase == DispatchPhase::Capture {
                            return;
                        }

                        let new_modal = (build_view)(workspace, cx);
                        workspace.modal_layer.update(cx, |modal_layer, _| {
                            modal_layer.open_modal = Some(new_modal.into());
                        });

                        cx.notify();
                    },
                )
            }),
        ));
    }
}

impl ModalLayer {
    pub fn new() -> Self {
        Self { open_modal: None }
    }

    // Workspace
    // - ModalLayer parent
    // - - container
    // - - - modal
    // - - - content of the modal
    // - - content of the workspace

    // app
    // workspace
    // container some layer that contains all modals and is 100% wide and high
    // modal (this has a shadow, some witdht)
    // whatever

    pub fn render(&self, workspace: &Workspace, cx: &ViewContext<Workspace>) -> Div<Workspace> {
        let mut parent = div().relative();

        for (_, action) in cx.global::<ModalRegistry>().registered_modals.iter() {
            parent = (action)(parent);
        }

        parent.when_some(self.open_modal.as_ref(), |parent, open_modal| {
            let container1 = div()
                .absolute()
                .size_full()
                .top_0()
                .left_0()
                .right_0()
                .bottom_0();

            // transparent layer
            let container2 = div()
                .flex()
                .h_96()
                .justify_center()
                .size_full()
                .relative()
                .top_20()
                .z_index(400);

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
