use core::fmt::Display;
use core::iter::once;
use core::mem::{self, MaybeUninit};
use core::{char, cmp};
use std::env;
use std::ffi::OsStr;
use std::io;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::AsRawHandle;

use encode_unicode::error::Utf16TupleError;
use encode_unicode::CharExt;
use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE, MAX_PATH};
use windows_sys::Win32::Storage::FileSystem::{FileNameInfo, GetFileInformationByHandleEx};
use windows_sys::Win32::System::Console::CONSOLE_MODE;
use windows_sys::Win32::System::Console::{
    FillConsoleOutputAttribute, FillConsoleOutputCharacterA, GetConsoleCursorInfo, GetConsoleMode,
    GetConsoleScreenBufferInfo, GetNumberOfConsoleInputEvents, GetStdHandle, ReadConsoleInputW,
    SetConsoleCursorInfo, SetConsoleCursorPosition, SetConsoleMode, SetConsoleTitleW,
    CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, ENABLE_PROCESSED_INPUT,
    ENABLE_VIRTUAL_TERMINAL_PROCESSING, INPUT_RECORD, INPUT_RECORD_0, KEY_EVENT, KEY_EVENT_RECORD,
    STD_ERROR_HANDLE, STD_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

use crate::common_term;
use crate::kb::Key;
use crate::term::{Term, TermTarget};

#[cfg(feature = "windows-console-colors")]
mod colors;

#[cfg(feature = "windows-console-colors")]
pub(crate) use self::colors::*;

pub(crate) const DEFAULT_WIDTH: u16 = 79;

pub(crate) fn as_handle(term: &Term) -> HANDLE {
    // convert between windows_sys::Win32::Foundation::HANDLE and std::os::windows::raw::HANDLE
    term.as_raw_handle() as HANDLE
}

pub(crate) fn is_a_terminal(out: &Term) -> bool {
    let (fd, others) = match out.target() {
        TermTarget::Stdout => (STD_OUTPUT_HANDLE, [STD_INPUT_HANDLE, STD_ERROR_HANDLE]),
        TermTarget::Stderr => (STD_ERROR_HANDLE, [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE]),
    };

    if unsafe { console_on_any(&[fd]) } {
        // False positives aren't possible. If we got a console then
        // we definitely have a tty on stdin.
        return true;
    }

    // At this point, we *could* have a false negative. We can determine that
    // this is true negative if we can detect the presence of a console on
    // any of the other streams. If another stream has a console, then we know
    // we're in a Windows console and can therefore trust the negative.
    if unsafe { console_on_any(&others) } {
        return false;
    }

    msys_tty_on(out)
}

pub(crate) fn is_a_color_terminal(out: &Term) -> bool {
    if !is_a_terminal(out) {
        return false;
    }
    if env::var("NO_COLOR").is_ok() {
        return false;
    }
    if msys_tty_on(out) {
        return match env::var("TERM") {
            Ok(term) => term != "dumb",
            Err(_) => true,
        };
    }
    enable_ansi_on(out)
}

pub(crate) fn is_a_true_color_terminal(out: &Term) -> bool {
    if !is_a_color_terminal(out) {
        return false;
    }
    // Powershell does not respect the COLORTERM var despite supporting true colors
    // but other shells may respect it
    if msys_tty_on(out) {
        return match env::var("COLORTERM") {
            Ok(term) => term == "truecolor" || term == "24bit",
            Err(_) => true,
        };
    }
    false
}

/// Enables or disables the `mode` flag on the given `HANDLE` and yields the previous mode.
fn set_console_mode(handle: HANDLE, mode: CONSOLE_MODE, enable: bool) -> Option<CONSOLE_MODE> {
    unsafe {
        let mut dw_mode = 0;
        if GetConsoleMode(handle, &mut dw_mode) == 0 {
            return None;
        }

        let new_dw_mode = match enable {
            true => dw_mode | mode,
            false => dw_mode & !mode,
        };

        if SetConsoleMode(handle, new_dw_mode) == 0 {
            return None;
        }

        Some(dw_mode)
    }
}

struct ConsoleModeGuard {
    handle: HANDLE,
    restore_mode: CONSOLE_MODE,
}

impl ConsoleModeGuard {
    fn set(handle: HANDLE, mode: CONSOLE_MODE, enable: bool) -> Option<Self> {
        Some(ConsoleModeGuard {
            handle,
            restore_mode: set_console_mode(handle, mode, enable)?,
        })
    }
}

impl Drop for ConsoleModeGuard {
    fn drop(&mut self) {
        unsafe {
            SetConsoleMode(self.handle, self.restore_mode);
        }
    }
}

fn enable_ansi_on(out: &Term) -> bool {
    set_console_mode(
        out.as_raw_handle(),
        ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        true,
    )
    .is_some()
}

unsafe fn console_on_any(fds: &[STD_HANDLE]) -> bool {
    for &fd in fds {
        let mut out = 0;
        let handle = GetStdHandle(fd);
        if GetConsoleMode(handle, &mut out) != 0 {
            return true;
        }
    }
    false
}

pub(crate) fn terminal_size(out: &Term) -> Option<(u16, u16)> {
    use windows_sys::Win32::System::Console::SMALL_RECT;

    // convert between windows_sys::Win32::Foundation::HANDLE and std::os::windows::raw::HANDLE
    let handle = out.as_raw_handle();
    let hand = handle as windows_sys::Win32::Foundation::HANDLE;

    if hand == INVALID_HANDLE_VALUE {
        return None;
    }

    let zc = COORD { X: 0, Y: 0 };
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: zc,
        dwCursorPosition: zc,
        wAttributes: 0,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: zc,
    };
    if unsafe { GetConsoleScreenBufferInfo(hand, &mut csbi) } == 0 {
        return None;
    }

    let rows = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16;
    let columns = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16;

    Some((rows, columns))
}

