use language::ContextProvider;
use task::static_source::TaskDefinitions;

pub(super) struct ExtensionContextProvider {
    definitions: TaskDefinitions,
}

impl ExtensionContextProvider {
    pub(super) fn new(definitions: TaskDefinitions) -> Self {
        Self { definitions }
    }
}

impl ContextProvider for ExtensionContextProvider {
    fn associated_tasks(&self) -> Option<TaskDefinitions> {
        Some(self.definitions.clone())
    }
}
