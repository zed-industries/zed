use gpui::ClickEvent;

use crate::{IconButtonShape, prelude::*};

#[derive(IntoElement)]
pub struct NumericStepper {
    id: ElementId,
    value: SharedString,
    on_decrement: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    on_increment: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    /// Whether to reserve space for the reset button.
    reserve_space_for_reset: bool,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl NumericStepper {
    pub fn new(
        id: impl Into<ElementId>,
        value: impl Into<SharedString>,
        on_decrement: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        on_increment: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            on_decrement: Box::new(on_decrement),
            on_increment: Box::new(on_increment),
            reserve_space_for_reset: false,
            on_reset: None,
        }
    }

    pub fn reserve_space_for_reset(mut self, reserve_space_for_reset: bool) -> Self {
        self.reserve_space_for_reset = reserve_space_for_reset;
        self
    }

    pub fn on_reset(
        mut self,
        on_reset: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reset = Some(Box::new(on_reset));
        self
    }
}

impl RenderOnce for NumericStepper {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let shape = IconButtonShape::Square;
        let icon_size = IconSize::Small;

        h_flex()
            .id(self.id)
            .gap_1()
            .map(|element| {
                if let Some(on_reset) = self.on_reset {
                    element.child(
                        IconButton::new("reset", IconName::RotateCcw)
                            .shape(shape)
                            .icon_size(icon_size)
                            .on_click(on_reset),
                    )
                } else if self.reserve_space_for_reset {
                    element.child(
                        h_flex()
                            .size(icon_size.square(window, cx))
                            .flex_none()
                            .into_any_element(),
                    )
                } else {
                    element
                }
            })
            .child(
                h_flex()
                    .gap_1()
                    .px_1()
                    .rounded_xs()
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
