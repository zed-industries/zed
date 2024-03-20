use anyhow::{Context, Result};
use git::blame::parse_git_blame;
use git::blame::run_git_blame;
use language::Buffer;
use language::BufferSnapshot;
use sum_tree::SumTree;

use core::fmt;
use git::blame::BlameEntry;
use gpui::{Model, ModelContext, Subscription, Task};
use project::{Item, Project};
use smallvec::SmallVec;

#[derive(Clone, Debug, Default)]
pub struct GitBlameEntry {
    rows: u32,
    // TODO: Do we want to remove rows from BlameEntry?
    blame: Option<BlameEntry>,
}

#[derive(Clone, Debug, Default)]
pub struct GitBlameEntrySummary {
    rows: u32,
}

impl sum_tree::Item for GitBlameEntry {
    type Summary = GitBlameEntrySummary;

    fn summary(&self) -> Self::Summary {
        GitBlameEntrySummary { rows: self.rows }
    }
}

impl sum_tree::Summary for GitBlameEntrySummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _cx: &()) {
        self.rows += summary.rows;
    }
}

impl<'a> sum_tree::Dimension<'a, GitBlameEntrySummary> for u32 {
    fn add_summary(&mut self, summary: &'a GitBlameEntrySummary, _cx: &()) {
        *self += summary.rows;
    }
}

// - As edits trickle in, call `GitBlame::interpolate()`
// Save detected:
// - Call `MultiBuffer::subscribe`, store that somewhere. Grab a snapshot
// - In the background, recalculate the entire blame for the snapshot
// - Finally, when the background task is done, come back to the main thread, see if theere have been any edits since the task was started, and interpolate those

pub struct GitBlame {
    project: Model<Project>,
    buffer: Model<Buffer>,
    entries: SumTree<GitBlameEntry>,
    buffer_snapshot: BufferSnapshot,
    buffer_edits: text::Subscription,
    task: Task<Result<()>>,
    _refresh_subscription: Subscription,
}

impl GitBlame {
    pub fn new(
        buffer: Model<Buffer>,
        project: Model<Project>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let entries = SumTree::from_item(
            GitBlameEntry {
                rows: buffer.read(cx).max_point().row + 1,
                blame: None,
            },
            &(),
        );

        // TODO: what to do with untitled buffers
        let project_entry_id = buffer.read(cx).entry_id(cx);

        let refresh_subscription = cx.subscribe(&project, move |this, _, event, cx| match event {
            project::Event::WorktreeUpdatedEntries(_, updated) => {
                if updated
                    .iter()
                    .any(|(_, entry_id, _)| project_entry_id == Some(*entry_id))
                {
                    log::debug!("Updated buffers. Regenerating blame data...",);
                    if let Err(error) = this.generate(cx) {
                        log::error!("Failed to update git blame information: {}", error);
                    }
                }
            }
            project::Event::WorktreeUpdatedGitRepositories(_) => {
                log::debug!("Status of git repositories updated. Regenerating blame data...",);
                if let Err(error) = this.generate(cx) {
                    log::error!("Failed to update git blame information: {}", error);
                }
            }
            _ => {}
        });

        let buffer_snapshot = buffer.read(cx).snapshot();
        let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());
        let mut this = Self {
            project,
            buffer,
            buffer_snapshot,
            entries,
            buffer_edits,
            task: Task::ready(Ok(())),
            _refresh_subscription: refresh_subscription,
        };
        this.generate(cx);
        this
    }

    pub fn blame_for_rows(
        &mut self,
        rows: impl IntoIterator<Item = Option<u32>>,
    ) -> impl Iterator<Item = Option<git::blame::BlameEntry>> {
        self.sync();

        todo!("fix this");
        std::iter::once(None)
    }

    fn sync(&mut self) {
        let edits = self.buffer_edits.consume();
        // interpolate
    }

    fn generate(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        let buffer = self.buffer.read(cx);

        // Collab version: move this to the project, check `if is_local()`.

        let buffer_snapshot = buffer.snapshot();

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

        let path = local_file.path().clone();
        let buffer_edits = self.buffer.update(cx, |buffer, _| buffer.subscribe());

        self.task = cx.spawn(|this, mut cx| async move {
            let background_buffer_snapshot = buffer_snapshot.clone();

            let task: Task<Result<SumTree<GitBlameEntry>>> =
                cx.background_executor().spawn(async move {
                    // todo!("don't allocate a string")
                    let git_blame_output = run_git_blame(
                        &working_directory,
                        &path,
                        &background_buffer_snapshot.as_rope().to_string(),
                    )?;
                    let parsed_git_blame = parse_git_blame(&git_blame_output)?;

                    let mut current_row = 0;
                    let mut entries = SumTree::from_iter(
                        parsed_git_blame.into_iter().flat_map(|entry| {
                            let mut entries = SmallVec::<[GitBlameEntry; 2]>::new();

                            if entry.range.start > current_row {
                                let skipped_rows = entry.range.start - current_row;
                                entries.push(GitBlameEntry {
                                    rows: skipped_rows,
                                    blame: None,
                                });
                            }
                            entries.push(GitBlameEntry {
                                rows: entry.range.len() as u32,
                                blame: Some(entry.clone()),
                            });

                            current_row = entry.range.end;
                            entries
                        }),
                        &(),
                    );

                    let max_row = background_buffer_snapshot.max_point().row;
                    if max_row > current_row {
                        entries.push(
                            GitBlameEntry {
                                rows: max_row - current_row,
                                blame: None,
                            },
                            &(),
                        );
                    }

                    Ok(entries)
                });

            let entries = task.await?;

            this.update(&mut cx, |this, cx| {
                this.buffer_edits = buffer_edits;
                this.buffer_snapshot = buffer_snapshot;
                this.entries = entries;
                cx.notify();
            })
        });

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayBlameEntry {
    pub display_row: u32,
    pub entry: BlameEntry,
}

impl fmt::Display for DisplayBlameEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = self
            .entry
            .committer_datetime()
            .map_err(|_| std::fmt::Error)?
            .format("%Y-%m-%d %H:%M")
            .to_string();

        let pretty_commit_id = format!("{}", self.entry.sha);
        let short_commit_id = pretty_commit_id.chars().take(6).collect::<String>();

        let name = self.entry.committer.as_deref().unwrap_or("<no name>");
        let name = if name.len() > 20 {
            format!("{}...", &name[..16])
        } else {
            name.to_string()
        };

        write!(f, "{:6} {:20} ({})", short_commit_id, name, datetime)
    }
}
