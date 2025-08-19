use anyhow::Result;
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use prost::Message as _;
use rpc::proto::Envelope;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MessageId(pub u32);

pub type MessageLen = u32;
pub const MESSAGE_LEN_SIZE: usize = size_of::<MessageLen>();

pub fn message_len_from_buffer(buffer: &[u8]) -> MessageLen {
    MessageLen::from_le_bytes(buffer.try_into().unwrap())
}

pub async fn read_message_with_len<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    message_len: MessageLen,
) -> Result<Envelope> {
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

    let len = message_len_from_buffer(buffer);

    read_message_with_len(stream, buffer, len).await
}

pub async fn write_message<S: AsyncWrite + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    message: Envelope,
) -> Result<()> {
    let message_len = message.encoded_len() as u32;
    stream
        .write_all(message_len.to_le_bytes().as_slice())
        .await?;
    buffer.clear();
    buffer.reserve(message_len as usize);
    message.encode(buffer)?;
    stream.write_all(buffer).await?;
    Ok(())
}

pub async fn read_message_raw<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<()> {
    buffer.resize(MESSAGE_LEN_SIZE, 0);
    stream.read_exact(buffer).await?;

    let message_len = message_len_from_buffer(buffer);
    buffer.resize(message_len as usize, 0);
    stream.read_exact(buffer).await?;

    Ok(())
}
