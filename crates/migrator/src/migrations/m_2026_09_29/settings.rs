use anyhow::Result;
use serde_json::{Map, Value};

use crate::migrations::migrate_settings;

pub fn move_copilot_enterprise_uri(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(settings: &mut Map<String, Value>) -> Result<()> {
    if settings
        .get("copilot")
        .is_some_and(|value| !value.is_object())
    {
        return Ok(());
    }

    let Some(edit_predictions) = settings
        .get_mut("edit_predictions")
        .and_then(Value::as_object_mut)
    else {
        return Ok(());
    };
    let Some(prediction_copilot) = edit_predictions
        .get_mut("copilot")
        .and_then(Value::as_object_mut)
    else {
        return Ok(());
    };
    let Some(enterprise_uri) = prediction_copilot.remove("enterprise_uri") else {
        return Ok(());
    };
    if prediction_copilot.is_empty() {
        edit_predictions.remove("copilot");
    }
    if edit_predictions.is_empty() {
        settings.remove("edit_predictions");
    }

    let copilot = settings
        .entry("copilot")
        .or_insert_with(|| Value::Object(Map::new()));
    if let Value::Object(copilot) = copilot {
        copilot.entry("enterprise_uri").or_insert(enterprise_uri);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn moves_enterprise_uri_in_all_settings_scopes_without_overwriting_existing_values() {
        let legacy = json!({
            "edit_predictions": {
                "copilot": { "enterprise_uri": "https://legacy.example", "other": true },
                "provider": "copilot"
            }
        });
        let mut value = json!({
            "edit_predictions": legacy["edit_predictions"],
            "macos": legacy,
            "preview": legacy,
            "profiles": {
                "direct": legacy,
                "wrapped": { "settings": legacy },
                "existing": {
                    "settings": {
                        "edit_predictions": { "copilot": { "enterprise_uri": "https://legacy.example" } },
                        "copilot": { "enterprise_uri": null, "other": true }
                    }
                }
            }
        });

        move_copilot_enterprise_uri(&mut value).expect("migration should succeed");
        for scope in [
            &value,
            &value["macos"],
            &value["preview"],
            &value["profiles"]["direct"],
            &value["profiles"]["wrapped"]["settings"],
        ] {
            assert_eq!(scope["copilot"]["enterprise_uri"], "https://legacy.example");
            assert_eq!(
                scope["edit_predictions"],
                json!({
                    "copilot": { "other": true },
                    "provider": "copilot"
                })
            );
        }
        assert_eq!(
            value["profiles"]["existing"]["settings"],
            json!({
                "copilot": { "enterprise_uri": null, "other": true }
            })
        );
    }
}
