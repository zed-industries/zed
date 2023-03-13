use super::{entity_messages, messages, request_messages, ConnectionId, TypedEnvelope};
use anyhow::{anyhow, Result};
use async_tungstenite::tungstenite::Message as WebSocketMessage;
use futures::{SinkExt as _, StreamExt as _};
use prost::Message as _;
use serde::Serialize;
use std::any::{Any, TypeId};
use std::fmt;
use std::{
    cmp,
    fmt::Debug,
    io, iter,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

include!(concat!(env!("OUT_DIR"), "/zed.messages.rs"));

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
    fn remote_entity_id(&self) -> u64;
}

pub trait RequestMessage: EnvelopedMessage {
    type Response: EnvelopedMessage;
}

pub trait AnyTypedEnvelope: 'static + Send + Sync {
    fn payload_type_id(&self) -> TypeId;
    fn payload_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send + Sync>;
    fn is_background(&self) -> bool;
    fn original_sender_id(&self) -> Option<PeerId>;
    fn sender_id(&self) -> ConnectionId;
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

    fn as_any(&self) -> &dyn Any {
        self
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

    fn sender_id(&self) -> ConnectionId {
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

messages!(
    (Ack, Foreground),
    (AddProjectCollaborator, Foreground),
    (ApplyCodeAction, Background),
    (ApplyCodeActionResponse, Background),
    (ApplyCompletionAdditionalEdits, Background),
    (ApplyCompletionAdditionalEditsResponse, Background),
    (BufferReloaded, Foreground),
    (BufferSaved, Foreground),
    (Call, Foreground),
    (CallCanceled, Foreground),
    (CancelCall, Foreground),
    (ChannelMessageSent, Foreground),
    (CopyProjectEntry, Foreground),
    (CreateBufferForPeer, Foreground),
    (CreateProjectEntry, Foreground),
    (CreateRoom, Foreground),
    (CreateRoomResponse, Foreground),
    (DeclineCall, Foreground),
    (DeleteProjectEntry, Foreground),
    (Error, Foreground),
    (Follow, Foreground),
    (FollowResponse, Foreground),
    (FormatBuffers, Foreground),
    (FormatBuffersResponse, Foreground),
    (FuzzySearchUsers, Foreground),
    (GetChannelMessages, Foreground),
    (GetChannelMessagesResponse, Foreground),
    (GetChannels, Foreground),
    (GetChannelsResponse, Foreground),
    (GetCodeActions, Background),
    (GetCodeActionsResponse, Background),
    (GetHover, Background),
    (GetHoverResponse, Background),
    (GetCompletions, Background),
    (GetCompletionsResponse, Background),
    (GetDefinition, Background),
    (GetDefinitionResponse, Background),
    (GetTypeDefinition, Background),
    (GetTypeDefinitionResponse, Background),
    (GetDocumentHighlights, Background),
    (GetDocumentHighlightsResponse, Background),
    (GetReferences, Background),
    (GetReferencesResponse, Background),
    (GetProjectSymbols, Background),
    (GetProjectSymbolsResponse, Background),
    (GetUsers, Foreground),
    (Hello, Foreground),
    (IncomingCall, Foreground),
    (UsersResponse, Foreground),
    (JoinChannel, Foreground),
    (JoinChannelResponse, Foreground),
    (JoinProject, Foreground),
    (JoinProjectResponse, Foreground),
    (JoinRoom, Foreground),
    (JoinRoomResponse, Foreground),
    (LeaveChannel, Foreground),
    (LeaveProject, Foreground),
    (LeaveRoom, Foreground),
    (OpenBufferById, Background),
    (OpenBufferByPath, Background),
    (OpenBufferForSymbol, Background),
    (OpenBufferForSymbolResponse, Background),
    (OpenBufferResponse, Background),
    (PerformRename, Background),
    (PerformRenameResponse, Background),
    (Ping, Foreground),
    (PrepareRename, Background),
    (PrepareRenameResponse, Background),
    (ProjectEntryResponse, Foreground),
    (RejoinRoom, Foreground),
    (RejoinRoomResponse, Foreground),
    (RemoveContact, Foreground),
    (ReloadBuffers, Foreground),
    (ReloadBuffersResponse, Foreground),
    (RemoveProjectCollaborator, Foreground),
    (RenameProjectEntry, Foreground),
    (RequestContact, Foreground),
    (RespondToContactRequest, Foreground),
    (RoomUpdated, Foreground),
    (SaveBuffer, Foreground),
    (SearchProject, Background),
    (SearchProjectResponse, Background),
    (SendChannelMessage, Foreground),
    (SendChannelMessageResponse, Foreground),
    (ShareProject, Foreground),
    (ShareProjectResponse, Foreground),
    (ShowContacts, Foreground),
    (StartLanguageServer, Foreground),
    (SynchronizeBuffers, Foreground),
    (SynchronizeBuffersResponse, Foreground),
    (Test, Foreground),
    (Unfollow, Foreground),
    (UnshareProject, Foreground),
    (UpdateBuffer, Foreground),
    (UpdateBufferFile, Foreground),
    (UpdateContacts, Foreground),
    (UpdateDiagnosticSummary, Foreground),
    (UpdateFollowers, Foreground),
    (UpdateInviteInfo, Foreground),
    (UpdateLanguageServer, Foreground),
    (UpdateParticipantLocation, Foreground),
    (UpdateProject, Foreground),
    (UpdateProjectCollaborator, Foreground),
    (UpdateWorktree, Foreground),
    (UpdateDiffBase, Background),
    (GetPrivateUserInfo, Foreground),
    (GetPrivateUserInfoResponse, Foreground),
);

request_messages!(
    (ApplyCodeAction, ApplyCodeActionResponse),
    (
        ApplyCompletionAdditionalEdits,
        ApplyCompletionAdditionalEditsResponse
    ),
    (Call, Ack),
    (CancelCall, Ack),
    (CopyProjectEntry, ProjectEntryResponse),
    (CreateProjectEntry, ProjectEntryResponse),
    (CreateRoom, CreateRoomResponse),
    (DeclineCall, Ack),
    (DeleteProjectEntry, ProjectEntryResponse),
    (Follow, FollowResponse),
    (FormatBuffers, FormatBuffersResponse),
    (GetChannelMessages, GetChannelMessagesResponse),
    (GetChannels, GetChannelsResponse),
    (GetCodeActions, GetCodeActionsResponse),
    (GetHover, GetHoverResponse),
    (GetCompletions, GetCompletionsResponse),
    (GetDefinition, GetDefinitionResponse),
    (GetTypeDefinition, GetTypeDefinitionResponse),
    (GetDocumentHighlights, GetDocumentHighlightsResponse),
    (GetReferences, GetReferencesResponse),
    (GetPrivateUserInfo, GetPrivateUserInfoResponse),
    (GetProjectSymbols, GetProjectSymbolsResponse),
    (FuzzySearchUsers, UsersResponse),
    (GetUsers, UsersResponse),
    (JoinChannel, JoinChannelResponse),
    (JoinProject, JoinProjectResponse),
    (JoinRoom, JoinRoomResponse),
    (LeaveRoom, Ack),
    (RejoinRoom, RejoinRoomResponse),
    (IncomingCall, Ack),
    (OpenBufferById, OpenBufferResponse),
    (OpenBufferByPath, OpenBufferResponse),
    (OpenBufferForSymbol, OpenBufferForSymbolResponse),
    (Ping, Ack),
    (PerformRename, PerformRenameResponse),
    (PrepareRename, PrepareRenameResponse),
    (ReloadBuffers, ReloadBuffersResponse),
    (RequestContact, Ack),
    (RemoveContact, Ack),
    (RespondToContactRequest, Ack),
    (RenameProjectEntry, ProjectEntryResponse),
    (SaveBuffer, BufferSaved),
    (SearchProject, SearchProjectResponse),
    (SendChannelMessage, SendChannelMessageResponse),
    (ShareProject, ShareProjectResponse),
    (SynchronizeBuffers, SynchronizeBuffersResponse),
    (Test, Test),
    (UpdateBuffer, Ack),
    (UpdateParticipantLocation, Ack),
    (UpdateProject, Ack),
    (UpdateWorktree, Ack),
);

entity_messages!(
    project_id,
    AddProjectCollaborator,
    ApplyCodeAction,
    ApplyCompletionAdditionalEdits,
    BufferReloaded,
    BufferSaved,
    CopyProjectEntry,
    CreateBufferForPeer,
    CreateProjectEntry,
    DeleteProjectEntry,
    Follow,
    FormatBuffers,
    GetCodeActions,
    GetCompletions,
    GetDefinition,
    GetTypeDefinition,
    GetDocumentHighlights,
    GetHover,
    GetReferences,
    GetProjectSymbols,
    JoinProject,
    LeaveProject,
    OpenBufferById,
    OpenBufferByPath,
    OpenBufferForSymbol,
    PerformRename,
    PrepareRename,
    ReloadBuffers,
    RemoveProjectCollaborator,
    RenameProjectEntry,
    SaveBuffer,
    SearchProject,
    StartLanguageServer,
    SynchronizeBuffers,
    Unfollow,
    UnshareProject,
    UpdateBuffer,
    UpdateBufferFile,
    UpdateDiagnosticSummary,
    UpdateFollowers,
    UpdateLanguageServer,
    UpdateProject,
    UpdateProjectCollaborator,
    UpdateWorktree,
    UpdateDiffBase
);

entity_messages!(channel_id, ChannelMessageSent);

const KIB: usize = 1024;
const MIB: usize = KIB * 1024;
const MAX_BUFFER_LEN: usize = MIB;

/// A stream of protobuf messages.
pub struct MessageStream<S> {
    stream: S,
    encoding_buffer: Vec<u8>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Message {
    Envelope(Envelope),
    Ping,
    Pong,
}

impl<S> MessageStream<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            encoding_buffer: Vec::new(),
        }
    }

    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.stream
    }
}

