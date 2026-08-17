use crate::ast::{self, SeparatorOperator, SourceLocation, maybe_location};
use crate::tokenizer::{Token, TokenEndReason, Tokenizer, TokenizerOptions, Tokens};
use crate::{TokenLocation, error};

use bon::Builder;

/// Options used to control the behavior of the parser.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ParserOptions {
    /// Whether or not to enable extended globbing (a.k.a. `extglob`).
    pub enable_extended_globbing: bool,
    /// Whether or not to enable POSIX compliance mode.
    pub posix_mode: bool,
    /// Whether or not to enable maximal compatibility with the `sh` shell.
    pub sh_mode: bool,
    /// Whether or not to perform tilde expansion.
    pub tilde_expansion: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            enable_extended_globbing: true,
            posix_mode: false,
            sh_mode: false,
            tilde_expansion: true,
        }
    }
}

impl ParserOptions {
    /// Returns the tokenizer options implied by these parser options.
    pub const fn tokenizer_options(&self) -> TokenizerOptions {
        TokenizerOptions {
            enable_extended_globbing: self.enable_extended_globbing,
            posix_mode: self.posix_mode,
            sh_mode: self.sh_mode,
        }
    }
}

/// Implements parsing for shell programs.
#[derive(Builder)]
pub struct Parser<R: std::io::BufRead> {
    /// The reader to use for input
    reader: R,
    /// Parsing options
    #[builder(default)]
    options: ParserOptions,
    /// Information about the source of the tokens
    #[builder(default)]
    source_info: SourceInfo,
}

impl<R: std::io::BufRead> Parser<R> {
    /// Returns a new parser instance.
    ///
    /// # Arguments
    ///
    /// * `reader` - The reader to use for input.
    /// * `options` - The options to use when parsing.
    /// * `source_info` - Information about the source of the tokens.
    pub fn new(reader: R, options: &ParserOptions, source_info: &SourceInfo) -> Self {
        Self {
            reader,
            options: options.clone(),
            source_info: source_info.clone(),
        }
    }

    /// Parses the input into an abstract syntax tree (AST) of a shell program.
    pub fn parse_program(&mut self) -> Result<ast::Program, error::ParseError> {
        //
        // References:
        //   * https://www.gnu.org/software/bash/manual/bash.html#Shell-Syntax
        //   * https://mywiki.wooledge.org/BashParser
        //   * https://aosabook.org/en/v1/bash.html
        //   * https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html
        //

        let tokens = self.tokenize()?;
        parse_tokens(&tokens, &self.options, &self.source_info)
    }

    /// Parses a function definition body from the input. The body is expected to be
    /// preceded by "()", but no function name.
    pub fn parse_function_parens_and_body(
        &mut self,
    ) -> Result<ast::FunctionBody, error::ParseError> {
        let tokens = self.tokenize()?;
        let parse_result = token_parser::function_parens_and_body(
            &Tokens { tokens: &tokens },
            &self.options,
            &self.source_info,
        );
        parse_result_to_error(parse_result, &tokens)
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, error::ParseError> {
        // First we tokenize the input, according to the policy implied by provided options.
        let mut tokenizer = Tokenizer::new(&mut self.reader, &self.options.tokenizer_options());

        tracing::debug!(target: "tokenize", "Tokenizing...");

        let mut tokens = vec![];
        loop {
            let result = match tokenizer.next_token() {
                Ok(result) => result,
                Err(e) => {
                    return Err(error::ParseError::Tokenizing {
                        inner: e,
                        position: tokenizer.current_location(),
                    });
                }
            };

            let reason = result.reason;
            if let Some(token) = result.token {
                tracing::debug!(target: "tokenize", "TOKEN {}: {:?} {reason:?}", tokens.len(), token);
                tokens.push(token);
            }

            if matches!(reason, TokenEndReason::EndOfInput) {
                break;
            }
        }

        tracing::debug!(target: "tokenize", "  => {} token(s)", tokens.len());

        Ok(tokens)
    }
}

/// Parses a sequence of tokens into the abstract syntax tree (AST) of a shell program.
///
/// # Arguments
///
/// * `tokens` - The tokens to parse.
/// * `options` - The options to use when parsing.
/// * `source_info` - Information about the source of the tokens.
pub fn parse_tokens(
    tokens: &Vec<Token>,
    options: &ParserOptions,
    source_info: &SourceInfo,
) -> Result<ast::Program, error::ParseError> {
    let parse_result = token_parser::program(&Tokens { tokens }, options, source_info);
    parse_result_to_error(parse_result, tokens)
}

fn parse_result_to_error<R>(
    parse_result: Result<R, peg::error::ParseError<usize>>,
    tokens: &Vec<Token>,
) -> Result<R, error::ParseError>
where
    R: std::fmt::Debug,
{
    match parse_result {
        Ok(program) => {
            tracing::debug!(target: "parse", "PROG: {:?}", program);
            Ok(program)
        }
        Err(parse_error) => {
            tracing::debug!(target: "parse", "Parse error: {:?}", parse_error);
            Err(error::convert_peg_parse_error(
                &parse_error,
                tokens.as_slice(),
            ))
        }
    }
}

impl peg::Parse for Tokens<'_> {
    type PositionRepr = usize;

    #[inline]
    fn start(&self) -> usize {
        0
    }

    #[inline]
    fn is_eof(&self, p: usize) -> bool {
        p >= self.tokens.len()
    }

    #[inline]
    fn position_repr(&self, p: usize) -> Self::PositionRepr {
        p
    }
}

