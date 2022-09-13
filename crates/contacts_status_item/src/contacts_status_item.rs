use gpui::{elements::*, Entity, RenderContext, View};

pub struct ContactsStatusItem;

impl Entity for ContactsStatusItem {
    type Event = ();
}

impl View for ContactsStatusItem {
    fn ui_name() -> &'static str {
        "ContactsStatusItem"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        Svg::new("icons/zed_22.svg").aligned().boxed()
    }
}

impl ContactsStatusItem {
    pub fn new() -> Self {
        Self
    }
}
