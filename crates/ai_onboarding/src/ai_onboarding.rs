use gpui::{
    Action, AnyElement, ClickEvent, IntoElement, ParentElement, SharedString, linear_color_stop,
    linear_gradient,
};
use ui::{Divider, List, ListItem, RegisterComponent, Vector, VectorName, prelude::*};
use zed_actions::agent::OpenConfiguration;

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

pub enum OnboardingSource {
    AgentPanel,
    EditPredictions,
}

#[derive(RegisterComponent)]
pub struct ZedAiOnboarding {
    pub is_signed_in: bool,
    pub has_accepted_terms_of_service: bool,
    pub plan: Option<proto::Plan>,
    pub account_too_young: bool,
    pub source: OnboardingSource,
}

impl ZedAiOnboarding {
    fn upgrade_plan(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("https://zed.dev/account/upgrade");
        cx.notify();
    }

    fn view_tos(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("https://zed.dev/terms-of-service");
        cx.notify();
    }

    fn continue_free(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        // dl: ccept TOS if needed, select Claude Sonnet
        cx.notify();
    }

    fn configure_providers(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.dispatch_action(OpenConfiguration.boxed_clone(), cx);
        cx.notify();
    }

    fn configure_github_copilot(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        // dl todo
        cx.notify();
    }
}

impl Render for ZedAiOnboarding {
    fn render(&mut self, _window: &mut ui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        const PLANS_DESCRIPTION: &str = "Choose how you want to start.";
        const YOUNG_ACCOUNT_DISCLAIMER: &str = "Given your GitHub account was created less than 30 days ago, we can't offer your a free trial.";
        const SIGN_IN_DISCLAIMER: &str = "You can start using AI features in Zed by subscribing to a Zed plan, for which you need to sign in";

        let in_ep_modal = matches!(self.source, OnboardingSource::EditPredictions);
        let is_signed_in = self.is_signed_in;
        let account_too_young = self.account_too_young;

        let free_plan_ad = v_flex()
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
            // accepts the tos if needed
            // dismissed this modal
            // selects Claude Sonnet 4 from Zed
            .child(
                Button::new("continue", "Continue Free")
                    .disabled(self.account_too_young)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click(cx.listener(Self::configure_providers)),
            );

        let pro_plan_ad = v_flex()
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
                    .child(BulletItem::new(
                        // "500 prompts per month (usage-based billing beyond it) with Claude models",
                        // dl: do we really need the usage-based disclaimer here?
                        "500 prompts per month with Claude models",
                    ))
                    .child(BulletItem::new("Unlimited edit predictions"))
                    .child(BulletItem::new(
                        "Try it out for 14 days with no charge, no credit card required",
                    )),
            )
            // accepts the tos if needed
            // open the zed.dev site, so they can go through the trial in Stripe
            // check whether the account is young
            // once done, the modal should go away
            .map(|this| {
                if account_too_young {
                    this.child(
                        Button::new("pro", "Start with Pro")
                            .full_width()
                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .on_click(cx.listener(Self::upgrade_plan)),
                    )
                } else {
                    this.child(
                        Button::new("trial", "Start Pro Trial")
                            .full_width()
                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .on_click(cx.listener(Self::upgrade_plan)),
                    )
                }
            });

