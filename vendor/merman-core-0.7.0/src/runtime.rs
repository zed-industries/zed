use chrono::{FixedOffset, NaiveDate, NaiveDateTime, TimeZone};
use std::cell::Cell;

thread_local! {
    static FIXED_TODAY_LOCAL: Cell<Option<NaiveDate>> = const { Cell::new(None) };
    static FIXED_LOCAL_OFFSET_MINUTES: Cell<Option<i32>> = const { Cell::new(None) };
}

pub(crate) fn with_fixed_today_local<R>(today: Option<NaiveDate>, f: impl FnOnce() -> R) -> R {
    FIXED_TODAY_LOCAL.with(|cell| {
        let prev = cell.replace(today);
        let out = f();
        cell.set(prev);
        out
    })
}

pub(crate) fn with_fixed_local_offset_minutes<R>(
    offset_minutes: Option<i32>,
    f: impl FnOnce() -> R,
) -> R {
    FIXED_LOCAL_OFFSET_MINUTES.with(|cell| {
        let prev = cell.replace(offset_minutes);
        let out = f();
        cell.set(prev);
        out
    })
}

pub(crate) fn today_naive_local() -> NaiveDate {
    FIXED_TODAY_LOCAL
        .with(|cell| cell.get())
        .unwrap_or_else(|| chrono::Local::now().date_naive())
}

pub(crate) fn datetime_from_naive_local(naive: NaiveDateTime) -> chrono::DateTime<FixedOffset> {
    if let Some(mins) = FIXED_LOCAL_OFFSET_MINUTES.with(|cell| cell.get()) {
        let offset = FixedOffset::east_opt(mins.saturating_mul(60))
            .unwrap_or_else(crate::time::utc_fixed_offset);
        return offset
            .from_local_datetime(&naive)
            .single()
            .unwrap_or_else(|| {
                chrono::DateTime::<FixedOffset>::from_naive_utc_and_offset(naive, offset)
            });
    }

    match chrono::Local.from_local_datetime(&naive) {
        chrono::LocalResult::Single(dt) => dt.fixed_offset(),
        chrono::LocalResult::Ambiguous(a, _b) => a.fixed_offset(),
        chrono::LocalResult::None => chrono::DateTime::<FixedOffset>::from_naive_utc_and_offset(
            naive,
            crate::time::utc_fixed_offset(),
        ),
    }
}

pub(crate) fn datetime_to_local_fixed(
    dt: chrono::DateTime<FixedOffset>,
) -> chrono::DateTime<FixedOffset> {
    if let Some(mins) = FIXED_LOCAL_OFFSET_MINUTES.with(|cell| cell.get()) {
        let offset = FixedOffset::east_opt(mins.saturating_mul(60))
            .unwrap_or_else(crate::time::utc_fixed_offset);
        return dt.with_timezone(&offset);
    }

    dt.with_timezone(&chrono::Local).fixed_offset()
}

pub(crate) fn datetime_to_naive_local(dt: chrono::DateTime<FixedOffset>) -> NaiveDateTime {
    datetime_to_local_fixed(dt).naive_local()
}
