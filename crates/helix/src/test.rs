mod helix_test_context;

use std::{cell::Cell, rc::Rc, time::Duration};

use editor::{
    HighlightKey, SELECTION_HIGHLIGHT_DEBOUNCE_TIMEOUT,
    test::editor_test_context::EditorTestContext,
};
use gpui::{ClipboardItem, KeyBinding, Modifiers, MouseButton, TestAppContext, UpdateGlobal as _};
use indoc::indoc;
use language::CursorShape;
use modal_editor::ModeIndicatorSource as _;
use settings::{SettingsStore, UseSystemClipboard};

use crate::{DEFAULT_REGISTER, HelixAddon, HelixRegisters, Mode};
use helix_test_context::{
    HelixTestContext, LineFeedHandling, init_test, set_helix_mode, set_helix_next_flag,
};

#[gpui::test]
async fn test_addon_follows_setting_and_flag(cx: &mut TestAppContext) {
    init_test(cx);
    let mut cx = EditorTestContext::new(cx).await;
    let has_helix_addon = |cx: &mut EditorTestContext| {
        cx.editor(|editor, _, _| editor.addon::<HelixAddon>().is_some())
    };
    assert!(!has_helix_addon(&mut cx));

    set_helix_mode(true, &mut cx);
    assert!(!has_helix_addon(&mut cx), "the flag is required");

    set_helix_next_flag(true, &mut cx);
    assert!(has_helix_addon(&mut cx));

    set_helix_mode(false, &mut cx);
    assert!(!has_helix_addon(&mut cx));

    set_helix_mode(true, &mut cx);
    assert!(has_helix_addon(&mut cx));

    set_helix_next_flag(false, &mut cx);
    assert!(!has_helix_addon(&mut cx));
}

#[gpui::test]
async fn test_key_contexts(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    let key_context = |cx: &mut HelixTestContext| {
        let context = cx.update_editor(|editor, window, cx| editor.key_context(window, cx));
        (
            context.contains("Helix"),
            context.get("helix_mode").map(|mode| mode.to_string()),
            context.contains("helix_count"),
            context
                .get("helix_pending")
                .map(|pending| pending.to_string()),
        )
    };
    cx.set_state("#[a|]#bc");
    assert_eq!(
        key_context(&mut cx),
        (true, Some("normal".into()), false, None)
    );

    cx.simulate_keys("2");
    assert_eq!(
        key_context(&mut cx),
        (true, Some("normal".into()), true, None)
    );

    cx.simulate_keys("v");
    assert_eq!(
        key_context(&mut cx),
        (true, Some("select".into()), false, None)
    );

    // Keys after `"` must reach the editor as text, not as commands.
    cx.simulate_keys("\"");
    assert_eq!(
        key_context(&mut cx),
        (false, Some("select".into()), false, Some("register".into()))
    );

    cx.simulate_keys("ai");
    assert_eq!(
        key_context(&mut cx),
        (false, Some("insert".into()), false, None)
    );
}

// Ported from `insert_mode_cursor_position` in `helix-term/tests/test/movement.rs`.
#[gpui::test]
async fn test_insert_mode_cursor_position(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.test(("#[|]#", "i", "#[|]#", LineFeedHandling::AsIs));

    cx.test(("#[\n|]#", "i", "#[|\n]#"));
    cx.test(("#[\n|]#", "i<esc>", "#[|\n]#"));
    cx.test(("#[\n|]#", "i<esc>i", "#[|\n]#"));
}

// Ported from `insert_to_normal_mode_cursor_position` in
// `helix-term/tests/test/movement.rs`, except the cases that need `<A-;>`.
#[gpui::test]
async fn test_insert_to_normal_mode_cursor_position(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.test((
        indoc! {"\
            #[f|]#oo
            #(b|)#ar"
        },
        "a",
        indoc! {"\
            #[fo|]#o
            #(ba|)#r"
        },
    ));

    cx.test((
        indoc! {"\
            #[f|]#oo
            #(b|)#ar"
        },
        "a<esc>",
        indoc! {"\
            #[f|]#oo
            #(b|)#ar"
        },
    ));
}

