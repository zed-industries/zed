use std::marker::PhantomData;
use std::path::PathBuf;

use gpui2::{AppContext, Div};

use crate::prelude::*;
use crate::{h_stack, HighlightedText};

#[derive(Clone)]
pub struct Symbol(pub Vec<HighlightedText>);

#[derive(IntoAnyElement)]
pub struct Breadcrumb<'a> {
    path: PathBuf,
    symbols: Vec<Symbol>,
    cx: &'a AppContext,
}

impl Breadcrumb {
    pub fn new(path: PathBuf, symbols: Vec<Symbol>, cx: &'a AppContext) -> Self {
        Self { path, symbols, cx }
    }

    fn render_separator<V>(&self, cx: &WindowContext) -> impl IntoAnyElement<V> {
        let color = ThemeColor::new(cx);
        div().child(" › ").text_color(color.text_muted)
    }

    fn render<V>(mut self) -> impl IntoAnyElement<V> {
        let color = ThemeColor::new(cx);

        let symbols_len = self.symbols.len();

        h_stack()
            .id("breadcrumb")
            .px_1()
            .text_sm()
            .text_color(color.text_muted)
            .rounded_md()
            .hover(|style| style.bg(color.ghost_element_hover))
            .active(|style| style.bg(color.ghost_element_active))
            .child(self.path.clone().to_str().unwrap().to_string())
            .child(if !self.symbols.is_empty() {
                self.render_separator(cx)
            } else {
                div()
            })
            .child(
                div().flex().children(
                    self.symbols
                        .iter()
                        .enumerate()
                        // TODO: Could use something like `intersperse` here instead.
                        .flat_map(|(ix, symbol)| {
                            let mut items =
                                vec![div().flex().children(symbol.0.iter().map(|segment| {
                                    div().child(segment.text.clone()).text_color(segment.color)
                                }))];

                            let is_last_segment = ix == symbols_len - 1;
                            if !is_last_segment {
                                items.push(self.render_separator(cx));
                            }

                            items
                        })
                        .collect::<Vec<_>>(),
                ),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use std::str::FromStr;

    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct BreadcrumbStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> BreadcrumbStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            view_state: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            let color = ThemeColor::new(cx);

            Story::container(cx)
                .child(Story::title_for::<_, Breadcrumb<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Breadcrumb::new(
                    PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
                    vec![
                        Symbol(vec![
                            HighlightedText {
                                text: "impl ".to_string(),
                                color: color.syntax.keyword,
                            },
                            HighlightedText {
                                text: "BreadcrumbStory".to_string(),
                                color: color.syntax.function,
                            },
                        ]),
                        Symbol(vec![
                            HighlightedText {
                                text: "fn ".to_string(),
                                color: color.syntax.keyword,
                            },
                            HighlightedText {
                                text: "render".to_string(),
                                color: color.syntax.function,
                            },
                        ]),
                    ],
                ))
        }
    }
}
