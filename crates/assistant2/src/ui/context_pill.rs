use std::rc::Rc;

use gpui::ClickEvent;
use ui::{prelude::*, IconButtonShape};

use crate::context::{Context, ContextKind};

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
        let padding_right = if self.on_remove.is_some() {
            px(2.)
        } else {
            px(4.)
        };
        let icon = match self.context.kind {
            ContextKind::File(_) => IconName::File,
            ContextKind::Directory => IconName::Folder,
            ContextKind::FetchedUrl => IconName::Globe,
            ContextKind::Thread(_) => IconName::MessageCircle,
        };

        h_flex()
            .gap_1()
            .pl_1()
            .pr(padding_right)
            .pb(px(1.))
            .border_1()
            .border_color(cx.theme().colors().border.opacity(0.5))
            .bg(cx.theme().colors().element_background)
            .rounded_md()
            .child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted))
            .child(Label::new(self.context.name.clone()).size(LabelSize::Small))
            .when_some(self.on_remove, |parent, on_remove| {
                parent.child(
                    IconButton::new(("remove", self.context.id.0), IconName::Close)
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
