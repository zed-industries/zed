use crate::config::{config_bool as cfg_bool, config_f64 as cfg_f64};
use crate::json::from_value_ref;
use crate::model::{
    Bounds, GanttAxisTickLayout, GanttDiagramLayout, GanttExcludeRangeLayout, GanttRowLayout,
    GanttSectionTitleLayout, GanttTaskBarLayout, GanttTaskLabelLayout, GanttTaskLayout,
};
use crate::text::{DeterministicTextMeasurer, TextMeasurer, TextStyle};
use crate::{Error, Result};
use chrono::{Datelike, FixedOffset, Timelike};
use std::collections::{HashMap, hash_map::Entry};

use merman_core::diagrams::gantt::{GanttDiagramRenderModel, GanttRenderTask};

// Mermaid's gantt renderer derives the width from the parent element's `offsetWidth`.
// In Mermaid CLI (and typical browser defaults), the body margin results in an effective
// width of 1184px for a 1200px viewport, which matches our upstream SVG baselines.
const DEFAULT_WIDTH: f64 = 1184.0;
const MS_PER_DAY: i64 = 86_400_000;

fn dt_utc_to_local_fixed(dt_utc: chrono::DateTime<chrono::Utc>) -> chrono::DateTime<FixedOffset> {
    merman_core::time::datetime_to_local_fixed(
        dt_utc.with_timezone(&merman_core::time::utc_fixed_offset()),
    )
}

fn cfg_i64(cfg: &serde_json::Value, path: &[&str]) -> Option<i64> {
    let mut cur = cfg;
    for k in path {
        cur = cur.get(*k)?;
    }
    cur.as_i64()
}

fn month_name_short(m: u32) -> &'static str {
    match m {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

fn month_name_long(m: u32) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

fn weekday_name_short(w: chrono::Weekday) -> &'static str {
    match w {
        chrono::Weekday::Mon => "Mon",
        chrono::Weekday::Tue => "Tue",
        chrono::Weekday::Wed => "Wed",
        chrono::Weekday::Thu => "Thu",
        chrono::Weekday::Fri => "Fri",
        chrono::Weekday::Sat => "Sat",
        chrono::Weekday::Sun => "Sun",
    }
}

fn weekday_name_long(w: chrono::Weekday) -> &'static str {
    match w {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    }
}

fn ordinal_suffix(n: u32) -> &'static str {
    let nn = n % 100;
    if (11..=13).contains(&nn) {
        return "th";
    }
    match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

fn format_dayjs_like(ms: i64, fmt: &str) -> Option<String> {
    let dt_utc = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms)?;
    let dt = dt_utc_to_local_fixed(dt_utc);
    let fmt = fmt.trim();

    let mut out = String::new();
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '[' {
            i += 1;
            while i < chars.len() && chars[i] != ']' {
                out.push(chars[i]);
                i += 1;
            }
            if i < chars.len() && chars[i] == ']' {
                i += 1;
            }
            continue;
        }

        let rest: String = chars[i..].iter().collect();
        let token = [
            "YYYY", "MMMM", "MMM", "dddd", "ddd", "YY", "MM", "DD", "Do", "HH", "hh", "mm", "ss",
            "SSS", "ZZ", "Z", "A", "a", "x", "X", "M", "D", "H", "h", "m", "s",
        ]
        .into_iter()
        .find(|t| rest.starts_with(t));

        if let Some(t) = token {
            match t {
                "YYYY" => out.push_str(&format!("{:04}", dt.year())),
                "YY" => out.push_str(&format!("{:02}", (dt.year() % 100).abs())),
                "MMMM" => out.push_str(month_name_long(dt.month())),
                "MMM" => out.push_str(month_name_short(dt.month())),
                "MM" => out.push_str(&format!("{:02}", dt.month())),
                "M" => out.push_str(&format!("{}", dt.month())),
                "DD" => out.push_str(&format!("{:02}", dt.day())),
                "D" => out.push_str(&format!("{}", dt.day())),
                "Do" => out.push_str(&format!("{}{}", dt.day(), ordinal_suffix(dt.day()))),
                "dddd" => out.push_str(weekday_name_long(dt.weekday())),
                "ddd" => out.push_str(weekday_name_short(dt.weekday())),
                "HH" => out.push_str(&format!("{:02}", dt.hour())),
                "H" => out.push_str(&format!("{}", dt.hour())),
                "hh" => {
                    let h = dt.hour() % 12;
                    let h = if h == 0 { 12 } else { h };
                    out.push_str(&format!("{:02}", h));
                }
                "h" => {
                    let h = dt.hour() % 12;
                    let h = if h == 0 { 12 } else { h };
                    out.push_str(&format!("{}", h));
                }
                "mm" => out.push_str(&format!("{:02}", dt.minute())),
                "m" => out.push_str(&format!("{}", dt.minute())),
                "ss" => out.push_str(&format!("{:02}", dt.second())),
                "s" => out.push_str(&format!("{}", dt.second())),
                "SSS" => out.push_str(&format!("{:03}", dt.timestamp_subsec_millis())),
                "A" => out.push_str(if dt.hour() < 12 { "AM" } else { "PM" }),
                "a" => out.push_str(if dt.hour() < 12 { "am" } else { "pm" }),
                "Z" => {
                    let off = dt.offset().local_minus_utc();
                    let sign = if off >= 0 { '+' } else { '-' };
                    let off = off.abs();
                    let hh = off / 3600;
                    let mm = (off % 3600) / 60;
                    out.push_str(&format!("{sign}{:02}:{:02}", hh, mm));
                }
                "ZZ" => {
                    let off = dt.offset().local_minus_utc();
                    let sign = if off >= 0 { '+' } else { '-' };
                    let off = off.abs();
                    let hh = off / 3600;
                    let mm = (off % 3600) / 60;
                    out.push_str(&format!("{sign}{:02}{:02}", hh, mm));
                }
                "x" => out.push_str(&format!("{ms}")),
                "X" => out.push_str(&format!("{}", ms / 1000)),
                _ => {}
            }
            i += t.len();
            continue;
        }

        out.push(c);
        i += 1;
    }

    Some(out)
}

