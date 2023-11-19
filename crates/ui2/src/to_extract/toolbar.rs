use gpui::{AnyElement, Div, RenderOnce};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Clone)]
pub struct ToolbarItem {}

#[derive(RenderOnce)]
pub struct Toolbar<V: 'static> {
    left_items: SmallVec<[AnyElement<V>; 2]>,
    right_items: SmallVec<[AnyElement<V>; 2]>,
}

impl<V: 'static> Component<V> for Toolbar<V> {
    type Rendered = Div<V>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
        div()
            .bg(cx.theme().colors().toolbar_background)
            .p_2()
            .flex()
            .justify_between()
            .child(div().flex().children(self.left_items))
            .child(div().flex().children(self.right_items))
    }
}

impl<V: 'static> Toolbar<V> {
    pub fn new() -> Self {
        Self {
            left_items: SmallVec::new(),
            right_items: SmallVec::new(),
        }
    }

    pub fn left_item(mut self, child: impl RenderOnce<V>) -> Self
    where
        Self: Sized,
    {
        self.left_items.push(child.render_into_any());
        self
    }

    pub fn left_items(mut self, iter: impl IntoIterator<Item = impl RenderOnce<V>>) -> Self
    where
        Self: Sized,
    {
        self.left_items
            .extend(iter.into_iter().map(|item| item.render_into_any()));
        self
    }

    pub fn right_item(mut self, child: impl RenderOnce<V>) -> Self
    where
        Self: Sized,
    {
        self.right_items.push(child.render_into_any());
        self
    }

    pub fn right_items(mut self, iter: impl IntoIterator<Item = impl RenderOnce<V>>) -> Self
    where
        Self: Sized,
    {
        self.right_items
            .extend(iter.into_iter().map(|item| item.render_into_any()));
        self
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use std::path::PathBuf;
    use std::str::FromStr;

    use gpui::{Div, Render};

    use crate::{Breadcrumb, HighlightedText, Icon, IconButton, Story, Symbol};

    use super::*;

    pub struct ToolbarStory;

    impl Render<Self> for ToolbarStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Toolbar<Self>>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Toolbar::new()
                        .left_item(Breadcrumb::new(
                            PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
                            vec![
                                Symbol(vec![
                                    HighlightedText {
                                        text: "impl ".into(),
                                        color: cx.theme().syntax_color("keyword"),
                                    },
                                    HighlightedText {
                                        text: "ToolbarStory".into(),
                                        color: cx.theme().syntax_color("function"),
                                    },
                                ]),
                                Symbol(vec![
                                    HighlightedText {
                                        text: "fn ".into(),
                                        color: cx.theme().syntax_color("keyword"),
                                    },
                                    HighlightedText {
                                        text: "render".into(),
                                        color: cx.theme().syntax_color("function"),
                                    },
                                ]),
                            ],
                        ))
                        .right_items(vec![
                            IconButton::new("toggle_inlay_hints", Icon::InlayHint),
                            IconButton::new("buffer_search", Icon::MagnifyingGlass),
                            IconButton::new("inline_assist", Icon::MagicWand),
                        ]),
                )
        }
    }
}
