use std::ops::Range;

use editor::{char_kind, display_map::DisplaySnapshot, movement, Bias, CharKind, DisplayPoint};
use gpui::{actions, impl_actions, AppContext, WindowContext};
use language::Selection;
use serde::Deserialize;
use workspace::Workspace;

use crate::{motion::right, normal::normal_object, state::Mode, visual::visual_object, Vim};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Word { ignore_punctuation: bool },
    Sentence,
    Quotes,
    BackQuotes,
    DoubleQuotes,
    Parentheses,
    SquareBrackets,
    CurlyBrackets,
    AngleBrackets,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Word {
    #[serde(default)]
    ignore_punctuation: bool,
}

actions!(
    vim,
    [
        Sentence,
        Quotes,
        BackQuotes,
        DoubleQuotes,
        Parentheses,
        SquareBrackets,
        CurlyBrackets,
        AngleBrackets
    ]
);
impl_actions!(vim, [Word]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(
        |_: &mut Workspace, &Word { ignore_punctuation }: &Word, cx: _| {
            object(Object::Word { ignore_punctuation }, cx)
        },
    );
    cx.add_action(|_: &mut Workspace, _: &Sentence, cx: _| object(Object::Sentence, cx));
    cx.add_action(|_: &mut Workspace, _: &Quotes, cx: _| object(Object::Quotes, cx));
    cx.add_action(|_: &mut Workspace, _: &BackQuotes, cx: _| object(Object::BackQuotes, cx));
    cx.add_action(|_: &mut Workspace, _: &DoubleQuotes, cx: _| object(Object::DoubleQuotes, cx));
    cx.add_action(|_: &mut Workspace, _: &Parentheses, cx: _| object(Object::Parentheses, cx));
    cx.add_action(|_: &mut Workspace, _: &SquareBrackets, cx: _| {
        object(Object::SquareBrackets, cx)
    });
    cx.add_action(|_: &mut Workspace, _: &CurlyBrackets, cx: _| object(Object::CurlyBrackets, cx));
    cx.add_action(|_: &mut Workspace, _: &AngleBrackets, cx: _| object(Object::AngleBrackets, cx));
}

fn object(object: Object, cx: &mut WindowContext) {
    match Vim::read(cx).state.mode {
        Mode::Normal => normal_object(object, cx),
        Mode::Visual { .. } => visual_object(object, cx),
        Mode::Insert => {
            // Shouldn't execute a text object in insert mode. Ignoring
        }
    }
}

impl Object {
    pub fn range(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        match self {
            Object::Word { ignore_punctuation } => {
                if around {
                    around_word(map, relative_to, ignore_punctuation)
                } else {
                    in_word(map, relative_to, ignore_punctuation)
                }
            }
            Object::Sentence => sentence(map, relative_to, around),
            Object::Quotes => surrounding_markers(map, relative_to, around, false, '\'', '\''),
            Object::BackQuotes => surrounding_markers(map, relative_to, around, false, '`', '`'),
            Object::DoubleQuotes => surrounding_markers(map, relative_to, around, false, '"', '"'),
            Object::Parentheses => surrounding_markers(map, relative_to, around, true, '(', ')'),
            Object::SquareBrackets => surrounding_markers(map, relative_to, around, true, '[', ']'),
            Object::CurlyBrackets => surrounding_markers(map, relative_to, around, true, '{', '}'),
            Object::AngleBrackets => surrounding_markers(map, relative_to, around, true, '<', '>'),
        }
    }

    pub fn expand_selection(
        self,
        map: &DisplaySnapshot,
        selection: &mut Selection<DisplayPoint>,
        around: bool,
    ) -> bool {
        if let Some(range) = self.range(map, selection.head(), around) {
            selection.start = range.start;
            selection.end = range.end;
            true
        } else {
            false
        }
    }
}

/// Return a range that surrounds the word relative_to is in
/// If relative_to is at the start of a word, return the word.
/// If relative_to is between words, return the space between
fn in_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Option<Range<DisplayPoint>> {
    // Use motion::right so that we consider the character under the cursor when looking for the start
    let language = map.buffer_snapshot.language_at(relative_to.to_point(map));
    let start = movement::find_preceding_boundary_in_line(
        map,
        right(map, relative_to, 1),
        |left, right| {
            char_kind(language, left).coerce_punctuation(ignore_punctuation)
                != char_kind(language, right).coerce_punctuation(ignore_punctuation)
        },
    );
    let end = movement::find_boundary_in_line(map, relative_to, |left, right| {
        char_kind(language, left).coerce_punctuation(ignore_punctuation)
            != char_kind(language, right).coerce_punctuation(ignore_punctuation)
    });

    Some(start..end)
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
) -> Option<Range<DisplayPoint>> {
    let language = map.buffer_snapshot.language_at(relative_to.to_point(map));
    let in_word = map
        .chars_at(relative_to)
        .next()
        .map(|(c, _)| char_kind(language, c) != CharKind::Whitespace)
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
) -> Option<Range<DisplayPoint>> {
    in_word(map, relative_to, ignore_punctuation)
        .map(|range| expand_to_include_whitespace(map, range, true))
}

