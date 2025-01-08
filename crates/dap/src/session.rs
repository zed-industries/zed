use collections::HashMap;
use gpui::ModelContext;
use std::sync::Arc;
use task::DebugAdapterConfig;

use crate::client::{DebugAdapterClient, DebugAdapterClientId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugSessionId(pub usize);

impl DebugSessionId {
    pub fn from_proto(client_id: u64) -> Self {
        Self(client_id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }
}

pub struct DebugSession {
    id: DebugSessionId,
    ignore_breakpoints: bool,
    configuration: DebugAdapterConfig,
    clients: HashMap<DebugAdapterClientId, Arc<DebugAdapterClient>>,
}

impl DebugSession {
    pub fn new(id: DebugSessionId, configuration: DebugAdapterConfig) -> Self {
        Self {
            id,
            configuration,
            ignore_breakpoints: false,
            clients: HashMap::default(),
        }
    }

    pub fn id(&self) -> DebugSessionId {
        self.id
    }

    pub fn name(&self) -> String {
        self.configuration.label.clone()
    }

    pub fn configuration(&self) -> &DebugAdapterConfig {
        &self.configuration
    }

    pub fn ignore_breakpoints(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn set_ignore_breakpoints(&mut self, ignore: bool, cx: &mut ModelContext<Self>) {
        self.ignore_breakpoints = ignore;
        cx.notify();
    }

    pub fn update_configuration(
        &mut self,
        f: impl FnOnce(&mut DebugAdapterConfig),
        cx: &mut ModelContext<Self>,
    ) {
        f(&mut self.configuration);
        cx.notify();
    }

    pub fn add_client(&mut self, client: Arc<DebugAdapterClient>, cx: &mut ModelContext<Self>) {
        self.clients.insert(client.id(), client);
        cx.notify();
    }

    pub fn remove_client(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<Self>,
    ) -> Option<Arc<DebugAdapterClient>> {
        let client = self.clients.remove(client_id);
        cx.notify();

        client
    }

    pub fn client_by_id(
        &self,
        client_id: &DebugAdapterClientId,
    ) -> Option<Arc<DebugAdapterClient>> {
        self.clients.get(client_id).cloned()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn clients_len(&self) -> usize {
        self.clients.len()
    }

    pub fn clients(&self) -> impl Iterator<Item = Arc<DebugAdapterClient>> + '_ {
        self.clients.values().cloned()
    }
}
