use gpui::AnyElement;
use smallvec::SmallVec;

use crate::prelude::*;

use super::Checkbox;

#[derive(IntoElement, RegisterComponent)]
pub struct SettingsContainer {
    children: SmallVec<[AnyElement; 2]>,
}

impl Default for SettingsContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsContainer {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for SettingsContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for SettingsContainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex().px_2().gap_1().children(self.children)
    }
}

impl Component for SettingsContainer {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn name() -> &'static str {
        "SettingsContainer"
    }

    fn description() -> Option<&'static str> {
        Some("A container for organizing and displaying settings in a structured manner.")
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), _window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Usage",
                        vec![
                            single_example(
                                "Empty Container",
                                SettingsContainer::new().into_any_element(),
                            ),
                            single_example(
                                "With Content",
                                SettingsContainer::new()
                                    .child(Label::new("Setting 1"))
                                    .child(Label::new("Setting 2"))
                                    .child(Label::new("Setting 3"))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "With Different Elements",
                        vec![single_example(
                            "Mixed Content",
                            SettingsContainer::new()
                                .child(Label::new("Text Setting"))
                                .child(Checkbox::new("checkbox", ToggleState::Unselected))
                                .child(Button::new("button", "Click me"))
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
