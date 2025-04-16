use gpui::Entity;
use ui::{Banner, Severity};
use ui::{ProgressBar, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrentPlan {
    Trial,
    Free,
    Paid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapReason {
    RequestLimit,
    SpendLimit,
}

#[derive(RegisterComponent)]
pub struct UsageBanner {
    current_plan: CurrentPlan,
    current_requests: u32,
    current_spend: u32,
    monthly_cap: u32,
    usage_based_enabled: bool,
    usage_progress: Entity<ProgressBar>,
}

impl UsageBanner {
    /// Creates a new UsageBanner with the provided values
    pub fn new(
        current_plan: CurrentPlan,
        current_requests: u32,
        current_spend: u32,
        monthly_cap: u32,
        usage_based_enabled: bool,
        cx: &mut App,
    ) -> Self {
        let usage_progress = cx.new(|cx| {
            ProgressBar::new(
                "usage_progress",
                current_requests as f32,
                request_cap_for_plan(&current_plan) as f32,
                cx,
            )
        });

        let banner = Self {
            current_plan,
            current_requests,
            current_spend,
            monthly_cap,
            usage_based_enabled,
            usage_progress,
        };

        // No need to update styling here as it will be done when rendering
        banner
    }

    /// Returns the request cap based on the current plan
    pub fn request_cap(&self) -> u32 {
        request_cap_for_plan(&self.current_plan)
    }

    /// Check if the user is capped due to hitting request limits
    pub fn is_capped_by_requests(&self) -> bool {
        self.current_requests >= self.request_cap()
    }

    /// Check if the user is capped due to hitting spend limits
    pub fn is_capped_by_spend(&self) -> bool {
        // Only check spend limit if spending is enabled and cap is set
        self.usage_based_enabled && self.monthly_cap > 0 && self.current_spend >= self.monthly_cap
    }

    /// Check if the user is approaching request limit (>=90%)
    pub fn is_approaching_request_limit(&self) -> bool {
        let threshold = (self.request_cap() as f32 * 0.9) as u32;
        self.current_requests >= threshold && self.current_requests < self.request_cap()
    }

    /// Check if the user is approaching spend limit (>=90%)
    pub fn is_approaching_spend_limit(&self) -> bool {
        // Only check if spending is enabled and cap is set
        self.usage_based_enabled
            && self.monthly_cap > 0
            && self.current_spend >= (self.monthly_cap as f32 * 0.9) as u32
            && self.current_spend < self.monthly_cap
    }

    /// Check if the user is capped and returns the reason
    pub fn cap_status(&self) -> Option<CapReason> {
        if self.is_capped_by_requests() {
            Some(CapReason::RequestLimit)
        } else if self.is_capped_by_spend() {
            Some(CapReason::SpendLimit)
        } else {
            None
        }
    }

    /// Check if the user is capped for any reason
    pub fn is_capped(&self) -> bool {
        matches!(
            self.cap_status(),
            Some(CapReason::RequestLimit | CapReason::SpendLimit)
        )
    }

    /// Update the current request count and progress bar
    pub fn update_requests(&mut self, requests: u32, cx: &mut Context<Self>) {
        self.current_requests = requests;
        self.update_progress_bar(cx);
        self.update_progress_styling(cx);
    }

    /// Update the current spend amount
    pub fn update_spend(&mut self, spend: u32, cx: &mut Context<Self>) {
        self.current_spend = spend;
        self.update_progress_bar(cx);
        self.update_progress_styling(cx);
    }

    /// Update the progress bar styling based on current usage levels
    fn update_progress_styling(&self, cx: &mut Context<Self>) {
        let is_near_cap = self.current_requests as f32 >= self.request_cap() as f32 * 0.9;
        let is_capped = self.is_capped();

        self.usage_progress.update(cx, |progress_bar, cx| {
            if is_capped {
                progress_bar.fg_color(cx.theme().status().error);
            } else if is_near_cap {
                progress_bar.fg_color(cx.theme().status().warning);
            } else {
                progress_bar.fg_color(cx.theme().status().info);
            }
        });
    }

    fn should_show_request_progress(&self) -> bool {
        // Show request progress for all plans as long as not capped
        // Only show if we have a non-zero request cap
        self.request_cap() > 0 && !self.is_capped_by_requests() && !self.is_capped_by_spend()
    }

    /// Show the spend progress bar once requests are capped
    /// if the user has usage based enabled
    fn should_show_spend_progress(&self) -> bool {
        // Only show spend progress for paid plans with usage-based pricing enabled
        // and when a monthly cap is set
        self.current_plan == CurrentPlan::Paid
            && self.usage_based_enabled
            && self.monthly_cap > 0
            && self.is_capped_by_requests()
            && !self.is_capped_by_spend()
    }

    /// Update the progress bar with current values
    fn update_progress_bar(&mut self, cx: &mut Context<Self>) {
        // Update the progress bar with new values
        // We need to recreate it to update both value and max
        self.usage_progress.update(cx, |progress_bar, cx| {
            // Update progress bar value
            *progress_bar = ProgressBar::new(
                "usage_progress",
                self.current_requests as f32,
                self.request_cap() as f32,
                cx,
            );
        });
    }

    fn severity(&self) -> Severity {
        if self.is_capped_by_spend() || self.is_capped_by_requests() {
            return Severity::Error;
        }

        if self.is_approaching_request_limit() || self.is_approaching_spend_limit() {
            return Severity::Warning;
        }

        Severity::Info
    }
}

/// Helper function to get the request cap based on plan type
fn request_cap_for_plan(plan: &CurrentPlan) -> u32 {
    match plan {
        CurrentPlan::Trial => 150,
        CurrentPlan::Free => 50,
        CurrentPlan::Paid => 500,
    }
}

impl Render for UsageBanner {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let formatted_requests = format!("{} / {}", self.current_requests, self.request_cap());
        let formatted_spend = format!(
            "${:.2} / ${:.2}",
            self.current_spend as f32 / 100.0,
            if self.monthly_cap > 0 {
                self.monthly_cap as f32 / 100.0
            } else {
                0.0
            }
        );

        let (message, action_button) = if self.is_capped_by_spend() {
            (
                "Monthly spending limit reached",
                Some(Button::new("manage", "Manage Spending").into_any_element()),
            )
        } else if self.is_capped_by_requests() {
            let msg = match self.current_plan {
                CurrentPlan::Trial => "Trial request limit reached",
                CurrentPlan::Free => "Free tier request limit reached",
                CurrentPlan::Paid => "Monthly request limit reached",
            };

            let action = match self.current_plan {
                CurrentPlan::Trial | CurrentPlan::Free => {
                    Some(Button::new("upgrade", "Upgrade").into_any_element())
                }
                CurrentPlan::Paid => {
                    if self.usage_based_enabled {
                        Some(Button::new("manage", "Manage").into_any_element())
                    } else {
                        Some(Button::new("enable-usage", "Try Usaged-Based").into_any_element())
                    }
                }
            };

            (msg, action)
        } else if self.is_approaching_request_limit() {
            let msg = "Approaching request limit";

            let action = match self.current_plan {
                CurrentPlan::Trial | CurrentPlan::Free => {
                    Some(Button::new("upgrade", "Upgrade").into_any_element())
                }
                CurrentPlan::Paid => {
                    if !self.usage_based_enabled {
                        Some(Button::new("enable-usage", "Manage").into_any_element())
                    } else {
                        None
                    }
                }
            };

            (msg, action)
        } else if self.is_approaching_spend_limit() {
            (
                "Approaching monthly spend limit",
                Some(Button::new("manage", "Manage Spending").into_any_element()),
            )
        } else {
            let msg = match self.current_plan {
                CurrentPlan::Trial => "Zed AI Trial",
                CurrentPlan::Free => "Zed AI Free",
                CurrentPlan::Paid => "Zed AI Paid",
            };

            (msg, None)
        };

        // Build the content section with usage information
        let mut content = h_flex().flex_1().gap_1().child(Label::new(message));

        // Add usage progress section if we should show it
        if self.should_show_request_progress() {
            content = content.child(
                h_flex()
                    .flex_1()
                    .justify_end()
                    .gap_1p5()
                    .child(
                        h_flex()
                            .items_center()
                            .w_full()
                            .max_w(px(180.))
                            .child(self.usage_progress.clone()),
                    )
                    .child(
                        Label::new(formatted_requests)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            );
        }

        // Add spending information for Paid users with usage-based pricing
        if self.should_show_spend_progress() {
            content = content.child(
                h_flex().flex_1().justify_end().gap_1p5().child(
                    Label::new(formatted_spend)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
            );
        }

        // Create the banner with appropriate severity and content
        let mut banner = Banner::new().severity(self.severity()).children(content);

        // Add action button if available
        if let Some(action) = action_button {
            banner = banner.action_slot(action);
        }

        banner
    }
}

impl Component for UsageBanner {
    fn scope() -> ComponentScope {
        ComponentScope::Notification
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        // Create instances of UsageBanner for different scenarios
        // Trial plan examples (cap = 150)
        let new_trial_user = cx.new(|cx| UsageBanner::new(CurrentPlan::Trial, 10, 0, 0, false, cx));
        let trial_user_warning =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Trial, 135, 0, 0, false, cx));
        let trial_user_capped =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Trial, 150, 0, 0, false, cx));

        // Free plan examples (cap = 50)
        let free_user = cx.new(|cx| UsageBanner::new(CurrentPlan::Free, 25, 0, 0, false, cx));
        let free_user_warning =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Free, 45, 0, 0, false, cx));
        let free_user_capped =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Free, 50, 0, 0, false, cx));

        // Pro plan examples without usage-based pricing (cap = 500)
        let paid_user = cx.new(|cx| UsageBanner::new(CurrentPlan::Paid, 250, 0, 0, false, cx));
        let paid_user_warning =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Paid, 450, 0, 0, false, cx));
        let paid_user_capped =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Paid, 500, 0, 0, false, cx));

        // Pro plan examples with usage-based pricing and monthly spend cap (cap = 500)
        let paid_user_usage_based =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Paid, 500, 5000, 20000, true, cx));
        let paid_user_usage_based_warning =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Paid, 500, 18000, 20000, true, cx));
        let paid_user_usage_based_capped =
            cx.new(|cx| UsageBanner::new(CurrentPlan::Paid, 500, 20000, 20000, true, cx));

        // Group examples by plan type
        let trial_examples = vec![
            single_example(
                "Trial - New User",
                div()
                    .size_full()
                    .child(new_trial_user.clone())
                    .into_any_element(),
            ),
            single_example(
                "Trial - Approaching Limit",
                div()
                    .size_full()
                    .child(trial_user_warning.clone())
                    .into_any_element(),
            ),
            single_example(
                "Trial - Request Limit Reached",
                div()
                    .size_full()
                    .child(trial_user_capped.clone())
                    .into_any_element(),
            ),
        ];

        let free_examples = vec![
            single_example(
                "Free - Normal Usage",
                div()
                    .size_full()
                    .child(free_user.clone())
                    .into_any_element(),
            ),
            single_example(
                "Free - Approaching Limit",
                div()
                    .size_full()
                    .child(free_user_warning.clone())
                    .into_any_element(),
            ),
            single_example(
                "Free - Request Limit Reached",
                div()
                    .size_full()
                    .child(free_user_capped.clone())
                    .into_any_element(),
            ),
        ];

        let paid_examples = vec![
            single_example(
                "Pro - Normal Usage",
                div()
                    .size_full()
                    .child(paid_user.clone())
                    .into_any_element(),
            ),
            single_example(
                "Pro - Approaching Limit",
                div()
                    .size_full()
                    .child(paid_user_warning.clone())
                    .into_any_element(),
            ),
            single_example(
                "Pro - Request Limit Reached",
                div()
                    .size_full()
                    .child(paid_user_capped.clone())
                    .into_any_element(),
            ),
        ];

        let paid_usage_based_examples = vec![
            single_example(
                "Pro with UBP - After Request Cap",
                div()
                    .size_full()
                    .child(paid_user_usage_based.clone())
                    .into_any_element(),
            ),
            single_example(
                "Pro with UBP - Approaching Spend Cap",
                div()
                    .size_full()
                    .child(paid_user_usage_based_warning.clone())
                    .into_any_element(),
            ),
            single_example(
                "Pro with UBP - Spend Cap Reached",
                div()
                    .size_full()
                    .child(paid_user_usage_based_capped.clone())
                    .into_any_element(),
            ),
        ];

        // Combine all examples
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
                    example_group(paid_examples).vertical().into_any_element(),
                    Label::new("Pro Plan with Usage-Based Pricing")
                        .size(LabelSize::Large)
                        .into_any_element(),
                    example_group(paid_usage_based_examples)
                        .vertical()
                        .into_any_element(),
                ])
                .into_any_element(),
        )
    }
}
