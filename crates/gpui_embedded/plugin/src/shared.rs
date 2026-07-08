//! Guest-side shared entities: projections of host-homed entities, and homes for
//! guest-owned entities projected into the host. See the "Shared entities" section of
//! `crates/gpui_embedded/wit/plugin.wit` and DESIGN.md.

use crate::wit;
use anyhow::{Context as _, Result, anyhow};
use gpui::{AnyEntity, App, AppContext as _, AsyncApp, Entity};
use gpui_embedded_shared::{
    AckSender, MethodHandler, RELEASE_METHOD, ResponseSender, SUBSCRIBE_METHOD, SharedMessage,
    SharedSpec, decode, encode,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub use gpui_embedded_shared::{
    CallReceipt, HandleShared, Methods, SendReceipt, SharedEntitySource, SharedProjection,
    SharedRef,
};

/// Guest-homed entity ids have the high bit set so they can never collide with host-minted
/// ids.
const GUEST_HOME_BIT: u64 = 1 << 63;

type ApplySnapshot = Rc<dyn Fn(&[u8], &mut AsyncApp) -> Result<()>>;
type SnapshotFn = Rc<dyn Fn(&App) -> Result<Vec<u8>>>;

struct PendingSend {
    sequence: u64,
    request_id: Option<u64>,
    method: String,
    payload: Vec<u8>,
}

struct ProjectionEntry {
    type_name: &'static str,
    entity_id: Option<u64>,
    apply_snapshot: ApplySnapshot,
    replica: AnyEntity,
    next_sequence: u64,
    /// Messages sent before the home side's announcement arrived; flushed in order, which is
    /// what makes sends to a not-yet-resolved entity pipeline correctly.
    pending_sends: Vec<PendingSend>,
    pending_acks: Vec<(u64, AckSender)>,
}

struct HomeEntry {
    methods: HashMap<String, MethodHandler>,
    snapshot_fn: SnapshotFn,
    applied_sequence: u64,
    published_ack: u64,
    /// Whether the other side holds a live projection; snapshots only flow when true.
    subscribed: bool,
    /// Anonymous shares keep their entity alive until released; named shares borrow.
    strong: Option<AnyEntity>,
}

#[derive(Default)]
struct Registry {
    projections_by_name: HashMap<String, ProjectionEntry>,
    names_by_entity_id: HashMap<u64, String>,
    homes: HashMap<u64, HomeEntry>,
    next_home_id: u64,
    next_request_id: u64,
    pending_responses: HashMap<u64, ResponseSender>,
}

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

/// A typed handle to an entity homed on the other side of the boundary.
pub struct Remote<S: SharedSpec> {
    name: String,
    replica: Entity<SharedProjection<S::Snapshot>>,
}

impl<S: SharedSpec> Clone for Remote<S> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            replica: self.replica.clone(),
        }
    }
}

/// Attach to the shared entity bound to `name` on the host. Returns immediately; the
/// replica fills in when the host's announcement and first snapshot arrive.
pub fn remote<S: SharedSpec>(name: impl Into<String>, cx: &mut App) -> Remote<S> {
    let name = name.into();
    let replica = cx.new(|_| SharedProjection::<S::Snapshot> { state: None });
    let apply_snapshot: ApplySnapshot = {
        let replica = replica.downgrade();
        Rc::new(move |bytes: &[u8], cx: &mut AsyncApp| {
            let snapshot: S::Snapshot = decode(bytes).context("decoding shared snapshot")?;
            replica.update(cx, |projection, cx| {
                projection.state = Some(snapshot);
                cx.notify();
            })
        })
    };
    REGISTRY.with(|registry| {
        registry.borrow_mut().projections_by_name.insert(
            name.clone(),
            ProjectionEntry {
                type_name: S::TYPE_NAME,
                entity_id: None,
                apply_snapshot,
                replica: replica.clone().into_any(),
                next_sequence: 0,
                pending_sends: Vec::new(),
                pending_acks: Vec::new(),
            },
        );
    });
    Remote { name, replica }
}

