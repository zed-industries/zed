use super::{ConnectionId, PeerId, TypedEnvelope};
use anyhow::Result;
use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use futures::{SinkExt as _, StreamExt as _};
use prost::Message;
use std::any::Any;
use std::sync::Arc;
use std::{
    io,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

include!(concat!(env!("OUT_DIR"), "/zed.messages.rs"));

pub trait EnvelopedMessage: Clone + Sized + Send + Sync + 'static {
    const NAME: &'static str;
    fn into_envelope(
        self,
        id: u32,
        responding_to: Option<u32>,
        original_sender_id: Option<u32>,
    ) -> Envelope;
    fn matches_envelope(envelope: &Envelope) -> bool;
    fn from_envelope(envelope: Envelope) -> Option<Self>;
}

pub trait EntityMessage: EnvelopedMessage {
    fn remote_entity_id(&self) -> u64;
}

pub trait RequestMessage: EnvelopedMessage {
    type Response: EnvelopedMessage;
}

macro_rules! messages {
    ($($name:ident),* $(,)?) => {
        pub fn build_typed_envelope(sender_id: ConnectionId, envelope: Envelope) -> Option<Arc<dyn Any + Send + Sync>> {
            match envelope.payload {
                $(Some(envelope::Payload::$name(payload)) => Some(Arc::new(TypedEnvelope {
                    sender_id,
                    original_sender_id: envelope.original_sender_id.map(PeerId),
                    message_id: envelope.id,
                    payload,
                })), )*
                _ => None
            }
        }

        $(
            impl EnvelopedMessage for $name {
                const NAME: &'static str = std::stringify!($name);

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

                fn matches_envelope(envelope: &Envelope) -> bool {
                    matches!(&envelope.payload, Some(envelope::Payload::$name(_)))
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
    AddPeer,
    Auth,
    AuthResponse,
    BufferSaved,
    ChannelMessageSent,
    CloseBuffer,
    CloseWorktree,
    GetChannels,
    GetChannelsResponse,
    GetUsers,
    GetUsersResponse,
    JoinChannel,
    JoinChannelResponse,
    OpenBuffer,
    OpenBufferResponse,
    OpenWorktree,
    OpenWorktreeResponse,
    RemovePeer,
    SaveBuffer,
    SendChannelMessage,
    ShareWorktree,
    ShareWorktreeResponse,
    UpdateBuffer,
    UpdateWorktree,
);

request_messages!(
    (Auth, AuthResponse),
    (GetChannels, GetChannelsResponse),
    (GetUsers, GetUsersResponse),
    (JoinChannel, JoinChannelResponse),
    (OpenBuffer, OpenBufferResponse),
    (OpenWorktree, OpenWorktreeResponse),
    (SaveBuffer, BufferSaved),
    (ShareWorktree, ShareWorktreeResponse),
);

entity_messages!(
    worktree_id,
    AddPeer,
    BufferSaved,
    CloseBuffer,
    CloseWorktree,
    OpenBuffer,
    OpenWorktree,
    RemovePeer,
    SaveBuffer,
    UpdateBuffer,
    UpdateWorktree,
);

entity_messages!(channel_id, ChannelMessageSent);

/// A stream of protobuf messages.
pub struct MessageStream<S> {
    stream: S,
}

impl<S> MessageStream<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
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
        let mut buffer = Vec::with_capacity(message.encoded_len());
        message
            .encode(&mut buffer)
            .map_err(|err| io::Error::from(err))?;
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
                    let envelope = Envelope::decode(bytes.as_slice()).map_err(io::Error::from)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    #[test]
    fn test_round_trip_message() {
        smol::block_on(async {
            let stream = test::Channel::new();
            let message1 = Auth {
                user_id: 5,
                access_token: "the-access-token".into(),
            }
            .into_envelope(3, None, None);

            let message2 = OpenBuffer {
                worktree_id: 0,
                path: "some/path".to_string(),
            }
            .into_envelope(5, None, None);

            let mut message_stream = MessageStream::new(stream);
            message_stream.write_message(&message1).await.unwrap();
            message_stream.write_message(&message2).await.unwrap();
            let decoded_message1 = message_stream.read_message().await.unwrap();
            let decoded_message2 = message_stream.read_message().await.unwrap();
            assert_eq!(decoded_message1, message1);
            assert_eq!(decoded_message2, message2);
        });
    }
}
