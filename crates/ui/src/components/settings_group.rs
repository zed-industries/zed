use gpui::AnyElement;
use smallvec::SmallVec;

use crate::{ListHeader, prelude::*};

use super::Checkbox;

/// A group of settings.
#[derive(IntoElement, RegisterComponent)]
pub struct SettingsGroup {
    header: SharedString,
    children: SmallVec<[AnyElement; 2]>,
}

impl SettingsGroup {
    pub fn new(header: impl Into<SharedString>) -> Self {
        Self {
            header: header.into(),
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for SettingsGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for SettingsGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .p_1()
            .gap_2()
            .child(ListHeader::new(self.header))
            .children(self.children)
    }
}

impl Component for SettingsGroup {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn name() -> &'static str {
        "SettingsGroup"
    }

    fn description() -> Option<&'static str> {
        Some("A group of settings with a header, used to organize related settings.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Usage",
                        vec![
                            single_example(
                                "Empty Group",
                                SettingsGroup::new("General Settings").into_any_element(),
                            ),
                            single_example(
                                "With Children",
                                SettingsGroup::new("Appearance")
                                    .child(
                                        Checkbox::new("dark_mode", ToggleState::Unselected)
                                            .label("Dark Mode"),
                                    )
                                    .child(
                                        Checkbox::new("high_contrast", ToggleState::Unselected)
                                            .label("High Contrast"),
                                    )
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Multiple Groups",
                        vec![single_example(
                            "Two Groups",
                            v_flex()
                                .gap_4()
                                .child(
                                    SettingsGroup::new("General").child(
                                        Checkbox::new("auto_update", ToggleState::Selected)
                                            .label("Auto Update"),
                                    ),
                                )
                                .child(
                                    SettingsGroup::new("Editor")
                                        .child(
                                            Checkbox::new("line_numbers", ToggleState::Selected)
                                                .label("Show Line Numbers"),
                                        )
                                        .child(
                                            Checkbox::new("word_wrap", ToggleState::Unselected)
                                                .label("Word Wrap"),
                                        ),
                                )
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
