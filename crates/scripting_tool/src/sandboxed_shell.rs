/// Models will commonly generate POSIX shell one-liner commands which
/// they run via io.popen() in Lua. Instead of giving those shell command
/// strings to the operating system - which is a security risk, and
/// which can eaisly fail on Windows, since Windows doesn't do POSIX - we
/// parse the shell command ourselves and translate it into a sequence of
/// commands in our normal sandbox. Essentially, this is an extremely
/// minimalstic shell which Lua popen() commands can execute in.
///
/// Our shell supports:
/// - Basic commands and args
/// - The operators `|`, `&&`, `;`, `>`, `1>`, `2>`, `&>`, `>&`
///
/// The operators currently have to have whitespace around them because the
/// `shlex` crate we use to tokenize the strings does not treat operators
/// as word boundaries, even though shells do. Fortunately, LLMs consistently
/// generate spaces around these operators anyway.
use mlua::{Error, Result};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShellCmd {
    pub command: String,
    pub args: Vec<String>,
    pub stdout_redirect: Option<String>,
    pub stderr_redirect: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// The `|` shell operator (highest precedence)
    Pipe,
    /// The `&&` shell operator (medium precedence)
    And,
    /// The `;` shell operator (lowest precedence)
    Semicolon,
}

impl Operator {
    fn precedence(&self) -> u8 {
        match self {
            Operator::Pipe => 3,
            Operator::And => 2,
            Operator::Semicolon => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShellAst {
    Command(ShellCmd),
    Operation {
        operator: Operator,
        left: Box<ShellAst>,
        right: Box<ShellAst>,
    },
}

impl ShellAst {
    /// Parse a shell string and build an abstract syntax tree.
    pub fn parse(string: impl AsRef<str>) -> Result<Self> {
        let string = string.as_ref();

        // Check for unsupported shell features
        if string.contains('$')
            || string.contains('`')
            || string.contains('(')
            || string.contains(')')
            || string.contains('{')
            || string.contains('}')
        {
            return Err(Error::RuntimeError(
              "Complex shell features (subshells, variables, backgrounding, etc.) are not available in this shell."
                  .to_string(),
          ));
        }

        let mut parser = ShellParser::new(string);
        parser.parse_expression(0)
    }
}

enum Redirect {
    Stdout,
    Stderr,
    Both,
}

struct ShellParser<'a> {
    lexer: shlex::Shlex<'a>,
    current_token: Option<String>,
}

impl<'a> ShellParser<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = shlex::Shlex::new(input);
        let current_token = lexer.next();

        Self {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next();
    }

    fn peek(&self) -> Option<&str> {
        self.current_token.as_deref()
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<ShellAst> {
        // Parse the first command or atom
        let mut left = ShellAst::Command(self.parse_command()?);

        // While we have operators with sufficient precedence, keep building the tree
        loop {
            let op = match self.parse_operator() {
                Some(op) if op.precedence() >= min_precedence => op,
                _ => break,
            };

            // Consume the operator token
            self.advance();

            // Special case for trailing semicolons - if we have no more tokens,
            // we don't need to parse another command
            if op == Operator::Semicolon && self.peek().is_none() {
                break;
            }

            // Parse the right side with higher precedence
            // For left-associative operators, we use op.precedence() + 1
            let right = self.parse_expression(op.precedence() + 1)?;

            // Build the operation node
            left = ShellAst::Operation {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_operator(&self) -> Option<Operator> {
        match self.peek()? {
            "|" => Some(Operator::Pipe),
            "&&" => Some(Operator::And),
            ";" => Some(Operator::Semicolon),
            _ => None,
        }
    }

    fn handle_redirection(&mut self, cmd: &mut ShellCmd, redirect: Redirect) -> Result<()> {
        self.advance(); // consume the redirection operator

        let target = self.peek().ok_or_else(|| {
            Error::RuntimeError("Missing redirection target in shell".to_string())
        })?;

        match redirect {
            Redirect::Stdout => {
                cmd.stdout_redirect = Some(target.to_string());
            }
            Redirect::Stderr => {
                cmd.stderr_redirect = Some(target.to_string());
            }
            Redirect::Both => {
                cmd.stdout_redirect = Some(target.to_string());
                cmd.stderr_redirect = Some(target.to_string());
            }
        }

        self.advance(); // consume the target

        Ok(())
    }

    fn parse_command(&mut self) -> Result<ShellCmd> {
        let mut cmd = ShellCmd::default();

        // Process tokens until we hit an operator or end of input
        loop {
            let redirect;

            match self.peek() {
                Some(token) => {
                    match token {
                        "|" | "&&" | ";" => break, // These are operators, not part of the command
                        ">" | "1>" => {
                            redirect = Some(Redirect::Stdout);
                        }
                        "2>" => {
                            redirect = Some(Redirect::Stderr);
                        }
                        "&>" | ">&" => {
                            redirect = Some(Redirect::Both);
                        }
                        "&" => {
                            // Reject ampersand as it's used for backgrounding processes
                            return Err(Error::RuntimeError(
                                "Background processes (using &) are not available in this shell."
                                    .to_string(),
                            ));
                        }
                        _ => {
                            redirect = None;
                        }
                    }
                }
                None => {
                    break; // We ran out of tokens; exit the loop.
                }
            }

            // We do this separate conditional after the borrow from the peek()
            // has expired, to avoid a borrow checker error.
            match redirect {
                Some(redirect) => {
                    self.handle_redirection(&mut cmd, redirect)?;
                }
                None => {
                    // It's either the command name or an argument
                    let mut token = self.current_token.take().unwrap();
                    self.advance();

                    // Handle trailing semicolons
                    let original_token_len = token.len();
                    while token.ends_with(';') {
                        token.pop();
                    }

                    let had_semicolon = token.len() != original_token_len;

                    if cmd.command.is_empty() {
                        cmd.command = token;
                    } else {
                        cmd.args.push(token);
                    }

                    if had_semicolon {
                        // Put the semicolon back as the next token, so after we break we parse it.
                        self.current_token = Some(";".to_string());
                        break;
                    }
                }
            }
        }

        if cmd.command.is_empty() {
            return Err(Error::RuntimeError(
                "Missing command to run in shell".to_string(),
            ));
        }

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        // Basic command with no args or operators
        let cmd = "ls";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Command(shell_cmd) = ast {
            assert_eq!(shell_cmd.command, "ls");
            assert!(shell_cmd.args.is_empty());
            assert_eq!(shell_cmd.stdout_redirect, None);
            assert_eq!(shell_cmd.stderr_redirect, None);
        } else {
            panic!("Expected Command node");
        }
    }

    #[test]
    fn test_command_with_args() {
        // Command with arguments
        let cmd = "ls -la /home";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Command(shell_cmd) = ast {
            assert_eq!(shell_cmd.command, "ls");
            assert_eq!(shell_cmd.args, vec!["-la".to_string(), "/home".to_string()]);
            assert_eq!(shell_cmd.stdout_redirect, None);
            assert_eq!(shell_cmd.stderr_redirect, None);
        } else {
            panic!("Expected Command node");
        }
    }

    #[test]
    fn test_simple_pipe() {
        // Test pipe operator
        let cmd = "ls -l | grep txt";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Operation {
            operator,
            left,
            right,
        } = ast
        {
            assert_eq!(operator, Operator::Pipe);

            if let ShellAst::Command(left_cmd) = *left {
                assert_eq!(left_cmd.command, "ls");
                assert_eq!(left_cmd.args, vec!["-l".to_string()]);
            } else {
                panic!("Expected Command node for left side");
            }

            if let ShellAst::Command(right_cmd) = *right {
                assert_eq!(right_cmd.command, "grep");
                assert_eq!(right_cmd.args, vec!["txt".to_string()]);
            } else {
                panic!("Expected Command node for right side");
            }
        } else {
            panic!("Expected Operation node");
        }
    }

    #[test]
    fn test_simple_and() {
        // Test && operator
        let cmd = "mkdir test && cd test";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Operation {
            operator,
            left,
            right,
        } = ast
        {
            assert_eq!(operator, Operator::And);

            if let ShellAst::Command(left_cmd) = *left {
                assert_eq!(left_cmd.command, "mkdir");
                assert_eq!(left_cmd.args, vec!["test".to_string()]);
            } else {
                panic!("Expected Command node for left side");
            }

            if let ShellAst::Command(right_cmd) = *right {
                assert_eq!(right_cmd.command, "cd");
                assert_eq!(right_cmd.args, vec!["test".to_string()]);
            } else {
                panic!("Expected Command node for right side");
            }
        } else {
            panic!("Expected Operation node");
        }
    }

    #[test]
    fn test_complex_chain_with_precedence() {
        // Test a more complex chain with different precedence levels
        let cmd = "echo hello | grep e && ls -l ; echo done";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        // The tree should be structured with precedence:
        // - Pipe has highest precedence
        // - Then And
        // - Then Semicolon (lowest)

        if let ShellAst::Operation {
            operator,
            left,
            right,
        } = &ast
        {
            assert_eq!(*operator, Operator::Semicolon);

            if let ShellAst::Operation {
                operator,
                left: inner_left,
                right: inner_right,
            } = &**left
            {
                assert_eq!(*operator, Operator::And);

                if let ShellAst::Operation {
                    operator,
                    left: pipe_left,
                    right: pipe_right,
                } = &**inner_left
                {
                    assert_eq!(*operator, Operator::Pipe);

                    if let ShellAst::Command(cmd) = &**pipe_left {
                        assert_eq!(cmd.command, "echo");
                        assert_eq!(cmd.args, vec!["hello".to_string()]);
                    } else {
                        panic!("Expected Command node for pipe left branch");
                    }

                    if let ShellAst::Command(cmd) = &**pipe_right {
                        assert_eq!(cmd.command, "grep");
                        assert_eq!(cmd.args, vec!["e".to_string()]);
                    } else {
                        panic!("Expected Command node for pipe right branch");
                    }
                } else {
                    panic!("Expected Pipe operation node");
                }

                if let ShellAst::Command(cmd) = &**inner_right {
                    assert_eq!(cmd.command, "ls");
                    assert_eq!(cmd.args, vec!["-l".to_string()]);
                } else {
                    panic!("Expected Command node for and right branch");
                }
            } else {
                panic!("Expected And operation node");
            }

            if let ShellAst::Command(cmd) = &**right {
                assert_eq!(cmd.command, "echo");
                assert_eq!(cmd.args, vec!["done".to_string()]);
            } else {
                panic!("Expected Command node for semicolon right branch");
            }
        } else {
            panic!("Expected Semicolon operation node");
        }
    }

    #[test]
    fn test_stdout_redirection() {
        // Test stdout redirection
        let cmd = "echo hello > output.txt";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Command(shell_cmd) = ast {
            assert_eq!(shell_cmd.command, "echo");
            assert_eq!(shell_cmd.args, vec!["hello".to_string()]);
            assert_eq!(shell_cmd.stdout_redirect, Some("output.txt".to_string()));
            assert_eq!(shell_cmd.stderr_redirect, None);
        } else {
            panic!("Expected Command node");
        }
    }

    #[test]
    fn test_stderr_redirection() {
        // Test stderr redirection
        let cmd = "find / -name test 2> errors.log";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Command(shell_cmd) = ast {
            assert_eq!(shell_cmd.command, "find");
            assert_eq!(
                shell_cmd.args,
                vec!["/".to_string(), "-name".to_string(), "test".to_string()]
            );
            assert_eq!(shell_cmd.stdout_redirect, None);
            assert_eq!(shell_cmd.stderr_redirect, Some("errors.log".to_string()));
        } else {
            panic!("Expected Command node");
        }
    }

    #[test]
    fn test_both_redirections() {
        // Test both stdout and stderr redirection
        let cmd = "make &> build.log";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Command(shell_cmd) = ast {
            assert_eq!(shell_cmd.command, "make");
            assert!(shell_cmd.args.is_empty());
            assert_eq!(shell_cmd.stdout_redirect, Some("build.log".to_string()));
            assert_eq!(shell_cmd.stderr_redirect, Some("build.log".to_string()));
        } else {
            panic!("Expected Command node");
        }

        // Test alternative syntax
        let cmd = "make >& build.log";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Command(shell_cmd) = ast {
            assert_eq!(shell_cmd.command, "make");
            assert!(shell_cmd.args.is_empty());
            assert_eq!(shell_cmd.stdout_redirect, Some("build.log".to_string()));
            assert_eq!(shell_cmd.stderr_redirect, Some("build.log".to_string()));
        } else {
            panic!("Expected Command node");
        }
    }

    #[test]
    fn test_multiple_operators() {
        // Test multiple operators in a single command
        let cmd =
            "find . -name \"*.rs\" | grep impl && echo \"Found implementations\" ; echo \"Done\"";

        // Verify the AST structure
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Operation {
            operator: semicolon_op,
            left: semicolon_left,
            right: semicolon_right,
        } = ast
        {
            assert_eq!(semicolon_op, Operator::Semicolon);

            if let ShellAst::Operation {
                operator: and_op,
                left: and_left,
                right: and_right,
            } = *semicolon_left
            {
                assert_eq!(and_op, Operator::And);

                if let ShellAst::Operation {
                    operator: pipe_op,
                    left: pipe_left,
                    right: pipe_right,
                } = *and_left
                {
                    assert_eq!(pipe_op, Operator::Pipe);

                    if let ShellAst::Command(cmd) = *pipe_left {
                        assert_eq!(cmd.command, "find");
                        assert_eq!(
                            cmd.args,
                            vec![".".to_string(), "-name".to_string(), "*.rs".to_string()]
                        );
                    } else {
                        panic!("Expected Command node for pipe left");
                    }

                    if let ShellAst::Command(cmd) = *pipe_right {
                        assert_eq!(cmd.command, "grep");
                        assert_eq!(cmd.args, vec!["impl".to_string()]);
                    } else {
                        panic!("Expected Command node for pipe right");
                    }
                } else {
                    panic!("Expected Pipe operation");
                }

                if let ShellAst::Command(cmd) = *and_right {
                    assert_eq!(cmd.command, "echo");
                    assert_eq!(cmd.args, vec!["Found implementations".to_string()]);
                } else {
                    panic!("Expected Command node for and right");
                }
            } else {
                panic!("Expected And operation");
            }

            if let ShellAst::Command(cmd) = *semicolon_right {
                assert_eq!(cmd.command, "echo");
                assert_eq!(cmd.args, vec!["Done".to_string()]);
            } else {
                panic!("Expected Command node for semicolon right");
            }
        } else {
            panic!("Expected Semicolon operation at root");
        }
    }

    #[test]
    fn test_pipe_with_redirections() {
        // Test pipe with redirections
        let cmd = "cat file.txt | grep error > results.txt 2> errors.log";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Operation {
            operator,
            left,
            right,
        } = ast
        {
            assert_eq!(operator, Operator::Pipe);

            if let ShellAst::Command(left_cmd) = *left {
                assert_eq!(left_cmd.command, "cat");
                assert_eq!(left_cmd.args, vec!["file.txt".to_string()]);
                assert_eq!(left_cmd.stdout_redirect, None);
                assert_eq!(left_cmd.stderr_redirect, None);
            } else {
                panic!("Expected Command node for left side");
            }

            if let ShellAst::Command(right_cmd) = *right {
                assert_eq!(right_cmd.command, "grep");
                assert_eq!(right_cmd.args, vec!["error".to_string()]);
                assert_eq!(right_cmd.stdout_redirect, Some("results.txt".to_string()));
                assert_eq!(right_cmd.stderr_redirect, Some("errors.log".to_string()));
            } else {
                panic!("Expected Command node for right side");
            }
        } else {
            panic!("Expected Operation node");
        }
    }

    #[test]
    fn test_quoted_arguments() {
        // Test quoted arguments
        let cmd = "echo \"hello world\" | grep \"o w\"";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Operation {
            operator,
            left,
            right,
        } = ast
        {
            assert_eq!(operator, Operator::Pipe);

            if let ShellAst::Command(left_cmd) = *left {
                assert_eq!(left_cmd.command, "echo");
                assert_eq!(left_cmd.args, vec!["hello world".to_string()]);
            } else {
                panic!("Expected Command node for left side");
            }

            if let ShellAst::Command(right_cmd) = *right {
                assert_eq!(right_cmd.command, "grep");
                assert_eq!(right_cmd.args, vec!["o w".to_string()]);
            } else {
                panic!("Expected Command node for right side");
            }
        } else {
            panic!("Expected Operation node");
        }
    }

    #[test]
    fn test_unsupported_features() {
        // Test unsupported shell features
        let result = ShellAst::parse("echo $HOME");
        assert!(result.is_err());

        let result = ShellAst::parse("echo `date`");
        assert!(result.is_err());

        let result = ShellAst::parse("echo $(date)");
        assert!(result.is_err());

        let result = ShellAst::parse("for i in {1..5}; do echo $i; done");
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_command() {
        let cmd = "find /path/to/dir -type f -name \"*.txt\" -exec grep \"pattern with spaces\";";
        let ast = ShellAst::parse(cmd).expect("parsing failed for {cmd:?}");

        if let ShellAst::Command(shell_cmd) = ast {
            assert_eq!(shell_cmd.command, "find");
            assert_eq!(
                shell_cmd.args,
                vec![
                    "/path/to/dir".to_string(),
                    "-type".to_string(),
                    "f".to_string(),
                    "-name".to_string(),
                    "*.txt".to_string(),
                    "-exec".to_string(),
                    "grep".to_string(),
                    "pattern with spaces".to_string(),
                ]
            );
            assert_eq!(shell_cmd.stdout_redirect, None);
            assert_eq!(shell_cmd.stderr_redirect, None);
        } else {
            panic!("Expected Command node");
        }
    }

    #[test]
    fn test_empty_command() {
        // Test empty command
        let result = ShellAst::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_redirection_target() {
        // Test missing redirection target
        let result = ShellAst::parse("echo hello >");
        assert!(result.is_err());

        let result = ShellAst::parse("ls 2>");
        assert!(result.is_err());
    }

    #[test]
    fn test_ampersand_as_argument() {
        // Test & as a background operator is not allowed
        let result = ShellAst::parse("grep & file.txt");
        assert!(result.is_err());

        // Verify the error message mentions background processes
        if let Err(Error::RuntimeError(msg)) = ShellAst::parse("grep & file.txt") {
            assert!(msg.contains("Background processes"));
        } else {
            panic!("Expected RuntimeError about background processes");
        }
    }
}
