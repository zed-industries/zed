use crate::{ButtonLink, ListItem, prelude::*};
use component::{Component, ComponentScope, example_group, single_example};
use gpui::{IntoElement, ParentElement, SharedString};

#[derive(IntoElement, RegisterComponent)]
pub struct ListBulletItem {
    label: SharedString,
    label_color: Option<Color>,
    children: Vec<AnyElement>,
}

impl ListBulletItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            label_color: None,
            children: Vec::new(),
        }
    }

    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = Some(color);
        self
    }
}

impl ParentElement for ListBulletItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
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
                    .map(|this| {
                        if !self.children.is_empty() {
                            this.child(h_flex().gap_0p5().flex_wrap().children(self.children))
                        } else {
                            this.child(
                                div().w_full().min_w_0().child(
                                    Label::new(self.label)
                                        .color(self.label_color.unwrap_or(Color::Default)),
                                ),
                            )
                        }
                    }),
            )
            .into_any_element()
    }
}

impl Component for ListBulletItem {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn description() -> Option<&'static str> {
        Some("A list item with a dash indicator for unordered lists.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let basic_examples = vec![
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
            single_example(
                "With Link",
                ListBulletItem::new("")
                    .child(Label::new("Create a Zed account by"))
                    .child(ButtonLink::new("visiting the website", "https://zed.dev"))
                    .into_any_element(),
            ),
        ];

        Some(
            v_flex()
                .gap_6()
                .child(example_group(basic_examples).vertical())
                .into_any_element(),
        )
    }
}
