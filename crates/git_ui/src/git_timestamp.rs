use gpui::App;
use project::project_settings::{GitDateSurface, ProjectSettings};
use settings::Settings as _;
use time::{OffsetDateTime, UtcOffset};

pub struct GitTimestampFormatter {
    date_style: settings::GitDateStyleSetting,
    absolute_date_format: Option<project::project_settings::GitDateFormat>,
    local_offset: UtcOffset,
    reference_time: OffsetDateTime,
}

impl GitTimestampFormatter {
    pub fn new(surface: GitDateSurface, cx: &App) -> Self {
        let settings = &ProjectSettings::get_global(cx).git;
        Self {
            date_style: settings.date_style(surface),
            absolute_date_format: settings.absolute_date_format(surface).cloned(),
            local_offset: UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
            reference_time: OffsetDateTime::now_utc(),
        }
    }

    pub fn date_style(&self) -> settings::GitDateStyleSetting {
        self.date_style
    }

    pub fn local_offset(&self) -> UtcOffset {
        self.local_offset
    }

    pub fn format_custom_absolute(&self, timestamp: OffsetDateTime) -> Option<String> {
        self.absolute_date_format
            .as_ref()
            .and_then(|format| format.format(timestamp, self.local_offset))
    }

    pub fn format(
        &self,
        timestamp: OffsetDateTime,
        fallback_format: time_format::TimestampFormat,
    ) -> String {
        let fallback_format = if fallback_format == time_format::TimestampFormat::Relative
            && self.date_style == settings::GitDateStyleSetting::Absolute
        {
            time_format::TimestampFormat::MediumAbsolute
        } else {
            fallback_format
        };

        if fallback_format != time_format::TimestampFormat::Relative
            && let Some(timestamp) = self.format_custom_absolute(timestamp)
        {
            return timestamp;
        }

        time_format::format_localized_timestamp(
            timestamp,
            self.reference_time,
            self.local_offset,
            fallback_format,
        )
    }
}

pub fn format_git_timestamp(
    timestamp: OffsetDateTime,
    fallback_format: time_format::TimestampFormat,
    cx: &App,
) -> String {
    format_git_timestamp_for_surface(timestamp, fallback_format, GitDateSurface::Default, cx)
}

pub fn format_git_timestamp_for_surface(
    timestamp: OffsetDateTime,
    fallback_format: time_format::TimestampFormat,
    surface: GitDateSurface,
    cx: &App,
) -> String {
    GitTimestampFormatter::new(surface, cx).format(timestamp, fallback_format)
}

#[cfg(test)]
mod timestamp_tests {
    use super::*;
    use gpui::{TestAppContext, UpdateGlobal};
    use project::project_settings::GitDateSurface::{Blame, Default as DefaultSurface, GitGraph};
    use settings::SettingsStore;
    use time_format::TimestampFormat::{MediumAbsolute, Relative};

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            ProjectSettings::register(cx);
        });
    }

    fn update_git_settings(
        cx: &mut TestAppContext,
        update: impl FnOnce(&mut settings::GitSettings),
    ) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    update(settings.git.get_or_insert_default())
                });
            });
        });
    }

    fn timestamp() -> OffsetDateTime {
        time::macros::datetime!(2020-06-15 12:34:56 UTC)
    }

    fn assert_format(
        cx: &mut TestAppContext,
        timestamp: OffsetDateTime,
        format: time_format::TimestampFormat,
        surface: GitDateSurface,
        expected: &str,
    ) {
        assert_eq!(
            cx.read(|cx| format_git_timestamp_for_surface(timestamp, format, surface, cx)),
            expected
        );
    }

    fn assert_custom(cx: &mut TestAppContext, surface: GitDateSurface, expected: Option<&str>) {
        assert_eq!(
            cx.read(
                |cx| GitTimestampFormatter::new(surface, cx).format_custom_absolute(timestamp())
            )
            .as_deref(),
            expected
        );
    }

    #[gpui::test]
    fn test_global_git_timestamp_settings(cx: &mut TestAppContext) {
        init_test(cx);
        assert_custom(cx, DefaultSurface, None);

        update_git_settings(cx, |git| {
            git.absolute_date_format = Some("commit [year]-[month]".to_string());
        });
        assert_custom(cx, DefaultSurface, Some("commit 2020-06"));
        assert_format(
            cx,
            timestamp(),
            MediumAbsolute,
            DefaultSurface,
            "commit 2020-06",
        );
        assert_format(
            cx,
            OffsetDateTime::now_utc(),
            Relative,
            DefaultSurface,
            "Just now",
        );

        update_git_settings(cx, |git| {
            git.date_style = Some(settings::GitDateStyleSetting::Absolute)
        });
        assert_format(cx, timestamp(), Relative, DefaultSurface, "commit 2020-06");
    }

    #[gpui::test]
    fn test_git_timestamp_surface_overrides(cx: &mut TestAppContext) {
        init_test(cx);
        update_git_settings(cx, |git| {
            git.date_style = Some(settings::GitDateStyleSetting::Absolute);
            git.absolute_date_format = Some("global [year]".to_string());
            let blame = git.blame.get_or_insert_default();
            blame.date_style = Some(settings::GitDateStyleSetting::Relative);
            blame.absolute_date_format = Some("blame [year]".to_string());
            git.git_graph.get_or_insert_default().absolute_date_format =
                Some("graph [year]".to_string());
        });
        assert_format(cx, OffsetDateTime::now_utc(), Relative, Blame, "Just now");
        assert_format(cx, timestamp(), MediumAbsolute, Blame, "blame 2020");
        assert_custom(cx, GitGraph, Some("graph 2020"));
        assert_custom(cx, DefaultSurface, Some("global 2020"));

        update_git_settings(cx, |git| {
            let blame = git.blame.get_or_insert_default();
            blame.date_style = None;
            blame.absolute_date_format = None;
        });
        assert_format(cx, timestamp(), Relative, Blame, "global 2020");
    }

    #[gpui::test]
    fn test_git_timestamp_falls_back_when_date_format_is_invalid(cx: &mut TestAppContext) {
        init_test(cx);
        update_git_settings(cx, |git| {
            git.absolute_date_format = Some("[invalid]".to_string())
        });
        assert_custom(cx, DefaultSurface, None);
        assert_format(
            cx,
            OffsetDateTime::now_utc(),
            Relative,
            DefaultSurface,
            "Just now",
        );

        update_git_settings(cx, |git| {
            git.date_style = Some(settings::GitDateStyleSetting::Absolute);
            git.blame.get_or_insert_default().absolute_date_format =
                Some("blame [invalid]".to_string());
        });
        cx.read(|cx| {
            let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
            assert_eq!(
                format_git_timestamp_for_surface(timestamp(), Relative, Blame, cx),
                time_format::format_localized_timestamp(
                    timestamp(),
                    OffsetDateTime::now_utc(),
                    local_offset,
                    MediumAbsolute,
                )
            );
        });
    }
}
