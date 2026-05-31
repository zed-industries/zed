use crate::{
    StoredEvent,
    example_spec::{ExampleSpec, RecentFile},
};
use anyhow::Result;
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use collections::HashMap;
use gpui::{App, Entity, Task};
use language::Buffer;
use project::Project;
use std::{collections::hash_map, fmt::Write as _, ops::Range, path::Path, sync::Arc};
use text::Point;

pub fn capture_example(
    project: Entity<Project>,
    buffer: Entity<Buffer>,
    cursor_anchor: language::Anchor,
    events: Vec<StoredEvent>,
    recently_opened_files: Vec<RecentFile>,
    recently_viewed_files: Vec<RecentFile>,
    uncommitted_diffs_by_path: HashMap<Arc<Path>, Entity<BufferDiff>>,
    populate_expected_patch: bool,
    cx: &mut App,
) -> Option<Task<Result<ExampleSpec>>> {
    let snapshot = buffer.read(cx).snapshot();
    let file = snapshot.file()?;
    let worktree_id = file.worktree_id(cx);
    let repository = project.read(cx).active_repository(cx)?;
    let repository_snapshot = repository.read(cx).snapshot();
    let worktree = project.read(cx).worktree_for_id(worktree_id, cx)?;
    let root_name = worktree.read(cx).root_name_str().to_owned();
    let cursor_path: Arc<Path> = file.path().as_std_path().into();
    if worktree.read(cx).abs_path() != repository_snapshot.work_directory_abs_path {
        return None;
    }

    let repository_url = repository_snapshot
        .remote_origin_url
        .clone()
        .or_else(|| repository_snapshot.remote_upstream_url.clone())?;
    let revision = repository_snapshot.head_commit.as_ref()?.sha.to_string();

    Some(cx.spawn(async move |cx| {
        let mut events = events;
        let mut diff_buffers_by_path: HashMap<Arc<Path>, (Entity<Buffer>, Entity<BufferDiff>)> =
            HashMap::default();
        for stored_event in &events {
            let zeta_prompt::Event::BufferChange { path, .. } = stored_event.event.as_ref();
            let Some((project_path, relative_path)) = project.read_with(cx, |project, cx| {
                let project_path = project
                    .find_project_path(path, cx)
                    .filter(|path| path.worktree_id == worktree_id)?;
                let relative_path: Arc<Path> = project_path.path.as_std_path().into();
                Some((project_path, relative_path))
            }) else {
                continue;
            };

            if let hash_map::Entry::Vacant(entry) = diff_buffers_by_path.entry(relative_path) {
                let Some(diff) = uncommitted_diffs_by_path.get(entry.key()).cloned() else {
                    continue;
                };
                let buffer = project
                    .update(cx, |project, cx| {
                        project.open_buffer(project_path.clone(), cx)
                    })
                    .await?;
                entry.insert((buffer, diff));
            }
        }

        events.retain(|stored_event| {
            let zeta_prompt::Event::BufferChange { path, .. } = stored_event.event.as_ref();
            let relative_path = strip_root_name(path, &root_name);
            diff_buffers_by_path.contains_key(relative_path)
        });

        let line_comment_prefix = snapshot
            .language()
            .and_then(|lang| lang.config().line_comments.first())
            .map(|s| s.to_string())
            .unwrap_or_default();

        let (cursor_excerpt, cursor_offset_in_excerpt, cursor_excerpt_range) = cx
            .background_executor()
            .spawn(async move { compute_cursor_excerpt(&snapshot, cursor_anchor) })
            .await;
        let uncommitted_diff_snapshots = diff_buffers_by_path
            .into_iter()
            .map(|(relative_path, (buffer, diff))| {
                let snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
                let diff_snapshot = diff.update(cx, |diff, cx| diff.snapshot(cx));
                (relative_path, (snapshot, diff_snapshot))
            })
            .collect();
        let uncommitted_diff = cx
            .background_executor()
            .spawn(async move { compute_uncommitted_diff(uncommitted_diff_snapshots) })
            .await;

        let mut edit_history = String::new();
        for stored_event in &events {
            write_event_with_relative_paths(&mut edit_history, &stored_event.event, &root_name);
            if !edit_history.ends_with('\n') {
                edit_history.push('\n');
            }
        }
        let uncommitted_diff_contains_edit_history = !edit_history.is_empty();

        // Initialize an empty patch with context lines, to make it easy
        // to write the expected patch by hand.
        let mut expected_patches = Vec::new();
        let mut rejected_patch = None;
        if populate_expected_patch {
            let mut empty_patch = String::new();
            let start_row = cursor_excerpt_range.start.row + 1;
            let row_count = cursor_excerpt_range.end.row - cursor_excerpt_range.start.row + 1;
            writeln!(&mut empty_patch, "--- a/{}", cursor_path.display()).ok();
            writeln!(&mut empty_patch, "+++ b/{}", cursor_path.display()).ok();
            writeln!(
                &mut empty_patch,
                "@@ -{},{} +{},{} @@",
                start_row, row_count, start_row, row_count,
            )
            .ok();
            for line in cursor_excerpt.lines() {
                writeln!(&mut empty_patch, " {}", line).ok();
            }

            expected_patches.push(empty_patch.clone());
            rejected_patch = Some(empty_patch);
        }

        let mut spec = ExampleSpec {
            name: generate_timestamp_name(),
            repository_url,
            revision,
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff,
            recently_opened_files,
            recently_viewed_files,
            uncommitted_diff_contains_edit_history,
            cursor_path,
            cursor_position: String::new(),
            edit_history,
            expected_patches,
            rejected_patch,
            telemetry: None,
            human_feedback: Vec::new(),
            rating: None,
        };
        spec.set_cursor_excerpt(
            &cursor_excerpt,
            cursor_offset_in_excerpt,
            &line_comment_prefix,
        );
        Ok(spec)
    }))
}

