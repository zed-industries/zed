//! Tests written like Helix's own (`helix-term/tests/test/helpers.rs`): text
//! with selections in Helix notation and keys in Helix key notation.

use std::{
    cmp::Reverse,
    ops::{Deref, DerefMut},
};

use editor::{MultiBufferOffset, SelectionEffects, test::editor_test_context::EditorTestContext};
use feature_flags::{FeatureFlag as _, FeatureFlagAppExt as _};
use gpui::{Entity, TestAppContext, UpdateGlobal as _};
use language::{Selection, SelectionGoal};
use settings::{HELIX_KEYMAP_PATH, KeybindSource, KeymapFile, SettingsStore};
use unicode_segmentation::UnicodeSegmentation as _;
use vim_mode_setting::HelixNextFeatureFlag;

use crate::{Helix, HelixAddon, Mode, selection::HelixRange};

pub(crate) fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        editor::init(cx);
        crate::init(cx);
        let mut default_bindings =
            KeymapFile::load_asset_allow_partial_failure("keymaps/default-macos.json", cx)
                .expect("default keymap should load");
        for binding in &mut default_bindings {
            binding.set_meta(KeybindSource::Default.meta());
        }
        cx.bind_keys(default_bindings);
        let mut bindings =
            KeymapFile::load_asset(HELIX_KEYMAP_PATH, Some(KeybindSource::Helix), cx)
                .expect("helix keymap should load");
        for binding in &mut bindings {
            binding.set_meta(KeybindSource::Helix.meta());
        }
        cx.bind_keys(bindings);
    });
}

pub(crate) fn set_helix_mode(enabled: bool, cx: &mut EditorTestContext) {
    cx.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| settings.helix_mode = Some(enabled));
        });
    });
}

pub(crate) fn set_helix_next_flag(enabled: bool, cx: &mut EditorTestContext) {
    cx.update(|_, cx| {
        let flags = if enabled {
            vec![HelixNextFeatureFlag::NAME.to_string()]
        } else {
            Vec::new()
        };
        cx.update_flags(false, flags);
    });
}

/// How the text of a [`TestCase`] is set up, as in Helix.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum LineFeedHandling {
    /// Appends a line ending unless the text already ends with one, like
    /// documents in Helix always do.
    #[default]
    Native,
    /// Uses the text as given.
    AsIs,
}

impl LineFeedHandling {
    fn apply(self, text: &str) -> String {
        match self {
            LineFeedHandling::AsIs => text.to_string(),
            LineFeedHandling::Native if text.ends_with('\n') => text.to_string(),
            LineFeedHandling::Native => format!("{text}\n"),
        }
    }
}

/// A Helix test case: annotated input, keys, annotated output.
pub(crate) struct TestCase {
    input: String,
    keys: String,
    output: String,
}

impl From<(&str, &str, &str)> for TestCase {
    fn from((input, keys, output): (&str, &str, &str)) -> Self {
        TestCase::from((input, keys, output, LineFeedHandling::default()))
    }
}

impl From<(&str, &str, &str, LineFeedHandling)> for TestCase {
    fn from((input, keys, output, line_feeds): (&str, &str, &str, LineFeedHandling)) -> Self {
        TestCase {
            input: line_feeds.apply(input),
            keys: keys.to_string(),
            output: line_feeds.apply(output),
        }
    }
}

/// Selections in Helix terms, offsets counted in chars like in Helix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HelixSelection {
    pub ranges: Vec<(usize, usize)>,
    pub primary: usize,
}

pub(crate) struct HelixTestContext {
    cx: EditorTestContext,
}

impl HelixTestContext {
    pub async fn new(cx: &mut TestAppContext) -> Self {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        set_helix_mode(true, &mut cx);
        set_helix_next_flag(true, &mut cx);
        Self { cx }
    }

    /// A multibuffer of excerpts marked with `«»`, as in
    /// [`EditorTestContext::new_multibuffer`].
    pub fn new_multibuffer<const COUNT: usize>(
        cx: &mut TestAppContext,
        excerpts: [&str; COUNT],
    ) -> Self {
        init_test(cx);
        let mut cx = EditorTestContext::new_multibuffer(cx, excerpts);
        set_helix_mode(true, &mut cx);
        set_helix_next_flag(true, &mut cx);
        Self { cx }
    }

    pub fn helix(&mut self) -> Entity<Helix> {
        self.cx.editor(|editor, _, _| {
            editor
                .addon::<HelixAddon>()
                .map(|addon| addon.entity.clone())
                .expect("helix should be active")
        })
    }

