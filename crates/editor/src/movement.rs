//! Movement module contains helper functions for calculating intended position
//! in editor given a given motion (e.g. it handles converting a "move left" command into coordinates in editor). It is exposed mostly for use by vim crate.

use super::{Bias, DisplayPoint, DisplaySnapshot, SelectionGoal, ToDisplayPoint};
use crate::{DisplayRow, EditorStyle, ToOffset, ToPoint, scroll::ScrollAnchor};
use gpui::{Pixels, WindowTextSystem};
use language::Point;
use multi_buffer::{MultiBufferRow, MultiBufferSnapshot};
use serde::Deserialize;
use workspace::searchable::Direction;

use std::{ops::Range, sync::Arc};

/// Defines search strategy for items in `movement` module.
/// `FindRange::SingeLine` only looks for a match on a single line at a time, whereas
/// `FindRange::MultiLine` keeps going until the end of a string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum FindRange {
    SingleLine,
    MultiLine,
}

/// TextLayoutDetails encompasses everything we need to move vertically
/// taking into account variable width characters.
pub struct TextLayoutDetails {
    pub(crate) text_system: Arc<WindowTextSystem>,
    pub(crate) editor_style: EditorStyle,
    pub(crate) rem_size: Pixels,
    pub scroll_anchor: ScrollAnchor,
    pub visible_rows: Option<f32>,
    pub vertical_scroll_margin: f32,
}

/// Returns a column to the left of the current point, wrapping
/// to the previous line if that point is at the start of line.
pub fn left(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    if point.column() > 0 {
        *point.column_mut() -= 1;
    } else if point.row().0 > 0 {
        *point.row_mut() -= 1;
        *point.column_mut() = map.line_len(point.row());
    }
    map.clip_point(point, Bias::Left)
}

/// Returns a column to the left of the current point, doing nothing if
/// that point is already at the start of line.
pub fn saturating_left(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    if point.column() > 0 {
        *point.column_mut() -= 1;
    } else if point.column() == 0 {
        // If the current sofr_wrap mode is used, the column corresponding to the display is 0,
        //  which does not necessarily mean that the actual beginning of a paragraph
        if map.display_point_to_fold_point(point, Bias::Left).column() > 0 {
            return left(map, point);
        }
    }
    map.clip_point(point, Bias::Left)
}

/// Returns a column to the right of the current point, wrapping
/// to the next line if that point is at the end of line.
pub fn right(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    if point.column() < map.line_len(point.row()) {
        *point.column_mut() += 1;
    } else if point.row() < map.max_point().row() {
        *point.row_mut() += 1;
        *point.column_mut() = 0;
    }
    map.clip_point(point, Bias::Right)
}

/// Returns a column to the right of the current point, not performing any wrapping
/// if that point is already at the end of line.
pub fn saturating_right(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    *point.column_mut() += 1;
    map.clip_point(point, Bias::Right)
}

/// Returns a display point for the preceding displayed line (which might be a soft-wrapped line).
pub fn up(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    goal: SelectionGoal,
    preserve_column_at_start: bool,
    text_layout_details: &TextLayoutDetails,
) -> (DisplayPoint, SelectionGoal) {
    up_by_rows(
        map,
        start,
        1,
        goal,
        preserve_column_at_start,
        text_layout_details,
    )
}

/// Returns a display point for the next displayed line (which might be a soft-wrapped line).
pub fn down(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    goal: SelectionGoal,
    preserve_column_at_end: bool,
    text_layout_details: &TextLayoutDetails,
) -> (DisplayPoint, SelectionGoal) {
    down_by_rows(
        map,
        start,
        1,
        goal,
        preserve_column_at_end,
        text_layout_details,
    )
}

pub(crate) fn up_by_rows(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    row_count: u32,
    goal: SelectionGoal,
    preserve_column_at_start: bool,
    text_layout_details: &TextLayoutDetails,
) -> (DisplayPoint, SelectionGoal) {
    let goal_x = match goal {
        SelectionGoal::HorizontalPosition(x) => x.into(),
        SelectionGoal::WrappedHorizontalPosition((_, x)) => x.into(),
        SelectionGoal::HorizontalRange { end, .. } => end.into(),
        _ => map.x_for_display_point(start, text_layout_details),
    };

    let prev_row = DisplayRow(start.row().0.saturating_sub(row_count));
    let mut point = map.clip_point(
        DisplayPoint::new(prev_row, map.line_len(prev_row)),
        Bias::Left,
    );
    if point.row() < start.row() {
        *point.column_mut() = map.display_column_for_x(point.row(), goal_x, text_layout_details)
    } else if preserve_column_at_start {
        return (start, goal);
    } else {
        point = DisplayPoint::new(DisplayRow(0), 0);
    }

    let mut clipped_point = map.clip_point(point, Bias::Left);
    if clipped_point.row() < point.row() {
        clipped_point = map.clip_point(point, Bias::Right);
    }
    (
        clipped_point,
        SelectionGoal::HorizontalPosition(goal_x.into()),
    )
}