impl<'a> peg::ParseElem<'a> for Tokens<'a> {
    type Element = &'a Token;

    #[inline]
    fn parse_elem(&'a self, pos: usize) -> peg::RuleResult<Self::Element> {
        match self.tokens.get(pos) {
            Some(c) => peg::RuleResult::Matched(pos + 1, c),
            None => peg::RuleResult::Failed,
        }
    }
}

impl<'a> peg::ParseSlice<'a> for Tokens<'a> {
    type Slice = String;

    fn parse_slice(&'a self, start: usize, end: usize) -> Self::Slice {
        let mut result = String::new();
        let mut last_token_was_word = false;

        for token in &self.tokens[start..end] {
            match token {
                Token::Operator(s, _) => {
                    result.push_str(s);
                    last_token_was_word = false;
                }
                Token::Word(s, _) => {
                    // Place spaces between adjacent words.
                    if last_token_was_word {
                        result.push(' ');
                    }

                    result.push_str(s);
                    last_token_was_word = true;
                }
            }
        }

        result
    }
}

/// Information about the source of tokens.
#[derive(Clone, Debug, Default)]
pub struct SourceInfo {
    /// The source of the tokens.
    pub source: String,
}

impl std::fmt::Display for SourceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

peg::parser! {
    grammar token_parser<'a>(parser_options: &ParserOptions, source_info: &SourceInfo) for Tokens<'a> {
        pub(crate) rule program() -> ast::Program =
            linebreak() c:complete_commands() linebreak() { ast::Program { complete_commands: c } } /
            linebreak() { ast::Program { complete_commands: vec![] } }

        rule complete_commands() -> Vec<ast::CompleteCommand> =
            c:complete_command() ++ newline_list()

        rule complete_command() -> ast::CompleteCommand =
            first:and_or() remainder:(s:separator_op() l:and_or() { (s, l) })* last_sep:separator_op()? {
                let mut and_ors = vec![first];
                let mut seps = vec![];

                for (sep, ao) in remainder {
                    seps.push(sep);
                    and_ors.push(ao);
                }

                // N.B. We default to synchronous if no separator op is given.
                seps.push(last_sep.unwrap_or(SeparatorOperator::Sequence));

                let mut items = vec![];
                for (i, ao) in and_ors.into_iter().enumerate() {
                    items.push(ast::CompoundListItem(ao, seps[i].clone()));
                }

                ast::CompoundList(items)
            }

        rule and_or() -> ast::AndOrList =
            first:pipeline() additional:_and_or_item()* { ast::AndOrList { first, additional } }

        rule _and_or_item() -> ast::AndOr =
            op:_and_or_op() linebreak() p:pipeline() { op(p) }

        rule _and_or_op() -> fn(ast::Pipeline) -> ast::AndOr =
            specific_operator("&&") { ast::AndOr::And } /
            specific_operator("||") { ast::AndOr::Or }

        rule pipeline() -> ast::Pipeline =
            timed:pipeline_timed()? bang:bang()* seq:pipe_sequence() {?
                if timed.is_none() && bang.is_empty() && seq.is_empty() {
                    Err("empty pipeline")
                } else {
                    let invert = bang.len() % 2 == 1;
                    Ok(ast::Pipeline { timed, bang: invert, seq })
                }
            }

        rule pipeline_timed() -> ast::PipelineTimed =
            non_posix_extensions_enabled() s:specific_word("time") posix_output:specific_word("-p")? {
                let start = s.location();
                if let Some(end) = posix_output {
                    ast::PipelineTimed::TimedWithPosixOutput(TokenLocation::within(start, end.location()))
                } else {
                    ast::PipelineTimed::Timed(start.to_owned())
                }
            }

        rule bang() -> bool = specific_word("!") { true }

        pub(crate) rule pipe_sequence() -> Vec<ast::Command> =
            c:(c:command() r:&pipe_extension_redirection()? {? // check for `|&` without consuming the stream.
                let mut c = c;
                if r.is_some() {
                    add_pipe_extension_redirection(&mut c)?;
                }
                Ok(c)
            }) ** (pipe_operator() linebreak()) {
                c
            }

        rule pipe_operator() =
            specific_operator("|") /
            pipe_extension_redirection()

        rule pipe_extension_redirection() -> &'input Token  =
            non_posix_extensions_enabled() p:specific_operator("|&") { p }

        // N.B. We needed to move the function definition branch up to avoid conflicts with array assignment syntax.
        rule command() -> ast::Command =
            f:function_definition() { ast::Command::Function(f) } /
            c:simple_command() { ast::Command::Simple(c) } /
            c:compound_command() r:redirect_list()? { ast::Command::Compound(c, r) } /
            // N.B. Extended test commands are bash extensions.
            non_posix_extensions_enabled() c:extended_test_command() { ast::Command::ExtendedTest(c) } /
            expected!("command")

        // N.B. The arithmetic command is a non-sh extension.
        // N.B. The arithmetic for clause command is a non-sh extension.
        pub(crate) rule compound_command() -> ast::CompoundCommand =
            non_posix_extensions_enabled() a:arithmetic_command() { ast::CompoundCommand::Arithmetic(a) } /
            b:brace_group() { ast::CompoundCommand::BraceGroup(b) } /
            s:subshell() { ast::CompoundCommand::Subshell(s) } /
            f:for_clause() { ast::CompoundCommand::ForClause(f) } /
            c:case_clause() { ast::CompoundCommand::CaseClause(c) } /
            i:if_clause() { ast::CompoundCommand::IfClause(i) } /
            w:while_clause() { ast::CompoundCommand::WhileClause(w) } /
            u:until_clause() { ast::CompoundCommand::UntilClause(u) } /
            non_posix_extensions_enabled() c:arithmetic_for_clause() { ast::CompoundCommand::ArithmeticForClause(c) } /
            expected!("compound command")

        pub(crate) rule arithmetic_command() -> ast::ArithmeticCommand =
            start:specific_operator("(") specific_operator("(") expr:arithmetic_expression() specific_operator(")") end:specific_operator(")") {
                let loc = TokenLocation::within(
                    start.location(),
                    end.location()
                );
                ast::ArithmeticCommand { expr, loc }
            }

        pub(crate) rule arithmetic_expression() -> ast::UnexpandedArithmeticExpr =
            raw_expr:$(arithmetic_expression_piece()*) { ast::UnexpandedArithmeticExpr { value: raw_expr } }

        rule arithmetic_expression_piece() =
            // Allow a parenthesized expression (with matching opening and closing parens).
            specific_operator("(") (!specific_operator(")") arithmetic_expression_piece())* specific_operator(")") {} /
            // Otherwise consume any token that's neither the normal end of the entire arithmetic expression, nor an
            // unexpected mismatched closing parenthesis. In the latter case, it may be that this really was never an
            // arithmetic expression in the first place and we need to backtrack and instead try parsing as a subshell
            // command instead.
            !arithmetic_end() !specific_operator(")") [_] {}

        // TODO: evaluate arithmetic end; the semicolon is used in arithmetic for loops.
        rule arithmetic_end() -> () =
            specific_operator(")") specific_operator(")") {} /
            specific_operator(";") {}

        rule subshell() -> ast::SubshellCommand =
            start:specific_operator("(") list:compound_list() end:specific_operator(")") {
                let loc = TokenLocation::within(start.location(), end.location());
                ast::SubshellCommand { list, loc }
            }

        rule compound_list() -> ast::CompoundList =
            linebreak() first:and_or() remainder:(s:separator() l:and_or() { (s, l) })* last_sep:separator()? {
                let mut and_ors = vec![first];
                let mut seps = vec![];

                for (sep, ao) in remainder {
                    seps.push(sep.unwrap_or(SeparatorOperator::Sequence));
                    and_ors.push(ao);
                }

                // N.B. We default to synchronous if no separator op is given.
                let last_sep = last_sep.unwrap_or(None);
                seps.push(last_sep.unwrap_or(SeparatorOperator::Sequence));

                let mut items = vec![];
                for (i, ao) in and_ors.into_iter().enumerate() {
                    items.push(ast::CompoundListItem(ao, seps[i].clone()));
                }

                ast::CompoundList(items)
            }

        rule for_clause() -> ast::ForClauseCommand =
            s:specific_word("for") n:name() linebreak() _in() w:wordlist()? sequential_sep() d:do_group() {
                let start = s.location();
                let end = &d.loc;
                let loc = TokenLocation::within(start, end);
                ast::ForClauseCommand { variable_name: n.to_owned(), values: w, body: d, loc }
            } /
            s:specific_word("for") n:name() sequential_sep()? d:do_group() {
                let start = s.location();
                let end = &d.loc;
                let loc = TokenLocation::within(start, end);
                ast::ForClauseCommand { variable_name: n.to_owned(), values: None, body: d, loc }
            }

        // N.B. The arithmetic for loop is a non-sh extension.
        rule arithmetic_for_clause() -> ast::ArithmeticForClauseCommand =
            s:specific_word("for")
            specific_operator("(") specific_operator("(")
                initializer:arithmetic_expression()? specific_operator(";")
                condition:arithmetic_expression()? specific_operator(";")
                updater:arithmetic_expression()?
            specific_operator(")") specific_operator(")")
            body:arithmetic_for_body() {
                let start = s.location();
                let end = &body.loc;
                let loc = TokenLocation::within(start, end);
                ast::ArithmeticForClauseCommand { initializer, condition, updater, body, loc }
            }

        rule arithmetic_for_body() -> ast::DoGroupCommand =
            sequential_sep() body:do_group() { body } /
            body:brace_group() { ast::DoGroupCommand { list: body.list, loc: body.loc } }

        rule extended_test_command() -> ast::ExtendedTestExprCommand =
            s:specific_word("[[") linebreak() expr:extended_test_expression() linebreak() e:specific_word("]]") {
                let start = s.location();
                let end = e.location();
                let loc = TokenLocation::within(start, end);

                ast::ExtendedTestExprCommand { expr, loc }
            }

        rule extended_test_expression() -> ast::ExtendedTestExpr = precedence! {
            left:(@) linebreak() specific_operator("||") linebreak() right:@ { ast::ExtendedTestExpr::Or(Box::from(left), Box::from(right)) }
            --
            left:(@) linebreak() specific_operator("&&") linebreak() right:@ { ast::ExtendedTestExpr::And(Box::from(left), Box::from(right)) }
            --
            specific_word("!") e:@ { ast::ExtendedTestExpr::Not(Box::from(e)) }
            --
            specific_operator("(") e:extended_test_expression() specific_operator(")") { ast::ExtendedTestExpr::Parenthesized(Box::from(e)) }
            --
            // Arithmetic operators
            left:word() specific_word("-eq") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::ArithmeticEqualTo, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("-ne") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::ArithmeticNotEqualTo, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("-lt") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::ArithmeticLessThan, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("-le") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::ArithmeticLessThanOrEqualTo, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("-gt") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::ArithmeticGreaterThan, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("-ge") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::ArithmeticGreaterThanOrEqualTo, ast::Word::from(left), ast::Word::from(right)) }
            // Non-arithmetic binary operators
            left:word() specific_word("-ef") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::FilesReferToSameDeviceAndInodeNumbers, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("-nt") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::LeftFileIsNewerOrExistsWhenRightDoesNot, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("-ot") right:word() { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::LeftFileIsOlderOrDoesNotExistWhenRightDoes, ast::Word::from(left), ast::Word::from(right)) }
            left:word() (specific_word("==") / specific_word("=")) right:word()  { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::StringExactlyMatchesPattern, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("!=") right:word()  { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::StringDoesNotExactlyMatchPattern, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_word("=~") right:regex_word()  {
                if right.value.starts_with(['\'', '\"']) {
                    // TODO: Confirm it ends with that too?
                    ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::StringContainsSubstring, ast::Word::from(left), right)
                } else {
                    ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::StringMatchesRegex, ast::Word::from(left), right)
                }
            }
            left:word() specific_operator("<") right:word()   { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::LeftSortsBeforeRight, ast::Word::from(left), ast::Word::from(right)) }
            left:word() specific_operator(">") right:word()   { ast::ExtendedTestExpr::BinaryTest(ast::BinaryPredicate::LeftSortsAfterRight, ast::Word::from(left), ast::Word::from(right)) }
            --
            p:extended_unary_predicate() f:word() { ast::ExtendedTestExpr::UnaryTest(p, ast::Word::from(f)) }
            --
            w:word() { ast::ExtendedTestExpr::UnaryTest(ast::UnaryPredicate::StringHasNonZeroLength, ast::Word::from(w)) }
        }

        rule extended_unary_predicate() -> ast::UnaryPredicate =
            specific_word("-a") { ast::UnaryPredicate::FileExists } /
            specific_word("-b") { ast::UnaryPredicate::FileExistsAndIsBlockSpecialFile } /
            specific_word("-c") { ast::UnaryPredicate::FileExistsAndIsCharSpecialFile } /
            specific_word("-d") { ast::UnaryPredicate::FileExistsAndIsDir } /
            specific_word("-e") { ast::UnaryPredicate::FileExists } /
            specific_word("-f") { ast::UnaryPredicate::FileExistsAndIsRegularFile } /
            specific_word("-g") { ast::UnaryPredicate::FileExistsAndIsSetgid } /
            specific_word("-h") { ast::UnaryPredicate::FileExistsAndIsSymlink } /
            specific_word("-k") { ast::UnaryPredicate::FileExistsAndHasStickyBit } /
            specific_word("-n") { ast::UnaryPredicate::StringHasNonZeroLength } /
            specific_word("-o") { ast::UnaryPredicate::ShellOptionEnabled } /
            specific_word("-p") { ast::UnaryPredicate::FileExistsAndIsFifo } /
            specific_word("-r") { ast::UnaryPredicate::FileExistsAndIsReadable } /
            specific_word("-s") { ast::UnaryPredicate::FileExistsAndIsNotZeroLength } /
            specific_word("-t") { ast::UnaryPredicate::FdIsOpenTerminal } /
            specific_word("-u") { ast::UnaryPredicate::FileExistsAndIsSetuid } /
            specific_word("-v") { ast::UnaryPredicate::ShellVariableIsSetAndAssigned } /
            specific_word("-w") { ast::UnaryPredicate::FileExistsAndIsWritable } /
            specific_word("-x") { ast::UnaryPredicate::FileExistsAndIsExecutable } /
            specific_word("-z") { ast::UnaryPredicate::StringHasZeroLength } /
            specific_word("-G") { ast::UnaryPredicate::FileExistsAndOwnedByEffectiveGroupId } /
            specific_word("-L") { ast::UnaryPredicate::FileExistsAndIsSymlink } /
            specific_word("-N") { ast::UnaryPredicate::FileExistsAndModifiedSinceLastRead } /
            specific_word("-O") { ast::UnaryPredicate::FileExistsAndOwnedByEffectiveUserId } /
            specific_word("-R") { ast::UnaryPredicate::ShellVariableIsSetAndNameRef } /
            specific_word("-S") { ast::UnaryPredicate::FileExistsAndIsSocket }

        // N.B. For some reason we seem to need to allow a select subset
        // of unescaped operators in regex words.
        rule regex_word() -> ast::Word =
            value:$((!specific_word("]]") regex_word_piece())+) {
                ast::Word::from(value)
            }

        rule regex_word_piece() =
            word() {} /
            specific_operator("|") {} /
            specific_operator("(") parenthesized_regex_word()* specific_operator(")") {}

        rule parenthesized_regex_word() =
            regex_word_piece() /
            !specific_operator(")") !specific_operator("]]") [_]

        rule name() -> &'input str =
            w:[Token::Word(_, _)] { w.to_str() }

        rule _in() -> () =
            specific_word("in") { }

        rule wordlist() -> Vec<ast::Word> =
            (w:word() { ast::Word::from(w) })+

        pub(crate) rule case_clause() -> ast::CaseClauseCommand =
            specific_word("case") w:word() linebreak() _in() linebreak() first_items:case_item()* last_item:case_item_ns()? specific_word("esac") {
                let mut cases = first_items;

                if let Some(last_item) = last_item {
                    cases.push(last_item);
                }

                ast::CaseClauseCommand { value: ast::Word::from(w), cases }
            }

        pub(crate) rule case_item_ns() -> ast::CaseItem =
            s:specific_operator("(")? p:pattern() specific_operator(")") c:compound_list() {
                let start = s.map(|s| s.location()).or_else(|| p.first().and_then(|w| w.loc.as_ref()));
                let end = c.location();

                let loc = maybe_location(start, end.as_ref());

                ast::CaseItem { patterns: p, cmd: Some(c), post_action: ast::CaseItemPostAction::ExitCase, loc }
            } /
            s:specific_operator("(")? p:pattern() e:specific_operator(")") linebreak() {
                let start = s.map(|s| s.location()).or_else(|| p.first().and_then(|w| w.loc.as_ref()));
                let end = Some(e.location());

                let loc = maybe_location(start, end);
                ast::CaseItem { patterns: p, cmd: None, post_action: ast::CaseItemPostAction::ExitCase, loc }
            }

        pub(crate) rule case_item() -> ast::CaseItem =
            s:specific_operator("(")? p:pattern() specific_operator(")") linebreak() post_action:case_item_post_action() linebreak() {
                let start = s.map(|s| s.location()).or_else(|| p.first().and_then(|w| w.loc.as_ref()));
                let end = Some(post_action.1);
                let loc = maybe_location(start, end);
                ast::CaseItem { patterns: p, cmd: None, post_action: post_action.0, loc }
            } /
            s:specific_operator("(")? p:pattern() specific_operator(")") c:compound_list() post_action:case_item_post_action() linebreak() {
                let start = s.map(|s| s.location()).or_else(|| p.first().and_then(|w| w.loc.as_ref()));
                let end = Some(post_action.1);
                let loc = maybe_location(start, end);
                ast::CaseItem { patterns: p, cmd: Some(c), post_action: post_action.0, loc }
            }

        rule case_item_post_action() -> (ast::CaseItemPostAction, &'input TokenLocation)  =
            s:specific_operator(";;") {
                (ast::CaseItemPostAction::ExitCase, s.location())
            } /
            non_posix_extensions_enabled() s:specific_operator(";;&") {
                (ast::CaseItemPostAction::ContinueEvaluatingCases, s.location())
            } /
            non_posix_extensions_enabled() s:specific_operator(";&") {
                (ast::CaseItemPostAction::UnconditionallyExecuteNextCaseItem, s.location())
            }

        rule pattern() -> Vec<ast::Word> =
            (w:word() { ast::Word::from(w) }) ++ specific_operator("|")

        rule if_clause() -> ast::IfClauseCommand =
            s:specific_word("if") condition:compound_list() specific_word("then") then:compound_list() elses:else_part()? e:specific_word("fi") {
                let start = s.location();
                let end = s.location();
                let loc = TokenLocation::within(start, end);

                ast::IfClauseCommand {
                    condition,
                    then,
                    elses,
                    loc
                }
            }

        rule else_part() -> Vec<ast::ElseClause> =
            cs:_conditional_else_part()+ u:_unconditional_else_part()? {
                let mut parts = vec![];
                for c in cs {
                    parts.push(c);
                }

                if let Some(uncond) = u {
                    parts.push(uncond);
                }

                parts
            } /
            e:_unconditional_else_part() { vec![e] }

        rule _conditional_else_part() -> ast::ElseClause =
            specific_word("elif") condition:compound_list() specific_word("then") body:compound_list() {
                ast::ElseClause { condition: Some(condition), body }
            }

        rule _unconditional_else_part() -> ast::ElseClause =
            specific_word("else") body:compound_list() {
                ast::ElseClause { condition: None, body }
             }

        rule while_clause() -> ast::WhileOrUntilClauseCommand =
            s:specific_word("while") c:compound_list() d:do_group() {
                let start = s.location();
                let end = &d.loc;
                let loc = TokenLocation::within(start, end);

                ast::WhileOrUntilClauseCommand(c, d, loc)
            }

        rule until_clause() -> ast::WhileOrUntilClauseCommand =
            s:specific_word("until") c:compound_list() d:do_group() {
                let start = s.location();
                let end = &d.loc;
                let loc = TokenLocation::within(start, end);

                ast::WhileOrUntilClauseCommand(c, d, loc)
            }

        // N.B. Non-sh extensions allows use of the 'function' word to indicate a function definition.
        rule function_definition() -> ast::FunctionDefinition =
            specific_word("function")? fname:fname() body:function_parens_and_body() {
                ast::FunctionDefinition { fname, body, source: source_info.source.clone() }
            } /
            specific_word("function") fname:fname() linebreak() body:function_body() {
                ast::FunctionDefinition { fname, body, source: source_info.source.clone() }
            } /
            expected!("function definition")

        pub(crate) rule function_parens_and_body() -> ast::FunctionBody =
            specific_operator("(") specific_operator(")") linebreak() body:function_body() { body }

        rule function_body() -> ast::FunctionBody =
            c:compound_command() r:redirect_list()? { ast::FunctionBody(c, r) }

        rule fname() -> ast::Word =
            // Special-case: don't allow it to end with an equals sign, to avoid the challenge of
            // misinterpreting certain declaration assignments as function definitions.
            // TODO: Find a way to make this still work without requiring this targeted exception.
            w:[Token::Word(word, l) if !word.ends_with('=')] { ast::Word::with_location(word, l) }

        rule brace_group() -> ast::BraceGroupCommand =
            start:specific_word("{") list:compound_list() end:specific_word("}") {
                let loc = TokenLocation::within(start.location(), end.location());
                ast::BraceGroupCommand { list, loc }
            }

        rule do_group() -> ast::DoGroupCommand =
            start:specific_word("do") list:compound_list() end:specific_word("done") {
                let loc = TokenLocation::within(start.location(), end.location());
                ast::DoGroupCommand { list, loc }
            }

        rule simple_command() -> ast::SimpleCommand =
            prefix:cmd_prefix() word_and_suffix:(word_or_name:cmd_word() suffix:cmd_suffix()? { (word_or_name, suffix) })? {
                match word_and_suffix {
                    Some((word_or_name, suffix)) => {
                        ast::SimpleCommand { prefix: Some(prefix), word_or_name: Some(ast::Word::from(word_or_name)), suffix }
                    }
                    None => {
                        ast::SimpleCommand { prefix: Some(prefix), word_or_name: None, suffix: None }
                    }
                }
            } /
            word_or_name:cmd_name() suffix:cmd_suffix()? {
                ast::SimpleCommand { prefix: None, word_or_name: Some(ast::Word::from(word_or_name)), suffix } } /
            expected!("simple command")

        rule cmd_name() -> &'input Token =
            non_reserved_word()

        rule cmd_word() -> &'input Token =
            !assignment_word() w:non_reserved_word() { w }

        rule cmd_prefix() -> ast::CommandPrefix =
            p:(
                i:io_redirect() { ast::CommandPrefixOrSuffixItem::IoRedirect(i) } /
                assignment_and_word:assignment_word() {
                    let (assignment, word) = assignment_and_word;
                    ast::CommandPrefixOrSuffixItem::AssignmentWord(assignment, word)
                }
            )+ { ast::CommandPrefix(p) }

        rule cmd_suffix() -> ast::CommandSuffix =
            s:(
                non_posix_extensions_enabled() sub:process_substitution() {
                    let (kind, subshell) = sub;
                    ast::CommandPrefixOrSuffixItem::ProcessSubstitution(kind, subshell)
                } /
                i:io_redirect() {
                    ast::CommandPrefixOrSuffixItem::IoRedirect(i)
                } /
                assignment_and_word:assignment_word() {
                    let (assignment, word) = assignment_and_word;
                    ast::CommandPrefixOrSuffixItem::AssignmentWord(assignment, word)
                } /
                w:word() {
                    ast::CommandPrefixOrSuffixItem::Word(ast::Word::from(w))
                }
            )+ { ast::CommandSuffix(s) }

        rule redirect_list() -> ast::RedirectList =
            r:io_redirect()+ { ast::RedirectList(r) } /
            expected!("redirect list")

        // N.B. here strings are extensions to the POSIX standard.
        rule io_redirect() -> ast::IoRedirect =
            n:io_number()? f:io_file() {
                    let (kind, target) = f;
                    ast::IoRedirect::File(n, kind, target)
                } /
            non_posix_extensions_enabled() specific_operator("&>>") target:filename() { ast::IoRedirect::OutputAndError(ast::Word::from(target), true) } /
            non_posix_extensions_enabled() specific_operator("&>") target:filename() { ast::IoRedirect::OutputAndError(ast::Word::from(target), false) } /
            non_posix_extensions_enabled() n:io_number()? specific_operator("<<<") w:word() { ast::IoRedirect::HereString(n, ast::Word::from(w)) } /
            n:io_number()? h:io_here() { ast::IoRedirect::HereDocument(n, h) } /
            expected!("I/O redirect")

        // N.B. Process substitution forms are extensions to the POSIX standard.
        rule io_file() -> (ast::IoFileRedirectKind, ast::IoFileRedirectTarget) =
            specific_operator("<")  f:io_filename() { (ast::IoFileRedirectKind::Read, f) } /
            specific_operator("<&") f:io_fd_duplication_source() { (ast::IoFileRedirectKind::DuplicateInput, f) } /
            specific_operator(">")  f:io_filename() { (ast::IoFileRedirectKind::Write, f) } /
            specific_operator(">&") f:io_fd_duplication_source() { (ast::IoFileRedirectKind::DuplicateOutput, f) } /
            specific_operator(">>") f:io_filename() { (ast::IoFileRedirectKind::Append, f) } /
            specific_operator("<>") f:io_filename() { (ast::IoFileRedirectKind::ReadAndWrite, f) } /
            specific_operator(">|") f:io_filename() { (ast::IoFileRedirectKind::Clobber, f) }

        rule io_fd_duplication_source() -> ast::IoFileRedirectTarget =
            w:word() { ast::IoFileRedirectTarget::Duplicate(ast::Word::from(w)) }

        rule io_fd() -> u32 =
            w:[Token::Word(_, _)] {? w.to_str().parse().or(Err("io_fd u32")) }

        rule io_filename() -> ast::IoFileRedirectTarget =
            non_posix_extensions_enabled() sub:process_substitution() {
                let (kind, subshell) = sub;
                ast::IoFileRedirectTarget::ProcessSubstitution(kind, subshell)
            } /
            f:filename() { ast::IoFileRedirectTarget::Filename(ast::Word::from(f)) }

        rule filename() -> &'input Token =
            word()

        pub(crate) rule io_here() -> ast::IoHereDocument =
           specific_operator("<<-") here_tag:here_tag() doc:[_] closing_tag:here_tag() {
                let requires_expansion = !here_tag.to_str().contains(['\'', '"', '\\']);
                ast::IoHereDocument {
                    remove_tabs: true,
                    requires_expansion,
                    here_end: ast::Word::from(here_tag),
                    doc: ast::Word::from(doc)
                }
            } /
            specific_operator("<<") here_tag:here_tag() doc:[_] closing_tag:here_tag() {
                let requires_expansion = !here_tag.to_str().contains(['\'', '"', '\\']);
                ast::IoHereDocument {
                    remove_tabs: false,
                    requires_expansion,
                    here_end: ast::Word::from(here_tag),
                    doc: ast::Word::from(doc)
                }
            }

        rule here_tag() -> &'input Token =
            word()

        rule process_substitution() -> (ast::ProcessSubstitutionKind, ast::SubshellCommand) =
            specific_operator("<") s:subshell() { (ast::ProcessSubstitutionKind::Read, s) } /
            specific_operator(">") s:subshell() { (ast::ProcessSubstitutionKind::Write, s) }

        rule newline_list() -> () =
            newline()+ {}

        rule linebreak() -> () =
            quiet! {
                newline()* {}
            }

        rule separator_op() -> ast::SeparatorOperator =
            specific_operator("&") { ast::SeparatorOperator::Async } /
            specific_operator(";") { ast::SeparatorOperator::Sequence }

        rule separator() -> Option<ast::SeparatorOperator> =
            s:separator_op() linebreak() { Some(s) } /
            newline_list() { None }

        rule sequential_sep() -> () =
            specific_operator(";") linebreak() /
            newline_list()

        //
        // Token interpretation
        //

        rule non_reserved_word() -> &'input Token =
            !reserved_word() w:word() { w }

        rule word() -> &'input Token =
            [Token::Word(_, _)]

        rule reserved_word() -> &'input Token =
            [Token::Word(w, _) if matches!(w.as_str(),
                "!" |
                "{" |
                "}" |
                "case" |
                "do" |
                "done" |
                "elif" |
                "else" |
                "esac" |
                "fi" |
                "for" |
                "if" |
                "in" |
                "then" |
                "until" |
                "while"
            )] /

            // N.B. bash also treats the following as reserved.
            non_posix_extensions_enabled() token:non_posix_reserved_word_token() { token }

        rule non_posix_reserved_word_token() -> &'input Token =
            specific_word("[[") /
            specific_word("]]") /
            specific_word("function") /
            specific_word("select")

        rule newline() -> () = quiet! {
            specific_operator("\n") {}
        }

        pub(crate) rule assignment_word() -> (ast::Assignment, ast::Word) =
            non_posix_extensions_enabled() [Token::Word(w, l)] specific_operator("(") elements:array_elements() end:specific_operator(")") {?
                let mut parsed = parse_array_assignment(w.as_str(), elements.as_slice())?;

                let mut all_as_word = w.to_owned();
                all_as_word.push('(');
                for (i, e) in elements.iter().enumerate() {
                    if i > 0 {
                        all_as_word.push(' ');
                    }
                    all_as_word.push_str(e);
                }
                all_as_word.push(')');

                let loc = TokenLocation::within(l, end.location());
                parsed.loc = loc.clone();
                Ok((parsed, ast::Word::with_location(&all_as_word, &loc)))
            } /
            [Token::Word(w, l)] {?
                let mut parsed = parse_assignment_word(w.as_str())?;
                parsed.loc = l.clone();
                Ok((parsed, ast::Word::with_location(w, l)))
            }

        rule array_elements() -> Vec<&'input String> =
             linebreak() e:array_element()* { e }

        rule array_element() -> &'input String =
            linebreak() [Token::Word(e, _)] linebreak() { e }

        // N.B. An I/O number must be a string of only digits, and it must be
        // followed by a '<' or '>' character (but not consume them). We also
        // need to make sure that there was no space between the number and the
        // redirection operator; unfortunately we don't have the space anymore
        // but we can infer it by looking at the tokens' locations.
        rule io_number() -> ast::IoFd =
            [Token::Word(w, num_loc) if w.chars().all(|c: char| c.is_ascii_digit())]
            &([Token::Operator(o, redir_loc) if
                    o.starts_with(['<', '>']) &&
                    locations_are_contiguous(num_loc, redir_loc)]) {

                w.parse().unwrap()
            }

        //
        // Helpers
        //
        rule specific_operator(expected: &str) -> &'input Token =
            [Token::Operator(w, _) if w.as_str() == expected]

        rule specific_word(expected: &str) -> &'input Token =
            [Token::Word(w, _) if w.as_str() == expected]

        rule non_posix_extensions_enabled() -> () =
            &[_] {? if !parser_options.sh_mode { Ok(()) } else { Err("posix") } }
    }
}

