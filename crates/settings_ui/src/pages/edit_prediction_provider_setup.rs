use edit_prediction::{
    ApiKeyState, Zeta2FeatureFlag,
    mercury::{MERCURY_CREDENTIALS_URL, mercury_api_token},
    sweep_ai::{SWEEP_CREDENTIALS_URL, sweep_api_token},
};
use extension_host::ExtensionStore;
use feature_flags::FeatureFlagAppExt as _;
use gpui::{AnyView, Entity, ScrollHandle, Subscription, prelude::*};
use language_model::{
    ConfigurationViewTargetAgent, LanguageModelProviderId, LanguageModelRegistry,
};
use language_models::provider::mistral::{CODESTRAL_API_URL, codestral_api_key};
use std::collections::HashMap;
use ui::{ButtonLink, ConfiguredApiCard, Icon, WithScrollbar, prelude::*};

use crate::{
    SettingField, SettingItem, SettingsFieldMetadata, SettingsPageItem, SettingsWindow, USER,
    components::{SettingsInputField, SettingsSectionHeader},
};

pub struct EditPredictionSetupPage {
    settings_window: Entity<SettingsWindow>,
    scroll_handle: ScrollHandle,
    extension_oauth_views: HashMap<LanguageModelProviderId, ExtensionOAuthProviderView>,
    _registry_subscription: Subscription,
}

struct ExtensionOAuthProviderView {
    provider_name: SharedString,
    provider_icon: IconName,
    provider_icon_path: Option<SharedString>,
    configuration_view: AnyView,
}

impl EditPredictionSetupPage {
    pub fn new(
        settings_window: Entity<SettingsWindow>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let registry_subscription = cx.subscribe_in(
            &LanguageModelRegistry::global(cx),
            window,
            |this, _, event: &language_model::Event, window, cx| match event {
                language_model::Event::AddedProvider(provider_id) => {
                    this.maybe_add_extension_oauth_view(provider_id, window, cx);
                }
                language_model::Event::RemovedProvider(provider_id) => {
                    this.extension_oauth_views.remove(provider_id);
                }
                _ => {}
            },
        );

        let mut this = Self {
            settings_window,
            scroll_handle: ScrollHandle::new(),
            extension_oauth_views: HashMap::default(),
            _registry_subscription: registry_subscription,
        };
        this.build_extension_oauth_views(window, cx);
        this
    }

    fn build_extension_oauth_views(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let oauth_provider_ids = get_extension_oauth_provider_ids(cx);
        for provider_id in oauth_provider_ids {
            self.maybe_add_extension_oauth_view(&provider_id, window, cx);
        }
    }

    fn maybe_add_extension_oauth_view(
        &mut self,
        provider_id: &LanguageModelProviderId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Check if this provider has OAuth configured in the extension manifest
        if !is_extension_oauth_provider(provider_id, cx) {
            return;
        }

        let registry = LanguageModelRegistry::global(cx).read(cx);
        let Some(provider) = registry.provider(provider_id) else {
            return;
        };

        let provider_name = provider.name().0;
        let provider_icon = provider.icon();
        let provider_icon_path = provider.icon_path();
        let configuration_view =
            provider.configuration_view(ConfigurationViewTargetAgent::EditPrediction, window, cx);

        self.extension_oauth_views.insert(
            provider_id.clone(),
            ExtensionOAuthProviderView {
                provider_name,
                provider_icon,
                provider_icon_path,
                configuration_view,
            },
        );
    }
}

