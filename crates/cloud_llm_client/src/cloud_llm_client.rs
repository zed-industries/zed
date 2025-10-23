pub mod predict_edits_v3;

use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context as _;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};
use uuid::Uuid;

/// The name of the header used to indicate which version of Zed the client is running.
pub const ZED_VERSION_HEADER_NAME: &str = "x-zed-version";

/// The name of the header used to indicate when a request failed due to an
/// expired LLM token.
///
/// The client may use this as a signal to refresh the token.
pub const EXPIRED_LLM_TOKEN_HEADER_NAME: &str = "x-zed-expired-token";

/// The name of the header used to indicate what plan the user is currently on.
pub const CURRENT_PLAN_HEADER_NAME: &str = "x-zed-plan";

/// The name of the header used to indicate the usage limit for model requests.
pub const MODEL_REQUESTS_USAGE_LIMIT_HEADER_NAME: &str = "x-zed-model-requests-usage-limit";

/// The name of the header used to indicate the usage amount for model requests.
pub const MODEL_REQUESTS_USAGE_AMOUNT_HEADER_NAME: &str = "x-zed-model-requests-usage-amount";

/// The name of the header used to indicate the usage limit for edit predictions.
pub const EDIT_PREDICTIONS_USAGE_LIMIT_HEADER_NAME: &str = "x-zed-edit-predictions-usage-limit";

/// The name of the header used to indicate the usage amount for edit predictions.
pub const EDIT_PREDICTIONS_USAGE_AMOUNT_HEADER_NAME: &str = "x-zed-edit-predictions-usage-amount";

/// The name of the header used to indicate the resource for which the subscription limit has been reached.
pub const SUBSCRIPTION_LIMIT_RESOURCE_HEADER_NAME: &str = "x-zed-subscription-limit-resource";

pub const MODEL_REQUESTS_RESOURCE_HEADER_VALUE: &str = "model_requests";
pub const EDIT_PREDICTIONS_RESOURCE_HEADER_VALUE: &str = "edit_predictions";

/// The name of the header used to indicate that the maximum number of consecutive tool uses has been reached.
pub const TOOL_USE_LIMIT_REACHED_HEADER_NAME: &str = "x-zed-tool-use-limit-reached";

/// The name of the header used to indicate the minimum required Zed version.
///
/// This can be used to force a Zed upgrade in order to continue communicating
/// with the LLM service.
pub const MINIMUM_REQUIRED_VERSION_HEADER_NAME: &str = "x-zed-minimum-required-version";

/// The name of the header used by the client to indicate to the server that it supports receiving status messages.
pub const CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME: &str =
    "x-zed-client-supports-status-messages";

/// The name of the header used by the server to indicate to the client that it supports sending status messages.
pub const SERVER_SUPPORTS_STATUS_MESSAGES_HEADER_NAME: &str =
    "x-zed-server-supports-status-messages";

/// The name of the header used by the client to indicate that it supports receiving xAI models.
pub const CLIENT_SUPPORTS_X_AI_HEADER_NAME: &str = "x-zed-client-supports-x-ai";

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageLimit {
    Limited(i32),
    Unlimited,
}

