use crate::{
    ExceptionId, LicenseId,
    error::{ParseError, Reason},
};

/// Parsing configuration for SPDX expression
#[derive(Default, Copy, Clone)]
pub struct ParseMode {
    /// The `AND`, `OR`, and `WITH` operators are required to be uppercase in
    /// the SPDX spec, but enabling this option allows them to be lowercased
    pub allow_lower_case_operators: bool,
    /// Allows the use of `/` as a synonym for the `OR` operator.
    ///
    /// This also allows for not having whitespace between the `/` and the terms
    /// on either side
    pub allow_slash_as_or_operator: bool,
    /// Allows some invalid/imprecise identifiers as synonyms for an actual
    /// license identifier.
    ///
    /// See [`IMPRECISE_NAMES`](crate::identifiers::IMPRECISE_NAMES) for a list
    /// of the current synonyms. Note that this list is not comprehensive but
    /// can be expanded upon when invalid identifiers are found in the wild.
    pub allow_imprecise_license_names: bool,
    /// The various GPL licenses diverge from every other license in the SPDX
    /// license list by having an `-or-later` variant that is used as a suffix
    /// on a base license (eg. `GPL-3.0-or-later`) rather than the canonical
    /// `GPL-3.0+`.
    ///
    /// This option just allows GPL licenses to be treated similarly to all of
    /// the other SPDX licenses.
    pub allow_postfix_plus_on_gpl: bool,
}

impl ParseMode {
    /// Strict, specification compliant SPDX parsing.
    ///
    /// 1. Only license identifiers in the SPDX license list, or
    ///    Document/LicenseRef, are allowed. The license identifiers are also
    ///    case-sensitive.
    /// 1. `WITH`, `AND`, and `OR` are the only valid operators
    pub const STRICT: Self = Self {
        allow_lower_case_operators: false,
        allow_slash_as_or_operator: false,
        allow_imprecise_license_names: false,
        allow_postfix_plus_on_gpl: false,
    };

    /// Allow non-conforming syntax for crates-io compatibility
    ///
    /// 1. Additional, invalid, identifiers are accepted and mapped to a correct
    ///    SPDX license identifier.
    ///    See [`IMPRECISE_NAMES`](crate::identifiers::IMPRECISE_NAMES) for the
    ///    list of additionally accepted identifiers and the license they
    ///    correspond to.
    /// 1. `/` can by used as a synonym for `OR`, and doesn't need to be
    ///    separated by whitespace from the terms it combines
    pub const LAX: Self = Self {
        allow_lower_case_operators: true,
        allow_slash_as_or_operator: true,
        allow_imprecise_license_names: true,
        allow_postfix_plus_on_gpl: true,
    };
}

/// A single token in an SPDX license expression
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    /// A recognized SPDX license id
    Spdx(LicenseId),
    /// A `LicenseRef-` prefixed id, with an optional `DocumentRef-`
    LicenseRef {
        doc_ref: Option<&'a str>,
        lic_ref: &'a str,
    },
    /// A recognized SPDX exception id
    Exception(ExceptionId),
    /// A postfix `+` indicating "or later" for a particular SPDX license id
    Plus,
    /// A `(` for starting a group
    OpenParen,
    /// A `)` for ending a group
    CloseParen,
    /// A `WITH` operator
    With,
    /// An `AND` operator
    And,
    /// An `OR` operator
    Or,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Token<'_> {
    fn len(&self) -> usize {
        match self {
            Token::Spdx(id) => id.name.len(),
            Token::Exception(e) => e.name.len(),
            Token::With => 4,
            Token::And => 3,
            Token::Or => 2,
            Token::Plus | Token::OpenParen | Token::CloseParen => 1,
            Token::LicenseRef { doc_ref, lic_ref } => {
                doc_ref.map_or(0, |d| {
                    // +1 is for the `:`
                    "DocumentRef-".len() + d.len() + 1
                }) + "LicenseRef-".len()
                    + lic_ref.len()
            }
        }
    }
}

/// Allows iteration through an SPDX license expression, yielding
/// a token or a `ParseError`.
///
/// Prefer to use `Expression::parse` or `Licensee::parse` rather
/// than directly using the lexer
pub struct Lexer<'a> {
    inner: &'a str,
    original: &'a str,
    offset: usize,
    mode: ParseMode,
}

impl<'a> Lexer<'a> {
    /// Creates a Lexer over a license expression
    #[must_use]
    pub fn new(text: &'a str) -> Self {
        Self {
            inner: text,
            original: text,
            offset: 0,
            mode: ParseMode::STRICT,
        }
    }

    /// Creates a Lexer over a license expression
    ///
    /// With `ParseMode::Lax` it allows non-conforming syntax
    /// used in crates-io crates.
    #[must_use]
    pub fn new_mode(text: &'a str, mode: ParseMode) -> Self {
        Self {
            inner: text,
            original: text,
            offset: 0,
            mode,
        }
    }

    #[inline]
    fn is_ref_char(c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '-' || *c == '.'
    }

