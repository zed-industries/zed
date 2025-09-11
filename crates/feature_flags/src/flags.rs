use crate::FeatureFlag;

pub struct PredictEditsRateCompletionsFeatureFlag;

impl FeatureFlag for PredictEditsRateCompletionsFeatureFlag {
    const NAME: &'static str = "predict-edits-rate-completions";
}

pub struct BillingV2FeatureFlag {}

impl FeatureFlag for BillingV2FeatureFlag {
    const NAME: &'static str = "billing-v2";
}

pub struct NotebookFeatureFlag;

impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
}

pub struct PanicFeatureFlag;

impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
}

pub struct JjUiFeatureFlag {}

impl FeatureFlag for JjUiFeatureFlag {
    const NAME: &'static str = "jj-ui";
}

pub struct GeminiAndNativeFeatureFlag;

impl FeatureFlag for GeminiAndNativeFeatureFlag {
    // This was previously called "acp".
    //
    // We renamed it because existing builds used it to enable the Claude Code
    // integration too, and we'd like to turn Gemini/Native on in new builds
    // without enabling Claude Code in old builds.
    const NAME: &'static str = "gemini-and-native";

    fn enabled_for_all() -> bool {
        true
    }
}

pub struct ClaudeCodeFeatureFlag;

impl FeatureFlag for ClaudeCodeFeatureFlag {
    const NAME: &'static str = "claude-code";

    fn enabled_for_all() -> bool {
        true
    }
}
