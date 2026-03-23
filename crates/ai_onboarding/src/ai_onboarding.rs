mod agent_api_keys_onboarding;
mod agent_panel_onboarding_card;
mod agent_panel_onboarding_content;
mod ai_upsell_card;
mod edit_prediction_onboarding_content;
mod plan_definitions;
mod young_account_banner;

pub use agent_api_keys_onboarding::{ApiKeysWithProviders, ApiKeysWithoutProviders};
pub use agent_panel_onboarding_card::AgentPanelOnboardingCard;
pub use agent_panel_onboarding_content::AgentPanelOnboarding;
pub use ai_upsell_card::AiUpsellCard;
use cloud_api_types::Plan;
pub use edit_prediction_onboarding_content::EditPredictionOnboarding;
pub use plan_definitions::PlanDefinitions;
pub use young_account_banner::YoungAccountBanner;

use std::sync::Arc;

use client::{Client, UserStore, zed_urls};
use gpui::{AnyElement, Entity, IntoElement, ParentElement};
use ui::{Divider, RegisterComponent, Tooltip, prelude::*};

#[derive(PartialEq)]
pub enum SignInStatus {
    SignedIn,
    SigningIn,
    SignedOut,
}

impl From<client::Status> for SignInStatus {
    fn from(status: client::Status) -> Self {
        if status.is_signing_in() {
            Self::SigningIn
        } else if status.is_signed_out() {
            Self::SignedOut
        } else {
            Self::SignedIn
        }
    }
}

#[derive(RegisterComponent, IntoElement)]
pub struct ZedAiOnboarding {
    pub sign_in_status: SignInStatus,
    pub plan: Option<Plan>,
    pub account_too_young: bool,
    pub continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
    pub sign_in: Arc<dyn Fn(&mut Window, &mut App)>,
    pub dismiss_onboarding: Option<Arc<dyn Fn(&mut Window, &mut App)>>,
}

impl ZedAiOnboarding {
    pub fn new(
        client: Arc<Client>,
        user_store: &Entity<UserStore>,
        continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
        cx: &mut App,
    ) -> Self {
        let store = user_store.read(cx);
        let status = *client.status().borrow();

        Self {
            sign_in_status: status.into(),
            plan: store.plan(),
            account_too_young: store.account_too_young(),
            continue_with_zed_ai,
            sign_in: Arc::new(move |_window, cx| {
                cx.spawn({
                    let client = client.clone();
                    async move |cx| client.sign_in_with_optional_connect(true, cx).await
                })
                .detach_and_log_err(cx);
            }),
            dismiss_onboarding: None,
        }
    }

    pub fn with_dismiss(
        mut self,
        dismiss_callback: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.dismiss_onboarding = Some(Arc::new(dismiss_callback));
        self
    }

