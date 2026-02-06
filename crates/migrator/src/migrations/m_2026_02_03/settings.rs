use anyhow::Result;
use release_channel::{ReleaseChannel, SupportedPlatform};
use serde_json::Value;
use strum::IntoEnumIterator as _;

pub fn migrate_experimental_sweep_mercury(value: &mut Value) -> Result<()> {
    let Some(root_object) = value.as_object_mut() else {
        return Ok(());
    };

    migrate_one(root_object);

    let override_keys = ReleaseChannel::iter()
        .map(|channel| channel.dev_name())
        .chain(SupportedPlatform::iter().map(|platform| platform.as_str()));

    for key in override_keys {
        if let Some(sub_object) = root_object.get_mut(key) {
            if let Some(sub_map) = sub_object.as_object_mut() {
                migrate_one(sub_map);
            }
        }
    }

    if let Some(profiles) = root_object.get_mut("profiles") {
        if let Some(profiles_object) = profiles.as_object_mut() {
            for (_profile_name, profile_settings) in profiles_object.iter_mut() {
                if let Some(profile_map) = profile_settings.as_object_mut() {
                    migrate_one(profile_map);
                }
            }
        }
    }

    Ok(())
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