fn strip_root_name<'a>(path: &'a Path, root_name: &str) -> &'a Path {
    path.strip_prefix(root_name).unwrap_or(path)
}

fn write_event_with_relative_paths(
    output: &mut String,
    event: &zeta_prompt::Event,
    root_name: &str,
) {
    fn write_relative_path(output: &mut String, path: &Path, root_name: &str) {
        for component in strip_root_name(path, root_name).components() {
            output.push('/');
            write!(output, "{}", component.as_os_str().to_string_lossy()).ok();
        }
    }

    let zeta_prompt::Event::BufferChange {
        path,
        old_path,
        diff,
        ..
    } = event;

    output.push_str("--- a");
    write_relative_path(output, old_path.as_ref(), root_name);
    output.push_str("\n+++ b");
    write_relative_path(output, path.as_ref(), root_name);
    output.push('\n');
    output.push_str(diff);
}

fn compute_cursor_excerpt(
    snapshot: &language::BufferSnapshot,
    cursor_anchor: language::Anchor,
) -> (String, usize, Range<Point>) {
    use text::ToOffset as _;
    use text::ToPoint as _;

    let cursor_offset = cursor_anchor.to_offset(snapshot);
    let (excerpt_point_range, excerpt_offset_range, cursor_offset_in_excerpt) =
        crate::cursor_excerpt::compute_cursor_excerpt(snapshot, cursor_offset);
    let syntax_ranges = crate::cursor_excerpt::compute_syntax_ranges(
        snapshot,
        cursor_offset,
        &excerpt_offset_range,
    );
    let excerpt_text: String = snapshot.text_for_range(excerpt_point_range).collect();
    let (_, context_range) = zeta_prompt::compute_editable_and_context_ranges(
        &excerpt_text,
        cursor_offset_in_excerpt,
        &syntax_ranges,
        100,
        50,
    );
    let context_text = excerpt_text[context_range.clone()].to_string();
    let cursor_in_context = cursor_offset_in_excerpt.saturating_sub(context_range.start);
    let context_buffer_start =
        (excerpt_offset_range.start + context_range.start).to_point(snapshot);
    let context_buffer_end = (excerpt_offset_range.start + context_range.end).to_point(snapshot);
    (
        context_text,
        cursor_in_context,
        context_buffer_start..context_buffer_end,
    )
}