        let tos_disclaimer = h_flex()
            .mt_2()
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
                    .on_click(cx.listener(Self::view_tos)),
            );

        // dl: we may not even need this if ToS accept is attached to signing in
        let bring_api_keys = v_flex()
            .mt_2()
            .gap_1()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new("BYOK")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .buffer_font(cx),
                    )
                    .child(Divider::horizontal()),
            )
            .child(
                List::new()
                    .child(BulletItem::new(
                        "You can also use Zed AI by bringing your own API keys",
                    ))
                    .child(BulletItem::new(
                        "No need for any of the plans; not even signing in",
                    )),
            )
            .child(
                Button::new("configure_models", "Continue Models")
                    .disabled(self.account_too_young)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click(cx.listener(Self::configure_providers)),
            );

        let github_copilot = v_flex()
            .gap_1()
            .child(Label::new(
                "Alternatively, you can use GitHub Copilot as your edit prediction provider.",
            ))
            .child(
                Button::new("configure_copilot", "Configure Copilot")
                    .disabled(self.account_too_young)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .on_click(cx.listener(Self::configure_github_copilot)),
            );

        let main_content = if is_signed_in {
            v_flex()
                .child(Headline::new("Welcome to Zed AI"))
                .child(
                    Label::new(PLANS_DESCRIPTION)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .mt_1(),
                )
                .when(account_too_young, |this| {
                    this.child(YOUNG_ACCOUNT_DISCLAIMER)
                })
                .child(free_plan_ad)
                .child(pro_plan_ad)
                .child(tos_disclaimer)
        } else {
            div().child(SIGN_IN_DISCLAIMER)
        };

        if in_ep_modal {
            return v_flex()
                .gap_2()
                .child(main_content)
                .child(ui::Divider::horizontal())
                .child(github_copilot);
        }

        div()
            .m_4()
            .p(px(3.))
            .elevation_2(cx)
            .rounded_lg()
            .bg(cx.theme().colors().background.alpha(0.5))
            .child(
                v_flex()
                    .relative()
                    .size_full()
                    .px_4()
                    .py_3()
                    .gap_2()
                    .border_1()
                    .rounded(px(5.))
                    .border_color(cx.theme().colors().text.alpha(0.1))
                    .overflow_hidden()
                    .bg(cx.theme().colors().panel_background)
                    .child(
                        div()
                            .absolute()
                            .top(px(-8.0))
                            .right_0()
                            .w(px(400.))
                            .h(px(92.))
                            .child(
                                Vector::new(
                                    VectorName::AiGrid,
                                    rems_from_px(400.),
                                    rems_from_px(92.),
                                )
                                .color(Color::Custom(cx.theme().colors().text.alpha(0.32))),
                            ),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .right_0()
                            .w(px(660.))
                            .h(px(401.))
                            .overflow_hidden()
                            .bg(linear_gradient(
                                75.,
                                linear_color_stop(
                                    cx.theme().colors().panel_background.alpha(0.01),
                                    1.0,
                                ),
                                linear_color_stop(cx.theme().colors().panel_background, 0.45),
                            )),
                    )
                    .child(main_content)
                    .child(bring_api_keys),
            )
    }
}

impl Component for ZedAiOnboarding {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        fn onboarding(
            is_signed_in: bool,
            has_accepted_terms_of_service: bool,
            plan: Option<proto::Plan>,
            account_too_young: bool,
            source: OnboardingSource,
            cx: &mut App,
        ) -> AnyElement {
            cx.new(|_cx| ZedAiOnboarding {
                is_signed_in,
                has_accepted_terms_of_service,
                plan,
                account_too_young,
                source,
            })
            .into_any_element()
        }

        Some(
            v_flex()
                .p_4()
                .gap_4()
                .children(vec![
                    single_example(
                        "Not Signed-In",
                        onboarding(false, false, None, false, OnboardingSource::AgentPanel, cx),
                    ),
                    single_example(
                        "Not accepted TOS",
                        onboarding(true, false, None, false, OnboardingSource::AgentPanel, cx),
                    ),
                    single_example(
                        "Account too young",
                        onboarding(true, false, None, true, OnboardingSource::AgentPanel, cx),
                    ),
                    single_example(
                        "Agent Panel (Free)",
                        onboarding(
                            true,
                            true,
                            Some(proto::Plan::Free),
                            false,
                            OnboardingSource::AgentPanel,
                            cx,
                        ),
                    ),
                    single_example(
                        "Agent Panel (Trial)",
                        onboarding(
                            true,
                            true,
                            Some(proto::Plan::ZedProTrial),
                            false,
                            OnboardingSource::AgentPanel,
                            cx,
                        ),
                    ),
                    single_example(
                        "Agent Panel (Pro)",
                        onboarding(
                            true,
                            true,
                            Some(proto::Plan::ZedPro),
                            false,
                            OnboardingSource::AgentPanel,
                            cx,
                        ),
                    ),
                    single_example(
                        "Edit Predictions (Free)",
                        onboarding(
                            true,
                            true,
                            Some(proto::Plan::Free),
                            false,
                            OnboardingSource::EditPredictions,
                            cx,
                        ),
                    ),
                ])
                .into_any_element(),
        )
    }
}
