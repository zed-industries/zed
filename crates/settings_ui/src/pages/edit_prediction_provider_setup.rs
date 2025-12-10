use edit_prediction::{
    ApiKeyState, EditPredictionStore, Zeta2FeatureFlag, mercury::MERCURY_CREDENTIALS_URL,
    sweep_ai::SWEEP_CREDENTIALS_URL,
};
use feature_flags::FeatureFlagAppExt as _;
use gpui::{Entity, ScrollHandle, prelude::*};
use ui::{ButtonLink, ConfiguredApiCard, Divider, List, ListBulletItem, WithScrollbar, prelude::*};

use crate::components::SettingsInputField;

pub struct EditPredictionSetupPage {
    scroll_handle: ScrollHandle,
}

impl EditPredictionSetupPage {
    pub fn new() -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for EditPredictionSetupPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // todo! skip ep_store for loading keys
        let ep_store = EditPredictionStore::try_global(cx);

        let providers = [
            cx.has_flag::<Zeta2FeatureFlag>().then(|| {
                render_api_key_provider(
                    IconName::Inception,
                    "Mercury",
                    ButtonLink::new(
                        "Mercury's console",
                        "https://platform.inceptionlabs.ai/dashboard/api-keys",
                    )
                    .into_any_element(),
                    |ep_store| &mut ep_store.mercury.api_token,
                    |_cx| MERCURY_CREDENTIALS_URL,
                    ep_store.clone(),
                    window,
                    cx,
                )
                .into_any_element()
            }),
            cx.has_flag::<Zeta2FeatureFlag>().then(|| {
                render_api_key_provider(
                    IconName::SweepAi,
                    "Sweep",
                    ButtonLink::new("Sweep's console", "https://app.sweep.dev/").into_any_element(),
                    |ep_store| &mut ep_store.sweep_ai.api_token,
                    |_cx| SWEEP_CREDENTIALS_URL,
                    ep_store.clone(),
                    window,
                    cx,
                )
                .into_any_element()
            }),
            Some(
                render_api_key_provider(
                    IconName::AiMistral,
                    "Codestral",
                    ButtonLink::new(
                        "the Codestral section of Mistral's console",
                        "https://console.mistral.ai/codestral",
                    )
                    .into_any_element(),
                    |state| &mut state.codestral_api_key_state,
                    |cx| language_models::MistralLanguageModelProvider::api_url(cx),
                    language_models::MistralLanguageModelProvider::try_global(cx)
                        .map(|provider| provider.state.clone()),
                    // todo! preview: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
                    window,
                    cx,
                )
                .into_any_element(),
            ),
        ];

        div()
            .size_full()
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
            .child(
                v_flex()
                    .id("ep-setup-page")
                    .min_w_0()
                    .size_full()
                    .px_8()
                    .pb_32()
                    .gap_4()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(Headline::new("Edit Prediction Providers"))
                    .children({
                        let mut elements = vec![];
                        for provider in providers {
                            let Some(provider) = provider else {
                                continue;
                            };
                            elements.push(provider);
                            elements.push(Divider::horizontal().into_any_element());
                        }
                        elements.pop();
                        elements
                    }),
            )
    }
}

fn render_api_key_provider<Ent: 'static>(
    icon: IconName,
    title: &'static str,
    link: AnyElement,
    api_key_state: fn(&mut Ent) -> &mut ApiKeyState,
    current_url: fn(&mut Context<Ent>) -> SharedString,
    entity: Option<Entity<Ent>>,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let (has_key, env_var_name) = entity
        .as_ref()
        // todo! expand key_configured to also tell whether key is from env, and what env var name is used, disable reset if from env
        .map(|entity| {
            entity.update(cx, |entity, _| {
                let state = api_key_state(entity);
                (state.has_key(), Some(state.env_var_name().clone()))
            })
        })
        .unwrap_or((false, None));

    let write_key = move |entity: Option<Entity<Ent>>, api_key: Option<String>, cx: &mut App| {
        if let Some(entity) = entity {
            entity
                .update(cx, |entity, cx| {
                    let key_state = api_key_state(entity);
                    let url = current_url(cx);
                    key_state.store(url, api_key, api_key_state, cx)
                })
                .detach_and_log_err(cx);
        }
    };

    let base_container = v_flex().id(title).min_w_0().gap_1p5();

    let icon_and_name = h_flex()
        .gap_1()
        .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
        .child(Label::new(title));

    let description = format!(
        "To use {} as an edit prediction provider, you need an API key. Follow these steps:",
        title
    );

    let container = if has_key {
        base_container.child(icon_and_name).child(
            ConfiguredApiCard::new("API key configured")
                .button_label("Reset Key")
                .button_tab_index(0)
                // .disabled()
                // todo! disabled if from env, should have env var on ApiKeyState to get the env var name
                .on_click(move |_, _, cx| {
                    write_key(entity.clone(), None, cx);
                }),
        )
    } else {
        base_container
            .child(
                v_flex()
                    .w_full()
                    .gap_1p5()
                    .child(icon_and_name)
                    .child(Label::new(description).color(Color::Muted)),
            )
            .child(
                List::new()
                    .child(
                        ListBulletItem::new("")
                            .child(Label::new("Create one by visiting").color(Color::Muted))
                            .child(link),
                    )
                    .child(
                        ListBulletItem::new("Paste your API key below and hit enter")
                            .label_color(Color::Muted),
                    ),
            )
            .child(
                SettingsInputField::new()
                    .tab_index(0)
                    .with_placeholder("sk_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                    .on_confirm(move |api_key, cx| {
                        write_key(entity.clone(), api_key.filter(|key| !key.is_empty()), cx);
                    }),
            )
            .when_some(env_var_name, |this, env_var_name| {
                this.child({
                    let label = format!(
                        "You can also assign the {} environment variable and restart Zed.",
                        env_var_name.as_ref()
                    );
                    Label::new(label)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .mt_0p5()
                })
            })
    };

    container
}

// todo! github copilot
