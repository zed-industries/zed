use crate::Location;

use anyhow::Result;
use gpui::AppContext;
use task::{TaskTemplates, TaskVariables, VariableName};

/// Language Contexts are used by Zed tasks to extract information about source file.
pub trait ContextProvider: Send + Sync {
    fn build_context(&self, _: Location, _: &mut AppContext) -> Result<TaskVariables> {
        Ok(TaskVariables::default())
    }

    fn associated_tasks(&self) -> Option<TaskTemplates> {
        None
    }
}

/// A context provider that finds out what symbol is currently focused in the buffer.
pub struct SymbolContextProvider;

impl ContextProvider for SymbolContextProvider {
    fn build_context(
        &self,
        location: Location,
        cx: &mut AppContext,
    ) -> gpui::Result<TaskVariables> {
        let symbols = location
            .buffer
            .read(cx)
            .snapshot()
            .symbols_containing(location.range.start, None);
        let symbol = symbols.unwrap_or_default().last().map(|symbol| {
            let range = symbol
                .name_ranges
                .last()
                .cloned()
                .unwrap_or(0..symbol.text.len());
            symbol.text[range].to_string()
        });
        Ok(TaskVariables::from_iter(
            Some(VariableName::Symbol).zip(symbol),
        ))
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

    fn build_context(&self, location: Location, cx: &mut AppContext) -> Result<TaskVariables> {
        SymbolContextProvider.build_context(location, cx)
    }
}