/// Attach to a shared entity through a capability reference received in a payload. No name
/// is involved: the ref's id addresses the entity directly, and a `$subscribe` control
/// message starts the snapshot flow (its ack is the initial snapshot). Materializing the
/// same ref twice returns the same replica.
pub fn remote_from_ref<S: SharedSpec>(reference: SharedRef<S>, cx: &mut App) -> Remote<S> {
    let entity_id = reference.entity_id();
    let name = format!("#{entity_id}");

    let existing = REGISTRY.with(|registry| {
        let registry = registry.borrow();
        registry
            .projections_by_name
            .get(&name)
            .map(|entry| entry.replica.clone())
    });
    if let Some(replica) = existing {
        match replica.downcast::<SharedProjection<S::Snapshot>>() {
            Ok(replica) => return Remote { name, replica },
            Err(_) => log::error!(
                "gpui_plugin: ref {entity_id} materialized twice with different specs"
            ),
        }
    }

    let replica = cx.new(|_| SharedProjection::<S::Snapshot> { state: None });
    let apply_snapshot: ApplySnapshot = {
        let replica = replica.downgrade();
        Rc::new(move |bytes: &[u8], cx: &mut AsyncApp| {
            let snapshot: S::Snapshot = decode(bytes).context("decoding shared snapshot")?;
            replica.update(cx, |projection, cx| {
                projection.state = Some(snapshot);
                cx.notify();
            })
        })
    };
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        registry.projections_by_name.insert(
            name.clone(),
            ProjectionEntry {
                type_name: S::TYPE_NAME,
                entity_id: Some(entity_id),
                apply_snapshot,
                replica: replica.clone().into_any(),
                next_sequence: 0,
                pending_sends: Vec::new(),
                pending_acks: Vec::new(),
            },
        );
        registry.names_by_entity_id.insert(entity_id, name.clone());
    });
    dispatch_outgoing(&name, SUBSCRIBE_METHOD, Vec::new(), None, None);
    Remote { name, replica }
}

impl<S: SharedSpec> Remote<S> {
    /// The local replica entity, for `cx.observe` and reads.
    pub fn replica(&self) -> &Entity<SharedProjection<S::Snapshot>> {
        &self.replica
    }

    /// Send a typed message to the entity's home side. Await the receipt for
    /// read-your-writes; drop it for fire-and-forget.
    pub fn send<M: SharedMessage<Spec = S>>(&self, message: M) -> SendReceipt {
        match encode(&message) {
            Ok(payload) => send_raw(&self.name, M::METHOD, payload),
            Err(error) => {
                log::error!(
                    "gpui_plugin: failed to encode {}::{}: {error:#}",
                    S::TYPE_NAME,
                    M::METHOD
                );
                SendReceipt::dropped()
            }
        }
    }

    /// Relinquish this projection. Anonymous homes drop their strong handle to the entity;
    /// snapshots stop flowing.
    pub fn release(self) {
        let _receipt = send_raw(&self.name, RELEASE_METHOD, Vec::new());
        REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            if let Some(entry) = registry.projections_by_name.remove(&self.name)
                && let Some(entity_id) = entry.entity_id
            {
                registry.names_by_entity_id.remove(&entity_id);
            }
        });
    }

    /// Call a typed method on the entity's home side, resolving with its return value. The
    /// response is delivered after the snapshot acking this call, so the replica already
    /// reflects the mutation when the receipt resolves.
    pub fn call<M: SharedMessage<Spec = S>>(&self, message: M) -> CallReceipt<M::Response> {
        match encode(&message) {
            Ok(payload) => call_raw(&self.name, M::METHOD, payload),
            Err(error) => {
                log::error!(
                    "gpui_plugin: failed to encode {}::{}: {error:#}",
                    S::TYPE_NAME,
                    M::METHOD
                );
                CallReceipt::dropped()
            }
        }
    }
}

/// The dynamic escape hatch: send an arbitrary method and payload to a shared entity by
/// name. The typed [`Remote::send`] is sugar over exactly this.
pub fn send_raw(name: &str, method: &str, payload: Vec<u8>) -> SendReceipt {
    let (ack_sender, receipt) = SendReceipt::channel();
    dispatch_outgoing(name, method, payload, Some(ack_sender), None);
    receipt
}

/// The dynamic call escape hatch; the typed [`Remote::call`] is sugar over this.
pub fn call_raw<R: gpui_embedded_shared::serde::de::DeserializeOwned>(
    name: &str,
    method: &str,
    payload: Vec<u8>,
) -> CallReceipt<R> {
    let (response_sender, receipt) = CallReceipt::channel();
    dispatch_outgoing(name, method, payload, None, Some(response_sender));
    receipt
}

fn dispatch_outgoing(
    name: &str,
    method: &str,
    payload: Vec<u8>,
    ack: Option<AckSender>,
    response: Option<ResponseSender>,
) {
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let request_id = response.map(|sender| {
            registry.next_request_id += 1;
            let request_id = registry.next_request_id;
            registry.pending_responses.insert(request_id, sender);
            request_id
        });
        let Some(entry) = registry.projections_by_name.get_mut(name) else {
            log::warn!("gpui_plugin: send to unknown shared entity {name:?}");
            return;
        };
        entry.next_sequence += 1;
        let sequence = entry.next_sequence;
        if let Some(ack) = ack {
            entry.pending_acks.push((sequence, ack));
        }
        if let Some(entity_id) = entry.entity_id {
            wit::send_shared_message(&wit::SharedMessage {
                entity_id,
                sequence,
                request_id,
                method: method.to_string(),
                payload,
            });
        } else {
            entry.pending_sends.push(PendingSend {
                sequence,
                request_id,
                method: method.to_string(),
                payload,
            });
        }
    });
}

