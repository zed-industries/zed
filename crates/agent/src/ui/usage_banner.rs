use gpui::Entity;
use ui::{Banner, ProgressBar, Severity, prelude::*};
use zed_llm_client::{Plan, UsageLimit};

#[derive(IntoElement)]
pub struct UsageBanner {}

impl UsageBanner {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for UsageBanner {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let severity = Severity::Info;

        Banner::new().severity(severity).children(
            h_flex()
                .flex_1()
                .gap_1()
                .child(Label::new("Zed Pro"))
                .child(
                    h_flex()
                        .flex_1()
                        .justify_end()
                        .gap_1p5()
                        .child(
                            h_flex()
                                .items_center()
                                .w_full()
                                .max_w(px(180.))
                                .child(ProgressBar::new("usage", 0., 100., cx)),
                        )
                        .child(
                            Label::new("0 / Unlimited")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                ),
        )
        // .action_slot(action)
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum SpendLimit {
//     Limited(i32),
//     Unlimited,
// }

// impl SpendLimit {
//     pub fn formatted(&self) -> String {
//         match self {
//             SpendLimit::Limited(amount) => format!("${:.2}", *amount as f32 / 100.0),
//             SpendLimit::Unlimited => "Unlimited".to_string(),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_format_spend_limit() {
//         assert_eq!(SpendLimit::Unlimited.formatted(), "Unlimited");
//         assert_eq!(SpendLimit::Limited(1099).formatted(), "$10.99");
//         assert_eq!(SpendLimit::Limited(2500).formatted(), "$25.00");
//         assert_eq!(SpendLimit::Limited(9900).formatted(), "$99.00");
//     }
// }

// #[derive(RegisterComponent)]
// pub struct UsageBanner {
//     current_plan: Plan,
//     current_requests: i32,
//     current_spend: u32,
//     monthly_cap: Option<u32>,
//     usage_based_enabled: bool,
//     severity: Severity,
//     usage_progress: Option<Entity<ProgressBar>>,
//     spending_progress: Option<Entity<ProgressBar>>,
// }

// impl UsageBanner {
//     pub fn new(
//         current_plan: Plan,
//         current_requests: i32,
//         current_spend: u32,
//         usage_based_enabled: bool,
//         cx: &mut App,
//     ) -> Self {
//         let usage_progress = match current_plan.model_requests_limit() {
//             UsageLimit::Limited(limit) => Some(cx.new(|cx| {
//                 ProgressBar::new("usage_progress", current_requests as f32, limit as f32, cx)
//             })),
//             UsageLimit::Unlimited => None,
//         };

//         Self {
//             current_plan,
//             current_requests,
//             current_spend,
//             monthly_cap: None,
//             usage_based_enabled,
//             usage_progress,
//             severity: Severity::default(),
//             spending_progress: None,
//         }
//     }

//     pub fn update_monthly_cap(&mut self, monthly_cap: Option<u32>) {
//         self.monthly_cap = monthly_cap;
//     }

//     pub fn request_cap(&self) -> UsageLimit {
//         self.current_plan.model_requests_limit()
//     }

//     /// Check if the user is capped due to hitting request limits
//     pub fn is_capped_by_requests(&self) -> bool {
//         match self.request_cap() {
//             UsageLimit::Limited(limit) => self.current_requests >= limit,
//             UsageLimit::Unlimited => false,
//         }
//     }

//     /// Check if the user is capped due to hitting spend limits
//     pub fn is_capped_by_spend(&self) -> bool {
//         if let Some(monthly_cap) = self.monthly_cap {
//             self.usage_based_enabled && monthly_cap > 0 && self.current_spend >= monthly_cap
//         } else {
//             false
//         }
//     }

//     /// Check if the user is approaching request limit (>=90%)
//     pub fn is_approaching_request_limit(&self) -> bool {
//         match self.request_cap() {
//             UsageLimit::Limited(limit) => {
//                 let threshold = (limit as f32 * 0.9) as i32;
//                 self.current_requests >= threshold && self.current_requests < limit
//             }
//             UsageLimit::Unlimited => false,
//         }
//     }

//     /// Check if the user is approaching spend limit (>=90%)
//     pub fn is_approaching_spend_limit(&self) -> bool {
//         if let Some(monthly_cap) = self.monthly_cap {
//             self.usage_based_enabled
//                 && monthly_cap > 0
//                 && self.current_spend >= (monthly_cap as f32 * 0.9) as u32
//                 && self.current_spend < monthly_cap
//         } else {
//             false
//         }
//     }

//     /// Check if the user is capped for any reason
//     pub fn is_capped(&self) -> bool {
//         self.is_approaching_request_limit() || self.is_approaching_spend_limit()
//     }

//     /// Update the current request count and progress bar
//     pub fn update_requests(&mut self, requests: i32, cx: &mut Context<Self>) {
//         self.current_requests = requests;
//         self.update_progress_bar(cx);
//         self.update_progress_styling(cx);
//     }

//     /// Update the current spend amount
//     pub fn update_spend(&mut self, spend: u32, cx: &mut Context<Self>) {
//         self.current_spend = spend;
//         self.update_progress_bar(cx);
//         self.update_progress_styling(cx);
//     }

//     pub fn update_spend_progress(&mut self, cx: &mut Context<Self>) {
//         if let Some(monthly_cap) = self.monthly_cap {
//             let progress = self.current_spend as f32 / monthly_cap as f32;
//             if let Some(spending_progress) = self.spending_progress.as_ref() {
//                 spending_progress.update(cx, |progress_bar, _cx| {
//                     progress_bar.value(progress);
//                 });
//             }
//         } else {
//             self.spending_progress = None;
//         }
//     }

//     /// Update the progress bar styling based on current usage levels
//     fn update_progress_styling(&self, cx: &mut Context<Self>) {
//         let is_near_cap = match self.request_cap() {
//             UsageLimit::Limited(limit) => self.current_requests as f32 >= limit as f32 * 0.9,
//             UsageLimit::Unlimited => false,
//         };
//         let is_capped = self.is_capped();

//         if let Some(usage_progress) = self.usage_progress.as_ref() {
//             usage_progress.update(cx, |progress_bar, cx| {
//                 progress_bar.fg_color(self.severity.color(cx));
//             });
//         }
//     }

//     fn should_show_request_progress(&self) -> bool {
//         match self.request_cap() {
//             UsageLimit::Limited(_) => !self.is_capped_by_requests() && !self.is_capped_by_spend(),
//             UsageLimit::Unlimited => false,
//         }
//     }

//     /// Show the spend progress bar once requests are capped
//     /// if the user has usage based enabled
//     fn should_show_spend_progress(&self) -> bool {
//         if let Some(monthly_cap) = self.monthly_cap {
//             self.current_plan == Plan::ZedPro
//                 && self.usage_based_enabled
//                 && monthly_cap > 0
//                 && self.is_capped_by_requests()
//                 && !self.is_capped_by_spend()
//         } else {
//             false
//         }
//     }

//     fn update_progress_bar(&mut self, cx: &mut Context<Self>) {
//         match self.request_cap() {
//             UsageLimit::Limited(limit) => {
//                 if let Some(usage_progress) = self.usage_progress.as_ref() {
//                     usage_progress.update(cx, |progress_bar, _cx| {
//                         progress_bar.value(self.current_requests as f32);
//                         progress_bar.max_value(limit as f32);
//                     });
//                 }
//             }
//             UsageLimit::Unlimited => {}
//         }
//     }

//     fn update_severity(&mut self) {
//         if self.is_capped_by_spend() || self.is_capped_by_requests() {
//             self.severity = Severity::Error;
//         } else if self.is_approaching_request_limit() || self.is_approaching_spend_limit() {
//             self.severity = Severity::Warning;
//         } else {
//             self.severity = Severity::Info;
//         }
//     }
// }

// impl Render for UsageBanner {
//     fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
//         let formatted_requests = match self.request_cap() {
//             UsageLimit::Limited(limit) => format!("{} / {}", self.current_requests, limit),
//             UsageLimit::Unlimited => format!("{} / ∞", self.current_requests),
//         };
//         let formatted_spend = if let Some(monthly_cap) = self.monthly_cap {
//             if monthly_cap > 0 {
//                 format!(
//                     "${:.2} / ${:.2}",
//                     self.current_spend as f32 / 100.0,
//                     monthly_cap as f32 / 100.0
//                 )
//             } else {
//                 format!("${:.2}", self.current_spend as f32 / 100.0)
//             }
//         } else {
//             format!("${:.2}", self.current_spend as f32 / 100.0)
//         };

//         let (message, action_button) = if self.is_capped_by_spend() {
//             (
//                 "Monthly spending limit reached",
//                 Some(Button::new("manage", "Manage Spending").into_any_element()),
//             )
//         } else if self.is_capped_by_requests() {
//             let msg = match self.current_plan {
//                 Plan::ZedProTrial => "Trial request limit reached",
//                 Plan::Free => "Free tier request limit reached",
//                 Plan::ZedPro => "Monthly request limit reached",
//             };

//             let action = match self.current_plan {
//                 Plan::ZedProTrial | Plan::Free => {
//                     Some(Button::new("upgrade", "Upgrade").into_any_element())
//                 }
//                 Plan::ZedPro => {
//                     if self.usage_based_enabled {
//                         Some(Button::new("manage", "Manage").into_any_element())
//                     } else {
//                         Some(Button::new("enable-usage", "Try Usaged-Based").into_any_element())
//                     }
//                 }
//             };

//             (msg, action)
//         } else if self.is_approaching_request_limit() {
//             let msg = "Approaching request limit";

//             let action = match self.current_plan {
//                 Plan::ZedProTrial | Plan::Free => {
//                     Some(Button::new("upgrade", "Upgrade").into_any_element())
//                 }
//                 Plan::ZedPro => {
//                     if !self.usage_based_enabled {
//                         Some(Button::new("enable-usage", "Manage").into_any_element())
//                     } else {
//                         None
//                     }
//                 }
//             };

//             (msg, action)
//         } else if self.is_approaching_spend_limit() {
//             (
//                 "Approaching monthly spend limit",
//                 Some(Button::new("manage", "Manage Spending").into_any_element()),
//             )
//         } else {
//             let msg = match self.current_plan {
//                 Plan::ZedProTrial => "Zed AI Trial",
//                 Plan::Free => "Zed AI Free",
//                 Plan::ZedPro => "Zed AI Paid",
//             };

//             (msg, None)
//         };

//         let mut content = h_flex().flex_1().gap_1().child(Label::new(message));

//         if self.should_show_request_progress() {
//             content = content.child(
//                 h_flex()
//                     .flex_1()
//                     .justify_end()
//                     .gap_1p5()
//                     .children(self.usage_progress.clone().map(|usage_progress| {
//                         h_flex()
//                             .items_center()
//                             .w_full()
//                             .max_w(px(180.))
//                             .child(usage_progress)
//                     }))
//                     .child(
//                         Label::new(formatted_requests)
//                             .size(LabelSize::Small)
//                             .color(Color::Muted),
//                     ),
//             );
//         }

//         if self.should_show_spend_progress() {
//             content = content.child(
//                 h_flex().flex_1().justify_end().gap_1p5().child(
//                     Label::new(formatted_spend)
//                         .size(LabelSize::Small)
//                         .color(Color::Muted),
//                 ),
//             );
//         }

//         Banner::new()
//             .severity(self.severity)
//             .children(content)
//             .map(|banner| {
//                 if let Some(action) = action_button {
//                     banner.action_slot(action)
//                 } else {
//                     banner
//                 }
//             })
//     }
// }

// impl Component for UsageBanner {
//     fn scope() -> ComponentScope {
//         ComponentScope::Notification
//     }

//     fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
//         let new_trial_user = cx.new(|cx| UsageBanner::new(Plan::ZedProTrial, 10, 0, false, cx));
//         let trial_user_warning =
//             cx.new(|cx| UsageBanner::new(Plan::ZedProTrial, 135, 0, false, cx));
//         let trial_user_capped = cx.new(|cx| UsageBanner::new(Plan::ZedProTrial, 150, 0, false, cx));

//         let free_user = cx.new(|cx| UsageBanner::new(Plan::Free, 25, 0, false, cx));
//         let free_user_warning = cx.new(|cx| UsageBanner::new(Plan::Free, 45, 0, false, cx));
//         let free_user_capped = cx.new(|cx| UsageBanner::new(Plan::Free, 50, 0, false, cx));

//         let paid_user = cx.new(|cx| UsageBanner::new(Plan::ZedPro, 250, 0, false, cx));
//         let paid_user_warning = cx.new(|cx| UsageBanner::new(Plan::ZedPro, 450, 0, false, cx));
//         let paid_user_capped = cx.new(|cx| UsageBanner::new(Plan::ZedPro, 500, 0, false, cx));

//         let paid_user_usage_based =
//             cx.new(|cx| UsageBanner::new(Plan::ZedPro, 500, 5000, true, cx));
//         let paid_user_usage_based_warning =
//             cx.new(|cx| UsageBanner::new(Plan::ZedPro, 500, 18000, true, cx));
//         let paid_user_usage_based_capped =
//             cx.new(|cx| UsageBanner::new(Plan::ZedPro, 500, 20000, true, cx));

//         let trial_examples = vec![
//             single_example(
//                 "Trial - New User",
//                 div()
//                     .size_full()
//                     .child(new_trial_user.clone())
//                     .into_any_element(),
//             ),
//             single_example(
//                 "Trial - Approaching Limit",
//                 div()
//                     .size_full()
//                     .child(trial_user_warning.clone())
//                     .into_any_element(),
//             ),
//             single_example(
//                 "Trial - Request Limit Reached",
//                 div()
//                     .size_full()
//                     .child(trial_user_capped.clone())
//                     .into_any_element(),
//             ),
//         ];

//         let free_examples = vec![
//             single_example(
//                 "Free - Normal Usage",
//                 div()
//                     .size_full()
//                     .child(free_user.clone())
//                     .into_any_element(),
//             ),
//             single_example(
//                 "Free - Approaching Limit",
//                 div()
//                     .size_full()
//                     .child(free_user_warning.clone())
//                     .into_any_element(),
//             ),
//             single_example(
//                 "Free - Request Limit Reached",
//                 div()
//                     .size_full()
//                     .child(free_user_capped.clone())
//                     .into_any_element(),
//             ),
//         ];

//         let paid_examples = vec![
//             single_example(
//                 "Pro - Normal Usage",
//                 div()
//                     .size_full()
//                     .child(paid_user.clone())
//                     .into_any_element(),
//             ),
//             single_example(
//                 "Pro - Approaching Limit",
//                 div()
//                     .size_full()
//                     .child(paid_user_warning.clone())
//                     .into_any_element(),
//             ),
//             single_example(
//                 "Pro - Request Limit Reached",
//                 div()
//                     .size_full()
//                     .child(paid_user_capped.clone())
//                     .into_any_element(),
//             ),
//         ];

//         let paid_usage_based_examples = vec![
//             single_example(
//                 "Pro with UBP - After Request Cap",
//                 div()
//                     .size_full()
//                     .child(paid_user_usage_based.clone())
//                     .into_any_element(),
//             ),
//             single_example(
//                 "Pro with UBP - Approaching Spend Cap",
//                 div()
//                     .size_full()
//                     .child(paid_user_usage_based_warning.clone())
//                     .into_any_element(),
//             ),
//             single_example(
//                 "Pro with UBP - Spend Cap Reached",
//                 div()
//                     .size_full()
//                     .child(paid_user_usage_based_capped.clone())
//                     .into_any_element(),
//             ),
//         ];

//         Some(
//             v_flex()
//                 .gap_6()
//                 .p_4()
//                 .children(vec![
//                     Label::new("Trial Plan")
//                         .size(LabelSize::Large)
//                         .into_any_element(),
//                     example_group(trial_examples).vertical().into_any_element(),
//                     Label::new("Free Plan")
//                         .size(LabelSize::Large)
//                         .into_any_element(),
//                     example_group(free_examples).vertical().into_any_element(),
//                     Label::new("Pro Plan")
//                         .size(LabelSize::Large)
//                         .into_any_element(),
//                     example_group(paid_examples).vertical().into_any_element(),
//                     Label::new("Pro Plan with Usage-Based Pricing")
//                         .size(LabelSize::Large)
//                         .into_any_element(),
//                     example_group(paid_usage_based_examples)
//                         .vertical()
//                         .into_any_element(),
//                 ])
//                 .into_any_element(),
//         )
//     }
// }