pub(crate) fn down_by_rows(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    row_count: u32,
    goal: SelectionGoal,
    preserve_column_at_end: bool,
    text_layout_details: &TextLayoutDetails,
) -> (DisplayPoint, SelectionGoal) {
    let goal_x = match goal {
        SelectionGoal::HorizontalPosition(x) => x.into(),
        SelectionGoal::WrappedHorizontalPosition((_, x)) => x.into(),
        SelectionGoal::HorizontalRange { end, .. } => end.into(),
        _ => map.x_for_display_point(start, text_layout_details),
    };

    let new_row = DisplayRow(start.row().0 + row_count);
    let mut point = map.clip_point(DisplayPoint::new(new_row, 0), Bias::Right);
    if point.row() > start.row() {
        *point.column_mut() = map.display_column_for_x(point.row(), goal_x, text_layout_details)
    } else if preserve_column_at_end {
        return (start, goal);
    } else {
        point = map.max_point();
    }

    let mut clipped_point = map.clip_point(point, Bias::Right);
    if clipped_point.row() > point.row() {
        clipped_point = map.clip_point(point, Bias::Left);
    }
    (
        clipped_point,
        SelectionGoal::HorizontalPosition(goal_x.into()),
    )
}

/// Returns a position of the start of line.
/// If `stop_at_soft_boundaries` is true, the returned position is that of the
/// displayed line (e.g. it could actually be in the middle of a text line if that line is soft-wrapped).
/// Otherwise it's always going to be the start of a logical line.
pub fn line_beginning(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    stop_at_soft_boundaries: bool,
) -> DisplayPoint {
    let point = display_point.to_point(map);
    let soft_line_start = map.clip_point(DisplayPoint::new(display_point.row(), 0), Bias::Right);
    let line_start = map.prev_line_boundary(point).1;

    if stop_at_soft_boundaries && display_point != soft_line_start {
        soft_line_start
    } else {
        line_start
    }
}

/// Returns the last indented position on a given line.
/// If `stop_at_soft_boundaries` is true, the returned [`DisplayPoint`] is that of a
/// displayed line (e.g. if there's soft wrap it's gonna be returned),
/// otherwise it's always going to be a start of a logical line.
pub fn indented_line_beginning(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    stop_at_soft_boundaries: bool,
    stop_at_indent: bool,
) -> DisplayPoint {
    let point = display_point.to_point(map);
    let soft_line_start = map.clip_point(DisplayPoint::new(display_point.row(), 0), Bias::Right);
    let indent_start = Point::new(
        point.row,
        map.buffer_snapshot
            .indent_size_for_line(MultiBufferRow(point.row))
            .len,
    )
    .to_display_point(map);
    let line_start = map.prev_line_boundary(point).1;

    if stop_at_soft_boundaries && soft_line_start > indent_start && display_point != soft_line_start
    {
        soft_line_start
    } else if stop_at_indent && display_point != indent_start {
        indent_start
    } else {
        line_start
    }
}

/// Returns a position of the end of line.
///
/// If `stop_at_soft_boundaries` is true, the returned position is that of the
/// displayed line (e.g. it could actually be in the middle of a text line if that line is soft-wrapped).
/// Otherwise it's always going to be the end of a logical line.
pub fn line_end(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    stop_at_soft_boundaries: bool,
) -> DisplayPoint {
    let soft_line_end = map.clip_point(
        DisplayPoint::new(display_point.row(), map.line_len(display_point.row())),
        Bias::Left,
    );
    if stop_at_soft_boundaries && display_point != soft_line_end {
        soft_line_end
    } else {
        map.next_line_boundary(display_point.to_point(map)).1
    }
}

/// Returns a position of the previous word boundary, where a word character is defined as either
/// uppercase letter, lowercase letter, '_' character or language-specific word character (like '-' in CSS).
pub fn previous_word_start(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let classifier = map.buffer_snapshot.char_classifier_at(raw_point);

    let mut is_first_iteration = true;
    find_preceding_boundary_display_point(map, point, FindRange::MultiLine, |left, right| {
        // Make alt-left skip punctuation to respect VSCode behaviour. For example: hello.| goes to |hello.
        if is_first_iteration
            && classifier.is_punctuation(right)
            && !classifier.is_punctuation(left)
            && left != '\n'
        {
            is_first_iteration = false;
            return false;
        }
        is_first_iteration = false;

        (classifier.kind(left) != classifier.kind(right) && !classifier.is_whitespace(right))
            || left == '\n'
    })
}

