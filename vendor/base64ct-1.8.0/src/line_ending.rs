//! Line endings.

/// Carriage return
pub(crate) const CHAR_CR: u8 = 0x0d;

/// Line feed
pub(crate) const CHAR_LF: u8 = 0x0a;

/// Line endings: variants of newline characters that can be used with Base64.
///
/// Use [`LineEnding::default`] to get an appropriate line ending for the
/// current operating system.
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum LineEnding {
    /// Carriage return: `\r` (Pre-OS X Macintosh)
    CR,

    /// Line feed: `\n` (Unix OSes)
    LF,

    /// Carriage return + line feed: `\r\n` (Windows)
    CRLF,
}

impl Default for LineEnding {
    // Default line ending matches conventions for target OS
    #[cfg(windows)]
    fn default() -> LineEnding {
        LineEnding::CRLF
    }
    #[cfg(not(windows))]
    fn default() -> LineEnding {
        LineEnding::LF
    }
}

#[allow(clippy::len_without_is_empty)]
impl LineEnding {
    /// Get the byte serialization of this [`LineEnding`].
    pub fn as_bytes(self) -> &'static [u8] {
        match self {
            LineEnding::CR => &[CHAR_CR],
            LineEnding::LF => &[CHAR_LF],
            LineEnding::CRLF => &[CHAR_CR, CHAR_LF],
        }
    }

    /// Get the encoded length of this [`LineEnding`].
    pub fn len(self) -> usize {
        self.as_bytes().len()
    }
}