peg::parser! {
    grammar assignments() for str {
        pub(crate) rule name_and_scalar_value() -> ast::Assignment =
            nae:name_and_equals() value:scalar_value() {
                let (name, append) = nae;
                ast::Assignment { name, value, append, loc: TokenLocation::default() }
            }

        pub(crate) rule name_and_equals() -> (ast::AssignmentName, bool) =
            name:name() append:("+"?) "=" {
                (name, append.is_some())
            }

        pub(crate) rule literal_array_element() -> (Option<String>, String) =
            "[" inner:$((!"]" [_])*) "]=" value:$([_]*) {
                (Some(inner.to_owned()), value.to_owned())
            } /
            value:$([_]+) {
                (None, value.to_owned())
            }

        rule name() -> ast::AssignmentName =
            aen:array_element_name() {
                let (name, index) = aen;
                ast::AssignmentName::ArrayElementName(name.to_owned(), index.to_owned())
            } /
            name:scalar_name() {
                ast::AssignmentName::VariableName(name.to_owned())
            }

        rule array_element_name() -> (&'input str, &'input str) =
            name:scalar_name() "[" ai:array_index() "]" { (name, ai) }

        rule array_index() -> &'input str =
            $((![']'] [_])*)

        rule scalar_name() -> &'input str =
            $(alpha_or_underscore() non_first_variable_char()*)

        rule non_first_variable_char() -> () =
            ['_' | '0'..='9' | 'a'..='z' | 'A'..='Z'] {}

        rule alpha_or_underscore() -> () =
            ['_' | 'a'..='z' | 'A'..='Z'] {}

        rule scalar_value() -> ast::AssignmentValue =
            v:$([_]*) { ast::AssignmentValue::Scalar(ast::Word::from(v.to_owned())) }
    }
}

