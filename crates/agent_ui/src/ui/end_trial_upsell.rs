use std::sync::Arc;

use ai_onboarding::{AgentPanelOnboardingCard, BulletItem};
use client::zed_urls;
use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};
use ui::{Divider, List, prelude::*};

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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pro_section = v_flex()
            .gap_1()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new("Pro")
                            .size(LabelSize::Small)
                            .color(Color::Accent)
                            .buffer_font(cx),
                    )
                    .child(Divider::horizontal()),
            )
            .child(
                List::new()
                    .child(BulletItem::new("500 prompts per month with Claude models"))
                    .child(BulletItem::new("Unlimited edit predictions")),
            )
            .child(
                Button::new("cta-button", "Upgrade to Zed Pro")
                    .full_width()
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .on_click(|_, _, cx| cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx))),
            );

        let free_section = v_flex()
            .mt_1p5()
            .gap_1()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new("Free")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .buffer_font(cx),
                    )
                    .child(Divider::horizontal()),
            )
            .child(
                List::new()
                    .child(BulletItem::new(
                        "50 prompts per month with the Claude models",
                    ))
                    .child(BulletItem::new(
                        "2000 accepted edit predictions using our open-source Zeta model",
                    )),
            )
            .child(
                Button::new("dismiss-button", "Stay on Free")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click({
                        let callback = self.dismiss_upsell.clone();
                        move |_, window, cx| callback(window, cx)
                    }),
            );

        AgentPanelOnboardingCard::new()
            .child(Headline::new("Your Zed Pro trial has expired."))
            .child(
                Label::new("You've been automatically reset to the Free plan.")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mb_1(),
            )
            .child(pro_section)
            .child(free_section)
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
