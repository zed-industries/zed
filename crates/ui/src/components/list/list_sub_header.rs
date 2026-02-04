use crate::prelude::*;
use component::{Component, ComponentScope, example_group_with_title, single_example};

#[derive(IntoElement, RegisterComponent)]
pub struct ListSubHeader {
    label: SharedString,
    start_slot: Option<IconName>,
    end_slot: Option<AnyElement>,
    inset: bool,
    selected: bool,
}

impl ListSubHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            start_slot: None,
            end_slot: None,
            inset: false,
            selected: false,
        }
    }

    pub fn left_icon(mut self, left_icon: Option<IconName>) -> Self {
        self.start_slot = left_icon;
        self
    }

    pub fn end_slot(mut self, end_slot: AnyElement) -> Self {
        self.end_slot = Some(end_slot);
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }
}

impl Toggleable for ListSubHeader {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for ListSubHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .flex_1()
            .w_full()
            .relative()
            .pb(DynamicSpacing::Base04.rems(cx))
            .px(DynamicSpacing::Base02.rems(cx))
            .child(
                div()
                    .h_5()
                    .when(self.inset, |this| this.px_2())
                    .when(self.selected, |this| {
                        this.bg(cx.theme().colors().ghost_element_selected)
                    })
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
                                self.start_slot.map(|i| {
                                    Icon::new(i).color(Color::Muted).size(IconSize::Small)
                                }),
                            )
                            .child(
                                Label::new(self.label.clone())
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .children(self.end_slot),
            )
    }
}

impl Component for ListSubHeader {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn description() -> Option<&'static str> {
        Some(
            "A sub-header component for organizing list content into subsections with optional icons and end slots.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Sub-headers",
                        vec![
                            single_example(
                                "Simple",
                                ListSubHeader::new("Subsection").into_any_element(),
                            ),
                            single_example(
                                "With Icon",
                                ListSubHeader::new("Documents")
                                    .left_icon(Some(IconName::File))
                                    .into_any_element(),
                            ),
                            single_example(
                                "With End Slot",
                                ListSubHeader::new("Recent")
                                    .end_slot(
                                        Label::new("3").color(Color::Muted).into_any_element(),
                                    )
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "States",
                        vec![
                            single_example(
                                "Selected",
                                ListSubHeader::new("Selected")
                                    .toggle_state(true)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Inset",
                                ListSubHeader::new("Inset Sub-header")
                                    .inset(true)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
