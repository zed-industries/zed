use crate::{formatting::FormattingFlags, parse_utils::StrRawness};

mod errors;

mod parsing;

#[cfg(test)]
mod tests;

pub(crate) use self::errors::{ParseError, ParseErrorKind};

#[derive(Debug, PartialEq)]
pub(crate) struct FormatStr {
    pub(crate) list: Vec<FmtStrComponent>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum FmtStrComponent {
    Str(String, StrRawness),
    Arg(FmtArg),
}

/// An argument in the format string eg: `"{foo:?}"`
#[derive(Debug, PartialEq)]
pub(crate) struct FmtArg {
    pub(crate) which_arg: WhichArg,
    pub(crate) formatting: FormattingFlags,
    pub(crate) rawness: StrRawness,
}

#[derive(Debug, PartialEq)]
pub(crate) enum WhichArg {
    Ident(String),
    Positional(Option<usize>),
}
