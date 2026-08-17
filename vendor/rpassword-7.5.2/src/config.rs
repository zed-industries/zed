use crate::DEFAULT_INPUT_PATH;
use crate::DEFAULT_OUTPUT_PATH;
use std::io::{Cursor, Read, Write};

/// Controls visual feedback when the user types a password.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub(crate) enum PasswordFeedback {
    /// Show nothing while typing (current default behavior).
    #[default]
    Hide,
    /// Show the given mask char for every character typed.
    /// e.g. `Mask('*')` shows stars.
    Mask(char),
    /// Show the actual character for the first N chars, then the given
    /// mask char for the rest.
    /// e.g. `PartialMask('*', 3)` shows first 3 chars in plaintext, then stars.
    PartialMask(char, usize),
}

/// Specifies the source for input.
pub(crate) enum InputTarget {
    FilePath(String),
    Reader(Box<dyn std::io::Read>),
}

/// Specifies the destination for output.
///
/// This enum defines where input is read from or where output is written to.
/// It supports file paths, in-memory cursors, or no input/output at all.
pub(crate) enum OutputTarget {
    FilePath(String),
    Writer(Box<dyn std::io::Write>),
    Void,
}

/// Configuration for prompting and reading a password.
///
/// Obtained from `ConfigBuilder::build()`.
pub struct Config {
    pub(crate) password_feedback: PasswordFeedback,
    pub(crate) input: InputTarget,
    pub(crate) output: OutputTarget,
}

/// A builder for creating a [`Config`].
///
/// This struct provides a convenient way to configure the behavior of password reading,
/// such as setting visual feedback, specifying an input path, discarding output, etc.
///
/// # Examples
///
/// ## Customising how the password is hidden
/// ```
/// use rpassword::{ConfigBuilder};
///
/// let config = ConfigBuilder::new()
///     .password_feedback_mask('*')
///     .password_feedback_partial_mask('*', 3)
///     .password_feedback_hide() // this is the default
///     .build();
/// ```
///
/// ## Setting custom input file path
/// ```
/// use rpassword::{ConfigBuilder};
///
/// let config = ConfigBuilder::new()
///     .input_file_path("path/to/file/containing/password")
///     .build();
/// ```
///
/// ## Reading from in-memory data
/// ```
/// use rpassword::{ConfigBuilder};
///
/// let config = ConfigBuilder::new()
///     .input_data("my-password\n")
///     .build();
/// ```
///
/// ## Reading from `impl Read`
/// ```
/// use std::io::Cursor;
/// use rpassword::{ConfigBuilder};
///
/// let config = ConfigBuilder::new()
///     .input_reader(Cursor::new("my-password\n"))
///     .build();
/// ```
///
/// ## Setting custom output file path
/// ```
/// use rpassword::{ConfigBuilder};
///
/// let config = ConfigBuilder::new()
///     .output_file_path("path/to/file/for/prompt/output")
///     .build();
/// ```
///
/// ## Writing to `impl Write`
/// ```
/// use std::io::Cursor;
/// use rpassword::{ConfigBuilder};
///
/// let config = ConfigBuilder::new()
///     .output_writer(Cursor::new(Vec::<u8>::new()))
///     .build();
/// ```
///
/// ## Discarding output
/// ```
/// use rpassword::{ConfigBuilder};
///
/// let config = ConfigBuilder::new()
///     .output_discard()
///     .build();
/// ```
pub struct ConfigBuilder {
    feedback: PasswordFeedback,
    input: InputTarget,
    output: OutputTarget,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder {
            feedback: PasswordFeedback::default(),
            input: InputTarget::FilePath(DEFAULT_INPUT_PATH.to_string()),
            output: OutputTarget::FilePath(DEFAULT_OUTPUT_PATH.to_string()),
        }
    }
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Sets the visual feedback to a mask with the given character.
    pub fn password_feedback_mask(self, mask: char) -> ConfigBuilder {
        ConfigBuilder {
            feedback: PasswordFeedback::Mask(mask),
            ..self
        }
    }

    /// Sets the visual feedback to a mask with the given character.
    pub fn password_feedback_partial_mask(self, mask: char, length: usize) -> ConfigBuilder {
        ConfigBuilder {
            feedback: PasswordFeedback::PartialMask(mask, length),
            ..self
        }
    }

    /// Sets the visual feedback none, hides the password entirely.
    pub fn password_feedback_hide(self) -> ConfigBuilder {
        ConfigBuilder {
            feedback: PasswordFeedback::Hide,
            ..self
        }
    }

    /// Reads the password from the file at the given path.
    pub fn input_file_path(self, file_path: impl Into<String>) -> ConfigBuilder {
        ConfigBuilder {
            input: InputTarget::FilePath(file_path.into()),
            ..self
        }
    }

    /// Reads the passwords from the data.
    pub fn input_data(self, data: impl Into<Vec<u8>>) -> ConfigBuilder {
        ConfigBuilder {
            input: InputTarget::Reader(Box::new(Cursor::new(data.into()))),
            ..self
        }
    }

    /// Reads the passwords from a custom Read implementation.
    pub fn input_reader(self, reader: impl Read + 'static) -> ConfigBuilder {
        ConfigBuilder {
            input: InputTarget::Reader(Box::new(reader)),
            ..self
        }
    }

    /// Sends the output to the file at the given path.
    pub fn output_file_path(self, file_path: impl Into<String>) -> ConfigBuilder {
        ConfigBuilder {
            output: OutputTarget::FilePath(file_path.into()),
            ..self
        }
    }

    /// Sends output to a custom Write implementation.
    pub fn output_writer(self, writer: impl Write + 'static) -> ConfigBuilder {
        ConfigBuilder {
            output: OutputTarget::Writer(Box::new(writer)),
            ..self
        }
    }

    /// Discards any output.
    pub fn output_discard(self) -> ConfigBuilder {
        ConfigBuilder {
            output: OutputTarget::Void,
            ..self
        }
    }

    /// Builds the final [`Config`].
    pub fn build(self) -> Config {
        Config {
            password_feedback: self.feedback,
            input: self.input,
            output: self.output,
        }
    }
}
