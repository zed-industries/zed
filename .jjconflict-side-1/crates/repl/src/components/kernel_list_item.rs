use gpui::AnyElement;
use ui::{Indicator, ListItem, prelude::*};

use crate::KernelSpecification;

#[derive(IntoElement)]
pub struct KernelListItem {
    kernel_specification: KernelSpecification,
    status_color: Color,
    buttons: Vec<AnyElement>,
    children: Vec<AnyElement>,
}

impl KernelListItem {
    pub fn new(kernel_specification: KernelSpecification) -> Self {
        Self {
            kernel_specification,
            status_color: Color::Disabled,
            buttons: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn status_color(mut self, color: Color) -> Self {
        self.status_color = color;
        self
    }

    pub fn button(mut self, button: impl IntoElement) -> Self {
        self.buttons.push(button.into_any_element());
        self
    }

    pub fn buttons(mut self, buttons: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.buttons
            .extend(buttons.into_iter().map(|button| button.into_any_element()));
        self
    }
}

impl ParentElement for KernelListItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for KernelListItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        ListItem::new(self.kernel_specification.name())
            .selectable(false)
            .start_slot(
                h_flex()
                    .size_3()
                    .justify_center()
                    .child(Indicator::dot().color(self.status_color)),
            )
            .children(self.children)
            .end_slot(h_flex().gap_2().children(self.buttons))
    }
}
