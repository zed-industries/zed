use brush_parser::ast;
use brush_parser::word::WordPiece;
use brush_parser::{Parser, ParserOptions, SourceInfo};
use std::io::BufReader;

pub fn extract_commands(command: &str) -> Option<Vec<String>> {
    let reader = BufReader::new(command.as_bytes());
    let options = ParserOptions::default();
    let source_info = SourceInfo::default();
    let mut parser = Parser::new(reader, &options, &source_info);

    let program = parser.parse_program().ok()?;

    let mut commands = Vec::new();
    extract_commands_from_program(&program, &mut commands)?;

    Some(commands)
}

fn extract_commands_from_program(program: &ast::Program, commands: &mut Vec<String>) -> Option<()> {
    for complete_command in &program.complete_commands {
        extract_commands_from_compound_list(complete_command, commands)?;
    }
    Some(())
}

fn extract_commands_from_compound_list(
    compound_list: &ast::CompoundList,
    commands: &mut Vec<String>,
) -> Option<()> {
    for item in &compound_list.0 {
        extract_commands_from_and_or_list(&item.0, commands)?;
    }
    Some(())
}

fn extract_commands_from_and_or_list(
    and_or_list: &ast::AndOrList,
    commands: &mut Vec<String>,
) -> Option<()> {
    extract_commands_from_pipeline(&and_or_list.first, commands)?;

    for and_or in &and_or_list.additional {
        match and_or {
            ast::AndOr::And(pipeline) | ast::AndOr::Or(pipeline) => {
                extract_commands_from_pipeline(pipeline, commands)?;
            }
        }
    }
    Some(())
}

fn extract_commands_from_pipeline(
    pipeline: &ast::Pipeline,
    commands: &mut Vec<String>,
) -> Option<()> {
    for command in &pipeline.seq {
        extract_commands_from_command(command, commands)?;
    }
    Some(())
}

fn extract_commands_from_command(command: &ast::Command, commands: &mut Vec<String>) -> Option<()> {
    match command {
        ast::Command::Simple(simple_command) => {
            extract_commands_from_simple_command(simple_command, commands)?;
        }
        ast::Command::Compound(compound_command, _redirect_list) => {
            extract_commands_from_compound_command(compound_command, commands)?;
        }
        ast::Command::Function(func_def) => {
            extract_commands_from_function_body(&func_def.body, commands)?;
        }
        ast::Command::ExtendedTest(test_expr) => {
            extract_commands_from_extended_test_expr(test_expr, commands)?;
        }
    }
    Some(())
}

fn extract_commands_from_simple_command(
    simple_command: &ast::SimpleCommand,
    commands: &mut Vec<String>,
) -> Option<()> {
    // Build a normalized command string from individual words, stripping shell
    // quotes so that security patterns match regardless of quoting style.
    // For example, both `rm -rf '/'` and `rm -rf /` normalize to "rm -rf /".
    //
    // If any word fails to normalize, we return None so that `extract_commands`
    // returns None — the same as a shell parse failure. The caller then falls
    // back to raw-input matching with always_allow disabled.
    let mut words = Vec::new();
    if let Some(word) = &simple_command.word_or_name {
        words.push(normalize_word(word)?);
    }
    if let Some(suffix) = &simple_command.suffix {
        for item in &suffix.0 {
            if let ast::CommandPrefixOrSuffixItem::Word(word) = item {
                words.push(normalize_word(word)?);
            }
        }
    }
    let command_str = words.join(" ");
    if !command_str.is_empty() {
        commands.push(command_str);
    }

    // Extract nested commands from command substitutions, process substitutions, etc.
    if let Some(prefix) = &simple_command.prefix {
        extract_commands_from_command_prefix(prefix, commands)?;
    }
    if let Some(word) = &simple_command.word_or_name {
        extract_commands_from_word(word, commands);
    }
    if let Some(suffix) = &simple_command.suffix {
        extract_commands_from_command_suffix(suffix, commands)?;
    }
    Some(())
}

/// Normalizes a shell word by stripping quoting syntax and returning the
/// semantic (unquoted) value. Returns `None` if word parsing fails.
fn normalize_word(word: &ast::Word) -> Option<String> {
    let options = ParserOptions::default();
    let pieces = brush_parser::word::parse(&word.value, &options).ok()?;
    let mut result = String::new();
    for piece_with_source in &pieces {
        normalize_word_piece_into(
            &piece_with_source.piece,
            &word.value,
            piece_with_source.start_index,
            piece_with_source.end_index,
            &mut result,
        );
    }
    Some(result)
}

