use brush_parser::ast;
use brush_parser::word::WordPiece;
use brush_parser::{Parser, ParserOptions, SourceInfo};
use std::io::BufReader;

type CommandIter<'a> = Box<dyn Iterator<Item = String> + 'a>;

pub fn extract_commands(command: &str) -> Result<impl Iterator<Item = String>, ShellParseError> {
    let reader = BufReader::new(command.as_bytes());
    let options = ParserOptions::default();
    let source_info = SourceInfo::default();
    let mut parser = Parser::new(reader, &options, &source_info);

    let program = parser
        .parse_program()
        .map_err(|e| ShellParseError::ParseError(e.to_string()))?;

    Ok(extract_commands_from_program(program))
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ShellParseError {
    ParseError(String),
}

fn extract_commands_from_program(program: ast::Program) -> impl Iterator<Item = String> {
    program
        .complete_commands
        .into_iter()
        .flat_map(extract_commands_from_compound_list)
}

fn extract_commands_from_compound_list(compound_list: ast::CompoundList) -> CommandIter<'static> {
    Box::new(
        compound_list
            .0
            .into_iter()
            .flat_map(|item| extract_commands_from_and_or_list(item.0)),
    )
}

fn extract_commands_from_and_or_list(and_or_list: ast::AndOrList) -> CommandIter<'static> {
    let first = extract_commands_from_pipeline(and_or_list.first);
    let additional = and_or_list.additional.into_iter().flat_map(|and_or| {
        let pipeline = match and_or {
            ast::AndOr::And(pipeline) | ast::AndOr::Or(pipeline) => pipeline,
        };
        extract_commands_from_pipeline(pipeline)
    });
    Box::new(first.chain(additional))
}

fn extract_commands_from_pipeline(pipeline: ast::Pipeline) -> CommandIter<'static> {
    Box::new(
        pipeline
            .seq
            .into_iter()
            .flat_map(extract_commands_from_command),
    )
}

fn extract_commands_from_command(command: ast::Command) -> CommandIter<'static> {
    match command {
        ast::Command::Simple(simple_command) => {
            extract_commands_from_simple_command(simple_command)
        }
        ast::Command::Compound(compound_command, _redirect_list) => {
            extract_commands_from_compound_command(compound_command)
        }
        ast::Command::Function(func_def) => extract_commands_from_function_body(func_def.body),
        ast::Command::ExtendedTest(test_expr) => {
            extract_commands_from_extended_test_expr(test_expr)
        }
    }
}

fn extract_commands_from_simple_command(
    simple_command: ast::SimpleCommand,
) -> CommandIter<'static> {
    let command_str = simple_command.to_string();
    let main_command = if !command_str.trim().is_empty() {
        Some(command_str)
    } else {
        None
    };

    let prefix_commands = simple_command
        .prefix
        .into_iter()
        .flat_map(extract_commands_from_command_prefix);

    let word_commands = simple_command
        .word_or_name
        .into_iter()
        .flat_map(extract_commands_from_word);

    let suffix_commands = simple_command
        .suffix
        .into_iter()
        .flat_map(extract_commands_from_command_suffix);

    Box::new(
        main_command
            .into_iter()
            .chain(prefix_commands)
            .chain(word_commands)
            .chain(suffix_commands),
    )
}

fn extract_commands_from_command_prefix(prefix: ast::CommandPrefix) -> CommandIter<'static> {
    Box::new(
        prefix
            .0
            .into_iter()
            .flat_map(extract_commands_from_prefix_or_suffix_item),
    )
}

fn extract_commands_from_command_suffix(suffix: ast::CommandSuffix) -> CommandIter<'static> {
    Box::new(
        suffix
            .0
            .into_iter()
            .flat_map(extract_commands_from_prefix_or_suffix_item),
    )
}

fn extract_commands_from_prefix_or_suffix_item(
    item: ast::CommandPrefixOrSuffixItem,
) -> CommandIter<'static> {
    match item {
        ast::CommandPrefixOrSuffixItem::IoRedirect(redirect) => {
            extract_commands_from_io_redirect(redirect)
        }
        ast::CommandPrefixOrSuffixItem::AssignmentWord(assignment, _word) => {
            extract_commands_from_assignment(assignment)
        }
        ast::CommandPrefixOrSuffixItem::Word(word) => extract_commands_from_word(word),
        ast::CommandPrefixOrSuffixItem::ProcessSubstitution(_kind, subshell) => {
            extract_commands_from_compound_list(subshell.list)
        }
    }
}

fn extract_commands_from_io_redirect(redirect: ast::IoRedirect) -> CommandIter<'static> {
    match redirect {
        ast::IoRedirect::File(_fd, _kind, target) => {
            if let ast::IoFileRedirectTarget::ProcessSubstitution(_kind, subshell) = target {
                extract_commands_from_compound_list(subshell.list)
            } else {
                Box::new(std::iter::empty())
            }
        }
        ast::IoRedirect::HereDocument(_fd, _here_doc) => Box::new(std::iter::empty()),
        ast::IoRedirect::HereString(_fd, word) => extract_commands_from_word(word),
        ast::IoRedirect::OutputAndError(word, _) => extract_commands_from_word(word),
    }
}

