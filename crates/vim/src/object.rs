use std::ops::Range;

use crate::{
    motion::right, normal::normal_object, state::Mode, utils::coerce_punctuation,
    visual::visual_object, Vim,
};
use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement::{self, FindRange},
    Bias, DisplayPoint,
};
use gpui::{actions, impl_actions, ViewContext, WindowContext};
use language::{char_kind, BufferSnapshot, CharKind, Point, Selection};
use serde::Deserialize;
use workspace::Workspace;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Word { ignore_punctuation: bool },
    Sentence,
    Paragraph,
    Quotes,
    BackQuotes,
    DoubleQuotes,
    VerticalBars,
    Parentheses,
    SquareBrackets,
    CurlyBrackets,
    AngleBrackets,
    Argument,
    Tag,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Word {
    #[serde(default)]
    ignore_punctuation: bool,
}

impl_actions!(vim, [Word]);

actions!(
    vim,
    [
        Sentence,
        Paragraph,
        Quotes,
        BackQuotes,
        DoubleQuotes,
        VerticalBars,
        Parentheses,
        SquareBrackets,
        CurlyBrackets,
        AngleBrackets,
        Argument,
        Tag
    ]
);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(
        |_: &mut Workspace, &Word { ignore_punctuation }: &Word, cx: _| {
            object(Object::Word { ignore_punctuation }, cx)
        },
    );
    workspace.register_action(|_: &mut Workspace, _: &Tag, cx: _| object(Object::Tag, cx));
    workspace
        .register_action(|_: &mut Workspace, _: &Sentence, cx: _| object(Object::Sentence, cx));
    workspace
        .register_action(|_: &mut Workspace, _: &Paragraph, cx: _| object(Object::Paragraph, cx));
    workspace.register_action(|_: &mut Workspace, _: &Quotes, cx: _| object(Object::Quotes, cx));
    workspace
        .register_action(|_: &mut Workspace, _: &BackQuotes, cx: _| object(Object::BackQuotes, cx));
    workspace.register_action(|_: &mut Workspace, _: &DoubleQuotes, cx: _| {
        object(Object::DoubleQuotes, cx)
    });
    workspace.register_action(|_: &mut Workspace, _: &Parentheses, cx: _| {
        object(Object::Parentheses, cx)
    });
    workspace.register_action(|_: &mut Workspace, _: &SquareBrackets, cx: _| {
        object(Object::SquareBrackets, cx)
    });
    workspace.register_action(|_: &mut Workspace, _: &CurlyBrackets, cx: _| {
        object(Object::CurlyBrackets, cx)
    });
    workspace.register_action(|_: &mut Workspace, _: &AngleBrackets, cx: _| {
        object(Object::AngleBrackets, cx)
    });
    workspace.register_action(|_: &mut Workspace, _: &VerticalBars, cx: _| {
        object(Object::VerticalBars, cx)
    });
    workspace
        .register_action(|_: &mut Workspace, _: &Argument, cx: _| object(Object::Argument, cx));
}

fn object(object: Object, cx: &mut WindowContext) {
    match Vim::read(cx).state().mode {
        Mode::Normal => normal_object(object, cx),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => visual_object(object, cx),
        Mode::Insert | Mode::Replace => {
            // Shouldn't execute a text object in insert mode. Ignoring
        }
    }
}

impl Object {
    pub fn is_multiline(self) -> bool {
        match self {
            Object::Word { .. }
            | Object::Quotes
            | Object::BackQuotes
            | Object::VerticalBars
            | Object::DoubleQuotes => false,
            Object::Sentence
            | Object::Paragraph
            | Object::Parentheses
            | Object::Tag
            | Object::AngleBrackets
            | Object::CurlyBrackets
            | Object::SquareBrackets
            | Object::Argument => true,
        }
    }

    pub fn always_expands_both_ways(self) -> bool {
        match self {
            Object::Word { .. } | Object::Sentence | Object::Paragraph | Object::Argument => false,
            Object::Quotes
            | Object::BackQuotes
            | Object::DoubleQuotes
            | Object::VerticalBars
            | Object::Parentheses
            | Object::SquareBrackets
            | Object::Tag
            | Object::CurlyBrackets
            | Object::AngleBrackets => true,
        }
    }