fn normalize_word_piece_into(
    piece: &WordPiece,
    raw_value: &str,
    start_index: usize,
    end_index: usize,
    result: &mut String,
) {
    match piece {
        WordPiece::Text(text) => result.push_str(text),
        WordPiece::SingleQuotedText(text) => result.push_str(text),
        WordPiece::AnsiCQuotedText(text) => result.push_str(text),
        WordPiece::EscapeSequence(text) => {
            result.push_str(text.strip_prefix('\\').unwrap_or(text));
        }
        WordPiece::DoubleQuotedSequence(pieces)
        | WordPiece::GettextDoubleQuotedSequence(pieces) => {
            for inner in pieces {
                normalize_word_piece_into(
                    &inner.piece,
                    raw_value,
                    inner.start_index,
                    inner.end_index,
                    result,
                );
            }
        }
        WordPiece::TildePrefix(prefix) => {
            result.push('~');
            result.push_str(prefix);
        }
        // For parameter expansions, command substitutions, and arithmetic expressions,
        // preserve the original source text so that patterns like `\$HOME` continue
        // to match.
        WordPiece::ParameterExpansion(_)
        | WordPiece::CommandSubstitution(_)
        | WordPiece::BackquotedCommandSubstitution(_)
        | WordPiece::ArithmeticExpression(_) => {
            if let Some(source) = raw_value.get(start_index..end_index) {
                result.push_str(source);
            }
        }
    }
}

fn extract_commands_from_command_prefix(
    prefix: &ast::CommandPrefix,
    commands: &mut Vec<String>,
) -> Option<()> {
    for item in &prefix.0 {
        extract_commands_from_prefix_or_suffix_item(item, commands)?;
    }
    Some(())
}

fn extract_commands_from_command_suffix(
    suffix: &ast::CommandSuffix,
    commands: &mut Vec<String>,
) -> Option<()> {
    for item in &suffix.0 {
        extract_commands_from_prefix_or_suffix_item(item, commands)?;
    }
    Some(())
}

fn extract_commands_from_prefix_or_suffix_item(
    item: &ast::CommandPrefixOrSuffixItem,
    commands: &mut Vec<String>,
) -> Option<()> {
    match item {
        ast::CommandPrefixOrSuffixItem::IoRedirect(redirect) => {
            extract_commands_from_io_redirect(redirect, commands)?;
        }
        ast::CommandPrefixOrSuffixItem::AssignmentWord(assignment, _word) => {
            extract_commands_from_assignment(assignment, commands);
        }
        ast::CommandPrefixOrSuffixItem::Word(word) => {
            extract_commands_from_word(word, commands);
        }
        ast::CommandPrefixOrSuffixItem::ProcessSubstitution(_kind, subshell) => {
            extract_commands_from_compound_list(&subshell.list, commands)?;
        }
    }
    Some(())
}

fn extract_commands_from_io_redirect(
    redirect: &ast::IoRedirect,
    commands: &mut Vec<String>,
) -> Option<()> {
    match redirect {
        ast::IoRedirect::File(_fd, _kind, target) => {
            if let ast::IoFileRedirectTarget::ProcessSubstitution(_kind, subshell) = target {
                extract_commands_from_compound_list(&subshell.list, commands)?;
            }
        }
        ast::IoRedirect::HereDocument(_fd, _here_doc) => {}
        ast::IoRedirect::HereString(_fd, word) => {
            extract_commands_from_word(word, commands);
        }
        ast::IoRedirect::OutputAndError(word, _) => {
            extract_commands_from_word(word, commands);
        }
    }
    Some(())
}

fn extract_commands_from_assignment(assignment: &ast::Assignment, commands: &mut Vec<String>) {
    match &assignment.value {
        ast::AssignmentValue::Scalar(word) => {
            extract_commands_from_word(word, commands);
        }
        ast::AssignmentValue::Array(words) => {
            for (opt_word, word) in words {
                if let Some(w) = opt_word {
                    extract_commands_from_word(w, commands);
                }
                extract_commands_from_word(word, commands);
            }
        }
    }
}

fn extract_commands_from_word(word: &ast::Word, commands: &mut Vec<String>) {
    let options = ParserOptions::default();
    if let Ok(pieces) = brush_parser::word::parse(&word.value, &options) {
        for piece_with_source in pieces {
            extract_commands_from_word_piece(&piece_with_source.piece, commands);
        }
    }
}

