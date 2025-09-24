use crate::FeatureFlag;

pub struct PredictEditsRateCompletionsFeatureFlag;

impl FeatureFlag for PredictEditsRateCompletionsFeatureFlag {
    const NAME: &'static str = "predict-edits-rate-completions";
}

pub struct BillingV2FeatureFlag {}

impl FeatureFlag for BillingV2FeatureFlag {
    const NAME: &'static str = "billing-v2";

    fn enabled_for_all() -> bool {
        true
    }
}

pub struct NotebookFeatureFlag;

impl FeatureFlag for NotebookFeatureFlag {
    const NAME: &'static str = "notebooks";
}

pub struct PanicFeatureFlag;

impl FeatureFlag for PanicFeatureFlag {
    const NAME: &'static str = "panic";
}
