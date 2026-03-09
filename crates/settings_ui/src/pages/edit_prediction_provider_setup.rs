use edit_prediction::{
    ApiKeyState,
    open_ai_compatible::{open_ai_compatible_api_token, open_ai_compatible_api_url},
};
use edit_prediction_ui::{get_available_providers, set_completion_provider};
use gpui::{Entity, ScrollHandle, prelude::*};
use language::language_settings::AllLanguageSettings;

use settings::Settings as _;
use ui::{ConfiguredApiCard, ContextMenu, DropdownMenu, DropdownStyle, prelude::*};
use workspace::AppState;

const OPENAI_COMPATIBLE_API_URL_PLACEHOLDER: &str = "https://api.openai.com/v1/completions";
const OPENAI_COMPATIBLE_MODEL_PLACEHOLDER: &str = "gpt-5.1-mini";

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
    let providers = [
        Some(render_provider_dropdown(window, cx)),
        render_github_copilot_provider(window, cx).map(IntoElement::into_any_element),
        Some(
            render_api_key_provider(
                IconName::AiOpenAiCompat,
                "OpenAI Compatible API",
                ApiKeyDocs::Custom {
                    message: "Set an API key here. It will be sent as Authorization: Bearer {key}."
                        .into(),
                },
                open_ai_compatible_api_token(cx),
                |cx| open_ai_compatible_api_url(cx),
                Some(
                    settings_window
                        .render_sub_page_items_section(
                            open_ai_compatible_settings().iter().enumerate(),
                            true,
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

fn render_github_copilot_provider(window: &mut Window, cx: &mut App) -> Option<impl IntoElement> {
    let configuration_view = window.use_state(cx, |_, cx| {
        copilot_ui::ConfigurationView::new(
            move |cx| {
                if let Some(app_state) = AppState::global(cx).upgrade() {
                    copilot::GlobalCopilotAuth::try_get_or_init(app_state, cx)
                        .is_some_and(|copilot| copilot.0.read(cx).is_authenticated())
                } else {
                    false
                }
            },
            copilot_ui::ConfigurationMode::EditPrediction,
            cx,
        )
    });

    Some(
        v_flex()
            .id("github-copilot")
            .min_w_0()
            .pt_8()
            .gap_1p5()
            .child(
                SettingsSectionHeader::new("GitHub Copilot")
                    .icon(IconName::Copilot)
                    .no_padding(true),
            )
            .child(configuration_view),
    )
}

fn render_provider_dropdown(window: &mut Window, cx: &mut App) -> AnyElement {
    let current_provider = AllLanguageSettings::get_global(cx)
        .edit_predictions
        .provider;
    let available_providers = get_available_providers(cx);
    let current_provider_name = if available_providers.contains(&current_provider) {
        current_provider.display_name().unwrap_or("No provider set")
    } else {
        "No provider set"
    };

    let menu = ContextMenu::build(window, cx, move |mut menu, _, cx| {
        let fs = <dyn fs::Fs>::global(cx);

        for provider in get_available_providers(cx) {
            let Some(name) = provider.display_name() else {
                continue;
            };
            let is_current = provider == current_provider;

            menu = menu.toggleable_entry(name, is_current, IconPosition::Start, None, {
                let fs = fs.clone();
                move |_, cx| {
                    set_completion_provider(fs.clone(), cx, provider);
                }
            });
        }
        menu
    });

    v_flex()
        .id("provider-selector")
        .min_w_0()
        .gap_1p5()
        .child(SettingsSectionHeader::new("Active Provider").no_padding(true))
        .child(
            h_flex()
                .pt_2p5()
                .w_full()
                .justify_between()
                .child(
                    v_flex()
                        .w_full()
                        .max_w_1_2()
                        .child(Label::new("Provider"))
                        .child(
                            Label::new("Select which provider to use for edit predictions.")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child(
                    DropdownMenu::new("provider-dropdown", current_provider_name, menu)
                        .tab_index(0)
                        .style(DropdownStyle::Outlined),
                ),
        )
        .into_any_element()
}

enum ApiKeyDocs {
    Custom { message: SharedString },
}

fn render_api_key_provider(
    icon: IconName,
    title: &'static str,
    docs: ApiKeyDocs,
    api_key_state: Entity<ApiKeyState>,
    current_url: fn(&mut App) -> SharedString,
    additional_fields: Option<AnyElement>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let weak_page = cx.weak_entity();
    _ = window.use_keyed_state(current_url(cx), cx, |_, cx| {
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
    let description = match docs {
        ApiKeyDocs::Custom { message } => h_flex().min_w_0().gap_0p5().child(
            Label::new(message)
                .size(LabelSize::Small)
                .color(Color::Muted),
        ),
    };
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

fn open_ai_compatible_settings() -> Box<[SettingsPageItem]> {
    Box::new([
        SettingsPageItem::SettingItem(SettingItem {
            title: "API URL",
            description: "The URL of your OpenAI-compatible server's completions API.",
            field: Box::new(SettingField {
                pick: |settings| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .as_ref()?
                        .open_ai_compatible_api
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
                        .open_ai_compatible_api
                        .get_or_insert_default()
                        .api_url = value;
                },
                json_path: Some("edit_predictions.open_ai_compatible_api.api_url"),
            }),
            metadata: Some(Box::new(SettingsFieldMetadata {
                placeholder: Some(OPENAI_COMPATIBLE_API_URL_PLACEHOLDER),
                ..Default::default()
            })),
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Model",
            description: "The model string to pass to the OpenAI-compatible server.",
            field: Box::new(SettingField {
                pick: |settings| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .as_ref()?
                        .open_ai_compatible_api
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
                        .open_ai_compatible_api
                        .get_or_insert_default()
                        .model = value;
                },
                json_path: Some("edit_predictions.open_ai_compatible_api.model"),
            }),
            metadata: Some(Box::new(SettingsFieldMetadata {
                placeholder: Some(OPENAI_COMPATIBLE_MODEL_PLACEHOLDER),
                ..Default::default()
            })),
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Prompt Format",
            description: "The prompt format to use when requesting predictions. Set to Infer to have the format inferred based on the model name",
            field: Box::new(SettingField {
                pick: |settings| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .as_ref()?
                        .open_ai_compatible_api
                        .as_ref()?
                        .prompt_format
                        .as_ref()
                },
                write: |settings, value| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .get_or_insert_default()
                        .open_ai_compatible_api
                        .get_or_insert_default()
                        .prompt_format = value;
                },
                json_path: Some("edit_predictions.open_ai_compatible_api.prompt_format"),
            }),
            files: USER,
            metadata: None,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Max Output Tokens",
            description: "The maximum number of tokens to generate.",
            field: Box::new(SettingField {
                pick: |settings| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .as_ref()?
                        .open_ai_compatible_api
                        .as_ref()?
                        .max_output_tokens
                        .as_ref()
                },
                write: |settings, value| {
                    settings
                        .project
                        .all_languages
                        .edit_predictions
                        .get_or_insert_default()
                        .open_ai_compatible_api
                        .get_or_insert_default()
                        .max_output_tokens = value;
                },
                json_path: Some("edit_predictions.open_ai_compatible_api.max_output_tokens"),
            }),
            metadata: None,
            files: USER,
        }),
    ])
}