fn format_yyyy_mm_dd(ms: i64) -> Option<String> {
    format_dayjs_like(ms, "YYYY-MM-DD")
}

fn weekend_start_day(weekend: &str) -> u32 {
    match weekend {
        "friday" => 5,
        _ => 6,
    }
}

fn is_invalid_date(
    ms: i64,
    date_format: &str,
    excludes: &[String],
    includes: &[String],
    weekend: &str,
) -> bool {
    let Some(formatted_date) = format_dayjs_like(ms, date_format) else {
        return false;
    };
    let Some(date_only) = format_yyyy_mm_dd(ms) else {
        return false;
    };

    if includes
        .iter()
        .any(|t| t == &formatted_date || t == &date_only)
    {
        return false;
    }

    let Some(dt_utc) = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms) else {
        return false;
    };
    let dt = dt_utc_to_local_fixed(dt_utc);
    let iso_weekday = dt.weekday().number_from_monday();

    if excludes.iter().any(|t| t == "weekends") {
        let start = weekend_start_day(weekend);
        if iso_weekday == start || iso_weekday == start + 1 {
            return true;
        }
    }

    let weekday_lower = weekday_name_long(dt.weekday()).to_lowercase();
    if excludes.iter().any(|t| t == &weekday_lower) {
        return true;
    }

    excludes
        .iter()
        .any(|t| t == &formatted_date || t == &date_only)
}

fn start_of_day_ms(ms: i64) -> Option<i64> {
    let dt_utc = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms)?;
    let dt = dt_utc_to_local_fixed(dt_utc);
    let d = dt.date_naive();
    let local_midnight = merman_core::time::datetime_from_naive_local(d.and_hms_opt(0, 0, 0)?);
    Some(local_midnight.timestamp_millis())
}

fn end_of_day_ms(ms: i64) -> Option<i64> {
    let start = start_of_day_ms(ms)?;
    Some(start + MS_PER_DAY - 1)
}

fn scale_time(ms: i64, min_ms: i64, max_ms: i64, range: f64) -> f64 {
    if max_ms <= min_ms {
        // D3 scaleTime returns the midpoint of the range for degenerate domains.
        // This matters for fixtures where parsing fails and `startTime == endTime` (width=0).
        return (range / 2.0).round();
    }
    let t = (ms - min_ms) as f64 / (max_ms - min_ms) as f64;
    (t * range).round()
}

fn collect_categories(tasks: &[GanttRenderTask]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for t in tasks {
        if !out.iter().any(|x| x == &t.task_type) {
            out.push(t.task_type.clone());
        }
    }
    out
}

fn get_max_intersections(tasks: &mut [GanttRenderTask], order_offset: i64) -> i64 {
    let mut timeline: Vec<i64> = vec![i64::MIN; tasks.len()];
    let mut sorted: Vec<usize> = (0..tasks.len()).collect();
    sorted.sort_by(|&a, &b| {
        let ta = tasks[a].start_ms;
        let tb = tasks[b].start_ms;
        ta.cmp(&tb)
            .then_with(|| tasks[a].order.cmp(&tasks[b].order))
    });

    let mut max_i: i64 = 0;
    for idx in sorted {
        for (j, slot) in timeline.iter_mut().enumerate() {
            if tasks[idx].start_ms >= *slot {
                *slot = tasks[idx].end_ms;
                tasks[idx].order = j as i64 + order_offset;
                max_i = max_i.max(j as i64);
                break;
            }
        }
    }
    max_i
}

fn tick_step(start: f64, stop: f64, count: f64) -> i64 {
    if !start.is_finite() || !stop.is_finite() || !count.is_finite() || count <= 0.0 {
        return 1;
    }
    let span = (stop - start).abs();
    if span <= 0.0 {
        return 1;
    }
    let step0 = span / count;
    let power = 10f64.powf(step0.log10().floor());
    let error = step0 / power;
    let factor = if error >= 7.5 {
        10.0
    } else if error >= 3.5 {
        5.0
    } else if error >= 1.5 {
        2.0
    } else {
        1.0
    };
    (factor * power).round().max(1.0) as i64
}

