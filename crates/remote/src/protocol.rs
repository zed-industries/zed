use anyhow::{Context, Result};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use prost::Message as _;
use rpc::proto::Envelope;
use std::mem::size_of;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MessageId(pub u32);

pub type MessageLen = u32;
pub const MESSAGE_LEN_SIZE: usize = size_of::<MessageLen>();

pub fn message_len_from_buffer(buffer: &[u8]) -> MessageLen {
    MessageLen::from_le_bytes(buffer.try_into().unwrap())
}

pub fn write_to_global_log(message: &str) {
    use std::io::Write;
    let mut file =
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/Users/thorstenball/mylog.log".to_string())
            .expect("Failed to open log file");
        writeln!(file, "{}", message).expect("Failed to write to log file");
}

pub async fn read_message_with_len<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    message_len: MessageLen,
) -> Result<Envelope> {
    buffer.resize(message_len as usize, 0);
    stream.read_exact(buffer).await.context("read exact failed")?;
    Ok(Envelope::decode(buffer.as_slice()).context("decode failed")?)
}

pub async fn read_message<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
) -> Result<Envelope> {
    buffer.resize(MESSAGE_LEN_SIZE, 0);
    stream.read_exact(buffer).await.context("read exact failed")?;
    // log::debug!("read_exact DONE");
    let len = message_len_from_buffer(buffer);
    // log::debug!("received message_len_from_buffer: {}", len);
    let s = read_message_with_len(stream, buffer, len).await.with_context(|| format!("read message with len={} failed", len))?;
    // log::debug!("received message with len: {}", len);
    Ok(s)
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

pub async fn write_message_log<S: AsyncWrite + Unpin>(
    stream: &mut S,
    buffer: &mut Vec<u8>,
    message: Envelope,
) -> Result<()> {
    let message_len = message.encoded_len() as u32;
    log::debug!("write_message_log. message_len: {}", message_len);
    stream
        .write_all(message_len.to_le_bytes().as_slice())
        .await.context("failed to write message_len")?;
    buffer.clear();
    buffer.reserve(message_len as usize);
    message.encode(buffer).context("Failed to encode message")?;
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
    log::debug!("read_message_raw. message_len: {}", message_len);
    buffer.resize(message_len as usize, 0);
    stream.read_exact(buffer).await?;

    Ok(())
}
