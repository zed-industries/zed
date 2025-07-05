use component::{example_group_with_title, single_example};
use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};
use ui::{Label, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct HeaderRow {
    label: SharedString,
    end_slot: Option<AnyElement>,
}

impl HeaderRow {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            end_slot: None,
        }
    }

    pub fn end_slot(mut self, slot: impl IntoElement) -> Self {
        self.end_slot = Some(slot.into_any_element());
        self
    }
}

impl RenderOnce for HeaderRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .h(px(32.))
            .w_full()
            .px_5()
            .justify_between()
            .child(Label::new(self.label))
            .when_some(self.end_slot, |this, slot| this.child(slot))
    }
}

impl Component for HeaderRow {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn sort_name() -> &'static str {
        "RowHeader"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let examples = example_group_with_title(
            "HeaderRow Examples",
            vec![
                single_example(
                    "Simple Header",
                    HeaderRow::new("Pick a Theme").into_any_element(),
                ),
                single_example(
                    "Header with Button",
                    HeaderRow::new("Pick a Theme")
                        .end_slot(
                            Button::new("more_themes", "More Themes")
                                .style(ButtonStyle::Subtle)
                                .color(Color::Muted),
                        )
                        .into_any_element(),
                ),
                single_example(
                    "Header with Icon Button",
                    HeaderRow::new("Settings")
                        .end_slot(
                            IconButton::new("refresh", IconName::RotateCw)
                                .style(ButtonStyle::Subtle),
                        )
                        .into_any_element(),
                ),
            ],
        );

        Some(v_flex().p_4().gap_4().child(examples).into_any_element())
    }
}
