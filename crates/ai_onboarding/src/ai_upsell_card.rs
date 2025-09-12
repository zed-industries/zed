use std::sync::Arc;

use client::{Client, UserStore, zed_urls};
use cloud_llm_client::{Plan, PlanV1, PlanV2};
use feature_flags::{BillingV2FeatureFlag, FeatureFlagAppExt};
use gpui::{AnyElement, App, Entity, IntoElement, RenderOnce, Window};
use ui::{CommonAnimationExt, Divider, Vector, VectorName, prelude::*};

use crate::{SignInStatus, YoungAccountBanner, plan_definitions::PlanDefinitions};

#[derive(IntoElement, RegisterComponent)]
pub struct AiUpsellCard {
    sign_in_status: SignInStatus,
    sign_in: Arc<dyn Fn(&mut Window, &mut App)>,
    account_too_young: bool,
    user_plan: Option<Plan>,
    tab_index: Option<isize>,
}

impl AiUpsellCard {
    pub fn new(
        client: Arc<Client>,
        user_store: &Entity<UserStore>,
        user_plan: Option<Plan>,
        cx: &mut App,
    ) -> Self {
        let status = *client.status().borrow();
        let store = user_store.read(cx);

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
            account_too_young: store.account_too_young(),
            tab_index: None,
        }
    }

    pub fn tab_index(mut self, tab_index: Option<isize>) -> Self {
        self.tab_index = tab_index;
        self
    }
}

impl RenderOnce for AiUpsellCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_v2_plan = self
            .user_plan
            .map_or(cx.has_flag::<BillingV2FeatureFlag>(), |plan| plan.is_v2());

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
            .child(PlanDefinitions.pro_plan(is_v2_plan, false));

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
            .child(PlanDefinitions.free_plan(is_v2_plan));

        let grid_bg = h_flex()
            .absolute()
            .inset_0()
            .w_full()
            .h(px(240.))
            .bg(gpui::pattern_slash(
                cx.theme().colors().border.opacity(0.1),
                2.,
                25.,
            ));

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

        let description = PlanDefinitions::AI_DESCRIPTION;

        let card = v_flex()
            .relative()
            .flex_grow()
            .p_4()
            .pt_3()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_lg()
            .overflow_hidden()
            .child(grid_bg)
            .child(gradient_bg);

        let plans_section = h_flex()
            .w_full()
            .mt_1p5()
            .mb_2p5()
            .items_start()
            .gap_6()
            .child(free_section)
            .child(pro_section);

        let footer_container = v_flex().items_center().gap_1();

        let certified_user_stamp = div()
            .absolute()
            .top_2()
            .right_2()
            .size(rems_from_px(72.))
            .child(
                Vector::new(
                    VectorName::ProUserStamp,
                    rems_from_px(72.),
                    rems_from_px(72.),
                )
                .color(Color::Custom(cx.theme().colors().text_accent.alpha(0.3)))
                .with_rotate_animation(10),
            );

        let pro_trial_stamp = div()
            .absolute()
            .top_2()
            .right_2()
            .size(rems_from_px(72.))
            .child(
                Vector::new(
                    VectorName::ProTrialStamp,
                    rems_from_px(72.),
                    rems_from_px(72.),
                )
                .color(Color::Custom(cx.theme().colors().text.alpha(0.2))),
            );

        match self.sign_in_status {
            SignInStatus::SignedIn => match self.user_plan {
                None | Some(Plan::V1(PlanV1::ZedFree) | Plan::V2(PlanV2::ZedFree)) => card
                    .child(Label::new("Try Zed AI").size(LabelSize::Large))
                    .map(|this| {
                        if self.account_too_young {
                            this.child(YoungAccountBanner).child(
                                v_flex()
                                    .mt_2()
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
                                    .child(PlanDefinitions.pro_plan(is_v2_plan, true))
                                    .child(
                                        Button::new("pro", "Get Started")
                                            .full_width()
                                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                            .on_click(move |_, _window, cx| {
                                                telemetry::event!(
                                                    "Upgrade To Pro Clicked",
                                                    state = "young-account"
                                                );
                                                cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx))
                                            }),
                                    ),
                            )
                        } else {
                            this.child(
                                div()
                                    .max_w_3_4()
                                    .mb_2()
                                    .child(Label::new(description).color(Color::Muted)),
                            )
                            .child(plans_section)
                            .child(
                                footer_container
                                    .child(
                                        Button::new("start_trial", "Start 14-day Free Pro Trial")
                                            .full_width()
                                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                            .when_some(self.tab_index, |this, tab_index| {
                                                this.tab_index(tab_index)
                                            })
                                            .on_click(move |_, _window, cx| {
                                                telemetry::event!(
                                                    "Start Trial Clicked",
                                                    state = "post-sign-in"
                                                );
                                                cx.open_url(&zed_urls::start_trial_url(cx))
                                            }),
                                    )
                                    .child(
                                        Label::new("No credit card required")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                        }
                    }),
                Some(plan @ (Plan::V1(PlanV1::ZedProTrial) | Plan::V2(PlanV2::ZedProTrial))) => {
                    card.child(pro_trial_stamp)
                        .child(Label::new("You're in the Zed Pro Trial").size(LabelSize::Large))
                        .child(
                            Label::new("Here's what you get for the next 14 days:")
                                .color(Color::Muted)
                                .mb_2(),
                        )
                        .child(PlanDefinitions.pro_trial(plan.is_v2(), false))
                }
                Some(plan @ (Plan::V1(PlanV1::ZedPro) | Plan::V2(PlanV2::ZedPro))) => card
                    .child(certified_user_stamp)
                    .child(Label::new("You're in the Zed Pro plan").size(LabelSize::Large))
                    .child(
                        Label::new("Here's what you get:")
                            .color(Color::Muted)
                            .mb_2(),
                    )
                    .child(PlanDefinitions.pro_plan(plan.is_v2(), false)),
            },
            // Signed Out State
            _ => card
                .child(Label::new("Try Zed AI").size(LabelSize::Large))
                .child(
                    div()
                        .max_w_3_4()
                        .mb_2()
                        .child(Label::new(description).color(Color::Muted)),
                )
                .child(plans_section)
                .child(
                    Button::new("sign_in", "Sign In")
                        .full_width()
                        .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                        .when_some(self.tab_index, |this, tab_index| this.tab_index(tab_index))
                        .on_click({
                            let callback = self.sign_in.clone();
                            move |_, window, cx| {
                                telemetry::event!("Start Trial Clicked", state = "pre-sign-in");
                                callback(window, cx)
                            }
                        }),
                ),
        }
    }
}