fn around_next_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Option<Range<DisplayPoint>> {
    let language = map.buffer_snapshot.language_at(relative_to.to_point(map));
    // Get the start of the word
    let start = movement::find_preceding_boundary_in_line(
        map,
        right(map, relative_to, 1),
        |left, right| {
            char_kind(language, left).coerce_punctuation(ignore_punctuation)
                != char_kind(language, right).coerce_punctuation(ignore_punctuation)
        },
    );

    let mut word_found = false;
    let end = movement::find_boundary(map, relative_to, |left, right| {
        let left_kind = char_kind(language, left).coerce_punctuation(ignore_punctuation);
        let right_kind = char_kind(language, right).coerce_punctuation(ignore_punctuation);

        let found = (word_found && left_kind != right_kind) || right == '\n' && left == '\n';

        if right_kind != CharKind::Whitespace {
            word_found = true;
        }

        found
    });

    Some(start..end)
}

fn sentence(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    around: bool,
) -> Option<Range<DisplayPoint>> {
    let mut start = None;
    let mut previous_end = relative_to;

    let mut chars = map.chars_at(relative_to).peekable();

    // Search backwards for the previous sentence end or current sentence start. Include the character under relative_to
    for (char, point) in chars
        .peek()
        .cloned()
        .into_iter()
        .chain(map.reverse_chars_at(relative_to))
    {
        if is_sentence_end(map, point) {
            break;
        }

        if is_possible_sentence_start(char) {
            start = Some(point);
        }

        previous_end = point;
    }

    // Search forward for the end of the current sentence or if we are between sentences, the start of the next one
    let mut end = relative_to;
    for (char, point) in chars {
        if start.is_none() && is_possible_sentence_start(char) {
            if around {
                start = Some(point);
                continue;
            } else {
                end = point;
                break;
            }
        }

        end = point;
        *end.column_mut() += char.len_utf8() as u32;
        end = map.clip_point(end, Bias::Left);

        if is_sentence_end(map, end) {
            break;
        }
    }

    let mut range = start.unwrap_or(previous_end)..end;
    if around {
        range = expand_to_include_whitespace(map, range, false);
    }

    Some(range)
}

fn is_possible_sentence_start(character: char) -> bool {
    !character.is_whitespace() && character != '.'
}

const SENTENCE_END_PUNCTUATION: &[char] = &['.', '!', '?'];
const SENTENCE_END_FILLERS: &[char] = &[')', ']', '"', '\''];
const SENTENCE_END_WHITESPACE: &[char] = &[' ', '\t', '\n'];
fn is_sentence_end(map: &DisplaySnapshot, point: DisplayPoint) -> bool {
    let mut next_chars = map.chars_at(point).peekable();
    if let Some((char, _)) = next_chars.next() {
        // We are at a double newline. This position is a sentence end.
        if char == '\n' && next_chars.peek().map(|(c, _)| c == &'\n').unwrap_or(false) {
            return true;
        }

        // The next text is not a valid whitespace. This is not a sentence end
        if !SENTENCE_END_WHITESPACE.contains(&char) {
            return false;
        }
    }

    for (char, _) in map.reverse_chars_at(point) {
        if SENTENCE_END_PUNCTUATION.contains(&char) {
            return true;
        }

        if !SENTENCE_END_FILLERS.contains(&char) {
            return false;
        }
    }

    return false;
}

