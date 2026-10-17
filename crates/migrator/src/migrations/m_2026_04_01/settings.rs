use anyhow::Result;
use serde_json::Value;

pub fn restructure_profiles_with_settings_key(value: &mut Value) -> Result<()> {
    let Some(root_object) = value.as_object_mut() else {
        return Ok(());
    };

    let Some(profiles) = root_object.get_mut("profiles") else {
        return Ok(());
    };

    let Some(profiles_map) = profiles.as_object_mut() else {
        return Ok(());
    };

    for profile_value in profiles_map.values_mut() {
        if profile_value
            .as_object()
            .is_some_and(|m| m.contains_key("settings") || m.contains_key("base"))
        {
            continue;
        }

        *profile_value = serde_json::json!({ "settings": profile_value });
    }

    Ok(())
}