impl Render for EditPredictionSetupPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings_window = self.settings_window.clone();

        let copilot_extension_installed = ExtensionStore::global(cx)
            .read(cx)
            .installed_extensions()
            .contains_key("copilot-chat");

        let mut providers: Vec<AnyElement> = Vec::new();

        // Built-in Copilot (hidden if copilot-chat extension is installed)
        if !copilot_extension_installed {
            providers.push(render_github_copilot_provider(window, cx).into_any_element());
        }

        // Extension providers with OAuth support
        for (provider_id, view) in &self.extension_oauth_views {
            let icon_element: AnyElement = if let Some(icon_path) = &view.provider_icon_path {
                Icon::from_external_svg(icon_path.clone())
                    .size(ui::IconSize::Medium)
                    .into_any_element()
            } else {
                Icon::new(view.provider_icon)
                    .size(ui::IconSize::Medium)
                    .into_any_element()
            };

            providers.push(
                v_flex()
                    .id(SharedString::from(provider_id.0.to_string()))
                    .min_w_0()
                    .gap_1p5()
                    .child(
                        h_flex().gap_2().items_center().child(icon_element).child(
                            Headline::new(view.provider_name.clone()).size(HeadlineSize::Small),
                        ),
                    )
                    .child(view.configuration_view.clone())
                    .into_any_element(),
            );
        }

        if cx.has_flag::<Zeta2FeatureFlag>() {
            providers.push(
                render_api_key_provider(
                    IconName::Inception,
                    "Mercury",
                    "https://platform.inceptionlabs.ai/dashboard/api-keys".into(),
                    mercury_api_token(cx),
                    |_cx| MERCURY_CREDENTIALS_URL,
                    None,
                    window,
                    cx,
                )
                .into_any_element(),
            );
        }

        if cx.has_flag::<Zeta2FeatureFlag>() {
            providers.push(
                render_api_key_provider(
                    IconName::SweepAi,
                    "Sweep",
                    "https://app.sweep.dev/".into(),
                    sweep_api_token(cx),
                    |_cx| SWEEP_CREDENTIALS_URL,
                    None,
                    window,
                    cx,
                )
                .into_any_element(),
            );
        }

        providers.push(
            render_api_key_provider(
                IconName::AiMistral,
                "Codestral",
                "https://console.mistral.ai/codestral".into(),
                codestral_api_key(cx),
                |cx| language_models::MistralLanguageModelProvider::api_url(cx),
                Some(settings_window.update(cx, |settings_window, cx| {
                    let codestral_settings = codestral_settings();
                    settings_window
                        .render_sub_page_items_section(
                            codestral_settings.iter().enumerate(),
                            None,
                            window,
                            cx,
                        )
                        .into_any_element()
                })),
                window,
                cx,
            )
            .into_any_element(),
        );

        div()
            .size_full()
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
            .child(
                v_flex()
                    .id("ep-setup-page")
                    .min_w_0()
                    .size_full()
                    .px_8()
                    .pb_16()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .children(providers),
            )
    }
}

