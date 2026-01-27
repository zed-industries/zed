use serde::{Deserialize, Serialize};

use crate::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Plan {
    V2(PlanV2),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanV2 {
    #[default]
    ZedFree,
    ZedPro,
    ZedProTrial,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanInfo {
    pub plan_v2: PlanV2,
    pub subscription_period: Option<SubscriptionPeriod>,
    pub usage: cloud_llm_client::CurrentUsage,
    pub trial_started_at: Option<Timestamp>,
    pub is_account_too_young: bool,
    pub has_overdue_invoices: bool,
}

impl PlanInfo {
    pub fn plan(&self) -> Plan {
        Plan::V2(self.plan_v2)
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
    fn test_plan_v2_deserialize_snake_case() {
        let plan = serde_json::from_value::<PlanV2>(json!("zed_free")).unwrap();
        assert_eq!(plan, PlanV2::ZedFree);

        let plan = serde_json::from_value::<PlanV2>(json!("zed_pro")).unwrap();
        assert_eq!(plan, PlanV2::ZedPro);

        let plan = serde_json::from_value::<PlanV2>(json!("zed_pro_trial")).unwrap();
        assert_eq!(plan, PlanV2::ZedProTrial);
    }
}