fn parse_assignment_word(word: &str) -> Result<ast::Assignment, &'static str> {
    let parse_result = assignments::name_and_scalar_value(word);
    parse_result.map_err(|_| "not assignment word")
}

// add `2>&1` to the command if the pipeline is `|&`
fn add_pipe_extension_redirection(c: &mut ast::Command) -> Result<(), &'static str> {
    fn add_to_redirect_list(l: &mut Option<ast::RedirectList>, r: ast::IoRedirect) {
        if let Some(l) = l {
            l.0.push(r);
        } else {
            let v = vec![r];
            *l = Some(ast::RedirectList(v));
        }
    }

    let r = ast::IoRedirect::File(
        Some(2),
        ast::IoFileRedirectKind::DuplicateOutput,
        ast::IoFileRedirectTarget::Fd(1),
    );

    match c {
        ast::Command::Simple(c) => {
            let r = ast::CommandPrefixOrSuffixItem::IoRedirect(r);
            if let Some(l) = &mut c.suffix {
                l.0.push(r);
            } else {
                c.suffix = Some(ast::CommandSuffix(vec![r]));
            }
        }
        ast::Command::Compound(_, l) => add_to_redirect_list(l, r),
        ast::Command::Function(f) => add_to_redirect_list(&mut f.body.1, r),
        ast::Command::ExtendedTest(_) => return Err("|& unimplemented for extended tests"),
    }

    Ok(())
}

