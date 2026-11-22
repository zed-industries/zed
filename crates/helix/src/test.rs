use editor::test::editor_test_context::EditorTestContext;
use feature_flags::{FeatureFlag as _, FeatureFlagAppExt as _};
use gpui::{ClipboardItem, KeyBinding, KeyContext, TestAppContext, UpdateGlobal as _};
use indoc::indoc;
use settings::{HELIX_KEYMAP_PATH, KeybindSource, KeymapFile, SettingsStore, UseSystemClipboard};
use vim_mode_setting::HelixNextFeatureFlag;

use crate::{
    DEFAULT_REGISTER, HelixAddon, HelixRegisters, Mode,
    mode::{InsertMode, NormalMode, SelectMode},
};

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        editor::init(cx);
        crate::init(cx);
        let mut bindings =
            KeymapFile::load_asset(HELIX_KEYMAP_PATH, Some(KeybindSource::Helix), cx)
                .expect("helix keymap should load");
        for binding in &mut bindings {
            binding.set_meta(KeybindSource::Helix.meta());
        }
        cx.bind_keys(bindings);
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

async fn helix_editor(cx: &mut TestAppContext) -> EditorTestContext {
    init_test(cx);
    let mut cx = EditorTestContext::new(cx).await;
    set_helix_mode(true, &mut cx);
    set_helix_next_flag(true, &mut cx);
    cx
}

fn mode(cx: &mut EditorTestContext) -> Option<Mode> {
    cx.editor(|editor, _, cx| {
        editor
            .addon::<HelixAddon>()
            .map(|addon| addon.entity.read(cx).mode)
    })
}

fn key_context(cx: &mut EditorTestContext) -> KeyContext {
    cx.update_editor(|editor, window, cx| editor.key_context(window, cx))
}

#[gpui::test]
async fn test_mode_switching(cx: &mut TestAppContext) {
    let mut cx = helix_editor(cx).await;
    cx.set_state("ˇabc");
    assert_eq!(mode(&mut cx), Some(Mode::Normal));
    cx.assert_editor_state("«aˇ»bc");
    let context = key_context(&mut cx);
    assert!(context.contains("Helix"));
    assert_eq!(
        context.get("helix_mode").map(|mode| mode.as_ref()),
        Some("normal")
    );

    cx.dispatch_action(SelectMode);
    assert_eq!(mode(&mut cx), Some(Mode::Select));
    assert_eq!(
        key_context(&mut cx)
            .get("helix_mode")
            .map(|mode| mode.as_ref()),
        Some("select")
    );

    cx.dispatch_action(NormalMode);
    assert_eq!(mode(&mut cx), Some(Mode::Normal));

    cx.dispatch_action(InsertMode);
    assert_eq!(mode(&mut cx), Some(Mode::Insert));
    let context = key_context(&mut cx);
    assert!(!context.contains("Helix"));
    assert_eq!(
        context.get("helix_mode").map(|mode| mode.as_ref()),
        Some("insert")
    );
    cx.assert_editor_state("ˇabc");

    cx.dispatch_action(NormalMode);
    assert_eq!(mode(&mut cx), Some(Mode::Normal));
    cx.assert_editor_state("«ˇa»bc");
}