    pub fn mode(&mut self) -> Mode {
        let helix = self.helix();
        self.cx.read(|cx| helix.read(cx).mode)
    }

    /// Runs a Helix test case from a fresh normal mode.
    #[track_caller]
    pub fn test(&mut self, test_case: impl Into<TestCase>) {
        let test_case = test_case.into();
        self.set_state(&test_case.input);
        self.simulate_keys(&test_case.keys);
        self.assert_state(&test_case.output);
    }

    /// Replaces the text and selections, written in Helix notation, and
    /// returns to normal mode.
    pub fn set_state(&mut self, annotated: &str) {
        let (text, selection) = print(annotated);
        let helix = self.helix();
        self.cx.update(|window, cx| {
            helix.update(cx, |helix, cx| {
                helix.count = None;
                helix.selected_register = None;
                helix.pending_input = None;
                helix.insert_session = None;
                helix.switch_mode(Mode::Normal, window, cx);
            })
        });
        self.cx.update_editor(|editor, window, cx| {
            editor.set_text(text.as_str(), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                let mut ids = (0..selection.ranges.len())
                    .map(|_| selections.new_selection_id())
                    .collect::<Vec<_>>();
                // Zed's primary selection is the newest one.
                let primary_id = ids.pop().unwrap_or_default();
                ids.insert(selection.primary.min(ids.len()), primary_id);
                selections.select(
                    selection
                        .ranges
                        .iter()
                        .zip(ids)
                        .map(|(&(anchor, head), id)| {
                            let anchor = byte_offset(&text, anchor);
                            let head = byte_offset(&text, head);
                            Selection {
                                id,
                                start: MultiBufferOffset(anchor.min(head)),
                                end: MultiBufferOffset(anchor.max(head)),
                                reversed: head < anchor,
                                goal: SelectionGoal::None,
                            }
                        })
                        .collect(),
                );
            });
        });
        self.cx.run_until_parked();
    }

    /// The text and selections in Helix notation.
    pub fn state(&mut self) -> String {
        let helix = self.helix();
        self.cx.update_editor(|editor, _, cx| {
            let text = editor.text(cx);
            let insert_mode_ranges = helix.read(cx).insert_mode_ranges(editor, cx);
            let display_map = editor.display_snapshot(cx);
            let selections = editor.selections.all::<MultiBufferOffset>(&display_map);
            let primary = selections
                .iter()
                .enumerate()
                .max_by_key(|(_, selection)| selection.id)
                .map_or(0, |(index, _)| index);
            let ranges = insert_mode_ranges
                .unwrap_or_else(|| selections.iter().map(HelixRange::from_selection).collect());
            let ranges = ranges
                .iter()
                .map(|range| {
                    (
                        char_offset(&text, range.anchor.0),
                        char_offset(&text, range.head.0),
                    )
                })
                .collect();
            plain(&text, &HelixSelection { ranges, primary })
        })
    }

    #[track_caller]
    pub fn assert_state(&mut self, annotated: &str) {
        let (text, selection) = print(annotated);
        assert_eq!(self.state(), plain(&text, &selection));
    }

    /// Types keys written in Helix notation, like `ihello<esc>` or `<A-;>`.
    pub fn simulate_keys(&mut self, keys: &str) {
        let keystrokes = parse_keys(keys)
            .into_iter()
            .map(|key| helix_key_to_keystroke(&key))
            .collect::<Vec<_>>();
        if !keystrokes.is_empty() {
            self.cx.simulate_keystrokes(&keystrokes.join(" "));
        }
    }

    /// The row and char column of the primary block cursor.
    pub fn cursor_coordinates(&mut self) -> (usize, usize) {
        self.cx.update_editor(|editor, _, cx| {
            let text = editor.text(cx);
            let display_map = editor.display_snapshot(cx);
            let snapshot = display_map.buffer_snapshot();
            let selection = editor.selections.newest::<MultiBufferOffset>(&display_map);
            let cursor = HelixRange::from_selection(&selection).cursor(snapshot).0;
            let before_cursor = &text[..cursor];
            let row = before_cursor.matches('\n').count();
            let line_start = before_cursor.rfind('\n').map_or(0, |index| index + 1);
            (row, text[line_start..cursor].chars().count())
        })
    }
}

