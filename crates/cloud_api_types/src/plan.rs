use serde::{Deserialize, Serialize};

use crate::{KnownOrUnknown, Timestamp};

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Plan {
    #[default]
    ZedFree,
    ZedPro,
    ZedProTrial,
    ZedStudent,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanInfo {
    /// We've named this field `plan_v3` to avoid breaking older clients when we start returning new plan variants.
    #[serde(rename = "plan_v3")]
    pub plan: KnownOrUnknown<Plan, String>,
    pub subscription_period: Option<SubscriptionPeriod>,
    pub usage: cloud_llm_client::CurrentUsage,
    pub trial_started_at: Option<Timestamp>,
    pub is_account_too_young: bool,
    pub has_overdue_invoices: bool,
}

impl PlanInfo {
    pub fn plan(&self) -> Plan {
        match &self.plan {
            KnownOrUnknown::Known(plan) => *plan,
            KnownOrUnknown::Unknown(_) => {
                // If we get a plan that we don't recognize, fall back to the Free plan.
                Plan::ZedFree
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct SubscriptionPeriod {
    pub started_at: Timestamp,
    pub ended_at: Timestamp,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_plan_deserialize_snake_case() {
        let plan = serde_json::from_value::<Plan>(json!("zed_free")).unwrap();
        assert_eq!(plan, Plan::ZedFree);

        let plan = serde_json::from_value::<Plan>(json!("zed_pro")).unwrap();
        assert_eq!(plan, Plan::ZedPro);

        let plan = serde_json::from_value::<Plan>(json!("zed_pro_trial")).unwrap();
        assert_eq!(plan, Plan::ZedProTrial);

        let plan = serde_json::from_value::<Plan>(json!("zed_student")).unwrap();
        assert_eq!(plan, Plan::ZedStudent);
    }
}
