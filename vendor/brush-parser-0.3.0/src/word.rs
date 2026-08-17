//! Parser for shell words, used in expansion and other contexts.
//!
//! Implements support for:
//!
//! - Text quoting (single, double, ANSI C).
//! - Escape sequences.
//! - Tilde prefixes.
//! - Parameter expansion expressions.
//! - Command substitution expressions.
//! - Arithmetic expansion expressions.

use crate::ParserOptions;
use crate::ast;
use crate::error;

/// Encapsulates a `WordPiece` together with its position in the string it came from.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub struct WordPieceWithSource {
    /// The word piece.
    pub piece: WordPiece,
    /// The start index of the piece in the source string.
    pub start_index: usize,
    /// The end index of the piece in the source string.
    pub end_index: usize,
}

/// Represents a piece of a word.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum WordPiece {
    /// A simple unquoted, unescaped string.
    Text(String),
    /// A string that is single-quoted.
    SingleQuotedText(String),
    /// A string that is ANSI-C quoted.
    AnsiCQuotedText(String),
    /// A sequence of pieces that are embedded in double quotes.
    DoubleQuotedSequence(Vec<WordPieceWithSource>),
    /// Gettext enabled variant of [`WordPiece::DoubleQuotedSequence`].
    GettextDoubleQuotedSequence(Vec<WordPieceWithSource>),
    /// A tilde prefix.
    TildePrefix(String),
    /// A parameter expansion.
    ParameterExpansion(ParameterExpr),
    /// A command substitution.
    CommandSubstitution(String),
    /// A backquoted command substitution.
    BackquotedCommandSubstitution(String),
    /// An escape sequence.
    EscapeSequence(String),
    /// An arithmetic expression.
    ArithmeticExpression(ast::UnexpandedArithmeticExpr),
}

/// Type of a parameter test.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum ParameterTestType {
    /// Check for unset or null.
    UnsetOrNull,
    /// Check for unset.
    Unset,
}

/// A parameter, used in a parameter expansion.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum Parameter {
    /// A 0-indexed positional parameter.
    Positional(u32),
    /// A special parameter.
    Special(SpecialParameter),
    /// A named variable.
    Named(String),
    /// An index into a named variable.
    NamedWithIndex {
        /// Variable name.
        name: String,
        /// Index.
        index: String,
    },
    /// A named array variable with all indices.
    NamedWithAllIndices {
        /// Variable name.
        name: String,
        /// Whether to concatenate the values.
        concatenate: bool,
    },
}

/// A special parameter, used in a parameter expansion.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum SpecialParameter {
    /// All positional parameters.
    AllPositionalParameters {
        /// Whether to concatenate the values.
        concatenate: bool,
    },
    /// The count of positional parameters.
    PositionalParameterCount,
    /// The last exit status in the shell.
    LastExitStatus,
    /// The current shell option flags.
    CurrentOptionFlags,
    /// The current shell process ID.
    ProcessId,
    /// The last background process ID managed by the shell.
    LastBackgroundProcessId,
    /// The name of the shell.
    ShellName,
}

