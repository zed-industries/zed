use crate::{DiffStat, Divider, prelude::*};
use gpui::{Animation, AnimationExt, pulsating_between};
use std::time::Duration;

#[derive(IntoElement)]
pub struct ParallelAgentsIllustration;

impl ParallelAgentsIllustration {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ParallelAgentsIllustration {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let icon_container = || h_flex().size_4().flex_shrink_0().justify_center();

        let title_bar = |id: &'static str, width: DefiniteLength, duration_ms: u64| {
            div()
                .h_2()
                .w(width)
                .rounded_full()
                .debug_bg_blue()
                .bg(cx.theme().colors().element_selected)
                .with_animation(
                    id,
                    Animation::new(Duration::from_millis(duration_ms))
                        .repeat()
                        .with_easing(pulsating_between(0.4, 0.8)),
                    |label, delta| label.opacity(delta),
                )
        };

        let time =
            |time: SharedString| Label::new(time).size(LabelSize::XSmall).color(Color::Muted);

        let worktree = |worktree: SharedString| {
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::GitWorktree)
                        .color(Color::Muted)
                        .size(IconSize::XSmall),
                )
                .child(
                    Label::new(worktree)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                )
        };

        let dot_separator = || {
            Label::new("•")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .alpha(0.5)
        };

        let agent = |id: &'static str,
                     icon: IconName,
                     width: DefiniteLength,
                     duration_ms: u64,
                     data: Vec<AnyElement>| {
            v_flex()
                .p_2()
                .child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .child(
                            icon_container()
                                .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted)),
                        )
                        .child(title_bar(id, width, duration_ms)),
                )
                .child(
                    h_flex()
                        .opacity(0.8)
                        .w_full()
                        .gap_2()
                        .child(icon_container())
                        .children(data),
                )
        };

        let agents = v_flex()
            .absolute()
            .w(rems_from_px(380.))
            .top_8()
            .rounded_t_sm()
            .border_1()
            .border_color(cx.theme().colors().border.opacity(0.5))
            .bg(cx.theme().colors().elevated_surface_background)
            .shadow_md()
            .child(agent(
                "zed-agent-bar",
                IconName::ZedAgent,
                relative(0.7),
                1800,
                vec![
                    worktree("happy-tree".into()).into_any_element(),
                    dot_separator().into_any_element(),
                    DiffStat::new("ds", 23, 13)
                        .label_size(LabelSize::XSmall)
                        .into_any_element(),
                    dot_separator().into_any_element(),
                    time("2m".into()).into_any_element(),
                ],
            ))
            .child(Divider::horizontal())
            .child(agent(
                "claude-bar",
                IconName::AiClaude,
                relative(0.85),
                2400,
                vec![
                    DiffStat::new("ds", 120, 84)
                        .label_size(LabelSize::XSmall)
                        .into_any_element(),
                    dot_separator().into_any_element(),
                    time("16m".into()).into_any_element(),
                ],
            ))
            .child(Divider::horizontal())
            .child(agent(
                "openai-bar",
                IconName::AiOpenAi,
                relative(0.4),
                3100,
                vec![
                    worktree("silent-forest".into()).into_any_element(),
                    dot_separator().into_any_element(),
                    time("37m".into()).into_any_element(),
                ],
            ))
            .child(Divider::horizontal());

        h_flex()
            .relative()
            .h(rems_from_px(180.))
            .bg(cx.theme().colors().editor_background)
            .justify_center()
            .items_end()
            .rounded_t_md()
            .overflow_hidden()
            .bg(gpui::black().opacity(0.2))
            .child(agents)
    }
}
