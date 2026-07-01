use std::sync::Arc;

use gpui::{AnyView, ScrollHandle, prelude::*};
use language_model::{
    ApiKeyConfiguration, ConfigurationViewTargetAgent, IconOrSvg, InlineDescription,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelRegistry,
    ProviderConfigurationView,
};
use ui::{ButtonLink, ConfiguredApiCard, Divider, DividerColor, prelude::*};

use crate::SettingsWindow;
use crate::components::SettingsInputField;

pub(crate) fn render_llm_providers_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let providers = LanguageModelRegistry::read_global(cx).visible_providers();

    v_flex()
        .id("llm-providers-page")
        .size_full()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .children(
            providers
                .iter()
                .enumerate()
                .map(|(index, provider)| {
                    render_provider_section(settings_window, provider, index == 0, window, cx)
                })
                .collect::<Vec<_>>(),
        )
        .into_any_element()
}

fn render_provider_section(
    settings_window: &SettingsWindow,
    provider: &Arc<dyn LanguageModelProvider>,
    is_first: bool,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let provider_id = provider.id();
    let provider_name = provider.name().0;

    let body = if let Some(config) = provider.api_key_configuration(cx) {
        render_api_key_providers_item(provider, provider_name.clone(), config)
    } else {
        match get_or_create_configuration_view(settings_window, &provider_id, provider, window, cx)
        {
            ProviderConfigurationView::Inline { view } => render_inline_body(provider, view, cx),
            ProviderConfigurationView::SubPage(_) => render_subpage_item(provider, cx),
        }
    };

    v_flex()
        .min_w_0()
        .map(|s| if is_first { s.pt_4() } else { s.pt_8() })
        .gap_1p5()
        .child(render_provider_header(provider_name, provider.icon(), cx))
        .child(body)
        .into_any_element()
}

/// An icon + name header with a faded divider, mirroring `SettingsSectionHeader`
/// but able to render providers' external SVG icons.
fn render_provider_header(
    provider_name: SharedString,
    icon: IconOrSvg,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let icon = match icon {
        IconOrSvg::Svg(path) => Icon::from_external_svg(path),
        IconOrSvg::Icon(name) => Icon::new(name),
    }
    .color(Color::Muted);

    v_flex()
        .w_full()
        .gap_1p5()
        .child(
            h_flex().gap_1p5().child(icon).child(
                Label::new(provider_name)
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .buffer_font(cx),
            ),
        )
        .child(Divider::horizontal().color(DividerColor::BorderFaded))
}

