use std::sync::Arc;

use gpui::{AnyView, ScrollHandle, prelude::*};
use language_model::{
    ConfigurationViewTargetAgent, IconOrSvg, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelRegistry,
};
use ui::{Disclosure, Divider, DividerColor, prelude::*};

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
                .map(|provider| {
                    render_provider_block(settings_window, provider, window, cx)
                })
                .collect::<Vec<_>>(),
        )
        .into_any_element()
}

fn render_provider_block(
    settings_window: &SettingsWindow,
    provider: &Arc<dyn LanguageModelProvider>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let provider_id = provider.id();
    let provider_name = provider.name().0.clone();
    let disclosure_id = SharedString::from(format!("provider-disclosure-{}", provider_id.0));

    let is_expanded = settings_window
        .expanded_provider_configurations
        .get(&provider_id)
        .copied()
        .unwrap_or(false);

    let configuration_view = if is_expanded {
        Some(get_or_create_configuration_view(
            settings_window,
            &provider_id,
            provider,
            window,
            cx,
        ))
    } else {
        None
    };

    let is_authenticated = provider.is_authenticated(cx);

    v_flex()
        .min_w_0()
        .w_full()
        .when(is_expanded, |this| this.mb_2())
        .child(
            div()
                .px_2()
                .child(Divider::horizontal().color(DividerColor::BorderFaded)),
        )
        .child(
            h_flex()
                .map(|this| {
                    if is_expanded {
                        this.mt_2().mb_1()
                    } else {
                        this.my_2()
                    }
                })
                .w_full()
                .justify_between()
                .child(
                    h_flex()
                        .id(disclosure_id.clone())
                        .px_2()
                        .py_0p5()
                        .w_full()
                        .justify_between()
                        .rounded_sm()
                        .hover(|hover| hover.bg(cx.theme().colors().element_hover))
                        .child(
                            h_flex()
                                .w_full()
                                .gap_1p5()
                                .child(
                                    match provider.icon() {
                                        IconOrSvg::Svg(path) => Icon::from_external_svg(path),
                                        IconOrSvg::Icon(name) => Icon::new(name),
                                    }
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                                )
                                .child(
                                    h_flex()
                                        .w_full()
                                        .gap_1()
                                        .child(Label::new(provider_name.clone()))
                                        .when(is_authenticated && !is_expanded, |this| {
                                            this.child(
                                                Icon::new(IconName::Check).color(Color::Success),
                                            )
                                        }),
                                ),
                        )
                        .child(
                            Disclosure::new(disclosure_id, is_expanded)
                                .opened_icon(IconName::ChevronUp)
                                .closed_icon(IconName::ChevronDown),
                        )
                        .on_click(cx.listener({
                            let provider_id = provider_id.clone();
                            move |this, _event, _window, _cx| {
                                let is_expanded = this
                                    .expanded_provider_configurations
                                    .entry(provider_id.clone())
                                    .or_insert(false);
                                *is_expanded = !*is_expanded;
                            }
                        })),
                ),
        )
        .child(
            v_flex()
                .min_w_0()
                .w_full()
                .px_2()
                .gap_1()
                .when_some(configuration_view, |this, view| this.child(view)),
        )
        .into_any_element()
}

fn get_or_create_configuration_view(
    settings_window: &SettingsWindow,
    provider_id: &LanguageModelProviderId,
    provider: &Arc<dyn LanguageModelProvider>,
    window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyView {
    if let Some(view) = settings_window.provider_configuration_views.get(provider_id) {
        return view.clone();
    }

    let view = provider.configuration_view(
        ConfigurationViewTargetAgent::ZedAgent,
        window,
        cx,
    );

    // Store the view for future renders by deferring a mutation
    let provider_id = provider_id.clone();
    let view_clone = view.clone();
    cx.defer_in(window, move |this, _window, _cx| {
        this.provider_configuration_views
            .insert(provider_id, view_clone);
    });

    view
}