// Ported from `cursor_position_append_eof` in `helix-term/tests/test/movement.rs`.
#[gpui::test]
async fn test_cursor_position_append_eof(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    // Selection is forwards
    cx.test(("#[foo|]#", "abar<esc>", "#[foobar|]#\n"));

    // Selection is backwards
    cx.test(("#[|foo]#", "abar<esc>", "#[foobar|]#\n"));

    // Zed buffers may lack the final line ending, which `a` adds like Helix.
    cx.test((
        "#[foo|]#",
        "abar<esc>",
        "#[foobar|]#\n",
        LineFeedHandling::AsIs,
    ));
}

#[gpui::test]
async fn test_insert_and_append(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.test(("#[abc|]# def", "ixy<esc>", "xy#[|abc]# def"));
    cx.test(("#[abc|]# def", "axy<esc>", "#[abcxy|]# def"));
    cx.test(("#[|abc]# def", "axy<esc>", "#[abcxy|]# def"));
    cx.test(("#[a|]#b #(c|)#d", "ix<esc>", "x#[|a]#b x#(|c)#d"));
    cx.test(("#[a|]#b #(c|)#d", "ax<esc>", "#[ax|]#b #(cx|)#d"));
}

// Like Helix, appending at an empty selection at the end of the buffer adds
// no line ending and selects nothing.
#[gpui::test]
async fn test_append_at_empty_selection_at_eof(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.test((
        "foo#[|]#",
        "abar<esc>",
        "foobar#[|]#",
        LineFeedHandling::AsIs,
    ));
}

#[gpui::test]
async fn test_insert_mode_highlights_selections(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("#[abc|]# def");
    cx.simulate_keys("ix");
    cx.assert_editor_background_highlights(HighlightKey::HelixInsertSelections, "x«abc» def");
    cx.simulate_keys("<esc>");
    cx.assert_editor_background_highlights(HighlightKey::HelixInsertSelections, "xabc def");

    cx.set_state("#[abc|]# def");
    cx.simulate_keys("axy");
    cx.assert_editor_background_highlights(HighlightKey::HelixInsertSelections, "«abcxy» def");

    // After `i` a single character is the cursor, after `a` it is selected.
    cx.set_state("#[a|]#bc");
    cx.simulate_keys("ix");
    cx.assert_editor_background_highlights(HighlightKey::HelixInsertSelections, "xabc");
    cx.set_state("#[a|]#bc");
    cx.simulate_keys("ax");
    cx.assert_editor_background_highlights(HighlightKey::HelixInsertSelections, "«ax»bc");
}

// Like Helix, moving the cursor in insert mode turns the selections into
// cursors, and typing afterwards does not extend them.
#[gpui::test]
async fn test_moving_in_insert_mode_drops_selections(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("#[abc|]# def");
    cx.simulate_keys("ix<left>");
    cx.assert_editor_background_highlights(HighlightKey::HelixInsertSelections, "xabc def");
    cx.simulate_keys("y<esc>");
    cx.assert_state("y#[x|]#abc def");

    cx.set_state("#[abc|]# def");
    cx.simulate_keys("a<right>z<esc>");
    cx.assert_state("abc z#[d|]#ef");
}

#[gpui::test]
async fn test_single_grapheme_selection_is_highlighted_as_cursor(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("#[f|]#oo foo");
    cx.executor()
        .advance_clock(SELECTION_HIGHLIGHT_DEBOUNCE_TIMEOUT);
    cx.run_until_parked();
    cx.assert_editor_background_highlights(HighlightKey::SelectedTextHighlight, "foo foo");

    cx.set_state("#[fo|]#o foo");
    cx.executor()
        .advance_clock(SELECTION_HIGHLIGHT_DEBOUNCE_TIMEOUT);
    cx.run_until_parked();
    cx.assert_editor_background_highlights(HighlightKey::SelectedTextHighlight, "foo «fo»o");

    // In select mode the character is selected text.
    cx.set_state("#[f|]#oo foo");
    cx.simulate_keys("v");
    cx.executor()
        .advance_clock(SELECTION_HIGHLIGHT_DEBOUNCE_TIMEOUT);
    cx.run_until_parked();
    cx.assert_editor_background_highlights(HighlightKey::SelectedTextHighlight, "foo «f»oo");
}

