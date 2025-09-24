use anyhow::anyhow;

/// Helper functions for encoding `Int64`s as `String`s.
pub enum JsonInteger {}

impl JsonInteger {
    /// Encode an integer as a string.
    pub fn encode(n: i64) -> String {
        base64::encode(n.to_le_bytes())
    }

    /// Decode an integer from a string.
    pub fn decode(s: String) -> anyhow::Result<i64> {
        let bytes: [u8; 8] = base64::decode(s.as_bytes())?
            .try_into()
            .map_err(|_| anyhow!("Int64 must be exactly eight bytes"))?;
        Ok(i64::from_le_bytes(bytes))
    }
}