fn auto_tick_interval(min_ms: i64, max_ms: i64) -> (i64, &'static str) {
    // Matches the shape of d3-time's default tick interval selection (used by Mermaid when no
    // custom tickInterval is specified). The key properties we need for SVG DOM parity are:
    // - choosing from the same "nice" interval set (e.g. 1h/3h/6h/12h, not 2h)
    // - aligning ticks to interval boundaries (handled in build_ticks)
    const TARGET_TICKS: f64 = 10.0;
    const MS: f64 = 1.0;
    const SEC: f64 = 1_000.0;
    const MIN: f64 = 60_000.0;
    const HOUR: f64 = 3_600_000.0;
    const DAY: f64 = MS_PER_DAY as f64;
    const WEEK: f64 = (MS_PER_DAY * 7) as f64;
    const MONTH: f64 = (MS_PER_DAY * 30) as f64;
    const YEAR: f64 = (MS_PER_DAY * 365) as f64;

    let span_ms = (max_ms - min_ms).abs().max(1) as f64;
    let target = span_ms / TARGET_TICKS;

    let mut intervals: Vec<(f64, i64, &'static str)> = Vec::new();
    for (every, unit_ms) in [
        (1, MS),
        (2, MS),
        (5, MS),
        (10, MS),
        (20, MS),
        (50, MS),
        (100, MS),
        (200, MS),
        (500, MS),
    ] {
        intervals.push((unit_ms * every as f64, every, "millisecond"));
    }
    for (every, unit_ms, unit) in [
        (1, SEC, "second"),
        (5, SEC, "second"),
        (15, SEC, "second"),
        (30, SEC, "second"),
        (1, MIN, "minute"),
        (5, MIN, "minute"),
        (15, MIN, "minute"),
        (30, MIN, "minute"),
        (1, HOUR, "hour"),
        (3, HOUR, "hour"),
        (6, HOUR, "hour"),
        (12, HOUR, "hour"),
        (1, DAY, "day"),
        (2, DAY, "day"),
        (1, WEEK, "week"),
        (1, MONTH, "month"),
        (3, MONTH, "month"),
        (1, YEAR, "year"),
    ] {
        intervals.push((unit_ms * (every as f64), every, unit));
    }
    intervals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut i = 0usize;
    while i < intervals.len() && intervals[i].0 < target {
        i += 1;
    }

    if i == 0 {
        let (_dur, every, unit) = intervals[0];
        return (every, unit);
    }

    if i >= intervals.len() {
        let years = tick_step(min_ms as f64 / YEAR, max_ms as f64 / YEAR, TARGET_TICKS);
        return (years, "year");
    }

    let (d0, e0, u0) = intervals[i - 1];
    let (d1, e1, u1) = intervals[i];
    if target / d0 < d1 / target {
        (e0, u0)
    } else {
        (e1, u1)
    }
}

fn parse_tick_interval(s: &str) -> Option<(i64, &str)> {
    let s = s.trim();
    let mut num = String::new();
    let mut idx = 0;
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num.push(ch);
            idx += 1;
        } else {
            break;
        }
    }
    let every = num.parse::<i64>().ok()?;
    if every <= 0 {
        return None;
    }
    let unit = &s[idx..];
    match unit {
        "millisecond" | "second" | "minute" | "hour" | "day" | "week" | "month" => {
            Some((every, unit))
        }
        _ => None,
    }
}

fn add_interval(ms: i64, every: i64, unit: &str) -> Option<i64> {
    let dt_utc = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms)?;
    let dt = dt_utc_to_local_fixed(dt_utc);
    let naive = dt.naive_local();

    let next = match unit {
        "millisecond" => naive + chrono::Duration::milliseconds(every),
        "second" => naive + chrono::Duration::seconds(every),
        "minute" => naive + chrono::Duration::minutes(every),
        "hour" => naive + chrono::Duration::hours(every),
        "day" => naive + chrono::Duration::days(every),
        "week" => naive + chrono::Duration::days(every * 7),
        "month" => {
            let mut y = naive.date().year();
            let mut m = naive.date().month() as i32 + every as i32;
            while m > 12 {
                y += 1;
                m -= 12;
            }
            while m < 1 {
                y -= 1;
                m += 12;
            }
            let d = naive.date().day().min(28);
            let date = chrono::NaiveDate::from_ymd_opt(y, m as u32, d)?;
            date.and_hms_opt(
                naive.time().hour(),
                naive.time().minute(),
                naive.time().second(),
            )?
        }
        "year" => {
            let y = naive.date().year() + every as i32;
            let m = naive.date().month();
            let d = naive.date().day().min(28);
            let date = chrono::NaiveDate::from_ymd_opt(y, m, d)?;
            date.and_hms_opt(
                naive.time().hour(),
                naive.time().minute(),
                naive.time().second(),
            )?
        }
        _ => return None,
    };

    let out = merman_core::time::datetime_from_naive_local(next);
    Some(out.timestamp_millis())
}

fn weekday_from_str(s: &str) -> Option<chrono::Weekday> {
    match s.trim().to_ascii_lowercase().as_str() {
        "monday" => Some(chrono::Weekday::Mon),
        "tuesday" => Some(chrono::Weekday::Tue),
        "wednesday" => Some(chrono::Weekday::Wed),
        "thursday" => Some(chrono::Weekday::Thu),
        "friday" => Some(chrono::Weekday::Fri),
        "saturday" => Some(chrono::Weekday::Sat),
        "sunday" => Some(chrono::Weekday::Sun),
        _ => None,
    }
}

