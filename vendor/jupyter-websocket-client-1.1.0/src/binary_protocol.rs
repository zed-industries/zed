//! Jupyter Kernel WebSocket v1 Binary Protocol
//!
//! This module implements the binary protocol used by jupyter-server for kernel communication.
//! The protocol is identified by the subprotocol: `v1.kernel.websocket.jupyter.org`
//!
//! ## Message Format
//!
//! ```text
//! [8 bytes: offset_count as little-endian u64]
//! [8 * offset_count bytes: offset array, each little-endian u64]
//! [channel name (UTF-8 bytes)]
//! [header JSON bytes]
//! [parent_header JSON bytes]
//! [metadata JSON bytes]
//! [content JSON bytes]
//! [optional: binary buffers]
//! ```
//!
//! Each offset points to the start of a section from the beginning of the buffer.

use anyhow::{Context, Result};
use jupyter_protocol::{Channel, JupyterMessage};
use serde_json::Value;

/// The WebSocket subprotocol for binary kernel messages
pub const KERNEL_WEBSOCKET_PROTOCOL: &str = "v1.kernel.websocket.jupyter.org";

/// Serialize a JupyterMessage to v1 binary format
pub fn serialize_v1(msg: &JupyterMessage, channel: &str) -> Result<Vec<u8>> {
    // Serialize each part to JSON bytes
    let channel_bytes = channel.as_bytes();
    let header_bytes = serde_json::to_vec(&msg.header).context("Failed to serialize header")?;
    let parent_header_bytes = match &msg.parent_header {
        Some(h) => serde_json::to_vec(h).context("Failed to serialize parent_header")?,
        None => b"{}".to_vec(),
    };
    let metadata_bytes =
        serde_json::to_vec(&msg.metadata).context("Failed to serialize metadata")?;
    let content_bytes = serde_json::to_vec(&msg.content).context("Failed to serialize content")?;

    // Collect all message parts
    let msg_list: Vec<&[u8]> = vec![
        channel_bytes,
        &header_bytes,
        &parent_header_bytes,
        &metadata_bytes,
        &content_bytes,
    ];

    // Add buffer parts if present
    let buffer_refs: Vec<&[u8]> = msg.buffers.iter().map(|b| b.as_ref()).collect();
    let all_parts: Vec<&[u8]> = msg_list
        .iter()
        .chain(buffer_refs.iter())
        .copied()
        .collect();

    // Calculate offsets
    // offset_count = number of parts + 1 (for the end marker)
    let offset_count = all_parts.len() + 1;
    let header_size = 8 * (1 + offset_count); // 8 bytes for count + 8 bytes per offset

    let mut offsets = Vec::with_capacity(offset_count);
    let mut current_offset = header_size;

    // First offset points to start of first part (right after header)
    offsets.push(current_offset as u64);

    // Each subsequent offset points to start of next part
    for part in &all_parts {
        current_offset += part.len();
        offsets.push(current_offset as u64);
    }

    // Build the final buffer
    let total_size = current_offset;
    let mut buffer = Vec::with_capacity(total_size);

    // Write offset count
    buffer.extend_from_slice(&(offset_count as u64).to_le_bytes());

    // Write offsets
    for offset in &offsets {
        buffer.extend_from_slice(&offset.to_le_bytes());
    }

    // Write all parts
    for part in &all_parts {
        buffer.extend_from_slice(part);
    }

    Ok(buffer)
}