// The key context depends on the count, and the window only rebuilds it when
// the editor is notified, so a quickly typed `0` after a digit was unbound.
#[gpui::test]
async fn test_state_changes_notify_editor(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    let notified = Rc::new(Cell::new(false));
    let _subscription = cx.update_editor({
        let notified = notified.clone();
        move |_, _, cx| cx.observe_self(move |_, _| notified.set(true))
    });
    let helix = cx.helix();
    cx.update(|_, cx| helix.update(cx, |helix, cx| helix.push_digit(1, cx)));
    assert!(notified.get());
}

#[gpui::test]
async fn test_insert_session_is_one_undo_step(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("#[a|]#bc");
    cx.simulate_keys("ione");
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.simulate_keys(" two<esc>");
    cx.assert_state("one two#[|a]#bc");

    cx.dispatch_action(editor::actions::Undo);
    assert_eq!(cx.buffer_text(), "abc");
}

// Ported from `test_vertical_move` in `helix-core/src/movement.rs`.
#[gpui::test]
async fn test_vertical_move(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("abcd#[|\n]#efg\nwrs");
    cx.simulate_keys("j");
    assert_eq!(cx.cursor_coordinates(), (1, 3));
}

const SINGLE_LINE_SAMPLE: &str = "This is a simple alphabetic line";
const MULTILINE_SAMPLE: &str = "\
    Multiline\n\
    text sample\n\
    which\n\
    is merely alphabetic\n\
    and whitespaced\n\
";

// Ported from `horizontal_moves_through_single_line_text` in
// `helix-core/src/movement.rs`, except moves by zero, which can't be typed.
#[gpui::test]
async fn test_horizontal_moves_through_single_line_text(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state(&format!("#[|]#{SINGLE_LINE_SAMPLE}"));
    for (keys, coordinates) in [
        ("l", (0, 1)),
        ("2l", (0, 3)),
        ("999l", (0, 32)),
        ("999l", (0, 32)),
        ("999h", (0, 0)),
    ] {
        cx.simulate_keys(keys);
        assert_eq!(cx.cursor_coordinates(), coordinates, "after {keys}");
    }
}

// Ported from `horizontal_moves_through_multiline_text` in
// `helix-core/src/movement.rs`, except moves by zero, which can't be typed.
#[gpui::test]
async fn test_horizontal_moves_through_multiline_text(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state(&format!("#[|]#{MULTILINE_SAMPLE}"));
    for (keys, coordinates) in [
        ("11l", (1, 1)),
        ("h", (1, 0)),
        ("5h", (0, 5)),
        ("999h", (0, 0)),
        ("3l", (0, 3)),
        ("999l", (5, 0)),
        ("999l", (5, 0)),
    ] {
        cx.simulate_keys(keys);
        assert_eq!(cx.cursor_coordinates(), coordinates, "after {keys}");
    }
}

// Ported from `selection_extending_moves_in_single_line_text` in
// `helix-core/src/movement.rs`.
#[gpui::test]
async fn test_selection_extending_moves_in_single_line_text(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state(&format!("#[|]#{SINGLE_LINE_SAMPLE}"));
    cx.simulate_keys("vl");
    cx.assert_state("#[Th|]#is is a simple alphabetic line");
    cx.simulate_keys("5l");
    cx.assert_state("#[This is|]# a simple alphabetic line");
    cx.simulate_keys("3h");
    cx.assert_state("#[This|]# is a simple alphabetic line");
}

// Ported from `vertical_moves_in_single_column` in
// `helix-core/src/movement.rs`, except moves by zero, which can't be typed.
#[gpui::test]
async fn test_vertical_moves_in_single_column(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state(&format!("#[|]#{MULTILINE_SAMPLE}"));
    for (keys, coordinates) in [
        ("j", (1, 0)),
        ("2j", (3, 0)),
        ("j", (4, 0)),
        ("999k", (0, 0)),
        ("4j", (4, 0)),
        ("5j", (5, 0)),
        ("999j", (5, 0)),
    ] {
        cx.simulate_keys(keys);
        assert_eq!(cx.cursor_coordinates(), coordinates, "after {keys}");
    }
}

