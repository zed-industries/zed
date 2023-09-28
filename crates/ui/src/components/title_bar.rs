use std::marker::PhantomData;

use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};

use crate::prelude::Shape;
use crate::{
    avatar, follow_group, icon_button, text_button, theme, tool_divider, traffic_lights, IconAsset,
    IconColor,
};

#[derive(Element)]
pub struct TitleBar<V: 'static> {
    view_type: PhantomData<V>,
}

pub fn title_bar<V: 'static>() -> TitleBar<V> {
    TitleBar {
        view_type: PhantomData,
    }
}

impl<V: 'static> TitleBar<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let player_list = vec![
            avatar("https://avatars.githubusercontent.com/u/1714999?v=4"),
            avatar("https://avatars.githubusercontent.com/u/1714999?v=4"),
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
                    .child(traffic_lights())
                    // === Project Info === //
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(text_button("maxbrunsfeld"))
                            .child(text_button("zed"))
                            .child(text_button("nate/gpui2-ui-components")),
                    )
                    .child(follow_group(player_list.clone()).player(0))
                    .child(follow_group(player_list.clone()).player(1))
                    .child(follow_group(player_list.clone()).player(2)),
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
                            .child(icon_button().icon(IconAsset::FolderX))
                            .child(icon_button().icon(IconAsset::Close)),
                    )
                    .child(tool_divider())
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(icon_button().icon(IconAsset::Mic))
                            .child(icon_button().icon(IconAsset::AudioOn))
                            .child(
                                icon_button()
                                    .icon(IconAsset::Screen)
                                    .color(IconColor::Accent),
                            ),
                    )
                    .child(
                        div().px_2().flex().items_center().child(
                            avatar("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .shape(Shape::RoundedRectangle),
                        ),
                    ),
            )
    }
}
