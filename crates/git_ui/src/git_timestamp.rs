use gpui::App;
use project::project_settings::{GitDateFormat, ProjectSettings};
use settings::{GitDateStyle, Settings};
use time::{OffsetDateTime, UtcOffset};
use time_format::TimestampFormat;

pub(crate) fn format_git_timestamp(
    timestamp: OffsetDateTime,
    format: TimestampFormat,
    cx: &App,
) -> String {
    format_timestamp(
        timestamp,
        OffsetDateTime::now_utc(),
        UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
        format,
        &ProjectSettings::get_global(cx).git.date_format,
    )
}

pub(crate) fn format_git_date(timestamp: OffsetDateTime, style: GitDateStyle, cx: &App) -> String {
    let format = match style {
        GitDateStyle::Relative => TimestampFormat::Relative,
        GitDateStyle::Absolute => TimestampFormat::EnhancedAbsolute,
    };
    format_git_timestamp(timestamp, format, cx)
}

fn format_timestamp(
    timestamp: OffsetDateTime,
    reference: OffsetDateTime,
    offset: UtcOffset,
    format: TimestampFormat,
    date_format: &GitDateFormat,
) -> String {
    if !matches!(format, TimestampFormat::Relative)
        && let Some(timestamp) = date_format.format(timestamp.to_offset(offset))
    {
        return timestamp;
    }
    time_format::format_localized_timestamp(timestamp, reference, offset, format)
}
