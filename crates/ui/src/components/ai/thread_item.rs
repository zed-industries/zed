use crate::{
    DecoratedIcon, DiffStat, HighlightedLabel, IconDecoration, IconDecorationKind, SpinnerLabel,
    prelude::*,
};

use gpui::{AnyView, ClickEvent, Hsla, SharedString, linear_color_stop, linear_gradient};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentThreadStatus {
    #[default]
    Completed,
    Running,
    WaitingForConfirmation,
    Error,
}

#[derive(IntoElement, RegisterComponent)]
pub struct ThreadItem {
    id: ElementId,
    icon: IconName,
    custom_icon_from_external_svg: Option<SharedString>,
    title: SharedString,
    timestamp: SharedString,
    notified: bool,
    status: AgentThreadStatus,
    selected: bool,
    focused: bool,
    hovered: bool,
    added: Option<usize>,
    removed: Option<usize>,
    worktree: Option<SharedString>,
    highlight_positions: Vec<usize>,
    worktree_highlight_positions: Vec<usize>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_hover: Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>,
    action_slot: Option<AnyElement>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
}

impl ThreadItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: IconName::ZedAgent,
            custom_icon_from_external_svg: None,
            title: title.into(),
            timestamp: "".into(),
            notified: false,
            status: AgentThreadStatus::default(),
            selected: false,
            focused: false,
            hovered: false,
            added: None,
            removed: None,
            worktree: None,
            highlight_positions: Vec::new(),
            worktree_highlight_positions: Vec::new(),
            on_click: None,
            on_hover: Box::new(|_, _, _| {}),
            action_slot: None,
            tooltip: None,
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

    pub fn custom_icon_from_external_svg(mut self, svg: impl Into<SharedString>) -> Self {
        self.custom_icon_from_external_svg = Some(svg.into());
        self
    }

    pub fn notified(mut self, notified: bool) -> Self {
        self.notified = notified;
        self
    }

    pub fn status(mut self, status: AgentThreadStatus) -> Self {
        self.status = status;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
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

    pub fn highlight_positions(mut self, positions: Vec<usize>) -> Self {
        self.highlight_positions = positions;
        self
    }

    pub fn worktree_highlight_positions(mut self, positions: Vec<usize>) -> Self {
        self.worktree_highlight_positions = positions;
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn on_hover(mut self, on_hover: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_hover = Box::new(on_hover);
        self
    }

    pub fn action_slot(mut self, element: impl IntoElement) -> Self {
        self.action_slot = Some(element.into_any_element());
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl RenderOnce for ThreadItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = cx.theme().colors();
        // let dot_separator = || {
        //     Label::new("•")
        //         .size(LabelSize::Small)
        //         .color(Color::Muted)
        //         .alpha(0.5)
        // };

        let icon_container = || h_flex().size_4().flex_none().justify_center();
        let agent_icon = if let Some(custom_svg) = self.custom_icon_from_external_svg {
            Icon::from_external_svg(custom_svg)
                .color(Color::Muted)
                .size(IconSize::Small)
        } else {
            Icon::new(self.icon)
                .color(Color::Muted)
                .size(IconSize::Small)
        };

        let decoration = |icon: IconDecorationKind, color: Hsla| {
            IconDecoration::new(icon, cx.theme().colors().surface_background, cx)
                .color(color)
                .position(gpui::Point {
                    x: px(-2.),
                    y: px(-2.),
                })
        };

        let decoration = if self.status == AgentThreadStatus::WaitingForConfirmation {
            Some(decoration(
                IconDecorationKind::Triangle,
                cx.theme().status().warning,
            ))
        } else if self.status == AgentThreadStatus::Error {
            Some(decoration(IconDecorationKind::X, cx.theme().status().error))
        } else if self.notified {
            Some(decoration(IconDecorationKind::Dot, color.text_accent))
        } else {
            None
        };

        let icon = if let Some(decoration) = decoration {
            icon_container().child(DecoratedIcon::new(agent_icon, Some(decoration)))
        } else {
            icon_container().child(agent_icon)
        };

        let is_running = matches!(
            self.status,
            AgentThreadStatus::Running | AgentThreadStatus::WaitingForConfirmation
        );
        let running_or_action = is_running || (self.hovered && self.action_slot.is_some());

        let title = self.title;
        let highlight_positions = self.highlight_positions;
        let title_label = if highlight_positions.is_empty() {
            Label::new(title).into_any_element()
        } else {
            HighlightedLabel::new(title, highlight_positions).into_any_element()
        };

        let base_bg = if self.selected {
            color.element_active
        } else {
            color.panel_background
        };

        let gradient_overlay = div()
            .absolute()
            .top_0()
            .right(px(-10.0))
            .w_8()
            .h_full()
            .bg(linear_gradient(
                90.,
                linear_color_stop(base_bg, 0.8),
                linear_color_stop(base_bg.opacity(0.0), 0.),
            ))
            .group_hover("thread-item", |s| {
                s.bg(linear_gradient(
                    90.,
                    linear_color_stop(color.element_hover, 0.8),
                    linear_color_stop(color.element_hover.opacity(0.0), 0.),
                ))
            });

        v_flex()
            .id(self.id.clone())
            .group("thread-item")
            .relative()
            .overflow_hidden()
            .cursor_pointer()
            .w_full()
            .map(|this| {
                if self.worktree.is_some() {
                    this.p_2()
                } else {
                    this.px_2().py_1()
                }
            })
            .when(self.selected, |s| s.bg(color.element_active))
            .border_1()
            .border_color(gpui::transparent_black())
            .when(self.focused, |s| s.border_color(color.panel_focused_border))
            .hover(|s| s.bg(color.element_hover))
            .on_hover(self.on_hover)
            .child(
                h_flex()
                    .min_w_0()
                    .w_full()
                    .gap_2()
                    .justify_between()
                    .child(
                        h_flex()
                            .id("content")
                            .min_w_0()
                            .flex_1()
                            .gap_1p5()
                            .child(icon)
                            .child(title_label)
                            .when_some(self.tooltip, |this, tooltip| this.tooltip(tooltip)),
                    )
                    .child(gradient_overlay)
                    .when(running_or_action, |this| {
                        this.child(
                            h_flex()
                                .gap_1()
                                .when(is_running, |this| {
                                    this.child(
                                        icon_container()
                                            .child(SpinnerLabel::new().color(Color::Accent)),
                                    )
                                })
                                .when(self.hovered, |this| {
                                    this.when_some(self.action_slot, |this, slot| this.child(slot))
                                }),
                        )
                    }),
            )
            .when_some(self.worktree, |this, worktree| {
                let worktree_highlight_positions = self.worktree_highlight_positions;
                let worktree_label = if worktree_highlight_positions.is_empty() {
                    Label::new(worktree)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .into_any_element()
                } else {
                    HighlightedLabel::new(worktree, worktree_highlight_positions)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .into_any_element()
                };

                this.child(
                    h_flex()
                        .min_w_0()
                        .gap_1p5()
                        .child(icon_container()) // Icon Spacing
                        .child(worktree_label)
                        // TODO: Uncomment the elements below when we're ready to expose this data
                        // .child(dot_separator())
                        // .child(
                        //     Label::new(self.timestamp)
                        //         .size(LabelSize::Small)
                        //         .color(Color::Muted),
                        // )
                        // .child(
                        //     Label::new("•")
                        //         .size(LabelSize::Small)
                        //         .color(Color::Muted)
                        //         .alpha(0.5),
                        // )
                        // .when(has_no_changes, |this| {
                        //     this.child(
                        //         Label::new("No Changes")
                        //             .size(LabelSize::Small)
                        //             .color(Color::Muted),
                        //     )
                        // })
                        .when(self.added.is_some() || self.removed.is_some(), |this| {
                            this.child(DiffStat::new(
                                self.id,
                                self.added.unwrap_or(0),
                                self.removed.unwrap_or(0),
                            ))
                        }),
                )
            })
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
                "Notified",
                container()
                    .child(
                        ThreadItem::new("ti-2", "Refine thread view scrolling behavior")
                            .timestamp("12:12 AM")
                            .notified(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Waiting for Confirmation",
                container()
                    .child(
                        ThreadItem::new("ti-2b", "Execute shell command in terminal")
                            .timestamp("12:15 AM")
                            .status(AgentThreadStatus::WaitingForConfirmation),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Error",
                container()
                    .child(
                        ThreadItem::new("ti-2c", "Failed to connect to language server")
                            .timestamp("12:20 AM")
                            .status(AgentThreadStatus::Error),
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
                            .status(AgentThreadStatus::Running),
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
            single_example(
                "Focused Item (Keyboard Selection)",
                container()
                    .child(
                        ThreadItem::new("ti-7", "Implement keyboard navigation")
                            .icon(IconName::AiClaude)
                            .timestamp("4:00 PM")
                            .focused(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Selected + Focused",
                container()
                    .child(
                        ThreadItem::new("ti-8", "Active and keyboard-focused thread")
                            .icon(IconName::AiGemini)
                            .timestamp("5:00 PM")
                            .selected(true)
                            .focused(true),
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
