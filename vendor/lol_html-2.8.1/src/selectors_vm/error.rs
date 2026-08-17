use cssparser::{BasicParseErrorKind, ParseErrorKind};
use selectors::parser::{SelectorParseError, SelectorParseErrorKind};
use thiserror::Error;

/// A CSS selector parsing error.
#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum SelectorError {
    /// Unexpected token in the selector.
    #[error("Unexpected token in selector.")]
    UnexpectedToken,

    /// Unexpected end of the selector.
    #[error("Unexpected end of selector.")]
    UnexpectedEnd,

    /// Missing attribute name in attribute selector.
    #[error("Missing attribute name in attribute selector.")]
    MissingAttributeName,

    /// The selector is empty.
    #[error("The selector is empty.")]
    EmptySelector,

    /// Dangling combinator in selector (e.g. `div >`).
    #[error("Dangling combinator in selector.")]
    DanglingCombinator,

    /// Unexpected token in the attribute selector.
    #[error("Unexpected token in the attribute selector.")]
    UnexpectedTokenInAttribute,

    /// Unsupported pseudo-class or pseudo-element in selector.
    #[error("Unsupported pseudo-class or pseudo-element in selector.")]
    UnsupportedPseudoClassOrElement,

    /// Nested negation in selector.
    #[error("Nested negation in selector.")]
    NestedNegation,

    /// Selectors with explicit namespaces are not supported.
    #[error("Selectors with explicit namespaces are not supported.")]
    NamespacedSelector,

    /// Invalid or unescaped class name in selector.
    #[error("Invalid or unescaped class name in selector.")]
    InvalidClassName,

    /// Unused
    #[error("Empty negation in selector.")]
    #[deprecated(note = "unused")]
    EmptyNegation,

    /// Unsupported combinator in the selector.
    #[error("Unsupported combinator `{0}` in selector.")]
    UnsupportedCombinator(char),

    /// CSS syntax in the selector which is yet unsupported.
    #[error("Unsupported syntax in selector.")]
    UnsupportedSyntax,
}

impl From<SelectorParseError<'_>> for SelectorError {
    #[cold]
    fn from(err: SelectorParseError<'_>) -> Self {
        // NOTE: always use explicit variants in this match, so we
        // get compile-time error if new error types were added to
        // the parser.
        #[deny(clippy::wildcard_enum_match_arm)]
        match err.kind {
            ParseErrorKind::Basic(err) => match err {
                BasicParseErrorKind::UnexpectedToken(_) => Self::UnexpectedToken,
                BasicParseErrorKind::EndOfInput => Self::UnexpectedEnd,
                BasicParseErrorKind::AtRuleBodyInvalid
                | BasicParseErrorKind::AtRuleInvalid(_)
                | BasicParseErrorKind::QualifiedRuleInvalid => Self::UnsupportedSyntax,
            },
            ParseErrorKind::Custom(err) => match err {
                SelectorParseErrorKind::NoQualifiedNameInAttributeSelector(_) => {
                    Self::MissingAttributeName
                }
                SelectorParseErrorKind::EmptySelector => Self::EmptySelector,
                SelectorParseErrorKind::DanglingCombinator => Self::DanglingCombinator,
                SelectorParseErrorKind::UnsupportedPseudoClassOrElement(_)
                | SelectorParseErrorKind::InvalidPseudoElementInsideWhere
                | SelectorParseErrorKind::NonPseudoElementAfterSlotted
                | SelectorParseErrorKind::InvalidPseudoElementAfterSlotted
                | SelectorParseErrorKind::PseudoElementExpectedColon(_)
                | SelectorParseErrorKind::PseudoElementExpectedIdent(_)
                | SelectorParseErrorKind::NoIdentForPseudo(_)
                // NOTE: according to the parser code this error occures only during
                // the parsing of vendor-specific pseudo-classes.
                | SelectorParseErrorKind::NonCompoundSelector => {
                    Self::UnsupportedPseudoClassOrElement
                }
                // NOTE: there are currently no cases in the parser code
                // that trigger this error.
                SelectorParseErrorKind::UnexpectedIdent(_) => {
                    debug_assert!(false);
                    Self::UnsupportedSyntax
                },
                SelectorParseErrorKind::ExpectedNamespace(_) => Self::NamespacedSelector,
                SelectorParseErrorKind::ExplicitNamespaceUnexpectedToken(_) => {
                    Self::UnexpectedToken
                }
                SelectorParseErrorKind::UnexpectedTokenInAttributeSelector(_)
                | SelectorParseErrorKind::ExpectedBarInAttr(_)
                | SelectorParseErrorKind::BadValueInAttr(_)
                | SelectorParseErrorKind::InvalidQualNameInAttr(_) => {
                    Self::UnexpectedTokenInAttribute
                }
                SelectorParseErrorKind::ClassNeedsIdent(_) => Self::InvalidClassName,
                SelectorParseErrorKind::InvalidState => {
                    debug_assert!(false, "invalid state");
                    Self::UnsupportedSyntax
                }
            },
        }
    }
}
