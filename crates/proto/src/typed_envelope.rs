use crate::{Envelope, PeerId};
use anyhow::{Result, anyhow};
use serde::Serialize;
use std::{
    any::{Any, TypeId},
    cmp,
    fmt::{self, Debug},
    path::{Path, PathBuf},
    sync::Arc,
};
use std::{marker::PhantomData, time::Instant};

pub trait EnvelopedMessage: Clone + Debug + Serialize + Sized + Send + Sync + 'static {
    const NAME: &'static str;
    const PRIORITY: MessagePriority;
    fn into_envelope(
        self,
        id: u32,
        responding_to: Option<u32>,
        original_sender_id: Option<PeerId>,
    ) -> Envelope;
    fn from_envelope(envelope: Envelope) -> Option<Self>;
}

pub trait EntityMessage: EnvelopedMessage {
    type Entity;
    fn remote_entity_id(&self) -> u64;
}

pub trait RequestMessage: EnvelopedMessage {
    type Response: EnvelopedMessage;
}

pub trait AnyTypedEnvelope: Any + Send + Sync {
    fn payload_type_id(&self) -> TypeId;
    fn payload_type_name(&self) -> &'static str;
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send + Sync>;
    fn is_background(&self) -> bool;
    fn original_sender_id(&self) -> Option<PeerId>;
    fn sender_id(&self) -> PeerId;
    fn message_id(&self) -> u32;
}

pub enum MessagePriority {
    Foreground,
    Background,
}

impl<T: EnvelopedMessage> AnyTypedEnvelope for TypedEnvelope<T> {
    fn payload_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn payload_type_name(&self) -> &'static str {
        T::NAME
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any + Send + Sync> {
        self
    }

    fn is_background(&self) -> bool {
        matches!(T::PRIORITY, MessagePriority::Background)
    }

    fn original_sender_id(&self) -> Option<PeerId> {
        self.original_sender_id
    }

    fn sender_id(&self) -> PeerId {
        self.sender_id
    }

    fn message_id(&self) -> u32 {
        self.message_id
    }
}

impl PeerId {
    pub fn from_u64(peer_id: u64) -> Self {
        let owner_id = (peer_id >> 32) as u32;
        let id = peer_id as u32;
        Self { owner_id, id }
    }

    pub fn as_u64(self) -> u64 {
        ((self.owner_id as u64) << 32) | (self.id as u64)
    }
}

impl Copy for PeerId {}

impl Eq for PeerId {}

impl Ord for PeerId {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.owner_id
            .cmp(&other.owner_id)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for PeerId {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for PeerId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.owner_id.hash(state);
        self.id.hash(state);
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner_id, self.id)
    }
}

pub trait FromProto {
    fn from_proto(proto: String) -> Self;
}

pub trait ToProto {
    fn to_proto(self) -> String;
}

impl FromProto for PathBuf {
    #[cfg(target_os = "windows")]
    fn from_proto(proto: String) -> Self {
        proto.split("/").collect()
    }

    #[cfg(not(target_os = "windows"))]
    fn from_proto(proto: String) -> Self {
        PathBuf::from(proto)
    }
}

impl FromProto for Arc<Path> {
    fn from_proto(proto: String) -> Self {
        PathBuf::from_proto(proto).into()
    }
}

impl ToProto for PathBuf {
    #[cfg(target_os = "windows")]
    fn to_proto(self) -> String {
        self.components()
            .map(|comp| comp.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("/")
    }

    #[cfg(not(target_os = "windows"))]
    fn to_proto(self) -> String {
        self.to_string_lossy().to_string()
    }
}

impl ToProto for &Path {
    #[cfg(target_os = "windows")]
    fn to_proto(self) -> String {
        self.components()
            .map(|comp| comp.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("/")
    }

    #[cfg(not(target_os = "windows"))]
    fn to_proto(self) -> String {
        self.to_string_lossy().to_string()
    }
}

pub struct Receipt<T> {
    pub sender_id: PeerId,
    pub message_id: u32,
    payload_type: PhantomData<T>,
}

impl<T> Clone for Receipt<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Receipt<T> {}

#[derive(Clone, Debug)]
pub struct TypedEnvelope<T> {
    pub sender_id: PeerId,
    pub original_sender_id: Option<PeerId>,
    pub message_id: u32,
    pub payload: T,
    pub received_at: Instant,
}

impl<T> TypedEnvelope<T> {
    pub fn original_sender_id(&self) -> Result<PeerId> {
        self.original_sender_id
            .ok_or_else(|| anyhow!("missing original_sender_id"))
    }
}

impl<T: RequestMessage> TypedEnvelope<T> {
    pub fn receipt(&self) -> Receipt<T> {
        Receipt {
            sender_id: self.sender_id,
            message_id: self.message_id,
            payload_type: PhantomData,
        }
    }
}
