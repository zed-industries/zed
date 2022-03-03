use super::{ConnectionId, PeerId, TypedEnvelope};
use anyhow::Result;
use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use futures::{SinkExt as _, StreamExt as _};
use prost::Message;
use std::any::{Any, TypeId};
use std::{
    io,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

include!(concat!(env!("OUT_DIR"), "/zed.messages.rs"));

pub trait EnvelopedMessage: Clone + Sized + Send + Sync + 'static {
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

macro_rules! messages {
    ($(($name:ident, $priority:ident)),* $(,)?) => {
        pub fn build_typed_envelope(sender_id: ConnectionId, envelope: Envelope) -> Option<Box<dyn AnyTypedEnvelope>> {
            match envelope.payload {
                $(Some(envelope::Payload::$name(payload)) => {
                    Some(Box::new(TypedEnvelope {
                        sender_id,
                        original_sender_id: envelope.original_sender_id.map(PeerId),
                        message_id: envelope.id,
                        payload,
                    }))
                }, )*
                _ => None
            }
        }

        $(
            impl EnvelopedMessage for $name {
                const NAME: &'static str = std::stringify!($name);
                const PRIORITY: MessagePriority = MessagePriority::$priority;

                fn into_envelope(
                    self,
                    id: u32,
                    responding_to: Option<u32>,
                    original_sender_id: Option<u32>,
                ) -> Envelope {
                    Envelope {
                        id,
                        responding_to,
                        original_sender_id,
                        payload: Some(envelope::Payload::$name(self)),
                    }
                }

                fn from_envelope(envelope: Envelope) -> Option<Self> {
                    if let Some(envelope::Payload::$name(msg)) = envelope.payload {
                        Some(msg)
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

macro_rules! request_messages {
    ($(($request_name:ident, $response_name:ident)),* $(,)?) => {
        $(impl RequestMessage for $request_name {
            type Response = $response_name;
        })*
    };
}

macro_rules! entity_messages {
    ($id_field:ident, $($name:ident),* $(,)?) => {
        $(impl EntityMessage for $name {
            fn remote_entity_id(&self) -> u64 {
                self.$id_field
            }
        })*
    };
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
    (ChannelMessageSent, Foreground),
    (CloseBuffer, Foreground),
    (DiskBasedDiagnosticsUpdated, Background),
    (DiskBasedDiagnosticsUpdating, Background),
    (Error, Foreground),
    (FormatBuffers, Foreground),
    (FormatBuffersResponse, Foreground),
    (GetChannelMessages, Foreground),
    (GetChannelMessagesResponse, Foreground),
    (GetChannels, Foreground),
    (GetChannelsResponse, Foreground),
    (GetCodeActions, Background),
    (GetCodeActionsResponse, Background),
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
    (GetUsersResponse, Foreground),
    (JoinChannel, Foreground),
    (JoinChannelResponse, Foreground),
    (JoinProject, Foreground),
    (JoinProjectResponse, Foreground),
    (LeaveChannel, Foreground),
    (LeaveProject, Foreground),
    (OpenBuffer, Background),
    (OpenBufferForSymbol, Background),
    (OpenBufferForSymbolResponse, Background),
    (OpenBufferResponse, Background),
    (PerformRename, Background),
    (PerformRenameResponse, Background),
    (PrepareRename, Background),
    (PrepareRenameResponse, Background),
    (RegisterProjectResponse, Foreground),
    (Ping, Foreground),
    (RegisterProject, Foreground),
    (RegisterWorktree, Foreground),
    (RemoveProjectCollaborator, Foreground),
    (SaveBuffer, Foreground),
    (SearchProject, Background),
    (SearchProjectResponse, Background),
    (SendChannelMessage, Foreground),
    (SendChannelMessageResponse, Foreground),
    (ShareProject, Foreground),
    (Test, Foreground),
    (UnregisterProject, Foreground),
    (UnregisterWorktree, Foreground),
    (UnshareProject, Foreground),
    (UpdateBuffer, Background),
    (UpdateBufferFile, Foreground),
    (UpdateContacts, Foreground),
    (UpdateDiagnosticSummary, Foreground),
    (UpdateWorktree, Foreground),
);

request_messages!(
    (ApplyCodeAction, ApplyCodeActionResponse),
    (
        ApplyCompletionAdditionalEdits,
        ApplyCompletionAdditionalEditsResponse
    ),
    (FormatBuffers, FormatBuffersResponse),
    (GetChannelMessages, GetChannelMessagesResponse),
    (GetChannels, GetChannelsResponse),
    (GetCodeActions, GetCodeActionsResponse),
    (GetCompletions, GetCompletionsResponse),
    (GetDefinition, GetDefinitionResponse),
    (GetDocumentHighlights, GetDocumentHighlightsResponse),
    (GetReferences, GetReferencesResponse),
    (GetProjectSymbols, GetProjectSymbolsResponse),
    (GetUsers, GetUsersResponse),
    (JoinChannel, JoinChannelResponse),
    (JoinProject, JoinProjectResponse),
    (OpenBuffer, OpenBufferResponse),
    (OpenBufferForSymbol, OpenBufferForSymbolResponse),
    (Ping, Ack),
    (PerformRename, PerformRenameResponse),
    (PrepareRename, PrepareRenameResponse),
    (RegisterProject, RegisterProjectResponse),
    (RegisterWorktree, Ack),
    (SaveBuffer, BufferSaved),
    (SearchProject, SearchProjectResponse),
    (SendChannelMessage, SendChannelMessageResponse),
    (ShareProject, Ack),
    (Test, Test),
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
    CloseBuffer,
    DiskBasedDiagnosticsUpdated,
    DiskBasedDiagnosticsUpdating,
    FormatBuffers,
    GetCodeActions,
    GetCompletions,
    GetDefinition,
    GetDocumentHighlights,
    GetReferences,
    GetProjectSymbols,
    JoinProject,
    LeaveProject,
    OpenBuffer,
    OpenBufferForSymbol,
    PerformRename,
    PrepareRename,
    RemoveProjectCollaborator,
    SaveBuffer,
    SearchProject,
    UnregisterWorktree,
    UnshareProject,
    UpdateBuffer,
    UpdateBufferFile,
    UpdateDiagnosticSummary,
    RegisterWorktree,
    UpdateWorktree,
);

entity_messages!(channel_id, ChannelMessageSent);

/// A stream of protobuf messages.
pub struct MessageStream<S> {
    stream: S,
    encoding_buffer: Vec<u8>,
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
    S: futures::Sink<WebSocketMessage, Error = WebSocketError> + Unpin,
{
    /// Write a given protobuf message to the stream.
    pub async fn write_message(&mut self, message: &Envelope) -> Result<(), WebSocketError> {
        #[cfg(any(test, feature = "test-support"))]
        const COMPRESSION_LEVEL: i32 = -7;

        #[cfg(not(any(test, feature = "test-support")))]
        const COMPRESSION_LEVEL: i32 = 4;

        self.encoding_buffer.resize(message.encoded_len(), 0);
        self.encoding_buffer.clear();
        message
            .encode(&mut self.encoding_buffer)
            .map_err(|err| io::Error::from(err))?;
        let buffer =
            zstd::stream::encode_all(self.encoding_buffer.as_slice(), COMPRESSION_LEVEL).unwrap();
        self.stream.send(WebSocketMessage::Binary(buffer)).await?;
        Ok(())
    }
}

impl<S> MessageStream<S>
where
    S: futures::Stream<Item = Result<WebSocketMessage, WebSocketError>> + Unpin,
{
    /// Read a protobuf message of the given type from the stream.
    pub async fn read_message(&mut self) -> Result<Envelope, WebSocketError> {
        while let Some(bytes) = self.stream.next().await {
            match bytes? {
                WebSocketMessage::Binary(bytes) => {
                    self.encoding_buffer.clear();
                    zstd::stream::copy_decode(bytes.as_slice(), &mut self.encoding_buffer).unwrap();
                    let envelope = Envelope::decode(self.encoding_buffer.as_slice())
                        .map_err(io::Error::from)?;
                    return Ok(envelope);
                }
                WebSocketMessage::Close(_) => break,
                _ => {}
            }
        }
        Err(WebSocketError::ConnectionClosed)
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