pub(crate) fn response_delivered(response: wit::SharedResponse) {
    let sender = REGISTRY.with(|registry| {
        registry
            .borrow_mut()
            .pending_responses
            .remove(&response.request_id)
    });
    let Some(sender) = sender else {
        log::warn!(
            "gpui_plugin: response for unknown request {}",
            response.request_id
        );
        return;
    };
    sender.send(response.outcome).ok();
}

/// Share a guest entity with the host under a well-known name. The guest becomes the home:
/// host messages dispatch to the handlers registered in `register`, and every `cx.notify`
/// publishes a snapshot to the host's projections.
pub fn share<S, T>(
    entity: &Entity<T>,
    name: impl Into<String>,
    register: impl FnOnce(&mut Methods<S, T>),
    cx: &mut App,
) where
    S: SharedSpec,
    T: SharedEntitySource<S>,
{
    let name = name.into();
    let mut methods = Methods::new(entity.downgrade());
    register(&mut methods);

    let snapshot_fn: SnapshotFn = {
        let entity = entity.downgrade();
        Rc::new(move |cx: &App| {
            let entity = entity.upgrade().context("shared entity dropped")?;
            encode(&entity.read(cx).snapshot(cx))
        })
    };

    let entity_id = insert_home(methods.into_map(), snapshot_fn, true, None);

    cx.observe(entity, move |_, cx| publish_home(entity_id, cx))
        .detach();

    wit::announce_shared_entity(&wit::SharedEntityAnnouncement {
        entity_id,
        type_name: S::TYPE_NAME.to_string(),
        name,
    });
    publish_home(entity_id, cx);
}

/// Share a guest entity anonymously, returning a capability reference to embed in snapshot
/// or message payloads. The home holds a strong handle to the entity until the reference is
/// released; snapshots start flowing when a projection subscribes.
pub fn share_anonymous<S, T>(
    entity: &Entity<T>,
    register: impl FnOnce(&mut Methods<S, T>),
    cx: &mut App,
) -> SharedRef<S>
where
    S: SharedSpec,
    T: SharedEntitySource<S>,
{
    let mut methods = Methods::new(entity.downgrade());
    register(&mut methods);

    let snapshot_fn: SnapshotFn = {
        let entity = entity.downgrade();
        Rc::new(move |cx: &App| {
            let entity = entity.upgrade().context("shared entity dropped")?;
            encode(&entity.read(cx).snapshot(cx))
        })
    };

    let entity_id = insert_home(
        methods.into_map(),
        snapshot_fn,
        false,
        Some(entity.clone().into_any()),
    );
    cx.observe(entity, move |_, cx| publish_home(entity_id, cx))
        .detach();
    SharedRef::from_raw(entity_id)
}

fn insert_home(
    methods: HashMap<String, MethodHandler>,
    snapshot_fn: SnapshotFn,
    subscribed: bool,
    strong: Option<AnyEntity>,
) -> u64 {
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        registry.next_home_id += 1;
        let entity_id = GUEST_HOME_BIT | registry.next_home_id;
        registry.homes.insert(
            entity_id,
            HomeEntry {
                methods,
                snapshot_fn,
                applied_sequence: 0,
                published_ack: 0,
                subscribed,
                strong,
            },
        );
        entity_id
    })
}

fn publish_home(entity_id: u64, cx: &mut App) {
    let publish = REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let home = registry.homes.get_mut(&entity_id)?;
        if !home.subscribed {
            return None;
        }
        home.published_ack = home.applied_sequence;
        Some((home.snapshot_fn.clone(), home.applied_sequence))
    });
    let Some((snapshot_fn, acked_sequence)) = publish else {
        return;
    };
    match snapshot_fn(cx) {
        Ok(payload) => wit::publish_shared_snapshot(&wit::SharedSnapshot {
            entity_id,
            acked_sequence,
            payload,
        }),
        Err(error) => log::error!("gpui_plugin: failed to snapshot shared entity: {error:#}"),
    }
}

