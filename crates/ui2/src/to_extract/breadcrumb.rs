use crate::{h_stack, prelude::*, HighlightedText};
use gpui::{prelude::*, Div, Stateful};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Symbol(pub Vec<HighlightedText>);

#[derive(RenderOnce)]
pub struct Breadcrumb {
    path: PathBuf,
    symbols: Vec<Symbol>,
}

impl Component for Breadcrumb {
    type Rendered = gpui::Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let symbols_len = self.symbols.len();
        h_stack()
            .id("breadcrumb")
            .px_1()
            .text_ui_sm()
            .text_color(cx.theme().colors().text_muted)
            .rounded_md()
            .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
            .active(|style| style.bg(cx.theme().colors().ghost_element_active))
            .child(SharedString::from(
                self.path.clone().to_str().unwrap().to_string(),
            ))
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

impl Breadcrumb {
    pub fn new(path: PathBuf, symbols: Vec<Symbol>) -> Self {
        Self { path, symbols }
    }

    fn render_separator(&self, cx: &WindowContext) -> Div {
        div()
            .child(" › ")
            .text_color(cx.theme().colors().text_muted)
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::Render;
    use std::str::FromStr;

    pub struct BreadcrumbStory;

    impl Render for BreadcrumbStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Breadcrumb>(cx))
                .child(Story::label(cx, "Default"))
                .child(Breadcrumb::new(
                    PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
                    vec![
                        Symbol(vec![
                            HighlightedText {
                                text: "impl ".into(),
                                color: cx.theme().syntax_color("keyword"),
                            },
                            HighlightedText {
                                text: "BreadcrumbStory".into(),
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
        }
    }
}
