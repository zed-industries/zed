use crate::{CommonAnimationExt, DiffStat, GradientFade, HighlightedLabel, Tooltip, prelude::*};

use gpui::{
    Animation, AnimationExt, AnyView, ClickEvent, Hsla, MouseButton, SharedString,
    pulsating_between,
};
use itertools::Itertools as _;
use std::{path::PathBuf, sync::Arc, time::Duration};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AgentThreadStatus {
    #[default]
    Completed,
    Running,
    WaitingForConfirmation,
    Error,
}

#[derive(Clone)]
pub struct ThreadItemWorktreeInfo {
    pub name: SharedString,
    pub full_path: SharedString,
    pub highlight_positions: Vec<usize>,
}

#[derive(IntoElement, RegisterComponent)]
pub struct ThreadItem {
    id: ElementId,
    icon: IconName,
    icon_color: Option<Color>,
    icon_visible: bool,
    custom_icon_from_external_svg: Option<SharedString>,
    title: SharedString,
    title_label_color: Option<Color>,
    title_generating: bool,
    highlight_positions: Vec<usize>,
    timestamp: SharedString,
    notified: bool,
    status: AgentThreadStatus,
    selected: bool,
    focused: bool,
    hovered: bool,
    rounded: bool,
    added: Option<usize>,
    removed: Option<usize>,
    project_paths: Option<Arc<[PathBuf]>>,
    project_name: Option<SharedString>,
    worktrees: Vec<ThreadItemWorktreeInfo>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_hover: Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>,
    action_slot: Option<AnyElement>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    base_bg: Option<Hsla>,
}

