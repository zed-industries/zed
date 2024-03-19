use anyhow::Result;
use collections::HashSet;

use core::fmt;
use git::blame::{BlameEntry, BufferBlame};
use gpui::{EventEmitter, Model, ModelContext, Subscription, Task};
use multi_buffer::MultiBuffer;
use project::{Item, Project};
use std::ops::Range;
use text::Point;

use crate::display_map::ToDisplayPoint;

use crate::DisplaySnapshot;

pub enum Event {
    ShowBufferBlame { buffer_blame: BufferBlame },
}

pub struct Blame {
    project: Model<Project>,
    buffer: Model<MultiBuffer>,

    task: Option<Task<Result<()>>>,

    _refresh_subscription: Subscription,
}

impl Blame {
    pub fn new(
        buffer: Model<MultiBuffer>,
        project: Model<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let project_entry_ids = buffer
            .read(cx)
            .all_buffers()
            .iter()
            .filter_map(|buffer| buffer.read(cx).entry_id(cx))
            .collect::<HashSet<_>>();

        let refresh_subscription = cx.subscribe(&project, move |this, _, event, cx| match event {
            project::Event::WorktreeUpdatedEntries(_, updated) => {
                if updated
                    .iter()
                    .any(|(_, entry_id, _)| project_entry_ids.contains(entry_id))
                {
                    log::debug!("Updated buffers. Regenerating blame data...",);
                    this.generate(cx);
                }
            }
            project::Event::WorktreeUpdatedGitRepositories(_) => {
                log::debug!("Status of git repositories updated. Regenerating blame data...",);
                this.generate(cx);
            }
            _ => {}
        });

        Self {
            project,
            buffer,
            task: None,
            _refresh_subscription: refresh_subscription,
        }
    }

    pub fn generate(&mut self, cx: &mut ModelContext<Self>) -> Option<()> {
        let buffer = self.buffer.read(cx).as_singleton()?;
        let file = buffer.read(cx).file()?.as_local()?.path();

        let buffer_project_path = buffer.read(cx).project_path(cx)?;
        let working_directory = self
            .project
            .read(cx)
            .get_workspace_root(&buffer_project_path, cx)?;
        let buffer_snapshot = buffer.read(cx).snapshot();

        let generation_task = cx.background_executor().spawn({
            let file = file.clone();
            async move { BufferBlame::new_with_cli(&working_directory, &file, &buffer_snapshot) }
        });

        self.task = Some(cx.spawn(move |this, mut cx| async move {
            generation_task.await.and_then(|blame| {
                this.update(&mut cx, |_, cx| {
                    cx.emit(Event::ShowBufferBlame {
                        buffer_blame: blame,
                    });
                    cx.notify();
                })
            })
        }));

        Some(())
    }
}

impl EventEmitter<Event> for Blame {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayBlameEntry {
    Folded {
        display_row: u32,
    },

    Unfolded {
        display_row_range: Range<u32>,
        entry: BlameEntry,
    },
}

impl fmt::Display for DisplayBlameEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DisplayBlameEntry::Folded { .. } => Ok(()),
            DisplayBlameEntry::Unfolded { entry, .. } => {
                let datetime = entry
                    .committer_datetime()
                    .map_err(|_| std::fmt::Error)?
                    .format("%Y-%m-%d %H:%M")
                    .to_string();

                let pretty_commit_id = format!("{}", entry.sha);
                let short_commit_id = pretty_commit_id.chars().take(6).collect::<String>();

                let name = entry.committer.as_deref().unwrap_or("<no name>");
                let name = if name.len() > 20 {
                    format!("{}...", &name[..16])
                } else {
                    name.to_string()
                };

                write!(f, "{:6} {:20} ({})", short_commit_id, name, datetime)
            }
        }
    }
}

pub fn blame_entry_to_display(entry: &BlameEntry, snapshot: &DisplaySnapshot) -> DisplayBlameEntry {
    // TODO: This is all wrong, I bet
    let hunk_start_point = Point::new(entry.range.start, 0);

    let start = hunk_start_point.to_display_point(snapshot).row();
    let hunk_end_row = entry.range.end.max(entry.range.start);
    let hunk_end_point = Point::new(hunk_end_row, 0);
    let end = hunk_end_point.to_display_point(snapshot).row();

    DisplayBlameEntry::Unfolded {
        display_row_range: start..end,
        entry: entry.clone(),
    }
}
