use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// A byte slice.
///
/// Uses copy-on-write to avoid unnecessary allocations. The bytes can be
/// accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the
/// `to_mut` method.
///
/// Provides a `Debug` implementation that shows the first 8 bytes and the length.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Bytes<'a>(Cow<'a, [u8]>);

impl<'a> Bytes<'a> {
    /// Acquire a mutable reference to the bytes.
    ///
    /// Clones the bytes if they are shared.
    pub fn to_mut(&mut self) -> &mut Vec<u8> {
        self.0.to_mut()
    }

    /// Get the bytes as a slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<'a> core::ops::Deref for Bytes<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.0.deref()
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Bytes(Cow::Borrowed(bytes))
    }
}

impl<'a> From<Vec<u8>> for Bytes<'a> {
    fn from(bytes: Vec<u8>) -> Self {
        Bytes(Cow::Owned(bytes))
    }
}

impl<'a> fmt::Debug for Bytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_list_bytes(&self.0, f)
    }
}

// Only for Debug impl of `Bytes`.
fn debug_list_bytes(bytes: &[u8], fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut list = fmt.debug_list();
    list.entries(bytes.iter().take(8).copied().map(DebugByte));
    if bytes.len() > 8 {
        list.entry(&DebugLen(bytes.len()));
    }
    list.finish()
}

struct DebugByte(u8);

impl fmt::Debug for DebugByte {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "0x{:02x}", self.0)
    }
}

struct DebugLen(usize);

impl fmt::Debug for DebugLen {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "...; {}", self.0)
    }
}

/// A byte slice that is a string of an unknown encoding.
///
/// Uses copy-on-write to avoid unnecessary allocations. The bytes can be
/// accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the
/// `to_mut` method.
///
/// Provides a `Debug` implementation that interprets the bytes as UTF-8.
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct ByteString<'a>(Cow<'a, [u8]>);

impl<'a> ByteString<'a> {
    /// Acquire a mutable reference to the bytes.
    ///
    /// Clones the bytes if they are shared.
    pub fn to_mut(&mut self) -> &mut Vec<u8> {
        self.0.to_mut()
    }

    /// Get the bytes as a slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<'a> core::borrow::Borrow<[u8]> for ByteString<'a> {
    fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
}

impl<'a> core::ops::Deref for ByteString<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.0.deref()
    }
}

impl<'a> From<&'a [u8]> for ByteString<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        ByteString(Cow::Borrowed(bytes))
    }
}

impl<'a> From<Vec<u8>> for ByteString<'a> {
    fn from(bytes: Vec<u8>) -> Self {
        ByteString(Cow::Owned(bytes))
    }
}

impl<'a> From<&'a str> for ByteString<'a> {
    fn from(s: &'a str) -> Self {
        ByteString(Cow::Borrowed(s.as_bytes()))
    }
}

impl<'a> fmt::Debug for ByteString<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "\"{}\"", String::from_utf8_lossy(&self.0))
    }
}

impl<'a> fmt::Display for ByteString<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", String::from_utf8_lossy(&self.0))
    }
}
