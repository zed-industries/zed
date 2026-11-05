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
    use crate::{
        blame_ui::GitBlameRenderer, git_graph::GitGraphSettings,
        git_panel_settings::GitPanelSettings,
    };
    use editor::BlameRenderer;
    use git::{Oid, blame::BlameEntry};
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
    fn try_parse_accepts_valid_and_rejects_invalid_formats() {
        for format in ["locale", "", " ", "iso", "%Y-%m-%d"] {
            assert!(GitDateFormat::try_parse(format).is_ok(), "{format:?}");
        }
        for format in ["%Q", "%"] {
            assert!(GitDateFormat::try_parse(format).is_err(), "{format:?}");
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
        GitPanelSettings::register(cx);
        GitGraphSettings::register(cx);
    }

    fn set_settings(value: &str, cx: &mut App) {
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(value, cx)
                .result()
                .expect("valid date settings");
        });
    }

    #[gpui::test]
    fn defaults_preserve_existing_date_styles(cx: &mut App) {
        init(cx);
        let git = &ProjectSettings::get_global(cx).git;
        assert!(matches!(git.date_format, GitDateFormat::Locale));
        assert_eq!(git.blame.date_style, GitDateStyle::Relative);
        assert_eq!(git.inline_blame.date_style, GitDateStyle::Relative);
        assert_eq!(git.branch_picker.date_style, GitDateStyle::Relative);
        assert_eq!(git.stash_picker.date_style, GitDateStyle::Relative);
        assert_eq!(
            GitPanelSettings::get_global(cx).date_style,
            GitDateStyle::Relative
        );
        assert_eq!(
            GitGraphSettings::get_global(cx).date_style,
            GitDateStyle::Absolute
        );
    }

    #[gpui::test]
    fn date_styles_are_independent_and_live(cx: &mut App) {
        init(cx);
        set_settings(
            r#"{
            "git": {
                "date_format": "%Y",
                "blame": {"date_style": "absolute"},
                "branch_picker": {"date_style": "absolute"},
                "stash_picker": {"date_style": "absolute"}
            },
            "git_panel": {"date_style": "absolute"},
            "git_graph": {"date_style": "relative"}
        }"#,
            cx,
        );
        let git = &ProjectSettings::get_global(cx).git;
        assert_eq!(git.blame.date_style, GitDateStyle::Absolute);
        assert_eq!(git.inline_blame.date_style, GitDateStyle::Relative);
        assert_eq!(git.branch_picker.date_style, GitDateStyle::Absolute);
        assert_eq!(git.stash_picker.date_style, GitDateStyle::Absolute);
        assert_eq!(
            GitPanelSettings::get_global(cx).date_style,
            GitDateStyle::Absolute
        );
        assert_eq!(
            GitGraphSettings::get_global(cx).date_style,
            GitDateStyle::Relative
        );
        let timestamp = datetime!(2024-06-08 12:00 UTC);
        assert_eq!(
            format_git_timestamp(timestamp, TimestampFormat::MediumAbsolute, cx),
            "2024"
        );
        set_settings(
            r#"{"git": {"date_format": "%m", "inline_blame": {"date_style": "absolute"}}}"#,
            cx,
        );
        let git = &ProjectSettings::get_global(cx).git;
        assert_eq!(git.blame.date_style, GitDateStyle::Relative);
        assert_eq!(git.inline_blame.date_style, GitDateStyle::Absolute);
        assert_eq!(
            format_git_date(timestamp, git.inline_blame.date_style, cx),
            "06"
        );
        assert_eq!(
            GitGraphSettings::get_global(cx).date_style,
            GitDateStyle::Absolute
        );
    }

    #[gpui::test]
    fn blame_gutter_width_reserves_longest_date(cx: &mut App) {
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
        ] {
            set_settings(
                &format!(
                    r#"{{"git": {{"date_format": "{format}", "blame": {{"date_style": "absolute"}}}}}}"#
                ),
                cx,
            );
            assert_eq!(GitBlameRenderer.max_timestamp_width(cx), width, "{format}");
        }
        set_settings(r#"{"git": {"blame": {"date_style": "absolute"}}}"#, cx);
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
        set_settings(
            r#"{"git": {"date_format": "a very long custom date: %A, %B %d %Y %H:%M:%S %z", "blame": {"date_style": "absolute"}}}"#,
            cx,
        );
        let entry = BlameEntry {
            sha: Oid::from_bytes(&[1; 20]).expect("valid commit ID"),
            range: 0..1,
            original_line_number: 1,
            author: Some("Author".into()),
            author_mail: None,
            author_time: Some(datetime!(2024-06-08 12:00 UTC).unix_timestamp()),
            author_tz: Some("+0000".into()),
            committer_name: None,
            committer_email: None,
            committer_time: None,
            committer_tz: None,
            summary: None,
            previous: None,
            filename: String::new(),
            boundary: false,
        };
        let timestamp =
            crate::commit_tooltip::blame_entry_timestamp(&entry, GitDateStyle::Absolute, cx);
        assert!(timestamp.len() > "2 years, 11 months ago".len());
        assert_eq!(
            GitBlameRenderer.max_timestamp_width(cx),
            longest_git_dates(cx)[0].len()
        );
        set_settings(
            r#"{"git": {"date_format": "%Y年%m月", "blame": {"date_style": "absolute"}}}"#,
            cx,
        );
        assert_eq!(GitBlameRenderer.max_timestamp_width(cx), 10);
        set_settings(
            r#"{"git": {"date_format": "%A %d", "blame": {"date_style": "absolute"}}}"#,
            cx,
        );
        assert_eq!(longest_git_dates(cx)[0], "Wednesday 25");
        let mut invalid_entry = entry;
        invalid_entry.author_time = Some(i64::MAX);
        assert_eq!(
            crate::commit_tooltip::blame_entry_timestamp(
                &invalid_entry,
                GitDateStyle::Absolute,
                cx
            ),
            "Error parsing date"
        );
    }
}
