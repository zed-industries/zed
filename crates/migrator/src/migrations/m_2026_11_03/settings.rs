use anyhow::Result;
use serde_json::{Map, Value};
use settings_content::{PlatformOverrides, ReleaseChannelOverrides};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct LegacyModes {
    vim: bool,
    helix: bool,
}

pub fn combine_modal_editing_settings(value: &mut Value) -> Result<()> {
    let Some(root) = value.as_object_mut() else {
        return Ok(());
    };

    let root_modes = migrate_one(root, LegacyModes::default(), true);

    for key in ReleaseChannelOverrides::OVERRIDE_KEYS
        .iter()
        .chain(PlatformOverrides::OVERRIDE_KEYS)
    {
        if let Some(override_settings) = root.get_mut(*key).and_then(Value::as_object_mut) {
            migrate_one(override_settings, root_modes, false);
        }
    }

    if let Some(profiles) = root.get_mut("profiles").and_then(Value::as_object_mut) {
        for profile in profiles.values_mut().filter_map(Value::as_object_mut) {
            let inherited = if profile.get("base").and_then(Value::as_str) == Some("default") {
                LegacyModes::default()
            } else {
                root_modes
            };
            if let Some(settings) = profile.get_mut("settings").and_then(Value::as_object_mut) {
                migrate_one(settings, inherited, false);
            } else {
                migrate_one(profile, inherited, false);
            }
        }
    }

    Ok(())
}

fn migrate_one(
    settings: &mut Map<String, Value>,
    inherited: LegacyModes,
    is_root: bool,
) -> LegacyModes {
    let had_legacy_setting =
        settings.contains_key("vim_mode") || settings.contains_key("helix_mode");
    let vim_mode = settings.remove("vim_mode");
    let helix_mode = settings.remove("helix_mode");

    if let Some(mode) = settings.get("modal_editing").and_then(Value::as_str) {
        return match mode {
            "vim" => LegacyModes {
                vim: true,
                helix: false,
            },
            "helix" => LegacyModes {
                vim: false,
                helix: true,
            },
            "none" => LegacyModes::default(),
            _ => inherited,
        };
    }

    let modes = LegacyModes {
        vim: vim_mode
            .and_then(|value| value.as_bool())
            .unwrap_or(inherited.vim),
        helix: helix_mode
            .and_then(|value| value.as_bool())
            .unwrap_or(inherited.helix),
    };

    if !had_legacy_setting || (!is_root && modes == inherited) {
        return modes;
    }

    let modal_editing = if modes.helix {
        "helix"
    } else if modes.vim {
        "vim"
    } else {
        "none"
    };
    settings.insert(
        "modal_editing".to_string(),
        Value::String(modal_editing.to_string()),
    );

    modes
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn combines_legacy_settings_in_all_settings_scopes() {
        let mut value = json!({
            "vim_mode": true,
            "macos": { "helix_mode": true },
            "preview": { "vim_mode": false },
            "profiles": {
                "direct": { "vim_mode": true },
                "wrapped": { "settings": { "helix_mode": true, "vim_mode": true } }
            }
        });

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(value["modal_editing"], "vim");
        assert_eq!(value["macos"]["modal_editing"], "helix");
        assert_eq!(value["preview"]["modal_editing"], "none");
        assert!(value["profiles"]["direct"].get("modal_editing").is_none());
        assert_eq!(
            value["profiles"]["wrapped"]["settings"]["modal_editing"],
            "helix"
        );

        for scope in [
            &value,
            &value["macos"],
            &value["preview"],
            &value["profiles"]["direct"],
            &value["profiles"]["wrapped"]["settings"],
        ] {
            assert!(scope.get("vim_mode").is_none());
            assert!(scope.get("helix_mode").is_none());
        }
    }

    #[test]
    fn keeps_new_setting_and_removes_legacy_settings() {
        let mut value = json!({
            "modal_editing": "none",
            "vim_mode": true,
            "helix_mode": true
        });

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(value, json!({ "modal_editing": "none" }));
    }

    #[test]
    fn preserves_inherited_legacy_modes_in_overrides() {
        let mut value = json!({
            "vim_mode": true,
            "macos": { "helix_mode": false },
            "linux": { "helix_mode": true },
            "windows": { "vim_mode": false }
        });

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(value["modal_editing"], "vim");
        assert!(value["macos"].get("modal_editing").is_none());
        assert_eq!(value["linux"]["modal_editing"], "helix");
        assert_eq!(value["windows"]["modal_editing"], "none");
    }

    #[test]
    fn profile_base_controls_inherited_mode() {
        let mut value = json!({
            "vim_mode": true,
            "profiles": {
                "user": { "settings": { "helix_mode": false } },
                "default": {
                    "base": "default",
                    "settings": { "helix_mode": false }
                }
            }
        });

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert!(
            value["profiles"]["user"]["settings"]
                .get("modal_editing")
                .is_none()
        );
        assert!(
            value["profiles"]["default"]["settings"]
                .get("modal_editing")
                .is_none()
        );
    }

    #[test]
    fn no_op_platform_override_does_not_clobber_release_override() {
        let mut value = json!({
            "vim_mode": true,
            "preview": { "vim_mode": false },
            "macos": { "helix_mode": false }
        });

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(value["modal_editing"], "vim");
        assert_eq!(value["preview"]["modal_editing"], "none");
        assert!(value["macos"].get("modal_editing").is_none());
    }

    #[test]
    fn ignores_settings_without_legacy_keys() {
        let mut value = json!({ "theme": "One Dark" });
        let original = value.clone();

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(value, original);
    }
}
