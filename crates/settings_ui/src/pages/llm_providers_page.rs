use std::sync::Arc;

use gpui::{ScrollHandle, prelude::*};
use language_model::{
    ConfigurationViewTargetAgent, IconOrSvg, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelRegistry, ProviderConfigurationView,
};
use ui::{ButtonLink, Divider, Tooltip, prelude::*};

use crate::SettingsWindow;

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
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .children(
            providers
                .iter()
                .enumerate()
                .map(|(index, provider)| {
                    v_flex()
                        .when(index > 0, |this| this.child(Divider::horizontal()))
                        .child(render_provider_row(settings_window, provider, window, cx))
                })
                .collect::<Vec<_>>(),
        )
        .into_any_element()
}

fn render_provider_row(
    settings_window: &SettingsWindow,
    provider: &Arc<dyn LanguageModelProvider>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let provider_id = provider.id();
    let provider_name = provider.name().0;
    let is_authenticated = provider.is_authenticated(cx);

    let icon = match provider.icon() {
        IconOrSvg::Svg(path) => Icon::from_external_svg(path),
        IconOrSvg::Icon(name) => Icon::new(name),
    }
    .size(IconSize::Small)
    .color(Color::Muted);

    let (control, api_key_url) =
        match get_or_create_configuration_view(settings_window, &provider_id, provider, window, cx)
        {
            ProviderConfigurationView::Inline { view, api_key_url } => {
                let control = view.into_any_element();
                (control, api_key_url)
            }
            ProviderConfigurationView::SubPage(_) => {
                let provider_id = provider_id.clone();
                let control = Button::new(format!("configure-{}", provider_id.0), "Configure")
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
                    }))
                    .into_any_element();
                (control, None)
            }
        };

    let left = v_flex()
        .flex_none()
        .gap_0p5()
        .child(
            h_flex()
                .gap_1p5()
                .child(icon)
                .child(Label::new(&provider_name)),
        )
        .when_some(api_key_url, |this, url| {
            this.child(render_where_to_find_key(provider_name, url))
        });

    h_flex()
        .min_w_0()
        .w_full()
        .py_4()
        .gap_4()
        .justify_between()
        .child(left)
        .child(control)
        .into_any_element()
}

fn render_where_to_find_key(provider_name: SharedString, url: SharedString) -> impl IntoElement {
    h_flex()
        .gap_0p5()
        .child(
            Label::new(format!("To find an API key, visit the"))
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(
            ButtonLink::new(format!("{provider_name} dashboard."), url)
                .label_size(LabelSize::Small),
        )
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
