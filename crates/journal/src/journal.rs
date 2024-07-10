use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate, NaiveTime, Timelike};
use editor::scroll::Autoscroll;
use editor::Editor;
use gpui::{actions, AppContext, ViewContext, WindowContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    sync::Arc,
};
use workspace::{AppState, OpenVisible, Workspace};

actions!(journal, [NewJournalEntry]);

/// Settings specific to journaling
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct JournalSettings {
    /// The path of the directory where journal entries are stored.
    ///
    /// Default: `~`
    pub path: Option<String>,
    /// What format to display the hours in.
    ///
    /// Default: hour12
    pub hour_format: Option<HourFormat>,
    /// The entry format for storing journal entries.
    /// Default: "journal/%Y/%m/%d.md"
    pub entry_format: Option<String>,
}

const DEFAULT_ENTRY_FORMAT: &str = "journal/%Y/%m/%d.md";

impl Default for JournalSettings {
    fn default() -> Self {
        Self {
            path: Some("~".into()),
            hour_format: Some(Default::default()),
            entry_format: Some(DEFAULT_ENTRY_FORMAT.into()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HourFormat {
    #[default]
    Hour12,
    Hour24,
}

impl settings::Settings for JournalSettings {
    const KEY: Option<&'static str> = Some("journal");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}

impl JournalSettings {
    pub fn get_entry_path(&self, date: NaiveDate) -> String {
        let format_str = self.entry_format.as_deref().unwrap_or(DEFAULT_ENTRY_FORMAT);
        let formatted_path = std::panic::catch_unwind(|| date.format(format_str).to_string());
        formatted_path.unwrap_or_else(|_| date.format(DEFAULT_ENTRY_FORMAT).to_string())
    }
}

pub fn init(_: Arc<AppState>, cx: &mut AppContext) {
    JournalSettings::register(cx);

    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &NewJournalEntry, cx| {
                new_journal_entry(workspace.app_state().clone(), cx);
            });
        },
    )
    .detach();
}

pub fn new_journal_entry(app_state: Arc<AppState>, cx: &mut WindowContext) {
    let settings = JournalSettings::get_global(cx);
    let journal_dir = match journal_dir(settings.path.as_ref().unwrap()) {
        Some(journal_dir) => journal_dir,
        None => {
            log::error!("Can't determine journal directory");
            return;
        }
    };

    let now = Local::now();
    let date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap();
    let entry_path = journal_dir.join(settings.get_entry_path(date));
    let entry_heading = heading_entry(now.time(), &settings.hour_format);

    let create_entry = cx.background_executor().spawn(async move {
        std::fs::create_dir_all(entry_path.parent().unwrap())?;
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&entry_path)?;
        Ok::<_, std::io::Error>((journal_dir, entry_path))
    });

    cx.spawn(|mut cx| async move {
        let (journal_dir, entry_path) = create_entry.await?;
        let (workspace, _) = cx
            .update(|cx| {
                workspace::open_paths(
                    &[journal_dir],
                    app_state,
                    workspace::OpenOptions::default(),
                    cx,
                )
            })?
            .await?;

        let opened = workspace
            .update(&mut cx, |workspace, cx| {
                workspace.open_paths(vec![entry_path], OpenVisible::All, None, cx)
            })?
            .await;

        if let Some(Some(Ok(item))) = opened.first() {
            if let Some(editor) = item.downcast::<Editor>().map(|editor| editor.downgrade()) {
                editor.update(&mut cx, |editor, cx| {
                    let len = editor.buffer().read(cx).len(cx);
                    editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                        s.select_ranges([len..len])
                    });
                    if len > 0 {
                        editor.insert("\n\n", cx);
                    }
                    editor.insert(&entry_heading, cx);
                    editor.insert("\n\n", cx);
                })?;
            }
        }

        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn journal_dir(path: &str) -> Option<PathBuf> {
    let expanded_journal_dir = shellexpand::full(path) //TODO handle this better
        .ok()
        .map(|dir| Path::new(&dir.to_string()).to_path_buf());

    return expanded_journal_dir;
}

fn heading_entry(now: NaiveTime, hour_format: &Option<HourFormat>) -> String {
    match hour_format {
        Some(HourFormat::Hour24) => {
            let hour = now.hour();
            format!("# {}:{:02}", hour, now.minute())
        }
        _ => {
            let (pm, hour) = now.hour12();
            let am_or_pm = if pm { "PM" } else { "AM" };
            format!("# {}:{:02} {}", hour, now.minute(), am_or_pm)
        }
    }
}

#[cfg(test)]
mod tests {
    mod heading_entry_tests {
        use super::super::*;

        #[test]
        fn test_heading_entry_defaults_to_hour_12() {
            let naive_time = NaiveTime::from_hms_milli_opt(15, 0, 0, 0).unwrap();
            let actual_heading_entry = heading_entry(naive_time, &None);
            let expected_heading_entry = "# 3:00 PM";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }

        #[test]
        fn test_heading_entry_is_hour_12() {
            let naive_time = NaiveTime::from_hms_milli_opt(15, 0, 0, 0).unwrap();
            let actual_heading_entry = heading_entry(naive_time, &Some(HourFormat::Hour12));
            let expected_heading_entry = "# 3:00 PM";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }

        #[test]
        fn test_heading_entry_is_hour_24() {
            let naive_time = NaiveTime::from_hms_milli_opt(15, 0, 0, 0).unwrap();
            let actual_heading_entry = heading_entry(naive_time, &Some(HourFormat::Hour24));
            let expected_heading_entry = "# 15:00";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }
    }

    mod entry_format_tests {
        use super::super::*;
        use chrono::NaiveDate;

        #[test]
        fn test_get_entry_path_with_default_settings() {
            let settings = JournalSettings::default();
            let date = NaiveDate::from_ymd_opt(2024, 7, 10);
            let entry_path = settings.get_entry_path(date.unwrap());

            assert_eq!(entry_path, format!("journal/2024/07/10.md"));
        }

        #[test]
        fn test_get_entry_path_with_custom_folder_structure() {
            let settings = JournalSettings {
                entry_format: Some("custom/%Y-%m-%d.md".to_string()),
                ..Default::default()
            };
            let date = NaiveDate::from_ymd_opt(2024, 7, 10).unwrap();
            let entry_path = settings.get_entry_path(date);

            assert_eq!(entry_path, format!("custom/2024-07-10.md"));
        }

        #[test]
        fn test_get_entry_format_with_invalid_characters() {
            let settings = JournalSettings {
                entry_format: Some("/inva%Plid/\0/path/file.md".to_string()),
                ..Default::default()
            };
            let date = NaiveDate::from_ymd_opt(2024, 7, 10).unwrap();
            let entry_path = settings.get_entry_path(date);

            // should fall back to default format
            assert_eq!(entry_path, format!("journal/2024/07/10.md"));
        }

        #[test]
        fn test_get_entry_path_with_custom_nested_folder_structure() {
            let settings = JournalSettings {
                entry_format: Some("entries/%Y/%m/%d-entry.md".to_string()),
                ..Default::default()
            };
            let date = NaiveDate::from_ymd_opt(2024, 7, 10).unwrap();
            let entry_path = settings.get_entry_path(date);

            assert_eq!(entry_path, "entries/2024/07/10-entry.md");
        }

        #[test]
        fn test_journal_dir_expands_tilde() {
            let path = "~";
            let expanded_dir = journal_dir(path).unwrap();
            let expected_dir = match shellexpand::full(path) {
                Ok(expanded) => Some(Path::new(&expanded.to_string()).to_path_buf()),
                Err(_) => None,
            }
            .unwrap();

            assert_eq!(expanded_dir, expected_dir);
        }

        #[test]
        fn test_journal_dir_with_custom_path() {
            let path = "/custom/path";
            let expanded_dir = journal_dir(path).unwrap();
            let expected_dir = Path::new("/custom/path");

            assert_eq!(expanded_dir, expected_dir);
        }

        #[test]
        fn test_journal_dir_resolve_absolute_file_path() {
            let settings = JournalSettings {
                path: Some("~".to_string()),
                entry_format: Some(DEFAULT_ENTRY_FORMAT.into()),
                ..Default::default()
            };

            let path = settings.path.clone().unwrap();
            let expanded_journal_dir = journal_dir(&path).unwrap();
            let date = NaiveDate::from_ymd_opt(2024, 7, 10).unwrap();
            let entry_path = settings.get_entry_path(date);

            let expected_journal_dir = match shellexpand::full(&path) {
                Ok(expanded) => Some(Path::new(&expanded.to_string()).to_path_buf()),
                Err(_) => None,
            }
            .unwrap();

            assert_eq!(expanded_journal_dir, expected_journal_dir);
            assert_eq!(
                expanded_journal_dir.join(&entry_path),
                expected_journal_dir.join(&entry_path)
            );
        }
    }
}
