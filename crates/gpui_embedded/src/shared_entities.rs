//! Host-side shared entities: homes with dynamic dispatch tables keyed by method name, and
//! projections of guest-homed entities. See the "Shared entities" section of
//! `wit/plugin.wit`.

use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use anyhow::{Context as _, Result, anyhow};
use futures::channel::oneshot;
use gpui::{App, Context, Entity, Subscription, WeakEntity};
use gpui_embedded_shared::{SharedMessage, SharedSpec, decode};

use crate::bindings;

/// A host entity type that can serve as the home of a shared entity of kind `S`.
pub trait SharedEntitySource<S: SharedSpec>: 'static {
    fn snapshot(&self, cx: &App) -> S::Snapshot;
}

/// A handler for one typed message, implemented by the home entity.
pub trait HandleShared<M: SharedMessage>: 'static + Sized {
    fn handle(&mut self, message: M, cx: &mut Context<Self>);
}

type MethodHandler = Rc<dyn Fn(&[u8], &mut App) -> Result<()>>;
type SnapshotFn = Rc<dyn Fn(&App) -> Result<Vec<u8>>>;
type ApplySnapshot = Rc<dyn Fn(&[u8], &mut App) -> Result<()>>;

pub(crate) struct HostSharedEntity {
    name: String,
    type_name: &'static str,
    methods: HashMap<&'static str, MethodHandler>,
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
    pub method: String,
    pub payload: Vec<u8>,
}

pub(crate) struct HostProjection {
    type_name: &'static str,
    pub entity_id: Option<u64>,
    pub apply_snapshot: ApplySnapshot,
    pub next_sequence: u64,
    pub pending_sends: Vec<PendingSend>,
    pub pending_acks: Vec<(u64, oneshot::Sender<()>)>,
}

#[derive(Default)]
pub(crate) struct HostShared {
    next_entity_id: u64,
    homes: HashMap<u64, HostSharedEntity>,
    pub projections_by_name: HashMap<String, HostProjection>,
    pub projection_names_by_id: HashMap<u64, String>,
    /// Guest announcements that arrived before the host attached a projection.
    pub unclaimed_announcements: HashMap<String, bindings::SharedEntityAnnouncement>,
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

    pub fn dispatch(
        &mut self,
        entity_id: u64,
        sequence: u64,
        method: &str,
        payload: &[u8],
        cx: &mut App,
    ) -> Result<()> {
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

/// Typed registration of dynamically dispatched methods, built during
/// [`crate::PluginHost::share`].
pub struct Methods<S: SharedSpec, T> {
    entity: WeakEntity<T>,
    map: HashMap<&'static str, MethodHandler>,
    _spec: PhantomData<S>,
}

impl<S: SharedSpec, T: 'static> Methods<S, T> {
    pub(crate) fn new(entity: WeakEntity<T>) -> Self {
        Self {
            entity,
            map: HashMap::new(),
            _spec: PhantomData,
        }
    }

    /// Register the handler for message type `M`. The wire stays dynamic — this inserts a
    /// decode-and-call closure under `M::METHOD`.
    pub fn on<M>(&mut self) -> &mut Self
    where
        M: SharedMessage<Spec = S>,
        T: HandleShared<M>,
    {
        let entity = self.entity.clone();
        self.map.insert(
            M::METHOD,
            Rc::new(move |payload, cx| {
                let message: M = decode(payload).context("decoding shared message")?;
                entity.update(cx, |entity, cx| entity.handle(message, cx))
            }),
        );
        self
    }

    /// The dynamic escape hatch: register a raw handler for an arbitrary method name.
    pub fn on_raw(
        &mut self,
        method: &'static str,
        handler: impl Fn(&Entity<T>, &[u8], &mut App) -> Result<()> + 'static,
    ) -> &mut Self {
        let entity = self.entity.clone();
        self.map.insert(
            method,
            Rc::new(move |payload, cx| {
                let entity = entity.upgrade().context("shared entity dropped")?;
                handler(&entity, payload, cx)
            }),
        );
        self
    }

    pub(crate) fn into_map(self) -> HashMap<&'static str, MethodHandler> {
        self.map
    }
}
