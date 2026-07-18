use crate::Tooltip;
use crate::prelude::*;
use num_format::{Locale, ToFormattedString};

#[derive(IntoElement, RegisterComponent)]
pub struct DiffStat {
    id: ElementId,
    added: usize,
    removed: usize,
    label_size: LabelSize,
    tooltip: Option<SharedString>,
}

impl DiffStat {
    pub fn new(id: impl Into<ElementId>, added: usize, removed: usize) -> Self {
        Self {
            id: id.into(),
            added,
            removed,
            label_size: LabelSize::Small,
            tooltip: None,
        }
    }

    pub fn label_size(mut self, label_size: LabelSize) -> Self {
        self.label_size = label_size;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

impl RenderOnce for DiffStat {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let tooltip = self.tooltip;
        let added = self.added.to_formatted_string(&Locale::en);
        let removed = self.removed.to_formatted_string(&Locale::en);

        h_flex()
            .id(self.id)
            .gap_1()
            .child(
                Label::new(format!("+\u{2009}{added}"))
                    .color(Color::Success)
                    .size(self.label_size),
            )
            .child(
                Label::new(format!("\u{2012}\u{2009}{removed}"))
                    .color(Color::Error)
                    .size(self.label_size),
            )
            .when_some(tooltip, |this, tooltip| {
                this.tooltip(Tooltip::text(tooltip))
            })
    }
}

impl Component for DiffStat {
    fn scope() -> ComponentScope {
        ComponentScope::VersionControl
    }

    fn description() -> &'static str {
        "A compact summary of additions and deletions for a diff, \
        displayed as colored insertion and deletion counts."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
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
                .child(DiffStat::new("id", 1_234, 5_678))
                .into_any_element(),
        )];

        example_group(diff_stat_example)
            .vertical()
            .into_any_element()
    }
}
