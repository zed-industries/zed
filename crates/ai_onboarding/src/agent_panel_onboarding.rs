use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{Action, ClickEvent, Entity, IntoElement, ParentElement};
use ui::{Divider, List, prelude::*};
use zed_actions::agent::OpenConfiguration;

use crate::{BulletItem, OnboardingContainer, ZedAiOnboarding};

pub struct AgentPanelOnboarding {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    continue_with_plan: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl AgentPanelOnboarding {
    pub fn new(
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        continue_with_plan: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            user_store,
            client,
            continue_with_plan: Arc::new(continue_with_plan),
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

        OnboardingContainer::new()
            .child(ZedAiOnboarding::new(
                self.client.clone(),
                &self.user_store,
                self.continue_with_plan.clone(),
                cx,
            ))
            .child(bring_api_keys)
    }
}
