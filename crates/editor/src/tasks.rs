use crate::Editor;

use collections::HashMap;
use gpui::{App, Task, Window};
use lsp::LanguageServerName;
use project::{Location, project_settings::ProjectSettings};
use settings::Settings as _;
use task::{TaskContext, TaskVariables, VariableName};
use text::{BufferId, ToOffset, ToPoint};

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

    pub fn lsp_task_sources(&self, cx: &App) -> HashMap<LanguageServerName, Vec<BufferId>> {
        let lsp_settings = &ProjectSettings::get_global(cx).lsp;

        self.buffer()
            .read(cx)
            .all_buffers()
            .into_iter()
            .filter_map(|buffer| {
                let lsp_tasks_source = buffer
                    .read(cx)
                    .language()?
                    .context_provider()?
                    .lsp_task_source()?;
                if lsp_settings
                    .get(&lsp_tasks_source)
                    .is_none_or(|s| s.enable_lsp_tasks)
                {
                    let buffer_id = buffer.read(cx).remote_id();
                    Some((lsp_tasks_source, buffer_id))
                } else {
                    None
                }
            })
            .fold(
                HashMap::default(),
                |mut acc, (lsp_task_source, buffer_id)| {
                    acc.entry(lsp_task_source)
                        .or_insert_with(Vec::new)
                        .push(buffer_id);
                    acc
                },
            )
    }
}
