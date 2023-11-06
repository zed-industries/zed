use gpui2::{Div, Render, View, VisualContext};

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
        cx.build_view(|cx| Self::new())
    }
}

impl Render for BufferSearch {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Div<Self> {
        h_stack()
            .bg(cx.theme().colors().toolbar_background)
            .p_2()
            .child(
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
