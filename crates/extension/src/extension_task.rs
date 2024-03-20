use language::LanguageContextProvider;
use task::static_source::DefinitionProvider;

pub(super) struct ExtensionContextProvider {
    definitions: DefinitionProvider,
}

impl ExtensionContextProvider {
    pub(super) fn new(definitions: DefinitionProvider) -> Self {
        Self { definitions }
    }
}

impl LanguageContextProvider for ExtensionContextProvider {
    fn associated_tasks(&self) -> Option<DefinitionProvider> {
        Some(self.definitions.clone())
    }
}
