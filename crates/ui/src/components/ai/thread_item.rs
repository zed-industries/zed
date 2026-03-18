use crate::{
    CommonAnimationExt, DecoratedIcon, DiffStat, GradientFade, HighlightedLabel, IconDecoration,
    IconDecorationKind, Tooltip, prelude::*,
};

use gpui::{
    Animation, AnimationExt, AnyView, ClickEvent, Hsla, MouseButton, SharedString,
    pulsating_between,
};
use std::time::Duration;

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
    generating_title: bool,
    selected: bool,
    focused: bool,
    hovered: bool,
    docked_right: bool,
    added: Option<usize>,
    removed: Option<usize>,
    worktree: Option<SharedString>,
    highlight_positions: Vec<usize>,
    worktree_highlight_positions: Vec<usize>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_hover: Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>,
    title_label_color: Option<Color>,
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
            generating_title: false,
            selected: false,
            focused: false,
            hovered: false,
            docked_right: false,
            added: None,
            removed: None,
            worktree: None,
            highlight_positions: Vec::new(),
            worktree_highlight_positions: Vec::new(),
            on_click: None,
            on_hover: Box::new(|_, _, _| {}),
            title_label_color: None,
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

    pub fn generating_title(mut self, generating: bool) -> Self {
        self.generating_title = generating;
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

    pub fn docked_right(mut self, docked_right: bool) -> Self {
        self.docked_right = docked_right;
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

    pub fn title_label_color(mut self, color: Color) -> Self {
        self.title_label_color = Some(color);
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
        let dot_separator = || {
            Label::new("•")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .alpha(0.5)
        };

        let icon_id = format!("icon-{}", self.id);
        let icon_container = || {
            h_flex()
                .id(icon_id.clone())
                .size_4()
                .flex_none()
                .justify_center()
        };
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

        let (decoration, icon_tooltip) = if self.status == AgentThreadStatus::Error {
            (
                Some(decoration(IconDecorationKind::X, cx.theme().status().error)),
                Some("Thread has an Error"),
            )
        } else if self.status == AgentThreadStatus::WaitingForConfirmation {
            (
                Some(decoration(
                    IconDecorationKind::Triangle,
                    cx.theme().status().warning,
                )),
                Some("Thread is Waiting for Confirmation"),
            )
        } else if self.notified {
            (
                Some(decoration(IconDecorationKind::Dot, color.text_accent)),
                Some("Thread's Generation is Complete"),
            )
        } else {
            (None, None)
        };

        let icon = if self.status == AgentThreadStatus::Running {
            icon_container()
                .child(
                    Icon::new(IconName::LoadCircle)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .with_rotate_animation(2),
                )
                .into_any_element()
        } else if let Some(decoration) = decoration {
            icon_container()
                .child(DecoratedIcon::new(agent_icon, Some(decoration)))
                .when_some(icon_tooltip, |icon, tooltip| {
                    icon.tooltip(Tooltip::text(tooltip))
                })
                .into_any_element()
        } else {
            icon_container().child(agent_icon).into_any_element()
        };

        let title = self.title;
        let highlight_positions = self.highlight_positions;
        let title_label = if self.generating_title {
            Label::new(title)
                .color(Color::Muted)
                .with_animation(
                    "generating-title",
                    Animation::new(Duration::from_secs(2))
                        .repeat()
                        .with_easing(pulsating_between(0.4, 0.8)),
                    |label, delta| label.alpha(delta),
                )
                .into_any_element()
        } else if highlight_positions.is_empty() {
            let label = Label::new(title);
            let label = if let Some(color) = self.title_label_color {
                label.color(color)
            } else {
                label
            };
            label.into_any_element()
        } else {
            let label = HighlightedLabel::new(title, highlight_positions);
            let label = if let Some(color) = self.title_label_color {
                label.color(color)
            } else {
                label
            };
            label.into_any_element()
        };

        let b_bg = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.8));

        let base_bg = if self.selected {
            color.element_active
        } else {
            b_bg
        };

        let gradient_overlay =
            GradientFade::new(base_bg, color.element_hover, color.element_active)
                .width(px(64.0))
                .right(px(-10.0))
                .gradient_stop(0.75)
                .group_name("thread-item");

        let has_diff_stats = self.added.is_some() || self.removed.is_some();
        let added_count = self.added.unwrap_or(0);
        let removed_count = self.removed.unwrap_or(0);
        let diff_stat_id = self.id.clone();
        let has_worktree = self.worktree.is_some();
        let has_timestamp = !self.timestamp.is_empty();
        let timestamp = self.timestamp;

        v_flex()
            .id(self.id.clone())
            .group("thread-item")
            .relative()
            .overflow_hidden()
            .cursor_pointer()
            .w_full()
            .p_1()
            .when(self.selected, |s| s.bg(color.element_active))
            .border_1()
            .border_color(gpui::transparent_black())
            .when(self.focused, |s| {
                s.when(self.docked_right, |s| s.border_r_2())
                    .border_color(color.border_focused)
            })
            .hover(|s| s.bg(color.element_hover))
            .active(|s| s.bg(color.element_active))
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
                    .when(self.hovered, |this| {
                        this.when_some(self.action_slot, |this, slot| {
                            let overlay = GradientFade::new(
                                base_bg,
                                color.element_hover,
                                color.element_active,
                            )
                            .width(px(64.0))
                            .right(px(6.))
                            .gradient_stop(0.75)
                            .group_name("thread-item");

                            this.child(
                                h_flex()
                                    .relative()
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                                    .child(overlay)
                                    .child(slot),
                            )
                        })
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
                        .when(has_diff_stats || has_timestamp, |this| {
                            this.child(dot_separator())
                        })
                        .when(has_diff_stats, |this| {
                            this.child(
                                DiffStat::new(diff_stat_id.clone(), added_count, removed_count)
                                    .tooltip("Unreviewed changes"),
                            )
                        })
                        .when(has_diff_stats && has_timestamp, |this| {
                            this.child(dot_separator())
                        })
                        .when(has_timestamp, |this| {
                            this.child(
                                Label::new(timestamp.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        }),
                )
            })
            .when(!has_worktree && (has_diff_stats || has_timestamp), |this| {
                this.child(
                    h_flex()
                        .min_w_0()
                        .gap_1p5()
                        .child(icon_container()) // Icon Spacing
                        .when(has_diff_stats, |this| {
                            this.child(
                                DiffStat::new(diff_stat_id, added_count, removed_count)
                                    .tooltip("Unreviewed Changes"),
                            )
                        })
                        .when(has_diff_stats && has_timestamp, |this| {
                            this.child(dot_separator())
                        })
                        .when(has_timestamp, |this| {
                            this.child(
                                Label::new(timestamp.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
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
                "Default (minutes)",
                container()
                    .child(
                        ThreadItem::new("ti-1", "Linking to the Agent Panel Depending on Settings")
                            .icon(IconName::AiOpenAi)
                            .timestamp("15m"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Timestamp Only (hours)",
                container()
                    .child(
                        ThreadItem::new("ti-1b", "Thread with just a timestamp")
                            .icon(IconName::AiClaude)
                            .timestamp("3h"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Notified (weeks)",
                container()
                    .child(
                        ThreadItem::new("ti-2", "Refine thread view scrolling behavior")
                            .timestamp("1w")
                            .notified(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Waiting for Confirmation",
                container()
                    .child(
                        ThreadItem::new("ti-2b", "Execute shell command in terminal")
                            .timestamp("2h")
                            .status(AgentThreadStatus::WaitingForConfirmation),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Error",
                container()
                    .child(
                        ThreadItem::new("ti-2c", "Failed to connect to language server")
                            .timestamp("5h")
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
                            .timestamp("23h")
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
                            .timestamp("2w")
                            .worktree("link-agent-panel"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "With Changes (months)",
                container()
                    .child(
                        ThreadItem::new("ti-5", "Managing user and project settings interactions")
                            .icon(IconName::AiClaude)
                            .timestamp("1mo")
                            .added(10)
                            .removed(3),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Worktree + Changes + Timestamp",
                container()
                    .child(
                        ThreadItem::new("ti-5b", "Full metadata example")
                            .icon(IconName::AiClaude)
                            .worktree("my-project")
                            .added(42)
                            .removed(17)
                            .timestamp("3w"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Selected Item",
                container()
                    .child(
                        ThreadItem::new("ti-6", "Refine textarea interaction behavior")
                            .icon(IconName::AiGemini)
                            .timestamp("45m")
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
                            .timestamp("12h")
                            .focused(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Focused + Docked Right",
                container()
                    .child(
                        ThreadItem::new("ti-7b", "Focused with right dock border")
                            .icon(IconName::AiClaude)
                            .timestamp("1w")
                            .focused(true)
                            .docked_right(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Selected + Focused",
                container()
                    .child(
                        ThreadItem::new("ti-8", "Active and keyboard-focused thread")
                            .icon(IconName::AiGemini)
                            .timestamp("2mo")
                            .selected(true)
                            .focused(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Hovered with Action Slot",
                container()
                    .child(
                        ThreadItem::new("ti-9", "Hover to see action button")
                            .icon(IconName::AiClaude)
                            .timestamp("6h")
                            .hovered(true)
                            .action_slot(
                                IconButton::new("delete", IconName::Trash)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted),
                            ),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Search Highlight",
                container()
                    .child(
                        ThreadItem::new("ti-10", "Implement keyboard navigation")
                            .icon(IconName::AiClaude)
                            .timestamp("4w")
                            .highlight_positions(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Worktree Search Highlight",
                container()
                    .child(
                        ThreadItem::new("ti-11", "Search in worktree name")
                            .icon(IconName::AiClaude)
                            .timestamp("3mo")
                            .worktree("my-project-name")
                            .worktree_highlight_positions(vec![3, 4, 5, 6, 7, 8, 9, 10, 11]),
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
