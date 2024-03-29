use crate::{LanguageRegistry, Location};

use anyhow::Result;
use gpui::{AppContext, Context, Model};
use std::sync::Arc;
use task::{static_source::tasks_for, static_source::TaskDefinitions, TaskSource, TaskVariables};

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

/// A source that pulls in the tasks from language registry.
pub struct LanguageSource {
    languages: Arc<LanguageRegistry>,
}

impl LanguageSource {
    pub fn new(
        languages: Arc<LanguageRegistry>,
        cx: &mut AppContext,
    ) -> Model<Box<dyn TaskSource>> {
        cx.new_model(|_| Box::new(Self { languages }) as Box<_>)
    }
}

impl TaskSource for LanguageSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn tasks_for_path(
        &mut self,
        _: Option<&std::path::Path>,
        _: &mut gpui::ModelContext<Box<dyn TaskSource>>,
    ) -> Vec<Arc<dyn task::Task>> {
        self.languages
            .to_vec()
            .into_iter()
            .filter_map(|language| {
                language
                    .context_provider()?
                    .associated_tasks()
                    .map(|tasks| (tasks, language))
            })
            .flat_map(|(tasks, language)| {
                let language_name = language.name();
                let id_base = format!("buffer_source_{language_name}");
                tasks_for(tasks, &id_base)
            })
            .collect()
    }
}
