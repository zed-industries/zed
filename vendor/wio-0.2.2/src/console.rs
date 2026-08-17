// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use error::{Error, Result};
use handle::Handle;
use std::{
    mem::{size_of_val, zeroed},
    os::windows::io::FromRawHandle,
    ptr::{null, null_mut},
};
use wide::ToWide;
use winapi::{
    um::{
        consoleapi::{AllocConsole, GetConsoleCP, GetConsoleOutputCP, GetNumberOfConsoleInputEvents, ReadConsoleInputW},
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::INVALID_HANDLE_VALUE,
        wincon::{AttachConsole, CHAR_INFO, CONSOLE_FONT_INFOEX, CONSOLE_SCREEN_BUFFER_INFO, CONSOLE_SCREEN_BUFFER_INFOEX, CONSOLE_TEXTMODE_BUFFER, COORD, CreateConsoleScreenBuffer, FlushConsoleInputBuffer, FOCUS_EVENT, FreeConsole, GetConsoleScreenBufferInfo, GetConsoleScreenBufferInfoEx, GetCurrentConsoleFont, INPUT_RECORD, KEY_EVENT, MENU_EVENT, MOUSE_EVENT, SetConsoleActiveScreenBuffer, SetConsoleCP, SetConsoleOutputCP, SetConsoleScreenBufferInfoEx, SMALL_RECT, WINDOW_BUFFER_SIZE_EVENT, WriteConsoleOutputW},
        winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE},
    },
    shared::minwindef::{DWORD, FALSE},
};

