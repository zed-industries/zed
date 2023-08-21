use super::{Bias, DisplayPoint, DisplaySnapshot, SelectionGoal, ToDisplayPoint};
use crate::{char_kind, CharKind, ToPoint};
use language::Point;
use std::ops::Range;

pub fn left(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    if point.column() > 0 {
        *point.column_mut() -= 1;
    } else if point.row() > 0 {
        *point.row_mut() -= 1;
        *point.column_mut() = map.line_len(point.row());
    }
    map.clip_point(point, Bias::Left)
}

pub fn saturating_left(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    if point.column() > 0 {
        *point.column_mut() -= 1;
    }
    map.clip_point(point, Bias::Left)
}

pub fn right(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    let max_column = map.line_len(point.row());
    if point.column() < max_column {
        *point.column_mut() += 1;
    } else if point.row() < map.max_point().row() {
        *point.row_mut() += 1;
        *point.column_mut() = 0;
    }
    map.clip_point(point, Bias::Right)
}

pub fn saturating_right(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    *point.column_mut() += 1;
    map.clip_point(point, Bias::Right)
}

pub fn up(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    goal: SelectionGoal,
    preserve_column_at_start: bool,
) -> (DisplayPoint, SelectionGoal) {
    up_by_rows(map, start, 1, goal, preserve_column_at_start)
}

pub fn down(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    goal: SelectionGoal,
    preserve_column_at_end: bool,
) -> (DisplayPoint, SelectionGoal) {
    down_by_rows(map, start, 1, goal, preserve_column_at_end)
}

pub fn up_by_rows(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    row_count: u32,
    goal: SelectionGoal,
    preserve_column_at_start: bool,
) -> (DisplayPoint, SelectionGoal) {
    let mut goal_column = match goal {
        SelectionGoal::Column(column) => column,
        SelectionGoal::ColumnRange { end, .. } => end,
        _ => map.column_to_chars(start.row(), start.column()),
    };

    let prev_row = start.row().saturating_sub(row_count);
    let mut point = map.clip_point(
        DisplayPoint::new(prev_row, map.line_len(prev_row)),
        Bias::Left,
    );
    if point.row() < start.row() {
        *point.column_mut() = map.column_from_chars(point.row(), goal_column);
    } else if preserve_column_at_start {
        return (start, goal);
    } else {
        point = DisplayPoint::new(0, 0);
        goal_column = 0;
    }

    let mut clipped_point = map.clip_point(point, Bias::Left);
    if clipped_point.row() < point.row() {
        clipped_point = map.clip_point(point, Bias::Right);
    }
    (clipped_point, SelectionGoal::Column(goal_column))
}

pub fn down_by_rows(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    row_count: u32,
    goal: SelectionGoal,
    preserve_column_at_end: bool,
) -> (DisplayPoint, SelectionGoal) {
    let mut goal_column = match goal {
        SelectionGoal::Column(column) => column,
        SelectionGoal::ColumnRange { end, .. } => end,
        _ => map.column_to_chars(start.row(), start.column()),
    };

    let new_row = start.row() + row_count;
    let mut point = map.clip_point(DisplayPoint::new(new_row, 0), Bias::Right);
    if point.row() > start.row() {
        *point.column_mut() = map.column_from_chars(point.row(), goal_column);
    } else if preserve_column_at_end {
        return (start, goal);
    } else {
        point = map.max_point();
        goal_column = map.column_to_chars(point.row(), point.column())
    }

    let mut clipped_point = map.clip_point(point, Bias::Right);
    if clipped_point.row() > point.row() {
        clipped_point = map.clip_point(point, Bias::Left);
    }
    (clipped_point, SelectionGoal::Column(goal_column))
}

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

pub fn indented_line_beginning(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    stop_at_soft_boundaries: bool,
) -> DisplayPoint {
    let point = display_point.to_point(map);
    let soft_line_start = map.clip_point(DisplayPoint::new(display_point.row(), 0), Bias::Right);
    let indent_start = Point::new(
        point.row,
        map.buffer_snapshot.indent_size_for_line(point.row).len,
    )
    .to_display_point(map);
    let line_start = map.prev_line_boundary(point).1;

    if stop_at_soft_boundaries && soft_line_start > indent_start && display_point != soft_line_start
    {
        soft_line_start
    } else if stop_at_soft_boundaries && display_point != indent_start {
        indent_start
    } else {
        line_start
    }
}

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

