use gpui::{AnyElement, IntoElement, ParentElement, linear_color_stop, linear_gradient};
use smallvec::SmallVec;
use ui::{Vector, VectorName, prelude::*};

#[derive(IntoElement)]
pub struct AgentPanelOnboardingCard {
    children: SmallVec<[AnyElement; 2]>,
}

impl AgentPanelOnboardingCard {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for AgentPanelOnboardingCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for AgentPanelOnboardingCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .m_2p5()
            .p(px(3.))
            .elevation_2(cx)
            .rounded_lg()
            .bg(cx.theme().colors().background.alpha(0.5))
            .child(
                v_flex()
                    .relative()
                    .size_full()
                    .px_4()
                    .py_3()
                    .gap_2()
                    .border_1()
                    .rounded(px(5.))
                    .border_color(cx.theme().colors().text.alpha(0.1))
                    .overflow_hidden()
                    .bg(cx.theme().colors().panel_background)
                    .child(
                        div()
                            .opacity(0.5)
                            .absolute()
                            .top(px(-8.0))
                            .right_0()
                            .w(px(400.))
                            .h(px(92.))
                            .rounded_md()
                            .child(
                                Vector::new(
                                    VectorName::AiGrid,
                                    rems_from_px(400.),
                                    rems_from_px(92.),
                                )
                                .color(Color::Custom(cx.theme().colors().text.alpha(0.32))),
                            ),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_0p5()
                            .right_0p5()
                            .w(px(660.))
                            .h(px(401.))
                            .overflow_hidden()
                            .rounded_md()
                            .bg(linear_gradient(
                                75.,
                                linear_color_stop(
                                    cx.theme().colors().panel_background.alpha(0.01),
                                    1.0,
                                ),
                                linear_color_stop(cx.theme().colors().panel_background, 0.45),
                            )),
                    )
                    .children(self.children),
            )
    }
}
