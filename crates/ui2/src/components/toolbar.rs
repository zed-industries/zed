use gpui2::AnyElement;
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Clone)]
pub struct ToolbarItem {}

#[derive(IntoAnyElement)]
pub struct Toolbar<S: 'static + Send + Sync> {
    left_items: SmallVec<[AnyElement<S>; 2]>,
    right_items: SmallVec<[AnyElement<S>; 2]>,
}

impl<S: 'static + Send + Sync> Toolbar<S> {
    pub fn new() -> Self {
        Self {
            left_items: SmallVec::new(),
            right_items: SmallVec::new(),
        }
    }

    pub fn left_item(mut self, child: impl IntoAnyElement<S>) -> Self
    where
        Self: Sized,
    {
        self.left_items.push(child.into_any());
        self
    }

    pub fn left_items(mut self, iter: impl IntoIterator<Item = impl IntoAnyElement<S>>) -> Self
    where
        Self: Sized,
    {
        self.left_items
            .extend(iter.into_iter().map(|item| item.into_any()));
        self
    }

    pub fn right_item(mut self, child: impl IntoAnyElement<S>) -> Self
    where
        Self: Sized,
    {
        self.right_items.push(child.into_any());
        self
    }

    pub fn right_items(mut self, iter: impl IntoIterator<Item = impl IntoAnyElement<S>>) -> Self
    where
        Self: Sized,
    {
        self.right_items
            .extend(iter.into_iter().map(|item| item.into_any()));
        self
    }

    fn render(mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
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
    use std::marker::PhantomData;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::{Breadcrumb, HighlightedText, Icon, IconButton, Story, Symbol};

    use super::*;

    #[derive(IntoAnyElement)]
    pub struct ToolbarStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> ToolbarStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
            let theme = theme(cx);

            Story::container(cx)
                .child(Story::title_for::<_, Toolbar<S>>(cx))
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
