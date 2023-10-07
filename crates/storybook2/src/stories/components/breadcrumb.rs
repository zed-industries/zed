use std::marker::PhantomData;
use std::path::PathBuf;
use std::str::FromStr;

use ui::prelude::*;
use ui::{Breadcrumb, HighlightedText, Symbol};

use crate::story::Story;

#[derive(Element)]
pub struct BreadcrumbStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> BreadcrumbStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        let theme = theme(cx);

        Story::container(cx)
            .child(Story::title_for::<_, Breadcrumb<S>>(cx))
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