fn extract_commands_from_word_piece(piece: &WordPiece, commands: &mut Vec<String>) {
    match piece {
        WordPiece::CommandSubstitution(cmd_str)
        | WordPiece::BackquotedCommandSubstitution(cmd_str) => {
            if let Some(nested_commands) = extract_commands(cmd_str) {
                commands.extend(nested_commands);
            }
        }
        WordPiece::DoubleQuotedSequence(pieces)
        | WordPiece::GettextDoubleQuotedSequence(pieces) => {
            for inner_piece_with_source in pieces {
                extract_commands_from_word_piece(&inner_piece_with_source.piece, commands);
            }
        }
        WordPiece::EscapeSequence(_)
        | WordPiece::SingleQuotedText(_)
        | WordPiece::Text(_)
        | WordPiece::AnsiCQuotedText(_)
        | WordPiece::TildePrefix(_)
        | WordPiece::ParameterExpansion(_)
        | WordPiece::ArithmeticExpression(_) => {}
    }
}

fn extract_commands_from_compound_command(
    compound_command: &ast::CompoundCommand,
    commands: &mut Vec<String>,
) -> Option<()> {
    match compound_command {
        ast::CompoundCommand::BraceGroup(brace_group) => {
            extract_commands_from_compound_list(&brace_group.list, commands)?;
        }
        ast::CompoundCommand::Subshell(subshell) => {
            extract_commands_from_compound_list(&subshell.list, commands)?;
        }
        ast::CompoundCommand::ForClause(for_clause) => {
            if let Some(words) = &for_clause.values {
                for word in words {
                    extract_commands_from_word(word, commands);
                }
            }
            extract_commands_from_do_group(&for_clause.body, commands)?;
        }
        ast::CompoundCommand::CaseClause(case_clause) => {
            extract_commands_from_word(&case_clause.value, commands);
            for item in &case_clause.cases {
                if let Some(body) = &item.cmd {
                    extract_commands_from_compound_list(body, commands)?;
                }
            }
        }
        ast::CompoundCommand::IfClause(if_clause) => {
            extract_commands_from_compound_list(&if_clause.condition, commands)?;
            extract_commands_from_compound_list(&if_clause.then, commands)?;
            if let Some(elses) = &if_clause.elses {
                for else_item in elses {
                    if let Some(condition) = &else_item.condition {
                        extract_commands_from_compound_list(condition, commands)?;
                    }
                    extract_commands_from_compound_list(&else_item.body, commands)?;
                }
            }
        }
        ast::CompoundCommand::WhileClause(while_clause)
        | ast::CompoundCommand::UntilClause(while_clause) => {
            extract_commands_from_compound_list(&while_clause.0, commands)?;
            extract_commands_from_do_group(&while_clause.1, commands)?;
        }
        ast::CompoundCommand::ArithmeticForClause(arith_for) => {
            extract_commands_from_do_group(&arith_for.body, commands)?;
        }
        ast::CompoundCommand::Arithmetic(_arith_cmd) => {}
    }
    Some(())
}

fn extract_commands_from_do_group(
    do_group: &ast::DoGroupCommand,
    commands: &mut Vec<String>,
) -> Option<()> {
    extract_commands_from_compound_list(&do_group.list, commands)
}

fn extract_commands_from_function_body(
    func_body: &ast::FunctionBody,
    commands: &mut Vec<String>,
) -> Option<()> {
    extract_commands_from_compound_command(&func_body.0, commands)
}

fn extract_commands_from_extended_test_expr(
    test_expr: &ast::ExtendedTestExprCommand,
    commands: &mut Vec<String>,
) -> Option<()> {
    extract_commands_from_extended_test_expr_inner(&test_expr.expr, commands)
}

