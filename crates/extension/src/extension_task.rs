use language::LanguageContextProvider;
use task::static_source::TaskDefinitions;

pub(super) struct ExtensionContextProvider {
    definitions: TaskDefinitions,
}

impl ExtensionContextProvider {
    pub(super) fn new(definitions: TaskDefinitions) -> Self {
        Self { definitions }
    }
}

impl LanguageContextProvider for ExtensionContextProvider {
    fn associated_tasks(&self) -> Option<TaskDefinitions> {
        Some(self.definitions.clone())
    }
}
