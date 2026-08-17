use std::fmt::{self, Display, Write as _};
use std::slice;
use std::str;

#[allow(non_camel_case_types)]
type c_char = i8;

pub struct CStr {
    ptr: *const u8,
}

impl CStr {
    pub unsafe fn from_ptr(ptr: *const c_char) -> Self {
        CStr { ptr: ptr.cast() }
    }
}

impl Display for CStr {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let len = unsafe { strlen(self.ptr) };
        let mut bytes = unsafe { slice::from_raw_parts(self.ptr, len) };
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
}

unsafe fn strlen(s: *const u8) -> usize {
    let mut end = s;
    while *end != 0 {
        end = end.add(1);
    }
    end.offset_from(s) as usize
}
