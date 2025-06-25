use component::{Component, example_group_with_title, single_example};
use gpui::{AnyElement, App, IntoElement, ParentElement as _, Styled as _, Window};
use ui_macros::RegisterComponent;

use crate::v_flex;

#[derive(RegisterComponent)]
struct Table;

impl Component for Table {
    fn name() -> &'static str {
        "Uniform Table"
    }

    fn scope() -> component::ComponentScope {
        component::ComponentScope::Layout
    }

    fn description() -> Option<&'static str> {
        Some("A table with uniform rows")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let data = vec![
            ["Alice", "25", "New York"],
            ["Bob", "30", "Los Angeles"],
            ["Charlie", "35", "Chicago"],
            ["Sam", "27", "Detroit"],
        ];
        Some(
            v_flex()
                .gap_6()
                .children([example_group_with_title(
                    "Basic",
                    vec![single_example(
                        "Simple Table",
                        gpui::uniform_table("simple table", 4)
                            .header(["Name", "Age", "City"])
                            .rows(move |range, _, _| {
                                data[range]
                                    .iter()
                                    .cloned()
                                    .map(|arr| arr.map(IntoElement::into_any_element))
                                    .collect()
                            })
                            .into_any_element(),
                    )],
                )])
                .into_any_element(),
        )
    }
}
