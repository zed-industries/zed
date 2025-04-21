use client::zed_urls;
use ui::{Banner, ProgressBar, Severity, prelude::*};
use zed_llm_client::{Plan, UsageLimit};

#[derive(IntoElement, RegisterComponent)]
pub struct UsageBanner {
    plan: Plan,
    requests: i32,
}

impl UsageBanner {
    pub fn new(plan: Plan, requests: i32) -> Self {
        Self { plan, requests }
    }
}

impl RenderOnce for UsageBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let request_limit = self.plan.model_requests_limit();

        let used_percentage = match request_limit {
            UsageLimit::Limited(limit) => Some((self.requests as f32 / limit as f32) * 100.),
            UsageLimit::Unlimited => None,
        };

        let (severity, message) = match request_limit {
            UsageLimit::Limited(limit) => {
                if self.requests >= limit {
                    let message = match self.plan {
                        Plan::ZedPro => "Monthly request limit reached",
                        Plan::ZedProTrial => "Trial request limit reached",
                        Plan::Free => "Free tier request limit reached",
                    };

                    (Severity::Error, message)
                } else if (self.requests as f32 / limit as f32) >= 0.9 {
                    (Severity::Warning, "Approaching request limit")
                } else {
                    let message = match self.plan {
                        Plan::ZedPro => "Zed Pro",
                        Plan::ZedProTrial => "Zed Pro (Trial)",
                        Plan::Free => "Zed Free",
                    };

                    (Severity::Info, message)
                }
            }
            UsageLimit::Unlimited => {
                let message = match self.plan {
                    Plan::ZedPro => "Zed Pro",
                    Plan::ZedProTrial => "Zed Pro (Trial)",
                    Plan::Free => "Zed Free",
                };

                (Severity::Info, message)
            }
        };

        let action = match self.plan {
            Plan::ZedProTrial | Plan::Free => {
                Button::new("upgrade", "Upgrade").on_click(|_, _window, cx| {
                    cx.open_url(&zed_urls::account_url(cx));
                })
            }
            Plan::ZedPro => Button::new("manage", "Manage").on_click(|_, _window, cx| {
                cx.open_url(&zed_urls::account_url(cx));
            }),
        };

        Banner::new().severity(severity).children(
            h_flex().flex_1().gap_1().child(Label::new(message)).child(
                h_flex()
                    .flex_1()
                    .justify_end()
                    .gap_1p5()
                    .children(used_percentage.map(|percent| {
                        h_flex()
                            .items_center()
                            .w_full()
                            .max_w(px(180.))
                            .child(ProgressBar::new("usage", percent, 100., cx))
                    }))
                    .child(
                        Label::new(match request_limit {
                            UsageLimit::Limited(limit) => {
                                format!("{} / {limit}", self.requests)
                            }
                            UsageLimit::Unlimited => format!("{} / ∞", self.requests),
                        })
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                    )
                    // Note: This should go in the banner's `action_slot`, but doing that messes with the size of the
                    // progress bar.
                    .child(action),
            ),
        )
    }
}

impl Component for UsageBanner {
    fn sort_name() -> &'static str {
        "AgentUsageBanner"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let trial_examples = vec![
            single_example(
                "Zed Pro Trial - New User",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::ZedProTrial, 10))
                    .into_any_element(),
            ),
            single_example(
                "Zed Pro Trial - Approaching Limit",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::ZedProTrial, 135))
                    .into_any_element(),
            ),
            single_example(
                "Zed Pro Trial - Request Limit Reached",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::ZedProTrial, 150))
                    .into_any_element(),
            ),
        ];

        let free_examples = vec![
            single_example(
                "Free - Normal Usage",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::Free, 25))
                    .into_any_element(),
            ),
            single_example(
                "Free - Approaching Limit",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::Free, 45))
                    .into_any_element(),
            ),
            single_example(
                "Free - Request Limit Reached",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::Free, 50))
                    .into_any_element(),
            ),
        ];

        let zed_pro_examples = vec![
            single_example(
                "Zed Pro - Normal Usage",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::ZedPro, 250))
                    .into_any_element(),
            ),
            single_example(
                "Zed Pro - Approaching Limit",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::ZedPro, 450))
                    .into_any_element(),
            ),
            single_example(
                "Zed Pro - Request Limit Reached",
                div()
                    .size_full()
                    .child(UsageBanner::new(Plan::ZedPro, 500))
                    .into_any_element(),
            ),
        ];

        Some(
            v_flex()
                .gap_6()
                .p_4()
                .children(vec![
                    Label::new("Trial Plan")
                        .size(LabelSize::Large)
                        .into_any_element(),
                    example_group(trial_examples).vertical().into_any_element(),
                    Label::new("Free Plan")
                        .size(LabelSize::Large)
                        .into_any_element(),
                    example_group(free_examples).vertical().into_any_element(),
                    Label::new("Pro Plan")
                        .size(LabelSize::Large)
                        .into_any_element(),
                    example_group(zed_pro_examples)
                        .vertical()
                        .into_any_element(),
                ])
                .into_any_element(),
        )
    }
}
