use crate::FeatureFlag;

pub struct PredictEditsRateCompletionsFeatureFlag;

impl FeatureFlag for PredictEditsRateCompletionsFeatureFlag {
    const NAME: &'static str = "predict-edits-rate-completions";
}

pub struct NotebookFeatureFlag;

impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
}

pub struct PanicFeatureFlag;

impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
}

pub struct CodexAcpFeatureFlag;

impl FeatureFlag for CodexAcpFeatureFlag {
    const NAME: &'static str = "codex-acp";
}
