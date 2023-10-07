use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use ui::prelude::*;
use ui::{theme, Breadcrumb, HighlightColor, HighlightedText, Icon, IconButton, Symbol, Toolbar};

use crate::story::Story;

#[derive(Element, Default)]
pub struct ToolbarStory {}

impl ToolbarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        struct LeftItemsPayload {
            pub theme: Arc<Theme>,
        }

        Story::container(cx)
            .child(Story::title_for::<_, Toolbar<V>>(cx))
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