fn ceil_tick_start(min_ms: i64, every: i64, unit: &str, week_start: Option<&str>) -> Option<i64> {
    let dt_utc = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(min_ms)?;
    let dt = dt_utc_to_local_fixed(dt_utc);
    let naive = dt.naive_local();

    let start = match unit {
        "millisecond" => {
            let e = every.max(1);
            // D3's `millisecond.every(e)` aligns using `Math.floor(date / e) * e`, and `range`
            // starts at `ceil(start)`. Use Euclidean division so negative timestamps match D3.
            let q = min_ms.div_euclid(e);
            let r = min_ms.rem_euclid(e);
            let aligned = if r == 0 { q * e } else { (q + 1) * e };
            return Some(aligned);
        }
        "second" => {
            let base = naive.date().and_hms_opt(
                naive.time().hour(),
                naive.time().minute(),
                naive.time().second(),
            )?;
            let mut cur = base;
            if cur < naive {
                cur += chrono::Duration::seconds(1);
            }
            let e = every.max(1);
            loop {
                let sec = cur.time().second() as i64;
                let rem = (sec % e + e) % e;
                if rem == 0 {
                    break;
                }
                cur += chrono::Duration::seconds(1);
            }
            cur
        }
        "minute" => {
            let base = naive
                .date()
                .and_hms_opt(naive.time().hour(), naive.time().minute(), 0)?;
            let mut cur = base;
            if cur < naive {
                cur += chrono::Duration::minutes(1);
            }
            let e = every.max(1);
            loop {
                let min = cur.time().minute() as i64;
                let rem = (min % e + e) % e;
                if rem == 0 {
                    break;
                }
                cur += chrono::Duration::minutes(1);
            }
            cur
        }
        "hour" => {
            let base = naive.date().and_hms_opt(naive.time().hour(), 0, 0)?;
            let mut cur = base;
            if cur < naive {
                cur += chrono::Duration::hours(1);
            }
            let e = every.max(1);
            loop {
                let hour = cur.time().hour() as i64;
                let rem = (hour % e + e) % e;
                if rem == 0 {
                    break;
                }
                cur += chrono::Duration::hours(1);
            }
            cur
        }
        "day" => {
            let mut cur = naive.date().and_hms_opt(0, 0, 0)?;
            if cur < naive {
                cur += chrono::Duration::days(1);
            }
            let e = every.max(1);
            if e > 1 {
                // D3's `timeDay.every(e)` uses `date.getDate() - 1` as the interval field, so the
                // modulus resets at each month boundary (days 1, 1+e, 1+2e, ... within a month).
                let mut d = cur.date();
                let day0 = d.day0() as i64;
                let rem = (day0 % e + e) % e;
                if rem != 0 {
                    d += chrono::Duration::days(e - rem);
                }
                cur = d.and_hms_opt(0, 0, 0)?;
            }
            cur
        }
        "week" => {
            let epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 4)?; // Sunday
            let start = week_start
                .and_then(weekday_from_str)
                .unwrap_or(chrono::Weekday::Sun);

            let mut d = naive.date();
            let cur_wd = d.weekday().num_days_from_sunday() as i64;
            let start_wd = start.num_days_from_sunday() as i64;
            let delta = (cur_wd - start_wd + 7) % 7;
            d -= chrono::Duration::days(delta);
            let mut cur = d.and_hms_opt(0, 0, 0)?;
            if cur < naive {
                cur += chrono::Duration::days(7);
            }

            let e = every.max(1);
            if e > 1 {
                let mut ws = cur.date();
                loop {
                    let weeks = ws.signed_duration_since(epoch).num_days() / 7;
                    let rem = (weeks % e + e) % e;
                    if rem == 0 {
                        break;
                    }
                    ws += chrono::Duration::days(7);
                }
                cur = ws.and_hms_opt(0, 0, 0)?;
            }
            cur
        }
        "month" => {
            let month_index = |y: i32, m: u32| (y as i64) * 12 + (m as i64 - 1);

            let mut y = naive.date().year();
            let mut m = naive.date().month();
            let mut cur = chrono::NaiveDate::from_ymd_opt(y, m, 1)?.and_hms_opt(0, 0, 0)?;
            if cur < naive {
                m += 1;
                if m > 12 {
                    m = 1;
                    y += 1;
                }
                cur = chrono::NaiveDate::from_ymd_opt(y, m, 1)?.and_hms_opt(0, 0, 0)?;
            }

            let e = every.max(1);
            if e > 1 {
                let mut idx = month_index(y, m);
                let rem = (idx % e + e) % e;
                if rem != 0 {
                    idx += e - rem;
                    y = (idx / 12) as i32;
                    m = (idx % 12) as u32 + 1;
                    cur = chrono::NaiveDate::from_ymd_opt(y, m, 1)?.and_hms_opt(0, 0, 0)?;
                }
            }
            cur
        }
        "year" => {
            let mut y = naive.date().year();
            let mut cur = chrono::NaiveDate::from_ymd_opt(y, 1, 1)?.and_hms_opt(0, 0, 0)?;
            if cur < naive {
                y += 1;
                cur = chrono::NaiveDate::from_ymd_opt(y, 1, 1)?.and_hms_opt(0, 0, 0)?;
            }
            let e = every.max(1) as i32;
            if e > 1 {
                let rem = (y % e + e) % e;
                if rem != 0 {
                    y += e - rem;
                    cur = chrono::NaiveDate::from_ymd_opt(y, 1, 1)?.and_hms_opt(0, 0, 0)?;
                }
            }
            cur
        }
        _ => return None,
    };

    let out = merman_core::time::datetime_from_naive_local(start);
    Some(out.timestamp_millis())
}

fn add_d3_time_day_every(ms: i64, every: i64) -> Option<i64> {
    let dt_utc = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms)?;
    let dt = dt_utc_to_local_fixed(dt_utc);
    let naive = dt.naive_local();

    let e = every.max(1);
    if e <= 1 {
        return add_interval(ms, 1, "day");
    }

    // D3's `timeDay.every(e)` uses a filtered interval based on `(date.getDate() - 1) % e`.
    // This means the modulus resets at month boundaries, and the "next tick" is not simply
    // `+e days` for months with non-multiple-of-e lengths.
    let cur_date = naive.date();
    let day0 = cur_date.day0() as i64;
    let next_day0 = day0 + 1;
    let rem = (next_day0 % e + e) % e;
    let delta = if rem == 0 { 0 } else { e - rem };
    let cand_day0 = next_day0 + delta;

    let (y, m) = (cur_date.year(), cur_date.month());
    let first_this_month = chrono::NaiveDate::from_ymd_opt(y, m, 1)?;
    let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
    let first_next_month = chrono::NaiveDate::from_ymd_opt(ny, nm, 1)?;
    let days_in_month = first_next_month
        .signed_duration_since(first_this_month)
        .num_days();

    let next_date = if cand_day0 < days_in_month {
        chrono::NaiveDate::from_ymd_opt(y, m, (cand_day0 + 1) as u32)?
    } else {
        first_next_month
    };

    let next = next_date.and_hms_opt(
        naive.time().hour(),
        naive.time().minute(),
        naive.time().second(),
    )?;

    let out = merman_core::time::datetime_from_naive_local(next);
    Some(out.timestamp_millis())
}

fn axis_format_to_strftime(axis_format: &str, date_format: &str, cfg_axis_format: &str) -> String {
    if !axis_format.trim().is_empty() {
        // Mermaid preserves any leading/trailing whitespace in `axisFormat` (it is treated as
        // literal text by d3-time-format). Keep the raw string for DOM parity.
        return axis_format.to_string();
    }
    if date_format.trim() == "D" {
        return "%d".to_string();
    }
    if !cfg_axis_format.trim().is_empty() {
        return cfg_axis_format.to_string();
    }
    "%Y-%m-%d".to_string()
}

