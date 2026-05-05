//! Client-side state machine for the port-forwarding observation pipeline.
//!
//! `PortResourceSet` is a pure reducer driven by events pushed from the remote
//! server.  `PortStore` wraps it in a GPUI entity and wires it to the RPC
//! session.  The observer and forwarding logic live in `remote_server`.

use collections::HashMap;
use gpui::{Entity, EventEmitter};
use rpc::{AnyProtoClient, TypedEnvelope, proto};
use std::sync::Arc;
use thiserror::Error;

// ─── Data types ────────────────────────────────────────────────────────────────

/// a single listening TCP socket reported by the remote host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortResource {
    /// stable id of the form `"tcp4:127.0.0.1:3000"`.
    pub id: Arc<str>,
    /// monotonically increasing version assigned by the server.
    pub version: u64,
    /// `"tcp4"` or `"tcp6"`.
    pub proto: Arc<str>,
    /// bind address as a string.
    pub bind_addr: Arc<str>,
    /// port number.
    pub port: u32,
    /// uid of the owning process (0 if unknown).
    pub uid: u32,
    /// inode of the socket.
    pub inode: u64,
    /// process name (always empty in PR 1).
    pub process: Arc<str>,
    /// `"loopback"`, `"wildcard"`, or `"specific"`.
    pub exposure: Arc<str>,
}

impl From<proto::PortResource> for PortResource {
    fn from(r: proto::PortResource) -> Self {
        Self {
            id: r.id.into(),
            version: r.version,
            proto: r.proto.into(),
            bind_addr: r.bind_addr.into(),
            port: r.port,
            uid: r.uid,
            inode: r.inode,
            process: r.process.into(),
            exposure: r.exposure.into(),
        }
    }
}

impl From<PortResource> for proto::PortResource {
    fn from(r: PortResource) -> Self {
        Self {
            id: r.id.to_string(),
            version: r.version,
            proto: r.proto.to_string(),
            bind_addr: r.bind_addr.to_string(),
            port: r.port,
            uid: r.uid,
            inode: r.inode,
            process: r.process.to_string(),
            exposure: r.exposure.to_string(),
        }
    }
}

// ─── Reducer ───────────────────────────────────────────────────────────────────

/// error returned when an event cannot be applied to the current state.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApplyError {
    #[error("version {incoming} is not greater than current {current}")]
    OutOfOrder { incoming: u64, current: u64 },
    #[error("duplicate event at version {0}")]
    Duplicate(u64),
}

/// pure state machine that tracks the current set of listening ports.
///
/// All mutations go through the `apply_*` methods, which enforce version
/// monotonicity.  The state is reset by `apply_resync_required`.
#[derive(Debug, Default, Clone)]
pub struct PortResourceSet {
    resources: HashMap<Arc<str>, PortResource>,
    version: u64,
}

impl PortResourceSet {
    /// replace the full resource set; must start from version 0 or after a resync.
    pub fn apply_initial(
        &mut self,
        resources: Vec<PortResource>,
        version: u64,
    ) -> Result<(), ApplyError> {
        if version == self.version && !self.resources.is_empty() {
            return Err(ApplyError::Duplicate(version));
        }
        if version < self.version {
            return Err(ApplyError::OutOfOrder {
                incoming: version,
                current: self.version,
            });
        }
        self.resources.clear();
        for resource in resources {
            self.resources.insert(resource.id.clone(), resource);
        }
        self.version = version;
        Ok(())
    }

    /// apply an incremental delta.
    pub fn apply_delta(
        &mut self,
        upserted: Vec<PortResource>,
        removed: Vec<Arc<str>>,
        version: u64,
    ) -> Result<(), ApplyError> {
        if version == self.version {
            return Err(ApplyError::Duplicate(version));
        }
        if version < self.version {
            return Err(ApplyError::OutOfOrder {
                incoming: version,
                current: self.version,
            });
        }
        for resource in upserted {
            self.resources.insert(resource.id.clone(), resource);
        }
        for id in removed {
            self.resources.remove(&id);
        }
        self.version = version;
        Ok(())
    }