    /// Return a matching text token if found - equivalent to the regex `^[-a-zA-Z0-9.:]+`
    fn find_text_token(text: &'a str) -> Option<&'a str> {
        let is_token_char = |c: &char| Self::is_ref_char(c) || *c == ':';
        match text.chars().take_while(is_token_char).count() {
            index if index > 0 => Some(&text[..index]),
            _ => None,
        }
    }

    /// Extract the text after `prefix` if made up of valid ref characters
    fn find_ref(prefix: &str, text: &'a str) -> Option<&'a str> {
        text.strip_prefix(prefix).map(|value| {
            let end = value.chars().take_while(Self::is_ref_char).count();
            &text[prefix.len()..prefix.len() + end]
        })
    }

    /// Return a license ref if found - equivalent to the regex `^LicenseRef-([-a-zA-Z0-9.]+)`
    #[inline]
    fn find_license_ref(text: &'a str) -> Option<&'a str> {
        Self::find_ref("LicenseRef-", text)
    }

    /// Return a document ref and license ref if found,
    /// equivalent to the regex `^DocumentRef-([-a-zA-Z0-9.]+):LicenseRef-([-a-zA-Z0-9.]+)`
    fn find_document_and_license_ref(text: &'a str) -> Option<(&'a str, &'a str)> {
        let split = text.split_once(':');
        let doc_ref = split.and_then(|(doc, _)| Self::find_ref("DocumentRef-", doc));
        let lic_ref = split.and_then(|(_, lic)| Self::find_license_ref(lic));
        Option::zip(doc_ref, lic_ref)
    }
}

/// A wrapper around a particular token that includes the span of the characters
/// in the original string, for diagnostic purposes
#[derive(Debug)]
pub struct LexerToken<'a> {
    /// The token that was lexed
    pub token: Token<'a>,
    /// The range of the token characters in the original license expression
    pub span: std::ops::Range<usize>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<LexerToken<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::unnecessary_wraps)]
        fn ok_token(token: Token<'_>) -> Option<Result<(Token<'_>, usize), ParseError>> {
            let len = token.len();
            Some(Ok((token, len)))
        }

        // Jump over any whitespace, updating `self.inner` and `self.offset` appropriately
        let non_whitespace_index = match self.inner.find(|c: char| !c.is_whitespace()) {
            Some(idx) => idx,
            None => self.inner.len(),
        };
        self.inner = &self.inner[non_whitespace_index..];
        self.offset += non_whitespace_index;

        match self.inner.chars().next() {
            None => None,
            // From SPDX 2.1 spec
            // There MUST NOT be whitespace between a license-id and any following "+".
            Some('+') => {
                if non_whitespace_index == 0 {
                    ok_token(Token::Plus)
                } else {
                    Some(Err(ParseError {
                        original: self.original.to_owned(),
                        span: self.offset - non_whitespace_index..self.offset,
                        reason: Reason::SeparatedPlus,
                    }))
                }
            }
            Some('(') => ok_token(Token::OpenParen),
            Some(')') => ok_token(Token::CloseParen),
            Some('/') if self.mode.allow_slash_as_or_operator => Some(Ok((Token::Or, 1))),
            Some(_) => match Lexer::find_text_token(self.inner) {
                None => Some(Err(ParseError {
                    original: self.original.to_owned(),
                    span: self.offset..self.offset + self.inner.len(),
                    reason: Reason::InvalidCharacters,
                })),
                Some(m) => {
                    if m == "WITH" {
                        ok_token(Token::With)
                    } else if m == "AND" {
                        ok_token(Token::And)
                    } else if m == "OR" {
                        ok_token(Token::Or)
                    } else if self.mode.allow_lower_case_operators && m == "and" {
                        ok_token(Token::And)
                    } else if self.mode.allow_lower_case_operators && m == "or" {
                        ok_token(Token::Or)
                    } else if self.mode.allow_lower_case_operators && m == "with" {
                        ok_token(Token::With)
                    } else if let Some(lic_id) = crate::license_id(m) {
                        ok_token(Token::Spdx(lic_id))
                    } else if let Some(exc_id) = crate::exception_id(m) {
                        ok_token(Token::Exception(exc_id))
                    } else if let Some((doc_ref, lic_ref)) = Lexer::find_document_and_license_ref(m)
                    {
                        ok_token(Token::LicenseRef {
                            doc_ref: Some(doc_ref),
                            lic_ref,
                        })
                    } else if let Some(lic_ref) = Lexer::find_license_ref(m) {
                        ok_token(Token::LicenseRef {
                            doc_ref: None,
                            lic_ref,
                        })
                    } else if let Some((lic_id, token_len)) =
                        if self.mode.allow_imprecise_license_names {
                            crate::imprecise_license_id(self.inner)
                        } else {
                            None
                        }
                    {
                        Some(Ok((Token::Spdx(lic_id), token_len)))
                    } else {
                        Some(Err(ParseError {
                            original: self.original.to_owned(),
                            span: self.offset..self.offset + m.len(),
                            reason: Reason::UnknownTerm,
                        }))
                    }
                }
            },
        }
        .map(|res| {
            res.map(|(tok, len)| {
                let start = self.offset;
                self.inner = &self.inner[len..];
                self.offset += len;

                LexerToken {
                    token: tok,
                    span: start..self.offset,
                }
            })
        })
    }
}
