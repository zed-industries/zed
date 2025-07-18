use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{Action, ClickEvent, Entity, IntoElement, ParentElement};
use language_model::{LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID};
use ui::{Divider, List, prelude::*};
use zed_actions::agent::{OpenConfiguration, ToggleModelSelector};

use crate::{AgentPanelOnboardingCard, BulletItem, ZedAiOnboarding};

pub struct AgentPanelOnboarding {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    configured_providers: Vec<(IconName, SharedString)>,
    continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl AgentPanelOnboarding {
    pub fn new(
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        continue_with_zed_ai: impl Fn(&mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(
            &LanguageModelRegistry::global(cx),
            |this: &mut Self, _registry, event: &language_model::Event, cx| match event {
                language_model::Event::ProviderStateChanged
                | language_model::Event::AddedProvider(_)
                | language_model::Event::RemovedProvider(_) => {
                    this.configured_providers = Self::compute_available_providers(cx)
                }
                _ => {}
            },
        )
        .detach();

        Self {
            user_store,
            client,
            configured_providers: Self::compute_available_providers(cx),
            continue_with_zed_ai: Arc::new(continue_with_zed_ai),
        }
    }

    fn compute_available_providers(cx: &App) -> Vec<(IconName, SharedString)> {
        LanguageModelRegistry::read_global(cx)
            .providers()
            .iter()
            .filter(|provider| {
                provider.is_authenticated(cx) && provider.id() != ZED_CLOUD_PROVIDER_ID
            })
            .map(|provider| (provider.icon(), provider.name().0.clone()))
            .collect()
    }

    fn configure_providers(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.dispatch_action(OpenConfiguration.boxed_clone(), cx);
        cx.notify();
    }

    fn render_api_keys_section(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_existing_providers = self.configured_providers.len() > 0;
        let configure_provider_label = if has_existing_providers {
            "Configure Other Provider"
        } else {
            "Configure Providers"
        };

        let content = if has_existing_providers {
            List::new()
                    .child(BulletItem::new(
                        "Or start now using API keys from your environment for the following providers:"
                    ))
                    .child(
                        h_flex()
                            .px_5()
                            .gap_2()
                            .flex_wrap()
                            .children(self.configured_providers.iter().cloned().map(|(icon, name)|
                                h_flex()
                                    .gap_1p5()
                                    .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                                    .child(Label::new(name))
                            ))
                    )
                    .child(BulletItem::new(
                        "No need for any of the plans or even to sign in",
                    ))
        } else {
            List::new()
                .child(BulletItem::new(
                    "You can also use AI in Zed by bringing your own API keys",
                ))
                .child(BulletItem::new(
                    "No need for any of the plans or even to sign in",
                ))
        };

        v_flex()
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
            .child(content)
            .when(has_existing_providers, |this| {
                this.child(
                    Button::new("pick-model", "Choose Model")
                        .full_width()
                        .style(ButtonStyle::Outlined)
                        .on_click(|_event, window, cx| {
                            window.dispatch_action(ToggleModelSelector.boxed_clone(), cx)
                        }),
                )
            })
            .child(
                Button::new("configure-providers", configure_provider_label)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click(cx.listener(Self::configure_providers)),
            )
    }
}

impl Render for AgentPanelOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        AgentPanelOnboardingCard::new()
            .child(ZedAiOnboarding::new(
                self.client.clone(),
                &self.user_store,
                self.continue_with_zed_ai.clone(),
                cx,
            ))
            .child(self.render_api_keys_section(cx))
    }
}
