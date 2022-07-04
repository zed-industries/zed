use super::{entity_messages, messages, request_messages, ConnectionId, PeerId, TypedEnvelope};
use anyhow::{anyhow, Result};
use async_tungstenite::tungstenite::Message as WebSocketMessage;
use futures::{SinkExt as _, StreamExt as _};
use prost::Message as _;
use serde::Serialize;
use std::any::{Any, TypeId};
use std::{cmp, iter, mem};
use std::{
    fmt::Debug,
    io,
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
        original_sender_id: Option<u32>,
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
    (RemoveContact, Foreground),
    (ChannelMessageSent, Foreground),
    (CopyProjectEntry, Foreground),
    (CreateProjectEntry, Foreground),
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
    (GetDocumentHighlights, Background),
    (GetDocumentHighlightsResponse, Background),
    (GetReferences, Background),
    (GetReferencesResponse, Background),
    (GetProjectSymbols, Background),
    (GetProjectSymbolsResponse, Background),
    (GetUsers, Foreground),
    (UsersResponse, Foreground),
    (JoinChannel, Foreground),
    (JoinChannelResponse, Foreground),
    (JoinProject, Foreground),
    (JoinProjectResponse, Foreground),
    (JoinProjectRequestCancelled, Foreground),
    (LeaveChannel, Foreground),
    (LeaveProject, Foreground),
    (OpenBufferById, Background),
    (OpenBufferByPath, Background),
    (OpenBufferForSymbol, Background),
    (OpenBufferForSymbolResponse, Background),
    (OpenBufferResponse, Background),
    (PerformRename, Background),
    (PerformRenameResponse, Background),
    (PrepareRename, Background),
    (PrepareRenameResponse, Background),
    (ProjectEntryResponse, Foreground),
    (ProjectUnshared, Foreground),
    (RegisterProjectResponse, Foreground),
    (Ping, Foreground),
    (RegisterProject, Foreground),
    (RegisterProjectActivity, Foreground),
    (ReloadBuffers, Foreground),
    (ReloadBuffersResponse, Foreground),
    (RemoveProjectCollaborator, Foreground),
    (RenameProjectEntry, Foreground),
    (RequestContact, Foreground),
    (RequestJoinProject, Foreground),
    (RespondToContactRequest, Foreground),
    (RespondToJoinProjectRequest, Foreground),
    (SaveBuffer, Foreground),
    (SearchProject, Background),
    (SearchProjectResponse, Background),
    (SendChannelMessage, Foreground),
    (SendChannelMessageResponse, Foreground),
    (ShowContacts, Foreground),
    (StartLanguageServer, Foreground),
    (Test, Foreground),
    (Unfollow, Foreground),
    (UnregisterProject, Foreground),
    (UpdateBuffer, Foreground),
    (UpdateBufferFile, Foreground),
    (UpdateContacts, Foreground),
    (UpdateDiagnosticSummary, Foreground),
    (UpdateFollowers, Foreground),
    (UpdateInviteInfo, Foreground),
    (UpdateLanguageServer, Foreground),
    (UpdateProject, Foreground),
    (UpdateWorktree, Foreground),
    (UpdateWorktreeExtensions, Background),
);

request_messages!(
    (ApplyCodeAction, ApplyCodeActionResponse),
    (
        ApplyCompletionAdditionalEdits,
        ApplyCompletionAdditionalEditsResponse
    ),
    (CopyProjectEntry, ProjectEntryResponse),
    (CreateProjectEntry, ProjectEntryResponse),
    (DeleteProjectEntry, ProjectEntryResponse),
    (Follow, FollowResponse),
    (FormatBuffers, FormatBuffersResponse),
    (GetChannelMessages, GetChannelMessagesResponse),
    (GetChannels, GetChannelsResponse),
    (GetCodeActions, GetCodeActionsResponse),
    (GetHover, GetHoverResponse),
    (GetCompletions, GetCompletionsResponse),
    (GetDefinition, GetDefinitionResponse),
    (GetDocumentHighlights, GetDocumentHighlightsResponse),
    (GetReferences, GetReferencesResponse),
    (GetProjectSymbols, GetProjectSymbolsResponse),
    (FuzzySearchUsers, UsersResponse),
    (GetUsers, UsersResponse),
    (JoinChannel, JoinChannelResponse),
    (JoinProject, JoinProjectResponse),
    (OpenBufferById, OpenBufferResponse),
    (OpenBufferByPath, OpenBufferResponse),
    (OpenBufferForSymbol, OpenBufferForSymbolResponse),
    (Ping, Ack),
    (PerformRename, PerformRenameResponse),
    (PrepareRename, PrepareRenameResponse),
    (RegisterProject, RegisterProjectResponse),
    (ReloadBuffers, ReloadBuffersResponse),
    (RequestContact, Ack),
    (RemoveContact, Ack),
    (RespondToContactRequest, Ack),
    (RenameProjectEntry, ProjectEntryResponse),
    (SaveBuffer, BufferSaved),
    (SearchProject, SearchProjectResponse),
    (SendChannelMessage, SendChannelMessageResponse),
    (Test, Test),
    (UnregisterProject, Ack),
    (UpdateBuffer, Ack),
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
    CreateProjectEntry,
    DeleteProjectEntry,
    Follow,
    FormatBuffers,
    GetCodeActions,
    GetCompletions,
    GetDefinition,
    GetDocumentHighlights,
    GetHover,
    GetReferences,
    GetProjectSymbols,
    JoinProject,
    JoinProjectRequestCancelled,
    LeaveProject,
    OpenBufferById,
    OpenBufferByPath,
    OpenBufferForSymbol,
    PerformRename,
    PrepareRename,
    ProjectUnshared,
    RegisterProjectActivity,
    ReloadBuffers,
    RemoveProjectCollaborator,
    RenameProjectEntry,
    RequestJoinProject,
    SaveBuffer,
    SearchProject,
    StartLanguageServer,
    Unfollow,
    UnregisterProject,
    UpdateBuffer,
    UpdateBufferFile,
    UpdateDiagnosticSummary,
    UpdateFollowers,
    UpdateLanguageServer,
    UpdateProject,
    UpdateWorktree,
    UpdateWorktreeExtensions,
);

entity_messages!(channel_id, ChannelMessageSent);

const MAX_BUFFER_LEN: usize = 1 * 1024 * 1024;

/// A stream of protobuf messages.
pub struct MessageStream<S> {
    stream: S,
    encoding_buffer: Vec<u8>,
}

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
                    .map_err(|err| io::Error::from(err))?;
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

impl Into<SystemTime> for Timestamp {
    fn into(self) -> SystemTime {
        UNIX_EPOCH
            .checked_add(Duration::new(self.seconds, self.nanos))
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

        let chunk_size = cmp::min(message.updated_entries.len(), max_chunk_size);
        let updated_entries = message.updated_entries.drain(..chunk_size).collect();
        done = message.updated_entries.is_empty();
        Some(UpdateWorktree {
            project_id: message.project_id,
            worktree_id: message.worktree_id,
            root_name: message.root_name.clone(),
            updated_entries,
            removed_entries: mem::take(&mut message.removed_entries),
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

        let mut stream = MessageStream::new(rx.map(|msg| anyhow::Ok(msg)));
        stream.read().await.unwrap();
        assert!(stream.encoding_buffer.capacity() <= MAX_BUFFER_LEN);
        stream.read().await.unwrap();
        assert!(stream.encoding_buffer.capacity() <= MAX_BUFFER_LEN);
    }
}