impl FromStr for UsageLimit {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "unlimited" => Ok(Self::Unlimited),
            limit => limit
                .parse::<i32>()
                .map(Self::Limited)
                .context("failed to parse limit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Plan {
    V1(PlanV1),
    V2(PlanV2),
}

impl Plan {
    pub fn is_v2(&self) -> bool {
        matches!(self, Self::V2(_))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanV1 {
    #[default]
    #[serde(alias = "Free")]
    ZedFree,
    #[serde(alias = "ZedPro")]
    ZedPro,
    #[serde(alias = "ZedProTrial")]
    ZedProTrial,
}

impl FromStr for PlanV1 {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "zed_free" => Ok(Self::ZedFree),
            "zed_pro" => Ok(Self::ZedPro),
            "zed_pro_trial" => Ok(Self::ZedProTrial),
            plan => Err(anyhow::anyhow!("invalid plan: {plan:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanV2 {
    #[default]
    ZedFree,
    ZedPro,
    ZedProTrial,
}

impl FromStr for PlanV2 {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "zed_free" => Ok(Self::ZedFree),
            "zed_pro" => Ok(Self::ZedPro),
            "zed_pro_trial" => Ok(Self::ZedProTrial),
            plan => Err(anyhow::anyhow!("invalid plan: {plan:?}")),
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, EnumString, EnumIter, Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LanguageModelProvider {
    Anthropic,
    OpenAi,
    Google,
    XAi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictEditsBody {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub outline: Option<String>,
    pub input_events: String,
    pub input_excerpt: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub speculated_output: Option<String>,
    /// Whether the user provided consent for sampling this interaction.
    #[serde(default, alias = "data_collection_permission")]
    pub can_collect_data: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub diagnostic_groups: Option<Vec<(String, serde_json::Value)>>,
    /// Info about the git repository state, only present when can_collect_data is true.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub git_info: Option<PredictEditsGitInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictEditsGitInfo {
    /// SHA of git HEAD commit at time of prediction.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub head_sha: Option<String>,
    /// URL of the remote called `origin`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub remote_origin_url: Option<String>,
    /// URL of the remote called `upstream`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub remote_upstream_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictEditsResponse {
    pub request_id: Uuid,
    pub output_excerpt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptEditPredictionBody {
    pub request_id: Uuid,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionMode {
    Normal,
    Max,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionIntent {
    UserPrompt,
    ToolResults,
    ThreadSummarization,
    ThreadContextSummarization,
    CreateFile,
    EditFile,
    InlineAssist,
    TerminalInlineAssist,
    GenerateGitCommitMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionBody {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub prompt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub intent: Option<CompletionIntent>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<CompletionMode>,
    pub provider: LanguageModelProvider,
    pub model: String,
    pub provider_request: serde_json::Value,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionRequestStatus {
    Queued {
        position: usize,
    },
    Started,
    Failed {
        code: String,
        message: String,
        request_id: Uuid,
        /// Retry duration in seconds.
        retry_after: Option<f64>,
    },
    UsageUpdated {
        amount: usize,
        limit: UsageLimit,
    },
    ToolUseLimitReached,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionEvent<T> {
    Status(CompletionRequestStatus),
    Event(T),
}

impl<T> CompletionEvent<T> {
    pub fn into_status(self) -> Option<CompletionRequestStatus> {
        match self {
            Self::Status(status) => Some(status),
            Self::Event(_) => None,
        }
    }

    pub fn into_event(self) -> Option<T> {
        match self {
            Self::Event(event) => Some(event),
            Self::Status(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WebSearchBody {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebSearchResponse {
    pub results: Vec<WebSearchResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebSearchResult {
    pub title: String,
    pub url: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct CountTokensBody {
    pub provider: LanguageModelProvider,
    pub model: String,
    pub provider_request: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct CountTokensResponse {
    pub tokens: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelId(pub Arc<str>);

impl std::fmt::Display for LanguageModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModel {
    pub provider: LanguageModelProvider,
    pub id: LanguageModelId,
    pub display_name: String,
    pub max_token_count: usize,
    pub max_token_count_in_max_mode: Option<usize>,
    pub max_output_tokens: usize,
    pub supports_tools: bool,
    pub supports_images: bool,
    pub supports_thinking: bool,
    pub supports_max_mode: bool,
    // only used by OpenAI and xAI
    #[serde(default)]
    pub supports_parallel_tool_calls: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub models: Vec<LanguageModel>,
    pub default_model: Option<LanguageModelId>,
    pub default_fast_model: Option<LanguageModelId>,
    pub recommended_models: Vec<LanguageModelId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubscriptionResponse {
    pub plan: PlanV1,
    pub usage: Option<CurrentUsage>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUsage {
    pub model_requests: UsageData,
    pub edit_predictions: UsageData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageData {
    pub used: u32,
    pub limit: UsageLimit,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_plan_v1_deserialize_snake_case() {
        let plan = serde_json::from_value::<PlanV1>(json!("zed_free")).unwrap();
        assert_eq!(plan, PlanV1::ZedFree);

        let plan = serde_json::from_value::<PlanV1>(json!("zed_pro")).unwrap();
        assert_eq!(plan, PlanV1::ZedPro);

        let plan = serde_json::from_value::<PlanV1>(json!("zed_pro_trial")).unwrap();
        assert_eq!(plan, PlanV1::ZedProTrial);
    }

    #[test]
    fn test_plan_v1_deserialize_aliases() {
        let plan = serde_json::from_value::<PlanV1>(json!("Free")).unwrap();
        assert_eq!(plan, PlanV1::ZedFree);

        let plan = serde_json::from_value::<PlanV1>(json!("ZedPro")).unwrap();
        assert_eq!(plan, PlanV1::ZedPro);

        let plan = serde_json::from_value::<PlanV1>(json!("ZedProTrial")).unwrap();
        assert_eq!(plan, PlanV1::ZedProTrial);
    }

    #[test]
    fn test_plan_v2_deserialize_snake_case() {
        let plan = serde_json::from_value::<PlanV2>(json!("zed_free")).unwrap();
        assert_eq!(plan, PlanV2::ZedFree);

        let plan = serde_json::from_value::<PlanV2>(json!("zed_pro")).unwrap();
        assert_eq!(plan, PlanV2::ZedPro);

        let plan = serde_json::from_value::<PlanV2>(json!("zed_pro_trial")).unwrap();
        assert_eq!(plan, PlanV2::ZedProTrial);
    }

    #[test]
    fn test_usage_limit_from_str() {
        let limit = UsageLimit::from_str("unlimited").unwrap();
        assert!(matches!(limit, UsageLimit::Unlimited));

        let limit = UsageLimit::from_str(&0.to_string()).unwrap();
        assert!(matches!(limit, UsageLimit::Limited(0)));

        let limit = UsageLimit::from_str(&50.to_string()).unwrap();
        assert!(matches!(limit, UsageLimit::Limited(50)));

        for value in ["not_a_number", "50xyz"] {
            let limit = UsageLimit::from_str(value);
            assert!(limit.is_err());
        }
    }
}
