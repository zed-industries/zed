use gpui::{svg, Hsla, IntoElement};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use ui_macros::DerivePathStr;

use crate::prelude::*;

const ICON_2_SIZE: f32 = 0.9375; // 15px @ 16px/rem
const SMALL_ICON_2_SIZE: f32 = 0.6875; // 11px @ 16px/rem

#[derive(
    Display,
    Debug,
    PartialEq,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Serialize,
    Deserialize,
    DerivePathStr,
)]
#[strum(serialize_all = "snake_case")]
#[path_str(prefix = "icons_2/15", suffix = ".svg")]
pub enum IconName2 {
    ArrowLeft,
    ArrowRight,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    DesktopDisplay,
    File,
    Folder,
    XMark,
}

#[derive(IntoElement)]
pub struct Icon2 {
    icon: IconName2,
    color: Hsla,
}

impl Icon2 {
    pub fn new(cx: &WindowContext, icon: IconName2) -> Self {
        Self {
            icon,
            color: Color::default().color(cx),
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Icon2 {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        svg()
            .size(rems(ICON_2_SIZE))
            .flex_none()
            .path(self.icon.path())
            .text_color(self.color)
    }
}

#[derive(
    Display,
    Debug,
    PartialEq,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Serialize,
    Deserialize,
    DerivePathStr,
)]
#[strum(serialize_all = "snake_case")]
#[path_str(prefix = "icons_2/11", suffix = ".svg")]
pub enum SmallIconName2 {
    ArrowLeft,
    ArrowRight,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    DesktopDisplay,
    File,
    Folder,
    XMark,
}

#[derive(IntoElement)]
pub struct SmallIcon2 {
    icon: SmallIconName2,
    color: Hsla,
}

impl SmallIcon2 {
    pub fn new(cx: &WindowContext, icon: SmallIconName2) -> Self {
        Self {
            icon,
            color: Color::default().color(cx),
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for SmallIcon2 {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        svg()
            .size(rems(SMALL_ICON_2_SIZE))
            .flex_none()
            .path(self.icon.path())
            .text_color(self.color)
    }
}

#[cfg(feature = "stories")]
pub mod story {
    use gpui::Render;
    use story::Story;
    use strum::IntoEnumIterator;

    use crate::prelude::*;

    use super::{Icon2, IconName2, SmallIcon2, SmallIconName2};

    pub struct Icon2Story;

    impl Render for Icon2Story {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
            let icons = IconName2::iter().map(|icon| {
                div()
                    // .bg(cx.theme().colors().surface_background)
                    .p_2()
                    .rounded_md()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(Icon2::new(cx, icon))
                // .child(icon.to_string())
            });
            let small_icons = SmallIconName2::iter().map(|icon| {
                v_flex()
                    .gap_0p5()
                    .text_xs()
                    .text_color(cx.theme().colors().text_muted)
                    .child(SmallIcon2::new(cx, icon))
                // .child(icon.to_string())
            });

            let layout = Story::container()
                .text_color(cx.theme().colors().text)
                .h_flex()
                .child(
                    // Controls
                    v_flex()
                        .w_64()
                        .h_full()
                        .bg(cx.theme().colors().surface_background)
                        .child(
                            div()
                                .p_4()
                                .text_color(cx.theme().colors().text)
                                .child("Icons"),
                        ),
                )
                .child(
                    div()
                        .flex_none()
                        .h_full()
                        .w_px()
                        .bg(cx.theme().colors().border),
                )
                .child(
                    // Icons
                    v_flex()
                        .h_full()
                        .p_4()
                        .gap_4()
                        .flex_grow()
                        .min_w_56()
                        .max_w(rems(48.))
                        .bg(cx.theme().colors().background)
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().colors().text_placeholder)
                                .child("Future filter input"),
                        )
                        .child(
                            h_flex()
                                .flex_wrap()
                                .gap_4()
                                .children(icons.clone())
                                .children(icons.clone())
                                .children(icons.clone())
                                .children(icons.clone())
                                .children(icons.clone())
                                .children(icons.clone())
                                .children(icons.clone())
                                .children(icons.clone())
                                .children(icons),
                        ),
                )
                .child(
                    div()
                        .flex_none()
                        .h_full()
                        .w_px()
                        .bg(cx.theme().colors().border),
                )
                .child(
                    // Icon Preview
                    v_flex()
                        .bg(cx.theme().colors().elevated_surface_background)
                        .h_full()
                        .flex_grow()
                        .min_w_56()
                        .p_4()
                        .gap_4()
                        .child("Nothing Selected"),
                );

            // Story::container().child(
            //     Story::section()
            //         .max_w(rems(48.))
            //         .gap_3()
            //         .child(
            //             Story::section_title().child("Icons").child(
            //                 h_flex()
            //                     .flex_wrap()
            //                     .gap_3()
            //                     .p_3()
            //                     .bg(cx.theme().colors().background)
            //                     .children(icons),
            //             ),
            //         )
            //         .child(Story::divider())
            //         .child(
            //             Story::section_title().child("Small Icons").child(
            //                 h_flex()
            //                     .p_3()
            //                     .bg(cx.theme().colors().background)
            //                     .flex_wrap()
            //                     .gap_3()
            //                     .children(small_icons),
            //             ),
            //         )
            //         .child(Story::divider())
            //         .child(
            //             Story::section_title().child("Examples").child(
            //                 v_flex().gap_2().child(
            //                     h_flex()
            //                         .gap_1()
            //                         .px_1p5()
            //                         .py_1()
            //                         .bg(cx.theme().colors().element_background)
            //                         .flex_initial()
            //                         .min_w(px(1.))
            //                         .child(SmallIcon2::new(cx, SmallIconName2::XMark))
            //                         .child(
            //                             h_flex()
            //                                 .gap_1p5()
            //                                 .text_size(px(15.))
            //                                 .text_color(Color::Default.color(cx))
            //                                 .font_ui(cx)
            //                                 .line_height(px(15.))
            //                                 .child("Delete"),
            //                         ),
            //                 ),
            //             ),
            //         ),
            // )

            layout
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_2_path() {
        assert_eq!(SmallIconName2::XMark.path(), "icons_2/11/x_mark.svg");
        assert_eq!(IconName2::XMark.path(), "icons_2/15/x_mark.svg");
    }
}
