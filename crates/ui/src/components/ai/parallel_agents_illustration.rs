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

        let loading_bar = |id: &'static str, width: DefiniteLength, duration_ms: u64| {
            div()
                .h(rems_from_px(5.))
                .w(width)
                .rounded_full()
                .bg(cx.theme().colors().element_selected)
                .with_animation(
                    id,
                    Animation::new(Duration::from_millis(duration_ms))
                        .repeat()
                        .with_easing(pulsating_between(0.1, 0.8)),
                    |label, delta| label.opacity(delta),
                )
        };

        let skeleton_bar = |width: DefiniteLength| {
            div().h(rems_from_px(5.)).w(width).rounded_full().bg(cx
                .theme()
                .colors()
                .text_muted
                .opacity(0.05))
        };

        let time =
            |time: SharedString| Label::new(time).size(LabelSize::XSmall).color(Color::Muted);

        let worktree = |worktree: SharedString| {
            h_flex()
                .gap_0p5()
                .child(
                    Icon::new(IconName::GitWorktree)
                        .color(Color::Muted)
                        .size(IconSize::Indicator),
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

        let agent = |title: SharedString, icon: IconName, selected: bool, data: Vec<AnyElement>| {
            v_flex()
                .when(selected, |this| {
                    this.bg(cx.theme().colors().element_active.opacity(0.2))
                })
                .p_1()
                .child(
                    h_flex()
                        .w_full()
                        .gap_1()
                        .child(
                            icon_container()
                                .child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted)),
                        )
                        .map(|this| {
                            if selected {
                                this.child(
                                    Label::new(title)
                                        .color(Color::Muted)
                                        .size(LabelSize::XSmall),
                                )
                            } else {
                                this.child(skeleton_bar(relative(0.7)))
                            }
                        }),
                )
                .child(
                    h_flex()
                        .opacity(0.8)
                        .w_full()
                        .gap_1()
                        .child(icon_container())
                        .children(data),
                )
        };

        let agents = v_flex()
            .col_span(3)
            .bg(cx.theme().colors().elevated_surface_background)
            .child(agent(
                "Fix branch label".into(),
                IconName::ZedAgent,
                true,
                vec![
                    worktree("bug-fix".into()).into_any_element(),
                    dot_separator().into_any_element(),
                    DiffStat::new("ds", 5, 2)
                        .label_size(LabelSize::XSmall)
                        .into_any_element(),
                    dot_separator().into_any_element(),
                    time("2m".into()).into_any_element(),
                ],
            ))
            .child(Divider::horizontal())
            .child(agent(
                "Improve thread id".into(),
                IconName::AiClaude,
                false,
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
                "Refactor archive view".into(),
                IconName::AiOpenAi,
                false,
                vec![
                    worktree("silent-forest".into()).into_any_element(),
                    dot_separator().into_any_element(),
                    time("37m".into()).into_any_element(),
                ],
            ));

        let thread_view = v_flex()
            .col_span(3)
            .h_full()
            .flex_1()
            .border_l_1()
            .border_color(cx.theme().colors().border.opacity(0.5))
            .bg(cx.theme().colors().panel_background)
            .child(
                h_flex()
                    .px_1p5()
                    .py_0p5()
                    .w_full()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border.opacity(0.5))
                    .child(
                        Label::new("Fix branch label")
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        Icon::new(IconName::Plus)
                            .size(IconSize::Indicator)
                            .color(Color::Muted),
                    ),
            )
            .child(
                div().p_1().child(
                    v_flex()
                        .px_1()
                        .py_1p5()
                        .gap_1()
                        .border_1()
                        .border_color(cx.theme().colors().border.opacity(0.5))
                        .bg(cx.theme().colors().editor_background)
                        .rounded_sm()
                        .shadow_sm()
                        .child(skeleton_bar(relative(0.7)))
                        .child(skeleton_bar(relative(0.2))),
                ),
            )
            .child(
                v_flex()
                    .p_2()
                    .gap_1()
                    .child(loading_bar("a", relative(0.55), 2200))
                    .child(loading_bar("b", relative(0.75), 2000))
                    .child(loading_bar("c", relative(0.25), 2400)),
            );

        let file_row = |indent: usize, is_folder: bool, bar_width: Rems| {
            let indent_px = rems_from_px((indent as f32) * 4.0);

            h_flex()
                .px_2()
                .py_px()
                .gap_1()
                .pl(indent_px)
                .child(
                    icon_container().child(
                        Icon::new(if is_folder {
                            IconName::FolderOpen
                        } else {
                            IconName::FileRust
                        })
                        .size(IconSize::Indicator)
                        .color(Color::Custom(cx.theme().colors().icon_muted.opacity(0.2))),
                    ),
                )
                .child(
                    div().h_1p5().w(bar_width).rounded_sm().bg(cx
                        .theme()
                        .colors()
                        .text
                        .opacity(if is_folder { 0.15 } else { 0.1 })),
                )
        };

        let project_panel = v_flex()
            .col_span(1)
            .h_full()
            .flex_1()
            .border_l_1()
            .border_color(cx.theme().colors().border.opacity(0.5))
            .bg(cx.theme().colors().panel_background)
            .child(
                v_flex()
                    .child(file_row(0, true, rems_from_px(42.0)))
                    .child(file_row(1, true, rems_from_px(28.0)))
                    .child(file_row(2, false, rems_from_px(52.0)))
                    .child(file_row(2, false, rems_from_px(36.0)))
                    .child(file_row(2, false, rems_from_px(44.0)))
                    .child(file_row(1, true, rems_from_px(34.0)))
                    .child(file_row(2, false, rems_from_px(48.0)))
                    .child(file_row(2, true, rems_from_px(26.0)))
                    .child(file_row(3, false, rems_from_px(40.0)))
                    .child(file_row(3, false, rems_from_px(56.0)))
                    .child(file_row(1, false, rems_from_px(38.0)))
                    .child(file_row(0, true, rems_from_px(30.0)))
                    .child(file_row(1, false, rems_from_px(46.0)))
                    .child(file_row(1, false, rems_from_px(32.0))),
            );

        let workspace = div()
            .absolute()
            .top_8()
            .grid()
            .grid_cols(7)
            .w(rems_from_px(380.))
            .rounded_t_sm()
            .border_1()
            .border_color(cx.theme().colors().border.opacity(0.5))
            .shadow_md()
            .child(agents)
            .child(thread_view)
            .child(project_panel);

        h_flex()
            .relative()
            .h(rems_from_px(180.))
            .bg(cx.theme().colors().editor_background.opacity(0.6))
            .justify_center()
            .items_end()
            .rounded_t_md()
            .overflow_hidden()
            .bg(gpui::black().opacity(0.2))
            .child(workspace)
    }
}
