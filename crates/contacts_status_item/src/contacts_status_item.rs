use gpui::{color::Color, elements::*, Appearance, Entity, RenderContext, View};

pub struct ContactsStatusItem;

impl Entity for ContactsStatusItem {
    type Event = ();
}

impl View for ContactsStatusItem {
    fn ui_name() -> &'static str {
        "ContactsStatusItem"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let color = match cx.appearance {
            Appearance::Light | Appearance::VibrantLight => Color::black(),
            Appearance::Dark | Appearance::VibrantDark => Color::white(),
        };
        Svg::new("icons/zed_22.svg")
            .with_color(color)
            .aligned()
            .boxed()
    }
}

impl ContactsStatusItem {
    pub fn new() -> Self {
        Self
    }
}
