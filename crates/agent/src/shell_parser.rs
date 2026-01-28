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
    let mut compound_lists: Vec<&ast::CompoundList> = Vec::new();
    let mut compound_commands: Vec<&ast::CompoundCommand> = Vec::new();
    let mut words: Vec<&ast::Word> = Vec::new();

    for complete_command in &program.complete_commands {
        compound_lists.push(complete_command);
    }

    while !compound_lists.is_empty() || !compound_commands.is_empty() || !words.is_empty() {
        while let Some(compound_list) = compound_lists.pop() {
            for item in &compound_list.0 {
                let and_or_list = &item.0;

                let mut pipelines = vec![&and_or_list.first];
                for and_or in &and_or_list.additional {
                    match and_or {
                        ast::AndOr::And(p) | ast::AndOr::Or(p) => pipelines.push(p),
                    }
                }

                for pipeline in pipelines {
                    for cmd in &pipeline.seq {
                        match cmd {
                            ast::Command::Simple(simple) => {
                                let command_str = simple.to_string();
                                if !command_str.trim().is_empty() {
                                    commands.push(command_str);
                                }

                                let prefix_suffix_items = simple
                                    .prefix
                                    .iter()
                                    .flat_map(|p| p.0.iter())
                                    .chain(simple.suffix.iter().flat_map(|s| s.0.iter()));

                                for item in prefix_suffix_items {
                                    match item {
                                        ast::CommandPrefixOrSuffixItem::IoRedirect(redirect) => {
                                            match redirect {
                                                ast::IoRedirect::File(_, _, target) => {
                                                    if let ast::IoFileRedirectTarget::ProcessSubstitution(_, subshell) = target {
                                                        compound_lists.push(&subshell.list);
                                                    }
                                                }
                                                ast::IoRedirect::HereString(_, word) => {
                                                    words.push(word);
                                                }
                                                ast::IoRedirect::OutputAndError(word, _) => {
                                                    words.push(word);
                                                }
                                                ast::IoRedirect::HereDocument(_, _) => {}
                                            }
                                        }
                                        ast::CommandPrefixOrSuffixItem::AssignmentWord(assignment, _) => {
                                            match &assignment.value {
                                                ast::AssignmentValue::Scalar(word) => {
                                                    words.push(word);
                                                }
                                                ast::AssignmentValue::Array(arr) => {
                                                    for (opt_word, word) in arr {
                                                        if let Some(w) = opt_word {
                                                            words.push(w);
                                                        }
                                                        words.push(word);
                                                    }
                                                }
                                            }
                                        }
                                        ast::CommandPrefixOrSuffixItem::Word(word) => {
                                            words.push(word);
                                        }
                                        ast::CommandPrefixOrSuffixItem::ProcessSubstitution(_, subshell) => {
                                            compound_lists.push(&subshell.list);
                                        }
                                    }
                                }

                                if let Some(word) = &simple.word_or_name {
                                    words.push(word);
                                }
                            }
                            ast::Command::Compound(compound, _) => {
                                compound_commands.push(compound);
                            }
                            ast::Command::Function(func_def) => {
                                compound_commands.push(&func_def.body.0);
                            }
                            ast::Command::ExtendedTest(test_expr) => {
                                let mut test_exprs = vec![&test_expr.expr];
                                while let Some(expr) = test_exprs.pop() {
                                    match expr {
                                        ast::ExtendedTestExpr::Not(inner)
                                        | ast::ExtendedTestExpr::Parenthesized(inner) => {
                                            test_exprs.push(inner);
                                        }
                                        ast::ExtendedTestExpr::And(left, right)
                                        | ast::ExtendedTestExpr::Or(left, right) => {
                                            test_exprs.push(left);
                                            test_exprs.push(right);
                                        }
                                        ast::ExtendedTestExpr::UnaryTest(_, word) => {
                                            words.push(word);
                                        }
                                        ast::ExtendedTestExpr::BinaryTest(_, word1, word2) => {
                                            words.push(word1);
                                            words.push(word2);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        while let Some(compound) = compound_commands.pop() {
            match compound {
                ast::CompoundCommand::BraceGroup(bg) => {
                    compound_lists.push(&bg.list);
                }
                ast::CompoundCommand::Subshell(sub) => {
                    compound_lists.push(&sub.list);
                }
                ast::CompoundCommand::ForClause(fc) => {
                    if let Some(values) = &fc.values {
                        words.extend(values.iter());
                    }
                    compound_lists.push(&fc.body.list);
                }
                ast::CompoundCommand::CaseClause(cc) => {
                    words.push(&cc.value);
                    for item in &cc.cases {
                        if let Some(body) = &item.cmd {
                            compound_lists.push(body);
                        }
                    }
                }
                ast::CompoundCommand::IfClause(ic) => {
                    compound_lists.push(&ic.condition);
                    compound_lists.push(&ic.then);
                    if let Some(elses) = &ic.elses {
                        for else_item in elses {
                            if let Some(cond) = &else_item.condition {
                                compound_lists.push(cond);
                            }
                            compound_lists.push(&else_item.body);
                        }
                    }
                }
                ast::CompoundCommand::WhileClause(wc) | ast::CompoundCommand::UntilClause(wc) => {
                    compound_lists.push(&wc.0);
                    compound_lists.push(&wc.1.list);
                }
                ast::CompoundCommand::ArithmeticForClause(afc) => {
                    compound_lists.push(&afc.body.list);
                }
                ast::CompoundCommand::Arithmetic(_) => {}
            }
        }

        while let Some(word) = words.pop() {
            if let Ok(pieces) = brush_parser::word::parse(&word.value, &options) {
                let mut word_pieces: Vec<&WordPiece> = pieces.iter().map(|p| &p.piece).collect();
                while let Some(piece) = word_pieces.pop() {
                    match piece {
                        WordPiece::CommandSubstitution(cmd_str)
                        | WordPiece::BackquotedCommandSubstitution(cmd_str) => {
                            if let Some(nested) = extract_commands(cmd_str) {
                                commands.extend(nested);
                            }
                        }
                        WordPiece::DoubleQuotedSequence(inner)
                        | WordPiece::GettextDoubleQuotedSequence(inner) => {
                            word_pieces.extend(inner.iter().map(|p| &p.piece));
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
            }
        }
    }

    Some(commands)
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
    fn test_and_operator() {
        let commands = extract_commands("ls && rm -rf /").expect("parse failed");
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"rm -rf /".to_string()));
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_or_operator() {
        let commands = extract_commands("ls || rm -rf /").expect("parse failed");
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"rm -rf /".to_string()));
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_semicolon() {
        let commands = extract_commands("ls; rm -rf /").expect("parse failed");
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"rm -rf /".to_string()));
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_pipe() {
        let commands = extract_commands("ls | xargs rm -rf").expect("parse failed");
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"xargs rm -rf".to_string()));
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_background() {
        let commands = extract_commands("ls & rm -rf /").expect("parse failed");
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"rm -rf /".to_string()));
        assert_eq!(commands.len(), 2);
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
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"rm -rf /".to_string()));
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_subshell() {
        let commands = extract_commands("(ls && rm -rf /)").expect("parse failed");
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"rm -rf /".to_string()));
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_mixed_operators() {
        let commands = extract_commands("ls; echo hello && rm -rf /").expect("parse failed");
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"echo hello".to_string()));
        assert!(commands.contains(&"rm -rf /".to_string()));
        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_no_spaces_around_operators() {
        let commands = extract_commands("ls&&rm").expect("parse failed");
        assert!(commands.contains(&"ls".to_string()));
        assert!(commands.contains(&"rm".to_string()));
        assert_eq!(commands.len(), 2);
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
