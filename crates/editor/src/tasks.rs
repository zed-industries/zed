use crate::Editor;

use std::{path::Path, sync::Arc};

use anyhow::Context;
use gpui::WindowContext;
use language::{BasicContextProvider, ContextProvider};
use project::{Location, WorktreeId};
use task::{TaskContext, TaskVariables, VariableName};
use util::ResultExt;
use workspace::Workspace;

pub(crate) fn task_context_for_location(
    workspace: &Workspace,
    location: Location,
    cx: &mut WindowContext<'_>,
) -> Option<TaskContext> {
    let cwd = workspace::tasks::task_cwd(workspace, cx)
        .log_err()
        .flatten();

    let buffer = location.buffer.clone();
    let language_context_provider = buffer
        .read(cx)
        .language()
        .and_then(|language| language.context_provider())
        .unwrap_or_else(|| Arc::new(BasicContextProvider));

    let worktree_abs_path = buffer
        .read(cx)
        .file()
        .map(|file| WorktreeId::from_usize(file.worktree_id()))
        .and_then(|worktree_id| {
            workspace
                .project()
                .read(cx)
                .worktree_for_id(worktree_id, cx)
                .map(|worktree| worktree.read(cx).abs_path())
        });
    let task_variables = combine_task_variables(
        worktree_abs_path.as_deref(),
        location,
        language_context_provider.as_ref(),
        cx,
    )
    .log_err()?;
    Some(TaskContext {
        cwd,
        task_variables,
    })
}

pub(crate) fn task_context_with_editor(
    workspace: &Workspace,
    editor: &mut Editor,
    cx: &mut WindowContext<'_>,
) -> Option<TaskContext> {
    let (selection, buffer, editor_snapshot) = {
        let selection = editor.selections.newest::<usize>(cx);
        let (buffer, _, _) = editor
            .buffer()
            .read(cx)
            .point_to_buffer_offset(selection.start, cx)?;
        let snapshot = editor.snapshot(cx);
        Some((selection, buffer, snapshot))
    }?;
    let selection_range = selection.range();
    let start = editor_snapshot
        .display_snapshot
        .buffer_snapshot
        .anchor_after(selection_range.start)
        .text_anchor;
    let end = editor_snapshot
        .display_snapshot
        .buffer_snapshot
        .anchor_after(selection_range.end)
        .text_anchor;
    let location = Location {
        buffer,
        range: start..end,
    };
    task_context_for_location(workspace, location.clone(), cx).map(|mut task_context| {
        for range in location
            .buffer
            .read(cx)
            .snapshot()
            .runnable_ranges(location.range)
        {
            for (capture_name, value) in range.extra_captures {
                task_context
                    .task_variables
                    .insert(VariableName::Custom(capture_name.into()), value);
            }
        }
        task_context
    })
}

pub fn task_context(workspace: &Workspace, cx: &mut WindowContext<'_>) -> TaskContext {
    let Some(editor) = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
    else {
        return Default::default();
    };
    editor.update(cx, |editor, cx| {
        task_context_with_editor(workspace, editor, cx).unwrap_or_default()
    })
}

fn combine_task_variables(
    worktree_abs_path: Option<&Path>,
    location: Location,
    context_provider: &dyn ContextProvider,
    cx: &mut WindowContext<'_>,
) -> anyhow::Result<TaskVariables> {
    if context_provider.is_basic() {
        context_provider
            .build_context(worktree_abs_path, &location, cx)
            .context("building basic provider context")
    } else {
        let mut basic_context = BasicContextProvider
            .build_context(worktree_abs_path, &location, cx)
            .context("building basic default context")?;
        basic_context.extend(
            context_provider
                .build_context(worktree_abs_path, &location, cx)
                .context("building provider context ")?,
        );
        Ok(basic_context)
    }
}