pub fn previous_word_start(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let language = map.buffer_snapshot.language_at(raw_point);

    find_preceding_boundary(map, point, |left, right| {
        (char_kind(language, left) != char_kind(language, right) && !right.is_whitespace())
            || left == '\n'
    })
}

pub fn previous_subword_start(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let language = map.buffer_snapshot.language_at(raw_point);
    find_preceding_boundary(map, point, |left, right| {
        let is_word_start =
            char_kind(language, left) != char_kind(language, right) && !right.is_whitespace();
        let is_subword_start =
            left == '_' && right != '_' || left.is_lowercase() && right.is_uppercase();
        is_word_start || is_subword_start || left == '\n'
    })
}

pub fn next_word_end(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let language = map.buffer_snapshot.language_at(raw_point);
    find_boundary(map, point, |left, right| {
        (char_kind(language, left) != char_kind(language, right) && !left.is_whitespace())
            || right == '\n'
    })
}

pub fn next_subword_end(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let raw_point = point.to_point(map);
    let language = map.buffer_snapshot.language_at(raw_point);
    find_boundary(map, point, |left, right| {
        let is_word_end =
            (char_kind(language, left) != char_kind(language, right)) && !left.is_whitespace();
        let is_subword_end =
            left != '_' && right == '_' || left.is_lowercase() && right.is_uppercase();
        is_word_end || is_subword_end || right == '\n'
    })
}

