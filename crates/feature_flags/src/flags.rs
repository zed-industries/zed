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

pub struct EditPredictionV2FeatureFlag;

impl FeatureFlag for EditPredictionV2FeatureFlag {
    const NAME: &'static str = "edit-prediction-v2";
}
