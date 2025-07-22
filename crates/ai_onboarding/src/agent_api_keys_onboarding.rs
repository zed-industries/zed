use gpui::{Action, IntoElement, ParentElement, RenderOnce, point};
use language_model::{LanguageModelRegistry, ZED_CLOUD_PROVIDER_ID};
use ui::{Divider, List, prelude::*};

use crate::BulletItem;

pub struct ApiKeysWithProviders {
    configured_providers: Vec<(IconName, SharedString)>,
}

impl ApiKeysWithProviders {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.subscribe(
            &LanguageModelRegistry::global(cx),
            |this: &mut Self, _registry, event: &language_model::Event, cx| match event {
                language_model::Event::ProviderStateChanged
                | language_model::Event::AddedProvider(_)
                | language_model::Event::RemovedProvider(_) => {
                    this.configured_providers = Self::compute_configured_providers(cx)
                }
                _ => {}
            },
        )
        .detach();

        Self {
            configured_providers: Self::compute_configured_providers(cx),
        }
    }

    fn compute_configured_providers(cx: &App) -> Vec<(IconName, SharedString)> {
        LanguageModelRegistry::read_global(cx)
            .providers()
            .iter()
            .filter(|provider| {
                provider.is_authenticated(cx) && provider.id() != ZED_CLOUD_PROVIDER_ID
            })
            .map(|provider| (provider.icon(), provider.name().0.clone()))
            .collect()
    }

    pub fn has_providers(&self) -> bool {
        !self.configured_providers.is_empty()
    }
}

impl Render for ApiKeysWithProviders {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let configured_providers_list =
            self.configured_providers
                .iter()
                .cloned()
                .map(|(icon, name)| {
                    h_flex()
                        .gap_1p5()
                        .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                        .child(Label::new(name))
                });

        v_flex()
            .mx_2p5()
            .mb_2()
            .child(
                h_flex()
                    .p_3()
                    .gap_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().panel_background)
                    .shadow(vec![gpui::BoxShadow {
                        color: gpui::black().opacity(0.15),
                        offset: point(px(1.), px(-1.)),
                        blur_radius: px(3.),
                        spread_radius: px(0.),
                    }])
                    .child(
                        div()
                            .flex_shrink_0()
                            .pt_0p5()
                            .child(
                                Icon::new(IconName::Info)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted)
                            )
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .flex_1()
                            .min_w_0()
                            .child(
                                Label::new("Or start now using API keys from your environment for the following providers:")
                                    .color(Color::Muted)
                            )
                            .when(!self.configured_providers.is_empty(), |this| {
                                this.child(
                                    h_flex()
                                        .gap_2()
                                        .flex_wrap()
                                        .children(configured_providers_list)
                                )
                            })
                    )
            )
    }
}

#[derive(IntoElement)]
pub struct ApiKeysWithoutProviders;

impl ApiKeysWithoutProviders {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ApiKeysWithoutProviders {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .mt_2()
            .gap_1()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new("API Keys")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .buffer_font(cx),
                    )
                    .child(Divider::horizontal()),
            )
            .child(List::new().child(BulletItem::new(
                "You can also use AI in Zed by bringing your own API keys",
            )))
            .child(
                Button::new("configure-providers", "Configure Providers")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click(move |_, window, cx| {
                        window.dispatch_action(
                            zed_actions::agent::OpenConfiguration.boxed_clone(),
                            cx,
                        );
                    }),
            )
    }
}
