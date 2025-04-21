use gpui::{App, Global};
use parking_lot::RwLock;

use crate::adapters::{DebugAdapter, DebugAdapterName};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Default)]
struct DapRegistryState {
    adapters: BTreeMap<DebugAdapterName, Arc<dyn DebugAdapter>>,
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

    pub fn adapter(&self, name: &str) -> Option<Arc<dyn DebugAdapter>> {
        self.0.read().adapters.get(name).cloned()
    }

    pub fn enumerate_adapters(&self) -> Vec<DebugAdapterName> {
        self.0.read().adapters.keys().cloned().collect()
    }
}
