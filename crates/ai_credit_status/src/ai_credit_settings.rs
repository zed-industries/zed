use settings::{RegisterSetting, Settings, SettingsContent};

#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct AiCreditStatusSettings {
    pub enabled: bool,
    pub refresh_seconds: u64,
    pub monthly_budget_usd: Option<f32>,
}

impl Settings for AiCreditStatusSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let settings = content.ai_credit_status.clone().unwrap_or_default();
        Self {
            enabled: settings.enabled.unwrap_or(true),
            refresh_seconds: settings.refresh_seconds.unwrap_or(60),
            monthly_budget_usd: settings.monthly_budget_usd,
        }
    }
}
