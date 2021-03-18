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