#[inline]
fn locations_are_contiguous(
    loc_left: &crate::TokenLocation,
    loc_right: &crate::TokenLocation,
) -> bool {
    loc_left.end.index == loc_right.start.index
}

fn parse_array_assignment(
    word: &str,
    elements: &[&String],
) -> Result<ast::Assignment, &'static str> {
    let (assignment_name, append) =
        assignments::name_and_equals(word).map_err(|_| "not array assignment word")?;

    let elements = elements
        .iter()
        .map(|element| assignments::literal_array_element(element))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "invalid array element in literal")?;

    let elements_as_words = elements
        .into_iter()
        .map(|(key, value)| {
            (
                key.map(|k| ast::Word::new(k.as_str())),
                ast::Word::new(value.as_str()),
            )
        })
        .collect();

    Ok(ast::Assignment {
        name: assignment_name,
        value: ast::AssignmentValue::Array(elements_as_words),
        append,
        loc: TokenLocation::default(),
    })
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::tokenize_str;
    use anyhow::Result;
    use insta::assert_ron_snapshot;

    #[derive(serde::Serialize)]
    struct ParseResult<'a, T> {
        input: &'a str,
        result: &'a T,
    }

    #[test]
    fn parse_case() -> Result<()> {
        let input = r"\
case x in
x)
    echo y;;
