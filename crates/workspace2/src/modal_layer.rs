use crate::Workspace;
use gpui::{
    div, px, AnyView, Component, Div, EventEmitter, ParentElement, Render, StatelessInteractive,
    Styled, Subscription, View, ViewContext,
};
use std::{any::TypeId, sync::Arc};
use ui::v_stack;

pub struct ModalLayer {
    open_modal: Option<AnyView>,
    subscription: Option<Subscription>,
    registered_modals: Vec<(TypeId, Box<dyn Fn(Div<Workspace>) -> Div<Workspace>>)>,
}

pub enum ModalEvent {
    Dismissed,
}

impl ModalLayer {
    pub fn new() -> Self {
        Self {
            open_modal: None,
            subscription: None,
            registered_modals: Vec::new(),
        }
    }

    pub fn register_modal<A: 'static, V, B>(&mut self, action: A, build_view: B)
    where
        V: EventEmitter<ModalEvent> + Render,
        B: Fn(&mut Workspace, &mut ViewContext<Workspace>) -> Option<View<V>> + 'static,
    {
        let build_view = Arc::new(build_view);

        self.registered_modals.push((
            TypeId::of::<A>(),
            Box::new(move |mut div| {
                let build_view = build_view.clone();

                div.on_action(move |workspace, event: &A, cx| {
                    let Some(new_modal) = (build_view)(workspace, cx) else {
                        return;
                    };
                    workspace.modal_layer().show_modal(new_modal, cx);
                })
            }),
        ));
    }

    pub fn show_modal<V>(&mut self, new_modal: View<V>, cx: &mut ViewContext<Workspace>)
    where
        V: EventEmitter<ModalEvent> + Render,
    {
        self.subscription = Some(cx.subscribe(&new_modal, |this, modal, e, cx| match e {
            ModalEvent::Dismissed => this.modal_layer().hide_modal(cx),
        }));
        self.open_modal = Some(new_modal.into());
        cx.notify();
    }

    pub fn hide_modal(&mut self, cx: &mut ViewContext<Workspace>) {
        self.open_modal.take();
        self.subscription.take();
        cx.notify();
    }

    pub fn wrapper_element(&self, cx: &ViewContext<Workspace>) -> Div<Workspace> {
        let mut parent = div().relative().size_full();

        for (_, action) in self.registered_modals.iter() {
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
