use std::path::PathBuf;
use std::str::FromStr;

use ui::prelude::*;
use ui::{Breadcrumb, HighlightedText, Symbol};

use crate::story::Story;

#[derive(Element, Default)]
pub struct BreadcrumbStory {}

impl BreadcrumbStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        Story::container(cx)
            .child(Story::title_for::<_, Breadcrumb>(cx))
            .child(Story::label(cx, "Default"))
            .child(Breadcrumb::new(
                PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
                vec![
                    Symbol(vec![
                        HighlightedText {
                            text: "impl ".to_string(),
                            color: HighlightColor::Keyword.hsla(&theme),
                        },
                        HighlightedText {
                            text: "BreadcrumbStory".to_string(),
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
            ))
    }
}