/// Returns a position of the previous word boundary, where a word character is defined as either
/// uppercase letter, lowercase letter, '_' character, language-specific word character (like '-' in CSS) or newline.
pub fn previous_word_start_or_newline(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let classifier = map.buffer_snapshot.char_classifier_at(raw_point);

    find_preceding_boundary_display_point(map, point, FindRange::MultiLine, |left, right| {
        (classifier.kind(left) != classifier.kind(right) && !right.is_whitespace())
            || left == '\n'
            || right == '\n'
    })
}

/// Returns a position of the previous subword boundary, where a subword is defined as a run of
/// word characters of the same "subkind" - where subcharacter kinds are '_' character,
/// lowerspace characters and uppercase characters.
pub fn previous_subword_start(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let classifier = map.buffer_snapshot.char_classifier_at(raw_point);

    find_preceding_boundary_display_point(map, point, FindRange::MultiLine, |left, right| {
        let is_word_start =
            classifier.kind(left) != classifier.kind(right) && !right.is_whitespace();
        let is_subword_start = classifier.is_word('-') && left == '-' && right != '-'
            || left == '_' && right != '_'
            || left.is_lowercase() && right.is_uppercase();
        is_word_start || is_subword_start || left == '\n'
    })
}

/// Returns a position of the next word boundary, where a word character is defined as either
/// uppercase letter, lowercase letter, '_' character or language-specific word character (like '-' in CSS).
pub fn next_word_end(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let classifier = map.buffer_snapshot.char_classifier_at(raw_point);
    let mut is_first_iteration = true;
    find_boundary(map, point, FindRange::MultiLine, |left, right| {
        // Make alt-right skip punctuation to respect VSCode behaviour. For example: |.hello goes to .hello|
        if is_first_iteration
            && classifier.is_punctuation(left)
            && !classifier.is_punctuation(right)
            && right != '\n'
        {
            is_first_iteration = false;
            return false;
        }
        is_first_iteration = false;

        (classifier.kind(left) != classifier.kind(right) && !classifier.is_whitespace(left))
            || right == '\n'
    })
}

/// Returns a position of the next word boundary, where a word character is defined as either
/// uppercase letter, lowercase letter, '_' character, language-specific word character (like '-' in CSS) or newline.
pub fn next_word_end_or_newline(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let classifier = map.buffer_snapshot.char_classifier_at(raw_point);

    let mut on_starting_row = true;
    find_boundary(map, point, FindRange::MultiLine, |left, right| {
        if left == '\n' {
            on_starting_row = false;
        }
        (classifier.kind(left) != classifier.kind(right)
            && ((on_starting_row && !left.is_whitespace())
                || (!on_starting_row && !right.is_whitespace())))
            || right == '\n'
    })
}

/// Returns a position of the next subword boundary, where a subword is defined as a run of
/// word characters of the same "subkind" - where subcharacter kinds are '_' character,
/// lowerspace characters and uppercase characters.
pub fn next_subword_end(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let classifier = map.buffer_snapshot.char_classifier_at(raw_point);

    find_boundary(map, point, FindRange::MultiLine, |left, right| {
        let is_word_end =
            (classifier.kind(left) != classifier.kind(right)) && !classifier.is_whitespace(left);
        let is_subword_end = classifier.is_word('-') && left != '-' && right == '-'
            || left != '_' && right == '_'
            || left.is_lowercase() && right.is_uppercase();
        is_word_end || is_subword_end || right == '\n'
    })
}

/// Returns a position of the start of the current paragraph, where a paragraph
/// is defined as a run of non-blank lines.
pub fn start_of_paragraph(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    mut count: usize,
) -> DisplayPoint {
    let point = display_point.to_point(map);
    if point.row == 0 {
        return DisplayPoint::zero();
    }

    let mut found_non_blank_line = false;
    for row in (0..point.row + 1).rev() {
        let blank = map.buffer_snapshot.is_line_blank(MultiBufferRow(row));
        if found_non_blank_line && blank {
            if count <= 1 {
                return Point::new(row, 0).to_display_point(map);
            }
            count -= 1;
            found_non_blank_line = false;
        }

        found_non_blank_line |= !blank;
    }

    DisplayPoint::zero()
}

/// Returns a position of the end of the current paragraph, where a paragraph
/// is defined as a run of non-blank lines.
pub fn end_of_paragraph(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    mut count: usize,
) -> DisplayPoint {
    let point = display_point.to_point(map);
    if point.row == map.buffer_snapshot.max_row().0 {
        return map.max_point();
    }

    let mut found_non_blank_line = false;
    for row in point.row..=map.buffer_snapshot.max_row().0 {
        let blank = map.buffer_snapshot.is_line_blank(MultiBufferRow(row));
        if found_non_blank_line && blank {
            if count <= 1 {
                return Point::new(row, 0).to_display_point(map);
            }
            count -= 1;
            found_non_blank_line = false;
        }

        found_non_blank_line |= !blank;
    }

    map.max_point()
}

