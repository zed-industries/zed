use crate::prelude::*;
use crate::theme;

#[derive(Clone)]
pub struct ToolbarItem {}

#[derive(Element)]
pub struct Toolbar<S: 'static + Send + Sync> {
    left_items: HackyChildren<S>,
    left_items_payload: HackyChildrenPayload,
    right_items: HackyChildren<S>,
    right_items_payload: HackyChildrenPayload,
}

impl<S: 'static + Send + Sync> Toolbar<S> {
    pub fn new(
        left_items: HackyChildren<S>,
        left_items_payload: HackyChildrenPayload,
        right_items: HackyChildren<S>,
        right_items_payload: HackyChildrenPayload,
    ) -> Self {
        Self {
            left_items,
            left_items_payload,
            right_items,
            right_items_payload,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        div()
            .fill(theme.highest.base.default.background)
            .p_2()
            .flex()
            .justify_between()
            .child(
                div()
                    .flex()
                    .children_any((self.left_items)(cx, self.left_items_payload.as_ref())),
            )
            .child(
                div()
                    .flex()
                    .children_any((self.right_items)(cx, self.right_items_payload.as_ref())),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use std::marker::PhantomData;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;

    use crate::{Breadcrumb, HighlightedText, Icon, IconButton, Story, Symbol};

    use super::*;

    #[derive(Element)]
    pub struct ToolbarStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> ToolbarStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
            let theme = theme(cx);

            struct LeftItemsPayload {
                pub theme: Arc<Theme>,
            }

            Story::container(cx)
                .child(Story::title_for::<_, Toolbar<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Toolbar::new(
                    |_, payload| {
                        let payload = payload.downcast_ref::<LeftItemsPayload>().unwrap();

                        let theme = payload.theme.clone();

                        vec![Breadcrumb::new(
                            PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
                            vec![
                                Symbol(vec![
                                    HighlightedText {
                                        text: "impl ".to_string(),
                                        color: HighlightColor::Keyword.hsla(&theme),
                                    },
                                    HighlightedText {
                                        text: "ToolbarStory".to_string(),
                                        color: HighlightColor::Function.hsla(&theme),
                                    },
                                ]),
                                Symbol(vec![
                                    HighlightedText {
                                        text: "fn ".to_string(),
                                        color: HighlightColor::Keyword.hsla(&theme),
                                    },
                                    HighlightedText {
                                        text: "render".to_string(),
                                        color: HighlightColor::Function.hsla(&theme),
                                    },
                                ]),
                            ],
                        )
                        .into_any()]
                    },
                    Box::new(LeftItemsPayload {
                        theme: theme.clone(),
                    }),
                    |_, _| {
                        vec![
                            IconButton::new(Icon::InlayHint).into_any(),
                            IconButton::new(Icon::MagnifyingGlass).into_any(),
                            IconButton::new(Icon::MagicWand).into_any(),
                        ]
                    },
                    Box::new(()),
                ))
        }
    }
}
