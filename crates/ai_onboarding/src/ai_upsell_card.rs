use std::sync::Arc;

use client::{Client, zed_urls};
use cloud_llm_client::Plan;
use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};
use ui::{Divider, List, Vector, VectorName, prelude::*};

use crate::{BulletItem, SignInStatus};

#[derive(IntoElement, RegisterComponent)]
pub struct AiUpsellCard {
    pub sign_in_status: SignInStatus,
    pub sign_in: Arc<dyn Fn(&mut Window, &mut App)>,
    pub user_plan: Option<Plan>,
}

impl AiUpsellCard {
    pub fn new(client: Arc<Client>, user_plan: Option<Plan>) -> Self {
        let status = *client.status().borrow();

        Self {
            user_plan,
            sign_in_status: status.into(),
            sign_in: Arc::new(move |_window, cx| {
                cx.spawn({
                    let client = client.clone();
                    async move |cx| client.sign_in_with_optional_connect(true, cx).await
                })
                .detach_and_log_err(cx);
            }),
        }
    }
}

impl RenderOnce for AiUpsellCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pro_section = v_flex()
            .flex_grow()
            .w_full()
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
                    .child(BulletItem::new("500 prompts with Claude models"))
                    .child(BulletItem::new(
                        "Unlimited edit predictions with Zeta, our open-source model",
                    )),
            );

        let free_section = v_flex()
            .flex_grow()
            .w_full()
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
                    .child(BulletItem::new("50 prompts with Claude models"))
                    .child(BulletItem::new("2,000 accepted edit predictions")),
            );

        let grid_bg = h_flex().absolute().inset_0().w_full().h(px(240.)).child(
            Vector::new(VectorName::Grid, rems_from_px(500.), rems_from_px(240.))
                .color(Color::Custom(cx.theme().colors().border.opacity(0.05))),
        );

        let gradient_bg = div()
            .absolute()
            .inset_0()
            .size_full()
            .bg(gpui::linear_gradient(
                180.,
                gpui::linear_color_stop(
                    cx.theme().colors().elevated_surface_background.opacity(0.8),
                    0.,
                ),
                gpui::linear_color_stop(
                    cx.theme().colors().elevated_surface_background.opacity(0.),
                    0.8,
                ),
            ));

        const DESCRIPTION: &str = "Zed offers a complete agentic experience, with robust editing and reviewing features to collaborate with AI.";

        let footer_buttons = match self.sign_in_status {
            SignInStatus::SignedIn => v_flex()
                .items_center()
                .gap_1()
                .child(
                    Button::new("sign_in", "Start 14-day Free Pro Trial")
                        .full_width()
                        .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                        .on_click(move |_, _window, cx| {
                            telemetry::event!("Start Trial Clicked", state = "post-sign-in");
                            cx.open_url(&zed_urls::start_trial_url(cx))
                        }),
                )
                .child(
                    Label::new("No credit card required")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
            _ => Button::new("sign_in", "Sign In")
                .full_width()
                .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                .on_click({
                    let callback = self.sign_in.clone();
                    move |_, window, cx| {
                        telemetry::event!("Start Trial Clicked", state = "pre-sign-in");
                        callback(window, cx)
                    }
                })
                .into_any_element(),
        };

        v_flex()
            .relative()
            .p_4()
            .pt_3()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_lg()
            .overflow_hidden()
            .child(grid_bg)
            .child(gradient_bg)
            .child(Label::new("Try Zed AI").size(LabelSize::Large))
            .child(
                div()
                    .max_w_3_4()
                    .mb_2()
                    .child(Label::new(DESCRIPTION).color(Color::Muted)),
            )
            .child(
                h_flex()
                    .w_full()
                    .mt_1p5()
                    .mb_2p5()
                    .items_start()
                    .gap_6()
                    .child(free_section)
                    .child(pro_section),
            )
            .child(footer_buttons)
    }
}

impl Component for AiUpsellCard {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn name() -> &'static str {
        "AI Upsell Card"
    }

    fn sort_name() -> &'static str {
        "AI Upsell Card"
    }

    fn description() -> Option<&'static str> {
        Some("A card presenting the Zed AI product during user's first-open onboarding flow.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .p_4()
                .gap_4()
                .children(vec![example_group(vec![
                    single_example(
                        "Signed Out State",
                        AiUpsellCard {
                            sign_in_status: SignInStatus::SignedOut,
                            sign_in: Arc::new(|_, _| {}),
                            user_plan: None,
                        }
                        .into_any_element(),
                    ),
                    single_example(
                        "Signed In State",
                        AiUpsellCard {
                            sign_in_status: SignInStatus::SignedIn,
                            sign_in: Arc::new(|_, _| {}),
                            user_plan: None,
                        }
                        .into_any_element(),
                    ),
                ])])
                .into_any_element(),
        )
    }
}
