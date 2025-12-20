use anyhow::{Context as _, Result};
use buffer_diff::BufferDiffSnapshot;
use edit_prediction::{EditPredictionStore, StoredEvent, example_spec::ExampleSpec, udiff};
use editor::Editor;
use gpui::{App, Entity, Task, Window, prelude::*};
use language::ToPoint as _;
use log;
use project::{Project, ProjectPath};
use std::{fmt::Write as _, path::Path, sync::Arc};
use text::ToOffset as _;
use workspace::Workspace;

pub(crate) fn capture_example(
    editor: Entity<Editor>,
    last_event_is_expected_patch: bool,
    cx: &mut App,
) -> Option<Task<Result<ExampleSpec>>> {
    let editor = editor.read(cx);
    let project = editor.project().cloned()?;
    let multibuffer = editor.buffer();
    let (buffer, cursor_anchor) = multibuffer
        .read(cx)
        .text_anchor_for_position(editor.selections.newest_anchor().head(), cx)?;

    let snapshot = buffer.read(cx).snapshot();
    let file = snapshot.file()?;
    let cursor_path: Arc<Path> = file.path().as_std_path().into();
    let worktree_id = file.worktree_id(cx);

    let repository = project.read(cx).active_repository(cx)?;
    let repository_snapshot = repository.read(cx).snapshot();

    let worktree = project.read(cx).worktree_for_id(worktree_id, cx)?;
    if worktree.read(cx).abs_path() != repository_snapshot.work_directory_abs_path {
        return None;
    }

    let repository_url = repository_snapshot
        .remote_origin_url
        .clone()
        .or_else(|| repository_snapshot.remote_upstream_url.clone())?;
    let revision = repository_snapshot.head_commit.as_ref()?.sha.to_string();

    let ep_store = EditPredictionStore::try_global(cx)?;
    let events = ep_store.update(cx, |store, cx| {
        store.edit_history_for_project_with_pause_split_last_event(&project, cx)
    });

    let cursor_excerpt = compute_cursor_excerpt(&snapshot, cursor_anchor);

    let git_store = project.read(cx).git_store().clone();

    Some(cx.spawn(async move |mut cx| {
        let file_data = collect_file_data(&project, &git_store, &events, &mut cx).await?;

        let (uncommitted_diff, edit_history, expected_patch) = cx
            .background_executor()
            .spawn(async move { compute_diffs(file_data, events, last_event_is_expected_patch) })
            .await;

        let name = generate_timestamp_name();

        Ok(ExampleSpec {
            name,
            repository_url,
            revision,
            uncommitted_diff,
            cursor_path,
            cursor_position: cursor_excerpt,
            edit_history,
            expected_patch,
        })
    }))
}

fn compute_cursor_excerpt(
    snapshot: &language::BufferSnapshot,
    cursor_anchor: language::Anchor,
) -> String {
    let cursor_point = cursor_anchor.to_point(snapshot);
    let (_editable_range, context_range) =
        edit_prediction::cursor_excerpt::editable_and_context_ranges_for_cursor_position(
            cursor_point,
            snapshot,
            100,
            50,
        );

    let context_start_offset = context_range.start.to_offset(snapshot);
    let cursor_offset = cursor_anchor.to_offset(snapshot);
    let cursor_offset_in_excerpt = cursor_offset.saturating_sub(context_start_offset);
    let mut excerpt = snapshot.text_for_range(context_range).collect::<String>();
    if cursor_offset_in_excerpt <= excerpt.len() {
        excerpt.insert_str(cursor_offset_in_excerpt, zeta_prompt::CURSOR_MARKER);
    }
    excerpt
}

struct FileData {
    full_path: Arc<Path>,
    buffer_snapshot: text::BufferSnapshot,
    diff_snapshot: BufferDiffSnapshot,
}

async fn collect_file_data(
    project: &Entity<Project>,
    git_store: &Entity<project::git_store::GitStore>,
    events: &[StoredEvent],
    cx: &mut gpui::AsyncApp,
) -> Result<Vec<FileData>> {
    let edited_paths: Vec<(ProjectPath, Arc<Path>)> = events
        .iter()
        .filter_map(|stored_event| {
            let zeta_prompt::Event::BufferChange { path, .. } = stored_event.event.as_ref();
            let project_path = project
                .read_with(cx, |project, cx| project.find_project_path(path, cx))
                .ok()
                .flatten()?;
            Some((project_path, path.clone()))
        })
        .collect();

    let mut file_data = Vec::new();

    for (project_path, full_path) in edited_paths {
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

        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot())?;
        let diff_snapshot = diff.update(cx, |diff, cx| diff.snapshot(cx))?;

        file_data.push(FileData {
            full_path,
            buffer_snapshot,
            diff_snapshot,
        });
    }

    Ok(file_data)
}

