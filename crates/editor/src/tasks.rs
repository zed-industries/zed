use crate::Editor;

use anyhow::Context;
use gpui::{Model, WindowContext};
use language::ContextProvider;
use project::{BasicContextProvider, Location, Project};
use task::{TaskContext, TaskVariables, VariableName};
use text::{Point, ToOffset, ToPoint};
use util::ResultExt;
use workspace::Workspace;

pub(crate) fn task_context_for_location(
    captured_variables: TaskVariables,
    workspace: &Workspace,
    location: Location,
    cx: &mut WindowContext<'_>,
) -> Option<TaskContext> {
    let cwd = workspace::tasks::task_cwd(workspace, cx)
        .log_err()
        .flatten();

    let mut task_variables = combine_task_variables(
        captured_variables,
        location,
        workspace.project().clone(),
        cx,
    )
    .log_err()?;
    // Remove all custom entries starting with _, as they're not intended for use by the end user.
    task_variables.sweep();

    Some(TaskContext {
        cwd,
        task_variables,
    })
}

fn task_context_with_editor(
    workspace: &Workspace,
    editor: &mut Editor,
    cx: &mut WindowContext<'_>,
) -> Option<TaskContext> {
    let (selection, buffer, editor_snapshot) = {
        let mut selection = editor.selections.newest::<Point>(cx);
        if editor.selections.line_mode {
            selection.start = Point::new(selection.start.row, 0);
            selection.end = Point::new(selection.end.row + 1, 0);
        }
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
    task_context_for_location(captured_variables, workspace, location.clone(), cx)
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
    mut captured_variables: TaskVariables,
    location: Location,
    project: Model<Project>,
    cx: &mut WindowContext<'_>,
) -> anyhow::Result<TaskVariables> {
    let language_context_provider = location
        .buffer
        .read(cx)
        .language()
        .and_then(|language| language.context_provider());
    let baseline = BasicContextProvider::new(project)
        .build_context(&captured_variables, &location, cx)
        .context("building basic default context")?;
    captured_variables.extend(baseline);
    if let Some(provider) = language_context_provider {
        captured_variables.extend(
            provider
                .build_context(&captured_variables, &location, cx)
                .context("building provider context ")?,
        );
    }
    Ok(captured_variables)
}
