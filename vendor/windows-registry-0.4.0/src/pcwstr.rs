use super::*;

pub struct OwnedPcwstr(Vec<u16>);

pub fn pcwstr<T: AsRef<str>>(value: T) -> OwnedPcwstr {
    OwnedPcwstr(
        value
            .as_ref()
            .encode_utf16()
            .chain(core::iter::once(0))
            .collect(),
    )
}

pub fn multi_pcwstr<T: AsRef<str>>(value: &[T]) -> OwnedPcwstr {
    OwnedPcwstr(
        value
            .iter()
            .flat_map(|value| value.as_ref().encode_utf16().chain(core::iter::once(0)))
            .chain(core::iter::once(0))
            .collect(),
    )
}

impl OwnedPcwstr {
    pub fn as_ptr(&self) -> *const u16 {
        debug_assert!(
            self.0.last() == Some(&0),
            "`OwnedPcwstr` isn't null-terminated"
        );
        self.0.as_ptr()
    }

    // Get the string as 8-bit bytes including the two terminating null bytes.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.as_ptr() as *const _, self.0.len() * 2) }
    }

    pub fn as_raw(&self) -> PCWSTR {
        PCWSTR(self.as_ptr())
    }
}