fn compute_uncommitted_diff(
    snapshots_by_path: HashMap<Arc<Path>, (language::BufferSnapshot, BufferDiffSnapshot)>,
) -> String {
    let mut uncommitted_diff = String::new();
    let mut snapshots_by_path = snapshots_by_path.into_iter().collect::<Vec<_>>();
    snapshots_by_path.sort_by(|(left_path, _), (right_path, _)| left_path.cmp(right_path));
    for (relative_path, (buffer_snapshot, diff_snapshot)) in snapshots_by_path {
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

fn generate_timestamp_name() -> String {
    let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]");
    match format {
        Ok(format) => {
            let now = time::OffsetDateTime::now_local()
                .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
            now.format(&format)
                .unwrap_or_else(|_| "unknown-time".to_string())
        }
        Err(_) => "unknown-time".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditPredictionStore;
    use client::RefreshLlmTokenListener;
    use client::{Client, UserStore};
    use clock::FakeSystemClock;
    use gpui::{AppContext as _, TestAppContext, http_client::FakeHttpClient};
    use indoc::indoc;
    use language::{Anchor, Point};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;

    #[gpui::test]
    async fn test_capture_example(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        let committed_contents = indoc! {"
            fn main() {
                one();
                two();
                three();
                four();
                five();
                six();
                seven();
                eight();
                nine();
            }
        "};

        let disk_contents = indoc! {"
            fn main() {
                // comment 1
                one();
                two();
                three();
                four();
                five();
                six();
                seven();
                eight();
                // comment 2
                nine();
            }
        "};

        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": {
                    "deleted.rs": "pub fn deleted_file() {\n    deleted();\n}\n",
                    "main.rs": disk_contents,
                    "new.rs": "pub fn new_file() {\n}\n",
                }
            }),
        )
        .await;

        // Create an external file outside the main project
        fs.insert_tree(
            "/external",
            json!({
                "external.rs": "fn external() {}\n",
            }),
        )
        .await;

        fs.set_head_for_repo(
            Path::new("/project/.git"),
            &[
                (
                    "src/deleted.rs",
                    "pub fn deleted_file() {\n    deleted();\n}\n".to_string(),
                ),
                ("src/main.rs", committed_contents.to_string()),
            ],
            "abc123def456",
        );
        fs.set_remote_for_repo(
            Path::new("/project/.git"),
            "origin",
            "https://github.com/test/repo.git",
        );

        let project = Project::test(fs.clone(), ["/project".as_ref()], cx).await;

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/project/src/main.rs", cx)
            })
            .await
            .unwrap();

        let ep_store = cx.read(|cx| EditPredictionStore::try_global(cx).unwrap());
        ep_store.update(cx, |ep_store, cx| {
            ep_store.register_buffer(&buffer, &project, cx)
        });
        cx.run_until_parked();

        let deleted_file_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/project/src/deleted.rs", cx)
            })
            .await
            .unwrap();
        ep_store.update(cx, |ep_store, cx| {
            ep_store.register_buffer(&deleted_file_buffer, &project, cx)
        });
        cx.run_until_parked();
        deleted_file_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..buffer.len(), "")], None, cx);
        });
        cx.run_until_parked();

        buffer.update(cx, |buffer, cx| {
            let point = Point::new(6, 0);
            buffer.edit([(point..point, "    // comment 3\n")], None, cx);
            let point = Point::new(4, 0);
            buffer.edit([(point..point, "    // comment 4\n")], None, cx);

            pretty_assertions::assert_eq!(
                buffer.text(),
                indoc! {"
                    fn main() {
                        // comment 1
                        one();
                        two();
                        // comment 4
                        three();
                        four();
                        // comment 3
                        five();
                        six();
                        seven();
                        eight();
                        // comment 2
                        nine();
                    }
                "}
            );
        });
        cx.run_until_parked();

        let new_file_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/project/src/new.rs", cx)
            })
            .await
            .unwrap();
        ep_store.update(cx, |ep_store, cx| {
            ep_store.register_buffer(&new_file_buffer, &project, cx)
        });
        cx.run_until_parked();
        new_file_buffer.update(cx, |buffer, cx| {
            let point = Point::new(1, 0);
            buffer.edit([(point..point, "    created();\n")], None, cx);
        });
        cx.run_until_parked();

        // Open and edit an external file (outside the main project's worktree)
        let external_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/external/external.rs", cx)
            })
            .await
            .unwrap();
        ep_store.update(cx, |ep_store, cx| {
            ep_store.register_buffer(&external_buffer, &project, cx)
        });
        cx.run_until_parked();
        external_buffer.update(cx, |buffer, cx| {
            let point = Point::new(0, 0);
            buffer.edit([(point..point, "// external edit\n")], None, cx);
        });
        cx.run_until_parked();

        // Verify the external edit was recorded in events
        let events = ep_store.update(cx, |store, cx| store.edit_history_for_project(&project, cx));
        assert!(
            matches!(
                events
                    .last()
                    .unwrap()
                    .event
                    .as_ref(),
                zeta_prompt::Event::BufferChange { path, .. } if path.as_ref() == "/external/external.rs"
            ),
            "external file edit should be in events"
        );

        let worktree_id = buffer.read_with(cx, |buffer, cx| buffer.file().unwrap().worktree_id(cx));
        let uncommitted_diffs_by_path = ep_store
            .update(cx, |store, cx| {
                store.uncommitted_diffs_for_events(project.clone(), worktree_id, events.clone(), cx)
            })
            .await
            .unwrap();
        let mut example = cx
            .update(|cx| {
                capture_example(
                    project.clone(),
                    buffer.clone(),
                    Anchor::min_for_buffer(buffer.read(cx).remote_id()),
                    events,
                    Vec::new(),
                    Vec::new(),
                    uncommitted_diffs_by_path,
                    true,
                    cx,
                )
                .unwrap()
            })
            .await
            .unwrap();
        example.name = "test".to_string();

        pretty_assertions::assert_eq!(
            example,
            ExampleSpec {
                name: "test".to_string(),
                repository_url: "https://github.com/test/repo.git".to_string(),
                revision: "abc123def456".to_string(),
                tags: Vec::new(),
                reasoning: None,
                uncommitted_diff: indoc! {"
                    --- a/src/deleted.rs
                    +++ b/src/deleted.rs
                    @@ -1,3 +1,0 @@
                    -pub fn deleted_file() {
                    -    deleted();
                    -}
                    --- a/src/main.rs
                    +++ b/src/main.rs
                    @@ -1,11 +1,15 @@
                     fn main() {
                    +    // comment 1
                         one();
                         two();
                    +    // comment 4
                         three();
                         four();
                    +    // comment 3
                         five();
                         six();
                         seven();
                         eight();
                    +    // comment 2
                         nine();
                     }
                    --- /dev/null
                    +++ b/src/new.rs
                    @@ -0,0 +1,3 @@
                    +pub fn new_file() {
                    +    created();
                    +}
                "}
                .to_string(),
                recently_opened_files: Vec::new(),
                recently_viewed_files: Vec::new(),
                uncommitted_diff_contains_edit_history: true,
                cursor_path: Path::new("src/main.rs").into(),
                cursor_position: indoc! {"
                    fn main() {
                    ^[CURSOR_POSITION]
                        // comment 1
                        one();
                        two();
                        // comment 4
                        three();
                        four();
                        // comment 3
                        five();
                        six();
                        seven();
                        eight();
                        // comment 2
                        nine();
                    }
                "}
                .to_string(),
                edit_history: indoc! {"
                    --- a/src/deleted.rs
                    +++ b/src/deleted.rs
                    @@ -1,3 +1,0 @@
                    -pub fn deleted_file() {
                    -    deleted();
                    -}
                    --- a/src/main.rs
                    +++ b/src/main.rs
                    @@ -2,8 +2,10 @@
                         // comment 1
                         one();
                         two();
                    +    // comment 4
                         three();
                         four();
                    +    // comment 3
                         five();
                         six();
                         seven();
                    --- a/src/new.rs
                    +++ b/src/new.rs
                    @@ -1,2 +1,3 @@
                     pub fn new_file() {
                    +    created();
                     }
                "}
                .to_string(),
                expected_patches: vec![
                    indoc! {"
                        --- a/src/main.rs
                        +++ b/src/main.rs
                        @@ -1,16 +1,16 @@
                         fn main() {
                             // comment 1
                             one();
                             two();
                             // comment 4
                             three();
                             four();
                             // comment 3
                             five();
                             six();
                             seven();
                             eight();
                             // comment 2
                             nine();
                         }
                    "}
                    .to_string()
                ],
                rejected_patch: Some(
                    indoc! {"
                        --- a/src/main.rs
                        +++ b/src/main.rs
                        @@ -1,16 +1,16 @@
                         fn main() {
                             // comment 1
                             one();
                             two();
                             // comment 4
                             three();
                             four();
                             // comment 3
                             five();
                             six();
                             seven();
                             eight();
                             // comment 2
                             nine();
                         }
                    "}
                    .to_string()
                ),
                telemetry: None,
                human_feedback: Vec::new(),
                rating: None,
            }
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            zlog::init_test();
            let http_client = FakeHttpClient::with_404_response();
            let client = Client::new(Arc::new(FakeSystemClock::new()), http_client, cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            language_model::init(cx);
            RefreshLlmTokenListener::register(client.clone(), user_store.clone(), cx);
            EditPredictionStore::global(&client, &user_store, cx);
        })
    }
}
