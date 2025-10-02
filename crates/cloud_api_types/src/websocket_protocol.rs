use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};

/// The version of the Cloud WebSocket protocol.
pub const PROTOCOL_VERSION: u32 = 0;

/// The name of the header used to indicate the protocol version in use.
pub const PROTOCOL_VERSION_HEADER_NAME: &str = "x-zed-protocol-version";

/// A message from Cloud to the Zed client.
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageToClient {
    /// The user was updated and should be refreshed.
    UserUpdated,
}

impl MessageToClient {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        ciborium::into_writer(self, &mut buffer).context("failed to serialize message")?;

        Ok(buffer)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        ciborium::from_reader(data).context("failed to deserialize message")
    }
}
