use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{Entity, IntoElement};
use ui::prelude::*;

use crate::ZedAiOnboarding;

pub struct EditPredictionOnboarding {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl EditPredictionOnboarding {
    pub fn new(
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            user_store,
            client,
            continue_with_zed_ai,
        }
    }
}

impl Render for EditPredictionOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ZedAiOnboarding::new(
            self.client.clone(),
            &self.user_store,
            self.continue_with_zed_ai.clone(),
            cx,
        )
    }
}
