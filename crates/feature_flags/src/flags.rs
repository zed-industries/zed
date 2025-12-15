use crate::FeatureFlag;

pub struct NotebookFeatureFlag;

impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
}

pub struct PanicFeatureFlag;

impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
}

pub struct InlineAssistantUseToolFeatureFlag;

impl FeatureFlag for InlineAssistantUseToolFeatureFlag {
    const NAME: &'static str = "inline-assistant-use-tool";

    fn enabled_for_staff() -> bool {
        true
    }
}
