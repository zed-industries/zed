mod agent_panel_onboarding;
mod edit_prediction_onboarding;
mod onboarding_container;

pub use agent_panel_onboarding::AgentPanelOnboarding;
pub use edit_prediction_onboarding::EditPredictionOnboarding;
pub use onboarding_container::OnboardingContainer;

use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{AnyElement, ClickEvent, Entity, IntoElement, ParentElement, SharedString};
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

    fn upgrade_plan(_: &ClickEvent, _: &mut Window, cx: &mut App) {
        cx.open_url("https://zed.dev/account/upgrade");
    }

    fn view_terms_of_service(_: &ClickEvent, _: &mut Window, cx: &mut App) {
        cx.open_url("https://zed.dev/terms-of-service");
    }

    fn render_free_plan_section(&self, cx: &mut App) -> impl IntoElement {
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
        let button_label = if self.account_too_young {
            "Start with Pro"
        } else {
            "Start Pro Trial"
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
                    .on_click(Self::upgrade_plan),
            )
    }

    fn render_accept_terms_of_service(&self) -> Div {
        let terms_link = Button::new("terms_of_service", "Terms of Service")
            .style(ButtonStyle::Subtle)
            .icon(IconName::ArrowUpRight)
            .icon_color(Color::Muted)
            .icon_size(IconSize::XSmall)
            .on_click(move |_, _window, cx| cx.open_url("https://zed.dev/terms-of-service"));

        v_flex()
            .w_full()
            .gap_1()
            .child(Headline::new("Before starting…"))
            .child(
                h_flex()
                    .child(Label::new(
                        "Make sure you have read and accepted Zed's AI terms of service.",
                    ))
                    .child(terms_link),
            )
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
                Button::new("accept_terms", "I accept")
                    .full_width()
                    .style(ButtonStyle::Tinted(TintColor::Accent))
                    .icon(IconName::Check)
                    .icon_size(IconSize::Small)
                    .on_click({
                        let callback = self.accept_terms_of_service.clone();
                        move |_, window, cx| (callback)(window, cx)
                    }),
            )
    }

    fn render_terms_or_service_disclaimer() -> impl IntoElement {
        h_flex()
            .child(
                Label::new("By using any Zed plans, you accept the")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                Button::new("view-tos", "terms of service.")
                    .icon(IconName::ArrowUpRight)
                    .icon_size(IconSize::Indicator)
                    .icon_color(Color::Muted)
                    .label_size(LabelSize::Small)
                    .on_click(Self::view_terms_of_service),
            )
    }

    fn render_young_account_disclaimer() -> impl IntoElement {
        const YOUNG_ACCOUNT_DISCLAIMER: &str = "Given your GitHub account was created less than 30 days ago, we can't offer your a free Pro plan trial.";

        Label::new(YOUNG_ACCOUNT_DISCLAIMER)
            .size(LabelSize::Small)
            .color(Color::Muted)
    }

    fn render_sign_in_disclaimer(&self, _cx: &mut App) -> Div {
        const SIGN_IN_DISCLAIMER: &str = "You can start using AI features in Zed by subscribing to a plan, for which you need to sign in.";
        let signing_in = matches!(self.sign_in_status, SignInStatus::SigningIn);

        v_flex()
            .gap_2()
            .child(Headline::new("Welcome to Zed AI"))
            .child(div().w_full().child(Label::new(SIGN_IN_DISCLAIMER)).mt_1())
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

        v_flex()
            .child(Headline::new("Welcome to Zed AI"))
            .child(
                Label::new(PLANS_DESCRIPTION)
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_1(),
            )
            .child(self.render_free_plan_section(cx))
            .child(self.render_pro_plan_section(cx))
            .child(
                v_flex()
                    .gap_1()
                    .mt_2()
                    .when(self.account_too_young, |this| {
                        this.child(Self::render_young_account_disclaimer())
                    })
                    .child(Self::render_terms_or_service_disclaimer()),
            )
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
            ZedAiOnboarding {
                sign_in_status,
                has_accepted_terms_of_service,
                plan,
                account_too_young,
                continue_with_zed_ai: Arc::new(|_, _| {}),
                sign_in: Arc::new(|_, _| {}),
                accept_terms_of_service: Arc::new(|_, _| {}),
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
