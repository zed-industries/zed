use std::error::Error as StdError;
use std::fmt::{self, Write};
use std::io::Error as IOError;
use std::string::FromUtf8Error;

use serde_json::error::Error as SerdeError;
use thiserror::Error;

#[cfg(feature = "dir_source")]
use walkdir::Error as WalkdirError;

#[cfg(feature = "script_helper")]
use rhai::{EvalAltResult, ParseError};

/// Error when rendering data on template.
#[derive(Debug)]
pub struct RenderError {
    pub template_name: Option<String>,
    pub line_no: Option<usize>,
    pub column_no: Option<usize>,
    reason: Box<RenderErrorReason>,
    unimplemented: bool,
    // backtrace: Backtrace,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let desc = self.reason.to_string();

        match (self.line_no, self.column_no) {
            (Some(line), Some(col)) => write!(
                f,
                "Error rendering \"{}\" line {}, col {}: {}",
                self.template_name.as_deref().unwrap_or("Unnamed template"),
                line,
                col,
                desc
            ),
            _ => write!(f, "{}", desc),
        }
    }
}

impl From<IOError> for RenderError {
    fn from(e: IOError) -> RenderError {
        RenderErrorReason::IOError(e).into()
    }
}

impl From<FromUtf8Error> for RenderError {
    fn from(e: FromUtf8Error) -> Self {
        RenderErrorReason::Utf8Error(e).into()
    }
}

impl From<TemplateError> for RenderError {
    fn from(e: TemplateError) -> Self {
        RenderErrorReason::TemplateError(e).into()
    }
}

/// Template rendering error
#[derive(Debug, Error)]
pub enum RenderErrorReason {
    #[error("Template not found {0}")]
    TemplateNotFound(String),
    #[error("Failed to parse template {0}")]
    TemplateError(
        #[from]
        #[source]
        TemplateError,
    ),
    #[error("Failed to access variable in strict mode {0:?}")]
    MissingVariable(Option<String>),
    #[error("Partial not found {0}")]
    PartialNotFound(String),
    #[error("Helper not found {0}")]
    HelperNotFound(String),
    #[error("Helper/Decorator {0} param at index {1} required but not found")]
    ParamNotFoundForIndex(&'static str, usize),
    #[error("Helper/Decorator {0} param with name {1} required but not found")]
    ParamNotFoundForName(&'static str, String),
    #[error("Helper/Decorator {0} param with name {1} type mismatch for {2}")]
    ParamTypeMismatchForName(&'static str, String, String),
    #[error("Helper/Decorator {0} hash with name {1} type mismatch for {2}")]
    HashTypeMismatchForName(&'static str, String, String),
    #[error("Decorator not found {0}")]
    DecoratorNotFound(String),
    #[error("Can not include current template in partial")]
    CannotIncludeSelf,
    #[error("Invalid logging level: {0}")]
    InvalidLoggingLevel(String),
    #[error("Invalid param type, {0} expected")]
    InvalidParamType(&'static str),
    #[error("Block content required")]
    BlockContentRequired,
    #[error("Invalid json path {0}")]
    InvalidJsonPath(String),
    #[error("Cannot access array/vector with string index, {0}")]
    InvalidJsonIndex(String),
    #[error("Failed to access JSON data: {0}")]
    SerdeError(
        #[from]
        #[source]
        SerdeError,
    ),
    #[error("IO Error: {0}")]
    IOError(
        #[from]
        #[source]
        IOError,
    ),
    #[error("FromUtf8Error: {0}")]
    Utf8Error(
        #[from]
        #[source]
        FromUtf8Error,
    ),
    #[error("Nested error: {0}")]
    NestedError(#[source] Box<dyn StdError + Send + Sync + 'static>),
    #[cfg(feature = "script_helper")]
    #[error("Cannot convert data to Rhai dynamic: {0}")]
    ScriptValueError(
        #[from]
        #[source]
        Box<EvalAltResult>,
    ),
    #[cfg(feature = "script_helper")]
    #[error("Failed to load rhai script: {0}")]
    ScriptLoadError(
        #[from]
        #[source]
        ScriptError,
    ),
    #[error("Unimplemented")]
    Unimplemented,
    #[error("{0}")]
    Other(String),
}

impl From<RenderErrorReason> for RenderError {
    fn from(e: RenderErrorReason) -> RenderError {
        RenderError {
            template_name: None,
            line_no: None,
            column_no: None,
            reason: Box::new(e),
            unimplemented: false,
        }
    }
}

impl RenderError {
    #[deprecated(since = "5.0.0", note = "Use RenderErrorReason instead")]
    pub fn new<T: AsRef<str>>(desc: T) -> RenderError {
        RenderErrorReason::Other(desc.as_ref().to_string()).into()
    }

    pub fn strict_error(path: Option<&String>) -> RenderError {
        RenderErrorReason::MissingVariable(path.map(|p| p.to_owned())).into()
    }

    #[deprecated(since = "5.0.0", note = "Use RenderErrorReason::NestedError instead")]
    pub fn from_error<E>(_error_info: &str, cause: E) -> RenderError
    where
        E: StdError + Send + Sync + 'static,
    {
        RenderErrorReason::NestedError(Box::new(cause)).into()
    }

    #[inline]
    pub(crate) fn is_unimplemented(&self) -> bool {
        matches!(*self.reason, RenderErrorReason::Unimplemented)
    }

    /// Get `RenderErrorReason` for this error
    pub fn reason(&self) -> &RenderErrorReason {
        self.reason.as_ref()
    }
}

impl StdError for RenderError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.reason())
    }
}

/// Template parsing error
#[derive(Debug, Error)]
pub enum TemplateErrorReason {
    #[error("helper {0:?} was opened, but {1:?} is closing")]
    MismatchingClosedHelper(String, String),
    #[error("decorator {0:?} was opened, but {1:?} is closing")]
    MismatchingClosedDecorator(String, String),
    #[error("invalid handlebars syntax: {0}")]
    InvalidSyntax(String),
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
    reason: Box<TemplateErrorReason>,
    template_name: Option<String>,
    line_no: Option<usize>,
    column_no: Option<usize>,
    segment: Option<String>,
}

impl TemplateError {
    #[allow(deprecated)]
    pub fn of(e: TemplateErrorReason) -> TemplateError {
        TemplateError {
            reason: Box::new(e),
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
    pub fn reason(&self) -> &TemplateErrorReason {
        &self.reason
    }

    /// Get the line number and column number of this error
    pub fn pos(&self) -> Option<(usize, usize)> {
        match (self.line_no, self.column_no) {
            (Some(line_no), Some(column_no)) => Some((line_no, column_no)),
            _ => None,
        }
    }

    /// Get template name of this error
    /// Returns `None` when the template has no associated name
    pub fn name(&self) -> Option<&String> {
        self.template_name.as_ref()
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
            let _ = writeln!(&mut buf, "{line_count:4} | {line_content}");
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
            (Some(line), Some(col), Some(seg)) => writeln!(
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_source_error() {
        let reason = RenderErrorReason::TemplateNotFound("unnamed".to_owned());
        let render_error = RenderError::from(reason);

        let reason2 = render_error.source().unwrap();
        assert!(matches!(
            reason2.downcast_ref::<RenderErrorReason>().unwrap(),
            RenderErrorReason::TemplateNotFound(_)
        ));
    }
}
