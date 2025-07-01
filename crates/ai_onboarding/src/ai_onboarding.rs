use ui::{prelude::*, IntoElement, RegisterComponent, RenderOnce};

pub enum OnboardingSource {
    AgentPanel,
    EditPredictions,
}

#[derive(RegisterComponent, IntoElement)]
pub struct ZedAiOnboarding {
    pub is_signed_in: bool,
    pub has_accepted_terms_of_service: bool,
    pub plan: Option<proto::Plan>,
    pub account_too_young: bool,
    pub source: OnboardingSource,
}

impl RenderOnce for ZedAiOnboarding {
    fn render(self, window: &mut ui::Window, cx: &mut ui::App) -> impl IntoElement {
        let free_plan_ad = v_flex()
            .child("Free")
            .child("50 Zed-hosted prompts per month with the Claude models")
            .child("2000 accepted edit predictions using our open-source, open-dataset Zeta model")
            // accepts the tos if needed
            // dismissed this modal
            // selects Claude Sonnet 4 from Zed
            .child(
                Button::new("continue", "Continue Free")
                    .disabled(self.account_too_young)
                    .full_width(),
            );

        let pro_plan_ad = v_flex()
            .child("Pro")
            .child("500 prompts per month (usage-based billing beyond 500) and unlimited edit predictions")
            .child("You can try it out for 14 days with no charge.")
            // accepts the tos if needed
            // open the zed.dev site, so they can go through the trial in Stripe
            // check whether the account is young
            // once done, the modal should go away
            .map(|this| if self.account_too_young {
                this.child(Button::new("pro", "Start with Pro").full_width())
            } else {
                this.child(Button::new("trial", "Start Trial").full_width())
            });

        let young_account_disclaimer = "Given your GitHub account was created less than 30 days ago, we can't offer your a free trial.";

        div()
            .child("Welcome to Zed AI")
            .child(if self.is_signed_in {
                div()
                    .child("Choose one of the available plans to start with Zed AI")
                    .when(self.account_too_young, |this| this.child(young_account_disclaimer))
                    .child(free_plan_ad)
                    .child(pro_plan_ad)
                    .child("By using any Zed plans, you accept the terms of service")
            } else {
                div()
                    .child("You can start using AI features in Zed by subscribing to a Zed plan, for which you need to sign in")
            })
            .child(ui::Divider::horizontal())
            .child("You don't need to use Zed plans if you don't want to.")
            .child(match self.source {
                OnboardingSource::AgentPanel => "Bring your own API keys",
                OnboardingSource::EditPredictions => {
                    "Use GitHub Copilot as your edit prediction provider"
                }
            })
            .child(
                Button::new(
                    "providers",
                    match self.source {
                        // takes the user to the panel's settings view
                        OnboardingSource::AgentPanel => "Configure Model Providers",
                        // opens the GH copilot setup modal
                        OnboardingSource::EditPredictions => "Configure GitHub Copilot",
                    },
                )
                .full_width(),
            )
    }
}

impl Component for ZedAiOnboarding {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        fn onboarding(
            is_signed_in: bool,
            has_accepted_terms_of_service: bool,
            plan: Option<proto::Plan>,
            account_too_young: bool,
            source: OnboardingSource,
        ) -> AnyElement {
            ZedAiOnboarding {
                is_signed_in,
                has_accepted_terms_of_service,
                plan,
                account_too_young,
                source,
            }
            .into_any_element()
        }

        Some(
            v_flex()
                .p_4()
                .gap_4()
                .children(vec![
                    single_example(
                        "Not Signed-In",
                        onboarding(false, false, None, false, OnboardingSource::AgentPanel),
                    ),
                    single_example(
                        "Not accepted TOS",
                        onboarding(true, false, None, false, OnboardingSource::AgentPanel),
                    ),
                    single_example(
                        "Account too young",
                        onboarding(true, false, None, true, OnboardingSource::AgentPanel),
                    ),
                    single_example(
                        "Agent Panel (Free)",
                        onboarding(
                            true,
                            true,
                            Some(proto::Plan::Free),
                            false,
                            OnboardingSource::AgentPanel,
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
                        ),
                    ),
                ])
                .into_any_element(),
        )
    }
}
