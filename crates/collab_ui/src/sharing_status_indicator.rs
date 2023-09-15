use crate::toggle_screen_sharing;
use call::ActiveCall;
use gpui::{
    color::Color,
    elements::{MouseEventHandler, Svg},
    platform::{Appearance, MouseButton},
    AnyElement, AppContext, Element, Entity, View, ViewContext,
};
use workspace::WorkspaceSettings;

pub fn init(cx: &mut AppContext) {
    let active_call = ActiveCall::global(cx);

    let mut status_indicator = None;
    cx.observe(&active_call, move |call, cx| {
        if let Some(room) = call.read(cx).room() {
            if room.read(cx).is_screen_sharing() {
                if status_indicator.is_none()
                    && settings::get::<WorkspaceSettings>(cx).show_call_status_icon
                {
                    status_indicator = Some(cx.add_status_bar_item(|_| SharingStatusIndicator));
                }
            } else if let Some(window) = status_indicator.take() {
                window.update(cx, |cx| cx.remove_window());
            }
        } else if let Some(window) = status_indicator.take() {
            window.update(cx, |cx| cx.remove_window());
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let color = match cx.window_appearance() {
            Appearance::Light | Appearance::VibrantLight => Color::black(),
            Appearance::Dark | Appearance::VibrantDark => Color::white(),
        };

        MouseEventHandler::new::<Self, _>(0, cx, |_, _| {
            Svg::new("icons/desktop.svg")
                .with_color(color)
                .constrained()
                .with_width(18.)
                .aligned()
        })
        .on_click(MouseButton::Left, |_, _, cx| {
            toggle_screen_sharing(&Default::default(), cx)
        })
        .into_any()
    }
}
