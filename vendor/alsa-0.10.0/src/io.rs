use crate::alsa;
use super::error::*;
use std::{slice, ptr, fmt};

/// [snd_output_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___output.html) wrapper
pub struct Output(*mut alsa::snd_output_t);

unsafe impl Send for Output {}

impl Drop for Output {
    fn drop(&mut self) { unsafe { alsa::snd_output_close(self.0) }; }
}

impl Output {

    pub fn buffer_open() -> Result<Output> {
        let mut q = ptr::null_mut();
        acheck!(snd_output_buffer_open(&mut q)).map(|_| Output(q))
    }

    pub fn buffer_string<T, F: FnOnce(&[u8]) -> T>(&self, f: F) -> T {
        let b = unsafe {
            let mut q = ptr::null_mut();
            let s = alsa::snd_output_buffer_string(self.0, &mut q);
            if s == 0 { &[] } else { slice::from_raw_parts(q as *const u8, s as usize) }
        };
        f(b)
    }
}

impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Output(")?;
        fmt::Display::fmt(self, f)?;
        write!(f, ")")
        /* self.buffer_string(|b| f.write_str(try!(str::from_utf8(b).map_err(|_| fmt::Error)))) */
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.buffer_string(|b| {
            let s = String::from_utf8_lossy(b);
            f.write_str(&*s)
        })
    }
}

pub fn output_handle(o: &Output) -> *mut alsa::snd_output_t { o.0 }
