use std::path::PathBuf;
use std::str::FromStr;

use ui::prelude::*;
use ui::Breadcrumb;

use crate::story::Story;

#[derive(Element, Default)]
pub struct BreadcrumbStory {}

impl BreadcrumbStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, Breadcrumb>(cx))
            .child(Story::label(cx, "Default"))
            .child(Breadcrumb::new(
                PathBuf::from_str("crates/ui/src/components/toolbar.rs").unwrap(),
            ))
    }
}
