use anyhow::Result;
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use prost::Message as _;
use rpc::proto::Envelope;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MessageId(pub u32);

pub type MessageLen = u32;
pub const MESSAGE_LEN_SIZE: usize = size_of::<MessageLen>();

/// Maximum size accepted for a single inbound envelope. Anything larger is
/// rejected before allocating, to prevent OOMs from malformed or hostile
/// streams. Bulk payloads should use chunked transfer (e.g. file uploads).
pub const MAX_INCOMING_MESSAGE_SIZE: usize = 8 * 1024 * 1024;

/// Converts an encoded message length into a `MessageLen`, rejecting any
/// length that exceeds `u32::MAX` so we never silently truncate at the wire.
pub fn message_len_for(encoded_len: usize) -> Result<MessageLen> {
    MessageLen::try_from(encoded_len).map_err(|_| {
        anyhow::anyhow!(
            "remote envelope too large: {encoded_len} bytes exceeds u32::MAX; \
             use chunked transfer for payloads of this size"
        )
    })
}

pub fn message_len_from_buffer(buffer: [u8; MESSAGE_LEN_SIZE]) -> MessageLen {
    MessageLen::from_le_bytes(buffer)
}

pub async fn read_message_with_len<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    message_len: MessageLen,
) -> Result<Envelope> {
    anyhow::ensure!(
        (message_len as usize) <= MAX_INCOMING_MESSAGE_SIZE,
        "incoming envelope too large: {message_len} bytes exceeds \
         MAX_INCOMING_MESSAGE_SIZE ({MAX_INCOMING_MESSAGE_SIZE})"
    );
    buffer.resize(message_len as usize, 0);
    stream.read_exact(buffer).await?;
    Ok(Envelope::decode(buffer.as_slice())?)
}

pub async fn read_message<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<Envelope> {
    buffer.resize(MESSAGE_LEN_SIZE, 0);
    stream.read_exact(buffer).await?;

    let mut header = [0u8; MESSAGE_LEN_SIZE];
    header.copy_from_slice(&buffer[..MESSAGE_LEN_SIZE]);
    let len = message_len_from_buffer(header);

    read_message_with_len(stream, buffer, len).await
}

pub async fn write_message<S: AsyncWrite + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    message: Envelope,
) -> Result<()> {
    let encoded_len = message.encoded_len();
    let message_len = message_len_for(encoded_len)?;
    stream
        .write_all(message_len.to_le_bytes().as_slice())
        .await?;
    buffer.clear();
    buffer.reserve(message_len as usize);
    message.encode(buffer)?;
    stream.write_all(buffer).await?;
    Ok(())
}

pub async fn write_size_prefixed_buffer<S: AsyncWrite + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<()> {
    let len = message_len_for(buffer.len())?;
    stream.write_all(len.to_le_bytes().as_slice()).await?;
    stream.write_all(buffer).await?;
    Ok(())
}

pub async fn read_message_raw<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<()> {
    buffer.resize(MESSAGE_LEN_SIZE, 0);
    stream.read_exact(buffer).await?;

    let mut header = [0u8; MESSAGE_LEN_SIZE];
    header.copy_from_slice(&buffer[..MESSAGE_LEN_SIZE]);
    let message_len = message_len_from_buffer(header);
    anyhow::ensure!(
        (message_len as usize) <= MAX_INCOMING_MESSAGE_SIZE,
        "incoming envelope too large: {message_len} bytes exceeds \
         MAX_INCOMING_MESSAGE_SIZE ({MAX_INCOMING_MESSAGE_SIZE})"
    );
    buffer.resize(message_len as usize, 0);
    stream.read_exact(buffer).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::Cursor;
    use rpc::proto::{self, EnvelopedMessage as _};

    #[test]
    fn message_len_for_accepts_u32_max() {
        let result = message_len_for(u32::MAX as usize);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), u32::MAX);
    }

    #[test]
    fn message_len_for_rejects_just_past_u32_max() {
        let oversized = u32::MAX as usize + 1;
        let result = message_len_for(oversized);
        assert!(result.is_err());
        assert!(
            format!("{:#}", result.unwrap_err()).contains("u32::MAX"),
            "expected error to mention u32::MAX"
        );
    }

    #[gpui::test]
    async fn read_message_rejects_envelope_over_max() {
        let mut stream = Vec::new();
        let oversized = (MAX_INCOMING_MESSAGE_SIZE + 1) as u32;
        stream.extend_from_slice(&oversized.to_le_bytes());
        // No payload bytes — the size check should fire before any read.
        let mut cursor = Cursor::new(stream);
        let mut buffer = Vec::new();
        let result = read_message(&mut cursor, &mut buffer).await;
        assert!(result.is_err());
        let err = format!("{:#}", result.unwrap_err());
        assert!(
            err.contains("incoming envelope too large"),
            "expected oversize error, got: {err}"
        );
    }

    #[gpui::test]
    async fn read_message_raw_rejects_envelope_over_max() {
        let mut stream = Vec::new();
        let oversized = (MAX_INCOMING_MESSAGE_SIZE + 1) as u32;
        stream.extend_from_slice(&oversized.to_le_bytes());
        let mut cursor = Cursor::new(stream);
        let mut buffer = Vec::new();
        let result = read_message_raw(&mut cursor, &mut buffer).await;
        assert!(result.is_err());
        let err = format!("{:#}", result.unwrap_err());
        assert!(
            err.contains("incoming envelope too large"),
            "expected oversize error, got: {err}"
        );
    }

    #[gpui::test]
    async fn write_then_read_round_trip() {
        let mut wire = Vec::new();
        let mut buffer = Vec::new();

        let envelope = proto::Ping {}.into_envelope(7, None, None);
        write_message(&mut wire, &mut buffer, envelope.clone())
            .await
            .unwrap();

        let mut cursor = Cursor::new(wire);
        buffer.clear();
        let received = read_message(&mut cursor, &mut buffer).await.unwrap();
        assert_eq!(received.id, envelope.id);
        assert_eq!(received.payload, envelope.payload);
    }
}
