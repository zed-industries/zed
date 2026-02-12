use crate::Editor;
use anyhow::{Context as _, Result};
use collections::HashMap;

use git::{
    GitHostingProviderRegistry, Oid,
    blame::{Blame, BlameEntry},
    commit::ParsedCommitMessage,
};
use gpui::{
    AnyElement, App, AppContext as _, Context, Entity, Hsla, ScrollHandle, Subscription, Task,
    TextStyle, WeakEntity, Window,
};
use itertools::Itertools;
use language::{Bias, BufferSnapshot, Edit};
use markdown::Markdown;
use multi_buffer::{MultiBuffer, RowInfo};
use project::{
    Project, ProjectItem as _,
    git_store::{GitStoreEvent, Repository},
};
use smallvec::SmallVec;
use std::{sync::Arc, time::Duration};
use sum_tree::SumTree;
use text::BufferId;
use workspace::Workspace;

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

    fn summary(&self, _cx: ()) -> Self::Summary {
        GitBlameEntrySummary { rows: self.rows }
    }
}

impl sum_tree::ContextLessSummary for GitBlameEntrySummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        self.rows += summary.rows;
    }
}

impl<'a> sum_tree::Dimension<'a, GitBlameEntrySummary> for u32 {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a GitBlameEntrySummary, _cx: ()) {
        *self += summary.rows;
    }
}

struct GitBlameBuffer {
    entries: SumTree<GitBlameEntry>,
    buffer_snapshot: BufferSnapshot,
    buffer_edits: text::Subscription<usize>,
    commit_details: HashMap<Oid, ParsedCommitMessage>,
}

pub struct GitBlame {
    project: Entity<Project>,
    multi_buffer: WeakEntity<MultiBuffer>,
    buffers: HashMap<BufferId, GitBlameBuffer>,
    task: Task<Result<()>>,
    focused: bool,
    changed_while_blurred: bool,
    user_triggered: bool,
    regenerate_on_edit_task: Task<Result<()>>,
    _regenerate_subscriptions: Vec<Subscription>,
}

pub trait BlameRenderer {
    fn max_author_length(&self) -> usize;

    fn render_blame_entry(
        &self,
        _: &TextStyle,
        _: BlameEntry,
        _: Option<ParsedCommitMessage>,
        _: Entity<Repository>,
        _: WeakEntity<Workspace>,
        _: Entity<Editor>,
        _: usize,
        _: Hsla,
        window: &mut Window,
        _: &mut App,
    ) -> Option<AnyElement>;

    fn render_inline_blame_entry(
        &self,
        _: &TextStyle,
        _: BlameEntry,
        _: &mut App,
    ) -> Option<AnyElement>;

    fn render_blame_entry_popover(
        &self,
        _: BlameEntry,
        _: ScrollHandle,
        _: Option<ParsedCommitMessage>,
        _: Entity<Markdown>,
        _: Entity<Repository>,
        _: WeakEntity<Workspace>,
        _: &mut Window,
        _: &mut App,
    ) -> Option<AnyElement>;

    fn open_blame_commit(
        &self,
        _: BlameEntry,
        _: Entity<Repository>,
        _: WeakEntity<Workspace>,
        _: &mut Window,
        _: &mut App,
    );
}

impl BlameRenderer for () {
    fn max_author_length(&self) -> usize {
        0
    }

    fn render_blame_entry(
        &self,
        _: &TextStyle,
        _: BlameEntry,
        _: Option<ParsedCommitMessage>,
        _: Entity<Repository>,
        _: WeakEntity<Workspace>,
        _: Entity<Editor>,
        _: usize,
        _: Hsla,
        _: &mut Window,
        _: &mut App,
    ) -> Option<AnyElement> {
        None
    }

    fn render_inline_blame_entry(
        &self,
        _: &TextStyle,
        _: BlameEntry,
        _: &mut App,
    ) -> Option<AnyElement> {
        None
    }

