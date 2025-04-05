use crate::Editor;

use gpui::{App, Task, Window};
use project::Location;
use task::{TaskContext, TaskVariables, VariableName};
use text::{ToOffset, ToPoint};

impl Editor {
    pub fn task_context(&self, window: &mut Window, cx: &mut App) -> Task<Option<TaskContext>> {
        let Some(project) = self.project.clone() else {
            return Task::ready(None);
        };
        let (selection, buffer, editor_snapshot) = {
            let selection = self.selections.newest_adjusted(cx);
            let Some((buffer, _)) = self
                .buffer()
                .read(cx)
                .point_to_buffer_offset(selection.start, cx)
            else {
                return Task::ready(None);
            };
            let snapshot = self.snapshot(window, cx);
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
            for (_, tasks) in self
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
}
