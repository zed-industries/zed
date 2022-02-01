use super::{Bias, DisplayPoint, DisplaySnapshot, SelectionGoal, ToDisplayPoint};
use crate::{char_kind, CharKind, ToPoint};
use anyhow::Result;
use std::{cmp, ops::Range};

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
    point: DisplayPoint,
    toggle_indent: bool,
) -> DisplayPoint {
    let (indent, is_blank) = map.line_indent(point.row());
    if toggle_indent && !is_blank && point.column() != indent {
        DisplayPoint::new(point.row(), indent)
    } else {
        DisplayPoint::new(point.row(), 0)
    }
}

pub fn line_end(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let line_end = DisplayPoint::new(point.row(), map.line_len(point.row()));
    map.clip_point(line_end, Bias::Left)
}

pub fn prev_word_boundary(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
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
    let mut prev_char_kind = CharKind::Newline;
    for c in map.chars_at(DisplayPoint::new(point.row(), 0)) {
        if column >= point.column() {
            break;
        }

        let char_kind = char_kind(c);
        if char_kind != prev_char_kind
            && char_kind != CharKind::Whitespace
            && char_kind != CharKind::Newline
        {
            *boundary.column_mut() = column;
        }

        prev_char_kind = char_kind;
        column += c.len_utf8() as u32;
    }
    boundary
}