fn compute_diffs(
    file_data: Vec<FileData>,
    mut events: Vec<StoredEvent>,
    last_event_is_expected_patch: bool,
) -> (String, String, String) {
    let mut uncommitted_diff = String::new();

    for data in &file_data {
        if let Some(head_text) = &data.diff_snapshot.base_text_string() {
            if let Ok(before_text) = compute_text_before_edit_history(
                &data.buffer_snapshot.text(),
                &data.full_path,
                &events,
            ) {
                let file_diff = language::unified_diff(head_text, &before_text);
                if !file_diff.is_empty() {
                    let path_str = data.full_path.to_string_lossy();
                    writeln!(uncommitted_diff, "--- a/{path_str}").ok();
                    writeln!(uncommitted_diff, "+++ b/{path_str}").ok();
                    uncommitted_diff.push_str(&file_diff);
                    if !uncommitted_diff.ends_with('\n') {
                        uncommitted_diff.push('\n');
                    }
                }
            }
        }
    }

    let mut edit_history = String::new();
    let mut expected_patch = String::new();

    if last_event_is_expected_patch {
        if let Some(stored_event) = events.pop() {
            zeta_prompt::write_event(&mut expected_patch, &stored_event.event);
        }
    }

    for stored_event in &events {
        zeta_prompt::write_event(&mut edit_history, &stored_event.event);
        if !edit_history.ends_with('\n') {
            edit_history.push('\n');
        }
    }

    (uncommitted_diff, edit_history, expected_patch)
}

fn compute_text_before_edit_history(
    current_text: &str,
    cursor_path: &Path,
    events: &[StoredEvent],
) -> Result<String> {
    let mut text = current_text.to_string();

    for stored_event in events.iter().rev() {
        let zeta_prompt::Event::BufferChange { path, diff, .. } = stored_event.event.as_ref();
        if path.as_ref() != cursor_path {
            continue;
        }

        let full_diff = format!("--- a/file\n+++ b/file\n{diff}");
        let inverted_diff = udiff::invert_diff(&full_diff);
        text = udiff::apply_diff_at_range(&inverted_diff, &text, &stored_event.edit.new)
            .with_context(|| format!("failed to apply inverted diff for {cursor_path:?}"))?;
    }

    Ok(text)
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

pub(crate) fn capture_example_as_markdown(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let markdown_language = workspace
        .app_state()
        .languages
        .language_for_name("Markdown");

    let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
        log::error!("no active editor");
        return;
    };

    let Some(example) = capture_example(editor, true, cx) else {
        return;
    };

    let project = workspace.project().clone();

    cx.spawn_in(window, async move |workspace_entity, cx| {
        let markdown_language = markdown_language.await?;
        let example_spec = example.await?;
        let markdown = example_spec.to_markdown();

        let buffer = project
            .update(cx, |project, cx| project.create_buffer(false, cx))?
            .await?;
        buffer.update(cx, |buffer, cx| {
            buffer.set_text(markdown, cx);
            buffer.set_language(Some(markdown_language), cx);
        })?;

        workspace_entity.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(
                Box::new(
                    cx.new(|cx| Editor::for_buffer(buffer, Some(project.clone()), window, cx)),
                ),
                None,
                true,
                window,
                cx,
            );
        })
    })
    .detach_and_log_err(cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::{Client, UserStore};
    use clock::FakeSystemClock;
    use gpui::{Entity, TestAppContext, http_client::FakeHttpClient};
    use indoc::indoc;
    use language::Point;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::Path;
    use workspace::AppState;

    #[gpui::test]
    async fn test_capture_example(cx: &mut TestAppContext) {
        let _app_state = init_test(cx);
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

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

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

        let editor: Entity<Editor> = workspace.update_in(cx, |workspace, window, cx| {
            let editor =
                cx.new(|cx| Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx));
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
            editor
        });
        cx.run_until_parked();

        let mut example = cx
            .update(|_, cx| capture_example(editor, false, cx))
            .unwrap()
            .await
            .unwrap();
        example.name = "test".to_string();

        assert_eq!(
            example,
            ExampleSpec {
                name: "test".to_string(),
                repository_url: "https://github.com/test/repo.git".to_string(),
                revision: "abc123def456".to_string(),
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
                cursor_path: Path::new("src/main.rs").into(),
                cursor_position: indoc! {"
                    <|user_cursor|>fn main() {
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
                expected_patch: "".to_string(),
            }
        );
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            zlog::init_test();
            release_channel::init(semver::Version::new(0, 0, 0), cx);

            let http_client = FakeHttpClient::with_404_response();
            let client = Client::new(Arc::new(FakeSystemClock::new()), http_client, cx);
            language_model::init(client.clone(), cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            EditPredictionStore::global(&client, &user_store, cx);

            let app_state = AppState::test(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            app_state
        })
    }
}
