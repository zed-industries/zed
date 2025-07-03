use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{
    Action, ClickEvent, Entity, IntoElement, ParentElement, linear_color_stop, linear_gradient,
};
use ui::{Divider, List, Vector, VectorName, prelude::*};
use zed_actions::agent::OpenConfiguration;

use crate::{BulletItem, ZedAiOnboarding};

pub struct AgentPanelOnboarding {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    continue_with_free_plan: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl AgentPanelOnboarding {
    pub fn new(
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        continue_with_free_plan: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            user_store,
            client,
            continue_with_free_plan: Arc::new(continue_with_free_plan),
        }
    }

    fn configure_providers(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.dispatch_action(OpenConfiguration.boxed_clone(), cx);
        cx.notify();
    }
}

impl Render for AgentPanelOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bring_api_keys = v_flex()
            .mt_2()
            .gap_1()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new("API Keys")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .buffer_font(cx),
                    )
                    .child(Divider::horizontal()),
            )
            .child(
                List::new()
                    .child(BulletItem::new(
                        "You can also use AI in Zed by bringing your own API keys",
                    ))
                    .child(BulletItem::new(
                        "No need for any of the plans or even to sign in",
                    )),
            )
            .child(
                Button::new("configure-providers", "Configure Models")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click(cx.listener(Self::configure_providers)),
            );

        div()
            .m_4()
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
                            .absolute()
                            .top(px(-8.0))
                            .right_0()
                            .w(px(400.))
                            .h(px(92.))
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
                            .top_0()
                            .right_0()
                            .w(px(660.))
                            .h(px(401.))
                            .overflow_hidden()
                            .bg(linear_gradient(
                                75.,
                                linear_color_stop(
                                    cx.theme().colors().panel_background.alpha(0.01),
                                    1.0,
                                ),
                                linear_color_stop(cx.theme().colors().panel_background, 0.45),
                            )),
                    )
                    .child(ZedAiOnboarding::new(
                        self.client.clone(),
                        &self.user_store,
                        self.continue_with_free_plan.clone(),
                        cx,
                    ))
                    .child(bring_api_keys),
            )
    }
}