    pub fn target_visual_mode(self, current_mode: Mode) -> Mode {
        match self {
            Object::Word { .. }
            | Object::Sentence
            | Object::Quotes
            | Object::BackQuotes
            | Object::DoubleQuotes => {
                if current_mode == Mode::VisualBlock {
                    Mode::VisualBlock
                } else {
                    Mode::Visual
                }
            }
            Object::Parentheses
            | Object::SquareBrackets
            | Object::CurlyBrackets
            | Object::AngleBrackets
            | Object::VerticalBars
            | Object::Tag
            | Object::Argument => Mode::Visual,
            Object::Paragraph => Mode::VisualLine,
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
            Object::Sentence => sentence(map, relative_to, around),
            Object::Paragraph => paragraph(map, relative_to, around),
            Object::Quotes => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '\'', '\'')
            }
            Object::BackQuotes => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '`', '`')
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
            Object::Tag => surrounding_html_tag(map, selection, around),
            Object::SquareBrackets => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '[', ']')
            }
            Object::CurlyBrackets => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '{', '}')
            }
            Object::AngleBrackets => {
                surrounding_markers(map, relative_to, around, self.is_multiline(), '<', '>')
            }
            Object::Argument => argument(map, relative_to, around),
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
    let scope = map
        .buffer_snapshot
        .language_scope_at(relative_to.to_point(map));
    let start = movement::find_preceding_boundary_display_point(
        map,
        right(map, relative_to, 1),
        movement::FindRange::SingleLine,
        |left, right| {
            coerce_punctuation(char_kind(&scope, left), ignore_punctuation)
                != coerce_punctuation(char_kind(&scope, right), ignore_punctuation)
        },
    );

    let end = movement::find_boundary(map, relative_to, FindRange::SingleLine, |left, right| {
        coerce_punctuation(char_kind(&scope, left), ignore_punctuation)
            != coerce_punctuation(char_kind(&scope, right), ignore_punctuation)
    });

    Some(start..end)
}