// Ported from `vertical_moves_jumping_column` in `helix-core/src/movement.rs`.
#[gpui::test]
async fn test_vertical_moves_jumping_column(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state(&format!("#[|]#{MULTILINE_SAMPLE}"));
    for (keys, coordinates) in [
        // Places cursor at the end of line
        ("8l", (0, 8)),
        // First descent preserves column as the target line is wider
        ("j", (1, 8)),
        // Second descent clamps column as the target line is shorter
        ("j", (2, 5)),
        // Third descent restores the original column
        ("j", (3, 8)),
        // Behaviour is preserved even through long jumps
        ("999k", (0, 8)),
        ("4j", (4, 8)),
        ("999j", (5, 0)),
    ] {
        cx.simulate_keys(keys);
        assert_eq!(cx.cursor_coordinates(), coordinates, "after {keys}");
    }
}

#[gpui::test]
async fn test_select_mode(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.test(("#[a|]#bc\ndef", "vlj", "#[abc\nde|]#f"));
    cx.test(("ab#[c|]#\ndef", "vhh", "#[|abc]#\ndef"));
    cx.test(("a#[b|]#c\ndef", "vjk<esc>", "a#[b|]#c\ndef"));
    cx.test(("a#[b|]#c", "vlvl", "abc#[\n|]#"));

    // At the end of the buffer, select mode selects the last character.
    cx.test(("foo\n#[|]#", "v", "foo#[\n|]#", LineFeedHandling::AsIs));

    // Like Helix, extending doesn't reach the empty line after the last
    // line ending, while moving does.
    cx.test(("#[f|]#oo\n", "vj", "#[f|]#oo\n", LineFeedHandling::AsIs));
    cx.test(("#[f|]#oo\n", "j", "foo\n#[|]#", LineFeedHandling::AsIs));
}

#[gpui::test]
async fn test_mode_switching(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("#[a|]#bc");
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keys("v");
    assert_eq!(cx.mode(), Mode::Select);
    cx.simulate_keys("v");
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keys("v<esc>");
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keys("i");
    assert_eq!(cx.mode(), Mode::Insert);
    cx.simulate_keys("<esc>");
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keys("<esc>");
    assert_eq!(cx.mode(), Mode::Normal);
    cx.simulate_keys("va");
    assert_eq!(cx.mode(), Mode::Insert);
}

#[gpui::test]
async fn test_empty_selections_are_widened(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("#[a|]#bc");
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |selections| {
            selections.select_ranges([editor::MultiBufferOffset(1)..editor::MultiBufferOffset(1)])
        });
    });
    cx.run_until_parked();
    cx.assert_state("a#[b|]#c");
}

