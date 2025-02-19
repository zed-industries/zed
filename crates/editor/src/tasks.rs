use crate::Editor;

use gpui::{App, AppContext as _, Task as AsyncTask, Window};
use project::Location;
use task::{TaskContext, TaskVariables, VariableName};
use text::{ToOffset, ToPoint};
use workspace::Workspace;

fn task_context_with_editor(
    editor: &mut Editor,
    window: &mut Window,
    cx: &mut App,
) -> AsyncTask<Option<TaskContext>> {
    let Some(project) = editor.project.clone() else {
        return AsyncTask::ready(None);
    };
    let (selection, buffer, editor_snapshot) = {
        let selection = editor.selections.newest_adjusted(cx);
        let Some((buffer, _)) = editor
            .buffer()
            .read(cx)
            .point_to_buffer_offset(selection.start, cx)
        else {
            return AsyncTask::ready(None);
        };
        let snapshot = editor.snapshot(window, cx);
        (selection, buffer, snapshot)
    };
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
    let captured_variables = {
        let mut variables = TaskVariables::default();
        let buffer = location.buffer.read(cx);
        let buffer_id = buffer.remote_id();
        let snapshot = buffer.snapshot();
        let starting_point = location.range.start.to_point(&snapshot);
        let starting_offset = starting_point.to_offset(&snapshot);
        for (_, tasks) in editor
            .tasks
            .range((buffer_id, 0)..(buffer_id, starting_point.row + 1))
        {
            if !tasks
                .context_range
                .contains(&crate::BufferOffset(starting_offset))
            {
                continue;
            }
            for (capture_name, value) in tasks.extra_variables.iter() {
                variables.insert(
                    VariableName::Custom(capture_name.to_owned().into()),
                    value.clone(),
                );
            }
        }
        variables
    };

    project.update(cx, |project, cx| {
        project.task_store().update(cx, |task_store, cx| {
            task_store.task_context_for_location(captured_variables, location, cx)
        })
    })
}

pub fn task_context(
    workspace: &Workspace,
    window: &mut Window,
    cx: &mut App,
) -> AsyncTask<TaskContext> {
    let Some(editor) = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
    else {
        return AsyncTask::ready(TaskContext::default());
    };
    editor.update(cx, |editor, cx| {
        let context_task = task_context_with_editor(editor, window, cx);
        cx.background_spawn(async move { context_task.await.unwrap_or_default() })
    })
}
