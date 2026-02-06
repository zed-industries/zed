use anyhow::Result;
use release_channel::{ReleaseChannel, SupportedPlatform};
use serde_json::Value;
use strum::IntoEnumIterator as _;

pub fn move_edit_prediction_provider_to_edit_predictions(value: &mut Value) -> Result<()> {
    let Some(root_object) = value.as_object_mut() else {
        return Ok(());
    };

    migrate_one(root_object)?;

    let override_keys = ReleaseChannel::iter()
        .map(|channel| channel.dev_name())
        .chain(SupportedPlatform::iter().map(|platform| platform.as_str()));

    for key in override_keys {
        if let Some(sub_object) = root_object.get_mut(key) {
            if let Some(sub_map) = sub_object.as_object_mut() {
                migrate_one(sub_map)?;
            }
        }
    }

    if let Some(profiles) = root_object.get_mut("profiles") {
        if let Some(profiles_object) = profiles.as_object_mut() {
            for (_profile_name, profile_settings) in profiles_object.iter_mut() {
                if let Some(profile_map) = profile_settings.as_object_mut() {
                    migrate_one(profile_map)?;
                }
            }
        }
    }

    Ok(())
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
        anyhow::bail!("Expected edit_predictions to be an object");
    };

    if !edit_predictions_obj.contains_key("provider") {
        edit_predictions_obj.insert("provider".to_string(), provider);
    }

    Ok(())
}