pub(crate) fn move_cursor_to(out: &Term, x: usize, y: usize) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::move_cursor_to(out, x, y);
    }
    if let Some((hand, _)) = get_console_screen_buffer_info(as_handle(out)) {
        unsafe {
            SetConsoleCursorPosition(
                hand,
                COORD {
                    X: x as i16,
                    Y: y as i16,
                },
            );
        }
    }
    Ok(())
}

pub(crate) fn move_cursor_up(out: &Term, n: usize) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::move_cursor_up(out, n);
    }

    if let Some((_, csbi)) = get_console_screen_buffer_info(as_handle(out)) {
        move_cursor_to(out, 0, csbi.dwCursorPosition.Y as usize - n)?;
    }
    Ok(())
}

pub(crate) fn move_cursor_down(out: &Term, n: usize) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::move_cursor_down(out, n);
    }

    if let Some((_, csbi)) = get_console_screen_buffer_info(as_handle(out)) {
        move_cursor_to(out, 0, csbi.dwCursorPosition.Y as usize + n)?;
    }
    Ok(())
}

pub(crate) fn move_cursor_left(out: &Term, n: usize) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::move_cursor_left(out, n);
    }

    if let Some((_, csbi)) = get_console_screen_buffer_info(as_handle(out)) {
        move_cursor_to(
            out,
            csbi.dwCursorPosition.X as usize - n,
            csbi.dwCursorPosition.Y as usize,
        )?;
    }
    Ok(())
}

pub(crate) fn move_cursor_right(out: &Term, n: usize) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::move_cursor_right(out, n);
    }

    if let Some((_, csbi)) = get_console_screen_buffer_info(as_handle(out)) {
        move_cursor_to(
            out,
            csbi.dwCursorPosition.X as usize + n,
            csbi.dwCursorPosition.Y as usize,
        )?;
    }
    Ok(())
}

pub(crate) fn clear_line(out: &Term) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::clear_line(out);
    }
    if let Some((hand, csbi)) = get_console_screen_buffer_info(as_handle(out)) {
        unsafe {
            let width = csbi.srWindow.Right - csbi.srWindow.Left;
            let pos = COORD {
                X: 0,
                Y: csbi.dwCursorPosition.Y,
            };
            let mut written = 0;
            FillConsoleOutputCharacterA(hand, b' ' as i8, width as u32, pos, &mut written);
            FillConsoleOutputAttribute(hand, csbi.wAttributes, width as u32, pos, &mut written);
            SetConsoleCursorPosition(hand, pos);
        }
    }
    Ok(())
}

