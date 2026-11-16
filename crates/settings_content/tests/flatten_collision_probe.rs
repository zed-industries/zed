use std::collections::BTreeMap;

const KNOWN_SHADOWED_KEYS: &[(&str, &str, &str)] = &[("SettingsContent", "terminal", "project")];

#[test]
fn no_key_collisions_between_named_fields_and_flattened_sections() {
    check(
        "SettingsContent",
        settings_content::SettingsContent::NAMED_DESERIALIZE_KEYS,
        vec![
            section::<settings_content::ProjectSettingsContent>("project"),
            section::<settings_content::ThemeSettingsContent>("theme"),
            section::<settings_content::ExtensionSettingsContent>("extension"),
            section::<settings_content::WorkspaceSettingsContent>("workspace"),
            section::<settings_content::EditorSettingsContent>("editor"),
            section::<settings_content::RemoteSettingsContent>("remote"),
        ],
    );
    check(
        "UserSettingsContent",
        settings_content::UserSettingsContent::NAMED_DESERIALIZE_KEYS,
        vec![
            section::<settings_content::SettingsContent>("content"),
            section::<settings_content::ReleaseChannelOverrides>("release_channel_overrides"),
            section::<settings_content::PlatformOverrides>("platform_overrides"),
        ],
    );
    check(
        "ProjectSettingsContent",
        settings_content::ProjectSettingsContent::NAMED_DESERIALIZE_KEYS,
        vec![
            section::<settings_content::AllLanguageSettingsContent>("all_languages"),
            section::<settings_content::WorktreeSettingsContent>("worktree"),
        ],
    );
    check(
        "AllLanguageSettingsContent",
        settings_content::AllLanguageSettingsContent::NAMED_DESERIALIZE_KEYS,
        vec![section::<settings_content::LanguageSettingsContent>(
            "defaults",
        )],
    );
    check(
        "TerminalSettingsContent",
        settings_content::TerminalSettingsContent::NAMED_DESERIALIZE_KEYS,
        vec![section::<settings_content::ProjectTerminalSettingsContent>(
            "project",
        )],
    );
}

fn property_names<T: schemars::JsonSchema>() -> Vec<String> {
    let schema = schemars::schema_for!(T);
    let value = serde_json::to_value(&schema).unwrap();
    value
        .get("properties")
        .and_then(|properties| properties.as_object())
        .map(|object| object.keys().cloned().collect())
        .unwrap_or_default()
}

fn section<T: schemars::JsonSchema>(name: &'static str) -> (&'static str, Vec<String>) {
    (name, property_names::<T>())
}

fn check(container: &str, named_keys: &[&str], sections: Vec<(&'static str, Vec<String>)>) {
    let mut owners = BTreeMap::<&str, Vec<&str>>::new();
    for (section_name, keys) in &sections {
        for key in keys {
            owners.entry(key).or_default().push(section_name);
        }
    }

    for named_key in named_keys {
        let Some(shadowed_sections) = owners.get(named_key) else {
            continue;
        };
        let known = shadowed_sections.iter().all(|section_name| {
            KNOWN_SHADOWED_KEYS.contains(&(container, named_key, section_name))
        });
        assert!(
            known,
            "{container}: named field `{named_key}` shadows the same key in flattened \
             section(s) {shadowed_sections:?}: the named field wins and the section never \
             sees the key; if intended, add it to KNOWN_SHADOWED_KEYS"
        );
    }
    for (key, sections_with_key) in owners {
        assert_eq!(
            sections_with_key.len(),
            1,
            "{container}: key `{key}` is provided by multiple flattened sections: \
             {sections_with_key:?}"
        );
    }
}
