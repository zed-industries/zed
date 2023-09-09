use std::marker::PhantomData;

use crate::component::icon_button::{icon_button, ButtonVariant};
use crate::component::tab::tab;
use crate::theme::theme;
use gpui2::elements::div::ScrollState;
use gpui2::style::StyleHelpers;
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct TabBar<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
}

pub fn tab_bar<V: 'static>(scroll_state: ScrollState) -> TabBar<V> {
    TabBar {
        view_type: PhantomData,
        scroll_state,
    }
}

impl<V: 'static> TabBar<V> {
    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div()
            .w_full()
            .flex()
            .items_center()
            .fill(theme.highest.base.default.background)
            // Left Side
            .child(
                div()
                    .px_1()
                    .flex()
                    // Nate
                    // This isn't what I wanted, but I wanted to try to get at least SOME x overflow scroll working
                    // Ideally this should be on the "Tabs" div below
                    // So only the tabs scroll, and the nav buttons stay pinned left, and the other controls stay pinned right
                    .overflow_x_scroll(self.scroll_state.clone())
                    .gap_2()
                    // Nav Buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(icon_button("icons/arrow_left.svg", ButtonVariant::Ghost))
                            .child(icon_button("icons/arrow_right.svg", ButtonVariant::Ghost)),
                    )
                    // Tabs
                    .child(
                        div()
                            .py_1()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(tab("Cargo.toml", false))
                            .child(tab("Channels Panel", true))
                            .child(tab("channels_panel.rs", false))
                            .child(tab("workspace.rs", false))
                            .child(tab("icon_button.rs", false))
                            .child(tab("storybook.rs", false))
                            .child(tab("theme.rs", false))
                            .child(tab("theme_registry.rs", false))
                            .child(tab("styleable_helpers.rs", false)),
                    ),
            )
            // Right Side
            .child(
                div()
                    .px_1()
                    .flex()
                    .flex_initial()
                    .gap_2()
                    // Nav Buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_px()
                            .child(icon_button("icons/plus.svg", ButtonVariant::Ghost))
                            .child(icon_button("icons/split.svg", ButtonVariant::Ghost)),
                    ),
            )
    }
}
