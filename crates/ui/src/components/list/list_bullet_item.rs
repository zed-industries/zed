use crate::{ListItem, prelude::*};
use gpui::{IntoElement, ParentElement, SharedString};

#[derive(IntoElement)]
pub struct ListBulletItem {
    label: SharedString,
}

impl ListBulletItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl RenderOnce for ListBulletItem {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let line_height = window.line_height() * 0.85;

        ListItem::new("list-item")
            .selectable(false)
            .child(
                h_flex()
                    .w_full()
                    .min_w_0()
                    .gap_1()
                    .items_start()
                    .child(
                        h_flex().h(line_height).justify_center().child(
                            Icon::new(IconName::Dash)
                                .size(IconSize::XSmall)
                                .color(Color::Hidden),
                        ),
                    )
                    .child(div().w_full().min_w_0().child(Label::new(self.label))),
            )
            .into_any_element()
    }
}