    fn render_blame_entry_popover(
        &self,
        _: BlameEntry,
        _: ScrollHandle,
        _: Option<ParsedCommitMessage>,
        _: Entity<Markdown>,
        _: Entity<Repository>,
        _: WeakEntity<Workspace>,
        _: &mut Window,
        _: &mut App,
    ) -> Option<AnyElement> {
        None
    }

    fn open_blame_commit(
        &self,
        _: BlameEntry,
        _: Entity<Repository>,
        _: WeakEntity<Workspace>,
        _: &mut Window,
        _: &mut App,
    ) {
    }
}

pub(crate) struct GlobalBlameRenderer(pub Arc<dyn BlameRenderer>);

impl gpui::Global for GlobalBlameRenderer {}

impl GitBlame {
    pub fn new(
        multi_buffer: Entity<MultiBuffer>,
        project: Entity<Project>,
        user_triggered: bool,
        focused: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        let multi_buffer_subscription = cx.subscribe(
            &multi_buffer,
            |git_blame, multi_buffer, event, cx| match event {
                multi_buffer::Event::DirtyChanged => {
                    if !multi_buffer.read(cx).is_dirty(cx) {
                        git_blame.generate(cx);
                    }
                }
                multi_buffer::Event::ExcerptsAdded { .. }
                | multi_buffer::Event::ExcerptsEdited { .. } => git_blame.regenerate_on_edit(cx),
                _ => {}
            },
        );

        let project_subscription = cx.subscribe(&project, {
            let multi_buffer = multi_buffer.downgrade();

            move |git_blame, _, event, cx| {
                if let project::Event::WorktreeUpdatedEntries(_, updated) = event {
                    let Some(multi_buffer) = multi_buffer.upgrade() else {
                        return;
                    };
                    let project_entry_id = multi_buffer
                        .read(cx)
                        .as_singleton()
                        .and_then(|it| it.read(cx).entry_id(cx));
                    if updated
                        .iter()
                        .any(|(_, entry_id, _)| project_entry_id == Some(*entry_id))
                    {
                        log::debug!("Updated buffers. Regenerating blame data...",);
                        git_blame.generate(cx);
                    }
                }
            }
        });

        let git_store = project.read(cx).git_store().clone();
        let git_store_subscription =
            cx.subscribe(&git_store, move |this, _, event, cx| match event {
                GitStoreEvent::RepositoryUpdated(_, _, _)
                | GitStoreEvent::RepositoryAdded
                | GitStoreEvent::RepositoryRemoved(_) => {
                    log::debug!("Status of git repositories updated. Regenerating blame data...",);
                    this.generate(cx);
                }
                _ => {}
            });

        let mut this = Self {
            project,
            multi_buffer: multi_buffer.downgrade(),
            buffers: HashMap::default(),
            user_triggered,
            focused,
            changed_while_blurred: false,
            task: Task::ready(Ok(())),
            regenerate_on_edit_task: Task::ready(Ok(())),
            _regenerate_subscriptions: vec![
                multi_buffer_subscription,
                project_subscription,
                git_store_subscription,
            ],
        };
        this.generate(cx);
        this
    }

    pub fn repository(&self, cx: &App, id: BufferId) -> Option<Entity<Repository>> {
        self.project
            .read(cx)
            .git_store()
            .read(cx)
            .repository_and_path_for_buffer_id(id, cx)
            .map(|(repo, _)| repo)
    }

    pub fn has_generated_entries(&self) -> bool {
        !self.buffers.is_empty()
    }

    pub fn details_for_entry(
        &self,
        buffer: BufferId,
        entry: &BlameEntry,
    ) -> Option<ParsedCommitMessage> {
        self.buffers
            .get(&buffer)?
            .commit_details
            .get(&entry.sha)
            .cloned()
    }

