use edit_prediction::{
    ApiKeyState, EditPredictionStore, MercuryFeatureFlag, SweepFeatureFlag,
    mercury::{MERCURY_CREDENTIALS_URL, mercury_api_token},
    sweep_ai::{SWEEP_CREDENTIALS_URL, sweep_api_token},
};
use feature_flags::FeatureFlagAppExt as _;
use gpui::{Entity, ScrollHandle, prelude::*};
use language_models::provider::mistral::{CODESTRAL_API_URL, codestral_api_key};
use project::Project;
use ui::{ButtonLink, ConfiguredApiCard, prelude::*};

use crate::{
    SettingField, SettingItem, SettingsFieldMetadata, SettingsPageItem, SettingsWindow, USER,
    components::{SettingsInputField, SettingsSectionHeader},
};

pub(crate) fn render_edit_prediction_setup_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let project = settings_window.original_window.as_ref().and_then(|window| {
        window
            .read_with(cx, |workspace, _| workspace.project().clone())
            .ok()
    });
    let providers = [
        project.and_then(|project| {
            render_github_copilot_provider(project, window, cx).map(IntoElement::into_any_element)
        }),
        cx.has_flag::<MercuryFeatureFlag>().then(|| {
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
            .into_any_element()
        }),
        cx.has_flag::<SweepFeatureFlag>().then(|| {
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
            .into_any_element()
        }),
        Some(
            render_api_key_provider(
                IconName::AiMistral,
                "Codestral",
                "https://console.mistral.ai/codestral".into(),
                codestral_api_key(cx),
                |cx| language_models::MistralLanguageModelProvider::api_url(cx),
                Some(
                    settings_window
                        .render_sub_page_items_section(
                            codestral_settings().iter().enumerate(),
                            window,
                            cx,
                        )
                        .into_any_element(),
                ),
                window,
                cx,
            )
            .into_any_element(),
        ),
    ];

    div()
        .size_full()
        .child(
            v_flex()
                .id("ep-setup-page")
                .min_w_0()
                .size_full()
                .px_8()
                .pb_16()
                .overflow_y_scroll()
                .track_scroll(&scroll_handle)
                .children(providers.into_iter().flatten()),
        )
        .into_any_element()
}

fn render_api_key_provider(
    icon: IconName,
    title: &'static str,
    link: SharedString,
    api_key_state: Entity<ApiKeyState>,
    current_url: fn(&mut App) -> SharedString,
    additional_fields: Option<AnyElement>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
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
                        .on_confirm(move |api_key, _window, cx| {
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

fn render_github_copilot_provider(
    project: Entity<Project>,
    window: &mut Window,
    cx: &mut App,
) -> Option<impl IntoElement> {
    let copilot = EditPredictionStore::try_global(cx)?
        .read(cx)
        .copilot_for_project(&project);
    let configuration_view = window.use_state(cx, |_, cx| {
        copilot_ui::ConfigurationView::new(
            move |cx| {
                copilot
                    .as_ref()
                    .is_some_and(|copilot| copilot.read(cx).is_authenticated())
            },
            copilot_ui::ConfigurationMode::EditPrediction,
            cx,
        )
    });

    Some(
        v_flex()
            .id("github-copilot")
            .min_w_0()
            .gap_1p5()
            .child(
                SettingsSectionHeader::new("GitHub Copilot")
                    .icon(IconName::Copilot)
                    .no_padding(true),
            )
            .child(configuration_view),
    )
}