pub fn start_of_paragraph(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    mut count: usize,
) -> DisplayPoint {
    let point = display_point.to_point(map);
    if point.row == 0 {
        return map.max_point();
    }

    let mut found_non_blank_line = false;
    for row in (0..point.row + 1).rev() {
        let blank = map.buffer_snapshot.is_line_blank(row);
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

pub fn end_of_paragraph(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    mut count: usize,
) -> DisplayPoint {
    let point = display_point.to_point(map);
    if point.row == map.max_buffer_row() {
        return DisplayPoint::zero();
    }

    let mut found_non_blank_line = false;
    for row in point.row..map.max_buffer_row() + 1 {
        let blank = map.buffer_snapshot.is_line_blank(row);
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

/// Scans for a boundary preceding the given start point `from` until a boundary is found, indicated by the
/// given predicate returning true. The predicate is called with the character to the left and right
/// of the candidate boundary location, and will be called with `\n` characters indicating the start
/// or end of a line.
pub fn find_preceding_boundary(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    mut is_boundary: impl FnMut(char, char) -> bool,
) -> DisplayPoint {
    let mut start_column = 0;
    let mut soft_wrap_row = from.row() + 1;

    let mut prev = None;
    for (ch, point) in map.reverse_chars_at(from) {
        // Recompute soft_wrap_indent if the row has changed
        if point.row() != soft_wrap_row {
            soft_wrap_row = point.row();

            if point.row() == 0 {
                start_column = 0;
            } else if let Some(indent) = map.soft_wrap_indent(point.row() - 1) {
                start_column = indent;
            }
        }

        // If the current point is in the soft_wrap, skip comparing it
        if point.column() < start_column {
            continue;
        }

        if let Some((prev_ch, prev_point)) = prev {
            if is_boundary(ch, prev_ch) {
                return map.clip_point(prev_point, Bias::Left);
            }
        }

        prev = Some((ch, point));
    }
    map.clip_point(DisplayPoint::zero(), Bias::Left)
}

/// Scans for a boundary preceding the given start point `from` until a boundary is found, indicated by the
/// given predicate returning true. The predicate is called with the character to the left and right
/// of the candidate boundary location, and will be called with `\n` characters indicating the start
/// or end of a line. If no boundary is found, the start of the line is returned.
pub fn find_preceding_boundary_in_line(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    mut is_boundary: impl FnMut(char, char) -> bool,
) -> DisplayPoint {
    let mut start_column = 0;
    if from.row() > 0 {
        if let Some(indent) = map.soft_wrap_indent(from.row() - 1) {
            start_column = indent;
        }
    }

    let mut prev = None;
    for (ch, point) in map.reverse_chars_at(from) {
        if let Some((prev_ch, prev_point)) = prev {
            if is_boundary(ch, prev_ch) {
                return map.clip_point(prev_point, Bias::Left);
            }
        }

        if ch == '\n' || point.column() < start_column {
            break;
        }

        prev = Some((ch, point));
    }

    map.clip_point(prev.map(|(_, point)| point).unwrap_or(from), Bias::Left)
}

/// Scans for a boundary following the given start point until a boundary is found, indicated by the
/// given predicate returning true. The predicate is called with the character to the left and right
/// of the candidate boundary location, and will be called with `\n` characters indicating the start
/// or end of a line.
pub fn find_boundary(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    mut is_boundary: impl FnMut(char, char) -> bool,
) -> DisplayPoint {
    let mut prev_ch = None;
    for (ch, point) in map.chars_at(from) {
        if let Some(prev_ch) = prev_ch {
            if is_boundary(prev_ch, ch) {
                return map.clip_point(point, Bias::Right);
            }
        }

        prev_ch = Some(ch);
    }
    map.clip_point(map.max_point(), Bias::Right)
}

/// Scans for a boundary following the given start point until a boundary is found, indicated by the
/// given predicate returning true. The predicate is called with the character to the left and right
/// of the candidate boundary location, and will be called with `\n` characters indicating the start
/// or end of a line. If no boundary is found, the end of the line is returned
pub fn find_boundary_in_line(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    mut is_boundary: impl FnMut(char, char) -> bool,
) -> DisplayPoint {
    let mut prev = None;
    for (ch, point) in map.chars_at(from) {
        if let Some((prev_ch, _)) = prev {
            if is_boundary(prev_ch, ch) {
                return map.clip_point(point, Bias::Right);
            }
        }

        prev = Some((ch, point));

        if ch == '\n' {
            break;
        }
    }

    // Return the last position checked so that we give a point right before the newline or eof.
    map.clip_point(prev.map(|(_, point)| point).unwrap_or(from), Bias::Right)
}

pub fn is_inside_word(map: &DisplaySnapshot, point: DisplayPoint) -> bool {
    let raw_point = point.to_point(map);
    let language = map.buffer_snapshot.language_at(raw_point);
    let ix = map.clip_point(point, Bias::Left).to_offset(map, Bias::Left);
    let text = &map.buffer_snapshot;
    let next_char_kind = text.chars_at(ix).next().map(|c| char_kind(language, c));
    let prev_char_kind = text
        .reversed_chars_at(ix)
        .next()
        .map(|c| char_kind(language, c));
    prev_char_kind.zip(next_char_kind) == Some((CharKind::Word, CharKind::Word))
}

pub fn surrounding_word(map: &DisplaySnapshot, position: DisplayPoint) -> Range<DisplayPoint> {
    let position = map
        .clip_point(position, Bias::Left)
        .to_offset(map, Bias::Left);
    let (range, _) = map.buffer_snapshot.surrounding_word(position);
    let start = range
        .start
        .to_point(&map.buffer_snapshot)
        .to_display_point(map);
    let end = range
        .end
        .to_point(&map.buffer_snapshot)
        .to_display_point(map);
    start..end
}

pub fn split_display_range_by_lines(
    map: &DisplaySnapshot,
    range: Range<DisplayPoint>,
) -> Vec<Range<DisplayPoint>> {
    let mut result = Vec::new();

    let mut start = range.start;
    // Loop over all the covered rows until the one containing the range end
    for row in range.start.row()..range.end.row() {
        let row_end_column = map.line_len(row);
        let end = map.clip_point(DisplayPoint::new(row, row_end_column), Bias::Left);
        if start != end {
            result.push(start..end);
        }
        start = map.clip_point(DisplayPoint::new(row + 1, 0), Bias::Left);
    }

    // Add the final range from the start of the last end to the original range end.
    result.push(start..range.end);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::Inlay, test::marked_display_snapshot, Buffer, DisplayMap, ExcerptRange,
        InlayId, MultiBuffer,
    };
    use settings::SettingsStore;
    use util::post_inc;

    #[gpui::test]
    fn test_previous_word_start(cx: &mut gpui::AppContext) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::AppContext) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                previous_word_start(&snapshot, display_points[1]),
                display_points[0]
            );
        }

        assert("\nˇ   ˇlorem", cx);
        assert("ˇ\nˇ   lorem", cx);
        assert("    ˇloremˇ", cx);
        assert("ˇ    ˇlorem", cx);
        assert("    ˇlorˇem", cx);
        assert("\nlorem\nˇ   ˇipsum", cx);
        assert("\n\nˇ\nˇ", cx);
        assert("    ˇlorem  ˇipsum", cx);
        assert("loremˇ-ˇipsum", cx);
        assert("loremˇ-#$@ˇipsum", cx);
        assert("ˇlorem_ˇipsum", cx);
        assert(" ˇdefγˇ", cx);
        assert(" ˇbcΔˇ", cx);
        assert(" abˇ——ˇcd", cx);
    }

    #[gpui::test]
    fn test_previous_subword_start(cx: &mut gpui::AppContext) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::AppContext) {
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
    fn test_find_preceding_boundary(cx: &mut gpui::AppContext) {
        init_test(cx);

        fn assert(
            marked_text: &str,
            cx: &mut gpui::AppContext,
            is_boundary: impl FnMut(char, char) -> bool,
        ) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                find_preceding_boundary(&snapshot, display_points[1], is_boundary),
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
    fn test_find_preceding_boundary_with_inlays(cx: &mut gpui::AppContext) {
        init_test(cx);

        let input_text = "abcdefghijklmnopqrstuvwxys";
        let family_id = cx
            .font_cache()
            .load_family(&["Helvetica"], &Default::default())
            .unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;
        let buffer = MultiBuffer::build_simple(input_text, cx);
        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let display_map =
            cx.add_model(|cx| DisplayMap::new(buffer, font_id, font_size, None, 1, 1, cx));

        // add all kinds of inlays between two word boundaries: we should be able to cross them all, when looking for another boundary
        let mut id = 0;
        let inlays = (0..buffer_snapshot.len())
            .map(|offset| {
                [
                    Inlay {
                        id: InlayId::Suggestion(post_inc(&mut id)),
                        position: buffer_snapshot.anchor_at(offset, Bias::Left),
                        text: format!("test").into(),
                    },
                    Inlay {
                        id: InlayId::Suggestion(post_inc(&mut id)),
                        position: buffer_snapshot.anchor_at(offset, Bias::Right),
                        text: format!("test").into(),
                    },
                    Inlay {
                        id: InlayId::Hint(post_inc(&mut id)),
                        position: buffer_snapshot.anchor_at(offset, Bias::Left),
                        text: format!("test").into(),
                    },
                    Inlay {
                        id: InlayId::Hint(post_inc(&mut id)),
                        position: buffer_snapshot.anchor_at(offset, Bias::Right),
                        text: format!("test").into(),
                    },
                ]
            })
            .flatten()
            .collect();
        let snapshot = display_map.update(cx, |map, cx| {
            map.splice_inlays(Vec::new(), inlays, cx);
            map.snapshot(cx)
        });

        assert_eq!(
            find_preceding_boundary(
                &snapshot,
                buffer_snapshot.len().to_display_point(&snapshot),
                |left, _| left == 'a',
            ),
            0.to_display_point(&snapshot),
            "Should not stop at inlays when looking for boundaries"
        );

        assert_eq!(
            find_preceding_boundary_in_line(
                &snapshot,
                buffer_snapshot.len().to_display_point(&snapshot),
                |left, _| left == 'a',
            ),
            0.to_display_point(&snapshot),
            "Should not stop at inlays when looking for boundaries in line"
        );
    }

    #[gpui::test]
    fn test_next_word_end(cx: &mut gpui::AppContext) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::AppContext) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                next_word_end(&snapshot, display_points[0]),
                display_points[1]
            );
        }

        assert("\nˇ   loremˇ", cx);
        assert("    ˇloremˇ", cx);
        assert("    lorˇemˇ", cx);
        assert("    loremˇ    ˇ\nipsum\n", cx);
        assert("\nˇ\nˇ\n\n", cx);
        assert("loremˇ    ipsumˇ   ", cx);
        assert("loremˇ-ˇipsum", cx);
        assert("loremˇ#$@-ˇipsum", cx);
        assert("loremˇ_ipsumˇ", cx);
        assert(" ˇbcΔˇ", cx);
        assert(" abˇ——ˇcd", cx);
    }

    #[gpui::test]
    fn test_next_subword_end(cx: &mut gpui::AppContext) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::AppContext) {
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
    fn test_find_boundary(cx: &mut gpui::AppContext) {
        init_test(cx);

        fn assert(
            marked_text: &str,
            cx: &mut gpui::AppContext,
            is_boundary: impl FnMut(char, char) -> bool,
        ) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                find_boundary(&snapshot, display_points[0], is_boundary),
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
    fn test_surrounding_word(cx: &mut gpui::AppContext) {
        init_test(cx);

        fn assert(marked_text: &str, cx: &mut gpui::AppContext) {
            let (snapshot, display_points) = marked_display_snapshot(marked_text, cx);
            assert_eq!(
                surrounding_word(&snapshot, display_points[1]),
                display_points[0]..display_points[2]
            );
        }

        assert("ˇˇloremˇ  ipsum", cx);
        assert("ˇloˇremˇ  ipsum", cx);
        assert("ˇloremˇˇ  ipsum", cx);
        assert("loremˇ ˇ  ˇipsum", cx);
        assert("lorem\nˇˇˇ\nipsum", cx);
        assert("lorem\nˇˇipsumˇ", cx);
        assert("lorem,ˇˇ ˇipsum", cx);
        assert("ˇloremˇˇ, ipsum", cx);
    }

    #[gpui::test]
    fn test_move_up_and_down_with_excerpts(cx: &mut gpui::AppContext) {
        init_test(cx);

        let family_id = cx
            .font_cache()
            .load_family(&["Helvetica"], &Default::default())
            .unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();

        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndefg\nhijkl\nmn", cx));
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            multibuffer.push_excerpts(
                buffer.clone(),
                [
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(1, 4),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(2, 0)..Point::new(3, 2),
                        primary: None,
                    },
                ],
                cx,
            );
            multibuffer
        });
        let display_map =
            cx.add_model(|cx| DisplayMap::new(multibuffer, font_id, 14.0, None, 2, 2, cx));
        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));

        assert_eq!(snapshot.text(), "\n\nabc\ndefg\n\n\nhijkl\nmn");

        // Can't move up into the first excerpt's header
        assert_eq!(
            up(
                &snapshot,
                DisplayPoint::new(2, 2),
                SelectionGoal::Column(2),
                false
            ),
            (DisplayPoint::new(2, 0), SelectionGoal::Column(0)),
        );
        assert_eq!(
            up(
                &snapshot,
                DisplayPoint::new(2, 0),
                SelectionGoal::None,
                false
            ),
            (DisplayPoint::new(2, 0), SelectionGoal::Column(0)),
        );

        // Move up and down within first excerpt
        assert_eq!(
            up(
                &snapshot,
                DisplayPoint::new(3, 4),
                SelectionGoal::Column(4),
                false
            ),
            (DisplayPoint::new(2, 3), SelectionGoal::Column(4)),
        );
        assert_eq!(
            down(
                &snapshot,
                DisplayPoint::new(2, 3),
                SelectionGoal::Column(4),
                false
            ),
            (DisplayPoint::new(3, 4), SelectionGoal::Column(4)),
        );

        // Move up and down across second excerpt's header
        assert_eq!(
            up(
                &snapshot,
                DisplayPoint::new(6, 5),
                SelectionGoal::Column(5),
                false
            ),
            (DisplayPoint::new(3, 4), SelectionGoal::Column(5)),
        );
        assert_eq!(
            down(
                &snapshot,
                DisplayPoint::new(3, 4),
                SelectionGoal::Column(5),
                false
            ),
            (DisplayPoint::new(6, 5), SelectionGoal::Column(5)),
        );

        // Can't move down off the end
        assert_eq!(
            down(
                &snapshot,
                DisplayPoint::new(7, 0),
                SelectionGoal::Column(0),
                false
            ),
            (DisplayPoint::new(7, 2), SelectionGoal::Column(2)),
        );
        assert_eq!(
            down(
                &snapshot,
                DisplayPoint::new(7, 2),
                SelectionGoal::Column(2),
                false
            ),
            (DisplayPoint::new(7, 2), SelectionGoal::Column(2)),
        );
    }

    fn init_test(cx: &mut gpui::AppContext) {
        cx.set_global(SettingsStore::test(cx));
        theme::init((), cx);
        language::init(cx);
        crate::init(cx);
    }
}
