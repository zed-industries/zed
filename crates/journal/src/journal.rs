use chrono::{Datelike, Local, NaiveTime, Timelike};
use editor::scroll::Autoscroll;
use editor::{Editor, SelectionEffects};
use gpui::{App, AppContext as _, Context, Window, actions};
pub use settings::HourFormat;
use settings::Settings;
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    sync::Arc,
};
use workspace::{AppState, OpenVisible, Workspace};

actions!(
    journal,
    [
        /// Creates a new journal entry for today.
        NewJournalEntry
    ]
);

/// Settings specific to journaling
#[derive(Clone, Debug)]
pub struct JournalSettings {
    /// The path of the directory where journal entries are stored.
    ///
    /// Default: `~`
    pub path: String,
    /// What format to display the hours in.
    ///
    /// Default: hour12
    pub hour_format: HourFormat,
}

impl settings::Settings for JournalSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let journal = content.journal.clone().unwrap();

        Self {
            path: journal.path.unwrap(),
            hour_format: journal.hour_format.unwrap(),
        }
    }
}

pub fn init(_: Arc<AppState>, cx: &mut App) {
    JournalSettings::register(cx);

    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &NewJournalEntry, window, cx| {
                new_journal_entry(workspace, window, cx);
            });
        },
    )
    .detach();
}

pub fn new_journal_entry(workspace: &Workspace, window: &mut Window, cx: &mut App) {
    let settings = JournalSettings::get_global(cx);
    let journal_dir = match journal_dir(&settings.path) {
        Some(journal_dir) => journal_dir,
        None => {
            log::error!("Can't determine journal directory");
            return;
        }
    };
    let journal_dir_clone = journal_dir.clone();

    let now = Local::now();
    let month_dir = journal_dir
        .join(format!("{:02}", now.year()))
        .join(format!("{:02}", now.month()));
    let entry_path = month_dir.join(format!("{:02}.md", now.day()));
    let now = now.time();
    let entry_heading = heading_entry(now, &settings.hour_format);

    let create_entry = cx.background_spawn(async move {
        std::fs::create_dir_all(month_dir)?;
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&entry_path)?;
        Ok::<_, std::io::Error>((journal_dir, entry_path))
    });

    let worktrees = workspace.visible_worktrees(cx).collect::<Vec<_>>();
    let mut open_new_workspace = true;
    'outer: for worktree in worktrees.iter() {
        let worktree_root = worktree.read(cx).abs_path();
        if *worktree_root == journal_dir_clone {
            open_new_workspace = false;
            break;
        }
        for directory in worktree.read(cx).directories(true, 1) {
            let full_directory_path = worktree_root.join(directory.path.as_std_path());
            if full_directory_path.ends_with(&journal_dir_clone) {
                open_new_workspace = false;
                break 'outer;
            }
        }
    }

    let app_state = workspace.app_state().clone();
    let view_snapshot = workspace.weak_handle();

    window
        .spawn(cx, async move |cx| {
            let (journal_dir, entry_path) = create_entry.await?;
            let opened = if open_new_workspace {
                let (new_workspace, _) = cx
                    .update(|_window, cx| {
                        workspace::open_paths(
                            &[journal_dir],
                            app_state,
                            workspace::OpenOptions::default(),
                            cx,
                        )
                    })?
                    .await?;
                new_workspace
                    .update(cx, |workspace, window, cx| {
                        workspace.open_paths(
                            vec![entry_path],
                            workspace::OpenOptions {
                                visible: Some(OpenVisible::All),
                                ..Default::default()
                            },
                            None,
                            window,
                            cx,
                        )
                    })?
                    .await
            } else {
                view_snapshot
                    .update_in(cx, |workspace, window, cx| {
                        workspace.open_paths(
                            vec![entry_path],
                            workspace::OpenOptions {
                                visible: Some(OpenVisible::All),
                                ..Default::default()
                            },
                            None,
                            window,
                            cx,
                        )
                    })?
                    .await
            };

            if let Some(Some(Ok(item))) = opened.first()
                && let Some(editor) = item.downcast::<Editor>().map(|editor| editor.downgrade())
            {
                editor.update_in(cx, |editor, window, cx| {
                    let len = editor.buffer().read(cx).len(cx);
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()),
                        window,
                        cx,
                        |s| s.select_ranges([len..len]),
                    );
                    if len > 0 {
                        editor.insert("\n\n", window, cx);
                    }
                    editor.insert(&entry_heading, window, cx);
                    editor.insert("\n\n", window, cx);
                })?;
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
}

fn journal_dir(path: &str) -> Option<PathBuf> {
    shellexpand::full(path) //TODO handle this better
        .ok()
        .map(|dir| Path::new(&dir.to_string()).to_path_buf().join("journal"))
}

fn heading_entry(now: NaiveTime, hour_format: &HourFormat) -> String {
    match hour_format {
        HourFormat::Hour24 => {
            let hour = now.hour();
            format!("# {}:{:02}", hour, now.minute())
        }
        HourFormat::Hour12 => {
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
            let actual_heading_entry = heading_entry(naive_time, &HourFormat::Hour12);
            let expected_heading_entry = "# 3:00 PM";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }

        #[test]
        fn test_heading_entry_is_hour_12() {
            let naive_time = NaiveTime::from_hms_milli_opt(15, 0, 0, 0).unwrap();
            let actual_heading_entry = heading_entry(naive_time, &HourFormat::Hour12);
            let expected_heading_entry = "# 3:00 PM";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }

        #[test]
        fn test_heading_entry_is_hour_24() {
            let naive_time = NaiveTime::from_hms_milli_opt(15, 0, 0, 0).unwrap();
            let actual_heading_entry = heading_entry(naive_time, &HourFormat::Hour24);
            let expected_heading_entry = "# 15:00";

            assert_eq!(actual_heading_entry, expected_heading_entry);
        }
    }
}