impl Deref for HelixTestContext {
    type Target = EditorTestContext;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl DerefMut for HelixTestContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

fn byte_offset(text: &str, char_offset: usize) -> usize {
    text.char_indices()
        .nth(char_offset)
        .map_or(text.len(), |(offset, _)| offset)
}

fn char_offset(text: &str, byte_offset: usize) -> usize {
    text[..byte_offset.min(text.len())].chars().count()
}

/// Parses Helix selection notation, ported from `helix_core::test::print`.
///
/// `#[|` starts the primary selection with the head before the anchor and
/// `]#` ends it; `#[` and `|]#` have the head after the anchor. `#(` and `)#`
/// do the same for the other selections. A line ending right after a
/// selection that ends with the same line ending is dropped, to allow
/// showing the cursor on a line ending.
#[track_caller]
pub(crate) fn print(annotated: &str) -> (String, HelixSelection) {
    let mut primary = None;
    let mut ranges = Vec::new();
    let mut graphemes = annotated.graphemes(true).peekable();
    let mut text = String::with_capacity(annotated.len());
    let last_grapheme =
        |text: &str| -> Option<String> { text.graphemes(true).next_back().map(String::from) };

    'outer: while let Some(grapheme) = graphemes.next() {
        let start = text.chars().count();
        if grapheme != "#" {
            text.push_str(grapheme);
            continue;
        }

        let (is_primary, close_pair) = match graphemes.next() {
            Some("[") => (true, "]"),
            Some("(") => (false, ")"),
            Some(other) => {
                text.push('#');
                text.push_str(other);
                continue;
            }
            None => break,
        };
        if is_primary && primary.is_some() {
            panic!("primary `#[` already appeared in {annotated:?}");
        }

        let head_at_start = graphemes.next_if_eq(&"|").is_some();
        while let Some(grapheme) = graphemes.next() {
            let next = graphemes.peek();
            let mut previous = last_grapheme(&text);
            if !(grapheme == close_pair && next == Some(&"#")) {
                text.push_str(grapheme);
                continue;
            }
            if !head_at_start {
                match previous.as_deref() {
                    Some("|") => {
                        text.pop();
                        previous = last_grapheme(&text);
                    }
                    Some(_) => {
                        text.push_str(grapheme);
                        continue;
                    }
                    None => {}
                }
            }

            graphemes.next();
            let next = graphemes.peek().map(|next| next.to_string());
            if matches!(previous.as_deref(), Some("\n" | "\r\n")) && next == previous {
                graphemes.next();
            }

            if is_primary {
                primary = Some(ranges.len());
            }
            let end = text.chars().count();
            ranges.push(if head_at_start {
                (end, start)
            } else {
                (start, end)
            });
            continue 'outer;
        }

        if head_at_start {
            panic!("missing end `{close_pair}#` in {annotated:?}");
        } else {
            panic!("missing end `|{close_pair}#` in {annotated:?}");
        }
    }

    let Some(primary) = primary else {
        panic!("missing primary `#[|]#` in {annotated:?}");
    };
    (text, HelixSelection { ranges, primary })
}

/// Writes text and selections in Helix notation, ported from
/// `helix_core::test::plain`.
pub(crate) fn plain(text: &str, selection: &HelixSelection) -> String {
    let mut insertions = selection
        .ranges
        .iter()
        .enumerate()
        .flat_map(
            |(index, &(anchor, head))| match (anchor < head, index == selection.primary) {
                (true, true) => [(anchor, "#["), (head, "|]#")],
                (true, false) => [(anchor, "#("), (head, "|)#")],
                (false, true) => [(anchor, "]#"), (head, "#[|")],
                (false, false) => [(anchor, ")#"), (head, "#(|")],
            },
        )
        .map(|(char_offset, marker)| (byte_offset(text, char_offset), marker))
        .collect::<Vec<_>>();
    insertions.sort_by_key(|(offset, _)| Reverse(*offset));
    let mut annotated = text.to_string();
    for (offset, marker) in insertions {
        annotated.insert_str(offset, marker);
    }
    annotated
}

/// Splits keys written in Helix notation, ported from
/// `helix_view::input::parse_macro`.
#[track_caller]
pub(crate) fn parse_keys(keys: &str) -> Vec<String> {
    let mut parsed = Vec::new();
    let mut rest = keys;
    while let Some(character) = rest.chars().next() {
        match character {
            '>' => panic!("unmatched `>` in {keys:?}"),
            '<' => {
                let Some(end) = rest.find('>') else {
                    panic!("`>` expected in {keys:?}");
                };
                parsed.push(rest[1..end].to_string());
                rest = &rest[end + 1..];
            }
            _ => {
                parsed.push(if character == '-' {
                    "minus".to_string()
                } else {
                    character.to_string()
                });
                rest = &rest[character.len_utf8()..];
            }
        }
    }
    parsed
}

