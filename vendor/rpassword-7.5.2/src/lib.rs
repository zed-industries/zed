//! This library makes it easy to read passwords in a console application on all platforms, Unix,
//! Windows, WASM, etc.
//!
//! Here's how you can read a password:
//! ```no_run
//! let password = rpassword::read_password().unwrap();
//! println!("Your password is {}", password);
//! ```
//!
//! You can also prompt for a password:
//! ```no_run
//! let password = rpassword::prompt_password("Your password: ").unwrap();
//! println!("Your password is {}", password);
//! ```
//!
//! For testing or custom use-cases, you can use `read_password_with_config` and `prompt_password_with_config`:
//! ```
//! use std::io::{Cursor, Write};
//!
//! let config = rpassword::ConfigBuilder::new()
//!     // Default input is the console, but we can pass any file path or raw data
//!     .input_data("my-password\n")
//!     // Default output is the console, but we can also discard it
//!     .output_discard()
//!     // Default behavior is to hide the password as it's being typed, but we can change that
//!     .password_feedback_mask('*')
//!     .build();
//!
//! let password = rpassword::read_password_with_config(config).unwrap();
//! println!("Your password is {}", password);
//! ```

use rtoolbox::fix_line_issues::fix_line_issues;
use rtoolbox::print_tty::print_writer;
use rtoolbox::safe_string::SafeString;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, Cursor, Write};

mod config;
mod feedback;

#[cfg(all(target_family = "unix", not(target_family = "wasm")))]
mod unix;
#[cfg(all(target_family = "unix", not(target_family = "wasm")))]
use unix::*;

#[cfg(target_family = "windows")]
mod windows;
#[cfg(target_family = "windows")]
use windows::*;

mod utf8;
#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(target_family = "wasm")]
use wasm::*;

use crate::config::{OutputTarget, PasswordFeedback};
use crate::feedback::FeedbackState;
pub use config::{Config, ConfigBuilder};

const BACKSPACE: char = '\x08';
const DEL: char = '\x7F';
const CTRL_C: char = '\x03';
const CTRL_D: char = '\x04';
const CTRL_U: char = '\x15';
const CTRL_W: char = '\x17';
const ESC: char = '\x1B';

trait RawPasswordInput {
    fn new(config: Config) -> io::Result<impl RawPasswordInput>;
    fn needs_terminal_configuration(&self) -> bool;
    fn apply_terminal_configuration(&mut self) -> io::Result<()>;
    fn read_char(&mut self) -> std::io::Result<char>;
    fn write_output(&mut self, output: &str) -> std::io::Result<()>;
    fn send_signal_sigint(&mut self) -> std::io::Result<()>;

    /// Reads a password from the console using the given config
    fn read_password(&mut self, password_feedback: PasswordFeedback) -> std::io::Result<String> {
        if self.needs_terminal_configuration() {
            self.apply_terminal_configuration()?;
        }

        let mut state = FeedbackState::new(password_feedback, self.needs_terminal_configuration());

        loop {
            let c = match self.read_char() {
                Ok(c) => c,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    }
                    return Err(e);
                }
            };
            match c {
                // LF / CR (Enter)
                '\n' | '\r' => {
                    let output = state.finish();
                    if !output.is_empty() {
                        self.write_output(output.as_str())?;
                    }
                    break;
                }
                // Backspace / DEL
                DEL | BACKSPACE => {
                    let output = state.pop_char();
                    if !output.is_empty() {
                        self.write_output(output.as_str())?;
                    }
                }
                // Ctrl-U: clear line
                CTRL_U => {
                    let output = state.clear();
                    if !output.is_empty() {
                        self.write_output(output.as_str())?;
                    }
                }
                // Ctrl-W: clear to last space
                CTRL_W => {
                    let output = state.clear_til_last_space();
                    if !output.is_empty() {
                        self.write_output(output.as_str())?;
                    }
                }
                // Ctrl-C: interrupt
                CTRL_C => {
                    let output = state.abort();
                    if !output.is_empty() {
                        self.write_output(output.as_str())?;
                    }
                    self.send_signal_sigint()?;
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Interrupted,
                        "interrupted",
                    ));
                }
                // Ctrl-D: EOF when empty
                CTRL_D => {
                    if state.is_empty() {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::UnexpectedEof,
                            "unexpected end of file",
                        ));
                    }
                }
                // ESC: consume and discard escape sequence like arrow keys
                ESC => {
                    let c = match self.read_char() {
                        Ok(c) => c,
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                                break;
                            }
                            return Err(e);
                        }
                    };

                    if c == '[' || c == 'O' {
                        // CSI (ESC [) or SS3 (ESC O): read until final byte (0x40-0x7E)
                        loop {
                            let c = match self.read_char() {
                                Ok(c) => c,
                                Err(e) => {
                                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                                        break;
                                    }
                                    return Err(e);
                                }
                            };
                            if ('\x40'..='\x7E').contains(&c) {
                                break;
                            }
                        }
                    }
                }
                c if !c.is_control() => {
                    let output = state.push_char(c);
                    if !output.is_empty() {
                        self.write_output(output.as_str())?;
                    }
                }
                // Discard unrecognized control characters and invalid input
                _ => {}
            }
        }

        Ok(state.into_password())
    }
}

