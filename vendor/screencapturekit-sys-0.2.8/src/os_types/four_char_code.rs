use std::{
    ascii,
    fmt::{self},
};

#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FourCharCode(u32);

impl fmt::Display for FourCharCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format())
    }
}

impl fmt::Debug for FourCharCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"")?;
        f.write_str(&self.format())?;
        write!(f, "\"")
    }
}

impl FourCharCode {
    #[inline]
    fn format(&self) -> String {
        // Format as escaped ASCII string.

        let raw = self
            .into_chars()
            .into_iter()
            .flat_map(ascii::escape_default)
            .collect::<Vec<u8>>();

        String::from_utf8(raw).unwrap()
    }
    /// Returns an instance from the integer value.
    #[inline]
    pub const fn from_int(int: u32) -> Self {
        Self(int)
    }

    /// Returns an instance from the 4-character code.
    #[inline]
    pub const fn from_chars(chars: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(chars))
    }

    /// Returns this descriptor's integer value.
    #[inline]
    pub const fn into_int(self) -> u32 {
        self.0
    }

    /// Returns this descriptor's 4-character code.
    #[inline]
    pub const fn into_chars(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    /// Returns `true` if all of the characters in `self` are ASCII.
    #[inline]
    pub const fn is_ascii(&self) -> bool {
        const NON_ASCII: u32 = u32::from_be_bytes([128; 4]);

        self.0 & NON_ASCII == 0
    }

    /// Returns `true` if all of the characters in `self` are ASCII graphic
    /// characters: U+0021 '!' ..= U+007E '~'.
    #[inline]
    pub const fn is_ascii_graphic(&self) -> bool {
        matches!(
            self.into_chars(),
            [b'!'..=b'~', b'!'..=b'~', b'!'..=b'~', b'!'..=b'~'],
        )
    }
}
