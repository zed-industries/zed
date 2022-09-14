use gpui::{color::Color, elements::*, Entity, RenderContext, View};

pub struct ContactsPopover;

impl Entity for ContactsPopover {
    type Event = ();
}

impl View for ContactsPopover {
    fn ui_name() -> &'static str {
        "ContactsPopover"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        Empty::new()
            .contained()
            .with_background_color(Color::red())
            .boxed()
    }
}

impl ContactsPopover {
    pub fn new() -> Self {
        Self
    }
}
