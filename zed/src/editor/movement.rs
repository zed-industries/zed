use super::{Bias, DisplayMap, DisplayPoint, SelectionGoal};
use anyhow::Result;
use gpui::AppContext;

pub fn left(map: &DisplayMap, mut point: DisplayPoint, app: &AppContext) -> Result<DisplayPoint> {
    if point.column() > 0 {
        *point.column_mut() -= 1;
    } else if point.row() > 0 {
        *point.row_mut() -= 1;
        *point.column_mut() = map.line_len(point.row(), app);
    }
    Ok(map.snapshot(app).clip_point(point, Bias::Left))
}

pub fn right(map: &DisplayMap, mut point: DisplayPoint, app: &AppContext) -> Result<DisplayPoint> {
    let max_column = map.line_len(point.row(), app);
    if point.column() < max_column {
        *point.column_mut() += 1;
    } else if point.row() < map.max_point(app).row() {
        *point.row_mut() += 1;
        *point.column_mut() = 0;
    }
    Ok(map.snapshot(app).clip_point(point, Bias::Right))
}

pub fn up(
    map: &DisplayMap,
    mut point: DisplayPoint,
    goal: SelectionGoal,
    app: &AppContext,
) -> Result<(DisplayPoint, SelectionGoal)> {
    let map = map.snapshot(app);
    let goal_column = if let SelectionGoal::Column(column) = goal {
        column
    } else {
        map.column_to_chars(point.row(), point.column())
    };

    if point.row() > 0 {
        *point.row_mut() -= 1;
        *point.column_mut() = map.column_from_chars(point.row(), goal_column);
    } else {
        point = DisplayPoint::new(0, 0);
    }

    Ok((point, SelectionGoal::Column(goal_column)))
}

pub fn down(
    map: &DisplayMap,
    mut point: DisplayPoint,
    goal: SelectionGoal,
    app: &AppContext,
) -> Result<(DisplayPoint, SelectionGoal)> {
    let max_point = map.max_point(app);
    let map = map.snapshot(app);
    let goal_column = if let SelectionGoal::Column(column) = goal {
        column
    } else {
        map.column_to_chars(point.row(), point.column())
    };

    if point.row() < max_point.row() {
        *point.row_mut() += 1;
        *point.column_mut() = map.column_from_chars(point.row(), goal_column);
    } else {
        point = max_point;
    }

    Ok((point, SelectionGoal::Column(goal_column)))
}

pub fn line_beginning(
    map: &DisplayMap,
    point: DisplayPoint,
    toggle_indent: bool,
    app: &AppContext,
) -> Result<DisplayPoint> {
    let (indent, is_blank) = map.line_indent(point.row(), app);
    if toggle_indent && !is_blank && point.column() != indent {
        Ok(DisplayPoint::new(point.row(), indent))
    } else {
        Ok(DisplayPoint::new(point.row(), 0))
    }
}

pub fn line_end(map: &DisplayMap, point: DisplayPoint, app: &AppContext) -> Result<DisplayPoint> {
    Ok(DisplayPoint::new(
        point.row(),
        map.line_len(point.row(), app),
    ))
}

pub fn prev_word_boundary(
    map: &DisplayMap,
    point: DisplayPoint,
    app: &AppContext,
) -> Result<DisplayPoint> {
    if point.column() == 0 {
        if point.row() == 0 {
            Ok(DisplayPoint::new(0, 0))
        } else {
            let row = point.row() - 1;
            Ok(DisplayPoint::new(row, map.line_len(row, app)))
        }
    } else {
        let mut boundary = DisplayPoint::new(point.row(), 0);
        let mut column = 0;
        let mut prev_c = None;
        for c in map.snapshot(app).chars_at(boundary) {
            if column >= point.column() {
                break;
            }

            if prev_c.is_none() || char_kind(prev_c.unwrap()) != char_kind(c) {
                *boundary.column_mut() = column;
            }

            prev_c = Some(c);
            column += c.len_utf8() as u32;
        }
        Ok(boundary)
    }
}

pub fn next_word_boundary(
    map: &DisplayMap,
    mut point: DisplayPoint,
    app: &AppContext,
) -> Result<DisplayPoint> {
    let mut prev_c = None;
    for c in map.snapshot(app).chars_at(point) {
        if prev_c.is_some() && (c == '\n' || char_kind(prev_c.unwrap()) != char_kind(c)) {
            break;
        }

        if c == '\n' {
            *point.row_mut() += 1;
            *point.column_mut() = 0;
        } else {
            *point.column_mut() += c.len_utf8() as u32;
        }
        prev_c = Some(c);
    }
    Ok(point)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum CharKind {
    Newline,
    Whitespace,
    Punctuation,
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