    fn render_dismiss_button(&self) -> Option<AnyElement> {
        self.dismiss_onboarding.as_ref().map(|dismiss_callback| {
            let callback = dismiss_callback.clone();

            h_flex()
                .absolute()
                .top_0()
                .right_0()
                .child(
                    IconButton::new("dismiss_onboarding", IconName::Close)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text("Dismiss"))
                        .on_click(move |_, window, cx| {
                            telemetry::event!("Banner Dismissed", source = "AI Onboarding",);
                            callback(window, cx)
                        }),
                )
                .into_any_element()
        })
    }

    fn render_sign_in_disclaimer(&self, _cx: &mut App) -> AnyElement {
        let signing_in = matches!(self.sign_in_status, SignInStatus::SigningIn);

        v_flex()
            .relative()
            .gap_1()
            .child(Headline::new("Welcome to Zed AI"))
            .child(
                Label::new("Sign in to try Zed Pro for 14 days, no credit card required.")
                    .color(Color::Muted)
                    .mb_2(),
            )
            .child(PlanDefinitions.pro_plan())
            .child(
                Button::new("sign_in", "Try Zed Pro for Free")
                    .disabled(signing_in)
                    .full_width()
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .on_click({
                        let callback = self.sign_in.clone();
                        move |_, window, cx| {
                            telemetry::event!("Start Trial Clicked", state = "pre-sign-in");
                            callback(window, cx)
                        }
                    }),
            )
            .children(self.render_dismiss_button())
            .into_any_element()
    }

    fn render_free_plan_state(&self, cx: &mut App) -> AnyElement {
        if self.account_too_young {
            v_flex()
                .relative()
                .max_w_full()
                .gap_1()
                .child(Headline::new("Welcome to Zed AI"))
                .child(YoungAccountBanner)
                .child(
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
                        .child(PlanDefinitions.pro_plan())
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
                .into_any_element()
        } else {
            v_flex()
                .relative()
                .gap_1()
                .child(Headline::new("Welcome to Zed AI"))
                .child(
                    v_flex()
                        .mt_2()
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
                                .child(
                                    Label::new("(Current Plan)")
                                        .size(LabelSize::Small)
                                        .color(Color::Custom(
                                            cx.theme().colors().text_muted.opacity(0.6),
                                        ))
                                        .buffer_font(cx),
                                )
                                .child(Divider::horizontal()),
                        )
                        .child(PlanDefinitions.free_plan()),
                )
                .children(self.render_dismiss_button())
                .child(
                    v_flex()
                        .mt_2()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(
                                    Label::new("Pro Trial")
                                        .size(LabelSize::Small)
                                        .color(Color::Accent)
                                        .buffer_font(cx),
                                )
                                .child(Divider::horizontal()),
                        )
                        .child(PlanDefinitions.pro_trial(true))
                        .child(
                            Button::new("pro", "Start Free Trial")
                                .full_width()
                                .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                .on_click(move |_, _window, cx| {
                                    telemetry::event!(
                                        "Start Trial Clicked",
                                        state = "post-sign-in"
                                    );
                                    cx.open_url(&zed_urls::start_trial_url(cx))
                                }),
                        ),
                )
                .into_any_element()
        }
    }

    fn render_trial_state(&self, _cx: &mut App) -> AnyElement {
        v_flex()
            .relative()
            .gap_1()
            .child(Headline::new("Welcome to the Zed Pro Trial"))
            .child(
                Label::new("Here's what you get for the next 14 days:")
                    .color(Color::Muted)
                    .mb_2(),
            )
            .child(PlanDefinitions.pro_trial(false))
            .children(self.render_dismiss_button())
            .into_any_element()
    }

    fn render_pro_plan_state(&self, _cx: &mut App) -> AnyElement {
        v_flex()
            .gap_1()
            .child(Headline::new("Welcome to Zed Pro"))
            .child(
                Label::new("Here's what you get:")
                    .color(Color::Muted)
                    .mb_2(),
            )
            .child(PlanDefinitions.pro_plan())
            .children(self.render_dismiss_button())
            .into_any_element()
    }

    fn render_student_plan_state(&self, _cx: &mut App) -> AnyElement {
        v_flex()
            .gap_1()
            .child(Headline::new("Welcome to Zed Student"))
            .child(
                Label::new("Here's what you get:")
                    .color(Color::Muted)
                    .mb_2(),
            )
            .child(PlanDefinitions.student_plan())
            .children(self.render_dismiss_button())
            .into_any_element()
    }
}

impl RenderOnce for ZedAiOnboarding {
    fn render(self, _window: &mut ui::Window, cx: &mut App) -> impl IntoElement {
        if matches!(self.sign_in_status, SignInStatus::SignedIn) {
            match self.plan {
                None => self.render_free_plan_state(cx),
                Some(Plan::ZedFree) => self.render_free_plan_state(cx),
                Some(Plan::ZedProTrial) => self.render_trial_state(cx),
                Some(Plan::ZedPro) => self.render_pro_plan_state(cx),
                Some(Plan::ZedStudent) => self.render_student_plan_state(cx),
            }
        } else {
            self.render_sign_in_disclaimer(cx)
        }
    }
}

impl Component for ZedAiOnboarding {
    fn scope() -> ComponentScope {
        ComponentScope::Onboarding
    }

    fn name() -> &'static str {
        "Agent Panel Banners"
    }

    fn sort_name() -> &'static str {
        "Agent Panel Banners"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        fn onboarding(
            sign_in_status: SignInStatus,
            plan: Option<Plan>,
            account_too_young: bool,
        ) -> AnyElement {
            ZedAiOnboarding {
                sign_in_status,
                plan,
                account_too_young,
                continue_with_zed_ai: Arc::new(|_, _| {}),
                sign_in: Arc::new(|_, _| {}),
                dismiss_onboarding: None,
            }
            .into_any_element()
        }

        Some(
            v_flex()
                .gap_4()
                .items_center()
                .max_w_4_5()
                .children(vec![
                    single_example(
                        "Not Signed-in",
                        onboarding(SignInStatus::SignedOut, None, false),
                    ),
                    single_example(
                        "Young Account",
                        onboarding(SignInStatus::SignedIn, None, true),
                    ),
                    single_example(
                        "Free Plan",
                        onboarding(SignInStatus::SignedIn, Some(Plan::ZedFree), false),
                    ),
                    single_example(
                        "Pro Trial",
                        onboarding(SignInStatus::SignedIn, Some(Plan::ZedProTrial), false),
                    ),
                    single_example(
                        "Pro Plan",
                        onboarding(SignInStatus::SignedIn, Some(Plan::ZedPro), false),
                    ),
                ])
                .into_any_element(),
        )
    }
}
