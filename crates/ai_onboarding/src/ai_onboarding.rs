mod agent_api_keys_onboarding;
mod agent_panel_onboarding_card;
mod agent_panel_onboarding_content;
mod ai_upsell_card;
mod edit_prediction_onboarding_content;
mod young_account_banner;

pub use agent_api_keys_onboarding::{ApiKeysWithProviders, ApiKeysWithoutProviders};
pub use agent_panel_onboarding_card::AgentPanelOnboardingCard;
pub use agent_panel_onboarding_content::AgentPanelOnboarding;
pub use ai_upsell_card::AiUpsellCard;
use cloud_llm_client::Plan;
pub use edit_prediction_onboarding_content::EditPredictionOnboarding;
pub use young_account_banner::YoungAccountBanner;

use std::sync::Arc;

use client::{Client, UserStore, zed_urls};
use gpui::{AnyElement, Entity, IntoElement, ParentElement, SharedString};
use ui::{Divider, List, ListItem, RegisterComponent, TintColor, Tooltip, prelude::*};

#[derive(IntoElement)]
pub struct BulletItem {
    label: SharedString,
}

impl BulletItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl RenderOnce for BulletItem {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let line_height = 0.85 * window.line_height();

        ListItem::new("list-item")
            .selectable(false)
            .child(
                h_flex()
                    .w_full()
                    .min_w_0()
                    .gap_1()
                    .items_start()
                    .child(
                        h_flex().h(line_height).justify_center().child(
                            Icon::new(IconName::Dash)
                                .size(IconSize::XSmall)
                                .color(Color::Hidden),
                        ),
                    )
                    .child(div().w_full().min_w_0().child(Label::new(self.label))),
            )
            .into_any_element()
    }
}

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
    pub has_accepted_terms_of_service: bool,
    pub plan: Option<Plan>,
    pub account_too_young: bool,
    pub continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
    pub sign_in: Arc<dyn Fn(&mut Window, &mut App)>,
    pub accept_terms_of_service: Arc<dyn Fn(&mut Window, &mut App)>,
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
            has_accepted_terms_of_service: store.has_accepted_terms_of_service(),
            plan: store.plan(),
            account_too_young: store.account_too_young(),
            continue_with_zed_ai,
            accept_terms_of_service: Arc::new({
                let store = user_store.clone();
                move |_window, cx| {
                    let task = store.update(cx, |store, cx| store.accept_terms_of_service(cx));
                    task.detach_and_log_err(cx);
                }
            }),
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

    fn free_plan_definition(&self, cx: &mut App) -> impl IntoElement {
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
                            .color(Color::Custom(cx.theme().colors().text_muted.opacity(0.6)))
                            .buffer_font(cx),
                    )
                    .child(Divider::horizontal()),
            )
            .child(
                List::new()
                    .child(BulletItem::new("50 prompts per month with Claude models"))
                    .child(BulletItem::new(
                        "2,000 accepted edit predictions with Zeta, our open-source model",
                    )),
            )
    }

    fn pro_trial_definition(&self) -> impl IntoElement {
        List::new()
            .child(BulletItem::new("150 prompts with Claude models"))
            .child(BulletItem::new(
                "Unlimited accepted edit predictions with Zeta, our open-source model",
            ))
    }

    fn pro_plan_definition(&self, cx: &mut App) -> impl IntoElement {
        v_flex().mt_2().gap_1().map(|this| {
            if self.account_too_young {
                this.child(
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
                        .child(BulletItem::new(
                            "Unlimited accepted edit predictions with Zeta, our open-source model",
                        ))
                        .child(BulletItem::new("$20 USD per month")),
                )
                .child(
                    Button::new("pro", "Get Started")
                        .full_width()
                        .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                        .on_click(move |_, _window, cx| {
                            telemetry::event!("Upgrade To Pro Clicked", state = "young-account");
                            cx.open_url(&zed_urls::upgrade_to_zed_pro_url(cx))
                        }),
                )
            } else {
                this.child(
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
                .child(
                    List::new()
                        .child(self.pro_trial_definition())
                        .child(BulletItem::new(
                            "Try it out for 14 days for free, no credit card required",
                        )),
                )
                .child(
                    Button::new("pro", "Start Free Trial")
                        .full_width()
                        .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                        .on_click(move |_, _window, cx| {
                            telemetry::event!("Start Trial Clicked", state = "post-sign-in");
                            cx.open_url(&zed_urls::start_trial_url(cx))
                        }),
                )
            }
        })
    }

    fn render_accept_terms_of_service(&self) -> AnyElement {
        v_flex()
            .gap_1()
            .w_full()
            .child(Headline::new("Accept Terms of Service"))
            .child(
                Label::new("We don’t sell your data, track you across the web, or compromise your privacy.")
                    .color(Color::Muted)
                    .mb_2(),
            )
            .child(
                Button::new("terms_of_service", "Review Terms of Service")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .icon(IconName::ArrowUpRight)
                    .icon_color(Color::Muted)
                    .icon_size(IconSize::XSmall)
                    .on_click(move |_, _window, cx| {
                        telemetry::event!("Review Terms of Service Clicked");
                        cx.open_url(&zed_urls::terms_of_service(cx))
                    }),
            )
            .child(
                Button::new("accept_terms", "Accept")
                    .full_width()
                    .style(ButtonStyle::Tinted(TintColor::Accent))
                    .on_click({
                        let callback = self.accept_terms_of_service.clone();
                        move |_, window, cx| {
                            telemetry::event!("Terms of Service Accepted");
                            (callback)(window, cx)}
                    }),
            )
            .into_any_element()
    }

    fn render_sign_in_disclaimer(&self, _cx: &mut App) -> AnyElement {
        let signing_in = matches!(self.sign_in_status, SignInStatus::SigningIn);

        v_flex()
            .gap_1()
            .child(Headline::new("Welcome to Zed AI"))
            .child(
                Label::new("Sign in to try Zed Pro for 14 days, no credit card required.")
                    .color(Color::Muted)
                    .mb_2(),
            )
            .child(self.pro_trial_definition())
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
            .into_any_element()
    }

    fn render_free_plan_state(&self, cx: &mut App) -> AnyElement {
        let young_account_banner = YoungAccountBanner;

        v_flex()
            .relative()
            .gap_1()
            .child(Headline::new("Welcome to Zed AI"))
            .map(|this| {
                if self.account_too_young {
                    this.child(young_account_banner)
                } else {
                    this.child(self.free_plan_definition(cx)).when_some(
                        self.dismiss_onboarding.as_ref(),
                        |this, dismiss_callback| {
                            let callback = dismiss_callback.clone();

                            this.child(
                                h_flex().absolute().top_0().right_0().child(
                                    IconButton::new("dismiss_onboarding", IconName::Close)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text("Dismiss"))
                                        .on_click(move |_, window, cx| {
                                            telemetry::event!(
                                                "Banner Dismissed",
                                                source = "AI Onboarding",
                                            );
                                            callback(window, cx)
                                        }),
                                ),
                            )
                        },
                    )
                }
            })
            .child(self.pro_plan_definition(cx))
            .into_any_element()
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
            .child(
                List::new()
                    .child(BulletItem::new("150 prompts with Claude models"))
                    .child(BulletItem::new(
                        "Unlimited edit predictions with Zeta, our open-source model",
                    )),
            )
            .when_some(
                self.dismiss_onboarding.as_ref(),
                |this, dismiss_callback| {
                    let callback = dismiss_callback.clone();
                    this.child(
                        h_flex().absolute().top_0().right_0().child(
                            IconButton::new("dismiss_onboarding", IconName::Close)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Dismiss"))
                                .on_click(move |_, window, cx| {
                                    telemetry::event!(
                                        "Banner Dismissed",
                                        source = "AI Onboarding",
                                    );
                                    callback(window, cx)
                                }),
                        ),
                    )
                },
            )
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
            .child(
                List::new()
                    .child(BulletItem::new("500 prompts with Claude models"))
                    .child(BulletItem::new(
                        "Unlimited edit predictions with Zeta, our open-source model",
                    )),
            )
            .child(
                Button::new("pro", "Continue with Zed Pro")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click({
                        let callback = self.continue_with_zed_ai.clone();
                        move |_, window, cx| {
                            telemetry::event!("Banner Dismissed", source = "AI Onboarding");
                            callback(window, cx)
                        }
                    }),
            )
            .into_any_element()
    }
}