impl<S> MessageStream<S>
where
    S: futures::Sink<WebSocketMessage, Error = anyhow::Error> + Unpin,
{
    pub async fn write(&mut self, message: Message) -> Result<(), anyhow::Error> {
        #[cfg(any(test, feature = "test-support"))]
        const COMPRESSION_LEVEL: i32 = -7;

        #[cfg(not(any(test, feature = "test-support")))]
        const COMPRESSION_LEVEL: i32 = 4;

        match message {
            Message::Envelope(message) => {
                self.encoding_buffer.reserve(message.encoded_len());
                message
                    .encode(&mut self.encoding_buffer)
                    .map_err(io::Error::from)?;
                let buffer =
                    zstd::stream::encode_all(self.encoding_buffer.as_slice(), COMPRESSION_LEVEL)
                        .unwrap();

                self.encoding_buffer.clear();
                self.encoding_buffer.shrink_to(MAX_BUFFER_LEN);
                self.stream.send(WebSocketMessage::Binary(buffer)).await?;
            }
            Message::Ping => {
                self.stream
                    .send(WebSocketMessage::Ping(Default::default()))
                    .await?;
            }
            Message::Pong => {
                self.stream
                    .send(WebSocketMessage::Pong(Default::default()))
                    .await?;
            }
        }

        Ok(())
    }
}