    /// advance version without changing resources (server heartbeat).
    pub fn apply_bookmark(&mut self, version: u64) -> Result<(), ApplyError> {
        if version == self.version {
            return Err(ApplyError::Duplicate(version));
        }
        if version < self.version {
            return Err(ApplyError::OutOfOrder {
                incoming: version,
                current: self.version,
            });
        }
        self.version = version;
        Ok(())
    }

    /// clear all state; called when the server signals a resync is needed.
    pub fn apply_resync_required(&mut self) {
        self.resources.clear();
        self.version = 0;
    }

    /// current resource map.
    pub fn resources(&self) -> &HashMap<Arc<str>, PortResource> {
        &self.resources
    }

    /// current version.
    pub fn version(&self) -> u64 {
        self.version
    }
}

// ─── GPUI entity ───────────────────────────────────────────────────────────────

/// events emitted by `PortStore`.
#[derive(Debug, Clone)]
pub enum PortStoreEvent {
    /// the full resource set was replaced.
    Reset,
    /// an incremental update was applied.
    Updated,
}

/// GPUI entity that holds `PortResourceSet` and drives it via RPC messages.
pub struct PortStore {
    set: PortResourceSet,
}

impl EventEmitter<PortStoreEvent> for PortStore {}

impl PortStore {
    /// create a new store (used on both local and remote sides).
    pub fn new() -> Self {
        Self {
            set: PortResourceSet::default(),
        }
    }

    /// register entity message handlers on `client`.
    ///
    /// The caller must also call `session.subscribe_to_entity(project_id, &port_store)`
    /// so that incoming messages are routed to this entity.
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_message_handler(Self::handle_port_collection_initial);
        client.add_entity_message_handler(Self::handle_port_collection_delta);
        client.add_entity_message_handler(Self::handle_port_collection_bookmark);
        client.add_entity_message_handler(Self::handle_port_resync_required);
    }

    /// current resource snapshot.
    pub fn resources(&self) -> &HashMap<Arc<str>, PortResource> {
        self.set.resources()
    }

    async fn handle_port_collection_initial(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::PortCollectionInitial>,
        mut cx: gpui::AsyncApp,
    ) -> anyhow::Result<()> {
        let msg = envelope.payload;
        let resources: Vec<PortResource> = msg
            .resources
            .into_iter()
            .map(PortResource::from)
            .collect();
        this.update(&mut cx, |store, cx| {
            store
                .set
                .apply_initial(resources, msg.version)
                .unwrap_or_else(|err| log::warn!("port initial rejected: {err}"));
            cx.emit(PortStoreEvent::Reset);
            cx.notify();
        });
        Ok(())
    }

    async fn handle_port_collection_delta(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::PortCollectionDelta>,
        mut cx: gpui::AsyncApp,
    ) -> anyhow::Result<()> {
        let msg = envelope.payload;
        let upserted: Vec<PortResource> = msg
            .upserted
            .into_iter()
            .map(PortResource::from)
            .collect();
        let removed: Vec<Arc<str>> = msg.removed.into_iter().map(|s| s.into()).collect();
        this.update(&mut cx, |store, cx| {
            store
                .set
                .apply_delta(upserted, removed, msg.version)
                .unwrap_or_else(|err| log::warn!("port delta rejected: {err}"));
            cx.emit(PortStoreEvent::Updated);
            cx.notify();
        });
        Ok(())
    }

    async fn handle_port_collection_bookmark(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::PortCollectionBookmark>,
        mut cx: gpui::AsyncApp,
    ) -> anyhow::Result<()> {
        let msg = envelope.payload;
        this.update(&mut cx, |store, _cx| {
            store
                .set
                .apply_bookmark(msg.version)
                .unwrap_or_else(|err| log::warn!("port bookmark rejected: {err}"));
        });
        Ok(())
    }

    async fn handle_port_resync_required(
        this: Entity<Self>,
        _envelope: TypedEnvelope<proto::PortResyncRequired>,
        mut cx: gpui::AsyncApp,
    ) -> anyhow::Result<()> {
        this.update(&mut cx, |store, cx| {
            store.set.apply_resync_required();
            cx.emit(PortStoreEvent::Reset);
            cx.notify();
        });
        Ok(())
    }
}

impl Default for PortStore {
    fn default() -> Self {
        Self::new()
    }
}


