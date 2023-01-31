use gpui::{
    color::Color,
    elements::{MouseEventHandler, Svg},
    Appearance, Element, ElementBox, Entity, MouseButton, RenderContext, View,
};

use crate::ToggleScreenSharing;

pub struct SharingStatusIndicator;

impl Entity for SharingStatusIndicator {
    type Event = ();
}

impl View for SharingStatusIndicator {
    fn ui_name() -> &'static str {
        "SharingStatusIndicator"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let color = match cx.appearance {
            Appearance::Light | Appearance::VibrantLight => Color::black(),
            Appearance::Dark | Appearance::VibrantDark => Color::white(),
        };

        MouseEventHandler::<Self>::new(0, cx, |_, _| {
            Svg::new("icons/disable_screen_sharing_12.svg")
                .with_color(color)
                .constrained()
                .with_width(18.)
                .aligned()
                .boxed()
        })
        .on_click(MouseButton::Left, |_, cx| {
            cx.dispatch_action(ToggleScreenSharing);
        })
        .boxed()
    }
}