esac\
";

        let tokens = tokenize_str(input)?;
        let command = super::token_parser::case_clause(
            &Tokens {
                tokens: tokens.as_slice(),
            },
            &ParserOptions::default(),
            &SourceInfo::default(),
        )?;

        assert_ron_snapshot!(ParseResult {
            input,
            result: &command
        });

        Ok(())
    }

    #[test]
    fn parse_case_ns() -> Result<()> {
        let input = r"\
case x in
x)
    echo y
esac\
";

        let tokens = tokenize_str(input)?;
        let command = super::token_parser::case_clause(
            &Tokens {
                tokens: tokens.as_slice(),
            },
            &ParserOptions::default(),
            &SourceInfo::default(),
        )?;

        assert_ron_snapshot!(ParseResult {
            input,
            result: &command
        });

        Ok(())
    }

    #[test]
    fn parse_arith_and_non_arith_parens() -> Result<()> {
        let input = r"( : && ( (( 0 )) || : ) )";

        let tokens = tokenize_str(input)?;
        let result = super::token_parser::program(
            &Tokens {
                tokens: tokens.as_slice(),
            },
            &ParserOptions::default(),
            &SourceInfo::default(),
        )?;

        assert_ron_snapshot!(ParseResult {
            input,
            result: &result
        });

        Ok(())
    }

    #[test]
    fn parse_redirection() -> Result<()> {
        let input = r"echo |& wc";

        let tokens = tokenize_str(input)?;
        let seq = super::token_parser::pipe_sequence(
            &Tokens {
                tokens: tokens.as_slice(),
            },
            &ParserOptions::default(),
            &SourceInfo::default(),
        )?;

        assert_ron_snapshot!(ParseResult {
            input,
            result: &seq
        });

        Ok(())
    }

    #[test]
    fn parse_here_doc_with_no_trailing_newline() -> Result<()> {
        let input = r"cat <<EOF
Something
EOF";

        let tokens = tokenize_str(input)?;
        let result = super::token_parser::program(
            &Tokens {
                tokens: tokens.as_slice(),
            },
            &ParserOptions::default(),
            &SourceInfo::default(),
        )?;

        assert_ron_snapshot!(ParseResult {
            input,
            result: &result
        });

        Ok(())
    }

    #[test]
    fn parse_function_with_pipe_redirection() -> Result<()> {
        let inputs = [r"foo() { echo 1; } 2>&1 | cat", r"foo() { echo 1; } |& cat"];

        for input in inputs {
            let tokens = tokenize_str(input)?;
            let seq = super::token_parser::pipe_sequence(
                &Tokens {
                    tokens: tokens.as_slice(),
                },
                &ParserOptions::default(),
                &SourceInfo::default(),
            )?;

            assert_ron_snapshot!(ParseResult {
                input,
                result: &seq
            });
        }

        Ok(())
    }

    #[test]
    fn test_parse_program() -> Result<()> {
        let input = r#"

#!/usr/bin/env bash

for f in A B C; do

    # sdfsdf
    echo "${f@L}" >&2

   done

"#;

        let tokens = tokenize_str(input)?;
        let result = super::token_parser::program(
            &Tokens {
                tokens: tokens.as_slice(),
            },
            &ParserOptions::default(),
            &SourceInfo::default(),
        )?;

        assert_ron_snapshot!(ParseResult {
            input,
            result: &result
        });

        Ok(())
    }
}
