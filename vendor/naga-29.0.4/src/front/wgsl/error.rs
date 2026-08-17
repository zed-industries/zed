//! Formatting WGSL front end error messages.

use crate::common::wgsl::TryToWgsl;
use crate::diagnostic_filter::ConflictingDiagnosticRuleError;
use crate::error::replace_control_chars;
use crate::proc::{Alignment, ConstantEvaluatorError, ResolveError};
use crate::{Scalar, SourceLocation, Span};

use super::parse::directive::enable_extension::{EnableExtension, UnimplementedEnableExtension};
use super::parse::directive::language_extension::{
    LanguageExtension, UnimplementedLanguageExtension,
};
use super::parse::lexer::Token;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use thiserror::Error;

use alloc::{
    borrow::Cow,
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::ops::Range;

#[derive(Clone, Debug)]
pub struct ParseError {
    message: String,
    // The first span should be the primary span, and the other ones should be complementary.
    labels: Vec<(Span, Cow<'static, str>)>,
    notes: Vec<String>,
}

impl ParseError {
    pub fn labels(&self) -> impl ExactSizeIterator<Item = (Span, &str)> + '_ {
        self.labels
            .iter()
            .map(|&(span, ref msg)| (span, msg.as_ref()))
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn diagnostic(&self) -> Diagnostic<()> {
        let diagnostic = Diagnostic::error()
            .with_message(self.message.to_string())
            .with_labels(
                self.labels
                    .iter()
                    .filter_map(|label| label.0.to_range().map(|range| (label, range)))
                    .map(|(label, range)| {
                        Label::primary((), range).with_message(label.1.to_string())
                    })
                    .collect(),
            )
            .with_notes(
                self.notes
                    .iter()
                    .map(|note| format!("note: {note}"))
                    .collect(),
            );
        diagnostic
    }

    /// Emits a summary of the error to standard error stream.
    #[cfg(feature = "stderr")]
    pub fn emit_to_stderr(&self, source: &str) {
        self.emit_to_stderr_with_path(source, "wgsl")
    }

    /// Emits a summary of the error to standard error stream.
    #[cfg(feature = "stderr")]
    pub fn emit_to_stderr_with_path<P>(&self, source: &str, path: P)
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref().display().to_string();
        let files = SimpleFile::new(path, replace_control_chars(source));
        let config = term::Config::default();

        cfg_if::cfg_if! {
            if #[cfg(feature = "termcolor")] {
                let writer = term::termcolor::StandardStream::stderr(term::termcolor::ColorChoice::Auto);
                term::emit_to_write_style(&mut writer.lock(), &config, &files, &self.diagnostic())
                    .expect("cannot write error");
            } else {
                let writer = std::io::stderr();
                term::emit_to_io_write(&mut writer.lock(), &config, &files, &self.diagnostic())
                    .expect("cannot write error");
            }
        }
    }

    /// Emits a summary of the error to a string.
    pub fn emit_to_string(&self, source: &str) -> String {
        self.emit_to_string_with_path(source, "wgsl")
    }

    /// Emits a summary of the error to a string.
    pub fn emit_to_string_with_path<P>(&self, source: &str, path: P) -> String
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref().display().to_string();
        let files = SimpleFile::new(path, replace_control_chars(source));
        let config = term::Config::default();

        let mut writer = crate::error::DiagnosticBuffer::new();
        writer
            .emit_to_self(&config, &files, &self.diagnostic())
            .expect("cannot write error");
        writer.into_string()
    }

    /// Returns a [`SourceLocation`] for the first label in the error message.
    pub fn location(&self, source: &str) -> Option<SourceLocation> {
        self.labels.first().map(|label| label.0.location(source))
    }
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl core::error::Error for ParseError {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExpectedToken<'a> {
    Token(Token<'a>),
    Identifier,
    AfterIdentListComma,
    AfterIdentListArg,
    /// LHS expression (identifier component_or_swizzle_specifier?, (`lhs_expression`) component_or_swizzle_specifier?, &`lhs_expression`, *`lhs_expression`)
    LhsExpression,
    /// Expected: constant, parenthesized expression, identifier
    PrimaryExpression,
    /// Expected: assignment, increment/decrement expression
    Assignment,
    /// Expected: 'case', 'default', '}'
    SwitchItem,
    /// Expected: ',', ')'
    WorkgroupSizeSeparator,
    /// Expected: 'struct', 'let', 'var', 'type', ';', 'fn', eof
    GlobalItem,
    /// Access of `var`, `let`, `const`.
    Variable,
    /// Access of a function
    Function,
    /// The `diagnostic` identifier of the `@diagnostic(…)` attribute.
    DiagnosticAttribute,
    /// statement
    Statement,
    /// for loop init statement (variable_or_value_statement, variable_updating_statement, func_call_statement)
    ForInit,
    /// for loop update statement (variable_updating_statement, func_call_statement)
    ForUpdate,
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum NumberError {
    #[error("invalid numeric literal format")]
    Invalid,
    #[error("numeric literal not representable by target type")]
    NotRepresentable,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InvalidAssignmentType {
    Other,
    Swizzle,
    ImmutableBinding(Span),
}

#[derive(Clone, Debug)]
pub(crate) enum Error<'a> {
    Unexpected(Span, ExpectedToken<'a>),
    UnexpectedComponents(Span),
    UnexpectedOperationInConstContext(Span),
    BadNumber(Span, NumberError),
    BadMatrixScalarKind(Span, Scalar),
    BadAccessor(Span),
    BadTexture(Span),
    BadTypeCast {
        span: Span,
        from_type: String,
        to_type: String,
    },
    NotStorageTexture(Span),
    BadTextureSampleType {
        span: Span,
        scalar: Scalar,
    },
    BadIncrDecrReferenceType(Span),
    InvalidResolve(ResolveError),
    /// A break if appeared outside of a continuing block
    InvalidBreakIf(Span),
    InvalidGatherComponent(Span),
    InvalidConstructorComponentType(Span, i32),
    InvalidIdentifierUnderscore(Span),
    ReservedIdentifierPrefix(Span),
    UnknownAddressSpace(Span),
    InvalidLocalVariableAddressSpace(Span),
    UnknownRayFlag(Span),
    RepeatedAttribute(Span),
    UnknownAttribute(Span),
    UnknownBuiltin(Span),
    UnknownAccess(Span),
    UnknownIdent(Span, &'a str),
    UnknownScalarType(Span),
    UnknownStorageFormat(Span),
    UnknownConservativeDepth(Span),
    UnknownEnableExtension(Span, &'a str),
    UnknownLanguageExtension(Span, &'a str),
    UnknownDiagnosticRuleName(Span),
    SizeAttributeTooLow(Span, u32),
    AlignAttributeTooLow(Span, Alignment),
    NonPowerOfTwoAlignAttribute(Span),
    InconsistentBinding(Span),
    TypeNotConstructible(Span),
    TypeNotInferable(Span),
    InitializationTypeMismatch {
        name: Span,
        expected: String,
        got: String,
    },
    DeclMissingTypeAndInit(Span),
    MissingAttribute(&'static str, Span),
    InvalidAddrOfOperand(Span),
    InvalidAtomicPointer(Span),
    InvalidAtomicOperandType(Span),
    InvalidRayQueryPointer(Span),
    NotPointer(Span),
    NotReference(&'static str, Span),
    InvalidAssignment {
        span: Span,
        ty: InvalidAssignmentType,
    },
    ReservedKeyword(Span),
    /// Redefinition of an identifier (used for both module-scope and local redefinitions).
    Redefinition {
        /// Span of the identifier in the previous definition.
        previous: Span,

        /// Span of the identifier in the new definition.
        current: Span,
    },
    /// A declaration refers to itself directly.
    RecursiveDeclaration {
        /// The location of the name of the declaration.
        ident: Span,

        /// The point at which it is used.
        usage: Span,
    },
    /// A declaration refers to itself indirectly, through one or more other
    /// definitions.
    CyclicDeclaration {
        /// The location of the name of some declaration in the cycle.
        ident: Span,

        /// The edges of the cycle of references.
        ///
        /// Each `(decl, reference)` pair indicates that the declaration whose
        /// name is `decl` has an identifier at `reference` whose definition is
        /// the next declaration in the cycle. The last pair's `reference` is
        /// the same identifier as `ident`, above.
        path: Box<[(Span, Span)]>,
    },
    InvalidSwitchSelector {
        span: Span,
    },
    InvalidSwitchCase {
        span: Span,
    },
    SwitchCaseTypeMismatch {
        span: Span,
    },
    CalledEntryPoint(Span),
    CalledLocalDecl(Span),
    WrongArgumentCount {
        span: Span,
        expected: Range<u32>,
        found: u32,
    },
    /// No overload of this function accepts this many arguments.
    TooManyArguments {
        /// The name of the function being called.
        function: String,

        /// The function name in the call expression.
        call_span: Span,

        /// The first argument that is unacceptable.
        arg_span: Span,

        /// Maximum number of arguments accepted by any overload of
        /// this function.
        max_arguments: u32,
    },
    /// A value passed to a builtin function has a type that is not
    /// accepted by any overload of the function.
    WrongArgumentType {
        /// The name of the function being called.
        function: String,

        /// The function name in the call expression.
        call_span: Span,

        /// The first argument whose type is unacceptable.
        arg_span: Span,

        /// The index of the first argument whose type is unacceptable.
        arg_index: u32,

        /// That argument's actual type.
        arg_ty: String,

        /// The set of argument types that would have been accepted for
        /// this argument, given the prior arguments.
        allowed: Vec<String>,
    },
    /// A value passed to a builtin function has a type that is not
    /// accepted, given the earlier arguments' types.
    InconsistentArgumentType {
        /// The name of the function being called.
        function: String,

        /// The function name in the call expression.
        call_span: Span,

        /// The first unacceptable argument.
        arg_span: Span,

        /// The index of the first unacceptable argument.
        arg_index: u32,

        /// The actual type of the first unacceptable argument.
        arg_ty: String,

        /// The prior argument whose type made the `arg_span` argument
        /// unacceptable.
        inconsistent_span: Span,

        /// The index of the `inconsistent_span` argument.
        inconsistent_index: u32,

        /// The type of the `inconsistent_span` argument.
        inconsistent_ty: String,

        /// The types that would have been accepted instead of the
        /// first unacceptable argument.
        allowed: Vec<String>,
    },
    FunctionReturnsVoid(Span),
    FunctionMustUseUnused(Span),
    FunctionMustUseReturnsVoid(Span, Span),
    InvalidWorkGroupUniformLoad(Span),
    Internal(&'static str),
    ExpectedConstExprConcreteIntegerScalar(Span),
    ExpectedNonNegative(Span),
    ExpectedPositiveArrayLength(Span),
    MissingWorkgroupSize(Span),
    ConstantEvaluatorError(Box<ConstantEvaluatorError>, Span),
    AutoConversion(Box<AutoConversionError>),
    AutoConversionLeafScalar(Box<AutoConversionLeafScalarError>),
    ConcretizationFailed(Box<ConcretizationFailedError>),
    ExceededLimitForNestedBraces {
        span: Span,
        limit: u8,
    },
    PipelineConstantIDValue(Span),
    NotBool(Span),
    ConstAssertFailed(Span),
    DirectiveAfterFirstGlobalDecl {
        directive_span: Span,
    },
    EnableExtensionNotYetImplemented {
        kind: UnimplementedEnableExtension,
        span: Span,
    },
    EnableExtensionNotEnabled {
        kind: EnableExtension,
        span: Span,
    },
    EnableExtensionNotSupported {
        kind: EnableExtension,
        span: Span,
    },
    LanguageExtensionNotYetImplemented {
        kind: UnimplementedLanguageExtension,
        span: Span,
    },
    DiagnosticInvalidSeverity {
        severity_control_name_span: Span,
    },
    DiagnosticDuplicateTriggeringRule(ConflictingDiagnosticRuleError),
    DiagnosticAttributeNotYetImplementedAtParseSite {
        site_name_plural: &'static str,
        spans: Vec<Span>,
    },
    DiagnosticAttributeNotSupported {
        on_what: DiagnosticAttributeNotSupportedPosition,
        spans: Vec<Span>,
    },
    SelectUnexpectedArgumentType {
        arg_span: Span,
        arg_type: String,
    },
    SelectRejectAndAcceptHaveNoCommonType {
        reject_span: Span,
        reject_type: String,
        accept_span: Span,
        accept_type: String,
    },
    ExpectedGlobalVariable {
        name_span: Span,
    },
    StructMemberTooLarge {
        member_name_span: Span,
    },
    TypeTooLarge {
        span: Span,
    },
    UnderspecifiedCooperativeMatrix,
    InvalidCooperativeLoadType(Span),
    UnsupportedCooperativeScalar(Span),
    UnexpectedIdentForEnumerant(Span),
    UnexpectedExprForEnumerant(Span),
    UnusedArgsForTemplate(Vec<Span>),
    UnexpectedTemplate(Span),
    MissingTemplateArg {
        span: Span,
        description: &'static str,
    },
    UnexpectedExprForTypeExpression(Span),
    MissingIncomingPayload(Span),
}

impl From<ConflictingDiagnosticRuleError> for Error<'_> {
    fn from(value: ConflictingDiagnosticRuleError) -> Self {
        Self::DiagnosticDuplicateTriggeringRule(value)
    }
}

/// Used for diagnostic refinement in [`Error::DiagnosticAttributeNotSupported`].
#[derive(Clone, Copy, Debug)]
pub(crate) enum DiagnosticAttributeNotSupportedPosition {
    SemicolonInModulePosition,
    Other { display_plural: &'static str },
}

impl From<&'static str> for DiagnosticAttributeNotSupportedPosition {
    fn from(display_plural: &'static str) -> Self {
        Self::Other { display_plural }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct AutoConversionError {
    pub dest_span: Span,
    pub dest_type: String,
    pub source_span: Span,
    pub source_type: String,
}

#[derive(Clone, Debug)]
pub(crate) struct AutoConversionLeafScalarError {
    pub dest_span: Span,
    pub dest_scalar: String,
    pub source_span: Span,
    pub source_type: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ConcretizationFailedError {
    pub expr_span: Span,
    pub expr_type: String,
    pub concretization_preferences: Vec<(String, ConstantEvaluatorError)>,
}

impl<'a> Error<'a> {
    #[cold]
    #[inline(never)]
    pub(crate) fn as_parse_error(&self, source: &'a str) -> ParseError {
        match *self {
            Error::Unexpected(unexpected_span, expected) => {
                let expected_str = match expected {
                    ExpectedToken::Token(token) => match token {
                        Token::Separator(c) => format!("`{c}`"),
                        Token::Paren(c) => format!("`{c}`"),
                        Token::Attribute => "@".to_string(),
                        Token::Number(_) => "number".to_string(),
                        Token::Word(s) => s.to_string(),
                        Token::Operation(c) => format!("operation (`{c}`)"),
                        Token::LogicalOperation(c) => format!("logical operation (`{c}`)"),
                        Token::ShiftOperation(c) => format!("bitshift (`{c}{c}`)"),
                        Token::AssignmentOperation(c) if c == '<' || c == '>' => {
                            format!("bitshift (`{c}{c}=`)")
                        }
                        Token::AssignmentOperation(c) => format!("operation (`{c}=`)"),
                        Token::IncrementOperation => "increment operation".to_string(),
                        Token::DecrementOperation => "decrement operation".to_string(),
                        Token::Arrow => "->".to_string(),
                        Token::TemplateArgsStart => "template args start".to_string(),
                        Token::TemplateArgsEnd => "template args end".to_string(),
                        Token::Unknown(c) => format!("unknown (`{c}`)"),
                        Token::Trivia => "trivia".to_string(),
                        Token::DocComment(s) => format!("doc comment ('{s}')"),
                        Token::ModuleDocComment(s) => format!("module doc comment ('{s}')"),
                        Token::End => "end".to_string(),
                    },
                    ExpectedToken::Identifier => "identifier".to_string(),
                    ExpectedToken::LhsExpression => "LHS expression (identifier component_or_swizzle_specifier?, (`lhs_expression`) component_or_swizzle_specifier?, &`lhs_expression`, *`lhs_expression`)".to_string(),
                    ExpectedToken::PrimaryExpression => "expression".to_string(),
                    ExpectedToken::Assignment => "assignment or increment/decrement".to_string(),
                    ExpectedToken::SwitchItem => concat!(
                        "switch item (`case` or `default`) or a closing curly bracket ",
                        "to signify the end of the switch statement (`}`)"
                    )
                    .to_string(),
                    ExpectedToken::WorkgroupSizeSeparator => {
                        "workgroup size separator (`,`) or a closing parenthesis".to_string()
                    }
                    ExpectedToken::GlobalItem => concat!(
                        "global item (`struct`, `const`, `var`, `alias`, ",
                        "`fn`, `diagnostic`, `enable`, `requires`, `;`) ",
                        "or the end of the file"
                    )
                    .to_string(),
                    ExpectedToken::Variable => "variable access".to_string(),
                    ExpectedToken::Function => "function name".to_string(),
                    ExpectedToken::AfterIdentListArg => {
                        "next argument, trailing comma, or end of list (`,` or `;`)".to_string()
                    }
                    ExpectedToken::AfterIdentListComma => {
                        "next argument or end of list (`;`)".to_string()
                    }
                    ExpectedToken::DiagnosticAttribute => {
                        "the `diagnostic` attribute identifier".to_string()
                    }
                    ExpectedToken::Statement => "statement".to_string(),
                    ExpectedToken::ForInit => "for loop initializer statement (`var`/`let`/`const` declaration, assignment, `i++`/`i--` statement, function call)".to_string(),
                    ExpectedToken::ForUpdate => "for loop update statement (assignment, `i++`/`i--` statement, function call)".to_string(),
                };
                ParseError {
                    message: format!(
                        "expected {}, found {:?}",
                        expected_str, &source[unexpected_span],
                    ),
                    labels: vec![(unexpected_span, format!("expected {expected_str}").into())],
                    notes: vec![],
                }
            }
            Error::UnexpectedComponents(bad_span) => ParseError {
                message: "unexpected components".to_string(),
                labels: vec![(bad_span, "unexpected components".into())],
                notes: vec![],
            },
            Error::UnexpectedOperationInConstContext(span) => ParseError {
                message: "this operation is not supported in a const context".to_string(),
                labels: vec![(span, "operation not supported here".into())],
                notes: vec![],
            },
            Error::BadNumber(bad_span, ref err) => ParseError {
                message: format!("{}: `{}`", err, &source[bad_span],),
                labels: vec![(bad_span, err.to_string().into())],
                notes: vec![],
            },
            Error::BadMatrixScalarKind(span, scalar) => ParseError {
                message: format!(
                    "matrix scalar type must be floating-point, but found `{}`",
                    scalar.to_wgsl_for_diagnostics()
                ),
                labels: vec![(span, "must be floating-point (e.g. `f32`)".into())],
                notes: vec![],
            },
            Error::BadAccessor(accessor_span) => ParseError {
                message: format!("invalid field accessor `{}`", &source[accessor_span],),
                labels: vec![(accessor_span, "invalid accessor".into())],
                notes: vec![],
            },
            Error::UnknownIdent(ident_span, ident) => ParseError {
                message: format!("no definition in scope for identifier: `{ident}`"),
                labels: vec![(ident_span, "unknown identifier".into())],
                notes: vec![],
            },
            Error::UnknownScalarType(bad_span) => ParseError {
                message: format!("unknown scalar type: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "unknown scalar type".into())],
                notes: vec!["Valid scalar types are f32, f64, i32, u32, bool".into()],
            },
            Error::NotStorageTexture(bad_span) => ParseError {
                message: "textureStore can only be applied to storage textures".to_string(),
                labels: vec![(bad_span, "not a storage texture".into())],
                notes: vec![],
            },
            Error::BadTextureSampleType { span, scalar } => ParseError {
                message: format!(
                    "texture sample type must be one of f32, i32 or u32, but found {}",
                    scalar.to_wgsl_for_diagnostics()
                ),
                labels: vec![(span, "must be one of f32, i32 or u32".into())],
                notes: vec![],
            },
            Error::BadIncrDecrReferenceType(span) => ParseError {
                message: concat!(
                    "increment/decrement operation requires ",
                    "reference type to be one of i32 or u32"
                )
                .to_string(),
                labels: vec![(span, "must be a reference type of i32 or u32".into())],
                notes: vec![],
            },
            Error::BadTexture(bad_span) => ParseError {
                message: format!(
                    "expected an image, but found `{}` which is not an image",
                    &source[bad_span]
                ),
                labels: vec![(bad_span, "not an image".into())],
                notes: vec![],
            },
            Error::BadTypeCast {
                span,
                ref from_type,
                ref to_type,
            } => {
                let msg = format!("cannot cast a {from_type} to a {to_type}");
                ParseError {
                    message: msg.clone(),
                    labels: vec![(span, msg.into())],
                    notes: vec![],
                }
            }
            Error::InvalidResolve(ref resolve_error) => ParseError {
                message: resolve_error.to_string(),
                labels: vec![],
                notes: vec![],
            },
            Error::InvalidBreakIf(bad_span) => ParseError {
                message: "A break if is only allowed in a continuing block".to_string(),
                labels: vec![(bad_span, "not in a continuing block".into())],
                notes: vec![],
            },
            Error::InvalidGatherComponent(bad_span) => ParseError {
                message: format!(
                    "textureGather component `{}` doesn't exist, must be 0, 1, 2, or 3",
                    &source[bad_span]
                ),
                labels: vec![(bad_span, "invalid component".into())],
                notes: vec![],
            },
            Error::InvalidConstructorComponentType(bad_span, component) => ParseError {
                message: format!("invalid type for constructor component at index [{component}]"),
                labels: vec![(bad_span, "invalid component type".into())],
                notes: vec![],
            },
            Error::InvalidIdentifierUnderscore(bad_span) => ParseError {
                message: "Identifier can't be `_`".to_string(),
                labels: vec![(bad_span, "invalid identifier".into())],
                notes: vec![
                    "Use phony assignment instead (`_ =` notice the absence of `let` or `var`)"
                        .to_string(),
                ],
            },
            Error::ReservedIdentifierPrefix(bad_span) => ParseError {
                message: format!(
                    "Identifier starts with a reserved prefix: `{}`",
                    &source[bad_span]
                ),
                labels: vec![(bad_span, "invalid identifier".into())],
                notes: vec![],
            },
            Error::UnknownAddressSpace(bad_span) => ParseError {
                message: format!("unknown address space: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "unknown address space".into())],
                notes: vec![],
            },
            Error::InvalidLocalVariableAddressSpace(bad_span) => ParseError {
                message: format!("invalid address space for local variable: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "local variables can only use 'function' address space".into())],
                notes: vec![],
            },
            Error::UnknownRayFlag(bad_span) => ParseError {
                message: format!("unknown ray flag: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "unknown ray flag".into())],
                notes: vec![],
            },
            Error::RepeatedAttribute(bad_span) => ParseError {
                message: format!("repeated attribute: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "repeated attribute".into())],
                notes: vec![],
            },
            Error::UnknownAttribute(bad_span) => ParseError {
                message: format!("unknown attribute: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "unknown attribute".into())],
                notes: vec![],
            },
            Error::UnknownBuiltin(bad_span) => ParseError {
                message: format!("unknown builtin: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "unknown builtin".into())],
                notes: vec![],
            },
            Error::UnknownAccess(bad_span) => ParseError {
                message: format!("unknown access: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "unknown access".into())],
                notes: vec![],
            },
            Error::UnknownStorageFormat(bad_span) => ParseError {
                message: format!("unknown storage format: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "unknown storage format".into())],
                notes: vec![],
            },
            Error::UnknownConservativeDepth(bad_span) => ParseError {
                message: format!("unknown conservative depth: `{}`", &source[bad_span]),
                labels: vec![(bad_span, "unknown conservative depth".into())],
                notes: vec![],
            },
            Error::UnknownEnableExtension(span, word) => ParseError {
                message: format!("unknown enable-extension `{word}`"),
                labels: vec![(span, "".into())],
                notes: vec![
                    "See available extensions at <https://www.w3.org/TR/WGSL/#enable-extension>."
                        .into(),
                ],
            },
            Error::UnknownLanguageExtension(span, name) => ParseError {
                message: format!("unknown language extension `{name}`"),
                labels: vec![(span, "".into())],
                notes: vec![concat!(
                    "See available extensions at ",
                    "<https://www.w3.org/TR/WGSL/#language-extensions-sec>."
                )
                .into()],
            },
            Error::UnknownDiagnosticRuleName(span) => ParseError {
                message: format!("unknown `diagnostic(…)` rule name `{}`", &source[span]),
                labels: vec![(span, "not a valid diagnostic rule name".into())],
                notes: vec![concat!(
                    "See available trigger rules at ",
                    "<https://www.w3.org/TR/WGSL/#filterable-triggering-rules>."
                )
                .into()],
            },
            Error::SizeAttributeTooLow(bad_span, min_size) => ParseError {
                message: format!("struct member size must be at least {min_size}"),
                labels: vec![(bad_span, format!("must be at least {min_size}").into())],
                notes: vec![],
            },
            Error::AlignAttributeTooLow(bad_span, min_align) => ParseError {
                message: format!("struct member alignment must be at least {min_align}"),
                labels: vec![(bad_span, format!("must be at least {min_align}").into())],
                notes: vec![],
            },
            Error::NonPowerOfTwoAlignAttribute(bad_span) => ParseError {
                message: "struct member alignment must be a power of 2".to_string(),
                labels: vec![(bad_span, "must be a power of 2".into())],
                notes: vec![],
            },
            Error::InconsistentBinding(span) => ParseError {
                message: "input/output binding is not consistent".to_string(),
                labels: vec![(span, "input/output binding is not consistent".into())],
                notes: vec![],
            },
            Error::TypeNotConstructible(span) => ParseError {
                message: format!("type `{}` is not constructible", &source[span]),
                labels: vec![(span, "type is not constructible".into())],
                notes: vec![],
            },
            Error::TypeNotInferable(span) => ParseError {
                message: "type can't be inferred".to_string(),
                labels: vec![(span, "type can't be inferred".into())],
                notes: vec![],
            },
            Error::InitializationTypeMismatch {
                name,
                ref expected,
                ref got,
            } => ParseError {
                message: format!(
                    "the type of `{}` is expected to be `{}`, but got `{}`",
                    &source[name], expected, got,
                ),
                labels: vec![(name, format!("definition of `{}`", &source[name]).into())],
                notes: vec![],
            },
            Error::DeclMissingTypeAndInit(name_span) => ParseError {
                message: format!(
                    "declaration of `{}` needs a type specifier or initializer",
                    &source[name_span]
                ),
                labels: vec![(name_span, "needs a type specifier or initializer".into())],
                notes: vec![],
            },
            Error::MissingAttribute(name, name_span) => ParseError {
                message: format!(
                    "variable `{}` needs a '{}' attribute",
                    &source[name_span], name
                ),
                labels: vec![(
                    name_span,
                    format!("definition of `{}`", &source[name_span]).into(),
                )],
                notes: vec![],
            },
            Error::InvalidAddrOfOperand(span) => ParseError {
                message: "cannot take the address of a vector component".to_string(),
                labels: vec![(span, "invalid operand for address-of".into())],
                notes: vec![],
            },
            Error::InvalidAtomicPointer(span) => ParseError {
                message: "atomic operation is done on a pointer to a non-atomic".to_string(),
                labels: vec![(span, "atomic pointer is invalid".into())],
                notes: vec![],
            },
            Error::InvalidAtomicOperandType(span) => ParseError {
                message: "atomic operand type is inconsistent with the operation".to_string(),
                labels: vec![(span, "atomic operand type is invalid".into())],
                notes: vec![],
            },
            Error::InvalidRayQueryPointer(span) => ParseError {
                message: "ray query operation is done on a pointer to a non-ray-query".to_string(),
                labels: vec![(span, "ray query pointer is invalid".into())],
                notes: vec![],
            },
            Error::NotPointer(span) => ParseError {
                message: "the operand of the `*` operator must be a pointer".to_string(),
                labels: vec![(span, "expression is not a pointer".into())],
                notes: vec![],
            },
            Error::NotReference(what, span) => ParseError {
                message: format!("{what} must be a reference"),
                labels: vec![(span, "expression is not a reference".into())],
                notes: vec![],
            },
            Error::InvalidAssignment { span, ty } => {
                let (extra_label, notes) = match ty {
                    InvalidAssignmentType::Swizzle => (
                        None,
                        vec![
                            "WGSL does not support assignments to swizzles".into(),
                            "consider assigning each component individually".into(),
                        ],
                    ),
                    InvalidAssignmentType::ImmutableBinding(binding_span) => (
                        Some((binding_span, "this is an immutable binding".into())),
                        vec![format!(
                            "consider declaring `{}` with `var` instead of `let`",
                            &source[binding_span]
                        )],
                    ),
                    InvalidAssignmentType::Other => (None, vec![]),
                };

                ParseError {
                    message: "invalid left-hand side of assignment".into(),
                    labels: core::iter::once((span, "cannot assign to this expression".into()))
                        .chain(extra_label)
                        .collect(),
                    notes,
                }
            }
            Error::ReservedKeyword(name_span) => ParseError {
                message: format!("name `{}` is a reserved keyword", &source[name_span]),
                labels: vec![(
                    name_span,
                    format!("definition of `{}`", &source[name_span]).into(),
                )],
                notes: vec![],
            },
            Error::Redefinition { previous, current } => ParseError {
                message: format!("redefinition of `{}`", &source[current]),
                labels: vec![
                    (
                        current,
                        format!("redefinition of `{}`", &source[current]).into(),
                    ),
                    (
                        previous,
                        format!("previous definition of `{}`", &source[previous]).into(),
                    ),
                ],
                notes: vec![],
            },
            Error::RecursiveDeclaration { ident, usage } => ParseError {
                message: format!("declaration of `{}` is recursive", &source[ident]),
                labels: vec![(ident, "".into()), (usage, "uses itself here".into())],
                notes: vec![],
            },
            Error::CyclicDeclaration { ident, ref path } => ParseError {
                message: format!("declaration of `{}` is cyclic", &source[ident]),
                labels: path
                    .iter()
                    .enumerate()
                    .flat_map(|(i, &(ident, usage))| {
                        [
                            (ident, "".into()),
                            (
                                usage,
                                if i == path.len() - 1 {
                                    "ending the cycle".into()
                                } else {
                                    format!("uses `{}`", &source[ident]).into()
                                },
                            ),
                        ]
                    })
                    .collect(),
                notes: vec![],
            },
            Error::InvalidSwitchSelector { span } => ParseError {
                message: "invalid `switch` selector".to_string(),
                labels: vec![(
                    span,
                    "`switch` selector must be a scalar integer"
                    .into(),
                )],
                notes: vec![],
            },
            Error::InvalidSwitchCase { span } => ParseError {
                message: "invalid `switch` case selector value".to_string(),
                labels: vec![(
                    span,
                    "`switch` case selector must be a scalar integer const expression"
                    .into(),
                )],
                notes: vec![],
            },
            Error::SwitchCaseTypeMismatch { span } => ParseError {
                message: "invalid `switch` case selector value".to_string(),
                labels: vec![(
                    span,
                    "`switch` case selector must have the same type as the `switch` selector expression"
                    .into(),
                )],
                notes: vec![],
            },
            Error::CalledEntryPoint(span) => ParseError {
                message: "entry point cannot be called".to_string(),
                labels: vec![(span, "entry point cannot be called".into())],
                notes: vec![],
            },
            Error::CalledLocalDecl(span) => ParseError {
                message: "local declaration cannot be called".to_string(),
                labels: vec![(span, "local declaration cannot be called".into())],
                notes: vec![],
            },
            Error::WrongArgumentCount {
                span,
                ref expected,
                found,
            } => ParseError {
                message: format!(
                    "wrong number of arguments: expected {}, found {}",
                    if expected.len() < 2 {
                        format!("{}", expected.start)
                    } else {
                        format!("{}..{}", expected.start, expected.end)
                    },
                    found
                ),
                labels: vec![(span, "wrong number of arguments".into())],
                notes: vec![],
            },
            Error::TooManyArguments {
                ref function,
                call_span,
                arg_span,
                max_arguments,
            } => ParseError {
                message: format!("too many arguments passed to `{function}`"),
                labels: vec![
                    (call_span, "".into()),
                    (arg_span, format!("unexpected argument #{}", max_arguments + 1).into())
                ],
                notes: vec![
                    format!("The `{function}` function accepts at most {max_arguments} argument(s)")
                ],
            },
            Error::WrongArgumentType {
                ref function,
                call_span,
                arg_span,
                arg_index,
                ref arg_ty,
                ref allowed,
            } => {
                let message = format!(
                    "wrong type passed as argument #{} to `{function}`",
                    arg_index + 1,
                );
                let labels = vec![
                    (call_span, "".into()),
                    (arg_span, format!("argument #{} has type `{arg_ty}`", arg_index + 1).into())
                ];

                let mut notes = vec![];
                notes.push(format!("`{function}` accepts the following types for argument #{}:", arg_index + 1));
                notes.extend(allowed.iter().map(|ty| format!("allowed type: {ty}")));

                ParseError { message, labels, notes }
            },
            Error::InconsistentArgumentType {
                ref function,
                call_span,
                arg_span,
                arg_index,
                ref arg_ty,
                inconsistent_span,
                inconsistent_index,
                ref inconsistent_ty,
                ref allowed
            } => {
                let message = format!(
                    "inconsistent type passed as argument #{} to `{function}`",
                    arg_index + 1,
                );
                let labels = vec![
                    (call_span, "".into()),
                    (arg_span, format!("argument #{} has type {arg_ty}", arg_index + 1).into()),
                    (inconsistent_span, format!(
                        "this argument has type {inconsistent_ty}, which constrains subsequent arguments"
                    ).into()),
                ];
                let mut notes = vec![
                    format!("Because argument #{} has type {inconsistent_ty}, only the following types", inconsistent_index + 1),
                    format!("(or types that automatically convert to them) are accepted for argument #{}:", arg_index + 1),
                ];
                notes.extend(allowed.iter().map(|ty| format!("allowed type: {ty}")));

                ParseError { message, labels, notes }
            }
            Error::FunctionReturnsVoid(span) => ParseError {
                message: "function does not return any value".to_string(),
                labels: vec![(span, "".into())],
                notes: vec![
                    "perhaps you meant to call the function in a separate statement?".into(),
                ],
            },
            Error::FunctionMustUseUnused(call) => ParseError {
                message: "unused return value from function annotated with @must_use".into(),
                labels: vec![(call, "".into())],
                notes: vec![
                    format!(
                        "function '{}' is declared with `@must_use` attribute",
                        &source[call],
                    ),
                    "use a phony assignment or declare a value using the function call as the initializer".into(),
                ],
            },
            Error::FunctionMustUseReturnsVoid(attr, signature) => ParseError {
                message: "function annotated with @must_use but does not return any value".into(),
                labels: vec![
                    (attr, "".into()),
                    (signature, "".into()),
                ],
                notes: vec![
                    "declare a return type or remove the attribute".into(),
                ],
            },
            Error::InvalidWorkGroupUniformLoad(span) => ParseError {
                message: "incorrect type passed to workgroupUniformLoad".into(),
                labels: vec![(span, "".into())],
                notes: vec!["passed type must be a workgroup pointer".into()],
            },
            Error::Internal(message) => ParseError {
                message: "internal WGSL front end error".to_string(),
                labels: vec![],
                notes: vec![message.into()],
            },
            Error::ExpectedConstExprConcreteIntegerScalar(span) => ParseError {
                message: concat!(
                    "must be a const-expression that ",
                    "resolves to a concrete integer scalar (`u32` or `i32`)"
                )
                .to_string(),
                labels: vec![(span, "must resolve to `u32` or `i32`".into())],
                notes: vec![],
            },
            Error::ExpectedNonNegative(span) => ParseError {
                message: "must be non-negative (>= 0)".to_string(),
                labels: vec![(span, "must be non-negative".into())],
                notes: vec![],
            },
            Error::ExpectedPositiveArrayLength(span) => ParseError {
                message: "array element count must be positive (> 0)".to_string(),
                labels: vec![(span, "must be positive".into())],
                notes: vec![],
            },
            Error::ConstantEvaluatorError(ref e, span) => ParseError {
                message: e.to_string(),
                labels: vec![(span, "see msg".into())],
                notes: vec![],
            },
            Error::MissingWorkgroupSize(span) => ParseError {
                message: "workgroup size is missing on compute shader entry point".to_string(),
                labels: vec![(
                    span,
                    "must be paired with a `@workgroup_size` attribute".into(),
                )],
                notes: vec![],
            },
            Error::AutoConversion(ref error) => {
                // destructuring ensures all fields are handled
                let AutoConversionError {
                    dest_span,
                    ref dest_type,
                    source_span,
                    ref source_type,
                } = **error;
                ParseError {
                    message: format!(
                        "automatic conversions cannot convert `{source_type}` to `{dest_type}`"
                    ),
                    labels: vec![
                        (
                            dest_span,
                            format!("a value of type {dest_type} is required here").into(),
                        ),
                        (
                            source_span,
                            format!("this expression has type {source_type}").into(),
                        ),
                    ],
                    notes: vec![],
                }
            }
            Error::AutoConversionLeafScalar(ref error) => {
                let AutoConversionLeafScalarError {
                    dest_span,
                    ref dest_scalar,
                    source_span,
                    ref source_type,
                } = **error;
                ParseError {
                    message: format!(
                        "automatic conversions cannot convert elements of `{source_type}` to `{dest_scalar}`"
                    ),
                    labels: vec![
                        (
                            dest_span,
                            format!(
                                "a value with elements of type {dest_scalar} is required here"
                            )
                            .into(),
                        ),
                        (
                            source_span,
                            format!("this expression has type {source_type}").into(),
                        ),
                    ],
                    notes: vec![],
                }
            }
            Error::ConcretizationFailed(ref error) => {
                let ConcretizationFailedError {
                    expr_span,
                    ref expr_type,
                    ref concretization_preferences,
                } = **error;
                ParseError {
                    message: "failed to convert expression to a concrete type".to_string(),
                    labels: vec![(
                        expr_span,
                        format!("this expression has type {expr_type}").into(),
                    )],
                    notes: concretization_preferences
                        .iter()
                        .map(|&(ref scalar, ref err)|
                            format!("the expression couldn't be converted to have {scalar} scalar type: {err}")
                        )
                        .collect(),
                }
            }
            Error::ExceededLimitForNestedBraces { span, limit } => ParseError {
                message: "brace nesting limit reached".into(),
                labels: vec![(span, "limit reached at this brace".into())],
                notes: vec![format!("nesting limit is currently set to {limit}")],
            },
            Error::PipelineConstantIDValue(span) => ParseError {
                message: "pipeline constant ID must be between 0 and 65535 inclusive".to_string(),
                labels: vec![(span, "must be between 0 and 65535 inclusive".into())],
                notes: vec![],
            },
            Error::NotBool(span) => ParseError {
                message: "must be a const-expression that resolves to a `bool`".to_string(),
                labels: vec![(span, "must resolve to `bool`".into())],
                notes: vec![],
            },
            Error::ConstAssertFailed(span) => ParseError {
                message: "`const_assert` failure".to_string(),
                labels: vec![(span, "evaluates to `false`".into())],
                notes: vec![],
            },
            Error::DirectiveAfterFirstGlobalDecl { directive_span } => ParseError {
                message: "expected global declaration, but found a global directive".into(),
                labels: vec![(
                    directive_span,
                    "written after first global declaration".into(),
                )],
                notes: vec![concat!(
                    "global directives are only allowed before global declarations; ",
                    "maybe hoist this closer to the top of the shader module?"
                )
                .into()],
            },
            Error::EnableExtensionNotYetImplemented { kind, span } => ParseError {
                message: format!(
                    "the `{}` enable-extension is not yet supported",
                    EnableExtension::Unimplemented(kind).to_ident()
                ),
                labels: vec![(
                    span,
                    concat!(
                        "this enable-extension specifies standard functionality ",
                        "which is not yet implemented in Naga"
                    )
                    .into(),
                )],
                notes: vec![format!(
                    concat!(
                        "Let Naga maintainers know that you ran into this at ",
                        "<https://github.com/gfx-rs/wgpu/issues/{}>, ",
                        "so they can prioritize it!"
                    ),
                    kind.tracking_issue_num()
                )],
            },
            Error::EnableExtensionNotEnabled { kind, span } => ParseError {
                message: format!("the `{}` enable extension is not enabled", kind.to_ident()),
                labels: vec![(
                    span,
                    format!(
                        concat!(
                            "the `{}` \"Enable Extension\" is needed for this functionality, ",
                            "but it is not currently enabled."
                        ),
                        kind.to_ident()
                    )
                    .into(),
                )],
                notes: if let EnableExtension::Unimplemented(kind) = kind {
                    vec![format!(
                        concat!(
                            "This \"Enable Extension\" is not yet implemented. ",
                            "Let Naga maintainers know that you ran into this at ",
                            "<https://github.com/gfx-rs/wgpu/issues/{}>, ",
                            "so they can prioritize it!"
                        ),
                        kind.tracking_issue_num()
                    )]
                } else {
                    vec![
                        format!(
                            "You can enable this extension by adding `enable {};` at the top of the shader, before any other items.",
                            kind.to_ident()
                        ),
                    ]
                },
            },
            Error::EnableExtensionNotSupported { kind, span } => ParseError {
                message: format!(
                    "the `{}` extension is not supported in the current environment",
                    kind.to_ident()
                ),
                labels: vec![(
                    span,
                    "unsupported enable-extension".into(),
                )],
                notes: vec![],
            },
            Error::LanguageExtensionNotYetImplemented { kind, span } => ParseError {
                message: format!(
                    "the `{}` language extension is not yet supported",
                    LanguageExtension::Unimplemented(kind).to_ident()
                ),
                labels: vec![(span, "".into())],
                notes: vec![format!(
                    concat!(
                        "Let Naga maintainers know that you ran into this at ",
                        "<https://github.com/gfx-rs/wgpu/issues/{}>, ",
                        "so they can prioritize it!"
                    ),
                    kind.tracking_issue_num()
                )],
            },
            Error::DiagnosticInvalidSeverity {
                severity_control_name_span,
            } => ParseError {
                message: "invalid `diagnostic(…)` severity".into(),
                labels: vec![(
                    severity_control_name_span,
                    "not a valid severity level".into(),
                )],
                notes: vec![concat!(
                    "See available severities at ",
                    "<https://www.w3.org/TR/WGSL/#diagnostic-severity>."
                )
                .into()],
            },
            Error::DiagnosticDuplicateTriggeringRule(ConflictingDiagnosticRuleError {
                triggering_rule_spans,
            }) => {
                let [first_span, second_span] = triggering_rule_spans;
                ParseError {
                    message: "found conflicting `diagnostic(…)` rule(s)".into(),
                    labels: vec![
                        (first_span, "first rule".into()),
                        (second_span, "second rule".into()),
                    ],
                    notes: vec![
                        concat!(
                            "Multiple `diagnostic(…)` rules with the same rule name ",
                            "conflict unless they are directives and the severity is the same.",
                        )
                        .into(),
                        "You should delete the rule you don't want.".into(),
                    ],
                }
            }
            Error::DiagnosticAttributeNotYetImplementedAtParseSite {
                site_name_plural,
                ref spans,
            } => ParseError {
                message: "`@diagnostic(…)` attribute(s) not yet implemented".into(),
                labels: {
                    let mut spans = spans.iter().cloned();
                    let first = spans
                        .next()
                        .map(|span| {
                            (
                                span,
                                format!("can't use this on {site_name_plural} (yet)").into(),
                            )
                        })
                        .expect("internal error: diag. attr. rejection on empty map");
                    core::iter::once(first)
                        .chain(spans.map(|span| (span, "".into())))
                        .collect()
                },
                notes: vec![format!(concat!(
                    "Let Naga maintainers know that you ran into this at ",
                    "<https://github.com/gfx-rs/wgpu/issues/5320>, ",
                    "so they can prioritize it!"
                ))],
            },
            Error::DiagnosticAttributeNotSupported { on_what, ref spans } => {
                // In this case the user may have intended to create a global diagnostic filter directive,
                // so display a note to them suggesting the correct syntax.
                let intended_diagnostic_directive = match on_what {
                    DiagnosticAttributeNotSupportedPosition::SemicolonInModulePosition => true,
                    DiagnosticAttributeNotSupportedPosition::Other { .. } => false,
                };
                let on_what_plural = match on_what {
                    DiagnosticAttributeNotSupportedPosition::SemicolonInModulePosition => {
                        "semicolons"
                    }
                    DiagnosticAttributeNotSupportedPosition::Other { display_plural } => {
                        display_plural
                    }
                };
                ParseError {
                    message: format!(
                        "`@diagnostic(…)` attribute(s) on {on_what_plural} are not supported",
                    ),
                    labels: spans
                        .iter()
                        .cloned()
                        .map(|span| (span, "".into()))
                        .collect(),
                    notes: vec![
                        concat!(
                            "`@diagnostic(…)` attributes are only permitted on `fn`s, ",
                            "some statements, and `switch`/`loop` bodies."
                        )
                        .into(),
                        {
                            if intended_diagnostic_directive {
                                concat!(
                                    "If you meant to declare a diagnostic filter that ",
                                    "applies to the entire module, move this line to ",
                                    "the top of the file and remove the `@` symbol."
                                )
                                .into()
                            } else {
                                concat!(
                                    "These attributes are well-formed, ",
                                    "you likely just need to move them."
                                )
                                .into()
                            }
                        },
                    ],
                }
            }
            Error::SelectUnexpectedArgumentType { arg_span, ref arg_type } => ParseError {
                message: "unexpected argument type for `select` call".into(),
                labels: vec![(arg_span, format!("this value of type {arg_type}").into())],
                notes: vec!["expected a scalar or a `vecN` of scalars".into()],
            },
            Error::SelectRejectAndAcceptHaveNoCommonType {
                reject_span,
                ref reject_type,
                accept_span,
                ref accept_type,
            } => ParseError {
                message: "type mismatch for reject and accept values in `select` call".into(),
                labels: vec![
                    (reject_span, format!("reject value of type {reject_type}").into()),
                    (accept_span, format!("accept value of type {accept_type}").into()),
                ],
                notes: vec![],
            },
            Error::ExpectedGlobalVariable { name_span } => ParseError {
                message: "expected global variable".to_string(),
                labels: vec![(name_span, "variable used here".into())],
                notes: vec![],
            },
            Error::StructMemberTooLarge { member_name_span } => ParseError {
                message: "struct member is too large".into(),
                labels: vec![(member_name_span, "this member exceeds the maximum size".into())],
                notes: vec![format!(
                    "the maximum size is {} bytes",
                    crate::valid::MAX_TYPE_SIZE
                )],
            },
            Error::TypeTooLarge { span } => ParseError {
                message: "type is too large".into(),
                labels: vec![(span, "this type exceeds the maximum size".into())],
                notes: vec![format!(
                    "the maximum size is {} bytes",
                    crate::valid::MAX_TYPE_SIZE
                )],
            },
            Error::UnderspecifiedCooperativeMatrix => ParseError {
                message: "cooperative matrix constructor is underspecified".into(),
                labels: vec![],
                notes: vec![format!("must be F32")],
            },
            Error::InvalidCooperativeLoadType(span) => ParseError {
                message: "cooperative load should have a generic type for coop_mat".into(),
                labels: vec![(span, "type needs the coop_mat<...>".into())],
                notes: vec![format!("must be a valid cooperative type")],
            },
            Error::UnsupportedCooperativeScalar(span) => ParseError {
                message: "cooperative scalar type is not supported".into(),
                labels: vec![(span, "type needs the scalar type specified".into())],
                notes: vec![format!("must be F32")],
            },
            Error::UnexpectedIdentForEnumerant(ident_span) => ParseError {
                message: format!(
                    "identifier `{}` resolves to a declaration",
                    &source[ident_span]
                ),
                labels: vec![(ident_span, "needs to resolve to a predeclared enumerant".into())],
                notes: vec![],
            },
            Error::UnexpectedExprForEnumerant(expr_span) => ParseError {
                message: "unexpected expression".to_string(),
                labels: vec![(expr_span, "needs to be an identifier resolving to a predeclared enumerant".into())],
                notes: vec![],
            },
            Error::UnusedArgsForTemplate(ref expr_spans) => ParseError {
                message: "unused expressions for template".to_string(),
                labels: expr_spans.iter().cloned().map(|span| -> (_, _){ (span, "unused".into()) }).collect(),
                notes: vec![],
            },
            Error::UnexpectedTemplate(span) => ParseError {
                message: "unexpected template".to_string(),
                labels: vec![(span, "expected identifier".into())],
                notes: vec![],
            },
            Error::MissingTemplateArg {
                span,
                description: arg,
            } => ParseError {
                message: format!(
                    "`{}` needs a template argument specified: {arg}",
                    &source[span]
                ),
                labels: vec![(span, "is missing a template argument".into())],
                notes: vec![],
            },
            Error::UnexpectedExprForTypeExpression(expr_span) => ParseError {
                message: "unexpected expression".to_string(),
                labels: vec![(expr_span, "needs to be an identifier resolving to a type declaration (alias or struct) or predeclared type(-generator)".into())],
                notes: vec![],
            },
            Error::MissingIncomingPayload(span) => ParseError {
                message: "incoming payload is missing on a `closest_hit`, `any_hit` or `miss` shader entry point".to_string(),
                labels: vec![(
                    span,
                    "must be paired with a `@incoming_payload` attribute".into(),
                )],
                notes: vec![],
            },
        }
    }
}
