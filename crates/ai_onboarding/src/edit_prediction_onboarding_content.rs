use std::sync::Arc;

use client::{Client, UserStore};
use cloud_llm_client::{Plan, PlanV2};
use gpui::{Entity, IntoElement, ParentElement};
use ui::prelude::*;

use crate::ZedAiOnboarding;

pub struct EditPredictionOnboarding {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    copilot_is_configured: bool,
    continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
    continue_with_copilot: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl EditPredictionOnboarding {
    pub fn new(
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        copilot_is_configured: bool,
        continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
        continue_with_copilot: Arc<dyn Fn(&mut Window, &mut App)>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            user_store,
            copilot_is_configured,
            client,
            continue_with_zed_ai,
            continue_with_copilot,
        }
    }
}

impl Render for EditPredictionOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_free_plan = self
            .user_store
            .read(cx)
            .plan()
            .is_some_and(|plan| plan == Plan::V2(PlanV2::ZedFree));

        let github_copilot = v_flex()
            .gap_1()
            .child(Label::new(if self.copilot_is_configured {
                "Alternatively, you can continue to use GitHub Copilot as that's already set up."
            } else {
                "Alternatively, you can use GitHub Copilot as your edit prediction provider."
            }))
            .child(
                Button::new(
                    "configure-copilot",
                    if self.copilot_is_configured {
                        "Use Copilot"
                    } else {
                        "Configure Copilot"
                    },
                )
                .full_width()
                .style(ButtonStyle::Outlined)
                .on_click({
                    let callback = self.continue_with_copilot.clone();
                    move |_, window, cx| callback(window, cx)
                }),
            );

        v_flex()
            .gap_2()
            .child(ZedAiOnboarding::new(
                self.client.clone(),
                &self.user_store,
                self.continue_with_zed_ai.clone(),
                cx,
            ))
            .when(is_free_plan, |this| {
                this.child(ui::Divider::horizontal()).child(github_copilot)
            })
    }
}