fn lines(count: usize) -> String {
    (0..count)
        .map(|line| format!("line {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[gpui::test]
async fn test_count(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state(&format!("#[|]#{}", lines(20)));

    cx.simulate_keys("3j");
    assert_eq!(cx.cursor_coordinates(), (3, 0));

    cx.simulate_keys("10j");
    assert_eq!(cx.cursor_coordinates(), (13, 0));

    cx.simulate_keys("10k");
    assert_eq!(cx.cursor_coordinates(), (3, 0));

    // A zero without a count before it is not a count.
    cx.simulate_keys("0j");
    assert_eq!(cx.cursor_coordinates(), (4, 0));
}

// The vim-based Helix mode kept the count through `editor::` commands, so
// `10` `ctrl-s` `j` moved ten lines.
#[gpui::test]
async fn test_count_is_reset_by_other_commands(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.update(|_, cx| {
        cx.bind_keys([KeyBinding::new(
            "ctrl-y",
            editor::actions::ToggleLineNumbers,
            Some("Editor"),
        )])
    });
    cx.set_state(&format!("#[|]#{}", lines(20)));

    cx.simulate_keys("10<C-y>j");
    assert_eq!(cx.cursor_coordinates(), (1, 0));

    // An unbound key is a command that does nothing, like in Helix.
    cx.simulate_keys("3xj");
    assert_eq!(cx.cursor_coordinates(), (2, 0));

    cx.simulate_keys("3vj");
    assert_eq!(cx.cursor_coordinates(), (3, 0));
}

fn selected_register(cx: &mut HelixTestContext) -> Option<char> {
    let helix = cx.helix();
    cx.read(|cx| helix.read(cx).selected_register)
}

#[gpui::test]
async fn test_select_register(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state(&format!("#[|]#{}", lines(5)));

    cx.simulate_keys("\"j");
    assert_eq!(selected_register(&mut cx), Some('j'));
    assert_eq!(cx.cursor_coordinates(), (0, 0));

    // The register applies to the next command only.
    cx.simulate_keys("j");
    assert_eq!(selected_register(&mut cx), None);
    assert_eq!(cx.cursor_coordinates(), (1, 0));

    // Digits after the register keep it, like in Helix.
    cx.simulate_keys("\"a2");
    assert_eq!(selected_register(&mut cx), Some('a'));
    cx.simulate_keys("j");
    assert_eq!(cx.cursor_coordinates(), (3, 0));
    assert_eq!(selected_register(&mut cx), None);

    // Selecting a register drops the count, like in Helix.
    cx.simulate_keys("2\"aj");
    assert_eq!(cx.cursor_coordinates(), (4, 0));

    cx.simulate_keys("\"<esc>");
    assert_eq!(selected_register(&mut cx), None);
    assert_eq!(cx.mode(), Mode::Normal);

    // Like Helix, a key that types no character cancels the register
    // selection and does nothing else.
    cx.simulate_keys("\"<up>");
    assert_eq!(selected_register(&mut cx), None);
    assert_eq!(cx.cursor_coordinates(), (4, 0));
    cx.simulate_keys("\"<C-a>a");
    assert_eq!(selected_register(&mut cx), None);
    assert_eq!(cx.mode(), Mode::Insert);
    cx.simulate_keys("<esc>v\"<esc>");
    assert_eq!(cx.mode(), Mode::Select);
    cx.simulate_keys("\" ");
    assert_eq!(selected_register(&mut cx), Some(' '));
}

fn set_use_system_clipboard(value: UseSystemClipboard, cx: &mut HelixTestContext) {
    cx.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.vim.get_or_insert_default().use_system_clipboard = Some(value)
            });
        });
    });
}

fn read_register(name: char, cx: &mut HelixTestContext) -> Option<Vec<String>> {
    cx.editor(|editor, _, cx| HelixRegisters::read(name, editor, cx))
}

