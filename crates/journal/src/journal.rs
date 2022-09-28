use chrono::{Datelike, Local, NaiveTime, Timelike};
use editor::{Autoscroll, Editor};
use gpui::{actions, MutableAppContext};
use settings::{HourFormat, JournalDirectory, Settings};
use std::{fs::OpenOptions, path::PathBuf, str::FromStr, sync::Arc};
use util::TryFutureExt as _;
use workspace::AppState;

actions!(journal, [NewJournalEntry]);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    cx.add_global_action(move |_: &NewJournalEntry, cx| new_journal_entry(app_state.clone(), cx));
}

pub fn new_journal_entry(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    let settings = cx.global::<Settings>();
    let journal_dir = match get_journal_dir(&settings.journal_overrides.journal_directory) {
        Some(journal_dir) => journal_dir,
        None => {
            log::error!("can't determine home directory");
            return;
        }
    };

    let now = Local::now();
    let month_dir = journal_dir
        .join(format!("{:02}", now.year()))
        .join(format!("{:02}", now.month()));
    let entry_path = month_dir.join(format!("{:02}.md", now.day()));
    let now = now.time();
    let hour_format = &settings.journal_overrides.hour_format;
    let entry_heading = get_heading_entry(now, &hour_format);

    let create_entry = cx.background().spawn(async move {
        std::fs::create_dir_all(month_dir)?;
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&entry_path)?;
        Ok::<_, std::io::Error>((journal_dir, entry_path))
    });

    cx.spawn(|mut cx| {
        async move {
            let (journal_dir, entry_path) = create_entry.await?;
            let (workspace, _) = cx
                .update(|cx| workspace::open_paths(&[journal_dir], &app_state, cx))
                .await;

            let opened = workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.open_paths(vec![entry_path], true, cx)
                })
                .await;

            if let Some(Some(Ok(item))) = opened.first() {
                if let Some(editor) = item.downcast::<Editor>() {
                    editor.update(&mut cx, |editor, cx| {
                        let len = editor.buffer().read(cx).len(cx);
                        editor.change_selections(Some(Autoscroll::Center), cx, |s| {
                            s.select_ranges([len..len])
                        });
                        if len > 0 {
                            editor.insert("\n\n", cx);
                        }
                        editor.insert(&entry_heading, cx);
                        editor.insert("\n\n", cx);
                    });
                }
            }

            Ok(())
        }
        .log_err()
    })
    .detach();
}

fn get_journal_dir(a: &Option<JournalDirectory>) -> Option<PathBuf> {
    let journal_default_dir = dirs::home_dir()?.join("journal");

    let journal_dir = match a {
        Some(JournalDirectory::Always { directory }) => {
            PathBuf::from_str(&directory).unwrap_or(journal_default_dir)
        }
        _ => journal_default_dir,
    };

    Some(journal_dir)
}

fn get_heading_entry(now: NaiveTime, hour_format: &Option<HourFormat>) -> String {
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
            let naive_time = NaiveTime::from_hms_milli(15, 0, 0, 0);
            let actual_heading_entry = get_heading_entry(naive_time, &None);
            let expected_heading_entry = "# 3:00 PM";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }

        #[test]
        fn test_heading_entry_is_hour_12() {
            let naive_time = NaiveTime::from_hms_milli(15, 0, 0, 0);
            let actual_heading_entry = get_heading_entry(naive_time, &Some(HourFormat::Hour12));
            let expected_heading_entry = "# 3:00 PM";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }

        #[test]
        fn test_heading_entry_is_hour_24() {
            let naive_time = NaiveTime::from_hms_milli(15, 0, 0, 0);
            let actual_heading_entry = get_heading_entry(naive_time, &Some(HourFormat::Hour24));
            let expected_heading_entry = "# 15:00";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }
    }
}