fn is_chrono_strftime_directive(directive: char) -> bool {
    matches!(
        directive,
        'a' | 'A'
            | 'b'
            | 'B'
            | 'c'
            | 'C'
            | 'd'
            | 'D'
            | 'e'
            | 'F'
            | 'g'
            | 'G'
            | 'H'
            | 'I'
            | 'j'
            | 'k'
            | 'l'
            | 'm'
            | 'M'
            | 'n'
            | 'p'
            | 'P'
            | 'r'
            | 'R'
            | 'S'
            | 't'
            | 'T'
            | 'u'
            | 'U'
            | 'V'
            | 'w'
            | 'W'
            | 'x'
            | 'X'
            | 'y'
            | 'Y'
            | 'z'
            | 'Z'
            | '+'
            | '%'
            | 'f'
    )
}

fn format_axis_tick_label(d: chrono::DateTime<FixedOffset>, axis_format: &str) -> String {
    fn flush(out: &mut String, buf: &mut String, d: chrono::DateTime<FixedOffset>) {
        if buf.is_empty() {
            return;
        }
        out.push_str(&d.format(buf.as_str()).to_string());
        buf.clear();
    }

    let mut out = String::new();
    let mut buf = String::new();
    let mut it = axis_format.chars().peekable();

    while let Some(ch) = it.next() {
        if ch != '%' {
            buf.push(ch);
            continue;
        }

        let Some(next) = it.next() else {
            // Trailing `%` in the format string: treat it as a literal percent.
            buf.push_str("%%");
            break;
        };

        if next == '%' {
            buf.push_str("%%");
            continue;
        }

        // Mermaid uses d3-time-format directives for gantt `axisFormat`. Most overlap with
        // chrono's strftime, except for a few extras (e.g. `%L`).
        let (modifier, directive) = if matches!(next, '-' | '_' | '0') {
            let Some(dir) = it.next() else {
                flush(&mut out, &mut buf, d);
                out.push('%');
                out.push(next);
                break;
            };
            (Some(next), dir)
        } else {
            (None, next)
        };

        match (modifier, directive) {
            (None, 'L') => {
                // d3: milliseconds (000-999)
                flush(&mut out, &mut buf, d);
                out.push_str(&format!("{:03}", d.timestamp_subsec_millis()));
            }
            (None, 'Q') => {
                // d3: milliseconds since UNIX epoch
                flush(&mut out, &mut buf, d);
                out.push_str(&d.with_timezone(&chrono::Utc).timestamp_millis().to_string());
            }
            (None, 's') => {
                // d3: seconds since UNIX epoch
                flush(&mut out, &mut buf, d);
                out.push_str(&d.with_timezone(&chrono::Utc).timestamp().to_string());
            }
            (None, 'q') => {
                // d3: quarter of the year [1, 4]
                flush(&mut out, &mut buf, d);
                let q = (d.month0() / 3) + 1;
                out.push_str(&q.to_string());
            }
            _ => {
                // Special case: chrono supports `%.<digits>f` subseconds precision. Keep it in
                // the buffered chrono format to preserve existing behavior.
                if modifier.is_none() && directive == '.' {
                    let mut tmp = String::new();
                    tmp.push('%');
                    tmp.push('.');
                    while let Some(peek) = it.peek().copied() {
                        if peek.is_ascii_digit() {
                            tmp.push(peek);
                            it.next();
                        } else {
                            break;
                        }
                    }
                    if let Some('f') = it.peek().copied() {
                        tmp.push('f');
                        let _ = it.next();
                        buf.push_str(&tmp);
                        continue;
                    }
                    flush(&mut out, &mut buf, d);
                    out.push_str(&tmp);
                    continue;
                }

                if is_chrono_strftime_directive(directive) {
                    buf.push('%');
                    if let Some(m) = modifier {
                        buf.push(m);
                    }
                    buf.push(directive);
                } else {
                    // Avoid panics from chrono by treating unknown directives as literals.
                    flush(&mut out, &mut buf, d);
                    out.push('%');
                    if let Some(m) = modifier {
                        out.push(m);
                    }
                    out.push(directive);
                }
            }
        }
    }

    flush(&mut out, &mut buf, d);
    out
}

fn build_ticks(
    min_ms: i64,
    max_ms: i64,
    range: f64,
    left_padding: f64,
    axis_format: &str,
    tick_interval: Option<&str>,
    week_start: Option<&str>,
) -> Vec<GanttAxisTickLayout> {
    const MAX_TICK_COUNT: f64 = 10_000.0;

    fn estimate_ticks(min_ms: i64, max_ms: i64, every: i64, unit: &str) -> f64 {
        if every <= 0 || min_ms > max_ms {
            return f64::INFINITY;
        }

        let time_diff_ms = (max_ms - min_ms).abs().max(1) as f64;
        let interval_ms = match unit {
            "millisecond" => every as f64,
            "second" => (every as f64) * 1_000.0,
            "minute" => (every as f64) * 60_000.0,
            "hour" => (every as f64) * 3_600_000.0,
            "day" => (every as f64) * (MS_PER_DAY as f64),
            "week" => (every as f64) * (MS_PER_DAY as f64) * 7.0,
            // dayjs.duration({ month: n }).asMilliseconds() uses a fixed 30-day lattice.
            "month" => (every as f64) * (MS_PER_DAY as f64) * 30.0,
            _ => return f64::INFINITY,
        };
        if interval_ms <= 0.0 {
            return f64::INFINITY;
        }

        (time_diff_ms / interval_ms).ceil()
    }

    // Mermaid skips applying custom ticks when the interval would generate an excessive amount of
    // tick marks (it falls back to d3's automatic tick selection instead).
    let parsed = tick_interval
        .and_then(parse_tick_interval)
        .filter(|(every, unit)| estimate_ticks(min_ms, max_ms, *every, unit) <= MAX_TICK_COUNT);
    let (every, unit) = parsed.unwrap_or_else(|| auto_tick_interval(min_ms, max_ms));
    let week_start = if parsed.is_some() && unit == "week" {
        week_start
    } else {
        None
    };

    let mut ticks = Vec::new();
    let mut cur = ceil_tick_start(min_ms, every, unit, week_start).unwrap_or(min_ms);
    let max_ticks = 2000;
    for _ in 0..max_ticks {
        if cur > max_ms {
            break;
        }
        let x = scale_time(cur, min_ms, max_ms, range) + left_padding;
        let label = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(cur)
            .map(|d| format_axis_tick_label(dt_utc_to_local_fixed(d), axis_format))
            .unwrap_or_default();
        ticks.push(GanttAxisTickLayout {
            time_ms: cur,
            x,
            label,
        });
        let next = if unit == "day" && every > 1 {
            add_d3_time_day_every(cur, every)
        } else {
            add_interval(cur, every, unit)
        };
        let Some(next) = next else {
            break;
        };
        if next <= cur {
            break;
        }
        cur = next;
    }
    ticks
}