/// Deserialize v1 binary format to (channel, JupyterMessage)
pub fn deserialize_v1(data: &[u8]) -> Result<(String, JupyterMessage)> {
    if data.len() < 8 {
        anyhow::bail!("Message too short: need at least 8 bytes for offset count");
    }

    // Read offset count
    let offset_count = u64::from_le_bytes(data[0..8].try_into().unwrap()) as usize;

    let header_size = 8 * (1 + offset_count);
    if data.len() < header_size {
        anyhow::bail!(
            "Message too short: need {} bytes for header, got {}",
            header_size,
            data.len()
        );
    }

    // Read offsets
    let mut offsets = Vec::with_capacity(offset_count);
    for i in 0..offset_count {
        let start = 8 + i * 8;
        let offset = u64::from_le_bytes(data[start..start + 8].try_into().unwrap()) as usize;
        offsets.push(offset);
    }

    // We need at least 5 parts: channel, header, parent_header, metadata, content
    if offset_count < 6 {
        anyhow::bail!(
            "Message has too few parts: need at least 6 offsets, got {}",
            offset_count
        );
    }

    // Extract parts using offsets
    let get_part = |idx: usize| -> Result<&[u8]> {
        let start = offsets[idx];
        let end = offsets[idx + 1];
        if start > data.len() || end > data.len() || start > end {
            anyhow::bail!(
                "Invalid offset range: {}..{} for data length {}",
                start,
                end,
                data.len()
            );
        }
        Ok(&data[start..end])
    };

    // Parse channel
    let channel_bytes = get_part(0)?;
    let channel_str =
        std::str::from_utf8(channel_bytes).context("Channel is not valid UTF-8")?;

    // Parse header
    let header_bytes = get_part(1)?;
    let header: jupyter_protocol::Header =
        serde_json::from_slice(header_bytes).context("Failed to parse header")?;

    // Parse parent_header
    let parent_header_bytes = get_part(2)?;
    let parent_header: Option<jupyter_protocol::Header> = if parent_header_bytes == b"{}" {
        None
    } else {
        Some(serde_json::from_slice(parent_header_bytes).context("Failed to parse parent_header")?)
    };

    // Parse metadata
    let metadata_bytes = get_part(3)?;
    let metadata: Value =
        serde_json::from_slice(metadata_bytes).context("Failed to parse metadata")?;

    // Parse content
    let content_bytes = get_part(4)?;
    let content_value: Value =
        serde_json::from_slice(content_bytes).context("Failed to parse content")?;

    // Get the msg_type from header to parse content correctly
    let msg_type = &header.msg_type;
    let content = jupyter_protocol::JupyterMessageContent::from_type_and_content(
        msg_type,
        content_value,
    )
    .context("Failed to parse message content")?;

    // Extract any binary buffers (parts after the first 5)
    let mut buffers = Vec::new();
    for i in 5..(offset_count - 1) {
        let buffer_bytes = get_part(i)?;
        buffers.push(bytes::Bytes::copy_from_slice(buffer_bytes));
    }

    // Convert channel string to Channel enum
    let channel = match channel_str {
        "shell" => Some(Channel::Shell),
        "control" => Some(Channel::Control),
        "stdin" => Some(Channel::Stdin),
        "iopub" => Some(Channel::IOPub),
        "heartbeat" => Some(Channel::Heartbeat),
        _ => None,
    };

    let message = JupyterMessage {
        zmq_identities: Vec::new(),
        header,
        parent_header,
        metadata,
        content,
        buffers,
        channel,
    };

    Ok((channel_str.to_string(), message))
}

/// Get the channel name for a message being sent
///
/// Messages should be sent on 'shell' channel by default for requests,
/// unless it's a control message (shutdown, interrupt).
pub fn default_channel_for_message(msg: &JupyterMessage) -> &'static str {
    use jupyter_protocol::JupyterMessageContent;
    match &msg.content {
        JupyterMessageContent::ShutdownRequest(_) => "control",
        JupyterMessageContent::InterruptRequest(_) => "control",
        JupyterMessageContent::InterruptReply(_) => "control",
        JupyterMessageContent::ShutdownReply(_) => "control",
        JupyterMessageContent::DebugRequest(_) => "control",
        JupyterMessageContent::DebugReply(_) => "control",
        _ => "shell",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jupyter_protocol::ExecuteRequest;

    #[test]
    fn test_roundtrip() {
        let msg = JupyterMessage::new(
            ExecuteRequest {
                code: "print('hello')".to_string(),
                silent: false,
                store_history: true,
                user_expressions: Default::default(),
                allow_stdin: false,
                stop_on_error: true,
            },
            None,
        );

        let channel = "shell";
        let serialized = serialize_v1(&msg, channel).expect("serialize failed");
        let (parsed_channel, parsed_msg) =
            deserialize_v1(&serialized).expect("deserialize failed");

        assert_eq!(parsed_channel, channel);
        assert_eq!(parsed_msg.header.msg_type, msg.header.msg_type);
        assert_eq!(parsed_msg.header.session, msg.header.session);
    }

    #[test]
    fn test_offset_format() {
        let msg = JupyterMessage::new(
            ExecuteRequest {
                code: "x".to_string(),
                silent: false,
                store_history: true,
                user_expressions: Default::default(),
                allow_stdin: false,
                stop_on_error: true,
            },
            None,
        );

        let serialized = serialize_v1(&msg, "shell").expect("serialize failed");

        // Check offset count is 6 (channel, header, parent_header, metadata, content, end)
        let offset_count = u64::from_le_bytes(serialized[0..8].try_into().unwrap());
        assert_eq!(offset_count, 6);

        // First offset should point past the header (8 * 7 = 56 bytes)
        let first_offset = u64::from_le_bytes(serialized[8..16].try_into().unwrap());
        assert_eq!(first_offset, 56);
    }

    #[test]
    fn test_channel_parsing() {
        let msg = JupyterMessage::new(
            ExecuteRequest {
                code: "x".to_string(),
                silent: false,
                store_history: true,
                user_expressions: Default::default(),
                allow_stdin: false,
                stop_on_error: true,
            },
            None,
        );

        // Test shell channel
        let serialized = serialize_v1(&msg, "shell").unwrap();
        let (channel, parsed) = deserialize_v1(&serialized).unwrap();
        assert_eq!(channel, "shell");
        assert!(matches!(parsed.channel, Some(Channel::Shell)));

        // Test iopub channel
        let serialized = serialize_v1(&msg, "iopub").unwrap();
        let (channel, parsed) = deserialize_v1(&serialized).unwrap();
        assert_eq!(channel, "iopub");
        assert!(matches!(parsed.channel, Some(Channel::IOPub)));
    }
}
