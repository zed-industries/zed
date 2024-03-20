use anyhow::{Context, Result};
use collections::{HashMap, HashSet};
use language::Buffer;
use language::Point;

use core::fmt;
use git::blame::{BlameEntry, BufferBlame};
use gpui::{EventEmitter, Model, ModelContext, Subscription, Task};
use multi_buffer::MultiBuffer;
use project::{Item, Project};
use std::ops::Range;
use text::BufferId;

use crate::DisplaySnapshot;

use crate::display_map::ToDisplayPoint;

pub enum Event {
    ShowMultiBufferBlame { blame: MultiBufferBlame },
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
                    if let Err(error) = this.generate(cx) {
                        log::error!("Failed to regenerate blame data: {}", error);
                    }
                }
            }
            project::Event::WorktreeUpdatedGitRepositories(_) => {
                log::debug!("Status of git repositories updated. Regenerating blame data...",);
                if let Err(error) = this.generate(cx) {
                    log::error!("Failed to regenerate blame data: {}", error);
                }
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

    pub fn generate(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        let mut tasks = Vec::new();

        for buffer in self.buffer.read(cx).all_buffers() {
            let task = self.generate_buffer(buffer, cx)?;
            tasks.push(task);
        }

        self.task = Some(cx.spawn(move |this, mut cx| async move {
            let blames: HashMap<BufferId, BufferBlame> = futures::future::join_all(tasks)
                .await
                .into_iter()
                .filter_map(|result| result.ok())
                .collect();

            let multi_buffer_blame = MultiBufferBlame::new(blames);

            this.update(&mut cx, |_, cx| {
                cx.emit(Event::ShowMultiBufferBlame {
                    blame: multi_buffer_blame,
                });
                cx.notify();
            })
        }));

        Ok(())
    }

    fn generate_buffer(
        &self,
        buffer: Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> Result<Task<Result<(BufferId, BufferBlame)>>> {
        let buffer = buffer.read(cx);

        let buffer_project_path = buffer
            .project_path(cx)
            .context("failed to get buffer project path")?;
        let working_directory = self
            .project
            .read(cx)
            .get_workspace_root(&buffer_project_path, cx)
            .context("failed to get workspace root")?;

        let file = buffer.file().context("failed to get buffer file")?;
        let local_file = file
            .as_local()
            .context("failed to turn file into local file")?;

        Ok(cx.background_executor().spawn({
            let path = local_file.path().clone();
            let buffer_snapshot = buffer.snapshot();

            async move {
                let blame = BufferBlame::new_with_cli(&working_directory, &path, &buffer_snapshot)?;
                Ok((buffer_snapshot.remote_id(), blame))
            }
        }))
    }
}

impl EventEmitter<Event> for Blame {}

#[derive(Clone)]
pub struct MultiBufferBlame {
    blames: HashMap<BufferId, BufferBlame>,
}

impl MultiBufferBlame {
    fn new(blames: HashMap<BufferId, BufferBlame>) -> Self {
        MultiBufferBlame { blames }
    }

    pub fn get(&self, buffer_id: BufferId) -> Option<&BufferBlame> {
        self.blames.get(&buffer_id)
    }
}

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

pub fn blame_entry_to_display(
    entry: &BlameEntry,
    buffer_range: Range<u32>,
    display_row_range: Range<u32>,
    snapshot: &DisplaySnapshot,
) -> Option<DisplayBlameEntry> {
    // buffer_range: original range

    let offset = buffer_range.start - display_row_range.start;

    println!(
        "buffer_range: {:?}, display_row_range: {:?}, entry.range: {:?}, offset: {}",
        buffer_range, display_row_range, entry.range, offset
    );

    if entry.range.end == buffer_range.start {
        return None;
    }

    let start = entry.range.start.max(buffer_range.start);
    let end = entry.range.end.min(buffer_range.end);

    let buffer_display_row_range = (start - offset)..(end - offset);
    println!("buffer display row range: {:?}", buffer_display_row_range);

    let start_point = Point::new(buffer_display_row_range.start, 0);
    let start_display_point = start_point.to_display_point(snapshot).row();
    println!("start display point: {:?}", start_display_point);

    let end_point = Point::new(buffer_display_row_range.end, 0);
    let end_display_point = end_point.to_display_point(snapshot).row();
    println!("end display point: {:?}", end_display_point);
    // let end_point = Point::new(end, 0);
    // let end = hunk_end_point.to_display_point(snapshot).row();

    Some(DisplayBlameEntry::Unfolded {
        display_row_range: start_display_point..end_display_point,
        entry: entry.clone(),
    })
}
