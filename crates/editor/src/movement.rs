use super::{Bias, DisplayPoint, DisplaySnapshot, SelectionGoal, ToDisplayPoint};
use crate::{char_kind, CharKind, ToPoint};
use anyhow::Result;
use language::Point;
use std::ops::Range;

pub fn left(map: &DisplaySnapshot, mut point: DisplayPoint) -> Result<DisplayPoint> {
    if point.column() > 0 {
        *point.column_mut() -= 1;
    } else if point.row() > 0 {
        *point.row_mut() -= 1;
        *point.column_mut() = map.line_len(point.row());
    }
    Ok(map.clip_point(point, Bias::Left))
}

pub fn right(map: &DisplaySnapshot, mut point: DisplayPoint) -> Result<DisplayPoint> {
    let max_column = map.line_len(point.row());
    if point.column() < max_column {
        *point.column_mut() += 1;
    } else if point.row() < map.max_point().row() {
        *point.row_mut() += 1;
        *point.column_mut() = 0;
    }
    Ok(map.clip_point(point, Bias::Right))
}

pub fn up(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    goal: SelectionGoal,
) -> Result<(DisplayPoint, SelectionGoal)> {
    let mut goal_column = if let SelectionGoal::Column(column) = goal {
        column
    } else {
        map.column_to_chars(start.row(), start.column())
    };

    let prev_row = start.row().saturating_sub(1);
    let mut point = map.clip_point(
        DisplayPoint::new(prev_row, map.line_len(prev_row)),
        Bias::Left,
    );
    if point.row() < start.row() {
        *point.column_mut() = map.column_from_chars(point.row(), goal_column);
    } else {
        point = DisplayPoint::new(0, 0);
        goal_column = 0;
    }

    let clip_bias = if point.column() == map.line_len(point.row()) {
        Bias::Left
    } else {
        Bias::Right
    };

    Ok((
        map.clip_point(point, clip_bias),
        SelectionGoal::Column(goal_column),
    ))
}

pub fn down(
    map: &DisplaySnapshot,
    start: DisplayPoint,
    goal: SelectionGoal,
) -> Result<(DisplayPoint, SelectionGoal)> {
    let mut goal_column = if let SelectionGoal::Column(column) = goal {
        column
    } else {
        map.column_to_chars(start.row(), start.column())
    };

    let next_row = start.row() + 1;
    let mut point = map.clip_point(DisplayPoint::new(next_row, 0), Bias::Right);
    if point.row() > start.row() {
        *point.column_mut() = map.column_from_chars(point.row(), goal_column);
    } else {
        point = map.max_point();
        goal_column = map.column_to_chars(point.row(), point.column())
    }

    let clip_bias = if point.column() == map.line_len(point.row()) {
        Bias::Left
    } else {
        Bias::Right
    };

    Ok((
        map.clip_point(point, clip_bias),
        SelectionGoal::Column(goal_column),
    ))
}

