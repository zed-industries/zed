use anyhow::Result;
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use prost::Message as _;
use rpc::proto::Envelope;
use std::io::Read as _;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MessageId(pub u32);

pub type MessageLen = u32;
pub const MESSAGE_LEN_SIZE: usize = size_of::<MessageLen>();

const KIB: usize = 1024;
const MIB: usize = KIB * 1024;
const COMPRESSION_THRESHOLD: usize = 16 * KIB;
const MAX_DECOMPRESSED_MESSAGE_LEN: usize = 512 * MIB;
const COMPRESSED_FRAME_BIT: MessageLen = 0x8000_0000;
const FRAME_LEN_MASK: MessageLen = 0x7fff_ffff;

const COMPRESSION_LEVEL: i32 = 4;

#[derive(Debug, Copy, Clone)]
struct FrameHeader {
    is_compressed: bool,
    len: usize,
}

pub fn message_len_from_buffer(buffer: &[u8]) -> Result<MessageLen> {
    let bytes = buffer
        .get(..MESSAGE_LEN_SIZE)
        .ok_or_else(|| anyhow::anyhow!("missing remote message length prefix"))?;
    Ok(MessageLen::from_le_bytes(bytes.try_into()?))
}

fn frame_header(message_len: MessageLen) -> FrameHeader {
    FrameHeader {
        is_compressed: message_len & COMPRESSED_FRAME_BIT != 0,
        len: (message_len & FRAME_LEN_MASK) as usize,
    }
}

fn frame_prefix(len: usize, is_compressed: bool) -> Result<[u8; MESSAGE_LEN_SIZE]> {
    anyhow::ensure!(
        len <= FRAME_LEN_MASK as usize,
        "remote message exceeds maximum frame length"
    );

    let mut message_len = len as MessageLen;
    if is_compressed {
        message_len |= COMPRESSED_FRAME_BIT;
    }
    Ok(message_len.to_le_bytes())
}

fn decode_compressed_payload(compressed: Vec<u8>, buffer: &mut Vec<u8>) -> Result<()> {
    buffer.clear();
    let decoder = zstd::stream::read::Decoder::new(compressed.as_slice())?;
    let mut decoder = decoder.take(MAX_DECOMPRESSED_MESSAGE_LEN as u64 + 1);
    decoder.read_to_end(buffer)?;

    anyhow::ensure!(
        buffer.len() <= MAX_DECOMPRESSED_MESSAGE_LEN,
        "compressed remote message exceeds maximum decompressed length"
    );
    Ok(())
}

fn maybe_compress(buffer: &[u8]) -> Result<Option<Vec<u8>>> {
    if buffer.len() < COMPRESSION_THRESHOLD {
        return Ok(None);
    }

    let compressed = zstd::stream::encode_all(buffer, COMPRESSION_LEVEL)?;
    if compressed.len() < buffer.len() {
        Ok(Some(compressed))
    } else {
        Ok(None)
    }
}

pub async fn read_message_with_len<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    message_len: MessageLen,
) -> Result<Envelope> {
    let frame_header = frame_header(message_len);
    buffer.resize(frame_header.len, 0);
    stream.read_exact(buffer).await?;

    if frame_header.is_compressed {
        let compressed = std::mem::take(buffer);
        decode_compressed_payload(compressed, buffer)?;
    }

    Ok(Envelope::decode(buffer.as_slice())?)
}

pub async fn read_message<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<Envelope> {
    buffer.resize(MESSAGE_LEN_SIZE, 0);
    stream.read_exact(buffer).await?;

    let len = message_len_from_buffer(buffer)?;

    read_message_with_len(stream, buffer, len).await
}

