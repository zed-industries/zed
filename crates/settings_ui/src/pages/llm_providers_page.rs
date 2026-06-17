use std::sync::Arc;

use gpui::{ScrollHandle, prelude::*};
use language_model::{
    ConfigurationViewTargetAgent, IconOrSvg, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelRegistry, ProviderConfigurationView,
};
use ui::{Divider, DividerColor, prelude::*};

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
                .map(|provider| render_provider_row(settings_window, provider, window, cx))
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

    let left = h_flex()
        .flex_none()
        .gap_1p5()
        .child(icon)
        .child(Label::new(provider_name))
        .when(is_authenticated, |this| {
            this.child(
                Icon::new(IconName::Check)
                    .size(IconSize::Small)
                    .color(Color::Success),
            )
        });

    // The provider tells us how it wants to be presented: a compact inline
    // control, or a richer view that belongs on its own sub-page.
    let control =
        match get_or_create_configuration_view(settings_window, &provider_id, provider, window, cx)
        {
            ProviderConfigurationView::Inline(view) => v_flex()
                .min_w_0()
                .w_full()
                .max_w(rems(24.))
                .child(view)
                .into_any_element(),
            ProviderConfigurationView::SubPage(_) => render_configure_button(&provider_id, cx),
        };

    v_flex()
        .min_w_0()
        .w_full()
        .child(
            div()
                .px_2()
                .child(Divider::horizontal().color(DividerColor::BorderFaded)),
        )
        .child(
            h_flex()
                .w_full()
                .py_2()
                .px_2()
                .gap_6()
                .justify_between()
                .items_start()
                .child(left)
                .child(control),
        )
        .into_any_element()
}

fn render_configure_button(
    provider_id: &LanguageModelProviderId,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let provider_id = provider_id.clone();
    Button::new(
        SharedString::from(format!("configure-{}", provider_id.0)),
        "Configure",
    )
    .style(ButtonStyle::Outlined)
    .label_size(LabelSize::Small)
    .tab_index(0isize)
    .on_click(cx.listener(move |this, _, window, cx| {
        open_provider_configuration(this, provider_id.clone(), window, cx);
    }))
    .into_any_element()
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
        ProviderConfigurationView::Inline(view) | ProviderConfigurationView::SubPage(view) => view,
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