pub fn start_of_excerpt(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    direction: Direction,
) -> DisplayPoint {
    let point = map.display_point_to_point(display_point, Bias::Left);
    let Some(excerpt) = map.buffer_snapshot.excerpt_containing(point..point) else {
        return display_point;
    };
    match direction {
        Direction::Prev => {
            let mut start = excerpt.start_anchor().to_display_point(&map);
            if start >= display_point && start.row() > DisplayRow(0) {
                let Some(excerpt) = map.buffer_snapshot.excerpt_before(excerpt.id()) else {
                    return display_point;
                };
                start = excerpt.start_anchor().to_display_point(&map);
            }
            start
        }
        Direction::Next => {
            let mut end = excerpt.end_anchor().to_display_point(&map);
            *end.row_mut() += 1;
            map.clip_point(end, Bias::Right)
        }
    }
}

pub fn end_of_excerpt(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    direction: Direction,
) -> DisplayPoint {
    let point = map.display_point_to_point(display_point, Bias::Left);
    let Some(excerpt) = map.buffer_snapshot.excerpt_containing(point..point) else {
        return display_point;
    };
    match direction {
        Direction::Prev => {
            let mut start = excerpt.start_anchor().to_display_point(&map);
            if start.row() > DisplayRow(0) {
                *start.row_mut() -= 1;
            }
            start = map.clip_point(start, Bias::Left);
            *start.column_mut() = 0;
            start
        }
        Direction::Next => {
            let mut end = excerpt.end_anchor().to_display_point(&map);
            *end.column_mut() = 0;
            if end <= display_point {
                *end.row_mut() += 1;
                let point_end = map.display_point_to_point(end, Bias::Right);
                let Some(excerpt) = map.buffer_snapshot.excerpt_containing(point_end..point_end)
                else {
                    return display_point;
                };
                end = excerpt.end_anchor().to_display_point(&map);
                *end.column_mut() = 0;
            }
            end
        }
    }
}

/// Scans for a boundary preceding the given start point `from` until a boundary is found,
/// indicated by the given predicate returning true.
/// The predicate is called with the character to the left and right of the candidate boundary location.
/// If FindRange::SingleLine is specified and no boundary is found before the start of the current line, the start of the current line will be returned.
pub fn find_preceding_boundary_point(
    buffer_snapshot: &MultiBufferSnapshot,
    from: Point,
    find_range: FindRange,
    mut is_boundary: impl FnMut(char, char) -> bool,
) -> Point {
    let mut prev_ch = None;
    let mut offset = from.to_offset(buffer_snapshot);

    for ch in buffer_snapshot.reversed_chars_at(offset) {
        if find_range == FindRange::SingleLine && ch == '\n' {
            break;
        }
        if let Some(prev_ch) = prev_ch {
            if is_boundary(ch, prev_ch) {
                break;
            }
        }

        offset -= ch.len_utf8();
        prev_ch = Some(ch);
    }

    offset.to_point(buffer_snapshot)
}

/// Scans for a boundary preceding the given start point `from` until a boundary is found,
/// indicated by the given predicate returning true.
/// The predicate is called with the character to the left and right of the candidate boundary location.
/// If FindRange::SingleLine is specified and no boundary is found before the start of the current line, the start of the current line will be returned.
pub fn find_preceding_boundary_display_point(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    find_range: FindRange,
    is_boundary: impl FnMut(char, char) -> bool,
) -> DisplayPoint {
    let result = find_preceding_boundary_point(
        &map.buffer_snapshot,
        from.to_point(map),
        find_range,
        is_boundary,
    );
    map.clip_point(result.to_display_point(map), Bias::Left)
}

/// Scans for a boundary following the given start point until a boundary is found, indicated by the
/// given predicate returning true. The predicate is called with the character to the left and right
/// of the candidate boundary location, and will be called with `\n` characters indicating the start
/// or end of a line. The function supports optionally returning the point just before the boundary
/// is found via return_point_before_boundary.
pub fn find_boundary_point(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    find_range: FindRange,
    mut is_boundary: impl FnMut(char, char) -> bool,
    return_point_before_boundary: bool,
) -> DisplayPoint {
    let mut offset = from.to_offset(map, Bias::Right);
    let mut prev_offset = offset;
    let mut prev_ch = None;

    for ch in map.buffer_snapshot.chars_at(offset) {
        if find_range == FindRange::SingleLine && ch == '\n' {
            break;
        }
        if let Some(prev_ch) = prev_ch {
            if is_boundary(prev_ch, ch) {
                if return_point_before_boundary {
                    return map.clip_point(prev_offset.to_display_point(map), Bias::Right);
                } else {
                    break;
                }
            }
        }
        prev_offset = offset;
        offset += ch.len_utf8();
        prev_ch = Some(ch);
    }
    map.clip_point(offset.to_display_point(map), Bias::Right)
}

