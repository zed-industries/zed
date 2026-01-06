use crate::{
    EditPredictionExampleCaptureFeatureFlag, StoredEvent,
    cursor_excerpt::editable_and_context_ranges_for_cursor_position, example_spec::ExampleSpec,
};
use anyhow::Result;
use buffer_diff::BufferDiffSnapshot;
use collections::HashMap;
use feature_flags::FeatureFlagAppExt as _;
use gpui::{App, Entity, Task};
use language::{Buffer, ToPoint as _};
use project::{Project, WorktreeId};
use std::{collections::hash_map, fmt::Write as _, path::Path, sync::Arc};
use text::BufferSnapshot as TextBufferSnapshot;

pub(crate) const DEFAULT_EXAMPLE_CAPTURE_RATE_PER_10K_PREDICTIONS: u16 = 10;

pub fn capture_example(
    project: Entity<Project>,
    buffer: Entity<Buffer>,
    cursor_anchor: language::Anchor,
    mut events: Vec<StoredEvent>,
    cx: &mut App,
) -> Option<Task<Result<ExampleSpec>>> {
    let snapshot = buffer.read(cx).snapshot();
    let file = snapshot.file()?;
    let worktree_id = file.worktree_id(cx);
    let repository = project.read(cx).active_repository(cx)?;
    let repository_snapshot = repository.read(cx).snapshot();
    let worktree = project.read(cx).worktree_for_id(worktree_id, cx)?;
    let cursor_path = worktree.read(cx).root_name().join(file.path());
    if worktree.read(cx).abs_path() != repository_snapshot.work_directory_abs_path {
        return None;
    }

    let repository_url = repository_snapshot
        .remote_origin_url
        .clone()
        .or_else(|| repository_snapshot.remote_upstream_url.clone())?;
    let revision = repository_snapshot.head_commit.as_ref()?.sha.to_string();

    let git_store = project.read(cx).git_store().clone();

    Some(cx.spawn(async move |mut cx| {
        let snapshots_by_path =
            collect_snapshots(&project, &git_store, worktree_id, &events, &mut cx).await?;

        events.retain(|stored_event| {
            match stored_event.event.as_ref() {
                zeta_prompt::Event::BufferChange { path, .. } => {
                    if !snapshots_by_path.contains_key(path) {
                        return false;
                    }
                }
            }
            true
        });

        let line_comment_prefix = snapshot
            .language()
            .and_then(|lang| lang.config().line_comments.first())
            .map(|s| s.to_string())
            .unwrap_or_default();
        let (cursor_excerpt, cursor_offset) = cx
            .background_executor()
            .spawn(async move { compute_cursor_excerpt(&snapshot, cursor_anchor) })
            .await;
        let uncommitted_diff = cx
            .background_executor()
            .spawn(async move { compute_uncommitted_diff(snapshots_by_path) })
            .await;

        let mut edit_history = String::new();
        for stored_event in &events {
            zeta_prompt::write_event(&mut edit_history, &stored_event.event);
            if !edit_history.ends_with('\n') {
                edit_history.push('\n');
            }
        }

        let mut spec = ExampleSpec {
            name: generate_timestamp_name(),
            repository_url,
            revision,
            tags: Vec::new(),
            reasoning: None,
            uncommitted_diff,
            cursor_path: cursor_path.as_std_path().into(),
            cursor_position: String::new(),
            edit_history,
            expected_patches: Vec::new(),
        };
        spec.set_cursor_excerpt(&cursor_excerpt, cursor_offset, &line_comment_prefix);
        Ok(spec)
    }))
}

fn compute_cursor_excerpt(
    snapshot: &language::BufferSnapshot,
    cursor_anchor: language::Anchor,
) -> (String, usize) {
    use text::ToOffset as _;

    let cursor_point = cursor_anchor.to_point(snapshot);
    let (_editable_range, context_range) =
        editable_and_context_ranges_for_cursor_position(cursor_point, snapshot, 100, 50);
    let context_start_offset = context_range.start.to_offset(snapshot);
    let cursor_offset = cursor_anchor.to_offset(snapshot);
    let cursor_offset_in_excerpt = cursor_offset.saturating_sub(context_start_offset);
    let excerpt = snapshot.text_for_range(context_range).collect::<String>();
    (excerpt, cursor_offset_in_excerpt)
}