fn write_register(name: char, values: &[&str], is_yank: bool, cx: &mut HelixTestContext) {
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

fn clipboard_text(cx: &mut HelixTestContext) -> Option<String> {
    cx.update(|_, cx| cx.read_from_clipboard().and_then(|item| item.text()))
}

#[gpui::test]
async fn test_registers(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("#[one|]# #(two|)# three");

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
    let mut cx = HelixTestContext::new(cx).await;
    let set_clipboard = |text: &str, cx: &mut HelixTestContext| {
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

#[gpui::test]
async fn test_mode_status(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    let mode_status = |cx: &mut HelixTestContext| {
        let helix = cx.helix();
        cx.read(|cx| {
            let status = helix.read(cx).mode_status(cx);
            (status.mode, status.pending)
        })
    };
    cx.set_state(&format!("#[|]#{}", lines(5)));
    assert_eq!(mode_status(&mut cx), ("NORMAL".into(), "".into()));

    cx.simulate_keys("\"");
    assert_eq!(mode_status(&mut cx), ("NORMAL".into(), "\"".into()));
    cx.simulate_keys("a12");
    assert_eq!(mode_status(&mut cx), ("NORMAL".into(), "\"a12".into()));
    cx.simulate_keys("j");
    assert_eq!(mode_status(&mut cx), ("NORMAL".into(), "".into()));

    cx.simulate_keys("v");
    assert_eq!(mode_status(&mut cx).0, "SELECT");
    cx.simulate_keys("i");
    assert_eq!(mode_status(&mut cx).0, "INSERT");
}

// Unlike vim mode, the mouse does not switch to select mode, like in Helix.
#[gpui::test]
async fn test_mouse_selection(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    // `pixel_position` measures from the drawn cursor, which is at the head
    // of a backward selection.
    cx.set_state("#[|o]#ne two three");

    let start = cx.pixel_position("one twˇo three");
    let end = cx.pixel_position("one ˇtwo three");
    let click = cx.pixel_position("one two thˇree");
    cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_move(end, MouseButton::Left, Modifiers::none());
    cx.simulate_mouse_up(end, MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    assert_eq!(cx.mode(), Mode::Normal);
    // TODO: Helix selects the characters under the pointer at both ends,
    // `two` here (phase 3).
    cx.assert_state("one #[|tw]#o three");

    cx.simulate_click(click, Modifiers::none());
    cx.run_until_parked();
    cx.assert_state("one two th#[r|]#ee");
}

// Excerpt boundaries are line endings that motions and edits treat like
// any other.
#[gpui::test]
async fn test_multibuffer(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new_multibuffer(cx, ["«one\ntwo»", "«three\nfour»"]);
    cx.assert_state("#[o|]#ne\ntwo\nthree\nfour");

    cx.simulate_keys("jj");
    cx.assert_state("one\ntwo\n#[t|]#hree\nfour");
    cx.simulate_keys("k");
    cx.assert_state("one\n#[t|]#wo\nthree\nfour");
    cx.simulate_keys("lll");
    cx.assert_state("one\ntwo#[\n|]#three\nfour");
    cx.simulate_keys("l");
    cx.assert_state("one\ntwo\n#[t|]#hree\nfour");

    cx.simulate_keys("hhaxy<esc>");
    cx.assert_state("one\ntw#[oxy|]#\nthree\nfour");
    cx.simulate_keys("lizz<esc>");
    cx.assert_state("one\ntwoxyzz#[|\n]#three\nfour");
}

fn set_vim_settings(
    cx: &mut EditorTestContext,
    update: impl FnOnce(&mut settings::VimSettingsContent),
) {
    cx.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| update(settings.vim.get_or_insert_default()));
        });
    });
}

#[gpui::test]
async fn test_cursor_shape(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    let cursor_shape = |cx: &mut HelixTestContext| cx.editor(|editor, _, _| editor.cursor_shape());
    cx.set_state("#[a|]#bc");
    assert_eq!(cursor_shape(&mut cx), CursorShape::Block);

    set_vim_settings(&mut cx, |vim| {
        vim.cursor_shape = Some(settings::CursorShapeSettings {
            normal: Some(settings::CursorShape::Underline),
            visual: Some(settings::CursorShape::Bar),
            insert: Some(settings::VimInsertModeCursorShape::Hollow),
            replace: None,
        });
    });
    cx.run_until_parked();
    assert_eq!(cursor_shape(&mut cx), CursorShape::Underline);
    cx.simulate_keys("v");
    assert_eq!(cursor_shape(&mut cx), CursorShape::Bar);
    cx.simulate_keys("<esc>i");
    assert_eq!(cursor_shape(&mut cx), CursorShape::Hollow);
}

#[gpui::test]
async fn test_default_mode(cx: &mut TestAppContext) {
    init_test(cx);
    let mut cx = EditorTestContext::new(cx).await;
    set_vim_settings(&mut cx, |vim| {
        vim.default_mode = Some(settings::ModeContent::Insert)
    });
    set_helix_mode(true, &mut cx);
    set_helix_next_flag(true, &mut cx);
    cx.set_state("ˇabc");
    cx.simulate_keystrokes("x");
    cx.assert_editor_state("xˇabc");
    cx.simulate_keystrokes("escape");
    cx.assert_editor_state("x«aˇ»bc");
}

// Turning Helix mode off in insert mode leaves no Helix state behind.
#[gpui::test]
async fn test_turning_off_in_insert_mode(cx: &mut TestAppContext) {
    let mut cx = HelixTestContext::new(cx).await;
    cx.set_state("#[abc|]# def");
    cx.simulate_keys("ix");
    set_helix_mode(false, &mut cx);
    cx.run_until_parked();
    assert!(cx.editor(|editor, _, _| editor.addon::<HelixAddon>().is_none()));
    cx.assert_editor_background_highlights(HighlightKey::HelixInsertSelections, "xabc def");
    cx.simulate_keystrokes("y");
    cx.assert_editor_state("xyˇabc def");
}
