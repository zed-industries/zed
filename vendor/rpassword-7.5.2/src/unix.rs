use crate::RawPasswordInput;
use crate::config::{Config, InputTarget, OutputTarget};
use crate::utf8::read_char;
use libc::{ECHO, ECHONL, ICANON, ISIG, TCSANOW, VMIN, VTIME, c_int, isatty, tcsetattr, termios};
use std::fs::OpenOptions;
use std::io::{self, Cursor, Read, Write};
use std::mem;
use std::os::fd::RawFd;
use std::os::unix::io::AsRawFd;

pub(crate) const DEFAULT_INPUT_PATH: &str = "/dev/tty";
pub(crate) const DEFAULT_OUTPUT_PATH: &str = "/dev/tty";

/// Turns a C function return into an IO Result
fn io_result(ret: c_int) -> std::io::Result<()> {
    match ret {
        0 => Ok(()),
        _ => Err(std::io::Error::last_os_error()),
    }
}

fn is_interactive_terminal(fd: c_int) -> bool {
    unsafe { isatty(fd) != 0 }
}

fn safe_tcgetattr(fd: c_int) -> std::io::Result<termios> {
    let mut term = mem::MaybeUninit::<termios>::uninit();
    io_result(unsafe { ::libc::tcgetattr(fd, term.as_mut_ptr()) })?;
    Ok(unsafe { term.assume_init() })
}

fn safe_tcsetattr(fd: c_int, term: &mut termios) -> std::io::Result<()> {
    io_result(unsafe { tcsetattr(fd, TCSANOW, term) })
}

pub(crate) struct RawModeInput {
    input: Box<dyn Read>,
    input_fd: Option<RawFd>,
    input_term_orig: Option<termios>,
    input_is_tty: bool,
    output: Box<dyn Write>,
    output_fd: Option<RawFd>,
    output_term_orig: Option<termios>,
    output_is_tty: bool,
}

impl Drop for RawModeInput {
    fn drop(&mut self) {
        if let Some(fd) = self.input_fd
            && let Some(ref mut term_orig) = self.input_term_orig
        {
            unsafe {
                tcsetattr(fd, TCSANOW, term_orig);
            }
        }
        if let Some(fd) = self.output_fd
            && let Some(ref mut term_orig) = self.output_term_orig
        {
            unsafe {
                tcsetattr(fd, TCSANOW, term_orig);
            }
        }
    }
}

impl RawPasswordInput for RawModeInput {
    fn new(config: Config) -> io::Result<impl RawPasswordInput> {
        let mut input_fd: Option<RawFd> = None;
        let input: Box<dyn Read> = match config.input {
            InputTarget::FilePath(path) => {
                let file = OpenOptions::new().read(true).open(path)?;
                input_fd = Some(file.as_raw_fd());
                Box::new(file)
            }
            InputTarget::Reader(reader) => Box::new(reader),
        };
        let input_is_tty = if let Some(fd) = input_fd {
            is_interactive_terminal(fd)
        } else {
            false
        };
        let input_term_orig = if input_is_tty && let Some(fd) = input_fd {
            Some(safe_tcgetattr(fd)?)
        } else {
            None
        };

        let mut output_fd: Option<RawFd> = None;
        let output: Box<dyn Write> = match config.output {
            OutputTarget::FilePath(path) => {
                let file = OpenOptions::new().write(true).open(path)?;
                output_fd = Some(file.as_raw_fd());
                Box::new(file)
            }
            OutputTarget::Writer(writer) => Box::new(writer),
            OutputTarget::Void => Box::new(Cursor::new(Vec::<u8>::new())), // TODO: Should use a SafeVec instead
        };
        let output_is_tty = if let Some(fd) = output_fd {
            is_interactive_terminal(fd)
        } else {
            false
        };
        let output_term_orig = if output_is_tty && let Some(fd) = output_fd {
            Some(safe_tcgetattr(fd)?)
        } else {
            None
        };

        Ok(RawModeInput {
            input,
            input_fd,
            input_term_orig,
            input_is_tty,
            output,
            output_fd,
            output_term_orig,
            output_is_tty,
        })
    }

    fn needs_terminal_configuration(&self) -> bool {
        self.input_is_tty
    }

    fn apply_terminal_configuration(&mut self) -> io::Result<()> {
        if self.input_is_tty
            && let Some(fd) = self.input_fd
        {
            let mut term = safe_tcgetattr(fd)?;
            term.c_lflag &= !(ECHO | ICANON | ECHONL | ISIG);
            term.c_cc[VMIN] = 1;
            term.c_cc[VTIME] = 0;
            safe_tcsetattr(fd, &mut term)?;
        }

        if self.output_is_tty
            && let Some(fd) = self.output_fd
        {
            let mut term = safe_tcgetattr(fd)?;
            term.c_lflag &= !(ECHO | ICANON | ECHONL | ISIG);
            term.c_cc[VMIN] = 1;
            term.c_cc[VTIME] = 0;
            safe_tcsetattr(fd, &mut term)?;
        }

        Ok(())
    }

    fn read_char(&mut self) -> std::io::Result<char> {
        read_char(&mut self.input)
    }

    fn write_output(&mut self, output: &str) -> std::io::Result<()> {
        self.output.write_all(output.as_bytes())?;
        self.output.flush()
    }

    fn send_signal_sigint(&mut self) -> io::Result<()> {
        if unsafe { libc::raise(libc::SIGINT) != 0 } {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::ConfigBuilder;
    use crate::read_password_with_config;

    #[test]
    fn test_read_password_with_config_errors_with_file_not_found() {
        let config = ConfigBuilder::new()
            .input_file_path("/does/not/exist")
            .output_discard()
            .build();

        // This should fail because the file does not exist
        let result = read_password_with_config(config);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.raw_os_error(), Some(libc::ENOENT));
    }
}
