use anyhow::Result;
use language::Buffer;
use language::BufferSnapshot;
use sum_tree::SumTree;
use text::Bias;
use text::Edit;

use git::blame::BlameEntry;
use gpui::{Model, ModelContext, Subscription, Task};
use project::{Item, Project};
use smallvec::SmallVec;

#[derive(Clone, Debug, Default)]
pub struct GitBlameEntry {
    pub rows: u32,
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
                    this.generate(cx);
                }
            }
            project::Event::WorktreeUpdatedGitRepositories(_) => {
                log::debug!("Status of git repositories updated. Regenerating blame data...",);
                this.generate(cx);
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

    fn generate(&mut self, cx: &mut ModelContext<Self>) {
        let buffer_edits = self.buffer.update(cx, |buffer, _| buffer.subscribe());
        let snapshot = self.buffer.read(cx).snapshot();
        let blame_entries = self.project.read(cx).blame_buffer(&self.buffer, cx);

        self.task = cx.spawn(|this, mut cx| async move {
            let entries = cx
                .background_executor()
                .spawn({
                    let snapshot = snapshot.clone();
                    async move {
                        let blame_entries = blame_entries.await?;

                        let mut current_row = 0;
                        let mut entries = SumTree::from_iter(
                            blame_entries.into_iter().flat_map(|entry| {
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

                        let max_row = snapshot.max_point().row;
                        if max_row > current_row {
                            entries.push(
                                GitBlameEntry {
                                    rows: max_row - current_row,
                                    blame: None,
                                },
                                &(),
                            );
                        }

                        anyhow::Ok(entries)
                    }
                })
                .await?;

            this.update(&mut cx, |this, cx| {
                this.buffer_edits = buffer_edits;
                this.buffer_snapshot = snapshot;
                this.entries = entries;
                cx.notify();
            })
        });
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Range, sync::Arc};

    use git::blame::BlameEntry;
    use gpui::Context;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use text::Point;
    use unindent::Unindent as _;

    use crate::git::blame::GitBlame;

    macro_rules! assert_blame_rows {
        ($blame:expr, $rows:expr, $expected:expr, $cx:expr) => {
            assert_eq!(
                $blame
                    .blame_for_rows($rows.map(Some), $cx)
                    .collect::<Vec<_>>(),
                $expected
            );
        };
    }

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
    }

    #[gpui::test]
    async fn test_blame_for_rows_with_edits(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/",
            json!({
                "/file.txt": r#"
               Line 1
               Line 2
               Line 3
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

        let blame_entries = vec![blame_entry("1b1b1b", 0..4)];
        let blame_runner = Arc::from(FakeGitBlameRunner {
            entries: blame_entries,
        });
        let git_blame = cx.new_model(|cx| GitBlame::new(blame_runner, buffer.clone(), project, cx));

        cx.executor().run_until_parked();

        git_blame.update(cx, |blame, cx| {
            // Sanity check before edits: make sure that we get the same blame entry for all
            // lines.
            assert_blame_rows!(
                blame,
                (0..4),
                vec![
                    Some(blame_entry("1b1b1b", 0..4)),
                    Some(blame_entry("1b1b1b", 0..4)),
                    Some(blame_entry("1b1b1b", 0..4)),
                    Some(blame_entry("1b1b1b", 0..4)),
                ],
                cx
            );
        });

        // Modify a single line
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 2)..Point::new(1, 2), "X")], None, cx);
        });
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows!(
                blame,
                (1..4),
                vec![
                    None,
                    Some(blame_entry("1b1b1b", 0..4)),
                    Some(blame_entry("1b1b1b", 0..4))
                ],
                cx
            );
        });

        // Before we insert a newline at the end, sanity check:
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows!(blame, (3..4), vec![Some(blame_entry("1b1b1b", 0..4))], cx);
        });
        // Insert a newline at the end
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(3, 6)..Point::new(3, 6), "\n")], None, cx);
        });
        // Only the new line is marked as edited:
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows!(
                blame,
                (3..5),
                vec![Some(blame_entry("1b1b1b", 0..4)), None],
                cx
            );
        });

        // Before we insert a newline at the start, sanity check:
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows!(blame, (2..3), vec![Some(blame_entry("1b1b1b", 0..4)),], cx);
        });

        // Usage example
        // Insert a newline at the start of the row
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "\n")], None, cx);
        });
        // Only the new line is marked as edited:
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows!(
                blame,
                (2..4),
                vec![None, Some(blame_entry("1b1b1b", 0..4)),],
                cx
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
