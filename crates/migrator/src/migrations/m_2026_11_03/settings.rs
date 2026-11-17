use anyhow::{Result, bail};
use serde_json::{Map, Value};
use settings_content::{PlatformOverrides, ReleaseChannelOverrides};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct LegacyModes {
    vim: bool,
    helix: bool,
}

impl LegacyModes {
    fn selection(self) -> &'static str {
        if self.helix {
            "helix"
        } else if self.vim {
            "vim"
        } else {
            "none"
        }
    }
}

pub fn combine_modal_editing_settings(value: &mut Value) -> Result<()> {
    migrate(value, None)
}

pub fn combine_modal_editing_settings_for_context(
    value: &mut Value,
    release_channel: &str,
    platform: &str,
) -> Result<()> {
    migrate(value, Some((release_channel, platform)))
}

fn migrate(value: &mut Value, context: Option<(&str, &str)>) -> Result<()> {
    let Some(root) = value.as_object_mut() else {
        return Ok(());
    };

    let root_modes = migrate_one(root, &[LegacyModes::default()], "root")?;
    let mut channel_modes = root_modes.clone();
    let mut active_channel_modes = root_modes.clone();

    for key in ReleaseChannelOverrides::OVERRIDE_KEYS {
        if let Some(override_settings) = root.get_mut(*key).and_then(Value::as_object_mut) {
            let modes = migrate_one(override_settings, &root_modes, key)?;
            if context.is_some_and(|(channel, _)| channel == *key) {
                active_channel_modes = modes.clone();
            }
            channel_modes.extend(modes);
        }
    }
    if context.is_some() {
        channel_modes = active_channel_modes;
    }

    let mut user_modes = channel_modes.clone();
    let mut active_user_modes = channel_modes.clone();
    for key in PlatformOverrides::OVERRIDE_KEYS {
        if let Some(override_settings) = root.get_mut(*key).and_then(Value::as_object_mut) {
            let modes = migrate_one(override_settings, &channel_modes, key)?;
            if context.is_some_and(|(_, platform)| platform == *key) {
                active_user_modes = modes.clone();
            }
            user_modes.extend(modes);
        }
    }
    if context.is_some() {
        user_modes = active_user_modes;
    }

    if let Some(profiles) = root.get_mut("profiles").and_then(Value::as_object_mut) {
        for (name, profile) in profiles {
            let Some(profile) = profile.as_object_mut() else {
                continue;
            };
            let inherited = if profile.get("base").and_then(Value::as_str) == Some("default") {
                &[LegacyModes::default()][..]
            } else {
                &user_modes
            };
            if let Some(settings) = profile.get_mut("settings").and_then(Value::as_object_mut) {
                migrate_one(settings, inherited, &format!("profiles.{name}.settings"))?;
            } else {
                migrate_one(profile, inherited, &format!("profiles.{name}"))?;
            }
        }
    }

    Ok(())
}

