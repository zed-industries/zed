use gpui::{color::Color, elements::*, Entity, RenderContext, View};

pub struct ContactsStatusItem;

impl Entity for ContactsStatusItem {
    type Event = ();
}

impl View for ContactsStatusItem {
    fn ui_name() -> &'static str {
        "ContactsStatusItem"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        MouseEventHandler::new::<Self, _, _>(0, cx, |state, cx| {
            Svg::new("icons/zed_32.svg")
                .with_color(if state.clicked.is_some() {
                    Color::red()
                } else {
                    Color::blue()
                })
                .boxed()
        })
        .on_down(gpui::MouseButton::Left, |_, cx| {})
        .on_up(gpui::MouseButton::Left, |_, cx| {})
        .contained()
        .with_background_color(Color::green())
        .boxed()
    }
}

impl ContactsStatusItem {
    pub fn new() -> Self {
        Self
    }
}
