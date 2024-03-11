use crate::prelude::*;
use crate::{h_flex, Icon, IconName, IconSize, Label};

#[derive(IntoElement)]
pub struct ListSubHeader {
    label: SharedString,
    start_slot: Option<IconName>,
    inset: bool,
}

impl ListSubHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            start_slot: None,
            inset: false,
        }
    }

    pub fn left_icon(mut self, left_icon: Option<IconName>) -> Self {
        self.start_slot = left_icon;
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }
}

impl RenderOnce for ListSubHeader {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        h_flex().flex_1().w_full().relative().py_1().child(
            div()
                .h_6()
                .when(self.inset, |this| this.px_2())
                .flex()
                .flex_1()
                .w_full()
                .gap_1()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .gap_1()
                        .items_center()
                        .children(
                            self.start_slot
                                .map(|i| Icon::new(i).color(Color::Muted).size(IconSize::Small)),
                        )
                        .child(Label::new(self.label.clone()).color(Color::Muted)),
                ),
        )
    }
}
