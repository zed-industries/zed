use std::path::PathBuf;

use crate::prelude::*;
use crate::{h_stack, theme};

#[derive(Element)]
pub struct Breadcrumb {
    path: PathBuf,
}

impl Breadcrumb {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

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
            .child(" › ")
            .child("impl Breadcrumb")
            .child(" › ")
            .child("fn render")
    }
}
