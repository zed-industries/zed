use std::{ops::Range, path::Path};

use crate::{Location, Runnable};

use anyhow::Result;
use collections::HashMap;
use gpui::AppContext;
use task::{TaskTemplates, TaskVariables, VariableName};
use text::{BufferId, Point, ToPoint};

pub struct RunnableRange {
    pub buffer_id: BufferId,
    pub run_range: Range<usize>,
    pub runnable: Runnable,
    pub extra_captures: HashMap<String, String>,
}
/// Language Contexts are used by Zed tasks to extract information about the source file where the tasks are supposed to be scheduled from.
/// Multiple context providers may be used together: by default, Zed provides a base [`BasicContextProvider`] context that fills all non-custom [`VariableName`] variants.
///
/// The context will be used to fill data for the tasks, and filter out the ones that do not have the variables required.
pub trait ContextProvider: Send + Sync {
    /// Builds a specific context to be placed on top of the basic one (replacing all conflicting entries) and to be used for task resolving later.
    fn build_context(
        &self,
        _worktree_abs_path: Option<&Path>,
        _location: &Location,
        _cx: &mut AppContext,
    ) -> Result<TaskVariables> {
        Ok(TaskVariables::default())
    }

    /// Provides all tasks, associated with the current language.
    fn associated_tasks(&self) -> Option<TaskTemplates> {
        None
    }

    // Determines whether the [`BasicContextProvider`] variables should be filled too (if `false`), or omitted (if `true`).
    fn is_basic(&self) -> bool {
        false
    }
}

/// A context provided that tries to provide values for all non-custom [`VariableName`] variants for a currently opened file.
/// Applied as a base for every custom [`ContextProvider`] unless explicitly oped out.
pub struct BasicContextProvider;

impl ContextProvider for BasicContextProvider {
    fn is_basic(&self) -> bool {
        true
    }

    fn build_context(
        &self,
        worktree_abs_path: Option<&Path>,
        location: &Location,
        cx: &mut AppContext,
    ) -> Result<TaskVariables> {
        let buffer = location.buffer.read(cx);
        let buffer_snapshot = buffer.snapshot();
        let symbols = buffer_snapshot.symbols_containing(location.range.start, None);
        let symbol = symbols.unwrap_or_default().last().map(|symbol| {
            let range = symbol
                .name_ranges
                .last()
                .cloned()
                .unwrap_or(0..symbol.text.len());
            symbol.text[range].to_string()
        });

        let current_file = buffer
            .file()
            .and_then(|file| file.as_local())
            .map(|file| file.abs_path(cx).to_string_lossy().to_string());
        let Point { row, column } = location.range.start.to_point(&buffer_snapshot);
        let row = row + 1;
        let column = column + 1;
        let selected_text = buffer
            .chars_for_range(location.range.clone())
            .collect::<String>();

        let mut task_variables = TaskVariables::from_iter([
            (VariableName::Row, row.to_string()),
            (VariableName::Column, column.to_string()),
        ]);

        if let Some(symbol) = symbol {
            task_variables.insert(VariableName::Symbol, symbol);
        }
        if !selected_text.trim().is_empty() {
            task_variables.insert(VariableName::SelectedText, selected_text);
        }
        if let Some(path) = current_file {
            task_variables.insert(VariableName::File, path);
        }
        if let Some(worktree_path) = worktree_abs_path {
            task_variables.insert(
                VariableName::WorktreeRoot,
                worktree_path.to_string_lossy().to_string(),
            );
        }

        Ok(task_variables)
    }
}

/// A ContextProvider that doesn't provide any task variables on it's own, though it has some associated tasks.
pub struct ContextProviderWithTasks {
    templates: TaskTemplates,
}

impl ContextProviderWithTasks {
    pub fn new(definitions: TaskTemplates) -> Self {
        Self {
            templates: definitions,
        }
    }
}

impl ContextProvider for ContextProviderWithTasks {
    fn associated_tasks(&self) -> Option<TaskTemplates> {
        Some(self.templates.clone())
    }

    fn build_context(
        &self,
        worktree_abs_path: Option<&Path>,
        location: &Location,
        cx: &mut AppContext,
    ) -> Result<TaskVariables> {
        BasicContextProvider.build_context(worktree_abs_path, location, cx)
    }
}