/// A parameter expression, used in a parameter expansion.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum ParameterExpr {
    /// A parameter, with optional indirection.
    Parameter {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
    },
    /// Conditionally use default values.
    UseDefaultValues {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// The type of test to perform.
        test_type: ParameterTestType,
        /// Default value to conditionally use.
        default_value: Option<String>,
    },
    /// Conditionally assign default values.
    AssignDefaultValues {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// The type of test to perform.
        test_type: ParameterTestType,
        /// Default value to conditionally assign.
        default_value: Option<String>,
    },
    /// Indicate error if null or unset.
    IndicateErrorIfNullOrUnset {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// The type of test to perform.
        test_type: ParameterTestType,
        /// Error message to conditionally yield.
        error_message: Option<String>,
    },
    /// Conditionally use an alternative value.
    UseAlternativeValue {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// The type of test to perform.
        test_type: ParameterTestType,
        /// Alternative value to conditionally use.
        alternative_value: Option<String>,
    },
    /// Compute the length of the given parameter.
    ParameterLength {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
    },
    /// Remove the smallest suffix from the given string matching the given pattern.
    RemoveSmallestSuffixPattern {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Optionally provides a pattern to match.
        pattern: Option<String>,
    },
    /// Remove the largest suffix from the given string matching the given pattern.
    RemoveLargestSuffixPattern {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Optionally provides a pattern to match.
        pattern: Option<String>,
    },
    /// Remove the smallest prefix from the given string matching the given pattern.
    RemoveSmallestPrefixPattern {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Optionally provides a pattern to match.
        pattern: Option<String>,
    },
    /// Remove the largest prefix from the given string matching the given pattern.
    RemoveLargestPrefixPattern {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Optionally provides a pattern to match.
        pattern: Option<String>,
    },
    /// Extract a substring from the given parameter.
    Substring {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Arithmetic expression that will be expanded to compute the offset
        /// at which the substring should be extracted.
        offset: ast::UnexpandedArithmeticExpr,
        /// Optionally provides an arithmetic expression that will be expanded
        /// to compute the length of substring to be extracted; if left
        /// unspecified, the remainder of the string will be extracted.
        length: Option<ast::UnexpandedArithmeticExpr>,
    },
    /// Transform the given parameter.
    Transform {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Type of transformation to apply.
        op: ParameterTransformOp,
    },
    /// Uppercase the first character of the given parameter.
    UppercaseFirstChar {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Optionally provides a pattern to match.
        pattern: Option<String>,
    },
    /// Uppercase the portion of the given parameter matching the given pattern.
    UppercasePattern {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Optionally provides a pattern to match.
        pattern: Option<String>,
    },
    /// Lowercase the first character of the given parameter.
    LowercaseFirstChar {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Optionally provides a pattern to match.
        pattern: Option<String>,
    },
    /// Lowercase the portion of the given parameter matching the given pattern.
    LowercasePattern {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Optionally provides a pattern to match.
        pattern: Option<String>,
    },
    /// Replace occurrences of the given pattern in the given parameter.
    ReplaceSubstring {
        /// The parameter.
        parameter: Parameter,
        /// Whether to treat the expanded parameter as an indirect
        /// reference, which should be subsequently dereferenced
        /// for the expansion.
        indirect: bool,
        /// Pattern to match.
        pattern: String,
        /// Replacement string.
        replacement: Option<String>,
        /// Kind of match to perform.
        match_kind: SubstringMatchKind,
    },
    /// Select variable names from the environment with a given prefix.
    VariableNames {
        /// The prefix to match.
        prefix: String,
        /// Whether to concatenate the results.
        concatenate: bool,
    },
    /// Select member keys from the named array.
    MemberKeys {
        /// Name of the array variable.
        variable_name: String,
        /// Whether to concatenate the results.
        concatenate: bool,
    },
}

/// Kind of substring match.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum SubstringMatchKind {
    /// Match the prefix of the string.
    Prefix,
    /// Match the suffix of the string.
    Suffix,
    /// Match the first occurrence in the string.
    FirstOccurrence,
    /// Match all instances in the string.
    Anywhere,
}

/// Kind of operation to apply to a parameter.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum ParameterTransformOp {
    /// Capitalizate initials.
    CapitalizeInitial,
    /// Expand escape sequences.
    ExpandEscapeSequences,
    /// Possibly quote with arrays expanded.
    PossiblyQuoteWithArraysExpanded {
        /// Whether or not to yield separate words.
        separate_words: bool,
    },
    /// Apply prompt expansion.
    PromptExpand,
    /// Quote the parameter.
    Quoted,
    /// Translate to a format usable in an assignment/declaration.
    ToAssignmentLogic,
    /// Translate to the parameter's attribute flags.
    ToAttributeFlags,
    /// Translate to lowercase.
    ToLowerCase,
    /// Translate to uppercase.
    ToUpperCase,
}

/// Represents a sub-word that is either a brace expression or some other word text.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum BraceExpressionOrText {
    /// A brace expression.
    Expr(BraceExpression),
    /// Other word text.
    Text(String),
}

/// Represents a brace expression to be expanded.
pub type BraceExpression = Vec<BraceExpressionMember>;

/// Member of a brace expression.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum BraceExpressionMember {
    /// An inclusive numerical sequence.
    NumberSequence {
        /// Start of the sequence.
        start: i64,
        /// Inclusive end of the sequence.
        end: i64,
        /// Increment value.
        increment: i64,
    },
    /// An inclusive character sequence.
    CharSequence {
        /// Start of the sequence.
        start: char,
        /// Inclusive end of the sequence.
        end: char,
        /// Increment value.
        increment: i64,
    },
    /// Child text or expressions.
    Child(Vec<BraceExpressionOrText>),
}

/// Parse a word into its constituent pieces.
///
/// # Arguments
///
/// * `word` - The word to parse.
/// * `options` - The parser options to use.
pub fn parse(
    word: &str,
    options: &ParserOptions,
) -> Result<Vec<WordPieceWithSource>, error::WordParseError> {
    cacheable_parse(word.to_owned(), options.to_owned())
}