/// Converts a Helix key like `C-w`, `A-;`, `esc` or `W` to a gpui keystroke.
#[track_caller]
fn helix_key_to_keystroke(key: &str) -> String {
    let mut tokens = key.split('-').collect::<Vec<_>>();
    // A trailing `-` is the minus key itself.
    let code = if key.len() > 1 && key.ends_with('-') {
        tokens.truncate(tokens.len().saturating_sub(2));
        "-".to_string()
    } else {
        tokens.pop().unwrap_or_default().to_string()
    };

    let mut keystroke = String::new();
    for modifier in tokens {
        keystroke.push_str(match modifier {
            "C" => "ctrl-",
            "A" => "alt-",
            "S" => "shift-",
            "Meta" | "Cmd" | "Win" => "cmd-",
            _ => panic!("invalid key modifier `{modifier}-` in {key:?}"),
        });
    }

    let code = match code.as_str() {
        "esc" => "escape".to_string(),
        "ret" => "enter".to_string(),
        "del" => "delete".to_string(),
        "ins" => "insert".to_string(),
        "minus" => "-".to_string(),
        "lt" => "<".to_string(),
        "gt" => ">".to_string(),
        " " => "space".to_string(),
        "backspace" | "space" | "tab" | "left" | "right" | "up" | "down" | "home" | "end"
        | "pageup" | "pagedown" => code,
        _ if code.chars().count() == 1 => {
            let character = code.chars().next().unwrap_or_default();
            if character.is_ascii_uppercase() {
                format!("shift-{}", character.to_ascii_lowercase())
            } else {
                code
            }
        }
        _ if code.starts_with('F') && code[1..].parse::<u8>().is_ok() => code.to_lowercase(),
        _ => panic!("invalid key `{code}` in {key:?}"),
    };
    keystroke.push_str(&code);
    keystroke
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let selection = |ranges: &[(usize, usize)], primary| HelixSelection {
            ranges: ranges.to_vec(),
            primary,
        };
        assert_eq!(
            print("#[|h]#ello"),
            ("hello".into(), selection(&[(1, 0)], 0))
        );
        assert_eq!(
            print("#[h|]#ello"),
            ("hello".into(), selection(&[(0, 1)], 0))
        );
        assert_eq!(
            print("#[|hell]#o"),
            ("hello".into(), selection(&[(4, 0)], 0))
        );
        assert_eq!(
            print("#[hell|]#o"),
            ("hello".into(), selection(&[(0, 4)], 0))
        );
        assert_eq!(
            print("#[|hello]#"),
            ("hello".into(), selection(&[(5, 0)], 0))
        );
        assert_eq!(
            print("#[hello|]#"),
            ("hello".into(), selection(&[(0, 5)], 0))
        );
        assert_eq!(
            print("#[|h]#ell#(|o)#"),
            ("hello".into(), selection(&[(1, 0), (5, 4)], 0))
        );
        assert_eq!(
            print("#[h|]#ell#(o|)#"),
            ("hello".into(), selection(&[(0, 1), (4, 5)], 0))
        );
        assert_eq!(
            print("#[|he]#l#(|lo)#"),
            ("hello".into(), selection(&[(2, 0), (5, 3)], 0))
        );
        assert_eq!(
            print("hello#[|\r\n]#hello#(|\r\n)#hello#(|\r\n)#"),
            (
                "hello\r\nhello\r\nhello\r\n".into(),
                selection(&[(7, 5), (14, 12), (21, 19)], 0)
            )
        );
        assert_eq!(print("#[\n|]#\n"), ("\n".into(), selection(&[(0, 1)], 0)));
    }

    #[test]
    fn test_plain() {
        assert_eq!(
            plain(
                "abc",
                &HelixSelection {
                    ranges: vec![(0, 1), (3, 2)],
                    primary: 0,
                }
            ),
            "#[a|]#b#(|c)#"
        );
    }

    #[test]
    fn test_parse_keys() {
        assert_eq!(
            parse_keys("ihello<esc>"),
            ["i", "h", "e", "l", "l", "o", "esc"]
        );
        assert_eq!(parse_keys("<A-;>-<C-w>"), ["A-;", "minus", "C-w"]);
        let keystrokes = parse_keys("<A-;><C-S-r>W<esc><ret><space> <S-minus>")
            .iter()
            .map(|key| helix_key_to_keystroke(key))
            .collect::<Vec<_>>();
        assert_eq!(
            keystrokes,
            [
                "alt-;",
                "ctrl-shift-r",
                "shift-w",
                "escape",
                "enter",
                "space",
                "space",
                "shift--"
            ]
        );
    }
}
