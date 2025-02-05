use collections::HashMap;
use gpui::Context;
use std::sync::Arc;
use task::DebugAdapterConfig;

use crate::client::{DebugAdapterClient, DebugAdapterClientId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugSessionId(pub usize);

impl DebugSessionId {
    pub fn from_proto(session_id: u64) -> Self {
        Self(session_id as usize)
    }

    pub fn to_proto(&self) -> u64 {
        self.0 as u64
    }
}

pub struct DebugSession {
    id: DebugSessionId,
    mode: DebugSessionMode,
    ignore_breakpoints: bool,
}

pub enum DebugSessionMode {
    Local(LocalDebugSession),
    Remote(RemoteDebugSession),
}

pub struct LocalDebugSession {
    configuration: DebugAdapterConfig,
    clients: HashMap<DebugAdapterClientId, Arc<DebugAdapterClient>>,
}

impl LocalDebugSession {
    pub fn configuration(&self) -> &DebugAdapterConfig {
        &self.configuration
    }

    pub fn update_configuration(
        &mut self,
        f: impl FnOnce(&mut DebugAdapterConfig),
        cx: &mut Context<DebugSession>,
    ) {
        f(&mut self.configuration);
        cx.notify();
    }

    pub fn add_client(&mut self, client: Arc<DebugAdapterClient>, cx: &mut Context<DebugSession>) {
        self.clients.insert(client.id(), client);
        cx.notify();
    }

    pub fn remove_client(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut Context<DebugSession>,
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

    pub fn client_ids(&self) -> impl Iterator<Item = DebugAdapterClientId> + '_ {
        self.clients.keys().cloned()
    }
}

pub struct RemoteDebugSession {
    label: String,
}

impl DebugSession {
    pub fn new_local(id: DebugSessionId, configuration: DebugAdapterConfig) -> Self {
        Self {
            id,
            ignore_breakpoints: false,
            mode: DebugSessionMode::Local(LocalDebugSession {
                configuration,
                clients: HashMap::default(),
            }),
        }
    }

    pub fn as_local(&self) -> Option<&LocalDebugSession> {
        match &self.mode {
            DebugSessionMode::Local(local) => Some(local),
            _ => None,
        }
    }

    pub fn as_local_mut(&mut self) -> Option<&mut LocalDebugSession> {
        match &mut self.mode {
            DebugSessionMode::Local(local) => Some(local),
            _ => None,
        }
    }

    pub fn new_remote(id: DebugSessionId, label: String, ignore_breakpoints: bool) -> Self {
        Self {
            id,
            ignore_breakpoints,
            mode: DebugSessionMode::Remote(RemoteDebugSession { label }),
        }
    }

    pub fn id(&self) -> DebugSessionId {
        self.id
    }

    pub fn name(&self) -> String {
        match &self.mode {
            DebugSessionMode::Local(local) => local.configuration.label.clone(),
            DebugSessionMode::Remote(remote) => remote.label.clone(),
        }
    }

    pub fn ignore_breakpoints(&self) -> bool {
        self.ignore_breakpoints
    }

    pub fn set_ignore_breakpoints(&mut self, ignore: bool, cx: &mut Context<Self>) {
        self.ignore_breakpoints = ignore;
        cx.notify();
    }
}
