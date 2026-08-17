// Copyright (c) 2023 CtrlC developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#[cfg(unix)]
pub mod platform {
    use std::io;

    pub unsafe fn setup() -> io::Result<()> {
        Ok(())
    }

    pub unsafe fn cleanup() -> io::Result<()> {
        Ok(())
    }

    pub unsafe fn raise_ctrl_c() {
        nix::sys::signal::raise(nix::sys::signal::SIGINT).unwrap();
    }

    pub unsafe fn print(fmt: ::std::fmt::Arguments) {
        use self::io::Write;
        let stdout = ::std::io::stdout();
        stdout.lock().write_fmt(fmt).unwrap();
    }
}

#[cfg(windows)]
pub mod platform {
    use std::io;
    use std::ptr;
    use windows_sys::Win32::Foundation::{
        GENERIC_READ, GENERIC_WRITE, HANDLE, INVALID_HANDLE_VALUE,
    };
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileA, WriteFile, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::Console::{
        AllocConsole, AttachConsole, FreeConsole, GenerateConsoleCtrlEvent, GetConsoleMode,
        GetStdHandle, SetStdHandle, ATTACH_PARENT_PROCESS, CTRL_C_EVENT, STD_ERROR_HANDLE,
        STD_OUTPUT_HANDLE,
    };

    /// Stores a piped stdout handle or a cache that gets
    /// flushed when we reattached to the old console.
    enum Output {
        Pipe(HANDLE),
        Cached(Vec<u8>),
    }

    static mut OLD_OUT: *mut Output = 0 as *mut Output;

    impl io::Write for Output {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            match *self {
                Output::Pipe(handle) => unsafe {
                    let mut n = 0u32;
                    if WriteFile(
                        handle,
                        buf.as_ptr(),
                        buf.len() as u32,
                        &mut n as *mut u32,
                        ptr::null_mut(),
                    ) == 0
                    {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(n as usize)
                    }
                },
                Output::Cached(ref mut s) => s.write(buf),
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Output {
        /// Stores current piped stdout or creates a new output cache that will
        /// be written to stdout at a later time.
        fn new() -> io::Result<Output> {
            unsafe {
                let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
                if stdout.is_null() || stdout == INVALID_HANDLE_VALUE {
                    return Err(io::Error::last_os_error());
                }

                let mut out = 0u32;
                match GetConsoleMode(stdout, &mut out as *mut u32) {
                    0 => Ok(Output::Pipe(stdout)),
                    _ => Ok(Output::Cached(Vec::new())),
                }
            }
        }

        /// Set stdout/stderr and flush cache.
        unsafe fn set_as_std(self) -> io::Result<()> {
            let stdout = match self {
                Output::Pipe(h) => h,
                Output::Cached(_) => get_stdout()?,
            };

            if SetStdHandle(STD_OUTPUT_HANDLE, stdout) == 0 {
                return Err(io::Error::last_os_error());
            }

            if SetStdHandle(STD_ERROR_HANDLE, stdout) == 0 {
                return Err(io::Error::last_os_error());
            }

            match self {
                Output::Pipe(_) => Ok(()),
                Output::Cached(ref s) => {
                    // Write cached output
                    use self::io::Write;
                    let out = io::stdout();
                    out.lock().write_all(&s[..])?;
                    Ok(())
                }
            }
        }
    }

    unsafe fn get_stdout() -> io::Result<HANDLE> {
        let stdout = CreateFileA(
            "CONOUT$\0".as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_WRITE,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            0 as HANDLE,
        );

        if stdout.is_null() || stdout == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(stdout)
        }
    }

    /// Detach from the current console and create a new one,
    /// We do this because GenerateConsoleCtrlEvent() sends ctrl-c events
    /// to all processes on the same console. We want events to be received
    /// only by our process.
    ///
    /// This breaks rust's stdout pre 1.18.0. Rust used to
    /// [cache the std handles](https://github.com/rust-lang/rust/pull/40516)
    ///
    pub unsafe fn setup() -> io::Result<()> {
        let old_out = Output::new()?;

        if FreeConsole() == 0 {
            return Err(io::Error::last_os_error());
        }

        if AllocConsole() == 0 {
            return Err(io::Error::last_os_error());
        }

        // AllocConsole will not always set stdout/stderr to the to the console buffer
        // of the new terminal.

        let stdout = get_stdout()?;
        if SetStdHandle(STD_OUTPUT_HANDLE, stdout) == 0 {
            return Err(io::Error::last_os_error());
        }

        if SetStdHandle(STD_ERROR_HANDLE, stdout) == 0 {
            return Err(io::Error::last_os_error());
        }

        OLD_OUT = Box::into_raw(Box::new(old_out));

        Ok(())
    }

    /// Reattach to the old console.
    pub unsafe fn cleanup() -> io::Result<()> {
        if FreeConsole() == 0 {
            return Err(io::Error::last_os_error());
        }

        if AttachConsole(ATTACH_PARENT_PROCESS) == 0 {
            return Err(io::Error::last_os_error());
        }

        Box::from_raw(OLD_OUT).set_as_std()?;

        Ok(())
    }

    /// This will signal the whole process group.
    pub unsafe fn raise_ctrl_c() {
        assert!(GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0) != 0);
    }

    /// Print to both consoles, this is not thread safe.
    pub unsafe fn print(fmt: ::std::fmt::Arguments) {
        use self::io::Write;
        {
            let stdout = io::stdout();
            stdout.lock().write_fmt(fmt).unwrap();
        }
        {
            assert!(!OLD_OUT.is_null());
            (*OLD_OUT).write_fmt(fmt).unwrap();
        }
    }
}

macro_rules! run_tests {
    ( $($test_fn:ident),* ) => {
        unsafe {
            $(
                harness::platform::print(format_args!("test {} ... ", stringify!($test_fn)));
                $test_fn();
                harness::platform::print(format_args!("ok\n"));
            )*
        }
    }
}

pub fn run_harness(f: fn()) {
    unsafe {
        platform::setup().unwrap();
    }

    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        unsafe {
            platform::cleanup().unwrap();
        }
        (default)(info);
    }));

    println!("");
    f();
    println!("");

    unsafe {
        platform::cleanup().unwrap();
    }
}