pub(crate) fn clear_chars(out: &Term, n: usize) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::clear_chars(out, n);
    }
    if let Some((hand, csbi)) = get_console_screen_buffer_info(as_handle(out)) {
        unsafe {
            let width = cmp::min(csbi.dwCursorPosition.X, n as i16);
            let pos = COORD {
                X: csbi.dwCursorPosition.X - width,
                Y: csbi.dwCursorPosition.Y,
            };
            let mut written = 0;
            FillConsoleOutputCharacterA(hand, b' ' as i8, width as u32, pos, &mut written);
            FillConsoleOutputAttribute(hand, csbi.wAttributes, width as u32, pos, &mut written);
            SetConsoleCursorPosition(hand, pos);
        }
    }
    Ok(())
}

pub(crate) fn clear_screen(out: &Term) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::clear_screen(out);
    }
    if let Some((hand, csbi)) = get_console_screen_buffer_info(as_handle(out)) {
        unsafe {
            let cells = csbi.dwSize.X as u32 * csbi.dwSize.Y as u32; // as u32, or else this causes stack overflows.
            let pos = COORD { X: 0, Y: 0 };
            let mut written = 0;
            FillConsoleOutputCharacterA(hand, b' ' as i8, cells, pos, &mut written); // cells as u32 no longer needed.
            FillConsoleOutputAttribute(hand, csbi.wAttributes, cells, pos, &mut written);
            SetConsoleCursorPosition(hand, pos);
        }
    }
    Ok(())
}

pub(crate) fn clear_to_end_of_screen(out: &Term) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::clear_to_end_of_screen(out);
    }
    if let Some((hand, csbi)) = get_console_screen_buffer_info(as_handle(out)) {
        unsafe {
            let bottom = csbi.srWindow.Right as u32 * csbi.srWindow.Bottom as u32;
            let cells = bottom - (csbi.dwCursorPosition.X as u32 * csbi.dwCursorPosition.Y as u32); // as u32, or else this causes stack overflows.
            let pos = COORD {
                X: 0,
                Y: csbi.dwCursorPosition.Y,
            };
            let mut written = 0;
            FillConsoleOutputCharacterA(hand, b' ' as i8, cells, pos, &mut written); // cells as u32 no longer needed.
            FillConsoleOutputAttribute(hand, csbi.wAttributes, cells, pos, &mut written);
            SetConsoleCursorPosition(hand, pos);
        }
    }
    Ok(())
}

pub(crate) fn show_cursor(out: &Term) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::show_cursor(out);
    }
    if let Some((hand, mut cci)) = get_console_cursor_info(as_handle(out)) {
        unsafe {
            cci.bVisible = 1;
            SetConsoleCursorInfo(hand, &cci);
        }
    }
    Ok(())
}

pub(crate) fn hide_cursor(out: &Term) -> io::Result<()> {
    if out.is_msys_tty {
        return common_term::hide_cursor(out);
    }
    if let Some((hand, mut cci)) = get_console_cursor_info(as_handle(out)) {
        unsafe {
            cci.bVisible = 0;
            SetConsoleCursorInfo(hand, &cci);
        }
    }
    Ok(())
}

fn get_console_screen_buffer_info(hand: HANDLE) -> Option<(HANDLE, CONSOLE_SCREEN_BUFFER_INFO)> {
    let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe { mem::zeroed() };
    match unsafe { GetConsoleScreenBufferInfo(hand, &mut csbi) } {
        0 => None,
        _ => Some((hand, csbi)),
    }
}

fn get_console_cursor_info(hand: HANDLE) -> Option<(HANDLE, CONSOLE_CURSOR_INFO)> {
    let mut cci: CONSOLE_CURSOR_INFO = unsafe { mem::zeroed() };
    match unsafe { GetConsoleCursorInfo(hand, &mut cci) } {
        0 => None,
        _ => Some((hand, cci)),
    }
}

