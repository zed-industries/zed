use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn move_edit_prediction_provider_to_edit_predictions(value: &mut Value) -> Result<()> {
    migrate_settings(value, migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(features) = obj.get_mut("features") else {
        return Ok(());
    };

    let Some(features_obj) = features.as_object_mut() else {
        return Ok(());
    };

    let Some(provider) = features_obj.remove("edit_prediction_provider") else {
        return Ok(());
    };

    let features_is_empty = features_obj.is_empty();

    if features_is_empty {
        obj.remove("features");
    }

    let edit_predictions = obj
        .entry("edit_predictions")
        .or_insert_with(|| Value::Object(Default::default()));

    let Some(edit_predictions_obj) = edit_predictions.as_object_mut() else {
        return Ok(());
    };

    if !edit_predictions_obj.contains_key("provider") {
        edit_predictions_obj.insert("provider".to_string(), provider);
    }

    Ok(())
}
