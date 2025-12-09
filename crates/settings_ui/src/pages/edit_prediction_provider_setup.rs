use anyhow::Result;
use edit_prediction::EditPredictionStore;
use gpui::{Entity, Task, prelude::*};
use ui::prelude::*;

use crate::components::SettingsInputField;

#[derive(IntoElement)]
pub struct EditPredictionSetupPage {}

impl EditPredictionSetupPage {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for EditPredictionSetupPage {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // todo! skip ep_store for loading keys
        let ep_store = EditPredictionStore::try_global(cx);

        let providers = [
            render_api_key_provider(
                "Mercury",
                |ep_store| ep_store.has_mercury_api_token(),
                |ep_store, api_token, cx| ep_store.mercury.set_api_token(api_token, cx),
                ep_store.clone(),
                window,
                cx,
            )
            .into_any_element(),
            render_api_key_provider(
                "Sweep",
                |ep_store| ep_store.has_sweep_api_token(),
                |ep_store, api_token, cx| ep_store.sweep_ai.set_api_token(api_token, cx),
                ep_store.clone(),
                window,
                cx,
            )
            .into_any_element(),
        ];

        v_flex()
            .gap_1()
            .child(
                v_flex()
                    .child(Headline::new("Edit Prediction Providers"))
                    .child(
                        Label::new("Configure other providers to get in-editor predictions.")
                            .color(Color::Muted),
                    ),
            )
            .children(providers)
    }
}

fn render_api_key_provider(
    title: &'static str,
    key_configured: impl FnOnce(&EditPredictionStore) -> bool,
    write_key: impl Fn(&mut EditPredictionStore, Option<String>, &mut App) -> Task<Result<()>> + 'static,
    ep_store: Option<Entity<EditPredictionStore>>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let has_key = ep_store
        .as_ref()
        .is_some_and(|ep_store| key_configured(ep_store.read(cx)));

    let configuration_block = if has_key {
        div()
            .child("API key configured")
            .child(Button::new(title, "Reset").on_click(move |_, _, cx| {
                if let Some(ep_store) = ep_store.as_ref() {
                    ep_store
                        .update(cx, |ep_store, cx| write_key(ep_store, None, cx))
                        .detach_and_log_err(cx)
                }
            }))
    } else {
        div().child(
            SettingsInputField::new()
                .with_placeholder("sk_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                .on_confirm(move |api_key, cx| {
                    if let Some(ep_store) = ep_store.as_ref() {
                        ep_store
                            .update(cx, |ep_store, cx| {
                                write_key(ep_store, api_key.filter(|key| !key.is_empty()), cx)
                            })
                            .detach_and_log_err(cx)
                    }
                }),
        )
    };

    v_flex()
        .id(title)
        .child(Label::new(title))
        .child(configuration_block)
}
