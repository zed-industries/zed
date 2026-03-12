use crate::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct DiffStat {
    id: ElementId,
    added: usize,
    removed: usize,
    label_size: LabelSize,
}

impl DiffStat {
    pub fn new(id: impl Into<ElementId>, added: usize, removed: usize) -> Self {
        Self {
            id: id.into(),
            added,
            removed,
            label_size: LabelSize::Small,
        }
    }

    pub fn label_size(mut self, label_size: LabelSize) -> Self {
        self.label_size = label_size;
        self
    }
}

impl RenderOnce for DiffStat {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .gap_1()
            .child(
                Label::new(format!("+\u{2009}{}", self.added))
                    .color(Color::Success)
                    .size(self.label_size),
            )
            .child(
                Label::new(format!("\u{2012}\u{2009}{}", self.removed))
                    .color(Color::Error)
                    .size(self.label_size),
            )
    }
}

impl Component for DiffStat {
    fn scope() -> ComponentScope {
        ComponentScope::VersionControl
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let container = || {
            h_flex()
                .py_4()
                .w_72()
                .justify_center()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().panel_background)
        };

        let diff_stat_example = vec![single_example(
            "Default",
            container()
                .child(DiffStat::new("id", 1, 2))
                .into_any_element(),
        )];

        Some(
            example_group(diff_stat_example)
                .vertical()
                .into_any_element(),
        )
    }
}