#[gpui::test]
async fn test_keymap(cx: &mut TestAppContext) {
    let mut cx = helix_editor(cx).await;
    cx.set_state(indoc! {"
        ˇabc
        def"});
    cx.simulate_keystrokes("l j");
    cx.assert_editor_state(indoc! {"
        abc
        d«eˇ»f"});

    cx.simulate_keystrokes("v h k");
    assert_eq!(mode(&mut cx), Some(Mode::Select));
    cx.assert_editor_state(indoc! {"
        «ˇabc
        de»f"});

    cx.simulate_keystrokes("escape");
    assert_eq!(mode(&mut cx), Some(Mode::Normal));
    cx.simulate_keystrokes("a x escape");
    cx.assert_editor_state(indoc! {"
        «abc
        dexˇ»f"});
}

fn lines(count: usize) -> String {
    (0..count)
        .map(|line| format!("line {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn cursor_row(cx: &mut EditorTestContext) -> u32 {
    cx.update_editor(|editor, _, cx| {
        let display_map = editor.display_snapshot(cx);
        editor
            .selections
            .newest::<text::Point>(&display_map)
            .head()
            .row
    })
}

#[gpui::test]
async fn test_count(cx: &mut TestAppContext) {
    let mut cx = helix_editor(cx).await;
    cx.set_state(&format!("ˇ{}", lines(20)));

    cx.simulate_keystrokes("3 j");
    assert_eq!(cursor_row(&mut cx), 3);

    cx.simulate_keystrokes("1 0 j");
    assert_eq!(cursor_row(&mut cx), 13);

    cx.simulate_keystrokes("1 0 k");
    assert_eq!(cursor_row(&mut cx), 3);

    // A zero without a count before it is not a count.
    cx.simulate_keystrokes("0 j");
    assert_eq!(cursor_row(&mut cx), 4);
}

#[gpui::test]
async fn test_count_is_reset_by_other_commands(cx: &mut TestAppContext) {
    let mut cx = helix_editor(cx).await;
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "ctrl-y",
            editor::actions::ToggleLineNumbers,
            Some("Editor"),
        )])
    });
    cx.set_state(&format!("ˇ{}", lines(20)));

    cx.simulate_keystrokes("1 0 ctrl-y j");
    assert_eq!(cursor_row(&mut cx), 1);

    // An unbound key is a command that does nothing, like in Helix.
    cx.simulate_keystrokes("3 x j");
    assert_eq!(cursor_row(&mut cx), 2);

    cx.simulate_keystrokes("3 v j");
    assert_eq!(cursor_row(&mut cx), 3);
}

fn selected_register(cx: &mut EditorTestContext) -> Option<char> {
    cx.editor(|editor, _, cx| {
        editor
            .addon::<HelixAddon>()
            .and_then(|addon| addon.entity.read(cx).selected_register)
    })
}

#[gpui::test]
async fn test_select_register(cx: &mut TestAppContext) {
    let mut cx = helix_editor(cx).await;
    cx.set_state(&format!("ˇ{}", lines(5)));

    cx.simulate_keystrokes("\"");
    let context = key_context(&mut cx);
    assert!(!context.contains("Helix"), "keys must arrive as text");
    assert_eq!(
        context.get("helix_pending").map(|pending| pending.as_ref()),
        Some("register")
    );
    cx.simulate_keystrokes("j");
    assert_eq!(selected_register(&mut cx), Some('j'));
    assert_eq!(cursor_row(&mut cx), 0);
    assert!(key_context(&mut cx).contains("Helix"));

    // The register applies to the next command only.
    cx.simulate_keystrokes("j");
    assert_eq!(selected_register(&mut cx), None);
    assert_eq!(cursor_row(&mut cx), 1);

    // Digits after the register keep it, like in Helix.
    cx.simulate_keystrokes("\" a 2");
    assert_eq!(selected_register(&mut cx), Some('a'));
    cx.simulate_keystrokes("j");
    assert_eq!(cursor_row(&mut cx), 3);
    assert_eq!(selected_register(&mut cx), None);

    // Selecting a register drops the count, like in Helix.
    cx.simulate_keystrokes("2 \" a j");
    assert_eq!(cursor_row(&mut cx), 4);

    cx.simulate_keystrokes("\" escape");
    assert_eq!(selected_register(&mut cx), None);
    assert!(key_context(&mut cx).contains("Helix"));
}

fn set_use_system_clipboard(value: UseSystemClipboard, cx: &mut EditorTestContext) {
    cx.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.vim.get_or_insert_default().use_system_clipboard = Some(value)
            });
        });
    });
}

fn read_register(name: char, cx: &mut EditorTestContext) -> Option<Vec<String>> {
    cx.editor(|editor, _, cx| HelixRegisters::read(name, editor, cx))
}

