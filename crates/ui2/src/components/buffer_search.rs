use gpui2::{view, Context, View};

use crate::prelude::*;
use crate::{h_stack, Icon, IconButton, IconColor, Input};

#[derive(Clone)]
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
        view(cx.entity(|cx| Self::new()), Self::render)
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoAnyElement<Self> {
        let theme = theme(cx);

        h_stack().bg(theme.toolbar).p_2().child(
            h_stack().child(Input::new("Search")).child(
                IconButton::<Self>::new("replace", Icon::Replace)
                    .when(self.is_replace_open, |this| this.color(IconColor::Accent))
                    .on_click(|buffer_search, cx| {
                        buffer_search.toggle_replace(cx);
                    }),
            ),
        )
    }
}
