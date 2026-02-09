use anyhow::Result;
use serde_json::Value;
use settings_content::{PlatformOverrides, ReleaseChannelOverrides};

/// Applies a migration callback to the root settings object as well as all
/// nested platform, release-channel, and profile override objects.
pub(crate) fn migrate_settings(
    value: &mut Value,
    mut migrate_one: impl FnMut(&mut serde_json::Map<String, Value>) -> Result<()>,
) -> Result<()> {
    let Some(root_object) = value.as_object_mut() else {
        return Ok(());
    };

    migrate_one(root_object)?;

    let override_keys = ReleaseChannelOverrides::OVERRIDE_KEYS
        .iter()
        .copied()
        .chain(PlatformOverrides::OVERRIDE_KEYS.iter().copied());

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

/// Applies a migration callback to a value and its `languages` children,
/// at the root level as well as all nested platform, release-channel, and
/// profile override objects.
pub(crate) fn migrate_language_setting(
    value: &mut Value,
    migrate_fn: fn(&mut Value, path: &[&str]) -> Result<()>,
) -> Result<()> {
    fn apply_to_value_and_languages(
        value: &mut Value,
        prefix: &[&str],
        migrate_fn: fn(&mut Value, path: &[&str]) -> Result<()>,
    ) -> Result<()> {
        migrate_fn(value, prefix)?;
        let languages = value
            .as_object_mut()
            .and_then(|obj| obj.get_mut("languages"))
            .and_then(|languages| languages.as_object_mut());
        if let Some(languages) = languages {
            for (language_name, language) in languages.iter_mut() {
                let mut path: Vec<&str> = prefix.to_vec();
                path.push("languages");
                path.push(language_name);
                migrate_fn(language, &path)?;
            }
        }
        Ok(())
    }

    if !value.is_object() {
        return Ok(());
    }

    apply_to_value_and_languages(value, &[], migrate_fn)?;

    let Some(root_object) = value.as_object_mut() else {
        return Ok(());
    };

    let override_keys = ReleaseChannelOverrides::OVERRIDE_KEYS
        .iter()
        .copied()
        .chain(PlatformOverrides::OVERRIDE_KEYS.iter().copied());

    for key in override_keys {
        if let Some(sub_value) = root_object.get_mut(key) {
            apply_to_value_and_languages(sub_value, &[key], migrate_fn)?;
        }
    }

    if let Some(profiles) = root_object.get_mut("profiles") {
        if let Some(profiles_object) = profiles.as_object_mut() {
            let profile_names: Vec<String> = profiles_object.keys().cloned().collect();
            for profile_name in &profile_names {
                if let Some(profile_settings) = profiles_object.get_mut(profile_name.as_str()) {
                    apply_to_value_and_languages(
                        profile_settings,
                        &["profiles", profile_name],
                        migrate_fn,
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub(crate) mod m_2025_01_02 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_01_29 {
    mod keymap;
    mod settings;

    pub(crate) use keymap::KEYMAP_PATTERNS;
    pub(crate) use settings::{SETTINGS_PATTERNS, replace_edit_prediction_provider_setting};
}

pub(crate) mod m_2025_01_30 {
    mod keymap;
    mod settings;

    pub(crate) use keymap::KEYMAP_PATTERNS;
    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_03_03 {
    mod keymap;

    pub(crate) use keymap::KEYMAP_PATTERNS;
}

pub(crate) mod m_2025_03_06 {
    mod keymap;

    pub(crate) use keymap::KEYMAP_PATTERNS;
}

pub(crate) mod m_2025_03_29 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_04_15 {
    mod keymap;
    mod settings;

    pub(crate) use keymap::KEYMAP_PATTERNS;
    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_04_21 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_04_23 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_05_05 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_05_08 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_06_16 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_06_25 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_06_27 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_07_08 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_10_01 {
    mod settings;

    pub(crate) use settings::flatten_code_actions_formatters;
}

pub(crate) mod m_2025_10_02 {
    mod settings;

    pub(crate) use settings::remove_formatters_on_save;
}

pub(crate) mod m_2025_10_03 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_10_16 {
    mod settings;

    pub(crate) use settings::restore_code_actions_on_format;
}

pub(crate) mod m_2025_10_17 {
    mod settings;

    pub(crate) use settings::make_file_finder_include_ignored_an_enum;
}

pub(crate) mod m_2025_10_21 {
    mod settings;

    pub(crate) use settings::make_relative_line_numbers_an_enum;
}

pub(crate) mod m_2025_11_12 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_11_20 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_11_25 {
    mod settings;

    pub(crate) use settings::remove_context_server_source;
}

pub(crate) mod m_2025_12_01 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_12_08 {
    mod keymap;

    pub(crate) use keymap::KEYMAP_PATTERNS;
}

pub(crate) mod m_2025_12_15 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2026_02_02 {
    mod settings;

    pub(crate) use settings::move_edit_prediction_provider_to_edit_predictions;
}

pub(crate) mod m_2026_02_03 {
    mod settings;

    pub(crate) use settings::migrate_experimental_sweep_mercury;
}
