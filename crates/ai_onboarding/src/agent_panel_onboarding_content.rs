use std::sync::Arc;

use client::{Client, UserStore};
use cloud_llm_client::{Plan, PlanV1, PlanV2};
use gpui::{Entity, IntoElement, ParentElement};
use language_model::{LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID};
use ui::prelude::*;

use crate::{AgentPanelOnboardingCard, ApiKeysWithoutProviders, ZedAiOnboarding};

pub struct AgentPanelOnboarding {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    has_configured_providers: bool,
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
                language_model::Event::ProviderStateChanged(_)
                | language_model::Event::AddedProvider(_)
                | language_model::Event::RemovedProvider(_)
                | language_model::Event::ProvidersChanged => {
                    this.has_configured_providers = Self::has_configured_providers(cx)
                }
                _ => {}
            },
        )
        .detach();

        Self {
            user_store,
            client,
            has_configured_providers: Self::has_configured_providers(cx),
            continue_with_zed_ai: Arc::new(continue_with_zed_ai),
        }
    }

    fn has_configured_providers(cx: &App) -> bool {
        LanguageModelRegistry::read_global(cx)
            .visible_providers()
            .iter()
            .any(|provider| provider.is_authenticated(cx) && provider.id() != ZED_CLOUD_PROVIDER_ID)
    }
}

impl Render for AgentPanelOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let enrolled_in_trial = self.user_store.read(cx).plan().is_some_and(|plan| {
            matches!(
                plan,
                Plan::V1(PlanV1::ZedProTrial) | Plan::V2(PlanV2::ZedProTrial)
            )
        });
        let is_pro_user = self.user_store.read(cx).plan().is_some_and(|plan| {
            matches!(plan, Plan::V1(PlanV1::ZedPro) | Plan::V2(PlanV2::ZedPro))
        });

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
                if enrolled_in_trial || is_pro_user || self.has_configured_providers {
                    this
                } else {
                    this.child(ApiKeysWithoutProviders::new())
                }
            })
    }
}
