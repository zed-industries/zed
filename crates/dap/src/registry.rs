use anyhow::Result;
use async_trait::async_trait;
use collections::FxHashMap;
use gpui::{App, Global, SharedString};
use language::LanguageName;
use parking_lot::RwLock;
use task::{
    AdapterSchema, AdapterSchemas, DebugRequest, DebugScenario, SpawnInTerminal, TaskTemplate,
};

use crate::adapters::{DebugAdapter, DebugAdapterName};
use std::{collections::BTreeMap, sync::Arc};

/// Given a user build configuration, locator creates a fill-in debug target ([DebugScenario]) on behalf of the user.
#[async_trait]
pub trait DapLocator: Send + Sync {
    fn name(&self) -> SharedString;
    /// Determines whether this locator can generate debug target for given task.
    async fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: &DebugAdapterName,
    ) -> Option<DebugScenario>;

    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest>;
}

#[derive(Default)]
struct DapRegistryState {
    adapters: BTreeMap<DebugAdapterName, Arc<dyn DebugAdapter>>,
    locators: FxHashMap<SharedString, Arc<dyn DapLocator>>,
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

    pub fn add_locator(&self, locator: Arc<dyn DapLocator>) {
        self.0.write().locators.insert(locator.name(), locator);
    }

    pub fn remove_adapter(&self, name: &str) {
        self.0.write().adapters.remove(name);
    }

    pub fn remove_locator(&self, locator: &str) {
        self.0.write().locators.remove(locator);
    }

    pub fn adapter_language(&self, adapter_name: &str) -> Option<LanguageName> {
        self.adapter(adapter_name)
            .and_then(|adapter| adapter.adapter_language_name())
    }

    pub fn adapters_schema(&self) -> task::AdapterSchemas {
        let mut schemas = vec![];

        let adapters = &self.0.read().adapters;

        for (name, adapter) in adapters.into_iter() {
            schemas.push(AdapterSchema {
                adapter: name.clone().into(),
                schema: adapter.dap_schema(),
            });
        }

        AdapterSchemas(schemas)
    }

    pub fn locators(&self) -> FxHashMap<SharedString, Arc<dyn DapLocator>> {
        self.0.read().locators.clone()
    }

    pub fn adapter(&self, name: &str) -> Option<Arc<dyn DebugAdapter>> {
        self.0.read().adapters.get(name).cloned()
    }

    pub fn enumerate_adapters<B: FromIterator<DebugAdapterName>>(&self) -> B {
        self.0.read().adapters.keys().cloned().collect()
    }
}