impl<S> MessageStream<S>
where
    S: futures::Stream<Item = Result<WebSocketMessage, anyhow::Error>> + Unpin,
{
    pub async fn read(&mut self) -> Result<Message, anyhow::Error> {
        while let Some(bytes) = self.stream.next().await {
            match bytes? {
                WebSocketMessage::Binary(bytes) => {
                    zstd::stream::copy_decode(bytes.as_slice(), &mut self.encoding_buffer).unwrap();
                    let envelope = Envelope::decode(self.encoding_buffer.as_slice())
                        .map_err(io::Error::from)?;

                    self.encoding_buffer.clear();
                    self.encoding_buffer.shrink_to(MAX_BUFFER_LEN);
                    return Ok(Message::Envelope(envelope));
                }
                WebSocketMessage::Ping(_) => return Ok(Message::Ping),
                WebSocketMessage::Pong(_) => return Ok(Message::Pong),
                WebSocketMessage::Close(_) => break,
                _ => {}
            }
        }
        Err(anyhow!("connection closed"))
    }
}

impl From<Timestamp> for SystemTime {
    fn from(val: Timestamp) -> Self {
        UNIX_EPOCH
            .checked_add(Duration::new(val.seconds, val.nanos))
            .unwrap()
    }
}

impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        Self {
            seconds: duration.as_secs(),
            nanos: duration.subsec_nanos(),
        }
    }
}