pub async fn write_message<S: AsyncWrite + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    message: Envelope,
) -> Result<()> {
    buffer.clear();
    buffer.reserve(message.encoded_len());
    message.encode(buffer)?;

    if let Some(compressed) = maybe_compress(buffer)? {
        stream
            .write_all(&frame_prefix(compressed.len(), true)?)
            .await?;
        stream.write_all(&compressed).await?;
        return Ok(());
    }

    stream
        .write_all(&frame_prefix(buffer.len(), false)?)
        .await?;
    stream.write_all(buffer).await?;
    Ok(())
}

pub async fn write_frame_raw<S: AsyncWrite + Unpin>(stream: &mut S, buffer: &[u8]) -> Result<()> {
    stream.write_all(buffer).await?;
    Ok(())
}

pub async fn read_frame_raw<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<()> {
    buffer.resize(MESSAGE_LEN_SIZE, 0);
    stream.read_exact(buffer).await?;

    let message_len = message_len_from_buffer(buffer)?;
    let frame_header = frame_header(message_len);
    let payload_offset = MESSAGE_LEN_SIZE;
    buffer.resize(payload_offset + frame_header.len, 0);
    stream.read_exact(&mut buffer[payload_offset..]).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::Cursor;
    use rpc::proto::{UpdateWorktree, envelope};

    #[gpui::test]
    async fn test_small_messages_are_not_compressed() {
        let message = Envelope {
            payload: Some(envelope::Payload::UpdateWorktree(UpdateWorktree {
                root_name: "small".into(),
                ..Default::default()
            })),
            ..Default::default()
        };

        let mut writer = Cursor::new(Vec::new());
        let mut buffer = Vec::new();
        write_message(&mut writer, &mut buffer, message.clone())
            .await
            .expect("write message");

        let bytes = writer.into_inner();
        let message_len = message_len_from_buffer(&bytes[..MESSAGE_LEN_SIZE])
            .expect("read frame prefix from encoded message");
        assert_eq!(message_len & COMPRESSED_FRAME_BIT, 0);

        let mut reader = Cursor::new(bytes);
        let decoded = read_message(&mut reader, &mut buffer)
            .await
            .expect("read message");
        assert_eq!(decoded, message);
    }

    #[gpui::test]
    async fn test_large_compressible_messages_are_compressed() {
        let message = Envelope {
            payload: Some(envelope::Payload::UpdateWorktree(UpdateWorktree {
                root_name: "abcdefg".repeat(20_000),
                ..Default::default()
            })),
            ..Default::default()
        };

        let mut writer = Cursor::new(Vec::new());
        let mut buffer = Vec::new();
        write_message(&mut writer, &mut buffer, message.clone())
            .await
            .expect("write message");

        let bytes = writer.into_inner();
        let message_len = message_len_from_buffer(&bytes[..MESSAGE_LEN_SIZE])
            .expect("read frame prefix from encoded message");
        assert_ne!(message_len & COMPRESSED_FRAME_BIT, 0);
        assert!(bytes.len() < message.encoded_len());

        let mut reader = Cursor::new(bytes);
        let decoded = read_message(&mut reader, &mut buffer)
            .await
            .expect("read message");
        assert_eq!(decoded, message);
    }

    #[gpui::test]
    async fn test_raw_frame_forwarding_preserves_compression() {
        let message = Envelope {
            payload: Some(envelope::Payload::UpdateWorktree(UpdateWorktree {
                root_name: "abcdefg".repeat(20_000),
                ..Default::default()
            })),
            ..Default::default()
        };

        let mut writer = Cursor::new(Vec::new());
        let mut buffer = Vec::new();
        write_message(&mut writer, &mut buffer, message)
            .await
            .expect("write message");
        let encoded = writer.into_inner();

        let mut reader = Cursor::new(encoded.clone());
        read_frame_raw(&mut reader, &mut buffer)
            .await
            .expect("read raw frame");
        assert_eq!(buffer, encoded);

        let mut forwarded = Cursor::new(Vec::new());
        write_frame_raw(&mut forwarded, &buffer)
            .await
            .expect("write raw frame");
        assert_eq!(forwarded.into_inner(), encoded);
    }
}
