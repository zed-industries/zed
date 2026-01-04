use crate::{
    example::{Example, ExampleBuffer, ExampleState},
    git,
    headless::EpAppState,
    paths::WORKTREES_DIR,
    progress::{InfoStyle, Progress, Step, StepProgress},
};
use anyhow::{Context as _, Result};
use edit_prediction::EditPredictionStore;
use edit_prediction::udiff::OpenedBuffers;
use futures::AsyncWriteExt as _;
use gpui::{AsyncApp, Entity};
use language::{Anchor, Buffer, LanguageNotFound, ToOffset, ToPoint};
use project::Project;
use project::buffer_store::BufferStoreEvent;
use std::{fs, path::PathBuf, sync::Arc};

pub async fn run_load_project(
    example: &mut Example,
    app_state: Arc<EpAppState>,
    mut cx: AsyncApp,
) -> Result<()> {
    if example.state.is_some() {
        return Ok(());
    }

    let progress = Progress::global().start(Step::LoadProject, &example.spec.name);

    let project = setup_project(example, &app_state, &progress, &mut cx).await?;

    let _open_buffers = apply_edit_history(example, &project, &mut cx).await?;

    let (buffer, cursor_position) = cursor_position(example, &project, &mut cx).await?;
    let (example_buffer, language_name) = buffer.read_with(&cx, |buffer, _cx| {
        let cursor_point = cursor_position.to_point(&buffer);
        let language_name = buffer
            .language()
            .map(|l| l.name().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        (
            ExampleBuffer {
                content: buffer.text(),
                cursor_row: cursor_point.row,
                cursor_column: cursor_point.column,
                cursor_offset: cursor_position.to_offset(&buffer),
            },
            language_name,
        )
    })?;

    progress.set_info(language_name, InfoStyle::Normal);

    example.buffer = Some(example_buffer);
    example.state = Some(ExampleState {
        buffer,
        project,
        cursor_position,
        _open_buffers,
    });
    Ok(())
}

async fn cursor_position(
    example: &Example,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<(Entity<Buffer>, Anchor)> {
    let language_registry = project.read_with(cx, |project, _| project.languages().clone())?;
    let result = language_registry
        .load_language_for_file_path(&example.spec.cursor_path)
        .await;

    if let Err(error) = result
        && !error.is::<LanguageNotFound>()
    {
        return Err(error);
    }

    let cursor_path = project
        .read_with(cx, |project, cx| {
            project.find_project_path(&example.spec.cursor_path, cx)
        })?
        .with_context(|| {
            format!(
                "failed to find cursor path {}",
                example.spec.cursor_path.display()
            )
        })?;
    let cursor_buffer = project
        .update(cx, |project, cx| project.open_buffer(cursor_path, cx))?
        .await?;

    let (cursor_excerpt, cursor_offset_within_excerpt) = example.spec.cursor_excerpt()?;

    let excerpt_offset = cursor_buffer.read_with(cx, |buffer, _cx| {
        let text = buffer.text();

        let mut matches = text.match_indices(&cursor_excerpt);
        let (excerpt_offset, _) = matches.next().with_context(|| {
            format!(
                "\nExcerpt:\n\n{cursor_excerpt}\nBuffer text:\n{text}\n.Example: {}\nCursor excerpt did not exist in buffer.",
                example.spec.name
            )
        })?;
        anyhow::ensure!(
            matches.next().is_none(),
            "More than one cursor position match found for {}",
            &example.spec.name
        );
        Ok(excerpt_offset)
    })??;

    let cursor_offset = excerpt_offset + cursor_offset_within_excerpt;
    let cursor_anchor =
        cursor_buffer.read_with(cx, |buffer, _| buffer.anchor_after(cursor_offset))?;

    Ok((cursor_buffer, cursor_anchor))
}

async fn setup_project(
    example: &mut Example,
    app_state: &Arc<EpAppState>,
    step_progress: &StepProgress,
    cx: &mut AsyncApp,
) -> Result<Entity<Project>> {
    let ep_store = cx
        .update(|cx| EditPredictionStore::try_global(cx))?
        .context("Store should be initialized at init")?;

    let worktree_path = setup_worktree(example, step_progress).await?;

    if let Some(project) = app_state.project_cache.get(&example.spec.repository_url) {
        ep_store.update(cx, |ep_store, _| {
            ep_store.clear_history_for_project(&project);
        })?;
        let buffer_store = project.read_with(cx, |project, _| project.buffer_store().clone())?;
        let buffers = buffer_store.read_with(cx, |buffer_store, _| {
            buffer_store.buffers().collect::<Vec<_>>()
        })?;
        for buffer in buffers {
            buffer
                .update(cx, |buffer, cx| buffer.reload(cx))?
                .await
                .ok();
        }
        return Ok(project);
    }

    let project = cx.update(|cx| {
        Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            false,
            cx,
        )
    })?;

    project
        .update(cx, |project, cx| {
            project.disable_worktree_scanner(cx);
            project.create_worktree(&worktree_path, true, cx)
        })?
        .await?;

    app_state
        .project_cache
        .insert(example.spec.repository_url.clone(), project.clone());

    let buffer_store = project.read_with(cx, |project, _| project.buffer_store().clone())?;
    cx.subscribe(&buffer_store, {
        let project = project.clone();
        move |_, event, cx| match event {
            BufferStoreEvent::BufferAdded(buffer) => {
                ep_store.update(cx, |store, cx| store.register_buffer(&buffer, &project, cx));
            }
            _ => {}
        }
    })?
    .detach();

    Ok(project)
}

async fn setup_worktree(example: &Example, step_progress: &StepProgress) -> Result<PathBuf> {
    let (repo_owner, repo_name) = example.repo_name().context("failed to get repo name")?;
    let repo_dir = git::repo_path_for_url(&example.spec.repository_url)?;
    let worktree_path = WORKTREES_DIR
        .join(repo_owner.as_ref())
        .join(repo_name.as_ref());
    let repo_lock = git::lock_repo(&repo_dir).await;

    if !repo_dir.is_dir() {
        step_progress.set_substatus(format!("cloning {}", repo_name));
        fs::create_dir_all(&repo_dir)?;
        git::run_git(&repo_dir, &["init"]).await?;
        git::run_git(
            &repo_dir,
            &["remote", "add", "origin", &example.spec.repository_url],
        )
        .await?;
    }

    // Resolve the example to a revision, fetching it if needed.
    step_progress.set_substatus("fetching");
    let revision = git::fetch_if_needed(&repo_dir, &example.spec.revision).await?;

    // Create the worktree for this example if needed.
    step_progress.set_substatus("preparing worktree");
    if worktree_path.is_dir() {
        git::run_git(&worktree_path, &["clean", "--force", "-d"]).await?;
        git::run_git(&worktree_path, &["reset", "--hard", "HEAD"]).await?;
        git::run_git(&worktree_path, &["checkout", revision.as_str()]).await?;
    } else {
        let worktree_path_string = worktree_path.to_string_lossy();
        let branch_name = example.spec.filename();
        git::run_git(
            &repo_dir,
            &["branch", "-f", &branch_name, revision.as_str()],
        )
        .await?;
        git::run_git(
            &repo_dir,
            &["worktree", "add", "-f", &worktree_path_string, &branch_name],
        )
        .await?;
    }
    drop(repo_lock);

    // Apply the uncommitted diff for this example.
    if !example.spec.uncommitted_diff.is_empty() {
        step_progress.set_substatus("applying diff");
        let mut apply_process = smol::process::Command::new("git")
            .current_dir(&worktree_path)
            .args(&["apply", "-"])
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        let mut stdin = apply_process.stdin.take().context("Failed to get stdin")?;
        stdin
            .write_all(example.spec.uncommitted_diff.as_bytes())
            .await?;
        stdin.close().await?;
        drop(stdin);

        let apply_result = apply_process.output().await?;
        anyhow::ensure!(
            apply_result.status.success(),
            "Failed to apply uncommitted diff patch with status: {}\nstderr:\n{}\nstdout:\n{}",
            apply_result.status,
            String::from_utf8_lossy(&apply_result.stderr),
            String::from_utf8_lossy(&apply_result.stdout),
        );
    }

    step_progress.clear_substatus();
    Ok(worktree_path)
}

async fn apply_edit_history(
    example: &Example,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<OpenedBuffers> {
    edit_prediction::udiff::apply_diff(&example.spec.edit_history, project, cx).await
}
