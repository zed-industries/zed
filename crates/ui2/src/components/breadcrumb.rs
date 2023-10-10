use std::marker::PhantomData;
use std::path::PathBuf;

use gpui3::Div;

use crate::prelude::*;
use crate::{h_stack, HighlightedText};

#[derive(Clone)]
pub struct Symbol(pub Vec<HighlightedText>);

#[derive(Element)]
pub struct Breadcrumb<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    path: PathBuf,
    symbols: Vec<Symbol>,
}

impl<S: 'static + Send + Sync + Clone> Breadcrumb<S> {
    pub fn new(path: PathBuf, symbols: Vec<Symbol>) -> Self {
        Self {
            state_type: PhantomData,
            path,
            symbols,
        }
    }

    fn render_separator(&self, theme: &Theme) -> Div<S> {
        div()
            .child(" › ")
            .text_color(HighlightColor::Default.hsla(theme))
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        let symbols_len = self.symbols.len();

        h_stack()
            .px_1()
            // TODO: Read font from theme (or settings?).
            .font("Zed Mono Extended")
            .text_sm()
            .text_color(theme.middle.base.default.foreground)
            .rounded_md()
            .hover()
            .fill(theme.highest.base.hovered.background)
            .child(self.path.clone().to_str().unwrap().to_string())
            .child(if !self.symbols.is_empty() {
                self.render_separator(&theme)
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
                                items.push(self.render_separator(&theme));
                            }

                            items
                        })
                        .collect::<Vec<_>>(),
                ),
            )
    }
}