/// Expands the passed range to include whitespace on one side or the other in a line. Attempts to add the
/// whitespace to the end first and falls back to the start if there was none.
fn expand_to_include_whitespace(
    map: &DisplaySnapshot,
    mut range: Range<DisplayPoint>,
    stop_at_newline: bool,
) -> Range<DisplayPoint> {
    let mut whitespace_included = false;

    let mut chars = map.chars_at(range.end).peekable();
    while let Some((char, point)) = chars.next() {
        if char == '\n' && stop_at_newline {
            break;
        }

        if char.is_whitespace() {
            // Set end to the next display_point or the character position after the current display_point
            range.end = chars.peek().map(|(_, point)| *point).unwrap_or_else(|| {
                let mut end = point;
                *end.column_mut() += char.len_utf8() as u32;
                map.clip_point(end, Bias::Left)
            });

            if char != '\n' {
                whitespace_included = true;
            }
        } else {
            // Found non whitespace. Quit out.
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

fn surrounding_markers(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    around: bool,
    search_across_lines: bool,
    start_marker: char,
    end_marker: char,
) -> Option<Range<DisplayPoint>> {
    let mut matched_ends = 0;
    let mut start = None;
    for (char, mut point) in map.reverse_chars_at(relative_to) {
        if char == start_marker {
            if matched_ends > 0 {
                matched_ends -= 1;
            } else {
                if around {
                    start = Some(point)
                } else {
                    *point.column_mut() += char.len_utf8() as u32;
                    start = Some(point)
                }
                break;
            }
        } else if char == end_marker {
            matched_ends += 1;
        } else if char == '\n' && !search_across_lines {
            break;
        }
    }

    let mut matched_starts = 0;
    let mut end = None;
    for (char, mut point) in map.chars_at(relative_to) {
        if char == end_marker {
            if start.is_none() {
                break;
            }

            if matched_starts > 0 {
                matched_starts -= 1;
            } else {
                if around {
                    *point.column_mut() += char.len_utf8() as u32;
                    end = Some(point);
                } else {
                    end = Some(point);
                }

                break;
            }
        }

        if char == start_marker {
            if start.is_none() {
                if around {
                    start = Some(point);
                } else {
                    *point.column_mut() += char.len_utf8() as u32;
                    start = Some(point);
                }
            } else {
                matched_starts += 1;
            }
        }

        if char == '\n' && !search_across_lines {
            break;
        }
    }

    let (Some(mut start), Some(mut end)) = (start, end) else {
        return None;
    };

    if !around {
        // if a block starts with a newline, move the start to after the newline.
        let mut was_newline = false;
        for (char, point) in map.chars_at(start) {
            if was_newline {
                start = point;
            } else if char == '\n' {
                was_newline = true;
                continue;
            }
            break;
        }
        // if a block ends with a newline, then whitespace, then the delimeter,
        // move the end to after the newline.
        let mut new_end = end;
        for (char, point) in map.reverse_chars_at(end) {
            if char == '\n' {
                end = new_end;
                break;
            }
            if !char.is_whitespace() {
                break;
            }
            new_end = point
        }
    }

    Some(start..end)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::test::{ExemptionFeatures, NeovimBackedTestContext};

    const WORD_LOCATIONS: &'static str = indoc! {"
        The quick ˇbrowˇnˇ•••
        fox ˇjuˇmpsˇ over
        the lazy dogˇ••
        ˇ
        ˇ
        ˇ
        Thˇeˇ-ˇquˇickˇ ˇbrownˇ•
        ˇ••
        ˇ••
        ˇ  fox-jumpˇs over
        the lazy dogˇ•
        ˇ
        "
    };

    #[gpui::test]
    async fn test_change_word_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.assert_binding_matches_all(["c", "i", "w"], WORD_LOCATIONS)
            .await;
        cx.assert_binding_matches_all(["c", "i", "shift-w"], WORD_LOCATIONS)
            .await;
        cx.assert_binding_matches_all(["c", "a", "w"], WORD_LOCATIONS)
            .await;
        cx.assert_binding_matches_all(["c", "a", "shift-w"], WORD_LOCATIONS)
            .await;
    }

    #[gpui::test]
    async fn test_delete_word_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.assert_binding_matches_all(["d", "i", "w"], WORD_LOCATIONS)
            .await;
        cx.assert_binding_matches_all(["d", "i", "shift-w"], WORD_LOCATIONS)
            .await;
        cx.assert_binding_matches_all(["d", "a", "w"], WORD_LOCATIONS)
            .await;
        cx.assert_binding_matches_all(["d", "a", "shift-w"], WORD_LOCATIONS)
            .await;
    }

    #[gpui::test]
    async fn test_visual_word_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("The quick ˇbrown\nfox").await;
        cx.simulate_shared_keystrokes(["v"]).await;
        cx.assert_shared_state("The quick «bˇ»rown\nfox").await;
        cx.simulate_shared_keystrokes(["i", "w"]).await;
        cx.assert_shared_state("The quick «brownˇ»\nfox").await;

        cx.assert_binding_matches_all(["v", "i", "w"], WORD_LOCATIONS)
            .await;
        cx.assert_binding_matches_all_exempted(
            ["v", "h", "i", "w"],
            WORD_LOCATIONS,
            ExemptionFeatures::NonEmptyVisualTextObjects,
        )
        .await;
        cx.assert_binding_matches_all_exempted(
            ["v", "l", "i", "w"],
            WORD_LOCATIONS,
            ExemptionFeatures::NonEmptyVisualTextObjects,
        )
        .await;
        cx.assert_binding_matches_all(["v", "i", "shift-w"], WORD_LOCATIONS)
            .await;

        cx.assert_binding_matches_all_exempted(
            ["v", "i", "h", "shift-w"],
            WORD_LOCATIONS,
            ExemptionFeatures::NonEmptyVisualTextObjects,
        )
        .await;
        cx.assert_binding_matches_all_exempted(
            ["v", "i", "l", "shift-w"],
            WORD_LOCATIONS,
            ExemptionFeatures::NonEmptyVisualTextObjects,
        )
        .await;

        cx.assert_binding_matches_all_exempted(
            ["v", "a", "w"],
            WORD_LOCATIONS,
            ExemptionFeatures::AroundObjectLeavesWhitespaceAtEndOfLine,
        )
        .await;
        cx.assert_binding_matches_all_exempted(
            ["v", "a", "shift-w"],
            WORD_LOCATIONS,
            ExemptionFeatures::AroundObjectLeavesWhitespaceAtEndOfLine,
        )
        .await;
    }

    const SENTENCE_EXAMPLES: &[&'static str] = &[
        "ˇThe quick ˇbrownˇ?ˇ ˇFox Jˇumpsˇ!ˇ Ovˇer theˇ lazyˇ.",
        indoc! {"
            ˇThe quick ˇbrownˇ
            fox jumps over
            the lazy doˇgˇ.ˇ ˇThe quick ˇ
            brown fox jumps over
        "},
        indoc! {"
            The quick brown fox jumps.
            Over the lazy dog
            ˇ
            ˇ
            ˇ  fox-jumpˇs over
            the lazy dog.ˇ
            ˇ
        "},
        r#"ˇThe ˇquick brownˇ.)ˇ]ˇ'ˇ" Brown ˇfox jumpsˇ.ˇ "#,
    ];

    #[gpui::test]
    async fn test_change_sentence_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["c", "i", "s"]);
        cx.add_initial_state_exemptions(
            "The quick brown fox jumps.\nOver the lazy dog\nˇ\nˇ\n  fox-jumps over\nthe lazy dog.\n\n",
            ExemptionFeatures::SentenceOnEmptyLines);
        cx.add_initial_state_exemptions(
            "The quick brown fox jumps.\nOver the lazy dog\n\n\nˇ  foxˇ-ˇjumpˇs over\nthe lazy dog.\n\n",
            ExemptionFeatures::SentenceAtStartOfLineWithWhitespace);
        cx.add_initial_state_exemptions(
            "The quick brown fox jumps.\nOver the lazy dog\n\n\n  fox-jumps over\nthe lazy dog.ˇ\nˇ\n",
            ExemptionFeatures::SentenceAfterPunctuationAtEndOfFile);
        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all(sentence_example).await;
        }

        let mut cx = cx.binding(["c", "a", "s"]);
        cx.add_initial_state_exemptions(
            "The quick brown?ˇ Fox Jumps! Over the lazy.",
            ExemptionFeatures::IncorrectLandingPosition,
        );
        cx.add_initial_state_exemptions(
            "The quick brown.)]\'\" Brown fox jumps.ˇ ",
            ExemptionFeatures::AroundObjectLeavesWhitespaceAtEndOfLine,
        );

        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all(sentence_example).await;
        }
    }

    #[gpui::test]
    async fn test_delete_sentence_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["d", "i", "s"]);
        cx.add_initial_state_exemptions(
            "The quick brown fox jumps.\nOver the lazy dog\nˇ\nˇ\n  fox-jumps over\nthe lazy dog.\n\n",
            ExemptionFeatures::SentenceOnEmptyLines);
        cx.add_initial_state_exemptions(
            "The quick brown fox jumps.\nOver the lazy dog\n\n\nˇ  foxˇ-ˇjumpˇs over\nthe lazy dog.\n\n",
            ExemptionFeatures::SentenceAtStartOfLineWithWhitespace);
        cx.add_initial_state_exemptions(
            "The quick brown fox jumps.\nOver the lazy dog\n\n\n  fox-jumps over\nthe lazy dog.ˇ\nˇ\n",
            ExemptionFeatures::SentenceAfterPunctuationAtEndOfFile);

        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all(sentence_example).await;
        }

        let mut cx = cx.binding(["d", "a", "s"]);
        cx.add_initial_state_exemptions(
            "The quick brown?ˇ Fox Jumps! Over the lazy.",
            ExemptionFeatures::IncorrectLandingPosition,
        );
        cx.add_initial_state_exemptions(
            "The quick brown.)]\'\" Brown fox jumps.ˇ ",
            ExemptionFeatures::AroundObjectLeavesWhitespaceAtEndOfLine,
        );

        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all(sentence_example).await;
        }
    }

    #[gpui::test]
    async fn test_visual_sentence_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx)
            .await
            .binding(["v", "i", "s"]);
        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all_exempted(sentence_example, ExemptionFeatures::SentenceOnEmptyLines)
                .await;
        }

        let mut cx = cx.binding(["v", "a", "s"]);
        for sentence_example in SENTENCE_EXAMPLES {
            cx.assert_all_exempted(
                sentence_example,
                ExemptionFeatures::AroundSentenceStartingBetweenIncludesWrongWhitespace,
            )
            .await;
        }
    }

    // Test string with "`" for opening surrounders and "'" for closing surrounders
    const SURROUNDING_MARKER_STRING: &str = indoc! {"
        ˇTh'ˇe ˇ`ˇ'ˇquˇi`ˇck broˇ'wn`
        'ˇfox juˇmps ovˇ`ˇer
        the ˇlazy dˇ'ˇoˇ`ˇg"};

    const SURROUNDING_OBJECTS: &[(char, char)] = &[
        ('\'', '\''), // Quote
        ('`', '`'),   // Back Quote
        ('"', '"'),   // Double Quote
        ('(', ')'),   // Parentheses
        ('[', ']'),   // SquareBrackets
        ('{', '}'),   // CurlyBrackets
        ('<', '>'),   // AngleBrackets
    ];

    #[gpui::test]
    async fn test_change_surrounding_character_objects(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for (start, end) in SURROUNDING_OBJECTS {
            if ((start == &'\'' || start == &'`' || start == &'"')
                && !ExemptionFeatures::QuotesSeekForward.supported())
                || (start == &'<' && !ExemptionFeatures::AngleBracketsFreezeNeovim.supported())
            {
                continue;
            }

            let marked_string = SURROUNDING_MARKER_STRING
                .replace('`', &start.to_string())
                .replace('\'', &end.to_string());

            cx.assert_binding_matches_all(["c", "i", &start.to_string()], &marked_string)
                .await;
            cx.assert_binding_matches_all(["c", "i", &end.to_string()], &marked_string)
                .await;
            cx.assert_binding_matches_all(["c", "a", &start.to_string()], &marked_string)
                .await;
            cx.assert_binding_matches_all(["c", "a", &end.to_string()], &marked_string)
                .await;
        }
    }

    #[gpui::test]
    async fn test_multiline_surrounding_character_objects(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "func empty(a string) bool {
               if a == \"\" {
                  return true
               }
               ˇreturn false
            }"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "i", "{"]).await;
        cx.assert_shared_state(indoc! {"
            func empty(a string) bool {
            «   if a == \"\" {
                  return true
               }
               return false
            ˇ»}"})
            .await;
        cx.set_shared_state(indoc! {
            "func empty(a string) bool {
                 if a == \"\" {
                     ˇreturn true
                 }
                 return false
            }"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "i", "{"]).await;
        cx.assert_shared_state(indoc! {"
            func empty(a string) bool {
                 if a == \"\" {
            «         return true
            ˇ»     }
                 return false
            }"})
            .await;
    }

    #[gpui::test]
    async fn test_delete_surrounding_character_objects(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for (start, end) in SURROUNDING_OBJECTS {
            if ((start == &'\'' || start == &'`' || start == &'"')
                && !ExemptionFeatures::QuotesSeekForward.supported())
                || (start == &'<' && !ExemptionFeatures::AngleBracketsFreezeNeovim.supported())
            {
                continue;
            }
            let marked_string = SURROUNDING_MARKER_STRING
                .replace('`', &start.to_string())
                .replace('\'', &end.to_string());

            cx.assert_binding_matches_all(["d", "i", &start.to_string()], &marked_string)
                .await;
            cx.assert_binding_matches_all(["d", "i", &end.to_string()], &marked_string)
                .await;
            cx.assert_binding_matches_all(["d", "a", &start.to_string()], &marked_string)
                .await;
            cx.assert_binding_matches_all(["d", "a", &end.to_string()], &marked_string)
                .await;
        }
    }
}