pub(crate) fn entity_announced(announcement: wit::SharedEntityAnnouncement) {
    let flushed = REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(entry) = registry.projections_by_name.get_mut(&announcement.name) else {
            log::info!(
                "gpui_plugin: no local projection for shared entity {:?} ({})",
                announcement.name,
                announcement.type_name
            );
            return Vec::new();
        };
        if entry.type_name != announcement.type_name {
            log::error!(
                "gpui_plugin: shared entity {:?} is a {} on the host but bound as {} here",
                announcement.name,
                announcement.type_name,
                entry.type_name
            );
            return Vec::new();
        }
        entry.entity_id = Some(announcement.entity_id);
        registry
            .names_by_entity_id
            .insert(announcement.entity_id, announcement.name.clone());
        let entry = registry
            .projections_by_name
            .get_mut(&announcement.name)
            .expect("inserted above");
        std::mem::take(&mut entry.pending_sends)
            .into_iter()
            .map(|send| (announcement.entity_id, send))
            .collect::<Vec<_>>()
    });
    for (entity_id, send) in flushed {
        wit::send_shared_message(&wit::SharedMessage {
            entity_id,
            sequence: send.sequence,
            request_id: send.request_id,
            method: send.method,
            payload: send.payload,
        });
    }
}

pub(crate) fn snapshot_delivered(snapshot: wit::SharedSnapshot, cx: &mut AsyncApp) {
    // Clone the applier out so the registry borrow is released before user code (observers
    // of the replica) runs; observers may re-enter this module via `send_raw`.
    let apply_snapshot = REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let name = registry.names_by_entity_id.get(&snapshot.entity_id)?;
        registry
            .projections_by_name
            .get(name)
            .map(|entry| entry.apply_snapshot.clone())
    });
    let result = match apply_snapshot {
        Some(apply_snapshot) => apply_snapshot(&snapshot.payload, cx),
        None => Err(anyhow!("snapshot for unknown entity {}", snapshot.entity_id)),
    };
    if let Err(error) = result {
        log::error!("gpui_plugin: failed to apply shared snapshot: {error:#}");
        return;
    }

    // The replica now includes everything through `acked_sequence`; resolving receipts after
    // the update above is what makes awaiting a send read-your-writes.
    let acked = REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let name = registry.names_by_entity_id.get(&snapshot.entity_id)?.clone();
        let entry = registry.projections_by_name.get_mut(&name)?;
        let mut acked = Vec::new();
        entry.pending_acks.retain_mut(|(sequence, sender)| {
            if *sequence <= snapshot.acked_sequence {
                let (drained_sender, _drained_receiver) = futures::channel::oneshot::channel();
                acked.push(std::mem::replace(sender, drained_sender));
                false
            } else {
                true
            }
        });
        Some(acked)
    });
    for sender in acked.into_iter().flatten() {
        sender.send(()).ok();
    }
}

pub(crate) fn message_delivered(message: wit::SharedMessage, cx: &mut AsyncApp) {
    enum Dispatch {
        Handler(MethodHandler),
        Control,
        Unknown,
    }
    let dispatch = REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(home) = registry.homes.get_mut(&message.entity_id) else {
            return Dispatch::Unknown;
        };
        home.applied_sequence = home.applied_sequence.max(message.sequence);
        match message.method.as_str() {
            SUBSCRIBE_METHOD => {
                home.subscribed = true;
                Dispatch::Control
            }
            RELEASE_METHOD => {
                home.subscribed = false;
                home.strong = None;
                Dispatch::Control
            }
            _ => home
                .methods
                .get(&message.method)
                .or_else(|| home.methods.get(gpui_embedded_shared::WILDCARD_METHOD))
                .cloned()
                .map(Dispatch::Handler)
                .unwrap_or(Dispatch::Unknown),
        }
    });
    let outcome = match dispatch {
        Dispatch::Handler(handler) => cx
            .update(|cx| handler(&message.method, &message.payload, cx))
            .map_err(|error| format!("{error:#}")),
        Dispatch::Control => encode(&()).map_err(|error| format!("{error:#}")),
        Dispatch::Unknown => Err(format!(
            "no handler for shared method {:?} on entity {}",
            message.method, message.entity_id
        )),
    };
    if let Err(error) = &outcome {
        log::error!("gpui_plugin: shared message failed: {error}");
    }

    // The handler's notify usually published already (observers run in its update cycle);
    // this covers handlers that don't notify, so the sender's receipt still resolves. It
    // must happen BEFORE the response goes back: responses may only arrive after the
    // snapshot acking their call, which is what makes calls read-your-writes.
    let needs_ack = REGISTRY.with(|registry| {
        registry
            .borrow()
            .homes
            .get(&message.entity_id)
            .is_some_and(|home| home.published_ack < home.applied_sequence)
    });
    if needs_ack {
        let entity_id = message.entity_id;
        cx.update(|cx| publish_home(entity_id, cx));
    }

    if let Some(request_id) = message.request_id {
        wit::send_shared_response(&wit::SharedResponse {
            request_id,
            outcome,
        });
    }
}