fn extract_commands_from_assignment(assignment: ast::Assignment) -> CommandIter<'static> {
    match assignment.value {
        ast::AssignmentValue::Scalar(word) => extract_commands_from_word(word),
        ast::AssignmentValue::Array(words) => {
            Box::new(words.into_iter().flat_map(|(opt_word, word)| {
                let opt_iter = opt_word.into_iter().flat_map(extract_commands_from_word);
                let word_iter = extract_commands_from_word(word);
                opt_iter.chain(word_iter)
            }))
        }
    }
}

fn extract_commands_from_word(word: ast::Word) -> CommandIter<'static> {
    let options = ParserOptions::default();
    match brush_parser::word::parse(&word.value, &options) {
        Ok(pieces) => Box::new(pieces.into_iter().flat_map(|piece_with_source| {
            extract_commands_from_word_piece(piece_with_source.piece)
        })),
        Err(_) => Box::new(std::iter::empty()),
    }
}

fn extract_commands_from_word_piece(piece: WordPiece) -> CommandIter<'static> {
    match piece {
        WordPiece::CommandSubstitution(cmd_str)
        | WordPiece::BackquotedCommandSubstitution(cmd_str) => match extract_commands(&cmd_str) {
            Ok(iter) => Box::new(iter.collect::<Vec<_>>().into_iter()),
            Err(_) => Box::new(std::iter::empty()),
        },
        WordPiece::DoubleQuotedSequence(pieces)
        | WordPiece::GettextDoubleQuotedSequence(pieces) => {
            Box::new(pieces.into_iter().flat_map(|inner_piece_with_source| {
                extract_commands_from_word_piece(inner_piece_with_source.piece)
            }))
        }
        WordPiece::EscapeSequence(_)
        | WordPiece::SingleQuotedText(_)
        | WordPiece::Text(_)
        | WordPiece::AnsiCQuotedText(_)
        | WordPiece::TildePrefix(_)
        | WordPiece::ParameterExpansion(_)
        | WordPiece::ArithmeticExpression(_) => Box::new(std::iter::empty()),
    }
}

fn extract_commands_from_compound_command(
    compound_command: ast::CompoundCommand,
) -> CommandIter<'static> {
    match compound_command {
        ast::CompoundCommand::BraceGroup(brace_group) => {
            extract_commands_from_compound_list(brace_group.list)
        }
        ast::CompoundCommand::Subshell(subshell) => {
            extract_commands_from_compound_list(subshell.list)
        }
        ast::CompoundCommand::ForClause(for_clause) => {
            let word_commands = for_clause
                .values
                .into_iter()
                .flat_map(|words| words.into_iter().flat_map(extract_commands_from_word));
            let body_commands = extract_commands_from_do_group(for_clause.body);
            Box::new(word_commands.chain(body_commands))
        }
        ast::CompoundCommand::CaseClause(case_clause) => {
            let value_commands = extract_commands_from_word(case_clause.value);
            let case_commands = case_clause.cases.into_iter().flat_map(|item| {
                item.cmd
                    .into_iter()
                    .flat_map(extract_commands_from_compound_list)
            });
            Box::new(value_commands.chain(case_commands))
        }
        ast::CompoundCommand::IfClause(if_clause) => {
            let condition_commands = extract_commands_from_compound_list(if_clause.condition);
            let then_commands = extract_commands_from_compound_list(if_clause.then);
            let else_commands = if_clause.elses.into_iter().flat_map(|elses| {
                elses.into_iter().flat_map(|else_item| {
                    let cond_iter = else_item
                        .condition
                        .into_iter()
                        .flat_map(extract_commands_from_compound_list);
                    let body_iter = extract_commands_from_compound_list(else_item.body);
                    cond_iter.chain(body_iter)
                })
            });
            Box::new(condition_commands.chain(then_commands).chain(else_commands))
        }
        ast::CompoundCommand::WhileClause(while_clause)
        | ast::CompoundCommand::UntilClause(while_clause) => {
            let condition_commands = extract_commands_from_compound_list(while_clause.0);
            let body_commands = extract_commands_from_do_group(while_clause.1);
            Box::new(condition_commands.chain(body_commands))
        }
        ast::CompoundCommand::ArithmeticForClause(arith_for) => {
            extract_commands_from_do_group(arith_for.body)
        }
        ast::CompoundCommand::Arithmetic(_arith_cmd) => Box::new(std::iter::empty()),
    }
}

fn extract_commands_from_do_group(do_group: ast::DoGroupCommand) -> CommandIter<'static> {
    extract_commands_from_compound_list(do_group.list)
}

fn extract_commands_from_function_body(func_body: ast::FunctionBody) -> CommandIter<'static> {
    extract_commands_from_compound_command(func_body.0)
}

