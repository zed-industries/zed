use anyhow::Result;
use gpui::AppContext;
use task::{static_source::TaskDefinitions, TaskVariables};

use crate::Location;

/// Language Contexts are used by Zed tasks to extract information about source file.
pub trait ContextProvider: Send + Sync {
    fn build_context(&self, _: Location, _: &mut AppContext) -> Result<TaskVariables> {
        Ok(TaskVariables::default())
    }
    fn associated_tasks(&self) -> Option<TaskDefinitions> {
        None
    }
}

/// A context provider that fills out LanguageContext without inspecting the contents.
pub struct DefaultContextProvider;

impl ContextProvider for DefaultContextProvider {
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
        let symbol = symbols.and_then(|symbols| {
            symbols.last().map(|symbol| {
                let range = symbol
                    .name_ranges
                    .last()
                    .cloned()
                    .unwrap_or(0..symbol.text.len());
                symbol.text[range].to_string()
            })
        });
        Ok(TaskVariables::from_iter(
            symbol.map(|symbol| ("ZED_SYMBOL".to_string(), symbol)),
        ))
    }
}