fn extract_commands_from_extended_test_expr_inner(
    expr: &ast::ExtendedTestExpr,
    commands: &mut Vec<String>,
) -> Option<()> {
    match expr {
        ast::ExtendedTestExpr::Not(inner) => {
            extract_commands_from_extended_test_expr_inner(inner, commands)?;
        }
        ast::ExtendedTestExpr::And(left, right) | ast::ExtendedTestExpr::Or(left, right) => {
            extract_commands_from_extended_test_expr_inner(left, commands)?;
            extract_commands_from_extended_test_expr_inner(right, commands)?;
        }
        ast::ExtendedTestExpr::Parenthesized(inner) => {
            extract_commands_from_extended_test_expr_inner(inner, commands)?;
        }
        ast::ExtendedTestExpr::UnaryTest(_, word) => {
            extract_commands_from_word(word, commands);
        }
        ast::ExtendedTestExpr::BinaryTest(_, word1, word2) => {
            extract_commands_from_word(word1, commands);
            extract_commands_from_word(word2, commands);
        }
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let commands = extract_commands("ls").expect("parse failed");
        assert_eq!(commands, vec!["ls"]);
    }

    #[test]
    fn test_command_with_args() {
        let commands = extract_commands("ls -la /tmp").expect("parse failed");
        assert_eq!(commands, vec!["ls -la /tmp"]);
    }

    #[test]
    fn test_single_quoted_argument_is_normalized() {
        let commands = extract_commands("rm -rf '/'").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf /"]);
    }

    #[test]
    fn test_single_quoted_command_name_is_normalized() {
        let commands = extract_commands("'rm' -rf /").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf /"]);
    }

    #[test]
    fn test_double_quoted_argument_is_normalized() {
        let commands = extract_commands("rm -rf \"/\"").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf /"]);
    }

    #[test]
    fn test_double_quoted_command_name_is_normalized() {
        let commands = extract_commands("\"rm\" -rf /").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf /"]);
    }

    #[test]
    fn test_escaped_argument_is_normalized() {
        let commands = extract_commands("rm -rf \\/").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf /"]);
    }

    #[test]
    fn test_partial_quoting_command_name_is_normalized() {
        let commands = extract_commands("r'm' -rf /").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf /"]);
    }

    #[test]
    fn test_partial_quoting_flag_is_normalized() {
        let commands = extract_commands("rm -r'f' /").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf /"]);
    }

    #[test]
    fn test_quoted_bypass_in_chained_command() {
        let commands = extract_commands("ls && 'rm' -rf '/'").expect("parse failed");
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_tilde_preserved_after_normalization() {
        let commands = extract_commands("rm -rf ~").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf ~"]);
    }

    #[test]
    fn test_quoted_tilde_normalized() {
        let commands = extract_commands("rm -rf '~'").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf ~"]);
    }

    #[test]
    fn test_parameter_expansion_preserved() {
        let commands = extract_commands("rm -rf $HOME").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf $HOME"]);
    }

    #[test]
    fn test_braced_parameter_expansion_preserved() {
        let commands = extract_commands("rm -rf ${HOME}").expect("parse failed");
        assert_eq!(commands, vec!["rm -rf ${HOME}"]);
    }

    #[test]
    fn test_and_operator() {
        let commands = extract_commands("ls && rm -rf /").expect("parse failed");
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_or_operator() {
        let commands = extract_commands("ls || rm -rf /").expect("parse failed");
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_semicolon() {
        let commands = extract_commands("ls; rm -rf /").expect("parse failed");
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_pipe() {
        let commands = extract_commands("ls | xargs rm -rf").expect("parse failed");
        assert_eq!(commands, vec!["ls", "xargs rm -rf"]);
    }

    #[test]
    fn test_background() {
        let commands = extract_commands("ls & rm -rf /").expect("parse failed");
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_command_substitution_dollar() {
        let commands = extract_commands("echo $(whoami)").expect("parse failed");
        assert!(commands.iter().any(|c| c.contains("echo")));
        assert!(commands.contains(&"whoami".to_string()));
    }

    #[test]
    fn test_command_substitution_backticks() {
        let commands = extract_commands("echo `whoami`").expect("parse failed");
        assert!(commands.iter().any(|c| c.contains("echo")));
        assert!(commands.contains(&"whoami".to_string()));
    }

    #[test]
    fn test_process_substitution_input() {
        let commands = extract_commands("cat <(ls)").expect("parse failed");
        assert!(commands.iter().any(|c| c.contains("cat")));
        assert!(commands.contains(&"ls".to_string()));
    }

    #[test]
    fn test_process_substitution_output() {
        let commands = extract_commands("ls >(cat)").expect("parse failed");
        assert!(commands.iter().any(|c| c.contains("ls")));
        assert!(commands.contains(&"cat".to_string()));
    }

    #[test]
    fn test_newline_separator() {
        let commands = extract_commands("ls\nrm -rf /").expect("parse failed");
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_subshell() {
        let commands = extract_commands("(ls && rm -rf /)").expect("parse failed");
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_mixed_operators() {
        let commands = extract_commands("ls; echo hello && rm -rf /").expect("parse failed");
        assert_eq!(commands, vec!["ls", "echo hello", "rm -rf /"]);
    }

    #[test]
    fn test_no_spaces_around_operators() {
        let commands = extract_commands("ls&&rm").expect("parse failed");
        assert_eq!(commands, vec!["ls", "rm"]);
    }

    #[test]
    fn test_nested_command_substitution() {
        let commands = extract_commands("echo $(cat $(whoami).txt)").expect("parse failed");
        assert!(commands.iter().any(|c| c.contains("echo")));
        assert!(commands.iter().any(|c| c.contains("cat")));
        assert!(commands.contains(&"whoami".to_string()));
    }

    #[test]
    fn test_empty_command() {
        let commands = extract_commands("").expect("parse failed");
        assert!(commands.is_empty());
    }

    #[test]
    fn test_invalid_syntax_returns_none() {
        let result = extract_commands("ls &&");
        assert!(result.is_none());
    }
}
