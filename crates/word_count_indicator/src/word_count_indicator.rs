use editor::Editor;
use gpui::{App, Context, Entity, Subscription, Window};
use ui::{Tooltip, prelude::*};
use workspace::{
    HideStatusItem, StatusBarSettings, StatusItemView,
    item::{ItemHandle, Settings},
};

/// Counts words with CJK-aware semantics: each CJK character (Han, kana,
/// hangul) carries meaning on its own and counts as one word; other words are
/// maximal runs of alphanumeric characters, matching Zed's
/// `CharClassifier::is_word` (alphanumeric plus underscore).
///
/// Without the CJK branch, a mixed-script string like "世界word" would be a
/// single run, since CJK ideographs are alphanumeric but not space-separated.
pub fn count_words(text: impl Iterator<Item = char>) -> usize {
    let mut count = 0;
    let mut in_word = false;
    for c in text {
        if is_cjk(c) {
            count += 1;
            in_word = false;
        } else if c.is_alphanumeric() || c == '_' {
            if !in_word {
                count += 1;
                in_word = true;
            }
        } else {
            in_word = false;
        }
    }
    count
}

/// CJK scripts whose characters are not space-separated, so whitespace-based
/// word segmentation does not apply.
fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x3400..=0x4DBF     // CJK Unified Ideographs Extension A
        | 0x4E00..=0x9FFF   // CJK Unified Ideographs
        | 0xF900..=0xFAFF   // CJK Compatibility Ideographs
        | 0x20000..=0x2A6DF // CJK Unified Ideographs Extension B
        | 0x3040..=0x30FF   // Hiragana + Katakana
        | 0x31F0..=0x31FF   // Katakana Phonetic Extensions
        | 0xFF66..=0xFF9D   // Halfwidth Katakana
        | 0xAC00..=0xD7A3   // Hangul Syllables
        | 0x1100..=0x11FF   // Hangul Jamo
        | 0x3130..=0x318F   // Hangul Compatibility Jamo
    )
}

#[derive(Default)]
pub struct WordCountIndicator {
    word_count: Option<usize>,
    _observe_active_editor: Option<Subscription>,
}

impl WordCountIndicator {
    fn update(&mut self, editor: Entity<Editor>, _: &mut Window, cx: &mut Context<Self>) {
        self.word_count = editor
            .read(cx)
            .active_buffer(cx)
            .map(|buffer| count_words(buffer.read(cx).text_snapshot().chars()));
        cx.notify();
    }
}

impl Render for WordCountIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).word_count_button {
            return div();
        }

        div().when_some(self.word_count, |el, count| {
            el.child(
                Button::new("word-count", format!("{} words", count))
                    .label_size(LabelSize::Small)
                    .tab_index(0isize)
                    .tooltip(Tooltip::text("Word Count")),
            )
        })
    }
}

impl StatusItemView for WordCountIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe_in(&editor, window, Self::update));
            self.update(editor, window, cx);
        } else {
            self.word_count = None;
            self._observe_active_editor = None;
            cx.notify();
        }
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .status_bar
                .get_or_insert_default()
                .word_count_button = Some(false);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::count_words;

    fn count(text: &str) -> usize {
        count_words(text.chars())
    }

    #[test]
    fn latin_words() {
        assert_eq!(count("hello world"), 2);
    }

    #[test]
    fn cjk_characters_count_individually() {
        assert_eq!(count("你好世界"), 4);
    }

    #[test]
    fn mixed_cjk_and_latin() {
        // 4 Han + "hello" + "world"
        assert_eq!(count("你好世界 hello world"), 6);
    }

    #[test]
    fn cjk_does_not_merge_with_adjacent_latin() {
        assert_eq!(count("世界word"), 3);
        assert_eq!(count("一行中英混排with数字42和标点！"), 13);
    }

    #[test]
    fn japanese_and_korean() {
        assert_eq!(count("カタカナ テスト"), 7);
        assert_eq!(count("한국어 텍스트"), 6);
    }

    #[test]
    fn numbers_and_underscores_form_words() {
        assert_eq!(count("foo_bar 42"), 2);
    }

    #[test]
    fn empty_and_whitespace() {
        assert_eq!(count(""), 0);
        assert_eq!(count("  \n\t "), 0);
    }

    #[test]
    fn emoji_are_not_words() {
        assert_eq!(count("🦀 emoji only"), 2);
    }
}
