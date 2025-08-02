use std::sync::Arc;

use client::{Client, UserStore};
use cloud_llm_client::Plan;
use gpui::{Entity, IntoElement, ParentElement};
use language_model::{LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID};
use ui::prelude::*;

use crate::{AgentPanelOnboardingCard, ApiKeysWithoutProviders, ZedAiOnboarding};

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
}

impl Render for AgentPanelOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let enrolled_in_trial = self.user_store.read(cx).plan() == Some(Plan::ZedProTrial);
        let is_pro_user = self.user_store.read(cx).plan() == Some(Plan::ZedPro);

        AgentPanelOnboardingCard::new()
            .child(
                ZedAiOnboarding::new(
                    self.client.clone(),
                    &self.user_store,
                    self.continue_with_zed_ai.clone(),
                    cx,
                )
                .with_dismiss({
                    let callback = self.continue_with_zed_ai.clone();
                    move |window, cx| callback(window, cx)
                }),
            )
            .map(|this| {
                if enrolled_in_trial || is_pro_user || self.configured_providers.len() >= 1 {
                    this
                } else {
                    this.child(ApiKeysWithoutProviders::new())
                }
            })
    }
}
