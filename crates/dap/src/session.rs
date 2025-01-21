use collections::HashMap;
use gpui::ModelContext;
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

pub enum DebugSession {
    Local(LocalDebugSession),
    Remote(RemoteDebugSession),
}

pub struct LocalDebugSession {
    id: DebugSessionId,
    ignore_breakpoints: bool,
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
        cx: &mut ModelContext<DebugSession>,
    ) {
        f(&mut self.configuration);
        cx.notify();
    }

    pub fn add_client(
        &mut self,
        client: Arc<DebugAdapterClient>,
        cx: &mut ModelContext<DebugSession>,
    ) {
        self.clients.insert(client.id(), client);
        cx.notify();
    }

    pub fn remove_client(
        &mut self,
        client_id: &DebugAdapterClientId,
        cx: &mut ModelContext<DebugSession>,
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

    pub fn id(&self) -> DebugSessionId {
        self.id
    }
}

pub struct RemoteDebugSession {
    id: DebugSessionId,
    ignore_breakpoints: bool,
    label: String,
}

impl DebugSession {
    pub fn new_local(id: DebugSessionId, configuration: DebugAdapterConfig) -> Self {
        Self::Local(LocalDebugSession {
            id,
            ignore_breakpoints: false,
            configuration,
            clients: HashMap::default(),
        })
    }

    pub fn as_local(&self) -> Option<&LocalDebugSession> {
        match self {
            DebugSession::Local(local) => Some(local),
            _ => None,
        }
    }

    pub fn as_local_mut(&mut self) -> Option<&mut LocalDebugSession> {
        match self {
            DebugSession::Local(local) => Some(local),
            _ => None,
        }
    }

    pub fn new_remote(id: DebugSessionId, label: String, ignore_breakpoints: bool) -> Self {
        Self::Remote(RemoteDebugSession {
            id,
            label: label.clone(),
            ignore_breakpoints,
        })
    }

    pub fn id(&self) -> DebugSessionId {
        match self {
            DebugSession::Local(local) => local.id,
            DebugSession::Remote(remote) => remote.id,
        }
    }

    pub fn name(&self) -> String {
        match self {
            DebugSession::Local(local) => local.configuration.label.clone(),
            DebugSession::Remote(remote) => remote.label.clone(),
        }
    }

    pub fn ignore_breakpoints(&self) -> bool {
        match self {
            DebugSession::Local(local) => local.ignore_breakpoints,
            DebugSession::Remote(remote) => remote.ignore_breakpoints,
        }
    }

    pub fn set_ignore_breakpoints(&mut self, ignore: bool, cx: &mut ModelContext<Self>) {
        match self {
            DebugSession::Local(local) => local.ignore_breakpoints = ignore,
            DebugSession::Remote(remote) => remote.ignore_breakpoints = ignore,
        }
        cx.notify();
    }
}
