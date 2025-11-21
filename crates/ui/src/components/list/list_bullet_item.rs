use crate::{ListItem, prelude::*};
use component::{Component, ComponentScope, example_group_with_title, single_example};
use gpui::{IntoElement, ParentElement, SharedString};

#[derive(IntoElement, RegisterComponent)]
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

impl Component for ListBulletItem {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn description() -> Option<&'static str> {
        Some("A list item with a bullet point indicator for unordered lists.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .child(example_group_with_title(
                    "Bullet Items",
                    vec![
                        single_example(
                            "Simple",
                            ListBulletItem::new("First bullet item").into_any_element(),
                        ),
                        single_example(
                            "Multiple Lines",
                            v_flex()
                                .child(ListBulletItem::new("First item"))
                                .child(ListBulletItem::new("Second item"))
                                .child(ListBulletItem::new("Third item"))
                                .into_any_element(),
                        ),
                        single_example(
                            "Long Text",
                            ListBulletItem::new(
                                "A longer bullet item that demonstrates text wrapping behavior",
                            )
                            .into_any_element(),
                        ),
                    ],
                ))
                .into_any_element(),
        )
    }
}