pub fn line_beginning(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    stop_at_soft_boundaries: bool,
) -> DisplayPoint {
    let point = display_point.to_point(map);
    let soft_line_start = map.clip_point(DisplayPoint::new(display_point.row(), 0), Bias::Right);
    let indent_start = Point::new(
        point.row,
        map.buffer_snapshot.indent_column_for_line(point.row),
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

pub fn previous_word_start(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    let mut line_start = 0;
    if point.row() > 0 {
        if let Some(indent) = map.soft_wrap_indent(point.row() - 1) {
            line_start = indent;
        }
    }

    if point.column() == line_start {
        if point.row() == 0 {
            return DisplayPoint::new(0, 0);
        } else {
            let row = point.row() - 1;
            point = map.clip_point(DisplayPoint::new(row, map.line_len(row)), Bias::Left);
        }
    }

    let mut boundary = DisplayPoint::new(point.row(), 0);
    let mut column = 0;
    let mut prev_char_kind = CharKind::Whitespace;
    for c in map.chars_at(DisplayPoint::new(point.row(), 0)) {
        if column >= point.column() {
            break;
        }

        let char_kind = char_kind(c);
        if char_kind != prev_char_kind && char_kind != CharKind::Whitespace && c != '\n' {
            *boundary.column_mut() = column;
        }

        prev_char_kind = char_kind;
        column += c.len_utf8() as u32;
    }
    boundary
}

pub fn next_word_end(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    let mut prev_char_kind = None;
    for c in map.chars_at(point) {
        let char_kind = char_kind(c);
        if let Some(prev_char_kind) = prev_char_kind {
            if c == '\n' {
                break;
            }
            if prev_char_kind != char_kind && prev_char_kind != CharKind::Whitespace {
                break;
            }
        }

        if c == '\n' {
            *point.row_mut() += 1;
            *point.column_mut() = 0;
        } else {
            *point.column_mut() += c.len_utf8() as u32;
        }
        prev_char_kind = Some(char_kind);
    }
    map.clip_point(point, Bias::Right)
}

pub fn is_inside_word(map: &DisplaySnapshot, point: DisplayPoint) -> bool {
    let ix = map.clip_point(point, Bias::Left).to_offset(map, Bias::Left);
    let text = &map.buffer_snapshot;
    let next_char_kind = text.chars_at(ix).next().map(char_kind);
    let prev_char_kind = text.reversed_chars_at(ix).next().map(char_kind);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Buffer, DisplayMap, MultiBuffer};
    use language::Point;

    #[gpui::test]
    fn test_move_up_and_down_with_excerpts(cx: &mut gpui::MutableAppContext) {
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
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
                    Point::new(0, 0)..Point::new(1, 4),
                    Point::new(2, 0)..Point::new(3, 2),
                ],
                cx,
            );
            multibuffer
        });

        let display_map =
            cx.add_model(|cx| DisplayMap::new(multibuffer, 2, font_id, 14.0, None, 2, 2, cx));

        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.text(), "\n\nabc\ndefg\n\n\nhijkl\nmn");

        // Can't move up into the first excerpt's header
        assert_eq!(
            up(&snapshot, DisplayPoint::new(2, 2), SelectionGoal::Column(2)).unwrap(),
            (DisplayPoint::new(2, 0), SelectionGoal::Column(0)),
        );
        assert_eq!(
            up(&snapshot, DisplayPoint::new(2, 0), SelectionGoal::None).unwrap(),
            (DisplayPoint::new(2, 0), SelectionGoal::Column(0)),
        );

        // Move up and down within first excerpt
        assert_eq!(
            up(&snapshot, DisplayPoint::new(3, 4), SelectionGoal::Column(4)).unwrap(),
            (DisplayPoint::new(2, 3), SelectionGoal::Column(4)),
        );
        assert_eq!(
            down(&snapshot, DisplayPoint::new(2, 3), SelectionGoal::Column(4)).unwrap(),
            (DisplayPoint::new(3, 4), SelectionGoal::Column(4)),
        );

        // Move up and down across second excerpt's header
        assert_eq!(
            up(&snapshot, DisplayPoint::new(6, 5), SelectionGoal::Column(5)).unwrap(),
            (DisplayPoint::new(3, 4), SelectionGoal::Column(5)),
        );
        assert_eq!(
            down(&snapshot, DisplayPoint::new(3, 4), SelectionGoal::Column(5)).unwrap(),
            (DisplayPoint::new(6, 5), SelectionGoal::Column(5)),
        );

        // Can't move down off the end
        assert_eq!(
            down(&snapshot, DisplayPoint::new(7, 0), SelectionGoal::Column(0)).unwrap(),
            (DisplayPoint::new(7, 2), SelectionGoal::Column(2)),
        );
        assert_eq!(
            down(&snapshot, DisplayPoint::new(7, 2), SelectionGoal::Column(2)).unwrap(),
            (DisplayPoint::new(7, 2), SelectionGoal::Column(2)),
        );
    }

    #[gpui::test]
    fn test_previous_word_start(cx: &mut gpui::MutableAppContext) {
        fn assert(marked_text: &str, cx: &mut gpui::MutableAppContext) {
            let (snapshot, display_points) = marked_snapshot(marked_text, cx);
            dbg!(&display_points);
            assert_eq!(
                previous_word_start(&snapshot, display_points[1]),
                display_points[0]
            );
        }

        assert("\n|   |lorem", cx);
        assert("    |lorem|", cx);
        assert("    |lor|em", cx);
        assert("\nlorem\n|   |ipsum", cx);
        assert("\n\n|\n|", cx);
        assert("    |lorem  |ipsum", cx);
        assert("lorem|-|ipsum", cx);
        assert("lorem|-#$@|ipsum", cx);
        assert("|lorem_|ipsum", cx);
    }

    #[gpui::test]
    fn test_next_word_end(cx: &mut gpui::MutableAppContext) {
        fn assert(marked_text: &str, cx: &mut gpui::MutableAppContext) {
            let (snapshot, display_points) = marked_snapshot(marked_text, cx);
            assert_eq!(
                next_word_end(&snapshot, display_points[0]),
                display_points[1]
            );
        }

        assert("\n|   lorem|", cx);
        assert("    |lorem|", cx);
        assert("    lor|em|", cx);
        assert("    lorem|    |\nipsum\n", cx);
        assert("\n|\n|\n\n", cx);
        assert("lorem|    ipsum|   ", cx);
        assert("lorem|-|ipsum", cx);
        assert("lorem|#$@-|ipsum", cx);
        assert("lorem|_ipsum|", cx);
    }

    // Returns a snapshot from text containing '|' character markers with the markers removed, and DisplayPoints for each one.
    fn marked_snapshot(
        text: &str,
        cx: &mut gpui::MutableAppContext,
    ) -> (DisplaySnapshot, Vec<DisplayPoint>) {
        let mut marked_offsets = Vec::new();
        let chunks = text.split('|');
        let mut text = String::new();

        for chunk in chunks {
            text.push_str(chunk);
            marked_offsets.push(text.len());
        }
        marked_offsets.pop();

        let tab_size = 4;
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;

        let buffer = MultiBuffer::build_simple(&text, cx);
        let display_map = cx
            .add_model(|cx| DisplayMap::new(buffer, tab_size, font_id, font_size, None, 1, 1, cx));
        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
        let marked_display_points = marked_offsets
            .into_iter()
            .map(|offset| offset.to_display_point(&snapshot))
            .collect();

        (snapshot, marked_display_points)
    }

    #[gpui::test]
    fn test_prev_next_word_boundary_multibyte(cx: &mut gpui::MutableAppContext) {
        let tab_size = 4;
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;

        let buffer = MultiBuffer::build_simple("a bcΔ defγ hi—jk", cx);
        let display_map = cx
            .add_model(|cx| DisplayMap::new(buffer, tab_size, font_id, font_size, None, 1, 1, cx));
        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            previous_word_start(&snapshot, DisplayPoint::new(0, 12)),
            DisplayPoint::new(0, 7)
        );
        assert_eq!(
            previous_word_start(&snapshot, DisplayPoint::new(0, 7)),
            DisplayPoint::new(0, 2)
        );
        assert_eq!(
            previous_word_start(&snapshot, DisplayPoint::new(0, 6)),
            DisplayPoint::new(0, 2)
        );
        assert_eq!(
            previous_word_start(&snapshot, DisplayPoint::new(0, 2)),
            DisplayPoint::new(0, 0)
        );
        assert_eq!(
            previous_word_start(&snapshot, DisplayPoint::new(0, 1)),
            DisplayPoint::new(0, 0)
        );

        assert_eq!(
            next_word_end(&snapshot, DisplayPoint::new(0, 0)),
            DisplayPoint::new(0, 1)
        );
        assert_eq!(
            next_word_end(&snapshot, DisplayPoint::new(0, 1)),
            DisplayPoint::new(0, 6)
        );
        assert_eq!(
            next_word_end(&snapshot, DisplayPoint::new(0, 2)),
            DisplayPoint::new(0, 6)
        );
        assert_eq!(
            next_word_end(&snapshot, DisplayPoint::new(0, 6)),
            DisplayPoint::new(0, 12)
        );
        assert_eq!(
            next_word_end(&snapshot, DisplayPoint::new(0, 7)),
            DisplayPoint::new(0, 12)
        );
    }

    #[gpui::test]
    fn test_surrounding_word(cx: &mut gpui::MutableAppContext) {
        let tab_size = 4;
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;
        let buffer = MultiBuffer::build_simple("lorem ipsum   dolor\n    sit\n\n\n\n", cx);
        let display_map = cx
            .add_model(|cx| DisplayMap::new(buffer, tab_size, font_id, font_size, None, 1, 1, cx));
        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));

        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 0)),
            DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 2)),
            DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 5)),
            DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 6)),
            DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 7)),
            DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 11)),
            DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 13)),
            DisplayPoint::new(0, 11)..DisplayPoint::new(0, 14),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 14)),
            DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 17)),
            DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 19)),
            DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 0)),
            DisplayPoint::new(1, 0)..DisplayPoint::new(1, 4),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 1)),
            DisplayPoint::new(1, 0)..DisplayPoint::new(1, 4),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 6)),
            DisplayPoint::new(1, 4)..DisplayPoint::new(1, 7),
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 7)),
            DisplayPoint::new(1, 4)..DisplayPoint::new(1, 7),
        );

        // Don't consider runs of multiple newlines to be a "word"
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(3, 0)),
            DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
        );
    }
}