impl RenderOnce for ZedAiOnboarding {
    fn render(self, _window: &mut ui::Window, cx: &mut App) -> impl IntoElement {
        if matches!(self.sign_in_status, SignInStatus::SignedIn) {
            if self.has_accepted_terms_of_service {
                match self.plan {
                    None | Some(Plan::ZedFree) => self.render_free_plan_state(cx),
                    Some(Plan::ZedProTrial) => self.render_trial_state(cx),
                    Some(Plan::ZedPro) => self.render_pro_plan_state(cx),
                }
            } else {
                self.render_accept_terms_of_service()
            }
        } else {
            self.render_sign_in_disclaimer(cx)
        }
    }
}

impl Component for ZedAiOnboarding {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        fn onboarding(
            sign_in_status: SignInStatus,
            has_accepted_terms_of_service: bool,
            plan: Option<Plan>,
            account_too_young: bool,
        ) -> AnyElement {
            ZedAiOnboarding {
                sign_in_status,
                has_accepted_terms_of_service,
                plan,
                account_too_young,
                continue_with_zed_ai: Arc::new(|_, _| {}),
                sign_in: Arc::new(|_, _| {}),
                accept_terms_of_service: Arc::new(|_, _| {}),
                dismiss_onboarding: None,
            }
            .into_any_element()
        }

        Some(
            v_flex()
                .p_4()
                .gap_4()
                .children(vec![
                    single_example(
                        "Not Signed-in",
                        onboarding(SignInStatus::SignedOut, false, None, false),
                    ),
                    single_example(
                        "Not Accepted ToS",
                        onboarding(SignInStatus::SignedIn, false, None, false),
                    ),
                    single_example(
                        "Account too young",
                        onboarding(SignInStatus::SignedIn, false, None, true),
                    ),
                    single_example(
                        "Free Plan",
                        onboarding(SignInStatus::SignedIn, true, Some(Plan::ZedFree), false),
                    ),
                    single_example(
                        "Pro Trial",
                        onboarding(SignInStatus::SignedIn, true, Some(Plan::ZedProTrial), false),
                    ),
                    single_example(
                        "Pro Plan",
                        onboarding(SignInStatus::SignedIn, true, Some(Plan::ZedPro), false),
                    ),
                ])
                .into_any_element(),
        )
    }
}