async fn collect_snapshots(
    project: &Entity<Project>,
    git_store: &Entity<project::git_store::GitStore>,
    worktree_id: WorktreeId,
    events: &[StoredEvent],
    cx: &mut gpui::AsyncApp,
) -> Result<HashMap<Arc<Path>, (TextBufferSnapshot, BufferDiffSnapshot)>> {
    let mut snapshots_by_path = HashMap::default();
    let root_name = project.read_with(cx, |project, cx| {
        project
            .worktree_for_id(worktree_id, cx)
            .unwrap()
            .read(cx)
            .root_name()
            .to_owned()
    })?;
    for stored_event in events {
        let zeta_prompt::Event::BufferChange { path, .. } = stored_event.event.as_ref();
        if let Some((project_path, full_path)) = project.read_with(cx, |project, cx| {
            let project_path = project
                .find_project_path(path, cx)
                .filter(|path| path.worktree_id == worktree_id)?;
            let full_path = root_name.join(&project_path.path).as_std_path().into();
            Some((project_path, full_path))
        })? {
            if let hash_map::Entry::Vacant(entry) = snapshots_by_path.entry(full_path) {
                let buffer = project
                    .update(cx, |project, cx| {
                        project.open_buffer(project_path.clone(), cx)
                    })?
                    .await?;
                let diff = git_store
                    .update(cx, |git_store, cx| {
                        git_store.open_uncommitted_diff(buffer.clone(), cx)
                    })?
                    .await?;
                let diff_snapshot = diff.update(cx, |diff, cx| diff.snapshot(cx))?;
                entry.insert((stored_event.old_snapshot.clone(), diff_snapshot));
            }
        }
    }
    Ok(snapshots_by_path)
}

fn compute_uncommitted_diff(
    snapshots_by_path: HashMap<Arc<Path>, (TextBufferSnapshot, BufferDiffSnapshot)>,
) -> String {
    let mut uncommitted_diff = String::new();
    for (full_path, (before_text, diff_snapshot)) in snapshots_by_path {
        if let Some(head_text) = &diff_snapshot.base_text_string() {
            let file_diff = language::unified_diff(head_text, &before_text.text());
            if !file_diff.is_empty() {
                let path_str = full_path.to_string_lossy();
                writeln!(uncommitted_diff, "--- a/{path_str}").ok();
                writeln!(uncommitted_diff, "+++ b/{path_str}").ok();
                uncommitted_diff.push_str(&file_diff);
                if !uncommitted_diff.ends_with('\n') {
                    uncommitted_diff.push('\n');
                }
            }
        }
    }
    uncommitted_diff
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

pub(crate) fn should_sample_edit_prediction_example_capture(cx: &App) -> bool {
    let capture_rate = language::language_settings::all_language_settings(None, cx)
        .edit_predictions
        .example_capture_rate
        .unwrap_or(DEFAULT_EXAMPLE_CAPTURE_RATE_PER_10K_PREDICTIONS);
    cx.has_flag::<EditPredictionExampleCaptureFeatureFlag>()
        && rand::random::<u16>() % 10_000 < capture_rate
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditPredictionStore;
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
                    "main.rs": disk_contents,
                }
            }),
        )
        .await;

        fs.set_head_for_repo(
            Path::new("/project/.git"),
            &[("src/main.rs", committed_contents.to_string())],
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

        let events = ep_store.update(cx, |store, cx| {
            store.edit_history_for_project_with_pause_split_last_event(&project, cx)
        });
        let mut example = cx
            .update(|cx| {
                capture_example(project.clone(), buffer.clone(), Anchor::MIN, events, cx).unwrap()
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
                    --- a/project/src/main.rs
                    +++ b/project/src/main.rs
                    @@ -1,4 +1,5 @@
                     fn main() {
                    +    // comment 1
                         one();
                         two();
                         three();
                    @@ -7,5 +8,6 @@
                         six();
                         seven();
                         eight();
                    +    // comment 2
                         nine();
                     }
                "}
                .to_string(),
                cursor_path: Path::new("project/src/main.rs").into(),
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
                    --- a/project/src/main.rs
                    +++ b/project/src/main.rs
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
                "}
                .to_string(),
                expected_patches: Vec::new()
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
            language_model::init(client.clone(), cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            EditPredictionStore::global(&client, &user_store, cx);
        })
    }
}
