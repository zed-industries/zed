use chrono::{Datelike, Local, Timelike};
use editor::{Autoscroll, Editor};
use gpui::{actions, MutableAppContext};
use std::{fs::OpenOptions, sync::Arc};
use util::TryFutureExt as _;
use workspace::AppState;

actions!(journal, [NewJournalEntry]);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    cx.add_global_action(move |_: &NewJournalEntry, cx| new_journal_entry(app_state.clone(), cx));
}

pub fn new_journal_entry(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    let now = Local::now();
    let home_dir = match dirs::home_dir() {
        Some(home_dir) => home_dir,
        None => {
            log::error!("can't determine home directory");
            return;
        }
    };

    let journal_dir = home_dir.join("journal");
    let month_dir = journal_dir
        .join(format!("{:02}", now.year()))
        .join(format!("{:02}", now.month()));
    let entry_path = month_dir.join(format!("{:02}.md", now.day()));
    let now = now.time();
    let (pm, hour) = now.hour12();
    let am_or_pm = if pm { "PM" } else { "AM" };
    let entry_heading = format!("# {}:{:02} {}\n\n", hour, now.minute(), am_or_pm);

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
            let (workspace, _, _) = cx
                .update(|cx| workspace::open_paths(&[journal_dir], &app_state, cx))
                .await;

            let opened = workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.open_paths(&[entry_path], cx)
                })
                .await;

            if let Some(Some(Ok(item))) = opened.first() {
                if let Some(editor) = item.downcast::<Editor>() {
                    editor.update(&mut cx, |editor, cx| {
                        let len = editor.buffer().read(cx).read(cx).len();
                        editor.select_ranges([len..len], Some(Autoscroll::Center), cx);
                        if len > 0 {
                            editor.insert("\n\n", cx);
                        }
                        editor.insert(&entry_heading, cx);
                    });
                }
            }

            Ok(())
        }
        .log_err()
    })
    .detach();
}