fn write_register(name: char, values: &[&str], is_yank: bool, cx: &mut EditorTestContext) {
    cx.update(|_, cx| {
        HelixRegisters::write(
            name,
            values.iter().map(|value| value.to_string()).collect(),
            is_yank,
            cx,
        )
    })
    .expect("register should be writable");
}

fn clipboard_text(cx: &mut EditorTestContext) -> Option<String> {
    cx.update(|_, cx| cx.read_from_clipboard().and_then(|item| item.text()))
}

#[gpui::test]
async fn test_registers(cx: &mut TestAppContext) {
    let mut cx = helix_editor(cx).await;
    cx.set_state("«oneˇ» «twoˇ» three");

    write_register('a', &["first", "second"], true, &mut cx);
    assert_eq!(
        read_register('a', &mut cx),
        Some(vec!["first".to_string(), "second".to_string()])
    );
    assert_eq!(read_register('b', &mut cx), None);

    write_register('_', &["discarded"], true, &mut cx);
    assert_eq!(read_register('_', &mut cx), Some(Vec::new()));

    assert_eq!(
        read_register('#', &mut cx),
        Some(vec!["1".to_string(), "2".to_string()])
    );
    assert_eq!(
        read_register('.', &mut cx),
        Some(vec!["one".to_string(), "two".to_string()])
    );
    assert_eq!(read_register('%', &mut cx), Some(vec!["file".to_string()]));
    for name in ['#', '.', '%'] {
        let result = cx.update(|_, cx| HelixRegisters::write(name, vec!["x".into()], true, cx));
        assert!(result.is_err(), "register {name} is read-only");
    }

    write_register('+', &["clip", "board"], true, &mut cx);
    assert_eq!(clipboard_text(&mut cx), Some("clip\nboard".to_string()));
    assert_eq!(
        read_register('+', &mut cx),
        Some(vec!["clip".to_string(), "board".to_string()])
    );
}

#[gpui::test]
async fn test_default_register_follows_use_system_clipboard(cx: &mut TestAppContext) {
    let mut cx = helix_editor(cx).await;
    let set_clipboard = |text: &str, cx: &mut EditorTestContext| {
        cx.update(|_, cx| cx.write_to_clipboard(ClipboardItem::new_string(text.to_string())))
    };

    set_use_system_clipboard(UseSystemClipboard::Never, &mut cx);
    set_clipboard("outside", &mut cx);
    write_register(DEFAULT_REGISTER, &["yanked"], true, &mut cx);
    assert_eq!(clipboard_text(&mut cx), Some("outside".to_string()));
    assert_eq!(
        read_register(DEFAULT_REGISTER, &mut cx),
        Some(vec!["yanked".to_string()])
    );

    set_use_system_clipboard(UseSystemClipboard::Always, &mut cx);
    write_register(DEFAULT_REGISTER, &["deleted"], false, &mut cx);
    assert_eq!(clipboard_text(&mut cx), Some("deleted".to_string()));
    set_clipboard("outside", &mut cx);
    assert_eq!(
        read_register(DEFAULT_REGISTER, &mut cx),
        Some(vec!["outside".to_string()])
    );

    set_use_system_clipboard(UseSystemClipboard::OnYank, &mut cx);
    write_register(DEFAULT_REGISTER, &["deleted"], false, &mut cx);
    assert_eq!(clipboard_text(&mut cx), Some("outside".to_string()));
    assert_eq!(
        read_register(DEFAULT_REGISTER, &mut cx),
        Some(vec!["deleted".to_string()])
    );
    write_register(DEFAULT_REGISTER, &["yanked"], true, &mut cx);
    assert_eq!(clipboard_text(&mut cx), Some("yanked".to_string()));
    set_clipboard("newer", &mut cx);
    assert_eq!(
        read_register(DEFAULT_REGISTER, &mut cx),
        Some(vec!["newer".to_string()])
    );
}
