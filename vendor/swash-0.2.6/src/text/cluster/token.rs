use super::{CharInfo, UserData};

/// Character input to the cluster parser.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Token {
    /// The character.
    pub ch: char,
    /// Offset of the character in code units.
    pub offset: u32,
    /// Length of the character in code units.
    pub len: u8,
    /// Character information.
    pub info: CharInfo,
    /// Arbitrary user data.
    pub data: UserData,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            ch: '\0',
            offset: 0,
            len: 1,
            info: Default::default(),
            data: 0,
        }
    }
}