pub struct ScreenBuffer(Handle);
impl ScreenBuffer {
    pub fn new() -> Result<ScreenBuffer> {
        let handle = unsafe { CreateConsoleScreenBuffer(
            GENERIC_READ | GENERIC_WRITE, FILE_SHARE_READ | FILE_SHARE_WRITE,
            null(), CONSOLE_TEXTMODE_BUFFER, null_mut(),
        )};
        if handle == INVALID_HANDLE_VALUE { return Error::last() }
        unsafe { Ok(ScreenBuffer(Handle::new(handle))) }
    }
    /// Gets the actual active console screen buffer
    pub fn from_conout() -> Result<ScreenBuffer> {
        let handle = unsafe { CreateFileW(
            "CONOUT$".to_wide_null().as_ptr(), GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ, null_mut(), OPEN_EXISTING,
            0, null_mut(),
        )};
        if handle == INVALID_HANDLE_VALUE { return Error::last() }
        unsafe { Ok(ScreenBuffer(Handle::new(handle))) }
    }
    pub fn set_active(&self) -> Result<()> {
        let res = unsafe { SetConsoleActiveScreenBuffer(*self.0) };
        if res == 0 { return Error::last() }
        Ok(())
    }
    pub fn info(&self) -> Result<ScreenBufferInfo> {
        let mut info = ScreenBufferInfo(unsafe { zeroed() });
        let res = unsafe { GetConsoleScreenBufferInfo(*self.0, &mut info.0) };
        if res == 0 { return Error::last() }
        Ok(info)
    }
    pub fn info_ex(&self) -> Result<ScreenBufferInfoEx> {
        let mut info: CONSOLE_SCREEN_BUFFER_INFOEX = unsafe { zeroed() };
        info.cbSize = size_of_val(&info) as u32;
        let res = unsafe { GetConsoleScreenBufferInfoEx(*self.0, &mut info) };
        if res == 0 { return Error::last() }
        // Yes, this is important
        info.srWindow.Right += 1;
        info.srWindow.Bottom += 1;
        Ok(ScreenBufferInfoEx(info))
    }
    pub fn set_info_ex(&self, mut info: ScreenBufferInfoEx) -> Result<()> {
        let res = unsafe { SetConsoleScreenBufferInfoEx(*self.0, &mut info.0) };
        if res == 0 { return Error::last() }
        Ok(())
    }
    // pub fn font_ex(&self) -> Result<FontEx> {
        // unsafe {
            // let mut info = zeroed();
            // info.cbSize = size_of_val(&info);
            // let res = GetCurrentConsoleFontEx(*self.0, w::FALSE, &mut info);
            // if res == 0 { return Error::last() }
            // Ok(FontEx(info))
        // }
    // }
    pub fn write_output(&self, buf: &[CharInfo], size: (i16, i16), pos: (i16, i16)) -> Result<()> {
        assert!(buf.len() == (size.0 as usize) * (size.1 as usize));
        let mut rect = SMALL_RECT {
            Left: pos.0,
            Top: pos.1,
            Right: pos.0 + size.0,
            Bottom: pos.1 + size.1,
        };
        let size = COORD { X: size.0, Y: size.1 };
        let pos = COORD { X: 0, Y: 0 };
        let res = unsafe { WriteConsoleOutputW(
            *self.0, buf.as_ptr() as *const CHAR_INFO, size, pos, &mut rect
        )};
        if res == 0 { return Error::last() }
        Ok(())
    }
    pub fn font_size(&self) -> Result<(i16, i16)> {
        unsafe {
            let mut font = zeroed();
            let res = GetCurrentConsoleFont(*self.0, FALSE, &mut font);
            if res == 0 { return Error::last() }
            Ok((font.dwFontSize.X, font.dwFontSize.Y))
        }
    }
}
impl FromRawHandle for ScreenBuffer {
    unsafe fn from_raw_handle(handle: HANDLE) -> ScreenBuffer {
        ScreenBuffer(Handle::new(handle))
    }
}
pub struct InputBuffer(Handle);
impl InputBuffer {
    /// Gets the actual active console input buffer
    pub fn from_conin() -> Result<InputBuffer> {
        let handle = unsafe { CreateFileW(
            "CONIN$".to_wide_null().as_ptr(), GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE, null_mut(), OPEN_EXISTING,
            0, null_mut(),
        )};
        if handle == INVALID_HANDLE_VALUE { Error::last() }
        else { unsafe { Ok(InputBuffer::from_raw_handle(handle)) } }
    }
    /// The number of input that is available to read
    pub fn available_input(&self) -> Result<u32> {
        let mut num = 0;
        let res = unsafe { GetNumberOfConsoleInputEvents(*self.0, &mut num) };
        if res == 0 { return Error::last() }
        Ok(num)
    }
    /// Reads a bunch of input events
    pub fn read_input(&self) -> Result<Vec<Input>> {
        let mut buf: [INPUT_RECORD; 0x1000] = unsafe { zeroed() };
        let mut size = 0;
        let res = unsafe { ReadConsoleInputW(
            *self.0, buf.as_mut_ptr(), buf.len() as DWORD, &mut size,
        )};
        if res == 0 { return Error::last() }
        Ok(buf[..(size as usize)].iter().map(|input| {
            unsafe { match input.EventType {
                KEY_EVENT => {
                    let e = input.Event.KeyEvent();
                    Input::Key {
                        key_down: e.bKeyDown != 0,
                        repeat_count: e.wRepeatCount,
                        key_code: e.wVirtualKeyCode,
                        scan_code: e.wVirtualScanCode,
                        wide_char: *e.uChar.UnicodeChar(),
                        control_key_state: e.dwControlKeyState,
                    }
                },
                MOUSE_EVENT => {
                    let e = input.Event.MouseEvent();
                    Input::Mouse {
                        position: (e.dwMousePosition.X, e.dwMousePosition.Y),
                        button_state: e.dwButtonState,
                        control_key_state: e.dwControlKeyState,
                        event_flags: e.dwEventFlags,
                    }
                },
                WINDOW_BUFFER_SIZE_EVENT => {
                    let s = input.Event.WindowBufferSizeEvent().dwSize;
                    Input::WindowBufferSize(s.X, s.Y)
                },
                MENU_EVENT => Input::Menu(input.Event.MenuEvent().dwCommandId),
                FOCUS_EVENT => Input::Focus(input.Event.FocusEvent().bSetFocus != 0),
                e => unreachable!("invalid event type: {}", e),
            } }
        }).collect())
    }
    /// Clears all pending input
    pub fn flush_input(&self) -> Result<()> {
        let res = unsafe { FlushConsoleInputBuffer(*self.0) };
        if res == 0 { return Error::last() }
        Ok(())
    }
}
impl FromRawHandle for InputBuffer {
    unsafe fn from_raw_handle(handle: HANDLE) -> InputBuffer {
        InputBuffer(Handle::from_raw_handle(handle))
    }
}
#[repr(transparent)] #[derive(Copy, Clone)]
pub struct ScreenBufferInfo(CONSOLE_SCREEN_BUFFER_INFO);
impl ScreenBufferInfo {
    pub fn size(&self) -> (i16, i16) {
        (self.0.dwSize.X, self.0.dwSize.Y)
    }
}
#[repr(transparent)] #[derive(Copy, Clone)]
pub struct ScreenBufferInfoEx(CONSOLE_SCREEN_BUFFER_INFOEX);
impl ScreenBufferInfoEx {
    pub fn raw_mut(&mut self) -> &mut CONSOLE_SCREEN_BUFFER_INFOEX {
        &mut self.0
    }
}
#[repr(transparent)] #[derive(Copy, Clone)]
pub struct FontInfoEx(CONSOLE_FONT_INFOEX);
#[derive(Copy, Clone)]
pub enum Input {
    Key {
        key_down: bool,
        repeat_count: u16,
        key_code: u16,
        scan_code: u16,
        wide_char: u16,
        control_key_state: u32,
    },
    Mouse {
        position: (i16, i16),
        button_state: u32,
        control_key_state: u32,
        event_flags: u32,
    },
    WindowBufferSize(i16, i16),
    Menu(u32),
    Focus(bool),
}
#[repr(transparent)] #[derive(Copy, Clone)]
pub struct CharInfo(CHAR_INFO);
impl CharInfo {
    pub fn new(ch: u16, attr: u16) -> CharInfo {
        let mut ci: CHAR_INFO = unsafe { zeroed() };
        unsafe { *ci.Char.UnicodeChar_mut() = ch };
        ci.Attributes = attr;
        CharInfo(ci)
    }
    pub fn character(&self) -> u16 { unsafe { *self.0.Char.UnicodeChar() } }
    pub fn attributes(&self) -> u16 { self.0.Attributes }
}
/// Allocates a console if the process does not already have a console.
pub fn alloc() -> Result<()> {
    match unsafe { AllocConsole() } {
        0 => Error::last(),
        _ => Ok(()),
    }
}
/// Detaches the process from its current console.
pub fn free() -> Result<()> {
    match unsafe { FreeConsole() } {
        0 => Error::last(),
        _ => Ok(()),
    }
}
/// Attaches the process to the console of the specified process.
/// Pass None to attach to the console of the parent process.
pub fn attach(processid: Option<u32>) -> Result<()> {
    match unsafe { AttachConsole(processid.unwrap_or(-1i32 as u32)) } {
        0 => Error::last(),
        _ => Ok(()),
    }
}
/// Gets the current input code page
pub fn input_code_page() -> u32 {
    unsafe { GetConsoleCP() }
}
/// Gets the current output code page
pub fn output_code_page() -> u32 {
    unsafe { GetConsoleOutputCP() }
}
/// Sets the current input code page
pub fn set_input_code_page(code: u32) -> Result<()> {
    let res = unsafe { SetConsoleCP(code) };
    if res == 0 { return Error::last() }
    Ok(())
}
/// Sets the current output code page
pub fn set_output_code_page(code: u32) -> Result<()> {
    let res = unsafe { SetConsoleOutputCP(code) };
    if res == 0 { return Error::last() }
    Ok(())
}
