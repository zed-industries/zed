use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::prelude::*;
use crate::{
    theme, Avatar, Button, IconButton, IconColor, PlayerStack, ToolDivider, TrafficLights,
};

#[derive(Element)]
pub struct TitleBar<V: 'static> {
    view_type: PhantomData<V>,
    is_active: Arc<AtomicBool>,
}

impl<V: 'static> TitleBar<V> {
    pub fn new(cx: &mut ViewContext<V>) -> Self {
        let is_active = Arc::new(AtomicBool::new(true));
        let active = is_active.clone();

        cx.observe_window_activation(move |_, is_active, cx| {
            active.store(is_active, std::sync::atomic::Ordering::SeqCst);
            cx.notify();
        })
        .detach();

        Self {
            view_type: PhantomData,
            is_active,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let has_focus = cx.window_is_active();

        let player_list = vec![
            Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4"),
            Avatar::new("https://avatars.githubusercontent.com/u/482957?v=4"),
            Avatar::new("https://avatars.githubusercontent.com/u/326587?v=4"),
            Avatar::new("https://avatars.githubusercontent.com/u/1789?v=4"),
        ];

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h_8()
            .fill(theme.lowest.base.default.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .h_full()
                    .gap_4()
                    .px_2()
                    .child(TrafficLights::new().window_has_focus(has_focus))
                    // === Project Info === //
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(Button::new("maxbrunsfeld"))
                            .child(Button::new("zed"))
                            .child(Button::new("nate/gpui2-ui-components")),
                    )
                    .child(PlayerStack::new(player_list.clone()).player(0))
                    .child(PlayerStack::new(player_list.clone()).player(1))
                    .child(PlayerStack::new(player_list.clone()).player(2)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(IconButton::folder_x())
                            .child(IconButton::close()),
                    )
                    .child(ToolDivider::new())
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(IconButton::mic())
                            .child(IconButton::audio_on())
                            .child(IconButton::screen().color(IconColor::Accent)),
                    )
                    .child(
                        div().px_2().flex().items_center().child(
                            Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .shape(Shape::RoundedRectangle),
                        ),
                    ),
            )
    }
}
