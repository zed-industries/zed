#[derive(PartialEq, Eq, Copy, Debug, Clone)]
/// The endianness (byte order) of a stream of bytes
pub enum Endian {
    Little,
    Big,
}

/// Little Endian byte order context
pub const LE: Endian = Endian::Little;
/// Big Endian byte order context
pub const BE: Endian = Endian::Big;
/// Network byte order context
pub const NETWORK: Endian = Endian::Big;
#[cfg(target_endian = "little")]
/// The machine's native byte order
pub const NATIVE: Endian = LE;
#[cfg(target_endian = "big")]
/// The machine's native byte order
pub const NATIVE: Endian = BE;

impl Default for Endian {
    #[inline]
    fn default() -> Self {
        NATIVE
    }
}

impl From<bool> for Endian {
    #[inline]
    fn from(little_endian: bool) -> Self {
        if little_endian {
            LE
        } else {
            BE
        }
    }
}

impl Endian {
    #[inline]
    pub fn network() -> Endian {
        NETWORK
    }
    #[inline]
    pub fn is_little(&self) -> bool {
        *self == LE
    }
}