fn render_api_key_providers_item(
    provider: &Arc<dyn LanguageModelProvider>,
    provider_name: SharedString,
    config: ApiKeyConfiguration,
) -> AnyElement {
    let ApiKeyConfiguration {
        has_key,
        is_from_env_var,
        env_var_name,
        api_key_url,
    } = config;

    if has_key {
        let configured_label = if is_from_env_var {
            "API Key Set in Environment Variable"
        } else {
            "API Key Configured"
        };
        let button_id = format!("reset-api-key-{}", provider.id().0);
        let provider = provider.clone();
        let env_var_name_for_tooltip = env_var_name;

        return ConfiguredApiCard::new(button_id, configured_label)
            .button_label("Reset Key")
            .button_tab_index(0)
            .disabled(is_from_env_var)
            .when(is_from_env_var, |this| {
                this.tooltip_label(format!(
                    "To reset your API key, unset the {env_var_name_for_tooltip} environment variable."
                ))
            })
            .on_click(move |_, _, cx| {
                provider.reset_credentials(cx).detach_and_log_err(cx);
            })
            .into_any_element();
    }

    let input_id = format!("{}-api-key-input", provider.id().0);
    let aria_label = format!("{provider_name} API Key");
    let provider = provider.clone();

    h_flex()
        .pt_2p5()
        .w_full()
        .min_w_0()
        .gap_4()
        .justify_between()
        .child(
            v_flex()
                .w_full()
                .min_w_0()
                .max_w_1_2()
                .gap_0p5()
                .child(Label::new("API Key"))
                .child(
                    h_flex()
                        .w_full()
                        .min_w_0()
                        .flex_wrap()
                        .gap_0p5()
                        .child(
                            Label::new("Visit the")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            ButtonLink::new(format!("{provider_name} dashboard"), api_key_url)
                                .no_icon(true)
                                .label_size(LabelSize::Small)
                                .label_color(Color::Muted),
                        )
                        .child(
                            Label::new("to generate an API key.")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child(
                    Label::new(format!(
                        "Or set the {env_var_name} env var and restart Zed for it to take effect."
                    ))
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
                ),
        )
        .child(
            SettingsInputField::new(input_id)
                .tab_index(0)
                .with_placeholder("xxxxxxxxxxxxxxxxxxxx")
                .aria_label(aria_label)
                .on_confirm(move |api_key, _window, cx| {
                    if let Some(key) = api_key.filter(|key| !key.is_empty()) {
                        provider.set_api_key(key, cx).detach_and_log_err(cx);
                    }
                }),
        )
        .into_any_element()
}

fn render_inline_body(
    provider: &Arc<dyn LanguageModelProvider>,
    view: AnyView,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let provider_name = provider.name().0;
    let title = provider.inline_title(cx);
    let description = provider.inline_description(cx);

    if title.is_none() && description.is_none() {
        return v_flex()
            .pt_1()
            .w_full()
            .min_w_0()
            .child(view)
            .into_any_element();
    }

    h_flex()
        .pt_2p5()
        .w_full()
        .min_w_0()
        .gap_4()
        .justify_between()
        .child(
            v_flex()
                .w_full()
                .min_w_0()
                .max_w_1_2()
                .when_some(title, |this, title| this.child(Label::new(title)))
                .when_some(description, |this, description| {
                    this.child(render_inline_description(provider_name, description))
                }),
        )
        .child(h_flex().flex_none().child(view))
        .into_any_element()
}

fn render_subpage_item(
    provider: &Arc<dyn LanguageModelProvider>,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let provider_id = provider.id();
    let provider_name = provider.name().0;
    let description = provider.inline_description(cx);

    h_flex()
        .pt_2p5()
        .w_full()
        .min_w_0()
        .gap_4()
        .justify_between()
        .child(
            v_flex()
                .w_full()
                .min_w_0()
                .max_w_1_2()
                .gap_0p5()
                .child(Label::new("Configure Provider"))
                .when_some(description, |this, description| {
                    this.child(render_inline_description(provider_name, description))
                }),
        )
        .child(
            Button::new(format!("configure-{}", provider_id.0), "Configure")
                .style(ButtonStyle::OutlinedGhost)
                .size(ButtonSize::Medium)
                .end_icon(
                    Icon::new(IconName::ChevronRight)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .tab_index(0isize)
                .on_click(cx.listener(move |this, _, window, cx| {
                    open_provider_configuration(this, provider_id.clone(), window, cx);
                })),
        )
        .into_any_element()
}

fn render_inline_description(
    provider_name: SharedString,
    description: InlineDescription,
) -> AnyElement {
    match description {
        InlineDescription::ApiKeyUrl(url) => h_flex()
            .gap_0p5()
            .child(
                Label::new("To find an API key, visit the")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                ButtonLink::new(format!("{provider_name} dashboard."), url)
                    .label_size(LabelSize::Small),
            )
            .into_any_element(),
        InlineDescription::Text(text) => Label::new(text)
            .size(LabelSize::Small)
            .color(Color::Muted)
            .into_any_element(),
    }
}

fn open_provider_configuration(
    settings_window: &mut SettingsWindow,
    provider_id: LanguageModelProviderId,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) {
    let title = LanguageModelRegistry::read_global(cx)
        .provider(&provider_id)
        .map(|provider| provider.name().0)
        .unwrap_or_else(|| provider_id.0.clone());

    settings_window.configuring_provider = Some(provider_id);

    settings_window.push_dynamic_sub_page(
        title,
        "Agent Configuration",
        Some("llm_providers"),
        false,
        render_provider_config_sub_page,
        window,
        cx,
    );
}

fn render_provider_config_sub_page(
    settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let Some(provider_id) = settings_window.configuring_provider.clone() else {
        return div().into_any_element();
    };
    let Some(provider) = LanguageModelRegistry::read_global(cx).provider(&provider_id) else {
        return div().into_any_element();
    };

    // A provider routed to a sub-page always provides a `SubPage` view; fall
    // back to whatever view it returns otherwise.
    let view = match get_or_create_configuration_view(
        settings_window,
        &provider_id,
        &provider,
        window,
        cx,
    ) {
        ProviderConfigurationView::Inline { view, .. }
        | ProviderConfigurationView::SubPage(view) => view,
    };

    v_flex()
        .id("provider-config-sub-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(view)
        .into_any_element()
}

fn get_or_create_configuration_view(
    settings_window: &SettingsWindow,
    provider_id: &LanguageModelProviderId,
    provider: &Arc<dyn LanguageModelProvider>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> ProviderConfigurationView {
    if let Some(view) = settings_window
        .provider_configuration_views
        .get(provider_id)
    {
        return view.clone();
    }

    let view = provider.configuration_view_v2(ConfigurationViewTargetAgent::ZedAgent, window, cx);

    // Store the view for future renders by deferring a mutation
    let provider_id = provider_id.clone();
    let view_clone = view.clone();
    cx.defer_in(window, move |this, _window, _cx| {
        this.provider_configuration_views
            .insert(provider_id, view_clone);
    });

    view
}