/// Reads a password from `impl BufRead`.
///
/// **Deprecated**: This method is deprecated. Use `read_password_with_config` with a temporary file instead.
/// See the example below for updated usage.
///
/// # Example of Updated Usage
/// ```
/// use std::io::{Cursor, Write};
/// use rpassword::{read_password_with_config, ConfigBuilder};
///
/// let config = ConfigBuilder::new()
///     .input_reader(Cursor::new("my-password\n")) // anything that implements Read is OK here
///     .output_discard()
///     .build();
///
/// let password = read_password_with_config(config).unwrap();
/// println!("The typed password is: {}", password);
/// ```
#[deprecated(
    since = "7.5.0",
    note = "Use `read_password_with_config` with `ConfigBuilder::input_reader` instead."
)]
pub fn read_password_from_bufread(reader: &mut impl BufRead) -> std::io::Result<String> {
    let mut password = SafeString::new();
    reader.read_line(&mut password)?;

    fix_line_issues(password.into_inner())
}

/// Prompts on `impl Write` and then reads a password from `impl BufRead`.
///
/// **Deprecated**: This method is deprecated. Use `prompt_password_with_config` with a temporary file instead.
/// See the example below for updated usage.
///
/// # Example of Updated Usage
/// ```
/// use std::io::{Cursor, Write};
/// use rpassword::{prompt_password_with_config, ConfigBuilder};
///
/// let mut input = Cursor::new(b"my-password\n".to_vec());
///
/// let config = ConfigBuilder::new()
///     .input_reader(Cursor::new("my-password\n")) // anything that implements Read is OK here
///     .output_writer(Cursor::new(Vec::<u8>::new())) // anything that implements Write is OK here
///     .build();
///
/// let password = prompt_password_with_config("Your password: ", config).unwrap();
/// println!("The typed password is: {}", password);
/// ```
#[deprecated(
    since = "7.5.0",
    note = "Use `prompt_password_with_config` with `ConfigBuilder::input_reader` and `ConfigBuilder::output_writer()` instead."
)]
#[allow(deprecated)]
pub fn prompt_password_from_bufread(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    prompt: impl ToString,
) -> std::io::Result<String> {
    print_writer(writer, prompt.to_string().as_str())
        .and_then(|_| read_password_from_bufread(reader))
}

/// Reads a password from TTY using the given config
pub fn read_password_with_config(config: Config) -> std::io::Result<String> {
    let password_feedback = config.password_feedback;
    let mut raw_mode_input = RawModeInput::new(config)?;
    raw_mode_input.read_password(password_feedback)
}

/// Reads a password from the TTY
pub fn read_password() -> std::io::Result<String> {
    read_password_with_config(ConfigBuilder::default().build())
}

/// Prompts on the TTY and then reads a password from TTY
pub fn prompt_password(prompt: impl ToString) -> std::io::Result<String> {
    prompt_password_with_config(prompt, ConfigBuilder::new().build())
}

/// Prompts and then reads a password using the given config
pub fn prompt_password_with_config(
    prompt: impl ToString,
    mut config: Config,
) -> std::io::Result<String> {
    {
        // Create an inner scope to allow using the config without moving it
        // The mut ref to the config is dropped at the end of the scope
        let mut output: Box<dyn Write> = match &mut config.output {
            OutputTarget::FilePath(path) => Box::new(OpenOptions::new().write(true).open(path)?),
            OutputTarget::Writer(writer) => Box::new(writer),
            OutputTarget::Void => Box::new(Cursor::new(Vec::<u8>::new())), // TODO: Should use a SafeVec instead
        };
        output.write_all(prompt.to_string().as_bytes())?;
        output.flush()?;
    }

    read_password_with_config(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn mock_input_crlf() -> Cursor<&'static [u8]> {
        Cursor::new(&b"A mocked response.\r\nAnother mocked response.\r\n"[..])
    }

    fn mock_input_lf() -> Cursor<&'static [u8]> {
        Cursor::new(&b"A mocked response.\nAnother mocked response.\n"[..])
    }

    #[test]
    #[allow(deprecated)]
    fn can_read_from_redirected_input_many_times() {
        let mut reader_crlf = mock_input_crlf();

        let response = read_password_from_bufread(&mut reader_crlf).unwrap();
        assert_eq!(response, "A mocked response.");
        let response = read_password_from_bufread(&mut reader_crlf).unwrap();
        assert_eq!(response, "Another mocked response.");

        let mut reader_lf = mock_input_lf();
        let response = read_password_from_bufread(&mut reader_lf).unwrap();
        assert_eq!(response, "A mocked response.");
        let response = read_password_from_bufread(&mut reader_lf).unwrap();
        assert_eq!(response, "Another mocked response.");
    }

    #[test]
    fn test_read_password_with_config_with_input_file() {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(b"password\n").unwrap();
        let path = temp_file.path().to_str().unwrap().to_string();

        let config = ConfigBuilder::new()
            .input_file_path(path.as_str())
            .output_discard()
            .build();

        let result = read_password_with_config(config);
        assert_eq!("password", result.unwrap());
    }

    #[test]
    fn test_read_password_with_config_with_input_cursor() {
        let config = ConfigBuilder::new()
            .input_data("hello world\x7F\x7F\x7F\n")
            .output_discard()
            .build();

        let result = read_password_with_config(config);
        assert_eq!("hello wo", result.unwrap());
    }
}
