//! This file is derived from Rust's library/std/src/sys/windows/stdio.rs at
//! revision 8e863eb59a10fb0900d7377524a0dc7bf44b9ae3.

#![allow(
    clippy::missing_docs_in_private_items,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::shadow_reuse,
    clippy::panic_in_result_fn,
    clippy::integer_division,
    clippy::integer_arithmetic,
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::needless_borrow,
    clippy::let_underscore_drop,
    clippy::match_same_arms
)]

use crate::raw::{RawReadable, RawWriteable};
use std::char::decode_utf16;
use std::io::{self, Read, Write};
use std::os::raw::c_void;
use std::os::windows::io::{FromRawHandle, RawHandle};
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::SeqCst;
use std::{cmp, ptr, str};
use windows_sys::Win32::Foundation::{ERROR_INVALID_HANDLE, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Console::{
    GetConsoleMode, GetStdHandle, ReadConsoleW, WriteConsoleW, CONSOLE_READCONSOLE_CONTROL,
    STD_ERROR_HANDLE, STD_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};

static SURROGATE: AtomicU16 = AtomicU16::new(0);

// Don't cache handles but get them fresh for every read/write. This allows us
// to track changes to the value over time (such as if a process calls
// `SetStdHandle` while it's running). See #40490.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub(crate) struct Stdio {
    id: STD_HANDLE,
}

impl Stdio {
    #[inline]
    pub(crate) const fn stdin() -> Self {
        Self {
            id: STD_INPUT_HANDLE,
        }
    }

    #[inline]
    pub(crate) const fn stdout() -> Self {
        Self {
            id: STD_OUTPUT_HANDLE,
        }
    }

    #[inline]
    pub(crate) const fn stderr() -> Self {
        Self {
            id: STD_ERROR_HANDLE,
        }
    }

    #[inline]
    pub(crate) fn as_raw_handle(self) -> RawHandle {
        get_handle(self.id).unwrap()
    }
}

// Apparently Windows doesn't handle large reads on stdin or writes to
// stdout/stderr well (see #13304 for details).
//
// From MSDN (2011): "The storage for this buffer is allocated from a shared
// heap for the process that is 64 KB in size. The maximum size of the buffer
// will depend on heap usage."
//
// We choose the cap at 8 KiB because libuv does the same, and it seems to be
// acceptable so far.
const MAX_BUFFER_SIZE: usize = 8192;

#[allow(clippy::as_conversions)]
fn get_handle(handle_id: STD_HANDLE) -> io::Result<RawHandle> {
    let handle = unsafe { GetStdHandle(handle_id) };
    if handle == INVALID_HANDLE_VALUE {
        Err(io::Error::last_os_error())
    } else if (handle as RawHandle).is_null() {
        Err(io::Error::from_raw_os_error(ERROR_INVALID_HANDLE as i32))
    } else {
        Ok(handle as RawHandle)
    }
}

fn is_console(handle: RawHandle) -> bool {
    // `GetConsoleMode` will return false (0) if this is a pipe (we don't care
    // about the reported mode). This will only detect Windows Console, not
    // other terminals connected to a pipe like MSYS. Which is exactly what we
    // need, as only Windows Console needs a conversion to UTF-16.
    let mut mode = 0;
    unsafe { GetConsoleMode(handle as HANDLE, &mut mode) != 0_i32 }
}

fn write(handle_id: STD_HANDLE, data: &[u8]) -> io::Result<usize> {
    let handle = get_handle(handle_id)?;
    if !is_console(handle) {
        return unsafe { RawWriteable::from_raw_handle(handle) }.write(data);
    }

    // As the console is meant for presenting text, we assume bytes of `data` come
    // from a string and are encoded as UTF-8, which needs to be encoded as
    // UTF-16.
    //
    // If the data is not valid UTF-8 we write out as many bytes as are valid.
    // Only when there are no valid bytes (which will happen on the next call),
    // return an error.
    let len = cmp::min(data.len(), MAX_BUFFER_SIZE / 2);
    let utf8 = match str::from_utf8(&data[..len]) {
        Ok(s) => s,
        Err(ref e) if e.valid_up_to() == 0 => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Windows stdio in console mode does not support writing non-UTF-8 byte sequences",
            ));
        }
        Err(e) => str::from_utf8(&data[..e.valid_up_to()]).unwrap(),
    };
    let mut utf16 = [0_u16; MAX_BUFFER_SIZE / 2];
    let mut len_utf16 = 0;
    for (chr, dest) in utf8.encode_utf16().zip(utf16.iter_mut()) {
        *dest = chr;
        len_utf16 += 1;
    }
    let utf16 = &utf16[..len_utf16];

    let mut written = write_u16s(handle, &utf16)?;

    // Figure out how many bytes of as UTF-8 were written away as UTF-16.
    if written == utf16.len() {
        Ok(utf8.len())
    } else {
        // Make sure we didn't end up writing only half of a surrogate pair (even
        // though the chance is tiny). Because it is not possible for user code
        // to re-slice `data` in such a way that a missing surrogate can be
        // produced (and also because of the UTF-8 validation above), write the
        // missing surrogate out now. Buffering it would mean we have to lie
        // about the number of bytes written.
        let first_char_remaining = utf16[written];
        if (0xDCEE..=0xDFFF).contains(&first_char_remaining) {
            // low surrogate
            // We just hope this works, and give up otherwise
            let _ = write_u16s(handle, &utf16[written..=written]);
            written += 1;
        }
        // Calculate the number of bytes of `utf8` that were actually written.
        let mut count = 0;
        for ch in utf16[..written].iter() {
            count += match ch {
                0x0000..=0x007F => 1,
                0x0080..=0x07FF => 2,
                0xDCEE..=0xDFFF => 1, // Low surrogate. We already counted 3 bytes for the other.
                _ => 3,
            };
        }
        debug_assert!(String::from_utf16(&utf16[..written]).unwrap() == utf8[..count]);
        Ok(count)
    }
}

