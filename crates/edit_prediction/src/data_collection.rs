use crate::{EditPredictionStore, StoredEvent};
use buffer_diff::BufferDiffSnapshot;
use collections::HashMap;
use gpui::{Context, Entity, Task};
use language::BufferSnapshot;
use project::{Project, WorktreeId};
use std::{fmt::Write as _, ops::Range, path::Path, sync::Arc};
use text::{OffsetRangeExt, Point};

pub type UncommittedDiffSnapshot = Vec<(Arc<Path>, BufferSnapshot, BufferDiffSnapshot)>;

pub fn uncommitted_diffs_for_events(
    project: Entity<Project>,
    worktree_id: WorktreeId,
    events: Vec<StoredEvent>,
    cx: &Context<'_, EditPredictionStore>,
) -> Task<Option<(UncommittedDiffSnapshot, Vec<StoredEvent>)>> {
    let project_id = project.entity_id();
    let git_store = project.read_with(cx, |project, _| project.git_store().clone());

    cx.spawn(async move |store, cx| {
        let events_with_paths = events
            .into_iter()
            .filter_map(|stored_event| {
                let zeta_prompt::Event::BufferChange { path, .. } = stored_event.event.as_ref();
                project
                    .read_with(cx, |project, cx| {
                        let project_path = project
                            .find_project_path(path, cx)
                            .filter(|path| path.worktree_id == worktree_id)?;
                        let relative_path: Arc<Path> = project_path.path.as_std_path().into();
                        Some((project_path, relative_path))
                    })
                    .map(|(project_path, relative_path)| {
                        (stored_event, project_path, relative_path)
                    })
            })
            .collect::<Vec<_>>();

        let mut snapshots_by_path: HashMap<Arc<Path>, (BufferSnapshot, BufferDiffSnapshot)> =
            HashMap::default();
        for (stored_event, project_path, relative_path) in events_with_paths.iter().rev() {
            if snapshots_by_path.contains_key(relative_path) {
                continue;
            }

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })
                .await
                .ok()?;
            let old_snapshot_remote_id = stored_event.old_snapshot.remote_id();
            let new_snapshot_version = stored_event.new_snapshot_version.clone();
            let zeta_prompt::Event::BufferChange {
                path: event_path, ..
            } = stored_event.event.as_ref();
            let cached_diff = stored_event.uncommitted_diff.clone().or_else(|| {
                store
                    .update(cx, |store, _| {
                        store
                            .projects
                            .get(&project_id)?
                            .events
                            .iter()
                            .find(|event| {
                                event.old_snapshot.remote_id() == old_snapshot_remote_id
                                    && event.new_snapshot_version == new_snapshot_version
                            })?
                            .uncommitted_diff
                            .clone()
                    })
                    .ok()
                    .flatten()
            });
            let diff = match cached_diff {
                Some(diff) => diff,
                None => {
                    let diff = git_store
                        .update(cx, |git_store, cx| {
                            git_store.open_uncommitted_diff(buffer.clone(), cx)
                        })
                        .await
                        .ok()?;
                    store
                        .update(cx, |store, _| {
                            if let Some(project) = store.projects.get_mut(&project_id) {
                                for event in project.events.iter_mut() {
                                    let zeta_prompt::Event::BufferChange { path, .. } =
                                        event.event.as_ref();
                                    if old_snapshot_remote_id == event.old_snapshot.remote_id()
                                        && path == event_path
                                    {
                                        event.uncommitted_diff = Some(diff.clone());
                                    }
                                }
                            }
                        })
                        .ok()?;
                    diff
                }
            };

            let buffer_snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
            let diff_snapshot = diff.update(cx, |diff, cx| diff.snapshot(cx));
            snapshots_by_path.insert(relative_path.clone(), (buffer_snapshot, diff_snapshot));
        }

        let events = events_with_paths
            .into_iter()
            .filter_map(|(stored_event, _, relative_path)| {
                snapshots_by_path
                    .contains_key(&relative_path)
                    .then_some(stored_event)
            })
            .collect();

        let uncommitted_diff_snapshots = snapshots_by_path
            .into_iter()
            .map(|(relative_path, (snapshot, diff_snapshot))| {
                (relative_path, snapshot, diff_snapshot)
            })
            .collect();

        Some((uncommitted_diff_snapshots, events))
    })
}

