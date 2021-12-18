use std::{fs::OpenOptions, sync::Arc};

use anyhow::anyhow;
use chrono::{Datelike, Local};
use gpui::{action, keymap::Binding, MutableAppContext};
use util::TryFutureExt as _;
use workspace::AppState;

action!(NewJournalEntry);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    cx.add_bindings(vec![Binding::new("ctrl-alt-cmd-j", NewJournalEntry, None)]);
    cx.add_global_action(move |_: &NewJournalEntry, cx| new_journal_entry(app_state.clone(), cx));
}

pub fn new_journal_entry(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    let paths = cx.background().spawn(async move {
        let now = Local::now();
        let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("can't determine home directory"))?;
        let journal_dir = home_dir.join("journal");
        let month_dir = journal_dir
            .join(now.year().to_string())
            .join(now.month().to_string());
        let entry_path = month_dir.join(format!("{}.md", now.day()));

        std::fs::create_dir_all(dbg!(month_dir))?;
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(dbg!(&entry_path))?;

        Ok::<_, anyhow::Error>((journal_dir, entry_path))
    });

    cx.spawn(|mut cx| {
        async move {
            let (journal_dir, entry_path) = paths.await?;
            let workspace = cx
                .update(|cx| workspace::open_paths(&[journal_dir], &app_state, cx))
                .await;

            let opened = workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.open_paths(&[entry_path], cx)
                })
                .await;

            if let Some(Some(Ok(item))) = opened.first() {
                log::info!("opened an item!");
            }

            Ok(())
        }
        .log_err()
    })
    .detach();
}
