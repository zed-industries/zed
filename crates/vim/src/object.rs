use std::ops::Range;

use editor::{char_kind, display_map::DisplaySnapshot, movement, Bias, CharKind, DisplayPoint};
use gpui::{actions, impl_actions, MutableAppContext};
use language::Selection;
use serde::Deserialize;
use workspace::Workspace;

use crate::{motion, normal::normal_object, state::Mode, visual::visual_object, Vim};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Word { ignore_punctuation: bool },
    Sentence,
    Paragraph,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Word {
    #[serde(default)]
    ignore_punctuation: bool,
}

actions!(vim, [Sentence, Paragraph]);
impl_actions!(vim, [Word]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(
        |_: &mut Workspace, &Word { ignore_punctuation }: &Word, cx: _| {
            object(Object::Word { ignore_punctuation }, cx)
        },
    );
    cx.add_action(|_: &mut Workspace, _: &Sentence, cx: _| object(Object::Sentence, cx));
    cx.add_action(|_: &mut Workspace, _: &Paragraph, cx: _| object(Object::Paragraph, cx));
}

fn object(object: Object, cx: &mut MutableAppContext) {
    match Vim::read(cx).state.mode {
        Mode::Normal => normal_object(object, cx),
        Mode::Visual { .. } => visual_object(object, cx),
        Mode::Insert => {
            // Shouldn't execute a text object in insert mode. Ignoring
        }
    }
}

impl Object {
    pub fn object_range(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
        around: bool,
    ) -> Range<DisplayPoint> {
        match self {
            Object::Word { ignore_punctuation } => {
                if around {
                    around_word(map, relative_to, ignore_punctuation)
                } else {
                    in_word(map, relative_to, ignore_punctuation)
                }
            }
            Object::Sentence => sentence(map, relative_to, around),
            _ => relative_to..relative_to,
        }
    }

    pub fn expand_selection(
        self,
        map: &DisplaySnapshot,
        selection: &mut Selection<DisplayPoint>,
        around: bool,
    ) {
        let range = self.object_range(map, selection.head(), around);
        selection.start = range.start;
        selection.end = range.end;
    }
}

/// Return a range that surrounds the word relative_to is in
/// If relative_to is at the start of a word, return the word.
/// If relative_to is between words, return the space between
fn in_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Range<DisplayPoint> {
    // Use motion::right so that we consider the character under the cursor when looking for the start
    let start = movement::find_preceding_boundary_in_line(
        map,
        motion::right(map, relative_to),
        |left, right| {
            char_kind(left).coerce_punctuation(ignore_punctuation)
                != char_kind(right).coerce_punctuation(ignore_punctuation)
        },
    );
    let end = movement::find_boundary_in_line(map, relative_to, |left, right| {
        char_kind(left).coerce_punctuation(ignore_punctuation)
            != char_kind(right).coerce_punctuation(ignore_punctuation)
    });

    start..end
}

/// Return a range that surrounds the word and following whitespace
/// relative_to is in.
/// If relative_to is at the start of a word, return the word and following whitespace.
/// If relative_to is between words, return the whitespace back and the following word

/// if in word
///   delete that word
///   if there is whitespace following the word, delete that as well
///   otherwise, delete any preceding whitespace
/// otherwise
///   delete whitespace around cursor
///   delete word following the cursor
fn around_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Range<DisplayPoint> {
    let in_word = map
        .chars_at(relative_to)
        .next()
        .map(|(c, _)| char_kind(c) != CharKind::Whitespace)
        .unwrap_or(false);

    if in_word {
        around_containing_word(map, relative_to, ignore_punctuation)
    } else {
        around_next_word(map, relative_to, ignore_punctuation)
    }
}

fn around_containing_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Range<DisplayPoint> {
    expand_to_include_whitespace(map, in_word(map, relative_to, ignore_punctuation), true)
}

fn around_next_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Range<DisplayPoint> {
    // Get the start of the word
    let start = movement::find_preceding_boundary_in_line(
        map,
        motion::right(map, relative_to),
        |left, right| {
            char_kind(left).coerce_punctuation(ignore_punctuation)
                != char_kind(right).coerce_punctuation(ignore_punctuation)
        },
    );

    let mut word_found = false;
    let end = movement::find_boundary(map, relative_to, |left, right| {
        let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
        let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);

        let found = (word_found && left_kind != right_kind) || right == '\n' && left == '\n';

        if right_kind != CharKind::Whitespace {
            word_found = true;
        }

        found
    });

    start..end
}

