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

pub(crate) fn longest_git_dates(cx: &App) -> [String; 3] {
    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let now = OffsetDateTime::now_utc();
    // Wednesday, September 25 at 23:59:59 has the longest weekday, month name, day, and time.
    let september = time::macros::datetime!(2024-09-25 23:59:59).assume_offset(offset);
    // December 25 is also a Wednesday and has the longest unpadded month number.
    let december = time::macros::datetime!(2024-12-25 23:59:59).assume_offset(offset);
    // The locale format shows recent commits as "Yesterday at 11:59 PM", which is wider than
    // an older date.
    [
        (september, now),
        (december, now),
        (september, september + time::Duration::days(1)),
    ]
    .map(|(sample, reference)| {
        format_timestamp(
            sample,
            reference,
            offset,
            TimestampFormat::EnhancedAbsolute,
            &ProjectSettings::get_global(cx).git.date_format,
        )
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blame_ui::GitBlameRenderer;
    use editor::BlameRenderer;
    use gpui::UpdateGlobal;
    use settings::SettingsStore;
    use time::macros::{datetime, offset};
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn absolute_formats_use_local_time() {
        let timestamp = datetime!(2024-06-08 23:45:12 UTC);
        for (format, expected) in [
            ("iso", "2024-06-09 08:45"),
            ("%Y-%m-%d %H:%M:%S %z", "2024-06-09 08:45:12 +0900"),
            ("%a %b %d %%", "Sun Jun 09 %"),
        ] {
            for fallback in [
                TimestampFormat::Absolute,
                TimestampFormat::EnhancedAbsolute,
                TimestampFormat::MediumAbsolute,
            ] {
                assert_eq!(
                    format_timestamp(
                        timestamp,
                        timestamp,
                        offset!(+9),
                        fallback,
                        &GitDateFormat::parse(format)
                    ),
                    expected
                );
            }
        }
    }

    #[test]
    fn relative_dates_ignore_absolute_format() {
        let timestamp = datetime!(2024-06-08 23:45:12 UTC);
        let reference = timestamp + time::Duration::days(3);
        let expected = time_format::format_localized_timestamp(
            timestamp,
            reference,
            UtcOffset::UTC,
            TimestampFormat::Relative,
        );
        for format in ["locale", "iso", "%Y", "", "%Q"] {
            assert_eq!(
                format_timestamp(
                    timestamp,
                    reference,
                    UtcOffset::UTC,
                    TimestampFormat::Relative,
                    &GitDateFormat::parse(format)
                ),
                expected
            );
        }
    }

    #[test]
    fn locale_and_invalid_formats_preserve_surface_defaults() {
        let timestamp = datetime!(2024-06-08 23:45:12 UTC);
        let reference = timestamp + time::Duration::days(3);
        for format in ["locale", "", " ", "%Q", "%"] {
            assert!(matches!(
                GitDateFormat::parse(format),
                GitDateFormat::Locale
            ));
            for fallback in [
                TimestampFormat::Absolute,
                TimestampFormat::EnhancedAbsolute,
                TimestampFormat::MediumAbsolute,
            ] {
                assert_eq!(
                    format_timestamp(
                        timestamp,
                        reference,
                        UtcOffset::UTC,
                        fallback,
                        &GitDateFormat::parse(format)
                    ),
                    time_format::format_localized_timestamp(
                        timestamp,
                        reference,
                        UtcOffset::UTC,
                        fallback
                    )
                );
            }
        }
    }

    fn init(cx: &mut App) {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        ProjectSettings::register(cx);
    }

    fn set_blame_date_format(format: &str, cx: &mut App) {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |content| {
                let git = content.git.get_or_insert_default();
                git.date_format = Some(format.into());
                git.blame.get_or_insert_default().date_style = Some(GitDateStyle::Absolute);
            });
        });
    }

    #[gpui::test]
    fn blame_gutter_width_fits_longest_date(cx: &mut App) {
        init(cx);
        assert_eq!(
            GitBlameRenderer.max_timestamp_width(cx),
            editor::MAX_RELATIVE_TIMESTAMP.len()
        );

        for (format, width) in [
            ("iso", "2024-09-25 23:59".len()),
            ("%d", "25".len()),
            ("%B %Y", "September 2024".len()),
            ("%A %H:%M", "Wednesday 23:59".len()),
            ("%-m/%-d/%Y", "12/25/2024".len()),
            ("%H:%M:%-S", "23:59:59".len()),
            ("%Y年%m月", 10),
        ] {
            set_blame_date_format(format, cx);
            assert_eq!(GitBlameRenderer.max_timestamp_width(cx), width, "{format}");
        }

        set_blame_date_format("locale", cx);
        let locale_dates = longest_git_dates(cx);
        assert!(
            locale_dates
                .iter()
                .any(|date| date.starts_with("Yesterday at ")),
            "{locale_dates:?}"
        );
        assert_eq!(
            GitBlameRenderer.max_timestamp_width(cx),
            locale_dates
                .iter()
                .map(|date| UnicodeWidthStr::width(date.as_str()))
                .max()
                .unwrap_or(0)
        );
    }
}
