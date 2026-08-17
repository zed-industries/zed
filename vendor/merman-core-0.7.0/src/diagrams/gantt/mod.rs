use crate::{Error, ParseMetadata, Result, utils};
use chrono::{Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Timelike};
use serde_json::{Value, json};
use std::collections::HashMap;

type DateTimeFixed = chrono::DateTime<FixedOffset>;

mod date;
mod datetime;
mod model;
mod parse;

use date::*;
use datetime::*;
use model::*;

pub use model::{GanttDiagramRenderModel, GanttRenderTask};
pub use parse::{parse_gantt, parse_gantt_model_for_render};

fn is_invalid_date(db: &GanttDb, date: DateTimeFixed, date_format: &str) -> bool {
    let formatted = format_dayjs_like(date, date_format);
    let date_only = format_dayjs_like(date, "YYYY-MM-DD");

    if db
        .includes
        .iter()
        .any(|v| v == &formatted || v == &date_only)
    {
        return false;
    }

    if db.excludes.iter().any(|v| v == "weekends") {
        let weekend_start = match db.weekend.as_str() {
            "friday" => 5u32,
            _ => 6u32,
        };
        let local = crate::runtime::datetime_to_local_fixed(date);
        let iso = local.weekday().number_from_monday(); // 1..=7
        if iso == weekend_start || iso == weekend_start + 1 {
            return true;
        }
    }

    let weekday =
        weekday_full_name(crate::runtime::datetime_to_local_fixed(date).weekday()).to_lowercase();
    if db.excludes.iter().any(|v| v == &weekday) {
        return true;
    }

    db.excludes
        .iter()
        .any(|v| v == &formatted || v == &date_only)
}

fn fix_task_dates(
    db: &GanttDb,
    mut start_time: DateTimeFixed,
    mut end_time: DateTimeFixed,
    date_format: &str,
) -> Result<(DateTimeFixed, Option<DateTimeFixed>)> {
    let mut invalid = false;
    let mut render_end_time: Option<DateTimeFixed> = None;
    while start_time <= end_time {
        if !invalid {
            render_end_time = Some(end_time);
        }
        invalid = is_invalid_date(db, start_time, date_format);
        if invalid {
            end_time = add_days_local(end_time, 1).unwrap_or(end_time);
        }
        start_time = add_days_local(start_time, 1).unwrap_or(start_time);
    }
    Ok((end_time, render_end_time))
}

#[cfg(test)]
mod tests;
