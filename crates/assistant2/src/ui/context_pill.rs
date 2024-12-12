use std::rc::Rc;

use gpui::ClickEvent;
use ui::{prelude::*, IconButtonShape};

use crate::context::Context;

#[derive(IntoElement)]
pub struct ContextPill {
    context: Context,
    on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext)>>,
}

impl ContextPill {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            on_remove: None,
        }
    }

    pub fn on_remove(mut self, on_remove: Rc<dyn Fn(&ClickEvent, &mut WindowContext)>) -> Self {
        self.on_remove = Some(on_remove);
        self
    }
}

impl RenderOnce for ContextPill {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .gap_1()
            .px_1()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(Label::new(self.context.name.clone()).size(LabelSize::Small))
            .when_some(self.on_remove, |parent, on_remove| {
                parent.child(
                    IconButton::new("remove", IconName::Close)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::XSmall)
                        .on_click({
                            let on_remove = on_remove.clone();
                            move |event, cx| on_remove(event, cx)
                        }),
                )
            })
    }
}
