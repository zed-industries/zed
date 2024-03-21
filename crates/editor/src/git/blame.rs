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
use std::ops::Range;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct GitBlameEntry {
    pub rows: u32,
    // TODO: Do we want to remove rows from BlameEntry?
    pub blame: Option<BlameEntry>,
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

    pub fn blame_for_rows<'a>(
        &'a mut self,
        rows: impl 'a + IntoIterator<Item = Option<u32>>,
        cx: &mut ModelContext<Self>,
    ) -> impl 'a + Iterator<Item = Option<BlameEntry>> {
        self.sync(cx);

        let mut cursor = self.entries.cursor::<u32>();
        rows.into_iter().map(move |row| {
            let row = row?;
            cursor.seek(&row, Bias::Right, &());
            cursor.item()?.blame.clone()
        })
    }

    fn sync(&mut self, cx: &mut ModelContext<Self>) {
        let edits = self.buffer_edits.consume();
        let new_snapshot = self.buffer.read(cx).snapshot();

        let mut row_edits = edits
            .into_iter()
            .map(|edit| {
                let old_point_range = self.buffer_snapshot.offset_to_point(edit.old.start)
                    ..self.buffer_snapshot.offset_to_point(edit.old.end);
                let new_point_range = new_snapshot.offset_to_point(edit.new.start)
                    ..new_snapshot.offset_to_point(edit.new.end);

                if edit.old.is_empty()
                    && old_point_range.start.column
                        == self.buffer_snapshot.line_len(old_point_range.start.row)
                    && new_snapshot.chars_at(edit.new.start).next() == Some('\n')
                {
                    Edit {
                        old: old_point_range.start.row + 1..old_point_range.end.row + 1,
                        new: new_point_range.start.row + 1..new_point_range.end.row + 1,
                    }
                } else if old_point_range.start.column == 0 && old_point_range.end.column == 0 {
                    Edit {
                        old: old_point_range.start.row..old_point_range.end.row,
                        new: new_point_range.start.row..new_point_range.end.row,
                    }
                } else {
                    Edit {
                        old: old_point_range.start.row..old_point_range.end.row + 1,
                        new: new_point_range.start.row..new_point_range.end.row + 1,
                    }
                }
            })
            .peekable();

        let mut new_entries = SumTree::new();
        let mut cursor = self.entries.cursor::<u32>();

        while let Some(mut edit) = row_edits.next() {
            // Coalesce contiguous edits.
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
                        blame: cursor.item().and_then(|entry| entry.blame.clone()),
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

            if row_edits
                .peek()
                .map_or(true, |next_edit| next_edit.old.start > cursor.end(&()))
            {
                if let Some(entry) = cursor.item() {
                    new_entries.push(
                        GitBlameEntry {
                            rows: cursor.end(&()) - edit.old.end,
                            blame: entry.blame.clone(),
                        },
                        &(),
                    );
                    cursor.next(&());
                }
            }
        }
        new_entries.append(cursor.suffix(&()), &());
        drop(cursor);

        self.buffer_snapshot = new_snapshot;
        self.entries = new_entries;
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
                    let parsed_git_blame = blame_runner.run(
                        &working_directory,
                        &path,
                        &background_buffer_snapshot.as_rope().to_string(),
                    )?;

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
        let mut entries = git::blame::parse_git_blame(&output)?;
        entries.sort_unstable_by(|a, b| a.range.start.cmp(&b.range.start));
        Ok(entries)
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
    use std::{ops::Range, sync::Arc};

    use git::blame::BlameEntry;
    use gpui::Context;
    use language::Buffer;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use text::{BufferId, Point};
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

        // What we want (output of `git blame --contents - file.txt`)
        //
        // 1b1b1b (Thorsten Ball              2024-03-20 14:28:27 +0100 1) AAA Line 1
        // 0d0d0d (Thorsten Ball              2024-03-20 14:28:51 +0100 2) BBB Line 2 - Modified 1
        // 3a3a3a (Thorsten Ball              2024-03-20 14:29:19 +0100 3) CCC Line 3 - Modified 2
        // 000000                             2024-03-20 14:32:09 +0100 4) modified in memory 1
        // 000000                             2024-03-20 14:32:09 +0100 5) modified in memory 1
        // 3a3a3a (Thorsten Ball              2024-03-20 14:29:19 +0100 6) DDD Line 4 - Modified 2
        // 0d0d0d (Thorsten Ball              2024-03-20 14:28:51 +0100 7) EEE Line 5 - Modified 1
        // 3a3a3a (Thorsten Ball              2024-03-20 14:29:19 +0100 8) FFF Line 6 - Modified 2

        let blame_entries = vec![
            blame_entry("1b1b1b", 0..1),
            blame_entry("0d0d0d", 1..2),
            blame_entry("3a3a3a", 2..3),
            blame_entry("3a3a3a", 5..6),
            blame_entry("0d0d0d", 6..7),
            blame_entry("3a3a3a", 7..8),
        ];

        let blame_runner = Arc::from(FakeGitBlameRunner {
            entries: blame_entries,
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/",
            json!({
                "/file.txt": r#"
               AAA Line 1
               BBB Line 2 - Modified 1
               CCC Line 3 - Modified 2
               modified in memory 1
               modified in memory 1
               DDD Line 4 - Modified 2
               EEE Line 5 - Modified 1
               FFF Line 6 - Modified 2
            "#
                .unindent()
            }),
        )
        .await;

        let project = Project::test(fs, ["/file.txt".as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| project.open_local_buffer("/file.txt", cx))
            .await
            .unwrap();

        let git_blame = cx.new_model(|cx| GitBlame::new(blame_runner, buffer.clone(), project, cx));

        cx.executor().run_until_parked();

        git_blame.update(cx, |blame, cx| {
            // All lines
            assert_eq!(
                blame
                    .blame_for_rows((0..8).map(Some), cx)
                    .collect::<Vec<_>>(),
                vec![
                    Some(blame_entry("1b1b1b", 0..1)),
                    Some(blame_entry("0d0d0d", 1..2)),
                    Some(blame_entry("3a3a3a", 2..3)),
                    None,
                    None,
                    Some(blame_entry("3a3a3a", 5..6)),
                    Some(blame_entry("0d0d0d", 6..7)),
                    Some(blame_entry("3a3a3a", 7..8)),
                ]
            );
            // Subset of lines
            assert_eq!(
                blame
                    .blame_for_rows((1..4).map(Some), cx)
                    .collect::<Vec<_>>(),
                vec![
                    Some(blame_entry("0d0d0d", 1..2)),
                    Some(blame_entry("3a3a3a", 2..3)),
                    None
                ]
            );
            // Subset of lines, with some not displayed
            assert_eq!(
                blame
                    .blame_for_rows(vec![Some(1), None, None], cx)
                    .collect::<Vec<_>>(),
                vec![Some(blame_entry("0d0d0d", 1..2)), None, None]
            );
        });

        // Modify a single line
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 2)..Point::new(1, 2), "X")], None, cx);
        });
        git_blame.update(cx, |blame, cx| {
            assert_eq!(
                blame
                    .blame_for_rows((1..4).map(Some), cx)
                    .collect::<Vec<_>>(),
                vec![None, Some(blame_entry("3a3a3a", 2..3)), None]
            );
        });

        // Before we insert a newline at the end, sanity check:
        git_blame.update(cx, |blame, cx| {
            assert_eq!(
                blame
                    .blame_for_rows((7..8).map(Some), cx)
                    .collect::<Vec<_>>(),
                vec![Some(blame_entry("3a3a3a", 7..8)),]
            );
        });
        // Insert a newline at the end
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(7, 23)..Point::new(7, 23), "\n")], None, cx);
        });
        // Only the new line is marked as edited:
        git_blame.update(cx, |blame, cx| {
            assert_eq!(
                blame
                    .blame_for_rows((7..9).map(Some), cx)
                    .collect::<Vec<_>>(),
                vec![Some(blame_entry("3a3a3a", 7..8)), None]
            );
        });

        // Before we insert a newline at the start, sanity check:
        git_blame.update(cx, |blame, cx| {
            assert_eq!(
                blame
                    .blame_for_rows((2..3).map(Some), cx)
                    .collect::<Vec<_>>(),
                vec![Some(blame_entry("3a3a3a", 2..3)),]
            );
        });
        // Insert a newline at the start of the row
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "\n")], None, cx);
        });
        // Only the new line is marked as edited:
        git_blame.update(cx, |blame, cx| {
            assert_eq!(
                blame
                    .blame_for_rows((2..4).map(Some), cx)
                    .collect::<Vec<_>>(),
                vec![None, Some(blame_entry("3a3a3a", 2..3)),]
            );
        });
    }

    fn blame_entry(sha: &str, range: Range<u32>) -> BlameEntry {
        BlameEntry {
            sha: sha.parse().unwrap(),
            range,
            ..Default::default()
        }
    }
}
