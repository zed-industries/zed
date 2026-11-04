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
    fn blame_gutter_width_uses_actual_custom_dates(cx: &mut App) {
        init(cx);
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
            GitBlameRenderer.timestamp_length(&entry, cx),
            timestamp.len()
        );
        set_settings(
            r#"{"git": {"date_format": "%Y年%m月", "blame": {"date_style": "absolute"}}}"#,
            cx,
        );
        assert_eq!(GitBlameRenderer.timestamp_length(&entry, cx), 10);
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
