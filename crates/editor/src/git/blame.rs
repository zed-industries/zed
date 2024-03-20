use anyhow::{Context, Result};
use language::Buffer;
use language::BufferSnapshot;
use sum_tree::SumTree;
use text::Bias;
use text::Edit;

use core::fmt;
use git::blame::BlameEntry;
use gpui::{Model, ModelContext, Subscription, Task};
use project::{Item, Project};
use smallvec::SmallVec;
use std::sync::Arc;

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
    blame_runner: Arc<dyn GitBlameRunner>,
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
        blame_runner: Arc<dyn GitBlameRunner>,
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

        let blame_runner = blame_runner.clone();
        let buffer_snapshot = buffer.read(cx).snapshot();
        let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());

        let mut this = Self {
            blame_runner,
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

        let mut cursor = self.entries.cursor::<u32>();
        // Seek along rows:

        todo!("fix this");
        std::iter::once(None)
    }

    fn sync(&mut self) {
        let edits = self.buffer_edits.consume();
        let new_snapshot = self.buffer.read(cx).snapshot();

        let mut row_edits = edits
            .into_iter()
            .map(|edit| Edit {
                old: self.buffer_snapshot.offset_to_point(edit.old.start).row
                    ..self.buffer_snapshot.offset_to_point(edit.old.end).row + 1,
                new: new_snapshot.offset_to_point(edit.new.start).row
                    ..new_snapshot.offset_to_point(edit.new.end).row + 1,
            })
            .peekable();

        let mut new_entries = SumTree::new();
        let mut cursor = self.entries.cursor::<u32>();

        while let Some(mut edit) = row_edits.next() {
            while let Some(next_edit) = row_edits.peek() {
                if edit.old.end >= next_edit.old.start {
                    edit.old.end = next_edit.old.end;
                    edit.new.end = next_edit.new.end;
                    row_edits.next();
                } else {
                    break;
                }
            }

            new_entries.append(cursor.slice(&edit.old.start, Bias::Right, &()), &());

            if edit.new.start > new_entries.summary().rows {
                new_entries.push(
                    GitBlameEntry {
                        rows: edit.new.start - new_entries.summary().rows,
                        blame: None,
                    },
                    &(),
                );
            }

            cursor.seek(&edit.old.end, Bias::Right, &());
            new_entries.push(
                GitBlameEntry {
                    rows: edit.new.len() as u32,
                    blame: None,
                },
                &(),
            );
        }
        new_entries.append(cursor.suffix(&()), &());

        self.buffer_snapshot = new_snapshot;
        self.entries = new_entries;

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

        let blame_runner = self.blame_runner.clone();

        self.task = cx.spawn(|this, mut cx| async move {
            let background_buffer_snapshot = buffer_snapshot.clone();

            let task: Task<Result<SumTree<GitBlameEntry>>> =
                cx.background_executor().spawn(async move {
                    // In your code, you would use `git_blame_runner` which is an instance of a type that implements `GitBlameRunner`.
                    // For example, in tests, you can provide a mock implementation of `GitBlameRunner`.
                    let mut parsed_git_blame = blame_runner.run(
                        &working_directory,
                        &path,
                        &background_buffer_snapshot.as_rope().to_string(),
                    )?;
                    parsed_git_blame.sort_by(|a, b| a.range.start.cmp(&b.range.start));

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

pub trait GitBlameRunner: Send + Sync {
    fn run(
        &self,
        working_directory: &std::path::Path,
        path: &std::path::Path,
        content: &str,
    ) -> Result<Vec<git::blame::BlameEntry>>;
}

pub struct RealGitBlameRunner;

impl RealGitBlameRunner {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl GitBlameRunner for RealGitBlameRunner {
    fn run(
        &self,
        working_directory: &std::path::Path,
        path: &std::path::Path,
        content: &str,
    ) -> Result<Vec<git::blame::BlameEntry>> {
        let output = git::blame::run_git_blame(working_directory, path, content)?;
        git::blame::parse_git_blame(&output)
    }
}

struct FakeGitBlameRunner {
    entries: Vec<git::blame::BlameEntry>,
}

impl GitBlameRunner for FakeGitBlameRunner {
    fn run(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
        _: &str,
    ) -> Result<Vec<git::blame::BlameEntry>> {
        Ok(self.entries.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use gpui::Context;
    use language::Buffer;
    use project::{FakeFs, Project};
    use settings::SettingsStore;
    use text::BufferId;
    use unindent::Unindent as _;

    use crate::git::blame::{FakeGitBlameRunner, GitBlame};

    fn init_test(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            theme::init(theme::LoadThemes::JustBase, cx);

            language::init(cx);
            client::init_settings(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);

            crate::init(cx);
        });
    }

    #[gpui::test]
    async fn test_blame_for_rows(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        // File on disk:
        //
        //     AAA Line 1
        //     BBB Line 2 - Modified 1
        //     CCC Line 3 - Modified 2
        //     DDD Line 4 - Modified 2
        //     EEE Line 5 - Modified 1
        //     FFF Line 6 - Modified 2
        //
        // File in memory:
        let content = r#"
               AAA Line 1
               BBB Line 2 - Modified 1
               CCC Line 3 - Modified 2
               modified in memory 1
               modified in memory 1
               DDD Line 4 - Modified 2
               EEE Line 5 - Modified 1
               FFF Line 6 - Modified 2
            "#
        .unindent();

        // What we want (output of `git blame --contents - file.txt`)
        //
        // 2fd52548 (Thorsten Ball              2024-03-20 14:28:27 +0100 1) AAA Line 1
        // 116b493a (Thorsten Ball              2024-03-20 14:28:51 +0100 2) BBB Line 2 - Modified 1
        // 0a851b8c (Thorsten Ball              2024-03-20 14:29:19 +0100 3) CCC Line 3 - Modified 2
        // 00000000 (External file (--contents) 2024-03-20 14:32:09 +0100 4) modified in memory 1
        // 00000000 (External file (--contents) 2024-03-20 14:32:09 +0100 5) modified in memory 1
        // 0a851b8c (Thorsten Ball              2024-03-20 14:29:19 +0100 6) DDD Line 4 - Modified 2
        // 116b493a (Thorsten Ball              2024-03-20 14:28:51 +0100 7) EEE Line 5 - Modified 1
        // 0a851b8c (Thorsten Ball              2024-03-20 14:29:19 +0100 8) FFF Line 6 - Modified 2

        // TODO: Replace this with BlameEntry structs
        let blame_incremental_output = r#"
            0000000000000000000000000000000000000000 4 4 2
            author External file (--contents)
            author-mail <external.file>
            author-time 1710941616
            author-tz +0100
            committer External file (--contents)
            committer-mail <external.file>
            committer-time 1710941616
            committer-tz +0100
            summary Version of my_new_file.txt from standard input
            previous 0a851b8c934ddfd3c463da9cf767c544403b1a2e my_new_file.txt
            filename my_new_file.txt
            0a851b8c934ddfd3c463da9cf767c544403b1a2e 3 3 1
            author Thorsten Ball
            author-mail <mrnugget@gmail.com>
            author-time 1710941359
            author-tz +0100
            committer Thorsten Ball
            committer-mail <mrnugget@gmail.com>
            committer-time 1710941359
            committer-tz +0100
            summary Yet another commit
            previous 116b493ab5349020da89547c9417a1d26cdbf337 my_new_file.txt
            filename my_new_file.txt
            0a851b8c934ddfd3c463da9cf767c544403b1a2e 4 6 1
            previous 116b493ab5349020da89547c9417a1d26cdbf337 my_new_file.txt
            filename my_new_file.txt
            0a851b8c934ddfd3c463da9cf767c544403b1a2e 6 8 1
            previous 116b493ab5349020da89547c9417a1d26cdbf337 my_new_file.txt
            filename my_new_file.txt
            116b493ab5349020da89547c9417a1d26cdbf337 2 2 1
            author Thorsten Ball
            author-mail <mrnugget@gmail.com>
            author-time 1710941331
            author-tz +0100
            committer Thorsten Ball
            committer-mail <mrnugget@gmail.com>
            committer-time 1710941331
            committer-tz +0100
            summary Another commit
            previous 2fd52548c580d5895e99d1a0b70244612ea7e0ee my_new_file.txt
            filename my_new_file.txt
            116b493ab5349020da89547c9417a1d26cdbf337 5 7 1
            previous 2fd52548c580d5895e99d1a0b70244612ea7e0ee my_new_file.txt
            filename my_new_file.txt
            2fd52548c580d5895e99d1a0b70244612ea7e0ee 1 1 1
            author Thorsten Ball
            author-mail <mrnugget@gmail.com>
            author-time 1710941307
            author-tz +0100
            committer Thorsten Ball
            committer-mail <mrnugget@gmail.com>
            committer-time 1710941307
            committer-tz +0100
            summary Another commit
            filename my_new_file.txt
            "#
        .unindent();

        let blame_entries = git::blame::parse_git_blame(&blame_incremental_output).unwrap();
        let blame_runner = Arc::from(FakeGitBlameRunner {
            entries: blame_entries,
        });

        let buffer = cx.new_model(|cx| {
            Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), content)
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, ["/file.txt".as_ref()], cx).await;

        let git_blame = cx.new_model(|cx| GitBlame::new(blame_runner, buffer, project, cx));

        cx.executor().run_until_parked();

        git_blame.update(cx, |blame, _| {
            assert!(!blame.entries.is_empty());

            let entries: Vec<_> = blame.blame_for_rows((0..8).map(Some)).collect();

            assert_eq!(entries.len(), 7);
        });
    }
}
