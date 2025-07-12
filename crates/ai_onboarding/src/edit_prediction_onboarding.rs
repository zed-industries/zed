use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{ClickEvent, Entity, IntoElement, ParentElement};
use language::language_settings::{AllLanguageSettings, EditPredictionProvider};
use project::Fs;
use settings::update_settings_file;
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
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            user_store,
            client,
            continue_with_zed_ai: Arc::new(|_window, cx| {
                set_edit_prediction_provider(EditPredictionProvider::Zed, cx);
            }),
        }
    }

    fn configure_github_copilot(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        set_edit_prediction_provider(EditPredictionProvider::Copilot, cx);
        cx.notify();
    }
}

pub(crate) fn set_edit_prediction_provider(provider: EditPredictionProvider, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);
    update_settings_file::<AllLanguageSettings>(fs, cx, move |settings, _| {
        settings
            .features
            .get_or_insert(Default::default())
            .edit_prediction_provider = Some(provider);
    });
}

impl Render for EditPredictionOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let github_copilot = v_flex()
            .gap_1()
            .child(Label::new(
                "Alternatively, you can use GitHub Copilot as your edit prediction provider.",
            ))
            .child(
                Button::new("configure-copilot", "Configure Copilot")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click(cx.listener(Self::configure_github_copilot)),
            );

        v_flex()
            .gap_2()
            .child(ZedAiOnboarding::new(
                self.client.clone(),
                &self.user_store,
                self.continue_with_zed_ai.clone(),
                cx,
            ))
            .child(ui::Divider::horizontal())
            .child(github_copilot)
    }
}
