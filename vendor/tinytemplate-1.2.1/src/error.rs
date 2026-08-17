//! Module containing the error type returned by TinyTemplate if an error occurs.

use instruction::{path_to_str, PathSlice};
use serde_json::Error as SerdeJsonError;
use serde_json::Value;
use std::error::Error as StdError;
use std::fmt;

/// Enum representing the potential errors that TinyTemplate can encounter.
#[derive(Debug)]
pub enum Error {
    ParseError {
        msg: String,
        line: usize,
        column: usize,
    },
    RenderError {
        msg: String,
        line: usize,
        column: usize,
    },
    SerdeError {
        err: SerdeJsonError,
    },
    GenericError {
        msg: String,
    },
    StdFormatError {
        err: fmt::Error,
    },
    CalledTemplateError {
        name: String,
        err: Box<Error>,
        line: usize,
        column: usize,
    },
    CalledFormatterError {
        name: String,
        err: Box<Error>,
        line: usize,
        column: usize,
    },

    #[doc(hidden)]
    __NonExhaustive,
}
impl From<SerdeJsonError> for Error {
    fn from(err: SerdeJsonError) -> Error {
        Error::SerdeError { err }
    }
}
impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Error {
        Error::StdFormatError { err }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError { msg, line, column } => write!(
                f,
                "Failed to parse the template (line {}, column {}). Reason: {}",
                line, column, msg
            ),
            Error::RenderError { msg, line, column } => {
                write!(
                    f,
                    "Encountered rendering error on line {}, column {}. Reason: {}",
                    line, column, msg
                )
            }
            Error::SerdeError { err } => {
                write!(f, "Unexpected serde error while converting the context to a serde_json::Value. Error: {}", err)
            }
            Error::GenericError { msg } => {
                write!(f, "{}", msg)
            }
            Error::StdFormatError { err } => {
                write!(f, "Unexpected formatting error: {}", err)
            }
            Error::CalledTemplateError {
                name,
                err,
                line,
                column,
            } => {
                write!(
                    f,
                    "Call to sub-template \"{}\" on line {}, column {} failed. Reason: {}",
                    name, line, column, err
                )
            }
            Error::CalledFormatterError {
                name,
                err,
                line,
                column,
            } => {
                write!(
                    f,
                    "Call to value formatter \"{}\" on line {}, column {} failed. Reason: {}",
                    name, line, column, err
                )
            }
            Error::__NonExhaustive => unreachable!(),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::ParseError { .. } => "ParseError",
            Error::RenderError { .. } => "RenderError",
            Error::SerdeError { .. } => "SerdeError",
            Error::GenericError { msg } => &msg,
            Error::StdFormatError { .. } => "StdFormatError",
            Error::CalledTemplateError { .. } => "CalledTemplateError",
            Error::CalledFormatterError { .. } => "CalledFormatterError",
            Error::__NonExhaustive => unreachable!(),
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub(crate) fn lookup_error(source: &str, step: &str, path: PathSlice, current: &Value) -> Error {
    let avail_str = if let Value::Object(object_map) = current {
        let mut avail_str = " Available values at this level are ".to_string();
        for (i, key) in object_map.keys().enumerate() {
            if i > 0 {
                avail_str.push_str(", ");
            }
            avail_str.push('\'');
            avail_str.push_str(key);
            avail_str.push('\'');
        }
        avail_str
    } else {
        "".to_string()
    };

    let (line, column) = get_offset(source, step);

    Error::RenderError {
        msg: format!(
            "Failed to find value '{}' from path '{}'.{}",
            step,
            path_to_str(path),
            avail_str
        ),
        line,
        column,
    }
}

pub(crate) fn truthiness_error(source: &str, path: PathSlice) -> Error {
    let (line, column) = get_offset(source, path.last().unwrap());
    Error::RenderError {
        msg: format!(
            "Path '{}' produced a value which could not be checked for truthiness.",
            path_to_str(path)
        ),
        line,
        column,
    }
}

pub(crate) fn unprintable_error() -> Error {
    Error::GenericError {
        msg: "Expected a printable value but found array or object.".to_string(),
    }
}

pub(crate) fn not_iterable_error(source: &str, path: PathSlice) -> Error {
    let (line, column) = get_offset(source, path.last().unwrap());
    Error::RenderError {
        msg: format!(
            "Expected an array for path '{}' but found a non-iterable value.",
            path_to_str(path)
        ),
        line,
        column,
    }
}

pub(crate) fn unknown_template(source: &str, name: &str) -> Error {
    let (line, column) = get_offset(source, name);
    Error::RenderError {
        msg: format!("Tried to call an unknown template '{}'", name),
        line,
        column,
    }
}

pub(crate) fn unknown_formatter(source: &str, name: &str) -> Error {
    let (line, column) = get_offset(source, name);
    Error::RenderError {
        msg: format!("Tried to call an unknown formatter '{}'", name),
        line,
        column,
    }
}

pub(crate) fn called_template_error(source: &str, template_name: &str, err: Error) -> Error {
    let (line, column) = get_offset(source, template_name);
    Error::CalledTemplateError {
        name: template_name.to_string(),
        err: Box::new(err),
        line,
        column,
    }
}

pub(crate) fn called_formatter_error(source: &str, formatter_name: &str, err: Error) -> Error {
    let (line, column) = get_offset(source, formatter_name);
    Error::CalledFormatterError {
        name: formatter_name.to_string(),
        err: Box::new(err),
        line,
        column,
    }
}

/// Find the line number and column of the target string within the source string. Will panic if
/// target is not a substring of source.
pub(crate) fn get_offset(source: &str, target: &str) -> (usize, usize) {
    let offset = target.as_ptr() as isize - source.as_ptr() as isize;
    let to_scan = &source[0..(offset as usize)];

    let mut line = 1;
    let mut column = 0;

    for byte in to_scan.bytes() {
        match byte as char {
            '\n' => {
                line += 1;
                column = 0;
            }
            _ => {
                column += 1;
            }
        }
    }

    (line, column)
}