impl From<u128> for Nonce {
    fn from(nonce: u128) -> Self {
        let upper_half = (nonce >> 64) as u64;
        let lower_half = nonce as u64;
        Self {
            upper_half,
            lower_half,
        }
    }
}

impl From<Nonce> for u128 {
    fn from(nonce: Nonce) -> Self {
        let upper_half = (nonce.upper_half as u128) << 64;
        let lower_half = nonce.lower_half as u128;
        upper_half | lower_half
    }
}

pub fn split_worktree_update(
    mut message: UpdateWorktree,
    max_chunk_size: usize,
) -> impl Iterator<Item = UpdateWorktree> {
    let mut done = false;
    iter::from_fn(move || {
        if done {
            return None;
        }

        let updated_entries_chunk_size = cmp::min(message.updated_entries.len(), max_chunk_size);
        let updated_entries = message
            .updated_entries
            .drain(..updated_entries_chunk_size)
            .collect();

        let removed_entries_chunk_size = cmp::min(message.removed_entries.len(), max_chunk_size);
        let removed_entries = message
            .removed_entries
            .drain(..removed_entries_chunk_size)
            .collect();

        done = message.updated_entries.is_empty() && message.removed_entries.is_empty();
        Some(UpdateWorktree {
            project_id: message.project_id,
            worktree_id: message.worktree_id,
            root_name: message.root_name.clone(),
            abs_path: message.abs_path.clone(),
            updated_entries,
            removed_entries,
            scan_id: message.scan_id,
            is_last_update: done && message.is_last_update,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    async fn test_buffer_size() {
        let (tx, rx) = futures::channel::mpsc::unbounded();
        let mut sink = MessageStream::new(tx.sink_map_err(|_| anyhow!("")));
        sink.write(Message::Envelope(Envelope {
            payload: Some(envelope::Payload::UpdateWorktree(UpdateWorktree {
                root_name: "abcdefg".repeat(10),
                ..Default::default()
            })),
            ..Default::default()
        }))
        .await
        .unwrap();
        assert!(sink.encoding_buffer.capacity() <= MAX_BUFFER_LEN);
        sink.write(Message::Envelope(Envelope {
            payload: Some(envelope::Payload::UpdateWorktree(UpdateWorktree {
                root_name: "abcdefg".repeat(1000000),
                ..Default::default()
            })),
            ..Default::default()
        }))
        .await
        .unwrap();
        assert!(sink.encoding_buffer.capacity() <= MAX_BUFFER_LEN);

        let mut stream = MessageStream::new(rx.map(anyhow::Ok));
        stream.read().await.unwrap();
        assert!(stream.encoding_buffer.capacity() <= MAX_BUFFER_LEN);
        stream.read().await.unwrap();
        assert!(stream.encoding_buffer.capacity() <= MAX_BUFFER_LEN);
    }

    #[gpui::test]
    fn test_converting_peer_id_from_and_to_u64() {
        let peer_id = PeerId {
            owner_id: 10,
            id: 3,
        };
        assert_eq!(PeerId::from_u64(peer_id.as_u64()), peer_id);
        let peer_id = PeerId {
            owner_id: u32::MAX,
            id: 3,
        };
        assert_eq!(PeerId::from_u64(peer_id.as_u64()), peer_id);
        let peer_id = PeerId {
            owner_id: 10,
            id: u32::MAX,
        };
        assert_eq!(PeerId::from_u64(peer_id.as_u64()), peer_id);
        let peer_id = PeerId {
            owner_id: u32::MAX,
            id: u32::MAX,
        };
        assert_eq!(PeerId::from_u64(peer_id.as_u64()), peer_id);
    }
}