fn extract_commands_from_extended_test_expr(
    test_expr: ast::ExtendedTestExprCommand,
) -> CommandIter<'static> {
    extract_commands_from_extended_test_expr_inner(test_expr.expr)
}

fn extract_commands_from_extended_test_expr_inner(
    expr: ast::ExtendedTestExpr,
) -> CommandIter<'static> {
    match expr {
        ast::ExtendedTestExpr::Not(inner) => extract_commands_from_extended_test_expr_inner(*inner),
        ast::ExtendedTestExpr::And(left, right) | ast::ExtendedTestExpr::Or(left, right) => {
            let left_commands = extract_commands_from_extended_test_expr_inner(*left);
            let right_commands = extract_commands_from_extended_test_expr_inner(*right);
            Box::new(left_commands.chain(right_commands))
        }
        ast::ExtendedTestExpr::Parenthesized(inner) => {
            extract_commands_from_extended_test_expr_inner(*inner)
        }
        ast::ExtendedTestExpr::UnaryTest(_, word) => extract_commands_from_word(word),
        ast::ExtendedTestExpr::BinaryTest(_, word1, word2) => {
            let word1_commands = extract_commands_from_word(word1);
            let word2_commands = extract_commands_from_word(word2);
            Box::new(word1_commands.chain(word2_commands))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let commands: Vec<_> = extract_commands("ls").unwrap().collect();
        assert_eq!(commands, vec!["ls"]);
    }

    #[test]
    fn test_command_with_args() {
        let commands: Vec<_> = extract_commands("ls -la /tmp").unwrap().collect();
        assert_eq!(commands, vec!["ls -la /tmp"]);
    }

    #[test]
    fn test_and_operator() {
        let commands: Vec<_> = extract_commands("ls && rm -rf /").unwrap().collect();
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_or_operator() {
        let commands: Vec<_> = extract_commands("ls || rm -rf /").unwrap().collect();
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_semicolon() {
        let commands: Vec<_> = extract_commands("ls; rm -rf /").unwrap().collect();
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_pipe() {
        let commands: Vec<_> = extract_commands("ls | xargs rm -rf").unwrap().collect();
        assert_eq!(commands, vec!["ls", "xargs rm -rf"]);
    }

    #[test]
    fn test_background() {
        let commands: Vec<_> = extract_commands("ls & rm -rf /").unwrap().collect();
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_command_substitution_dollar() {
        let commands: Vec<_> = extract_commands("echo $(whoami)").unwrap().collect();
        assert!(commands.iter().any(|c| c.contains("echo")));
        assert!(commands.contains(&"whoami".to_string()));
    }

    #[test]
    fn test_command_substitution_backticks() {
        let commands: Vec<_> = extract_commands("echo `whoami`").unwrap().collect();
        assert!(commands.iter().any(|c| c.contains("echo")));
        assert!(commands.contains(&"whoami".to_string()));
    }

    #[test]
    fn test_process_substitution_input() {
        let commands: Vec<_> = extract_commands("cat <(ls)").unwrap().collect();
        assert!(commands.iter().any(|c| c.contains("cat")));
        assert!(commands.contains(&"ls".to_string()));
    }

    #[test]
    fn test_process_substitution_output() {
        let commands: Vec<_> = extract_commands("ls >(cat)").unwrap().collect();
        assert!(commands.iter().any(|c| c.contains("ls")));
        assert!(commands.contains(&"cat".to_string()));
    }

    #[test]
    fn test_newline_separator() {
        let commands: Vec<_> = extract_commands("ls\nrm -rf /").unwrap().collect();
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_subshell() {
        let commands: Vec<_> = extract_commands("(ls && rm -rf /)").unwrap().collect();
        assert_eq!(commands, vec!["ls", "rm -rf /"]);
    }

    #[test]
    fn test_mixed_operators() {
        let commands: Vec<_> = extract_commands("ls; echo hello && rm -rf /")
            .unwrap()
            .collect();
        assert_eq!(commands, vec!["ls", "echo hello", "rm -rf /"]);
    }

    #[test]
    fn test_no_spaces_around_operators() {
        let commands: Vec<_> = extract_commands("ls&&rm").unwrap().collect();
        assert_eq!(commands, vec!["ls", "rm"]);
    }

    #[test]
    fn test_nested_command_substitution() {
        let commands: Vec<_> = extract_commands("echo $(cat $(whoami).txt)")
            .unwrap()
            .collect();
        assert!(commands.iter().any(|c| c.contains("echo")));
        assert!(commands.iter().any(|c| c.contains("cat")));
        assert!(commands.contains(&"whoami".to_string()));
    }

    #[test]
    fn test_empty_command() {
        let commands: Vec<_> = extract_commands("").unwrap().collect();
        assert!(commands.is_empty());
    }

    #[test]
    fn test_invalid_syntax_error() {
        let result = extract_commands("ls &&");
        assert!(result.is_err());
    }
}
