use crate::{Chip, Indicator, SpinnerLabel, prelude::*};
use gpui::{ClickEvent, SharedString};

#[derive(IntoElement, RegisterComponent)]
pub struct ThreadItem {
    id: ElementId,
    icon: IconName,
    title: SharedString,
    timestamp: SharedString,
    running: bool,
    generation_done: bool,
    selected: bool,
    has_changes: bool,
    worktree: Option<SharedString>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl ThreadItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: IconName::ZedAgent,
            title: title.into(),
            timestamp: "".into(),
            running: false,
            generation_done: false,
            selected: false,
            has_changes: false,
            worktree: None,
            on_click: None,
        }
    }

    pub fn timestamp(mut self, timestamp: impl Into<SharedString>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = icon;
        self
    }

    pub fn running(mut self, running: bool) -> Self {
        self.running = running;
        self
    }

    pub fn generation_done(mut self, generation_done: bool) -> Self {
        self.generation_done = generation_done;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn has_changes(mut self, has_changes: bool) -> Self {
        self.has_changes = has_changes;
        self
    }

    pub fn worktree(mut self, worktree: impl Into<SharedString>) -> Self {
        self.worktree = Some(worktree.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ThreadItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let icon_container = || h_flex().size_4().justify_center();
        let icon = if self.generation_done {
            icon_container().child(Indicator::dot().color(Color::Accent))
        } else if self.running {
            icon_container().child(SpinnerLabel::new().color(Color::Accent))
        } else {
            icon_container().child(
                Icon::new(self.icon)
                    .color(Color::Muted)
                    .size(IconSize::Small),
            )
        };

        v_flex()
            .id(self.id)
            .cursor_pointer()
            .p_2()
            .when(self.selected, |this| {
                this.bg(cx.theme().colors().element_active)
            })
            .hover(|s| s.bg(cx.theme().colors().element_hover))
            .child(
                h_flex()
                    .w_full()
                    .gap_1p5()
                    .child(icon)
                    .child(Label::new(self.title).truncate()),
            )
            .child(
                h_flex()
                    .gap_1p5()
                    .child(icon_container()) // Icon Spacing
                    .when_some(self.worktree, |this, name| {
                        this.child(Chip::new(name).label_size(LabelSize::XSmall))
                    })
                    .child(
                        Label::new(self.timestamp)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new("•")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .alpha(0.5),
                    )
                    .when(!self.has_changes, |this| {
                        this.child(
                            Label::new("No Changes")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
            .when_some(self.on_click, |this, on_click| this.on_click(on_click))
    }
}

impl Component for ThreadItem {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let container = || {
            v_flex()
                .w_72()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().panel_background)
        };

        let thread_item_examples = vec![
            single_example(
                "Default",
                container()
                    .child(
                        ThreadItem::new("ti-1", "Linking to the Agent Panel Depending on Settings")
                            .icon(IconName::AiOpenAi)
                            .timestamp("1:33 AM"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Generation Done",
                container()
                    .child(
                        ThreadItem::new("ti-2", "Refine thread view scrolling behavior")
                            .timestamp("12:12 AM")
                            .generation_done(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Running Agent",
                container()
                    .child(
                        ThreadItem::new("ti-3", "Add line numbers option to FileEditBlock")
                            .icon(IconName::AiClaude)
                            .timestamp("7:30 PM")
                            .running(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "In Worktree",
                container()
                    .child(
                        ThreadItem::new("ti-4", "Add line numbers option to FileEditBlock")
                            .icon(IconName::AiClaude)
                            .timestamp("7:37 PM")
                            .worktree("link-agent-panel"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Selected Item",
                container()
                    .child(
                        ThreadItem::new("ti-5", "Refine textarea interaction behavior")
                            .icon(IconName::AiGemini)
                            .timestamp("3:00 PM")
                            .selected(true),
                    )
                    .into_any_element(),
            ),
        ];

        Some(
            example_group(thread_item_examples)
                .vertical()
                .into_any_element(),
        )
    }
}
