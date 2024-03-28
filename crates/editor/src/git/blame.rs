use anyhow::Result;
use collections::HashMap;
use git::{
    blame::{Blame, BlameEntry},
    Oid,
};
use gpui::{Model, ModelContext, Subscription, Task};
use language::{Bias, Buffer, BufferSnapshot, Edit};
use project::{Item, Project};
use smallvec::SmallVec;
use sum_tree::SumTree;
use url::Url;

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
    permalinks: HashMap<Oid, Url>,
    messages: HashMap<Oid, String>,
    buffer_snapshot: BufferSnapshot,
    buffer_edits: text::Subscription,
    task: Task<Result<()>>,
    generated: bool,
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

        let refresh_subscription = cx.subscribe(&project, {
            let buffer = buffer.clone();

            move |this, _, event, cx| match event {
                project::Event::WorktreeUpdatedEntries(_, updated) => {
                    let project_entry_id = buffer.read(cx).entry_id(cx);
                    if updated
                        .iter()
                        .any(|(_, entry_id, _)| project_entry_id == Some(*entry_id))
                    {
                        log::debug!("Updated buffers. Regenerating blame data...",);
                        this.generate(cx);
                    }
                }
                project::Event::WorktreeUpdatedGitRepositories => {
                    log::debug!("Status of git repositories updated. Regenerating blame data...",);
                    this.generate(cx);
                }
                _ => {}
            }
        });

        let buffer_snapshot = buffer.read(cx).snapshot();
        let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());

        let mut this = Self {
            project,
            buffer,
            buffer_snapshot,
            entries,
            buffer_edits,
            permalinks: HashMap::default(),
            messages: HashMap::default(),
            task: Task::ready(Ok(())),
            generated: false,
            _refresh_subscription: refresh_subscription,
        };
        this.generate(cx);
        this
    }

    pub fn has_generated_entries(&self) -> bool {
        self.generated
    }

    pub fn permalink_for_entry(&self, entry: &BlameEntry) -> Option<Url> {
        self.permalinks.get(&entry.sha).cloned()
    }

    pub fn message_for_entry(&self, entry: &BlameEntry) -> Option<String> {
        self.messages.get(&entry.sha).cloned()
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
            cursor.seek_forward(&row, Bias::Right, &());
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

                if old_point_range.start.column
                    == self.buffer_snapshot.line_len(old_point_range.start.row)
                    && (new_snapshot.chars_at(edit.new.start).next() == Some('\n')
                        || self.buffer_snapshot.line_len(old_point_range.end.row) == 0)
                {
                    Edit {
                        old: old_point_range.start.row + 1..old_point_range.end.row + 1,
                        new: new_point_range.start.row + 1..new_point_range.end.row + 1,
                    }
                } else if old_point_range.start.column == 0
                    && old_point_range.end.column == 0
                    && new_point_range.end.column == 0
                {
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
            if !edit.new.is_empty() {
                new_entries.push(
                    GitBlameEntry {
                        rows: edit.new.len() as u32,
                        blame: None,
                    },
                    &(),
                );
            }

            let old_end = cursor.end(&());
            if row_edits
                .peek()
                .map_or(true, |next_edit| next_edit.old.start >= old_end)
            {
                if let Some(entry) = cursor.item() {
                    if old_end > edit.old.end {
                        new_entries.push(
                            GitBlameEntry {
                                rows: cursor.end(&()) - edit.old.end,
                                blame: entry.blame.clone(),
                            },
                            &(),
                        );
                    }

                    cursor.next(&());
                }
            }
        }
        new_entries.append(cursor.suffix(&()), &());
        drop(cursor);

        self.buffer_snapshot = new_snapshot;
        self.entries = new_entries;
    }

    #[cfg(test)]
    fn check_invariants(&mut self, cx: &mut ModelContext<Self>) {
        self.sync(cx);
        assert_eq!(
            self.entries.summary().rows,
            self.buffer.read(cx).max_point().row + 1
        );
    }

    fn generate(&mut self, cx: &mut ModelContext<Self>) {
        let buffer_edits = self.buffer.update(cx, |buffer, _| buffer.subscribe());
        let snapshot = self.buffer.read(cx).snapshot();
        let blame = self.project.read(cx).blame_buffer(&self.buffer, None, cx);

        self.task = cx.spawn(|this, mut cx| async move {
            let (entries, permalinks, messages) = cx
                .background_executor()
                .spawn({
                    let snapshot = snapshot.clone();
                    async move {
                        let Blame {
                            entries,
                            permalinks,
                            messages,
                        } = blame.await?;

                        let mut current_row = 0;
                        let mut entries = SumTree::from_iter(
                            entries.into_iter().flat_map(|entry| {
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
                        if max_row >= current_row {
                            entries.push(
                                GitBlameEntry {
                                    rows: (max_row + 1) - current_row,
                                    blame: None,
                                },
                                &(),
                            );
                        }

                        anyhow::Ok((entries, permalinks, messages))
                    }
                })
                .await?;

            this.update(&mut cx, |this, cx| {
                this.buffer_edits = buffer_edits;
                this.buffer_snapshot = snapshot;
                this.entries = entries;
                this.permalinks = permalinks;
                this.messages = messages;
                this.generated = true;
                cx.notify();
            })
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::Context;
    use language::{Point, Rope};
    use project::FakeFs;
    use rand::prelude::*;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{cmp, env, ops::Range, path::Path};
    use unindent::Unindent as _;
    use util::RandomCharIter;

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
            "/my-repo",
            json!({
                ".git": {},
                "file.txt": r#"
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

        fs.set_blame_for_repo(
            Path::new("/my-repo/.git"),
            vec![(
                Path::new("file.txt"),
                Blame {
                    entries: vec![
                        blame_entry("1b1b1b", 0..1),
                        blame_entry("0d0d0d", 1..2),
                        blame_entry("3a3a3a", 2..3),
                        blame_entry("3a3a3a", 5..6),
                        blame_entry("0d0d0d", 6..7),
                        blame_entry("3a3a3a", 7..8),
                    ],
                    ..Default::default()
                },
            )],
        );
        let project = Project::test(fs, ["/my-repo".as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/my-repo/file.txt", cx)
            })
            .await
            .unwrap();

        let git_blame = cx.new_model(|cx| GitBlame::new(buffer.clone(), project, cx));

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
            "/my-repo",
            json!({
                ".git": {},
                "file.txt": r#"
                    Line 1
                    Line 2
                    Line 3
                "#
                .unindent()
            }),
        )
        .await;

        fs.set_blame_for_repo(
            Path::new("/my-repo/.git"),
            vec![(
                Path::new("file.txt"),
                Blame {
                    entries: vec![blame_entry("1b1b1b", 0..4)],
                    ..Default::default()
                },
            )],
        );

        let project = Project::test(fs, ["/my-repo".as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/my-repo/file.txt", cx)
            })
            .await
            .unwrap();

        let git_blame = cx.new_model(|cx| GitBlame::new(buffer.clone(), project, cx));

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

        // Modify a single line, at the start of the line
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 0)..Point::new(0, 0), "X")], None, cx);
        });
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows!(
                blame,
                (0..2),
                vec![None, Some(blame_entry("1b1b1b", 0..4))],
                cx
            );
        });
        // Modify a single line, in the middle of the line
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

    #[gpui::test(iterations = 100)]
    async fn test_blame_random(mut rng: StdRng, cx: &mut gpui::TestAppContext) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);
        let max_edits_per_operation = env::var("MAX_EDITS_PER_OPERATION")
            .map(|i| {
                i.parse()
                    .expect("invalid `MAX_EDITS_PER_OPERATION` variable")
            })
            .unwrap_or(5);

        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        let buffer_initial_text_len = rng.gen_range(5..15);
        let mut buffer_initial_text = Rope::from(
            RandomCharIter::new(&mut rng)
                .take(buffer_initial_text_len)
                .collect::<String>()
                .as_str(),
        );

        let mut newline_ixs = (0..buffer_initial_text_len).choose_multiple(&mut rng, 5);
        newline_ixs.sort_unstable();
        for newline_ix in newline_ixs.into_iter().rev() {
            let newline_ix = buffer_initial_text.clip_offset(newline_ix, Bias::Right);
            buffer_initial_text.replace(newline_ix..newline_ix, "\n");
        }
        log::info!("initial buffer text: {:?}", buffer_initial_text);

        fs.insert_tree(
            "/my-repo",
            json!({
                ".git": {},
                "file.txt": buffer_initial_text.to_string()
            }),
        )
        .await;

        let blame_entries = gen_blame_entries(buffer_initial_text.max_point().row, &mut rng);
        log::info!("initial blame entries: {:?}", blame_entries);
        fs.set_blame_for_repo(
            Path::new("/my-repo/.git"),
            vec![(
                Path::new("file.txt"),
                Blame {
                    entries: blame_entries,
                    ..Default::default()
                },
            )],
        );

        let project = Project::test(fs.clone(), ["/my-repo".as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/my-repo/file.txt", cx)
            })
            .await
            .unwrap();

        let git_blame = cx.new_model(|cx| GitBlame::new(buffer.clone(), project, cx));
        cx.executor().run_until_parked();
        git_blame.update(cx, |blame, cx| blame.check_invariants(cx));

        for _ in 0..operations {
            match rng.gen_range(0..100) {
                0..=19 => {
                    log::info!("quiescing");
                    cx.executor().run_until_parked();
                }
                20..=69 => {
                    log::info!("editing buffer");
                    buffer.update(cx, |buffer, cx| {
                        buffer.randomly_edit(&mut rng, max_edits_per_operation, cx);
                        log::info!("buffer text: {:?}", buffer.text());
                    });

                    let blame_entries = gen_blame_entries(
                        buffer.read_with(cx, |buffer, _| buffer.max_point().row),
                        &mut rng,
                    );
                    log::info!("regenerating blame entries: {:?}", blame_entries);

                    fs.set_blame_for_repo(
                        Path::new("/my-repo/.git"),
                        vec![(
                            Path::new("file.txt"),
                            Blame {
                                entries: blame_entries,
                                ..Default::default()
                            },
                        )],
                    );
                }
                _ => {
                    git_blame.update(cx, |blame, cx| blame.check_invariants(cx));
                }
            }
        }

        git_blame.update(cx, |blame, cx| blame.check_invariants(cx));
    }

    fn gen_blame_entries(max_row: u32, rng: &mut StdRng) -> Vec<BlameEntry> {
        let mut last_row = 0;
        let mut blame_entries = Vec::new();
        for ix in 0..5 {
            if last_row < max_row {
                let row_start = rng.gen_range(last_row..max_row);
                let row_end = rng.gen_range(row_start + 1..cmp::min(row_start + 3, max_row) + 1);
                blame_entries.push(blame_entry(&ix.to_string(), row_start..row_end));
                last_row = row_end;
            } else {
                break;
            }
        }
        blame_entries
    }

    fn blame_entry(sha: &str, range: Range<u32>) -> BlameEntry {
        BlameEntry {
            sha: sha.parse().unwrap(),
            range,
            ..Default::default()
        }
    }
}
