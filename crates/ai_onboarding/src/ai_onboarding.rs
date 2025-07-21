mod agent_panel_onboarding_card;
mod agent_panel_onboarding_content;
mod edit_prediction_onboarding_content;
mod young_account_banner;

pub use agent_panel_onboarding_card::AgentPanelOnboardingCard;
pub use agent_panel_onboarding_content::AgentPanelOnboarding;
pub use edit_prediction_onboarding_content::EditPredictionOnboarding;
pub use young_account_banner::YoungAccountBanner;

use std::sync::Arc;

use client::{Client, UserStore, zed_urls};
use gpui::{AnyElement, Entity, IntoElement, ParentElement, SharedString};
use ui::{Divider, List, ListItem, RegisterComponent, TintColor, prelude::*};

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

impl IntoElement for BulletItem {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        ListItem::new("list-item")
            .selectable(false)
            .start_slot(
                Icon::new(IconName::Dash)
                    .size(IconSize::XSmall)
                    .color(Color::Hidden),
            )
            .child(div().w_full().child(Label::new(self.label)))
            .into_any_element()
    }
}

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
    pub plan: Option<proto::Plan>,
    pub account_too_young: bool,
    pub continue_with_zed_ai: Arc<dyn Fn(&mut Window, &mut App)>,
    pub sign_in: Arc<dyn Fn(&mut Window, &mut App)>,
    pub accept_terms_of_service: Arc<dyn Fn(&mut Window, &mut App)>,
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
            has_accepted_terms_of_service: store.current_user_has_accepted_terms().unwrap_or(false),
            plan: store.current_plan(),
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
                    async move |cx| {
                        client.authenticate_and_connect(true, cx).await;
                    }
                })
                .detach();
            }),
        }
    }

    fn render_free_plan_section(&self, cx: &mut App) -> impl IntoElement {
        v_flex()
            .mt_2()
            .gap_1()
            .when(self.account_too_young, |this| this.opacity(0.4))
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
                Button::new("continue", "Continue Free")
                    .disabled(self.account_too_young)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click({
                        let callback = self.continue_with_zed_ai.clone();
                        move |_, window, cx| callback(window, cx)
                    }),
            )
    }

    fn render_pro_plan_section(&self, cx: &mut App) -> impl IntoElement {
        let (button_label, button_url) = if self.account_too_young {
            ("Start with Pro", zed_urls::upgrade_to_zed_pro_url(cx))
        } else {
            ("Start Pro Trial", zed_urls::start_trial_url(cx))
        };

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
            .child(
                List::new()
                    .child(BulletItem::new("500 prompts per month with Claude models"))
                    .child(BulletItem::new("Unlimited edit predictions"))
                    .when(!self.account_too_young, |this| {
                        this.child(BulletItem::new(
                            "Try it out for 14 days with no charge, no credit card required",
                        ))
                    }),
            )
            .child(
                Button::new("pro", button_label)
                    .full_width()
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .on_click(move |_, _window, cx| cx.open_url(&button_url)),
            )
    }

    fn render_accept_terms_of_service(&self) -> Div {
        v_flex()
            .w_full()
            .gap_1()
            .child(Headline::new("Before starting…"))
            .child(Label::new(
                "Make sure you have read and accepted Zed AI's terms of service.",
            ))
            .child(
                Button::new("terms_of_service", "View and Read the Terms of Service")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .icon(IconName::ArrowUpRight)
                    .icon_color(Color::Muted)
                    .icon_size(IconSize::XSmall)
                    .on_click(move |_, _window, cx| {
                        cx.open_url("https://zed.dev/terms-of-service")
                    }),
            )
            .child(
                Button::new("accept_terms", "I've read it and accept it")
                    .full_width()
                    .style(ButtonStyle::Tinted(TintColor::Accent))
                    .on_click({
                        let callback = self.accept_terms_of_service.clone();
                        move |_, window, cx| (callback)(window, cx)
                    }),
            )
    }

    fn render_sign_in_disclaimer(&self, _cx: &mut App) -> Div {
        const SIGN_IN_DISCLAIMER: &str =
            "To start using AI in Zed with our hosted models, sign in and subscribe to a plan.";
        let signing_in = matches!(self.sign_in_status, SignInStatus::SigningIn);

        v_flex()
            .gap_2()
            .child(Headline::new("Welcome to Zed AI"))
            .child(div().w_full().child(Label::new(SIGN_IN_DISCLAIMER)))
            .child(
                Button::new("sign_in", "Sign In with GitHub")
                    .icon(IconName::Github)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .disabled(signing_in)
                    .full_width()
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .on_click({
                        let callback = self.sign_in.clone();
                        move |_, window, cx| callback(window, cx)
                    }),
            )
    }

    fn render_free_plan_onboarding(&self, cx: &mut App) -> Div {
        const PLANS_DESCRIPTION: &str = "Choose how you want to start.";
        let young_account_banner = YoungAccountBanner;

        v_flex()
            .child(Headline::new("Welcome to Zed AI"))
            .child(
                Label::new(PLANS_DESCRIPTION)
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_1()
                    .mb_3(),
            )
            .when(self.account_too_young, |this| {
                this.child(young_account_banner)
            })
            .child(self.render_free_plan_section(cx))
            .child(self.render_pro_plan_section(cx))
    }

    fn render_trial_onboarding(&self, _cx: &mut App) -> Div {
        v_flex()
            .child(Headline::new("Welcome to the trial of Zed Pro"))
            .child(
                Label::new("Here's what you get for the next 14 days:")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_1(),
            )
            .child(
                List::new()
                    .child(BulletItem::new("150 prompts with Claude models"))
                    .child(BulletItem::new(
                        "Unlimited edit predictions with Zeta, our open-source model",
                    )),
            )
            .child(
                Button::new("trial", "Start Trial")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click({
                        let callback = self.continue_with_zed_ai.clone();
                        move |_, window, cx| callback(window, cx)
                    }),
            )
    }

    fn render_pro_plan_onboarding(&self, _cx: &mut App) -> Div {
        v_flex()
            .child(Headline::new("Welcome to Zed Pro"))
            .child(
                Label::new("Here's what you get:")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_1(),
            )
            .child(
                List::new()
                    .child(BulletItem::new("500 prompts with Claude models"))
                    .child(BulletItem::new("Unlimited edit predictions")),
            )
            .child(
                Button::new("pro", "Continue with Zed Pro")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click({
                        let callback = self.continue_with_zed_ai.clone();
                        move |_, window, cx| callback(window, cx)
                    }),
            )
    }
}

