use super::*;

pub(super) fn today_midnight_local() -> DateTimeFixed {
    let date = crate::runtime::today_naive_local();
    let naive = NaiveDateTime::new(date, chrono::NaiveTime::MIN);
    local_from_naive(naive)
}

pub(super) fn local_from_naive(naive: NaiveDateTime) -> DateTimeFixed {
    crate::runtime::datetime_from_naive_local(naive)
}

pub(super) fn add_days_local(dt: DateTimeFixed, days: i64) -> Option<DateTimeFixed> {
    let naive = crate::runtime::datetime_to_naive_local(dt);
    let date = naive.date();
    let time = naive.time();

    let new_date = if days >= 0 {
        date.checked_add_days(chrono::Days::new(days as u64))?
    } else {
        date.checked_sub_days(chrono::Days::new((-days) as u64))?
    };
    Some(local_from_naive(NaiveDateTime::new(new_date, time)))
}

pub(super) fn add_months_local(dt: DateTimeFixed, months: i64) -> Option<DateTimeFixed> {
    let naive = crate::runtime::datetime_to_naive_local(dt);
    let mut year = naive.year();
    let mut month0 = naive.month0() as i64; // 0..=11

    month0 += months;
    year += month0.div_euclid(12) as i32;
    month0 = month0.rem_euclid(12);

    let month = (month0 as u32) + 1;
    let day = naive.day().min(last_day_of_month(year, month));
    let date = NaiveDate::from_ymd_opt(year, month, day)?;
    Some(local_from_naive(NaiveDateTime::new(date, naive.time())))
}

pub(super) fn add_years_local(dt: DateTimeFixed, years: i64) -> Option<DateTimeFixed> {
    let naive = crate::runtime::datetime_to_naive_local(dt);
    let year = naive.year().checked_add(years as i32)?;
    let month = naive.month();
    let day = naive.day().min(last_day_of_month(year, month));
    let date = NaiveDate::from_ymd_opt(year, month, day)?;
    Some(local_from_naive(NaiveDateTime::new(date, naive.time())))
}

pub(super) fn last_day_of_month(year: i32, month: u32) -> u32 {
    let Some((next_year, next_month)) = next_month_start(year, month) else {
        return 31;
    };
    let Some(first_next) = NaiveDate::from_ymd_opt(next_year, next_month, 1) else {
        return 31;
    };
    first_next.pred_opt().map_or(1, |last| last.day())
}

fn next_month_start(year: i32, month: u32) -> Option<(i32, u32)> {
    match month {
        1..=11 => Some((year, month + 1)),
        12 => year.checked_add(1).map(|next_year| (next_year, 1)),
        _ => None,
    }
}
