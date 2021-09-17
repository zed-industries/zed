use crate::presence::Presence;
use gpui::{
    elements::Empty, Element, ElementBox, Entity, ModelHandle, RenderContext, View, ViewContext,
};

pub struct PeoplePanel {
    presence: ModelHandle<Presence>,
}

impl PeoplePanel {
    pub fn new(presence: ModelHandle<Presence>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe(&presence, |_, _, cx| cx.notify());
        Self { presence }
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
