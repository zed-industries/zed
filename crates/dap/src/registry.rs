use anyhow::Result;
use async_trait::async_trait;
use collections::FxHashMap;
use gpui::{App, Global};
use parking_lot::RwLock;
use task::{DebugRequest, SpawnInTerminal};

use crate::adapters::{DebugAdapter, DebugAdapterName};
use std::{collections::BTreeMap, sync::Arc};

/// Given a user build configuration, locator creates a fill-in debug target ([DebugRequest]) on behalf of the user.
#[async_trait]
pub trait DapLocator: Send + Sync {
    /// Determines whether this locator can generate debug target for given task.
    fn accepts(&self, build_config: &SpawnInTerminal) -> bool;
    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest>;
}

#[derive(Default)]
struct DapRegistryState {
    adapters: BTreeMap<DebugAdapterName, Arc<dyn DebugAdapter>>,
    locators: FxHashMap<String, Arc<dyn DapLocator>>,
}

#[derive(Clone, Default)]
/// Stores available debug adapters.
pub struct DapRegistry(Arc<RwLock<DapRegistryState>>);
impl Global for DapRegistry {}

impl DapRegistry {
    pub fn global(cx: &mut App) -> &mut Self {
        let ret = cx.default_global::<Self>();

        #[cfg(any(test, feature = "test-support"))]
        if ret.adapter(crate::FakeAdapter::ADAPTER_NAME).is_none() {
            ret.add_adapter(Arc::new(crate::FakeAdapter::new()));
        }

        ret
    }

    pub fn add_adapter(&self, adapter: Arc<dyn DebugAdapter>) {
        let name = adapter.name();
        let _previous_value = self.0.write().adapters.insert(name, adapter);
        debug_assert!(
            _previous_value.is_none(),
            "Attempted to insert a new debug adapter when one is already registered"
        );
    }

    pub fn add_locator(&self, name: String, locator: Arc<dyn DapLocator>) {
        let _previous_value = self.0.write().locators.insert(name, locator);
        debug_assert!(
            _previous_value.is_none(),
            "Attempted to insert a new debug locator when one is already registered"
        );
    }

    pub fn locators(&self) -> FxHashMap<String, Arc<dyn DapLocator>> {
        self.0.read().locators.clone()
    }

    pub fn adapter(&self, name: &str) -> Option<Arc<dyn DebugAdapter>> {
        self.0.read().adapters.get(name).cloned()
    }

    pub fn enumerate_adapters(&self) -> Vec<DebugAdapterName> {
        self.0.read().adapters.keys().cloned().collect()
    }
}