#[cached::proc_macro::cached(size = 64, result = true)]
fn cacheable_parse(
    word: String,
    options: ParserOptions,
) -> Result<Vec<WordPieceWithSource>, error::WordParseError> {
    tracing::debug!(target: "expansion", "Parsing word '{}'", word);

    let pieces = expansion_parser::unexpanded_word(word.as_str(), &options)
        .map_err(|err| error::WordParseError::Word(word.clone(), err.into()))?;

    tracing::debug!(target: "expansion", "Parsed word '{}' => {{{:?}}}", word, pieces);

    Ok(pieces)
}

/// Parse the given word into a parameter expression.
///
/// # Arguments
///
/// * `word` - The word to parse.
/// * `options` - The parser options to use.
pub fn parse_parameter(
    word: &str,
    options: &ParserOptions,
) -> Result<Parameter, error::WordParseError> {
    expansion_parser::parameter(word, options)
        .map_err(|err| error::WordParseError::Parameter(word.to_owned(), err.into()))
}

/// Parse brace expansion from a given word .
///
/// # Arguments
///
/// * `word` - The word to parse.
/// * `options` - The parser options to use.
pub fn parse_brace_expansions(
    word: &str,
    options: &ParserOptions,
) -> Result<Option<Vec<BraceExpressionOrText>>, error::WordParseError> {
    expansion_parser::brace_expansions(word, options)
        .map_err(|err| error::WordParseError::BraceExpansion(word.to_owned(), err.into()))
}

