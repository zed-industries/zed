#[cfg(target_family = "wasm")]
mod wasm {
    use std::io::Write;

    /// Displays a message on the STDOUT
    pub fn print_tty(prompt: impl ToString) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();
        write!(stdout, "{}", prompt.to_string().as_str())?;
        stdout.flush()?;
        Ok(())
    }
}

#[cfg(target_family = "unix")]
mod unix {
    use std::io::Write;

    /// Displays a message on the TTY
    pub fn print_tty(prompt: impl ToString) -> std::io::Result<()> {
        let mut stream = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        stream
            .write_all(prompt.to_string().as_str().as_bytes())
            .and_then(|_| stream.flush())
    }
}

#[cfg(target_family = "windows")]
mod windows {
    use std::io::Write;
    use std::os::windows::io::FromRawHandle;
    use windows_sys::core::PCSTR;
    use windows_sys::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileA, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::Console::{GetConsoleOutputCP, SetConsoleOutputCP};

    /// Displays a message on the TTY
    pub fn print_tty(prompt: impl ToString) -> std::io::Result<()> {
        let previous_console_output_cp = unsafe {
            GetConsoleOutputCP()
        };

        let handle = unsafe {
            CreateFileA(
                b"CONOUT$\x00".as_ptr() as PCSTR,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                0,
                INVALID_HANDLE_VALUE,
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            unsafe {
                SetConsoleOutputCP(previous_console_output_cp);
            }
            return Err(std::io::Error::last_os_error());
        }

        let mut stream = unsafe { std::fs::File::from_raw_handle(handle as _) };

        unsafe {
            SetConsoleOutputCP(65001); // 65001 is UTF-8
        };
        let result = stream
            .write_all(prompt.to_string().as_str().as_bytes())
            .and_then(|_| stream.flush());
        unsafe {
            SetConsoleOutputCP(previous_console_output_cp);
        }
        result
    }
}

/// Prints a message to a writer
pub fn print_writer(stream: &mut impl Write, prompt: impl ToString) -> std::io::Result<()> {
    stream
        .write_all(prompt.to_string().as_str().as_bytes())
        .and_then(|_| stream.flush())
}

use std::io::Write;
#[cfg(target_family = "unix")]
pub use unix::print_tty;
#[cfg(target_family = "wasm")]
pub use wasm::print_tty;
#[cfg(target_family = "windows")]
pub use windows::print_tty;
