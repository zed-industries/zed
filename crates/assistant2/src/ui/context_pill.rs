use ui::prelude::*;

use crate::context::Context;

#[derive(IntoElement)]
pub struct ContextPill {
    context: Context,
}

impl ContextPill {
    pub fn new(context: Context) -> Self {
        Self { context }
    }
}

impl RenderOnce for ContextPill {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .px_1()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(Label::new(self.context.name.clone()).size(LabelSize::Small))
    }
}