peg::parser! {
    grammar expansion_parser(parser_options: &ParserOptions) for str {
        // Helper rule that enables pegviz to be used to visualize debug peg traces.
        rule traced<T>(e: rule<T>) -> T =
            &(input:$([_]*) {
                #[cfg(feature = "debug-tracing")]
                println!("[PEG_INPUT_START]\n{input}\n[PEG_TRACE_START]");
            })
            e:e()? {?
                #[cfg(feature = "debug-tracing")]
                println!("[PEG_TRACE_STOP]");
                e.ok_or("")
            }

        pub(crate) rule unexpanded_word() -> Vec<WordPieceWithSource> = traced(<word(<![_]>)>)

        rule word<T>(stop_condition: rule<T>) -> Vec<WordPieceWithSource> =
            tilde:tilde_prefix_with_source()? pieces:word_piece_with_source(<stop_condition()>, false /*in_command*/)* {
                let mut all_pieces = Vec::new();
                if let Some(tilde) = tilde {
                    all_pieces.push(tilde);
                }
                all_pieces.extend(pieces);
                all_pieces
            }

        // Takes a word as input.
        pub(crate) rule brace_expansions() -> Option<Vec<BraceExpressionOrText>> =
            pieces:(brace_expansion_piece(<![_]>)+) { Some(pieces) } /
            [_]* { None }

        // Returns either a complete brace expression (without any prefix or suffix), or a
        // non-brace-expression string.
        rule brace_expansion_piece<T>(stop_condition: rule<T>) -> BraceExpressionOrText =
            expr:brace_expr() {
                BraceExpressionOrText::Expr(expr)
            } /
            text:$(non_brace_expr_text(<stop_condition()>)+) { BraceExpressionOrText::Text(text.to_owned()) }

        // Parses text that is not considered to contain a brace expression.
        rule non_brace_expr_text<T>(stop_condition: rule<T>) -> () =
            !"{" word_piece(<['{'] {} / stop_condition() {}>, false) {} /
            !brace_expr() !stop_condition() "{" {}

        // Parses a complete brace expression, with no prefix or suffix.
        pub(crate) rule brace_expr() -> BraceExpression =
            "{" inner:brace_expr_inner() "}" { inner }

        // Parses the text inside a complete brace expression; basically the complete brace
        // expression without the opening and closing brace characters.
        pub(crate) rule brace_expr_inner() -> BraceExpression =
            brace_text_list_expr() /
            seq:brace_sequence_expr() { vec![seq] }

        // Parses a list of brace expression members, including the separating commas; does
        // not include the opening and closing braces.
        pub(crate) rule brace_text_list_expr() -> BraceExpression =
            brace_text_list_member() **<2,> ","

        // Parses an element that can occur in a brace expression member list, not including the
        // terminating comma or closing brace.
        pub(crate) rule brace_text_list_member() -> BraceExpressionMember =
            // Matches an empty-string member, without consuming the comma or closing brace that terminates it.
            &[',' | '}'] { BraceExpressionMember::Child(vec![BraceExpressionOrText::Text(String::new())]) } /
            // Matches a nested string that may include some combination of concatenated textual strings
            // and brace expressions.
            child_pieces:(brace_expansion_piece(<[',' | '}']>)+) {
                BraceExpressionMember::Child(child_pieces)
            }

        pub(crate) rule brace_sequence_expr() -> BraceExpressionMember =
            start:number() ".." end:number() increment:(".." n:number() { n })? {
                BraceExpressionMember::NumberSequence { start, end, increment: increment.unwrap_or(1) }
            } /
            start:character() ".." end:character() increment:(".." n:number() { n })? {
                BraceExpressionMember::CharSequence { start, end, increment: increment.unwrap_or(1) }
            }

        rule number() -> i64 = sign:number_sign()? n:$(['0'..='9']+) {
            let sign = sign.unwrap_or(1);
            let num: i64 = n.parse().unwrap();
            num * sign
        }

        rule number_sign() -> i64 =
            ['-'] { -1 } /
            ['+'] { 1 }

        rule character() -> char = ['a'..='z' | 'A'..='Z']

        pub(crate) rule is_arithmetic_word() =
            arithmetic_word(<![_]>)

            // N.B. We don't bother returning the word pieces, as all users of this rule
        // only try to extract the consumed input string and not the parse result.
        rule arithmetic_word<T>(stop_condition: rule<T>) =
            arithmetic_word_piece(<stop_condition()>)* {}

        pub(crate) rule is_arithmetic_word_piece() =
            arithmetic_word_piece(<![_]>)

        // This rule matches an individual "piece" of an arithmetic expression. It needs to handle
        // matching nested parenthesized expressions as well. We stop consuming the input when
        // we reach the provided stop condition, which typically denotes the end of the containing
        // arithmetic expression.
        rule arithmetic_word_piece<T>(stop_condition: rule<T>) =
            // This branch matches a parenthesized piece; we consume the opening parenthesis and
            // delegate the rest to a helper rule. We don't worry about the stop condition passed
            // into us, because if we see an opening parenthesis then we *must* find its closing
            // partner.
            "(" arithmetic_word_plus_right_paren() {} /
            // This branch matches any standard piece of a word, stopping as soon as we reach
            // either the overall stop condition *OR* an opening parenthesis. We add this latter
            // condition to ensure that *we* handle matching parentheses.
            !"(" word_piece(<param_rule_or_open_paren(<stop_condition()>)>, false /*in_command*/) {}

        // This is a helper rule that matches either the provided stop condition or an opening parenthesis.
        rule param_rule_or_open_paren<T>(stop_condition: rule<T>) -> () =
            stop_condition() {} /
            "(" {}

        // This rule matches an arithmetic word followed by a right parenthesis. It must consume the right parenthesis.
        rule arithmetic_word_plus_right_paren() =
            arithmetic_word(<[')']>) ")" /

        rule word_piece_with_source<T>(stop_condition: rule<T>, in_command: bool) -> WordPieceWithSource =
            start_index:position!() piece:word_piece(<stop_condition()>, in_command) end_index:position!() {
                WordPieceWithSource { piece, start_index, end_index }
            }

        rule word_piece<T>(stop_condition: rule<T>, in_command: bool) -> WordPiece =
            // Rules that match quoted text.
            s:double_quoted_sequence() { WordPiece::DoubleQuotedSequence(s) } /
            s:single_quoted_literal_text() { WordPiece::SingleQuotedText(s.to_owned()) } /
            s:ansi_c_quoted_text() { WordPiece::AnsiCQuotedText(s.to_owned()) } /
            s:gettext_double_quoted_sequence() { WordPiece::GettextDoubleQuotedSequence(s) } /
            // Rules that match pieces starting with a dollar sign ('$').
            dollar_sign_word_piece() /
            // Rules that match unquoted text that doesn't start with an unescaped dollar sign.
            normal_escape_sequence() /
            unquoted_literal_text(<stop_condition()>, in_command)

        rule dollar_sign_word_piece() -> WordPiece =
            arithmetic_expansion() /
            command_substitution() /
            parameter_expansion()

        rule double_quoted_word_piece() -> WordPiece =
            arithmetic_expansion() /
            command_substitution() /
            parameter_expansion() /
            double_quoted_escape_sequence() /
            double_quoted_text()

        rule double_quoted_sequence() -> Vec<WordPieceWithSource> =
            "\"" i:double_quoted_sequence_inner()* "\"" { i }

        rule gettext_double_quoted_sequence() -> Vec<WordPieceWithSource> =
            "$\"" i:double_quoted_sequence_inner()* "\"" { i }

        rule double_quoted_sequence_inner() -> WordPieceWithSource =
            start_index:position!() piece:double_quoted_word_piece() end_index:position!() {
                WordPieceWithSource {
                    piece,
                    start_index,
                    end_index
                }
            }

        rule single_quoted_literal_text() -> &'input str =
            "\'" inner:$([^'\'']*) "\'" { inner }

        rule ansi_c_quoted_text() -> &'input str =
            "$\'" inner:$(("\\'" / [^'\''])*) "\'" { inner }

        rule unquoted_literal_text<T>(stop_condition: rule<T>, in_command: bool) -> WordPiece =
            s:$(unquoted_literal_text_piece(<stop_condition()>, in_command)+) { WordPiece::Text(s.to_owned()) }

        // TODO: Find a way to remove the special-case logic for extglob + subshell commands
        rule unquoted_literal_text_piece<T>(stop_condition: rule<T>, in_command: bool) =
            is_true(in_command) extglob_pattern() /
            is_true(in_command) subshell_command() /
            !stop_condition() !normal_escape_sequence() [^'\'' | '\"' | '$' | '`'] {}

        rule is_true(value: bool) = &[_] {? if value { Ok(()) } else { Err("not true") } }

        rule extglob_pattern() =
            ("@" / "!" / "?" / "+" / "*") "(" extglob_body_piece()* ")" {}

        rule extglob_body_piece() =
            word_piece(<[')']>, true /*in_command*/) {}

        rule subshell_command() =
            "(" command() ")" {}

        rule double_quoted_text() -> WordPiece =
            s:double_quote_body_text() { WordPiece::Text(s.to_owned()) }

        rule double_quote_body_text() -> &'input str =
            $((!double_quoted_escape_sequence() !dollar_sign_word_piece() [^'\"'])+)

        rule normal_escape_sequence() -> WordPiece =
            s:$("\\" [c]) { WordPiece::EscapeSequence(s.to_owned()) }

        rule double_quoted_escape_sequence() -> WordPiece =
            s:$("\\" ['$' | '`' | '\"' | '\\']) { WordPiece::EscapeSequence(s.to_owned()) }

        rule tilde_prefix_with_source() -> WordPieceWithSource =
            start_index:position!() piece:tilde_prefix() end_index:position!() {
                WordPieceWithSource {
                    piece,
                    start_index,
                    end_index
                }
            }

        // TODO: Handle colon syntax
        rule tilde_prefix() -> WordPiece =
            tilde_parsing_enabled() "~" cs:$((!['/' | ':' | ';'] [c])*) { WordPiece::TildePrefix(cs.to_owned()) }

        // TODO: Deal with fact that there may be a quoted word or escaped closing brace chars.
        // TODO: Improve on how we handle a '$' not followed by a valid variable name or parameter.
        rule parameter_expansion() -> WordPiece =
            "${" e:parameter_expression() "}" {
                WordPiece::ParameterExpansion(e)
            } /
            "$" parameter:unbraced_parameter() {
                WordPiece::ParameterExpansion(ParameterExpr::Parameter { parameter, indirect: false })
            } /
            "$" !['\''] {
                WordPiece::Text("$".to_owned())
            }

        rule parameter_expression() -> ParameterExpr =
            indirect:parameter_indirection() parameter:parameter() test_type:parameter_test_type() "-" default_value:parameter_expression_word()? {
                ParameterExpr::UseDefaultValues { parameter, indirect, test_type, default_value }
            } /
            indirect:parameter_indirection() parameter:parameter() test_type:parameter_test_type() "=" default_value:parameter_expression_word()? {
                ParameterExpr::AssignDefaultValues { parameter, indirect, test_type, default_value }
            } /
            indirect:parameter_indirection() parameter:parameter() test_type:parameter_test_type() "?" error_message:parameter_expression_word()? {
                ParameterExpr::IndicateErrorIfNullOrUnset { parameter, indirect, test_type, error_message }
            } /
            indirect:parameter_indirection() parameter:parameter() test_type:parameter_test_type() "+" alternative_value:parameter_expression_word()? {
                ParameterExpr::UseAlternativeValue { parameter, indirect, test_type, alternative_value }
            } /
            "#" parameter:parameter() {
                ParameterExpr::ParameterLength { parameter, indirect: false }
            } /
            indirect:parameter_indirection() parameter:parameter() "%%" pattern:parameter_expression_word()? {
                ParameterExpr::RemoveLargestSuffixPattern { parameter, indirect, pattern }
            } /
            indirect:parameter_indirection() parameter:parameter() "%" pattern:parameter_expression_word()? {
                ParameterExpr::RemoveSmallestSuffixPattern { parameter, indirect, pattern }
            } /
            indirect:parameter_indirection() parameter:parameter() "##" pattern:parameter_expression_word()? {
                ParameterExpr::RemoveLargestPrefixPattern { parameter, indirect, pattern }
            } /
            indirect:parameter_indirection() parameter:parameter() "#" pattern:parameter_expression_word()? {
                ParameterExpr::RemoveSmallestPrefixPattern { parameter, indirect, pattern }
            } /
            // N.B. The following case is for non-sh extensions.
            non_posix_extensions_enabled() e:non_posix_parameter_expression() { e } /
            indirect:parameter_indirection() parameter:parameter() {
                ParameterExpr::Parameter { parameter, indirect }
            }

        rule parameter_test_type() -> ParameterTestType =
            colon:":"? {
                if colon.is_some() {
                    ParameterTestType::UnsetOrNull
                } else {
                    ParameterTestType::Unset
                }
            }

        rule non_posix_parameter_expression() -> ParameterExpr =
            "!" variable_name:variable_name() "[*]" {
                ParameterExpr::MemberKeys { variable_name: variable_name.to_owned(), concatenate: true }
            } /
            "!" variable_name:variable_name() "[@]" {
                ParameterExpr::MemberKeys { variable_name: variable_name.to_owned(), concatenate: false }
            } /
            indirect:parameter_indirection() parameter:parameter() ":" offset:substring_offset() length:(":" l:substring_length() { l })? {
                ParameterExpr::Substring { parameter, indirect, offset, length }
            } /
            indirect:parameter_indirection() parameter:parameter() "@" op:non_posix_parameter_transformation_op() {
                ParameterExpr::Transform { parameter, indirect, op }
            } /
            "!" prefix:variable_name() "*" {
                ParameterExpr::VariableNames { prefix: prefix.to_owned(), concatenate: true }
            } /
            "!" prefix:variable_name() "@" {
                ParameterExpr::VariableNames { prefix: prefix.to_owned(), concatenate: false }
            } /
            indirect:parameter_indirection() parameter:parameter() "/#" pattern:parameter_search_pattern() replacement:parameter_replacement_str()? {
                ParameterExpr::ReplaceSubstring { parameter, indirect, pattern, replacement, match_kind: SubstringMatchKind::Prefix }
            } /
            indirect:parameter_indirection() parameter:parameter() "/%" pattern:parameter_search_pattern() replacement:parameter_replacement_str()? {
                ParameterExpr::ReplaceSubstring { parameter, indirect, pattern, replacement, match_kind: SubstringMatchKind::Suffix }
            } /
            indirect:parameter_indirection() parameter:parameter() "//" pattern:parameter_search_pattern() replacement:parameter_replacement_str()? {
                ParameterExpr::ReplaceSubstring { parameter, indirect, pattern, replacement, match_kind: SubstringMatchKind::Anywhere }
            } /
            indirect:parameter_indirection() parameter:parameter() "/" pattern:parameter_search_pattern() replacement:parameter_replacement_str()? {
                ParameterExpr::ReplaceSubstring { parameter, indirect, pattern, replacement, match_kind: SubstringMatchKind::FirstOccurrence }
            } /
            indirect:parameter_indirection() parameter:parameter() "^^" pattern:parameter_expression_word()? {
                ParameterExpr::UppercasePattern { parameter, indirect, pattern }
            } /
            indirect:parameter_indirection() parameter:parameter() "^" pattern:parameter_expression_word()? {
                ParameterExpr::UppercaseFirstChar { parameter, indirect, pattern }
            } /
            indirect:parameter_indirection() parameter:parameter() ",," pattern:parameter_expression_word()? {
                ParameterExpr::LowercasePattern { parameter, indirect, pattern }
            } /
            indirect:parameter_indirection() parameter:parameter() "," pattern:parameter_expression_word()? {
                ParameterExpr::LowercaseFirstChar { parameter, indirect, pattern }
            }

        rule parameter_indirection() -> bool =
            non_posix_extensions_enabled() "!" { true } /
            { false }

        rule non_posix_parameter_transformation_op() -> ParameterTransformOp =
            "U" { ParameterTransformOp::ToUpperCase } /
            "u" { ParameterTransformOp::CapitalizeInitial } /
            "L" { ParameterTransformOp::ToLowerCase } /
            "Q" { ParameterTransformOp::Quoted } /
            "E" { ParameterTransformOp::ExpandEscapeSequences } /
            "P" { ParameterTransformOp::PromptExpand } /
            "A" { ParameterTransformOp::ToAssignmentLogic } /
            "K" { ParameterTransformOp::PossiblyQuoteWithArraysExpanded { separate_words: false } } /
            "a" { ParameterTransformOp::ToAttributeFlags } /
            "k" { ParameterTransformOp::PossiblyQuoteWithArraysExpanded { separate_words: true } }


        rule unbraced_parameter() -> Parameter =
            p:unbraced_positional_parameter() { Parameter::Positional(p) } /
            p:special_parameter() { Parameter::Special(p) } /
            p:variable_name() { Parameter::Named(p.to_owned()) }

        // N.B. The indexing syntax is not a standard sh-ism.
        pub(crate) rule parameter() -> Parameter =
            p:positional_parameter() { Parameter::Positional(p) } /
            p:special_parameter() { Parameter::Special(p) } /
            non_posix_extensions_enabled() p:variable_name() "[@]" { Parameter::NamedWithAllIndices { name: p.to_owned(), concatenate: false } } /
            non_posix_extensions_enabled() p:variable_name() "[*]" { Parameter::NamedWithAllIndices { name: p.to_owned(), concatenate: true } } /
            non_posix_extensions_enabled() p:variable_name() "[" index:$(arithmetic_word(<"]">)) "]" {?
                Ok(Parameter::NamedWithIndex { name: p.to_owned(), index: index.to_owned() })
            } /
            p:variable_name() { Parameter::Named(p.to_owned()) }

        rule positional_parameter() -> u32 =
            n:$(['1'..='9'](['0'..='9']*)) {? n.parse().or(Err("u32")) }
        rule unbraced_positional_parameter() -> u32 =
            n:$(['1'..='9']) {? n.parse().or(Err("u32")) }

        rule special_parameter() -> SpecialParameter =
            "@" { SpecialParameter::AllPositionalParameters { concatenate: false } } /
            "*" { SpecialParameter::AllPositionalParameters { concatenate: true } } /
            "#" { SpecialParameter::PositionalParameterCount } /
            "?" { SpecialParameter::LastExitStatus } /
            "-" { SpecialParameter::CurrentOptionFlags } /
            "$" { SpecialParameter::ProcessId } /
            "!" { SpecialParameter::LastBackgroundProcessId } /
            "0" { SpecialParameter::ShellName }

        rule variable_name() -> &'input str =
            $(!['0'..='9'] ['_' | '0'..='9' | 'a'..='z' | 'A'..='Z']+)

        pub(crate) rule command_substitution() -> WordPiece =
            "$(" c:command() ")" { WordPiece::CommandSubstitution(c.to_owned()) } /
            "`" c:backquoted_command() "`" { WordPiece::BackquotedCommandSubstitution(c) }

        pub(crate) rule command() -> &'input str =
            $(command_piece()*)

        pub(crate) rule command_piece() -> () =
            word_piece(<[')']>, true /*in_command*/) {} /
            ([' ' | '\t'])+ {}

        rule backquoted_command() -> String =
            chars:(backquoted_char()*) { chars.into_iter().collect() }

        rule backquoted_char() -> &'input str =
            "\\`" { "`" } /
            "\\\\" { "\\\\" } /
            s:$([^'`']) { s }

        rule arithmetic_expansion() -> WordPiece =
            "$((" e:$(arithmetic_word(<"))">)) "))" { WordPiece::ArithmeticExpression(ast::UnexpandedArithmeticExpr { value: e.to_owned() } ) }

        rule substring_offset() -> ast::UnexpandedArithmeticExpr =
            s:$(arithmetic_word(<[':' | '}']>)) { ast::UnexpandedArithmeticExpr { value: s.to_owned() } }

        rule substring_length() -> ast::UnexpandedArithmeticExpr =
            s:$(arithmetic_word(<[':' | '}']>)) { ast::UnexpandedArithmeticExpr { value: s.to_owned() } }

        rule parameter_replacement_str() -> String =
            "/" s:$(word(<['}']>)) { s.to_owned() }

        rule parameter_search_pattern() -> String =
            s:$(word(<['}' | '/']>)) { s.to_owned() }

        rule parameter_expression_word() -> String =
            s:$(word(<['}']>)) { s.to_owned() }

        rule extglob_enabled() -> () =
            &[_] {? if parser_options.enable_extended_globbing { Ok(()) } else { Err("no extglob") } }

        rule non_posix_extensions_enabled() -> () =
            &[_] {? if !parser_options.sh_mode { Ok(()) } else { Err("posix") } }

        rule tilde_parsing_enabled() -> () =
            &[_] {? if parser_options.tilde_expansion { Ok(()) } else { Err("no tilde expansion") } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use insta::assert_ron_snapshot;

    #[derive(serde::Serialize)]
    struct ParseTestResults<'a> {
        input: &'a str,
        result: Vec<WordPieceWithSource>,
    }

    fn test_parse(word: &str) -> Result<ParseTestResults<'_>> {
        let parsed = super::parse(word, &ParserOptions::default())?;
        Ok(ParseTestResults {
            input: word,
            result: parsed,
        })
    }

    #[test]
    fn parse_ansi_c_quoted_text() -> Result<()> {
        assert_ron_snapshot!(test_parse(r"$'hi\nthere\t'")?);
        Ok(())
    }

    #[test]
    fn parse_double_quoted_text() -> Result<()> {
        assert_ron_snapshot!(test_parse(r#""a ${b} c""#)?);
        Ok(())
    }

    #[test]
    fn parse_gettext_double_quoted_text() -> Result<()> {
        assert_ron_snapshot!(test_parse(r#"$"a ${b} c""#)?);
        Ok(())
    }

    #[test]
    fn parse_command_substitution() -> Result<()> {
        super::expansion_parser::command_piece("echo", &ParserOptions::default())?;
        super::expansion_parser::command_piece("hi", &ParserOptions::default())?;
        super::expansion_parser::command("echo hi", &ParserOptions::default())?;
        super::expansion_parser::command_substitution("$(echo hi)", &ParserOptions::default())?;

        assert_ron_snapshot!(test_parse("$(echo hi)")?);

        Ok(())
    }

    #[test]
    fn parse_command_substitution_with_embedded_quotes() -> Result<()> {
        super::expansion_parser::command_piece("echo", &ParserOptions::default())?;
        super::expansion_parser::command_piece(r#""hi""#, &ParserOptions::default())?;
        super::expansion_parser::command(r#"echo "hi""#, &ParserOptions::default())?;
        super::expansion_parser::command_substitution(
            r#"$(echo "hi")"#,
            &ParserOptions::default(),
        )?;

        assert_ron_snapshot!(test_parse(r#"$(echo "hi")"#)?);
        Ok(())
    }

    #[test]
    fn parse_command_substitution_with_embedded_extglob() -> Result<()> {
        assert_ron_snapshot!(test_parse("$(echo !(x))")?);
        Ok(())
    }

    #[test]
    fn parse_backquoted_command() -> Result<()> {
        assert_ron_snapshot!(test_parse("`echo hi`")?);
        Ok(())
    }

    #[test]
    fn parse_backquoted_command_in_double_quotes() -> Result<()> {
        assert_ron_snapshot!(test_parse(r#""`echo hi`""#)?);
        Ok(())
    }

    #[test]
    fn parse_extglob_with_embedded_parameter() -> Result<()> {
        assert_ron_snapshot!(test_parse("+([$var])")?);
        Ok(())
    }

    #[test]
    fn parse_arithmetic_expansion() -> Result<()> {
        assert_ron_snapshot!(test_parse("$((0))")?);
        Ok(())
    }

    #[test]
    fn parse_arithmetic_expansion_with_parens() -> Result<()> {
        assert_ron_snapshot!(test_parse("$((((1+2)*3)))")?);
        Ok(())
    }

    #[test]
    fn test_arithmetic_word_parsing() {
        let options = ParserOptions::default();

        assert!(super::expansion_parser::is_arithmetic_word("a", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word("b", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word(" a + b ", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word("(a)", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word("((a))", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word("(((a)))", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word("(1+2)", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word("(1+2)*3", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word("((1+2)*3)", &options).is_ok());
    }

    #[test]
    fn test_arithmetic_word_piece_parsing() {
        let options = ParserOptions::default();

        assert!(super::expansion_parser::is_arithmetic_word_piece("a", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece("b", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece(" a + b ", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece("(a)", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece("((a))", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece("(((a)))", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece("(1+2)", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece("((1+2))", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece("((1+2)*3)", &options).is_ok());
        assert!(super::expansion_parser::is_arithmetic_word_piece("(a", &options).is_err());
        assert!(super::expansion_parser::is_arithmetic_word_piece("(a))", &options).is_err());
        assert!(super::expansion_parser::is_arithmetic_word_piece("((a)", &options).is_err());
    }

    #[test]
    fn test_brace_expansion_parsing() -> Result<()> {
        let options = ParserOptions::default();

        let inputs = ["x{a,b}y", "{a,b{1,2}}"];

        for input in inputs {
            assert_ron_snapshot!(super::parse_brace_expansions(input, &options)?.ok_or_else(
                || anyhow::anyhow!("Expected brace expansion to be parsed successfully")
            )?);
        }

        Ok(())
    }
}
