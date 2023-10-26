use gpui2::AnyElement;
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Clone)]
pub struct ToolbarItem {}

#[derive(Component)]
pub struct Toolbar<S: 'static> {
    left_items: SmallVec<[AnyElement<S>; 2]>,
    right_items: SmallVec<[AnyElement<S>; 2]>,
}

impl<S: 'static> Toolbar<S> {
    pub fn new() -> Self {
        Self {
            left_items: SmallVec::new(),
            right_items: SmallVec::new(),
        }
    }

    pub fn left_item(mut self, child: impl Component<S>) -> Self
    where
        Self: Sized,
    {
        self.left_items.push(child.render());
        self
    }

    pub fn left_items(mut self, iter: impl IntoIterator<Item = impl Component<S>>) -> Self
    where
        Self: Sized,
    {
        self.left_items
            .extend(iter.into_iter().map(|item| item.render()));
        self
    }

    pub fn right_item(mut self, child: impl Component<S>) -> Self
    where
        Self: Sized,
    {
        self.right_items.push(child.render());
        self
    }

    pub fn right_items(mut self, iter: impl IntoIterator<Item = impl Component<S>>) -> Self
    where
        Self: Sized,
    {
        self.right_items
            .extend(iter.into_iter().map(|item| item.render()));
        self
    }

    fn render(mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
        let theme = theme(cx);

        div()
            .bg(theme.toolbar)
            .p_2()
            .flex()
            .justify_between()
            .child(div().flex().children(self.left_items.drain(..)))
            .child(div().flex().children(self.right_items.drain(..)))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::{Breadcrumb, HighlightedText, Icon, IconButton, Story, Symbol};

    use super::*;

    #[derive(Component)]
    pub struct ToolbarStory;

    impl ToolbarStory {
        pub fn new() -> Self {
            Self
        }

        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            let theme = theme(cx);

            Story::container(cx)
                .child(Story::title_for::<_, Toolbar<V>>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Toolbar::new()
                        .left_item(Breadcrumb::new(
                            PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
                            vec![
                                Symbol(vec![
                                    HighlightedText {
                                        text: "impl ".to_string(),
                                        color: theme.syntax.keyword,
                                    },
                                    HighlightedText {
                                        text: "ToolbarStory".to_string(),
                                        color: theme.syntax.function,
                                    },
                                ]),
                                Symbol(vec![
                                    HighlightedText {
                                        text: "fn ".to_string(),
                                        color: theme.syntax.keyword,
                                    },
                                    HighlightedText {
                                        text: "render".to_string(),
                                        color: theme.syntax.function,
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
