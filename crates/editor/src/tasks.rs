use crate::Editor;

use gpui::{Task as AsyncTask, WindowContext};
use project::Location;
use task::{TaskContext, TaskVariables, VariableName};
use text::{Point, ToOffset, ToPoint};
use workspace::Workspace;

fn task_context_with_editor(
    editor: &mut Editor,
    cx: &mut WindowContext<'_>,
) -> AsyncTask<Option<TaskContext>> {
    let Some(project) = editor.project.clone() else {
        return AsyncTask::ready(None);
    };
    let (selection, buffer, editor_snapshot) = {
        let mut selection = editor.selections.newest::<Point>(cx);
        if editor.selections.line_mode {
            selection.start = Point::new(selection.start.row, 0);
            selection.end = Point::new(selection.end.row + 1, 0);
        }
        let Some((buffer, _, _)) = editor
            .buffer()
            .read(cx)
            .point_to_buffer_offset(selection.start, cx)
        else {
            return AsyncTask::ready(None);
        };
        let snapshot = editor.snapshot(cx);
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

    let context_task = project.update(cx, |project, cx| {
        project.task_context_for_location(captured_variables, location.clone(), cx)
    });
    cx.spawn(|_| context_task)
}

pub fn task_context(workspace: &Workspace, cx: &mut WindowContext<'_>) -> AsyncTask<TaskContext> {
    let Some(editor) = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
    else {
        return AsyncTask::ready(TaskContext::default());
    };
    editor.update(cx, |editor, cx| {
        let context_task = task_context_with_editor(editor, cx);
        cx.background_executor()
            .spawn(async move { context_task.await.unwrap_or_default() })
    })
}