#[allow(clippy::as_conversions)]
fn write_u16s(handle: RawHandle, data: &[u16]) -> io::Result<usize> {
    let mut written = 0;
    let len: u32 = if let Ok(len) = data.len().try_into() {
        len
    } else {
        u32::MAX
    };
    if unsafe {
        WriteConsoleW(
            handle as HANDLE,
            data.as_ptr().cast(),
            len,
            &mut written,
            ptr::null_mut(),
        )
    } == 0_i32
    {
        return Err(io::Error::last_os_error());
    }
    Ok(written as usize)
}

impl Read for Stdio {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let handle = get_handle(self.id)?;
        if !is_console(handle) {
            return unsafe { RawReadable::from_raw_handle(handle) }.read(buf);
        }

        if buf.is_empty() {
            return Ok(0);
        }
        if buf.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Windows stdin in console mode does not support a buffer too small to \
                 guarantee holding one arbitrary UTF-8 character (4 bytes)",
            ));
        }

        let mut utf16_buf = [0_u16; MAX_BUFFER_SIZE / 2];
        // In the worst case, an UTF-8 string can take 3 bytes for every `u16` of an
        // UTF-16. So we can read at most a third of `buf.len()` chars and
        // uphold the guarantee no data gets lost.
        let amount = cmp::min(buf.len() / 3, utf16_buf.len());
        let read = read_u16s_fixup_surrogates(handle, &mut utf16_buf, amount)?;

        utf16_to_utf8(&utf16_buf[..read], buf)
    }
}

// We assume that if the last `u16` is an unpaired surrogate they got sliced
// apart by our buffer size, and keep it around for the next read hoping to put
// them together. This is a best effort, and may not work if we are not the
// only reader on Stdio.
fn read_u16s_fixup_surrogates(
    handle: RawHandle,
    buf: &mut [u16],
    mut amount: usize,
) -> io::Result<usize> {
    // Insert possibly remaining unpaired surrogate from last read.
    let mut start = 0;

    let s = SURROGATE.swap(0, SeqCst);
    if s != 0 {
        buf[0] = s;
        start = 1;
        if amount == 1 {
            // Special case: `Stdio::read` guarantees we can always read at least one new
            // `u16` and combine it with an unpaired surrogate, because the
            // UTF-8 buffer is at least 4 bytes.
            amount = 2;
        }
    }
    let mut amount = read_u16s(handle, &mut buf[start..amount])? + start;

    if amount > 0 {
        let last_char = buf[amount - 1];
        if (0xD800..=0xDBFF).contains(&last_char) {
            // high surrogate
            SURROGATE.store(last_char, SeqCst);
            amount -= 1;
        }
    }
    Ok(amount)
}

#[allow(clippy::as_conversions)]
fn read_u16s(handle: RawHandle, buf: &mut [u16]) -> io::Result<usize> {
    // Configure the `pInputControl` parameter to not only return on `\r\n` but
    // also Ctrl-Z, the traditional DOS method to indicate end of character
    // stream / user input (SUB). See #38274 and https://stackoverflow.com/questions/43836040/win-api-readconsole.
    const CTRL_Z: u16 = 0x1A;
    const CTRL_Z_MASK: u32 = 1 << CTRL_Z;
    let input_control = CONSOLE_READCONSOLE_CONTROL {
        nLength: std::mem::size_of::<CONSOLE_READCONSOLE_CONTROL>() as u32,
        nInitialChars: 0,
        dwCtrlWakeupMask: CTRL_Z_MASK,
        dwControlKeyState: 0,
    };

    let mut amount = 0;
    let len: u32 = if let Ok(len) = buf.len().try_into() {
        len
    } else {
        u32::MAX
    };
    if unsafe {
        ReadConsoleW(
            handle as HANDLE,
            buf.as_mut_ptr().cast::<c_void>(),
            len,
            &mut amount,
            &input_control,
        )
    } == 0_i32
    {
        return Err(io::Error::last_os_error());
    }

    if amount > 0 && buf[amount as usize - 1] == CTRL_Z {
        amount -= 1;
    }
    Ok(amount as usize)
}

#[allow(unused)]
fn utf16_to_utf8(utf16: &[u16], utf8: &mut [u8]) -> io::Result<usize> {
    let mut written = 0;
    for chr in decode_utf16(utf16.iter().copied()) {
        match chr {
            Ok(chr) => {
                let _ = chr.encode_utf8(&mut utf8[written..]);
                written += chr.len_utf8();
            }
            Err(_) => {
                // We can't really do any better than forget all data and return an error.
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Windows stdin in console mode does not support non-UTF-16 input; \
                     encountered unpaired surrogate",
                ));
            }
        }
    }
    Ok(written)
}

impl Write for Stdio {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        write(self.id, buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
