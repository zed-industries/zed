use editor::test::editor_test_context::EditorTestContext;
use feature_flags::{FeatureFlag as _, FeatureFlagAppExt as _};
use gpui::{TestAppContext, UpdateGlobal as _};
use settings::SettingsStore;
use vim_mode_setting::HelixNextFeatureFlag;

use crate::HelixAddon;

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        editor::init(cx);
        crate::init(cx);
    });
}

fn set_helix_mode(enabled: bool, cx: &mut EditorTestContext) {
    cx.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| settings.helix_mode = Some(enabled));
        });
    });
}

fn set_helix_next_flag(enabled: bool, cx: &mut EditorTestContext) {
    cx.update(|_, cx| {
        let flags = if enabled {
            vec![HelixNextFeatureFlag::NAME.to_string()]
        } else {
            Vec::new()
        };
        cx.update_flags(false, flags);
    });
}

fn has_helix_addon(cx: &mut EditorTestContext) -> bool {
    cx.editor(|editor, _, _| editor.addon::<HelixAddon>().is_some())
}

fn key_context_has_helix(cx: &mut EditorTestContext) -> bool {
    cx.update_editor(|editor, window, cx| editor.key_context(window, cx).contains("Helix"))
}

#[gpui::test]
async fn test_addon_follows_setting_and_flag(cx: &mut TestAppContext) {
    init_test(cx);
    let mut cx = EditorTestContext::new(cx).await;
    assert!(!has_helix_addon(&mut cx));

    set_helix_mode(true, &mut cx);
    assert!(!has_helix_addon(&mut cx), "the flag is required");

    set_helix_next_flag(true, &mut cx);
    assert!(has_helix_addon(&mut cx));
    assert!(key_context_has_helix(&mut cx));

    set_helix_mode(false, &mut cx);
    assert!(!has_helix_addon(&mut cx));

    set_helix_mode(true, &mut cx);
    assert!(has_helix_addon(&mut cx));

    set_helix_next_flag(false, &mut cx);
    assert!(!has_helix_addon(&mut cx));
}