pub fn next_word_boundary(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    let mut prev_char_kind = None;
    for c in map.chars_at(point) {
        let char_kind = char_kind(c);
        if let Some(prev_char_kind) = prev_char_kind {
            if c == '\n' {
                break;
            }
            if prev_char_kind != char_kind
                && prev_char_kind != CharKind::Whitespace
                && prev_char_kind != CharKind::Newline
            {
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

pub fn surrounding_word(
    map: &DisplaySnapshot,
    point: DisplayPoint,
) -> (Range<DisplayPoint>, Option<CharKind>) {
    let mut start = map.clip_point(point, Bias::Left).to_offset(map, Bias::Left);
    let mut end = start;

    let text = &map.buffer_snapshot;
    let mut next_chars = text.chars_at(start).peekable();
    let mut prev_chars = text.reversed_chars_at(start).peekable();
    let word_kind = cmp::max(
        prev_chars.peek().copied().map(char_kind),
        next_chars.peek().copied().map(char_kind),
    );

    for ch in prev_chars {
        if Some(char_kind(ch)) == word_kind {
            start -= ch.len_utf8();
        } else {
            break;
        }
    }

    for ch in next_chars {
        if Some(char_kind(ch)) == word_kind {
            end += ch.len_utf8();
        } else {
            break;
        }
    }

    (
        start.to_point(&map.buffer_snapshot).to_display_point(map)
            ..end.to_point(&map.buffer_snapshot).to_display_point(map),
        word_kind,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::{BlockDisposition, BlockProperties},
        Buffer, DisplayMap, ExcerptProperties, MultiBuffer,
    };
    use gpui::{elements::Empty, Element};
    use language::Point;
    use std::sync::Arc;

    #[gpui::test]
    fn test_move_up_and_down_with_excerpts(cx: &mut gpui::MutableAppContext) {
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();

        let buffer = cx.add_model(|cx| Buffer::new(0, "abc\ndefg\nhijkl\nmn", cx));
        let mut excerpt1_header_position = None;
        let mut excerpt2_header_position = None;
        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            let excerpt1_id = multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer,
                    range: Point::new(0, 0)..Point::new(1, 4),
                },
                cx,
            );
            let excerpt2_id = multibuffer.push_excerpt(
                ExcerptProperties {
                    buffer: &buffer,
                    range: Point::new(2, 0)..Point::new(3, 2),
                },
                cx,
            );

            excerpt1_header_position = Some(
                multibuffer
                    .read(cx)
                    .anchor_in_excerpt(excerpt1_id, language::Anchor::min()),
            );
            excerpt2_header_position = Some(
                multibuffer
                    .read(cx)
                    .anchor_in_excerpt(excerpt2_id, language::Anchor::min()),
            );
            multibuffer
        });

        let display_map =
            cx.add_model(|cx| DisplayMap::new(multibuffer, 2, font_id, 14.0, None, cx));
        display_map.update(cx, |display_map, cx| {
            display_map.insert_blocks(
                [
                    BlockProperties {
                        position: excerpt1_header_position.unwrap(),
                        height: 2,
                        render: Arc::new(|_| Empty::new().boxed()),
                        disposition: BlockDisposition::Above,
                    },
                    BlockProperties {
                        position: excerpt2_header_position.unwrap(),
                        height: 3,
                        render: Arc::new(|_| Empty::new().boxed()),
                        disposition: BlockDisposition::Above,
                    },
                ],
                cx,
            )
        });

        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(snapshot.text(), "\n\nabc\ndefg\n\n\n\nhijkl\nmn");

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
            up(&snapshot, DisplayPoint::new(7, 5), SelectionGoal::Column(5)).unwrap(),
            (DisplayPoint::new(3, 4), SelectionGoal::Column(5)),
        );
        assert_eq!(
            down(&snapshot, DisplayPoint::new(3, 4), SelectionGoal::Column(5)).unwrap(),
            (DisplayPoint::new(7, 5), SelectionGoal::Column(5)),
        );

        // Can't move down off the end
        assert_eq!(
            down(&snapshot, DisplayPoint::new(8, 0), SelectionGoal::Column(0)).unwrap(),
            (DisplayPoint::new(8, 2), SelectionGoal::Column(2)),
        );
        assert_eq!(
            down(&snapshot, DisplayPoint::new(8, 2), SelectionGoal::Column(2)).unwrap(),
            (DisplayPoint::new(8, 2), SelectionGoal::Column(2)),
        );
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
        let display_map =
            cx.add_model(|cx| DisplayMap::new(buffer, tab_size, font_id, font_size, None, cx));
        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            prev_word_boundary(&snapshot, DisplayPoint::new(0, 12)),
            DisplayPoint::new(0, 7)
        );
        assert_eq!(
            prev_word_boundary(&snapshot, DisplayPoint::new(0, 7)),
            DisplayPoint::new(0, 2)
        );
        assert_eq!(
            prev_word_boundary(&snapshot, DisplayPoint::new(0, 6)),
            DisplayPoint::new(0, 2)
        );
        assert_eq!(
            prev_word_boundary(&snapshot, DisplayPoint::new(0, 2)),
            DisplayPoint::new(0, 0)
        );
        assert_eq!(
            prev_word_boundary(&snapshot, DisplayPoint::new(0, 1)),
            DisplayPoint::new(0, 0)
        );

        assert_eq!(
            next_word_boundary(&snapshot, DisplayPoint::new(0, 0)),
            DisplayPoint::new(0, 1)
        );
        assert_eq!(
            next_word_boundary(&snapshot, DisplayPoint::new(0, 1)),
            DisplayPoint::new(0, 6)
        );
        assert_eq!(
            next_word_boundary(&snapshot, DisplayPoint::new(0, 2)),
            DisplayPoint::new(0, 6)
        );
        assert_eq!(
            next_word_boundary(&snapshot, DisplayPoint::new(0, 6)),
            DisplayPoint::new(0, 12)
        );
        assert_eq!(
            next_word_boundary(&snapshot, DisplayPoint::new(0, 7)),
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
        let buffer = MultiBuffer::build_simple("lorem ipsum   dolor\n    sit", cx);
        let display_map =
            cx.add_model(|cx| DisplayMap::new(buffer, tab_size, font_id, font_size, None, cx));
        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));

        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 0)),
            (
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 2)),
            (
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 5)),
            (
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 6)),
            (
                DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 7)),
            (
                DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 11)),
            (
                DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 13)),
            (
                DisplayPoint::new(0, 11)..DisplayPoint::new(0, 14),
                Some(CharKind::Whitespace)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 14)),
            (
                DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 17)),
            (
                DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 19)),
            (
                DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 0)),
            (
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 4),
                Some(CharKind::Whitespace)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 1)),
            (
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 4),
                Some(CharKind::Whitespace)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 6)),
            (
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 7),
                Some(CharKind::Word)
            )
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 7)),
            (
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 7),
                Some(CharKind::Word)
            )
        );
    }
}
