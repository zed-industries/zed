use std::ops::Range;

use crate::{Location, Runnable};

use anyhow::Result;
use collections::HashMap;
use gpui::AppContext;
use task::{TaskTemplates, TaskVariables};
use text::BufferId;

pub struct RunnableRange {
    pub buffer_id: BufferId,
    pub run_range: Range<usize>,
    pub full_range: Range<usize>,
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
        _variables: &TaskVariables,
        _location: &Location,
        _cx: &mut AppContext,
    ) -> Result<TaskVariables> {
        Ok(TaskVariables::default())
    }

    /// Provides all tasks, associated with the current language.
    fn associated_tasks(&self) -> Option<TaskTemplates> {
        None
    }
}