pub fn compute_uncommitted_diff(snapshot: UncommittedDiffSnapshot) -> String {
    let mut uncommitted_diff = String::new();
    let mut snapshots_by_path = snapshot;
    snapshots_by_path.sort_by(|(left_path, _, _), (right_path, _, _)| left_path.cmp(right_path));
    for (relative_path, buffer_snapshot, diff_snapshot) in snapshots_by_path {
        let base_snapshot = diff_snapshot.base_text();
        let is_existing_file = diff_snapshot.base_text_exists();

        let new_path_str = relative_path.to_string_lossy();
        let old_path_str = if is_existing_file {
            new_path_str.as_ref()
        } else {
            "/dev/null"
        };
        writeln!(
            uncommitted_diff,
            "--- {}{old_path_str}",
            if is_existing_file { "a/" } else { "" }
        )
        .ok();
        writeln!(uncommitted_diff, "+++ b/{new_path_str}").ok();

        if !is_existing_file {
            let new_text = buffer_snapshot.text();
            writeln!(
                uncommitted_diff,
                "@@ -0,0 +1,{} @@",
                new_text.lines().count()
            )
            .ok();
            for line in new_text.lines() {
                writeln!(uncommitted_diff, "+{line}").ok();
            }
            continue;
        }

        let mut ranges: Vec<(Range<u32>, Range<u32>)> = Vec::new();
        for hunk in (&diff_snapshot).hunks(&buffer_snapshot) {
            let old_start = base_snapshot
                .offset_to_point(hunk.diff_base_byte_range.start)
                .row;
            let old_end =
                exclusive_end_row(base_snapshot.offset_to_point(hunk.diff_base_byte_range.end));
            let new_start = hunk.range.start.row;
            let new_end = exclusive_end_row(hunk.range.end);
            let old_range = old_start.saturating_sub(3)..old_end + 3;
            let new_range = new_start.saturating_sub(3)..new_end + 3;

            if let Some((last_old_range, last_new_range)) = ranges.last_mut()
                && (old_range.start <= last_old_range.end || new_range.start <= last_new_range.end)
            {
                last_old_range.end = last_old_range.end.max(old_range.end);
                last_new_range.end = last_new_range.end.max(new_range.end);
                continue;
            }
            ranges.push((old_range, new_range));
        }

        for (old_range, new_range) in ranges {
            uncommitted_diff.push_str(&language::unified_diff_with_offsets(
                &base_snapshot
                    .text_for_range(
                        Point::new(old_range.start, 0)
                            ..row_start_or_max(base_snapshot, old_range.end),
                    )
                    .collect::<String>(),
                &buffer_snapshot
                    .text_for_range(
                        Point::new(new_range.start, 0)
                            ..row_start_or_max(&buffer_snapshot, new_range.end),
                    )
                    .collect::<String>(),
                old_range.start,
                new_range.start,
            ));
        }
        if !uncommitted_diff.ends_with('\n') {
            uncommitted_diff.push('\n');
        }
    }
    uncommitted_diff
}

pub fn estimate_uncomitted_diff_byte_size(snapshot: &UncommittedDiffSnapshot) -> usize {
    let mut size = 0;
    for (_, buffer_snapshot, diff_snapshot) in snapshot {
        for hunk in diff_snapshot.hunks(buffer_snapshot) {
            size += hunk.diff_base_byte_range.len();
            size += hunk.range.to_offset(buffer_snapshot).len();
        }
    }
    size
}

fn row_start_or_max(snapshot: &language::BufferSnapshot, row: u32) -> Point {
    if row >= snapshot.max_point().row {
        snapshot.max_point()
    } else {
        Point::new(row, 0)
    }
}

fn exclusive_end_row(point: Point) -> u32 {
    if point.column == 0 {
        point.row
    } else {
        point.row + 1
    }
}