pub fn find_preceding_boundary_trail(
    map: &DisplaySnapshot,
    head: DisplayPoint,
    mut is_boundary: impl FnMut(char, char) -> bool,
) -> (Option<DisplayPoint>, DisplayPoint) {
    let mut offset = head.to_offset(map, Bias::Left);
    let mut trail_offset = None;

    let mut prev_ch = map.buffer_snapshot.chars_at(offset).next();
    let mut forward = map.buffer_snapshot.reversed_chars_at(offset).peekable();

    // Skip newlines
    while let Some(&ch) = forward.peek() {
        if ch == '\n' {
            prev_ch = forward.next();
            offset -= ch.len_utf8();
            trail_offset = Some(offset);
        } else {
            break;
        }
    }

    // Find the boundary
    let start_offset = offset;
    for ch in forward {
        if let Some(prev_ch) = prev_ch {
            if is_boundary(prev_ch, ch) {
                if start_offset == offset {
                    trail_offset = Some(offset);
                } else {
                    break;
                }
            }
        }
        offset -= ch.len_utf8();
        prev_ch = Some(ch);
    }

    let trail = trail_offset
        .map(|trail_offset: usize| map.clip_point(trail_offset.to_display_point(map), Bias::Left));

    (
        trail,
        map.clip_point(offset.to_display_point(map), Bias::Left),
    )
}

/// Finds the location of a boundary
pub fn find_boundary_trail(
    map: &DisplaySnapshot,
    head: DisplayPoint,
    mut is_boundary: impl FnMut(char, char) -> bool,
) -> (Option<DisplayPoint>, DisplayPoint) {
    let mut offset = head.to_offset(map, Bias::Right);
    let mut trail_offset = None;

    let mut prev_ch = map.buffer_snapshot.reversed_chars_at(offset).next();
    let mut forward = map.buffer_snapshot.chars_at(offset).peekable();

    // Skip newlines
    while let Some(&ch) = forward.peek() {
        if ch == '\n' {
            prev_ch = forward.next();
            offset += ch.len_utf8();
            trail_offset = Some(offset);
        } else {
            break;
        }
    }

    // Find the boundary
    let start_offset = offset;
    for ch in forward {
        if let Some(prev_ch) = prev_ch {
            if is_boundary(prev_ch, ch) {
                if start_offset == offset {
                    trail_offset = Some(offset);
                } else {
                    break;
                }
            }
        }
        offset += ch.len_utf8();
        prev_ch = Some(ch);
    }

    let trail = trail_offset
        .map(|trail_offset: usize| map.clip_point(trail_offset.to_display_point(map), Bias::Right));

    (
        trail,
        map.clip_point(offset.to_display_point(map), Bias::Right),
    )
}

pub fn find_boundary(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    find_range: FindRange,
    is_boundary: impl FnMut(char, char) -> bool,
) -> DisplayPoint {
    find_boundary_point(map, from, find_range, is_boundary, false)
}

pub fn find_boundary_exclusive(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    find_range: FindRange,
    is_boundary: impl FnMut(char, char) -> bool,
) -> DisplayPoint {
    find_boundary_point(map, from, find_range, is_boundary, true)
}

/// Returns an iterator over the characters following a given offset in the [`DisplaySnapshot`].
/// The returned value also contains a range of the start/end of a returned character in
/// the [`DisplaySnapshot`]. The offsets are relative to the start of a buffer.
pub fn chars_after(
    map: &DisplaySnapshot,
    mut offset: usize,
) -> impl Iterator<Item = (char, Range<usize>)> + '_ {
    map.buffer_snapshot.chars_at(offset).map(move |ch| {
        let before = offset;
        offset += ch.len_utf8();
        (ch, before..offset)
    })
}

/// Returns a reverse iterator over the characters following a given offset in the [`DisplaySnapshot`].
/// The returned value also contains a range of the start/end of a returned character in
/// the [`DisplaySnapshot`]. The offsets are relative to the start of a buffer.
pub fn chars_before(
    map: &DisplaySnapshot,
    mut offset: usize,
) -> impl Iterator<Item = (char, Range<usize>)> + '_ {
    map.buffer_snapshot
        .reversed_chars_at(offset)
        .map(move |ch| {
            let after = offset;
            offset -= ch.len_utf8();
            (ch, offset..after)
        })
}

