use futures_io::{AsyncRead, AsyncWrite};
use futures_lite::{AsyncReadExt, AsyncWriteExt as _};
use prost::Message;
use std::io;

include!(concat!(env!("OUT_DIR"), "/zed.messages.rs"));

pub trait Request {
    type Response;
}

impl Request for from_client::Auth {
    type Response = from_server::Ack;
}

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
}

impl<T> MessageStream<T>
where
    T: AsyncWrite + Unpin,
{
    /// Write a given protobuf message to the stream.
    pub async fn write_message(&mut self, message: &impl Message) -> futures_io::Result<()> {
        self.buffer.clear();
        message.encode_length_delimited(&mut self.buffer).unwrap();
        self.byte_stream.write_all(&self.buffer).await
    }
}

impl<T> MessageStream<T>
where
    T: AsyncRead + Unpin,
{
    /// Read a protobuf message of the given type from the stream.
    pub async fn read_message<M: Message + Default>(&mut self) -> futures_io::Result<M> {
        // Ensure the buffer is large enough to hold the maximum delimiter length
        const MAX_DELIMITER_LEN: usize = 10;
        self.buffer.clear();
        self.buffer.resize(MAX_DELIMITER_LEN, 0);

        // Read until a complete length delimiter can be decoded.
        let mut read_start_offset = 0;
        let (encoded_len, delimiter_len) = loop {
            let bytes_read = self
                .byte_stream
                .read(&mut self.buffer[read_start_offset..])
                .await?;
            read_start_offset += bytes_read;

            let mut buffer = &self.buffer[0..read_start_offset];
            match prost::decode_length_delimiter(&mut buffer) {
                Err(_) => {
                    if read_start_offset >= MAX_DELIMITER_LEN {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "invalid message length delimiter",
                        ));
                    }
                }
                Ok(encoded_len) => {
                    let delimiter_len = read_start_offset - buffer.len();
                    break (encoded_len, delimiter_len);
                }
            }
        };

        // Read the message itself.
        self.buffer.resize(delimiter_len + encoded_len, 0);
        self.byte_stream
            .read_exact(&mut self.buffer[read_start_offset..])
            .await?;
        let message = M::decode(&self.buffer[delimiter_len..])?;

        Ok(message)
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

            // In reality there will never be both `FromClient` and `FromServer` messages
            // sent in the same direction on the same stream.
            let message1 = FromClient {
                id: 3,
                variant: Some(from_client::Variant::Auth(from_client::Auth {
                    user_id: 5,
                    access_token: "the-access-token".into(),
                })),
            };
            let message2 = FromServer {
                request_id: Some(4),
                variant: Some(from_server::Variant::Ack(from_server::Ack {
                    error_message: Some(
                        format!(
                            "a {}long error message that requires a two-byte length delimiter",
                            "very ".repeat(60)
                        )
                        .into(),
                    ),
                })),
            };

            let mut message_stream = MessageStream::new(byte_stream);
            message_stream.write_message(&message1).await.unwrap();
            message_stream.write_message(&message2).await.unwrap();
            let decoded_message1 = message_stream.read_message::<FromClient>().await.unwrap();
            let decoded_message2 = message_stream.read_message::<FromServer>().await.unwrap();
            assert_eq!(decoded_message1, message1);
            assert_eq!(decoded_message2, message2);
        });
    }

    #[test]
    fn test_read_message_when_length_delimiter_is_not_complete_in_first_read() {
        smol::block_on(async {
            let byte_stream = ChunkedStream {
                bytes: Vec::new(),
                read_offset: 0,
                chunk_size: 2,
            };

            // This message is so long that its length delimiter requires three bytes,
            // so it won't be delivered in a single read from the chunked byte stream.
            let message = FromServer {
                request_id: Some(4),
                variant: Some(from_server::Variant::Ack(from_server::Ack {
                    error_message: Some("long ".repeat(256 * 256).into()),
                })),
            };
            assert!(prost::length_delimiter_len(message.encoded_len()) > byte_stream.chunk_size);

            let mut message_stream = MessageStream::new(byte_stream);
            message_stream.write_message(&message).await.unwrap();
            let decoded_message = message_stream.read_message::<FromServer>().await.unwrap();
            assert_eq!(decoded_message, message);
        });
    }

    #[test]
    fn test_protobuf_parse_error() {
        smol::block_on(async {
            let byte_stream = ChunkedStream {
                bytes: Vec::new(),
                read_offset: 0,
                chunk_size: 2,
            };

            let message = FromClient {
                id: 3,
                variant: Some(from_client::Variant::Auth(from_client::Auth {
                    user_id: 5,
                    access_token: "the-access-token".into(),
                })),
            };

            let mut message_stream = MessageStream::new(byte_stream);
            message_stream.write_message(&message).await.unwrap();

            // Read the wrong type of message from the stream.
            let result = message_stream.read_message::<FromServer>().await;
            assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
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
