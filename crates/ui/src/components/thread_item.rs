use crate::{
    Chip, DecoratedIcon, DiffStat, IconDecoration, IconDecorationKind, SpinnerLabel, prelude::*,
};
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
    added: Option<usize>,
    removed: Option<usize>,
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
            added: None,
            removed: None,
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

    pub fn added(mut self, added: usize) -> Self {
        self.added = Some(added);
        self
    }

    pub fn removed(mut self, removed: usize) -> Self {
        self.removed = Some(removed);
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
        let agent_icon = Icon::new(self.icon)
            .color(Color::Muted)
            .size(IconSize::Small);

        let icon = if self.generation_done {
            DecoratedIcon::new(
                agent_icon,
                Some(
                    IconDecoration::new(
                        IconDecorationKind::Dot,
                        cx.theme().colors().surface_background,
                        cx,
                    )
                    .color(cx.theme().colors().text_accent)
                    .position(gpui::Point {
                        x: px(-2.),
                        y: px(-2.),
                    }),
                ),
            )
            .into_any_element()
        } else {
            agent_icon.into_any_element()
        };

        let has_no_changes = self.added.is_none() && self.removed.is_none();

        v_flex()
            .id(self.id.clone())
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
                    .child(Label::new(self.title).truncate())
                    .when(self.running, |this| {
                        this.child(icon_container().child(SpinnerLabel::new().color(Color::Accent)))
                    }),
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
                    .when(has_no_changes, |this| {
                        this.child(
                            Label::new("No Changes")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    })
                    .when(self.added.is_some() || self.removed.is_some(), |this| {
                        this.child(DiffStat::new(
                            self.id,
                            self.added.unwrap_or(0),
                            self.removed.unwrap_or(0),
                        ))
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
                "With Changes",
                container()
                    .child(
                        ThreadItem::new("ti-5", "Managing user and project settings interactions")
                            .icon(IconName::AiClaude)
                            .timestamp("7:37 PM")
                            .added(10)
                            .removed(3),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Selected Item",
                container()
                    .child(
                        ThreadItem::new("ti-6", "Refine textarea interaction behavior")
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