/// Get extension provider IDs that have OAuth configured.
fn get_extension_oauth_provider_ids(cx: &App) -> Vec<LanguageModelProviderId> {
    let extension_store = ExtensionStore::global(cx).read(cx);

    extension_store
        .installed_extensions()
        .iter()
        .flat_map(|(extension_id, entry)| {
            entry.manifest.language_model_providers.iter().filter_map(
                move |(provider_id, provider_entry)| {
                    // Check if this provider has OAuth configured
                    let has_oauth = provider_entry
                        .auth
                        .as_ref()
                        .is_some_and(|auth| auth.oauth.is_some());

                    if has_oauth {
                        Some(LanguageModelProviderId(
                            format!("{}:{}", extension_id, provider_id).into(),
                        ))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

/// Check if a provider ID corresponds to an extension with OAuth configured.
fn is_extension_oauth_provider(provider_id: &LanguageModelProviderId, cx: &App) -> bool {
    // Extension provider IDs are in the format "extension_id:provider_id"
    let Some((extension_id, local_provider_id)) = provider_id.0.split_once(':') else {
        return false;
    };

    let extension_store = ExtensionStore::global(cx).read(cx);
    let Some(entry) = extension_store.installed_extensions().get(extension_id) else {
        return false;
    };

    entry
        .manifest
        .language_model_providers
        .get(local_provider_id)
        .and_then(|p| p.auth.as_ref())
        .is_some_and(|auth| auth.oauth.is_some())
}

fn render_api_key_provider(
    icon: IconName,
    title: &'static str,
    link: SharedString,
    api_key_state: Entity<ApiKeyState>,
    current_url: fn(&mut App) -> SharedString,
    additional_fields: Option<AnyElement>,
    window: &mut Window,
    cx: &mut Context<EditPredictionSetupPage>,
) -> impl IntoElement {
    let weak_page = cx.weak_entity();
    _ = window.use_keyed_state(title, cx, |_, cx| {
        let task = api_key_state.update(cx, |key_state, cx| {
            key_state.load_if_needed(current_url(cx), |state| state, cx)
        });
        cx.spawn(async move |_, cx| {
            task.await.ok();
            weak_page
                .update(cx, |_, cx| {
                    cx.notify();
                })
                .ok();
        })
    });

    let (has_key, env_var_name, is_from_env_var) = api_key_state.read_with(cx, |state, _| {
        (
            state.has_key(),
            Some(state.env_var_name().clone()),
            state.is_from_env_var(),
        )
    });

    let write_key = move |api_key: Option<String>, cx: &mut App| {
        api_key_state
            .update(cx, |key_state, cx| {
                let url = current_url(cx);
                key_state.store(url, api_key, |key_state| key_state, cx)
            })
            .detach_and_log_err(cx);
    };

    let base_container = v_flex().id(title).min_w_0().pt_8().gap_1p5();
    let header = SettingsSectionHeader::new(title)
        .icon(icon)
        .no_padding(true);
    let button_link_label = format!("{} dashboard", title);
    let description = h_flex()
        .min_w_0()
        .gap_0p5()
        .child(
            Label::new("Visit the")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(
            ButtonLink::new(button_link_label, link)
                .no_icon(true)
                .label_size(LabelSize::Small)
                .label_color(Color::Muted),
        )
        .child(
            Label::new("to generate an API key.")
                .size(LabelSize::Small)
                .color(Color::Muted),
        );
    let configured_card_label = if is_from_env_var {
        "API Key Set in Environment Variable"
    } else {
        "API Key Configured"
    };

    let container = if has_key {
        base_container.child(header).child(
            ConfiguredApiCard::new(configured_card_label)
                .button_label("Reset Key")
                .button_tab_index(0)
                .disabled(is_from_env_var)
                .when_some(env_var_name, |this, env_var_name| {
                    this.when(is_from_env_var, |this| {
                        this.tooltip_label(format!(
                            "To reset your API key, unset the {} environment variable.",
                            env_var_name
                        ))
                    })
                })
                .on_click(move |_, _, cx| {
                    write_key(None, cx);
                }),
        )
    } else {
        base_container.child(header).child(
            h_flex()
                .pt_2p5()
                .w_full()
                .justify_between()
                .child(
                    v_flex()
                        .w_full()
                        .max_w_1_2()
                        .child(Label::new("API Key"))
                        .child(description)
                        .when_some(env_var_name, |this, env_var_name| {
                            this.child({
                                let label = format!(
                                    "Or set the {} env var and restart Zed.",
                                    env_var_name.as_ref()
                                );
                                Label::new(label).size(LabelSize::Small).color(Color::Muted)
                            })
                        }),
                )
                .child(
                    SettingsInputField::new()
                        .tab_index(0)
                        .with_placeholder("xxxxxxxxxxxxxxxxxxxx")
                        .on_confirm(move |api_key, cx| {
                            write_key(api_key.filter(|key| !key.is_empty()), cx);
                        }),
                ),
        )
    };

    container.when_some(additional_fields, |this, additional_fields| {
        this.child(
            div()
                .map(|this| if has_key { this.mt_1() } else { this.mt_4() })
                .px_neg_8()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(additional_fields),
        )
    })
}

fn codestral_settings() -> Box<[SettingsPageItem]> {
    Box::new([
        SettingsPageItem::SettingItem(SettingItem {
            title: "API URL",
            description: "The API URL to use for Codestral.",
            field: Box::new(SettingField {
                pick: |settings| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .as_ref()?
                        .codestral
                        .as_ref()?
                        .api_url
                        .as_ref()
                },
                write: |settings, value| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .get_or_insert_default()
                        .codestral
                        .get_or_insert_default()
                        .api_url = value;
                },
                json_path: Some("edit_predictions.codestral.api_url"),
            }),
            metadata: Some(Box::new(SettingsFieldMetadata {
                placeholder: Some(CODESTRAL_API_URL),
                ..Default::default()
            })),
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Max Tokens",
            description: "The maximum number of tokens to generate.",
            field: Box::new(SettingField {
                pick: |settings| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .as_ref()?
                        .codestral
                        .as_ref()?
                        .max_tokens
                        .as_ref()
                },
                write: |settings, value| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .get_or_insert_default()
                        .codestral
                        .get_or_insert_default()
                        .max_tokens = value;
                },
                json_path: Some("edit_predictions.codestral.max_tokens"),
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Model",
            description: "The Codestral model id to use.",
            field: Box::new(SettingField {
                pick: |settings| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .as_ref()?
                        .codestral
                        .as_ref()?
                        .model
                        .as_ref()
                },
                write: |settings, value| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .get_or_insert_default()
                        .codestral
                        .get_or_insert_default()
                        .model = value;
                },
                json_path: Some("edit_predictions.codestral.model"),
            }),
            metadata: Some(Box::new(SettingsFieldMetadata {
                placeholder: Some("codestral-latest"),
                ..Default::default()
            })),
            files: USER,
        }),
    ])
}

pub(crate) fn render_github_copilot_provider(
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let configuration_view = window.use_state(cx, |_, cx| {
        copilot::ConfigurationView::new(
            |cx| {
                copilot::Copilot::global(cx)
                    .is_some_and(|copilot| copilot.read(cx).is_authenticated())
            },
            copilot::ConfigurationMode::EditPrediction,
            cx,
        )
    });

    v_flex()
        .id("github-copilot")
        .min_w_0()
        .gap_1p5()
        .child(
            SettingsSectionHeader::new("GitHub Copilot")
                .icon(IconName::Copilot)
                .no_padding(true),
        )
        .child(configuration_view)
}