/// Returns a list of lines (represented as a [`DisplayPoint`] range) contained
/// within a passed range.
///
/// The line ranges are **always* going to be in bounds of a requested range, which means that
/// the first and the last lines might not necessarily represent the
/// full range of a logical line (as their `.start`/`.end` values are clipped to those of a passed in range).
pub fn split_display_range_by_lines(
    map: &DisplaySnapshot,
    range: Range<DisplayPoint>,
) -> Vec<Range<DisplayPoint>> {
    let mut result = Vec::new();

    let mut start = range.start;
    // Loop over all the covered rows until the one containing the range end
    for row in range.start.row().0..range.end.row().0 {
        let row_end_column = map.line_len(DisplayRow(row));
        let end = map.clip_point(
            DisplayPoint::new(DisplayRow(row), row_end_column),
            Bias::Left,
        );
        if start != end {
            result.push(start..end);
        }
        start = map.clip_point(DisplayPoint::new(DisplayRow(row + 1), 0), Bias::Left);
    }

    // Add the final range from the start of the last end to the original range end.
    result.push(start..range.end);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Buffer, DisplayMap, DisplayRow, ExcerptRange, FoldPlaceholder, MultiBuffer,
        display_map::Inlay,
        test::{editor_test_context::EditorTestContext, marked_display_snapshot},
    };
    use gpui::{AppContext as _, font, px};
    use language::Capability;
    use project::{Project, project_settings::DiagnosticSeverity};
    use settings::SettingsStore;
    use util::post_inc;

    #[gpui::test]
    fn test_previous_word_start(cx: &mut gpui::App) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::App) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            let actual = previous_word_start(&snapshot, display_points[1]);
            let expected = display_points[0];
            if actual != expected {
                eprintln!(
                    "previous_word_start mismatch for '{}': actual={:?}, expected={:?}",
                    marked_text, actual, expected
                );
            }
            assert_eq!(actual, expected);
        }

        assert("\nˇ   ˇlorem", cx);
        assert("ˇ\nˇ   lorem", cx);
        assert("    ˇloremˇ", cx);
        assert("ˇ    ˇlorem", cx);
        assert("    ˇlorˇem", cx);
        assert("\nlorem\nˇ   ˇipsum", cx);
        assert("\n\nˇ\nˇ", cx);
        assert("    ˇlorem  ˇipsum", cx);
        assert("ˇlorem-ˇipsum", cx);
        assert("loremˇ-#$@ˇipsum", cx);
        assert("ˇlorem_ˇipsum", cx);
        assert(" ˇdefγˇ", cx);
        assert(" ˇbcΔˇ", cx);
        // Test punctuation skipping behavior
        assert("ˇhello.ˇ", cx);
        assert("helloˇ...ˇ", cx);
        assert("helloˇ.---..ˇtest", cx);
        assert("test  ˇ.--ˇtest", cx);
        assert("oneˇ,;:!?ˇtwo", cx);
    }

    #[gpui::test]
    fn test_previous_subword_start(cx: &mut gpui::App) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::App) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                previous_subword_start(&snapshot, display_points[1]),
                display_points[0]
            );
        }

        // Subword boundaries are respected
        assert("lorem_ˇipˇsum", cx);
        assert("lorem_ˇipsumˇ", cx);
        assert("ˇlorem_ˇipsum", cx);
        assert("lorem_ˇipsum_ˇdolor", cx);
        assert("loremˇIpˇsum", cx);
        assert("loremˇIpsumˇ", cx);

        // Word boundaries are still respected
        assert("\nˇ   ˇlorem", cx);
        assert("    ˇloremˇ", cx);
        assert("    ˇlorˇem", cx);
        assert("\nlorem\nˇ   ˇipsum", cx);
        assert("\n\nˇ\nˇ", cx);
        assert("    ˇlorem  ˇipsum", cx);
        assert("loremˇ-ˇipsum", cx);
        assert("loremˇ-#$@ˇipsum", cx);
        assert(" ˇdefγˇ", cx);
        assert(" bcˇΔˇ", cx);
        assert(" ˇbcδˇ", cx);
        assert(" abˇ——ˇcd", cx);
    }

    #[gpui::test]
    fn test_find_preceding_boundary(cx: &mut gpui::App) {
        init_test(cx);

        fn assert(
            marked_text: &str,
            cx: &mut gpui::App,
            is_boundary: impl FnMut(char, char) -> bool,
        ) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                find_preceding_boundary_display_point(
                    &snapshot,
                    display_points[1],
                    FindRange::MultiLine,
                    is_boundary
                ),
                display_points[0]
            );
        }

        assert("abcˇdef\ngh\nijˇk", cx, |left, right| {
            left == 'c' && right == 'd'
        });
        assert("abcdef\nˇgh\nijˇk", cx, |left, right| {
            left == '\n' && right == 'g'
        });
        let mut line_count = 0;
        assert("abcdef\nˇgh\nijˇk", cx, |left, _| {
            if left == '\n' {
                line_count += 1;
                line_count == 2
            } else {
                false
            }
        });
    }

    #[gpui::test]
    fn test_find_preceding_boundary_with_inlays(cx: &mut gpui::App) {
        init_test(cx);

        let input_text = "abcdefghijklmnopqrstuvwxys";
        let font = font("Helvetica");
        let font_size = px(14.0);
        let buffer = MultiBuffer::build_simple(input_text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);

        let display_map = cx.new(|cx| {
            DisplayMap::new(
                buffer,
                font,
                font_size,
                None,
                1,
                1,
                FoldPlaceholder::test(),
                DiagnosticSeverity::Warning,
                cx,
            )
        });

        // add all kinds of inlays between two word boundaries: we should be able to cross them all, when looking for another boundary
        let mut id = 0;
        let inlays = (0..buffer_snapshot.len())
            .flat_map(|offset| {
                [
                    Inlay::edit_prediction(
                        post_inc(&mut id),
                        buffer_snapshot.anchor_at(offset, Bias::Left),
                        "test",
                    ),
                    Inlay::edit_prediction(
                        post_inc(&mut id),
                        buffer_snapshot.anchor_at(offset, Bias::Right),
                        "test",
                    ),
                    Inlay::mock_hint(
                        post_inc(&mut id),
                        buffer_snapshot.anchor_at(offset, Bias::Left),
                        "test",
                    ),
                    Inlay::mock_hint(
                        post_inc(&mut id),
                        buffer_snapshot.anchor_at(offset, Bias::Right),
                        "test",
                    ),
                ]
            })
            .collect();
        let snapshot = display_map.update(cx, |map, cx| {
            map.splice_inlays(&[], inlays, cx);
            map.snapshot(cx)
        });

        assert_eq!(
            find_preceding_boundary_display_point(
                &snapshot,
                buffer_snapshot.len().to_display_point(&snapshot),
                FindRange::MultiLine,
                |left, _| left == 'e',
            ),
            snapshot
                .buffer_snapshot
                .offset_to_point(5)
                .to_display_point(&snapshot),
            "Should not stop at inlays when looking for boundaries"
        );
    }

    #[gpui::test]
    fn test_next_word_end(cx: &mut gpui::App) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::App) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            let actual = next_word_end(&snapshot, display_points[0]);
            let expected = display_points[1];
            if actual != expected {
                eprintln!(
                    "next_word_end mismatch for '{}': actual={:?}, expected={:?}",
                    marked_text, actual, expected
                );
            }
            assert_eq!(actual, expected);
        }

        assert("\nˇ   loremˇ", cx);
        assert("    ˇloremˇ", cx);
        assert("    lorˇemˇ", cx);
        assert("    loremˇ    ˇ\nipsum\n", cx);
        assert("\nˇ\nˇ\n\n", cx);
        assert("loremˇ    ipsumˇ   ", cx);
        assert("loremˇ-ipsumˇ", cx);
        assert("loremˇ#$@-ˇipsum", cx);
        assert("loremˇ_ipsumˇ", cx);
        assert(" ˇbcΔˇ", cx);
        assert(" abˇ——ˇcd", cx);
        // Test punctuation skipping behavior
        assert("ˇ.helloˇ", cx);
        assert("display_pointsˇ[0ˇ]", cx);
        assert("ˇ...ˇhello", cx);
        assert("helloˇ.---..ˇtest", cx);
        assert("testˇ.--ˇ test", cx);
        assert("oneˇ,;:!?ˇtwo", cx);
    }

    #[gpui::test]
    fn test_next_subword_end(cx: &mut gpui::App) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::App) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                next_subword_end(&snapshot, display_points[0]),
                display_points[1]
            );
        }

        // Subword boundaries are respected
        assert("loˇremˇ_ipsum", cx);
        assert("ˇloremˇ_ipsum", cx);
        assert("loremˇ_ipsumˇ", cx);
        assert("loremˇ_ipsumˇ_dolor", cx);
        assert("loˇremˇIpsum", cx);
        assert("loremˇIpsumˇDolor", cx);

        // Word boundaries are still respected
        assert("\nˇ   loremˇ", cx);
        assert("    ˇloremˇ", cx);
        assert("    lorˇemˇ", cx);
        assert("    loremˇ    ˇ\nipsum\n", cx);
        assert("\nˇ\nˇ\n\n", cx);
        assert("loremˇ    ipsumˇ   ", cx);
        assert("loremˇ-ˇipsum", cx);
        assert("loremˇ#$@-ˇipsum", cx);
        assert("loremˇ_ipsumˇ", cx);
        assert(" ˇbcˇΔ", cx);
        assert(" abˇ——ˇcd", cx);
    }

    #[gpui::test]
    fn test_find_boundary(cx: &mut gpui::App) {
        init_test(cx);

        fn assert(
            marked_text: &str,
            cx: &mut gpui::App,
            is_boundary: impl FnMut(char, char) -> bool,
        ) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                find_boundary(
                    &snapshot,
                    display_points[0],
                    FindRange::MultiLine,
                    is_boundary,
                ),
                display_points[1]
            );
        }

        assert("abcˇdef\ngh\nijˇk", cx, |left, right| {
            left == 'j' && right == 'k'
        });
        assert("abˇcdef\ngh\nˇijk", cx, |left, right| {
            left == '\n' && right == 'i'
        });
        let mut line_count = 0;
        assert("abcˇdef\ngh\nˇijk", cx, |left, _| {
            if left == '\n' {
                line_count += 1;
                line_count == 2
            } else {
                false
            }
        });
    }

    #[gpui::test]
    async fn test_move_up_and_down_with_excerpts(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            init_test(cx);
        });

        let mut cx = EditorTestContext::new(cx).await;
        let editor = cx.editor.clone();
        let window = cx.window;
        _ = cx.update_window(window, |_, window, cx| {
            let text_layout_details = editor.read(cx).text_layout_details(window);

            let font = font("Helvetica");

            let buffer = cx.new(|cx| Buffer::local("abc\ndefg\nhijkl\nmn", cx));
            let multibuffer = cx.new(|cx| {
                let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
                multibuffer.push_excerpts(
                    buffer.clone(),
                    [
                        ExcerptRange::new(Point::new(0, 0)..Point::new(1, 4)),
                        ExcerptRange::new(Point::new(2, 0)..Point::new(3, 2)),
                    ],
                    cx,
                );
                multibuffer
            });
            let display_map = cx.new(|cx| {
                DisplayMap::new(
                    multibuffer,
                    font,
                    px(14.0),
                    None,
                    0,
                    1,
                    FoldPlaceholder::test(),
                    DiagnosticSeverity::Warning,
                    cx,
                )
            });
            let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));

            assert_eq!(snapshot.text(), "abc\ndefg\n\nhijkl\nmn");

            let col_2_x = snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(0), 2), &text_layout_details);

            // Can't move up into the first excerpt's header
            assert_eq!(
                up(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(0), 2),
                    SelectionGoal::HorizontalPosition(col_2_x.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(0), 0),
                    SelectionGoal::HorizontalPosition(col_2_x.0),
                ),
            );
            assert_eq!(
                up(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(0), 0),
                    SelectionGoal::None,
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(0), 0),
                    SelectionGoal::HorizontalPosition(0.0),
                ),
            );

            let col_4_x = snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(1), 4), &text_layout_details);

            // Move up and down within first excerpt
            assert_eq!(
                up(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(1), 4),
                    SelectionGoal::HorizontalPosition(col_4_x.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(0), 3),
                    SelectionGoal::HorizontalPosition(col_4_x.0)
                ),
            );
            assert_eq!(
                down(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(0), 3),
                    SelectionGoal::HorizontalPosition(col_4_x.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(1), 4),
                    SelectionGoal::HorizontalPosition(col_4_x.0)
                ),
            );

            let col_5_x = snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(3), 5), &text_layout_details);

            // Move up and down across second excerpt's header
            assert_eq!(
                up(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(3), 5),
                    SelectionGoal::HorizontalPosition(col_5_x.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(1), 4),
                    SelectionGoal::HorizontalPosition(col_5_x.0)
                ),
            );
            assert_eq!(
                down(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(1), 4),
                    SelectionGoal::HorizontalPosition(col_5_x.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(3), 5),
                    SelectionGoal::HorizontalPosition(col_5_x.0)
                ),
            );

            let max_point_x = snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(4), 2), &text_layout_details);

            // Can't move down off the end, and attempting to do so leaves the selection goal unchanged
            assert_eq!(
                down(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(4), 0),
                    SelectionGoal::HorizontalPosition(0.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(4), 2),
                    SelectionGoal::HorizontalPosition(0.0)
                ),
            );
            assert_eq!(
                down(
                    &snapshot,
                    DisplayPoint::new(DisplayRow(4), 2),
                    SelectionGoal::HorizontalPosition(max_point_x.0),
                    false,
                    &text_layout_details
                ),
                (
                    DisplayPoint::new(DisplayRow(4), 2),
                    SelectionGoal::HorizontalPosition(max_point_x.0)
                ),
            );
        });
    }

    fn init_test(cx: &mut gpui::App) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        workspace::init_settings(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        crate::init(cx);
        Project::init_settings(cx);
    }
}
