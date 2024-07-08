use gpui::ClickEvent;

use crate::{prelude::*, IconButtonShape};

#[derive(IntoElement)]
pub struct NumericStepper {
    value: SharedString,
    on_decrement: Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
    on_increment: Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl NumericStepper {
    pub fn new(
        value: impl Into<SharedString>,
        on_decrement: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
        on_increment: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        Self {
            value: value.into(),
            on_decrement: Box::new(on_decrement),
            on_increment: Box::new(on_increment),
            on_reset: None,
        }
    }

    pub fn on_reset(
        mut self,
        on_reset: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_reset = Some(Box::new(on_reset));
        self
    }
}

impl RenderOnce for NumericStepper {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let shape = IconButtonShape::Square;
        let icon_size = IconSize::Small;

        h_flex()
            .gap_1()
            .map(|element| {
                if let Some(on_reset) = self.on_reset {
                    element.child(
                        IconButton::new("reset", IconName::RotateCcw)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(on_reset),
                    )
                } else {
                    element.child(
                        h_flex()
                            .size(icon_size.square(cx))
                            .flex_none()
                            .into_any_element(),
                    )
                }
            })
            .child(
                h_flex()
                    .gap_1()
                    .px_1()
                    .rounded_sm()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        IconButton::new("decrement", IconName::Dash)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(self.on_decrement),
                    )
                    .child(Label::new(self.value))
                    .child(
                        IconButton::new("increment", IconName::Plus)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(self.on_increment),
                    ),
            )
    }
}