// /// Return the range containing a sentence.
// fn sentence(map: &DisplaySnapshot, relative_to: DisplayPoint, around: bool) -> Range<DisplayPoint> {
//     let mut previous_end = relative_to;
//     let mut start = None;

//     // Seek backwards to find a period or double newline. Record the last non whitespace character as the
//     // possible start of the sentence. Alternatively if two newlines are found right after each other, return that.
//     let mut rev_chars = map.reverse_chars_at(relative_to).peekable();
//     while let Some((char, point)) = rev_chars.next() {
//         dbg!(char, point);
//         if char == '.' {
//             break;
//         }

//         if char == '\n'
//             && (rev_chars.peek().map(|(c, _)| c == &'\n').unwrap_or(false) || start.is_none())
//         {
//             break;
//         }

//         if !char.is_whitespace() {
//             start = Some(point);
//         }

//         previous_end = point;
//     }

//     let mut end = relative_to;
//     let mut chars = map.chars_at(relative_to).peekable();
//     while let Some((char, point)) = chars.next() {
//         if !char.is_whitespace() {
//             if start.is_none() {
//                 start = Some(point);
//             }

//             // Set the end to the point after the current non whitespace character
//             end = point;
//             *end.column_mut() += char.len_utf8() as u32;
//         }

//         if char == '.' {
//             break;
//         }

//         if char == '\n' {
//             if start.is_none() {
//                 if let Some((_, next_point)) = chars.peek() {
//                     end = *next_point;
//                 }
//                 break;

//             if chars.peek().map(|(c, _)| c == &'\n').unwrap_or(false) {
//                 break;
//             }
//         }
//     }

//     start.unwrap_or(previous_end)..end
// }

fn sentence(map: &DisplaySnapshot, relative_to: DisplayPoint, around: bool) -> Range<DisplayPoint> {
    let mut start = None;
    let mut previous_end = relative_to;

    for (char, point) in map.reverse_chars_at(relative_to) {
        if is_sentence_end(map, point) {
            break;
        }

        if is_possible_sentence_start(char) {
            start = Some(point);
        }

        previous_end = point;
    }

    // Handle case where cursor was before the sentence start
    let mut chars = map.chars_at(relative_to).peekable();
    if start.is_none() {
        if let Some((char, point)) = chars.peek() {
            if is_possible_sentence_start(*char) {
                start = Some(*point);
            }
        }
    }

    let mut end = relative_to;
    for (char, point) in chars {
        if start.is_some() {
            if !char.is_whitespace() {
                end = point;
                *end.column_mut() += char.len_utf8() as u32;
                end = map.clip_point(end, Bias::Left);
            }

            if is_sentence_end(map, point) {
                break;
            }
        } else if is_possible_sentence_start(char) {
            if around {
                start = Some(point);
            } else {
                end = point;
                break;
            }
        }
    }

    let mut range = start.unwrap_or(previous_end)..end;
    if around {
        range = expand_to_include_whitespace(map, range, false);
    }

    range
}

fn is_possible_sentence_start(character: char) -> bool {
    !character.is_whitespace() && character != '.'
}

const SENTENCE_END_PUNCTUATION: &[char] = &['.', '!', '?'];
const SENTENCE_END_FILLERS: &[char] = &[')', ']', '"', '\''];
const SENTENCE_END_WHITESPACE: &[char] = &[' ', '\t', '\n'];
fn is_sentence_end(map: &DisplaySnapshot, point: DisplayPoint) -> bool {
    let mut chars = map.chars_at(point).peekable();

    if let Some((char, _)) = chars.next() {
        if char == '\n' && chars.peek().map(|(c, _)| c == &'\n').unwrap_or(false) {
            return true;
        }

        if !SENTENCE_END_PUNCTUATION.contains(&char) {
            return false;
        }
    } else {
        return false;
    }

    for (char, _) in chars {
        if SENTENCE_END_WHITESPACE.contains(&char) {
            return true;
        }

        if !SENTENCE_END_FILLERS.contains(&char) {
            return false;
        }
    }

    return true;
}

