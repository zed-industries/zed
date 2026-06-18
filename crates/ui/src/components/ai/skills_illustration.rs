use crate::prelude::*;
use gpui::{linear_color_stop, linear_gradient};

#[derive(IntoElement)]
pub struct SkillsIllustration;

impl SkillsIllustration {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for SkillsIllustration {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let skill_crease = |label: SharedString, source: SharedString| {
            h_flex()
                .py_1()
                .px_1p5()
                .gap_1p5()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().element_active.opacity(0.5))
                .justify_center()
                .rounded_md()
                .shadow_sm()
                .child(
                    Icon::new(IconName::Sparkle)
                        .color(Color::Muted)
                        .size(IconSize::XSmall),
                )
                .child(Label::new(label).size(LabelSize::XSmall).buffer_font(cx))
                .child(
                    Label::new(format!("({source})"))
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .buffer_font(cx),
                )
        };

        let skill_list = v_flex()
            .absolute()
            .top_8()
            .gap_2p5()
            .items_center()
            .child(
                h_flex()
                    .gap_2p5()
                    .child(skill_crease("img-gen".into(), "studio".into()))
                    .child(skill_crease("frontend-design".into(), "global".into())),
            )
            .child(
                h_flex()
                    .gap_2p5()
                    .child(skill_crease("brainstorming".into(), "global".into()))
                    .child(skill_crease("borrow-checker-expert".into(), "zed".into())),
            )
            .child(
                h_flex()
                    .gap_2p5()
                    .child(skill_crease("grill-with-docs".into(), "global".into()))
                    .child(skill_crease("video-edit".into(), "studio".into())),
            );

        let gradient_bg = cx.theme().colors().editor_background;
        let gradient_fade = div()
            .absolute()
            .rounded_t_md()
            .inset_0()
            .bg(linear_gradient(
                0.,
                linear_color_stop(gradient_bg.opacity(0.8), 0.),
                linear_color_stop(gradient_bg.opacity(0.0), 1.),
            ));

        v_flex()
            .relative()
            .h(rems_from_px(150.))
            .justify_end()
            .items_center()
            .rounded_t_md()
            .overflow_hidden()
            .bg(gpui::black().opacity(0.2))
            .child(skill_list)
            .child(gradient_fade)
    }
}
