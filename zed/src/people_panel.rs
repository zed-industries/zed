use gpui::{
    elements::Empty, Element, ElementBox, Entity, ModelHandle, RenderContext, View, ViewContext,
};

use crate::user::UserStore;

pub struct PeoplePanel {
    user_store: ModelHandle<UserStore>,
}

impl PeoplePanel {
    pub fn new(user_store: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&user_store, |_, _, cx| cx.notify());
        Self { user_store }
    }
}

pub enum Event {}

impl Entity for PeoplePanel {
    type Event = Event;
}

impl View for PeoplePanel {
    fn ui_name() -> &'static str {
        "PeoplePanel"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        Empty::new().boxed()
    }
}