pub fn layout_gantt_diagram(
    model: &serde_json::Value,
    config: &serde_json::Value,
    text_measurer: &dyn TextMeasurer,
) -> Result<GanttDiagramLayout> {
    let model: GanttDiagramRenderModel = from_value_ref(model).map_err(Error::Json)?;
    layout_gantt_diagram_typed(&model, config, text_measurer)
}

pub fn layout_gantt_diagram_typed(
    model: &GanttDiagramRenderModel,
    config: &serde_json::Value,
    text_measurer: &dyn TextMeasurer,
) -> Result<GanttDiagramLayout> {
    let mut m = model.clone();

    let gantt_cfg = config.get("gantt").unwrap_or(config);
    let bar_gap = cfg_f64(gantt_cfg, &["barGap"]).unwrap_or(4.0);
    let bar_height = cfg_f64(gantt_cfg, &["barHeight"]).unwrap_or(20.0);
    let top_padding = cfg_f64(gantt_cfg, &["topPadding"]).unwrap_or(50.0);
    let left_padding = cfg_f64(gantt_cfg, &["leftPadding"]).unwrap_or(75.0);
    let right_padding = cfg_f64(gantt_cfg, &["rightPadding"]).unwrap_or(75.0);
    let grid_line_start_padding = cfg_f64(gantt_cfg, &["gridLineStartPadding"]).unwrap_or(35.0);
    let title_top_margin = cfg_f64(gantt_cfg, &["titleTopMargin"]).unwrap_or(25.0);
    let font_size = cfg_f64(gantt_cfg, &["fontSize"]).unwrap_or(11.0);
    let section_font_size = cfg_f64(gantt_cfg, &["sectionFontSize"]).unwrap_or(11.0);
    let number_section_styles = cfg_i64(gantt_cfg, &["numberSectionStyles"]).unwrap_or(4);

    let cfg_display_mode = gantt_cfg
        .get("displayMode")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let cfg_top_axis = cfg_bool(gantt_cfg, &["topAxis"]).unwrap_or(false);
    let cfg_axis_format = gantt_cfg
        .get("axisFormat")
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d");

    let width = gantt_cfg
        .get("useWidth")
        .and_then(|v| v.as_f64())
        .unwrap_or(DEFAULT_WIDTH);
    let gap = bar_height + bar_gap;

    let categories = collect_categories(&m.tasks);
    let is_compact = m.display_mode == "compact" || cfg_display_mode == "compact";

    let mut category_heights: Vec<(String, i64)> = Vec::new();
    if is_compact {
        let mut section_order: Vec<String> = Vec::new();
        let mut section_map: HashMap<String, Vec<usize>> = HashMap::new();
        for (idx, t) in m.tasks.iter().enumerate() {
            match section_map.entry(t.section.clone()) {
                Entry::Occupied(mut entry) => entry.get_mut().push(idx),
                Entry::Vacant(entry) => {
                    section_order.push(entry.key().clone());
                    entry.insert(vec![idx]);
                }
            }
        }

        let mut order_offset: i64 = 0;
        for sec in section_order {
            let idxs = section_map.get(&sec).cloned().unwrap_or_default();
            let mut subset: Vec<GanttRenderTask> =
                idxs.iter().map(|&i| m.tasks[i].clone()).collect();
            let max_i = get_max_intersections(&mut subset, order_offset);
            for (pos, &orig_idx) in idxs.iter().enumerate() {
                m.tasks[orig_idx].order = subset[pos].order;
            }
            let height = max_i + 1;
            order_offset += height;
            category_heights.push((sec, height));
        }
    } else {
        for c in &categories {
            let count = m.tasks.iter().filter(|t| &t.task_type == c).count() as i64;
            category_heights.push((c.clone(), count));
        }
    }

    let mut height = 2.0 * top_padding;
    if is_compact {
        for (_k, h) in &category_heights {
            height += *h as f64 * gap;
        }
    } else {
        height += m.tasks.len() as f64 * gap;
    }

    let has_tasks = !m.tasks.is_empty();
    let (min_ms, max_ms) = if has_tasks {
        let min_ms = m.tasks.iter().map(|t| t.start_ms).min().unwrap_or(0);
        let max_ms = m.tasks.iter().map(|t| t.end_ms).max().unwrap_or(min_ms);
        (min_ms, max_ms)
    } else {
        (0, 0)
    };
    let range = (width - left_padding - right_padding).max(1.0);
    let span_days = (max_ms - min_ms).abs() / MS_PER_DAY;
    let has_excludes_layer =
        has_tasks && (!m.excludes.is_empty() || !m.includes.is_empty()) && span_days <= 365 * 5;

    // Sort by start time for rendering.
    m.tasks.sort_by(|a, b| a.start_ms.cmp(&b.start_ms));

    // Exclude day ranges.
    let mut excludes_layout: Vec<GanttExcludeRangeLayout> = Vec::new();
    if has_excludes_layer {
        let mut cur = start_of_day_ms(min_ms).unwrap_or(min_ms);
        let max_day = start_of_day_ms(max_ms).unwrap_or(max_ms);
        let mut range_start: Option<i64> = None;
        let mut range_end: Option<i64> = None;

        while cur <= max_day {
            let invalid =
                is_invalid_date(cur, &m.date_format, &m.excludes, &m.includes, &m.weekend);
            if invalid {
                if range_start.is_none() {
                    range_start = Some(cur);
                    range_end = Some(cur);
                } else {
                    range_end = Some(cur);
                }
            } else if let (Some(s), Some(e)) = (range_start.take(), range_end.take()) {
                let id = format!(
                    "exclude-{}",
                    format_yyyy_mm_dd(s).unwrap_or_else(|| "invalid".to_string())
                );
                let x0 = scale_time(s, min_ms, max_ms, range) + left_padding;
                let eod = end_of_day_ms(e).unwrap_or(e);
                let x1 = scale_time(eod, min_ms, max_ms, range) + left_padding;
                excludes_layout.push(GanttExcludeRangeLayout {
                    id,
                    start_ms: s,
                    end_ms: eod,
                    x: x0,
                    y: grid_line_start_padding,
                    width: (x1 - x0).max(0.0),
                    height: (height - top_padding - grid_line_start_padding).max(0.0),
                });
            }
            cur += MS_PER_DAY;
        }
    }

    // Background rows.
    //
    // Mermaid draws the row rectangles by iterating the tasks in their render order (sorted by
    // `startTime`). This means the row insertion order is *not* necessarily ascending by `order`
    // (e.g. forward references can cause `order=0` to have the latest start date).
    let mut row_orders: Vec<i64> = Vec::new();
    for t in &m.tasks {
        if !row_orders.contains(&t.order) {
            row_orders.push(t.order);
        }
    }

    let mut rows: Vec<GanttRowLayout> = Vec::new();
    for order in &row_orders {
        let ttype = m
            .tasks
            .iter()
            .find(|t| t.order == *order)
            .map(|t| t.task_type.clone())
            .unwrap_or_default();

        let mut sec_num = 0_i64;
        for (i, c) in categories.iter().enumerate() {
            if &ttype == c {
                sec_num = (i as i64) % number_section_styles;
            }
        }

        let y = *order as f64 * gap + top_padding - 2.0;
        rows.push(GanttRowLayout {
            index: *order,
            x: 0.0,
            y,
            width: width - right_padding / 2.0,
            height: gap,
            class: format!("section section{sec_num}"),
        });
    }

    // Tasks (bars + labels).
    // Mermaid gantt task labels inherit the diagram font family (defaulting to
    // `"trebuchet ms", verdana, arial, sans-serif`), not the axis group's `sans-serif`.
    // Use the effective Mermaid font family here so `getBBox().width`-derived `width-*` class
    // values match upstream SVG baselines.
    let task_font_family = gantt_cfg
        .get("fontFamily")
        .and_then(|v| v.as_str())
        .or_else(|| config.get("fontFamily").and_then(|v| v.as_str()))
        .unwrap_or("\"trebuchet ms\", verdana, arial, sans-serif")
        .to_string();
    let text_style = TextStyle {
        font_family: Some(task_font_family.clone()),
        font_size,
        font_weight: None,
    };

    let mut tasks: Vec<GanttTaskLayout> = Vec::new();
    for t in &m.tasks {
        let start_x = scale_time(t.start_ms, min_ms, max_ms, range);
        let end_x = scale_time(t.end_ms, min_ms, max_ms, range);
        let render_end_x = scale_time(t.render_end_ms.unwrap_or(t.end_ms), min_ms, max_ms, range);

        let mut bar_x = start_x + left_padding;
        if t.milestone {
            bar_x = start_x + left_padding + 0.5 * (end_x - start_x) - 0.5 * bar_height;
        }

        let bar_y = if t.vert {
            grid_line_start_padding
        } else {
            t.order as f64 * gap + top_padding
        };
        let bar_width = if t.milestone {
            bar_height
        } else if t.vert {
            0.08 * bar_height
        } else {
            (render_end_x - start_x).max(0.0)
        };
        let bar_height_actual = if t.vert {
            m.tasks.len() as f64 * gap + bar_height * 2.0
        } else {
            bar_height
        };

        let mut sec_num = 0_i64;
        for (i, c) in categories.iter().enumerate() {
            if &t.task_type == c {
                sec_num = (i as i64) % number_section_styles;
            }
        }

        let mut task_class = String::new();
        if t.active {
            if t.crit {
                task_class.push_str(" activeCrit");
            } else {
                task_class.push_str(" active");
            }
        } else if t.done {
            if t.crit {
                task_class.push_str(" doneCrit");
            } else {
                task_class.push_str(" done");
            }
        } else if t.crit {
            task_class.push_str(" crit");
        }
        if task_class.is_empty() {
            task_class.push_str(" task");
        }
        if t.milestone {
            task_class = format!(" milestone{task_class}");
        }
        if t.vert {
            task_class = format!(" vert{task_class}");
        }
        task_class.push_str(&format!("{sec_num}"));
        if !t.classes.is_empty() {
            task_class.push(' ');
            task_class.push_str(&t.classes.join(" "));
        }

        let bar = GanttTaskBarLayout {
            id: t.id.clone(),
            x: bar_x,
            y: bar_y,
            width: bar_width,
            height: bar_height_actual,
            rx: 3.0,
            ry: 3.0,
            class: format!("task{task_class}"),
        };

        // Mermaid measures `textWidth` via `this.getBBox().width`, which does not include trailing
        // whitespace. Preserve the original task text for rendering, but trim it for measurement.
        let text_width = text_measurer.measure(t.task.trim_end(), &text_style).width;

        // Mermaid uses `renderEndTime` for the X-position calculation but `endTime` for the class
        // overflow check. Mirror this quirk for DOM parity.
        let mut start_x_for_label = start_x;
        let mut end_x_for_label = render_end_x;
        if t.milestone {
            start_x_for_label += 0.5 * (end_x - start_x) - 0.5 * bar_height;
            end_x_for_label = start_x_for_label + bar_height;
        }
        let start_x_for_class = start_x;
        let end_x_for_class = if t.milestone {
            start_x + bar_height
        } else {
            end_x
        };

        let label_x = if t.vert {
            start_x + left_padding
        } else if text_width > (end_x_for_label - start_x_for_label).abs() {
            if end_x_for_label + text_width + 1.5 * left_padding > width {
                start_x_for_label + left_padding - 5.0
            } else {
                end_x_for_label + left_padding + 5.0
            }
        } else {
            (end_x_for_label - start_x_for_label) / 2.0 + start_x_for_label + left_padding
        };

        let label_y = if t.vert {
            grid_line_start_padding + m.tasks.len() as f64 * gap + 60.0
        } else {
            t.order as f64 * gap + bar_height / 2.0 + (font_size / 2.0 - 2.0) + top_padding
        };

        let base_classes = if t.classes.is_empty() {
            String::new()
        } else {
            format!("{} ", t.classes.join(" "))
        };

        // Mermaid checks overflow for both horizontal and vertical labels:
        // `if (textWidth > endX - startX) { ... }` (Mermaid@11.12.2 ganttRenderer.js).
        let class_overflows = text_width > (end_x_for_class - start_x_for_class).abs();
        let outside_left =
            class_overflows && (end_x_for_class + text_width + 1.5 * left_padding > width);
        let outside_right = class_overflows && !outside_left;

        let label_class = if outside_left {
            format!("{base_classes}taskTextOutsideLeft taskTextOutside{sec_num}")
        } else if outside_right {
            format!(
                "{base_classes}taskTextOutsideRight taskTextOutside{sec_num} width-{text_width}"
            )
        } else {
            format!("{base_classes}taskText taskText{sec_num} width-{text_width}")
        };

        let label = GanttTaskLabelLayout {
            id: format!("{}-text", t.id),
            text: t.task.clone(),
            font_size,
            width: text_width,
            x: label_x,
            y: label_y,
            class: label_class.trim().to_string(),
        };

        tasks.push(GanttTaskLayout {
            id: t.id.clone(),
            task: t.task.clone(),
            section: t.section.clone(),
            task_type: t.task_type.clone(),
            order: t.order,
            start_ms: t.start_ms,
            end_ms: t.end_ms,
            render_end_ms: t.render_end_ms,
            milestone: t.milestone,
            vert: t.vert,
            bar,
            label,
        });
    }

    // Section titles.
    let mut section_titles: Vec<GanttSectionTitleLayout> = Vec::new();
    let mut prev_gap: i64 = 0;
    for (idx, (sec, h)) in category_heights.iter().enumerate() {
        let lines = DeterministicTextMeasurer::normalized_text_lines(sec);
        let dy_em = -((lines.len().saturating_sub(1)) as f64) / 2.0;

        let mut sec_num = 0_i64;
        for (j, c) in categories.iter().enumerate() {
            if sec == c {
                sec_num = (j as i64) % number_section_styles;
            }
        }

        let y = if idx == 0 {
            (*h as f64 * gap) / 2.0 + top_padding
        } else {
            prev_gap += category_heights[idx - 1].1;
            (*h as f64 * gap) / 2.0 + prev_gap as f64 * gap + top_padding
        };

        section_titles.push(GanttSectionTitleLayout {
            section: sec.clone(),
            index: idx as i64,
            x: 10.0,
            y,
            dy_em,
            lines,
            class: format!("sectionTitle sectionTitle{sec_num}"),
        });
    }

    let axis_format = axis_format_to_strftime(&m.axis_format, &m.date_format, cfg_axis_format);
    let tick_interval = m.tick_interval.as_deref();
    let week_start = if m.weekday.trim().is_empty() {
        gantt_cfg.get("weekday").and_then(|v| v.as_str())
    } else {
        Some(m.weekday.as_str())
    };
    let bottom_ticks = if has_tasks {
        build_ticks(
            min_ms,
            max_ms,
            range,
            left_padding,
            &axis_format,
            tick_interval,
            week_start,
        )
    } else {
        Vec::new()
    };
    let top_axis_enabled = m.top_axis || cfg_top_axis;
    let top_ticks = if has_tasks && top_axis_enabled {
        build_ticks(
            min_ms,
            max_ms,
            range,
            left_padding,
            &axis_format,
            tick_interval,
            week_start,
        )
    } else {
        Vec::new()
    };

    let bounds = Some(Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: width,
        max_y: height,
    });

    Ok(GanttDiagramLayout {
        bounds,
        width,
        height,
        left_padding,
        right_padding,
        top_padding,
        grid_line_start_padding,
        bar_height,
        bar_gap,
        title_top_margin,
        font_size,
        section_font_size,
        number_section_styles,
        display_mode: if m.display_mode.is_empty() {
            cfg_display_mode
        } else {
            m.display_mode.clone()
        },
        date_format: m.date_format.clone(),
        axis_format: m.axis_format.clone(),
        tick_interval: m.tick_interval.clone(),
        top_axis: top_axis_enabled,
        today_marker: m.today_marker.clone(),
        categories,
        rows,
        section_titles,
        tasks,
        excludes: excludes_layout,
        has_excludes_layer,
        bottom_ticks,
        top_ticks,
        title: m.title.clone(),
        title_x: width / 2.0,
        title_y: title_top_margin,
    })
}