impl ThreadItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: IconName::ZedAgent,
            icon_color: None,
            icon_visible: true,
            custom_icon_from_external_svg: None,
            title: title.into(),
            title_label_color: None,
            title_generating: false,
            highlight_positions: Vec::new(),
            timestamp: "".into(),
            notified: false,
            status: AgentThreadStatus::default(),
            selected: false,
            focused: false,
            hovered: false,
            rounded: false,
            added: None,
            removed: None,

            project_paths: None,
            project_name: None,
            worktrees: Vec::new(),
            on_click: None,
            on_hover: Box::new(|_, _, _| {}),
            action_slot: None,
            tooltip: None,
            base_bg: None,
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

    pub fn icon_color(mut self, color: Color) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn icon_visible(mut self, visible: bool) -> Self {
        self.icon_visible = visible;
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

    pub fn title_generating(mut self, generating: bool) -> Self {
        self.title_generating = generating;
        self
    }

    pub fn title_label_color(mut self, color: Color) -> Self {
        self.title_label_color = Some(color);
        self
    }

    pub fn highlight_positions(mut self, positions: Vec<usize>) -> Self {
        self.highlight_positions = positions;
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

    pub fn project_paths(mut self, paths: Arc<[PathBuf]>) -> Self {
        self.project_paths = Some(paths);
        self
    }

    pub fn project_name(mut self, name: impl Into<SharedString>) -> Self {
        self.project_name = Some(name.into());
        self
    }

    pub fn worktrees(mut self, worktrees: Vec<ThreadItemWorktreeInfo>) -> Self {
        self.worktrees = worktrees;
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
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

    pub fn base_bg(mut self, color: Hsla) -> Self {
        self.base_bg = Some(color);
        self
    }
}

impl RenderOnce for ThreadItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = cx.theme().colors();
        let sidebar_base_bg = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.25));

        let raw_bg = self.base_bg.unwrap_or(sidebar_base_bg);
        let apparent_bg = color.background.blend(raw_bg);

        let base_bg = if self.selected {
            apparent_bg.blend(color.element_active)
        } else {
            apparent_bg
        };

        let hover_color = color
            .element_active
            .blend(color.element_background.opacity(0.2));
        let hover_bg = apparent_bg.blend(hover_color);

        let gradient_overlay = GradientFade::new(base_bg, hover_bg, hover_bg)
            .width(px(64.0))
            .right(px(-10.0))
            .gradient_stop(0.75)
            .group_name("thread-item");

        let dot_separator = || {
            Label::new("•")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .alpha(0.5)
        };

        let icon_id = format!("icon-{}", self.id);
        let icon_visible = self.icon_visible;
        let icon_container = || {
            h_flex()
                .id(icon_id.clone())
                .size_4()
                .flex_none()
                .justify_center()
                .when(!icon_visible, |this| this.invisible())
        };
        let icon_color = self.icon_color.unwrap_or(Color::Muted);
        let agent_icon = if let Some(custom_svg) = self.custom_icon_from_external_svg {
            Icon::from_external_svg(custom_svg)
                .color(icon_color)
                .size(IconSize::Small)
        } else {
            Icon::new(self.icon).color(icon_color).size(IconSize::Small)
        };

        let (status_icon, icon_tooltip) = if self.status == AgentThreadStatus::Error {
            (
                Some(
                    Icon::new(IconName::Close)
                        .size(IconSize::Small)
                        .color(Color::Error),
                ),
                Some("Thread has an Error"),
            )
        } else if self.status == AgentThreadStatus::WaitingForConfirmation {
            (
                Some(
                    Icon::new(IconName::Warning)
                        .size(IconSize::XSmall)
                        .color(Color::Warning),
                ),
                Some("Thread is Waiting for Confirmation"),
            )
        } else if self.notified {
            (
                Some(
                    Icon::new(IconName::Circle)
                        .size(IconSize::Small)
                        .color(Color::Accent),
                ),
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
        } else if let Some(status_icon) = status_icon {
            icon_container()
                .child(status_icon)
                .when_some(icon_tooltip, |icon, tooltip| {
                    icon.tooltip(Tooltip::text(tooltip))
                })
                .into_any_element()
        } else {
            icon_container().child(agent_icon).into_any_element()
        };

        let title = self.title;
        let highlight_positions = self.highlight_positions;

        let title_label = if self.title_generating {
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
            Label::new(title)
                .when_some(self.title_label_color, |label, color| label.color(color))
                .into_any_element()
        } else {
            HighlightedLabel::new(title, highlight_positions)
                .when_some(self.title_label_color, |label, color| label.color(color))
                .into_any_element()
        };

        let has_diff_stats = self.added.is_some() || self.removed.is_some();
        let diff_stat_id = self.id.clone();
        let added_count = self.added.unwrap_or(0);
        let removed_count = self.removed.unwrap_or(0);

        let project_paths = self.project_paths.as_ref().and_then(|paths| {
            let paths_str = paths
                .as_ref()
                .iter()
                .filter_map(|p| p.file_name())
                .filter_map(|name| name.to_str())
                .join(", ");
            if paths_str.is_empty() {
                None
            } else {
                Some(paths_str)
            }
        });

        let has_project_name = self.project_name.is_some();
        let has_project_paths = project_paths.is_some();
        let has_worktree = !self.worktrees.is_empty();
        let has_timestamp = !self.timestamp.is_empty();
        let timestamp = self.timestamp;

        v_flex()
            .id(self.id.clone())
            .cursor_pointer()
            .group("thread-item")
            .relative()
            .overflow_hidden()
            .w_full()
            .py_1()
            .px_1p5()
            .when(self.selected, |s| s.bg(color.element_active))
            .border_1()
            .border_color(gpui::transparent_black())
            .when(self.focused, |s| s.border_color(color.border_focused))
            .when(self.rounded, |s| s.rounded_sm())
            .hover(|s| s.bg(hover_color))
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
                            let overlay = GradientFade::new(base_bg, hover_bg, hover_bg)
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
            .when(
                has_project_name
                    || has_project_paths
                    || has_worktree
                    || has_diff_stats
                    || has_timestamp,
                |this| {
                    // Collect all full paths for the shared tooltip.
                    let worktree_tooltip: SharedString = self
                        .worktrees
                        .iter()
                        .map(|wt| wt.full_path.as_ref())
                        .collect::<Vec<_>>()
                        .join("\n")
                        .into();

                    let worktree_tooltip_title = if self.worktrees.len() > 1 {
                        "Thread Running in Local Git Worktrees"
                    } else {
                        "Thread Running in a Local Git Worktree"
                    };

                    // Deduplicate chips by name — e.g. two paths both named
                    // "olivetti" produce a single chip. Highlight positions
                    // come from the first occurrence.
                    let mut seen_names: Vec<SharedString> = Vec::new();
                    let mut worktree_labels: Vec<AnyElement> = Vec::new();

                    for wt in self.worktrees {
                        if seen_names.contains(&wt.name) {
                            continue;
                        }

                        let chip_index = seen_names.len();
                        seen_names.push(wt.name.clone());

                        let label = if wt.highlight_positions.is_empty() {
                            Label::new(wt.name)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .into_any_element()
                        } else {
                            HighlightedLabel::new(wt.name, wt.highlight_positions)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .into_any_element()
                        };
                        let tooltip_title = worktree_tooltip_title;
                        let tooltip_meta = worktree_tooltip.clone();

                        worktree_labels.push(
                            h_flex()
                                .id(format!("{}-worktree-{chip_index}", self.id.clone()))
                                .gap_0p5()
                                .child(
                                    Icon::new(IconName::GitWorktree)
                                        .size(IconSize::XSmall)
                                        .color(Color::Muted),
                                )
                                .child(label)
                                .tooltip(move |_, cx| {
                                    Tooltip::with_meta(
                                        tooltip_title,
                                        None,
                                        tooltip_meta.clone(),
                                        cx,
                                    )
                                })
                                .into_any_element(),
                        );
                    }

                    this.child(
                        h_flex()
                            .min_w_0()
                            .gap_1p5()
                            .child(icon_container()) // Icon Spacing
                            .when_some(self.project_name, |this, name| {
                                this.child(
                                    Label::new(name).size(LabelSize::Small).color(Color::Muted),
                                )
                            })
                            .when(
                                has_project_name && (has_project_paths || has_worktree),
                                |this| this.child(dot_separator()),
                            )
                            .when_some(project_paths, |this, paths| {
                                this.child(
                                    Label::new(paths)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .into_any_element(),
                                )
                            })
                            .when(has_project_paths && has_worktree, |this| {
                                this.child(dot_separator())
                            })
                            .children(worktree_labels)
                            .when(
                                (has_project_name || has_project_paths || has_worktree)
                                    && (has_diff_stats || has_timestamp),
                                |this| this.child(dot_separator()),
                            )
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
                },
            )
            .when_some(self.on_click, |this, on_click| this.on_click(on_click))
    }
}

impl Component for ThreadItem {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let color = cx.theme().colors();
        let bg = color
            .title_bar_background
            .blend(color.panel_background.opacity(0.25));

        let container = || {
            v_flex()
                .w_72()
                .border_1()
                .border_color(color.border_variant)
                .bg(bg)
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
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                name: "link-agent-panel".into(),
                                full_path: "link-agent-panel".into(),
                                highlight_positions: Vec::new(),
                            }]),
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
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                name: "my-project".into(),
                                full_path: "my-project".into(),
                                highlight_positions: Vec::new(),
                            }])
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
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                name: "my-project-name".into(),
                                full_path: "my-project-name".into(),
                                highlight_positions: vec![3, 4, 5, 6, 7, 8, 9, 10, 11],
                            }]),
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
