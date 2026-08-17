use crate::libyaml::cstr::CStr;
use std::fmt::{self, Debug, Display};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use unsafe_libyaml as sys;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub(crate) struct Error {
    kind: sys::yaml_error_type_t,
    problem: CStr<'static>,
    problem_offset: u64,
    problem_mark: Mark,
    context: Option<CStr<'static>>,
    context_mark: Mark,
}

impl Error {
    pub unsafe fn parse_error(parser: *const sys::yaml_parser_t) -> Self {
        Error {
            kind: unsafe { (*parser).error },
            problem: match NonNull::new(unsafe { (*parser).problem as *mut _ }) {
                Some(problem) => unsafe { CStr::from_ptr(problem) },
                None => CStr::from_bytes_with_nul(b"libyaml parser failed but there is no error\0"),
            },
            problem_offset: unsafe { (*parser).problem_offset },
            problem_mark: Mark {
                sys: unsafe { (*parser).problem_mark },
            },
            context: match NonNull::new(unsafe { (*parser).context as *mut _ }) {
                Some(context) => Some(unsafe { CStr::from_ptr(context) }),
                None => None,
            },
            context_mark: Mark {
                sys: unsafe { (*parser).context_mark },
            },
        }
    }

    pub unsafe fn emit_error(emitter: *const sys::yaml_emitter_t) -> Self {
        Error {
            kind: unsafe { (*emitter).error },
            problem: match NonNull::new(unsafe { (*emitter).problem as *mut _ }) {
                Some(problem) => unsafe { CStr::from_ptr(problem) },
                None => {
                    CStr::from_bytes_with_nul(b"libyaml emitter failed but there is no error\0")
                }
            },
            problem_offset: 0,
            problem_mark: Mark {
                sys: unsafe { MaybeUninit::<sys::yaml_mark_t>::zeroed().assume_init() },
            },
            context: None,
            context_mark: Mark {
                sys: unsafe { MaybeUninit::<sys::yaml_mark_t>::zeroed().assume_init() },
            },
        }
    }

    pub fn mark(&self) -> Mark {
        self.problem_mark
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.problem)?;
        if self.problem_mark.sys.line != 0 || self.problem_mark.sys.column != 0 {
            write!(formatter, " at {}", self.problem_mark)?;
        } else if self.problem_offset != 0 {
            write!(formatter, " at position {}", self.problem_offset)?;
        }
        if let Some(context) = &self.context {
            write!(formatter, ", {}", context)?;
            if (self.context_mark.sys.line != 0 || self.context_mark.sys.column != 0)
                && (self.context_mark.sys.line != self.problem_mark.sys.line
                    || self.context_mark.sys.column != self.problem_mark.sys.column)
            {
                write!(formatter, " at {}", self.context_mark)?;
            }
        }
        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut formatter = formatter.debug_struct("Error");
        if let Some(kind) = match self.kind {
            sys::YAML_MEMORY_ERROR => Some("MEMORY"),
            sys::YAML_READER_ERROR => Some("READER"),
            sys::YAML_SCANNER_ERROR => Some("SCANNER"),
            sys::YAML_PARSER_ERROR => Some("PARSER"),
            sys::YAML_COMPOSER_ERROR => Some("COMPOSER"),
            sys::YAML_WRITER_ERROR => Some("WRITER"),
            sys::YAML_EMITTER_ERROR => Some("EMITTER"),
            _ => None,
        } {
            formatter.field("kind", &format_args!("{}", kind));
        }
        formatter.field("problem", &self.problem);
        if self.problem_mark.sys.line != 0 || self.problem_mark.sys.column != 0 {
            formatter.field("problem_mark", &self.problem_mark);
        } else if self.problem_offset != 0 {
            formatter.field("problem_offset", &self.problem_offset);
        }
        if let Some(context) = &self.context {
            formatter.field("context", context);
            if self.context_mark.sys.line != 0 || self.context_mark.sys.column != 0 {
                formatter.field("context_mark", &self.context_mark);
            }
        }
        formatter.finish()
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Mark {
    pub(super) sys: sys::yaml_mark_t,
}

impl Mark {
    pub fn index(&self) -> u64 {
        self.sys.index
    }

    pub fn line(&self) -> u64 {
        self.sys.line
    }

    pub fn column(&self) -> u64 {
        self.sys.column
    }
}

impl Display for Mark {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.sys.line != 0 || self.sys.column != 0 {
            write!(
                formatter,
                "line {} column {}",
                self.sys.line + 1,
                self.sys.column + 1,
            )
        } else {
            write!(formatter, "position {}", self.sys.index)
        }
    }
}

impl Debug for Mark {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut formatter = formatter.debug_struct("Mark");
        if self.sys.line != 0 || self.sys.column != 0 {
            formatter.field("line", &(self.sys.line + 1));
            formatter.field("column", &(self.sys.column + 1));
        } else {
            formatter.field("index", &self.sys.index);
        }
        formatter.finish()
    }
}
