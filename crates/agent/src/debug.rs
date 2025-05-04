use gpui::Global;
use language_model::RequestUsage;
use std::ops::{Deref, DerefMut};
use ui::prelude::*;
use zed_llm_client::{Plan, UsageLimit};

/// Debug only: Used for testing various account states
///
/// Use this by initializing it with
/// `cx.set_global(DebugAccountState::default());` somewhere
///
/// Then call `cx.debug_account()` to get access
#[allow(unused, dead_code)]
#[derive(Clone, Debug)]
pub struct DebugAccountState {
    pub enabled: bool,
    pub trial_expired: bool,
    pub plan: Plan,
    pub custom_prompt_usage: RequestUsage,
    pub usage_based_billing_enabled: bool,
    pub monthly_spending_cap: i32,
    pub custom_edit_prediction_usage: UsageLimit,
}

#[allow(unused, dead_code)]
impl DebugAccountState {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }

    pub fn set_trial_expired(&mut self, trial_expired: bool) -> &mut Self {
        self.trial_expired = trial_expired;
        self
    }

    pub fn set_plan(&mut self, plan: Plan) -> &mut Self {
        self.plan = plan;
        self
    }

    pub fn set_custom_prompt_usage(&mut self, custom_prompt_usage: RequestUsage) -> &mut Self {
        self.custom_prompt_usage = custom_prompt_usage;
        self
    }

    pub fn set_usage_based_billing_enabled(
        &mut self,
        usage_based_billing_enabled: bool,
    ) -> &mut Self {
        self.usage_based_billing_enabled = usage_based_billing_enabled;
        self
    }

    pub fn set_monthly_spending_cap(&mut self, monthly_spending_cap: i32) -> &mut Self {
        self.monthly_spending_cap = monthly_spending_cap;
        self
    }

    pub fn set_custom_edit_prediction_usage(
        &mut self,
        custom_edit_prediction_usage: UsageLimit,
    ) -> &mut Self {
        self.custom_edit_prediction_usage = custom_edit_prediction_usage;
        self
    }
}

impl Default for DebugAccountState {
    fn default() -> Self {
        Self {
            enabled: false,
            trial_expired: false,
            plan: Plan::Free,
            custom_prompt_usage: RequestUsage {
                limit: UsageLimit::Unlimited,
                amount: 0,
            },
            usage_based_billing_enabled: false,
            // $50.00
            monthly_spending_cap: 5000,
            custom_edit_prediction_usage: UsageLimit::Unlimited,
        }
    }
}

impl DebugAccountState {
    pub fn _get_global(cx: &App) -> &DebugAccountState {
        &cx.global::<GlobalDebugAccountState>().0
    }
}

#[derive(Clone, Debug)]
pub struct GlobalDebugAccountState(pub DebugAccountState);

impl Global for GlobalDebugAccountState {}

impl Deref for GlobalDebugAccountState {
    type Target = DebugAccountState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalDebugAccountState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait DebugAccount {
    fn debug_account(&self) -> &DebugAccountState;
}

impl DebugAccount for App {
    fn debug_account(&self) -> &DebugAccountState {
        &self.global::<GlobalDebugAccountState>().0
    }
}
