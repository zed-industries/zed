use gpui::{AnyElement, IntoElement, ParentElement, linear_color_stop, linear_gradient};
use smallvec::SmallVec;
use ui::prelude::*;

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
        let color = cx.theme().colors();

        div().min_w_0().p_2p5().bg(color.editor_background).child(
            div()
                .min_w_0()
                .p(px(3.))
                .rounded_lg()
                .elevation_2(cx)
                .bg(color.background.opacity(0.5))
                .child(
                    v_flex()
                        .relative()
                        .size_full()
                        .min_w_0()
                        .px_4()
                        .py_3()
                        .gap_2()
                        .border_1()
                        .rounded(px(5.))
                        .border_color(color.text.opacity(0.1))
                        .bg(color.panel_background)
                        .overflow_hidden()
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .size_full()
                                .rounded_md()
                                .overflow_hidden()
                                .bg(linear_gradient(
                                    360.,
                                    linear_color_stop(color.panel_background, 1.0),
                                    linear_color_stop(color.editor_background, 0.45),
                                )),
                        )
                        .children(self.children),
                ),
        )
    }
}
