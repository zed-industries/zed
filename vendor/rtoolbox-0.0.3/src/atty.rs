// Copyright (c) 2015-2019 Doug Tangren
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

#[cfg(windows)]
use windows_sys::Win32::System::Console::STD_HANDLE;

#[cfg(windows)]
type WCHAR = u16;

/// possible stream sources
#[derive(Clone, Copy, Debug)]
pub enum Stream {
    Stdout,
    Stderr,
    Stdin,
}

/// returns true if this is a tty
#[cfg(target_family = "unix")]
pub fn is(stream: Stream) -> bool {
    let fd = match stream {
        Stream::Stdout => libc::STDOUT_FILENO,
        Stream::Stderr => libc::STDERR_FILENO,
        Stream::Stdin => libc::STDIN_FILENO,
    };
    unsafe { libc::isatty(fd) != 0 }
}

/// returns true if this is a tty
#[cfg(target_os = "hermit")]
pub fn is(stream: Stream) -> bool {
    let fd = match stream {
        Stream::Stdout => hermit_abi::STDOUT_FILENO,
        Stream::Stderr => hermit_abi::STDERR_FILENO,
        Stream::Stdin => hermit_abi::STDIN_FILENO,
    };
    hermit_abi::isatty(fd)
}

/// returns true if this is a tty
#[cfg(windows)]
pub fn is(stream: Stream) -> bool {
    use windows_sys::Win32::System::Console::{
        STD_ERROR_HANDLE as STD_ERROR, STD_INPUT_HANDLE as STD_INPUT,
        STD_OUTPUT_HANDLE as STD_OUTPUT,
    };

    let (fd, others) = match stream {
        Stream::Stdin => (STD_INPUT, [STD_ERROR, STD_OUTPUT]),
        Stream::Stderr => (STD_ERROR, [STD_INPUT, STD_OUTPUT]),
        Stream::Stdout => (STD_OUTPUT, [STD_INPUT, STD_ERROR]),
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

    // Otherwise, we fall back to a very strange msys hack to see if we can
    // sneakily detect the presence of a tty.
    unsafe { msys_tty_on(fd) }
}

/// returns true if this is _not_ a tty
pub fn isnt(stream: Stream) -> bool {
    !is(stream)
}

/// Returns true if any of the given fds are on a console.
#[cfg(windows)]
unsafe fn console_on_any(fds: &[STD_HANDLE]) -> bool {
    use windows_sys::Win32::System::Console::{GetConsoleMode, GetStdHandle};

    for &fd in fds {
        let mut out = 0;
        let handle = GetStdHandle(fd);
        if GetConsoleMode(handle, &mut out) != 0 {
            return true;
        }
    }
    false
}

/// Returns true if there is an MSYS tty on the given handle.
#[cfg(windows)]
unsafe fn msys_tty_on(fd: STD_HANDLE) -> bool {
    use std::os::raw::c_void;
    use std::{mem, slice};

    use windows_sys::Win32::Foundation::MAX_PATH;
    use windows_sys::Win32::Storage::FileSystem::GetFileInformationByHandleEx;
    use windows_sys::Win32::Storage::FileSystem::{FileNameInfo, FILE_NAME_INFO};
    use windows_sys::Win32::System::Console::GetStdHandle;

    let size = mem::size_of::<FILE_NAME_INFO>();
    let mut name_info_bytes = vec![0u8; size + MAX_PATH as usize * mem::size_of::<WCHAR>()];
    let res = GetFileInformationByHandleEx(
        GetStdHandle(fd),
        FileNameInfo,
        &mut *name_info_bytes as *mut _ as *mut c_void,
        name_info_bytes.len() as u32,
    );
    if res == 0 {
        return false;
    }
    let name_info: &FILE_NAME_INFO = &*(name_info_bytes.as_ptr() as *const FILE_NAME_INFO);
    let s = slice::from_raw_parts(
        name_info.FileName.as_ptr(),
        name_info.FileNameLength as usize / 2,
    );
    let name = String::from_utf16_lossy(s);
    // This checks whether 'pty' exists in the file name, which indicates that
    // a pseudo-terminal is attached. To mitigate against false positives
    // (e.g., an actual file name that contains 'pty'), we also require that
    // either the strings 'msys-' or 'cygwin-' are in the file name as well.)
    let is_msys = name.contains("msys-") || name.contains("cygwin-");
    let is_pty = name.contains("-pty");
    is_msys && is_pty
}

/// returns true if this is a tty
#[cfg(target_family = "wasm")]
pub fn is(_stream: Stream) -> bool {
    false
}
