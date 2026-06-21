use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct AiCreditStatusSettingsContent {
    /// Whether to show AI credit usage in the status bar.
    ///
    /// Default: true
    pub enabled: Option<bool>,
    /// How often to refresh credit usage data, in seconds.
    ///
    /// Default: 60
    pub refresh_seconds: Option<u64>,
    /// Optional monthly budget in USD used as the 100% mark for providers that
    /// do not expose remaining credits directly (e.g. OpenAI, Anthropic, Mistral).
    ///
    /// Default: null
    pub monthly_budget_usd: Option<f32>,
}