impl RenderOnce for ZedAiOnboarding {
    fn render(self, _window: &mut ui::Window, cx: &mut App) -> impl IntoElement {
        if matches!(self.sign_in_status, SignInStatus::SignedIn) {
            if self.has_accepted_terms_of_service {
                match self.plan {
                    None | Some(proto::Plan::Free) => self.render_free_plan_onboarding(cx),
                    Some(proto::Plan::ZedProTrial) => self.render_trial_onboarding(cx),
                    Some(proto::Plan::ZedPro) => self.render_pro_plan_onboarding(cx),
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
            plan: Option<proto::Plan>,
            account_too_young: bool,
        ) -> AnyElement {
            div()
                .w(px(800.))
                .child(ZedAiOnboarding {
                    sign_in_status,
                    has_accepted_terms_of_service,
                    plan,
                    account_too_young,
                    continue_with_zed_ai: Arc::new(|_, _| {}),
                    sign_in: Arc::new(|_, _| {}),
                    accept_terms_of_service: Arc::new(|_, _| {}),
                })
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
                        onboarding(SignInStatus::SignedIn, true, None, true),
                    ),
                    single_example(
                        "Free Plan",
                        onboarding(SignInStatus::SignedIn, true, Some(proto::Plan::Free), false),
                    ),
                    single_example(
                        "Pro Trial",
                        onboarding(
                            SignInStatus::SignedIn,
                            true,
                            Some(proto::Plan::ZedProTrial),
                            false,
                        ),
                    ),
                    single_example(
                        "Pro Plan",
                        onboarding(
                            SignInStatus::SignedIn,
                            true,
                            Some(proto::Plan::ZedPro),
                            false,
                        ),
                    ),
                ])
                .into_any_element(),
        )
    }
}