fn migrate_one(
    settings: &mut Map<String, Value>,
    inherited: &[LegacyModes],
    scope: &str,
) -> Result<Vec<LegacyModes>> {
    let had_legacy_setting =
        settings.contains_key("vim_mode") || settings.contains_key("helix_mode");
    if let Some(mode) = settings.get("modal_editing").and_then(Value::as_str) {
        let modes = match mode {
            "vim" => Some(LegacyModes {
                vim: true,
                helix: false,
            }),
            "helix" => Some(LegacyModes {
                vim: false,
                helix: true,
            }),
            "none" => Some(LegacyModes::default()),
            _ => None,
        };
        settings.remove("vim_mode");
        settings.remove("helix_mode");
        return Ok(inherited
            .iter()
            .map(|inherited| modes.unwrap_or(*inherited))
            .collect());
    }

    if !had_legacy_setting {
        return Ok(inherited.to_vec());
    }

    let vim_mode = settings.get("vim_mode").and_then(Value::as_bool);
    let helix_mode = settings.get("helix_mode").and_then(Value::as_bool);
    let modes: Vec<_> = inherited
        .iter()
        .map(|inherited| LegacyModes {
            vim: vim_mode.unwrap_or(inherited.vim),
            helix: helix_mode.unwrap_or(inherited.helix),
        })
        .collect();

    if let Some(selection) = modes.first().map(|modes| modes.selection())
        && modes.iter().all(|modes| modes.selection() == selection)
    {
        settings.insert("modal_editing".into(), Value::String(selection.into()));
    } else if !modes
        .iter()
        .zip(inherited)
        .all(|(modes, inherited)| modes.selection() == inherited.selection())
    {
        // Partial boolean overrides can depend on both the release channel and
        // platform. A single enum at this scope cannot encode that dependency.
        bail!(
            "The Vim/Helix overrides in `{scope}` cannot be converted without changing behavior on another release channel or platform. Replace those overrides with explicit `modal_editing` settings. Your existing editing mode is still used until then."
        );
    }
    settings.remove("vim_mode");
    settings.remove("helix_mode");

    Ok(modes)
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
                "direct": { "vim_mode": true, "helix_mode": false },
                "wrapped": { "settings": { "helix_mode": true, "vim_mode": true } }
            }
        });

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(value["modal_editing"], "vim");
        assert_eq!(value["macos"]["modal_editing"], "helix");
        assert_eq!(value["preview"]["modal_editing"], "none");
        assert_eq!(value["profiles"]["direct"]["modal_editing"], "vim");
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
        assert_eq!(value["macos"]["modal_editing"], "vim");
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

        assert_eq!(
            value["profiles"]["user"]["settings"]["modal_editing"],
            "vim"
        );
        assert_eq!(
            value["profiles"]["default"]["settings"]["modal_editing"],
            "none"
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

    #[test]
    fn preserves_explicit_platform_disable_over_channel_enable() {
        let mut value = json!({
            "preview": { "vim_mode": true },
            "macos": { "vim_mode": false }
        });
        let original = value.clone();

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(value["macos"]["modal_editing"], "none");
        assert_effective_modes(&original, &value, "preview", "macos", None);
        assert_effective_modes(&original, &value, "preview", "linux", None);
        assert_effective_modes(&original, &value, "stable", "macos", None);
    }

    #[test]
    fn preserves_explicit_profile_disable_over_channel_enable() {
        let mut value = json!({
            "preview": { "vim_mode": true },
            "profiles": { "standard": { "settings": { "vim_mode": false } } }
        });
        let original = value.clone();

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(
            value["profiles"]["standard"]["settings"]["modal_editing"],
            "none"
        );
        assert_effective_modes(&original, &value, "preview", "macos", Some("standard"));
    }

    #[test]
    fn keeps_explicit_overrides_that_match_the_root() {
        let mut value = json!({
            "vim_mode": true,
            "macos": { "vim_mode": true },
            "profiles": { "vim": { "settings": { "vim_mode": true } } }
        });

        combine_modal_editing_settings(&mut value).expect("migration should succeed");

        assert_eq!(value["macos"]["modal_editing"], "vim");
        assert_eq!(value["profiles"]["vim"]["settings"]["modal_editing"], "vim");
    }

    #[test]
    fn reports_overrides_that_cannot_be_represented_by_one_selection() {
        for (input, scope) in [
            (
                json!({
                    "vim_mode": true,
                    "preview": { "helix_mode": true },
                    "macos": { "vim_mode": false }
                }),
                "macos",
            ),
            (
                json!({
                    "preview": { "helix_mode": true },
                    "macos": { "vim_mode": true }
                }),
                "macos",
            ),
            (
                json!({
                    "helix_mode": true,
                    "preview": { "vim_mode": true },
                    "profiles": { "vim": { "settings": { "helix_mode": false } } }
                }),
                "profiles.vim.settings",
            ),
        ] {
            let error = combine_modal_editing_settings(&mut input.clone())
                .expect_err("context-dependent overrides must not be rewritten on disk");
            assert!(error.to_string().contains(scope));

            for channel in ["stable", "preview"] {
                for platform in ["macos", "linux"] {
                    let mut migrated = input.clone();
                    combine_modal_editing_settings_for_context(&mut migrated, channel, platform)
                        .expect("in-memory migration should succeed");
                    for profile in [None, Some("vim")] {
                        assert_effective_modes(&input, &migrated, channel, platform, profile);
                    }
                }
            }
        }
    }

    #[test]
    fn preserves_effective_modes_for_all_partial_boolean_overrides() {
        let overrides: Vec<_> = [None, Some(false), Some(true)]
            .into_iter()
            .flat_map(|vim| {
                [None, Some(false), Some(true)]
                    .into_iter()
                    .map(move |helix| {
                        let mut settings = Map::new();
                        if let Some(vim) = vim {
                            settings.insert("vim_mode".into(), vim.into());
                        }
                        if let Some(helix) = helix {
                            settings.insert("helix_mode".into(), helix.into());
                        }
                        settings
                    })
            })
            .collect();

        for root in &overrides {
            for channel in &overrides {
                for platform in &overrides {
                    for profile in &overrides {
                        let mut input = Value::Object(root.clone());
                        input["preview"] = Value::Object(channel.clone());
                        input["macos"] = Value::Object(platform.clone());
                        input["profiles"] = json!({
                            "user": { "settings": profile },
                            "default": { "base": "default", "settings": profile }
                        });
                        let mut persistent = input.clone();
                        let persistent = combine_modal_editing_settings(&mut persistent)
                            .is_ok()
                            .then_some(persistent);

                        for channel in ["stable", "preview"] {
                            for platform in ["macos", "linux"] {
                                let mut contextual = input.clone();
                                combine_modal_editing_settings_for_context(
                                    &mut contextual,
                                    channel,
                                    platform,
                                )
                                .expect("contextual migration should succeed");

                                for profile in [None, Some("user"), Some("default")] {
                                    assert_effective_modes(
                                        &input,
                                        &contextual,
                                        channel,
                                        platform,
                                        profile,
                                    );
                                    if let Some(persistent) = &persistent {
                                        assert_effective_modes(
                                            &input, persistent, channel, platform, profile,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn assert_effective_modes(
        original: &Value,
        migrated: &Value,
        channel: &str,
        platform: &str,
        profile: Option<&str>,
    ) {
        assert_eq!(
            effective_mode(original, channel, platform, profile),
            effective_mode(migrated, channel, platform, profile),
            "channel={channel}, platform={platform}, profile={profile:?}, original={original}, migrated={migrated}"
        );
    }

    fn effective_mode(
        settings: &Value,
        channel: &str,
        platform: &str,
        profile: Option<&str>,
    ) -> &'static str {
        let mut vim = false;
        let mut helix = false;
        let profile = profile.and_then(|name| settings.get("profiles")?.get(name));
        let mut scopes = Vec::new();
        if profile
            .and_then(|profile| profile.get("base"))
            .and_then(Value::as_str)
            != Some("default")
        {
            scopes.extend([
                Some(settings),
                settings.get(channel),
                settings.get(platform),
            ]);
        }
        scopes.push(profile.and_then(|profile| profile.get("settings")));
        for scope in scopes.into_iter().flatten() {
            if let Some(mode) = scope.get("modal_editing").and_then(Value::as_str) {
                vim = mode == "vim";
                helix = mode == "helix";
            } else {
                vim = scope
                    .get("vim_mode")
                    .and_then(Value::as_bool)
                    .unwrap_or(vim);
                helix = scope
                    .get("helix_mode")
                    .and_then(Value::as_bool)
                    .unwrap_or(helix);
            }
        }
        if helix {
            "helix"
        } else if vim {
            "vim"
        } else {
            "none"
        }
    }
}
