use crate::{
    example::{Example, ExampleBuffer, ExampleState},
    headless::EpAppState,
};
use anyhow::{Result, anyhow};
use collections::HashMap;
use edit_prediction::EditPredictionStore;
use edit_prediction::udiff::OpenedBuffers;
use futures::{
    AsyncWriteExt as _,
    lock::{Mutex, OwnedMutexGuard},
};
use gpui::{AsyncApp, Entity};
use language::{Anchor, Buffer, ToOffset, ToPoint};
use project::buffer_store::BufferStoreEvent;
use project::{Project, ProjectPath};
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{paths::PathStyle, rel_path::RelPath};
use zeta_prompt::CURSOR_MARKER;

pub async fn run_load_project(example: &mut Example, app_state: Arc<EpAppState>, mut cx: AsyncApp) {
    if example.state.is_some() {
        return;
    }

    let project = setup_project(example, &app_state, &mut cx).await;
    let buffer_store = project
        .read_with(&cx, |project, _| project.buffer_store().clone())
        .unwrap();

    let ep_store = cx
        .update(|cx| EditPredictionStore::try_global(cx).unwrap())
        .unwrap();

    cx.subscribe(&buffer_store, {
        let project = project.clone();
        move |_, event, cx| match event {
            BufferStoreEvent::BufferAdded(buffer) => {
                ep_store.update(cx, |store, cx| store.register_buffer(&buffer, &project, cx));
            }
            _ => {}
        }
    })
    .unwrap()
    .detach();

    let _open_buffers = apply_edit_history(example, &project, &mut cx)
        .await
        .unwrap();
    let (buffer, cursor_position) = cursor_position(example, &project, &mut cx).await;
    example.buffer = buffer
        .read_with(&cx, |buffer, _cx| {
            let cursor_point = cursor_position.to_point(&buffer);
            Some(ExampleBuffer {
                content: buffer.text(),
                cursor_row: cursor_point.row,
                cursor_column: cursor_point.column,
                cursor_offset: cursor_position.to_offset(&buffer),
            })
        })
        .unwrap();
    example.state = Some(ExampleState {
        buffer,
        project,
        cursor_position,
        _open_buffers,
    });
}

async fn cursor_position(
    example: &Example,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> (Entity<Buffer>, Anchor) {
    let worktree = project
        .read_with(cx, |project, cx| {
            project.visible_worktrees(cx).next().unwrap()
        })
        .unwrap();

    let cursor_path = RelPath::new(&example.cursor_path, PathStyle::Posix)
        .unwrap()
        .into_arc();
    let cursor_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer(
                ProjectPath {
                    worktree_id: worktree.read(cx).id(),
                    path: cursor_path,
                },
                cx,
            )
        })
        .unwrap()
        .await
        .unwrap();
    let cursor_offset_within_excerpt = example
        .cursor_position
        .find(CURSOR_MARKER)
        .ok_or_else(|| anyhow!("missing cursor marker"))
        .unwrap();
    let mut cursor_excerpt = example.cursor_position.clone();
    cursor_excerpt.replace_range(
        cursor_offset_within_excerpt..(cursor_offset_within_excerpt + CURSOR_MARKER.len()),
        "",
    );
    let excerpt_offset = cursor_buffer.read_with(cx, |buffer, _cx| {
        let text = buffer.text();

        let mut matches = text.match_indices(&cursor_excerpt);
        let (excerpt_offset, _) = matches.next().unwrap_or_else(|| {
            panic!(
                "\nExcerpt:\n\n{cursor_excerpt}\nBuffer text:\n{text}\n.Cursor excerpt did not exist in buffer."
            );
        });
        assert!(matches.next().is_none(), "More than one cursor position match found for {}", &example.name);
        excerpt_offset
    }).unwrap();

    let cursor_offset = excerpt_offset + cursor_offset_within_excerpt;
    let cursor_anchor = cursor_buffer
        .read_with(cx, |buffer, _| buffer.anchor_after(cursor_offset))
        .unwrap();

    (cursor_buffer, cursor_anchor)
}

