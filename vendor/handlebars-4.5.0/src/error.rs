use std::error::Error as StdError;
use std::fmt::{self, Write};
use std::io::Error as IOError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

use serde_json::error::Error as SerdeError;
use thiserror::Error;

#[cfg(feature = "dir_source")]
use walkdir::Error as WalkdirError;

#[cfg(feature = "script_helper")]
use rhai::{EvalAltResult, ParseError};

/// Error when rendering data on template.
#[derive(Debug, Default, Error)]
pub struct RenderError {
    pub desc: String,
    pub template_name: Option<String>,
    pub line_no: Option<usize>,
    pub column_no: Option<usize>,
    #[source]
    cause: Option<Box<dyn StdError + Send + Sync + 'static>>,
    unimplemented: bool,
    // backtrace: Backtrace,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match (self.line_no, self.column_no) {
            (Some(line), Some(col)) => write!(
                f,
                "Error rendering \"{}\" line {}, col {}: {}",
                self.template_name.as_deref().unwrap_or("Unnamed template"),
                line,
                col,
                self.desc
            ),
            _ => write!(f, "{}", self.desc),
        }
    }
}

impl From<IOError> for RenderError {
    fn from(e: IOError) -> RenderError {
        RenderError::from_error("Cannot generate output.", e)
    }
}

impl From<SerdeError> for RenderError {
    fn from(e: SerdeError) -> RenderError {
        RenderError::from_error("Failed to access JSON data.", e)
    }
}

impl From<FromUtf8Error> for RenderError {
    fn from(e: FromUtf8Error) -> RenderError {
        RenderError::from_error("Failed to generate bytes.", e)
    }
}

impl From<ParseIntError> for RenderError {
    fn from(e: ParseIntError) -> RenderError {
        RenderError::from_error("Cannot access array/vector with string index.", e)
    }
}

impl From<TemplateError> for RenderError {
    fn from(e: TemplateError) -> RenderError {
        RenderError::from_error("Failed to parse template.", e)
    }
}

#[cfg(feature = "script_helper")]
impl From<Box<EvalAltResult>> for RenderError {
    fn from(e: Box<EvalAltResult>) -> RenderError {
        RenderError::from_error("Cannot convert data to Rhai dynamic", e)
    }
}

#[cfg(feature = "script_helper")]
impl From<ScriptError> for RenderError {
    fn from(e: ScriptError) -> RenderError {
        RenderError::from_error("Failed to load rhai script", e)
    }
}

impl RenderError {
    pub fn new<T: AsRef<str>>(desc: T) -> RenderError {
        RenderError {
            desc: desc.as_ref().to_owned(),
            ..Default::default()
        }
    }

    pub(crate) fn unimplemented() -> RenderError {
        RenderError {
            unimplemented: true,
            ..Default::default()
        }
    }

    pub fn strict_error(path: Option<&String>) -> RenderError {
        let msg = match path {
            Some(path) => format!("Variable {:?} not found in strict mode.", path),
            None => "Value is missing in strict mode".to_owned(),
        };
        RenderError::new(&msg)
    }

    pub fn from_error<E>(error_info: &str, cause: E) -> RenderError
    where
        E: StdError + Send + Sync + 'static,
    {
        let mut e = RenderError::new(error_info);
        e.cause = Some(Box::new(cause));

        e
    }

    #[inline]
    pub(crate) fn is_unimplemented(&self) -> bool {
        self.unimplemented
    }
}

/// Template parsing error
#[derive(Debug, Error)]
pub enum TemplateErrorReason {
    #[error("helper {0:?} was opened, but {1:?} is closing")]
    MismatchingClosedHelper(String, String),
    #[error("decorator {0:?} was opened, but {1:?} is closing")]
    MismatchingClosedDecorator(String, String),
    #[error("invalid handlebars syntax.")]
    InvalidSyntax,
    #[error("invalid parameter {0:?}")]
    InvalidParam(String),
    #[error("nested subexpression is not supported")]
    NestedSubexpression,
    #[error("Template \"{1}\": {0}")]
    IoError(IOError, String),
    #[cfg(feature = "dir_source")]
    #[error("Walk dir error: {err}")]
    WalkdirError {
        #[from]
        err: WalkdirError,
    },
}

/// Error on parsing template.
#[derive(Debug, Error)]
pub struct TemplateError {
    #[deprecated(note = "public access to reason to be removed soon, use .reason() instead.")]
    pub reason: TemplateErrorReason,
    pub template_name: Option<String>,
    pub line_no: Option<usize>,
    pub column_no: Option<usize>,
    segment: Option<String>,
}

impl TemplateError {
    #[allow(deprecated)]
    pub fn of(e: TemplateErrorReason) -> TemplateError {
        TemplateError {
            reason: e,
            template_name: None,
            line_no: None,
            column_no: None,
            segment: None,
        }
    }

    pub fn at(mut self, template_str: &str, line_no: usize, column_no: usize) -> TemplateError {
        self.line_no = Some(line_no);
        self.column_no = Some(column_no);
        self.segment = Some(template_segment(template_str, line_no, column_no));
        self
    }

    pub fn in_template(mut self, name: String) -> TemplateError {
        self.template_name = Some(name);
        self
    }

    /// Get underlying reason for the error
    #[allow(deprecated)]
    pub fn reason(&self) -> &TemplateErrorReason {
        &self.reason
    }
}

impl From<(IOError, String)> for TemplateError {
    fn from(err_info: (IOError, String)) -> TemplateError {
        let (e, name) = err_info;
        TemplateError::of(TemplateErrorReason::IoError(e, name))
    }
}

#[cfg(feature = "dir_source")]
impl From<WalkdirError> for TemplateError {
    fn from(e: WalkdirError) -> TemplateError {
        TemplateError::of(TemplateErrorReason::from(e))
    }
}

fn template_segment(template_str: &str, line: usize, col: usize) -> String {
    let range = 3;
    let line_start = if line >= range { line - range } else { 0 };
    let line_end = line + range;

    let mut buf = String::new();
    for (line_count, line_content) in template_str.lines().enumerate() {
        if line_count >= line_start && line_count <= line_end {
            let _ = writeln!(&mut buf, "{:4} | {}", line_count, line_content);
            if line_count == line - 1 {
                buf.push_str("     |");
                for c in 0..line_content.len() {
                    if c != col {
                        buf.push('-');
                    } else {
                        buf.push('^');
                    }
                }
                buf.push('\n');
            }
        }
    }

    buf
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match (self.line_no, self.column_no, &self.segment) {
            (Some(line), Some(col), &Some(ref seg)) => writeln!(
                f,
                "Template error: {}\n    --> Template error in \"{}\":{}:{}\n     |\n{}     |\n     = reason: {}",
                self.reason(),
                self.template_name
                    .as_ref()
                    .unwrap_or(&"Unnamed template".to_owned()),
                line,
                col,
                seg,
                self.reason()
            ),
            _ => write!(f, "{}", self.reason()),
        }
    }
}

#[cfg(feature = "script_helper")]
#[derive(Debug, Error)]
pub enum ScriptError {
    #[error(transparent)]
    IoError(#[from] IOError),

    #[error(transparent)]
    ParseError(#[from] ParseError),
}