fn surrounding_html_tag(
    map: &DisplaySnapshot,
    selection: Selection<DisplayPoint>,
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
    let offset = selection.head().to_offset(map, Bias::Left);
    let excerpt = snapshot.excerpt_containing(offset..offset)?;
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
                let is_valid = if selection.end.to_offset(map, Bias::Left)
                    - selection.start.to_offset(map, Bias::Left)
                    <= 1
                {
                    offset <= last_child.end_byte()
                } else {
                    selection.start.to_offset(map, Bias::Left) >= first_child.start_byte()
                        && selection.end.to_offset(map, Bias::Left) <= last_child.start_byte() + 1
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
    let scope = map
        .buffer_snapshot
        .language_scope_at(relative_to.to_point(map));
    let in_word = map
        .chars_at(relative_to)
        .next()
        .map(|(c, _)| char_kind(&scope, c) != CharKind::Whitespace)
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
    let scope = map
        .buffer_snapshot
        .language_scope_at(relative_to.to_point(map));
    // Get the start of the word
    let start = movement::find_preceding_boundary_display_point(
        map,
        right(map, relative_to, 1),
        FindRange::SingleLine,
        |left, right| {
            coerce_punctuation(char_kind(&scope, left), ignore_punctuation)
                != coerce_punctuation(char_kind(&scope, right), ignore_punctuation)
        },
    );

    let mut word_found = false;
    let end = movement::find_boundary(map, relative_to, FindRange::MultiLine, |left, right| {
        let left_kind = coerce_punctuation(char_kind(&scope, left), ignore_punctuation);
        let right_kind = coerce_punctuation(char_kind(&scope, right), ignore_punctuation);

        let found = (word_found && left_kind != right_kind) || right == '\n' && left == '\n';

        if right_kind != CharKind::Whitespace {
            word_found = true;
        }

        found
    });

    Some(start..end)
}

fn argument(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    around: bool,
) -> Option<Range<DisplayPoint>> {
    let snapshot = &map.buffer_snapshot;
    let offset = relative_to.to_offset(map, Bias::Left);

    // The `argument` vim text object uses the syntax tree, so we operate at the buffer level and map back to the display level
    let excerpt = snapshot.excerpt_containing(offset..offset)?;
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
            return matches!(
                buffer.chars_at(open.start).next(),
                Some('(' | '[' | '{' | '<' | '|')
            );
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
            if cursor.goto_first_child_for_byte(offset).is_none() {
                return None;
            }
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
    let current_line_is_empty = map.buffer_snapshot.is_line_blank(point.row);

    if around {
        if paragraph_ends_with_eof {
            if current_line_is_empty {
                return None;
            }

            let paragraph_start_row = paragraph_start.row();
            if paragraph_start_row != 0 {
                let previous_paragraph_last_line_start =
                    Point::new(paragraph_start_row - 1, 0).to_display_point(map);
                paragraph_start = start_of_paragraph(map, previous_paragraph_last_line_start);
            }
        } else {
            let next_paragraph_start = Point::new(paragraph_end_row + 1, 0).to_display_point(map);
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

    let is_current_line_blank = map.buffer_snapshot.is_line_blank(point.row);

    for row in (0..point.row).rev() {
        let blank = map.buffer_snapshot.is_line_blank(row);
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
    if point.row == map.max_buffer_row() {
        return map.max_point();
    }

    let is_current_line_blank = map.buffer_snapshot.is_line_blank(point.row);

    for row in point.row + 1..map.max_buffer_row() + 1 {
        let blank = map.buffer_snapshot.is_line_blank(row);
        if blank != is_current_line_blank {
            let previous_row = row - 1;
            return Point::new(previous_row, map.buffer_snapshot.line_len(previous_row))
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

    if let Some((ch, range)) = movement::chars_after(map, point).next() {
        if ch == open_marker {
            if open_marker == close_marker {
                let mut total = 0;
                for (ch, _) in movement::chars_before(map, point) {
                    if ch == '\n' {
                        break;
                    }
                    if ch == open_marker {
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
        for (ch, range) in movement::chars_before(map, point) {
            if ch == '\n' && !search_across_lines {
                break;
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
            if ch == open_marker {
                opening = Some(range);
                break;
            } else if ch == close_marker {
                break;
            }
        }
    }

    let Some(mut opening) = opening else {
        return None;
    };

    let mut matched_opens = 0;
    let mut closing = None;

    for (ch, range) in movement::chars_after(map, opening.end) {
        if ch == '\n' && !search_across_lines {
            break;
        }

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

    let Some(mut closing) = closing else {
        return None;
    };

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

        for (ch, range) in movement::chars_before(map, closing.start) {
            if !ch.is_whitespace() {
                break;
            }
            if ch != '\n' {
                closing.start = range.start
            }
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
    use indoc::indoc;

    use crate::{
        state::Mode,
        test::{ExemptionFeatures, NeovimBackedTestContext, VimTestContext},
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

        /*
                cx.set_shared_state("The quick ˇbrown\nfox").await;
                cx.simulate_shared_keystrokes(["v"]).await;
                cx.assert_shared_state("The quick «bˇ»rown\nfox").await;
                cx.simulate_shared_keystrokes(["i", "w"]).await;
                cx.assert_shared_state("The quick «brownˇ»\nfox").await;
        */
        cx.set_shared_state("The quick brown\nˇ\nfox").await;
        cx.simulate_shared_keystrokes(["v"]).await;
        cx.assert_shared_state("The quick brown\n«\nˇ»fox").await;
        cx.simulate_shared_keystrokes(["i", "w"]).await;
        cx.assert_shared_state("The quick brown\n«\nˇ»fox").await;

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

    const PARAGRAPH_EXAMPLES: &[&'static str] = &[
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
            cx.assert_binding_matches_all(["c", "i", "p"], paragraph_example)
                .await;
            cx.assert_binding_matches_all(["c", "a", "p"], paragraph_example)
                .await;
        }
    }

    #[gpui::test]
    async fn test_delete_paragraph_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for paragraph_example in PARAGRAPH_EXAMPLES {
            cx.assert_binding_matches_all(["d", "i", "p"], paragraph_example)
                .await;
            cx.assert_binding_matches_all(["d", "a", "p"], paragraph_example)
                .await;
        }
    }

    #[gpui::test]
    async fn test_paragraph_object_with_landing_positions_not_at_beginning_of_line(
        cx: &mut gpui::TestAppContext,
    ) {
        // Landing position not at the beginning of the line
        const PARAGRAPH_LANDING_POSITION_EXAMPLE: &'static str = indoc! {"
            The quick brown fox jumpsˇ
            over the lazy dog.ˇ
            ˇ ˇ\tˇ
            ˇ ˇ
            ˇ\tˇ ˇ\tˇ
            ˇThe quick brown fox jumpsˇ
            ˇover the lazy dog.ˇ
            ˇ ˇ\tˇ
            ˇ
            ˇ ˇ\tˇ
            ˇ\tˇ ˇ\tˇ
        "};

        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.assert_binding_matches_all_exempted(
            ["c", "i", "p"],
            PARAGRAPH_LANDING_POSITION_EXAMPLE,
            ExemptionFeatures::IncorrectLandingPosition,
        )
        .await;
        cx.assert_binding_matches_all_exempted(
            ["c", "a", "p"],
            PARAGRAPH_LANDING_POSITION_EXAMPLE,
            ExemptionFeatures::IncorrectLandingPosition,
        )
        .await;
        cx.assert_binding_matches_all_exempted(
            ["d", "i", "p"],
            PARAGRAPH_LANDING_POSITION_EXAMPLE,
            ExemptionFeatures::IncorrectLandingPosition,
        )
        .await;
        cx.assert_binding_matches_all_exempted(
            ["d", "a", "p"],
            PARAGRAPH_LANDING_POSITION_EXAMPLE,
            ExemptionFeatures::IncorrectLandingPosition,
        )
        .await;
    }

    #[gpui::test]
    async fn test_visual_paragraph_object(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        const EXAMPLES: &[&'static str] = &[
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
            cx.assert_binding_matches_all(["v", "i", "p"], paragraph_example)
                .await;
            cx.assert_binding_matches_all(["v", "a", "p"], paragraph_example)
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
    async fn test_singleline_surrounding_character_objects(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_wrap(12).await;

        cx.set_shared_state(indoc! {
            "helˇlo \"world\"!"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "i", "\""]).await;
        cx.assert_shared_state(indoc! {
            "hello \"«worldˇ»\"!"
        })
        .await;

        cx.set_shared_state(indoc! {
            "hello \"wˇorld\"!"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "i", "\""]).await;
        cx.assert_shared_state(indoc! {
            "hello \"«worldˇ»\"!"
        })
        .await;

        cx.set_shared_state(indoc! {
            "hello \"wˇorld\"!"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "a", "\""]).await;
        cx.assert_shared_state(indoc! {
            "hello« \"world\"ˇ»!"
        })
        .await;

        cx.set_shared_state(indoc! {
            "hello \"wˇorld\" !"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "a", "\""]).await;
        cx.assert_shared_state(indoc! {
            "hello «\"world\" ˇ»!"
        })
        .await;

        cx.set_shared_state(indoc! {
            "hello \"wˇorld\"•
            goodbye"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "a", "\""]).await;
        cx.assert_shared_state(indoc! {
            "hello «\"world\" ˇ»
            goodbye"
        })
        .await;
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

        cx.set_shared_state(indoc! {
            "func empty(a string) bool {
                 if a == \"\" ˇ{
                     return true
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
        cx.simulate_keystrokes(["c", "i", "|"]);
        cx.assert_state(
            indoc! {"
            fn boop() {
                baz(|ˇ| { bar(|j, k| { })})
            }"
            },
            Mode::Insert,
        );
        cx.simulate_keystrokes(["escape", "1", "8", "|"]);
        cx.assert_state(
            indoc! {"
            fn boop() {
                baz(|| { bar(ˇ|j, k| { })})
            }"
            },
            Mode::Normal,
        );

        cx.simulate_keystrokes(["v", "a", "|"]);
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
        cx.simulate_keystrokes(["v", "i", "a"]);
        cx.assert_state("fn boop<«A: Debugˇ», B>() {}", Mode::Visual);

        // Function arguments
        cx.set_state(
            "fn boop(ˇarg_a: (Tuple, Of, Types), arg_b: String) {}",
            Mode::Normal,
        );
        cx.simulate_keystrokes(["d", "a", "a"]);
        cx.assert_state("fn boop(ˇarg_b: String) {}", Mode::Normal);

        cx.set_state("std::namespace::test(\"strinˇg\", a.b.c())", Mode::Normal);
        cx.simulate_keystrokes(["v", "a", "a"]);
        cx.assert_state("std::namespace::test(«\"string\", ˇ»a.b.c())", Mode::Visual);

        // Tuple, vec, and array arguments
        cx.set_state(
            "fn boop(arg_a: (Tuple, Ofˇ, Types), arg_b: String) {}",
            Mode::Normal,
        );
        cx.simulate_keystrokes(["c", "i", "a"]);
        cx.assert_state(
            "fn boop(arg_a: (Tuple, ˇ, Types), arg_b: String) {}",
            Mode::Insert,
        );

        cx.set_state("let a = (test::call(), 'p', my_macro!{ˇ});", Mode::Normal);
        cx.simulate_keystrokes(["c", "a", "a"]);
        cx.assert_state("let a = (test::call(), 'p'ˇ);", Mode::Insert);

        cx.set_state("let a = [test::call(ˇ), 300];", Mode::Normal);
        cx.simulate_keystrokes(["c", "i", "a"]);
        cx.assert_state("let a = [ˇ, 300];", Mode::Insert);

        cx.set_state(
            "let a = vec![Vec::new(), vecˇ![test::call(), 300]];",
            Mode::Normal,
        );
        cx.simulate_keystrokes(["c", "a", "a"]);
        cx.assert_state("let a = vec![Vec::new()ˇ];", Mode::Insert);

        // Cursor immediately before / after brackets
        cx.set_state("let a = [test::call(first_arg)ˇ]", Mode::Normal);
        cx.simulate_keystrokes(["v", "i", "a"]);
        cx.assert_state("let a = [«test::call(first_arg)ˇ»]", Mode::Visual);

        cx.set_state("let a = [test::callˇ(first_arg)]", Mode::Normal);
        cx.simulate_keystrokes(["v", "i", "a"]);
        cx.assert_state("let a = [«test::call(first_arg)ˇ»]", Mode::Visual);
    }

    #[gpui::test]
    async fn test_delete_surrounding_character_objects(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        for (start, end) in SURROUNDING_OBJECTS {
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

    #[gpui::test]
    async fn test_tags(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new_html(cx).await;

        cx.set_state("<html><head></head><body><b>hˇi!</b></body>", Mode::Normal);
        cx.simulate_keystrokes(["v", "i", "t"]);
        cx.assert_state(
            "<html><head></head><body><b>«hi!ˇ»</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes(["a", "t"]);
        cx.assert_state(
            "<html><head></head><body>«<b>hi!</b>ˇ»</body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes(["a", "t"]);
        cx.assert_state(
            "<html><head></head>«<body><b>hi!</b></body>ˇ»",
            Mode::Visual,
        );

        // The cursor is before the tag
        cx.set_state(
            "<html><head></head><body> ˇ  <b>hi!</b></body>",
            Mode::Normal,
        );
        cx.simulate_keystrokes(["v", "i", "t"]);
        cx.assert_state(
            "<html><head></head><body>   <b>«hi!ˇ»</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes(["a", "t"]);
        cx.assert_state(
            "<html><head></head><body>   «<b>hi!</b>ˇ»</body>",
            Mode::Visual,
        );

        // The cursor is in the open tag
        cx.set_state(
            "<html><head></head><body><bˇ>hi!</b><b>hello!</b></body>",
            Mode::Normal,
        );
        cx.simulate_keystrokes(["v", "a", "t"]);
        cx.assert_state(
            "<html><head></head><body>«<b>hi!</b>ˇ»<b>hello!</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes(["i", "t"]);
        cx.assert_state(
            "<html><head></head><body>«<b>hi!</b><b>hello!</b>ˇ»</body>",
            Mode::Visual,
        );

        // current selection length greater than 1
        cx.set_state(
            "<html><head></head><body><«b>hi!ˇ»</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes(["i", "t"]);
        cx.assert_state(
            "<html><head></head><body><b>«hi!ˇ»</b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes(["a", "t"]);
        cx.assert_state(
            "<html><head></head><body>«<b>hi!</b>ˇ»</body>",
            Mode::Visual,
        );

        cx.set_state(
            "<html><head></head><body><«b>hi!</ˇ»b></body>",
            Mode::Visual,
        );
        cx.simulate_keystrokes(["a", "t"]);
        cx.assert_state(
            "<html><head></head>«<body><b>hi!</b></body>ˇ»",
            Mode::Visual,
        );
    }
}