impl Component for AiUpsellCard {
    fn scope() -> ComponentScope {
        ComponentScope::Onboarding
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
                .gap_4()
                .items_center()
                .max_w_4_5()
                .child(single_example(
                    "Signed Out State",
                    AiUpsellCard {
                        sign_in_status: SignInStatus::SignedOut,
                        sign_in: Arc::new(|_, _| {}),
                        account_too_young: false,
                        user_plan: None,
                        tab_index: Some(0),
                    }
                    .into_any_element(),
                ))
                .child(example_group_with_title(
                    "Signed In States",
                    vec![
                        single_example(
                            "Free Plan",
                            AiUpsellCard {
                                sign_in_status: SignInStatus::SignedIn,
                                sign_in: Arc::new(|_, _| {}),
                                account_too_young: false,
                                user_plan: Some(Plan::V1(PlanV1::ZedFree)),
                                tab_index: Some(1),
                            }
                            .into_any_element(),
                        ),
                        single_example(
                            "Free Plan but Young Account",
                            AiUpsellCard {
                                sign_in_status: SignInStatus::SignedIn,
                                sign_in: Arc::new(|_, _| {}),
                                account_too_young: true,
                                user_plan: Some(Plan::V1(PlanV1::ZedFree)),
                                tab_index: Some(1),
                            }
                            .into_any_element(),
                        ),
                        single_example(
                            "Pro Trial",
                            AiUpsellCard {
                                sign_in_status: SignInStatus::SignedIn,
                                sign_in: Arc::new(|_, _| {}),
                                account_too_young: false,
                                user_plan: Some(Plan::V1(PlanV1::ZedProTrial)),
                                tab_index: Some(1),
                            }
                            .into_any_element(),
                        ),
                        single_example(
                            "Pro Plan",
                            AiUpsellCard {
                                sign_in_status: SignInStatus::SignedIn,
                                sign_in: Arc::new(|_, _| {}),
                                account_too_young: false,
                                user_plan: Some(Plan::V1(PlanV1::ZedPro)),
                                tab_index: Some(1),
                            }
                            .into_any_element(),
                        ),
                    ],
                ))
                .into_any_element(),
        )
    }
}
