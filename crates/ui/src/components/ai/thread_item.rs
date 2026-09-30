use crate::{CommonAnimationExt, DiffStat, GradientFade, HighlightedLabel, Tooltip, prelude::*};

use gpui::{
    Animation, AnimationExt, ClickEvent, Hsla, MouseButton, SharedString,
    WindowBackgroundAppearance, pulsating_between,
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WorktreeKind {
    #[default]
    Main,
    Linked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorktreeHead {
    Branch(SharedString),
    Detached,
}

#[derive(Clone, Default)]
pub struct ThreadItemWorktreeInfo {
    pub worktree_name: Option<SharedString>,
    /// `None` when the head state is unknown, e.g. for a closed project.
    pub head: Option<WorktreeHead>,
    pub full_path: SharedString,
    pub highlight_positions: Vec<usize>,
    pub kind: WorktreeKind,
}

impl ThreadItemWorktreeInfo {
    fn display_labels(
        &self,
        show_main_worktree_name: bool,
    ) -> Option<(Option<SharedString>, Option<SharedString>)> {
        match self.kind {
            WorktreeKind::Main => match &self.head {
                Some(WorktreeHead::Branch(branch_name)) => Some((
                    if show_main_worktree_name {
                        self.worktree_name.clone()
                    } else {
                        None
                    },
                    Some(branch_name.clone()),
                )),
                _ => None,
            },
            WorktreeKind::Linked => {
                let head_label = self.head.as_ref().map(|head| match head {
                    WorktreeHead::Branch(branch_name) => branch_name.clone(),
                    WorktreeHead::Detached => "Detached HEAD".into(),
                });

                (self.worktree_name.is_some() || head_label.is_some())
                    .then(|| (self.worktree_name.clone(), head_label))
            }
        }
    }
}

#[derive(IntoElement, RegisterComponent)]
pub struct ThreadItem {
    id: ElementId,
    icon: IconName,
    icon_char: Option<SharedString>,
    icon_color: Option<Color>,
    icon_visible: bool,
    custom_icon_from_external_svg: Option<SharedString>,
    title: SharedString,
    title_slot: Option<AnyElement>,
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
    is_truncated: bool,
    added: Option<usize>,
    removed: Option<usize>,
    project_paths: Option<Arc<[PathBuf]>>,
    project_name: Option<SharedString>,
    worktrees: Vec<ThreadItemWorktreeInfo>,
    is_remote: bool,
    archived: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_hover: Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>,
    action_slot: Option<AnyElement>,
    base_bg: Option<Hsla>,
}

impl ThreadItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: IconName::ZedAgent,
            icon_char: None,
            icon_color: None,
            icon_visible: true,
            custom_icon_from_external_svg: None,
            title: title.into(),
            title_slot: None,
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
            is_truncated: true,
            added: None,
            removed: None,
            project_paths: None,
            project_name: None,
            worktrees: Vec::new(),
            is_remote: false,
            archived: false,
            on_click: None,
            on_hover: Box::new(|_, _, _| {}),
            action_slot: None,
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

    /// Renders the given character in place of the icon. Takes precedence over
    /// [`Self::icon`] and [`Self::custom_icon_from_external_svg`].
    pub fn icon_char(mut self, icon_char: impl Into<SharedString>) -> Self {
        self.icon_char = Some(icon_char.into());
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

    pub fn title_slot(mut self, element: impl IntoElement) -> Self {
        self.title_slot = Some(element.into_any_element());
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

    pub fn is_remote(mut self, is_remote: bool) -> Self {
        self.is_remote = is_remote;
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = archived;
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

    pub fn is_truncated(mut self, is_truncated: bool) -> Self {
        self.is_truncated = is_truncated;
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

    pub fn base_bg(mut self, color: Hsla) -> Self {
        self.base_bg = Some(color);
        self
    }
}

impl RenderOnce for ThreadItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = cx.theme().colors();
        let raw_bg = self.base_bg.unwrap_or(color.surface_background);
        let show_main_worktree_name = self.worktrees.len() > 1;
        // The fade gradient paints a solid color over the title to blend it into
        // the row background, but a transparent window has no opaque surface to
        // fade into, so it renders as a visible patch; truncate the title instead.
        let opaque_window = cx.theme().window_background_appearance()
            == WindowBackgroundAppearance::Opaque
            && raw_bg.a >= 1.0;
        let apparent_bg = color.background.blend(raw_bg);

        let base_bg = if self.selected {
            apparent_bg.blend(color.ghost_element_selected)
        } else {
            apparent_bg
        };

        let hover_bg = apparent_bg.blend(color.ghost_element_hover);
        let active_bg = apparent_bg.blend(color.ghost_element_active);

        let gradient_overlay = GradientFade::new(base_bg, hover_bg, active_bg)
            .width(px(64.0))
            .right(px(-10.0))
            .gradient_stop(0.7)
            .group_name("thread-item");

        let separator_color = Color::Custom(color.text_muted.opacity(0.4));
        let dot_separator = || {
            Label::new("•")
                .size(LabelSize::Small)
                .color(separator_color)
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
        let agent_icon = if let Some(icon_char) = self.icon_char {
            Label::new(icon_char)
                .size(LabelSize::Small)
                .color(icon_color)
                .into_any_element()
        } else if let Some(custom_svg) = self.custom_icon_from_external_svg {
            Icon::from_external_svg(custom_svg)
                .color(icon_color)
                .size(IconSize::Small)
                .into_any_element()
        } else {
            Icon::new(self.icon)
                .color(icon_color)
                .size(IconSize::Small)
                .into_any_element()
        };

        let status_icon = if self.status == AgentThreadStatus::Error {
            Some(
                Icon::new(IconName::Close)
                    .size(IconSize::Small)
                    .color(Color::Error),
            )
        } else if self.status == AgentThreadStatus::WaitingForConfirmation {
            Some(
                Icon::new(IconName::Warning)
                    .size(IconSize::XSmall)
                    .color(Color::Warning),
            )
        } else if self.notified {
            Some(
                Icon::new(IconName::Circle)
                    .size(IconSize::Small)
                    .color(Color::Accent),
            )
        } else {
            None
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
            icon_container().child(status_icon).into_any_element()
        } else {
            icon_container().child(agent_icon).into_any_element()
        };

        let title = self.title;
        let highlight_positions = self.highlight_positions;

        let title_label = if let Some(title_slot) = self.title_slot {
            title_slot
        } else if self.title_generating {
            Label::new(title)
                .color(Color::Muted)
                .when(!opaque_window, |label| label.truncate())
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
                .when(!opaque_window, |label| label.truncate())
                .into_any_element()
        } else {
            HighlightedLabel::new(title, highlight_positions)
                .when_some(self.title_label_color, |label, color| label.color(color))
                .when(!opaque_window, |label| label.truncate())
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
        let has_timestamp = !self.timestamp.is_empty();
        let timestamp = self.timestamp;

        let show_tooltip = matches!(
            self.status,
            AgentThreadStatus::Error | AgentThreadStatus::WaitingForConfirmation
        );

        let worktrees: Vec<_> = self
            .worktrees
            .into_iter()
            .filter_map(|worktree| {
                let (worktree_name, branch_name) =
                    worktree.display_labels(show_main_worktree_name)?;
                Some((worktree, worktree_name, branch_name))
            })
            .collect();

        let has_worktree = !worktrees.is_empty();

        let has_metadata = has_project_name
            || has_project_paths
            || has_worktree
            || has_diff_stats
            || has_timestamp;

        v_flex()
            .id(self.id.clone())
            .cursor_pointer()
            .group("thread-item")
            .relative()
            .flex_shrink_0()
            .overflow_hidden()
            .w_full()
            .py_1()
            .px_1p5()
            .when(self.selected, |s| s.bg(color.ghost_element_selected))
            .border_1()
            .border_r_2()
            .border_color(gpui::transparent_black())
            .when(self.focused, |s| s.border_color(color.panel_focused_border))
            .when(self.rounded, |s| s.rounded_sm())
            .hover(|s| s.bg(color.ghost_element_hover))
            .active(|s| s.bg(color.ghost_element_active))
            .on_hover(self.on_hover)
            .child(
                h_flex()
                    .min_w_0()
                    .w_full()
                    .h_6()
                    .gap_2()
                    .justify_between()
                    .child(
                        h_flex()
                            .id("content")
                            .min_w_0()
                            .flex_1()
                            .gap_1p5()
                            .child(icon)
                            .child(title_label),
                    )
                    .when(self.is_truncated && opaque_window, |this| {
                        this.child(gradient_overlay)
                    })
                    .when(self.hovered, |this| {
                        this.when_some(self.action_slot, |this, slot| {
                            this.child(
                                h_flex()
                                    .relative()
                                    .when(opaque_window, |this| {
                                        this.child(
                                            GradientFade::new(base_bg, hover_bg, active_bg)
                                                .width(px(120.0))
                                                .right(px(8.))
                                                .gradient_stop(0.90)
                                                .group_name("thread-item"),
                                        )
                                    })
                                    .child(
                                        h_flex()
                                            .pr_1p5()
                                            .child(slot)
                                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                                cx.stop_propagation()
                                            }),
                                    ),
                            )
                        })
                    }),
            )
            .when(has_metadata, |this| {
                this.child(
                    h_flex()
                        .gap_1p5()
                        .child(icon_container()) // Icon Spacing
                        .child(
                            h_flex()
                                .min_w_0()
                                .gap_1()
                                .when(self.archived, |this| {
                                    this.child(
                                        Icon::new(IconName::Archive).size(IconSize::XSmall).color(
                                            Color::Custom(
                                                cx.theme().colors().icon_muted.opacity(0.5),
                                            ),
                                        ),
                                    )
                                })
                                .when(
                                    has_project_name || has_project_paths || has_worktree,
                                    |this| {
                                        this.when_some(self.project_name, |this, name| {
                                            this.child(
                                                Label::new(name)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
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
                                                    .color(Color::Muted),
                                            )
                                        })
                                        .when(has_project_paths && has_worktree, |this| {
                                            this.child(dot_separator())
                                        })
                                        .children(
                                            worktrees.into_iter().enumerate().map(
                                                |(
                                                    index,
                                                    (worktree, worktree_name, branch_name),
                                                )| {
                                                    let has_real_branch = matches!(
                                                        worktree.head,
                                                        Some(WorktreeHead::Branch(_))
                                                    );
                                                    let worktree_label =
                                                        worktree_name.map(|name| {
                                                            if worktree
                                                                .highlight_positions
                                                                .is_empty()
                                                            {
                                                                Label::new(name)
                                                                    .size(LabelSize::Small)
                                                                    .color(Color::Muted)
                                                                    .truncate()
                                                                    .into_any_element()
                                                            } else {
                                                                HighlightedLabel::new(
                                                                    name,
                                                                    worktree
                                                                        .highlight_positions
                                                                        .clone(),
                                                                )
                                                                .size(LabelSize::Small)
                                                                .color(Color::Muted)
                                                                .truncate()
                                                                .into_any_element()
                                                            }
                                                        });

                                                    let branch_label = branch_name.map(|branch| {
                                                        Label::new(branch)
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted)
                                                            .truncate()
                                                            .into_any_element()
                                                    });

                                                    let show_separator = worktree_label.is_some()
                                                        && branch_label.is_some();
                                                    // The git icons' SVGs have built-in side
                                                    // padding, which makes the gap after a
                                                    // preceding `•` look wider than the one
                                                    // before it.
                                                    let offset_leading_icon = index == 0
                                                        && (has_project_name || has_project_paths)
                                                        && (worktree_label.is_some()
                                                            || has_real_branch);

                                                    h_flex()
                                                        .min_w_0()
                                                        .gap_1()
                                                        .when(offset_leading_icon, |this| {
                                                            this.ml_neg_0p5()
                                                        })
                                                        .when_some(worktree_label, |this, label| {
                                                            this.child(
                                                                h_flex()
                                                                    .min_w_0()
                                                                    .gap_0p5()
                                                                    .child(
                                                                        Icon::new(
                                                                            IconName::GitWorktree,
                                                                        )
                                                                        .size(IconSize::XSmall)
                                                                        .color(Color::Muted),
                                                                    )
                                                                    .child(label),
                                                            )
                                                        })
                                                        .when(show_separator, |this| {
                                                            this.child(
                                                                dot_separator().flex_shrink_0(),
                                                            )
                                                        })
                                                        .when_some(branch_label, |this, label| {
                                                            this.child(
                                                                h_flex()
                                                                    .min_w_0()
                                                                    .gap_0p5()
                                                                    .when(has_real_branch, |this| {
                                                                        this.child(
                                                                            Icon::new(
                                                                                IconName::GitBranch,
                                                                            )
                                                                            .size(IconSize::XSmall)
                                                                            .color(Color::Muted),
                                                                        )
                                                                    })
                                                                    .child(label),
                                                            )
                                                        })
                                                },
                                            ),
                                        )
                                    },
                                )
                                .when(
                                    (has_project_name || has_project_paths || has_worktree)
                                        && (has_diff_stats || has_timestamp),
                                    |this| this.child(dot_separator()),
                                )
                                .when(has_diff_stats, |this| {
                                    this.child(DiffStat::new(
                                        diff_stat_id,
                                        added_count,
                                        removed_count,
                                    ))
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
                        ),
                )
            })
            .when(show_tooltip, |this| {
                let status = self.status;
                this.tooltip(Tooltip::element(move |_, _| match status {
                    AgentThreadStatus::Error => h_flex()
                        .gap_1()
                        .child(
                            Icon::new(IconName::Close)
                                .size(IconSize::Small)
                                .color(Color::Error),
                        )
                        .child(Label::new("Thread has an Error"))
                        .into_any_element(),
                    AgentThreadStatus::WaitingForConfirmation => h_flex()
                        .gap_1()
                        .child(
                            Icon::new(IconName::Warning)
                                .size(IconSize::Small)
                                .color(Color::Warning),
                        )
                        .child(Label::new("Waiting for Confirmation"))
                        .into_any_element(),
                    _ => gpui::Empty.into_any_element(),
                }))
            })
            .when_some(self.on_click, |this, on_click| this.on_click(on_click))
    }
}

impl Component for ThreadItem {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn description() -> &'static str {
        "A row representing an agent thread in a list, showing its title, status, \
        timestamp, and contextual metadata such as worktree and branch information."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let color = cx.theme().colors();
        let bg = color.surface_background;

        let container = || {
            v_flex()
                .w_72()
                .border_1()
                .border_color(color.border_variant)
                .bg(bg)
        };

        let thread_item_examples = vec![
            single_example(
                "Default",
                container()
                    .child(
                        ThreadItem::new("ti-1", "Linking to the Agent Panel Depending on Settings")
                            .icon(IconName::AiOpenAi)
                            .timestamp("15m"),
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
                                worktree_name: Some("link-agent-panel".into()),
                                full_path: "link-agent-panel".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Linked,
                                head: None,
                            }]),
                    )
                    .into_any_element(),
            ),
            single_example(
                "With Changes",
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
                                worktree_name: Some("my-project".into()),
                                full_path: "my-project".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Linked,
                                head: None,
                            }])
                            .added(42)
                            .removed(17)
                            .timestamp("3w"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Worktree + Branch + Changes + Timestamp",
                container()
                    .child(
                        ThreadItem::new("ti-5c", "Full metadata with branch")
                            .icon(IconName::AiClaude)
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                worktree_name: Some("my-project".into()),
                                full_path: "/worktrees/my-project/zed".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Linked,
                                head: Some(WorktreeHead::Branch("feature-branch".into())),
                            }])
                            .added(42)
                            .removed(17)
                            .timestamp("3w"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Long Branch + Changes (truncation)",
                container()
                    .child(
                        ThreadItem::new("ti-5d", "Metadata overflow with long branch name")
                            .icon(IconName::AiClaude)
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                worktree_name: Some("my-project".into()),
                                full_path: "/worktrees/my-project/zed".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Linked,
                                head: Some(WorktreeHead::Branch(
                                    "fix-very-long-branch-name-here".into(),
                                )),
                            }])
                            .added(108)
                            .removed(53)
                            .timestamp("2d"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Main Worktree Branch + Changes + Timestamp",
                container()
                    .child(
                        ThreadItem::new("ti-5e", "Main worktree branch with diff stats")
                            .icon(IconName::ZedAgent)
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                worktree_name: Some("zed".into()),
                                full_path: "/projects/zed".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Main,
                                head: Some(WorktreeHead::Branch("sidebar-show-branch-name".into())),
                            }])
                            .added(23)
                            .removed(8)
                            .timestamp("5m"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Long Worktree Name (truncation)",
                container()
                    .child(
                        ThreadItem::new("ti-5f", "Thread with a very long worktree name")
                            .icon(IconName::AiClaude)
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                worktree_name: Some(
                                    "very-long-worktree-name-that-should-truncate".into(),
                                ),
                                full_path: "/worktrees/very-long-worktree-name/zed".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Linked,
                                head: None,
                            }])
                            .timestamp("1h"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Worktree with Search Highlights",
                container()
                    .child(
                        ThreadItem::new("ti-5g", "Filtered thread with highlighted worktree")
                            .icon(IconName::AiClaude)
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                worktree_name: Some("jade-glen".into()),
                                full_path: "/worktrees/jade-glen/zed".into(),
                                highlight_positions: vec![0, 1, 2, 3],
                                kind: WorktreeKind::Linked,
                                head: Some(WorktreeHead::Branch("fix-scrolling".into())),
                            }])
                            .timestamp("3d"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Multiple Worktrees (no branches)",
                container()
                    .child(
                        ThreadItem::new("ti-5h", "Thread spanning multiple worktrees")
                            .icon(IconName::AiClaude)
                            .worktrees(vec![
                                ThreadItemWorktreeInfo {
                                    worktree_name: Some("jade-glen".into()),
                                    full_path: "/worktrees/jade-glen/zed".into(),
                                    highlight_positions: Vec::new(),
                                    kind: WorktreeKind::Linked,
                                    head: None,
                                },
                                ThreadItemWorktreeInfo {
                                    worktree_name: Some("fawn-otter".into()),
                                    full_path: "/worktrees/fawn-otter/zed-slides".into(),
                                    highlight_positions: Vec::new(),
                                    kind: WorktreeKind::Linked,
                                    head: None,
                                },
                            ])
                            .timestamp("2h"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Multiple Worktrees with Branches",
                container()
                    .child(
                        ThreadItem::new("ti-5i", "Multi-root with per-worktree branches")
                            .icon(IconName::ZedAgent)
                            .worktrees(vec![
                                ThreadItemWorktreeInfo {
                                    worktree_name: Some("jade-glen".into()),
                                    full_path: "/worktrees/jade-glen/zed".into(),
                                    highlight_positions: Vec::new(),
                                    kind: WorktreeKind::Linked,
                                    head: Some(WorktreeHead::Branch("fix".into())),
                                },
                                ThreadItemWorktreeInfo {
                                    worktree_name: Some("fawn-otter".into()),
                                    full_path: "/worktrees/fawn-otter/zed-slides".into(),
                                    highlight_positions: Vec::new(),
                                    kind: WorktreeKind::Linked,
                                    head: Some(WorktreeHead::Branch("main".into())),
                                },
                            ])
                            .timestamp("15m"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Project Name + Worktree + Branch",
                container()
                    .child(
                        ThreadItem::new("ti-5j", "Thread with project context")
                            .icon(IconName::AiClaude)
                            .project_name("my-remote-server")
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                worktree_name: Some("jade-glen".into()),
                                full_path: "/worktrees/jade-glen/zed".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Linked,
                                head: Some(WorktreeHead::Branch("feature-branch".into())),
                            }])
                            .timestamp("1d"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Project Paths + Worktree (archive view)",
                container()
                    .child(
                        ThreadItem::new("ti-5k", "Archived thread with folder paths")
                            .icon(IconName::AiClaude)
                            .project_paths(Arc::from(vec![
                                PathBuf::from("/projects/zed"),
                                PathBuf::from("/projects/zed-slides"),
                            ]))
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                worktree_name: Some("jade-glen".into()),
                                full_path: "/worktrees/jade-glen/zed".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Linked,
                                head: Some(WorktreeHead::Branch("feature".into())),
                            }])
                            .timestamp("2mo"),
                    )
                    .into_any_element(),
            ),
            single_example(
                "All Metadata",
                container()
                    .child(
                        ThreadItem::new("ti-5l", "Thread with every metadata field populated")
                            .icon(IconName::ZedAgent)
                            .project_name("remote-dev")
                            .worktrees(vec![ThreadItemWorktreeInfo {
                                worktree_name: Some("my-worktree".into()),
                                full_path: "/worktrees/my-worktree/zed".into(),
                                highlight_positions: Vec::new(),
                                kind: WorktreeKind::Linked,
                                head: Some(WorktreeHead::Branch("main".into())),
                            }])
                            .added(15)
                            .removed(4)
                            .timestamp("8h"),
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
                "Action Slot",
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
        ];

        example_group(thread_item_examples)
            .vertical()
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Background, Modifiers, TestAppContext, VisualTestContext, point};

    #[gpui::test]
    fn test_thread_action_padding_preserves_row_background(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let (view, cx) = cx.add_window_view(|_, _| ThreadItemTestView { clicks: 0 });
        let action_bounds = cx.debug_bounds("ACTION_SLOT").expect("action bounds");
        let position = point(action_bounds.left() + px(2.), action_bounds.center().y);
        cx.simulate_mouse_move(position, None, Modifiers::default());
        let before = painted_backgrounds(cx);
        cx.simulate_mouse_down(position, MouseButton::Left, Modifiers::default());
        assert_eq!(painted_backgrounds(cx), before);
        cx.simulate_mouse_up(position, MouseButton::Left, Modifiers::default());
        assert_eq!(view.read_with(cx, |view, _| view.clicks), 0);

        let position = point(px(25.), action_bounds.center().y);
        cx.simulate_mouse_move(position, None, Modifiers::default());
        cx.simulate_mouse_down(position, MouseButton::Left, Modifiers::default());
        let active = cx.update(|_, cx| Background::from(cx.theme().colors().ghost_element_active));
        assert_eq!(painted_backgrounds(cx).first(), Some(&active));
        cx.simulate_mouse_up(position, MouseButton::Left, Modifiers::default());
        assert_eq!(view.read_with(cx, |view, _| view.clicks), 1);
    }

    struct ThreadItemTestView {
        clicks: usize,
    }

    impl Render for ThreadItemTestView {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().p(px(20.)).child(
                div().w(px(400.)).child(
                    ThreadItem::new("thread", "Thread")
                        .hovered(true)
                        .action_slot(
                            div()
                                .debug_selector(|| "ACTION_SLOT".to_owned())
                                .pl(px(16.))
                                .child(
                                    IconButton::new("action", IconName::Archive)
                                        .on_click(|_, _, _| {}),
                                ),
                        )
                        .on_click(cx.listener(|view, _, _, _| view.clicks += 1)),
                ),
            )
        }
    }

    fn painted_backgrounds(cx: &mut VisualTestContext) -> Vec<Background> {
        cx.update(|window, _| {
            window
                .painted_quads()
                .into_iter()
                .map(|quad| quad.background)
                .collect()
        })
    }

    #[test]
    fn test_thread_item_worktree_display_labels() {
        let main = ThreadItemWorktreeInfo {
            worktree_name: Some("zed".into()),
            head: Some(WorktreeHead::Branch("main".into())),
            ..Default::default()
        };
        assert_eq!(
            main.display_labels(false),
            Some((None, Some("main".into())))
        );
        assert_eq!(
            main.display_labels(true),
            Some((Some("zed".into()), Some("main".into())))
        );

        let main_without_branch = ThreadItemWorktreeInfo {
            worktree_name: Some("zed".into()),
            ..Default::default()
        };
        assert_eq!(main_without_branch.display_labels(false), None);

        let linked = ThreadItemWorktreeInfo {
            worktree_name: Some("feature".into()),
            head: Some(WorktreeHead::Branch("feature-branch".into())),
            kind: WorktreeKind::Linked,
            ..Default::default()
        };
        assert_eq!(
            linked.display_labels(false),
            Some((Some("feature".into()), Some("feature-branch".into())))
        );

        let detached = ThreadItemWorktreeInfo {
            worktree_name: Some("detached".into()),
            head: Some(WorktreeHead::Detached),
            kind: WorktreeKind::Linked,
            ..Default::default()
        };
        assert_eq!(
            detached.display_labels(false),
            Some((Some("detached".into()), Some("Detached HEAD".into())))
        );
    }
}
