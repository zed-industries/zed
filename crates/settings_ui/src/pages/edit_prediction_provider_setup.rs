use anyhow::Result;
use edit_prediction::EditPredictionStore;
use gpui::{Entity, Task, prelude::*};
use ui::{ConfiguredApiCard, Divider, prelude::*};

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
                "Based on diffusion LLMs (dLLMs), which generate tokens in parallel.",
                |ep_store| ep_store.has_mercury_api_token(),
                |ep_store, api_token, cx| ep_store.mercury.set_api_token(api_token, cx),
                ep_store.clone(),
                window,
                cx,
            )
            .into_any_element(),
            render_api_key_provider(
                "Sweep",
                "Write code 2x faster with Sweep's AI.",
                |ep_store| ep_store.has_sweep_api_token(),
                |ep_store, api_token, cx| ep_store.sweep_ai.set_api_token(api_token, cx),
                ep_store.clone(),
                window,
                cx,
            )
            .into_any_element(),
        ];

        v_flex()
            .p_8()
            .pt_0()
            .gap_4()
            .child(Headline::new("Edit Prediction Providers"))
            .children({
                let provider_count = providers.len();
                providers
                    .into_iter()
                    .enumerate()
                    .flat_map(move |(index, provider)| {
                        [
                            provider,
                            if index + 1 != provider_count {
                                Divider::horizontal().into_any_element()
                            } else {
                                gpui::Empty.into_any_element()
                            },
                        ]
                    })
            })
    }
}

fn render_api_key_provider(
    title: &'static str,
    description: &'static str,
    key_configured: impl FnOnce(&EditPredictionStore) -> bool,
    write_key: impl Fn(&mut EditPredictionStore, Option<String>, &mut App) -> Task<Result<()>> + 'static,
    ep_store: Option<Entity<EditPredictionStore>>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let has_key = ep_store
        .as_ref()
        // todo! expand key_configured to also tell whether key is from env, and what env var name is used, disable reset if from env
        .is_some_and(|ep_store| key_configured(ep_store.read(cx)));

    let configuration_block = if has_key {
        ConfiguredApiCard::new("API key configured")
            .button_label("Reset Key")
            .button_tab_index(0)
            .on_click(move |_, _, cx| {
                if let Some(ep_store) = ep_store.as_ref() {
                    ep_store
                        .update(cx, |ep_store, cx| write_key(ep_store, None, cx))
                        .detach_and_log_err(cx)
                }
            })
            .into_any_element()
    } else {
        SettingsInputField::new()
            .tab_index(0)
            .with_placeholder("sk_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
            .on_confirm(move |api_key, cx| {
                if let Some(ep_store) = ep_store.as_ref() {
                    ep_store
                        .update(cx, |ep_store, cx| {
                            write_key(ep_store, api_key.filter(|key| !key.is_empty()), cx)
                        })
                        .detach_and_log_err(cx)
                }
            })
            .into_any_element()
    };

    v_flex()
        .id(title)
        .min_w_0()
        .size_full()
        .child(Label::new(title))
        .child(
            Label::new(description)
                .color(Color::Muted)
                .size(LabelSize::Small)
                .mb_1(),
        )
        .child(configuration_block)
}
