use super::{Context, Keysym};
use crate::xkb::ffi::compose::*;
use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsStr;
use std::mem;
use std::str;

pub type CompileFlags = u32;
pub const COMPILE_NO_FLAGS: CompileFlags = 0;

pub type Format = u32;
pub const FORMAT_TEXT_V1: Format = 1;

pub type StateFlags = u32;
pub const STATE_NO_FLAGS: StateFlags = 0;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum Status {
    Nothing = 0,
    Composing = 1,
    Composed,
    Cancelled,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum FeedResult {
    Ignored,
    Accepted,
}

pub struct Table {
    ptr: *mut xkb_compose_table,
}

impl Table {
    /// Build a table from a locale.
    /// The locale is typically obtained from environment variables.
    ///
    /// # Panics
    /// May panic if the locale contain inner null characters.
    #[allow(clippy::result_unit_err, clippy::missing_errors_doc)]
    pub fn new_from_locale(
        context: &Context,
        locale: &OsStr,
        flags: CompileFlags,
    ) -> Result<Table, ()> {
        use std::os::unix::ffi::OsStrExt;

        let locale_cstr = CStr::from_bytes_with_nul(locale.as_bytes());
        let locale_cstr = match locale_cstr {
            Ok(loc) => Cow::from(loc),
            Err(_) => Cow::from(CString::new(locale.as_bytes().to_vec()).unwrap()),
        };

        let ptr = unsafe {
            xkb_compose_table_new_from_locale(context.get_raw_ptr(), locale_cstr.as_ptr(), flags)
        };
        if ptr.is_null() {
            Err(())
        } else {
            Ok(Table { ptr })
        }
    }

    #[allow(
        clippy::result_unit_err,
        clippy::missing_panics_doc,
        clippy::missing_errors_doc
    )]
    pub fn new_from_buffer<T: AsRef<[u8]>>(
        context: &Context,
        buffer: T,
        locale: &str,
        format: Format,
        flags: CompileFlags,
    ) -> Result<Table, ()> {
        let buffer = buffer.as_ref();
        let locale = CString::new(locale).unwrap();
        let ptr = unsafe {
            xkb_compose_table_new_from_buffer(
                context.get_raw_ptr(),
                buffer.as_ptr().cast(),
                buffer.len() as _,
                locale.as_ptr(),
                format,
                flags,
            )
        };
        if ptr.is_null() {
            Err(())
        } else {
            Ok(Table { ptr })
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        unsafe {
            xkb_compose_table_unref(self.ptr);
        }
    }
}

impl Clone for Table {
    fn clone(&self) -> Table {
        Table {
            ptr: unsafe { xkb_compose_table_ref(self.ptr) },
        }
    }
}

pub struct State {
    ptr: *mut xkb_compose_state,
}

impl State {
    /// # Safety
    /// `ptr` must be a valid pointer to `xkb_compose_state`
    #[must_use]
    pub unsafe fn from_raw_ptr(ptr: *mut xkb_compose_state) -> State {
        State { ptr }
    }

    pub fn get_raw_ptr(&self) -> *mut xkb_compose_state {
        self.ptr
    }

    #[must_use]
    pub fn new(table: &Table, flags: StateFlags) -> State {
        State {
            ptr: unsafe { xkb_compose_state_new(table.ptr, flags) },
        }
    }

    #[must_use]
    pub fn compose_table(&self) -> Table {
        Table {
            ptr: unsafe { xkb_compose_table_ref(xkb_compose_state_get_compose_table(self.ptr)) },
        }
    }

    pub fn feed(&mut self, keysym: Keysym) -> FeedResult {
        unsafe { mem::transmute(xkb_compose_state_feed(self.ptr, keysym.raw())) }
    }

    pub fn reset(&mut self) {
        unsafe {
            xkb_compose_state_reset(self.ptr);
        }
    }

    #[must_use]
    pub fn status(&self) -> Status {
        unsafe { mem::transmute(xkb_compose_state_get_status(self.ptr)) }
    }

    #[must_use]
    pub fn utf8(&self) -> Option<String> {
        let mut buffer = [0_u8; 256];

        unsafe {
            match xkb_compose_state_get_utf8(self.ptr, buffer.as_mut_ptr().cast(), buffer.len()) {
                0 => None,
                n => Some(str::from_utf8_unchecked(&buffer[..n as usize]).into()),
            }
        }
    }

    #[must_use]
    pub fn keysym(&self) -> Option<Keysym> {
        unsafe {
            match Keysym::new(xkb_compose_state_get_one_sym(self.ptr)) {
                xkeysym::NO_SYMBOL => None,
                value => Some(value),
            }
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            xkb_compose_state_unref(self.ptr);
        }
    }
}

impl Clone for State {
    fn clone(&self) -> State {
        State {
            ptr: unsafe { xkb_compose_state_ref(self.ptr) },
        }
    }
}
