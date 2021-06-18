use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt as _};
use prost::Message;
use std::{convert::TryInto, io};

include!(concat!(env!("OUT_DIR"), "/zed.messages.rs"));

pub trait EnvelopedMessage: Sized + Send + 'static {
    const NAME: &'static str;
    fn into_envelope(self, id: u32, responding_to: Option<u32>) -> Envelope;
    fn matches_envelope(envelope: &Envelope) -> bool;
    fn from_envelope(envelope: Envelope) -> Option<Self>;
}

pub trait RequestMessage: EnvelopedMessage {
    type Response: EnvelopedMessage;
}

macro_rules! message {
    ($name:ident) => {
        impl EnvelopedMessage for $name {
            const NAME: &'static str = std::stringify!($name);

            fn into_envelope(self, id: u32, responding_to: Option<u32>) -> Envelope {
                Envelope {
                    id,
                    responding_to,
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
    };
}

macro_rules! request_message {
    ($req:ident, $resp:ident) => {
        message!($req);
        message!($resp);
        impl RequestMessage for $req {
            type Response = $resp;
        }
    };
}

request_message!(Auth, AuthResponse);
request_message!(ShareWorktree, ShareWorktreeResponse);
request_message!(OpenWorktree, OpenWorktreeResponse);
request_message!(OpenBuffer, OpenBufferResponse);

/// A stream of protobuf messages.
pub struct MessageStream<T> {
    byte_stream: T,
    buffer: Vec<u8>,
}

impl<T> MessageStream<T> {
    pub fn new(byte_stream: T) -> Self {
        Self {
            byte_stream,
            buffer: Default::default(),
        }
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.byte_stream
    }
}

impl<T> MessageStream<T>
where
    T: AsyncWrite + Unpin,
{
    /// Write a given protobuf message to the stream.
    pub async fn write_message(&mut self, message: &Envelope) -> io::Result<()> {
        let message_len: u32 = message
            .encoded_len()
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "message is too large"))?;
        self.buffer.clear();
        self.buffer.extend_from_slice(&message_len.to_be_bytes());
        message.encode(&mut self.buffer)?;
        self.byte_stream.write_all(&self.buffer).await
    }
}

impl<T> MessageStream<T>
where
    T: AsyncRead + Unpin,
{
    /// Read a protobuf message of the given type from the stream.
    pub async fn read_message(&mut self) -> io::Result<Envelope> {
        let mut delimiter_buf = [0; 4];
        self.byte_stream.read_exact(&mut delimiter_buf).await?;
        let message_len = u32::from_be_bytes(delimiter_buf) as usize;
        self.buffer.resize(message_len, 0);
        self.byte_stream.read_exact(&mut self.buffer).await?;
        Ok(Envelope::decode(self.buffer.as_slice())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    #[test]
    fn test_round_trip_message() {
        smol::block_on(async {
            let byte_stream = ChunkedStream {
                bytes: Vec::new(),
                read_offset: 0,
                chunk_size: 3,
            };

            let message1 = Auth {
                user_id: 5,
                access_token: "the-access-token".into(),
            }
            .into_envelope(3, None);

            let message2 = ShareWorktree {
                worktree: Some(Worktree {
                    paths: vec!["ok".to_string()],
                }),
            }
            .into_envelope(5, None);

            let mut message_stream = MessageStream::new(byte_stream);
            message_stream.write_message(&message1).await.unwrap();
            message_stream.write_message(&message2).await.unwrap();
            let decoded_message1 = message_stream.read_message().await.unwrap();
            let decoded_message2 = message_stream.read_message().await.unwrap();
            assert_eq!(decoded_message1, message1);
            assert_eq!(decoded_message2, message2);
        });
    }

    struct ChunkedStream {
        bytes: Vec<u8>,
        read_offset: usize,
        chunk_size: usize,
    }

    impl AsyncWrite for ChunkedStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            let bytes_written = buf.len().min(self.chunk_size);
            self.bytes.extend_from_slice(&buf[0..bytes_written]);
            Poll::Ready(Ok(bytes_written))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    impl AsyncRead for ChunkedStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            let bytes_read = buf
                .len()
                .min(self.chunk_size)
                .min(self.bytes.len() - self.read_offset);
            let end_offset = self.read_offset + bytes_read;
            buf[0..bytes_read].copy_from_slice(&self.bytes[self.read_offset..end_offset]);
            self.read_offset = end_offset;
            Poll::Ready(Ok(bytes_read))
        }
    }
}
