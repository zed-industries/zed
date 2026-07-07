//! Guest-side shared entities: projections of host-homed entities, and homes for
//! guest-owned entities projected into the host. See the "Shared entities" section of
//! `crates/gpui_embedded/wit/plugin.wit` and DESIGN.md.

use crate::wit;
use anyhow::{Context as _, Result, anyhow};
use futures::channel::oneshot;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, WeakEntity};
use gpui_embedded_shared::{SharedMessage, SharedSpec, decode, encode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::rc::Rc;
use std::task::Poll;

pub use gpui_embedded_shared::SharedProjection;

/// Guest-homed entity ids have the high bit set so they can never collide with host-minted
/// ids.
const GUEST_HOME_BIT: u64 = 1 << 63;

/// A guest entity type that can serve as the home of a shared entity of kind `S`.
pub trait SharedEntitySource<S: SharedSpec>: 'static {
    fn snapshot(&self, cx: &App) -> S::Snapshot;
}

/// A handler for one typed message, implemented by the home entity.
pub trait HandleShared<M: SharedMessage>: 'static + Sized {
    fn handle(&mut self, message: M, cx: &mut Context<Self>);
}

type ApplySnapshot = Rc<dyn Fn(&[u8], &mut AsyncApp) -> Result<()>>;
type MethodHandler = Rc<dyn Fn(&[u8], &mut AsyncApp) -> Result<()>>;
type SnapshotFn = Rc<dyn Fn(&App) -> Result<Vec<u8>>>;

struct PendingSend {
    sequence: u64,
    method: String,
    payload: Vec<u8>,
}

struct ProjectionEntry {
    type_name: &'static str,
    entity_id: Option<u64>,
    apply_snapshot: ApplySnapshot,
    next_sequence: u64,
    /// Messages sent before the home side's announcement arrived; flushed in order, which is
    /// what makes sends to a not-yet-resolved entity pipeline correctly.
    pending_sends: Vec<PendingSend>,
    pending_acks: Vec<(u64, oneshot::Sender<()>)>,
}

struct HomeEntry {
    methods: HashMap<String, MethodHandler>,
    snapshot_fn: SnapshotFn,
    applied_sequence: u64,
    published_ack: u64,
}

#[derive(Default)]
struct Registry {
    projections_by_name: HashMap<String, ProjectionEntry>,
    names_by_entity_id: HashMap<u64, String>,
    homes: HashMap<u64, HomeEntry>,
    next_home_id: u64,
}

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

/// Resolves once the home side has applied the send and the local replica reflects it, so
/// awaiting it gives read-your-writes. Dropping it just means "don't wait"; the message is
/// unaffected.
pub struct SendReceipt(oneshot::Receiver<()>);

impl Future for SendReceipt {
    type Output = Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0)
            .poll(cx)
            .map(|result| result.map_err(|_| anyhow!("shared entity went away before ack")))
    }
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
                next_sequence: 0,
                pending_sends: Vec::new(),
                pending_acks: Vec::new(),
            },
        );
    });
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
                let (_tx, rx) = oneshot::channel();
                SendReceipt(rx)
            }
        }
    }
}

/// The dynamic escape hatch: send an arbitrary method and payload to a shared entity by
/// name. The typed [`Remote::send`] is sugar over exactly this.
pub fn send_raw(name: &str, method: &str, payload: Vec<u8>) -> SendReceipt {
    let (ack_tx, ack_rx) = oneshot::channel();
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(entry) = registry.projections_by_name.get_mut(name) else {
            log::warn!("gpui_plugin: send to unknown shared entity {name:?}");
            return;
        };
        entry.next_sequence += 1;
        let sequence = entry.next_sequence;
        entry.pending_acks.push((sequence, ack_tx));
        if let Some(entity_id) = entry.entity_id {
            wit::send_shared_message(&wit::SharedMessage {
                entity_id,
                sequence,
                method: method.to_string(),
                payload,
            });
        } else {
            entry.pending_sends.push(PendingSend {
                sequence,
                method: method.to_string(),
                payload,
            });
        }
    });
    SendReceipt(ack_rx)
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

    let entity_id = REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        registry.next_home_id += 1;
        let entity_id = GUEST_HOME_BIT | registry.next_home_id;
        registry.homes.insert(
            entity_id,
            HomeEntry {
                methods: methods.map,
                snapshot_fn,
                applied_sequence: 0,
                published_ack: 0,
            },
        );
        entity_id
    });

    cx.observe(entity, move |_, cx| publish_home(entity_id, cx))
        .detach();

    wit::announce_shared_entity(&wit::SharedEntityAnnouncement {
        entity_id,
        type_name: S::TYPE_NAME.to_string(),
        name,
    });
    publish_home(entity_id, cx);
}

/// Typed registration of dynamically dispatched methods for a guest-homed entity.
pub struct Methods<S: SharedSpec, T> {
    entity: WeakEntity<T>,
    map: HashMap<String, MethodHandler>,
    _spec: PhantomData<S>,
}

impl<S: SharedSpec, T: 'static> Methods<S, T> {
    fn new(entity: WeakEntity<T>) -> Self {
        Self {
            entity,
            map: HashMap::new(),
            _spec: PhantomData,
        }
    }

    /// Register the handler for message type `M`.
    pub fn on<M>(&mut self) -> &mut Self
    where
        M: SharedMessage<Spec = S>,
        T: HandleShared<M>,
    {
        let entity = self.entity.clone();
        self.map.insert(
            M::METHOD.to_string(),
            Rc::new(move |payload, cx: &mut AsyncApp| {
                let message: M = decode(payload).context("decoding shared message")?;
                entity.update(cx, |entity, cx| entity.handle(message, cx))
            }),
        );
        self
    }
}

fn publish_home(entity_id: u64, cx: &mut App) {
    let publish = REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let home = registry.homes.get_mut(&entity_id)?;
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
                let (drained_tx, _drained_rx) = oneshot::channel();
                acked.push(std::mem::replace(sender, drained_tx));
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
    let handler = REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let home = registry.homes.get_mut(&message.entity_id)?;
        home.applied_sequence = home.applied_sequence.max(message.sequence);
        home.methods.get(&message.method).cloned()
    });
    let Some(handler) = handler else {
        log::error!(
            "gpui_plugin: no handler for shared method {:?} on entity {}",
            message.method,
            message.entity_id
        );
        return;
    };
    if let Err(error) = handler(&message.payload, cx) {
        log::error!("gpui_plugin: shared message failed: {error:#}");
    }

    // The handler's notify usually published already (observers run in its update cycle);
    // this covers handlers that don't notify, so the sender's receipt still resolves.
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
}
