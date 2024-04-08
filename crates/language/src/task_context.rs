use crate::Location;

use anyhow::Result;
use gpui::AppContext;
use task::{TaskDefinitions, TaskVariables, VariableName};

/// Language Contexts are used by Zed tasks to extract information about source file.
pub trait ContextProvider: Send + Sync {
    fn build_context(&self, _: Location, _: &mut AppContext) -> Result<TaskVariables> {
        Ok(TaskVariables::default())
    }

    fn associated_tasks(&self) -> Option<TaskDefinitions> {
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
    definitions: TaskDefinitions,
}

impl ContextProviderWithTasks {
    pub fn new(definitions: TaskDefinitions) -> Self {
        Self { definitions }
    }
}

impl ContextProvider for ContextProviderWithTasks {
    fn associated_tasks(&self) -> Option<TaskDefinitions> {
        Some(self.definitions.clone())
    }

    fn build_context(&self, location: Location, cx: &mut AppContext) -> Result<TaskVariables> {
        SymbolContextProvider.build_context(location, cx)
    }
}
