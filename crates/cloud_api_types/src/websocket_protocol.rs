use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};

/// The version of the Cloud WebSocket protocol.
pub const PROTOCOL_VERSION: u32 = 0;

/// The name of the header used to indicate the protocol version in use.
pub const PROTOCOL_VERSION_HEADER_NAME: &str = "x-zed-protocol-version";

/// A message from Cloud to the Zed client.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageToClient {
    /// The user was updated and should be refreshed.
    UserUpdated,
    /// The user's notifications were updated.
    NotificationsUpdated,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notifications_updated_message_round_trips() -> Result<()> {
        let message = MessageToClient::NotificationsUpdated;

        assert_eq!(
            MessageToClient::deserialize(&message.serialize()?)?,
            message
        );

        Ok(())
    }

    #[test]
    fn notifications_updated_message_uses_the_expected_wire_format() -> Result<()> {
        assert_eq!(
            MessageToClient::NotificationsUpdated.serialize()?,
            b"\x74NotificationsUpdated"
        );

        Ok(())
    }
}
