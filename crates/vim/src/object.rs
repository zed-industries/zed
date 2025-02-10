use std::ops::Range;

use crate::{
    motion::right,
    state::{Mode, Operator},
    Vim,
};
use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement::{self, FindRange},
    Bias, DisplayPoint, Editor,
};
use gpui::{actions, impl_actions, Window};
use itertools::Itertools;
use language::{BufferSnapshot, CharKind, Point, Selection, TextObject, TreeSitterOptions};
use multi_buffer::MultiBufferRow;
use schemars::JsonSchema;
use serde::Deserialize;
use ui::Context;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Object {
    Word { ignore_punctuation: bool },
    Subword { ignore_punctuation: bool },
    Sentence,
    Paragraph,
    Quotes,
    BackQuotes,
    AnyQuotes,
    DoubleQuotes,
    VerticalBars,
    AnyBrackets,
    Parentheses,
    SquareBrackets,
    CurlyBrackets,
    AngleBrackets,
    Argument,
    IndentObj { include_below: bool },
    Tag,
    Method,
    Class,
    Comment,
    EntireFile,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
struct Word {
    #[serde(default)]
    ignore_punctuation: bool,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
struct Subword {
    #[serde(default)]
    ignore_punctuation: bool,
}
#[derive(Clone, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
struct IndentObj {
    #[serde(default)]
    include_below: bool,
}

impl_actions!(vim, [Word, Subword, IndentObj]);

actions!(
    vim,
    [
        Sentence,
        Paragraph,
        Quotes,
        BackQuotes,
        AnyQuotes,
        DoubleQuotes,
        VerticalBars,
        Parentheses,
        AnyBrackets,
        SquareBrackets,
        CurlyBrackets,
        AngleBrackets,
        Argument,
        Tag,
        Method,
        Class,
        Comment,
        EntireFile
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(
        editor,
        cx,
        |vim, &Word { ignore_punctuation }: &Word, window, cx| {
            vim.object(Object::Word { ignore_punctuation }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &Subword { ignore_punctuation }: &Subword, window, cx| {
            vim.object(Object::Subword { ignore_punctuation }, window, cx)
        },
    );
    Vim::action(editor, cx, |vim, _: &Tag, window, cx| {
        vim.object(Object::Tag, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Sentence, window, cx| {
        vim.object(Object::Sentence, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Paragraph, window, cx| {
        vim.object(Object::Paragraph, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Quotes, window, cx| {
        vim.object(Object::Quotes, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &AnyQuotes, window, cx| {
        vim.object(Object::AnyQuotes, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &AnyBrackets, window, cx| {
        vim.object(Object::AnyBrackets, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &DoubleQuotes, window, cx| {
        vim.object(Object::DoubleQuotes, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &DoubleQuotes, window, cx| {
        vim.object(Object::DoubleQuotes, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Parentheses, window, cx| {
        vim.object(Object::Parentheses, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &SquareBrackets, window, cx| {
        vim.object(Object::SquareBrackets, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &CurlyBrackets, window, cx| {
        vim.object(Object::CurlyBrackets, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &AngleBrackets, window, cx| {
        vim.object(Object::AngleBrackets, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &VerticalBars, window, cx| {
        vim.object(Object::VerticalBars, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Argument, window, cx| {
        vim.object(Object::Argument, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Method, window, cx| {
        vim.object(Object::Method, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Class, window, cx| {
        vim.object(Object::Class, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &EntireFile, window, cx| {
        vim.object(Object::EntireFile, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Comment, window, cx| {
        if !matches!(vim.active_operator(), Some(Operator::Object { .. })) {
            vim.push_operator(Operator::Object { around: true }, window, cx);
        }
        vim.object(Object::Comment, window, cx)
    });
    Vim::action(
        editor,
        cx,
        |vim, &IndentObj { include_below }: &IndentObj, window, cx| {
            vim.object(Object::IndentObj { include_below }, window, cx)
        },
    );
}

impl Vim {
    fn object(&mut self, object: Object, window: &mut Window, cx: &mut Context<Self>) {
        match self.mode {
            Mode::Normal => self.normal_object(object, window, cx),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                self.visual_object(object, window, cx)
            }
            Mode::Insert | Mode::Replace | Mode::HelixNormal => {
                // Shouldn't execute a text object in insert mode. Ignoring
            }
        }
    }
}

impl Object {
    pub fn is_multiline(self) -> bool {
        match self {
            Object::Word { .. }
            | Object::Subword { .. }
            | Object::Quotes
            | Object::BackQuotes
            | Object::AnyQuotes
            | Object::VerticalBars
            | Object::DoubleQuotes => false,
            Object::Sentence
            | Object::Paragraph
            | Object::AnyBrackets
            | Object::Parentheses
            | Object::Tag
            | Object::AngleBrackets
            | Object::CurlyBrackets
            | Object::SquareBrackets
            | Object::Argument
            | Object::Method
            | Object::Class
            | Object::EntireFile
            | Object::Comment
            | Object::IndentObj { .. } => true,
        }
    }

    pub fn always_expands_both_ways(self) -> bool {
        match self {
            Object::Word { .. }
            | Object::Subword { .. }
            | Object::Sentence
            | Object::Paragraph
            | Object::Argument
            | Object::IndentObj { .. } => false,
            Object::Quotes
            | Object::BackQuotes
            | Object::AnyQuotes
            | Object::DoubleQuotes
            | Object::VerticalBars
            | Object::AnyBrackets
            | Object::Parentheses
            | Object::SquareBrackets
            | Object::Tag
            | Object::Method
            | Object::Class
            | Object::Comment
            | Object::EntireFile
            | Object::CurlyBrackets
            | Object::AngleBrackets => true,
        }
    }

    pub fn target_visual_mode(self, current_mode: Mode, around: bool) -> Mode {
        match self {
            Object::Word { .. }
            | Object::Subword { .. }
            | Object::Sentence
            | Object::Quotes
            | Object::AnyQuotes
            | Object::BackQuotes
            | Object::DoubleQuotes => {
                if current_mode == Mode::VisualBlock {
                    Mode::VisualBlock
                } else {
                    Mode::Visual
                }
            }
            Object::Parentheses
            | Object::AnyBrackets
            | Object::SquareBrackets
            | Object::CurlyBrackets
            | Object::AngleBrackets
            | Object::VerticalBars
            | Object::Tag
            | Object::Comment
            | Object::Argument
            | Object::IndentObj { .. } => Mode::Visual,
            Object::Method | Object::Class => {
                if around {
                    Mode::VisualLine
                } else {
                    Mode::Visual
                }
            }
            Object::Paragraph | Object::EntireFile => Mode::VisualLine,
        }
    }

    pub fn range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let relative_to = selection.head();
        match self {
            Object::Word { ignore_punctuation } => {
                if around {
                    around_word(map, relative_to, ignore_punctuation)
                } else {
                    in_word(map, relative_to, ignore_punctuation)
                }
            }
            Object::Subword { ignore_punctuation } => {
                if around {
                    around_subword(map, relative_to, ignore_punctuation)
                } else {
                    in_subword(map, relative_to, ignore_punctuation)
                }
            }
            Object::Sentence => sentence(map, relative_to, around),
            Object::Paragraph => paragraph(map, relative_to, around),
            Object::Quotes => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '\'', '\'')
            }
            Object::BackQuotes => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '`', '`')
            }
            Object::AnyQuotes => {
                let quote_types = ['\'', '"', '`']; // Types of quotes to handle
                let relative_offset = relative_to.to_offset(map, Bias::Left) as isize;

                // Find the closest matching quote range
                quote_types
                    .iter()
                    .flat_map(|&quote| {
                        // Get ranges for each quote type
                        surrounding_markers(
                            map,
                            relative_to,
                            around,
                            self.is_multiline(),
                            quote,
                            quote,
                        )
                    })
                    .min_by_key(|range| calculate_range_distance(range, relative_offset, map))
            }
            Object::DoubleQuotes => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '"', '"')
            }
            Object::VerticalBars => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '|', '|')
            }
            Object::Parentheses => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '(', ')')
            }
            Object::Tag => {
                let head = selection.head();
                let range = selection.range();
                surrounding_html_tag(map, head, range, around)
            }
            Object::AnyBrackets => {
                let bracket_pairs = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
                let relative_offset = relative_to.to_offset(map, Bias::Left) as isize;

                bracket_pairs
                    .iter()
                    .flat_map(|&(open_bracket, close_bracket)| {
                        surrounding_markers(
                            map,
                            relative_to,
                            around,
                            self.is_multiline(),
                            open_bracket,
                            close_bracket,
                        )
                    })
                    .min_by_key(|range| calculate_range_distance(range, relative_offset, map))
            }
            Object::SquareBrackets => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '[', ']')
            }
            Object::CurlyBrackets => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '{', '}')
            }
            Object::AngleBrackets => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '<', '>')
            }
            Object::Method => text_object(
                map,
                relative_to,
                if around {
                    TextObject::AroundFunction
                } else {
                    TextObject::InsideFunction
                },
            ),
            Object::Comment => text_object(
                map,
                relative_to,
                if around {
                    TextObject::AroundComment
                } else {
                    TextObject::InsideComment
                },
            ),
            Object::Class => text_object(
                map,
                relative_to,
                if around {
                    TextObject::AroundClass
                } else {
                    TextObject::InsideClass
                },
            ),
            Object::Argument => argument(map, relative_to, around),
            Object::IndentObj { include_below } => indent(map, relative_to, around, include_below),
            Object::EntireFile => entire_file(map),
        }
    }

    pub fn expand_selection(
        self,
        map: &DisplaySnapshot,
        selection: &mut Selection<DisplayPoint>,
        around: bool,
    ) -> bool {
        if let Some(range) = self.range(map, selection.clone(), around) {
            selection.start = range.start;
            selection.end = range.end;
            true
        } else {
            false
        }
    }
}

/// Returns a range that surrounds the word `relative_to` is in.
///
/// If `relative_to` is at the start of a word, return the word.
/// If `relative_to` is between words, return the space between.
fn in_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Option<Range<DisplayPoint>> {
    // Use motion::right so that we consider the character under the cursor when looking for the start
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(relative_to.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    let start = movement::find_preceding_boundary_display_point(
        map,
        right(map, relative_to, 1),
        movement::FindRange::SingleLine,
        |left, right| classifier.kind(left) != classifier.kind(right),
    );

    let end = movement::find_boundary(map, relative_to, FindRange::SingleLine, |left, right| {
        classifier.kind(left) != classifier.kind(right)
    });

    Some(start..end)
}

fn in_subword(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Option<Range<DisplayPoint>> {
    let offset = relative_to.to_offset(map, Bias::Left);
    // Use motion::right so that we consider the character under the cursor when looking for the start
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(relative_to.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    let in_subword = map
        .buffer_chars_at(offset)
        .next()
        .map(|(c, _)| {
            if classifier.is_word('-') {
                !classifier.is_whitespace(c) && c != '_' && c != '-'
            } else {
                !classifier.is_whitespace(c) && c != '_'
            }
        })
        .unwrap_or(false);

    let start = if in_subword {
        movement::find_preceding_boundary_display_point(
            map,
            right(map, relative_to, 1),
            movement::FindRange::SingleLine,
            |left, right| {
                let is_word_start = classifier.kind(left) != classifier.kind(right);
                let is_subword_start = classifier.is_word('-') && left == '-' && right != '-'
                    || left == '_' && right != '_'
                    || left.is_lowercase() && right.is_uppercase();
                is_word_start || is_subword_start
            },
        )
    } else {
        movement::find_boundary(map, relative_to, FindRange::SingleLine, |left, right| {
            let is_word_start = classifier.kind(left) != classifier.kind(right);
            let is_subword_start = classifier.is_word('-') && left == '-' && right != '-'
                || left == '_' && right != '_'
                || left.is_lowercase() && right.is_uppercase();
            is_word_start || is_subword_start
        })
    };

    let end = movement::find_boundary(map, relative_to, FindRange::SingleLine, |left, right| {
        let is_word_end = classifier.kind(left) != classifier.kind(right);
        let is_subword_end = classifier.is_word('-') && left != '-' && right == '-'
            || left != '_' && right == '_'
            || left.is_lowercase() && right.is_uppercase();
        is_word_end || is_subword_end
    });

    Some(start..end)
}

pub fn surrounding_html_tag(
    map: &DisplaySnapshot,
    head: DisplayPoint,
    range: Range<DisplayPoint>,
    around: bool,
) -> Option<Range<DisplayPoint>> {
    fn read_tag(chars: impl Iterator<Item = char>) -> String {
        chars
            .take_while(|c| c.is_alphanumeric() || *c == ':' || *c == '-' || *c == '_' || *c == '.')
            .collect()
    }
    fn open_tag(mut chars: impl Iterator<Item = char>) -> Option<String> {
        if Some('<') != chars.next() {
            return None;
        }
        Some(read_tag(chars))
    }
    fn close_tag(mut chars: impl Iterator<Item = char>) -> Option<String> {
        if (Some('<'), Some('/')) != (chars.next(), chars.next()) {
            return None;
        }
        Some(read_tag(chars))
    }

    let snapshot = &map.buffer_snapshot;
    let offset = head.to_offset(map, Bias::Left);
    let mut excerpt = snapshot.excerpt_containing(offset..offset)?;
    let buffer = excerpt.buffer();
    let offset = excerpt.map_offset_to_buffer(offset);

    // Find the most closest to current offset
    let mut cursor = buffer.syntax_layer_at(offset)?.node().walk();
    let mut last_child_node = cursor.node();
    while cursor.goto_first_child_for_byte(offset).is_some() {
        last_child_node = cursor.node();
    }

    let mut last_child_node = Some(last_child_node);
    while let Some(cur_node) = last_child_node {
        if cur_node.child_count() >= 2 {
            let first_child = cur_node.child(0);
            let last_child = cur_node.child(cur_node.child_count() - 1);
            if let (Some(first_child), Some(last_child)) = (first_child, last_child) {
                let open_tag = open_tag(buffer.chars_for_range(first_child.byte_range()));
                let close_tag = close_tag(buffer.chars_for_range(last_child.byte_range()));
                // It needs to be handled differently according to the selection length
                let is_valid = if range.end.to_offset(map, Bias::Left)
                    - range.start.to_offset(map, Bias::Left)
                    <= 1
                {
                    offset <= last_child.end_byte()
                } else {
                    range.start.to_offset(map, Bias::Left) >= first_child.start_byte()
                        && range.end.to_offset(map, Bias::Left) <= last_child.start_byte() + 1
                };
                if open_tag.is_some() && open_tag == close_tag && is_valid {
                    let range = if around {
                        first_child.byte_range().start..last_child.byte_range().end
                    } else {
                        first_child.byte_range().end..last_child.byte_range().start
                    };
                    if excerpt.contains_buffer_range(range.clone()) {
                        let result = excerpt.map_range_from_buffer(range);
                        return Some(
                            result.start.to_display_point(map)..result.end.to_display_point(map),
                        );
                    }
                }
            }
        }
        last_child_node = cur_node.parent();
    }
    None
}

/// Returns a range that surrounds the word and following whitespace
/// relative_to is in.
///
/// If `relative_to` is at the start of a word, return the word and following whitespace.
/// If `relative_to` is between words, return the whitespace back and the following word.
///
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
    let offset = relative_to.to_offset(map, Bias::Left);
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(offset)
        .ignore_punctuation(ignore_punctuation);
    let in_word = map
        .buffer_chars_at(offset)
        .next()
        .map(|(c, _)| !classifier.is_whitespace(c))
        .unwrap_or(false);

    if in_word {
        around_containing_word(map, relative_to, ignore_punctuation)
    } else {
        around_next_word(map, relative_to, ignore_punctuation)
    }
}

/// Calculate distance between a range and a cursor position
///
/// Returns a score where:
/// - Lower values indicate better matches
/// - Range containing cursor gets priority (returns range length)
/// - For non-containing ranges, uses minimum distance to boundaries as primary factor
/// - Range length is used as secondary factor for tiebreaking
fn calculate_range_distance(
    range: &Range<DisplayPoint>,
    cursor_offset: isize,
    map: &DisplaySnapshot,
) -> isize {
    let start_offset = range.start.to_offset(map, Bias::Left) as isize;
    let end_offset = range.end.to_offset(map, Bias::Right) as isize;
    let range_length = end_offset - start_offset;

    // If cursor is inside the range, return range length
    if cursor_offset >= start_offset && cursor_offset <= end_offset {
        return range_length;
    }

    // Calculate minimum distance to range boundaries
    let start_distance = (cursor_offset - start_offset).abs();
    let end_distance = (cursor_offset - end_offset).abs();
    let min_distance = start_distance.min(end_distance);

    // Use min_distance as primary factor, range_length as secondary
    // Multiply by large number to ensure distance is primary factor
    min_distance * 10000 + range_length
}

fn around_subword(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Option<Range<DisplayPoint>> {
    // Use motion::right so that we consider the character under the cursor when looking for the start
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(relative_to.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    let start = movement::find_preceding_boundary_display_point(
        map,
        right(map, relative_to, 1),
        movement::FindRange::SingleLine,
        |left, right| {
            let is_word_start = classifier.kind(left) != classifier.kind(right);
            let is_subword_start = classifier.is_word('-') && left != '-' && right == '-'
                || left != '_' && right == '_'
                || left.is_lowercase() && right.is_uppercase();
            is_word_start || is_subword_start
        },
    );

    let end = movement::find_boundary(map, relative_to, FindRange::SingleLine, |left, right| {
        let is_word_end = classifier.kind(left) != classifier.kind(right);
        let is_subword_end = classifier.is_word('-') && left != '-' && right == '-'
            || left != '_' && right == '_'
            || left.is_lowercase() && right.is_uppercase();
        is_word_end || is_subword_end
    });

    Some(start..end).map(|range| expand_to_include_whitespace(map, range, true))
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
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(relative_to.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    // Get the start of the word
    let start = movement::find_preceding_boundary_display_point(
        map,
        right(map, relative_to, 1),
        FindRange::SingleLine,
        |left, right| classifier.kind(left) != classifier.kind(right),
    );

    let mut word_found = false;
    let end = movement::find_boundary(map, relative_to, FindRange::MultiLine, |left, right| {
        let left_kind = classifier.kind(left);
        let right_kind = classifier.kind(right);

        let found = (word_found && left_kind != right_kind) || right == '\n' && left == '\n';

        if right_kind != CharKind::Whitespace {
            word_found = true;
        }

        found
    });

    Some(start..end)
}

fn entire_file(map: &DisplaySnapshot) -> Option<Range<DisplayPoint>> {
    Some(DisplayPoint::zero()..map.max_point())
}

fn text_object(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    target: TextObject,
) -> Option<Range<DisplayPoint>> {
    let snapshot = &map.buffer_snapshot;
    let offset = relative_to.to_offset(map, Bias::Left);

    let mut excerpt = snapshot.excerpt_containing(offset..offset)?;
    let buffer = excerpt.buffer();
    let offset = excerpt.map_offset_to_buffer(offset);

    let mut matches: Vec<Range<usize>> = buffer
        .text_object_ranges(offset..offset, TreeSitterOptions::default())
        .filter_map(|(r, m)| if m == target { Some(r) } else { None })
        .collect();
    matches.sort_by_key(|r| (r.end - r.start));
    if let Some(buffer_range) = matches.first() {
        let range = excerpt.map_range_from_buffer(buffer_range.clone());
        return Some(range.start.to_display_point(map)..range.end.to_display_point(map));
    }

    let around = target.around()?;
    let mut matches: Vec<Range<usize>> = buffer
        .text_object_ranges(offset..offset, TreeSitterOptions::default())
        .filter_map(|(r, m)| if m == around { Some(r) } else { None })
        .collect();
    matches.sort_by_key(|r| (r.end - r.start));
    let around_range = matches.first()?;

    let mut matches: Vec<Range<usize>> = buffer
        .text_object_ranges(around_range.clone(), TreeSitterOptions::default())
        .filter_map(|(r, m)| if m == target { Some(r) } else { None })
        .collect();
    matches.sort_by_key(|r| r.start);
    if let Some(buffer_range) = matches.first() {
        if !buffer_range.is_empty() {
            let range = excerpt.map_range_from_buffer(buffer_range.clone());
            return Some(range.start.to_display_point(map)..range.end.to_display_point(map));
        }
    }
    let buffer_range = excerpt.map_range_from_buffer(around_range.clone());
    return Some(buffer_range.start.to_display_point(map)..buffer_range.end.to_display_point(map));
}

fn argument(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    around: bool,
) -> Option<Range<DisplayPoint>> {
    let snapshot = &map.buffer_snapshot;
    let offset = relative_to.to_offset(map, Bias::Left);

    // The `argument` vim text object uses the syntax tree, so we operate at the buffer level and map back to the display level
    let mut excerpt = snapshot.excerpt_containing(offset..offset)?;
    let buffer = excerpt.buffer();

    fn comma_delimited_range_at(
        buffer: &BufferSnapshot,
        mut offset: usize,
        include_comma: bool,
    ) -> Option<Range<usize>> {
        // Seek to the first non-whitespace character
        offset += buffer
            .chars_at(offset)
            .take_while(|c| c.is_whitespace())
            .map(char::len_utf8)
            .sum::<usize>();

        let bracket_filter = |open: Range<usize>, close: Range<usize>| {
            // Filter out empty ranges
            if open.end == close.start {
                return false;
            }

            // If the cursor is outside the brackets, ignore them
            if open.start == offset || close.end == offset {
                return false;
            }

            // TODO: Is there any better way to filter out string brackets?
            // Used to filter out string brackets
            matches!(
                buffer.chars_at(open.start).next(),
                Some('(' | '[' | '{' | '<' | '|')
            )
        };

        // Find the brackets containing the cursor
        let (open_bracket, close_bracket) =
            buffer.innermost_enclosing_bracket_ranges(offset..offset, Some(&bracket_filter))?;

        let inner_bracket_range = open_bracket.end..close_bracket.start;

        let layer = buffer.syntax_layer_at(offset)?;
        let node = layer.node();
        let mut cursor = node.walk();

        // Loop until we find the smallest node whose parent covers the bracket range. This node is the argument in the parent argument list
        let mut parent_covers_bracket_range = false;
        loop {
            let node = cursor.node();
            let range = node.byte_range();
            let covers_bracket_range =
                range.start == open_bracket.start && range.end == close_bracket.end;
            if parent_covers_bracket_range && !covers_bracket_range {
                break;
            }
            parent_covers_bracket_range = covers_bracket_range;

            // Unable to find a child node with a parent that covers the bracket range, so no argument to select
            cursor.goto_first_child_for_byte(offset)?;
        }

        let mut argument_node = cursor.node();

        // If the child node is the open bracket, move to the next sibling.
        if argument_node.byte_range() == open_bracket {
            if !cursor.goto_next_sibling() {
                return Some(inner_bracket_range);
            }
            argument_node = cursor.node();
        }
        // While the child node is the close bracket or a comma, move to the previous sibling
        while argument_node.byte_range() == close_bracket || argument_node.kind() == "," {
            if !cursor.goto_previous_sibling() {
                return Some(inner_bracket_range);
            }
            argument_node = cursor.node();
            if argument_node.byte_range() == open_bracket {
                return Some(inner_bracket_range);
            }
        }

        // The start and end of the argument range, defaulting to the start and end of the argument node
        let mut start = argument_node.start_byte();
        let mut end = argument_node.end_byte();

        let mut needs_surrounding_comma = include_comma;

        // Seek backwards to find the start of the argument - either the previous comma or the opening bracket.
        // We do this because multiple nodes can represent a single argument, such as with rust `vec![a.b.c, d.e.f]`
        while cursor.goto_previous_sibling() {
            let prev = cursor.node();

            if prev.start_byte() < open_bracket.end {
                start = open_bracket.end;
                break;
            } else if prev.kind() == "," {
                if needs_surrounding_comma {
                    start = prev.start_byte();
                    needs_surrounding_comma = false;
                }
                break;
            } else if prev.start_byte() < start {
                start = prev.start_byte();
            }
        }

        // Do the same for the end of the argument, extending to next comma or the end of the argument list
        while cursor.goto_next_sibling() {
            let next = cursor.node();

            if next.end_byte() > close_bracket.start {
                end = close_bracket.start;
                break;
            } else if next.kind() == "," {
                if needs_surrounding_comma {
                    // Select up to the beginning of the next argument if there is one, otherwise to the end of the comma
                    if let Some(next_arg) = next.next_sibling() {
                        end = next_arg.start_byte();
                    } else {
                        end = next.end_byte();
                    }
                }
                break;
            } else if next.end_byte() > end {
                end = next.end_byte();
            }
        }

        Some(start..end)
    }

    let result = comma_delimited_range_at(buffer, excerpt.map_offset_to_buffer(offset), around)?;

    if excerpt.contains_buffer_range(result.clone()) {
        let result = excerpt.map_range_from_buffer(result);
        Some(result.start.to_display_point(map)..result.end.to_display_point(map))
    } else {
        None
    }
}

fn indent(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    around: bool,
    include_below: bool,
) -> Option<Range<DisplayPoint>> {
    let point = relative_to.to_point(map);
    let row = point.row;

    let desired_indent = map.line_indent_for_buffer_row(MultiBufferRow(row));

    // Loop backwards until we find a non-blank line with less indent
    let mut start_row = row;
    for prev_row in (0..row).rev() {
        let indent = map.line_indent_for_buffer_row(MultiBufferRow(prev_row));
        if indent.is_line_empty() {
            continue;
        }
        if indent.spaces < desired_indent.spaces || indent.tabs < desired_indent.tabs {
            if around {
                // When around is true, include the first line with less indent
                start_row = prev_row;
            }
            break;
        }
        start_row = prev_row;
    }

    // Loop forwards until we find a non-blank line with less indent
    let mut end_row = row;
    let max_rows = map.buffer_snapshot.max_row().0;
    for next_row in (row + 1)..=max_rows {
        let indent = map.line_indent_for_buffer_row(MultiBufferRow(next_row));
        if indent.is_line_empty() {
            continue;
        }
        if indent.spaces < desired_indent.spaces || indent.tabs < desired_indent.tabs {
            if around && include_below {
                // When around is true and including below, include this line
                end_row = next_row;
            }
            break;
        }
        end_row = next_row;
    }

    let end_len = map.buffer_snapshot.line_len(MultiBufferRow(end_row));
    let start = map.point_to_display_point(Point::new(start_row, 0), Bias::Right);
    let end = map.point_to_display_point(Point::new(end_row, end_len), Bias::Left);
    Some(start..end)
}

fn sentence(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    around: bool,
) -> Option<Range<DisplayPoint>> {
    let mut start = None;
    let relative_offset = relative_to.to_offset(map, Bias::Left);
    let mut previous_end = relative_offset;

    let mut chars = map.buffer_chars_at(previous_end).peekable();

    // Search backwards for the previous sentence end or current sentence start. Include the character under relative_to
    for (char, offset) in chars
        .peek()
        .cloned()
        .into_iter()
        .chain(map.reverse_buffer_chars_at(previous_end))
    {
        if is_sentence_end(map, offset) {
            break;
        }

        if is_possible_sentence_start(char) {
            start = Some(offset);
        }

        previous_end = offset;
    }

    // Search forward for the end of the current sentence or if we are between sentences, the start of the next one
    let mut end = relative_offset;
    for (char, offset) in chars {
        if start.is_none() && is_possible_sentence_start(char) {
            if around {
                start = Some(offset);
                continue;
            } else {
                end = offset;
                break;
            }
        }

        if char != '\n' {
            end = offset + char.len_utf8();
        }

        if is_sentence_end(map, end) {
            break;
        }
    }

    let mut range = start.unwrap_or(previous_end).to_display_point(map)..end.to_display_point(map);
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
fn is_sentence_end(map: &DisplaySnapshot, offset: usize) -> bool {
    let mut next_chars = map.buffer_chars_at(offset).peekable();
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

    for (char, _) in map.reverse_buffer_chars_at(offset) {
        if SENTENCE_END_PUNCTUATION.contains(&char) {
            return true;
        }

        if !SENTENCE_END_FILLERS.contains(&char) {
            return false;
        }
    }

    false
}

/// Expands the passed range to include whitespace on one side or the other in a line. Attempts to add the
/// whitespace to the end first and falls back to the start if there was none.
fn expand_to_include_whitespace(
    map: &DisplaySnapshot,
    range: Range<DisplayPoint>,
    stop_at_newline: bool,
) -> Range<DisplayPoint> {
    let mut range = range.start.to_offset(map, Bias::Left)..range.end.to_offset(map, Bias::Right);
    let mut whitespace_included = false;

    let chars = map.buffer_chars_at(range.end).peekable();
    for (char, offset) in chars {
        if char == '\n' && stop_at_newline {
            break;
        }

        if char.is_whitespace() {
            if char != '\n' {
                range.end = offset + char.len_utf8();
                whitespace_included = true;
            }
        } else {
            // Found non whitespace. Quit out.
            break;
        }
    }

    if !whitespace_included {
        for (char, point) in map.reverse_buffer_chars_at(range.start) {
            if char == '\n' && stop_at_newline {
                break;
            }

            if !char.is_whitespace() {
                break;
            }

            range.start = point;
        }
    }

    range.start.to_display_point(map)..range.end.to_display_point(map)
}

/// If not `around` (i.e. inner), returns a range that surrounds the paragraph
/// where `relative_to` is in. If `around`, principally returns the range ending
/// at the end of the next paragraph.
///
/// Here, the "paragraph" is defined as a block of non-blank lines or a block of
/// blank lines. If the paragraph ends with a trailing newline (i.e. not with
/// EOF), the returned range ends at the trailing newline of the paragraph (i.e.
/// the trailing newline is not subject to subsequent operations).
///
/// Edge cases:
/// - If `around` and if the current paragraph is the last paragraph of the
///   file and is blank, then the selection results in an error.
/// - If `around` and if the current paragraph is the last paragraph of the
///   file and is not blank, then the returned range starts at the start of the
///   previous paragraph, if it exists.
fn paragraph(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    around: bool,
) -> Option<Range<DisplayPoint>> {
    let mut paragraph_start = start_of_paragraph(map, relative_to);
    let mut paragraph_end = end_of_paragraph(map, relative_to);

    let paragraph_end_row = paragraph_end.row();
    let paragraph_ends_with_eof = paragraph_end_row == map.max_point().row();
    let point = relative_to.to_point(map);
    let current_line_is_empty = map.buffer_snapshot.is_line_blank(MultiBufferRow(point.row));

    if around {
        if paragraph_ends_with_eof {
            if current_line_is_empty {
                return None;
            }

            let paragraph_start_row = paragraph_start.row();
            if paragraph_start_row.0 != 0 {
                let previous_paragraph_last_line_start =
                    DisplayPoint::new(paragraph_start_row - 1, 0);
                paragraph_start = start_of_paragraph(map, previous_paragraph_last_line_start);
            }
        } else {
            let next_paragraph_start = DisplayPoint::new(paragraph_end_row + 1, 0);
            paragraph_end = end_of_paragraph(map, next_paragraph_start);
        }
    }

    let range = paragraph_start..paragraph_end;
    Some(range)
}

/// Returns a position of the start of the current paragraph, where a paragraph
/// is defined as a run of non-blank lines or a run of blank lines.
pub fn start_of_paragraph(map: &DisplaySnapshot, display_point: DisplayPoint) -> DisplayPoint {
    let point = display_point.to_point(map);
    if point.row == 0 {
        return DisplayPoint::zero();
    }

    let is_current_line_blank = map.buffer_snapshot.is_line_blank(MultiBufferRow(point.row));

    for row in (0..point.row).rev() {
        let blank = map.buffer_snapshot.is_line_blank(MultiBufferRow(row));
        if blank != is_current_line_blank {
            return Point::new(row + 1, 0).to_display_point(map);
        }
    }

    DisplayPoint::zero()
}

/// Returns a position of the end of the current paragraph, where a paragraph
/// is defined as a run of non-blank lines or a run of blank lines.
/// The trailing newline is excluded from the paragraph.
pub fn end_of_paragraph(map: &DisplaySnapshot, display_point: DisplayPoint) -> DisplayPoint {
    let point = display_point.to_point(map);
    if point.row == map.buffer_snapshot.max_row().0 {
        return map.max_point();
    }

    let is_current_line_blank = map.buffer_snapshot.is_line_blank(MultiBufferRow(point.row));

    for row in point.row + 1..map.buffer_snapshot.max_row().0 + 1 {
        let blank = map.buffer_snapshot.is_line_blank(MultiBufferRow(row));
        if blank != is_current_line_blank {
            let previous_row = row - 1;
            return Point::new(
                previous_row,
                map.buffer_snapshot.line_len(MultiBufferRow(previous_row)),
            )
            .to_display_point(map);
        }
    }

    map.max_point()
}

fn surrounding_markers(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    around: bool,
    search_across_lines: bool,
    open_marker: char,
    close_marker: char,
) -> Option<Range<DisplayPoint>> {
    let point = relative_to.to_offset(map, Bias::Left);

    let mut matched_closes = 0;
    let mut opening = None;

    let mut before_ch = match movement::chars_before(map, point).next() {
        Some((ch, _)) => ch,
        _ => '\0',
    };
    if let Some((ch, range)) = movement::chars_after(map, point).next() {
        if ch == open_marker && before_ch != '\\' {
            if open_marker == close_marker {
                let mut total = 0;
                for ((ch, _), (before_ch, _)) in movement::chars_before(map, point).tuple_windows()
                {
                    if ch == '\n' {
                        break;
                    }
                    if ch == open_marker && before_ch != '\\' {
                        total += 1;
                    }
                }
                if total % 2 == 0 {
                    opening = Some(range)
                }
            } else {
                opening = Some(range)
            }
        }
    }

    if opening.is_none() {
        let mut chars_before = movement::chars_before(map, point).peekable();
        while let Some((ch, range)) = chars_before.next() {
            if ch == '\n' && !search_across_lines {
                break;
            }

            if let Some((before_ch, _)) = chars_before.peek() {
                if *before_ch == '\\' {
                    continue;
                }
            }

            if ch == open_marker {
                if matched_closes == 0 {
                    opening = Some(range);
                    break;
                }
                matched_closes -= 1;
            } else if ch == close_marker {
                matched_closes += 1
            }
        }
    }
    if opening.is_none() {
        for (ch, range) in movement::chars_after(map, point) {
            if before_ch != '\\' {
                if ch == open_marker {
                    opening = Some(range);
                    break;
                } else if ch == close_marker {
                    break;
                }
            }

            before_ch = ch;
        }
    }

    let mut opening = opening?;

    let mut matched_opens = 0;
    let mut closing = None;
    before_ch = match movement::chars_before(map, opening.end).next() {
        Some((ch, _)) => ch,
        _ => '\0',
    };
    for (ch, range) in movement::chars_after(map, opening.end) {
        if ch == '\n' && !search_across_lines {
            break;
        }

        if before_ch != '\\' {
            if ch == close_marker {
                if matched_opens == 0 {
                    closing = Some(range);
                    break;
                }
                matched_opens -= 1;
            } else if ch == open_marker {
                matched_opens += 1;
            }
        }

        before_ch = ch;
    }

    let mut closing = closing?;

    if around && !search_across_lines {
        let mut found = false;

        for (ch, range) in movement::chars_after(map, closing.end) {
            if ch.is_whitespace() && ch != '\n' {
                found = true;
                closing.end = range.end;
            } else {
                break;
            }
        }

        if !found {
            for (ch, range) in movement::chars_before(map, opening.start) {
                if ch.is_whitespace() && ch != '\n' {
                    opening.start = range.start
                } else {
                    break;
                }
            }
        }
    }

    if !around && search_across_lines {
        if let Some((ch, range)) = movement::chars_after(map, opening.end).next() {
            if ch == '\n' {
                opening.end = range.end
            }
        }

        let mut last_newline_end = None;
        for (ch, range) in movement::chars_before(map, closing.start) {
            if !ch.is_whitespace() {
                break;
            }
            if ch == '\n' {
                last_newline_end = Some(range.end);
                break;
            }
        }
        // Adjust closing.start to exclude whitespace after a newline, if present
        if let Some(end) = last_newline_end {
            closing.start = end;
        }
    }

    let result = if around {
        opening.start..closing.end
    } else {
        opening.end..closing.start
    };

    Some(
        map.clip_point(result.start.to_display_point(map), Bias::Left)
            ..map.clip_point(result.end.to_display_point(map), Bias::Right),
    )
}

#[cfg(test)]
mod test {
    use gpui::KeyBinding;
    use indoc::indoc;

    use crate::{
        object::AnyBrackets,
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    const WORD_LOCATIONS: &str = indoc! {"
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

        cx.simulate_at_each_offset("c i w", WORD_LOCATIONS)
            .await
            .assert_matches();
        cx.simulate_at_each_offset("c i shift-w", WORD_LOCATIONS)
            .await
            .assert_matches();
        cx.simulate_at_each_offset("c a w", WORD_LOCATIONS)
            .await
            .assert_matches();
        cx.simulate_at_each_offset("c a shift-w", WORD_LOCATIONS)
            .await
            .assert_matches();
    }

    #[gpui::test]
    async fn test_delete_word_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.simulate_at_each_offset("d i w", WORD_LOCATIONS)
            .await
            .assert_matches();
        cx.simulate_at_each_offset("d i shift-w", WORD_LOCATIONS)
            .await
            .assert_matches();
        cx.simulate_at_each_offset("d a w", WORD_LOCATIONS)
            .await
            .assert_matches();
        cx.simulate_at_each_offset("d a shift-w", WORD_LOCATIONS)
            .await
            .assert_matches();
    }

    #[gpui::test]
    async fn test_visual_word_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        /*
                cx.set_shared_state("The quick ˇbrown\nfox").await;
                cx.simulate_shared_keystrokes(["v"]).await;
                cx.assert_shared_state("The quick «bˇ»rown\nfox").await;
                cx.simulate_shared_keystrokes(["i", "w"]).await;
                cx.assert_shared_state("The quick «brownˇ»\nfox").await;
        */
        cx.set_shared_state("The quick brown\nˇ\nfox").await;
        cx.simulate_shared_keystrokes("v").await;
        cx.shared_state()
            .await
            .assert_eq("The quick brown\n«\nˇ»fox");
        cx.simulate_shared_keystrokes("i w").await;
        cx.shared_state()
            .await
            .assert_eq("The quick brown\n«\nˇ»fox");

        cx.simulate_at_each_offset("v i w", WORD_LOCATIONS)
            .await
            .assert_matches();
        cx.simulate_at_each_offset("v i shift-w", WORD_LOCATIONS)
            .await
            .assert_matches();
    }

    const PARAGRAPH_EXAMPLES: &[&str] = &[
        // Single line
        "ˇThe quick brown fox jumpˇs over the lazy dogˇ.ˇ",
        // Multiple lines without empty lines
        indoc! {"
            ˇThe quick brownˇ
            ˇfox jumps overˇ
            the lazy dog.ˇ
        "},
        // Heading blank paragraph and trailing normal paragraph
        indoc! {"
            ˇ
            ˇ
            ˇThe quick brown fox jumps
            ˇover the lazy dog.
            ˇ
            ˇ
            ˇThe quick brown fox jumpsˇ
            ˇover the lazy dog.ˇ
        "},
        // Inserted blank paragraph and trailing blank paragraph
        indoc! {"
            ˇThe quick brown fox jumps
            ˇover the lazy dog.
            ˇ
            ˇ
            ˇ
            ˇThe quick brown fox jumpsˇ
            ˇover the lazy dog.ˇ
            ˇ
            ˇ
            ˇ
        "},
        // "Blank" paragraph with whitespace characters
        indoc! {"
            ˇThe quick brown fox jumps
            over the lazy dog.

            ˇ \t

            ˇThe quick brown fox jumps
            over the lazy dog.ˇ
            ˇ
            ˇ \t
            \t \t
        "},
        // Single line "paragraphs", where selection size might be zero.
        indoc! {"
            ˇThe quick brown fox jumps over the lazy dog.
            ˇ
            ˇThe quick brown fox jumpˇs over the lazy dog.ˇ
            ˇ
        "},
    ];

    #[gpui::test]
    async fn test_change_paragraph_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for paragraph_example in PARAGRAPH_EXAMPLES {
            cx.simulate_at_each_offset("c i p", paragraph_example)
                .await
                .assert_matches();
            cx.simulate_at_each_offset("c a p", paragraph_example)
                .await
                .assert_matches();
        }
    }

    #[gpui::test]
    async fn test_delete_paragraph_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for paragraph_example in PARAGRAPH_EXAMPLES {
            cx.simulate_at_each_offset("d i p", paragraph_example)
                .await
                .assert_matches();
            cx.simulate_at_each_offset("d a p", paragraph_example)
                .await
                .assert_matches();
        }
    }

    #[gpui::test]
    async fn test_visual_paragraph_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        const EXAMPLES: &[&str] = &[
            indoc! {"
                ˇThe quick brown
                fox jumps over
                the lazy dog.
            "},
            indoc! {"
                ˇ

                ˇThe quick brown fox jumps
                over the lazy dog.
                ˇ

                ˇThe quick brown fox jumps
                over the lazy dog.
            "},
            indoc! {"
                ˇThe quick brown fox jumps over the lazy dog.
                ˇ
                ˇThe quick brown fox jumps over the lazy dog.

            "},
        ];

        for paragraph_example in EXAMPLES {
            cx.simulate_at_each_offset("v i p", paragraph_example)
                .await
                .assert_matches();
            cx.simulate_at_each_offset("v a p", paragraph_example)
                .await
                .assert_matches();
        }
    }

    // Test string with "`" for opening surrounders and "'" for closing surrounders
    const SURROUNDING_MARKER_STRING: &str = indoc! {"
        ˇTh'ˇe ˇ`ˇ'ˇquˇi`ˇck broˇ'wn`
        'ˇfox juˇmps ov`ˇer
        the ˇlazy d'o`ˇg"};

    const SURROUNDING_OBJECTS: &[(char, char)] = &[
        ('"', '"'), // Double Quote
        ('(', ')'), // Parentheses
    ];

    #[gpui::test]
    async fn test_change_surrounding_character_objects(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for (start, end) in SURROUNDING_OBJECTS {
            let marked_string = SURROUNDING_MARKER_STRING
                .replace('`', &start.to_string())
                .replace('\'', &end.to_string());

            cx.simulate_at_each_offset(&format!("c i {start}"), &marked_string)
                .await
                .assert_matches();
            cx.simulate_at_each_offset(&format!("c i {end}"), &marked_string)
                .await
                .assert_matches();
            cx.simulate_at_each_offset(&format!("c a {start}"), &marked_string)
                .await
                .assert_matches();
            cx.simulate_at_each_offset(&format!("c a {end}"), &marked_string)
                .await
                .assert_matches();
        }
    }
    #[gpui::test]
    async fn test_singleline_surrounding_character_objects(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_wrap(12).await;

        cx.set_shared_state(indoc! {
            "\"ˇhello world\"!"
        })
        .await;
        cx.simulate_shared_keystrokes("v i \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "\"«hello worldˇ»\"!"
        });

        cx.set_shared_state(indoc! {
            "\"hˇello world\"!"
        })
        .await;
        cx.simulate_shared_keystrokes("v i \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "\"«hello worldˇ»\"!"
        });

        cx.set_shared_state(indoc! {
            "helˇlo \"world\"!"
        })
        .await;
        cx.simulate_shared_keystrokes("v i \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "hello \"«worldˇ»\"!"
        });

        cx.set_shared_state(indoc! {
            "hello \"wˇorld\"!"
        })
        .await;
        cx.simulate_shared_keystrokes("v i \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "hello \"«worldˇ»\"!"
        });

        cx.set_shared_state(indoc! {
            "hello \"wˇorld\"!"
        })
        .await;
        cx.simulate_shared_keystrokes("v a \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "hello« \"world\"ˇ»!"
        });

        cx.set_shared_state(indoc! {
            "hello \"wˇorld\" !"
        })
        .await;
        cx.simulate_shared_keystrokes("v a \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "hello «\"world\" ˇ»!"
        });

        cx.set_shared_state(indoc! {
            "hello \"wˇorld\"•
            goodbye"
        })
        .await;
        cx.simulate_shared_keystrokes("v a \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "hello «\"world\" ˇ»
            goodbye"
        });
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
        cx.simulate_shared_keystrokes("v i {").await;
        cx.shared_state().await.assert_eq(indoc! {"
            func empty(a string) bool {
            «   if a == \"\" {
                  return true
               }
               return false
            ˇ»}"});
        cx.set_shared_state(indoc! {
            "func empty(a string) bool {
                 if a == \"\" {
                     ˇreturn true
                 }
                 return false
            }"
        })
        .await;
        cx.simulate_shared_keystrokes("v i {").await;
        cx.shared_state().await.assert_eq(indoc! {"
            func empty(a string) bool {
                 if a == \"\" {
            «         return true
            ˇ»     }
                 return false
            }"});

        cx.set_shared_state(indoc! {
            "func empty(a string) bool {
                 if a == \"\" ˇ{
                     return true
                 }
                 return false
            }"
        })
        .await;
        cx.simulate_shared_keystrokes("v i {").await;
        cx.shared_state().await.assert_eq(indoc! {"
            func empty(a string) bool {
                 if a == \"\" {
            «         return true
            ˇ»     }
                 return false
            }"});
    }

    #[gpui::test]
    async fn test_singleline_surrounding_character_objects_with_escape(
        cx: &mut gpui::TestAppContext,
    ) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {
            "h\"e\\\"lˇlo \\\"world\"!"
        })
        .await;
        cx.simulate_shared_keystrokes("v i \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "h\"«e\\\"llo \\\"worldˇ»\"!"
        });

        cx.set_shared_state(indoc! {
            "hello \"teˇst \\\"inside\\\" world\""
        })
        .await;
        cx.simulate_shared_keystrokes("v i \"").await;
        cx.shared_state().await.assert_eq(indoc! {
            "hello \"«test \\\"inside\\\" worldˇ»\""
        });
    }

    #[gpui::test]
    async fn test_vertical_bars(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(
            indoc! {"
            fn boop() {
                baz(ˇ|a, b| { bar(|j, k| { })})
            }"
            },
            Mode::Normal,
        );
        cx.simulate_keystrokes("c i |");
        cx.assert_state(
            indoc! {"
            fn boop() {
                baz(|ˇ| { bar(|j, k| { })})
            }"
            },
            Mode::Insert,
        );
        cx.simulate_keystrokes("escape 1 8 |");
        cx.assert_state(
            indoc! {"
            fn boop() {
                baz(|| { bar(ˇ|j, k| { })})
            }"
            },
            Mode::Normal,
        );

        cx.simulate_keystrokes("v a |");
        cx.assert_state(
            indoc! {"
            fn boop() {
                baz(|| { bar(«|j, k| ˇ»{ })})
            }"
            },
            Mode::Visual,
        );
    }

    #[gpui::test]
    async fn test_argument_object(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Generic arguments
        cx.set_state("fn boop<A: ˇDebug, B>() {}", Mode::Normal);
        cx.simulate_keystrokes("v i a");
        cx.assert_state("fn boop<«A: Debugˇ», B>() {}", Mode::Visual);

        // Function arguments
        cx.set_state(
            "fn boop(ˇarg_a: (Tuple, Of, Types), arg_b: String) {}",
            Mode::Normal,
        );
        cx.simulate_keystrokes("d a a");
        cx.assert_state("fn boop(ˇarg_b: String) {}", Mode::Normal);

        cx.set_state("std::namespace::test(\"strinˇg\", a.b.c())", Mode::Normal);
        cx.simulate_keystrokes("v a a");
        cx.assert_state("std::namespace::test(«\"string\", ˇ»a.b.c())", Mode::Visual);

        // Tuple, vec, and array arguments
        cx.set_state(
            "fn boop(arg_a: (Tuple, Ofˇ, Types), arg_b: String) {}",
            Mode::Normal,
        );
        cx.simulate_keystrokes("c i a");
        cx.assert_state(
            "fn boop(arg_a: (Tuple, ˇ, Types), arg_b: String) {}",
            Mode::Insert,
        );

        cx.set_state("let a = (test::call(), 'p', my_macro!{ˇ});", Mode::Normal);
        cx.simulate_keystrokes("c a a");
        cx.assert_state("let a = (test::call(), 'p'ˇ);", Mode::Insert);

        cx.set_state("let a = [test::call(ˇ), 300];", Mode::Normal);
        cx.simulate_keystrokes("c i a");
        cx.assert_state("let a = [ˇ, 300];", Mode::Insert);

        cx.set_state(
            "let a = vec![Vec::new(), vecˇ![test::call(), 300]];",
            Mode::Normal,
        );
        cx.simulate_keystrokes("c a a");
        cx.assert_state("let a = vec![Vec::new()ˇ];", Mode::Insert);

        // Cursor immediately before / after brackets
        cx.set_state("let a = [test::call(first_arg)ˇ]", Mode::Normal);
        cx.simulate_keystrokes("v i a");
        cx.assert_state("let a = [«test::call(first_arg)ˇ»]", Mode::Visual);

        cx.set_state("let a = [test::callˇ(first_arg)]", Mode::Normal);
        cx.simulate_keystrokes("v i a");
        cx.assert_state("let a = [«test::call(first_arg)ˇ»]", Mode::Visual);
    }

    #[gpui::test]
    async fn test_indent_object(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Base use case
        cx.set_state(
            indoc! {"
                fn boop() {
                    // Comment
                    baz();ˇ

                    loop {
                        bar(1);
                        bar(2);
                    }

                    result
                }
            "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("v i i");
        cx.assert_state(
            indoc! {"
                fn boop() {
                «    // Comment
                    baz();

                    loop {
                        bar(1);
                        bar(2);
                    }

                    resultˇ»
                }
            "},
            Mode::Visual,
        );

        // Around indent (include line above)
        cx.set_state(
            indoc! {"
                const ABOVE: str = true;
                fn boop() {

                    hello();
                    worˇld()
                }
            "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("v a i");
        cx.assert_state(
            indoc! {"
                const ABOVE: str = true;
                «fn boop() {

                    hello();
                    world()ˇ»
                }
            "},
            Mode::Visual,
        );

        // Around indent (include line above & below)
        cx.set_state(
            indoc! {"
                const ABOVE: str = true;
                fn boop() {
                    hellˇo();
                    world()

                }
                const BELOW: str = true;
            "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c a shift-i");
        cx.assert_state(
            indoc! {"
                const ABOVE: str = true;
                ˇ
                const BELOW: str = true;
            "},
            Mode::Insert,
        );
    }

    #[gpui::test]
    async fn test_delete_surrounding_character_objects(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for (start, end) in SURROUNDING_OBJECTS {
            let marked_string = SURROUNDING_MARKER_STRING
                .replace('`', &start.to_string())
                .replace('\'', &end.to_string());

            cx.simulate_at_each_offset(&format!("d i {start}"), &marked_string)
                .await
                .assert_matches();
            cx.simulate_at_each_offset(&format!("d i {end}"), &marked_string)
                .await
                .assert_matches();
            cx.simulate_at_each_offset(&format!("d a {start}"), &marked_string)
                .await
                .assert_matches();
            cx.simulate_at_each_offset(&format!("d a {end}"), &marked_string)
                .await
                .assert_matches();
        }
    }

    #[gpui::test]
    async fn test_anyquotes_object(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        const TEST_CASES: &[(&str, &str, &str, Mode)] = &[
            // Single quotes
            (
                "c i q",
                "Thisˇ is a 'quote' example.",
                "This is a 'ˇ' example.",
                Mode::Insert,
            ),
            (
                "c a q",
                "Thisˇ is a 'quote' example.",
                "This is a ˇexample.",
                Mode::Insert,
            ),
            (
                "c i q",
                "This is a \"simple 'qˇuote'\" example.",
                "This is a \"simple 'ˇ'\" example.",
                Mode::Insert,
            ),
            (
                "c a q",
                "This is a \"simple 'qˇuote'\" example.",
                "This is a \"simpleˇ\" example.",
                Mode::Insert,
            ),
            (
                "c i q",
                "This is a 'qˇuote' example.",
                "This is a 'ˇ' example.",
                Mode::Insert,
            ),
            (
                "c a q",
                "This is a 'qˇuote' example.",
                "This is a ˇexample.",
                Mode::Insert,
            ),
            (
                "d i q",
                "This is a 'qˇuote' example.",
                "This is a 'ˇ' example.",
                Mode::Normal,
            ),
            (
                "d a q",
                "This is a 'qˇuote' example.",
                "This is a ˇexample.",
                Mode::Normal,
            ),
            // Double quotes
            (
                "c i q",
                "This is a \"qˇuote\" example.",
                "This is a \"ˇ\" example.",
                Mode::Insert,
            ),
            (
                "c a q",
                "This is a \"qˇuote\" example.",
                "This is a ˇexample.",
                Mode::Insert,
            ),
            (
                "d i q",
                "This is a \"qˇuote\" example.",
                "This is a \"ˇ\" example.",
                Mode::Normal,
            ),
            (
                "d a q",
                "This is a \"qˇuote\" example.",
                "This is a ˇexample.",
                Mode::Normal,
            ),
            // Back quotes
            (
                "c i q",
                "This is a `qˇuote` example.",
                "This is a `ˇ` example.",
                Mode::Insert,
            ),
            (
                "c a q",
                "This is a `qˇuote` example.",
                "This is a ˇexample.",
                Mode::Insert,
            ),
            (
                "d i q",
                "This is a `qˇuote` example.",
                "This is a `ˇ` example.",
                Mode::Normal,
            ),
            (
                "d a q",
                "This is a `qˇuote` example.",
                "This is a ˇexample.",
                Mode::Normal,
            ),
        ];

        for (keystrokes, initial_state, expected_state, expected_mode) in TEST_CASES {
            cx.set_state(initial_state, Mode::Normal);

            cx.simulate_keystrokes(keystrokes);

            cx.assert_state(expected_state, *expected_mode);
        }

        const INVALID_CASES: &[(&str, &str, Mode)] = &[
            ("c i q", "this is a 'qˇuote example.", Mode::Normal), // Missing closing simple quote
            ("c a q", "this is a 'qˇuote example.", Mode::Normal), // Missing closing simple quote
            ("d i q", "this is a 'qˇuote example.", Mode::Normal), // Missing closing simple quote
            ("d a q", "this is a 'qˇuote example.", Mode::Normal), // Missing closing simple quote
            ("c i q", "this is a \"qˇuote example.", Mode::Normal), // Missing closing double quote
            ("c a q", "this is a \"qˇuote example.", Mode::Normal), // Missing closing double quote
            ("d i q", "this is a \"qˇuote example.", Mode::Normal), // Missing closing double quote
            ("d a q", "this is a \"qˇuote example.", Mode::Normal), // Missing closing back quote
            ("c i q", "this is a `qˇuote example.", Mode::Normal), // Missing closing back quote
            ("c a q", "this is a `qˇuote example.", Mode::Normal), // Missing closing back quote
            ("d i q", "this is a `qˇuote example.", Mode::Normal), // Missing closing back quote
            ("d a q", "this is a `qˇuote example.", Mode::Normal), // Missing closing back quote
        ];

        for (keystrokes, initial_state, mode) in INVALID_CASES {
            cx.set_state(initial_state, Mode::Normal);

            cx.simulate_keystrokes(keystrokes);

            cx.assert_state(initial_state, *mode);
        }
    }

    #[gpui::test]
    async fn test_anybrackets_object(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.update(|_, cx| {
            cx.bind_keys([KeyBinding::new(
                "b",
                AnyBrackets,
                Some("vim_operator == a || vim_operator == i || vim_operator == cs"),
            )]);
        });

        const TEST_CASES: &[(&str, &str, &str, Mode)] = &[
            // Bracket (Parentheses)
            (
                "c i b",
                "Thisˇ is a (simple [quote]) example.",
                "This is a (ˇ) example.",
                Mode::Insert,
            ),
            (
                "c i b",
                "This is a [simple (qˇuote)] example.",
                "This is a [simple (ˇ)] example.",
                Mode::Insert,
            ),
            (
                "c a b",
                "This is a [simple (qˇuote)] example.",
                "This is a [simple ˇ] example.",
                Mode::Insert,
            ),
            (
                "c a b",
                "Thisˇ is a (simple [quote]) example.",
                "This is a ˇ example.",
                Mode::Insert,
            ),
            (
                "c i b",
                "This is a (qˇuote) example.",
                "This is a (ˇ) example.",
                Mode::Insert,
            ),
            (
                "c a b",
                "This is a (qˇuote) example.",
                "This is a ˇ example.",
                Mode::Insert,
            ),
            (
                "d i b",
                "This is a (qˇuote) example.",
                "This is a (ˇ) example.",
                Mode::Normal,
            ),
            (
                "d a b",
                "This is a (qˇuote) example.",
                "This is a ˇ example.",
                Mode::Normal,
            ),
            // Square brackets
            (
                "c i b",
                "This is a [qˇuote] example.",
                "This is a [ˇ] example.",
                Mode::Insert,
            ),
            (
                "c a b",
                "This is a [qˇuote] example.",
                "This is a ˇ example.",
                Mode::Insert,
            ),
            (
                "d i b",
                "This is a [qˇuote] example.",
                "This is a [ˇ] example.",
                Mode::Normal,
            ),
            (
                "d a b",
                "This is a [qˇuote] example.",
                "This is a ˇ example.",
                Mode::Normal,
            ),
            // Curly brackets
            (
                "c i b",
                "This is a {qˇuote} example.",
                "This is a {ˇ} example.",
                Mode::Insert,
            ),
            (
                "c a b",
                "This is a {qˇuote} example.",
                "This is a ˇ example.",
                Mode::Insert,
            ),
            (
                "d i b",
                "This is a {qˇuote} example.",
                "This is a {ˇ} example.",
                Mode::Normal,
            ),
            (
                "d a b",
                "This is a {qˇuote} example.",
                "This is a ˇ example.",
                Mode::Normal,
            ),
        ];

        for (keystrokes, initial_state, expected_state, expected_mode) in TEST_CASES {
            cx.set_state(initial_state, Mode::Normal);

            cx.simulate_keystrokes(keystrokes);

            cx.assert_state(expected_state, *expected_mode);
        }

        const INVALID_CASES: &[(&str, &str, Mode)] = &[
            ("c i b", "this is a (qˇuote example.", Mode::Normal), // Missing closing bracket
            ("c a b", "this is a (qˇuote example.", Mode::Normal), // Missing closing bracket
            ("d i b", "this is a (qˇuote example.", Mode::Normal), // Missing closing bracket
            ("d a b", "this is a (qˇuote example.", Mode::Normal), // Missing closing bracket
            ("c i b", "this is a [qˇuote example.", Mode::Normal), // Missing closing square bracket
            ("c a b", "this is a [qˇuote example.", Mode::Normal), // Missing closing square bracket
            ("d i b", "this is a [qˇuote example.", Mode::Normal), // Missing closing square bracket
            ("d a b", "this is a [qˇuote example.", Mode::Normal), // Missing closing square bracket
            ("c i b", "this is a {qˇuote example.", Mode::Normal), // Missing closing curly bracket
            ("c a b", "this is a {qˇuote example.", Mode::Normal), // Missing closing curly bracket
            ("d i b", "this is a {qˇuote example.", Mode::Normal), // Missing closing curly bracket
            ("d a b", "this is a {qˇuote example.", Mode::Normal), // Missing closing curly bracket
        ];

        for (keystrokes, initial_state, mode) in INVALID_CASES {
            cx.set_state(initial_state, Mode::Normal);

            cx.simulate_keystrokes(keystrokes);

            cx.assert_state(initial_state, *mode);
        }
    }

    #[gpui::test]
    async fn test_anybrackets_trailing_space(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("(trailingˇ whitespace          )")
            .await;
        cx.simulate_shared_keystrokes("v i b").await;
        cx.shared_state().await.assert_matches();
        cx.simulate_shared_keystrokes("escape y i b").await;
        cx.shared_clipboard()
            .await
            .assert_eq("trailing whitespace          ");
    }

    #[gpui::test]
    async fn test_tags(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new_html(cx).await;

        cx.set_state("<html><head></head><body><b>hˇi!</b></body>", Mode::Normal);
        cx.simulate_keystrokes("v i t");
        cx.assert_state(
            "<html><head></head><body><b>«hi!ˇ»</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes("a t");
        cx.assert_state(
            "<html><head></head><body>«<b>hi!</b>ˇ»</body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes("a t");
        cx.assert_state(
            "<html><head></head>«<body><b>hi!</b></body>ˇ»",
            Mode::Visual,
        );

        // The cursor is before the tag
        cx.set_state(
            "<html><head></head><body> ˇ  <b>hi!</b></body>",
            Mode::Normal,
        );
        cx.simulate_keystrokes("v i t");
        cx.assert_state(
            "<html><head></head><body>   <b>«hi!ˇ»</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes("a t");
        cx.assert_state(
            "<html><head></head><body>   «<b>hi!</b>ˇ»</body>",
            Mode::Visual,
        );

        // The cursor is in the open tag
        cx.set_state(
            "<html><head></head><body><bˇ>hi!</b><b>hello!</b></body>",
            Mode::Normal,
        );
        cx.simulate_keystrokes("v a t");
        cx.assert_state(
            "<html><head></head><body>«<b>hi!</b>ˇ»<b>hello!</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes("i t");
        cx.assert_state(
            "<html><head></head><body>«<b>hi!</b><b>hello!</b>ˇ»</body>",
            Mode::Visual,
        );

        // current selection length greater than 1
        cx.set_state(
            "<html><head></head><body><«b>hi!ˇ»</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes("i t");
        cx.assert_state(
            "<html><head></head><body><b>«hi!ˇ»</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes("a t");
        cx.assert_state(
            "<html><head></head><body>«<b>hi!</b>ˇ»</body>",
            Mode::Visual,
        );

        cx.set_state(
            "<html><head></head><body><«b>hi!</ˇ»b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes("a t");
        cx.assert_state(
            "<html><head></head>«<body><b>hi!</b></body>ˇ»",
            Mode::Visual,
        );
    }
}
