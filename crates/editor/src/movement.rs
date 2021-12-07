use super::{Bias, DisplayMapSnapshot, DisplayPoint, SelectionGoal, ToDisplayPoint};
use anyhow::Result;
use language::traits::{DocumentSnapshot, ToDocumentPoint};
use std::{cmp, ops::Range};

pub fn left<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    mut point: DisplayPoint,
) -> Result<DisplayPoint> {
    if point.column() > 0 {
        *point.column_mut() -= 1;
    } else if point.row() > 0 {
        *point.row_mut() -= 1;
        *point.column_mut() = map.line_len(point.row());
    }
    Ok(map.clip_point(point, Bias::Left))
}

pub fn right<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    mut point: DisplayPoint,
) -> Result<DisplayPoint> {
    let max_column = map.line_len(point.row());
    if point.column() < max_column {
        *point.column_mut() += 1;
    } else if point.row() < map.max_point().row() {
        *point.row_mut() += 1;
        *point.column_mut() = 0;
    }
    Ok(map.clip_point(point, Bias::Right))
}

pub fn up<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    mut point: DisplayPoint,
    goal: SelectionGoal,
) -> Result<(DisplayPoint, SelectionGoal)> {
    let goal_column = if let SelectionGoal::Column(column) = goal {
        column
    } else {
        map.column_to_chars(point.row(), point.column())
    };

    loop {
        if point.row() > 0 {
            *point.row_mut() -= 1;
            *point.column_mut() = map.column_from_chars(point.row(), goal_column);
            if !map.is_block_line(point.row()) {
                break;
            }
        } else {
            point = DisplayPoint::new(0, 0);
            break;
        }
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

pub fn down<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    mut point: DisplayPoint,
    goal: SelectionGoal,
) -> Result<(DisplayPoint, SelectionGoal)> {
    let max_point = map.max_point();
    let goal_column = if let SelectionGoal::Column(column) = goal {
        column
    } else {
        map.column_to_chars(point.row(), point.column())
    };

    loop {
        if point.row() < max_point.row() {
            *point.row_mut() += 1;
            *point.column_mut() = map.column_from_chars(point.row(), goal_column);
            if !map.is_block_line(point.row()) {
                break;
            }
        } else {
            point = max_point;
            break;
        }
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

pub fn line_beginning<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
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

pub fn line_end<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    point: DisplayPoint,
) -> DisplayPoint {
    let line_end = DisplayPoint::new(point.row(), map.line_len(point.row()));
    map.clip_point(line_end, Bias::Left)
}

pub fn prev_word_boundary<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    mut point: DisplayPoint,
) -> DisplayPoint {
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

pub fn next_word_boundary<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    mut point: DisplayPoint,
) -> DisplayPoint {
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
    point
}

pub fn is_inside_word<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    point: DisplayPoint,
) -> bool {
    let ix = map.clip_point(point, Bias::Left).to_offset(map, Bias::Left);
    let text = &map.buffer_snapshot;
    let next_char_kind = text.chars_at(ix).next().map(char_kind);
    let prev_char_kind = text.reversed_chars_at(ix).next().map(char_kind);
    prev_char_kind.zip(next_char_kind) == Some((CharKind::Word, CharKind::Word))
}

pub fn surrounding_word<S: DocumentSnapshot>(
    map: &DisplayMapSnapshot<S>,
    point: DisplayPoint,
) -> Range<DisplayPoint> {
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

    start.to_point(&map.buffer_snapshot).to_display_point(map)
        ..end.to_point(&map.buffer_snapshot).to_display_point(map)
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum CharKind {
    Newline,
    Punctuation,
    Whitespace,
    Word,
}

fn char_kind(c: char) -> CharKind {
    if c == '\n' {
        CharKind::Newline
    } else if c.is_whitespace() {
        CharKind::Whitespace
    } else if c.is_alphanumeric() || c == '_' {
        CharKind::Word
    } else {
        CharKind::Punctuation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display_map::DisplayMap;
    use language::buffer::Buffer;

    #[gpui::test]
    fn test_prev_next_word_boundary_multibyte(cx: &mut gpui::MutableAppContext) {
        let tab_size = 4;
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;

        let buffer = cx.add_model(|cx| Buffer::new(0, "a bcΔ defγ hi—jk", cx));
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
        let buffer = cx.add_model(|cx| Buffer::new(0, "lorem ipsum   dolor\n    sit", cx));
        let display_map =
            cx.add_model(|cx| DisplayMap::new(buffer, tab_size, font_id, font_size, None, cx));
        let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));

        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 0)),
            DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 2)),
            DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 5)),
            DisplayPoint::new(0, 0)..DisplayPoint::new(0, 5)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 6)),
            DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 7)),
            DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 11)),
            DisplayPoint::new(0, 6)..DisplayPoint::new(0, 11)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 13)),
            DisplayPoint::new(0, 11)..DisplayPoint::new(0, 14)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 14)),
            DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 17)),
            DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(0, 19)),
            DisplayPoint::new(0, 14)..DisplayPoint::new(0, 19)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 0)),
            DisplayPoint::new(1, 0)..DisplayPoint::new(1, 4)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 1)),
            DisplayPoint::new(1, 0)..DisplayPoint::new(1, 4)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 6)),
            DisplayPoint::new(1, 4)..DisplayPoint::new(1, 7)
        );
        assert_eq!(
            surrounding_word(&snapshot, DisplayPoint::new(1, 7)),
            DisplayPoint::new(1, 4)..DisplayPoint::new(1, 7)
        );
    }
}
