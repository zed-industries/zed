use call::ActiveCall;
use gpui::{
    color::Color,
    elements::{MouseEventHandler, Svg},
    Appearance, Element, ElementBox, Entity, MouseButton, MutableAppContext, RenderContext, View,
};
use settings::Settings;

use crate::ToggleScreenSharing;

pub fn init(cx: &mut MutableAppContext) {
    let active_call = ActiveCall::global(cx);

    let mut status_indicator = None;
    cx.observe(&active_call, move |call, cx| {
        if let Some(room) = call.read(cx).room() {
            if room.read(cx).is_screen_sharing() {
                if status_indicator.is_none() && cx.global::<Settings>().show_call_status_icon {
                    status_indicator = Some(cx.add_status_bar_item(|_| SharingStatusIndicator));
                }
            } else if let Some((window_id, _)) = status_indicator.take() {
                cx.remove_status_bar_item(window_id);
            }
        } else if let Some((window_id, _)) = status_indicator.take() {
            cx.remove_status_bar_item(window_id);
        }
    })
    .detach();
}

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