async fn setup_project(
    example: &mut Example,
    app_state: &Arc<EpAppState>,
    cx: &mut AsyncApp,
) -> Entity<Project> {
    setup_worktree(example).await;

    let project = cx
        .update(|cx| {
            Project::local(
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                None,
                cx,
            )
        })
        .unwrap();

    let worktree = project
        .update(cx, |project, cx| {
            project.create_worktree(&example.worktree_path(), true, cx)
        })
        .unwrap()
        .await
        .unwrap();
    worktree
        .read_with(cx, |worktree, _cx| {
            worktree.as_local().unwrap().scan_complete()
        })
        .unwrap()
        .await;
    project
}

pub async fn setup_worktree(example: &Example) {
    let repo_dir = example.repo_path();
    let repo_lock = lock_repo(&repo_dir).await;

    if !repo_dir.is_dir() {
        fs::create_dir_all(&repo_dir).unwrap();
        run_git(&repo_dir, &["init"]).await.unwrap();
        run_git(
            &repo_dir,
            &["remote", "add", "origin", &example.repository_url],
        )
        .await
        .unwrap();
    }

    // Resolve the example to a revision, fetching it if needed.
    let revision = run_git(
        &repo_dir,
        &["rev-parse", &format!("{}^{{commit}}", example.revision)],
    )
    .await;
    let revision = if let Ok(revision) = revision {
        revision
    } else {
        if run_git(
            &repo_dir,
            &["fetch", "--depth", "1", "origin", &example.revision],
        )
        .await
        .is_err()
        {
            run_git(&repo_dir, &["fetch", "origin"]).await.unwrap();
        }
        let revision = run_git(&repo_dir, &["rev-parse", "FETCH_HEAD"])
            .await
            .unwrap();
        if revision != example.revision {
            run_git(&repo_dir, &["tag", &example.revision, &revision])
                .await
                .unwrap();
        }
        revision
    };

    // Create the worktree for this example if needed.
    let worktree_path = example.worktree_path();
    if worktree_path.is_dir() {
        run_git(&worktree_path, &["clean", "--force", "-d"])
            .await
            .unwrap();
        run_git(&worktree_path, &["reset", "--hard", "HEAD"])
            .await
            .unwrap();
        run_git(&worktree_path, &["checkout", revision.as_str()])
            .await
            .unwrap();
    } else {
        let worktree_path_string = worktree_path.to_string_lossy();
        run_git(
            &repo_dir,
            &["branch", "-f", &example.name, revision.as_str()],
        )
        .await
        .unwrap();
        run_git(
            &repo_dir,
            &[
                "worktree",
                "add",
                "-f",
                &worktree_path_string,
                &example.name,
            ],
        )
        .await
        .unwrap();
    }
    drop(repo_lock);

    // Apply the uncommitted diff for this example.
    if !example.uncommitted_diff.is_empty() {
        let mut apply_process = smol::process::Command::new("git")
            .current_dir(&worktree_path)
            .args(&["apply", "-"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = apply_process.stdin.take().unwrap();
        stdin
            .write_all(example.uncommitted_diff.as_bytes())
            .await
            .unwrap();
        stdin.close().await.unwrap();
        drop(stdin);

        let apply_result = apply_process.output().await.unwrap();
        if !apply_result.status.success() {
            panic!(
                "Failed to apply uncommitted diff patch with status: {}\nstderr:\n{}\nstdout:\n{}",
                apply_result.status,
                String::from_utf8_lossy(&apply_result.stderr),
                String::from_utf8_lossy(&apply_result.stdout),
            );
        }
    }
}

async fn apply_edit_history(
    example: &Example,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<OpenedBuffers> {
    edit_prediction::udiff::apply_diff(&example.edit_history, project, cx).await
}

thread_local! {
    static REPO_LOCKS: RefCell<HashMap<PathBuf, Arc<Mutex<()>>>> = RefCell::new(HashMap::default());
}

#[must_use]
pub async fn lock_repo(path: impl AsRef<Path>) -> OwnedMutexGuard<()> {
    REPO_LOCKS
        .with(|cell| {
            cell.borrow_mut()
                .entry(path.as_ref().to_path_buf())
                .or_default()
                .clone()
        })
        .lock_owned()
        .await
}

async fn run_git(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = smol::process::Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .await?;

    anyhow::ensure!(
        output.status.success(),
        "`git {}` within `{}` failed with status: {}\nstderr:\n{}\nstdout:\n{}",
        args.join(" "),
        repo_path.display(),
        output.status,
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout),
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
