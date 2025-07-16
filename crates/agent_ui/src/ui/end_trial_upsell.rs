use std::sync::Arc;

use ai_onboarding::AgentPanelOnboardingCard;
use client::zed_urls;
use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};
use ui::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct EndTrialUpsell {
    dismiss_upsell: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl EndTrialUpsell {
    pub fn new(dismiss_upsell: Arc<dyn Fn(&mut Window, &mut App)>) -> Self {
        Self { dismiss_upsell }
    }
}

impl RenderOnce for EndTrialUpsell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        AgentPanelOnboardingCard::new()
            .child(Headline::new("Your Zed Pro trial has expired."))
            .child(
                Label::new("You've been automatically reset to the Free plan.")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mb_1(),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Button::new("cta-button", "Upgrade to Zed Pro")
                            .full_width()
                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .on_click(|_, _, cx| cx.open_url(&zed_urls::account_url(cx))),
                    )
                    .child(
                        Button::new("dismiss-button", "Stay on Free")
                            .full_width()
                            .on_click({
                                let callback = self.dismiss_upsell.clone();
                                move |_, window, cx| callback(window, cx)
                            }),
                    ),
            )
    }
}

impl Component for EndTrialUpsell {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn sort_name() -> &'static str {
        "AgentEndTrialUpsell"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .p_4()
                .gap_4()
                .child(EndTrialUpsell {
                    dismiss_upsell: Arc::new(|_, _| {}),
                })
                .into_any_element(),
        )
    }
}