    pub fn blame_for_rows<'a>(
        &'a mut self,
        rows: &'a [RowInfo],
        cx: &'a mut App,
    ) -> impl Iterator<Item = Option<(BufferId, BlameEntry)>> + use<'a> {
        rows.iter().map(move |info| {
            let buffer_id = info.buffer_id?;
            self.sync(cx, buffer_id);

            let buffer_row = info.buffer_row?;
            let mut cursor = self.buffers.get(&buffer_id)?.entries.cursor::<u32>(());
            cursor.seek_forward(&buffer_row, Bias::Right);
            Some((buffer_id, cursor.item()?.blame.clone()?))
        })
    }

    pub fn max_author_length(&mut self, cx: &mut App) -> usize {
        let mut max_author_length = 0;
        self.sync_all(cx);

        for buffer in self.buffers.values() {
            for entry in buffer.entries.iter() {
                let author_len = entry
                    .blame
                    .as_ref()
                    .and_then(|entry| entry.author.as_ref())
                    .map(|author| author.len());
                if let Some(author_len) = author_len
                    && author_len > max_author_length
                {
                    max_author_length = author_len;
                }
            }
        }

        max_author_length
    }

    pub fn blur(&mut self, _: &mut Context<Self>) {
        self.focused = false;
    }

    pub fn focus(&mut self, cx: &mut Context<Self>) {
        if self.focused {
            return;
        }
        self.focused = true;
        if self.changed_while_blurred {
            self.changed_while_blurred = false;
            self.generate(cx);
        }
    }

    fn sync_all(&mut self, cx: &mut App) {
        let Some(multi_buffer) = self.multi_buffer.upgrade() else {
            return;
        };
        multi_buffer
            .read(cx)
            .excerpt_buffer_ids()
            .into_iter()
            .for_each(|id| self.sync(cx, id));
    }

    fn sync(&mut self, cx: &mut App, buffer_id: BufferId) {
        let Some(blame_buffer) = self.buffers.get_mut(&buffer_id) else {
            return;
        };
        let Some(buffer) = self
            .multi_buffer
            .upgrade()
            .and_then(|multi_buffer| multi_buffer.read(cx).buffer(buffer_id))
        else {
            return;
        };
        let edits = blame_buffer.buffer_edits.consume();
        let new_snapshot = buffer.read(cx).snapshot();

        let mut row_edits = edits
            .into_iter()
            .map(|edit| {
                let old_point_range = blame_buffer.buffer_snapshot.offset_to_point(edit.old.start)
                    ..blame_buffer.buffer_snapshot.offset_to_point(edit.old.end);
                let new_point_range = new_snapshot.offset_to_point(edit.new.start)
                    ..new_snapshot.offset_to_point(edit.new.end);

                if old_point_range.start.column
                    == blame_buffer
                        .buffer_snapshot
                        .line_len(old_point_range.start.row)
                    && (new_snapshot.chars_at(edit.new.start).next() == Some('\n')
                        || blame_buffer
                            .buffer_snapshot
                            .line_len(old_point_range.end.row)
                            == 0)
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

        let mut new_entries = SumTree::default();
        let mut cursor = blame_buffer.entries.cursor::<u32>(());

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

            new_entries.append(cursor.slice(&edit.old.start, Bias::Right), ());

            if edit.new.start > new_entries.summary().rows {
                new_entries.push(
                    GitBlameEntry {
                        rows: edit.new.start - new_entries.summary().rows,
                        blame: cursor.item().and_then(|entry| entry.blame.clone()),
                    },
                    (),
                );
            }

            cursor.seek(&edit.old.end, Bias::Right);
            if !edit.new.is_empty() {
                new_entries.push(
                    GitBlameEntry {
                        rows: edit.new.len() as u32,
                        blame: None,
                    },
                    (),
                );
            }

            let old_end = cursor.end();
            if row_edits
                .peek()
                .is_none_or(|next_edit| next_edit.old.start >= old_end)
                && let Some(entry) = cursor.item()
            {
                if old_end > edit.old.end {
                    new_entries.push(
                        GitBlameEntry {
                            rows: cursor.end() - edit.old.end,
                            blame: entry.blame.clone(),
                        },
                        (),
                    );
                }

                cursor.next();
            }
        }
        new_entries.append(cursor.suffix(), ());
        drop(cursor);

        blame_buffer.buffer_snapshot = new_snapshot;
        blame_buffer.entries = new_entries;
    }

    #[cfg(test)]
    fn check_invariants(&mut self, cx: &mut Context<Self>) {
        self.sync_all(cx);
        for (&id, buffer) in &self.buffers {
            assert_eq!(
                buffer.entries.summary().rows,
                self.multi_buffer
                    .upgrade()
                    .unwrap()
                    .read(cx)
                    .buffer(id)
                    .unwrap()
                    .read(cx)
                    .max_point()
                    .row
                    + 1
            );
        }
    }

    #[ztracing::instrument(skip_all)]
    fn generate(&mut self, cx: &mut Context<Self>) {
        if !self.focused {
            self.changed_while_blurred = true;
            return;
        }
        let buffers_to_blame = self
            .multi_buffer
            .update(cx, |multi_buffer, _| {
                multi_buffer
                    .all_buffer_ids()
                    .into_iter()
                    .filter_map(|id| Some(multi_buffer.buffer(id)?.downgrade()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let project = self.project.downgrade();

        self.task = cx.spawn(async move |this, cx| {
            let mut all_results = Vec::new();
            let mut all_errors = Vec::new();

            for buffers in buffers_to_blame.chunks(4) {
                let span = ztracing::debug_span!("for each chunk of buffers");
                let _enter = span.enter();
                let blame = cx.update(|cx| {
                    buffers
                        .iter()
                        .map(|buffer| {
                            let buffer = buffer.upgrade().context("buffer was dropped")?;
                            let project = project.upgrade().context("project was dropped")?;
                            let id = buffer.read(cx).remote_id();
                            let snapshot = buffer.read(cx).snapshot();
                            let buffer_edits = buffer.update(cx, |buffer, _| buffer.subscribe());
                            let remote_url = project
                                .read(cx)
                                .git_store()
                                .read(cx)
                                .repository_and_path_for_buffer_id(buffer.read(cx).remote_id(), cx)
                                .and_then(|(repo, _)| repo.read(cx).default_remote_url());
                            let blame_buffer = project
                                .update(cx, |project, cx| project.blame_buffer(&buffer, None, cx));
                            Ok(async move {
                                (id, snapshot, buffer_edits, blame_buffer.await, remote_url)
                            })
                        })
                        .collect::<Result<Vec<_>>>()
                })?;
                let provider_registry =
                    cx.update(|cx| GitHostingProviderRegistry::default_global(cx));
                let (results, errors) = cx
                    .background_spawn({
                        async move {
                            let blame = futures::future::join_all(blame).await;
                            let mut res = vec![];
                            let mut errors = vec![];
                            for (id, snapshot, buffer_edits, blame, remote_url) in blame {
                                match blame {
                                    Ok(Some(Blame { entries, messages })) => {
                                        let entries = build_blame_entry_sum_tree(
                                            entries,
                                            snapshot.max_point().row,
                                        );
                                        let commit_details = messages
                                            .into_iter()
                                            .map(|(oid, message)| {
                                                let parsed_commit_message =
                                                    ParsedCommitMessage::parse(
                                                        oid.to_string(),
                                                        message,
                                                        remote_url.as_deref(),
                                                        Some(provider_registry.clone()),
                                                    );
                                                (oid, parsed_commit_message)
                                            })
                                            .collect();
                                        res.push((
                                            id,
                                            snapshot,
                                            buffer_edits,
                                            Some(entries),
                                            commit_details,
                                        ));
                                    }
                                    Ok(None) => res.push((
                                        id,
                                        snapshot,
                                        buffer_edits,
                                        None,
                                        Default::default(),
                                    )),
                                    Err(e) => errors.push(e),
                                }
                            }
                            (res, errors)
                        }
                    })
                    .await;
                all_results.extend(results);
                all_errors.extend(errors)
            }

            this.update(cx, |this, cx| {
                this.buffers.clear();
                for (id, snapshot, buffer_edits, entries, commit_details) in all_results {
                    let Some(entries) = entries else {
                        continue;
                    };
                    this.buffers.insert(
                        id,
                        GitBlameBuffer {
                            buffer_edits,
                            buffer_snapshot: snapshot,
                            entries,
                            commit_details,
                        },
                    );
                }
                cx.notify();
                if !all_errors.is_empty() {
                    this.project.update(cx, |_, cx| {
                        let all_errors = all_errors
                            .into_iter()
                            .map(|e| format!("{e:#}"))
                            .dedup()
                            .collect::<Vec<_>>();
                        let all_errors = all_errors.join(", ");
                        if this.user_triggered {
                            log::error!("failed to get git blame data: {all_errors}");
                            cx.emit(project::Event::Toast {
                                notification_id: "git-blame".into(),
                                message: all_errors,
                                link: None,
                            });
                        } else {
                            // If we weren't triggered by a user, we just log errors in the background, instead of sending
                            // notifications.
                            log::debug!("failed to get git blame data: {all_errors}");
                        }
                    })
                }
            })
        });
    }

    fn regenerate_on_edit(&mut self, cx: &mut Context<Self>) {
        // todo(lw): hot foreground spawn
        self.regenerate_on_edit_task = cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(REGENERATE_ON_EDIT_DEBOUNCE_INTERVAL)
                .await;

            this.update(cx, |this, cx| {
                this.generate(cx);
            })
        });
    }
}

const REGENERATE_ON_EDIT_DEBOUNCE_INTERVAL: Duration = Duration::from_secs(2);

fn build_blame_entry_sum_tree(entries: Vec<BlameEntry>, max_row: u32) -> SumTree<GitBlameEntry> {
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
        (),
    );

    if max_row >= current_row {
        entries.push(
            GitBlameEntry {
                rows: (max_row + 1) - current_row,
                blame: None,
            },
            (),
        );
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use git::repository::repo_path;
    use gpui::Context;
    use language::{Point, Rope};
    use project::FakeFs;
    use rand::prelude::*;
    use serde_json::json;
    use settings::SettingsStore;
    use std::{cmp, env, ops::Range, path::Path};
    use text::BufferId;
    use unindent::Unindent as _;
    use util::{RandomCharIter, path};

    // macro_rules! assert_blame_rows {
    //     ($blame:expr, $rows:expr, $expected:expr, $cx:expr) => {
    //         assert_eq!(
    //             $blame
    //                 .blame_for_rows($rows.map(MultiBufferRow).map(Some), $cx)
    //                 .collect::<Vec<_>>(),
    //             $expected
    //         );
    //     };
    // }

    #[track_caller]
    fn assert_blame_rows(
        blame: &mut GitBlame,
        buffer_id: BufferId,
        rows: Range<u32>,
        expected: Vec<Option<BlameEntry>>,
        cx: &mut Context<GitBlame>,
    ) {
        pretty_assertions::assert_eq!(
            blame
                .blame_for_rows(
                    &rows
                        .map(|row| RowInfo {
                            buffer_row: Some(row),
                            buffer_id: Some(buffer_id),
                            ..Default::default()
                        })
                        .collect::<Vec<_>>(),
                    cx
                )
                .collect::<Vec<_>>(),
            expected
                .into_iter()
                .map(|it| Some((buffer_id, it?)))
                .collect::<Vec<_>>()
        );
    }

    fn init_test(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            theme::init(theme::LoadThemes::JustBase, cx);

            crate::init(cx);
        });
    }

    #[gpui::test]
    async fn test_blame_error_notifications(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/my-repo",
            json!({
                ".git": {},
                "file.txt": r#"
                    irrelevant contents
                "#
                .unindent()
            }),
        )
        .await;

        // Creating a GitBlame without a corresponding blame state
        // will result in an error.

        let project = Project::test(fs, ["/my-repo".as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/my-repo/file.txt", cx)
            })
            .await
            .unwrap();
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let blame = cx.new(|cx| GitBlame::new(buffer.clone(), project.clone(), true, true, cx));

        let event = project.next_event(cx).await;
        assert_eq!(
            event,
            project::Event::Toast {
                notification_id: "git-blame".into(),
                message: "Failed to blame \"file.txt\": failed to get blame for \"file.txt\""
                    .to_string(),
                link: None
            }
        );

        blame.update(cx, |blame, cx| {
            assert_eq!(
                blame
                    .blame_for_rows(
                        &(0..1)
                            .map(|row| RowInfo {
                                buffer_row: Some(row),
                                ..Default::default()
                            })
                            .collect::<Vec<_>>(),
                        cx
                    )
                    .collect::<Vec<_>>(),
                vec![None]
            );
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
                repo_path("file.txt"),
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
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let git_blame = cx.new(|cx| GitBlame::new(buffer.clone(), project, false, true, cx));

        cx.executor().run_until_parked();

        git_blame.update(cx, |blame, cx| {
            // All lines
            pretty_assertions::assert_eq!(
                blame
                    .blame_for_rows(
                        &(0..8)
                            .map(|buffer_row| RowInfo {
                                buffer_row: Some(buffer_row),
                                buffer_id: Some(buffer_id),
                                ..Default::default()
                            })
                            .collect::<Vec<_>>(),
                        cx
                    )
                    .collect::<Vec<_>>(),
                vec![
                    Some((buffer_id, blame_entry("1b1b1b", 0..1))),
                    Some((buffer_id, blame_entry("0d0d0d", 1..2))),
                    Some((buffer_id, blame_entry("3a3a3a", 2..3))),
                    None,
                    None,
                    Some((buffer_id, blame_entry("3a3a3a", 5..6))),
                    Some((buffer_id, blame_entry("0d0d0d", 6..7))),
                    Some((buffer_id, blame_entry("3a3a3a", 7..8))),
                ]
            );
            // Subset of lines
            pretty_assertions::assert_eq!(
                blame
                    .blame_for_rows(
                        &(1..4)
                            .map(|buffer_row| RowInfo {
                                buffer_row: Some(buffer_row),
                                buffer_id: Some(buffer_id),
                                ..Default::default()
                            })
                            .collect::<Vec<_>>(),
                        cx
                    )
                    .collect::<Vec<_>>(),
                vec![
                    Some((buffer_id, blame_entry("0d0d0d", 1..2))),
                    Some((buffer_id, blame_entry("3a3a3a", 2..3))),
                    None
                ]
            );
            // Subset of lines, with some not displayed
            pretty_assertions::assert_eq!(
                blame
                    .blame_for_rows(
                        &[
                            RowInfo {
                                buffer_row: Some(1),
                                buffer_id: Some(buffer_id),
                                ..Default::default()
                            },
                            Default::default(),
                            Default::default(),
                        ],
                        cx
                    )
                    .collect::<Vec<_>>(),
                vec![Some((buffer_id, blame_entry("0d0d0d", 1..2))), None, None]
            );
        });
    }

    #[gpui::test]
    async fn test_blame_for_rows_with_edits(cx: &mut gpui::TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/my-repo"),
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
            Path::new(path!("/my-repo/.git")),
            vec![(
                repo_path("file.txt"),
                Blame {
                    entries: vec![blame_entry("1b1b1b", 0..4)],
                    ..Default::default()
                },
            )],
        );

        let project = Project::test(fs, [path!("/my-repo").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/my-repo/file.txt"), cx)
            })
            .await
            .unwrap();
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        let git_blame = cx.new(|cx| GitBlame::new(buffer.clone(), project, false, true, cx));

        cx.executor().run_until_parked();

        git_blame.update(cx, |blame, cx| {
            // Sanity check before edits: make sure that we get the same blame entry for all
            // lines.
            assert_blame_rows(
                blame,
                buffer_id,
                0..4,
                vec![
                    Some(blame_entry("1b1b1b", 0..4)),
                    Some(blame_entry("1b1b1b", 0..4)),
                    Some(blame_entry("1b1b1b", 0..4)),
                    Some(blame_entry("1b1b1b", 0..4)),
                ],
                cx,
            );
        });

        // Modify a single line, at the start of the line
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(0, 0)..Point::new(0, 0), "X")], None, cx);
        });
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows(
                blame,
                buffer_id,
                0..2,
                vec![None, Some(blame_entry("1b1b1b", 0..4))],
                cx,
            );
        });
        // Modify a single line, in the middle of the line
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(1, 2)..Point::new(1, 2), "X")], None, cx);
        });
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows(
                blame,
                buffer_id,
                1..4,
                vec![
                    None,
                    Some(blame_entry("1b1b1b", 0..4)),
                    Some(blame_entry("1b1b1b", 0..4)),
                ],
                cx,
            );
        });

        // Before we insert a newline at the end, sanity check:
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows(
                blame,
                buffer_id,
                3..4,
                vec![Some(blame_entry("1b1b1b", 0..4))],
                cx,
            );
        });
        // Insert a newline at the end
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(3, 6)..Point::new(3, 6), "\n")], None, cx);
        });
        // Only the new line is marked as edited:
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows(
                blame,
                buffer_id,
                3..5,
                vec![Some(blame_entry("1b1b1b", 0..4)), None],
                cx,
            );
        });

        // Before we insert a newline at the start, sanity check:
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows(
                blame,
                buffer_id,
                2..3,
                vec![Some(blame_entry("1b1b1b", 0..4))],
                cx,
            );
        });

        // Usage example
        // Insert a newline at the start of the row
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(Point::new(2, 0)..Point::new(2, 0), "\n")], None, cx);
        });
        // Only the new line is marked as edited:
        git_blame.update(cx, |blame, cx| {
            assert_blame_rows(
                blame,
                buffer_id,
                2..4,
                vec![None, Some(blame_entry("1b1b1b", 0..4))],
                cx,
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
        let buffer_initial_text_len = rng.random_range(5..15);
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
            path!("/my-repo"),
            json!({
                ".git": {},
                "file.txt": buffer_initial_text.to_string()
            }),
        )
        .await;

        let blame_entries = gen_blame_entries(buffer_initial_text.max_point().row, &mut rng);
        log::info!("initial blame entries: {:?}", blame_entries);
        fs.set_blame_for_repo(
            Path::new(path!("/my-repo/.git")),
            vec![(
                repo_path("file.txt"),
                Blame {
                    entries: blame_entries,
                    ..Default::default()
                },
            )],
        );

        let project = Project::test(fs.clone(), [path!("/my-repo").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/my-repo/file.txt"), cx)
            })
            .await
            .unwrap();
        let mbuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        let git_blame = cx.new(|cx| GitBlame::new(mbuffer.clone(), project, false, true, cx));
        cx.executor().run_until_parked();
        git_blame.update(cx, |blame, cx| blame.check_invariants(cx));

        for _ in 0..operations {
            match rng.random_range(0..100) {
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
                        Path::new(path!("/my-repo/.git")),
                        vec![(
                            repo_path("file.txt"),
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
                let row_start = rng.random_range(last_row..max_row);
                let row_end = rng.random_range(row_start + 1..cmp::min(row_start + 3, max_row) + 1);
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
