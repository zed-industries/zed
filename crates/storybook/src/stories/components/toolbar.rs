use std::path::PathBuf;
use std::str::FromStr;

use ui::prelude::*;
use ui::{Breadcrumb, Icon, IconButton, Toolbar};

use crate::story::Story;

#[derive(Element, Default)]
pub struct ToolbarStory {}

impl ToolbarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, Toolbar<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Toolbar::new(
                |_, _| {
                    vec![Breadcrumb::new(
                        PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
                        vec![],
                    )
                    .into_any()]
                },
                Box::new(()),
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
