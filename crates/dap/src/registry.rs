use anyhow::Result;
use async_trait::async_trait;
use collections::FxHashMap;
use gpui::{App, Global, SharedString};
use language::LanguageName;
use parking_lot::RwLock;
use task::{
    AdapterSchema, AdapterSchemas, DebugRequest, DebugScenario, SpawnInTerminal, TaskTemplate,
};

use crate::{
    adapters::{DebugAdapter, DebugAdapterName},
    inline_value::InlineValueProvider,
};
use std::{collections::BTreeMap, sync::Arc};

/// Given a user build configuration, locator creates a fill-in debug target ([DebugRequest]) on behalf of the user.
#[async_trait]
pub trait DapLocator: Send + Sync {
    fn name(&self) -> SharedString;
    /// Determines whether this locator can generate debug target for given task.
    fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: DebugAdapterName,
    ) -> Option<DebugScenario>;

    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest>;
}

#[derive(Default)]
struct DapRegistryState {
    adapters: BTreeMap<DebugAdapterName, Arc<dyn DebugAdapter>>,
    locators: FxHashMap<SharedString, Arc<dyn DapLocator>>,
    inline_value_providers: FxHashMap<String, Arc<dyn InlineValueProvider>>,
}

#[derive(Clone, Default)]
/// Stores available debug adapters.
pub struct DapRegistry(Arc<RwLock<DapRegistryState>>);
impl Global for DapRegistry {}

impl DapRegistry {
    pub fn global(cx: &mut App) -> &mut Self {
        cx.default_global::<Self>()
    }

    pub fn add_adapter(&self, adapter: Arc<dyn DebugAdapter>) {
        let name = adapter.name();
        let _previous_value = self.0.write().adapters.insert(name, adapter);
    }

    pub fn adapter_language(&self, adapter_name: &str) -> Option<LanguageName> {
        self.adapter(adapter_name)
            .and_then(|adapter| adapter.adapter_language_name())
    }

    pub fn add_locator(&self, locator: Arc<dyn DapLocator>) {
        let _previous_value = self.0.write().locators.insert(locator.name(), locator);
        debug_assert!(
            _previous_value.is_none(),
            "Attempted to insert a new debug locator when one is already registered"
        );
    }

    pub async fn adapters_schema(&self) -> task::AdapterSchemas {
        let mut schemas = AdapterSchemas(vec![]);

        // Clone to avoid holding lock over await points
        let adapters = self.0.read().adapters.clone();

        for (name, adapter) in adapters.into_iter() {
            schemas.0.push(AdapterSchema {
                adapter: name.into(),
                schema: adapter.dap_schema().await,
            });
        }

        schemas
    }

    pub fn add_inline_value_provider(
        &self,
        language: String,
        provider: Arc<dyn InlineValueProvider>,
    ) {
        let _previous_value = self
            .0
            .write()
            .inline_value_providers
            .insert(language, provider);
        debug_assert!(
            _previous_value.is_none(),
            "Attempted to insert a new inline value provider when one is already registered"
        );
    }

    pub fn locators(&self) -> FxHashMap<SharedString, Arc<dyn DapLocator>> {
        self.0.read().locators.clone()
    }

    pub fn adapter(&self, name: &str) -> Option<Arc<dyn DebugAdapter>> {
        self.0.read().adapters.get(name).cloned()
    }

    pub fn inline_value_provider(&self, language: &str) -> Option<Arc<dyn InlineValueProvider>> {
        self.0.read().inline_value_providers.get(language).cloned()
    }

    pub fn enumerate_adapters(&self) -> Vec<DebugAdapterName> {
        self.0.read().adapters.keys().cloned().collect()
    }
}