pub(crate) fn key_from_key_code(code: VIRTUAL_KEY) -> Key {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse;

    match code {
        KeyboardAndMouse::VK_LEFT => Key::ArrowLeft,
        KeyboardAndMouse::VK_RIGHT => Key::ArrowRight,
        KeyboardAndMouse::VK_UP => Key::ArrowUp,
        KeyboardAndMouse::VK_DOWN => Key::ArrowDown,
        KeyboardAndMouse::VK_RETURN => Key::Enter,
        KeyboardAndMouse::VK_ESCAPE => Key::Escape,
        KeyboardAndMouse::VK_BACK => Key::Backspace,
        KeyboardAndMouse::VK_TAB => Key::Tab,
        KeyboardAndMouse::VK_HOME => Key::Home,
        KeyboardAndMouse::VK_END => Key::End,
        KeyboardAndMouse::VK_DELETE => Key::Del,
        KeyboardAndMouse::VK_SHIFT => Key::Shift,
        KeyboardAndMouse::VK_MENU => Key::Alt,
        _ => Key::Unknown,
    }
}

pub(crate) fn read_secure() -> io::Result<String> {
    let mut rv = String::new();
    loop {
        match read_single_key(false)? {
            Key::Enter => {
                break;
            }
            Key::Char('\x08') => {
                if !rv.is_empty() {
                    let new_len = rv.len() - 1;
                    rv.truncate(new_len);
                }
            }
            Key::Char(c) => {
                rv.push(c);
            }
            _ => {}
        }
    }
    Ok(rv)
}

pub(crate) fn read_single_key(ctrlc_key: bool) -> io::Result<Key> {
    let key_event = {
        let _guard = ctrlc_key.then(|| {
            ConsoleModeGuard::set(
                unsafe { GetStdHandle(STD_INPUT_HANDLE) },
                ENABLE_PROCESSED_INPUT,
                false,
            )
        });
        read_key_event()?
    };

    let unicode_char = unsafe { key_event.uChar.UnicodeChar };
    if unicode_char == 0 {
        Ok(key_from_key_code(key_event.wVirtualKeyCode))
    } else {
        // This is a unicode character, in utf-16. Try to decode it by itself.
        match char::from_utf16_tuple((unicode_char, None)) {
            Ok(c) => {
                // Maintain backward compatibility. The previous implementation (_getwch()) would return
                // a special keycode for `Enter`, while ReadConsoleInputW() prefers to use '\r'.
                if c == '\r' {
                    Ok(Key::Enter)
                } else if c == '\t' {
                    Ok(Key::Tab)
                } else if c == '\x08' {
                    Ok(Key::Backspace)
                } else if c == '\x1B' {
                    Ok(Key::Escape)
                } else if c == '\x03' && ctrlc_key {
                    Ok(Key::CtrlC)
                } else {
                    Ok(Key::Char(c))
                }
            }
            // This is part of a surrogate pair. Try to read the second half.
            Err(Utf16TupleError::MissingSecond) => {
                // Confirm that there is a next character to read.
                if get_key_event_count()? == 0 {
                    let message = format!(
                        "Read invalid utf16 {}: {}",
                        unicode_char,
                        Utf16TupleError::MissingSecond
                    );
                    return Err(io::Error::new(io::ErrorKind::InvalidData, message));
                }

                // Read the next character.
                let next_event = read_key_event()?;
                let next_surrogate = unsafe { next_event.uChar.UnicodeChar };

                // Attempt to decode it.
                match char::from_utf16_tuple((unicode_char, Some(next_surrogate))) {
                    Ok(c) => Ok(Key::Char(c)),

                    // Return an InvalidData error. This is the recommended value for UTF-related I/O errors.
                    // (This error is given when reading a non-UTF8 file into a String, for example.)
                    Err(e) => {
                        let message = format!(
                            "Read invalid surrogate pair ({unicode_char}, {next_surrogate}): {e}",
                        );
                        Err(io::Error::new(io::ErrorKind::InvalidData, message))
                    }
                }
            }

            // Return an InvalidData error. This is the recommended value for UTF-related I/O errors.
            // (This error is given when reading a non-UTF8 file into a String, for example.)
            Err(e) => {
                let message = format!("Read invalid utf16 {unicode_char}: {e}");
                Err(io::Error::new(io::ErrorKind::InvalidData, message))
            }
        }
    }
}

fn get_stdin_handle() -> io::Result<HANDLE> {
    let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    if handle == INVALID_HANDLE_VALUE {
        Err(io::Error::last_os_error())
    } else {
        Ok(handle)
    }
}

