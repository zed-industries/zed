use std::fmt::{self, Debug, Display, Write as _};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;
use std::str;

#[derive(Copy, Clone)]
pub(crate) struct CStr<'a> {
    ptr: NonNull<u8>,
    marker: PhantomData<&'a [u8]>,
}

unsafe impl<'a> Send for CStr<'a> {}
unsafe impl<'a> Sync for CStr<'a> {}

impl<'a> CStr<'a> {
    pub fn from_bytes_with_nul(bytes: &'static [u8]) -> Self {
        assert_eq!(bytes.last(), Some(&b'\0'));
        let ptr = NonNull::from(bytes).cast();
        unsafe { Self::from_ptr(ptr) }
    }

    pub unsafe fn from_ptr(ptr: NonNull<i8>) -> Self {
        CStr {
            ptr: ptr.cast(),
            marker: PhantomData,
        }
    }

    pub fn len(self) -> usize {
        let start = self.ptr.as_ptr();
        let mut end = start;
        unsafe {
            while *end != 0 {
                end = end.add(1);
            }
            end.offset_from(start) as usize
        }
    }

    pub fn to_bytes(self) -> &'a [u8] {
        let len = self.len();
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), len) }
    }
}

impl<'a> Display for CStr<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let ptr = self.ptr.as_ptr();
        let len = self.len();
        let bytes = unsafe { slice::from_raw_parts(ptr, len) };
        display_lossy(bytes, formatter)
    }
}

impl<'a> Debug for CStr<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let ptr = self.ptr.as_ptr();
        let len = self.len();
        let bytes = unsafe { slice::from_raw_parts(ptr, len) };
        debug_lossy(bytes, formatter)
    }
}

fn display_lossy(mut bytes: &[u8], formatter: &mut fmt::Formatter) -> fmt::Result {
    loop {
        match str::from_utf8(bytes) {
            Ok(valid) => return formatter.write_str(valid),
            Err(utf8_error) => {
                let valid_up_to = utf8_error.valid_up_to();
                let valid = unsafe { str::from_utf8_unchecked(&bytes[..valid_up_to]) };
                formatter.write_str(valid)?;
                formatter.write_char(char::REPLACEMENT_CHARACTER)?;
                if let Some(error_len) = utf8_error.error_len() {
                    bytes = &bytes[valid_up_to + error_len..];
                } else {
                    return Ok(());
                }
            }
        }
    }
}

pub(crate) fn debug_lossy(mut bytes: &[u8], formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_char('"')?;

    while !bytes.is_empty() {
        let from_utf8_result = str::from_utf8(bytes);
        let valid = match from_utf8_result {
            Ok(valid) => valid,
            Err(utf8_error) => {
                let valid_up_to = utf8_error.valid_up_to();
                unsafe { str::from_utf8_unchecked(&bytes[..valid_up_to]) }
            }
        };

        let mut written = 0;
        for (i, ch) in valid.char_indices() {
            let esc = ch.escape_debug();
            if esc.len() != 1 && ch != '\'' {
                formatter.write_str(&valid[written..i])?;
                for ch in esc {
                    formatter.write_char(ch)?;
                }
                written = i + ch.len_utf8();
            }
        }
        formatter.write_str(&valid[written..])?;

        match from_utf8_result {
            Ok(_valid) => break,
            Err(utf8_error) => {
                let end_of_broken = if let Some(error_len) = utf8_error.error_len() {
                    valid.len() + error_len
                } else {
                    bytes.len()
                };
                for b in &bytes[valid.len()..end_of_broken] {
                    write!(formatter, "\\x{:02x}", b)?;
                }
                bytes = &bytes[end_of_broken..];
            }
        }
    }

    formatter.write_char('"')
}
