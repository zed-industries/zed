use crate::Token;
use crate::tokenizer;

/// Represents an error that occurred while parsing tokens.
#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    /// A parsing error occurred near the given token.
    #[error("syntax error near token `{}' (line {} col {})", .0.to_str(), .0.location().start.line, .0.location().start.column)]
    ParsingNearToken(Token),

    /// A parsing error occurred at the end of the input.
    #[error("syntax error at end of input")]
    ParsingAtEndOfInput,

    /// An error occurred while tokenizing the input stream.
    #[error("{} (detected near {})", .inner, .position.as_ref().map_or_else(|| String::from("<unknown position>"), |p| std::format!("line {} col {}", p.line, p.column)))]
    Tokenizing {
        /// The inner error.
        inner: tokenizer::TokenizerError,
        /// Optionally provides the position of the error.
        position: Option<tokenizer::SourcePosition>,
    },
}

#[cfg(feature = "diagnostics")]
#[allow(clippy::cast_sign_loss)]
#[allow(unused)] // Workaround unused warnings in nightly versions of the compiler
pub mod miette {
    use super::ParseError;
    use miette::SourceOffset;

    impl ParseError {
        /// Convert the original error to one miette can pretty print
        pub fn to_pretty_error(self, input: impl Into<String>) -> PrettyError {
            let input = input.into();
            let location = match self {
                Self::ParsingNearToken(ref token) => Some(SourceOffset::from_location(
                    &input,
                    token.location().start.line,
                    token.location().start.column,
                )),
                Self::Tokenizing { ref position, .. } => position
                    .as_ref()
                    .map(|p| SourceOffset::from_location(&input, p.line, p.column)),
                Self::ParsingAtEndOfInput => {
                    Some(SourceOffset::from_location(&input, usize::MAX, usize::MAX))
                }
            };

            PrettyError {
                cause: self,
                input,
                location,
            }
        }
    }

    /// Represents an error that occurred while parsing tokens.
    #[derive(thiserror::Error, Debug, miette::Diagnostic)]
    #[error("Cannot parse the input script")]
    pub struct PrettyError {
        cause: ParseError,
        #[source_code]
        input: String,
        #[label("{cause}")]
        location: Option<SourceOffset>,
    }
}

/// Represents a parsing error with its location information
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ParseErrorLocation {
    #[from]
    inner: peg::error::ParseError<peg::str::LineCol>,
}

/// Represents an error that occurred while parsing a word.
#[derive(Debug, thiserror::Error)]
pub enum WordParseError {
    /// An error occurred while parsing an arithmetic expression.
    #[error("failed to parse arithmetic expression")]
    ArithmeticExpression(ParseErrorLocation),

    /// An error occurred while parsing a shell pattern.
    #[error("failed to parse pattern")]
    Pattern(ParseErrorLocation),

    /// An error occurred while parsing a prompt string.
    #[error("failed to parse prompt string")]
    Prompt(ParseErrorLocation),

    /// An error occurred while parsing a parameter.
    #[error("failed to parse parameter '{0}'")]
    Parameter(String, ParseErrorLocation),

    /// An error occurred while parsing for brace expansion.
    #[error("failed to parse for brace expansion: '{0}'")]
    BraceExpansion(String, ParseErrorLocation),

    /// An error occurred while parsing a word.
    #[error("failed to parse word '{0}'")]
    Word(String, ParseErrorLocation),
}

/// Represents an error that occurred while parsing a (non-extended) test command.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct TestCommandParseError(#[from] peg::error::ParseError<usize>);

/// Represents an error that occurred while parsing a key-binding specification.
#[derive(Debug, thiserror::Error)]
pub enum BindingParseError {
    /// An unknown error occurred while parsing a key-binding specification.
    #[error("unknown error while parsing key-binding: '{0}'")]
    Unknown(String),

    /// A key code was missing from the key-binding specification.
    #[error("missing key code in key-binding")]
    MissingKeyCode,
}

pub(crate) fn convert_peg_parse_error(
    err: &peg::error::ParseError<usize>,
    tokens: &[Token],
) -> ParseError {
    let approx_token_index = err.location;

    if approx_token_index < tokens.len() {
        ParseError::ParsingNearToken(tokens[approx_token_index].clone())
    } else {
        ParseError::ParsingAtEndOfInput
    }
}
