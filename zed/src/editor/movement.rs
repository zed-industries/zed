use super::{DisplayMap, DisplayPoint};
use anyhow::Result;
use gpui::AppContext;
use std::cmp;

pub fn left(map: &DisplayMap, mut point: DisplayPoint, app: &AppContext) -> Result<DisplayPoint> {
    if point.column() > 0 {
        *point.column_mut() -= 1;
    } else if point.row() > 0 {
        *point.row_mut() -= 1;
        *point.column_mut() = map.line_len(point.row(), app)?;
    }
    Ok(point)
}

pub fn right(map: &DisplayMap, mut point: DisplayPoint, app: &AppContext) -> Result<DisplayPoint> {
    let max_column = map.line_len(point.row(), app).unwrap();
    if point.column() < max_column {
        *point.column_mut() += 1;
    } else if point.row() < map.max_point(app).row() {
        *point.row_mut() += 1;
        *point.column_mut() = 0;
    }
    Ok(point)
}

pub fn up(
    map: &DisplayMap,
    mut point: DisplayPoint,
    goal_column: Option<u32>,
    app: &AppContext,
) -> Result<(DisplayPoint, Option<u32>)> {
    let goal_column = goal_column.or(Some(point.column()));
    if point.row() > 0 {
        *point.row_mut() -= 1;
        *point.column_mut() = cmp::min(goal_column.unwrap(), map.line_len(point.row(), app)?);
    } else {
        point = DisplayPoint::new(0, 0);
    }

    Ok((point, goal_column))
}

pub fn down(
    map: &DisplayMap,
    mut point: DisplayPoint,
    goal_column: Option<u32>,
    app: &AppContext,
) -> Result<(DisplayPoint, Option<u32>)> {
    let goal_column = goal_column.or(Some(point.column()));
    let max_point = map.max_point(app);
    if point.row() < max_point.row() {
        *point.row_mut() += 1;
        *point.column_mut() = cmp::min(goal_column.unwrap(), map.line_len(point.row(), app)?)
    } else {
        point = max_point;
    }

    Ok((point, goal_column))
}

pub fn line_beginning(
    map: &DisplayMap,
    point: DisplayPoint,
    toggle_indent: bool,
    app: &AppContext,
) -> Result<DisplayPoint> {
    let (indent, is_blank) = map.line_indent(point.row(), app)?;
    if toggle_indent && !is_blank && point.column() != indent {
        Ok(DisplayPoint::new(point.row(), indent))
    } else {
        Ok(DisplayPoint::new(point.row(), 0))
    }
}

pub fn line_end(map: &DisplayMap, point: DisplayPoint, app: &AppContext) -> Result<DisplayPoint> {
    Ok(DisplayPoint::new(
        point.row(),
        map.line_len(point.row(), app)?,
    ))
}

pub fn prev_word_boundary(
    map: &DisplayMap,
    point: DisplayPoint,
    app: &AppContext,
) -> Result<DisplayPoint> {
    todo!()
}

pub fn next_word_boundary(
    map: &DisplayMap,
    mut point: DisplayPoint,
    app: &AppContext,
) -> Result<DisplayPoint> {
    let mut prev_c = None;
    for c in map.chars_at(point, app)? {
        if prev_c.is_some() && (c == '\n' || is_word_char(prev_c.unwrap()) != is_word_char(c)) {
            break;
        }

        if c == '\n' {
            *point.row_mut() += 1;
            *point.column_mut() = 0;
        } else {
            *point.column_mut() += 1;
        }
        prev_c = Some(c);
    }
    Ok(point)
}

fn is_word_char(c: char) -> bool {
    match c {
        '/' | '\\' | '(' | ')' | '"' | '\'' | ':' | ',' | '.' | ';' | '<' | '>' | '~' | '!'
        | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '|' | '+' | '=' | '[' | ']' | '{' | '}'
        | '`' | '?' | '-' | '…' | ' ' | '\n' => false,
        _ => true,
    }
}
