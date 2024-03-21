use anyhow::Result;
use chrono::{Datelike, Local, NaiveTime, Timelike};
use editor::scroll::Autoscroll;
use editor::Editor;
use gpui::{actions, AppContext, ViewContext, WindowContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
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
}

impl Default for JournalSettings {
    fn default() -> Self {
        Self {
            path: Some("~".into()),
            hour_format: Some(Default::default()),
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

    fn load(
        defaults: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut AppContext,
    ) -> Result<Self> {
        Self::load_via_json_merge(defaults, user_values)
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
    let month_dir = journal_dir
        .join(format!("{:02}", now.year()))
        .join(format!("{:02}", now.month()));
    let entry_path = month_dir.join(format!("{:02}.md", now.day()));
    let now = now.time();
    let entry_heading = heading_entry(now, &settings.hour_format);

    let create_entry = cx.background_executor().spawn(async move {
        std::fs::create_dir_all(month_dir)?;
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
        .map(|dir| Path::new(&dir.to_string()).to_path_buf().join("journal"));

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
}