/// Get the number of pending events in the ReadConsoleInput queue. Note that while
/// these aren't necessarily key events, the only way that multiple events can be
/// put into the queue simultaneously is if a unicode character spanning multiple u16's
/// is read.
///
/// Therefore, this is accurate as long as at least one KEY_EVENT has already been read.
fn get_key_event_count() -> io::Result<u32> {
    let handle = get_stdin_handle()?;
    let mut event_count: u32 = unsafe { mem::zeroed() };

    let success = unsafe { GetNumberOfConsoleInputEvents(handle, &mut event_count) };
    if success == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(event_count)
    }
}

fn read_key_event() -> io::Result<KEY_EVENT_RECORD> {
    let handle = get_stdin_handle()?;
    let mut buffer: INPUT_RECORD = unsafe { mem::zeroed() };

    let mut events_read: u32 = unsafe { mem::zeroed() };

    let mut key_event: KEY_EVENT_RECORD;
    loop {
        let success = unsafe { ReadConsoleInputW(handle, &mut buffer, 1, &mut events_read) };
        if success == 0 {
            return Err(io::Error::last_os_error());
        }
        if events_read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "ReadConsoleInput returned no events, instead of waiting for an event",
            ));
        }

        if events_read == 1 && buffer.EventType != KEY_EVENT as u16 {
            // This isn't a key event; ignore it.
            continue;
        }

        key_event = unsafe { mem::transmute::<INPUT_RECORD_0, KEY_EVENT_RECORD>(buffer.Event) };

        if key_event.bKeyDown == 0 {
            // This is a key being released; ignore it.
            continue;
        }

        return Ok(key_event);
    }
}

pub(crate) fn wants_emoji() -> bool {
    // If WT_SESSION is set, we can assume we're running in the new
    // Windows Terminal.  The correct way to detect this is not available
    // yet.  See https://github.com/microsoft/terminal/issues/1040
    env::var("WT_SESSION").is_ok()
}

/// Returns true if there is an MSYS tty on the given handle.
pub(crate) fn msys_tty_on(term: &Term) -> bool {
    let handle = term.as_raw_handle();
    unsafe {
        // Check whether the Windows 10 native pty is enabled
        {
            let mut out = MaybeUninit::uninit();
            let res = GetConsoleMode(handle as HANDLE, out.as_mut_ptr());
            if res != 0 // If res is true then out was initialized.
                && (out.assume_init() & ENABLE_VIRTUAL_TERMINAL_PROCESSING)
                    == ENABLE_VIRTUAL_TERMINAL_PROCESSING
            {
                return true;
            }
        }

        /// Mirrors windows_sys::Win32::Storage::FileSystem::FILE_NAME_INFO, giving
        /// it a fixed length that we can stack allocate
        #[repr(C)]
        #[allow(non_snake_case)]
        struct FILE_NAME_INFO {
            FileNameLength: u32,
            FileName: [u16; MAX_PATH as usize],
        }

        let mut name_info = FILE_NAME_INFO {
            FileNameLength: 0,
            FileName: [0; MAX_PATH as usize],
        };
        let res = GetFileInformationByHandleEx(
            handle as HANDLE,
            FileNameInfo,
            &mut name_info as *mut _ as *mut c_void,
            mem::size_of::<FILE_NAME_INFO>() as u32,
        );
        if res == 0 {
            return false;
        }

        // Use `get` because `FileNameLength` can be out of range.
        let s = match name_info
            .FileName
            .get(..name_info.FileNameLength as usize / 2)
        {
            Some(s) => s,
            None => return false,
        };
        let name = String::from_utf16_lossy(s);
        // This checks whether 'pty' exists in the file name, which indicates that
        // a pseudo-terminal is attached. To mitigate against false positives
        // (e.g., an actual file name that contains 'pty'), we also require that
        // either the strings 'msys-' or 'cygwin-' are in the file name as well.)
        let is_msys = name.contains("msys-") || name.contains("cygwin-");
        let is_pty = name.contains("-pty");
        is_msys && is_pty
    }
}

pub(crate) fn set_title<T: Display>(title: T) {
    let buffer: Vec<u16> = OsStr::new(&format!("{title}"))
        .encode_wide()
        .chain(once(0))
        .collect();
    unsafe {
        SetConsoleTitleW(buffer.as_ptr());
    }
}
