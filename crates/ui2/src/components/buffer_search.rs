use gpui3::{view, Context, View};

use crate::prelude::*;
use crate::{h_stack, Icon, IconButton, IconColor, Input};

pub struct BufferSearch {
    is_replace_open: bool,
}

impl BufferSearch {
    pub fn new() -> Self {
        Self {
            is_replace_open: false,
        }
    }

    fn toggle_replace(&mut self, cx: &mut ViewContext<Self>) {
        self.is_replace_open = !self.is_replace_open;

        cx.notify();
    }

    pub fn view(cx: &mut WindowContext) -> View<Self> {
        let theme = theme(cx);

        view(cx.entity(|cx| Self::new()), Self::render)
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<ViewState = Self> {
        let theme = theme(cx);

        h_stack()
            .fill(theme.highest.base.default.background)
            .p_2()
            .child(
                h_stack()
                    .child(Input::new("Search (↑/↓ for previous/next query)"))
                    .child(
                        IconButton::<Self>::new(Icon::Replace)
                            .when(self.is_replace_open, |this| this.color(IconColor::Accent))
                            .on_click(|buffer_search, cx| {
                                buffer_search.toggle_replace(cx);
                            }),
                    ),
            )
    }
}
