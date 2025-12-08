use crate::FeatureFlag;

pub struct NotebookFeatureFlag;

impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
}

pub struct PanicFeatureFlag;

impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
}

pub struct InlineAssistantV2FeatureFlag;

impl FeatureFlag for InlineAssistantV2FeatureFlag {
    const NAME: &'static str = "inline-assistant-v2";
}