/// Expands the passed range to include whitespace on one side or the other in a line. Attempts to add the
/// whitespace to the end first and falls back to the start if there was none.
fn expand_to_include_whitespace(
    map: &DisplaySnapshot,
    mut range: Range<DisplayPoint>,
    stop_at_newline: bool,
) -> Range<DisplayPoint> {
    let mut whitespace_included = false;
    for (char, point) in map.chars_at(range.end) {
        range.end = point;

        if char == '\n' && stop_at_newline {
            break;
        }

        if char.is_whitespace() {
            whitespace_included = true;
        } else {
            break;
        }
    }

    if !whitespace_included {
        for (char, point) in map.reverse_chars_at(range.start) {
            if char == '\n' && stop_at_newline {
                break;
            }

            if !char.is_whitespace() {
                break;
            }

            range.start = point;
        }
    }

    range
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::test_contexts::NeovimBackedTestContext;

    const WORD_LOCATIONS: &'static str = indoc! {"
        The quick ˇbrowˇnˇ   
        fox ˇjuˇmpsˇ over
        the lazy dogˇ  
        ˇ
        ˇ
        ˇ
        Thˇeˇ-ˇquˇickˇ ˇbrownˇ 
        ˇ  
        ˇ  
        ˇ  fox-jumpˇs over
        the lazy dogˇ 
        ˇ
        "};

    #[gpui::test]
    async fn test_change_in_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new("test_change_in_word", cx)
            .await
            .binding(["c", "i", "w"]);
        cx.assert_all(WORD_LOCATIONS).await;
        let mut cx = cx.consume().binding(["c", "i", "shift-w"]);
        cx.assert_all(WORD_LOCATIONS).await;
    }

    #[gpui::test]
    async fn test_delete_in_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new("test_delete_in_word", cx)
            .await
            .binding(["d", "i", "w"]);
        cx.assert_all(WORD_LOCATIONS).await;
        let mut cx = cx.consume().binding(["d", "i", "shift-w"]);
        cx.assert_all(WORD_LOCATIONS).await;
    }

    #[gpui::test]
    async fn test_change_around_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new("test_change_around_word", cx)
            .await
            .binding(["c", "a", "w"]);
        cx.assert_all(WORD_LOCATIONS).await;
        let mut cx = cx.consume().binding(["c", "a", "shift-w"]);
        cx.assert_all(WORD_LOCATIONS).await;
    }

    #[gpui::test]
    async fn test_delete_around_word(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new("test_delete_around_word", cx)
            .await
            .binding(["d", "a", "w"]);
        cx.assert_all(WORD_LOCATIONS).await;
        let mut cx = cx.consume().binding(["d", "a", "shift-w"]);
        cx.assert_all(WORD_LOCATIONS).await;
    }

    const SENTENCE_EXAMPLES: &[&'static str] = &[
        "ˇThe quick ˇbrownˇ?ˇ ˇFox Jˇumpsˇ!ˇ Ovˇer theˇ lazyˇ.",
        indoc! {"
            ˇThe quick ˇbrownˇ   
            fox jumps over
            the lazy doˇgˇ.ˇ ˇThe quick ˇ
            brown fox jumps over
        "},
        // Double newlines are broken currently
        // indoc! {"
        //     The quick brown fox jumps.
        //     Over the lazy dog
        //     ˇ
        //     ˇ
        //     ˇ  fox-jumpˇs over
        //     the lazy dog.ˇ
        //     ˇ
        // "},
        r#"The quick brown.)]'" Brown fox jumps."#,
    ];

    #[gpui::test]
    async fn test_change_in_sentence(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new("test_change_in_sentence", cx)
            .await
            .binding(["c", "i", "s"]);
        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all(sentence_example).await;
        }
    }

    #[gpui::test]
    async fn test_delete_in_sentence(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new("test_delete_in_sentence", cx)
            .await
            .binding(["d", "i", "s"]);
        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all(sentence_example).await;
        }
    }

    #[gpui::test]
    #[ignore] // End cursor position is incorrect
    async fn test_change_around_sentence(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new("test_change_around_sentence", cx)
            .await
            .binding(["c", "a", "s"]);
        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all(sentence_example).await;
        }
    }

    #[gpui::test]
    #[ignore] // End cursor position is incorrect
    async fn test_delete_around_sentence(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new("test_delete_around_sentence", cx)
            .await
            .binding(["d", "a", "s"]);
        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all(sentence_example).await;
        }
    }
}
