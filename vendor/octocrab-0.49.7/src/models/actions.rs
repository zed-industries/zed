use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunner {
    pub id: RunnerId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_group_id: Option<RunnerGroupId>,
    pub name: String,
    pub os: String,
    pub status: String,
    pub busy: bool,
    pub labels: Vec<SelfHostedRunnerLabel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunnerLabel {
    pub id: RunnerLabelId,
    pub name: String,
    #[serde(rename = "type")]
    pub label_type: SelfHostedRunnerLabelType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum SelfHostedRunnerLabelType {
    ReadOnly,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunnerJitConfig {
    pub runner: SelfHostedRunner,
    pub encoded_jit_config: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SelfHostedRunnerToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}
