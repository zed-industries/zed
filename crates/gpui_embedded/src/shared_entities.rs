//! Host-side shared entities: homes with dynamic dispatch tables keyed by method name, and
//! projections of guest-homed entities. See the "Shared entities" section of
//! `wit/plugin.wit`.

use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{Context as _, Result, anyhow};
use gpui::{App, Subscription};
use gpui_embedded_shared::{AckSender, MethodHandler, Methods, ResponseSender, SharedSpec};

use crate::bindings;

type SnapshotFn = Rc<dyn Fn(&App) -> Result<Vec<u8>>>;
type ApplySnapshot = Rc<dyn Fn(&[u8], &mut App) -> Result<()>>;

pub(crate) struct HostSharedEntity {
    name: String,
    type_name: &'static str,
    methods: HashMap<String, MethodHandler>,
    pub snapshot_fn: SnapshotFn,
    pub applied_sequence: u64,
    pub published_ack: u64,
    _observation: Subscription,
}

impl HostSharedEntity {
    pub fn new<S: SharedSpec, T: 'static>(
        name: String,
        type_name: &'static str,
        methods: Methods<S, T>,
        snapshot_fn: SnapshotFn,
        observation: Subscription,
    ) -> Self {
        Self {
            name,
            type_name,
            methods: methods.into_map(),
            snapshot_fn,
            applied_sequence: 0,
            published_ack: 0,
            _observation: observation,
        }
    }
}

pub(crate) struct PendingSend {
    pub sequence: u64,
    pub request_id: Option<u64>,
    pub method: String,
    pub payload: Vec<u8>,
}

pub(crate) struct HostProjection {
    type_name: &'static str,
    pub entity_id: Option<u64>,
    pub apply_snapshot: ApplySnapshot,
    pub next_sequence: u64,
    pub pending_sends: Vec<PendingSend>,
    pub pending_acks: Vec<(u64, AckSender)>,
}

#[derive(Default)]
pub(crate) struct HostShared {
    next_entity_id: u64,
    homes: HashMap<u64, HostSharedEntity>,
    pub projections_by_name: HashMap<String, HostProjection>,
    pub projection_names_by_id: HashMap<u64, String>,
    /// Guest announcements that arrived before the host attached a projection.
    pub unclaimed_announcements: HashMap<String, bindings::SharedEntityAnnouncement>,
    pub next_request_id: u64,
    pub pending_responses: HashMap<u64, ResponseSender>,
}

impl HostShared {
    /// Mint an entity id before the entity record exists, so snapshot-publishing
    /// subscriptions can capture it.
    pub fn insert_placeholder(&mut self) -> u64 {
        self.next_entity_id += 1;
        self.next_entity_id
    }

    pub fn fill_placeholder(&mut self, entity_id: u64, entity: HostSharedEntity) {
        self.homes.insert(entity_id, entity);
    }

    pub fn home_mut(&mut self, entity_id: u64) -> Option<&mut HostSharedEntity> {
        self.homes.get_mut(&entity_id)
    }

    pub fn insert_projection<S: SharedSpec>(&mut self, name: String, apply: ApplySnapshot) {
        self.projections_by_name.insert(
            name,
            HostProjection {
                type_name: S::TYPE_NAME,
                entity_id: None,
                apply_snapshot: apply,
                next_sequence: 0,
                pending_sends: Vec::new(),
                pending_acks: Vec::new(),
            },
        );
    }

    /// Bind a guest announcement to a waiting projection. Returns the sends queued while
    /// unresolved, in order, ready to be pipelined to the guest.
    pub fn bind_projection(
        &mut self,
        announcement: &bindings::SharedEntityAnnouncement,
    ) -> Option<Vec<PendingSend>> {
        let Some(projection) = self.projections_by_name.get_mut(&announcement.name) else {
            self.unclaimed_announcements
                .insert(announcement.name.clone(), announcement.clone());
            return None;
        };
        if projection.type_name != announcement.type_name {
            log::error!(
                "gpui_embedded: shared entity {:?} is a {} in the guest but bound as {} here",
                announcement.name,
                announcement.type_name,
                projection.type_name
            );
            return None;
        }
        projection.entity_id = Some(announcement.entity_id);
        self.projection_names_by_id
            .insert(announcement.entity_id, announcement.name.clone());
        let projection = self
            .projections_by_name
            .get_mut(&announcement.name)
            .expect("looked up above");
        Some(std::mem::take(&mut projection.pending_sends))
    }

    /// Run the handler and return the encoded response.
    pub fn dispatch(
        &mut self,
        entity_id: u64,
        sequence: u64,
        method: &str,
        payload: &[u8],
        cx: &mut App,
    ) -> Result<Vec<u8>> {
        let home = self
            .homes
            .get_mut(&entity_id)
            .ok_or_else(|| anyhow!("message for unknown shared entity {entity_id}"))?;
        home.applied_sequence = home.applied_sequence.max(sequence);
        let handler = home.methods.get(method).cloned().ok_or_else(|| {
            anyhow!(
                "shared entity {:?} ({}) has no method {method:?}",
                home.name,
                home.type_name
            )
        })?;
        let name = home.name.clone();
        handler(payload, cx)
            .with_context(|| format!("dispatching {method:?} to shared entity {name:?}"))
    }
}
