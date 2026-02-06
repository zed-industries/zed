use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn migrate_experimental_sweep_mercury(value: &mut Value) -> Result<()> {
    migrate_settings(value, |obj| {
        migrate_one(obj);
        Ok(())
    })
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) {
    if let Some(edit_predictions) = obj.get_mut("edit_predictions") {
        if let Some(edit_predictions_obj) = edit_predictions.as_object_mut() {
            migrate_provider_field(edit_predictions_obj, "provider");
        }
    }

    if let Some(features) = obj.get_mut("features") {
        if let Some(features_obj) = features.as_object_mut() {
            migrate_provider_field(features_obj, "edit_prediction_provider");
        }
    }
}

fn migrate_provider_field(obj: &mut serde_json::Map<String, Value>, field_name: &str) {
    let Some(provider) = obj.get(field_name) else {
        return;
    };

    let Some(provider_obj) = provider.as_object() else {
        return;
    };

    let Some(experimental_name) = provider_obj.get("experimental") else {
        return;
    };

    let Some(name) = experimental_name.as_str() else {
        return;
    };

    if name == "sweep" || name == "mercury" {
        obj.insert(field_name.to_string(), Value::String(name.to_string()));
    }
}
